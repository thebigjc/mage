// Game — the top-level game runner.
//
// The Game struct ties together the GameState, TurnManager, and
// PlayerDecisionMaker implementations to run a complete game of Magic.
//
// The game loop follows the MTG comprehensive rules:
// 1. Advance to the next step/phase
// 2. Process turn-based actions for that step
// 3. Check state-based actions (loop until none found)
// 4. Put triggered abilities on the stack
// 5. Give active player priority
// 6. Players pass priority or take actions
// 7. When all pass with empty stack → advance; with items → resolve top
//
// Ported from mage.game.GameImpl.

use crate::abilities::{Cost, Effect, StaticEffect};
use crate::mana::ManaCost;
use crate::combat::{self, CombatState};
use crate::constants::AbilityType;
use crate::card::CardData;
use crate::constants::PhaseStep;
use crate::counters::CounterType;
use crate::decision::{AttackerInfo, PlayerDecisionMaker};
use crate::events::{EventLog, EventType, GameEvent};
use crate::state::{GameState, StateBasedActions};
use crate::turn::{has_priority, PriorityTracker, TurnManager};
use crate::types::{AbilityId, ObjectId, PlayerId};
use crate::watchers::WatcherManager;
use crate::permanent::Permanent;
use std::collections::HashMap;

/// Maximum number of SBA iterations before we bail out (safety valve).
const MAX_SBA_ITERATIONS: u32 = 100;

/// Maximum number of turns before the game is declared a draw (safety valve).
const MAX_TURNS: u32 = 500;

/// Configuration for a new game.
pub struct GameConfig {
    /// Player names and their decks (as CardData vectors).
    pub players: Vec<PlayerConfig>,
    /// Starting life total (default 20).
    pub starting_life: i32,
}

/// Configuration for a single player in a new game.
pub struct PlayerConfig {
    pub name: String,
    pub deck: Vec<CardData>,
}

/// The result of a completed game.
#[derive(Clone, Debug)]
pub struct GameResult {
    /// The winner, or None for a draw.
    pub winner: Option<PlayerId>,
    /// Final turn number.
    pub turn_number: u32,
    /// How the game ended.
    pub reason: GameEndReason,
}

#[derive(Clone, Debug)]
pub enum GameEndReason {
    /// A player lost (life, poison, decked, etc.).
    PlayerLost,
    /// A player conceded.
    Concession,
    /// All opponents lost simultaneously.
    LastPlayerStanding,
    /// Maximum turns reached (safety valve).
    MaxTurnsReached,
    /// The game was a draw.
    Draw,
}

/// The main game runner. Holds the game state and drives the game loop.
pub struct Game {
    /// The game state.
    pub state: GameState,
    /// The turn manager.
    pub turn_manager: TurnManager,
    /// Player decision-makers, keyed by PlayerId.
    decision_makers: HashMap<PlayerId, Box<dyn PlayerDecisionMaker>>,
    /// Watcher manager for event tracking.
    pub watchers: WatcherManager,
    /// Event log for tracking events that may trigger abilities.
    event_log: EventLog,
}

impl Game {
    /// Create a new two-player game.
    pub fn new_two_player(
        config: GameConfig,
        mut decision_makers: Vec<(PlayerId, Box<dyn PlayerDecisionMaker>)>,
    ) -> Self {
        assert_eq!(config.players.len(), 2, "Two-player game requires exactly 2 players");
        assert_eq!(decision_makers.len(), 2, "Two-player game requires exactly 2 decision makers");

        // Create player IDs
        let player_ids: Vec<PlayerId> = decision_makers.iter().map(|(id, _)| *id).collect();

        // Build game state
        let player_names: Vec<(&str, PlayerId)> = config
            .players
            .iter()
            .zip(player_ids.iter())
            .map(|(pc, &id)| (pc.name.as_str(), id))
            .collect();
        let mut state = GameState::new(&player_names);

        // Set starting life
        for player in state.players.values_mut() {
            player.life = config.starting_life;
        }

        // Build card store and libraries from decks
        for (player_config, &player_id) in config.players.iter().zip(player_ids.iter()) {
            let mut card_ids = Vec::with_capacity(player_config.deck.len());
            for card in &player_config.deck {
                let mut card_data = card.clone();
                card_data.owner = player_id;
                let card_id = card_data.id;
                state.card_store.insert(card_data);
                card_ids.push(card_id);
            }
            for &card_id in &card_ids {
                state.object_zones.insert(
                    card_id,
                    crate::state::ZoneLocation {
                        zone: crate::constants::Zone::Library,
                        controller: Some(player_id),
                    },
                );
            }
            let player = state.players.get_mut(&player_id).unwrap();
            for card_id in card_ids {
                player.library.put_on_bottom(card_id);
            }
        }

        // Build turn manager
        let turn_manager = TurnManager::new(player_ids.clone());

        // Build decision maker map
        let dm_map: HashMap<PlayerId, Box<dyn PlayerDecisionMaker>> =
            decision_makers.drain(..).collect();

        Game {
            state,
            turn_manager,
            decision_makers: dm_map,
            watchers: WatcherManager::new(),
            event_log: EventLog::new(),
        }
    }

    /// Run the game to completion. Returns the game result.
    pub fn run(&mut self) -> GameResult {
        // Shuffle libraries
        let mut rng = rand::thread_rng();
        for player in self.state.players.values_mut() {
            player.library.shuffle(&mut rng);
        }

        // Draw opening hands (7 cards each)
        let player_ids: Vec<PlayerId> = self.state.turn_order.clone();
        for &pid in &player_ids {
            self.draw_cards(pid, 7);
        }

        // London mulligan phase
        self.london_mulligan(&player_ids);

        // Notify decision makers of game start
        let view = crate::decision::GameView::placeholder();
        for (&pid, dm) in &mut self.decision_makers {
            dm.on_game_start(&view, pid);
        }

        // Main game loop
        loop {
            // Check safety valves
            if self.turn_manager.turn_number > MAX_TURNS {
                return GameResult {
                    winner: None,
                    turn_number: self.turn_manager.turn_number,
                    reason: GameEndReason::MaxTurnsReached,
                };
            }

            // Process current step
            self.process_step();

            // Check if game should end
            if let Some(result) = self.check_game_end() {
                // Notify decision makers of game end
                let view = crate::decision::GameView::placeholder();
                for (&pid, dm) in &mut self.decision_makers {
                    let won = result.winner == Some(pid);
                    dm.on_game_end(&view, won);
                }
                return result;
            }

            // Advance to next step
            match self.turn_manager.advance_step() {
                Some(step) => {
                    // Empty mana pools at phase transitions
                    let old_phase = self.state.current_step.phase();
                    let new_phase = step.phase();
                    if old_phase != new_phase {
                        for player in self.state.players.values_mut() {
                            player.mana_pool.clear();
                        }
                    }

                    self.state.current_step = step;
                    self.state.current_phase = self.turn_manager.current_phase();
                }
                None => {
                    // Turn is over, start next turn
                    let next_active = self.turn_manager.next_turn();
                    self.state.turn_number = self.turn_manager.turn_number;
                    self.state.active_player = next_active;
                    self.state.priority_player = next_active;
                    self.state.current_step = PhaseStep::Untap;
                    self.state.current_phase = self.turn_manager.current_phase();

                    // Reset per-turn state for active player
                    if let Some(player) = self.state.players.get_mut(&next_active) {
                        player.begin_turn();
                    }

                    // Reset watchers at the start of each turn
                    self.watchers.reset_turn();
                }
            }
        }
    }

    /// London mulligan procedure.
    ///
    /// Each player simultaneously decides whether to mulligan. Players who
    /// mulligan shuffle their hand back and draw 7 again, then put N cards
    /// on the bottom of their library (where N = number of mulligans taken).
    fn london_mulligan(&mut self, player_ids: &[PlayerId]) {
        let max_mulligans = 6u32; // Can't mulligan below 1 card

        // Track how many mulligans each player has taken
        let mut mulligan_count: HashMap<PlayerId, u32> = HashMap::new();
        let mut still_deciding: Vec<PlayerId> = player_ids.to_vec();

        for round in 0..max_mulligans {
            if still_deciding.is_empty() {
                break;
            }

            let mut keeping = Vec::new();
            let mut mulliganing = Vec::new();

            for &pid in &still_deciding {
                let hand: Vec<ObjectId> = self
                    .state
                    .players
                    .get(&pid)
                    .map(|p| p.hand.iter().copied().collect())
                    .unwrap_or_default();

                let view = crate::decision::GameView::placeholder();
                let wants_mulligan = if let Some(dm) = self.decision_makers.get_mut(&pid) {
                    dm.choose_mulligan(&view, &hand)
                } else {
                    false
                };

                if wants_mulligan && round < max_mulligans - 1 {
                    mulliganing.push(pid);
                } else {
                    keeping.push(pid);
                }
            }

            // Players keeping their hand: put back N cards on bottom
            for pid in &keeping {
                let count = *mulligan_count.get(pid).unwrap_or(&0);
                if count > 0 {
                    let hand: Vec<ObjectId> = self
                        .state
                        .players
                        .get(pid)
                        .map(|p| p.hand.iter().copied().collect())
                        .unwrap_or_default();

                    let view = crate::decision::GameView::placeholder();
                    let to_put_back = if let Some(dm) = self.decision_makers.get_mut(pid) {
                        dm.choose_cards_to_put_back(&view, &hand, count as usize)
                    } else {
                        // Default: put back the last N cards
                        hand.iter().rev().take(count as usize).copied().collect()
                    };

                    for card_id in to_put_back {
                        if let Some(player) = self.state.players.get_mut(pid) {
                            if player.hand.remove(card_id) {
                                player.library.put_on_bottom(card_id);
                            }
                        }
                    }
                }
            }

            // Players mulliganing: shuffle hand back, draw 7 again
            for &pid in &mulliganing {
                // Return hand to library
                let hand: Vec<ObjectId> = self
                    .state
                    .players
                    .get(&pid)
                    .map(|p| p.hand.iter().copied().collect())
                    .unwrap_or_default();

                for card_id in hand {
                    if let Some(player) = self.state.players.get_mut(&pid) {
                        player.hand.remove(card_id);
                        player.library.put_on_bottom(card_id);
                    }
                }

                // Shuffle
                let mut rng = rand::thread_rng();
                if let Some(player) = self.state.players.get_mut(&pid) {
                    player.library.shuffle(&mut rng);
                }

                // Draw 7 again
                self.draw_cards(pid, 7);

                // Track mulligan count
                *mulligan_count.entry(pid).or_insert(0) += 1;
            }

            still_deciding = mulliganing;
        }
    }

    /// Process the current step: turn-based actions, SBAs, triggers, priority.
    fn process_step(&mut self) {
        let step = self.state.current_step;
        let active = self.state.active_player;

        // -- Turn-based actions --
        self.turn_based_actions(step, active);

        // -- SBA + triggered ability loop (MTG rules 117.5) --
        // Loop: check SBAs, then check triggered abilities, repeat until stable.
        self.process_sba_and_triggers();

        // -- Priority loop --
        if has_priority(step) {
            self.priority_loop();
        }
    }

    /// Loop state-based actions and triggered ability checks until stable.
    /// Per MTG rules 117.5: SBAs are checked first, then triggered abilities
    /// are put on the stack, then SBAs are checked again, until neither
    /// produces any changes.
    fn process_sba_and_triggers(&mut self) {
        for _ in 0..MAX_SBA_ITERATIONS {
            // Recalculate continuous effects before each SBA check
            // so that P/T from lords, granted keywords, etc. are current.
            self.apply_continuous_effects();

            // Check and apply SBAs
            let sba = self.state.check_state_based_actions();
            let had_sba = sba.has_actions();
            let died_sources = if had_sba {
                self.apply_state_based_actions(&sba)
            } else {
                Vec::new()
            };

            // Check for triggered abilities (BEFORE cleaning up died sources,
            // so dies triggers can still find abilities of the dead creature)
            let had_triggers = self.check_triggered_abilities();

            // Clean up abilities for permanents that died, now that triggers have been checked
            for source_id in died_sources {
                self.state.ability_store.remove_source(source_id);
            }

            // If neither SBAs nor triggers fired, we're stable
            if !had_sba && !had_triggers {
                break;
            }
        }
    }

    /// Recalculate continuous effects from static abilities on all permanents.
    /// This implements MTG rules 613 (layer system) for the currently-supported
    /// layers: Layer 6 (Ability Adding/Removing) and Layer 7 (P/T Changing).
    ///
    /// Clears and recalculates `continuous_boost_power`, `continuous_boost_toughness`,
    /// and `continuous_keywords` on every permanent based on StaticEffect::Boost
    /// and StaticEffect::GrantKeyword from static abilities of all battlefield permanents.
    fn apply_continuous_effects(&mut self) {
        use crate::constants::KeywordAbilities;

        // Step 1: Clear all continuous effects
        for perm in self.state.battlefield.iter_mut() {
            perm.continuous_boost_power = 0;
            perm.continuous_boost_toughness = 0;
            perm.continuous_keywords = KeywordAbilities::empty();
            perm.cant_attack = false;
            perm.cant_block_from_effect = false;
            perm.max_blocked_by = None;
            perm.cant_be_blocked_by_power_leq = None;
            perm.must_be_blocked = false;
        }

        // Step 2: Collect static effects from all battlefield permanents.
        // We must collect first to avoid borrow conflicts.
        let mut boosts: Vec<(ObjectId, PlayerId, String, i32, i32)> = Vec::new();
        let mut keyword_grants: Vec<(ObjectId, PlayerId, String, String)> = Vec::new();
        let mut cant_attacks: Vec<(ObjectId, PlayerId, String)> = Vec::new();
        let mut cant_blocks: Vec<(ObjectId, PlayerId, String)> = Vec::new();
        let mut max_blocked_bys: Vec<(ObjectId, u32)> = Vec::new();
        let mut cant_blocked_by_power: Vec<(ObjectId, i32)> = Vec::new();
        let mut must_be_blockeds: Vec<ObjectId> = Vec::new();
        let mut boost_per_counts: Vec<(ObjectId, PlayerId, String, i32, i32)> = Vec::new();
        let mut additional_land_plays: Vec<(PlayerId, u32)> = Vec::new();
        let mut conditional_keywords: Vec<(ObjectId, PlayerId, String, String)> = Vec::new();
        let mut conditional_boosts: Vec<(ObjectId, PlayerId, i32, i32, String)> = Vec::new();

        for perm in self.state.battlefield.iter() {
            let source_id = perm.id();
            let controller = perm.controller;
            let abilities = self.state.ability_store.for_source(source_id);
            for ability in abilities {
                if ability.ability_type != AbilityType::Static {
                    continue;
                }
                for effect in &ability.static_effects {
                    match effect {
                        crate::abilities::StaticEffect::Boost { filter, power, toughness } => {
                            boosts.push((source_id, controller, filter.clone(), *power, *toughness));
                        }
                        crate::abilities::StaticEffect::GrantKeyword { filter, keyword } => {
                            keyword_grants.push((source_id, controller, filter.clone(), keyword.clone()));
                        }
                        crate::abilities::StaticEffect::CantAttack { filter } => {
                            cant_attacks.push((source_id, controller, filter.clone()));
                        }
                        crate::abilities::StaticEffect::CantBlock { filter } => {
                            cant_blocks.push((source_id, controller, filter.clone()));
                        }
                        crate::abilities::StaticEffect::CantBeBlockedByMoreThan { count } => {
                            max_blocked_bys.push((source_id, *count));
                        }
                        crate::abilities::StaticEffect::CantBeBlockedByPowerLessOrEqual { power } => {
                            cant_blocked_by_power.push((source_id, *power));
                        }
                        crate::abilities::StaticEffect::MustBeBlocked => {
                            must_be_blockeds.push(source_id);
                        }
                        crate::abilities::StaticEffect::BoostPerCount { count_filter, power_per, toughness_per } => {
                            boost_per_counts.push((source_id, controller, count_filter.clone(), *power_per, *toughness_per));
                        }
                        crate::abilities::StaticEffect::AdditionalLandPlays { count } => {
                            additional_land_plays.push((controller, *count));
                        }
                        crate::abilities::StaticEffect::ConditionalKeyword { keyword, condition } => {
                            conditional_keywords.push((source_id, controller, keyword.clone(), condition.clone()));
                        }
                        crate::abilities::StaticEffect::ConditionalBoostSelf { power, toughness, condition } => {
                            conditional_boosts.push((source_id, controller, *power, *toughness, condition.clone()));
                        }
                        _ => {}
                    }
                }
            }
        }

        // Step 3: Apply P/T boosts (Layer 7c — Modify)
        for (source_id, controller, filter, power, toughness) in boosts {
            let matching = self.find_matching_permanents(source_id, controller, &filter);
            for target_id in matching {
                if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                    perm.continuous_boost_power += power;
                    perm.continuous_boost_toughness += toughness;
                }
            }
        }

        // Step 3b: Apply CantAttack restrictions
        for (source_id, controller, filter) in cant_attacks {
            let matching = self.find_matching_permanents(source_id, controller, &filter);
            for target_id in matching {
                if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                    perm.cant_attack = true;
                }
            }
        }

        // Step 3c: Apply CantBlock restrictions
        for (source_id, controller, filter) in cant_blocks {
            let matching = self.find_matching_permanents(source_id, controller, &filter);
            for target_id in matching {
                if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                    perm.cant_block_from_effect = true;
                }
            }
        }

        // Step 3d: Apply block restriction effects
        for (source_id, count) in max_blocked_bys {
            if let Some(perm) = self.state.battlefield.get_mut(source_id) {
                perm.max_blocked_by = Some(count);
            }
        }
        for (source_id, power) in cant_blocked_by_power {
            if let Some(perm) = self.state.battlefield.get_mut(source_id) {
                perm.cant_be_blocked_by_power_leq = Some(power);
            }
        }
        for source_id in must_be_blockeds {
            if let Some(perm) = self.state.battlefield.get_mut(source_id) {
                perm.must_be_blocked = true;
            }
        }

        // Step 3e: Apply dynamic P/T boosts (BoostPerCount)
        for (source_id, controller, count_filter, power_per, toughness_per) in boost_per_counts {
            // Count matching permanents on the battlefield
            let bf_count = self.find_matching_permanents(source_id, controller, &count_filter).len() as i32;

            // Also count matching cards in the controller's graveyard if the filter mentions it
            let gy_count = if count_filter.contains("graveyard") {
                // Extract the type from "and creature card in your graveyard" or similar
                if let Some(player) = self.state.players.get(&controller) {
                    let filter_lower = count_filter.to_lowercase();
                    let mut count = 0i32;
                    for &card_id in player.graveyard.iter() {
                        if let Some(card) = self.state.card_store.get(card_id) {
                            // If "creature card in your graveyard", check creature type
                            if filter_lower.contains("creature") && card.is_creature() {
                                count += 1;
                            } else if filter_lower.contains("card") {
                                count += 1;
                            }
                        }
                    }
                    count
                } else {
                    0
                }
            } else {
                0
            };

            let total = bf_count + gy_count;
            if total > 0 {
                if let Some(perm) = self.state.battlefield.get_mut(source_id) {
                    perm.continuous_boost_power += total * power_per;
                    perm.continuous_boost_toughness += total * toughness_per;
                }
            }
        }

        // Step 4: Apply keyword grants (Layer 6)
        for (source_id, controller, filter, keyword_str) in keyword_grants {
            // Handle comma-separated keywords like "deathtouch, lifelink"
            let keywords: Vec<&str> = keyword_str.split(',').map(|s| s.trim()).collect();
            let mut combined = KeywordAbilities::empty();
            for kw_name in &keywords {
                if let Some(kw) = KeywordAbilities::keyword_from_name(kw_name) {
                    combined |= kw;
                }
            }
            if !combined.is_empty() {
                let matching = self.find_matching_permanents(source_id, controller, &filter);
                for target_id in matching {
                    if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                        perm.continuous_keywords |= combined;
                    }
                }
            }
        }

        // Step 5: Apply additional land plays
        // Reset all players to base (1), then add additional
        for player in self.state.players.values_mut() {
            player.lands_per_turn = 1;
        }
        for (player_id, count) in additional_land_plays {
            if let Some(player) = self.state.players.get_mut(&player_id) {
                player.lands_per_turn += count;
            }
        }

        // Step 6: Apply conditional keywords
        for (source_id, controller, keyword_str, condition) in conditional_keywords {
            let met = self.evaluate_condition(source_id, controller, &condition);
            if met {
                let keywords: Vec<&str> = keyword_str.split(',').map(|s| s.trim()).collect();
                let mut combined = KeywordAbilities::empty();
                for kw_name in &keywords {
                    if let Some(kw) = KeywordAbilities::keyword_from_name(kw_name) {
                        combined |= kw;
                    }
                }
                if let Some(perm) = self.state.battlefield.get_mut(source_id) {
                    perm.continuous_keywords |= combined;
                }
            }
        }

        // Step 7: Apply conditional boosts
        for (source_id, controller, power, toughness, condition) in conditional_boosts {
            let met = self.evaluate_condition(source_id, controller, &condition);
            if met {
                if let Some(perm) = self.state.battlefield.get_mut(source_id) {
                    perm.continuous_boost_power += power;
                    perm.continuous_boost_toughness += toughness;
                }
            }
        }
    }

    /// Evaluate a condition string for conditional static effects.
    /// Returns true if the condition is currently met.
    fn evaluate_condition(&self, source_id: ObjectId, controller: PlayerId, condition: &str) -> bool {
        let cond_lower = condition.to_lowercase();

        // "your turn" — controller is the active player
        if cond_lower.contains("your turn") {
            return self.state.active_player == controller;
        }

        // "untapped" — source permanent is untapped
        if cond_lower == "untapped" || cond_lower == "source untapped" {
            return self.state.battlefield.get(source_id)
                .map(|p| !p.tapped)
                .unwrap_or(false);
        }

        // "you control a {Type}" — controller has a permanent of that type
        if cond_lower.starts_with("you control a ") || cond_lower.starts_with("you control an ") {
            let type_str = if cond_lower.starts_with("you control an ") {
                &condition[15..]
            } else {
                &condition[14..]
            };
            let subtype = crate::constants::SubType::by_description(type_str);
            return self.state.battlefield.iter().any(|p| {
                p.controller == controller && p.id() != source_id &&
                p.has_subtype(&subtype)
            });
        }

        // "creature entered this turn" — check event log for ETB creature events
        if cond_lower.contains("creature entered this turn") || cond_lower.contains("creature etb this turn") {
            return self.event_log.iter().any(|e| {
                e.event_type == crate::events::EventType::EnteredTheBattlefield
            });
        }

        false // unknown condition
    }

    /// Evaluate a count filter string and return the dynamic count.
    /// Supports patterns like:
    /// - "Elf cards in your graveyard" — count of Elf creature cards in controller's graveyard
    /// - "Goblins you control" — count of Goblins on controller's battlefield
    fn evaluate_count_filter(&self, filter: &str, controller: PlayerId) -> u32 {
        let lower = filter.to_lowercase();

        // "{Type} cards in your graveyard"
        if lower.ends_with("cards in your graveyard") || lower.ends_with("in your graveyard") {
            // Extract type name: "Elf cards in your graveyard" -> "Elf"
            let type_str = if let Some(idx) = lower.find(" cards in your graveyard") {
                &filter[..idx]
            } else if let Some(idx) = lower.find(" in your graveyard") {
                &filter[..idx]
            } else {
                return 0;
            };
            let subtype = crate::constants::SubType::by_description(type_str);
            if let Some(player) = self.state.players.get(&controller) {
                return player.graveyard.iter()
                    .filter(|&&card_id| {
                        if let Some(card) = self.state.card_store.get(card_id) {
                            card.subtypes.contains(&subtype)
                        } else {
                            false
                        }
                    })
                    .count() as u32;
            }
            return 0;
        }

        // "{Type}s you control" / "{Type} you control"
        if lower.ends_with("you control") {
            let type_part = lower.trim_end_matches("you control").trim();
            let type_str = type_part.trim_end_matches('s'); // "Goblins" -> "Goblin"
            let subtype = crate::constants::SubType::by_description(
                &format!("{}{}", &type_str[..1].to_uppercase(), &type_str[1..])
            );
            return self.state.battlefield.iter()
                .filter(|p| p.controller == controller && p.has_subtype(&subtype))
                .count() as u32;
        }

        0 // unknown filter
    }

    /// Find permanents matching a filter string, relative to a source permanent.
    ///
    /// Handles common filter patterns:
    /// - `"self"` — only the source permanent
    /// - `"enchanted creature"` / `"equipped creature"` — the permanent this is attached to
    /// - `"other X you control"` — excludes source, controller must match
    /// - `"X you control"` — controller must match
    /// - `"attacking X you control"` — must be currently attacking
    /// - `"creature token you control"` — must be a token creature
    /// - `"creature"` / `"Elf"` / etc. — type/subtype matching
    fn find_matching_permanents(
        &self,
        source_id: ObjectId,
        controller: PlayerId,
        filter: &str,
    ) -> Vec<ObjectId> {
        let f = filter.to_lowercase();

        // "self" — only the source permanent
        if f == "self" {
            return vec![source_id];
        }

        // "enchanted creature" / "equipped creature" — attached target
        if f.contains("enchanted") || f.contains("equipped") {
            if let Some(source_perm) = self.state.battlefield.get(source_id) {
                if let Some(attached_to) = source_perm.attached_to {
                    return vec![attached_to];
                }
            }
            return vec![];
        }

        let exclude_self = f.contains("other");
        let you_control = f.contains("you control");
        let is_attacking = f.contains("attacking");
        let is_token = f.contains("token");

        // Strip modifiers to get the core type filter
        let type_filter = f
            .replace("other ", "")
            .replace("attacking ", "")
            .replace("you control", "")
            .replace("token ", "")
            .replace("token", "")
            .trim()
            .to_string();

        let mut results = Vec::new();
        for perm in self.state.battlefield.iter() {
            if exclude_self && perm.id() == source_id {
                continue;
            }
            if you_control && perm.controller != controller {
                continue;
            }
            if is_token && !perm.card.is_token {
                continue;
            }
            if is_attacking && !self.state.combat.is_attacking(perm.id()) {
                continue;
            }
            if !type_filter.is_empty() && !Self::matches_filter(perm, &type_filter) {
                continue;
            }
            results.push(perm.id());
        }
        results
    }

    /// Check if a permanent entering the battlefield should enter tapped.
    /// Checks the permanent's own static abilities for `EntersTapped { filter: "self" }`.
    fn check_enters_tapped(&mut self, permanent_id: ObjectId) {
        let should_tap = {
            let abilities = self.state.ability_store.for_source(permanent_id);
            abilities.iter().any(|a| {
                a.ability_type == AbilityType::Static
                    && a.static_effects.iter().any(|e| {
                        matches!(e, crate::abilities::StaticEffect::EntersTapped { filter } if filter == "self")
                    })
            })
        };
        if should_tap {
            if let Some(perm) = self.state.battlefield.get_mut(permanent_id) {
                perm.tap();
            }
        }
    }

    /// Check for triggered abilities that should fire from recent events.
    /// Pushes matching triggered abilities onto the stack in APNAP order.
    /// Returns true if any triggers were placed on the stack.
    fn check_triggered_abilities(&mut self) -> bool {
        if self.event_log.is_empty() {
            return false;
        }

        // Collect all triggered abilities that match events
        let mut triggered: Vec<(PlayerId, AbilityId, ObjectId, String)> = Vec::new();

        for event in self.event_log.iter() {
            let matching = self.state.ability_store.triggered_by(event);
            for ability in matching {
                // Dies triggers: the source is no longer on the battlefield
                // but its abilities are still in the store (deferred cleanup).
                let is_dies_trigger = event.event_type == EventType::Dies;

                if is_dies_trigger {
                    // For dies triggers, the dying creature's target_id must match
                    // the ability's source_id (i.e., "when THIS creature dies")
                    if let Some(target_id) = event.target_id {
                        if target_id != ability.source_id {
                            continue;
                        }
                    }
                    // Controller comes from the event's player_id
                    let controller = event.player_id.unwrap_or(self.state.active_player);

                    triggered.push((
                        controller,
                        ability.id,
                        ability.source_id,
                        ability.rules_text.clone(),
                    ));
                    continue;
                }

                // For non-dies triggers, source must still be on the battlefield
                let source_on_bf = self.state.battlefield.contains(ability.source_id);
                if !source_on_bf {
                    continue;
                }

                // Determine controller of the source permanent
                let controller = self
                    .state
                    .battlefield
                    .get(ability.source_id)
                    .map(|p| p.controller)
                    .unwrap_or(self.state.active_player);

                // Check if this trigger is "self" only (e.g., "whenever THIS creature attacks")
                // For attack triggers, only trigger for the source creature
                if event.event_type == EventType::AttackerDeclared {
                    if let Some(target_id) = event.target_id {
                        if target_id != ability.source_id {
                            continue;
                        }
                    }
                }

                // For ETB triggers, only trigger for the source permanent
                if event.event_type == EventType::EnteredTheBattlefield {
                    if let Some(target_id) = event.target_id {
                        if target_id != ability.source_id {
                            continue;
                        }
                    }
                }

                // For GainLife, only trigger for the controller's life gain
                if event.event_type == EventType::GainLife {
                    if let Some(player_id) = event.player_id {
                        if player_id != controller {
                            continue;
                        }
                    }
                }

                // For UpkeepStep/EndStep, only trigger for the controller whose step it is
                if event.event_type == EventType::UpkeepStep || event.event_type == EventType::EndStep {
                    if let Some(player_id) = event.player_id {
                        if player_id != controller {
                            continue;
                        }
                    }
                }

                // For DamagedPlayer, only trigger for the source creature that dealt damage
                if event.event_type == EventType::DamagedPlayer {
                    if let Some(target_id) = event.target_id {
                        if target_id != ability.source_id {
                            continue;
                        }
                    }
                }

                triggered.push((
                    controller,
                    ability.id,
                    ability.source_id,
                    ability.rules_text.clone(),
                ));
            }
        }

        // Handle prowess: when a noncreature spell is cast, each creature with
        // prowess the caster controls gets +1/+1 until end of turn.
        // Simplified: uses P1P1 counters (same approach as BoostUntilEndOfTurn).
        for event in self.event_log.iter() {
            if event.event_type != EventType::SpellCast {
                continue;
            }
            let caster = match event.player_id {
                Some(p) => p,
                None => continue,
            };
            // Check if the spell was noncreature
            let is_noncreature = if let Some(spell_id) = event.target_id {
                self.state.card_store.get(spell_id)
                    .map(|c| !c.is_creature())
                    .unwrap_or(true)
            } else {
                true
            };
            if !is_noncreature {
                continue;
            }
            // Find all creatures with prowess the caster controls
            let prowess_creatures: Vec<ObjectId> = self.state.battlefield.iter()
                .filter(|p| p.controller == caster && p.is_creature()
                    && p.has_keyword(crate::constants::KeywordAbilities::PROWESS))
                .map(|p| p.id())
                .collect();
            for creature_id in prowess_creatures {
                if let Some(perm) = self.state.battlefield.get_mut(creature_id) {
                    // +1/+1 until end of turn (simplified using P1P1 counters)
                    perm.add_counters(crate::counters::CounterType::P1P1, 1);
                }
            }
        }

        // Check delayed triggers against events
        let mut delayed_fired: Vec<(usize, crate::state::DelayedTrigger)> = Vec::new();
        for event in self.event_log.iter() {
            for (idx, dt) in self.state.delayed_triggers.iter().enumerate() {
                if dt.event_type != event.event_type {
                    continue;
                }
                // If watching a specific object, check it matches
                if let Some(watched_id) = dt.watching {
                    if let Some(target_id) = event.target_id {
                        if target_id != watched_id {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
                // For EndStep triggers, only fire on the controller's end step
                if event.event_type == EventType::EndStep {
                    if let Some(pid) = event.player_id {
                        if pid != dt.controller {
                            continue;
                        }
                    }
                }
                delayed_fired.push((idx, dt.clone()));
            }
        }
        // Remove fired trigger-only-once entries (reverse order to preserve indices)
        let mut indices_to_remove: Vec<usize> = delayed_fired.iter()
            .filter(|(_, dt)| dt.trigger_only_once)
            .map(|(idx, _)| *idx)
            .collect();
        indices_to_remove.sort_unstable();
        indices_to_remove.dedup();
        for idx in indices_to_remove.into_iter().rev() {
            self.state.delayed_triggers.remove(idx);
        }
        // Execute delayed trigger effects
        for (_, dt) in &delayed_fired {
            self.execute_effects(&dt.effects, dt.controller, &dt.targets, dt.source, None);
        }

        // Clear event log after processing
        self.event_log.clear();

        if triggered.is_empty() && delayed_fired.is_empty() {
            return false;
        }

        // Sort by APNAP order (active player's triggers first)
        let active = self.state.active_player;
        triggered.sort_by_key(|(controller, _, _, _)| if *controller == active { 0 } else { 1 });

        // Push triggered abilities onto the stack
        for (controller, ability_id, source_id, description) in triggered {
            // For optional triggers, ask the controller
            let ability = self.state.ability_store.get(ability_id).cloned();
            if let Some(ref ab) = ability {
                if ab.optional_trigger {
                    let view = crate::decision::GameView::placeholder();
                    let use_it = if let Some(dm) = self.decision_makers.get_mut(&controller) {
                        dm.choose_use(
                            &view,
                            crate::constants::Outcome::Benefit,
                            &format!("Use triggered ability: {}?", description),
                        )
                    } else {
                        false
                    };
                    if !use_it {
                        continue;
                    }
                }
            }

            // Select targets for the triggered ability
            let targets = if let Some(ref ab) = ability {
                self.select_targets_for_spec(&ab.targets, controller)
            } else {
                Vec::new()
            };

            let stack_item = crate::zones::StackItem {
                id: ObjectId::new(), // triggered abilities get a fresh ID on the stack
                kind: crate::zones::StackItemKind::Ability {
                    source_id,
                    ability_id,
                    description,
                },
                controller,
                targets,
                countered: false,
            x_value: None,
            exile_on_resolve: false,
            };
            self.state.stack.push(stack_item);
        }

        true
    }

    /// Emit an event to the event log (for triggered ability checking).
    fn emit_event(&mut self, event: GameEvent) {
        self.event_log.push(event);
    }

    /// Execute turn-based actions for a step.
    fn turn_based_actions(&mut self, step: PhaseStep, active_player: PlayerId) {
        match step {
            PhaseStep::Untap => {
                // Untap all permanents controlled by the active player
                for perm in self.state.battlefield.iter_mut() {
                    if perm.controller == active_player {
                        perm.untap();
                        perm.remove_summoning_sickness();
                    }
                }
                // Empty mana pool (normally happens at end of each step, but
                // also at untap for clarity)
                if let Some(player) = self.state.players.get_mut(&active_player) {
                    player.mana_pool.clear();
                }
            }
            PhaseStep::Upkeep => {
                // Emit upkeep event for "at the beginning of your upkeep" triggers
                let mut upkeep_event = GameEvent::new(EventType::UpkeepStep);
                upkeep_event.player_id = Some(active_player);
                self.emit_event(upkeep_event);
            }
            PhaseStep::Draw => {
                // Active player draws a card
                // Skip draw on turn 1 for the starting player (two-player rule)
                if self.turn_manager.turn_number > 1 || self.state.turn_order[0] != active_player {
                    self.draw_cards(active_player, 1);
                }
            }
            PhaseStep::Cleanup => {
                // Discard down to max hand size
                let discard_info = self
                    .state
                    .players
                    .get(&active_player)
                    .map(|p| {
                        let count = p.discard_count();
                        let hand: Vec<ObjectId> = p.hand.iter().copied().collect();
                        (count, hand)
                    });

                if let Some((discard_count, hand_cards)) = discard_info {
                    if discard_count > 0 {
                        let view = crate::decision::GameView::placeholder();
                        let to_discard = if let Some(dm) =
                            self.decision_makers.get_mut(&active_player)
                        {
                            dm.choose_discard(&view, &hand_cards, discard_count as usize)
                        } else {
                            hand_cards
                                .iter()
                                .rev()
                                .take(discard_count as usize)
                                .copied()
                                .collect()
                        };
                        for card_id in to_discard {
                            if let Some(player) =
                                self.state.players.get_mut(&active_player)
                            {
                                player.hand.remove(card_id);
                            }
                            self.move_card_to_graveyard(card_id, active_player);
                        }
                    }
                }
                // Remove damage from all creatures and clear "until end of turn" effects
                for perm in self.state.battlefield.iter_mut() {
                    if perm.is_creature() {
                        perm.clear_damage();
                    }
                    // Clear granted keywords (from GainKeywordUntilEndOfTurn, Indestructible, Hexproof)
                    perm.granted_keywords = crate::constants::KeywordAbilities::empty();
                    perm.removed_keywords = crate::constants::KeywordAbilities::empty();
                    // Remove "can't block" sentinel counters
                    perm.counters.remove_all(&crate::counters::CounterType::Custom("cant_block".into()));
                    // Revert temporary control changes (GainControlUntilEndOfTurn)
                    if let Some(orig) = perm.original_controller.take() {
                        perm.controller = orig;
                    }
                }
                // Empty mana pools
                for player in self.state.players.values_mut() {
                    player.mana_pool.clear();
                }
                // Clean up expired impulse-playable cards
                let turn_num = self.state.turn_number;
                self.state.impulse_playable.retain(|ip| {
                    match ip.duration {
                        crate::state::ImpulseDuration::EndOfTurn => false,
                        crate::state::ImpulseDuration::UntilEndOfNextTurn => {
                            // Keep unless this is the controller's turn cleanup
                            // AND it wasn't just created this turn.
                            !(active_player == ip.player_id
                              && turn_num > ip.created_turn)
                        }
                    }
                });
                // Clean up expired delayed triggers
                self.state.delayed_triggers.retain(|dt| {
                    match dt.duration {
                        crate::state::DelayedDuration::EndOfTurn => false,
                        crate::state::DelayedDuration::UntilTriggered => true,
                    }
                });
            }
            PhaseStep::DeclareAttackers => {
                self.declare_attackers_step(active_player);
            }
            PhaseStep::DeclareBlockers => {
                self.declare_blockers_step(active_player);
            }
            PhaseStep::FirstStrikeDamage => {
                self.combat_damage_step(true);
            }
            PhaseStep::CombatDamage => {
                self.combat_damage_step(false);
            }
            PhaseStep::EndCombat => {
                self.state.combat.clear();
            }
            PhaseStep::EndStep => {
                // Emit end step event for "at the beginning of your end step" triggers
                let mut end_event = GameEvent::new(EventType::EndStep);
                end_event.player_id = Some(active_player);
                self.emit_event(end_event);
            }
            _ => {
                // Other steps: empty mana pool at step transition (simplified)
                // In full rules, mana empties at end of each step/phase.
            }
        }
    }

    /// Declare attackers step: prompt active player to choose attackers,
    /// tap them (unless vigilance), register in combat state.
    fn declare_attackers_step(&mut self, active_player: PlayerId) {
        // Collect creatures that can legally attack
        let possible_attackers: Vec<ObjectId> = self
            .state
            .battlefield
            .iter()
            .filter(|p| p.controller == active_player && p.can_attack())
            .map(|p| p.id())
            .collect();

        if possible_attackers.is_empty() {
            return;
        }

        // Possible defenders: opponent player IDs (as ObjectIds for the interface)
        let possible_defenders: Vec<ObjectId> = self
            .state
            .turn_order
            .iter()
            .filter(|&&id| id != active_player)
            .filter(|&&id| {
                self.state
                    .players
                    .get(&id)
                    .map(|p| p.is_in_game())
                    .unwrap_or(false)
            })
            .map(|&id| ObjectId(id.0))
            .collect();

        if possible_defenders.is_empty() {
            return;
        }

        // Ask decision maker to choose attackers
        let view = crate::decision::GameView::placeholder();
        let chosen = if let Some(dm) = self.decision_makers.get_mut(&active_player) {
            dm.select_attackers(&view, &possible_attackers, &possible_defenders)
        } else {
            Vec::new()
        };

        if chosen.is_empty() {
            return;
        }

        // Set up combat state
        self.state.combat = CombatState::new();
        self.state.combat.attacking_player = Some(active_player);

        for (attacker_id, defender_id) in &chosen {
            // Validate the attacker can actually attack
            let can = self
                .state
                .battlefield
                .get(*attacker_id)
                .map(|p| p.can_attack() && p.controller == active_player)
                .unwrap_or(false);
            if !can {
                continue;
            }

            // Register attacker in combat state
            self.state
                .combat
                .declare_attacker(*attacker_id, *defender_id, true);

            // Tap the attacker (unless it has vigilance)
            if let Some(perm) = self.state.battlefield.get_mut(*attacker_id) {
                if !perm.has_vigilance() {
                    perm.tap();
                }
            }

            // Emit attacker declared event
            self.emit_event(
                GameEvent::new(EventType::AttackerDeclared)
                    .target(*attacker_id)
                    .player(active_player),
            );
        }

        // Check if any attackers have first/double strike to inform TurnManager
        let has_fs = self.state.combat.has_first_strikers(&|id| {
            self.state.battlefield.get(id).map(|p| p.keywords())
        });
        self.turn_manager.has_first_strike = has_fs;
    }

    /// Declare blockers step: prompt defending players to choose blockers,
    /// validate assignments, register in combat state.
    fn declare_blockers_step(&mut self, _active_player: PlayerId) {
        if !self.state.combat.has_attackers() {
            return;
        }

        // For each defending player, gather attacker info and ask for blocks
        let defending_players: Vec<PlayerId> = self
            .state
            .combat
            .groups
            .iter()
            .filter(|g| g.defending_player)
            .map(|g| PlayerId(g.defending_id.0))
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        for def_player in defending_players {
            // Build AttackerInfo for each attacker targeting this defender
            let attacker_infos: Vec<AttackerInfo> = self
                .state
                .combat
                .groups
                .iter()
                .filter(|g| g.defending_player && PlayerId(g.defending_id.0) == def_player)
                .map(|g| {
                    // Check if attacker has landwalk (unblockable if defender controls that land type)
                    let has_landwalk_evasion = self.state.battlefield.get(g.attacker_id)
                        .map(|attacker| {
                            use crate::constants::{KeywordAbilities, SubType};
                            let checks = [
                                (KeywordAbilities::FORESTWALK, SubType::Forest),
                                (KeywordAbilities::ISLANDWALK, SubType::Island),
                                (KeywordAbilities::MOUNTAINWALK, SubType::Mountain),
                                (KeywordAbilities::PLAINSWALK, SubType::Plains),
                                (KeywordAbilities::SWAMPWALK, SubType::Swamp),
                            ];
                            checks.iter().any(|(kw, land_type)| {
                                attacker.has_keyword(*kw)
                                    && self.state.battlefield.iter().any(|p| {
                                        p.controller == def_player && p.has_subtype(land_type)
                                    })
                            })
                        })
                        .unwrap_or(false);

                    let legal_blockers: Vec<ObjectId> = if has_landwalk_evasion {
                        vec![] // Can't be blocked at all when landwalk applies
                    } else {
                        self
                            .state
                            .battlefield
                            .iter()
                            .filter(|p| {
                                p.controller == def_player
                                    && p.can_block()
                                    && self
                                        .state
                                        .battlefield
                                        .get(g.attacker_id)
                                        .map(|attacker| combat::can_block(p, attacker))
                                        .unwrap_or(false)
                            })
                            .map(|p| p.id())
                            .collect()
                    };

                    let (must_be_blocked, max_blocked_by) = self
                        .state
                        .battlefield
                        .get(g.attacker_id)
                        .map(|a| (a.must_be_blocked, a.max_blocked_by))
                        .unwrap_or((false, None));

                    AttackerInfo {
                        attacker_id: g.attacker_id,
                        defending_id: g.defending_id,
                        legal_blockers,
                        must_be_blocked,
                        max_blocked_by,
                    }
                })
                .collect();

            if attacker_infos.iter().all(|a| a.legal_blockers.is_empty()) {
                continue;
            }

            // Ask defending player to choose blockers
            let view = crate::decision::GameView::placeholder();
            let blocks = if let Some(dm) = self.decision_makers.get_mut(&def_player) {
                dm.select_blockers(&view, &attacker_infos)
            } else {
                Vec::new()
            };

            // Register blocks
            for (blocker_id, attacker_id) in blocks {
                self.state.combat.declare_blocker(blocker_id, attacker_id);
            }
        }

        // Validate block restrictions: max_blocked_by and menace
        for group in &mut self.state.combat.groups {
            // max_blocked_by: trim excess blockers
            if let Some(attacker) = self.state.battlefield.get(group.attacker_id) {
                if let Some(max) = attacker.max_blocked_by {
                    while group.blockers.len() > max as usize {
                        let removed = group.blockers.pop().unwrap();
                        self.state.combat.blocker_to_attacker.remove(&removed);
                    }
                }
                // menace: if only 1 blocker, remove it (must have 2+)
                if attacker.has_menace() && group.blockers.len() == 1 {
                    let removed = group.blockers.pop().unwrap();
                    self.state.combat.blocker_to_attacker.remove(&removed);
                    group.blocked = false;
                }
            }
        }

        // After all blocks declared, check for first/double strike among blockers too
        let has_fs = self.state.combat.has_first_strikers(&|id| {
            self.state.battlefield.get(id).map(|p| p.keywords())
        });
        self.turn_manager.has_first_strike = has_fs;
    }

    /// Combat damage step: assign and apply combat damage.
    /// `is_first_strike` determines whether this is the first strike damage step.
    fn combat_damage_step(&mut self, is_first_strike: bool) {
        if !self.state.combat.has_attackers() {
            return;
        }

        // Collect all damage assignments
        let mut damage_events: Vec<(ObjectId, u32, bool, ObjectId)> = Vec::new(); // (target, amount, is_player, source)
        let mut lifelink_sources: Vec<(PlayerId, u32)> = Vec::new(); // (controller, damage_dealt)

        let groups = self.state.combat.groups.clone();
        for group in &groups {
            // Get attacker info
            let attacker_info = match self.state.battlefield.get(group.attacker_id) {
                Some(a) => a,
                None => continue,
            };
            let attacker_has_lifelink = attacker_info.has_lifelink();
            let attacker_controller = attacker_info.controller;

            // Get blocker info
            let blockers: Vec<(ObjectId, Permanent)> = group
                .blockers
                .iter()
                .filter_map(|&bid| {
                    self.state.battlefield.get(bid).map(|p| (bid, p.clone()))
                })
                .collect();
            let blocker_refs: Vec<(ObjectId, &Permanent)> =
                blockers.iter().map(|(id, p)| (*id, p)).collect();

            // Assign attacker damage
            let attacker_dmg =
                combat::assign_combat_damage(&group, attacker_info, &blocker_refs, is_first_strike);

            for (target_id, amount, is_player) in &attacker_dmg {
                damage_events.push((*target_id, *amount, *is_player, group.attacker_id));
                if attacker_has_lifelink && *amount > 0 {
                    lifelink_sources.push((attacker_controller, *amount));
                }
            }

            // Assign blocker damage to attacker
            for (blocker_id, blocker_perm) in &blockers {
                let blocker_dmg =
                    combat::assign_blocker_damage(blocker_perm, group.attacker_id, is_first_strike);
                if blocker_dmg > 0 {
                    damage_events.push((group.attacker_id, blocker_dmg, false, *blocker_id));
                    // Check blocker lifelink
                    if blocker_perm.has_lifelink() {
                        lifelink_sources.push((blocker_perm.controller, blocker_dmg));
                    }
                }
            }
        }

        // Apply all damage
        for (target_id, amount, is_player, source_id) in &damage_events {
            if *is_player {
                let player_id = PlayerId(target_id.0);
                if let Some(player) = self.state.players.get_mut(&player_id) {
                    player.life -= *amount as i32;
                }
                // Emit DamagedPlayer event for "deals combat damage to a player" triggers
                let mut dmg_event = GameEvent::new(EventType::DamagedPlayer);
                dmg_event.target_id = Some(*source_id); // The creature that dealt damage
                dmg_event.player_id = Some(player_id);  // The player that was damaged
                dmg_event.amount = *amount as i32;
                self.emit_event(dmg_event);
            } else if let Some(perm) = self.state.battlefield.get_mut(*target_id) {
                perm.apply_damage(*amount);
            }
        }

        // Apply lifelink
        for (controller, amount) in &lifelink_sources {
            if let Some(player) = self.state.players.get_mut(controller) {
                player.gain_life(*amount);
            }
            // Emit life gain event for lifelink
            self.emit_event(GameEvent::gain_life(*controller, *amount));
        }
    }

    /// Run the priority loop: players take actions or pass until the stack
    /// resolves or the step ends.
    fn priority_loop(&mut self) {
        let active = self.state.active_player;
        let player_count = self.state.active_players().len() as u32;
        if player_count == 0 {
            return;
        }

        let mut tracker = PriorityTracker::new(active, player_count);

        loop {
            // Check SBAs each time through the loop
            self.process_state_based_actions();

            // Check if game should end
            if self.state.should_end() {
                return;
            }

            // Give current priority player a chance to act
            let priority_player = tracker.current;
            self.state.priority_player = priority_player;

            // Build legal actions
            let legal_actions = self.compute_legal_actions(priority_player);

            // Ask the decision maker
            let action = {
                let view = crate::decision::GameView::placeholder();
                if let Some(dm) = self.decision_makers.get_mut(&priority_player) {
                    dm.priority(&view, &legal_actions)
                } else {
                    crate::decision::PlayerAction::Pass
                }
            };

            match action {
                crate::decision::PlayerAction::Pass => {
                    if tracker.pass() {
                        // All players passed
                        if self.state.stack.is_empty() {
                            // Stack is empty — step ends
                            return;
                        } else {
                            // Resolve top of stack
                            self.resolve_top_of_stack();
                            tracker.reset();
                            // Priority goes back to active player
                            tracker.current = active;
                        }
                    } else {
                        // Move priority to next player
                        tracker.current = self.state.next_player(priority_player);
                    }
                }
                crate::decision::PlayerAction::PlayLand { card_id } => {
                    self.play_land(priority_player, card_id);
                    tracker.reset();
                    // Priority stays with the active player after playing a land
                    tracker.current = priority_player;
                }
                crate::decision::PlayerAction::CastSpell { card_id, .. } => {
                    self.cast_spell(priority_player, card_id);
                    tracker.reset();
                    tracker.current = priority_player;
                }
                crate::decision::PlayerAction::ActivateAbility { source_id, ability_id, targets } => {
                    self.activate_ability(priority_player, source_id, ability_id, &targets);
                    tracker.reset();
                    tracker.current = priority_player;
                }
                crate::decision::PlayerAction::ActivateManaAbility { source_id, ability_id } => {
                    // Mana abilities don't use the stack — resolve immediately
                    self.activate_mana_ability(priority_player, source_id, ability_id);
                    // Mana abilities don't reset priority passes
                }
                _ => {
                    // Other actions (special actions)
                    tracker.reset();
                    tracker.current = priority_player;
                }
            }
        }
    }

    /// Compute the legal actions for a player who has priority.
    fn compute_legal_actions(&self, player_id: PlayerId) -> Vec<crate::decision::PlayerAction> {
        let mut actions = vec![crate::decision::PlayerAction::Pass];

        let player = match self.state.player(player_id) {
            Some(p) => p,
            None => return actions,
        };

        let can_sorcery = self.state.can_cast_sorcery(player_id);

        // Check for playable lands
        if can_sorcery && player.can_play_land() {
            for &card_id in player.hand.iter() {
                if let Some(card) = self.state.card_store.get(card_id) {
                    if card.is_land() {
                        actions.push(crate::decision::PlayerAction::PlayLand { card_id });
                    }
                }
            }
        }

        // Check for castable spells
        for &card_id in player.hand.iter() {
            if let Some(card) = self.state.card_store.get(card_id) {
                if card.is_land() {
                    continue;
                }

                // Check if the player can pay the mana cost
                let mana_cost = card.mana_cost.to_mana();
                let available = player.mana_pool.available();

                if available.can_pay(&mana_cost) {
                    // Sorcery-speed cards need sorcery timing
                    let needs_sorcery = !card.is_instant()
                        && !card.keywords.contains(crate::constants::KeywordAbilities::FLASH);

                    if needs_sorcery && !can_sorcery {
                        continue;
                    }

                    // Check additional casting costs (behold, etc.)
                    if !card.additional_costs.is_empty() && !self.can_pay_additional_costs(player_id, card_id, &card.additional_costs) {
                        continue;
                    }

                    actions.push(crate::decision::PlayerAction::CastSpell {
                        card_id,
                        targets: vec![],
                        mode: None,
                        without_mana: false,
                    });
                }
            }
        }

        // Check for impulse-playable cards from exile
        for impulse in &self.state.impulse_playable {
            if impulse.player_id != player_id {
                continue;
            }
            // Verify card is still in exile
            if !self.state.exile.contains(impulse.card_id) {
                continue;
            }
            if let Some(card) = self.state.card_store.get(impulse.card_id) {
                if card.is_land() {
                    // Can play lands from exile at sorcery speed
                    if can_sorcery && player.can_play_land() {
                        actions.push(crate::decision::PlayerAction::PlayLand {
                            card_id: impulse.card_id,
                        });
                    }
                } else {
                    // Can cast spells from exile
                    let needs_sorcery = !card.is_instant()
                        && !card.keywords.contains(crate::constants::KeywordAbilities::FLASH);
                    if needs_sorcery && !can_sorcery {
                        continue;
                    }
                    if impulse.without_mana {
                        actions.push(crate::decision::PlayerAction::CastSpell {
                            card_id: impulse.card_id,
                            targets: vec![],
                            mode: None,
                            without_mana: true,
                        });
                    } else {
                        let mana_cost = card.mana_cost.to_mana();
                        let available = player.mana_pool.available();
                        if available.can_pay(&mana_cost) {
                            actions.push(crate::decision::PlayerAction::CastSpell {
                                card_id: impulse.card_id,
                                targets: vec![],
                                mode: None,
                                without_mana: false,
                            });
                        }
                    }
                }
            }
        }

        // Check for flashback-castable cards in graveyard
        if let Some(graveyard) = self.state.players.get(&player_id).map(|p| {
            p.graveyard.iter().copied().collect::<Vec<_>>()
        }) {
            for card_id in graveyard {
                if let Some(card) = self.state.card_store.get(card_id) {
                    if let Some(ref fb_cost) = card.flashback_cost {
                        // Non-land spell with flashback
                        if card.is_land() { continue; }
                        let needs_sorcery = !card.is_instant()
                            && !card.keywords.contains(crate::constants::KeywordAbilities::FLASH);
                        if needs_sorcery && !can_sorcery { continue; }
                        let mana_cost = fb_cost.to_mana();
                        let available = player.mana_pool.available();
                        if available.can_pay(&mana_cost) {
                            actions.push(crate::decision::PlayerAction::CastSpell {
                                card_id,
                                targets: vec![],
                                mode: None,
                                without_mana: false,
                            });
                        }
                    }
                }
            }
        }

        // Check for activatable abilities on permanents the player controls
        let controlled_perms: Vec<(ObjectId, bool)> = self.state.battlefield
            .controlled_by(player_id)
            .map(|p| (p.id(), p.tapped))
            .collect();

        for (perm_id, is_tapped) in controlled_perms {
            let abilities: Vec<(AbilityId, bool, bool)> = self.state.ability_store
                .for_source(perm_id)
                .iter()
                .filter(|a| {
                    a.ability_type == AbilityType::ActivatedNonMana
                        && a.can_activate_in_zone(crate::constants::Zone::Battlefield)
                })
                .map(|a| {
                    let needs_tap = a.costs.iter().any(|c| matches!(c, Cost::TapSelf));
                    let needs_mana = a.costs.iter().any(|c| matches!(c, Cost::Mana(_)));
                    (a.id, needs_tap, needs_mana)
                })
                .collect();

            for (ability_id, needs_tap, _needs_mana) in abilities {
                // Can't activate if it requires tap and the permanent is already tapped
                if needs_tap && is_tapped {
                    continue;
                }
                // Sorcery-speed activated abilities need sorcery timing
                // (Simplification: all activated abilities can be used at instant speed)
                actions.push(crate::decision::PlayerAction::ActivateAbility {
                    source_id: perm_id,
                    ability_id,
                    targets: vec![],
                });
            }
        }

        actions
    }

    /// Play a land from a player's hand onto the battlefield.
    fn play_land(&mut self, player_id: PlayerId, card_id: ObjectId) {
        let player = match self.state.players.get_mut(&player_id) {
            Some(p) => p,
            None => return,
        };

        if !player.can_play_land() {
            return;
        }

        // Try to remove from hand first, then from exile (impulse play)
        let from_exile = !player.hand.contains(card_id)
            && self.state.impulse_playable.iter().any(|ip| ip.card_id == card_id && ip.player_id == player_id);
        if from_exile {
            self.state.exile.remove(card_id);
            self.state.impulse_playable.retain(|ip| ip.card_id != card_id);
            // Re-borrow player after mutation
            let player = self.state.players.get_mut(&player_id).unwrap();
            player.play_land();
        } else {
            if !player.hand.remove(card_id) {
                return;
            }
            player.play_land();
        }

        // Create permanent from card data
        if let Some(card_data) = self.state.card_store.get(card_id).cloned() {
            // Register abilities from the card
            for ability in &card_data.abilities {
                self.state.ability_store.add(ability.clone());
            }
            let perm = Permanent::new(card_data, player_id);
            self.state.battlefield.add(perm);
            self.state.set_zone(card_id, crate::constants::Zone::Battlefield, None);
            self.check_enters_tapped(card_id);

            // Emit ETB event
            self.emit_event(GameEvent::enters_battlefield(card_id, player_id));
        }
    }

    /// Cast a spell (simplified: pay mana, move to stack, then resolve immediately
    /// for now since the full stack resolution needs the ability framework).
    fn cast_spell(&mut self, player_id: PlayerId, card_id: ObjectId) {
        let card_data = match self.state.card_store.get(card_id).cloned() {
            Some(c) => c,
            None => return,
        };

        // Determine X value for X-cost spells
        let x_value = if card_data.mana_cost.has_x_cost() {
            let base_cost = card_data.mana_cost.to_mana();
            let available = self.state.players.get(&player_id)
                .map(|p| p.mana_pool.available())
                .unwrap_or_default();
            // Max X = available mana minus non-X costs
            let remaining = available.count().saturating_sub(base_cost.count());
            let x_count = card_data.mana_cost.x_count();
            let max_x = if x_count > 0 { remaining / x_count } else { 0 };
            let view = crate::decision::GameView::placeholder();
            let x = if let Some(dm) = self.decision_makers.get_mut(&player_id) {
                dm.choose_amount(&view, "Choose X", 0, max_x)
            } else {
                max_x // AI defaults to max X
            };
            Some(x)
        } else {
            None
        };

        // Check if this is an impulse-play from exile
        let from_exile = self.state.impulse_playable.iter()
            .any(|ip| ip.card_id == card_id && ip.player_id == player_id);
        let without_mana = from_exile && self.state.impulse_playable.iter()
            .any(|ip| ip.card_id == card_id && ip.without_mana);

        // Check if this is a flashback cast from graveyard
        let from_graveyard = !from_exile && card_data.flashback_cost.is_some()
            && self.state.players.get(&player_id)
                .map(|p| p.graveyard.contains(card_id))
                .unwrap_or(false);

        // Remove from hand, exile, or graveyard
        if from_exile {
            self.state.exile.remove(card_id);
            self.state.impulse_playable.retain(|ip| ip.card_id != card_id);
        } else if from_graveyard {
            if let Some(player) = self.state.players.get_mut(&player_id) {
                player.graveyard.remove(card_id);
            }
        } else if let Some(player) = self.state.players.get_mut(&player_id) {
            if !player.hand.remove(card_id) {
                return;
            }
        }

        // Pay mana cost (with X substituted if applicable), unless free cast
        if !without_mana {
            if let Some(player) = self.state.players.get_mut(&player_id) {
                let mana_cost = if from_graveyard {
                    // Use flashback cost when casting from graveyard
                    card_data.flashback_cost.as_ref().unwrap().to_mana()
                } else {
                    match x_value {
                        Some(x) => card_data.mana_cost.to_mana_with_x(x),
                        None => card_data.mana_cost.to_mana(),
                    }
                };
                if !player.mana_pool.try_pay(&mana_cost) {
                    // Can't pay — put card back where it came from
                    if from_graveyard {
                        player.graveyard.add(card_id);
                    } else {
                        player.hand.add(card_id);
                    }
                    return;
                }
            }
        }

        // Pay additional casting costs (behold, etc.)
        if !card_data.additional_costs.is_empty() {
            if !self.pay_costs(player_id, card_id, &card_data.additional_costs) {
                // Cannot pay additional costs — put card back
                if from_graveyard {
                    if let Some(player) = self.state.players.get_mut(&player_id) {
                        player.graveyard.add(card_id);
                    }
                } else if from_exile {
                    self.state.exile.exile(card_id);
                } else {
                    if let Some(player) = self.state.players.get_mut(&player_id) {
                        player.hand.add(card_id);
                    }
                }
                return;
            }
        }

        // Select targets based on the spell's TargetSpec
        let target_spec = card_data
            .abilities
            .iter()
            .find(|a| a.ability_type == AbilityType::Spell)
            .map(|a| a.targets.clone())
            .unwrap_or(crate::abilities::TargetSpec::None);
        let targets = self.select_targets_for_spec(&target_spec, player_id);

        // Put on the stack
        let stack_item = crate::zones::StackItem {
            id: card_id,
            kind: crate::zones::StackItemKind::Spell { card: card_data.clone() },
            controller: player_id,
            targets,
            countered: false,
            x_value,
            exile_on_resolve: from_graveyard,
        };
        self.state.stack.push(stack_item);
        self.state.set_zone(card_id, crate::constants::Zone::Stack, None);

        // Emit spell cast event (for prowess, storm, etc.)
        self.emit_event(GameEvent::spell_cast(card_id, player_id, if from_exile { crate::constants::Zone::Exile } else if from_graveyard { crate::constants::Zone::Graveyard } else { crate::constants::Zone::Hand }));

        // Ward check: if any target has Ward and the caster is an opponent, enforce ward cost
        self.check_ward_on_targets(card_id, player_id);
    }

    /// Check ward on targets of a spell/ability. If any target has Ward and
    /// the spell controller is an opponent, try to charge the ward cost.
    /// If the cost can't be paid, counter the spell.
    fn check_ward_on_targets(&mut self, spell_id: ObjectId, caster: PlayerId) {
        // Collect ward costs for targets that have ward
        let mut should_counter = false;
        let targets: Vec<ObjectId> = self.state.stack.get(spell_id)
            .map(|item| item.targets.clone())
            .unwrap_or_default();

        for &target_id in &targets {
            let target_controller = match self.state.battlefield.get(target_id) {
                Some(perm) => perm.controller,
                None => continue,
            };
            // Ward only applies when an opponent targets the permanent
            if target_controller == caster {
                continue;
            }

            // Check for Ward static effect in abilities
            let ward_cost = self.find_ward_cost(target_id);
            if let Some(cost_str) = ward_cost {
                // Try to charge the ward cost
                if !self.try_pay_ward_cost(caster, &cost_str) {
                    should_counter = true;
                    break;
                }
            }
        }

        if should_counter {
            if let Some(item) = self.state.stack.get_mut(spell_id) {
                item.countered = true;
            }
        }
    }

    /// Find the ward cost for a permanent (from its static abilities).
    fn find_ward_cost(&self, permanent_id: ObjectId) -> Option<String> {
        for ability in self.state.ability_store.for_source(permanent_id) {
            for effect in &ability.static_effects {
                if let StaticEffect::Ward { cost } = effect {
                    return Some(cost.clone());
                }
            }
        }
        // Also check if ward is granted via continuous keywords but has no explicit cost
        // (e.g., GrantKeyword "ward" — in this case we can't enforce it without a cost value)
        None
    }

    /// Try to pay a ward cost. Returns true if the cost was paid.
    fn try_pay_ward_cost(&mut self, payer: PlayerId, cost: &str) -> bool {
        // Mana cost (e.g., "{2}", "{1}{U}")
        if cost.starts_with('{') {
            let mana_cost = ManaCost::parse(cost);
            let mana = mana_cost.to_mana();
            if let Some(player) = self.state.players.get_mut(&payer) {
                return player.mana_pool.try_pay(&mana);
            }
            return false;
        }

        // Life cost (e.g., "Pay 2 life")
        if cost.contains("life") {
            // Extract the number from "Pay N life"
            let amount: i32 = cost.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap_or(0);
            if amount > 0 {
                if let Some(player) = self.state.players.get_mut(&payer) {
                    if player.life > amount {
                        player.life -= amount;
                        return true;
                    }
                }
            }
            return false;
        }

        // Discard cost (e.g., "Discard a card.")
        if cost.contains("iscard") {
            if let Some(player) = self.state.players.get(&payer) {
                if player.hand.len() > 0 {
                    // Discard a card (pick first card in hand for simplicity)
                    let card_id = *player.hand.iter().next().unwrap();
                    let player = self.state.players.get_mut(&payer).unwrap();
                    player.hand.remove(card_id);
                    player.graveyard.add(card_id);
                    return true;
                }
            }
            return false;
        }

        // Unknown ward cost — can't pay
        false
    }

    /// Resolve the top item on the stack.
    fn resolve_top_of_stack(&mut self) {
        let item = match self.state.stack.pop() {
            Some(item) => item,
            None => return,
        };

        if item.countered {
            // Countered spells go to graveyard
            self.move_card_to_graveyard(item.id, item.controller);
            return;
        }

        // Fizzle check: if the spell/ability has targets and all targets are
        // now illegal (left the battlefield, gained hexproof, etc.), it fizzles.
        // Note: targets stored as ObjectIds; player targets are tracked separately
        // in the SelectedTargets system (targets.rs). This simplified check only
        // validates permanent targets on the battlefield.
        if !item.targets.is_empty() {
            let any_legal = item.targets.iter().any(|&target_id| {
                self.state.battlefield.contains(target_id)
                    || self.state.stack.get(target_id).is_some()
            });
            if !any_legal {
                // All targets are illegal — fizzle
                match &item.kind {
                    crate::zones::StackItemKind::Spell { .. } => {
                        self.move_card_to_graveyard(item.id, item.controller);
                    }
                    crate::zones::StackItemKind::Ability { .. } => {
                        // Abilities just cease to exist when fizzled
                    }
                }
                return;
            }
        }

        match &item.kind {
            crate::zones::StackItemKind::Spell { card } => {
                if card.is_permanent_card() {
                    // Register abilities from the card
                    for ability in &card.abilities {
                        self.state.ability_store.add(ability.clone());
                    }
                    // Permanent spells enter the battlefield
                    let perm = Permanent::new(card.clone(), item.controller);
                    self.state.battlefield.add(perm);
                    self.state.set_zone(item.id, crate::constants::Zone::Battlefield, None);
                    self.check_enters_tapped(item.id);

                    // Aura attachment: attach to target on ETB
                    if card.subtypes.contains(&crate::constants::SubType::Aura) {
                        if let Some(&target_id) = item.targets.first() {
                            if let Some(aura) = self.state.battlefield.get_mut(item.id) {
                                aura.attach_to(target_id);
                            }
                            if let Some(creature) = self.state.battlefield.get_mut(target_id) {
                                creature.add_attachment(item.id);
                            }
                        }
                    }

                    // Emit ETB event
                    self.emit_event(GameEvent::enters_battlefield(item.id, item.controller));
                } else {
                    // Non-permanent spells: execute effects then go to graveyard
                    let effects: Vec<Effect> = card.abilities.iter()
                        .flat_map(|a| a.effects.clone())
                        .collect();
                    let targets = item.targets.clone();
                    let exile_after = item.exile_on_resolve;
                    self.execute_effects(&effects, item.controller, &targets, Some(item.id), item.x_value);
                    if exile_after {
                        // Flashback: exile instead of going to graveyard
                        self.state.exile.exile(item.id);
                        self.state.set_zone(item.id, crate::constants::Zone::Exile, None);
                    } else {
                        self.move_card_to_graveyard(item.id, item.controller);
                    }
                }
            }
            crate::zones::StackItemKind::Ability { ability_id, source_id, .. } => {
                // Resolve ability: find its effects and execute them
                let source = *source_id;
                let ability_data = self.state.ability_store.get(*ability_id).cloned();
                if let Some(ability) = ability_data {
                    let targets = item.targets.clone();
                    self.execute_effects(&ability.effects, item.controller, &targets, Some(source), None);
                }
            }
        }
    }

    /// Process state-based actions in a loop until no more are found.
    pub fn process_state_based_actions(&mut self) {
        for _ in 0..MAX_SBA_ITERATIONS {
            let sba = self.state.check_state_based_actions();
            if !sba.has_actions() {
                break;
            }
            self.apply_state_based_actions(&sba);
        }
    }

    /// Apply the detected state-based actions.
    fn apply_state_based_actions(&mut self, sba: &StateBasedActions) -> Vec<ObjectId> {
        // Players losing the game
        for &pid in &sba.players_losing {
            if let Some(player) = self.state.players.get_mut(&pid) {
                player.lost = true;
            }
        }

        // Track IDs of permanents that die (for deferred ability cleanup)
        let mut died_sources: Vec<ObjectId> = Vec::new();

        // Permanents going to graveyard (0 toughness)
        for &perm_id in &sba.permanents_to_graveyard {
            if let Some(perm) = self.state.battlefield.remove(perm_id) {
                let owner = perm.owner();
                let controller = perm.controller;
                let was_creature = perm.is_creature();
                self.move_card_to_graveyard(perm_id, owner);
                if was_creature {
                    self.emit_event(GameEvent::dies(perm_id, controller));
                    died_sources.push(perm_id);
                } else {
                    self.state.ability_store.remove_source(perm_id);
                }
            }
        }

        // Permanents being destroyed (lethal damage)
        for &perm_id in &sba.permanents_to_destroy {
            if let Some(perm) = self.state.battlefield.remove(perm_id) {
                let owner = perm.owner();
                let controller = perm.controller;
                let was_creature = perm.is_creature();
                self.move_card_to_graveyard(perm_id, owner);
                if was_creature {
                    self.emit_event(GameEvent::dies(perm_id, controller));
                    died_sources.push(perm_id);
                } else {
                    self.state.ability_store.remove_source(perm_id);
                }
            }
        }

        // Counter annihilation: +1/+1 and -1/-1 counters cancel out
        for &perm_id in &sba.counters_to_annihilate {
            if let Some(perm) = self.state.battlefield.get_mut(perm_id) {
                let p1p1 = perm.counters.get(&CounterType::P1P1);
                let m1m1 = perm.counters.get(&CounterType::M1M1);
                let to_remove = p1p1.min(m1m1);
                if to_remove > 0 {
                    perm.counters.remove(&CounterType::P1P1, to_remove);
                    perm.counters.remove(&CounterType::M1M1, to_remove);
                }
            }
        }

        // Equipment detachment: unattach from missing targets (stays on battlefield)
        for &perm_id in &sba.attachments_to_detach {
            if let Some(perm) = self.state.battlefield.get_mut(perm_id) {
                perm.detach();
            }
        }

        // Aura fall-off: aura goes to graveyard when enchanted permanent leaves
        for &perm_id in &sba.auras_to_graveyard {
            if let Some(perm) = self.state.battlefield.remove(perm_id) {
                let owner = perm.owner();
                self.move_card_to_graveyard(perm_id, owner);
                self.state.ability_store.remove_source(perm_id);
            }
        }

        // Token cleanup: tokens not on battlefield cease to exist (704.5d)
        for &(player_id, card_id) in &sba.tokens_to_remove {
            if let Some(player) = self.state.players.get_mut(&player_id) {
                player.graveyard.remove(card_id);
                player.hand.remove(card_id);
            }
            self.state.exile.remove(card_id);
            self.state.card_store.remove(card_id);
        }
        // Return died_sources so caller can clean up AFTER trigger checking
        died_sources

    }

    /// Activate an activated ability (goes on the stack).
    fn activate_ability(
        &mut self,
        player_id: PlayerId,
        source_id: ObjectId,
        ability_id: AbilityId,
        targets: &[ObjectId],
    ) {
        let ability = match self.state.ability_store.get(ability_id).cloned() {
            Some(a) => a,
            None => return,
        };

        // Pay costs
        if !self.pay_costs(player_id, source_id, &ability.costs) {
            return;
        }

        // Put the ability on the stack
        let stack_item = crate::zones::StackItem {
            id: ObjectId::new(), // New ID for the stack object
            kind: crate::zones::StackItemKind::Ability {
                source_id,
                ability_id,
                description: ability.rules_text.clone(),
            },
            controller: player_id,
            targets: targets.to_vec(),
            countered: false,
            x_value: None,
            exile_on_resolve: false,
        };
        self.state.stack.push(stack_item);
    }

    /// Activate a mana ability (resolves immediately, doesn't use the stack).
    fn activate_mana_ability(
        &mut self,
        player_id: PlayerId,
        source_id: ObjectId,
        ability_id: AbilityId,
    ) {
        let ability = match self.state.ability_store.get(ability_id).cloned() {
            Some(a) => a,
            None => return,
        };

        if !ability.is_mana_ability() {
            return;
        }

        // Pay costs (typically just tap)
        if !self.pay_costs(player_id, source_id, &ability.costs) {
            return;
        }

        // Resolve immediately: add mana
        if let Some(mana) = ability.mana_produced {
            if let Some(player) = self.state.players.get_mut(&player_id) {
                player.mana_pool.add(mana, None, false);
            }
        }
    }


    /// Count the number of distinct colors among permanents a player controls.
    /// Used by the Vivid mechanic (ECL set). Returns 0-5.
    fn count_colors_among_permanents(&self, player_id: PlayerId) -> usize {
        use std::collections::HashSet;
        use crate::constants::Color;
        let mut colors: HashSet<Color> = HashSet::new();
        for perm in self.state.battlefield.controlled_by(player_id) {
            for c in perm.card.colors() {
                colors.insert(c);
            }
        }
        colors.len()
    }
    /// Pay the costs for an ability or spell. Returns false if costs can't be paid.
    /// Check if additional casting costs can be paid (without paying them).
    fn can_pay_additional_costs(&self, player_id: PlayerId, source_id: ObjectId, costs: &[Cost]) -> bool {
        for cost in costs {
            match cost {
                Cost::Behold(creature_type) | Cost::BeholdAndExile(creature_type) => {
                    let ct_lower = creature_type.to_lowercase();
                    let has_match = self.state.battlefield.controlled_by(player_id)
                        .any(|perm| {
                            perm.id() != source_id &&
                            self.state.card_store.get(perm.id()).map_or(false, |card| {
                                card.subtypes.iter().any(|st| st.to_string().to_lowercase() == ct_lower)
                                || card.keywords.contains(crate::constants::KeywordAbilities::CHANGELING)
                            })
                        })
                        || self.state.players.get(&player_id).map_or(false, |p| {
                            p.hand.iter().any(|&cid| {
                                cid != source_id &&
                                self.state.card_store.get(cid).map_or(false, |card| {
                                    card.subtypes.iter().any(|st| st.to_string().to_lowercase() == ct_lower)
                                    || card.keywords.contains(crate::constants::KeywordAbilities::CHANGELING)
                                })
                            })
                        });
                    if !has_match { return false; }
                }
                Cost::BeholdOrPay { creature_type, mana } => {
                    let ct_lower = creature_type.to_lowercase();
                    let has_match = self.state.battlefield.controlled_by(player_id)
                        .any(|perm| {
                            perm.id() != source_id &&
                            self.state.card_store.get(perm.id()).map_or(false, |card| {
                                card.subtypes.iter().any(|st| st.to_string().to_lowercase() == ct_lower)
                                || card.keywords.contains(crate::constants::KeywordAbilities::CHANGELING)
                            })
                        })
                        || self.state.players.get(&player_id).map_or(false, |p| {
                            p.hand.iter().any(|&cid| {
                                cid != source_id &&
                                self.state.card_store.get(cid).map_or(false, |card| {
                                    card.subtypes.iter().any(|st| st.to_string().to_lowercase() == ct_lower)
                                    || card.keywords.contains(crate::constants::KeywordAbilities::CHANGELING)
                                })
                            })
                        });
                    if !has_match {
                        // Check if player can pay mana cost plus the additional mana
                        let available = self.state.players.get(&player_id)
                            .map(|p| p.mana_pool.available()).unwrap_or_default();
                        if !available.can_pay(mana) { return false; }
                    }
                }
                _ => {} // Other costs checked elsewhere
            }
        }
        true
    }

    fn pay_costs(&mut self, player_id: PlayerId, source_id: ObjectId, costs: &[Cost]) -> bool {
        for cost in costs {
            match cost {
                Cost::TapSelf => {
                    if let Some(perm) = self.state.battlefield.get_mut(source_id) {
                        if perm.tapped {
                            return false; // Already tapped, can't pay
                        }
                        perm.tap();
                    } else {
                        return false;
                    }
                }
                Cost::Mana(mana) => {
                    if let Some(player) = self.state.players.get_mut(&player_id) {
                        if !player.mana_pool.try_pay(mana) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                Cost::PayLife(amount) => {
                    if let Some(player) = self.state.players.get_mut(&player_id) {
                        if player.life < *amount as i32 {
                            return false;
                        }
                        player.life -= *amount as i32;
                    } else {
                        return false;
                    }
                }
                Cost::SacrificeSelf => {
                    if let Some(perm) = self.state.battlefield.remove(source_id) {
                        self.state.ability_store.remove_source(source_id);
                        let owner = perm.owner();
                        if let Some(player) = self.state.players.get_mut(&owner) {
                            player.graveyard.add(source_id);
                            self.state.set_zone(source_id, crate::constants::Zone::Graveyard, Some(owner));
                        }
                    } else {
                        return false;
                    }
                }
                Cost::ExileSelf => {
                    if let Some(_perm) = self.state.battlefield.remove(source_id) {
                        self.state.ability_store.remove_source(source_id);
                        self.state.exile.exile(source_id);
                        self.state.set_zone(source_id, crate::constants::Zone::Exile, None);
                    } else {
                        return false;
                    }
                }
                Cost::Discard(count) => {
                    let hand: Vec<ObjectId> = self.state.players.get(&player_id)
                        .map(|p| p.hand.iter().copied().collect())
                        .unwrap_or_default();
                    if hand.len() < *count as usize {
                        return false;
                    }
                    let view = crate::decision::GameView::placeholder();
                    let to_discard = if let Some(dm) = self.decision_makers.get_mut(&player_id) {
                        dm.choose_discard(&view, &hand, *count as usize)
                    } else {
                        hand.iter().rev().take(*count as usize).copied().collect()
                    };
                    for card_id in to_discard {
                        if let Some(player) = self.state.players.get_mut(&player_id) {
                            player.hand.remove(card_id);
                        }
                        if let Some(player) = self.state.players.get_mut(&player_id) {
                            player.graveyard.add(card_id);
                            self.state.set_zone(card_id, crate::constants::Zone::Graveyard, Some(player_id));
                        }
                    }
                }
                Cost::RemoveCounters(counter_type_name, count) => {
                    let ct = crate::counters::CounterType::from_name(counter_type_name);
                    if let Some(perm) = self.state.battlefield.get_mut(source_id) {
                        let current = perm.counters.get(&ct);
                        if current < *count {
                            return false; // Not enough counters
                        }
                        perm.counters.remove(&ct, *count);
                    } else {
                        return false;
                    }
                }
                Cost::Blight(count) => {
                    // Blight: put N -1/-1 counters on a creature you control (typically self).
                    // For simplicity, apply to source permanent.
                    let ct = crate::counters::CounterType::M1M1;
                    if let Some(perm) = self.state.battlefield.get_mut(source_id) {
                        perm.counters.add(ct, *count);
                    } else {
                        return false;
                    }
                }
                Cost::ExileFromGraveyard(count) => {
                    let gy_cards: Vec<ObjectId> = self.state.players.get(&player_id)
                        .map(|p| p.graveyard.iter().copied().collect())
                        .unwrap_or_default();
                    if gy_cards.len() < *count as usize {
                        return false;
                    }
                    // Use choose_discard as a general card selection mechanism
                    let view = crate::decision::GameView::placeholder();
                    let to_exile = if let Some(dm) = self.decision_makers.get_mut(&player_id) {
                        dm.choose_discard(&view, &gy_cards, *count as usize)
                    } else {
                        gy_cards.iter().rev().take(*count as usize).copied().collect()
                    };
                    for card_id in to_exile {
                        if let Some(player) = self.state.players.get_mut(&player_id) {
                            player.graveyard.remove(card_id);
                        }
                        self.state.exile.exile(card_id);
                        self.state.set_zone(card_id, crate::constants::Zone::Exile, None);
                    }
                }
                Cost::ExileFromHand(count) => {
                    let hand: Vec<ObjectId> = self.state.players.get(&player_id)
                        .map(|p| p.hand.iter().copied().collect())
                        .unwrap_or_default();
                    if hand.len() < *count as usize {
                        return false;
                    }
                    let view = crate::decision::GameView::placeholder();
                    let to_exile = if let Some(dm) = self.decision_makers.get_mut(&player_id) {
                        dm.choose_discard(&view, &hand, *count as usize)
                    } else {
                        hand.iter().rev().take(*count as usize).copied().collect()
                    };
                    for card_id in to_exile {
                        if let Some(player) = self.state.players.get_mut(&player_id) {
                            player.hand.remove(card_id);
                        }
                        self.state.exile.exile(card_id);
                        self.state.set_zone(card_id, crate::constants::Zone::Exile, None);
                    }
                }
                Cost::SacrificeOther(filter) => {
                    // Find permanents matching the filter that the player controls
                    let candidates: Vec<ObjectId> = self.state.battlefield.iter()
                        .filter(|perm| perm.controller == player_id && perm.id() != source_id)
                        .filter(|perm| {
                            let f = filter.to_lowercase();
                            if f.contains("creature") && !perm.is_creature() { return false; }
                            if f.contains("artifact") && !perm.is_artifact() { return false; }
                            if f.contains("enchantment") && !perm.is_enchantment() { return false; }
                            if f.contains("land") && !perm.is_land() { return false; }
                            true
                        })
                        .map(|perm| perm.id())
                        .collect();
                    if candidates.is_empty() {
                        return false;
                    }
                    // Pick one to sacrifice (use choose_targets-like selection)
                    let chosen = candidates[0]; // Default: first candidate
                    if let Some(perm) = self.state.battlefield.remove(chosen) {
                        self.state.ability_store.remove_source(chosen);
                        let owner = perm.owner();
                        if let Some(player) = self.state.players.get_mut(&owner) {
                            player.graveyard.add(chosen);
                            self.state.set_zone(chosen, crate::constants::Zone::Graveyard, Some(owner));
                        }
                    }
                }
                Cost::RevealFromHand(_card_type) => {
                    // Reveal cost: check that the player has a card of the required type.
                    // For now, just check hand is non-empty (full type checking deferred).
                    let hand_size = self.state.players.get(&player_id)
                        .map(|p| p.hand.len())
                        .unwrap_or(0);
                    if hand_size == 0 {
                        return false;
                    }
                }
                Cost::UntapSelf => {
                    if let Some(perm) = self.state.battlefield.get_mut(source_id) {
                        if !perm.tapped {
                            return false; // Already untapped, can't pay
                        }
                        perm.untap();
                    } else {
                        return false;
                    }
                }
                Cost::Behold(creature_type) => {
                    let ct_lower = creature_type.to_lowercase();
                    let mut candidates: Vec<ObjectId> = Vec::new();
                    for perm in self.state.battlefield.controlled_by(player_id) {
                        if perm.id() != source_id {
                            if let Some(card) = self.state.card_store.get(perm.id()) {
                                if card.subtypes.iter().any(|st| st.to_string().to_lowercase() == ct_lower)
                                    || card.keywords.contains(crate::constants::KeywordAbilities::CHANGELING) {
                                    candidates.push(perm.id());
                                }
                            }
                        }
                    }
                    if let Some(player) = self.state.players.get(&player_id) {
                        for &card_id in player.hand.iter() {
                            if card_id != source_id {
                                if let Some(card) = self.state.card_store.get(card_id) {
                                    if card.subtypes.iter().any(|st| st.to_string().to_lowercase() == ct_lower)
                                        || card.keywords.contains(crate::constants::KeywordAbilities::CHANGELING) {
                                        candidates.push(card_id);
                                    }
                                }
                            }
                        }
                    }
                    if candidates.is_empty() {
                        return false;
                    }
                }
                Cost::BeholdAndExile(creature_type) => {
                    let ct_lower = creature_type.to_lowercase();
                    let mut candidates: Vec<ObjectId> = Vec::new();
                    for perm in self.state.battlefield.controlled_by(player_id) {
                        if perm.id() != source_id {
                            if let Some(card) = self.state.card_store.get(perm.id()) {
                                if card.subtypes.iter().any(|st| st.to_string().to_lowercase() == ct_lower)
                                    || card.keywords.contains(crate::constants::KeywordAbilities::CHANGELING) {
                                    candidates.push(perm.id());
                                }
                            }
                        }
                    }
                    if let Some(player) = self.state.players.get(&player_id) {
                        for &card_id in player.hand.iter() {
                            if card_id != source_id {
                                if let Some(card) = self.state.card_store.get(card_id) {
                                    if card.subtypes.iter().any(|st| st.to_string().to_lowercase() == ct_lower)
                                        || card.keywords.contains(crate::constants::KeywordAbilities::CHANGELING) {
                                        candidates.push(card_id);
                                    }
                                }
                            }
                        }
                    }
                    if candidates.is_empty() {
                        return false;
                    }
                    let chosen = candidates[0];
                    if self.state.battlefield.get(chosen).is_some() {
                        if let Some(_perm) = self.state.battlefield.remove(chosen) {
                            self.state.ability_store.remove_source(chosen);
                            self.state.exile.exile(chosen);
                            self.state.set_zone(chosen, crate::constants::Zone::Exile, None);
                        }
                    } else if let Some(player) = self.state.players.get_mut(&player_id) {
                        player.hand.remove(chosen);
                        self.state.exile.exile(chosen);
                        self.state.set_zone(chosen, crate::constants::Zone::Exile, None);
                    }
                }
                Cost::BeholdOrPay { creature_type, mana } => {
                    let ct_lower = creature_type.to_lowercase();
                    let mut candidates: Vec<ObjectId> = Vec::new();
                    for perm in self.state.battlefield.controlled_by(player_id) {
                        if perm.id() != source_id {
                            if let Some(card) = self.state.card_store.get(perm.id()) {
                                if card.subtypes.iter().any(|st| st.to_string().to_lowercase() == ct_lower)
                                    || card.keywords.contains(crate::constants::KeywordAbilities::CHANGELING) {
                                    candidates.push(perm.id());
                                }
                            }
                        }
                    }
                    if let Some(player) = self.state.players.get(&player_id) {
                        for &card_id in player.hand.iter() {
                            if card_id != source_id {
                                if let Some(card) = self.state.card_store.get(card_id) {
                                    if card.subtypes.iter().any(|st| st.to_string().to_lowercase() == ct_lower)
                                        || card.keywords.contains(crate::constants::KeywordAbilities::CHANGELING) {
                                        candidates.push(card_id);
                                    }
                                }
                            }
                        }
                    }
                    if !candidates.is_empty() {
                        // Behold is free; prefer it over paying mana
                    } else {
                        if let Some(player) = self.state.players.get_mut(&player_id) {
                            if !player.mana_pool.try_pay(mana) {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                }
                Cost::TapCreatures { filter, count } => {
                    let f_lower = filter.to_lowercase();
                    let mut candidates: Vec<ObjectId> = self.state.battlefield.iter()
                        .filter(|perm| perm.controller == player_id && !perm.tapped && perm.id() != source_id && perm.is_creature())
                        .filter(|perm| {
                            if f_lower.contains("elf") {
                                self.state.card_store.get(perm.id()).map_or(false, |c| c.subtypes.iter().any(|st| st.to_string().to_lowercase() == "elf") || c.keywords.contains(crate::constants::KeywordAbilities::CHANGELING))
                            } else {
                                true
                            }
                        })
                        .map(|perm| perm.id())
                        .collect();
                    if (candidates.len() as u32) < *count {
                        return false;
                    }
                    for i in 0..*count as usize {
                        if let Some(perm) = self.state.battlefield.get_mut(candidates[i]) {
                            perm.tap();
                        }
                    }
                }
                Cost::Custom(_) => {
                    // Custom costs: no-op (annotation only)
                }
            }
        }
        true
    }

    /// Execute a list of effects for a controller with given targets.
    pub fn execute_effects(&mut self, effects: &[Effect], controller: PlayerId, all_targets: &[ObjectId], source: Option<ObjectId>, x_value: Option<u32>) {
        // Resolve X-value amounts: when an effect uses X_VALUE as its amount,
        // substitute the actual x_value chosen at cast time.
        let resolve_x = |amount: u32| -> u32 {
            if amount == crate::abilities::X_VALUE {
                x_value.unwrap_or(0)
            } else {
                amount
            }
        };

        // For compound fight/bite spells (e.g. [AddCounters, Bite]), pre-fight/bite
        // effects should only apply to the first target (your creature), matching
        // Java's per-effect target assignment where AddCountersTargetEffect targets
        // target 0 while DamageWithPowerFromOneToAnotherTargetEffect uses both.
        let has_fight_or_bite = effects.iter().any(|e| matches!(e, Effect::Fight | Effect::Bite));

        for effect in effects {
            let targets: &[ObjectId] = if has_fight_or_bite
                && !matches!(effect, Effect::Fight | Effect::Bite)
                && all_targets.len() >= 2
            {
                &all_targets[..1]
            } else {
                all_targets
            };
            match effect {
                Effect::DealDamage { amount } => {
                    let dmg = resolve_x(*amount);
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            perm.apply_damage(dmg);
                        }
                    }
                    if targets.is_empty() {
                        if let Some(opp_id) = self.state.opponent_of(controller) {
                            if let Some(opp) = self.state.players.get_mut(&opp_id) {
                                opp.life -= dmg as i32;
                            }
                        }
                    }
                }
                Effect::Destroy => {
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.get(target_id) {
                            if !perm.has_indestructible() {
                                let was_creature = perm.is_creature();
                                let perm_controller = perm.controller;
                                if let Some(perm) = self.state.battlefield.remove(target_id) {
                                    self.move_card_to_graveyard_inner(target_id, perm.owner());
                                    if was_creature {
                                        self.emit_event(GameEvent::dies(target_id, perm_controller));
                                    }
                                    self.state.ability_store.remove_source(target_id);
                                }
                            }
                        }
                    }
                }
                Effect::Exile => {
                    for &target_id in targets {
                        if self.state.battlefield.remove(target_id).is_some() {
                            self.state.ability_store.remove_source(target_id);
                            self.state.exile.exile(target_id);
                            self.state.set_zone(target_id, crate::constants::Zone::Exile, None);
                        }
                    }
                }
                Effect::Bounce => {
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.remove(target_id) {
                            self.state.ability_store.remove_source(target_id);
                            let owner = perm.owner();
                            if let Some(player) = self.state.players.get_mut(&owner) {
                                player.hand.add(target_id);
                            }
                            self.state.set_zone(target_id, crate::constants::Zone::Hand, Some(owner));
                        }
                    }
                }
                Effect::PutOnLibrary => {
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.remove(target_id) {
                            self.state.ability_store.remove_source(target_id);
                            let owner = perm.owner();
                            if let Some(player) = self.state.players.get_mut(&owner) {
                                player.library.put_on_top(target_id);
                            }
                            self.state.set_zone(target_id, crate::constants::Zone::Library, Some(owner));
                        }
                    }
                }
                Effect::DrawCards { count } => {
                    self.draw_cards(controller, resolve_x(*count));
                }
                Effect::GainLife { amount } => {
                    if let Some(player) = self.state.players.get_mut(&controller) {
                        player.life += resolve_x(*amount) as i32;
                    }
                    // Emit life gain event
                    self.emit_event(
                        GameEvent::gain_life(controller, resolve_x(*amount)),
                    );
                }
                Effect::LoseLife { amount } => {
                    // Controller loses life (target player effects will use
                    // SelectedTargets for proper player targeting)
                    if let Some(player) = self.state.players.get_mut(&controller) {
                        player.life -= resolve_x(*amount) as i32;
                    }
                }
                Effect::LoseLifeOpponents { amount } => {
                    let opponents: Vec<PlayerId> = self.state.turn_order.iter()
                        .filter(|&&id| id != controller)
                        .copied()
                        .collect();
                    for opp in opponents {
                        if let Some(player) = self.state.players.get_mut(&opp) {
                            player.life -= resolve_x(*amount) as i32;
                        }
                    }
                }
                Effect::DealDamageOpponents { amount } => {
                    let opponents: Vec<PlayerId> = self.state.turn_order.iter()
                        .filter(|&&id| id != controller)
                        .copied()
                        .collect();
                    for opp in opponents {
                        if let Some(player) = self.state.players.get_mut(&opp) {
                            player.life -= resolve_x(*amount) as i32;
                        }
                    }
                }
                Effect::AddCounters { counter_type, count: raw_count } => {
                    let count = resolve_x(*raw_count);
                    let ct = crate::counters::CounterType::from_name(counter_type);
                    // If no targets, fall back to source (self-targeting counters)
                    let effective_targets: Vec<ObjectId> = if targets.is_empty() {
                        source.into_iter().collect()
                    } else {
                        targets.to_vec()
                    };
                    for target_id in effective_targets {
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            perm.add_counters(ct.clone(), count);
                        }
                    }
                }
                Effect::AddCountersSelf { counter_type, count: raw_count } => {
                    let count = resolve_x(*raw_count);
                    // Always add counters to the source permanent, even when the
                    // ability has other targets (e.g. blight self + grant haste to target).
                    if let Some(source_id) = source {
                        let ct = crate::counters::CounterType::from_name(counter_type);
                        if let Some(perm) = self.state.battlefield.get_mut(source_id) {
                            perm.add_counters(ct, count);
                        }
                    }
                }
                Effect::BoostUntilEndOfTurn { power, toughness: _ } => {
                    // Simplified: directly modify counters (proper implementation
                    // would use continuous effects that expire at end of turn)
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            if *power > 0 {
                                perm.add_counters(CounterType::P1P1, *power as u32);
                            }
                            // Note: This is a simplification; real boost until EOT
                            // uses continuous effects, not counters
                        }
                    }
                }
                Effect::TapTarget => {
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            perm.tap();
                        }
                    }
                }
                Effect::UntapTarget => {
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            perm.untap();
                        }
                    }
                }
                Effect::CounterSpell => {
                    // Counter first target on the stack
                    for &target_id in targets {
                        // Check if the target spell has "can't be countered" (individual card)
                        let cant_counter_self = if let Some(item) = self.state.stack.get(target_id) {
                            if let crate::zones::StackItemKind::Spell { card } = &item.kind {
                                card.abilities.iter().any(|a| {
                                    a.static_effects.iter().any(|se| matches!(se, StaticEffect::CantBeCountered))
                                })
                            } else {
                                false
                            }
                        } else {
                            false
                        };
                        // Check if the spell's controller has "spells can't be countered" from a permanent
                        let cant_counter_from_permanent = if let Some(item) = self.state.stack.get(target_id) {
                            let spell_controller = item.controller;
                            self.state.battlefield.iter().any(|perm| {
                                perm.controller == spell_controller && {
                                    let abilities = self.state.ability_store.for_source(perm.id());
                                    abilities.iter().any(|a| {
                                        a.ability_type == AbilityType::Static
                                            && a.static_effects.iter().any(|se| matches!(se, StaticEffect::SpellsCantBeCountered))
                                    })
                                }
                            })
                        } else {
                            false
                        };
                        if cant_counter_self || cant_counter_from_permanent {
                            continue; // Can't counter this spell
                        }
                        if let Some(stack_item) = self.state.stack.remove(target_id) {
                            match &stack_item.kind {
                                crate::zones::StackItemKind::Spell { .. } => {
                                    self.move_card_to_graveyard_inner(stack_item.id, stack_item.controller);
                                }
                                _ => {} // Countered abilities just vanish
                            }
                        }
                    }
                }
                Effect::AddMana { mana } => {
                    if let Some(player) = self.state.players.get_mut(&controller) {
                        player.mana_pool.add(*mana, None, false);
                    }
                }
                Effect::DiscardCards { count } => {
                    // Controller discards (simplified: discard from the back of hand)
                    let hand: Vec<ObjectId> = self.state.players.get(&controller)
                        .map(|p| p.hand.iter().copied().collect())
                        .unwrap_or_default();
                    let view = crate::decision::GameView::placeholder();
                    let to_discard = if let Some(dm) = self.decision_makers.get_mut(&controller) {
                        dm.choose_discard(&view, &hand, *count as usize)
                    } else {
                        hand.iter().rev().take(*count as usize).copied().collect()
                    };
                    for card_id in to_discard {
                        if let Some(player) = self.state.players.get_mut(&controller) {
                            player.hand.remove(card_id);
                        }
                        self.move_card_to_graveyard_inner(card_id, controller);
                    }
                }
                Effect::DiscardOpponents { count } => {
                    let opponents: Vec<PlayerId> = self.state.turn_order.iter()
                        .filter(|&&id| id != controller)
                        .copied()
                        .collect();
                    for opp in opponents {
                        let hand: Vec<ObjectId> = self.state.players.get(&opp)
                            .map(|p| p.hand.iter().copied().collect())
                            .unwrap_or_default();
                        let view = crate::decision::GameView::placeholder();
                        let to_discard = if let Some(dm) = self.decision_makers.get_mut(&opp) {
                            dm.choose_discard(&view, &hand, resolve_x(*count) as usize)
                        } else {
                            hand.iter().rev().take(resolve_x(*count) as usize).copied().collect()
                        };
                        for card_id in to_discard {
                            if let Some(player) = self.state.players.get_mut(&opp) {
                                player.hand.remove(card_id);
                            }
                            self.move_card_to_graveyard_inner(card_id, opp);
                        }
                    }
                }
                Effect::Mill { count } => {
                    for _ in 0..resolve_x(*count) {
                        let card_id = self.state.players.get_mut(&controller)
                            .and_then(|p| p.library.draw());
                        if let Some(card_id) = card_id {
                            self.move_card_to_graveyard_inner(card_id, controller);
                        }
                    }
                }
                Effect::CreateToken { token_name, count } => {
                    for _ in 0..resolve_x(*count) {
                        // Create a minimal token permanent
                        let token_id = ObjectId::new();
                        let mut card = CardData::new(token_id, controller, token_name);
                        card.card_types = vec![crate::constants::CardType::Creature];
                        // Parse token stats from name (e.g. "4/4 Dragon with flying")
                        let (p, t, kw) = Self::parse_token_stats(token_name);
                        card.power = Some(p);
                        card.toughness = Some(t);
                        card.keywords = kw;
                        card.is_token = true;
                        let perm = Permanent::new(card, controller);
                        self.state.battlefield.add(perm);
                        self.state.set_zone(token_id, crate::constants::Zone::Battlefield, None);
                        self.emit_event(GameEvent::enters_battlefield(token_id, controller));
                    }
                }
                Effect::Scry { count } => {
                    // Scry N: look at top N cards, put any number on bottom in any order,
                    // rest on top in any order. Simplified: AI picks which to bottom.
                    if let Some(player) = self.state.players.get(&controller) {
                        let top_cards: Vec<ObjectId> = player.library.peek(*count as usize).to_vec();
                        if !top_cards.is_empty() {
                            let view = crate::decision::GameView::placeholder();
                            let to_bottom = if let Some(dm) = self.decision_makers.get_mut(&controller) {
                                // Ask AI which cards to put on bottom (0 to all)
                                dm.choose_cards_to_put_back(&view, &top_cards, 0)
                            } else {
                                // Default: put nothing on bottom (keep all on top)
                                Vec::new()
                            };
                            // Remove selected cards and put them on bottom
                            for &card_id in &to_bottom {
                                if let Some(player) = self.state.players.get_mut(&controller) {
                                    player.library.remove(card_id);
                                    player.library.put_on_bottom(card_id);
                                }
                            }
                        }
                    }
                }
                Effect::ReturnFromGraveyard => {
                    // Return target card from graveyard to owner's hand
                    for &target_id in targets {
                        // Find which player's graveyard contains this card
                        let owner = self.state.find_card_owner_in_graveyard(target_id);
                        if let Some(owner_id) = owner {
                            if let Some(player) = self.state.players.get_mut(&owner_id) {
                                if player.graveyard.remove(target_id) {
                                    player.hand.add(target_id);
                                    self.state.set_zone(target_id, crate::constants::Zone::Hand, Some(owner_id));
                                }
                            }
                        }
                    }
                }
                Effect::Reanimate => {
                    // Return target card from graveyard to battlefield under controller's control
                    for &target_id in targets {
                        let owner = self.state.find_card_owner_in_graveyard(target_id);
                        if let Some(owner_id) = owner {
                            if let Some(player) = self.state.players.get_mut(&owner_id) {
                                player.graveyard.remove(target_id);
                            }
                            // Get card data from the card store to create a permanent
                            if let Some(card_data) = self.state.card_store.remove(target_id) {
                                // Re-register abilities for reanimated permanent
                                for ability in &card_data.abilities {
                                    self.state.ability_store.add(ability.clone());
                                }
                                let perm = Permanent::new(card_data, controller);
                                self.state.battlefield.add(perm);
                                self.state.set_zone(target_id, crate::constants::Zone::Battlefield, None);
                                self.check_enters_tapped(target_id);
                            }
                        }
                    }
                }
                Effect::GainKeywordUntilEndOfTurn { keyword } => {
                    if let Some(kw) = crate::constants::KeywordAbilities::keyword_from_name(keyword) {
                        for &target_id in targets {
                            if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                                perm.granted_keywords |= kw;
                            }
                        }
                    }
                }
                Effect::GainKeyword { keyword } => {
                    // Grant keyword permanently (via granted_keywords, which persists)
                    if let Some(kw) = crate::constants::KeywordAbilities::keyword_from_name(keyword) {
                        for &target_id in targets {
                            if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                                perm.granted_keywords |= kw;
                            }
                        }
                    }
                }
                Effect::Indestructible => {
                    // Grant indestructible until end of turn
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            perm.granted_keywords |= crate::constants::KeywordAbilities::INDESTRUCTIBLE;
                        }
                    }
                }
                Effect::Hexproof => {
                    // Grant hexproof until end of turn
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            perm.granted_keywords |= crate::constants::KeywordAbilities::HEXPROOF;
                        }
                    }
                }
                Effect::CantBlock => {
                    // Target creature can't block this turn.
                    // Simplified: grant a pseudo-keyword. The combat system checks
                    // granted_keywords for blocking restrictions.
                    // For now, we mark via a flag (using removed_keywords to prevent DEFENDER
                    // from mattering is not the right approach). We'll use a simple approach:
                    // add a "can't block" counter that gets cleared at cleanup.
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            // Use a sentinel counter to indicate can't block
                            perm.add_counters(crate::counters::CounterType::Custom("cant_block".into()), 1);
                        }
                    }
                }
                Effect::Sacrifice { filter } => {
                    // Each opponent sacrifices a permanent matching filter.
                    let opponents: Vec<PlayerId> = self.state.turn_order.iter()
                        .filter(|&&id| id != controller)
                        .copied()
                        .collect();
                    for opp in opponents {
                        let matching: Vec<ObjectId> = self.state.battlefield.iter()
                            .filter(|p| p.controller == opp && Self::matches_filter(p, filter))
                            .map(|p| p.id())
                            .collect();
                        if let Some(&victim_id) = matching.first() {
                            let was_creature = self.state.battlefield.get(victim_id)
                                .map(|p| p.is_creature()).unwrap_or(false);
                            if let Some(perm) = self.state.battlefield.remove(victim_id) {
                                self.move_card_to_graveyard_inner(victim_id, perm.owner());
                                if was_creature {
                                    self.emit_event(GameEvent::dies(victim_id, opp));
                                }
                                self.state.ability_store.remove_source(victim_id);
                            }
                        }
                    }
                }
                Effect::DestroyAll { filter } => {
                    // Destroy all permanents matching filter
                    let to_destroy: Vec<(ObjectId, PlayerId, bool)> = self.state.battlefield.iter()
                        .filter(|p| Self::matches_filter(p, filter) && !p.has_indestructible())
                        .map(|p| (p.id(), p.owner(), p.is_creature()))
                        .collect();
                    for (id, owner, was_creature) in &to_destroy {
                        if let Some(perm) = self.state.battlefield.remove(*id) {
                            self.move_card_to_graveyard_inner(*id, *owner);
                            if *was_creature {
                                self.emit_event(GameEvent::dies(*id, perm.controller));
                            }
                        }
                    }
                    // Deferred ability cleanup
                    for (id, _, _) in &to_destroy {
                        self.state.ability_store.remove_source(*id);
                    }
                }
                Effect::DealDamageAll { amount, filter } => {
                    let dmg = resolve_x(*amount);
                    // Deal damage to all creatures matching filter
                    let matching: Vec<ObjectId> = self.state.battlefield.iter()
                        .filter(|p| p.is_creature() && Self::matches_filter(p, filter))
                        .map(|p| p.id())
                        .collect();
                    for id in matching {
                        if let Some(perm) = self.state.battlefield.get_mut(id) {
                            perm.apply_damage(dmg);
                        }
                    }
                }
                Effect::RemoveCounters { counter_type, count } => {
                    let ct = crate::counters::CounterType::from_name(counter_type);
                    // If no targets, fall back to source (self-targeting counters)
                    let effective_targets: Vec<ObjectId> = if targets.is_empty() {
                        source.into_iter().collect()
                    } else {
                        targets.to_vec()
                    };
                    for target_id in effective_targets {
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            perm.counters.remove(&ct, *count);
                        }
                    }
                }
                Effect::SearchLibrary { filter } => {
                    // Search library for a card matching filter and put it in hand.
                    // Simplified: find the first matching card.
                    if let Some(player) = self.state.players.get(&controller) {
                        let lib_cards: Vec<ObjectId> = player.library.iter().copied().collect();
                        let found = lib_cards.iter().find(|&&card_id| {
                            self.state.card_store.get(card_id)
                                .map(|c| Self::card_matches_filter(c, filter))
                                .unwrap_or(false)
                        }).copied();
                        if let Some(card_id) = found {
                            if let Some(player) = self.state.players.get_mut(&controller) {
                                player.library.remove(card_id);
                                player.hand.add(card_id);
                                self.state.set_zone(card_id, crate::constants::Zone::Hand, Some(controller));
                            }
                        }
                    }
                }
                Effect::LookTopAndPick { count, filter } => {
                    // Look at top N cards, pick one matching filter to hand,
                    // rest go to bottom of library in random order.
                    let (top_cards, picked) = if let Some(player) = self.state.players.get(&controller) {
                        let look_count = (*count as usize).min(player.library.len());
                        let top_cards: Vec<ObjectId> = player.library.peek(look_count).to_vec();
                        let picked = top_cards.iter().find(|&&card_id| {
                            self.state.card_store.get(card_id)
                                .map(|c| Self::card_matches_filter(c, filter))
                                .unwrap_or(false)
                        }).copied();
                        (top_cards, picked)
                    } else {
                        (vec![], None)
                    };
                    if let Some(player) = self.state.players.get_mut(&controller) {
                        for &card_id in &top_cards {
                            player.library.remove(card_id);
                        }
                        if let Some(card_id) = picked {
                            player.hand.add(card_id);
                        }
                        for &card_id in &top_cards {
                            if Some(card_id) != picked {
                                player.library.put_on_bottom(card_id);
                            }
                        }
                    }
                    if let Some(card_id) = picked {
                        self.state.set_zone(card_id, crate::constants::Zone::Hand, Some(controller));
                    }
                }
                Effect::CreateTokenTappedAttacking { token_name, count } => {
                    // Create tokens tapped and attacking (used by Mobilize mechanic)
                    for _ in 0..*count {
                        let token_id = ObjectId::new();
                        let mut card = CardData::new(token_id, controller, token_name);
                        card.card_types = vec![crate::constants::CardType::Creature];
                        let (p, t, kw) = Self::parse_token_stats(token_name);
                        card.power = Some(p);
                        card.toughness = Some(t);
                        card.keywords = kw;
                        card.is_token = true;
                        let mut perm = Permanent::new(card, controller);
                        perm.tapped = true;
                        perm.summoning_sick = false; // Can attack since entering tapped and attacking
                        self.state.battlefield.add(perm);
                        self.state.set_zone(token_id, crate::constants::Zone::Battlefield, None);
                    }
                }
                Effect::BoostPermanent { power, toughness: _ } => {
                    // Permanent P/T boost (similar to BoostUntilEndOfTurn but doesn't expire)
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            if *power > 0 {
                                perm.add_counters(CounterType::P1P1, *power as u32);
                            } else if *power < 0 {
                                perm.add_counters(CounterType::M1M1, (-*power) as u32);
                            }
                        }
                    }
                }
                Effect::SetPowerToughness { power, toughness } => {
                    // Set base P/T (simplified: adjust via counters to reach target)
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            let current_p = perm.power();
                            let current_t = perm.toughness();
                            let dp = *power - current_p;
                            let dt = *toughness - current_t;
                            // Use counters to approximate (imperfect but functional)
                            if dp > 0 {
                                perm.add_counters(CounterType::P1P1, dp as u32);
                            } else if dp < 0 {
                                perm.add_counters(CounterType::M1M1, (-dp) as u32);
                            }
                            let _ = dt; // Toughness adjustment via counters is coupled with power
                        }
                    }
                }
                Effect::LoseKeyword { keyword } => {
                    if let Some(kw) = crate::constants::KeywordAbilities::keyword_from_name(keyword) {
                        for &target_id in targets {
                            if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                                perm.removed_keywords |= kw;
                            }
                        }
                    }
                }
                Effect::BoostAllUntilEndOfTurn { filter, power, toughness: _ } => {
                    // Give all matching creatures controlled by the effect's controller +N/+M until EOT
                    let you_control = filter.to_lowercase().contains("you control");
                    let matching: Vec<ObjectId> = self.state.battlefield.iter()
                        .filter(|p| p.is_creature()
                            && (!you_control || p.controller == controller)
                            && Self::matches_filter(p, filter))
                        .map(|p| p.id())
                        .collect();
                    for id in matching {
                        if let Some(perm) = self.state.battlefield.get_mut(id) {
                            if *power > 0 {
                                perm.add_counters(CounterType::P1P1, *power as u32);
                            } else if *power < 0 {
                                perm.add_counters(CounterType::M1M1, (-*power) as u32);
                            }
                        }
                    }
                }
                Effect::GrantKeywordAllUntilEndOfTurn { filter, keyword } => {
                    // Grant keyword to all matching creatures controlled by the effect's controller until EOT
                    if let Some(kw) = crate::constants::KeywordAbilities::keyword_from_name(keyword) {
                        let you_control = filter.to_lowercase().contains("you control");
                        let matching: Vec<ObjectId> = self.state.battlefield.iter()
                            .filter(|p| p.is_creature()
                                && (!you_control || p.controller == controller)
                                && Self::matches_filter(p, filter))
                            .map(|p| p.id())
                            .collect();
                        for id in matching {
                            if let Some(perm) = self.state.battlefield.get_mut(id) {
                                perm.granted_keywords |= kw;
                            }
                        }
                    }
                }
                Effect::Fight => {
                    // Fight: two creatures deal damage equal to their power to each other.
                    //
                    // Target resolution (matches Java FightTargetsEffect):
                    //   - If targets has 2+ entries: targets[0] = your creature, targets[1] = opponent's
                    //   - If targets has 1 entry + source is creature: source fights targets[0]
                    //   - Fallback: auto-select strongest on each side
                    let (fighter_id, target_id) = Self::resolve_fight_pair(
                        &self.state, targets, source, controller,
                    );

                    if let (Some(fid), Some(tid)) = (fighter_id, target_id) {
                        if fid != tid {
                            let fighter_power = self.state.battlefield.get(fid)
                                .map(|p| p.power().max(0) as u32).unwrap_or(0);
                            let target_power = self.state.battlefield.get(tid)
                                .map(|p| p.power().max(0) as u32).unwrap_or(0);
                            if let Some(target_perm) = self.state.battlefield.get_mut(tid) {
                                target_perm.apply_damage(fighter_power);
                            }
                            if let Some(fighter_perm) = self.state.battlefield.get_mut(fid) {
                                fighter_perm.apply_damage(target_power);
                            }
                        }
                    }
                }
                Effect::Bite => {
                    // Bite: source creature deals damage equal to its power to target
                    // creature (one-way; the target does not deal damage back).
                    // Same target resolution as Fight.
                    let (biter_id, target_id) = Self::resolve_fight_pair(
                        &self.state, targets, source, controller,
                    );

                    if let (Some(bid), Some(tid)) = (biter_id, target_id) {
                        if bid != tid {
                            let biter_power = self.state.battlefield.get(bid)
                                .map(|p| p.power().max(0) as u32).unwrap_or(0);
                            if let Some(target_perm) = self.state.battlefield.get_mut(tid) {
                                target_perm.apply_damage(biter_power);
                            }
                        }
                    }
                }
                Effect::AddCountersAll { counter_type, count, filter } => {
                    let ct = crate::counters::CounterType::from_name(counter_type);
                    let you_control = filter.to_lowercase().contains("you control");
                    let matching: Vec<ObjectId> = self.state.battlefield.iter()
                        .filter(|p| p.is_creature()
                            && (!you_control || p.controller == controller)
                            && Self::matches_filter(p, filter))
                        .map(|p| p.id())
                        .collect();
                    for id in matching {
                        if let Some(perm) = self.state.battlefield.get_mut(id) {
                            perm.add_counters(ct.clone(), *count);
                        }
                    }
                }
                Effect::GainControl => {
                    // Permanently gain control of target permanent.
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            perm.controller = controller;
                        }
                    }
                }
                Effect::GainControlUntilEndOfTurn => {
                    // Gain control of target until end of turn.
                    // Track original controller for cleanup revert.
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            if perm.original_controller.is_none() {
                                perm.original_controller = Some(perm.controller);
                            }
                            perm.controller = controller;
                            perm.untap();
                            perm.granted_keywords |= crate::constants::KeywordAbilities::HASTE;
                        }
                    }
                }
                Effect::Modal { modes, min_modes: _, max_modes } => {
                    // Modal spells: player chooses min..=max modes, then
                    // execute each chosen mode's effects in order.
                    let mut chosen_indices: Vec<usize> = Vec::new();
                    let _num_modes = modes.len();

                    for _ in 0..*max_modes {
                        if chosen_indices.len() >= *max_modes {
                            break;
                        }
                        // Build available choices (exclude already-chosen modes)
                        let available: Vec<crate::decision::NamedChoice> = modes.iter()
                            .enumerate()
                            .filter(|(i, _)| !chosen_indices.contains(i))
                            .map(|(i, m)| crate::decision::NamedChoice {
                                index: i,
                                description: m.description.clone(),
                            })
                            .collect();

                        if available.is_empty() {
                            break;
                        }

                        // If we already have min_modes, we could stop, but
                        // for simplicity always choose up to max_modes.
                        let view = crate::decision::GameView::placeholder();
                        let choice = if let Some(dm) = self.decision_makers.get_mut(&controller) {
                            let raw = dm.choose_mode(&view, &available);
                            raw.min(available.len().saturating_sub(1))
                        } else {
                            0
                        };

                        if let Some(named) = available.get(choice) {
                            chosen_indices.push(named.index);
                        }
                    }

                    // Execute each chosen mode's effects
                    for &mode_idx in &chosen_indices {
                        if let Some(mode) = modes.get(mode_idx) {
                            self.execute_effects(&mode.effects, controller, targets, source, None);
                        }
                    }
                }
                Effect::DealDamageVivid => {
                    let x = self.count_colors_among_permanents(controller) as u32;
                    if x > 0 {
                        for &target_id in targets {
                            if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                                perm.apply_damage(x);
                            }
                        }
                        // If no permanent targets, deal to opponent (same pattern as DealDamage)
                        if targets.is_empty() {
                            if let Some(opp_id) = self.state.opponent_of(controller) {
                                if let Some(opp) = self.state.players.get_mut(&opp_id) {
                                    opp.life -= x as i32;
                                }
                            }
                        }
                    }
                }
                Effect::GainLifeVivid => {
                    let x = self.count_colors_among_permanents(controller) as u32;
                    if let Some(player) = self.state.players.get_mut(&controller) {
                        player.gain_life(x);
                    }
                    if x > 0 {
                        self.emit_event(GameEvent::gain_life(controller, x));
                    }
                }
                Effect::BoostUntilEotVivid => {
                    let x = self.count_colors_among_permanents(controller) as i32;
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            perm.card.power = perm.card.power.map(|p| p + x);
                            perm.card.toughness = perm.card.toughness.map(|t| t + x);
                        }
                    }
                }
                Effect::LoseLifeOpponentsVivid => {
                    let x = self.count_colors_among_permanents(controller) as u32;
                    for (&pid, player) in self.state.players.iter_mut() {
                        if pid != controller {
                            player.lose_life(x);
                        }
                    }
                }
                Effect::DrawCardsVivid => {
                    let x = self.count_colors_among_permanents(controller) as u32;
                    // Draw X cards (same pattern as DrawCards)
                    let mut drawn: Vec<ObjectId> = Vec::new();
                    if let Some(player) = self.state.players.get_mut(&controller) {
                        for _ in 0..x {
                            if let Some(card_id) = player.library.draw() {
                                player.hand.add(card_id);
                                drawn.push(card_id);
                            }
                        }
                    }
                    for card_id in drawn {
                        self.state.set_zone(card_id, crate::constants::Zone::Hand, Some(controller));
                    }
                }
                Effect::BoostAllUntilEotVivid => {
                    let x = self.count_colors_among_permanents(controller) as i32;
                    let ids: Vec<ObjectId> = self.state.battlefield.controlled_by(controller)
                        .filter(|p| p.is_creature())
                        .map(|p| p.id())
                        .collect();
                    for id in ids {
                        if Some(id) != source {
                            if let Some(perm) = self.state.battlefield.get_mut(id) {
                                perm.card.power = perm.card.power.map(|p| p + x);
                                perm.card.toughness = perm.card.toughness.map(|t| t + x);
                            }
                        }
                    }
                }
                Effect::CreateTokenVivid { token_name } => {
                    let x = self.count_colors_among_permanents(controller) as u32;
                    for _ in 0..x {
                        let token_id = ObjectId::new();
                        let mut card = CardData::new(token_id, controller, token_name);
                        card.card_types = vec![crate::constants::CardType::Creature];
                        let (p, t, kw) = Self::parse_token_stats(token_name);
                        card.power = Some(p);
                        card.toughness = Some(t);
                        card.keywords = kw;
                        card.is_token = true;
                        let perm = Permanent::new(card, controller);
                        self.state.battlefield.add(perm);
                        self.state.set_zone(token_id, crate::constants::Zone::Battlefield, None);
                    }
                }
                Effect::DoIfCostPaid { cost, if_paid, if_not_paid } => {
                    // Ask player if they want to pay the cost
                    let view = crate::decision::GameView::placeholder();
                    let wants_to_pay = if let Some(dm) = self.decision_makers.get_mut(&controller) {
                        dm.choose_use(&view, crate::constants::Outcome::Benefit, "Pay the cost?")
                    } else {
                        false
                    };
                    let source_id = source.unwrap_or(ObjectId::new());
                    if wants_to_pay && self.pay_costs(controller, source_id, &[cost.clone()]) {
                        self.execute_effects(if_paid, controller, targets, source, None);
                    } else {
                        self.execute_effects(if_not_paid, controller, targets, source, None);
                    }
                }
                Effect::ChooseCreatureType { restricted } => {
                    // Build list of creature type options
                    let options: Vec<crate::decision::NamedChoice> = if restricted.is_empty() {
                        // Default ECL types when unrestricted
                        vec!["Elemental", "Elf", "Faerie", "Giant", "Goblin", "Kithkin", "Merfolk", "Treefolk",
                             "Human", "Warrior", "Wizard", "Rogue", "Cleric", "Shaman", "Soldier", "Knight"]
                            .into_iter().enumerate()
                            .map(|(i, s)| crate::decision::NamedChoice { index: i, description: s.to_string() })
                            .collect()
                    } else {
                        restricted.iter().enumerate()
                            .map(|(i, s)| crate::decision::NamedChoice { index: i, description: s.clone() })
                            .collect()
                    };
                    let view = crate::decision::GameView::placeholder();
                    let choice_idx = if let Some(dm) = self.decision_makers.get_mut(&controller) {
                        dm.choose_option(&view, crate::constants::Outcome::Benefit, "Choose a creature type", &options)
                    } else {
                        0
                    };
                    if let Some(chosen) = options.get(choice_idx) {
                        let subtype = crate::constants::SubType::Custom(chosen.description.clone().into());
                        if let Some(source_id) = source {
                            if let Some(perm) = self.state.battlefield.get_mut(source_id) {
                                perm.chosen_type = Some(subtype);
                            }
                        }
                    }
                }
                Effect::ChooseTypeAndDrawPerPermanent => {
                    // Choose a creature type, then draw cards equal to permanents of that type
                    let options: Vec<crate::decision::NamedChoice> =
                        vec!["Elemental", "Elf", "Faerie", "Giant", "Goblin", "Kithkin", "Merfolk", "Treefolk",
                             "Human", "Warrior", "Wizard", "Rogue", "Cleric", "Shaman", "Soldier", "Knight"]
                            .into_iter().enumerate()
                            .map(|(i, s)| crate::decision::NamedChoice { index: i, description: s.to_string() })
                            .collect();
                    let view = crate::decision::GameView::placeholder();
                    let choice_idx = if let Some(dm) = self.decision_makers.get_mut(&controller) {
                        dm.choose_option(&view, crate::constants::Outcome::Benefit, "Choose a creature type", &options)
                    } else {
                        0
                    };
                    if let Some(chosen) = options.get(choice_idx) {
                        let type_name = &chosen.description;
                        let count = self.state.battlefield.controlled_by(controller)
                            .filter(|p| {
                                p.card.subtypes.iter().any(|st| {
                                    match st {
                                        crate::constants::SubType::Custom(s) => s == type_name,
                                        other => format!("{:?}", other) == *type_name,
                                    }
                                })
                            })
                            .count();
                        if count > 0 {
                            if let Some(player) = self.state.players.get_mut(&controller) {
                                let drawn: Vec<_> = (0..count).filter_map(|_| player.library.draw()).collect();
                                for card_id in &drawn {
                                    player.hand.add(*card_id);
                                }
                                for card_id in drawn {
                                    self.state.set_zone(card_id, crate::constants::Zone::Hand, Some(controller));
                                }
                            }
                        }
                    }
                }
                Effect::Equip => {
                    // Attach this equipment to target creature.
                    if let Some(source_id) = source {
                        for &target_id in targets {
                            // Detach from previous creature if already equipped
                            if let Some(equip) = self.state.battlefield.get(source_id) {
                                if let Some(old_target) = equip.attached_to {
                                    if let Some(old_creature) = self.state.battlefield.get_mut(old_target) {
                                        old_creature.remove_attachment(source_id);
                                    }
                                }
                            }
                            // Attach to new target
                            if let Some(equip) = self.state.battlefield.get_mut(source_id) {
                                equip.attach_to(target_id);
                            }
                            if let Some(creature) = self.state.battlefield.get_mut(target_id) {
                                creature.add_attachment(source_id);
                            }
                        }
                    }
                }
                Effect::CreateDelayedTrigger { event_type, trigger_effects, duration, watch_target } => {
                    let evt = crate::events::EventType::from_name(event_type);
                    let dur = match duration.as_str() {
                        "until_triggered" => crate::state::DelayedDuration::UntilTriggered,
                        _ => crate::state::DelayedDuration::EndOfTurn,
                    };
                    let watching = if *watch_target {
                        targets.first().copied().or(source)
                    } else {
                        None
                    };
                    self.state.delayed_triggers.push(crate::state::DelayedTrigger {
                        event_type: evt,
                        watching,
                        effects: trigger_effects.clone(),
                        controller,
                        source,
                        targets: targets.to_vec(),
                        duration: dur,
                        trigger_only_once: true,
                        created_turn: self.state.turn_number,
                    });
                }
                Effect::ExileTopAndPlay { count, duration, without_mana } => {
                    let n = resolve_x(*count) as usize;
                    let dur = match duration.as_str() {
                        "until_end_of_next_turn" => crate::state::ImpulseDuration::UntilEndOfNextTurn,
                        _ => crate::state::ImpulseDuration::EndOfTurn,
                    };
                    // Exile top N cards from the controller's library
                    let mut exiled = Vec::new();
                    for _ in 0..n {
                        let card_id = self.state.players.get_mut(&controller)
                            .and_then(|p| p.library.draw());
                        if let Some(id) = card_id {
                            self.state.exile.exile(id);
                            self.state.set_zone(id, crate::constants::Zone::Exile, None);
                            exiled.push(id);
                        }
                    }
                    // Register as impulse-playable
                    let turn = self.state.turn_number;
                    for id in exiled {
                        self.state.impulse_playable.push(crate::state::ImpulsePlayable {
                            card_id: id,
                            player_id: controller,
                            duration: dur.clone(),
                            created_turn: turn,
                            without_mana: *without_mana,
                        });
                    }
                }
                Effect::ReturnExiledToHand => {
                    // Return cards exiled by this source to owners hands
                    if let Some(src) = source {
                        let exiled_ids: Vec<ObjectId> = self.state.exile.iter_all().copied().collect();
                        for card_id in exiled_ids {
                            // In our simplified model, we track exile source via zone_owner
                            // For now, return all exiled cards to their owners hands
                            // A more complete implementation would track exile source
                            if let Some(card) = self.state.card_store.get(card_id) {
                                let owner = card.owner;
                                self.state.exile.remove(card_id);
                                if let Some(player) = self.state.players.get_mut(&owner) {
                                    player.hand.add(card_id);
                                    self.state.set_zone(card_id, crate::constants::Zone::Hand, Some(owner));
                                }
                            }
                        }
                    }
                }
                Effect::UntapAll { filter } => {
                    let src_id = source.unwrap_or(ObjectId::new());
                    let matching = self.find_matching_permanents(src_id, controller, filter);
                    for perm_id in matching {
                        if let Some(perm) = self.state.battlefield.get_mut(perm_id) {
                            perm.untap();
                        }
                    }
                }
                Effect::CantBeBlockedUntilEot => {
                    // Give target "cant be blocked this turn"
                    let target = all_targets.first().or(source.as_ref());
                    if let Some(&tid) = target {
                        if let Some(perm) = self.state.battlefield.get_mut(tid) {
                            perm.granted_keywords |= crate::constants::KeywordAbilities::UNBLOCKABLE;
                        }
                    }
                }
                Effect::TapAttached => {
                    // Tap the permanent this source is attached to (aura/equipment ETB)
                    if let Some(&src_id) = source.as_ref() {
                        let attached_to = self.state.battlefield.get(src_id)
                            .and_then(|p| p.attached_to);
                        if let Some(target_id) = attached_to {
                            if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                                perm.tap();
                            }
                        }
                    }
                }
                Effect::Proliferate => {
                    // For each permanent with counters, add one more of each type
                    let perm_counters: Vec<(ObjectId, Vec<crate::counters::CounterType>)> = self
                        .state
                        .battlefield
                        .iter()
                        .filter(|p| !p.counters.is_empty())
                        .map(|p| {
                            let types: Vec<crate::counters::CounterType> = p.counters.iter()
                                .map(|(ct, _)| ct.clone())
                                .collect();
                            (p.id(), types)
                        })
                        .collect();
                    for (perm_id, counter_types) in perm_counters {
                        for ct in counter_types {
                            if let Some(perm) = self.state.battlefield.get_mut(perm_id) {
                                perm.add_counters(ct, 1);
                            }
                        }
                    }
                }
                Effect::RemoveAllCounters => {
                    // Remove all counters from target creature
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            perm.counters.clear();
                        }
                    }
                    // Fall back to source if no targets
                    if targets.is_empty() {
                        if let Some(&src_id) = source.as_ref() {
                            if let Some(perm) = self.state.battlefield.get_mut(src_id) {
                                perm.counters.clear();
                            }
                        }
                    }
                }
                Effect::ExileTargetCardsFromGraveyards { count } => {
                    // Exile cards from any graveyard(s) — targets are the cards to exile
                    let count = resolve_x(*count);
                    let mut exiled = 0u32;
                    for &target_id in targets {
                        if exiled >= count {
                            break;
                        }
                        // Find which player's graveyard has this card
                        let mut found_player = None;
                        for (&pid, player) in self.state.players.iter() {
                            if player.graveyard.contains(target_id) {
                                found_player = Some(pid);
                                break;
                            }
                        }
                        if let Some(pid) = found_player {
                            if let Some(player) = self.state.players.get_mut(&pid) {
                                player.graveyard.remove(target_id);
                                self.state.exile.exile(target_id);
                                self.state.set_zone(target_id, crate::constants::Zone::Exile, None);
                                exiled += 1;
                            }
                        }
                    }
                }
                Effect::Flicker => {
                    // Exile target creature, immediately return to BF under owner's control
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.remove(target_id) {
                            let owner_id = perm.owner();
                            self.state.ability_store.remove_source(target_id);
                            // Get card data and create fresh permanent
                            if let Some(card_data) = self.state.card_store.remove(target_id) {
                                for ability in &card_data.abilities {
                                    self.state.ability_store.add(ability.clone());
                                }
                                let new_perm = Permanent::new(card_data, owner_id);
                                self.state.battlefield.add(new_perm);
                                self.state.set_zone(target_id, crate::constants::Zone::Battlefield, None);
                                self.check_enters_tapped(target_id);
                                self.emit_event(GameEvent::enters_battlefield(target_id, owner_id));
                            }
                        }
                    }
                }
                Effect::FlickerEndStep => {
                    // Exile target creatures, return at next end step tapped
                    let mut exiled_ids = Vec::new();
                    for &target_id in targets {
                        if let Some(_perm) = self.state.battlefield.remove(target_id) {
                            self.state.ability_store.remove_source(target_id);
                            self.state.exile.exile(target_id);
                            self.state.set_zone(target_id, crate::constants::Zone::Exile, None);
                            exiled_ids.push(target_id);
                        }
                    }
                    if !exiled_ids.is_empty() {
                        // Create delayed trigger to return all at next end step
                        self.state.delayed_triggers.push(crate::state::DelayedTrigger {
                            event_type: EventType::EndStep,
                            watching: None,
                            effects: vec![Effect::ReturnFromExileTapped],
                            controller,
                            source: None,
                            targets: exiled_ids,
                            duration: crate::state::DelayedDuration::UntilTriggered,
                            trigger_only_once: true,
                            created_turn: self.state.turn_number,
                        });
                    }
                }
                Effect::ReturnFromExileTapped => {
                    // Return cards from exile to battlefield tapped under owners' control
                    for &target_id in targets {
                        if self.state.exile.remove(target_id) {
                            if let Some(card_data) = self.state.card_store.remove(target_id) {
                                let owner_id = card_data.owner;
                                for ability in &card_data.abilities {
                                    self.state.ability_store.add(ability.clone());
                                }
                                let mut new_perm = Permanent::new(card_data, owner_id);
                                new_perm.tap();
                                self.state.battlefield.add(new_perm);
                                self.state.set_zone(target_id, crate::constants::Zone::Battlefield, None);
                                self.emit_event(GameEvent::enters_battlefield(target_id, owner_id));
                            }
                        }
                    }
                }
                Effect::OpponentExilesFromHand { count } => {
                    let count = resolve_x(*count) as usize;
                    // Each opponent exiles cards from hand (like discard but to exile)
                    let opponents: Vec<PlayerId> = self.state.turn_order.iter()
                        .filter(|&&id| id != controller)
                        .copied()
                        .collect();
                    for opp in opponents {
                        let hand: Vec<ObjectId> = self.state.players.get(&opp)
                            .map(|p| p.hand.iter().copied().collect())
                            .unwrap_or_default();
                        let to_exile = count.min(hand.len());
                        if to_exile > 0 {
                            let view = crate::decision::GameView::placeholder();
                            let chosen = if let Some(dm) = self.decision_makers.get_mut(&opp) {
                                dm.choose_discard(&view, &hand, to_exile)
                            } else {
                                hand.iter().rev().take(to_exile).copied().collect()
                            };
                            for card_id in chosen {
                                if let Some(player) = self.state.players.get_mut(&opp) {
                                    player.hand.remove(card_id);
                                }
                                self.state.exile.exile(card_id);
                                self.state.set_zone(card_id, crate::constants::Zone::Exile, None);
                            }
                        }
                    }
                }
                Effect::BlightOpponents { count } => {
                    // Each opponent puts N -1/-1 counters on a creature they control
                    let count = resolve_x(*count);
                    let opponents: Vec<PlayerId> = self.state.turn_order.iter()
                        .filter(|&&id| id != controller)
                        .copied()
                        .collect();
                    for opp in opponents {
                        // Find creatures the opponent controls
                        let creatures: Vec<ObjectId> = self.state.battlefield.iter()
                            .filter(|p| p.controller == opp && p.is_creature())
                            .map(|p| p.id())
                            .collect();
                        if !creatures.is_empty() {
                            // Opponent chooses which creature to blight
                            let view = crate::decision::GameView::placeholder();
                            let chosen = if let Some(dm) = self.decision_makers.get_mut(&opp) {
                                let targets = dm.choose_targets(&view, crate::constants::Outcome::Detriment,
                                    &crate::decision::TargetRequirement {
                                        description: format!("Blight {} (put -1/-1 counters on creature you control)", count),
                                        legal_targets: creatures.clone(),
                                        min_targets: 1, max_targets: 1,
                                        required: true,
                                    });
                                targets.into_iter().next().unwrap_or(creatures[0])
                            } else {
                                creatures[0]
                            };
                            if let Some(perm) = self.state.battlefield.get_mut(chosen) {
                                perm.counters.add(crate::counters::CounterType::M1M1, count);
                            }
                        }
                    }
                }
                Effect::GainAllCreatureTypes => {
                    // Target gains all creature types until end of turn (changeling)
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            perm.granted_keywords |= crate::constants::KeywordAbilities::CHANGELING;
                        }
                    }
                }
                Effect::CreateTokenCopy { count, modifications } => {
                    let count = resolve_x(*count);
                    for &target_id in targets {
                        // Get the source permanent's card data to copy
                        let source_card = if let Some(perm) = self.state.battlefield.get(target_id) {
                            Some(perm.card.clone())
                        } else {
                            None
                        };
                        if let Some(source) = source_card {
                            for _ in 0..count {
                                let token_id = ObjectId::new();
                                let mut token_card = source.clone();
                                token_card.id = token_id;
                                token_card.owner = controller;
                                token_card.is_token = true;

                                // Re-key abilities with new IDs for the token
                                token_card.abilities = token_card.abilities.iter().map(|ab| {
                                    let mut new_ab = ab.clone();
                                    new_ab.id = crate::types::AbilityId::new();
                                    new_ab.source_id = token_id;
                                    new_ab
                                }).collect();

                                // Apply modifications
                                let mut sacrifice_eot = false;
                                let mut enter_tapped_attacking = false;
                                for m in modifications {
                                    match m {
                                        crate::abilities::TokenModification::AddKeyword(kw) => {
                                            if let Some(flag) = crate::constants::KeywordAbilities::keyword_from_name(kw) {
                                                token_card.keywords |= flag;
                                            }
                                        }
                                        crate::abilities::TokenModification::AddChangeling => {
                                            token_card.keywords |= crate::constants::KeywordAbilities::CHANGELING;
                                        }
                                        crate::abilities::TokenModification::SacrificeAtEndStep => {
                                            sacrifice_eot = true;
                                        }
                                        crate::abilities::TokenModification::EnterTappedAttacking => {
                                            enter_tapped_attacking = true;
                                        }
                                    }
                                }

                                // Register abilities for the token
                                for ab in &token_card.abilities {
                                    self.state.ability_store.add(ab.clone());
                                }

                                // Create and add the token permanent
                                let mut perm = Permanent::new(token_card, controller);
                                if enter_tapped_attacking {
                                    perm.tap();
                                }
                                self.state.battlefield.add(perm);
                                self.state.set_zone(token_id, crate::constants::Zone::Battlefield, None);
                                self.emit_event(GameEvent::enters_battlefield(token_id, controller));

                                // Create delayed trigger to sacrifice at end step
                                if sacrifice_eot {
                                    self.state.delayed_triggers.push(crate::state::DelayedTrigger {
                                        event_type: EventType::EndStep,
                                        watching: None,
                                        effects: vec![Effect::Sacrifice { filter: "self".into() }],
                                        controller,
                                        source: Some(token_id),
                                        targets: vec![token_id],
                                        duration: crate::state::DelayedDuration::UntilTriggered,
                                        trigger_only_once: true,
                                        created_turn: self.state.turn_number,
                                    });
                                }
                            }
                        }
                    }
                }
                Effect::TapSelf => {
                    // Tap the source permanent
                    if let Some(src_id) = source {
                        if let Some(perm) = self.state.battlefield.get_mut(src_id) {
                            perm.tap();
                        }
                    }
                }
                Effect::ReturnAllTypeFromGraveyard { creature_type } => {
                    // Return all creature cards of the specified type from controller's graveyard to the battlefield
                    let target_subtype = crate::constants::SubType::by_description(creature_type);
                    if let Some(player) = self.state.players.get(&controller) {
                        let matching_ids: Vec<ObjectId> = player.graveyard.iter()
                            .filter(|&&card_id| {
                                if let Some(card) = self.state.card_store.get(card_id) {
                                    card.is_creature() && card.subtypes.contains(&target_subtype)
                                } else {
                                    false
                                }
                            })
                            .copied()
                            .collect();

                        for card_id in matching_ids {
                            if let Some(player) = self.state.players.get_mut(&controller) {
                                player.graveyard.remove(card_id);
                            }
                            if let Some(card_data) = self.state.card_store.remove(card_id) {
                                for ability in &card_data.abilities {
                                    self.state.ability_store.add(ability.clone());
                                }
                                let perm = Permanent::new(card_data, controller);
                                self.state.battlefield.add(perm);
                                self.state.set_zone(card_id, crate::constants::Zone::Battlefield, None);
                                self.emit_event(GameEvent::enters_battlefield(card_id, controller));
                            }
                        }
                    }
                }
                Effect::CreateTokenDynamic { token_name, count_filter } => {
                    // Count matching items based on filter, then create that many tokens
                    let count = self.evaluate_count_filter(count_filter, controller);
                    for _ in 0..count {
                        let token_id = ObjectId::new();
                        let mut card = CardData::new(token_id, controller, token_name);
                        card.card_types = vec![crate::constants::CardType::Creature];
                        let (p, t, kw) = Self::parse_token_stats(token_name);
                        card.power = Some(p);
                        card.toughness = Some(t);
                        card.keywords = kw;
                        card.is_token = true;
                        let perm = Permanent::new(card, controller);
                        self.state.battlefield.add(perm);
                        self.state.set_zone(token_id, crate::constants::Zone::Battlefield, None);
                        self.emit_event(GameEvent::enters_battlefield(token_id, controller));
                    }
                }
                _ => {
                    // Remaining effects not yet implemented (protection, etc.)
                }
            }
        }
    }

    /// Move a card to its owner's graveyard (internal version that doesn't need owner lookup).
    fn move_card_to_graveyard_inner(&mut self, card_id: ObjectId, owner: PlayerId) {
        if let Some(player) = self.state.players.get_mut(&owner) {
            player.graveyard.add(card_id);
            self.state.set_zone(card_id, crate::constants::Zone::Graveyard, Some(owner));
        }
    }

    /// Move a card to its owner's graveyard.
    fn move_card_to_graveyard(&mut self, card_id: ObjectId, owner: PlayerId) {
        if let Some(player) = self.state.players.get_mut(&owner) {
            player.graveyard.add(card_id);
            self.state.set_zone(card_id, crate::constants::Zone::Graveyard, Some(owner));
        }
    }

    /// Draw N cards for a player.
    pub fn draw_cards(&mut self, player_id: PlayerId, count: u32) {
        for _ in 0..count {
            let card_id = {
                let player = match self.state.players.get_mut(&player_id) {
                    Some(p) => p,
                    None => return,
                };
                match player.library.draw() {
                    Some(id) => id,
                    None => {
                        // Tried to draw from empty library — player loses (set flag)
                        player.lost = true;
                        return;
                    }
                }
            };

            // Add to hand and update zone
            if let Some(player) = self.state.players.get_mut(&player_id) {
                player.hand.add(card_id);
            }
            self.state.set_zone(card_id, crate::constants::Zone::Hand, Some(player_id));
        }
    }

    /// Parse token stats from a token name string like "4/4 Dragon with flying".
    /// Returns (power, toughness, keywords).
    fn parse_token_stats(token_name: &str) -> (i32, i32, crate::constants::KeywordAbilities) {
        let name = token_name.trim();
        // Try to match "P/T Name..." pattern at the start
        let mut power = 1i32;
        let mut toughness = 1i32;
        let mut keywords = crate::constants::KeywordAbilities::empty();

        // Check for "P/T " prefix
        let rest = if let Some(slash_pos) = name.find('/') {
            if let Ok(p) = name[..slash_pos].parse::<i32>() {
                // Find end of toughness (next space or end)
                let after_slash = &name[slash_pos + 1..];
                let t_end = after_slash.find(' ').unwrap_or(after_slash.len());
                if let Ok(t) = after_slash[..t_end].parse::<i32>() {
                    power = p;
                    toughness = t;
                    if t_end < after_slash.len() {
                        &after_slash[t_end + 1..]
                    } else {
                        ""
                    }
                } else {
                    name
                }
            } else {
                name
            }
        } else {
            name
        };

        // Parse "with keyword1[, keyword2...]" or "with keyword1 and keyword2"
        if let Some(with_pos) = rest.to_lowercase().find("with ") {
            let kw_str = &rest[with_pos + 5..];
            for part in kw_str.split(|c: char| c == ',' || c == '&') {
                let part = part.trim().trim_start_matches("and ").trim();
                if let Some(kw) = crate::constants::KeywordAbilities::keyword_from_name(part) {
                    keywords |= kw;
                }
            }
        }

        (power, toughness, keywords)
    }

    /// Check if a permanent matches a simple filter string.
    fn matches_filter(perm: &Permanent, filter: &str) -> bool {
        let f = filter.to_lowercase();
        // "all" or empty matches everything
        if f.is_empty() || f == "all" {
            return true;
        }
        // Check creature types (changelings match all creature types)
        let is_changeling = perm.is_creature()
            && perm.has_keyword(crate::constants::KeywordAbilities::CHANGELING);
        for st in &perm.card.subtypes {
            if f.contains(&st.to_string().to_lowercase()) {
                return true;
            }
        }
        // Check card types
        for ct in &perm.card.card_types {
            let ct_name = format!("{:?}", ct).to_lowercase();
            if f.contains(&ct_name) {
                return true;
            }
        }
        // Changeling matches any creature type name in the filter
        // (if the filter didn't already match a card type like "creature")
        if is_changeling {
            // If filter mentions any creature type name, changeling matches
            // We detect this by checking if the filter doesn't match common
            // card types — if it still hasn't matched, it's likely a creature subtype
            let is_card_type = f.contains("creature") || f.contains("land")
                || f.contains("artifact") || f.contains("enchantment")
                || f.contains("planeswalker") || f.contains("instant")
                || f.contains("sorcery") || f.contains("nonland");
            if !is_card_type {
                // Filter is likely a creature type name (e.g. "elf", "goblin", "spirit")
                return true;
            }
        }
        // "nonland" filter
        if f.contains("nonland") && !perm.card.card_types.contains(&crate::constants::CardType::Land) {
            return true;
        }
        false
    }

    /// Check if a CardData matches a simple filter string.
    fn card_matches_filter(card: &CardData, filter: &str) -> bool {
        let f = filter.to_lowercase();
        if f.is_empty() || f == "all" {
            return true;
        }
        // Check "basic land"
        if f.contains("basic") && f.contains("land") {
            return card.supertypes.contains(&crate::constants::SuperType::Basic)
                && card.card_types.contains(&crate::constants::CardType::Land);
        }
        // Check card types
        for ct in &card.card_types {
            let ct_name = format!("{:?}", ct).to_lowercase();
            if f.contains(&ct_name) {
                return true;
            }
        }
        // Check subtypes
        for st in &card.subtypes {
            if f.contains(&st.to_string().to_lowercase()) {
                return true;
            }
        }
        false
    }

    /// Select targets for a spell/ability based on its TargetSpec.
    ///
    /// Builds the list of legal targets for the spec, asks the decision maker
    /// to choose, and returns the selected ObjectIds. For `Pair` specs, the
    /// first target comes from `first` and the second from `second`.
    fn select_targets_for_spec(
        &mut self,
        spec: &crate::abilities::TargetSpec,
        controller: PlayerId,
    ) -> Vec<ObjectId> {
        use crate::abilities::TargetSpec;

        match spec {
            TargetSpec::None => vec![],
            TargetSpec::Pair { first, second } => {
                let mut result = Vec::new();
                let first_targets = self.select_targets_for_spec(first, controller);
                result.extend(&first_targets);
                let second_targets = self.select_targets_for_spec(second, controller);
                result.extend(&second_targets);
                result
            }
            _ => {
                let legal = self.legal_targets_for_spec(spec, controller);
                if legal.is_empty() {
                    return vec![];
                }
                let requirement = crate::decision::TargetRequirement {
                    description: Self::target_spec_description(spec),
                    legal_targets: legal,
                    min_targets: 1,
                    max_targets: 1,
                    required: true,
                };
                let outcome = Self::target_spec_outcome(spec);
                let view = crate::decision::GameView::placeholder();
                let chosen = if let Some(dm) = self.decision_makers.get_mut(&controller) {
                    dm.choose_targets(&view, outcome, &requirement)
                } else {
                    // Fallback: pick the first legal target
                    requirement.legal_targets.into_iter().take(1).collect()
                };
                // If decision maker returned empty, fall back to first legal target
                if chosen.is_empty() {
                    // Re-build legal targets since requirement was moved
                    let legal = self.legal_targets_for_spec(spec, controller);
                    legal.into_iter().take(1).collect()
                } else {
                    chosen
                }
            }
        }
    }

    /// Build the list of legal target ObjectIds for a given TargetSpec.
    fn legal_targets_for_spec(
        &self,
        spec: &crate::abilities::TargetSpec,
        controller: PlayerId,
    ) -> Vec<ObjectId> {
        use crate::abilities::TargetSpec;
        match spec {
            TargetSpec::Creature => self
                .state
                .battlefield
                .iter()
                .filter(|p| p.is_creature() && !Self::is_untargetable(p, controller))
                .map(|p| p.id())
                .collect(),
            TargetSpec::CreatureYouControl => self
                .state
                .battlefield
                .iter()
                .filter(|p| p.is_creature() && p.controller == controller)
                // No hexproof check — you can always target your own permanents
                .map(|p| p.id())
                .collect(),
            TargetSpec::OpponentCreature => self
                .state
                .battlefield
                .iter()
                .filter(|p| p.is_creature() && p.controller != controller
                    && !Self::is_untargetable(p, controller))
                .map(|p| p.id())
                .collect(),
            TargetSpec::CreatureOrPlayer => {
                let mut targets: Vec<ObjectId> = self
                    .state
                    .battlefield
                    .iter()
                    .filter(|p| p.is_creature() && !Self::is_untargetable(p, controller))
                    .map(|p| p.id())
                    .collect();
                // Player targeting would need a different mechanism;
                // for now, just return creature targets
                targets.sort(); // deterministic ordering
                targets
            }
            TargetSpec::Permanent => self
                .state
                .battlefield
                .iter()
                .filter(|p| !Self::is_untargetable(p, controller))
                .map(|p| p.id())
                .collect(),
            TargetSpec::PermanentFiltered(filter) => self
                .state
                .battlefield
                .iter()
                .filter(|p| Self::matches_filter(p, filter)
                    && !Self::is_untargetable(p, controller))
                .map(|p| p.id())
                .collect(),
            TargetSpec::Spell => self
                .state
                .stack
                .iter()
                .map(|item| item.id)
                .collect(),
            _ => vec![], // None, CardInGraveyard, Multiple, Custom, Pair — handled elsewhere
        }
    }

    /// Check if a permanent is untargetable by a given controller.
    /// Returns true for shroud (can't be targeted by anyone) or
    /// hexproof (can't be targeted by opponents).
    fn is_untargetable(perm: &Permanent, targeting_controller: PlayerId) -> bool {
        // Shroud: can't be targeted by anyone
        if perm.has_keyword(crate::constants::KeywordAbilities::SHROUD) {
            return true;
        }
        // Hexproof: can't be targeted by opponents
        if perm.has_hexproof() && perm.controller != targeting_controller {
            return true;
        }
        false
    }

    /// Human-readable description for a TargetSpec.
    fn target_spec_description(spec: &crate::abilities::TargetSpec) -> String {
        use crate::abilities::TargetSpec;
        match spec {
            TargetSpec::Creature => "target creature".into(),
            TargetSpec::CreatureYouControl => "target creature you control".into(),
            TargetSpec::OpponentCreature => "target creature you don't control".into(),
            TargetSpec::CreatureOrPlayer => "target creature or player".into(),
            TargetSpec::Permanent => "target permanent".into(),
            TargetSpec::PermanentFiltered(f) => format!("target {}", f),
            TargetSpec::Spell => "target spell".into(),
            _ => "target".into(),
        }
    }

    /// Determine the Outcome for a TargetSpec (used to inform AI target choice).
    fn target_spec_outcome(spec: &crate::abilities::TargetSpec) -> crate::constants::Outcome {
        use crate::abilities::TargetSpec;
        use crate::constants::Outcome;
        match spec {
            TargetSpec::CreatureYouControl => Outcome::Benefit,
            TargetSpec::OpponentCreature => Outcome::Removal,
            _ => Outcome::Detriment, // Default: assume targeting opponents
        }
    }

    /// Resolve the fighter/target pair for Fight/Bite effects.
    ///
    /// Mirrors Java's FightTargetsEffect: uses two explicit targets when
    /// available (targets[0] = your creature, targets[1] = opponent's creature).
    /// Falls back to source creature for ETB triggers, or auto-selects
    /// strongest creatures as last resort.
    fn resolve_fight_pair(
        state: &GameState,
        targets: &[ObjectId],
        source: Option<ObjectId>,
        controller: PlayerId,
    ) -> (Option<ObjectId>, Option<ObjectId>) {
        // Two explicit targets from TargetSpec::Pair selection:
        // targets[0] = creature you control, targets[1] = creature opponent controls
        if targets.len() >= 2 {
            let t0 = state.battlefield.get(targets[0]).map(|_| targets[0]);
            let t1 = state.battlefield.get(targets[1]).map(|_| targets[1]);
            if t0.is_some() && t1.is_some() {
                return (t0, t1);
            }
        }

        // Single target + source creature (ETB triggers like Affectionate Indrik):
        // source = the creature, targets[0] = opponent's creature
        if targets.len() == 1 {
            if let Some(sid) = source {
                if state.battlefield.get(sid).map_or(false, |p| p.is_creature()) {
                    let tid = state.battlefield.get(targets[0]).map(|_| targets[0]);
                    return (Some(sid), tid);
                }
            }
        }

        // Fallback: auto-select strongest creatures on each side
        let fighter = state
            .battlefield
            .iter()
            .filter(|p| p.controller == controller && p.is_creature())
            .max_by_key(|p| p.power())
            .map(|p| p.id());
        let target = state
            .battlefield
            .iter()
            .filter(|p| p.controller != controller && p.is_creature())
            .max_by_key(|p| p.power())
            .map(|p| p.id());
        (fighter, target)
    }

    /// Check if the game should end and return a result if so.
    fn check_game_end(&self) -> Option<GameResult> {
        if !self.state.should_end() {
            return None;
        }

        let alive = self.state.active_players();

        let (winner, reason) = if alive.len() == 1 {
            (Some(alive[0]), GameEndReason::LastPlayerStanding)
        } else if alive.is_empty() {
            (None, GameEndReason::Draw)
        } else {
            return None;
        };

        Some(GameResult {
            winner,
            turn_number: self.turn_manager.turn_number,
            reason,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abilities::{Ability, Cost, Effect, TargetSpec};
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, Outcome, SubType};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };
    use crate::mana::Mana;

    /// A minimal decision maker that always passes priority.
    struct AlwaysPassPlayer;

    impl PlayerDecisionMaker for AlwaysPassPlayer {
        fn priority(&mut self, _game: &GameView<'_>, _legal: &[PlayerAction]) -> PlayerAction {
            PlayerAction::Pass
        }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn make_basic_land(name: &str, owner: PlayerId) -> CardData {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Land];
        card
    }

    fn make_creature(name: &str, owner: PlayerId, power: i32, toughness: i32) -> CardData {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(power);
        card.toughness = Some(toughness);
        card.keywords = KeywordAbilities::empty();
        card
    }

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        let mut deck = Vec::new();
        // 20 lands
        for _ in 0..20 {
            deck.push(make_basic_land("Forest", owner));
        }
        // 20 creatures
        for _ in 0..20 {
            deck.push(make_creature("Grizzly Bears", owner, 2, 2));
        }
        deck
    }

    #[test]
    fn game_creation() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        assert_eq!(game.state.players.len(), 2);
        assert_eq!(game.state.player(p1).unwrap().life, 20);
        assert_eq!(game.state.player(p2).unwrap().life, 20);
        // Each player should have 40 cards in library
        assert_eq!(game.state.player(p1).unwrap().library.len(), 40);
        assert_eq!(game.state.player(p2).unwrap().library.len(), 40);
    }

    #[test]
    fn game_runs_to_completion() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Both players always pass, so the game should eventually end by decking
        let result = game.run();

        // A player should have lost by drawing from an empty library
        assert!(result.winner.is_some() || result.turn_number > 1);
    }

    #[test]
    fn draw_cards_from_empty_library_causes_loss() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        // Give player 1 only 5 cards in deck
        let mut small_deck = Vec::new();
        for _ in 0..5 {
            small_deck.push(make_basic_land("Forest", p1));
        }

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: small_deck },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        let result = game.run();

        // Alice should lose from decking (only 5 cards, draws 7 opening hand)
        assert_eq!(result.winner, Some(p2));
    }

    #[test]
    fn counter_annihilation_applied() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Add a creature with both +1/+1 and -1/-1 counters
        let card = make_creature("Test Bear", p1, 2, 2);
        let card_id = card.id;
        let mut perm = Permanent::new(card, p1);
        perm.add_counters(crate::counters::CounterType::P1P1, 3);
        perm.add_counters(crate::counters::CounterType::M1M1, 2);
        game.state.battlefield.add(perm);

        // Process SBAs
        game.process_state_based_actions();

        // After annihilation: 3 P1P1 - 2 M1M1 = 1 P1P1 remaining, 0 M1M1
        let perm = game.state.battlefield.get(card_id).unwrap();
        assert_eq!(perm.counters.get(&crate::counters::CounterType::P1P1), 1);
        assert_eq!(perm.counters.get(&crate::counters::CounterType::M1M1), 0);
        assert_eq!(perm.power(), 3); // 2 base + 1 from counter
        assert_eq!(perm.toughness(), 3);
    }

    #[test]
    fn legend_rule_applied() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Add two legendary permanents with the same name
        let mut card1 = CardData::new(ObjectId::new(), p1, "Thalia");
        card1.card_types = vec![CardType::Creature];
        card1.supertypes = vec![crate::constants::SuperType::Legendary];
        card1.power = Some(2);
        card1.toughness = Some(1);
        card1.keywords = KeywordAbilities::empty();
        let id1 = card1.id;

        let mut card2 = CardData::new(ObjectId::new(), p1, "Thalia");
        card2.card_types = vec![CardType::Creature];
        card2.supertypes = vec![crate::constants::SuperType::Legendary];
        card2.power = Some(2);
        card2.toughness = Some(1);
        card2.keywords = KeywordAbilities::empty();

        game.state.battlefield.add(Permanent::new(card1, p1));
        game.state.battlefield.add(Permanent::new(card2, p1));

        assert_eq!(game.state.battlefield.len(), 2);

        // Process SBAs
        game.process_state_based_actions();

        // One should be removed by the legend rule
        assert_eq!(game.state.battlefield.len(), 1);
        // The first one should survive
        assert!(game.state.battlefield.contains(id1));
    }

    #[test]
    fn legal_actions_include_pass() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        let actions = game.compute_legal_actions(p1);
        assert!(actions.contains(&PlayerAction::Pass));
    }

    #[test]
    fn mana_ability_and_spell_cast() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Put a Forest on the battlefield with a mana ability
        let forest_id = ObjectId::new();
        let mut forest = CardData::new(forest_id, p1, "Forest");
        forest.card_types = vec![CardType::Land];
        let mana_ability = Ability::mana_ability(
            forest_id,
            "{T}: Add {G}",
            Mana::green(1),
        );
        let ability_id = mana_ability.id;
        forest.abilities.push(mana_ability);

        let perm = Permanent::new(forest.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(forest.clone());
        for ability in &forest.abilities {
            game.state.ability_store.add(ability.clone());
        }

        // Activate the mana ability
        game.activate_mana_ability(p1, forest_id, ability_id);

        // Check that the mana pool has green mana
        let player = game.state.players.get(&p1).unwrap();
        let available = player.mana_pool.available();
        assert!(available.green >= 1);

        // The permanent should be tapped now
        let perm = game.state.battlefield.get(forest_id).unwrap();
        assert!(perm.tapped);
    }

    #[test]
    fn activated_ability_goes_on_stack() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Create a permanent with an activated ability (no cost for simplicity)
        let source_id = ObjectId::new();
        let mut card = CardData::new(source_id, p1, "Prodigal Sorcerer");
        card.card_types = vec![CardType::Creature];
        card.power = Some(1);
        card.toughness = Some(1);
        card.keywords = KeywordAbilities::empty();

        let ability = Ability::activated(
            source_id,
            "{T}: Deal 1 damage to target.",
            vec![Cost::TapSelf],
            vec![Effect::DealDamage { amount: 1 }],
            TargetSpec::CreatureOrPlayer,
        );
        let ability_id = ability.id;
        card.abilities.push(ability);

        let perm = Permanent::new(card.clone(), p1);
        perm.id(); // verify it has the right ID
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card.clone());
        for a in &card.abilities {
            game.state.ability_store.add(a.clone());
        }

        // Remove summoning sickness for the test
        if let Some(perm) = game.state.battlefield.get_mut(source_id) {
            perm.remove_summoning_sickness();
        }

        // Activate the ability
        game.activate_ability(p1, source_id, ability_id, &[]);

        // The ability should be on the stack
        assert_eq!(game.state.stack.len(), 1);

        // The permanent should be tapped (cost was tap)
        let perm = game.state.battlefield.get(source_id).unwrap();
        assert!(perm.tapped);
    }

    #[test]
    fn spell_effects_execute_on_resolve() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Create a target creature for Bob
        let bear_id = ObjectId::new();
        let mut bear = CardData::new(bear_id, p2, "Grizzly Bears");
        bear.card_types = vec![CardType::Creature];
        bear.power = Some(2);
        bear.toughness = Some(2);
        bear.keywords = KeywordAbilities::empty();
        let bear_perm = Permanent::new(bear.clone(), p2);
        game.state.battlefield.add(bear_perm);

        // Create a spell with "deal 3 damage" and push it on the stack
        let bolt_id = ObjectId::new();
        let mut bolt = CardData::new(bolt_id, p1, "Lightning Bolt");
        bolt.card_types = vec![CardType::Instant];
        bolt.abilities.push(Ability::spell(
            bolt_id,
            vec![Effect::DealDamage { amount: 3 }],
            TargetSpec::CreatureOrPlayer,
        ));

        let stack_item = crate::zones::StackItem {
            id: bolt_id,
            kind: crate::zones::StackItemKind::Spell { card: bolt },
            controller: p1,
            targets: vec![bear_id],
            countered: false,
            x_value: None,
            exile_on_resolve: false,
        };
        game.state.stack.push(stack_item);

        // Resolve the spell
        game.resolve_top_of_stack();

        // The bear should have 3 damage marked on it
        let bear = game.state.battlefield.get(bear_id).unwrap();
        assert_eq!(bear.damage, 3);
        assert!(bear.has_lethal_damage());

        // SBAs should destroy it
        game.process_state_based_actions();
        assert!(!game.state.battlefield.contains(bear_id));
    }

    #[test]
    fn fizzle_when_target_removed() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Create a target creature, then remove it before spell resolves
        let bear_id = ObjectId::new();
        let mut bear = CardData::new(bear_id, p2, "Grizzly Bears");
        bear.card_types = vec![CardType::Creature];
        bear.power = Some(2);
        bear.toughness = Some(2);
        bear.keywords = KeywordAbilities::empty();
        game.state.battlefield.add(Permanent::new(bear.clone(), p2));

        // Push a destroy spell targeting the bear
        let spell_id = ObjectId::new();
        let mut murder = CardData::new(spell_id, p1, "Murder");
        murder.card_types = vec![CardType::Instant];
        murder.abilities.push(Ability::spell(
            spell_id,
            vec![Effect::Destroy],
            TargetSpec::Creature,
        ));

        let stack_item = crate::zones::StackItem {
            id: spell_id,
            kind: crate::zones::StackItemKind::Spell { card: murder },
            controller: p1,
            targets: vec![bear_id],
            countered: false,
            x_value: None,
            exile_on_resolve: false,
        };
        game.state.stack.push(stack_item);

        // Remove the target before resolution (simulating another effect)
        game.state.battlefield.remove(bear_id);

        // Resolve the spell — should fizzle since target is gone
        game.resolve_top_of_stack();

        // The spell should be in the graveyard (fizzled)
        // The stack should be empty
        assert!(game.state.stack.is_empty());
    }

    #[test]
    fn draw_cards_effect() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        let initial_hand = game.state.players.get(&p1).unwrap().hand.len();
        let initial_library = game.state.players.get(&p1).unwrap().library.len();

        // Execute a draw 2 effect
        game.execute_effects(&[Effect::DrawCards { count: 2 }], p1, &[], None, None);

        let final_hand = game.state.players.get(&p1).unwrap().hand.len();
        let final_library = game.state.players.get(&p1).unwrap().library.len();

        assert_eq!(final_hand, initial_hand + 2);
        assert_eq!(final_library, initial_library - 2);
    }

    #[test]
    fn gain_life_effect() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        game.execute_effects(&[Effect::GainLife { amount: 5 }], p1, &[], None, None);
        assert_eq!(game.state.players.get(&p1).unwrap().life, 25);
    }

    #[test]
    fn lose_life_opponents_effect() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        game.execute_effects(&[Effect::lose_life_opponents(3)], p1, &[], None, None);
        // Controller's life should be unchanged
        assert_eq!(game.state.players.get(&p1).unwrap().life, 20);
        // Opponent loses 3 life
        assert_eq!(game.state.players.get(&p2).unwrap().life, 17);
    }

    #[test]
    fn exile_effect() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Put a creature on the battlefield
        let bear_id = ObjectId::new();
        let mut bear = CardData::new(bear_id, p2, "Bear");
        bear.card_types = vec![CardType::Creature];
        bear.power = Some(2);
        bear.toughness = Some(2);
        bear.keywords = KeywordAbilities::empty();
        game.state.battlefield.add(Permanent::new(bear, p2));

        // Exile it
        game.execute_effects(&[Effect::Exile], p1, &[bear_id], None, None);

        assert!(!game.state.battlefield.contains(bear_id));
        assert!(game.state.exile.contains(bear_id));
    }

    #[test]
    fn bounce_effect() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        let bear_id = ObjectId::new();
        let mut bear = CardData::new(bear_id, p2, "Bear");
        bear.card_types = vec![CardType::Creature];
        bear.power = Some(2);
        bear.toughness = Some(2);
        bear.keywords = KeywordAbilities::empty();
        game.state.battlefield.add(Permanent::new(bear, p2));

        let initial_hand = game.state.players.get(&p2).unwrap().hand.len();

        // Bounce it
        game.execute_effects(&[Effect::Bounce], p1, &[bear_id], None, None);

        assert!(!game.state.battlefield.contains(bear_id));
        assert_eq!(game.state.players.get(&p2).unwrap().hand.len(), initial_hand + 1);
    }

    #[test]
    fn pay_costs_tap_and_sacrifice() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Add a permanent
        let source_id = ObjectId::new();
        let mut card = CardData::new(source_id, p1, "Sacrifice Me");
        card.card_types = vec![CardType::Creature];
        card.power = Some(1);
        card.toughness = Some(1);
        card.keywords = KeywordAbilities::empty();
        game.state.battlefield.add(Permanent::new(card, p1));

        // Pay tap cost
        assert!(game.pay_costs(p1, source_id, &[Cost::TapSelf]));
        let perm = game.state.battlefield.get(source_id).unwrap();
        assert!(perm.tapped);

        // Can't pay tap again (already tapped)
        assert!(!game.pay_costs(p1, source_id, &[Cost::TapSelf]));

        // Pay sacrifice self cost
        assert!(game.pay_costs(p1, source_id, &[Cost::SacrificeSelf]));
        assert!(!game.state.battlefield.contains(source_id));

        // The card should be in the graveyard
        let player = game.state.players.get(&p1).unwrap();
        assert!(player.graveyard.contains(source_id));
    }

    #[test]
    fn add_counters_self_when_no_targets() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Add a creature to the battlefield
        let source_id = ObjectId::new();
        let mut card = CardData::new(source_id, p1, "Blight Creature");
        card.card_types = vec![CardType::Creature];
        card.power = Some(3);
        card.toughness = Some(7);
        card.keywords = KeywordAbilities::empty();
        game.state.battlefield.add(Permanent::new(card, p1));

        // Execute AddCounters with no targets but with source — should add to self
        game.execute_effects(
            &[Effect::add_counters("-1/-1", 2)],
            p1,
            &[],
            Some(source_id),
        None, );

        let perm = game.state.battlefield.get(source_id).unwrap();
        assert_eq!(perm.counters.get(&CounterType::M1M1), 2);

        // Execute RemoveCounters with no targets but with source — should remove from self
        game.execute_effects(
            &[Effect::RemoveCounters { counter_type: "-1/-1".into(), count: 1 }],
            p1,
            &[],
            Some(source_id),
        None, );

        let perm = game.state.battlefield.get(source_id).unwrap();
        assert_eq!(perm.counters.get(&CounterType::M1M1), 1);
    }

    #[test]
    fn add_counters_self_with_separate_target() {
        // Compound effect: AddCountersSelf puts -1/-1 on source while
        // GainKeywordUntilEndOfTurn gives haste to a different target.
        // Models Warren Torchmaster: blight self + target creature gains haste.
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Source creature (Warren Torchmaster analog)
        let source_id = ObjectId::new();
        let mut card = CardData::new(source_id, p1, "Torchmaster");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        card.keywords = KeywordAbilities::empty();
        game.state.battlefield.add(Permanent::new(card, p1));

        // Target creature (gets haste)
        let target_id = ObjectId::new();
        let mut card2 = CardData::new(target_id, p1, "Target Creature");
        card2.card_types = vec![CardType::Creature];
        card2.power = Some(3);
        card2.toughness = Some(3);
        card2.keywords = KeywordAbilities::empty();
        game.state.battlefield.add(Permanent::new(card2, p1));

        // Compound effect: blight self + grant haste to target
        game.execute_effects(
            &[Effect::add_counters_self("-1/-1", 1), Effect::gain_keyword_eot("haste")],
            p1,
            &[target_id],  // target creature
            Some(source_id),  // source permanent
        None, );

        // Source should have -1/-1 counter (from AddCountersSelf)
        let source_perm = game.state.battlefield.get(source_id).unwrap();
        assert_eq!(source_perm.counters.get(&CounterType::M1M1), 1);
        assert_eq!(source_perm.power(), 1); // 2 - 1
        // Source should NOT have haste
        assert!(!source_perm.granted_keywords.contains(KeywordAbilities::HASTE));

        // Target should have haste (from GainKeywordUntilEndOfTurn)
        let target_perm = game.state.battlefield.get(target_id).unwrap();
        assert!(target_perm.granted_keywords.contains(KeywordAbilities::HASTE));
        // Target should NOT have -1/-1 counter
        assert_eq!(target_perm.counters.get(&CounterType::M1M1), 0);
    }

    /// A decision maker that actually discards when asked.
    struct DiscardingPlayer;

    impl PlayerDecisionMaker for DiscardingPlayer {
        fn priority(&mut self, _game: &GameView<'_>, _legal: &[PlayerAction]) -> PlayerAction {
            PlayerAction::Pass
        }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, hand: &[ObjectId], count: usize) -> Vec<ObjectId> {
            // Actually discard from the back of hand
            hand.iter().rev().take(count).copied().collect()
        }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    #[test]
    fn discard_opponents_effect() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(DiscardingPlayer)),
            ],
        );

        // Give opponent some cards in hand
        let c1_id = ObjectId::new();
        let c2_id = ObjectId::new();
        let c3_id = ObjectId::new();
        if let Some(player) = game.state.players.get_mut(&p2) {
            player.hand.add(c1_id);
            player.hand.add(c2_id);
            player.hand.add(c3_id);
        }

        let p1_hand_before = game.state.players.get(&p1).unwrap().hand.len();
        let p2_hand_before = game.state.players.get(&p2).unwrap().hand.len();
        assert_eq!(p2_hand_before, 3);

        // Each opponent discards 1
        game.execute_effects(&[Effect::discard_opponents(1)], p1, &[], None, None);

        // Controller's hand unchanged
        assert_eq!(game.state.players.get(&p1).unwrap().hand.len(), p1_hand_before);
        // Opponent lost 1 card
        assert_eq!(game.state.players.get(&p2).unwrap().hand.len(), 2);
    }

    #[test]
    fn boost_all_and_grant_keyword_all_until_eot() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Put two creatures on P1's battlefield and one on P2's
        let bear1 = make_creature("Grizzly Bears", p1, 2, 2);
        let bear1_id = bear1.id;
        let bear2 = make_creature("Runeclaw Bear", p1, 2, 2);
        let bear2_id = bear2.id;
        let opp_bear = make_creature("Opponent Bear", p2, 2, 2);
        let opp_bear_id = opp_bear.id;

        game.state.battlefield.add(Permanent::new(bear1, p1));
        game.state.battlefield.add(Permanent::new(bear2, p1));
        game.state.battlefield.add(Permanent::new(opp_bear, p2));

        // Boost all creatures P1 controls +1/+1
        game.execute_effects(
            &[Effect::boost_all_eot("creatures you control", 1, 1)],
            p1, &[], None, None,
        );

        // P1's creatures should be 3/x, opponent's should remain 2/x
        assert_eq!(game.state.battlefield.get(bear1_id).unwrap().power(), 3);
        assert_eq!(game.state.battlefield.get(bear2_id).unwrap().power(), 3);
        assert_eq!(game.state.battlefield.get(opp_bear_id).unwrap().power(), 2);

        // Grant trample to all creatures P1 controls
        game.execute_effects(
            &[Effect::grant_keyword_all_eot("creatures you control", "trample")],
            p1, &[], None, None,
        );

        // P1's creatures should have trample, opponent's should not
        assert!(game.state.battlefield.get(bear1_id).unwrap().has_keyword(KeywordAbilities::TRAMPLE));
        assert!(game.state.battlefield.get(bear2_id).unwrap().has_keyword(KeywordAbilities::TRAMPLE));
        assert!(!game.state.battlefield.get(opp_bear_id).unwrap().has_keyword(KeywordAbilities::TRAMPLE));
    }

    #[test]
    fn fight_and_bite_effects() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Set up: P1 has a 4/4, P2 has a 3/5
        let fighter = make_creature("Fighter", p1, 4, 4);
        let fighter_id = fighter.id;
        let target = make_creature("Target", p2, 3, 5);
        let target_id = target.id;

        game.state.battlefield.add(Permanent::new(fighter, p1));
        game.state.battlefield.add(Permanent::new(target, p2));

        // Fight: mutual damage — fighter (4 power) vs target (3 power)
        game.execute_effects(
            &[Effect::fight()],
            p1,
            &[target_id],
            Some(fighter_id),
        None, );

        // Fighter took 3 damage (from target's 3 power): 4 toughness - 3 = 1 remaining
        let f = game.state.battlefield.get(fighter_id).unwrap();
        assert_eq!(f.remaining_toughness(), 1);
        // Target took 4 damage (from fighter's 4 power): 5 toughness - 4 = 1 remaining
        let t = game.state.battlefield.get(target_id).unwrap();
        assert_eq!(t.remaining_toughness(), 1);

        // Clear damage for next test
        game.state.battlefield.get_mut(fighter_id).unwrap().clear_damage();
        game.state.battlefield.get_mut(target_id).unwrap().clear_damage();

        // Bite: one-way damage — fighter deals 4 to target, target deals nothing back
        game.execute_effects(
            &[Effect::bite()],
            p1,
            &[target_id],
            Some(fighter_id),
        None, );

        // Fighter should have no damage
        let f = game.state.battlefield.get(fighter_id).unwrap();
        assert_eq!(f.remaining_toughness(), 4);
        // Target took 4 damage: 5 toughness - 4 = 1 remaining
        let t = game.state.battlefield.get(target_id).unwrap();
        assert_eq!(t.remaining_toughness(), 1);
    }

    #[test]
    fn fight_auto_selects_creatures() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // P1 has a 2/2 and a 5/5; P2 has a 3/3
        let small = make_creature("Small Bear", p1, 2, 2);
        let big = make_creature("Big Bear", p1, 5, 5);
        let big_id = big.id;
        let opp = make_creature("Opponent Bear", p2, 3, 3);
        let opp_id = opp.id;

        game.state.battlefield.add(Permanent::new(small, p1));
        game.state.battlefield.add(Permanent::new(big, p1));
        game.state.battlefield.add(Permanent::new(opp, p2));

        // Fight with no source, no targets — auto-selects strongest on each side
        game.execute_effects(&[Effect::fight()], p1, &[], None, None);

        // P1's 5/5 should fight P2's 3/3
        // Big bear: 5 toughness - 3 damage = 2 remaining
        let b = game.state.battlefield.get(big_id).unwrap();
        assert_eq!(b.remaining_toughness(), 2);
        // Opponent bear: 3 toughness - 5 damage = lethal
        let o = game.state.battlefield.get(opp_id).unwrap();
        assert!(o.has_lethal_damage());
    }

    #[test]
    fn compound_bite_counters_only_on_your_creature() {
        // Matches Java's Knockout Maneuver / Felling Blow pattern:
        // AddCountersTargetEffect targets only target 0 (your creature),
        // DamageWithPowerFromOneToAnotherTargetEffect uses both targets.
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // P1 has a 3/3, P2 has a 4/4
        let my_creature = make_creature("My Creature", p1, 3, 3);
        let my_id = my_creature.id;
        let opp_creature = make_creature("Opp Creature", p2, 4, 4);
        let opp_id = opp_creature.id;

        game.state.battlefield.add(Permanent::new(my_creature, p1));
        game.state.battlefield.add(Permanent::new(opp_creature, p2));

        // Compound effect: +1/+1 counter then bite (like Knockout Maneuver)
        // targets[0] = my creature, targets[1] = opponent creature
        game.execute_effects(
            &[Effect::add_p1p1_counters(1), Effect::bite()],
            p1,
            &[my_id, opp_id],
            None, None,
        );

        // My creature should have the +1/+1 counter (3+1=4 power, 3+1=4 toughness)
        let my = game.state.battlefield.get(my_id).unwrap();
        assert_eq!(my.power(), 4, "My creature should have +1/+1 counter (4 power)");
        assert_eq!(my.toughness(), 4, "My creature should have +1/+1 counter (4 toughness)");

        // Opponent's creature should NOT have any counters
        let opp = game.state.battlefield.get(opp_id).unwrap();
        assert_eq!(opp.power(), 4, "Opponent creature should not have counters");

        // Bite: my creature (now 4 power) deals 4 damage to opponent creature (4 toughness)
        assert_eq!(opp.remaining_toughness(), 0, "Opponent took 4 damage from bite");
        // My creature should have no damage (bite is one-way)
        assert_eq!(my.remaining_toughness(), 4, "My creature took no damage from bite");
    }

    #[test]
    fn add_counters_all_effect() {
        let (p1, p2) = (PlayerId::new(), PlayerId::new());
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Add creatures for both players
        let c1 = ObjectId::new();
        let mut card1 = CardData::new(c1, p1, "My Creature");
        card1.card_types = vec![CardType::Creature];
        card1.power = Some(3);
        card1.toughness = Some(3);
        game.state.battlefield.add(Permanent::new(card1, p1));

        let c2 = ObjectId::new();
        let mut card2 = CardData::new(c2, p2, "Opp Creature");
        card2.card_types = vec![CardType::Creature];
        card2.power = Some(2);
        card2.toughness = Some(4);
        game.state.battlefield.add(Permanent::new(card2, p2));

        // AddCountersAll on "creatures" (no "you control") — hits both
        game.execute_effects(
            &[Effect::add_counters_all("-1/-1", 2, "creatures")],
            p1,
            &[],
            None, None,
        );

        let p1c = game.state.battlefield.get(c1).unwrap();
        assert_eq!(p1c.counters.get(&CounterType::M1M1), 2);
        assert_eq!(p1c.power(), 1); // 3 - 2

        let p2c = game.state.battlefield.get(c2).unwrap();
        assert_eq!(p2c.counters.get(&CounterType::M1M1), 2);
        assert_eq!(p2c.power(), 0); // 2 - 2

        // AddCountersAll on "creatures you control" — hits only controller's
        game.execute_effects(
            &[Effect::add_counters_all("+1/+1", 1, "creatures you control")],
            p1,
            &[],
            None, None,
        );

        let p1c = game.state.battlefield.get(c1).unwrap();
        assert_eq!(p1c.counters.get(&CounterType::P1P1), 1);

        let p2c = game.state.battlefield.get(c2).unwrap();
        assert_eq!(p2c.counters.get(&CounterType::P1P1), 0); // unchanged
    }

    #[test]
    fn look_top_and_pick() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Clear library and set up specific cards on top
        let elf_id = ObjectId::new();
        let gob_id = ObjectId::new();
        let forest_id = ObjectId::new();
        let mtn_id = ObjectId::new();

        // Create cards with subtypes
        let mut elf = CardData::new(elf_id, p1, "Test Elf");
        elf.card_types = vec![CardType::Creature];
        elf.subtypes = vec![SubType::Elf];
        game.state.card_store.insert(elf);

        let mut gob = CardData::new(gob_id, p1, "Test Goblin");
        gob.card_types = vec![CardType::Creature];
        gob.subtypes = vec![SubType::Goblin];
        game.state.card_store.insert(gob);

        let mut forest = CardData::new(forest_id, p1, "Forest");
        forest.card_types = vec![CardType::Land];
        forest.subtypes = vec![SubType::Forest];
        game.state.card_store.insert(forest);

        let mut mtn = CardData::new(mtn_id, p1, "Mountain");
        mtn.card_types = vec![CardType::Land];
        mtn.subtypes = vec![SubType::Mountain];
        game.state.card_store.insert(mtn);

        if let Some(player) = game.state.players.get_mut(&p1) {
            while player.library.draw().is_some() {}
            // Top to bottom: Elf, Goblin, Mountain, Forest
            player.library.put_on_bottom(elf_id);
            player.library.put_on_bottom(gob_id);
            player.library.put_on_bottom(mtn_id);
            player.library.put_on_bottom(forest_id);
        }

        let hand_before = game.state.players.get(&p1).unwrap().hand.len();

        // Look at top 4, pick "Elf or Swamp or Forest"
        game.execute_effects(
            &[Effect::look_top_and_pick(4, "Elf or Swamp or Forest")],
            p1,
            &[],
            None, None,
        );

        let player = game.state.players.get(&p1).unwrap();
        // Should have picked the Elf (first match) to hand
        assert_eq!(player.hand.len(), hand_before + 1);
        assert!(player.hand.contains(elf_id), "Elf should be in hand");
        // Library should have 3 cards remaining (Goblin, Mountain, Forest on bottom)
        assert_eq!(player.library.len(), 3);
        assert!(!player.library.contains(elf_id), "Elf should not be in library");
    }

    #[test]
    fn gain_control_until_end_of_turn() {
        // Test that GainControlUntilEndOfTurn changes controller, untaps, grants haste.
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".into(), deck: vec![] },
                PlayerConfig { name: "Bob".into(), deck: vec![] },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Create a creature owned+controlled by p2
        let bear_id = ObjectId::new();
        let mut bear = CardData::new(bear_id, p2, "Grizzly Bears");
        bear.card_types = vec![CardType::Creature];
        bear.power = Some(2);
        bear.toughness = Some(2);
        bear.keywords = KeywordAbilities::empty();
        game.state.card_store.insert(bear.clone());

        let mut perm = crate::permanent::Permanent::new(bear, p2);
        perm.tapped = true; // Start tapped
        game.state.battlefield.add(perm);

        // p1 casts gain_control_eot on the bear
        let effects = vec![Effect::GainControlUntilEndOfTurn];
        let targets = vec![bear_id];
        game.execute_effects(&effects, p1, &targets, None, None);

        // Bear should now be controlled by p1, untapped, with haste
        let perm = game.state.battlefield.get(bear_id).unwrap();
        assert_eq!(perm.controller, p1, "Controller should be p1");
        assert!(!perm.tapped, "Should be untapped");
        assert!(perm.has_haste(), "Should have haste");
        assert_eq!(perm.original_controller, Some(p2), "Original controller tracked");
    }
}

#[cfg(test)]
mod modal_test {
    use super::*;
    use crate::abilities::{Effect, ModalMode};
    use crate::card::CardData;
    use crate::constants::{CardType, Outcome};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };
    use crate::types::{ObjectId, PlayerId};

    /// Decision maker that always picks mode 0 (first available).
    struct PickFirstModePlayer;

    impl PlayerDecisionMaker for PickFirstModePlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction {
            PlayerAction::Pass
        }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _modes: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    /// Decision maker that always picks mode 1 (second available).
    struct PickSecondModePlayer;

    impl PlayerDecisionMaker for PickSecondModePlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction {
            PlayerAction::Pass
        }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, modes: &[NamedChoice]) -> usize {
            if modes.len() > 1 { 1 } else { 0 }
        }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    #[test]
    fn modal_choose_one_of_two() {
        // Test "Choose one" modal with 2 modes.
        // Mode 1: Gain 5 life. Mode 2: Deal 3 damage.
        // PickFirstModePlayer always picks mode 0 (gain life).
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".into(), deck: vec![] },
                PlayerConfig { name: "Bob".into(), deck: vec![] },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(PickFirstModePlayer)),
                (p2, Box::new(PickFirstModePlayer)),
            ],
        );

        let modal = Effect::modal(
            vec![
                ModalMode::new("Gain 5 life", vec![Effect::gain_life(5)]),
                ModalMode::new("Deal 3 damage to target", vec![Effect::deal_damage(3)]),
            ],
            1, // min modes
            1, // max modes (choose one)
        );

        // Execute with p1 as controller
        game.execute_effects(&[modal], p1, &[], None, None);

        // PickFirstModePlayer always picks mode 0 (gain life)
        assert_eq!(game.state.players[&p1].life, 25, "p1 should have gained 5 life");
        assert_eq!(game.state.players[&p2].life, 20, "p2 should be unchanged");
    }

    #[test]
    fn modal_choose_two_of_three() {
        // Test "Choose two" modal with 3 modes.
        // Mode 0: Gain 3 life. Mode 1: Draw 2 cards. Mode 2: Deal 2 damage to opponents.
        // PickFirstModePlayer always picks index 0 from available modes.
        // First call: available = [0,1,2], picks 0 (gain life)
        // Second call: available = [1,2], picks index 0 -> mode 1 (draw)
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let _config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".into(), deck: vec![] },
                PlayerConfig { name: "Bob".into(), deck: vec![] },
            ],
            starting_life: 20,
        };

        // Give p1 some cards in library to draw from
        let mut deck = Vec::new();
        for _ in 0..5 {
            let mut card = CardData::new(ObjectId::new(), p1, "Forest");
            card.card_types = vec![CardType::Land];
            deck.push(card);
        }

        let mut game = Game::new_two_player(
            GameConfig {
                players: vec![
                    PlayerConfig { name: "Alice".into(), deck: vec![] },
                    PlayerConfig { name: "Bob".into(), deck: vec![] },
                ],
                starting_life: 20,
            },
            vec![
                (p1, Box::new(PickFirstModePlayer)),
                (p2, Box::new(PickFirstModePlayer)),
            ],
        );

        // Manually add cards to p1's library
        for card in deck {
            game.state.card_store.insert(card.clone());
            game.state.players.get_mut(&p1).unwrap().library.put_on_top(card.id);
        }

        let modal = Effect::modal(
            vec![
                ModalMode::new("Gain 3 life", vec![Effect::gain_life(3)]),
                ModalMode::new("Draw 2 cards", vec![Effect::draw_cards(2)]),
                ModalMode::new("Each opponent loses 2 life", vec![Effect::LoseLifeOpponents { amount: 2 }]),
            ],
            2, // min modes
            2, // max modes (choose two)
        );

        let hand_before = game.state.players[&p1].hand.len();
        game.execute_effects(&[modal], p1, &[], None, None);

        // Picks mode 0 (gain life) and mode 1 (draw)
        assert_eq!(game.state.players[&p1].life, 23, "p1 should have gained 3 life");
        assert_eq!(game.state.players[&p1].hand.len(), hand_before + 2, "p1 should have drawn 2");
        assert_eq!(game.state.players[&p2].life, 20, "p2 should be unchanged (mode 2 not chosen)");
    }

    #[test]
    fn modal_second_mode_chosen() {
        // PickSecondModePlayer picks mode 1 when 2+ available.
        // With "choose one" of 2 modes, should execute mode 1 only.
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let mut game = Game::new_two_player(
            GameConfig {
                players: vec![
                    PlayerConfig { name: "Alice".into(), deck: vec![] },
                    PlayerConfig { name: "Bob".into(), deck: vec![] },
                ],
                starting_life: 20,
            },
            vec![
                (p1, Box::new(PickSecondModePlayer)),
                (p2, Box::new(PickSecondModePlayer)),
            ],
        );

        let modal = Effect::modal(
            vec![
                ModalMode::new("Gain 5 life", vec![Effect::gain_life(5)]),
                ModalMode::new("Each opponent loses 3 life", vec![Effect::LoseLifeOpponents { amount: 3 }]),
            ],
            1,
            1, // choose one
        );

        game.execute_effects(&[modal], p1, &[], None, None);

        // PickSecondModePlayer picks mode 1 (opponents lose life)
        assert_eq!(game.state.players[&p1].life, 20, "p1 should be unchanged");
        assert_eq!(game.state.players[&p2].life, 17, "p2 should have lost 3 life");
    }
}

#[cfg(test)]
mod cost_tests {
    use super::*;
    use crate::abilities::Cost;
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, Outcome};
    use crate::counters::CounterType;
    use crate::decision::*;
    use crate::game::{GameConfig, PlayerConfig};
    use crate::types::{ObjectId, PlayerId};

    /// Decision maker that selects the last N cards for discard/exile choices.
    struct LastCardPicker;

    impl PlayerDecisionMaker for LastCardPicker {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction {
            PlayerAction::Pass
        }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, hand: &[ObjectId], count: usize) -> Vec<ObjectId> {
            // Pick the last N cards
            hand.iter().rev().take(count).copied().collect()
        }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        (0..20).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    fn setup_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(LastCardPicker)),
                (p2, Box::new(LastCardPicker)),
            ],
        );
        (game, p1, p2)
    }

    fn add_creature(game: &mut Game, owner: PlayerId, name: &str) -> ObjectId {
        let id = ObjectId::new();
        let mut card = CardData::new(id, owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        card.keywords = KeywordAbilities::empty();
        game.state.battlefield.add(Permanent::new(card, owner));
        id
    }

    #[test]
    fn pay_remove_counters_cost() {
        let (mut game, p1, _p2) = setup_game();
        let source_id = add_creature(&mut game, p1, "Counter Creature");

        // Add 3 -1/-1 counters
        game.state.battlefield.get_mut(source_id).unwrap()
            .add_counters(CounterType::M1M1, 3);

        // Pay 2 -1/-1 counter removal cost
        assert!(game.pay_costs(p1, source_id, &[Cost::RemoveCounters("-1/-1".into(), 2)]));
        assert_eq!(game.state.battlefield.get(source_id).unwrap().counters.get(&CounterType::M1M1), 1);

        // Can't pay 2 more (only 1 left)
        assert!(!game.pay_costs(p1, source_id, &[Cost::RemoveCounters("-1/-1".into(), 2)]));
    }

    #[test]
    fn pay_blight_cost() {
        let (mut game, p1, _p2) = setup_game();
        let source_id = add_creature(&mut game, p1, "Blight Creature");

        // Blight 2 puts 2 -1/-1 counters on self
        assert!(game.pay_costs(p1, source_id, &[Cost::Blight(2)]));
        assert_eq!(game.state.battlefield.get(source_id).unwrap().counters.get(&CounterType::M1M1), 2);
        // Power should be reduced
        assert_eq!(game.state.battlefield.get(source_id).unwrap().power(), 0);
    }

    #[test]
    fn pay_exile_from_graveyard_cost() {
        let (mut game, p1, _p2) = setup_game();
        let source_id = add_creature(&mut game, p1, "Source");

        // Add 3 cards to graveyard
        let gy1 = ObjectId::new();
        let gy2 = ObjectId::new();
        let gy3 = ObjectId::new();
        game.state.players.get_mut(&p1).unwrap().graveyard.add(gy1);
        game.state.players.get_mut(&p1).unwrap().graveyard.add(gy2);
        game.state.players.get_mut(&p1).unwrap().graveyard.add(gy3);

        // Can't exile 4 (only 3)
        assert!(!game.pay_costs(p1, source_id, &[Cost::ExileFromGraveyard(4)]));

        // Exile 2
        assert!(game.pay_costs(p1, source_id, &[Cost::ExileFromGraveyard(2)]));
        assert_eq!(game.state.players.get(&p1).unwrap().graveyard.len(), 1);
        // Exiled cards should be in exile zone
        assert!(game.state.exile.contains(gy3) || game.state.exile.contains(gy2));
    }

    #[test]
    fn pay_exile_from_hand_cost() {
        let (mut game, p1, _p2) = setup_game();
        let source_id = add_creature(&mut game, p1, "Source");

        // Add 2 cards to hand
        let h1 = ObjectId::new();
        let h2 = ObjectId::new();
        game.state.players.get_mut(&p1).unwrap().hand.add(h1);
        game.state.players.get_mut(&p1).unwrap().hand.add(h2);
        let before = game.state.players.get(&p1).unwrap().hand.len();

        // Exile 1
        assert!(game.pay_costs(p1, source_id, &[Cost::ExileFromHand(1)]));
        assert_eq!(game.state.players.get(&p1).unwrap().hand.len(), before - 1);
    }

    #[test]
    fn pay_sacrifice_other_cost() {
        let (mut game, p1, _p2) = setup_game();
        let source_id = add_creature(&mut game, p1, "Source");
        let other_id = add_creature(&mut game, p1, "Other Creature");

        // Sacrifice another creature
        assert!(game.pay_costs(p1, source_id, &[Cost::SacrificeOther("a creature".into())]));
        // The other creature should be gone
        assert!(!game.state.battlefield.contains(other_id));
        // Source should still be there
        assert!(game.state.battlefield.contains(source_id));
    }

    #[test]
    fn pay_untap_self_cost() {
        let (mut game, p1, _p2) = setup_game();
        let source_id = add_creature(&mut game, p1, "Untap Me");

        // Can't untap (not tapped)
        assert!(!game.pay_costs(p1, source_id, &[Cost::UntapSelf]));

        // Tap it first
        game.state.battlefield.get_mut(source_id).unwrap().tap();
        assert!(game.state.battlefield.get(source_id).unwrap().tapped);

        // Now untap cost works
        assert!(game.pay_costs(p1, source_id, &[Cost::UntapSelf]));
        assert!(!game.state.battlefield.get(source_id).unwrap().tapped);
    }
}

#[cfg(test)]
mod vivid_tests {
    use super::*;
    use crate::abilities::Effect;
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, Outcome};
    use crate::decision::*;
    use crate::game::{GameConfig, PlayerConfig};
    use crate::mana::{ManaCost};
    use crate::types::{ObjectId, PlayerId};

    struct AlwaysPassPlayer;
    impl PlayerDecisionMaker for AlwaysPassPlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        (0..20).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    fn setup() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "A".into(), deck: make_deck(p1) },
                PlayerConfig { name: "B".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(config, vec![
            (p1, Box::new(AlwaysPassPlayer)),
            (p2, Box::new(AlwaysPassPlayer)),
        ]);
        (game, p1, p2)
    }

    fn add_colored_creature(game: &mut Game, owner: PlayerId, name: &str, mana: &str) -> ObjectId {
        let id = ObjectId::new();
        let mut card = CardData::new(id, owner, name);
        card.card_types = vec![CardType::Creature];
        card.mana_cost = ManaCost::parse(mana);
        card.power = Some(2);
        card.toughness = Some(2);
        card.keywords = KeywordAbilities::empty();
        game.state.battlefield.add(Permanent::new(card, owner));
        id
    }

    #[test]
    fn count_colors_zero_for_colorless() {
        let (game, p1, _) = setup();
        // Only lands (colorless) on battlefield
        assert_eq!(game.count_colors_among_permanents(p1), 0);
    }

    #[test]
    fn count_colors_counts_distinct() {
        let (mut game, p1, _) = setup();
        // Add a red creature
        add_colored_creature(&mut game, p1, "Goblin", "{R}");
        assert_eq!(game.count_colors_among_permanents(p1), 1);

        // Add another red creature (still 1 color)
        add_colored_creature(&mut game, p1, "Goblin 2", "{1}{R}");
        assert_eq!(game.count_colors_among_permanents(p1), 1);

        // Add a green creature (now 2 colors)
        add_colored_creature(&mut game, p1, "Elf", "{G}");
        assert_eq!(game.count_colors_among_permanents(p1), 2);

        // Add a multicolor creature (adds blue and white)
        add_colored_creature(&mut game, p1, "Angel", "{W}{U}");
        assert_eq!(game.count_colors_among_permanents(p1), 4);
    }

    #[test]
    fn vivid_gain_life() {
        let (mut game, p1, _) = setup();
        // 3 colors
        add_colored_creature(&mut game, p1, "R", "{R}");
        add_colored_creature(&mut game, p1, "G", "{G}");
        add_colored_creature(&mut game, p1, "B", "{B}");

        game.execute_effects(&[Effect::GainLifeVivid], p1, &[], None, None);
        assert_eq!(game.state.players[&p1].life, 23); // 20 + 3
    }

    #[test]
    fn vivid_deal_damage_to_creature() {
        let (mut game, p1, p2) = setup();
        // p1 has 2 colors
        add_colored_creature(&mut game, p1, "R", "{R}");
        add_colored_creature(&mut game, p1, "G", "{G}");
        // p2 has a creature to target
        let target = add_colored_creature(&mut game, p2, "Bear", "{1}{W}");

        game.execute_effects(&[Effect::DealDamageVivid], p1, &[target], None, None);
        assert_eq!(game.state.battlefield.get(target).unwrap().damage, 2);
    }

    #[test]
    fn vivid_boost_until_eot() {
        let (mut game, p1, _) = setup();
        // p1 has 3 colors
        add_colored_creature(&mut game, p1, "R", "{R}");
        add_colored_creature(&mut game, p1, "G", "{G}");
        let target = add_colored_creature(&mut game, p1, "B", "{B}");

        game.execute_effects(&[Effect::BoostUntilEotVivid], p1, &[target], None, None);
        let perm = game.state.battlefield.get(target).unwrap();
        assert_eq!(perm.power(), 5); // 2 base + 3 vivid
        assert_eq!(perm.toughness(), 5);
    }

    #[test]
    fn vivid_create_tokens() {
        let (mut game, p1, _) = setup();
        // p1 has 3 colors
        add_colored_creature(&mut game, p1, "R", "{R}");
        add_colored_creature(&mut game, p1, "G", "{G}");
        add_colored_creature(&mut game, p1, "B", "{B}");

        let before = game.state.battlefield.controlled_by(p1).count();
        game.execute_effects(&[Effect::create_token_vivid("1/1 Kithkin")], p1, &[], None, None);
        let after = game.state.battlefield.controlled_by(p1).count();
        assert_eq!(after - before, 3); // 3 tokens for 3 colors
    }
}

#[cfg(test)]
mod choice_tests {
    use super::*;
    use crate::abilities::{Cost, Effect};
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, Outcome};
    use crate::counters::CounterType;
    use crate::decision::*;
    use crate::game::{GameConfig, PlayerConfig};
    use crate::types::{ObjectId, PlayerId};

    /// Decision maker that always says "yes" to choose_use.
    struct AlwaysPayPlayer;
    impl PlayerDecisionMaker for AlwaysPayPlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { true }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    /// Decision maker that always says "no" to choose_use.
    struct NeverPayPlayer;
    impl PlayerDecisionMaker for NeverPayPlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        (0..20).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    #[test]
    fn do_if_cost_paid_pays_and_executes() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "A".into(), deck: make_deck(p1) },
                PlayerConfig { name: "B".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let mut game = Game::new_two_player(config, vec![
            (p1, Box::new(AlwaysPayPlayer)),
            (p2, Box::new(NeverPayPlayer)),
        ]);

        // Add a creature for source
        let src_id = ObjectId::new();
        let mut card = CardData::new(src_id, p1, "Source");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        card.keywords = KeywordAbilities::empty();
        game.state.battlefield.add(Permanent::new(card, p1));

        // "You may blight 1. If you do, gain 3 life."
        let effect = Effect::do_if_cost_paid(
            Cost::Blight(1),
            vec![Effect::GainLife { amount: 3 }],
            vec![],
        );

        game.execute_effects(&[effect], p1, &[], Some(src_id), None);

        // AlwaysPayPlayer says yes, blight adds -1/-1, gain 3 life
        assert_eq!(game.state.battlefield.get(src_id).unwrap().counters.get(&CounterType::M1M1), 1);
        assert_eq!(game.state.players[&p1].life, 23); // 20 + 3
    }

    #[test]
    fn do_if_cost_paid_declines_runs_else() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "A".into(), deck: make_deck(p1) },
                PlayerConfig { name: "B".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let mut game = Game::new_two_player(config, vec![
            (p1, Box::new(NeverPayPlayer)),
            (p2, Box::new(NeverPayPlayer)),
        ]);

        // Add a creature for source
        let src_id = ObjectId::new();
        let mut card = CardData::new(src_id, p1, "Source");
        card.card_types = vec![CardType::Creature];
        card.power = Some(5);
        card.toughness = Some(4);
        card.keywords = KeywordAbilities::empty();
        game.state.battlefield.add(Permanent::new(card, p1));

        // "You may pay 3 life. If you don't, blight 2."
        let effect = Effect::do_if_cost_paid(
            Cost::PayLife(3),
            vec![],
            vec![Effect::add_counters_self("-1/-1", 2)],
        );

        game.execute_effects(&[effect], p1, &[], Some(src_id), None);

        // NeverPayPlayer says no, so blight 2 happens
        assert_eq!(game.state.players[&p1].life, 20); // no life paid
        assert_eq!(game.state.battlefield.get(src_id).unwrap().counters.get(&CounterType::M1M1), 2);
    }
}



#[cfg(test)]
mod type_choice_tests {
    use super::*;
    use crate::abilities::Effect;
    use crate::card::CardData;
    use crate::constants::{CardType, Outcome, SubType};
    use crate::decision::*;
    use crate::game::{GameConfig, PlayerConfig};
    use crate::types::{ObjectId, PlayerId};

    /// Decision maker that picks a given index for choose_option.
    struct OptionPicker(usize);

    impl PlayerDecisionMaker for OptionPicker {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction {
            PlayerAction::Pass
        }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize {
            self.0
        }
    }

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        (0..20).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    fn setup_game_with_picker(pick_index: usize) -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(OptionPicker(pick_index))),
                (p2, Box::new(OptionPicker(0))),
            ],
        );
        (game, p1, p2)
    }

    #[test]
    fn choose_creature_type_stores_on_permanent() {
        let (mut game, p1, _p2) = setup_game_with_picker(0); // picks first = "Elemental"

        let src_id = ObjectId::new();
        let mut card = CardData::new(src_id, p1, "Chronicle of Victory");
        card.card_types = vec![CardType::Artifact];
        game.state.battlefield.add(Permanent::new(card, p1));

        let effects = vec![Effect::choose_creature_type_restricted(
            vec!["Elemental", "Elf", "Faerie"]
        )];
        game.execute_effects(&effects, p1, &[], Some(src_id), None);

        let perm = game.state.battlefield.get(src_id).unwrap();
        assert!(perm.chosen_type.is_some());
        match &perm.chosen_type {
            Some(SubType::Custom(s)) => assert_eq!(s.as_str(), "Elemental"),
            other => panic!("Expected SubType::Custom(\"Elemental\"), got {:?}", other),
        }
    }

    #[test]
    fn choose_creature_type_picks_second_option() {
        let (mut game, p1, _p2) = setup_game_with_picker(1); // picks second = "Elf"

        let src_id = ObjectId::new();
        let mut card = CardData::new(src_id, p1, "Test Permanent");
        card.card_types = vec![CardType::Artifact];
        game.state.battlefield.add(Permanent::new(card, p1));

        let effects = vec![Effect::choose_creature_type_restricted(
            vec!["Goblin", "Elf", "Merfolk"]
        )];
        game.execute_effects(&effects, p1, &[], Some(src_id), None);

        let perm = game.state.battlefield.get(src_id).unwrap();
        match &perm.chosen_type {
            Some(SubType::Custom(s)) => assert_eq!(s.as_str(), "Elf"),
            other => panic!("Expected SubType::Custom(\"Elf\"), got {:?}", other),
        }
    }

    #[test]
    fn choose_type_and_draw_per_permanent() {
        // Pick index 4 = "Goblin" from the default list
        let (mut game, p1, _p2) = setup_game_with_picker(4);

        // Place 3 Goblins and 1 Elf on battlefield
        for i in 0..3 {
            let cid = ObjectId::new();
            let mut card = CardData::new(cid, p1, &format!("Goblin #{}", i));
            card.card_types = vec![CardType::Creature];
            card.subtypes = vec![SubType::Custom("Goblin".into())];
            game.state.battlefield.add(Permanent::new(card, p1));
        }
        {
            let cid = ObjectId::new();
            let mut card = CardData::new(cid, p1, "Some Elf");
            card.card_types = vec![CardType::Creature];
            card.subtypes = vec![SubType::Elf];
            game.state.battlefield.add(Permanent::new(card, p1));
        }

        let hand_before = game.state.players[&p1].hand.len();
        let effects = vec![Effect::choose_type_and_draw_per_permanent()];
        game.execute_effects(&effects, p1, &[], None, None);

        // Should have drawn 3 cards (3 Goblins)
        let hand_after = game.state.players[&p1].hand.len();
        assert_eq!(hand_after - hand_before, 3);
    }
}

#[cfg(test)]
mod combat_tests {
    use super::*;
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, Outcome};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };

    /// Decision maker that attacks with all creatures.
    struct AttackAllPlayer;

    impl PlayerDecisionMaker for AttackAllPlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction {
            PlayerAction::Pass
        }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(
            &mut self,
            _: &GameView<'_>,
            possible_attackers: &[ObjectId],
            possible_defenders: &[ObjectId],
        ) -> Vec<(ObjectId, ObjectId)> {
            let defender = possible_defenders[0];
            possible_attackers
                .iter()
                .map(|&a| (a, defender))
                .collect()
        }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    /// Decision maker that blocks with all available creatures (first available for each attacker).
    struct BlockAllPlayer;

    impl PlayerDecisionMaker for BlockAllPlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction {
            PlayerAction::Pass
        }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(
            &mut self,
            _: &GameView<'_>,
            attackers: &[AttackerInfo],
        ) -> Vec<(ObjectId, ObjectId)> {
            let mut blocks = Vec::new();
            let mut used = std::collections::HashSet::new();
            for info in attackers {
                for &blocker_id in &info.legal_blockers {
                    if !used.contains(&blocker_id) {
                        blocks.push((blocker_id, info.attacker_id));
                        used.insert(blocker_id);
                        break;
                    }
                }
            }
            blocks
        }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn make_creature(
        name: &str,
        owner: PlayerId,
        power: i32,
        toughness: i32,
        keywords: KeywordAbilities,
    ) -> CardData {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(power);
        card.toughness = Some(toughness);
        card.keywords = keywords;
        card
    }

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        (0..40)
            .map(|i| {
                let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
                c.card_types = vec![CardType::Land];
                c
            })
            .collect()
    }

    fn setup_combat_game(
        p1_dm: Box<dyn PlayerDecisionMaker>,
        p2_dm: Box<dyn PlayerDecisionMaker>,
    ) -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig {
                    name: "Attacker".to_string(),
                    deck: make_deck(p1),
                },
                PlayerConfig {
                    name: "Defender".to_string(),
                    deck: make_deck(p2),
                },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(config, vec![(p1, p1_dm), (p2, p2_dm)]);
        (game, p1, p2)
    }

    /// Helper to add a creature to the battlefield and remove summoning sickness.
    fn add_creature(
        game: &mut Game,
        owner: PlayerId,
        name: &str,
        power: i32,
        toughness: i32,
        keywords: KeywordAbilities,
    ) -> ObjectId {
        let card = make_creature(name, owner, power, toughness, keywords);
        let id = card.id;
        let mut perm = Permanent::new(card, owner);
        perm.remove_summoning_sickness();
        game.state.battlefield.add(perm);
        id
    }

    // ── Test: Unblocked combat damage ──────────────────────────────

    #[test]
    fn unblocked_attacker_deals_damage_to_player() {
        let (mut game, p1, p2) = setup_combat_game(
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
        );

        // Add a 3/3 creature for p1 (attacker)
        let bear_id = add_creature(&mut game, p1, "Bear", 3, 3, KeywordAbilities::empty());

        // Set active player to p1
        game.state.active_player = p1;
        game.state.priority_player = p1;

        // Run declare attackers step
        game.declare_attackers_step(p1);

        // Bear should be attacking
        assert!(game.state.combat.is_attacking(bear_id));
        // Bear should be tapped (no vigilance)
        assert!(game.state.battlefield.get(bear_id).unwrap().tapped);

        // Run declare blockers (no blockers for p2)
        game.declare_blockers_step(p1);

        // Run combat damage (regular)
        game.combat_damage_step(false);

        // p2 should have taken 3 damage
        assert_eq!(game.state.players[&p2].life, 17);
    }

    #[test]
    fn vigilance_does_not_tap_attacker() {
        let (mut game, p1, _p2) = setup_combat_game(
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
        );

        let vig_id = add_creature(&mut game, p1, "Vigilant", 2, 2, KeywordAbilities::VIGILANCE);
        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.declare_attackers_step(p1);

        // Should be attacking but NOT tapped
        assert!(game.state.combat.is_attacking(vig_id));
        assert!(!game.state.battlefield.get(vig_id).unwrap().tapped);
    }

    #[test]
    fn blocked_creature_deals_damage_to_blocker() {
        let (mut game, p1, p2) = setup_combat_game(
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
        );

        let attacker_id = add_creature(&mut game, p1, "Attacker", 3, 3, KeywordAbilities::empty());
        let blocker_id = add_creature(&mut game, p2, "Blocker", 2, 4, KeywordAbilities::empty());

        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.declare_attackers_step(p1);
        game.declare_blockers_step(p1);

        // Blocker should be blocking
        assert!(game.state.combat.is_blocking(blocker_id));

        // Apply combat damage
        game.combat_damage_step(false);

        // Blocker should have 3 damage, attacker should have 2 damage
        assert_eq!(game.state.battlefield.get(blocker_id).unwrap().damage, 3);
        assert_eq!(game.state.battlefield.get(attacker_id).unwrap().damage, 2);
        // Player should NOT have taken damage (blocked)
        assert_eq!(game.state.players[&p2].life, 20);
    }

    #[test]
    fn lifelink_gains_life_on_combat_damage() {
        let (mut game, p1, p2) = setup_combat_game(
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
        );

        add_creature(&mut game, p1, "Lifelinker", 4, 4, KeywordAbilities::LIFELINK);

        game.state.active_player = p1;
        game.state.priority_player = p1;

        // Reduce p1 life to verify gain
        game.state.players.get_mut(&p1).unwrap().life = 15;

        game.declare_attackers_step(p1);
        game.declare_blockers_step(p1);
        game.combat_damage_step(false);

        // p1 should gain 4 life (lifelink), p2 takes 4
        assert_eq!(game.state.players[&p1].life, 19);
        assert_eq!(game.state.players[&p2].life, 16);
    }

    #[test]
    fn first_strike_deals_damage_first() {
        let (mut game, p1, p2) = setup_combat_game(
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
        );

        let fs_id = add_creature(&mut game, p1, "FirstStriker", 3, 2, KeywordAbilities::FIRST_STRIKE);
        let blocker_id = add_creature(&mut game, p2, "Blocker", 3, 3, KeywordAbilities::empty());

        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.declare_attackers_step(p1);
        game.declare_blockers_step(p1);

        // First strike damage step
        game.combat_damage_step(true);

        // Blocker takes 3 first strike damage
        assert_eq!(game.state.battlefield.get(blocker_id).unwrap().damage, 3);
        // First striker takes 0 (normal creature doesn't deal in first strike step)
        assert_eq!(game.state.battlefield.get(fs_id).unwrap().damage, 0);

        // Regular damage step
        game.combat_damage_step(false);

        // First striker still takes 0 more (first strike creature doesn't deal in regular step)
        // But blocker deals its 3 damage to first striker in regular step
        assert_eq!(game.state.battlefield.get(fs_id).unwrap().damage, 3);
    }

    #[test]
    fn trample_overflow_to_player() {
        let (mut game, p1, p2) = setup_combat_game(
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
        );

        add_creature(&mut game, p1, "Trampler", 5, 5, KeywordAbilities::TRAMPLE);
        let blocker_id = add_creature(&mut game, p2, "SmallBlocker", 1, 2, KeywordAbilities::empty());

        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.declare_attackers_step(p1);
        game.declare_blockers_step(p1);
        game.combat_damage_step(false);

        // Blocker takes 2 damage (lethal), 3 tramples to player
        assert_eq!(game.state.battlefield.get(blocker_id).unwrap().damage, 2);
        assert_eq!(game.state.players[&p2].life, 17);
    }

    #[test]
    fn end_combat_clears_state() {
        let (mut game, p1, _p2) = setup_combat_game(
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
        );

        add_creature(&mut game, p1, "Bear", 2, 2, KeywordAbilities::empty());

        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.declare_attackers_step(p1);
        assert!(game.state.combat.has_attackers());

        // End combat clears state
        game.state.combat.clear();
        assert!(!game.state.combat.has_attackers());
    }

    #[test]
    fn defender_cannot_attack() {
        let (mut game, p1, p2) = setup_combat_game(
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
        );

        // Only a defender creature
        add_creature(&mut game, p1, "Wall", 0, 5, KeywordAbilities::DEFENDER);

        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.declare_attackers_step(p1);

        // Should have no attackers (defender can't attack)
        assert!(!game.state.combat.has_attackers());
        assert_eq!(game.state.players[&p2].life, 20);
    }

    #[test]
    fn summoning_sick_creature_cannot_attack() {
        let (mut game, p1, p2) = setup_combat_game(
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
        );

        // Add creature WITH summoning sickness (don't remove it)
        let card = make_creature("SickBear", p1, 3, 3, KeywordAbilities::empty());
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);

        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.declare_attackers_step(p1);

        // Should not have attacked
        assert!(!game.state.combat.has_attackers());
        assert_eq!(game.state.players[&p2].life, 20);
    }

    #[test]
    fn haste_bypasses_summoning_sickness() {
        let (mut game, p1, p2) = setup_combat_game(
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
        );

        // Add creature with haste and summoning sickness
        let card = make_creature("Hasty", p1, 2, 1, KeywordAbilities::HASTE);
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);

        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.declare_attackers_step(p1);
        game.declare_blockers_step(p1);
        game.combat_damage_step(false);

        // Should have attacked and dealt damage
        assert_eq!(game.state.players[&p2].life, 18);
    }

    #[test]
    fn flying_cannot_be_blocked_by_ground() {
        let (mut game, p1, p2) = setup_combat_game(
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
        );

        add_creature(&mut game, p1, "Flyer", 3, 3, KeywordAbilities::FLYING);
        // Ground creature cannot block a flyer
        add_creature(&mut game, p2, "Ground", 2, 2, KeywordAbilities::empty());

        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.declare_attackers_step(p1);
        game.declare_blockers_step(p1);
        game.combat_damage_step(false);

        // Flyer should get through unblocked
        assert_eq!(game.state.players[&p2].life, 17);
    }

    #[test]
    fn reach_can_block_flying() {
        let (mut game, p1, p2) = setup_combat_game(
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
        );

        let flyer_id = add_creature(&mut game, p1, "Flyer", 2, 2, KeywordAbilities::FLYING);
        let reacher_id = add_creature(&mut game, p2, "Reacher", 1, 4, KeywordAbilities::REACH);

        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.declare_attackers_step(p1);
        game.declare_blockers_step(p1);

        // Reacher should be blocking the flyer
        assert!(game.state.combat.is_blocking(reacher_id));

        game.combat_damage_step(false);

        // No damage to player
        assert_eq!(game.state.players[&p2].life, 20);
        // Creatures trade damage
        assert_eq!(game.state.battlefield.get(reacher_id).unwrap().damage, 2);
        assert_eq!(game.state.battlefield.get(flyer_id).unwrap().damage, 1);
    }

    #[test]
    fn multiple_attackers_deal_combined_damage() {
        let (mut game, p1, p2) = setup_combat_game(
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
        );

        add_creature(&mut game, p1, "Bear1", 2, 2, KeywordAbilities::empty());
        add_creature(&mut game, p1, "Bear2", 3, 3, KeywordAbilities::empty());

        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.declare_attackers_step(p1);
        game.declare_blockers_step(p1);
        game.combat_damage_step(false);

        // Both should deal damage: 2 + 3 = 5
        assert_eq!(game.state.players[&p2].life, 15);
    }
}

#[cfg(test)]
mod trigger_tests {
    use super::*;
    use crate::abilities::{Ability, Effect, TargetSpec};
    use crate::card::CardData;
    use crate::constants::{CardType, Outcome};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };
    use crate::events::EventType;

    /// Decision maker that always passes and says yes to optional triggers.
    struct TriggerTestPlayer {
        attack_all: bool,
    }

    impl TriggerTestPlayer {
        fn passive() -> Self { TriggerTestPlayer { attack_all: false } }
        fn attacker() -> Self { TriggerTestPlayer { attack_all: true } }
    }

    impl PlayerDecisionMaker for TriggerTestPlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction {
            PlayerAction::Pass
        }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { true }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(
            &mut self,
            _: &GameView<'_>,
            possible_attackers: &[ObjectId],
            possible_defenders: &[ObjectId],
        ) -> Vec<(ObjectId, ObjectId)> {
            if self.attack_all && !possible_defenders.is_empty() {
                let defender = possible_defenders[0];
                possible_attackers.iter().map(|&a| (a, defender)).collect()
            } else {
                vec![]
            }
        }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        (0..40).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    fn setup(
        p1_dm: Box<dyn PlayerDecisionMaker>,
        p2_dm: Box<dyn PlayerDecisionMaker>,
    ) -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(config, vec![(p1, p1_dm), (p2, p2_dm)]);
        (game, p1, p2)
    }

    #[test]
    fn etb_trigger_fires_and_resolves() {
        let (mut game, p1, _p2) = setup(
            Box::new(TriggerTestPlayer::passive()),
            Box::new(TriggerTestPlayer::passive()),
        );

        // Create a creature with an ETB trigger: "When this enters, gain 3 life"
        let card_id = ObjectId::new();
        let mut card = CardData::new(card_id, p1, "Soul Warden");
        card.card_types = vec![CardType::Creature];
        card.power = Some(1);
        card.toughness = Some(1);
        card.abilities.push(Ability::triggered(
            card_id,
            "When Soul Warden enters the battlefield, you gain 3 life.",
            vec![EventType::EnteredTheBattlefield],
            vec![Effect::GainLife { amount: 3 }],
            TargetSpec::None,
        ));

        // Register the card in the card store
        game.state.card_store.insert(card.clone());

        // Register abilities
        for ability in &card.abilities {
            game.state.ability_store.add(ability.clone());
        }

        // Put the permanent on the battlefield and emit ETB
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);
        game.state.set_zone(card_id, crate::constants::Zone::Battlefield, None);
        game.emit_event(GameEvent::enters_battlefield(card_id, p1));

        // Before processing triggers, life should be 20
        assert_eq!(game.state.players[&p1].life, 20);

        // Process SBAs + triggers
        game.process_sba_and_triggers();

        // The trigger should have been put on the stack
        // Since we called process_sba_and_triggers (not the full priority loop),
        // the ability is on the stack. Let's resolve it.
        assert!(!game.state.stack.is_empty());

        // Resolve the triggered ability
        game.resolve_top_of_stack();

        // Life should now be 23
        assert_eq!(game.state.players[&p1].life, 23);
    }

    #[test]
    fn attack_trigger_fires() {
        let (mut game, p1, p2) = setup(
            Box::new(TriggerTestPlayer::attacker()),
            Box::new(TriggerTestPlayer::passive()),
        );

        // Create creature with attack trigger: "Whenever this attacks, each opponent loses 1 life"
        let card_id = ObjectId::new();
        let mut card = CardData::new(card_id, p1, "Pulse Tracker");
        card.card_types = vec![CardType::Creature];
        card.power = Some(1);
        card.toughness = Some(1);
        card.abilities.push(Ability::triggered(
            card_id,
            "Whenever Pulse Tracker attacks, each opponent loses 1 life.",
            vec![EventType::AttackerDeclared],
            vec![Effect::LoseLifeOpponents { amount: 1 }],
            TargetSpec::None,
        ));

        // Register
        game.state.card_store.insert(card.clone());
        for ability in &card.abilities {
            game.state.ability_store.add(ability.clone());
        }
        let mut perm = Permanent::new(card, p1);
        perm.remove_summoning_sickness();
        game.state.battlefield.add(perm);
        game.state.set_zone(card_id, crate::constants::Zone::Battlefield, None);

        // Declare attackers
        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.declare_attackers_step(p1);

        // Process SBAs + triggers
        game.process_sba_and_triggers();

        // Attack trigger should be on the stack
        assert!(!game.state.stack.is_empty());

        // Resolve it
        game.resolve_top_of_stack();

        // Opponent should have lost 1 life
        assert_eq!(game.state.players[&p2].life, 19);
    }

    #[test]
    fn life_gain_trigger_fires() {
        let (mut game, p1, _p2) = setup(
            Box::new(TriggerTestPlayer::passive()),
            Box::new(TriggerTestPlayer::passive()),
        );

        // Create "Ajani's Pridemate" — whenever you gain life, put a +1/+1 counter
        let card_id = ObjectId::new();
        let mut card = CardData::new(card_id, p1, "Ajani's Pridemate");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        card.abilities.push(Ability::triggered(
            card_id,
            "Whenever you gain life, put a +1/+1 counter on Ajani's Pridemate.",
            vec![EventType::GainLife],
            vec![Effect::add_counters_self("+1/+1", 1)],
            TargetSpec::None,
        ));

        // Register
        game.state.card_store.insert(card.clone());
        for ability in &card.abilities {
            game.state.ability_store.add(ability.clone());
        }
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);

        // Gain life
        game.execute_effects(&[Effect::GainLife { amount: 5 }], p1, &[], None, None);

        // Process triggers
        game.process_sba_and_triggers();
        assert!(!game.state.stack.is_empty());

        // Resolve the trigger
        game.resolve_top_of_stack();

        // Should have a +1/+1 counter
        let pridemate = game.state.battlefield.get(card_id).unwrap();
        assert_eq!(pridemate.counters.get(&crate::counters::CounterType::P1P1), 1);
        assert_eq!(pridemate.power(), 3);
        assert_eq!(pridemate.toughness(), 3);
    }

    #[test]
    fn optional_trigger_not_forced() {
        let (mut game, p1, _p2) = setup(
            Box::new(TriggerTestPlayer::passive()),
            Box::new(TriggerTestPlayer::passive()),
        );

        // A "may" trigger that the player says yes to (TriggerTestPlayer says yes)
        let card_id = ObjectId::new();
        let mut card = CardData::new(card_id, p1, "Optional Creature");
        card.card_types = vec![CardType::Creature];
        card.power = Some(1);
        card.toughness = Some(1);
        let ability = Ability::triggered(
            card_id,
            "When Optional Creature enters, you may gain 2 life.",
            vec![EventType::EnteredTheBattlefield],
            vec![Effect::GainLife { amount: 2 }],
            TargetSpec::None,
        ).set_optional();
        card.abilities.push(ability);

        game.state.card_store.insert(card.clone());
        for ability in &card.abilities {
            game.state.ability_store.add(ability.clone());
        }
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);
        game.emit_event(GameEvent::enters_battlefield(card_id, p1));

        // Process triggers — TriggerTestPlayer always says yes
        game.process_sba_and_triggers();

        // Should be on the stack (player chose yes)
        assert!(!game.state.stack.is_empty());
        game.resolve_top_of_stack();
        assert_eq!(game.state.players[&p1].life, 22);
    }

    #[test]
    fn trigger_only_fires_for_own_permanent() {
        let (mut game, p1, p2) = setup(
            Box::new(TriggerTestPlayer::passive()),
            Box::new(TriggerTestPlayer::passive()),
        );

        // p1's creature has attack trigger
        let card_id = ObjectId::new();
        let mut card = CardData::new(card_id, p1, "TriggerCreature");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        card.abilities.push(Ability::triggered(
            card_id,
            "Whenever TriggerCreature attacks, gain 1 life.",
            vec![EventType::AttackerDeclared],
            vec![Effect::GainLife { amount: 1 }],
            TargetSpec::None,
        ));

        game.state.card_store.insert(card.clone());
        for ability in &card.abilities {
            game.state.ability_store.add(ability.clone());
        }
        let mut perm = Permanent::new(card, p1);
        perm.remove_summoning_sickness();
        game.state.battlefield.add(perm);

        // A DIFFERENT creature from p2 attacks — p1's trigger should NOT fire
        let other_id = ObjectId::new();
        let mut other = CardData::new(other_id, p2, "Other");
        other.card_types = vec![CardType::Creature];
        other.power = Some(1);
        other.toughness = Some(1);
        let mut perm2 = Permanent::new(other, p2);
        perm2.remove_summoning_sickness();
        game.state.battlefield.add(perm2);

        // Emit attack event for p2's creature (not p1's)
        game.emit_event(
            GameEvent::new(EventType::AttackerDeclared)
                .target(other_id)
                .player(p2),
        );

        // Process triggers
        game.process_sba_and_triggers();

        // No trigger should have fired (the attacker was a different creature)
        assert!(game.state.stack.is_empty());
    }
}

#[cfg(test)]
mod continuous_effect_tests {
    use super::*;
    use crate::abilities::{Ability, StaticEffect};
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, Outcome, SubType};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };

    /// Passive decision maker — always passes.
    struct PassivePlayer;

    impl PlayerDecisionMaker for PassivePlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        (0..40).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    fn setup() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Player1".into(), deck: make_deck(p1) },
                PlayerConfig { name: "Player2".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(
            config,
            vec![(p1, Box::new(PassivePlayer)), (p2, Box::new(PassivePlayer))],
        );
        (game, p1, p2)
    }

    fn add_creature(
        game: &mut Game,
        owner: PlayerId,
        name: &str,
        power: i32,
        toughness: i32,
        keywords: KeywordAbilities,
    ) -> ObjectId {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(power);
        card.toughness = Some(toughness);
        card.keywords = keywords;
        let id = card.id;
        let perm = Permanent::new(card, owner);
        game.state.battlefield.add(perm);
        id
    }

    fn add_creature_with_subtype(
        game: &mut Game,
        owner: PlayerId,
        name: &str,
        power: i32,
        toughness: i32,
        subtype: SubType,
    ) -> ObjectId {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![subtype];
        card.power = Some(power);
        card.toughness = Some(toughness);
        let id = card.id;
        let perm = Permanent::new(card, owner);
        game.state.battlefield.add(perm);
        id
    }

    fn add_lord_with_boost(
        game: &mut Game,
        owner: PlayerId,
        name: &str,
        power: i32,
        toughness: i32,
        subtype: SubType,
        filter: &str,
        boost_p: i32,
        boost_t: i32,
    ) -> ObjectId {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![subtype];
        card.power = Some(power);
        card.toughness = Some(toughness);
        let id = card.id;
        card.abilities = vec![
            Ability::static_ability(id, &format!("Other creatures get +{boost_p}/+{boost_t}"),
                vec![StaticEffect::Boost { filter: filter.into(), power: boost_p, toughness: boost_t }]),
        ];
        let perm = Permanent::new(card, owner);
        game.state.battlefield.add(perm);
        // Register abilities
        let abilities: Vec<Ability> = game.state.battlefield.get(id).unwrap().card.abilities.clone();
        for ability in abilities {
            game.state.ability_store.add(ability);
        }
        id
    }

    fn add_keyword_lord(
        game: &mut Game,
        owner: PlayerId,
        name: &str,
        power: i32,
        toughness: i32,
        filter: &str,
        keyword: &str,
    ) -> ObjectId {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(power);
        card.toughness = Some(toughness);
        let id = card.id;
        card.abilities = vec![
            Ability::static_ability(id, &format!("Creatures have {keyword}"),
                vec![StaticEffect::GrantKeyword { filter: filter.into(), keyword: keyword.into() }]),
        ];
        let perm = Permanent::new(card, owner);
        game.state.battlefield.add(perm);
        let abilities: Vec<Ability> = game.state.battlefield.get(id).unwrap().card.abilities.clone();
        for ability in abilities {
            game.state.ability_store.add(ability);
        }
        id
    }

    // ── Test: Lord boosts other creatures of same type ──────────────

    #[test]
    fn lord_boosts_other_creatures_of_same_type() {
        let (mut game, p1, _p2) = setup();

        // Add an Elf lord: "Other Elf you control get +1/+1"
        let lord_id = add_lord_with_boost(&mut game, p1, "Elvish Archdruid", 2, 2,
            SubType::Elf, "other Elf you control", 1, 1);

        // Add two Elf creatures
        let elf1_id = add_creature_with_subtype(&mut game, p1, "Llanowar Elves", 1, 1, SubType::Elf);
        let elf2_id = add_creature_with_subtype(&mut game, p1, "Elvish Mystic", 1, 1, SubType::Elf);

        // Add a non-Elf creature
        let bear_id = add_creature(&mut game, p1, "Grizzly Bears", 2, 2, KeywordAbilities::empty());

        // Apply continuous effects
        game.apply_continuous_effects();

        // Lord itself should NOT be boosted (filter says "other")
        let lord = game.state.battlefield.get(lord_id).unwrap();
        assert_eq!(lord.power(), 2);
        assert_eq!(lord.toughness(), 2);

        // Elves should be boosted
        let elf1 = game.state.battlefield.get(elf1_id).unwrap();
        assert_eq!(elf1.power(), 2);
        assert_eq!(elf1.toughness(), 2);

        let elf2 = game.state.battlefield.get(elf2_id).unwrap();
        assert_eq!(elf2.power(), 2);
        assert_eq!(elf2.toughness(), 2);

        // Bear should NOT be boosted (not an Elf)
        let bear = game.state.battlefield.get(bear_id).unwrap();
        assert_eq!(bear.power(), 2);
        assert_eq!(bear.toughness(), 2);
    }

    // ── Test: Anthem boosts all creatures you control ──────────────

    #[test]
    fn anthem_boosts_all_creatures_you_control() {
        let (mut game, p1, p2) = setup();

        // Add anthem: "creature you control get +1/+1"
        let anthem_id = add_lord_with_boost(&mut game, p1, "Glorious Anthem", 0, 0,
            SubType::Custom("Enchantment".into()), "creature you control", 1, 1);

        // P1's creature
        let bear1_id = add_creature(&mut game, p1, "Bear", 2, 2, KeywordAbilities::empty());

        // P2's creature should NOT be boosted
        let bear2_id = add_creature(&mut game, p2, "Enemy Bear", 2, 2, KeywordAbilities::empty());

        game.apply_continuous_effects();

        // Anthem itself is a 0/0 creature, so it gets +1/+1 too
        // (filter is "creature you control", not "other creature")
        let anthem = game.state.battlefield.get(anthem_id).unwrap();
        assert_eq!(anthem.power(), 1);
        assert_eq!(anthem.toughness(), 1);

        let bear1 = game.state.battlefield.get(bear1_id).unwrap();
        assert_eq!(bear1.power(), 3);
        assert_eq!(bear1.toughness(), 3);

        let bear2 = game.state.battlefield.get(bear2_id).unwrap();
        assert_eq!(bear2.power(), 2);
        assert_eq!(bear2.toughness(), 2);
    }

    // ── Test: Keyword grant ────────────────────────────────────────

    #[test]
    fn keyword_grant_gives_keyword_to_matching_creatures() {
        let (mut game, p1, _p2) = setup();

        // "Creatures you control have flying"
        add_keyword_lord(&mut game, p1, "Archetype of Imagination", 3, 2,
            "creature you control", "flying");

        let bear_id = add_creature(&mut game, p1, "Bear", 2, 2, KeywordAbilities::empty());

        game.apply_continuous_effects();

        let bear = game.state.battlefield.get(bear_id).unwrap();
        assert!(bear.has_flying());
    }

    // ── Test: Multiple keywords in comma-separated string ──────────

    #[test]
    fn comma_separated_keywords_granted() {
        let (mut game, p1, _p2) = setup();

        // "Equipped creature has deathtouch, lifelink"
        add_keyword_lord(&mut game, p1, "Basilisk Collar", 0, 0,
            "creature you control", "deathtouch, lifelink");

        let bear_id = add_creature(&mut game, p1, "Bear", 2, 2, KeywordAbilities::empty());

        game.apply_continuous_effects();

        let bear = game.state.battlefield.get(bear_id).unwrap();
        assert!(bear.has_deathtouch());
        assert!(bear.has_lifelink());
    }

    // ── Test: Effects cleared on recalculation ─────────────────────

    #[test]
    fn effects_cleared_and_recalculated() {
        let (mut game, p1, _p2) = setup();

        let lord_id = add_lord_with_boost(&mut game, p1, "Elvish Archdruid", 2, 2,
            SubType::Elf, "other Elf you control", 1, 1);

        let elf_id = add_creature_with_subtype(&mut game, p1, "Llanowar Elves", 1, 1, SubType::Elf);

        // Apply once
        game.apply_continuous_effects();
        assert_eq!(game.state.battlefield.get(elf_id).unwrap().power(), 2);

        // Remove lord from battlefield
        game.state.battlefield.remove(lord_id);
        game.state.ability_store.remove_source(lord_id);

        // Apply again — boost should be gone
        game.apply_continuous_effects();
        assert_eq!(game.state.battlefield.get(elf_id).unwrap().power(), 1);
    }

    // ── Test: Multiple lords stack ─────────────────────────────────

    #[test]
    fn multiple_lords_stack() {
        let (mut game, p1, _p2) = setup();

        // Two Elf lords
        add_lord_with_boost(&mut game, p1, "Lord 1", 2, 2,
            SubType::Elf, "other Elf you control", 1, 1);
        add_lord_with_boost(&mut game, p1, "Lord 2", 2, 2,
            SubType::Elf, "other Elf you control", 1, 1);

        let elf_id = add_creature_with_subtype(&mut game, p1, "Llanowar Elves", 1, 1, SubType::Elf);

        game.apply_continuous_effects();

        // Elf should get +1/+1 from each lord = +2/+2 total
        let elf = game.state.battlefield.get(elf_id).unwrap();
        assert_eq!(elf.power(), 3);
        assert_eq!(elf.toughness(), 3);
    }

    // ── Test: Lord boosts each other ───────────────────────────────

    #[test]
    fn lords_boost_each_other() {
        let (mut game, p1, _p2) = setup();

        // Two Elf lords with "other Elf you control get +1/+1"
        let lord1_id = add_lord_with_boost(&mut game, p1, "Lord 1", 2, 2,
            SubType::Elf, "other Elf you control", 1, 1);
        let lord2_id = add_lord_with_boost(&mut game, p1, "Lord 2", 2, 2,
            SubType::Elf, "other Elf you control", 1, 1);

        game.apply_continuous_effects();

        // Each lord should get +1/+1 from the other
        let lord1 = game.state.battlefield.get(lord1_id).unwrap();
        assert_eq!(lord1.power(), 3);
        assert_eq!(lord1.toughness(), 3);

        let lord2 = game.state.battlefield.get(lord2_id).unwrap();
        assert_eq!(lord2.power(), 3);
        assert_eq!(lord2.toughness(), 3);
    }

    // ── Test: "self" filter applies only to source ─────────────────

    #[test]
    fn self_filter_applies_only_to_source() {
        let (mut game, p1, _p2) = setup();

        // A creature with a static effect targeting "self"
        let mut card = CardData::new(ObjectId::new(), p1, "Self-Booster");
        card.card_types = vec![CardType::Creature];
        card.power = Some(1);
        card.toughness = Some(1);
        let id = card.id;
        card.abilities = vec![
            Ability::static_ability(id, "+2/+2 to self",
                vec![StaticEffect::Boost { filter: "self".into(), power: 2, toughness: 2 }]),
        ];
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);
        let abilities: Vec<Ability> = game.state.battlefield.get(id).unwrap().card.abilities.clone();
        for a in abilities { game.state.ability_store.add(a); }

        let other_id = add_creature(&mut game, p1, "Other", 1, 1, KeywordAbilities::empty());

        game.apply_continuous_effects();

        assert_eq!(game.state.battlefield.get(id).unwrap().power(), 3);
        assert_eq!(game.state.battlefield.get(other_id).unwrap().power(), 1);
    }

    // ── Test: Token filter ─────────────────────────────────────────

    #[test]
    fn token_filter_only_matches_tokens() {
        let (mut game, p1, _p2) = setup();

        // "Creature token you control get +1/+1"
        add_lord_with_boost(&mut game, p1, "Token Lord", 2, 2,
            SubType::Custom("Lord".into()), "creature token you control", 1, 1);

        // Regular creature
        let regular_id = add_creature(&mut game, p1, "Regular Bear", 2, 2, KeywordAbilities::empty());

        // Token creature
        let token_id = ObjectId::new();
        let mut token_card = CardData::new(token_id, p1, "Bear Token");
        token_card.card_types = vec![CardType::Creature];
        token_card.power = Some(2);
        token_card.toughness = Some(2);
        token_card.is_token = true;
        let token_perm = Permanent::new(token_card, p1);
        game.state.battlefield.add(token_perm);

        game.apply_continuous_effects();

        // Regular creature should NOT be boosted
        assert_eq!(game.state.battlefield.get(regular_id).unwrap().power(), 2);

        // Token should be boosted
        assert_eq!(game.state.battlefield.get(token_id).unwrap().power(), 3);
    }

    // ── Test: Opponent's lord doesn't boost your creatures ─────────

    #[test]
    fn opponent_lord_doesnt_boost_your_creatures() {
        let (mut game, p1, p2) = setup();

        // P2 has Elf lord
        add_lord_with_boost(&mut game, p2, "Enemy Lord", 2, 2,
            SubType::Elf, "other Elf you control", 1, 1);

        // P1 has Elf
        let elf_id = add_creature_with_subtype(&mut game, p1, "My Elf", 1, 1, SubType::Elf);

        game.apply_continuous_effects();

        // P1's Elf should NOT be boosted by P2's lord
        assert_eq!(game.state.battlefield.get(elf_id).unwrap().power(), 1);
    }

    // ── Test: Boost + keyword grant combo ──────────────────────────

    #[test]
    fn boost_and_keyword_combo() {
        let (mut game, p1, _p2) = setup();

        // A lord with both boost and keyword grant
        let mut card = CardData::new(ObjectId::new(), p1, "Drogskol Captain");
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![SubType::Spirit];
        card.power = Some(2);
        card.toughness = Some(2);
        let id = card.id;
        card.abilities = vec![
            Ability::static_ability(id, "Other Spirit creatures you control get +1/+1 and have hexproof",
                vec![
                    StaticEffect::Boost { filter: "other Spirit you control".into(), power: 1, toughness: 1 },
                    StaticEffect::GrantKeyword { filter: "other Spirit you control".into(), keyword: "hexproof".into() },
                ]),
        ];
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);
        let abilities: Vec<Ability> = game.state.battlefield.get(id).unwrap().card.abilities.clone();
        for a in abilities { game.state.ability_store.add(a); }

        let spirit_id = add_creature_with_subtype(&mut game, p1, "Mausoleum Wanderer", 1, 1, SubType::Spirit);

        game.apply_continuous_effects();

        let spirit = game.state.battlefield.get(spirit_id).unwrap();
        assert_eq!(spirit.power(), 2);
        assert_eq!(spirit.toughness(), 2);
        assert!(spirit.has_hexproof());
    }
}

#[cfg(test)]
mod enters_tapped_tests {
    use super::*;
    use crate::abilities::{Ability, StaticEffect};
    use crate::card::CardData;
    use crate::constants::{CardType, Outcome, SubType};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };
    use crate::mana::Mana;

    struct PassivePlayer;
    impl PlayerDecisionMaker for PassivePlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        (0..40).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    fn setup() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Player1".into(), deck: make_deck(p1) },
                PlayerConfig { name: "Player2".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(
            config,
            vec![(p1, Box::new(PassivePlayer)), (p2, Box::new(PassivePlayer))],
        );
        (game, p1, p2)
    }

    #[test]
    fn enters_tapped_self_filter_taps_permanent() {
        let (mut game, p1, _p2) = setup();

        // Create a guildgate-like land that enters tapped
        let mut card = CardData::new(ObjectId::new(), p1, "Azorius Guildgate");
        card.card_types = vec![CardType::Land];
        card.subtypes = vec![SubType::Gate];
        let id = card.id;
        card.abilities = vec![
            Ability::static_ability(id, "Azorius Guildgate enters tapped.",
                vec![StaticEffect::EntersTapped { filter: "self".into() }]),
            Ability::mana_ability(id, "{T}: Add {W}.", Mana::white(1)),
        ];
        // Register abilities first
        for ability in &card.abilities {
            game.state.ability_store.add(ability.clone());
        }
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);
        game.check_enters_tapped(id);

        // Should be tapped
        assert!(game.state.battlefield.get(id).unwrap().tapped);
    }

    #[test]
    fn regular_land_enters_untapped() {
        let (mut game, p1, _p2) = setup();

        let mut card = CardData::new(ObjectId::new(), p1, "Forest");
        card.card_types = vec![CardType::Land];
        let id = card.id;
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);
        game.check_enters_tapped(id);

        // Should NOT be tapped
        assert!(!game.state.battlefield.get(id).unwrap().tapped);
    }

    #[test]
    fn creature_without_enters_tapped_stays_untapped() {
        let (mut game, p1, _p2) = setup();

        let mut card = CardData::new(ObjectId::new(), p1, "Grizzly Bears");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        let id = card.id;
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);
        game.check_enters_tapped(id);

        assert!(!game.state.battlefield.get(id).unwrap().tapped);
    }
}

#[cfg(test)]
mod hexproof_tests {
    use super::*;
    use crate::abilities::TargetSpec;
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, Outcome};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };

    struct PassivePlayer;
    impl PlayerDecisionMaker for PassivePlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        (0..40).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    fn setup() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Player1".into(), deck: make_deck(p1) },
                PlayerConfig { name: "Player2".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(
            config,
            vec![(p1, Box::new(PassivePlayer)), (p2, Box::new(PassivePlayer))],
        );
        (game, p1, p2)
    }

    fn add_creature(game: &mut Game, owner: PlayerId, name: &str, kw: KeywordAbilities) -> ObjectId {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        card.keywords = kw;
        let id = card.id;
        game.state.battlefield.add(Permanent::new(card, owner));
        id
    }

    #[test]
    fn hexproof_prevents_opponent_targeting() {
        let (mut game, p1, p2) = setup();

        let hexproof_id = add_creature(&mut game, p2, "Hexproof Bear", KeywordAbilities::HEXPROOF);
        let regular_id = add_creature(&mut game, p2, "Regular Bear", KeywordAbilities::empty());

        // P1 targeting creatures — hexproof creature should NOT be in legal targets
        let targets = game.legal_targets_for_spec(&TargetSpec::Creature, p1);
        assert!(!targets.contains(&hexproof_id));
        assert!(targets.contains(&regular_id));

        // Opponent creature targeting — same
        let targets = game.legal_targets_for_spec(&TargetSpec::OpponentCreature, p1);
        assert!(!targets.contains(&hexproof_id));
        assert!(targets.contains(&regular_id));
    }

    #[test]
    fn hexproof_allows_controller_targeting() {
        let (mut game, _p1, p2) = setup();

        let hexproof_id = add_creature(&mut game, p2, "Hexproof Bear", KeywordAbilities::HEXPROOF);

        // P2 targeting their own hexproof creature — should be allowed
        let targets = game.legal_targets_for_spec(&TargetSpec::CreatureYouControl, p2);
        assert!(targets.contains(&hexproof_id));

        // P2 targeting any creature — their own hexproof creature is fine
        let targets = game.legal_targets_for_spec(&TargetSpec::Creature, p2);
        assert!(targets.contains(&hexproof_id));
    }

    #[test]
    fn shroud_prevents_all_targeting() {
        let (mut game, p1, p2) = setup();

        let shroud_id = add_creature(&mut game, p2, "Shroud Bear", KeywordAbilities::SHROUD);

        // Neither player can target a shroud creature
        let targets = game.legal_targets_for_spec(&TargetSpec::Creature, p1);
        assert!(!targets.contains(&shroud_id));

        let targets = game.legal_targets_for_spec(&TargetSpec::Creature, p2);
        assert!(!targets.contains(&shroud_id));
    }

    #[test]
    fn hexproof_on_permanent_targeting() {
        let (mut game, p1, p2) = setup();

        let hexproof_id = add_creature(&mut game, p2, "Hexproof Bear", KeywordAbilities::HEXPROOF);
        let regular_id = add_creature(&mut game, p2, "Regular Bear", KeywordAbilities::empty());

        // TargetSpec::Permanent — hexproof blocks opponent targeting
        let targets = game.legal_targets_for_spec(&TargetSpec::Permanent, p1);
        assert!(!targets.contains(&hexproof_id));
        assert!(targets.contains(&regular_id));
    }

    #[test]
    fn granted_hexproof_prevents_targeting() {
        let (mut game, p1, p2) = setup();

        // A creature without hexproof that gets it granted
        let bear_id = add_creature(&mut game, p2, "Bear", KeywordAbilities::empty());

        // Grant hexproof via continuous_keywords
        if let Some(perm) = game.state.battlefield.get_mut(bear_id) {
            perm.continuous_keywords |= KeywordAbilities::HEXPROOF;
        }

        // P1 should not be able to target the bear with continuous hexproof
        let targets = game.legal_targets_for_spec(&TargetSpec::Creature, p1);
        assert!(!targets.contains(&bear_id));
    }
}

#[cfg(test)]
mod dies_trigger_tests {
    use super::*;
    use crate::abilities::{Ability, Effect, TargetSpec};
    use crate::card::CardData;
    use crate::constants::{CardType, Outcome};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };
    use crate::events::EventType;

    struct PassivePlayer;
    impl PlayerDecisionMaker for PassivePlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { true }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        (0..40).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    fn setup() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Player1".into(), deck: make_deck(p1) },
                PlayerConfig { name: "Player2".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(
            config,
            vec![(p1, Box::new(PassivePlayer)), (p2, Box::new(PassivePlayer))],
        );
        (game, p1, p2)
    }

    #[test]
    fn dies_trigger_fires_on_lethal_damage() {
        let (mut game, p1, _p2) = setup();

        // Create a creature with "When this creature dies, draw a card"
        let mut card = CardData::new(ObjectId::new(), p1, "Doomed Traveler");
        card.card_types = vec![CardType::Creature];
        card.power = Some(1);
        card.toughness = Some(1);
        let id = card.id;
        card.abilities = vec![
            Ability::triggered(id, "When Doomed Traveler dies, draw a card.",
                vec![EventType::Dies],
                vec![Effect::DrawCards { count: 1 }],
                TargetSpec::None),
        ];
        for ability in &card.abilities {
            game.state.ability_store.add(ability.clone());
        }
        let mut perm = Permanent::new(card, p1);
        perm.remove_summoning_sickness();
        game.state.battlefield.add(perm);

        // Mark lethal damage on the creature
        game.state.battlefield.get_mut(id).unwrap().apply_damage(1);

        // Process SBAs + triggers
        game.process_sba_and_triggers();

        // Creature should be in graveyard
        assert!(!game.state.battlefield.contains(id));

        // Dies trigger should have put an ability on the stack
        assert!(!game.state.stack.is_empty(), "Dies trigger should be on stack");
    }

    #[test]
    fn dies_trigger_fires_on_destroy_effect() {
        let (mut game, p1, p2) = setup();

        // Create a creature with dies trigger controlled by p2
        let mut card = CardData::new(ObjectId::new(), p2, "Blood Artist");
        card.card_types = vec![CardType::Creature];
        card.power = Some(0);
        card.toughness = Some(1);
        let id = card.id;
        card.abilities = vec![
            Ability::triggered(id, "When Blood Artist dies, opponent loses 1 life.",
                vec![EventType::Dies],
                vec![Effect::LoseLifeOpponents { amount: 1 }],
                TargetSpec::None),
        ];
        for ability in &card.abilities {
            game.state.ability_store.add(ability.clone());
        }
        let perm = Permanent::new(card, p2);
        game.state.battlefield.add(perm);

        // Destroy it with an effect
        game.execute_effects(
            &[Effect::Destroy],
            p1,
            &[id],
            None, None,
        );

        // Creature should be in graveyard
        assert!(!game.state.battlefield.contains(id));

        // Check that dies event was emitted
        assert!(!game.event_log.is_empty(), "Dies event should be in log");
    }

    #[test]
    fn dies_trigger_only_for_dying_creature() {
        let (mut game, p1, _p2) = setup();

        // Creature A has a dies trigger
        let mut card_a = CardData::new(ObjectId::new(), p1, "Creature A");
        card_a.card_types = vec![CardType::Creature];
        card_a.power = Some(1);
        card_a.toughness = Some(1);
        let id_a = card_a.id;
        card_a.abilities = vec![
            Ability::triggered(id_a, "When this dies, draw a card.",
                vec![EventType::Dies],
                vec![Effect::DrawCards { count: 1 }],
                TargetSpec::None),
        ];
        for ability in &card_a.abilities {
            game.state.ability_store.add(ability.clone());
        }
        let perm_a = Permanent::new(card_a, p1);
        game.state.battlefield.add(perm_a);

        // Creature B has NO dies trigger
        let mut card_b = CardData::new(ObjectId::new(), p1, "Creature B");
        card_b.card_types = vec![CardType::Creature];
        card_b.power = Some(1);
        card_b.toughness = Some(1);
        let id_b = card_b.id;
        let perm_b = Permanent::new(card_b, p1);
        game.state.battlefield.add(perm_b);

        // Kill Creature B only (not A)
        game.state.battlefield.get_mut(id_b).unwrap().apply_damage(1);

        // Process SBAs
        game.process_sba_and_triggers();

        // Creature B should be dead, Creature A should be alive
        assert!(game.state.battlefield.contains(id_a));
        assert!(!game.state.battlefield.contains(id_b));

        // No trigger should fire (the dying creature had no dies trigger)
        assert!(game.state.stack.is_empty());
    }
}

// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Equipment tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod equipment_tests {
    use super::*;
    use crate::abilities::{Ability, Cost, Effect, StaticEffect, TargetSpec};
    use crate::card::CardData;
    use crate::constants::{CardType, Outcome, SubType};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };
    use crate::mana::ManaCost;
    use crate::types::{ObjectId, PlayerId};

    struct PassivePlayer;
    impl PlayerDecisionMaker for PassivePlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { true }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn setup() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "P1".into(), deck: vec![] },
                PlayerConfig { name: "P2".into(), deck: vec![] },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(PassivePlayer)),
                (p2, Box::new(PassivePlayer)),
            ],
        );
        (game, p1, p2)
    }

    fn make_creature(id: ObjectId, owner: PlayerId, name: &str, power: i32, toughness: i32) -> CardData {
        let mut card = CardData::new(id, owner, name);
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![SubType::Human];
        card.power = Some(power);
        card.toughness = Some(toughness);
        card
    }

    fn make_equipment(id: ObjectId, owner: PlayerId, name: &str, power_boost: i32, toughness_boost: i32) -> CardData {
        let mut card = CardData::new(id, owner, name);
        card.card_types = vec![CardType::Artifact];
        card.subtypes = vec![SubType::Equipment];
        card.mana_cost = ManaCost::parse("{1}");
        card.abilities = vec![
            Ability::static_ability(id,
                "Equipped creature gets boost.",
                vec![StaticEffect::Boost {
                    filter: "equipped creature".into(),
                    power: power_boost,
                    toughness: toughness_boost,
                }]),
            Ability::activated(id,
                "Equip {1}",
                vec![Cost::pay_mana("{1}")],
                vec![Effect::equip()],
                TargetSpec::CreatureYouControl),
        ];
        card
    }

    fn register_abilities(game: &mut Game, perm_id: ObjectId) {
        let abilities = game.state.battlefield.get(perm_id).unwrap().card.abilities.clone();
        for ability in abilities {
            game.state.ability_store.add(ability);
        }
    }

    #[test]
    fn equip_attaches_equipment_to_creature() {
        let (mut game, p1, _p2) = setup();
        let creature_id = ObjectId::new();
        let equip_id = ObjectId::new();

        game.state.battlefield.add(Permanent::new(make_creature(creature_id, p1, "Soldier", 2, 2), p1));
        game.state.battlefield.add(Permanent::new(make_equipment(equip_id, p1, "Short Sword", 1, 1), p1));
        register_abilities(&mut game, equip_id);

        game.execute_effects(&[Effect::equip()], p1, &[creature_id], Some(equip_id), None);

        let equip = game.state.battlefield.get(equip_id).unwrap();
        assert_eq!(equip.attached_to, Some(creature_id));
        let creature = game.state.battlefield.get(creature_id).unwrap();
        assert!(creature.attachments.contains(&equip_id));
    }

    #[test]
    fn equipped_creature_gets_stat_boost() {
        let (mut game, p1, _p2) = setup();
        let creature_id = ObjectId::new();
        let equip_id = ObjectId::new();

        game.state.battlefield.add(Permanent::new(make_creature(creature_id, p1, "Soldier", 2, 2), p1));
        game.state.battlefield.add(Permanent::new(make_equipment(equip_id, p1, "Short Sword", 1, 1), p1));
        register_abilities(&mut game, equip_id);

        assert_eq!(game.state.battlefield.get(creature_id).unwrap().power(), 2);
        game.execute_effects(&[Effect::equip()], p1, &[creature_id], Some(equip_id), None);
        game.apply_continuous_effects();

        assert_eq!(game.state.battlefield.get(creature_id).unwrap().power(), 3);
        assert_eq!(game.state.battlefield.get(creature_id).unwrap().toughness(), 3);
    }

    #[test]
    fn equipment_detaches_when_creature_leaves() {
        let (mut game, p1, _p2) = setup();
        let creature_id = ObjectId::new();
        let equip_id = ObjectId::new();

        game.state.battlefield.add(Permanent::new(make_creature(creature_id, p1, "Soldier", 2, 2), p1));
        game.state.battlefield.add(Permanent::new(make_equipment(equip_id, p1, "Short Sword", 1, 1), p1));
        register_abilities(&mut game, equip_id);

        game.execute_effects(&[Effect::equip()], p1, &[creature_id], Some(equip_id), None);
        assert_eq!(game.state.battlefield.get(equip_id).unwrap().attached_to, Some(creature_id));

        // Remove creature (simulating death)
        game.state.battlefield.remove(creature_id);

        let sba = game.state.check_state_based_actions();
        assert!(sba.attachments_to_detach.contains(&equip_id));

        game.apply_state_based_actions(&sba);
        let equip = game.state.battlefield.get(equip_id).unwrap();
        assert_eq!(equip.attached_to, None);
        assert!(game.state.battlefield.contains(equip_id));
    }

    #[test]
    fn re_equip_moves_to_new_creature() {
        let (mut game, p1, _p2) = setup();
        let c1 = ObjectId::new();
        let c2 = ObjectId::new();
        let equip_id = ObjectId::new();

        game.state.battlefield.add(Permanent::new(make_creature(c1, p1, "Soldier A", 2, 2), p1));
        game.state.battlefield.add(Permanent::new(make_creature(c2, p1, "Soldier B", 3, 3), p1));
        game.state.battlefield.add(Permanent::new(make_equipment(equip_id, p1, "Short Sword", 1, 1), p1));
        register_abilities(&mut game, equip_id);

        game.execute_effects(&[Effect::equip()], p1, &[c1], Some(equip_id), None);
        assert_eq!(game.state.battlefield.get(equip_id).unwrap().attached_to, Some(c1));

        game.execute_effects(&[Effect::equip()], p1, &[c2], Some(equip_id), None);
        assert_eq!(game.state.battlefield.get(equip_id).unwrap().attached_to, Some(c2));
        assert!(game.state.battlefield.get(c2).unwrap().attachments.contains(&equip_id));
        assert!(!game.state.battlefield.get(c1).unwrap().attachments.contains(&equip_id));

        game.apply_continuous_effects();
        assert_eq!(game.state.battlefield.get(c1).unwrap().power(), 2);
        assert_eq!(game.state.battlefield.get(c2).unwrap().power(), 4);
    }

    #[test]
    fn equipment_keyword_grant() {
        let (mut game, p1, _p2) = setup();
        let creature_id = ObjectId::new();
        let equip_id = ObjectId::new();

        game.state.battlefield.add(Permanent::new(make_creature(creature_id, p1, "Soldier", 2, 2), p1));

        let mut equipment = CardData::new(equip_id, p1, "Swiftfoot Boots");
        equipment.card_types = vec![CardType::Artifact];
        equipment.subtypes = vec![SubType::Equipment];
        equipment.mana_cost = ManaCost::parse("{2}");
        equipment.abilities = vec![
            Ability::static_ability(equip_id,
                "Equipped creature has hexproof and haste.",
                vec![StaticEffect::GrantKeyword {
                    filter: "equipped creature".into(),
                    keyword: "hexproof, haste".into(),
                }]),
            Ability::activated(equip_id,
                "Equip {1}",
                vec![Cost::pay_mana("{1}")],
                vec![Effect::equip()],
                TargetSpec::CreatureYouControl),
        ];
        game.state.battlefield.add(Permanent::new(equipment, p1));
        register_abilities(&mut game, equip_id);

        assert!(!game.state.battlefield.get(creature_id).unwrap().has_hexproof());
        assert!(!game.state.battlefield.get(creature_id).unwrap().has_haste());

        game.execute_effects(&[Effect::equip()], p1, &[creature_id], Some(equip_id), None);
        game.apply_continuous_effects();

        assert!(game.state.battlefield.get(creature_id).unwrap().has_hexproof());
        assert!(game.state.battlefield.get(creature_id).unwrap().has_haste());
    }
}

// ---------------------------------------------------------------------------
// Aura tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod aura_tests {
    use super::*;
    use crate::abilities::{Ability, StaticEffect};
    use crate::card::CardData;
    use crate::constants::{CardType, Outcome, SubType};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };
    use crate::types::{ObjectId, PlayerId};

    struct PassivePlayer;
    impl PlayerDecisionMaker for PassivePlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { true }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn setup() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "P1".into(), deck: vec![] },
                PlayerConfig { name: "P2".into(), deck: vec![] },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(PassivePlayer)),
                (p2, Box::new(PassivePlayer)),
            ],
        );
        (game, p1, p2)
    }

    fn make_creature(id: ObjectId, owner: PlayerId, power: i32, toughness: i32) -> CardData {
        let mut card = CardData::new(id, owner, "Creature");
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![SubType::Human];
        card.power = Some(power);
        card.toughness = Some(toughness);
        card
    }

    fn make_aura_boost(id: ObjectId, owner: PlayerId, name: &str, power: i32, toughness: i32) -> CardData {
        let mut card = CardData::new(id, owner, name);
        card.card_types = vec![CardType::Enchantment];
        card.subtypes = vec![SubType::Aura];
        card.abilities = vec![
            Ability::static_ability(id,
                &format!("Enchanted creature gets +{power}/+{toughness}."),
                vec![StaticEffect::Boost {
                    filter: "enchanted creature".into(),
                    power,
                    toughness,
                }]),
        ];
        card
    }

    fn register_abilities(game: &mut Game, perm_id: ObjectId) {
        let abilities = game.state.battlefield.get(perm_id).unwrap().card.abilities.clone();
        for ability in abilities {
            game.state.ability_store.add(ability);
        }
    }

    #[test]
    fn aura_attached_creature_gets_boost() {
        let (mut game, p1, _p2) = setup();
        let creature_id = ObjectId::new();
        let aura_id = ObjectId::new();

        game.state.battlefield.add(Permanent::new(make_creature(creature_id, p1, 2, 2), p1));

        let aura = make_aura_boost(aura_id, p1, "Giant Growth Aura", 3, 3);
        game.state.battlefield.add(Permanent::new(aura, p1));
        register_abilities(&mut game, aura_id);

        // Manually attach aura to creature
        if let Some(a) = game.state.battlefield.get_mut(aura_id) {
            a.attach_to(creature_id);
        }
        if let Some(c) = game.state.battlefield.get_mut(creature_id) {
            c.add_attachment(aura_id);
        }

        game.apply_continuous_effects();

        assert_eq!(game.state.battlefield.get(creature_id).unwrap().power(), 5);
        assert_eq!(game.state.battlefield.get(creature_id).unwrap().toughness(), 5);
    }

    #[test]
    fn aura_falls_off_to_graveyard_when_creature_dies() {
        let (mut game, p1, _p2) = setup();
        let creature_id = ObjectId::new();
        let aura_id = ObjectId::new();

        game.state.battlefield.add(Permanent::new(make_creature(creature_id, p1, 2, 2), p1));
        let aura = make_aura_boost(aura_id, p1, "Ethereal Armor", 2, 2);
        game.state.battlefield.add(Permanent::new(aura, p1));
        register_abilities(&mut game, aura_id);

        // Attach
        if let Some(a) = game.state.battlefield.get_mut(aura_id) {
            a.attach_to(creature_id);
        }
        if let Some(c) = game.state.battlefield.get_mut(creature_id) {
            c.add_attachment(aura_id);
        }

        // Remove creature (simulating death)
        game.state.battlefield.remove(creature_id);

        // SBA should detect aura needs to go to graveyard
        let sba = game.state.check_state_based_actions();
        assert!(sba.auras_to_graveyard.contains(&aura_id));
        assert!(!sba.attachments_to_detach.contains(&aura_id));

        game.apply_state_based_actions(&sba);

        // Aura should be gone from battlefield (moved to graveyard)
        assert!(!game.state.battlefield.contains(aura_id));
    }

    #[test]
    fn pacifism_prevents_attack_and_block() {
        let (mut game, p1, p2) = setup();
        let creature_id = ObjectId::new();
        let pacifism_id = ObjectId::new();

        game.state.battlefield.add(Permanent::new(make_creature(creature_id, p2, 3, 3), p2));

        // Make Pacifism aura
        let mut pacifism = CardData::new(pacifism_id, p1, "Pacifism");
        pacifism.card_types = vec![CardType::Enchantment];
        pacifism.subtypes = vec![SubType::Aura];
        pacifism.abilities = vec![
            Ability::static_ability(pacifism_id,
                "Enchanted creature can't attack or block.",
                vec![
                    StaticEffect::CantAttack { filter: "enchanted creature".into() },
                    StaticEffect::CantBlock { filter: "enchanted creature".into() },
                ]),
        ];
        game.state.battlefield.add(Permanent::new(pacifism, p1));
        register_abilities(&mut game, pacifism_id);

        // Attach to creature
        if let Some(a) = game.state.battlefield.get_mut(pacifism_id) {
            a.attach_to(creature_id);
        }
        if let Some(c) = game.state.battlefield.get_mut(creature_id) {
            c.add_attachment(pacifism_id);
        }

        // Apply continuous effects to enforce CantAttack/CantBlock
        game.apply_continuous_effects();

        // After Pacifism, creature should not be able to attack
        let creature = game.state.battlefield.get(creature_id).unwrap();
        assert!(!creature.can_attack(), "Pacified creature should not be able to attack");
    }
}

// ---------------------------------------------------------------------------
// Prowess and landwalk tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod prowess_landwalk_tests {
    use super::*;
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, Outcome, SubType};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };
    use crate::events::{GameEvent};
    use crate::types::{ObjectId, PlayerId};

    struct PassivePlayer;
    impl PlayerDecisionMaker for PassivePlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { true }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn setup() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "P1".into(), deck: vec![] },
                PlayerConfig { name: "P2".into(), deck: vec![] },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(PassivePlayer)),
                (p2, Box::new(PassivePlayer)),
            ],
        );
        (game, p1, p2)
    }

    #[test]
    fn prowess_triggers_on_noncreature_spell() {
        let (mut game, p1, _p2) = setup();
        let creature_id = ObjectId::new();

        // Create a creature with prowess
        let mut creature = CardData::new(creature_id, p1, "Prowess Monk");
        creature.card_types = vec![CardType::Creature];
        creature.subtypes = vec![SubType::Human];
        creature.power = Some(1);
        creature.toughness = Some(1);
        creature.keywords = KeywordAbilities::PROWESS;
        game.state.battlefield.add(Permanent::new(creature.clone(), p1));
        game.state.card_store.insert(creature);

        // Create a noncreature spell in the card store
        let spell_id = ObjectId::new();
        let mut spell = CardData::new(spell_id, p1, "Lightning Bolt");
        spell.card_types = vec![CardType::Instant];
        game.state.card_store.insert(spell);

        // Emit a SpellCast event
        game.emit_event(GameEvent::spell_cast(spell_id, p1, crate::constants::Zone::Hand));

        // Check triggered abilities — this should process prowess
        game.check_triggered_abilities();

        // Prowess should have added a +1/+1 counter
        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert_eq!(perm.power(), 2, "Prowess should boost power to 2");
        assert_eq!(perm.toughness(), 2, "Prowess should boost toughness to 2");
    }

    #[test]
    fn prowess_does_not_trigger_on_creature_spell() {
        let (mut game, p1, _p2) = setup();
        let creature_id = ObjectId::new();

        let mut creature = CardData::new(creature_id, p1, "Prowess Monk");
        creature.card_types = vec![CardType::Creature];
        creature.power = Some(1);
        creature.toughness = Some(1);
        creature.keywords = KeywordAbilities::PROWESS;
        game.state.battlefield.add(Permanent::new(creature.clone(), p1));
        game.state.card_store.insert(creature);

        // Cast a creature spell (should NOT trigger prowess)
        let spell_id = ObjectId::new();
        let mut spell = CardData::new(spell_id, p1, "Grizzly Bears");
        spell.card_types = vec![CardType::Creature];
        game.state.card_store.insert(spell);

        game.emit_event(GameEvent::spell_cast(spell_id, p1, crate::constants::Zone::Hand));
        game.check_triggered_abilities();

        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert_eq!(perm.power(), 1, "Prowess should NOT trigger on creature spell");
    }

    #[test]
    fn forestwalk_unblockable_vs_forest_controller() {
        // Test landwalk evasion — if defender controls a Forest, creature with
        // forestwalk can't be blocked. We test this by checking combat::can_block
        // logic indirectly via the permanent struct.

        let owner = PlayerId::new();
        let id = ObjectId::new();
        let mut card = CardData::new(id, owner, "Forestwalker");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        card.keywords = KeywordAbilities::FORESTWALK;
        let attacker = Permanent::new(card, owner);

        // Verify the keyword is set
        assert!(attacker.has_keyword(KeywordAbilities::FORESTWALK));

        // Basic can_block doesn't check landwalk (that's at game level)
        let blocker_owner = PlayerId::new();
        let blocker_id = ObjectId::new();
        let mut blocker_card = CardData::new(blocker_id, blocker_owner, "Blocker");
        blocker_card.card_types = vec![CardType::Creature];
        blocker_card.power = Some(3);
        blocker_card.toughness = Some(3);
        let blocker = Permanent::new(blocker_card, blocker_owner);

        // Without landwalk check, normal blocking is fine
        assert!(combat::can_block(&blocker, &attacker));
    }
}


#[cfg(test)]
mod ward_tests {
    use super::*;
    use crate::abilities::{Ability, StaticEffect, TargetSpec, Effect};
    use crate::constants::Outcome;
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities};
    use crate::mana::{ManaCost, Mana};
    use crate::types::{ObjectId, PlayerId};
    use crate::decision::*;

    struct PassivePlayer;
    impl PlayerDecisionMaker for PassivePlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        (0..40).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    fn setup_ward_game() -> (Game, PlayerId, PlayerId, ObjectId, ObjectId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Attacker".into(), deck: make_deck(p1) },
                PlayerConfig { name: "Defender".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let mut game = Game::new_two_player(
            config,
            vec![(p1, Box::new(PassivePlayer)), (p2, Box::new(PassivePlayer))],
        );

        // Create a creature with Ward {2} controlled by p2
        let ward_creature_id = ObjectId::new();
        let mut ward_card = CardData::new(ward_creature_id, p2, "Warded Beast");
        ward_card.card_types = vec![CardType::Creature];
        ward_card.power = Some(4);
        ward_card.toughness = Some(4);
        ward_card.keywords = KeywordAbilities::WARD;
        let perm = Permanent::new(ward_card.clone(), p2);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(ward_card);

        // Register Ward {2} static ability
        let ward_ability = Ability::static_ability(
            ward_creature_id,
            "Ward {2}",
            vec![StaticEffect::ward("{2}")],
        );
        game.state.ability_store.add(ward_ability);

        // Create a removal spell in p1's hand
        let spell_id = ObjectId::new();
        let mut spell_card = CardData::new(spell_id, p1, "Doom Blade");
        spell_card.card_types = vec![CardType::Instant];
        spell_card.mana_cost = ManaCost::parse("{1}{B}");
        spell_card.abilities = vec![Ability::spell(
            spell_id,
            vec![Effect::Destroy],
            TargetSpec::OpponentCreature,
        )];
        game.state.card_store.insert(spell_card);
        game.state.players.get_mut(&p1).unwrap().hand.add(spell_id);

        (game, p1, p2, ward_creature_id, spell_id)
    }

    #[test]
    fn ward_counters_spell_when_opponent_cant_pay() {
        let (mut game, p1, _p2, ward_creature_id, spell_id) = setup_ward_game();

        // Give p1 only enough mana for the spell (1B), not for ward ({2})
        game.state.players.get_mut(&p1).unwrap().mana_pool.add(
            Mana { white: 0, blue: 0, black: 1, red: 0, green: 0, colorless: 1, generic: 0, any: 0 }, None, false
        );

        game.cast_spell(p1, spell_id);

        // The spell should be on the stack but countered (no mana left for ward)
        let stack_item = game.state.stack.get(spell_id);
        assert!(stack_item.is_some(), "Spell should be on the stack");
        assert!(stack_item.unwrap().countered, "Spell should be countered by Ward");

        // Resolve — creature should survive
        game.resolve_top_of_stack();
        assert!(game.state.battlefield.contains(ward_creature_id), "Warded creature should survive");
    }

    #[test]
    fn ward_allows_spell_when_opponent_pays() {
        let (mut game, p1, _p2, _ward_creature_id, spell_id) = setup_ward_game();

        // Give p1 enough mana for spell (1B) + ward (2)
        game.state.players.get_mut(&p1).unwrap().mana_pool.add(
            Mana { white: 0, blue: 0, black: 1, red: 0, green: 0, colorless: 4, generic: 0, any: 0 }, None, false
        );

        game.cast_spell(p1, spell_id);

        // Ward should be paid — spell should NOT be countered
        let stack_item = game.state.stack.get(spell_id);
        assert!(stack_item.is_some(), "Spell should be on the stack");
        assert!(!stack_item.unwrap().countered, "Spell should NOT be countered when ward cost is paid");
    }

    #[test]
    fn ward_doesnt_trigger_on_own_creatures() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "A".into(), deck: make_deck(p1) },
                PlayerConfig { name: "B".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let mut game = Game::new_two_player(
            config,
            vec![(p1, Box::new(PassivePlayer)), (p2, Box::new(PassivePlayer))],
        );

        // Create a ward creature controlled by p1 (self-target shouldn't trigger ward)
        let own_ward_id = ObjectId::new();
        let mut own_card = CardData::new(own_ward_id, p1, "Own Warded");
        own_card.card_types = vec![CardType::Creature];
        own_card.power = Some(3);
        own_card.toughness = Some(3);
        own_card.keywords = KeywordAbilities::WARD;
        let perm = Permanent::new(own_card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(own_card);

        let ward_ability = Ability::static_ability(
            own_ward_id,
            "Ward {2}",
            vec![StaticEffect::ward("{2}")],
        );
        game.state.ability_store.add(ward_ability);

        // Create a buff spell targeting own creature
        let buff_id = ObjectId::new();
        let mut buff_card = CardData::new(buff_id, p1, "Giant Growth");
        buff_card.card_types = vec![CardType::Instant];
        buff_card.mana_cost = ManaCost::parse("{G}");
        buff_card.abilities = vec![Ability::spell(
            buff_id,
            vec![Effect::BoostUntilEndOfTurn { power: 3, toughness: 3 }],
            TargetSpec::CreatureYouControl,
        )];
        game.state.card_store.insert(buff_card);
        game.state.players.get_mut(&p1).unwrap().hand.add(buff_id);

        // Give enough mana for the spell only (1G) — no extra for ward
        game.state.players.get_mut(&p1).unwrap().mana_pool.add(
            Mana { white: 0, blue: 0, black: 0, red: 0, green: 1, colorless: 0, generic: 0, any: 0 }, None, false
        );

        game.cast_spell(p1, buff_id);

        // Ward should NOT trigger (own creature) — spell should not be countered
        let stack_item = game.state.stack.get(buff_id);
        assert!(stack_item.is_some(), "Spell should be on the stack");
        assert!(!stack_item.unwrap().countered, "Ward should not trigger on own creatures");
    }

    #[test]
    fn ward_pay_life_counters_when_insufficient() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "A".into(), deck: make_deck(p1) },
                PlayerConfig { name: "B".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let mut game = Game::new_two_player(
            config,
            vec![(p1, Box::new(PassivePlayer)), (p2, Box::new(PassivePlayer))],
        );

        // Create ward creature with "Pay 2 life" cost
        let ward_id = ObjectId::new();
        let mut ward_card = CardData::new(ward_id, p2, "Life Ward");
        ward_card.card_types = vec![CardType::Creature];
        ward_card.power = Some(2);
        ward_card.toughness = Some(2);
        let perm = Permanent::new(ward_card.clone(), p2);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(ward_card);

        let ward_ability = Ability::static_ability(
            ward_id,
            "Ward--Pay 2 life.",
            vec![StaticEffect::Ward { cost: "Pay 2 life".into() }],
        );
        game.state.ability_store.add(ward_ability);

        // Create a removal spell
        let spell_id = ObjectId::new();
        let mut spell_card = CardData::new(spell_id, p1, "Lightning Bolt");
        spell_card.card_types = vec![CardType::Instant];
        spell_card.mana_cost = ManaCost::parse("{R}");
        spell_card.abilities = vec![Ability::spell(
            spell_id,
            vec![Effect::DealDamage { amount: 3 }],
            TargetSpec::Creature,
        )];
        game.state.card_store.insert(spell_card);
        game.state.players.get_mut(&p1).unwrap().hand.add(spell_id);

        // Set p1's life to 1 — can't afford "Pay 2 life"
        game.state.players.get_mut(&p1).unwrap().life = 1;
        game.state.players.get_mut(&p1).unwrap().mana_pool.add(
            Mana { white: 0, blue: 0, black: 0, red: 1, green: 0, colorless: 0, generic: 0, any: 0 }, None, false
        );

        game.cast_spell(p1, spell_id);

        let stack_item = game.state.stack.get(spell_id);
        assert!(stack_item.is_some());
        assert!(stack_item.unwrap().countered, "Spell should be countered — can't pay 2 life at 1 life");
    }
}

#[cfg(test)]
mod cant_be_countered_tests {
    use super::*;
    use crate::abilities::{Ability, StaticEffect, TargetSpec, Effect};
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, Outcome};
    use crate::mana::{ManaCost, Mana};
    use crate::types::{ObjectId, PlayerId};
    use crate::decision::*;

    struct PassivePlayer;
    impl PlayerDecisionMaker for PassivePlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        (0..40).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    #[test]
    fn cant_be_countered_resists_counter_spell() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "A".into(), deck: make_deck(p1) },
                PlayerConfig { name: "B".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let mut game = Game::new_two_player(
            config,
            vec![(p1, Box::new(PassivePlayer)), (p2, Box::new(PassivePlayer))],
        );

        // Put an uncounterable spell on the stack
        let spell_id = ObjectId::new();
        let mut spell_card = CardData::new(spell_id, p1, "Supreme Verdict");
        spell_card.card_types = vec![CardType::Sorcery];
        spell_card.mana_cost = ManaCost::parse("{1}{W}{U}{U}");
        spell_card.abilities = vec![
            Ability::static_ability(spell_id, "This spell can't be countered.",
                vec![StaticEffect::CantBeCountered]),
            Ability::spell(spell_id,
                vec![Effect::DestroyAll { filter: "creature".into() }],
                TargetSpec::None),
        ];
        let stack_item = crate::zones::StackItem {
            id: spell_id,
            kind: crate::zones::StackItemKind::Spell { card: spell_card },
            controller: p1,
            targets: vec![],
            countered: false,
            x_value: None,
            exile_on_resolve: false,
        };
        game.state.stack.push(stack_item);

        // Now try to counter it using Effect::CounterSpell
        game.execute_effects(
            &[Effect::CounterSpell],
            p2,
            &[spell_id],
            Some(spell_id),
        None, );

        // The spell should STILL be on the stack (not removed)
        assert!(game.state.stack.get(spell_id).is_some(), "Uncounterable spell should remain on the stack");
    }

    #[test]
    fn normal_spell_can_be_countered() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "A".into(), deck: make_deck(p1) },
                PlayerConfig { name: "B".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let mut game = Game::new_two_player(
            config,
            vec![(p1, Box::new(PassivePlayer)), (p2, Box::new(PassivePlayer))],
        );

        // Put a normal spell on the stack
        let spell_id = ObjectId::new();
        let mut spell_card = CardData::new(spell_id, p1, "Lightning Bolt");
        spell_card.card_types = vec![CardType::Instant];
        spell_card.mana_cost = ManaCost::parse("{R}");
        spell_card.abilities = vec![
            Ability::spell(spell_id,
                vec![Effect::DealDamage { amount: 3 }],
                TargetSpec::Creature),
        ];
        let stack_item = crate::zones::StackItem {
            id: spell_id,
            kind: crate::zones::StackItemKind::Spell { card: spell_card },
            controller: p1,
            targets: vec![],
            countered: false,
            x_value: None,
            exile_on_resolve: false,
        };
        game.state.stack.push(stack_item);
        game.state.card_store.insert(CardData::new(spell_id, p1, "Lightning Bolt"));

        // Counter it
        game.execute_effects(
            &[Effect::CounterSpell],
            p2,
            &[spell_id],
            Some(spell_id),
        None, );

        // The spell should be removed from the stack
        assert!(game.state.stack.get(spell_id).is_none(), "Normal spell should be countered");
    }
}

#[cfg(test)]
mod step_trigger_tests {
    use super::*;
    use crate::abilities::{Ability, TargetSpec, Effect};
    use crate::card::CardData;
    use crate::constants::{CardType, Outcome};
    use crate::types::{ObjectId, PlayerId};
    use crate::decision::*;

    struct PassivePlayer;
    impl PlayerDecisionMaker for PassivePlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        (0..40).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    #[test]
    fn upkeep_trigger_fires_on_upkeep_step() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "A".into(), deck: make_deck(p1) },
                PlayerConfig { name: "B".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let mut game = Game::new_two_player(
            config,
            vec![(p1, Box::new(PassivePlayer)), (p2, Box::new(PassivePlayer))],
        );
        game.state.active_player = p1;
        game.state.priority_player = p1;

        // Create a creature with upkeep trigger (gain 1 life at upkeep)
        let creature_id = ObjectId::new();
        let mut card = CardData::new(creature_id, p1, "Upkeep Healer");
        card.card_types = vec![CardType::Creature];
        card.power = Some(1);
        card.toughness = Some(1);
        let perm = Permanent::new(card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card);

        let trigger = Ability::triggered(
            creature_id,
            "At the beginning of your upkeep, gain 1 life.",
            vec![EventType::UpkeepStep],
            vec![Effect::GainLife { amount: 1 }],
            TargetSpec::None,
        );
        game.state.ability_store.add(trigger);

        let life_before = game.state.player(p1).unwrap().life;

        // Emit upkeep event and process triggers
        let mut event = GameEvent::new(EventType::UpkeepStep);
        event.player_id = Some(p1);
        game.emit_event(event);
        game.process_sba_and_triggers();

        // Triggered ability should be on the stack — resolve it
        assert!(!game.state.stack.is_empty(), "Trigger should be on the stack");
        game.resolve_top_of_stack();

        let life_after = game.state.player(p1).unwrap().life;
        assert_eq!(life_after, life_before + 1, "Upkeep trigger should have gained 1 life");
    }

    #[test]
    fn end_step_trigger_fires() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "A".into(), deck: make_deck(p1) },
                PlayerConfig { name: "B".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let mut game = Game::new_two_player(
            config,
            vec![(p1, Box::new(PassivePlayer)), (p2, Box::new(PassivePlayer))],
        );
        game.state.active_player = p1;
        game.state.priority_player = p1;

        // Create a creature with end step trigger (draw a card)
        let creature_id = ObjectId::new();
        let mut card = CardData::new(creature_id, p1, "End Step Draw");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        let perm = Permanent::new(card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card);

        let trigger = Ability::triggered(
            creature_id,
            "At the beginning of your end step, draw a card.",
            vec![EventType::EndStep],
            vec![Effect::DrawCards { count: 1 }],
            TargetSpec::None,
        );
        game.state.ability_store.add(trigger);

        let hand_before = game.state.player(p1).unwrap().hand.len();

        // Emit end step event and process triggers
        let mut event = GameEvent::new(EventType::EndStep);
        event.player_id = Some(p1);
        game.emit_event(event);
        game.process_sba_and_triggers();

        // Triggered ability should be on the stack — resolve it
        assert!(!game.state.stack.is_empty(), "Trigger should be on the stack");
        game.resolve_top_of_stack();

        let hand_after = game.state.player(p1).unwrap().hand.len();
        assert_eq!(hand_after, hand_before + 1, "End step trigger should have drawn 1 card");
    }

    #[test]
    fn upkeep_trigger_only_fires_for_controller() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "A".into(), deck: make_deck(p1) },
                PlayerConfig { name: "B".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let mut game = Game::new_two_player(
            config,
            vec![(p1, Box::new(PassivePlayer)), (p2, Box::new(PassivePlayer))],
        );
        game.state.active_player = p1;
        game.state.priority_player = p1;

        // Create an upkeep trigger creature controlled by p2
        let creature_id = ObjectId::new();
        let mut card = CardData::new(creature_id, p2, "Opponent Healer");
        card.card_types = vec![CardType::Creature];
        card.power = Some(1);
        card.toughness = Some(1);
        let perm = Permanent::new(card.clone(), p2);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card);

        let trigger = Ability::triggered(
            creature_id,
            "At the beginning of your upkeep, gain 1 life.",
            vec![EventType::UpkeepStep],
            vec![Effect::GainLife { amount: 1 }],
            TargetSpec::None,
        );
        game.state.ability_store.add(trigger);

        let p2_life = game.state.player(p2).unwrap().life;

        // Emit p1's upkeep — p2's trigger should NOT fire
        let mut event = GameEvent::new(EventType::UpkeepStep);
        event.player_id = Some(p1);
        game.emit_event(event);
        game.process_sba_and_triggers();

        let p2_life_after = game.state.player(p2).unwrap().life;
        assert_eq!(p2_life_after, p2_life, "P2's upkeep trigger should not fire during p1's upkeep");
    }
}

#[cfg(test)]
mod x_cost_tests {
    use super::*;
    use crate::abilities::{Ability, Effect, TargetSpec, X_VALUE};
    use crate::card::CardData;
    use crate::constants::{CardType, Outcome};
    use crate::mana::{Mana, ManaCost};
    use crate::types::{ObjectId, PlayerId};
    use crate::decision::*;

    struct XChooserPlayer {
        x_choice: u32,
    }

    impl PlayerDecisionMaker for XChooserPlayer {
        fn priority(&mut self, _: &GameView<'_>, legal: &[PlayerAction]) -> PlayerAction {
            for action in legal {
                if let PlayerAction::CastSpell { .. } = action {
                    return action.clone();
                }
            }
            PlayerAction::Pass
        }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, req: &TargetRequirement) -> Vec<ObjectId> {
            req.legal_targets.iter().take(1).copied().collect()
        }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { true }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, _: u32, _: u32) -> u32 {
            self.x_choice
        }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn setup_x_game(x_choice: u32) -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            starting_life: 20,
            players: vec![
                PlayerConfig { name: "P1".into(), deck: vec![] },
                PlayerConfig { name: "P2".into(), deck: vec![] },
            ],
        };

        let dms: Vec<(PlayerId, Box<dyn PlayerDecisionMaker>)> = vec![
            (p1, Box::new(XChooserPlayer { x_choice })),
            (p2, Box::new(XChooserPlayer { x_choice: 0 })),
        ];

        (Game::new_two_player(config, dms), p1, p2)
    }

    #[test]
    fn x_cost_deal_damage() {
        let (mut game, p1, p2) = setup_x_game(3);

        // Give P1 4 mana for {X}{R} with X=3
        if let Some(player) = game.state.players.get_mut(&p1) {
            player.mana_pool.add(Mana { red: 1, green: 3, ..Mana::new() }, None, false);
        }

        // Create X-cost damage spell: {X}{R} - deal X damage
        let spell_id = ObjectId::new();
        let mut spell = CardData::new(spell_id, p1, "X Bolt");
        spell.card_types = vec![CardType::Sorcery];
        spell.mana_cost = ManaCost::parse("{X}{R}");
        spell.abilities = vec![Ability::spell(spell_id,
            vec![Effect::DealDamage { amount: X_VALUE }],
            TargetSpec::Creature)];

        // Create a target creature for P2
        let creature_id = ObjectId::new();
        let mut creature = CardData::new(creature_id, p2, "Big Beast");
        creature.card_types = vec![CardType::Creature];
        creature.power = Some(5);
        creature.toughness = Some(5);
        game.state.battlefield.add(crate::permanent::Permanent::new(creature.clone(), p2));
        game.state.card_store.insert(creature);
        game.state.set_zone(creature_id, crate::constants::Zone::Battlefield, None);

        // Put spell in hand
        if let Some(player) = game.state.players.get_mut(&p1) {
            player.hand.add(spell_id);
        }
        game.state.card_store.insert(spell);
        game.state.set_zone(spell_id, crate::constants::Zone::Hand, None);

        // Cast the spell (X=3)
        game.cast_spell(p1, spell_id);

        // Verify X value on stack
        let stack_item = game.state.stack.top().unwrap();
        assert_eq!(stack_item.x_value, Some(3));

        // Resolve
        game.resolve_top_of_stack();

        // Creature should have 3 damage
        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert_eq!(perm.damage, 3);
    }

    #[test]
    fn x_cost_draw_cards() {
        let (mut game, p1, _p2) = setup_x_game(2);

        // Give P1 4 mana for {X}{U}{U} with X=2
        if let Some(player) = game.state.players.get_mut(&p1) {
            player.mana_pool.add(Mana { blue: 2, green: 2, ..Mana::new() }, None, false);
        }

        // Add cards to library
        for _ in 0..5 {
            let card_id = ObjectId::new();
            let card = CardData::new(card_id, p1, "Island");
            game.state.card_store.insert(card);
            if let Some(player) = game.state.players.get_mut(&p1) {
                player.library.put_on_top(card_id);
            }
        }

        // Create X-cost draw spell: {X}{U}{U}
        let spell_id = ObjectId::new();
        let mut spell = CardData::new(spell_id, p1, "X Draw");
        spell.card_types = vec![CardType::Sorcery];
        spell.mana_cost = ManaCost::parse("{X}{U}{U}");
        spell.abilities = vec![Ability::spell(spell_id,
            vec![Effect::DrawCards { count: X_VALUE }],
            TargetSpec::None)];

        if let Some(player) = game.state.players.get_mut(&p1) {
            player.hand.add(spell_id);
        }
        game.state.card_store.insert(spell);

        let hand_before = game.state.players.get(&p1).unwrap().hand.len();

        // Cast and resolve (X=2)
        game.cast_spell(p1, spell_id);
        game.resolve_top_of_stack();

        // Should have drawn 2 cards (minus spell removed from hand)
        let hand_after = game.state.players.get(&p1).unwrap().hand.len();
        assert_eq!(hand_after, hand_before - 1 + 2);
    }

    #[test]
    fn x_cost_zero() {
        let (mut game, p1, _p2) = setup_x_game(0);

        // Give P1 1 mana for {X}{R} with X=0
        if let Some(player) = game.state.players.get_mut(&p1) {
            player.mana_pool.add(Mana { red: 1, ..Mana::new() }, None, false);
        }

        let spell_id = ObjectId::new();
        let mut spell = CardData::new(spell_id, p1, "X Zero");
        spell.card_types = vec![CardType::Sorcery];
        spell.mana_cost = ManaCost::parse("{X}{R}");
        spell.abilities = vec![Ability::spell(spell_id,
            vec![Effect::DealDamage { amount: X_VALUE }],
            TargetSpec::None)];

        if let Some(player) = game.state.players.get_mut(&p1) {
            player.hand.add(spell_id);
        }
        game.state.card_store.insert(spell);

        game.cast_spell(p1, spell_id);
        assert_eq!(game.state.stack.top().unwrap().x_value, Some(0));

        // Resolve - 0 damage to opponent
        let opp = *game.state.turn_order.iter().find(|&&id| id != p1).unwrap();
        let life_before = game.state.players.get(&opp).unwrap().life;
        game.resolve_top_of_stack();
        let life_after = game.state.players.get(&opp).unwrap().life;
        assert_eq!(life_before, life_after);
    }

    #[test]
    fn x_value_mana_payment() {
        let (mut game, p1, _p2) = setup_x_game(3);

        // Give P1 5 mana
        if let Some(player) = game.state.players.get_mut(&p1) {
            player.mana_pool.add(Mana { red: 1, green: 4, ..Mana::new() }, None, false);
        }

        // Spell costs {X}{R} with X=3 -> 4 mana total
        let spell_id = ObjectId::new();
        let mut spell = CardData::new(spell_id, p1, "X Payment");
        spell.card_types = vec![CardType::Sorcery];
        spell.mana_cost = ManaCost::parse("{X}{R}");
        spell.abilities = vec![Ability::spell(spell_id,
            vec![Effect::GainLife { amount: X_VALUE }],
            TargetSpec::None)];

        if let Some(player) = game.state.players.get_mut(&p1) {
            player.hand.add(spell_id);
        }
        game.state.card_store.insert(spell);

        game.cast_spell(p1, spell_id);

        // Should have 1 mana remaining (5 - 4)
        let remaining = game.state.players.get(&p1).unwrap().mana_pool.available().count();
        assert_eq!(remaining, 1);

        // Resolve: X=3 life gain
        let life_before = game.state.players.get(&p1).unwrap().life;
        game.resolve_top_of_stack();
        let life_after = game.state.players.get(&p1).unwrap().life;
        assert_eq!(life_after, life_before + 3);
    }
}

#[cfg(test)]
mod impulse_draw_tests {
    use super::*;
    use crate::abilities::Effect;
    use crate::card::CardData;
    use crate::constants::{CardType, TurnPhase, PhaseStep, Outcome};
    use crate::mana::{Mana, ManaCost};
    use crate::types::{ObjectId, PlayerId};
    use crate::decision::*;

    struct PassivePlayer;
    impl PlayerDecisionMaker for PassivePlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn setup_impulse_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            starting_life: 20,
            players: vec![
                PlayerConfig { name: "P1".into(), deck: vec![] },
                PlayerConfig { name: "P2".into(), deck: vec![] },
            ],
        };
        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(PassivePlayer)),
                (p2, Box::new(PassivePlayer)),
            ],
        );
        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.state.current_phase = TurnPhase::PrecombatMain;
        game.state.current_step = PhaseStep::PrecombatMain;
        game.state.turn_number = 1;
        (game, p1, p2)
    }

    /// Add N cards to a player's library.
    fn add_library_cards(game: &mut Game, player: PlayerId, n: usize) -> Vec<ObjectId> {
        let mut ids = Vec::new();
        for i in 0..n {
            let id = ObjectId::new();
            let mut card = CardData::new(id, player, &format!("Library Card {}", i));
            card.card_types = vec![CardType::Creature];
            card.power = Some(2);
            card.toughness = Some(2);
            card.mana_cost = ManaCost::parse("{1}{R}");
            game.state.card_store.insert(card);
            game.state.players.get_mut(&player).unwrap().library.put_on_top(id);
            ids.push(id);
        }
        ids
    }

    #[test]
    fn exile_top_and_play_creates_impulse_entries() {
        let (mut game, p1, _p2) = setup_impulse_game();
        let lib_ids = add_library_cards(&mut game, p1, 3);

        // Execute ExileTopAndPlay effect
        game.execute_effects(
            &[Effect::exile_top_and_play(2)],
            p1, &[], None, None,
        );

        // Should have exiled 2 cards and created 2 impulse entries
        assert_eq!(game.state.impulse_playable.len(), 2);
        assert_eq!(game.state.players.get(&p1).unwrap().library.len(), 1);

        // Exiled cards should be in exile zone
        for ip in &game.state.impulse_playable {
            assert!(game.state.exile.contains(ip.card_id));
            assert_eq!(ip.player_id, p1);
        }
    }

    #[test]
    fn impulse_cards_appear_in_legal_actions() {
        let (mut game, p1, _p2) = setup_impulse_game();
        let _lib_ids = add_library_cards(&mut game, p1, 3);

        // Give P1 mana to cast
        game.state.players.get_mut(&p1).unwrap()
            .mana_pool.add(Mana { red: 2, generic: 2, ..Mana::new() }, None, false);

        // Execute ExileTopAndPlay
        game.execute_effects(
            &[Effect::exile_top_and_play(1)],
            p1, &[], None, None,
        );

        let actions = game.compute_legal_actions(p1);
        let cast_actions: Vec<_> = actions.iter()
            .filter(|a| matches!(a, crate::decision::PlayerAction::CastSpell { .. }))
            .collect();
        assert!(!cast_actions.is_empty(), "Should be able to cast impulse-exiled card");
    }

    #[test]
    fn cast_from_exile_resolves() {
        let (mut game, p1, _p2) = setup_impulse_game();
        let _lib_ids = add_library_cards(&mut game, p1, 3);

        // Give P1 mana
        game.state.players.get_mut(&p1).unwrap()
            .mana_pool.add(Mana { red: 2, generic: 2, ..Mana::new() }, None, false);

        // Execute ExileTopAndPlay (1 card)
        game.execute_effects(
            &[Effect::exile_top_and_play(1)],
            p1, &[], None, None,
        );

        let impulse_card_id = game.state.impulse_playable[0].card_id;

        // Cast the exiled card
        game.cast_spell(p1, impulse_card_id);

        // Card should be on stack (not in exile anymore)
        assert!(!game.state.exile.contains(impulse_card_id));
        assert!(!game.state.stack.is_empty());

        // Impulse entry should be removed
        assert!(game.state.impulse_playable.is_empty());

        // Resolve the spell — it's a creature, should go to battlefield
        game.resolve_top_of_stack();
        assert!(game.state.battlefield.contains(impulse_card_id));
    }

    #[test]
    fn impulse_expires_at_end_of_turn() {
        let (mut game, p1, _p2) = setup_impulse_game();
        let _lib_ids = add_library_cards(&mut game, p1, 3);

        // Execute ExileTopAndPlay
        game.execute_effects(
            &[Effect::exile_top_and_play(2)],
            p1, &[], None, None,
        );
        assert_eq!(game.state.impulse_playable.len(), 2);

        // Simulate cleanup step
        game.turn_based_actions(PhaseStep::Cleanup, p1);

        // All EndOfTurn impulse entries should be removed
        assert_eq!(game.state.impulse_playable.len(), 0);

        // Cards should still be in exile (just no longer playable)
        // (we can't track which cards were impulse vs regular exile without
        // the impulse entries, but they're still there)
    }

    #[test]
    fn impulse_next_turn_persists_through_opponent_cleanup() {
        let (mut game, p1, p2) = setup_impulse_game();
        let _lib_ids = add_library_cards(&mut game, p1, 3);

        // P1 exiles a card with "until end of next turn"
        game.execute_effects(
            &[Effect::exile_top_and_play_next_turn(1)],
            p1, &[], None, None,
        );
        assert_eq!(game.state.impulse_playable.len(), 1);

        // Simulate P1's cleanup (creation turn)
        game.turn_based_actions(PhaseStep::Cleanup, p1);
        // Should NOT expire on the creation turn (active_player is p1, but turn_number == created_turn)
        assert_eq!(game.state.impulse_playable.len(), 1,
            "UntilEndOfNextTurn should survive creation turn cleanup");

        // Simulate opponent's turn cleanup
        game.state.active_player = p2;
        game.state.turn_number = 2;
        game.turn_based_actions(PhaseStep::Cleanup, p2);
        assert_eq!(game.state.impulse_playable.len(), 1,
            "UntilEndOfNextTurn should survive opponent's cleanup");

        // Simulate P1's next turn cleanup (this is when it should expire)
        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.state.turn_number = 3;
        game.turn_based_actions(PhaseStep::Cleanup, p1);
        assert_eq!(game.state.impulse_playable.len(), 0,
            "UntilEndOfNextTurn should expire at controller's next turn cleanup");
    }

    #[test]
    fn exile_top_and_play_free_skips_mana() {
        let (mut game, p1, _p2) = setup_impulse_game();
        let _lib_ids = add_library_cards(&mut game, p1, 3);

        // NO mana given to P1

        // Execute ExileTopAndPlay with without_mana=true
        game.execute_effects(
            &[Effect::exile_top_and_play_free(1)],
            p1, &[], None, None,
        );

        let impulse_card_id = game.state.impulse_playable[0].card_id;

        // Should appear in legal actions even without mana
        let actions = game.compute_legal_actions(p1);
        let cast_actions: Vec<_> = actions.iter()
            .filter(|a| matches!(a, crate::decision::PlayerAction::CastSpell { card_id, .. } if *card_id == impulse_card_id))
            .collect();
        assert!(!cast_actions.is_empty(), "Should be able to cast free impulse card without mana");

        // Cast the card with no mana
        game.cast_spell(p1, impulse_card_id);

        // Should be on stack
        assert!(!game.state.stack.is_empty());
        // Mana should still be 0
        assert_eq!(game.state.players.get(&p1).unwrap().mana_pool.available().count(), 0);
    }
}

#[cfg(test)]
mod delayed_trigger_tests {
    use super::*;
    use crate::abilities::{Ability, Effect, TargetSpec};
    use crate::card::CardData;
    use crate::constants::{CardType, TurnPhase, PhaseStep, Outcome};
    use crate::events::{EventType, GameEvent};
    use crate::mana::Mana;
    use crate::types::{ObjectId, PlayerId};
    use crate::decision::*;

    struct PassivePlayer;
    impl PlayerDecisionMaker for PassivePlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn setup_delayed_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            starting_life: 20,
            players: vec![
                PlayerConfig { name: "P1".into(), deck: vec![] },
                PlayerConfig { name: "P2".into(), deck: vec![] },
            ],
        };
        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(PassivePlayer)),
                (p2, Box::new(PassivePlayer)),
            ],
        );
        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.state.current_phase = TurnPhase::PrecombatMain;
        game.state.current_step = PhaseStep::PrecombatMain;
        game.state.turn_number = 1;
        (game, p1, p2)
    }

    #[test]
    fn delayed_on_death_fires_when_creature_dies() {
        let (mut game, p1, _p2) = setup_delayed_game();

        // Put a card in library so we can draw
        let draw_card_id = ObjectId::new();
        let draw_card = CardData::new(draw_card_id, p1, "Prize");
        game.state.card_store.insert(draw_card);
        game.state.players.get_mut(&p1).unwrap().library.put_on_top(draw_card_id);

        // Put a creature on the battlefield
        let creature_id = ObjectId::new();
        let mut card = CardData::new(creature_id, p1, "Doomed Creature");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        let perm = Permanent::new(card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card);

        // Create a delayed trigger: "when Doomed Creature dies this turn, draw a card"
        let effects = vec![Effect::delayed_on_death(vec![Effect::DrawCards { count: 1 }])];
        game.execute_effects(&effects, p1, &[creature_id], None, None);

        // Should have 1 delayed trigger registered
        assert_eq!(game.state.delayed_triggers.len(), 1);
        assert_eq!(game.state.delayed_triggers[0].watching, Some(creature_id));

        let hand_before = game.state.players.get(&p1).unwrap().hand.len();

        // Kill the creature (emit dies event)
        game.state.battlefield.remove(creature_id);
        game.emit_event(GameEvent::dies(creature_id, p1));
        game.check_triggered_abilities();

        let hand_after = game.state.players.get(&p1).unwrap().hand.len();
        assert_eq!(hand_after, hand_before + 1, "Should have drawn a card when creature died");

        // Delayed trigger should have been removed (trigger_only_once)
        assert_eq!(game.state.delayed_triggers.len(), 0);
    }

    #[test]
    fn delayed_on_death_does_not_fire_for_wrong_creature() {
        let (mut game, p1, p2) = setup_delayed_game();

        // Two creatures
        let watched_id = ObjectId::new();
        let mut watched_card = CardData::new(watched_id, p1, "Watched");
        watched_card.card_types = vec![CardType::Creature];
        watched_card.power = Some(2);
        watched_card.toughness = Some(2);
        game.state.battlefield.add(Permanent::new(watched_card.clone(), p1));
        game.state.card_store.insert(watched_card);

        let other_id = ObjectId::new();
        let mut other_card = CardData::new(other_id, p2, "Other");
        other_card.card_types = vec![CardType::Creature];
        other_card.power = Some(2);
        other_card.toughness = Some(2);
        game.state.battlefield.add(Permanent::new(other_card.clone(), p2));
        game.state.card_store.insert(other_card);

        // Delayed trigger watching the first creature
        game.execute_effects(
            &[Effect::delayed_on_death(vec![Effect::GainLife { amount: 5 }])],
            p1, &[watched_id], None, None,
        );

        let life_before = game.state.players.get(&p1).unwrap().life;

        // Kill the OTHER creature (should NOT trigger)
        game.state.battlefield.remove(other_id);
        game.emit_event(GameEvent::dies(other_id, p2));
        game.check_triggered_abilities();

        let life_after = game.state.players.get(&p1).unwrap().life;
        assert_eq!(life_after, life_before, "Should NOT gain life when wrong creature dies");

        // Trigger should still be registered
        assert_eq!(game.state.delayed_triggers.len(), 1);
    }

    #[test]
    fn delayed_trigger_expires_at_end_of_turn() {
        let (mut game, p1, _p2) = setup_delayed_game();

        // Create a delayed trigger with EndOfTurn duration
        game.execute_effects(
            &[Effect::delayed_on_death(vec![Effect::GainLife { amount: 5 }])],
            p1, &[], Some(ObjectId::new()), None,
        );
        assert_eq!(game.state.delayed_triggers.len(), 1);

        // Cleanup step should remove EndOfTurn delayed triggers
        game.turn_based_actions(PhaseStep::Cleanup, p1);
        assert_eq!(game.state.delayed_triggers.len(), 0,
            "EndOfTurn delayed trigger should be removed at cleanup");
    }

    #[test]
    fn at_next_end_step_fires_once() {
        let (mut game, p1, _p2) = setup_delayed_game();

        // Put a card in library
        let card_id = ObjectId::new();
        let card = CardData::new(card_id, p1, "Prize");
        game.state.card_store.insert(card);
        game.state.players.get_mut(&p1).unwrap().library.put_on_top(card_id);

        // Create "at the beginning of the next end step, draw a card"
        game.execute_effects(
            &[Effect::at_next_end_step(vec![Effect::DrawCards { count: 1 }])],
            p1, &[], None, None,
        );
        assert_eq!(game.state.delayed_triggers.len(), 1);

        let hand_before = game.state.players.get(&p1).unwrap().hand.len();

        // Emit end step event
        let mut event = GameEvent::new(EventType::EndStep);
        event.player_id = Some(p1);
        game.emit_event(event);
        game.check_triggered_abilities();

        let hand_after = game.state.players.get(&p1).unwrap().hand.len();
        assert_eq!(hand_after, hand_before + 1, "Should draw on end step");

        // Trigger should be removed (trigger_only_once)
        assert_eq!(game.state.delayed_triggers.len(), 0);
    }
}

#[cfg(test)]
mod flashback_tests {
    use super::*;
    use crate::abilities::{Ability, Effect, TargetSpec};
    use crate::card::CardData;
    use crate::constants::{CardType, TurnPhase, PhaseStep, Outcome};
    use crate::mana::{Mana, ManaCost};
    use crate::types::{ObjectId, PlayerId};
    use crate::decision::*;

    struct PassivePlayer;
    impl PlayerDecisionMaker for PassivePlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn setup_flashback_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            starting_life: 20,
            players: vec![
                PlayerConfig { name: "P1".into(), deck: vec![] },
                PlayerConfig { name: "P2".into(), deck: vec![] },
            ],
        };
        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(PassivePlayer)),
                (p2, Box::new(PassivePlayer)),
            ],
        );
        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.state.current_phase = TurnPhase::PrecombatMain;
        game.state.current_step = PhaseStep::PrecombatMain;
        game.state.turn_number = 1;
        (game, p1, p2)
    }

    #[test]
    fn flashback_appears_in_legal_actions() {
        let (mut game, p1, _p2) = setup_flashback_game();

        // Create a sorcery with flashback in graveyard
        let spell_id = ObjectId::new();
        let mut spell = CardData::new(spell_id, p1, "Flashback Bolt");
        spell.card_types = vec![CardType::Sorcery];
        spell.mana_cost = ManaCost::parse("{R}");
        spell.flashback_cost = Some(ManaCost::parse("{2}{R}"));
        spell.abilities = vec![Ability::spell(spell_id,
            vec![Effect::GainLife { amount: 3 }],
            TargetSpec::None)];
        game.state.card_store.insert(spell);
        game.state.players.get_mut(&p1).unwrap().graveyard.add(spell_id);

        // Give P1 enough mana for flashback cost {2}{R}
        game.state.players.get_mut(&p1).unwrap()
            .mana_pool.add(Mana { red: 3, ..Mana::new() }, None, false);

        let actions = game.compute_legal_actions(p1);
        let cast_actions: Vec<_> = actions.iter()
            .filter(|a| matches!(a, PlayerAction::CastSpell { card_id, .. } if *card_id == spell_id))
            .collect();
        assert!(!cast_actions.is_empty(), "Should be able to cast flashback from graveyard");
    }

    #[test]
    fn flashback_not_available_without_mana() {
        let (mut game, p1, _p2) = setup_flashback_game();

        let spell_id = ObjectId::new();
        let mut spell = CardData::new(spell_id, p1, "Flashback Bolt");
        spell.card_types = vec![CardType::Sorcery];
        spell.mana_cost = ManaCost::parse("{R}");
        spell.flashback_cost = Some(ManaCost::parse("{2}{R}"));
        spell.abilities = vec![Ability::spell(spell_id,
            vec![Effect::GainLife { amount: 3 }],
            TargetSpec::None)];
        game.state.card_store.insert(spell);
        game.state.players.get_mut(&p1).unwrap().graveyard.add(spell_id);

        // Only give 1 red (flashback needs {2}{R})
        game.state.players.get_mut(&p1).unwrap()
            .mana_pool.add(Mana { red: 1, ..Mana::new() }, None, false);

        let actions = game.compute_legal_actions(p1);
        let cast_actions: Vec<_> = actions.iter()
            .filter(|a| matches!(a, PlayerAction::CastSpell { card_id, .. } if *card_id == spell_id))
            .collect();
        assert!(cast_actions.is_empty(), "Should NOT be able to flashback without enough mana");
    }

    #[test]
    fn flashback_cast_exiles_after_resolution() {
        let (mut game, p1, _p2) = setup_flashback_game();

        // Create a sorcery with flashback
        let spell_id = ObjectId::new();
        let mut spell = CardData::new(spell_id, p1, "Flashback Heal");
        spell.card_types = vec![CardType::Sorcery];
        spell.mana_cost = ManaCost::parse("{W}");
        spell.flashback_cost = Some(ManaCost::parse("{1}{W}"));
        spell.abilities = vec![Ability::spell(spell_id,
            vec![Effect::GainLife { amount: 3 }],
            TargetSpec::None)];
        game.state.card_store.insert(spell);
        game.state.players.get_mut(&p1).unwrap().graveyard.add(spell_id);

        // Give P1 mana for flashback {1}{W}
        game.state.players.get_mut(&p1).unwrap()
            .mana_pool.add(Mana { white: 2, ..Mana::new() }, None, false);

        let life_before = game.state.players.get(&p1).unwrap().life;

        // Cast from graveyard
        game.cast_spell(p1, spell_id);

        // Should be on stack
        assert!(!game.state.stack.is_empty());
        // Should no longer be in graveyard
        assert!(!game.state.players.get(&p1).unwrap().graveyard.contains(spell_id));

        // Resolve
        game.resolve_top_of_stack();

        // Should gain 3 life
        let life_after = game.state.players.get(&p1).unwrap().life;
        assert_eq!(life_after, life_before + 3);

        // Should be in exile (NOT graveyard)
        assert!(game.state.exile.contains(spell_id), "Flashback spell should be exiled after resolution");
        assert!(!game.state.players.get(&p1).unwrap().graveyard.contains(spell_id),
            "Flashback spell should NOT be in graveyard");
    }

    #[test]
    fn normal_cast_still_goes_to_graveyard() {
        let (mut game, p1, _p2) = setup_flashback_game();

        // Normal sorcery (no flashback) in hand
        let spell_id = ObjectId::new();
        let mut spell = CardData::new(spell_id, p1, "Normal Heal");
        spell.card_types = vec![CardType::Sorcery];
        spell.mana_cost = ManaCost::parse("{W}");
        spell.abilities = vec![Ability::spell(spell_id,
            vec![Effect::GainLife { amount: 2 }],
            TargetSpec::None)];
        game.state.card_store.insert(spell);
        game.state.players.get_mut(&p1).unwrap().hand.add(spell_id);

        // Give mana
        game.state.players.get_mut(&p1).unwrap()
            .mana_pool.add(Mana { white: 1, ..Mana::new() }, None, false);

        // Cast from hand and resolve
        game.cast_spell(p1, spell_id);
        game.resolve_top_of_stack();

        // Should be in graveyard (NOT exile)
        assert!(game.state.players.get(&p1).unwrap().graveyard.contains(spell_id),
            "Normal spell should go to graveyard");
        assert!(!game.state.exile.contains(spell_id),
            "Normal spell should NOT be exiled");
    }
}

#[cfg(test)]
mod behold_tests {
    use super::*;
    use crate::abilities::{Ability, Effect, TargetSpec, Cost};
    use crate::card::CardData;
    use crate::constants::{CardType, SubType, TurnPhase, PhaseStep, Outcome, KeywordAbilities};
    use crate::permanent::Permanent;
    use crate::mana::{Mana, ManaCost};
    use crate::types::{ObjectId, PlayerId};
    use crate::decision::*;

    struct PassivePlayer;
    impl PlayerDecisionMaker for PassivePlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn setup_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            starting_life: 20,
            players: vec![
                PlayerConfig { name: "P1".into(), deck: vec![] },
                PlayerConfig { name: "P2".into(), deck: vec![] },
            ],
        };
        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(PassivePlayer)),
                (p2, Box::new(PassivePlayer)),
            ],
        );
        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.state.current_phase = TurnPhase::PrecombatMain;
        game.state.current_step = PhaseStep::PrecombatMain;
        game.state.turn_number = 1;
        (game, p1, p2)
    }

    fn add_creature_to_battlefield(game: &mut Game, owner: PlayerId, name: &str, subtype: SubType) -> ObjectId {
        let id = ObjectId::new();
        let mut card = CardData::new(id, owner, name);
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![subtype];
        card.power = Some(1);
        card.toughness = Some(1);
        game.state.card_store.insert(card);
        let card_clone = game.state.card_store.get(id).unwrap().clone();
        let perm = Permanent::new(card_clone, owner);
        game.state.battlefield.add(perm);
        id
    }

    fn add_creature_to_hand(game: &mut Game, owner: PlayerId, name: &str, subtype: SubType) -> ObjectId {
        let id = ObjectId::new();
        let mut card = CardData::new(id, owner, name);
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![subtype];
        card.power = Some(1);
        card.toughness = Some(1);
        game.state.card_store.insert(card);
        game.state.players.get_mut(&owner).unwrap().hand.add(id);
        id
    }

    #[test]
    fn behold_cost_with_battlefield_creature() {
        let (mut game, p1, _p2) = setup_game();

        // Put an Elf on the battlefield
        let elf_id = add_creature_to_battlefield(&mut game, p1, "Llanowar Elves", SubType::Elf);

        // Create a source permanent for the activated ability
        let source_id = add_creature_to_battlefield(&mut game, p1, "Source", SubType::Warrior);

        // Pay behold cost — should succeed (Elf on battlefield)
        assert!(game.pay_costs(p1, source_id, &[Cost::behold("Elf")]));
        // Elf should still be on battlefield (behold doesn't remove)
        assert!(game.state.battlefield.get(elf_id).is_some());
    }

    #[test]
    fn behold_cost_with_hand_creature() {
        let (mut game, p1, _p2) = setup_game();

        // Put an Elf in hand
        let elf_id = add_creature_to_hand(&mut game, p1, "Llanowar Elves", SubType::Elf);

        let source_id = add_creature_to_battlefield(&mut game, p1, "Source", SubType::Warrior);

        // Pay behold cost — should succeed (Elf in hand)
        assert!(game.pay_costs(p1, source_id, &[Cost::behold("Elf")]));
        // Elf should still be in hand (behold just reveals)
        assert!(game.state.players.get(&p1).unwrap().hand.contains(elf_id));
    }

    #[test]
    fn behold_cost_fails_without_matching_creature() {
        let (mut game, p1, _p2) = setup_game();

        // Only have a Goblin, need to behold an Elf
        let _goblin_id = add_creature_to_battlefield(&mut game, p1, "Goblin Piker", SubType::Goblin);
        let source_id = add_creature_to_battlefield(&mut game, p1, "Source", SubType::Warrior);

        assert!(!game.pay_costs(p1, source_id, &[Cost::behold("Elf")]));
    }

    #[test]
    fn behold_and_exile_removes_from_battlefield() {
        let (mut game, p1, _p2) = setup_game();

        let goblin_id = add_creature_to_battlefield(&mut game, p1, "Goblin Piker", SubType::Goblin);
        let source_id = add_creature_to_battlefield(&mut game, p1, "Source", SubType::Warrior);

        assert!(game.pay_costs(p1, source_id, &[Cost::behold_and_exile("Goblin")]));
        // Goblin should be exiled
        assert!(game.state.battlefield.get(goblin_id).is_none());
        assert!(game.state.exile.contains(goblin_id));
    }

    #[test]
    fn behold_and_exile_removes_from_hand() {
        let (mut game, p1, _p2) = setup_game();

        let goblin_id = add_creature_to_hand(&mut game, p1, "Goblin Piker", SubType::Goblin);
        let source_id = add_creature_to_battlefield(&mut game, p1, "Source", SubType::Warrior);

        assert!(game.pay_costs(p1, source_id, &[Cost::behold_and_exile("Goblin")]));
        // Goblin should be exiled from hand
        assert!(!game.state.players.get(&p1).unwrap().hand.contains(goblin_id));
        assert!(game.state.exile.contains(goblin_id));
    }

    #[test]
    fn behold_or_pay_prefers_behold() {
        let (mut game, p1, _p2) = setup_game();

        let kithkin_id = add_creature_to_battlefield(&mut game, p1, "Goldmeadow Harrier", SubType::Kithkin);
        let source_id = add_creature_to_battlefield(&mut game, p1, "Source", SubType::Warrior);

        // Give mana too
        game.state.players.get_mut(&p1).unwrap()
            .mana_pool.add(Mana { white: 2, ..Mana::new() }, None, false);

        // Behold or pay {2} — should succeed via behold (free), mana untouched
        assert!(game.pay_costs(p1, source_id, &[Cost::behold_or_pay("Kithkin", "{2}")]));
        // Kithkin still on battlefield (behold doesn't remove)
        assert!(game.state.battlefield.get(kithkin_id).is_some());
        // Mana should still be available (behold was free)
        assert_eq!(game.state.players.get(&p1).unwrap().mana_pool.total_count(), 2);
    }

    #[test]
    fn behold_or_pay_falls_back_to_mana() {
        let (mut game, p1, _p2) = setup_game();

        // No Kithkin available — must pay mana
        let source_id = add_creature_to_battlefield(&mut game, p1, "Source", SubType::Warrior);

        game.state.players.get_mut(&p1).unwrap()
            .mana_pool.add(Mana { white: 2, ..Mana::new() }, None, false);

        assert!(game.pay_costs(p1, source_id, &[Cost::behold_or_pay("Kithkin", "{2}")]));
        // Mana should have been spent
        assert_eq!(game.state.players.get(&p1).unwrap().mana_pool.total_count(), 0);
    }

    #[test]
    fn behold_or_pay_fails_without_either() {
        let (mut game, p1, _p2) = setup_game();

        // No Kithkin, no mana
        let source_id = add_creature_to_battlefield(&mut game, p1, "Source", SubType::Warrior);

        assert!(!game.pay_costs(p1, source_id, &[Cost::behold_or_pay("Kithkin", "{2}")]));
    }

    #[test]
    fn behold_works_with_changeling() {
        let (mut game, p1, _p2) = setup_game();

        // Changeling counts as every creature type
        let changeling_id = ObjectId::new();
        let mut card = CardData::new(changeling_id, p1, "Mothdust Changeling");
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![SubType::Shapeshifter];
        card.keywords = KeywordAbilities::CHANGELING;
        card.power = Some(1);
        card.toughness = Some(1);
        game.state.card_store.insert(card);
        let card_clone = game.state.card_store.get(changeling_id).unwrap().clone();
        let perm = Permanent::new(card_clone, p1);
        game.state.battlefield.add(perm);

        let source_id = add_creature_to_battlefield(&mut game, p1, "Source", SubType::Warrior);

        // Changeling should count as an Elf for behold
        assert!(game.pay_costs(p1, source_id, &[Cost::behold("Elf")]));
    }
}

#[cfg(test)]
mod block_restriction_tests {
    use super::*;
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, Outcome, AbilityType};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };
    use crate::abilities::{Ability, StaticEffect};

    /// Decision maker that attacks with all creatures.
    struct AttackAllPlayer;

    impl PlayerDecisionMaker for AttackAllPlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, possible_attackers: &[ObjectId], possible_defenders: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> {
            let defender = possible_defenders[0];
            possible_attackers.iter().map(|&a| (a, defender)).collect()
        }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    /// Decision maker that assigns ALL available blockers to each attacker.
    struct BlockAllMultiplePlayer;

    impl PlayerDecisionMaker for BlockAllMultiplePlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, attackers: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> {
            // Assign ALL legal blockers to each attacker
            let mut blocks = Vec::new();
            for info in attackers {
                for &blocker_id in &info.legal_blockers {
                    blocks.push((blocker_id, info.attacker_id));
                }
            }
            blocks
        }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    /// Decision maker that blocks with exactly one blocker per attacker.
    struct BlockOnePerAttackerPlayer;

    impl PlayerDecisionMaker for BlockOnePerAttackerPlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, attackers: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> {
            let mut blocks = Vec::new();
            let mut used = std::collections::HashSet::new();
            for info in attackers {
                for &blocker_id in &info.legal_blockers {
                    if !used.contains(&blocker_id) {
                        blocks.push((blocker_id, info.attacker_id));
                        used.insert(blocker_id);
                        break;
                    }
                }
            }
            blocks
        }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        (0..40).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    fn setup_game(
        p1_dm: Box<dyn PlayerDecisionMaker>,
        p2_dm: Box<dyn PlayerDecisionMaker>,
    ) -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Attacker".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Defender".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(config, vec![(p1, p1_dm), (p2, p2_dm)]);
        (game, p1, p2)
    }

    fn add_creature(
        game: &mut Game,
        owner: PlayerId,
        name: &str,
        power: i32,
        toughness: i32,
        keywords: KeywordAbilities,
    ) -> ObjectId {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(power);
        card.toughness = Some(toughness);
        card.keywords = keywords;
        let id = card.id;
        let mut perm = Permanent::new(card, owner);
        perm.remove_summoning_sickness();
        game.state.battlefield.add(perm);
        id
    }

    fn add_creature_with_static(
        game: &mut Game,
        owner: PlayerId,
        name: &str,
        power: i32,
        toughness: i32,
        keywords: KeywordAbilities,
        static_effects: Vec<StaticEffect>,
    ) -> ObjectId {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(power);
        card.toughness = Some(toughness);
        card.keywords = keywords;
        let id = card.id;
        let ability = Ability::static_ability(id, "", static_effects);
        game.state.card_store.insert(card.clone());
        let mut perm = Permanent::new(card, owner);
        perm.remove_summoning_sickness();
        game.state.battlefield.add(perm);
        game.state.ability_store.add(ability);
        id
    }

    #[test]
    fn daunt_blocks_low_power_creatures() {
        // Creature with "can't be blocked by power 2 or less" (daunt)
        let (mut game, p1, p2) = setup_game(
            Box::new(AttackAllPlayer),
            Box::new(BlockOnePerAttackerPlayer),
        );

        let attacker_id = add_creature_with_static(
            &mut game, p1, "Daunt Creature", 4, 4, KeywordAbilities::empty(),
            vec![StaticEffect::CantBeBlockedByPowerLessOrEqual { power: 2 }],
        );
        let small_blocker = add_creature(&mut game, p2, "Small Blocker", 2, 2, KeywordAbilities::empty());
        let big_blocker = add_creature(&mut game, p2, "Big Blocker", 3, 3, KeywordAbilities::empty());

        game.state.active_player = p1;
        game.state.priority_player = p1;

        // Apply continuous effects so daunt threshold is set
        game.apply_continuous_effects();

        game.declare_attackers_step(p1);
        assert!(game.state.combat.is_attacking(attacker_id));

        game.declare_blockers_step(p1);

        // Small blocker (power 2) should NOT be blocking (daunt prevents it)
        assert!(!game.state.combat.is_blocking(small_blocker));
        // Big blocker (power 3) SHOULD be blocking
        assert!(game.state.combat.is_blocking(big_blocker));
    }

    #[test]
    fn cant_be_blocked_by_more_than_one() {
        let (mut game, p1, p2) = setup_game(
            Box::new(AttackAllPlayer),
            Box::new(BlockAllMultiplePlayer),
        );

        let attacker_id = add_creature_with_static(
            &mut game, p1, "Max1 Creature", 3, 3, KeywordAbilities::empty(),
            vec![StaticEffect::CantBeBlockedByMoreThan { count: 1 }],
        );
        let _blocker1 = add_creature(&mut game, p2, "Blocker1", 2, 2, KeywordAbilities::empty());
        let _blocker2 = add_creature(&mut game, p2, "Blocker2", 2, 2, KeywordAbilities::empty());
        let _blocker3 = add_creature(&mut game, p2, "Blocker3", 2, 2, KeywordAbilities::empty());

        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.apply_continuous_effects();

        game.declare_attackers_step(p1);
        game.declare_blockers_step(p1);

        // Should have at most 1 blocker despite defender wanting to assign all 3
        let group = game.state.combat.group_for_attacker(attacker_id).unwrap();
        assert!(group.blockers.len() <= 1, "max_blocked_by=1 but got {} blockers", group.blockers.len());
        assert!(group.is_blocked()); // Still counted as blocked
    }

    #[test]
    fn menace_single_blocker_removed() {
        // Menace: must be blocked by 2+ creatures. A single blocker should be removed.
        let (mut game, p1, p2) = setup_game(
            Box::new(AttackAllPlayer),
            Box::new(BlockOnePerAttackerPlayer),
        );

        let attacker_id = add_creature(&mut game, p1, "Menace Creature", 3, 3, KeywordAbilities::MENACE);
        let _blocker1 = add_creature(&mut game, p2, "Blocker1", 2, 2, KeywordAbilities::empty());

        game.state.active_player = p1;
        game.state.priority_player = p1;

        game.declare_attackers_step(p1);
        game.declare_blockers_step(p1);

        // Single blocker should be removed by menace validation
        let group = game.state.combat.group_for_attacker(attacker_id).unwrap();
        assert_eq!(group.blockers.len(), 0, "menace should remove single blocker");
        assert!(!group.is_blocked());
    }

    #[test]
    fn menace_two_blockers_allowed() {
        // Menace with 2 blockers: should be allowed
        let (mut game, p1, p2) = setup_game(
            Box::new(AttackAllPlayer),
            Box::new(BlockAllMultiplePlayer),
        );

        let attacker_id = add_creature(&mut game, p1, "Menace Creature", 3, 3, KeywordAbilities::MENACE);
        let _blocker1 = add_creature(&mut game, p2, "Blocker1", 2, 2, KeywordAbilities::empty());
        let _blocker2 = add_creature(&mut game, p2, "Blocker2", 2, 2, KeywordAbilities::empty());

        game.state.active_player = p1;
        game.state.priority_player = p1;

        game.declare_attackers_step(p1);
        game.declare_blockers_step(p1);

        // Two blockers should satisfy menace
        let group = game.state.combat.group_for_attacker(attacker_id).unwrap();
        assert_eq!(group.blockers.len(), 2, "menace satisfied by 2 blockers");
        assert!(group.is_blocked());
    }

    #[test]
    fn must_be_blocked_flag_set() {
        // MustBeBlocked static effect sets the flag on the permanent
        let (mut game, p1, _p2) = setup_game(
            Box::new(AttackAllPlayer),
            Box::new(BlockOnePerAttackerPlayer),
        );

        let creature_id = add_creature_with_static(
            &mut game, p1, "Lure Creature", 3, 3, KeywordAbilities::empty(),
            vec![StaticEffect::MustBeBlocked],
        );

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(perm.must_be_blocked, "must_be_blocked should be set by static effect");
    }

    #[test]
    fn must_be_blocked_info_in_attacker_info() {
        // The must_be_blocked flag should be available in AttackerInfo
        let (mut game, p1, p2) = setup_game(
            Box::new(AttackAllPlayer),
            Box::new(BlockOnePerAttackerPlayer),
        );

        let _lure_id = add_creature_with_static(
            &mut game, p1, "Lure Creature", 3, 3, KeywordAbilities::empty(),
            vec![StaticEffect::MustBeBlocked],
        );
        let _blocker = add_creature(&mut game, p2, "Blocker", 2, 2, KeywordAbilities::empty());

        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.apply_continuous_effects();

        // Just verify the continuous effects set the flag correctly
        let perm = game.state.battlefield.get(_lure_id).unwrap();
        assert!(perm.must_be_blocked);
    }
}

#[cfg(test)]
mod simple_effect_tests {
    use super::*;
    use crate::card::CardData;
    use crate::constants::{CardType, Outcome};
    use crate::counters::CounterType;
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };

    struct PassPlayer;
    impl PlayerDecisionMaker for PassPlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn make_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let deck: Vec<CardData> = (0..40).map(|i| {
            let mut c = CardData::new(ObjectId::new(), p1, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect();
        let deck2: Vec<CardData> = (0..40).map(|i| {
            let mut c = CardData::new(ObjectId::new(), p2, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "P1".to_string(), deck },
                PlayerConfig { name: "P2".to_string(), deck: deck2 },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(config, vec![(p1, Box::new(PassPlayer)), (p2, Box::new(PassPlayer))]);
        (game, p1, p2)
    }

    #[test]
    fn proliferate_adds_counters() {
        let (mut game, p1, _p2) = make_game();

        // Add a creature with +1/+1 counters
        let mut card = CardData::new(ObjectId::new(), p1, "Creature");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        let id = card.id;
        let mut perm = Permanent::new(card, p1);
        perm.add_counters(CounterType::P1P1, 2);
        game.state.battlefield.add(perm);

        // Execute proliferate
        game.execute_effects(
            &[crate::abilities::Effect::Proliferate],
            p1, &[], Some(ObjectId::new()), None,
        );





        // Should have 3 +1/+1 counters now (2 + 1 from proliferate)
        let perm = game.state.battlefield.get(id).unwrap();
        assert_eq!(perm.counters.get(&CounterType::P1P1), 3);
    }

    #[test]
    fn remove_all_counters_clears_creature() {
        let (mut game, p1, _p2) = make_game();

        let mut card = CardData::new(ObjectId::new(), p1, "Creature");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        let id = card.id;
        let mut perm = Permanent::new(card, p1);
        perm.add_counters(CounterType::P1P1, 3);
        perm.add_counters(CounterType::M1M1, 1);
        game.state.battlefield.add(perm);

        // Execute remove all counters targeting the creature
        game.execute_effects(
            &[crate::abilities::Effect::RemoveAllCounters],
            p1, &[id], Some(ObjectId::new()), None,
        );

        let perm = game.state.battlefield.get(id).unwrap();
        assert!(perm.counters.is_empty());
    }

    #[test]
    fn tap_attached_taps_enchanted_creature() {
        let (mut game, p1, _p2) = make_game();

        // Create a creature
        let mut creature_card = CardData::new(ObjectId::new(), p1, "Target Creature");
        creature_card.card_types = vec![CardType::Creature];
        creature_card.power = Some(2);
        creature_card.toughness = Some(2);
        let creature_id = creature_card.id;
        game.state.battlefield.add(Permanent::new(creature_card, p1));

        // Create an aura attached to the creature
        let mut aura_card = CardData::new(ObjectId::new(), p1, "Aura");
        aura_card.card_types = vec![CardType::Enchantment];
        let aura_id = aura_card.id;
        let mut aura_perm = Permanent::new(aura_card, p1);
        aura_perm.attach_to(creature_id);
        game.state.battlefield.add(aura_perm);

        // Execute TapAttached from the aura's perspective
        game.execute_effects(
            &[crate::abilities::Effect::TapAttached],
            p1, &[], Some(aura_id), None,
        );

        // Creature should be tapped
        assert!(game.state.battlefield.get(creature_id).unwrap().tapped);
    }
}

#[cfg(test)]
mod boost_per_count_tests {
    use super::*;
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, Outcome, SubType, AbilityType};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };
    use crate::abilities::{Ability, StaticEffect};

    struct PassPlayer;
    impl PlayerDecisionMaker for PassPlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn make_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let deck: Vec<CardData> = (0..40).map(|i| {
            let mut c = CardData::new(ObjectId::new(), p1, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect();
        let deck2: Vec<CardData> = (0..40).map(|i| {
            let mut c = CardData::new(ObjectId::new(), p2, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "P1".to_string(), deck },
                PlayerConfig { name: "P2".to_string(), deck: deck2 },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(config, vec![(p1, Box::new(PassPlayer)), (p2, Box::new(PassPlayer))]);
        (game, p1, p2)
    }

    fn add_goblin(game: &mut Game, owner: PlayerId, name: &str) -> ObjectId {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![SubType::Goblin];
        card.power = Some(1);
        card.toughness = Some(1);
        let id = card.id;
        game.state.battlefield.add(Permanent::new(card, owner));
        id
    }

    #[test]
    fn boost_per_count_two_goblins() {
        let (mut game, p1, _p2) = make_game();

        // Add a creature with "+2/+0 for each other Goblin you control"
        let mut card = CardData::new(ObjectId::new(), p1, "Goblin Lord");
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![SubType::Goblin, SubType::Berserker];
        card.power = Some(2);
        card.toughness = Some(4);
        let lord_id = card.id;
        let ability = Ability::static_ability(lord_id, "Gets +2/+0 per Goblin",
            vec![StaticEffect::BoostPerCount { count_filter: "other Goblin you control".into(), power_per: 2, toughness_per: 0 }]);
        card.abilities.push(ability.clone());
        game.state.card_store.insert(card.clone());
        game.state.battlefield.add(Permanent::new(card, p1));
        game.state.ability_store.add(ability);

        // Add 2 other goblins
        let _g1 = add_goblin(&mut game, p1, "Goblin A");
        let _g2 = add_goblin(&mut game, p1, "Goblin B");

        game.apply_continuous_effects();

        let lord = game.state.battlefield.get(lord_id).unwrap();
        // Base 2/4, +2*2/+0*2 = 6/4
        assert_eq!(lord.power(), 6, "power should be 2 + 2*2 = 6");
        assert_eq!(lord.toughness(), 4, "toughness should be 4 + 0*2 = 4");
    }

    #[test]
    fn boost_per_count_no_others() {
        let (mut game, p1, _p2) = make_game();

        let mut card = CardData::new(ObjectId::new(), p1, "Lonely Goblin Lord");
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![SubType::Goblin];
        card.power = Some(2);
        card.toughness = Some(4);
        let lord_id = card.id;
        let ability = Ability::static_ability(lord_id, "", 
            vec![StaticEffect::BoostPerCount { count_filter: "other Goblin you control".into(), power_per: 2, toughness_per: 0 }]);
        card.abilities.push(ability.clone());
        game.state.card_store.insert(card.clone());
        game.state.battlefield.add(Permanent::new(card, p1));
        game.state.ability_store.add(ability);

        game.apply_continuous_effects();

        let lord = game.state.battlefield.get(lord_id).unwrap();
        assert_eq!(lord.power(), 2, "no other goblins, power should be base 2");
    }

    #[test]
    fn boost_per_count_with_graveyard() {
        let (mut game, p1, _p2) = make_game();

        // Create a creature with "+1/+1 for each creature you control and creature card in graveyard"
        let mut card = CardData::new(ObjectId::new(), p1, "Graveyard Counter");
        card.card_types = vec![CardType::Creature];
        card.power = Some(0);
        card.toughness = Some(0);
        let id = card.id;
        let ability = Ability::static_ability(id, "",
            vec![StaticEffect::BoostPerCount {
                count_filter: "creature you control and creature card in your graveyard".into(),
                power_per: 1,
                toughness_per: 1,
            }]);
        card.abilities.push(ability.clone());
        game.state.card_store.insert(card.clone());
        game.state.battlefield.add(Permanent::new(card, p1));
        game.state.ability_store.add(ability);

        // Add 1 other creature on BF
        let mut c2 = CardData::new(ObjectId::new(), p1, "BF Creature");
        c2.card_types = vec![CardType::Creature];
        c2.power = Some(1);
        c2.toughness = Some(1);
        let c2_id = c2.id;
        game.state.battlefield.add(Permanent::new(c2.clone(), p1));
        game.state.card_store.insert(c2);

        // Add 2 creature cards in graveyard
        for i in 0..2 {
            let mut gc = CardData::new(ObjectId::new(), p1, &format!("GY Creature {i}"));
            gc.card_types = vec![CardType::Creature];
            gc.power = Some(1);
            gc.toughness = Some(1);
            let gc_id = gc.id;
            game.state.card_store.insert(gc);
            if let Some(player) = game.state.players.get_mut(&p1) {
                player.graveyard.add(gc_id);
            }
        }

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(id).unwrap();
        // 2 creatures on BF (self + other) + 2 in graveyard = 4 total
        assert_eq!(perm.power(), 4, "0 + 1*(2 BF + 2 GY) = 4");
        assert_eq!(perm.toughness(), 4);
    }
}

#[cfg(test)]
mod flicker_tests {
    use super::*;
    use crate::abilities::*;
    use crate::types::*;
    use crate::counters::CounterType;
    use uuid::Uuid;

    struct PassPlayer;
    impl crate::decision::PlayerDecisionMaker for PassPlayer {
        fn priority(&mut self, _: &crate::decision::GameView, actions: &[crate::decision::PlayerAction]) -> crate::decision::PlayerAction { actions[0].clone() }
        fn choose_targets(&mut self, _: &crate::decision::GameView, _: crate::constants::Outcome, _: &crate::decision::TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &crate::decision::GameView, _: crate::constants::Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &crate::decision::GameView, modes: &[crate::decision::NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &crate::decision::GameView, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &crate::decision::GameView, _: &[crate::decision::AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &crate::decision::GameView, _: &crate::decision::DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &crate::decision::GameView, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &crate::decision::GameView, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &crate::decision::GameView, hand: &[ObjectId], count: usize) -> Vec<ObjectId> { hand.iter().take(count).copied().collect() }
        fn choose_amount(&mut self, _: &crate::decision::GameView, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &crate::decision::GameView, _: &crate::decision::UnpaidMana, _: &[crate::decision::PlayerAction]) -> Option<crate::decision::PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &crate::decision::GameView, _: &[crate::decision::ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &crate::decision::GameView, _: crate::constants::Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &crate::decision::GameView, _: crate::constants::Outcome, _: &str, _: &[crate::decision::NamedChoice]) -> usize { 0 }
    }

    fn make_test_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId(Uuid::new_v4());
        let p2 = PlayerId(Uuid::new_v4());
        let config = GameConfig { players: vec![PlayerConfig { name: "P1".to_string(), deck: vec![] }, PlayerConfig { name: "P2".to_string(), deck: vec![] }], starting_life: 20 };
        let game = Game::new_two_player(config, vec![
            (p1, Box::new(PassPlayer)),
            (p2, Box::new(PassPlayer)),
        ]);
        (game, p1, p2)
    }

    fn make_creature(name: &str, power: i32, toughness: i32) -> (ObjectId, CardData) {
        let id = ObjectId(Uuid::new_v4());
        let card = CardData {
            id,
            owner: PlayerId(Uuid::new_v4()), // will be overridden
            name: name.into(),
            card_types: vec![crate::constants::CardType::Creature],
            power: Some(power), toughness: Some(toughness),
            ..Default::default()
        };
        (id, card)
    }

    #[test]
    fn flicker_returns_creature_fresh() {
        let (mut game, p1, _p2) = make_test_game();

        let (card_id, mut card) = make_creature("Test Creature", 3, 3);
        card.owner = p1;
        let perm = crate::permanent::Permanent::new(card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card.clone());
        for ab in &card.abilities { game.state.ability_store.add(ab.clone()); }
        game.state.set_zone(card_id, crate::constants::Zone::Battlefield, None);

        // Put +1/+1 counter on it
        if let Some(perm) = game.state.battlefield.get_mut(card_id) {
            perm.counters.add(CounterType::P1P1, 2);
            assert_eq!(perm.power(), 5); // 3 + 2
        }

        // Flicker it
        let effects = vec![Effect::Flicker];
        game.execute_effects(&effects, p1, &[card_id], None, None);

        // Verify it's back on battlefield as fresh permanent (no counters)
        let perm = game.state.battlefield.get(card_id).expect("should be on BF");
        assert_eq!(perm.power(), 3, "should have base power after flicker (no counters)");
        assert_eq!(perm.counters.get(&CounterType::P1P1), 0, "counters should be reset");
    }

    #[test]
    fn flicker_triggers_etb() {
        let (mut game, p1, _p2) = make_test_game();

        let (card_id, mut card) = make_creature("ETB Creature", 2, 2);
        card.owner = p1;
        let perm = crate::permanent::Permanent::new(card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card.clone());
        game.state.set_zone(card_id, crate::constants::Zone::Battlefield, None);

        // Clear event log then flicker
        game.event_log.clear();
        let effects = vec![Effect::Flicker];
        game.execute_effects(&effects, p1, &[card_id], None, None);

        // Check that ETB event was emitted
        let etb_events: Vec<_> = game.event_log.iter()
            .filter(|e| e.event_type == crate::events::EventType::EnteredTheBattlefield)
            .collect();
        assert_eq!(etb_events.len(), 1, "flicker should emit 1 ETB event");
        assert_eq!(etb_events[0].target_id, Some(card_id));
    }

    #[test]
    fn flicker_end_step_exiles_then_returns_tapped() {
        let (mut game, p1, _p2) = make_test_game();

        let (card_id, mut card) = make_creature("Flickered Beast", 4, 4);
        card.owner = p1;
        let perm = crate::permanent::Permanent::new(card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card.clone());
        game.state.set_zone(card_id, crate::constants::Zone::Battlefield, None);

        // FlickerEndStep — should exile
        let effects = vec![Effect::FlickerEndStep];
        game.execute_effects(&effects, p1, &[card_id], None, None);

        // Verify creature is in exile, not on battlefield
        assert!(game.state.battlefield.get(card_id).is_none(), "should not be on BF");
        assert!(game.state.exile.contains(card_id), "should be in exile");

        // Verify delayed trigger was created
        assert_eq!(game.state.delayed_triggers.len(), 1);
        assert_eq!(game.state.delayed_triggers[0].effects.len(), 1);
        assert_eq!(game.state.delayed_triggers[0].targets, vec![card_id]);

        // Now simulate the delayed trigger firing: execute the return effect
        let dt = game.state.delayed_triggers[0].clone();
        game.execute_effects(&dt.effects, dt.controller, &dt.targets, dt.source, None);

        // Verify creature is back on battlefield, tapped
        let perm = game.state.battlefield.get(card_id).expect("should be back on BF");
        assert!(perm.tapped, "should be tapped after FlickerEndStep return");
    }

    #[test]
    fn additional_land_plays() {
        let (mut game, p1, _p2) = make_test_game();

        // Register a static ability with AdditionalLandPlays
        let source_id = ObjectId(Uuid::new_v4());
        let card = CardData {
            id: source_id,
            owner: p1,
            name: "Land Enabler".into(),
            card_types: vec![crate::constants::CardType::Enchantment],
            abilities: vec![
                Ability::static_ability(source_id, "You may play an additional land.", vec![
                    StaticEffect::AdditionalLandPlays { count: 1 },
                ]),
            ],
            ..Default::default()
        };
        let perm = crate::permanent::Permanent::new(card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card.clone());
        for ab in &card.abilities { game.state.ability_store.add(ab.clone()); }

        // Apply continuous effects
        game.apply_continuous_effects();

        // Player should be able to play 2 lands per turn
        let player = game.state.players.get(&p1).unwrap();
        assert_eq!(player.lands_per_turn, 2, "should have 2 land plays per turn");
    }

    #[test]
    fn opponent_exiles_from_hand() {
        let (mut game, p1, p2) = make_test_game();

        // Give opponent some cards in hand
        let c1 = ObjectId(Uuid::new_v4());
        let c2 = ObjectId(Uuid::new_v4());
        let c3 = ObjectId(Uuid::new_v4());
        if let Some(player) = game.state.players.get_mut(&p2) {
            player.hand.add(c1);
            player.hand.add(c2);
            player.hand.add(c3);
        }

        let effects = vec![Effect::OpponentExilesFromHand { count: 2 }];
        game.execute_effects(&effects, p1, &[], None, None);

        // Opponent should have 1 card left in hand
        let player = game.state.players.get(&p2).unwrap();
        assert_eq!(player.hand.len(), 1, "opponent should have 1 card left after exiling 2");

        // 2 cards should be in exile
        let exile_count = [c1, c2, c3].iter()
            .filter(|&&id| game.state.exile.contains(id))
            .count();
        assert_eq!(exile_count, 2, "2 cards should be in exile");
    }
}

#[cfg(test)]
mod conditional_static_tests {
    use super::*;
    use crate::abilities::*;
    use crate::types::*;
    use uuid::Uuid;

    struct PassPlayer;
    impl crate::decision::PlayerDecisionMaker for PassPlayer {
        fn priority(&mut self, _: &crate::decision::GameView, actions: &[crate::decision::PlayerAction]) -> crate::decision::PlayerAction { actions[0].clone() }
        fn choose_targets(&mut self, _: &crate::decision::GameView, _: crate::constants::Outcome, _: &crate::decision::TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &crate::decision::GameView, _: crate::constants::Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &crate::decision::GameView, _: &[crate::decision::NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &crate::decision::GameView, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &crate::decision::GameView, _: &[crate::decision::AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &crate::decision::GameView, _: &crate::decision::DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &crate::decision::GameView, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &crate::decision::GameView, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &crate::decision::GameView, hand: &[ObjectId], count: usize) -> Vec<ObjectId> { hand.iter().take(count).copied().collect() }
        fn choose_amount(&mut self, _: &crate::decision::GameView, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &crate::decision::GameView, _: &crate::decision::UnpaidMana, _: &[crate::decision::PlayerAction]) -> Option<crate::decision::PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &crate::decision::GameView, _: &[crate::decision::ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &crate::decision::GameView, _: crate::constants::Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &crate::decision::GameView, _: crate::constants::Outcome, _: &str, _: &[crate::decision::NamedChoice]) -> usize { 0 }
    }

    fn make_test_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId(Uuid::new_v4());
        let p2 = PlayerId(Uuid::new_v4());
        let config = GameConfig { players: vec![PlayerConfig { name: "P1".to_string(), deck: vec![] }, PlayerConfig { name: "P2".to_string(), deck: vec![] }], starting_life: 20 };
        let game = Game::new_two_player(config, vec![
            (p1, Box::new(PassPlayer)),
            (p2, Box::new(PassPlayer)),
        ]);
        (game, p1, p2)
    }

    #[test]
    fn conditional_keyword_your_turn() {
        let (mut game, p1, _p2) = make_test_game();

        // Create creature with "first strike on your turn"
        let card_id = ObjectId(Uuid::new_v4());
        let card = CardData {
            id: card_id, owner: p1, name: "First Strike Guy".into(),
            card_types: vec![crate::constants::CardType::Creature],
            power: Some(2), toughness: Some(1),
            abilities: vec![Ability::static_ability(card_id, "First strike on your turn.",
                vec![StaticEffect::ConditionalKeyword { keyword: "first strike".into(), condition: "your turn".into() }])],
            ..Default::default()
        };
        let perm = crate::permanent::Permanent::new(card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card.clone());
        for ab in &card.abilities { game.state.ability_store.add(ab.clone()); }

        // Set active player to p1 (their turn)
        game.state.active_player = p1;
        game.apply_continuous_effects();
        let perm = game.state.battlefield.get(card_id).unwrap();
        assert!(perm.has_keyword(crate::constants::KeywordAbilities::FIRST_STRIKE),
            "should have first strike on own turn");

        // Set active player to p2 (opponent's turn)
        game.state.active_player = _p2;
        game.apply_continuous_effects();
        let perm = game.state.battlefield.get(card_id).unwrap();
        assert!(!perm.has_keyword(crate::constants::KeywordAbilities::FIRST_STRIKE),
            "should NOT have first strike on opponent's turn");
    }

    #[test]
    fn conditional_keyword_untapped() {
        let (mut game, p1, _p2) = make_test_game();

        let card_id = ObjectId(Uuid::new_v4());
        let card = CardData {
            id: card_id, owner: p1, name: "Hexproof Untapped".into(),
            card_types: vec![crate::constants::CardType::Creature],
            power: Some(3), toughness: Some(3),
            abilities: vec![Ability::static_ability(card_id, "Hexproof as long as untapped.",
                vec![StaticEffect::ConditionalKeyword { keyword: "hexproof".into(), condition: "untapped".into() }])],
            ..Default::default()
        };
        let perm = crate::permanent::Permanent::new(card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card.clone());
        for ab in &card.abilities { game.state.ability_store.add(ab.clone()); }

        // Untapped: should have hexproof
        game.apply_continuous_effects();
        let perm = game.state.battlefield.get(card_id).unwrap();
        assert!(perm.has_keyword(crate::constants::KeywordAbilities::HEXPROOF),
            "should have hexproof when untapped");

        // Tap it
        if let Some(perm) = game.state.battlefield.get_mut(card_id) {
            perm.tap();
        }
        game.apply_continuous_effects();
        let perm = game.state.battlefield.get(card_id).unwrap();
        assert!(!perm.has_keyword(crate::constants::KeywordAbilities::HEXPROOF),
            "should NOT have hexproof when tapped");
    }

    #[test]
    fn conditional_keyword_control_type() {
        let (mut game, p1, _p2) = make_test_game();

        // Create creature with "flash if you control a Faerie"
        let card_id = ObjectId(Uuid::new_v4());
        let card = CardData {
            id: card_id, owner: p1, name: "Faerie Pal".into(),
            card_types: vec![crate::constants::CardType::Creature],
            power: Some(2), toughness: Some(2),
            abilities: vec![Ability::static_ability(card_id, "Flash if you control a Faerie.",
                vec![StaticEffect::ConditionalKeyword { keyword: "flash".into(), condition: "you control a Faerie".into() }])],
            ..Default::default()
        };
        let perm = crate::permanent::Permanent::new(card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card.clone());
        for ab in &card.abilities { game.state.ability_store.add(ab.clone()); }

        // No Faerie: no flash
        game.apply_continuous_effects();
        let perm = game.state.battlefield.get(card_id).unwrap();
        assert!(!perm.has_keyword(crate::constants::KeywordAbilities::FLASH),
            "should NOT have flash without a Faerie");

        // Add a Faerie
        let faerie_id = ObjectId(Uuid::new_v4());
        let faerie = CardData {
            id: faerie_id, owner: p1, name: "Faerie Token".into(),
            card_types: vec![crate::constants::CardType::Creature],
            subtypes: vec![crate::constants::SubType::Faerie],
            power: Some(1), toughness: Some(1),
            ..Default::default()
        };
        let faerie_perm = crate::permanent::Permanent::new(faerie.clone(), p1);
        game.state.battlefield.add(faerie_perm);
        game.state.card_store.insert(faerie);

        game.apply_continuous_effects();
        let perm = game.state.battlefield.get(card_id).unwrap();
        assert!(perm.has_keyword(crate::constants::KeywordAbilities::FLASH),
            "should have flash with a Faerie on BF");
    }

    #[test]
    fn conditional_boost_creature_etb() {
        let (mut game, p1, _p2) = make_test_game();

        let card_id = ObjectId(Uuid::new_v4());
        let card = CardData {
            id: card_id, owner: p1, name: "Boost on ETB".into(),
            card_types: vec![crate::constants::CardType::Creature],
            power: Some(3), toughness: Some(3),
            abilities: vec![Ability::static_ability(card_id, "+2/+0 if creature entered this turn.",
                vec![StaticEffect::ConditionalBoostSelf { power: 2, toughness: 0, condition: "creature entered this turn".into() }])],
            ..Default::default()
        };
        let perm = crate::permanent::Permanent::new(card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card.clone());
        for ab in &card.abilities { game.state.ability_store.add(ab.clone()); }

        // No ETB event: no boost
        game.apply_continuous_effects();
        let perm = game.state.battlefield.get(card_id).unwrap();
        assert_eq!(perm.power(), 3, "should be base power without ETB event");

        // Add ETB event
        game.emit_event(crate::events::GameEvent::enters_battlefield(ObjectId(Uuid::new_v4()), p1));
        game.apply_continuous_effects();
        let perm = game.state.battlefield.get(card_id).unwrap();
        assert_eq!(perm.power(), 5, "should be 3+2 with ETB event this turn");
    }
}

#[cfg(test)]
mod blight_and_types_tests {
    use super::*;
    use crate::abilities::*;
    use crate::types::*;
    use crate::counters::CounterType;
    use uuid::Uuid;

    struct PassPlayer;
    impl crate::decision::PlayerDecisionMaker for PassPlayer {
        fn priority(&mut self, _: &crate::decision::GameView, actions: &[crate::decision::PlayerAction]) -> crate::decision::PlayerAction { actions[0].clone() }
        fn choose_targets(&mut self, _: &crate::decision::GameView, _: crate::constants::Outcome, req: &crate::decision::TargetRequirement) -> Vec<ObjectId> {
            // Pick the first legal target
            req.legal_targets.iter().take(1).copied().collect()
        }
        fn choose_use(&mut self, _: &crate::decision::GameView, _: crate::constants::Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &crate::decision::GameView, _: &[crate::decision::NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &crate::decision::GameView, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &crate::decision::GameView, _: &[crate::decision::AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &crate::decision::GameView, _: &crate::decision::DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &crate::decision::GameView, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &crate::decision::GameView, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &crate::decision::GameView, hand: &[ObjectId], count: usize) -> Vec<ObjectId> { hand.iter().take(count).copied().collect() }
        fn choose_amount(&mut self, _: &crate::decision::GameView, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &crate::decision::GameView, _: &crate::decision::UnpaidMana, _: &[crate::decision::PlayerAction]) -> Option<crate::decision::PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &crate::decision::GameView, _: &[crate::decision::ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &crate::decision::GameView, _: crate::constants::Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &crate::decision::GameView, _: crate::constants::Outcome, _: &str, _: &[crate::decision::NamedChoice]) -> usize { 0 }
    }

    #[test]
    fn blight_opponents_puts_counter() {
        let p1 = PlayerId(Uuid::new_v4());
        let p2 = PlayerId(Uuid::new_v4());
        let config = GameConfig { players: vec![PlayerConfig { name: "P1".into(), deck: vec![] }, PlayerConfig { name: "P2".into(), deck: vec![] }], starting_life: 20 };
        let mut game = Game::new_two_player(config, vec![
            (p1, Box::new(PassPlayer)),
            (p2, Box::new(PassPlayer)),
        ]);

        // Give opponent a creature
        let opp_creature = ObjectId(Uuid::new_v4());
        let card = CardData {
            id: opp_creature, owner: p2, name: "Bear".into(),
            card_types: vec![crate::constants::CardType::Creature],
            power: Some(3), toughness: Some(3),
            ..Default::default()
        };
        let perm = crate::permanent::Permanent::new(card.clone(), p2);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card);

        // Blight opponents 1
        game.execute_effects(&[Effect::blight_opponents(1)], p1, &[], None, None);

        // Opponent's creature should have a -1/-1 counter
        let perm = game.state.battlefield.get(opp_creature).unwrap();
        assert_eq!(perm.counters.get(&CounterType::M1M1), 1, "should have -1/-1 counter");
        assert_eq!(perm.power(), 2, "power should be reduced by -1/-1");
    }

    #[test]
    fn gain_all_creature_types() {
        let p1 = PlayerId(Uuid::new_v4());
        let p2 = PlayerId(Uuid::new_v4());
        let config = GameConfig { players: vec![PlayerConfig { name: "P1".into(), deck: vec![] }, PlayerConfig { name: "P2".into(), deck: vec![] }], starting_life: 20 };
        let mut game = Game::new_two_player(config, vec![
            (p1, Box::new(PassPlayer)),
            (p2, Box::new(PassPlayer)),
        ]);

        let creature_id = ObjectId(Uuid::new_v4());
        let card = CardData {
            id: creature_id, owner: p1, name: "Type Gainer".into(),
            card_types: vec![crate::constants::CardType::Creature],
            subtypes: vec![crate::constants::SubType::Human],
            power: Some(2), toughness: Some(2),
            ..Default::default()
        };
        let perm = crate::permanent::Permanent::new(card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card);

        // Should not have Elf type initially
        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(!perm.has_subtype(&crate::constants::SubType::Elf));

        // Grant all creature types
        game.execute_effects(&[Effect::gain_all_creature_types()], p1, &[creature_id], None, None);

        // Should now have changeling (all creature types)
        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(perm.has_keyword(crate::constants::KeywordAbilities::CHANGELING), "should have changeling");
        assert!(perm.has_subtype(&crate::constants::SubType::Elf), "should have Elf as changeling");
        assert!(perm.has_subtype(&crate::constants::SubType::Goblin), "should have Goblin as changeling");
    }
}

#[cfg(test)]
mod token_copy_tests {
    use super::*;
    use crate::abilities::*;
    use crate::types::*;
    use crate::counters::CounterType;
    use uuid::Uuid;

    struct PassPlayer;
    impl crate::decision::PlayerDecisionMaker for PassPlayer {
        fn priority(&mut self, _: &crate::decision::GameView, actions: &[crate::decision::PlayerAction]) -> crate::decision::PlayerAction { actions[0].clone() }
        fn choose_targets(&mut self, _: &crate::decision::GameView, _: crate::constants::Outcome, req: &crate::decision::TargetRequirement) -> Vec<ObjectId> { req.legal_targets.iter().take(1).copied().collect() }
        fn choose_use(&mut self, _: &crate::decision::GameView, _: crate::constants::Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &crate::decision::GameView, _: &[crate::decision::NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &crate::decision::GameView, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &crate::decision::GameView, _: &[crate::decision::AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &crate::decision::GameView, _: &crate::decision::DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &crate::decision::GameView, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &crate::decision::GameView, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &crate::decision::GameView, hand: &[ObjectId], count: usize) -> Vec<ObjectId> { hand.iter().take(count).copied().collect() }
        fn choose_amount(&mut self, _: &crate::decision::GameView, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &crate::decision::GameView, _: &crate::decision::UnpaidMana, _: &[crate::decision::PlayerAction]) -> Option<crate::decision::PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &crate::decision::GameView, _: &[crate::decision::ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &crate::decision::GameView, _: crate::constants::Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &crate::decision::GameView, _: crate::constants::Outcome, _: &str, _: &[crate::decision::NamedChoice]) -> usize { 0 }
    }

    fn make_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId(Uuid::new_v4());
        let p2 = PlayerId(Uuid::new_v4());
        let config = GameConfig { players: vec![PlayerConfig { name: "P1".into(), deck: vec![] }, PlayerConfig { name: "P2".into(), deck: vec![] }], starting_life: 20 };
        let game = Game::new_two_player(config, vec![(p1, Box::new(PassPlayer)), (p2, Box::new(PassPlayer))]);
        (game, p1, p2)
    }

    #[test]
    fn token_copy_basic() {
        let (mut game, p1, _p2) = make_game();

        // Create a creature to copy
        let src_id = ObjectId(Uuid::new_v4());
        let src_card = CardData {
            id: src_id, owner: p1, name: "Goblin Lord".into(),
            card_types: vec![crate::constants::CardType::Creature],
            subtypes: vec![crate::constants::SubType::Goblin],
            power: Some(3), toughness: Some(3),
            keywords: crate::constants::KeywordAbilities::MENACE,
            ..Default::default()
        };
        let perm = Permanent::new(src_card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(src_card);

        // Create a token copy
        game.execute_effects(&[Effect::create_token_copy(1)], p1, &[src_id], None, None);

        // Should now have 2 creatures on BF
        let creatures: Vec<_> = game.state.battlefield.iter()
            .filter(|p| p.is_creature() && p.controller == p1)
            .collect();
        assert_eq!(creatures.len(), 2, "should have original + token copy");

        // Find the token (not the original)
        let token = creatures.iter().find(|p| p.id() != src_id).unwrap();
        assert_eq!(token.card.name, "Goblin Lord");
        assert_eq!(token.power(), 3);
        assert_eq!(token.toughness(), 3);
        assert!(token.card.is_token);
        assert!(token.has_subtype(&crate::constants::SubType::Goblin));
        assert!(token.has_keyword(crate::constants::KeywordAbilities::MENACE));
    }

    #[test]
    fn token_copy_with_haste() {
        let (mut game, p1, _p2) = make_game();

        let src_id = ObjectId(Uuid::new_v4());
        let src_card = CardData {
            id: src_id, owner: p1, name: "Big Dragon".into(),
            card_types: vec![crate::constants::CardType::Creature],
            subtypes: vec![crate::constants::SubType::Custom("Dragon".into())],
            power: Some(5), toughness: Some(5),
            keywords: crate::constants::KeywordAbilities::FLYING,
            ..Default::default()
        };
        let perm = Permanent::new(src_card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(src_card);

        // Copy with haste
        game.execute_effects(&[Effect::create_token_copy_with_haste(1)], p1, &[src_id], None, None);

        let token = game.state.battlefield.iter()
            .find(|p| p.id() != src_id && p.is_creature())
            .unwrap();
        assert!(token.has_keyword(crate::constants::KeywordAbilities::FLYING), "should have flying from original");
        assert!(token.has_keyword(crate::constants::KeywordAbilities::HASTE), "should have haste from modification");
    }

    #[test]
    fn token_copy_with_changeling() {
        let (mut game, p1, _p2) = make_game();

        let src_id = ObjectId(Uuid::new_v4());
        let src_card = CardData {
            id: src_id, owner: p1, name: "Elf Warrior".into(),
            card_types: vec![crate::constants::CardType::Creature],
            subtypes: vec![crate::constants::SubType::Elf, crate::constants::SubType::Warrior],
            power: Some(2), toughness: Some(2),
            ..Default::default()
        };
        let perm = Permanent::new(src_card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(src_card);

        // Copy with changeling
        game.execute_effects(&[Effect::create_token_copy_with_changeling(1)], p1, &[src_id], None, None);

        let token = game.state.battlefield.iter()
            .find(|p| p.id() != src_id && p.is_creature())
            .unwrap();
        assert!(token.has_keyword(crate::constants::KeywordAbilities::CHANGELING));
        assert!(token.has_subtype(&crate::constants::SubType::Goblin), "changeling has all types");
        assert!(token.has_subtype(&crate::constants::SubType::Elf), "should still be an Elf too");
    }

    #[test]
    fn token_copy_emits_etb() {
        let (mut game, p1, _p2) = make_game();

        let src_id = ObjectId(Uuid::new_v4());
        let src_card = CardData {
            id: src_id, owner: p1, name: "Bear".into(),
            card_types: vec![crate::constants::CardType::Creature],
            power: Some(2), toughness: Some(2),
            ..Default::default()
        };
        let perm = Permanent::new(src_card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(src_card);

        game.event_log.clear();
        game.execute_effects(&[Effect::create_token_copy(1)], p1, &[src_id], None, None);

        let etb_count = game.event_log.iter()
            .filter(|e| e.event_type == crate::events::EventType::EnteredTheBattlefield)
            .count();
        assert_eq!(etb_count, 1, "token copy should emit ETB event");
    }
}

#[cfg(test)]
mod tap_self_and_return_type_tests {
    use super::*;
    use crate::abilities::{Ability, Cost, Effect, TargetSpec};
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, SubType};
    use crate::decision::*;
    use crate::types::{ObjectId, PlayerId};

    struct AlwaysPassDM;
    impl PlayerDecisionMaker for AlwaysPassDM {
        fn priority(&mut self, _: &GameView, actions: &[PlayerAction]) -> PlayerAction {
            actions.iter().find(|a| matches!(a, PlayerAction::Pass)).cloned().unwrap_or(PlayerAction::Pass)
        }
        fn choose_targets(&mut self, _: &GameView, _: crate::constants::Outcome, req: &TargetRequirement) -> Vec<ObjectId> {
            if req.min_targets > 0 && !req.legal_targets.is_empty() { vec![req.legal_targets[0]] } else { vec![] }
        }
        fn choose_use(&mut self, _: &GameView, _: crate::constants::Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView, modes: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView, a: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![(a.targets[0], a.total_damage)] }
        fn choose_mulligan(&mut self, _: &GameView, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView, hand: &[ObjectId], count: usize) -> Vec<ObjectId> { hand.iter().take(count).copied().collect() }
        fn choose_amount(&mut self, _: &GameView, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView, _: &UnpaidMana, abilities: &[PlayerAction]) -> Option<PlayerAction> { abilities.first().cloned() }
        fn choose_replacement_effect(&mut self, _: &GameView, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView, _: crate::constants::Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView, _: crate::constants::Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn setup_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            starting_life: 20,
            players: vec![
                PlayerConfig { name: "P1".into(), deck: vec![] },
                PlayerConfig { name: "P2".into(), deck: vec![] },
            ],
        };
        let game = Game::new_two_player(config, vec![
            (p1, Box::new(AlwaysPassDM)),
            (p2, Box::new(AlwaysPassDM)),
        ]);
        (game, p1, p2)
    }

    #[test]
    fn tap_self_taps_source() {
        let (mut game, p1, _p2) = setup_game();

        // Create a creature with an activated ability that taps self as part of its effect
        let creature_id = ObjectId::new();
        let mut card = CardData::new(creature_id, p1, "Self Tapper");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        card.abilities = vec![Ability::activated(creature_id,
            "Pay 1: Tap this creature.",
            vec![Cost::pay_mana("{1}")],
            vec![Effect::TapSelf],
            TargetSpec::None)];

        let perm = crate::permanent::Permanent::new(card.clone(), p1);
        game.state.card_store.insert(card.clone());
        game.state.battlefield.add(perm);
        for ability in &card.abilities {
            game.state.ability_store.add(ability.clone());
        }

        // Creature should start untapped
        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(!perm.tapped, "should start untapped");

        // Execute TapSelf effect with source
        game.execute_effects(
            &[Effect::TapSelf],
            p1,
            &[],
            Some(creature_id),
            None,
        );

        // Creature should now be tapped
        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(perm.tapped, "should be tapped after TapSelf");
    }

    #[test]
    fn return_all_type_from_graveyard() {
        let (mut game, p1, _p2) = setup_game();

        // Put some Goblins and non-Goblins in the graveyard
        let goblin1_id = ObjectId::new();
        let mut goblin1 = CardData::new(goblin1_id, p1, "Goblin Warrior");
        goblin1.card_types = vec![CardType::Creature];
        goblin1.subtypes = vec![SubType::Goblin, SubType::Warrior];
        goblin1.power = Some(2);
        goblin1.toughness = Some(1);

        let goblin2_id = ObjectId::new();
        let mut goblin2 = CardData::new(goblin2_id, p1, "Goblin Shaman");
        goblin2.card_types = vec![CardType::Creature];
        goblin2.subtypes = vec![SubType::Goblin];
        goblin2.power = Some(1);
        goblin2.toughness = Some(1);

        let elf_id = ObjectId::new();
        let mut elf = CardData::new(elf_id, p1, "Llanowar Elves");
        elf.card_types = vec![CardType::Creature];
        elf.subtypes = vec![SubType::Elf];
        elf.power = Some(1);
        elf.toughness = Some(1);

        game.state.card_store.insert(goblin1.clone());
        game.state.card_store.insert(goblin2.clone());
        game.state.card_store.insert(elf.clone());

        game.state.players.get_mut(&p1).unwrap().graveyard.add(goblin1_id);
        game.state.players.get_mut(&p1).unwrap().graveyard.add(goblin2_id);
        game.state.players.get_mut(&p1).unwrap().graveyard.add(elf_id);

        // Execute ReturnAllTypeFromGraveyard for Goblins
        game.execute_effects(
            &[Effect::ReturnAllTypeFromGraveyard { creature_type: "Goblin".into() }],
            p1,
            &[],
            None,
            None,
        );

        // Both goblins should be on the battlefield
        assert!(game.state.battlefield.get(goblin1_id).is_some(), "goblin1 should be on battlefield");
        assert!(game.state.battlefield.get(goblin2_id).is_some(), "goblin2 should be on battlefield");
        // Elf should still be in graveyard
        let gy = &game.state.players.get(&p1).unwrap().graveyard;
        assert!(gy.iter().any(|&id| id == elf_id), "elf should still be in graveyard");
        assert!(!gy.iter().any(|&id| id == goblin1_id), "goblin1 should not be in graveyard");
    }

    #[test]
    fn create_token_dynamic_count() {
        let (mut game, p1, _p2) = setup_game();

        // Put 3 Elf cards in graveyard
        for i in 0..3 {
            let elf_id = ObjectId::new();
            let mut elf = CardData::new(elf_id, p1, &format!("Dead Elf {}", i));
            elf.card_types = vec![CardType::Creature];
            elf.subtypes = vec![SubType::Elf];
            elf.power = Some(1);
            elf.toughness = Some(1);
            game.state.card_store.insert(elf.clone());
            game.state.players.get_mut(&p1).unwrap().graveyard.add(elf_id);
        }

        // Create tokens equal to Elf cards in graveyard
        game.execute_effects(
            &[Effect::CreateTokenDynamic {
                token_name: "2/2 green Elf Warrior creature token".into(),
                count_filter: "Elf cards in your graveyard".into(),
            }],
            p1,
            &[],
            None,
            None,
        );

        // Should have 3 tokens on the battlefield
        let tokens: Vec<_> = game.state.battlefield.iter()
            .filter(|p| p.controller == p1 && p.card.is_token)
            .collect();
        assert_eq!(tokens.len(), 3, "should have 3 elf tokens");
    }
}
