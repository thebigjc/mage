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

use crate::abilities::{Cost, Effect, StaticEffect, TargetSpec, TriggerScope};
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
    resolving_ability_id: Option<AbilityId>,
    variable_blight_amount: Option<u32>,
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
            let player = state.players.get_mut(&player_id)
                .expect("player just inserted into state");
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
            resolving_ability_id: None,
            variable_blight_amount: None,
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

                    self.state.trigger_counts_this_turn.clear();
                    self.state.ability_resolution_counts_this_turn.clear();
                    self.state.cast_from_exile_once_used.clear();
                    self.state.tokens_created_this_turn.clear();

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
            perm.abilities_lost = false;
            perm.base_power_override = None;
            perm.base_toughness_override = None;
            perm.cant_untap = false;
            perm.assign_damage_with_toughness = false;
            perm.colorless_override = false;
            perm.subtypes_override = None;
            perm.hexproof_from_colors.clear();
        }
        self.state.damage_doublings.clear();
        self.state.mana_doubling_basic_lands = 0;
        self.state.enhanced_mana_productions.clear();
        self.state.trigger_doublings.clear();
        self.state.token_replacement_effects.clear();

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
        let mut lose_all_abilities: Vec<(ObjectId, PlayerId, String)> = Vec::new();
        let mut set_base_pts: Vec<(ObjectId, PlayerId, String, i32, i32)> = Vec::new();
        let mut cant_untaps: Vec<(ObjectId, PlayerId, String)> = Vec::new();
        let mut set_power_color_counts: Vec<(ObjectId, PlayerId)> = Vec::new();
        let mut assign_damage_toughness: Vec<(ObjectId, PlayerId, String, Option<String>)> = Vec::new();
        let mut damage_doublings: Vec<(ObjectId, PlayerId)> = Vec::new();
        let mut boost_per_turn_events: Vec<(ObjectId, PlayerId, String, String, i32, i32)> = Vec::new();
        let mut becomes_creature_attached: Vec<(ObjectId, Vec<String>, bool)> = Vec::new();
        let mut hexproof_from_own_colors: Vec<(ObjectId, PlayerId)> = Vec::new();

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
                        crate::abilities::StaticEffect::LoseAllAbilities { filter } => {
                            lose_all_abilities.push((source_id, controller, filter.clone()));
                        }
                        crate::abilities::StaticEffect::SetBasePowerToughness { filter, power, toughness } => {
                            set_base_pts.push((source_id, controller, filter.clone(), *power, *toughness));
                        }
                        crate::abilities::StaticEffect::CantUntap { filter } => {
                            cant_untaps.push((source_id, controller, filter.clone()));
                        }
                        crate::abilities::StaticEffect::SetPowerToColorCount => {
                            set_power_color_counts.push((source_id, controller));
                        }
                        crate::abilities::StaticEffect::AssignDamageWithToughness { filter, condition } => {
                            assign_damage_toughness.push((source_id, controller, filter.clone(), condition.clone()));
                        }
                        crate::abilities::StaticEffect::DamageDoublingFromType => {
                            damage_doublings.push((source_id, controller));
                        }
                        crate::abilities::StaticEffect::ManaDoublingBasicLands => {
                            self.state.mana_doubling_basic_lands += 1;
                        }
                        crate::abilities::StaticEffect::EnhancedManaProduction => {
                            if let Some(perm) = self.state.battlefield.get(source_id) {
                                if let (Some(attached_to), Some(color)) = (perm.attached_to, perm.chosen_color) {
                                    self.state.enhanced_mana_productions.push((source_id, attached_to, color));
                                }
                            }
                        }
                        crate::abilities::StaticEffect::TriggerDoubling { filter } => {
                            self.state.trigger_doublings.push((source_id, controller, filter.clone()));
                        }
                        crate::abilities::StaticEffect::BoostPerTurnEvent { filter, event, power_per, toughness_per } => {
                            boost_per_turn_events.push((source_id, controller, filter.clone(), event.clone(), *power_per, *toughness_per));
                        }
                        crate::abilities::StaticEffect::BecomesCreatureAttached { subtypes, colorless } => {
                            if let Some(perm) = self.state.battlefield.get(source_id) {
                                if let Some(attached_to) = perm.attached_to {
                                    becomes_creature_attached.push((attached_to, subtypes.clone(), *colorless));
                                }
                            }
                        }
                        crate::abilities::StaticEffect::HexproofFromOwnColors => {
                            hexproof_from_own_colors.push((source_id, controller));
                        }
                        crate::abilities::StaticEffect::ReplaceTokenCreation => {
                            if let Some(perm) = self.state.battlefield.get(source_id) {
                                if let Some(attached_to) = perm.attached_to {
                                    if self.state.battlefield.get(attached_to).is_some_and(|p| p.card.card_types.contains(&crate::constants::CardType::Creature)) {
                                        self.state.token_replacement_effects.push((source_id, controller, attached_to));
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        // Step 2a: Apply BecomesCreatureAttached (Layer 4 — Type changing, Layer 5 — Color changing)
        for (target_id, subtypes, colorless) in becomes_creature_attached {
            if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                let new_subtypes: Vec<crate::constants::SubType> = subtypes.iter()
                    .map(|s| crate::constants::SubType::by_description(s))
                    .collect();
                perm.subtypes_override = Some(new_subtypes);
                if colorless {
                    perm.colorless_override = true;
                }
            }
        }

        // Step 2b: Apply "loses all abilities" (Layer 6 — Ability removing)
        for (source_id, controller, filter) in lose_all_abilities {
            let matching = self.find_matching_permanents(source_id, controller, &filter);
            for target_id in matching {
                if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                    perm.removed_keywords = KeywordAbilities::all();
                    perm.abilities_lost = true;
                }
            }
        }

        // Step 2c: Apply base P/T overrides (Layer 7b — Set)
        for (source_id, controller, filter, power, toughness) in set_base_pts {
            let matching = self.find_matching_permanents(source_id, controller, &filter);
            for target_id in matching {
                if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                    perm.base_power_override = Some(power);
                    perm.base_toughness_override = Some(toughness);
                }
            }
        }

        // Step 2c2: Apply SetPowerToColorCount (Layer 7b — Vivid power)
        for (source_id, controller) in set_power_color_counts {
            let color_count = self.count_colors_among_permanents(controller) as i32;
            if let Some(perm) = self.state.battlefield.get_mut(source_id) {
                perm.base_power_override = Some(color_count);
            }
        }

        // Step 2d: Apply CantUntap restrictions
        for (source_id, controller, filter) in cant_untaps {
            let matching = self.find_matching_permanents(source_id, controller, &filter);
            for target_id in matching {
                if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                    perm.cant_untap = true;
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
                            // If filter mentions "creature", only count creatures;
                            // otherwise count all cards in graveyard
                            if filter_lower.contains("creature") {
                                if card.is_creature() {
                                    count += 1;
                                }
                            } else {
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

        for (source_id, controller, filter, event, power_per, toughness_per) in boost_per_turn_events {
            let count = match event.as_str() {
                "creatures_entered" => self.watchers.player_stats(controller).creatures_entered as i32,
                _ => 0,
            };
            if count > 0 {
                let matching = self.find_matching_permanents(source_id, controller, &filter);
                for target_id in matching {
                    if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                        perm.continuous_boost_power += count * power_per;
                        perm.continuous_boost_toughness += count * toughness_per;
                    }
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

        // Step 4b: Apply HexproofFromOwnColors (Layer 6 — Ability adding)
        for (source_id, controller) in hexproof_from_own_colors {
            let creature_colors: Vec<(ObjectId, Vec<crate::constants::Color>)> = self.state.battlefield.iter()
                .filter(|p| p.is_creature() && p.controller == controller && p.id() != source_id)
                .map(|p| {
                    let colors = if p.all_colors_until_eot {
                        vec![crate::constants::Color::White, crate::constants::Color::Blue,
                             crate::constants::Color::Black, crate::constants::Color::Red,
                             crate::constants::Color::Green]
                    } else if p.colorless_override {
                        vec![]
                    } else {
                        p.card.colors()
                    };
                    (p.id(), colors)
                })
                .collect();
            for (target_id, colors) in creature_colors {
                if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                    for c in colors {
                        if !perm.hexproof_from_colors.contains(&c) {
                            perm.hexproof_from_colors.push(c);
                        }
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

        // Step 8: Apply "assigns combat damage equal to toughness" (after all P/T is finalized)
        for (source_id, controller, filter, condition) in assign_damage_toughness {
            let matching = self.find_matching_permanents(source_id, controller, &filter);
            for target_id in matching {
                if let Some(cond) = &condition {
                    if cond == "toughness_greater_than_power" {
                        if let Some(perm) = self.state.battlefield.get(target_id) {
                            if perm.toughness() <= perm.power() {
                                continue;
                            }
                        }
                    }
                }
                if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                    perm.assign_damage_with_toughness = true;
                }
            }
        }

        // Step 9: Collect damage doubling effects (source permanent's chosen_type)
        for (source_id, controller) in damage_doublings {
            if let Some(perm) = self.state.battlefield.get(source_id) {
                if let Some(ref chosen) = perm.chosen_type {
                    self.state.damage_doublings.push((controller, chosen.clone()));
                }
            }
        }
    }

    /// Evaluate a condition string for conditional static effects.
    /// Returns true if the condition is currently met.
    fn evaluate_condition(&self, source_id: ObjectId, controller: PlayerId, condition: &str) -> bool {
        self.evaluate_condition_with_targets(source_id, controller, condition, &[])
    }

    fn evaluate_condition_with_targets(&self, source_id: ObjectId, controller: PlayerId, condition: &str, targets: &[ObjectId]) -> bool {
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

        // "source is a {Type}" — source permanent has the specified subtype
        if cond_lower.starts_with("source is a ") || cond_lower.starts_with("source is an ") {
            let type_str = if cond_lower.starts_with("source is an ") {
                &condition[13..]
            } else {
                &condition[12..]
            };
            let subtype = crate::constants::SubType::by_description(type_str);
            return self.state.battlefield.get(source_id)
                .map(|p| p.has_subtype(&subtype))
                .unwrap_or(false);
        }

        // "target is a {Type}" — first target has the specified subtype
        if cond_lower.starts_with("target is a ") || cond_lower.starts_with("target is an ") {
            let type_str = if cond_lower.starts_with("target is an ") {
                &condition[13..]
            } else {
                &condition[12..]
            };
            let subtype = crate::constants::SubType::by_description(type_str);
            if let Some(&target_id) = targets.first() {
                return self.state.battlefield.get(target_id)
                    .map(|p| p.has_subtype(&subtype))
                    .unwrap_or(false);
            }
            return false;
        }

        // "you control N or more {filter}" — count permanents matching filter
        if let Some(rest) = cond_lower.strip_prefix("you control ") {
            if let Some(idx) = rest.find(" or more ") {
                let n_str = &rest[..idx];
                let filter = &condition[("you control ".len() + idx + " or more ".len())..];
                if let Ok(n) = n_str.parse::<u32>() {
                    let count = self.count_permanents_matching(controller, filter);
                    return count >= n;
                }
            }
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

    fn count_permanents_matching(&self, controller: PlayerId, filter: &str) -> u32 {
        let filter_lower = filter.to_lowercase();
        self.state.battlefield.iter()
            .filter(|p| p.controller == controller)
            .filter(|p| {
                if filter_lower.contains(" and/or ") {
                    let parts: Vec<&str> = filter.split(" and/or ").collect();
                    parts.iter().any(|part| self.permanent_matches_filter_part(p, part.trim()))
                } else if filter_lower.contains(" or ") {
                    let parts: Vec<&str> = filter.split(" or ").collect();
                    parts.iter().any(|part| self.permanent_matches_filter_part(p, part.trim()))
                } else {
                    self.permanent_matches_filter_part(p, filter)
                }
            })
            .count() as u32
    }

    fn permanent_matches_filter_part(&self, perm: &crate::permanent::Permanent, filter_part: &str) -> bool {
        let part_lower = filter_part.trim().to_lowercase();
        if part_lower == "lands" || part_lower == "land" {
            return perm.card.card_types.contains(&crate::constants::CardType::Land);
        }
        if part_lower == "creatures" || part_lower == "creature" {
            return perm.card.card_types.contains(&crate::constants::CardType::Creature);
        }
        if part_lower == "artifacts" || part_lower == "artifact" {
            return perm.card.card_types.contains(&crate::constants::CardType::Artifact);
        }
        if part_lower == "enchantments" || part_lower == "enchantment" {
            return perm.card.card_types.contains(&crate::constants::CardType::Enchantment);
        }
        let trimmed = filter_part.trim();
        let singular = trimmed.strip_suffix('s').unwrap_or(trimmed);
        let subtype = crate::constants::SubType::by_description(singular);
        perm.has_subtype(&subtype)
    }

    /// Evaluate a dynamic value source string and return the computed value.
    /// Supports patterns like:
    /// - "Elf cards in your graveyard" — count of Elf creature cards in controller's graveyard
    /// - "Goblins you control" / "Kithkin you control" — count of matching permanents
    /// - "greatest power among Giants you control" — max power among matching creatures
    fn evaluate_count_filter(&self, filter: &str, controller: PlayerId) -> u32 {
        if filter.contains(" + ") {
            return filter.split(" + ")
                .map(|part| self.evaluate_count_filter(part.trim(), controller))
                .sum();
        }

        let lower = filter.to_lowercase();

        // "{Type1}s and {Type2}s you control" — count permanents matching either type (OR, no double-counting)
        if lower.ends_with("you control") && lower.contains(" and ") {
            let type_part = lower.trim_end_matches("you control").trim();
            let types: Vec<String> = type_part.split(" and ")
                .map(|t| Self::depluralize_type(t.trim()))
                .collect();
            let subtypes: Vec<crate::constants::SubType> = types.iter()
                .map(|t| crate::constants::SubType::by_description(t))
                .collect();
            return self.state.battlefield.iter()
                .filter(|p| p.controller == controller
                    && subtypes.iter().any(|st| p.has_subtype(st)))
                .count() as u32;
        }

        // "greatest mana value among {Type}s you control"
        if lower.starts_with("greatest mana value among") && lower.ends_with("you control") {
            let middle = &filter[25..]; // skip "greatest mana value among "
            let type_part = middle.trim_end_matches("you control").trim();
            let type_str = type_part.trim_end_matches('s');
            let subtype = crate::constants::SubType::by_description(type_str);
            return self.state.battlefield.iter()
                .filter(|p| p.controller == controller && p.has_subtype(&subtype))
                .map(|p| p.card.mana_value())
                .max()
                .unwrap_or(0);
        }

        // "greatest power among {Type}s you control"
        if lower.starts_with("greatest power among") && lower.ends_with("you control") {
            let middle = &filter[21..]; // skip "greatest power among "
            let type_part = middle.trim_end_matches("you control").trim();
            let type_str = type_part.trim_end_matches('s');
            let subtype = crate::constants::SubType::by_description(type_str);
            return self.state.battlefield.iter()
                .filter(|p| p.controller == controller && p.has_subtype(&subtype))
                .map(|p| std::cmp::max(0, p.power()) as u32)
                .max()
                .unwrap_or(0);
        }

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

        if lower == "attacking creatures you control" {
            return self.state.battlefield.iter()
                .filter(|p| p.controller == controller && p.is_creature()
                    && self.state.combat.attackers.contains(&p.id()))
                .count() as u32;
        }

        // "{Type}s you control" / "{Type} you control"
        if lower.ends_with("you control") {
            let type_part = lower.trim_end_matches("you control").trim();
            let type_str = Self::depluralize_type(type_part);
            let subtype = crate::constants::SubType::by_description(&type_str);
            return self.state.battlefield.iter()
                .filter(|p| p.controller == controller && p.has_subtype(&subtype))
                .count() as u32;
        }

        0 // unknown filter
    }

    fn depluralize_type(lower: &str) -> String {
        let irregular: &[(&str, &str)] = &[
            ("elves", "Elf"),
            ("wolves", "Wolf"),
            ("dwarves", "Dwarf"),
        ];
        for &(plural, singular) in irregular {
            if lower == plural {
                return singular.to_string();
            }
        }
        let trimmed = lower.trim_end_matches('s');
        let mut chars = trimmed.chars();
        match chars.next() {
            Some(c) => c.to_uppercase().to_string() + chars.as_str(),
            None => String::new(),
        }
    }

    /// Calculate the total cost reduction that applies to a spell being cast by a player.
    /// Scans all permanents on the battlefield for CostReduction static effects
    /// whose filter matches the spell's characteristics.
    pub fn calculate_cost_reduction(&self, player_id: PlayerId, card: &crate::card::CardData) -> u32 {
        let mut total_reduction = 0u32;
        for perm in self.state.battlefield.iter() {
            if perm.controller != player_id {
                continue;
            }
            let abilities = self.state.ability_store.for_source(perm.id());
            for ability in abilities {
                if ability.ability_type != crate::constants::AbilityType::Static {
                    continue;
                }
                for effect in &ability.static_effects {
                    if let crate::abilities::StaticEffect::CostReduction { filter, amount, condition } = effect {
                        if self.spell_matches_cost_filter(card, filter) {
                            if let Some(cond) = condition {
                                if cond == "toughness_greater_than_power" {
                                    let t = card.toughness.unwrap_or(0);
                                    let p = card.power.unwrap_or(0);
                                    if t <= p {
                                        continue;
                                    }
                                }
                            }
                            total_reduction += amount;
                        }
                    }
                    if let crate::abilities::StaticEffect::CostReductionDynamic { filter, value_source } = effect {
                        if self.spell_matches_cost_filter(card, filter) {
                            let dynamic_amount = self.evaluate_count_filter(value_source, player_id);
                            total_reduction += dynamic_amount;
                        }
                    }
                }
            }
        }
        for ability in &card.abilities {
            if ability.ability_type != crate::constants::AbilityType::Static {
                continue;
            }
            for effect in &ability.static_effects {
                if let crate::abilities::StaticEffect::CostReductionDynamic { value_source, .. } = effect {
                    let dynamic_amount = self.evaluate_count_filter(value_source, player_id);
                    total_reduction += dynamic_amount;
                }
            }
        }
        total_reduction
    }

    pub fn spell_has_convoke(&self, player_id: PlayerId, card: &crate::card::CardData) -> bool {
        if card.keywords.contains(crate::constants::KeywordAbilities::CONVOKE) {
            return true;
        }
        for perm in self.state.battlefield.iter() {
            if perm.controller != player_id {
                continue;
            }
            let abilities = self.state.ability_store.for_source(perm.id());
            for ability in abilities {
                if ability.ability_type != crate::constants::AbilityType::Static {
                    continue;
                }
                for effect in &ability.static_effects {
                    if let crate::abilities::StaticEffect::GrantConvoke { filter } = effect {
                        if self.spell_matches_cost_filter(card, filter) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn calculate_convoke_mana(&self, player_id: PlayerId) -> crate::mana::Mana {
        let mut convoke = crate::mana::Mana::new();
        for perm in self.state.battlefield.iter() {
            if perm.controller != player_id || perm.tapped || !perm.is_creature() {
                continue;
            }
            let colors = perm.card.colors();
            if colors.is_empty() {
                convoke.generic += 1;
            } else {
                convoke.any += 1;
            }
        }
        convoke
    }

    fn pay_convoke_cost(&mut self, player_id: PlayerId, shortfall: &crate::mana::Mana) -> crate::mana::Mana {
        let mut produced = crate::mana::Mana::new();
        let mut remaining_colored = crate::mana::Mana::new();
        remaining_colored.white = shortfall.white;
        remaining_colored.blue = shortfall.blue;
        remaining_colored.black = shortfall.black;
        remaining_colored.red = shortfall.red;
        remaining_colored.green = shortfall.green;
        let mut remaining_generic = shortfall.generic;

        let creature_ids: Vec<ObjectId> = self.state.battlefield.iter()
            .filter(|p| p.controller == player_id && !p.tapped && p.is_creature())
            .map(|p| p.id())
            .collect();

        for cid in &creature_ids {
            if remaining_colored.count() == 0 && remaining_generic == 0 {
                break;
            }
            let colors = self.state.card_store.get(*cid)
                .map(|c| c.colors())
                .unwrap_or_default();

            let mut tapped = false;
            for color in &colors {
                let field = match color {
                    crate::constants::Color::White => &mut remaining_colored.white,
                    crate::constants::Color::Blue => &mut remaining_colored.blue,
                    crate::constants::Color::Black => &mut remaining_colored.black,
                    crate::constants::Color::Red => &mut remaining_colored.red,
                    crate::constants::Color::Green => &mut remaining_colored.green,
                };
                if *field > 0 {
                    *field -= 1;
                    if let Some(perm) = self.state.battlefield.get_mut(*cid) {
                        perm.tapped = true;
                    }
                    match color {
                        crate::constants::Color::White => produced.white += 1,
                        crate::constants::Color::Blue => produced.blue += 1,
                        crate::constants::Color::Black => produced.black += 1,
                        crate::constants::Color::Red => produced.red += 1,
                        crate::constants::Color::Green => produced.green += 1,
                    }
                    tapped = true;
                    break;
                }
            }

            if !tapped && remaining_generic > 0 {
                remaining_generic -= 1;
                if let Some(perm) = self.state.battlefield.get_mut(*cid) {
                    perm.tapped = true;
                }
                produced.colorless += 1;
            }
        }
        produced
    }

    pub fn spell_has_conspire(&self, player_id: PlayerId, card: &crate::card::CardData) -> bool {
        if card.keywords.contains(crate::constants::KeywordAbilities::CONSPIRE) {
            return true;
        }
        for perm in self.state.battlefield.iter() {
            if perm.controller != player_id {
                continue;
            }
            let abilities = self.state.ability_store.for_source(perm.id());
            for ability in abilities {
                if ability.ability_type != crate::constants::AbilityType::Static {
                    continue;
                }
                for effect in &ability.static_effects {
                    if let crate::abilities::StaticEffect::GrantConspire { filter } = effect {
                        if self.spell_matches_cost_filter(card, filter) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn count_conspire_eligible_creatures(&self, player_id: PlayerId, spell_colors: &[crate::constants::Color]) -> u32 {
        let mut count = 0u32;
        for perm in self.state.battlefield.iter() {
            if perm.controller != player_id || perm.tapped || !perm.is_creature() {
                continue;
            }
            let creature_colors = perm.card.colors();
            let shares_color = creature_colors.iter().any(|c| spell_colors.contains(c))
                || perm.has_keyword(crate::constants::KeywordAbilities::CHANGELING);
            if shares_color {
                count += 1;
            }
        }
        count
    }

    fn pay_conspire_cost(&mut self, player_id: PlayerId, spell_colors: &[crate::constants::Color]) {
        let mut tapped = 0u32;
        let creature_ids: Vec<ObjectId> = self.state.battlefield.iter()
            .filter(|p| p.controller == player_id && !p.tapped && p.is_creature())
            .filter(|p| {
                let colors = p.card.colors();
                colors.iter().any(|c| spell_colors.contains(c))
                    || p.has_keyword(crate::constants::KeywordAbilities::CHANGELING)
            })
            .map(|p| p.id())
            .collect();

        for cid in creature_ids {
            if tapped >= 2 {
                break;
            }
            if let Some(perm) = self.state.battlefield.get_mut(cid) {
                perm.tapped = true;
                tapped += 1;
            }
        }
    }

    fn copy_spell_on_stack(&mut self, spell_id: ObjectId, controller: PlayerId) {
        let stack_item = self.state.stack.get(spell_id);
        if let Some(item) = stack_item {
            if let crate::zones::StackItemKind::Spell { card } = &item.kind {
                let card_copy = card.clone();
                let targets = item.targets.clone();
                let x_value = item.x_value;
                let copy_id = ObjectId::new();
                let copy_item = crate::zones::StackItem {
                    id: copy_id,
                    kind: crate::zones::StackItemKind::Spell { card: card_copy },
                    controller,
                    targets,
                    countered: false,
                    x_value,
                    exile_on_resolve: false,
                };
                self.state.stack.push(copy_item);
            }
        }
    }

    fn find_triggering_spell(&self, controller: PlayerId) -> Option<ObjectId> {
        for item in self.state.stack.iter() {
            if let crate::zones::StackItemKind::Spell { card } = &item.kind {
                if item.controller == controller && (card.is_instant() || card.is_sorcery()) {
                    return Some(item.id);
                }
            }
        }
        None
    }

    fn grant_keywords_to_spell(&mut self, spell_id: ObjectId, keywords: &[String]) {
        if let Some(item) = self.state.stack.get_mut(spell_id) {
            if let crate::zones::StackItemKind::Spell { card } = &mut item.kind {
                for kw_name in keywords {
                    if let Some(flag) = crate::constants::KeywordAbilities::keyword_from_name(kw_name) {
                        card.keywords |= flag;
                    }
                }
            }
        }
    }

    /// Check if a spell/card matches a cost reduction filter string.
    fn spell_matches_cost_filter(&self, card: &crate::card::CardData, filter: &str) -> bool {
        let lower = filter.to_lowercase();

        // "self" — only the source card itself (doesn't apply to other spells)
        if lower == "self" {
            return false;
        }

        // Subtype match: "Elf", "Goblin", "Merfolk", etc.
        let subtype = crate::constants::SubType::by_description(filter);
        if card.subtypes.contains(&subtype) {
            return true;
        }

        // "creature spells" / "creature"
        if lower == "creature spells" || lower == "creature" {
            return card.card_types.contains(&crate::constants::CardType::Creature);
        }

        // "noncreature spells"
        if lower == "noncreature spells" {
            return !card.card_types.contains(&crate::constants::CardType::Creature);
        }

        // "instant and sorcery spells"
        if lower.contains("instant") && lower.contains("sorcery") {
            return card.is_instant() || card.card_types.contains(&crate::constants::CardType::Sorcery);
        }

        // Card type match: "artifact", "enchantment", etc.
        for ct in &card.card_types {
            let ct_name = format!("{ct:?}").to_lowercase();
            if lower == ct_name || lower == format!("{ct_name} spells") {
                return true;
            }
        }

        false
    }

    /// Find permanents matching a filter string, relative to a source permanent.
    ///
    /// Handles common filter patterns:
    /// - `"self"` — only the source permanent
    /// - `"enchanted creature"` / `"equipped creature"` — the permanent this is attached to
    /// - `"other X you control"` — excludes source, controller must match
    fn get_damage_multiplier(&self, source_id: ObjectId) -> u32 {
        if self.state.damage_doublings.is_empty() {
            return 1;
        }
        let perm = match self.state.battlefield.get(source_id) {
            Some(p) => p,
            None => return 1,
        };
        let mut multiplier = 1u32;
        for (controller, ref subtype) in &self.state.damage_doublings {
            if perm.controller == *controller && perm.has_subtype(subtype) {
                multiplier *= 2;
            }
        }
        multiplier
    }

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
        let is_nontoken = f.contains("nontoken");
        let is_token = !is_nontoken && f.contains("token");

        // Strip modifiers to get the core type filter
        let type_filter = f
            .replace("other ", "")
            .replace("attacking ", "")
            .replace("you control", "")
            .replace("nontoken ", "")
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
            if is_nontoken && perm.card.is_token {
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

    fn check_enters_with_counters(&mut self, permanent_id: ObjectId) {
        let counters_to_add: Vec<(String, u32)> = {
            let abilities = self.state.ability_store.for_source(permanent_id);
            abilities.iter()
                .filter(|a| a.ability_type == AbilityType::Static)
                .flat_map(|a| a.static_effects.iter())
                .filter_map(|e| {
                    if let crate::abilities::StaticEffect::EntersWithCounters { counter_type, count } = e {
                        Some((counter_type.clone(), *count))
                    } else {
                        None
                    }
                })
                .collect()
        };
        for (counter_type, count) in counters_to_add {
            let ct = crate::counters::CounterType::from_name(&counter_type);
            if let Some(perm) = self.state.battlefield.get_mut(permanent_id) {
                perm.add_counters(ct, count);
            }
        }
    }

    fn check_enter_as_copy(&mut self, permanent_id: ObjectId) {
        let copy_info: Option<(String, Vec<String>)> = {
            let abilities = self.state.ability_store.for_source(permanent_id);
            abilities.iter()
                .filter(|a| a.ability_type == AbilityType::Static)
                .flat_map(|a| a.static_effects.iter())
                .find_map(|e| {
                    if let crate::abilities::StaticEffect::EnterAsACopy { filter, add_keywords } = e {
                        Some((filter.clone(), add_keywords.clone()))
                    } else {
                        None
                    }
                })
        };

        let (filter, add_keywords) = match copy_info {
            Some(info) => info,
            None => return,
        };

        let controller = match self.state.battlefield.get(permanent_id) {
            Some(p) => p.controller,
            None => return,
        };

        let eligible: Vec<ObjectId> = self.state.battlefield.iter()
            .filter(|p| p.is_creature() && p.id() != permanent_id)
            .filter(|_p| {
                let _f = filter.to_lowercase();
                true
            })
            .map(|p| p.id())
            .collect();

        if eligible.is_empty() {
            return;
        }

        let mut options: Vec<crate::decision::NamedChoice> = eligible.iter().enumerate()
            .map(|(i, &cid)| {
                let name = self.state.battlefield.get(cid)
                    .map(|p| {
                        let pt = if let (Some(pow), Some(tou)) = (p.card.power, p.card.toughness) {
                            format!(" ({pow}/{tou})")
                        } else {
                            String::new()
                        };
                        format!("{}{}", p.name(), pt)
                    })
                    .unwrap_or_else(|| "Unknown".to_string());
                crate::decision::NamedChoice { index: i, description: name }
            })
            .collect();
        options.push(crate::decision::NamedChoice {
            index: eligible.len(),
            description: "Don't copy".to_string(),
        });

        let view = crate::decision::GameView::placeholder();
        let choice_idx = if let Some(dm) = self.decision_makers.get_mut(&controller) {
            dm.choose_option(&view, crate::constants::Outcome::Benefit,
                "Choose a creature to copy", &options)
        } else {
            eligible.len()
        };

        if choice_idx >= eligible.len() {
            return;
        }

        let target_id = eligible[choice_idx];
        let source_card = match self.state.battlefield.get(target_id) {
            Some(p) => p.card.clone(),
            None => return,
        };

        self.state.ability_store.remove_source(permanent_id);

        if let Some(perm) = self.state.battlefield.get_mut(permanent_id) {
            let original_id = perm.card.id;
            let original_owner = perm.card.owner;

            perm.card = source_card;
            perm.card.id = original_id;
            perm.card.owner = original_owner;
            perm.card.is_token = false;

            for kw_name in &add_keywords {
                if let Some(flag) = crate::constants::KeywordAbilities::keyword_from_name(kw_name) {
                    perm.card.keywords |= flag;
                }
            }

            perm.card.abilities = perm.card.abilities.iter().map(|ab| {
                let mut new_ab = ab.clone();
                new_ab.id = crate::types::AbilityId::new();
                new_ab.source_id = original_id;
                new_ab
            }).collect();

            perm.summoning_sick = perm.card.is_creature();
        }

        if let Some(perm) = self.state.battlefield.get(permanent_id) {
            for ab in &perm.card.abilities {
                self.state.ability_store.add(ab.clone());
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

        let mut triggered: Vec<(PlayerId, AbilityId, ObjectId, String, Option<ObjectId>, i32)> = Vec::new();

        for event in self.event_log.iter() {
            let matching = self.state.ability_store.triggered_by(event);
            for ability in matching {
                let is_dies_trigger = event.event_type == EventType::Dies;

                if is_dies_trigger {
                    match ability.trigger_scope {
                        TriggerScope::SelfOnly => {
                            if let Some(target_id) = event.target_id {
                                if target_id != ability.source_id {
                                    continue;
                                }
                            }
                        }
                        TriggerScope::OtherControlled => {
                            if let Some(target_id) = event.target_id {
                                if target_id == ability.source_id {
                                    continue;
                                }
                            }
                            let source_on_bf = self.state.battlefield.contains(ability.source_id);
                            if !source_on_bf {
                                continue;
                            }
                            let ab_controller = self.state.battlefield.get(ability.source_id)
                                .map(|p| p.controller);
                            if let Some(ac) = ab_controller {
                                if let Some(player_id) = event.player_id {
                                    if player_id != ac {
                                        continue;
                                    }
                                }
                            }
                        }
                        TriggerScope::Any => {
                            let source_on_bf = self.state.battlefield.contains(ability.source_id);
                            if !source_on_bf {
                                continue;
                            }
                        }
                    }
                    let controller = if ability.trigger_scope == TriggerScope::SelfOnly {
                        event.player_id.unwrap_or(self.state.active_player)
                    } else {
                        self.state.battlefield.get(ability.source_id)
                            .map(|p| p.controller)
                            .unwrap_or(self.state.active_player)
                    };

                    if ability.triggers_per_turn > 0 {
                        let count = self.state.trigger_counts_this_turn.get(&ability.id).copied().unwrap_or(0);
                        if count >= ability.triggers_per_turn {
                            continue;
                        }
                    }

                    triggered.push((
                        controller,
                        ability.id,
                        ability.source_id,
                        ability.rules_text.clone(),
                        event.target_id,
                        event.amount,
                    ));
                    continue;
                }

                let source_on_bf = self.state.battlefield.contains(ability.source_id);
                if !source_on_bf {
                    continue;
                }

                let controller = self
                    .state
                    .battlefield
                    .get(ability.source_id)
                    .map(|p| p.controller)
                    .unwrap_or(self.state.active_player);

                if event.event_type == EventType::AttackerDeclared || event.event_type == EventType::BlockerDeclared {
                    match ability.trigger_scope {
                        TriggerScope::SelfOnly => {
                            if let Some(target_id) = event.target_id {
                                if target_id != ability.source_id {
                                    continue;
                                }
                            }
                        }
                        TriggerScope::OtherControlled => {
                            if let Some(target_id) = event.target_id {
                                if target_id == ability.source_id {
                                    continue;
                                }
                            }
                            if let Some(target_id) = event.target_id {
                                let creature_controller = self.state.battlefield.get(target_id)
                                    .map(|p| p.controller);
                                if creature_controller != Some(controller) {
                                    continue;
                                }
                            }
                        }
                        TriggerScope::Any => {
                            if let Some(target_id) = event.target_id {
                                let creature_controller = self.state.battlefield.get(target_id)
                                    .map(|p| p.controller);
                                if creature_controller != Some(controller) {
                                    continue;
                                }
                            }
                        }
                    }
                }

                if event.event_type == EventType::EnteredTheBattlefield {
                    match ability.trigger_scope {
                        TriggerScope::SelfOnly => {
                            if let Some(target_id) = event.target_id {
                                if target_id != ability.source_id {
                                    continue;
                                }
                            }
                        }
                        TriggerScope::OtherControlled => {
                            if let Some(target_id) = event.target_id {
                                if target_id == ability.source_id {
                                    continue;
                                }
                            }
                            if let Some(player_id) = event.player_id {
                                if player_id != controller {
                                    continue;
                                }
                            }
                            if let Some(required_zone) = ability.trigger_from_zone {
                                if event.zone != Some(required_zone) {
                                    continue;
                                }
                            }
                        }
                        TriggerScope::Any => {}
                    }
                }

                if event.event_type == EventType::GainLife {
                    if let Some(player_id) = event.player_id {
                        if player_id != controller {
                            continue;
                        }
                    }
                }

                if event.event_type == EventType::UpkeepStep || event.event_type == EventType::EndStep || event.event_type == EventType::PrecombatMainPre {
                    if let Some(player_id) = event.player_id {
                        if player_id != controller {
                            continue;
                        }
                    }
                }

                if event.event_type == EventType::DamagedPlayer {
                    if let Some(target_id) = event.target_id {
                        if target_id != ability.source_id {
                            continue;
                        }
                    }
                }

                if ability.triggers_per_turn > 0 {
                    let count = self.state.trigger_counts_this_turn.get(&ability.id).copied().unwrap_or(0);
                    if count >= ability.triggers_per_turn {
                        continue;
                    }
                }

                triggered.push((
                    controller,
                    ability.id,
                    ability.source_id,
                    ability.rules_text.clone(),
                    event.target_id,
                    event.amount,
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
        let mut delayed_fired: Vec<(usize, crate::state::DelayedTrigger, Option<ObjectId>)> = Vec::new();
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
                if dt.copy_spell {
                    if let Some(pid) = event.player_id {
                        if pid != dt.controller {
                            continue;
                        }
                    } else {
                        continue;
                    }
                    if let Some(spell_id) = event.target_id {
                        if let Some(card) = self.state.card_store.get(spell_id) {
                            if !card.is_instant() && !card.is_sorcery() {
                                continue;
                            }
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
                if let Some(ref _filter) = dt.controller_filter {
                    if let Some(source_id) = event.target_id {
                        if let Some(perm) = self.state.battlefield.get(source_id) {
                            if perm.controller != dt.controller {
                                continue;
                            }
                            if !perm.is_creature() {
                                continue;
                            }
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
                delayed_fired.push((idx, dt.clone(), event.target_id));
            }
        }
        // Remove fired trigger-only-once entries (reverse order to preserve indices)
        let mut indices_to_remove: Vec<usize> = delayed_fired.iter()
            .filter(|(_, dt, _)| dt.trigger_only_once)
            .map(|(idx, _, _)| *idx)
            .collect();
        indices_to_remove.sort_unstable();
        indices_to_remove.dedup();
        for idx in indices_to_remove.into_iter().rev() {
            self.state.delayed_triggers.remove(idx);
        }
        // Execute delayed trigger effects
        for (_, dt, event_target) in &delayed_fired {
            if dt.copy_spell {
                if let Some(spell_id) = event_target {
                    self.copy_spell_on_stack(*spell_id, dt.controller);
                }
            } else {
                let x_val = dt.stored_value.map(|v| v.max(0) as u32);
                self.execute_effects(&dt.effects, dt.controller, &dt.targets, dt.source, x_val);
            }
        }

        // Clear event log after processing
        self.event_log.clear();

        if triggered.is_empty() && delayed_fired.is_empty() {
            return false;
        }

        // Apply trigger doubling: duplicate triggers whose source matches a TriggerDoubling filter
        if !self.state.trigger_doublings.is_empty() {
            let mut extra: Vec<(PlayerId, AbilityId, ObjectId, String, Option<ObjectId>, i32)> = Vec::new();
            for &(ref _controller, ref _ability_id, ref source_id, ref _desc, ref _event_target, event_amount) in &triggered {
                let controller = *_controller;
                let ability_id = *_ability_id;
                let source_id = *source_id;
                let desc = _desc.clone();
                let et = *_event_target;
                for &(doubler_source, doubler_controller, ref filter) in &self.state.trigger_doublings {
                    if doubler_controller != controller {
                        continue;
                    }
                    if let Some(source_perm) = self.state.battlefield.get(source_id) {
                        let f = filter.to_lowercase();
                        let exclude_doubler = f.contains("other");
                        if exclude_doubler && source_id == doubler_source {
                            continue;
                        }
                        let stripped = f.replace("other ", "").replace("you control", "").trim().to_string();
                        if !stripped.is_empty() && !Self::matches_filter(source_perm, &stripped) {
                            continue;
                        }
                        extra.push((controller, ability_id, source_id, desc.clone(), et, event_amount));
                    }
                }
            }
            triggered.extend(extra);
        }

        // Sort by APNAP order (active player's triggers first)
        let active = self.state.active_player;
        triggered.sort_by_key(|(controller, _, _, _, _, _)| if *controller == active { 0 } else { 1 });

        // Push triggered abilities onto the stack
        for (controller, ability_id, source_id, description, event_target, event_amount) in triggered {
            let ability = self.state.ability_store.get(ability_id).cloned();
            if let Some(ref ab) = ability {
                if ab.optional_trigger {
                    let view = crate::decision::GameView::placeholder();
                    let use_it = if let Some(dm) = self.decision_makers.get_mut(&controller) {
                        dm.choose_use(
                            &view,
                            crate::constants::Outcome::Benefit,
                            &format!("Use triggered ability: {description}?"),
                        )
                    } else {
                        false
                    };
                    if !use_it {
                        continue;
                    }
                }
            }

            let mut targets = if let Some(ref ab) = ability {
                self.select_targets_for_spec(&ab.targets, controller, &[])
            } else {
                Vec::new()
            };

            if let Some(et) = event_target {
                if let Some(ref ab) = ability {
                    if ab.trigger_scope != TriggerScope::SelfOnly && targets.is_empty() {
                        targets.push(et);
                    }
                }
            }

            if let Some(ref ab) = ability {
                if ab.triggers_per_turn > 0 {
                    *self.state.trigger_counts_this_turn.entry(ability_id).or_insert(0) += 1;
                }
            }

            let stack_item = crate::zones::StackItem {
                id: ObjectId::new(),
                kind: crate::zones::StackItemKind::Ability {
                    source_id,
                    ability_id,
                    description,
                },
                controller,
                targets,
                countered: false,
                x_value: if event_amount > 0 { Some(event_amount as u32) } else { None },
                exile_on_resolve: false,
            };
            self.state.stack.push(stack_item);
        }

        true
    }

    /// Emit an event to the event log (for triggered ability checking).
    fn emit_event(&mut self, event: GameEvent) {
        self.watchers.watch(&event);
        self.event_log.push(event);
    }

    /// Execute turn-based actions for a step.
    fn turn_based_actions(&mut self, step: PhaseStep, active_player: PlayerId) {
        match step {
            PhaseStep::Untap => {
                // Untap all permanents controlled by the active player
                // (skip permanents with cant_untap restriction)
                for perm in self.state.battlefield.iter_mut() {
                    if perm.controller == active_player {
                        if !perm.cant_untap {
                            perm.untap();
                        }
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
            PhaseStep::PrecombatMain => {
                let mut main_event = GameEvent::new(EventType::PrecombatMainPre);
                main_event.player_id = Some(active_player);
                self.emit_event(main_event);
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
                    // Clear temporary type additions (BecomesCreature)
                    perm.added_card_types.clear();
                    perm.base_power_eot = None;
                    perm.base_toughness_eot = None;
                    perm.all_colors_until_eot = false;
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
                    match &ip.duration {
                        crate::state::ImpulseDuration::EndOfTurn => false,
                        crate::state::ImpulseDuration::UntilEndOfNextTurn => {
                            !(active_player == ip.player_id
                              && turn_num > ip.created_turn)
                        }
                        crate::state::ImpulseDuration::WhileSourceControlled { .. } => true,
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
                self.emit_event(
                    GameEvent::new(EventType::BlockerDeclared)
                        .target(blocker_id)
                        .player(def_player),
                );
            }
        }

        // Validate block restrictions: max_blocked_by and menace
        for group in &mut self.state.combat.groups {
            // max_blocked_by: trim excess blockers
            if let Some(attacker) = self.state.battlefield.get(group.attacker_id) {
                if let Some(max) = attacker.max_blocked_by {
                    while group.blockers.len() > max as usize {
                        let removed = group.blockers.pop()
                            .expect("pop after len check");
                        self.state.combat.blocker_to_attacker.remove(&removed);
                    }
                }
                // menace: if only 1 blocker, remove it (must have 2+)
                if attacker.has_menace() && group.blockers.len() == 1 {
                    let removed = group.blockers.pop()
                        .expect("pop after len == 1 check");
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
                combat::assign_combat_damage(group, attacker_info, &blocker_refs, is_first_strike);

            let attacker_mult = self.get_damage_multiplier(group.attacker_id);
            for (target_id, amount, is_player) in &attacker_dmg {
                let final_amount = *amount * attacker_mult;
                damage_events.push((*target_id, final_amount, *is_player, group.attacker_id));
                if attacker_has_lifelink && final_amount > 0 {
                    lifelink_sources.push((attacker_controller, final_amount));
                }
            }

            // Assign blocker damage to attacker
            for (blocker_id, blocker_perm) in &blockers {
                let blocker_dmg =
                    combat::assign_blocker_damage(blocker_perm, group.attacker_id, is_first_strike);
                if blocker_dmg > 0 {
                    let blocker_mult = self.get_damage_multiplier(*blocker_id);
                    let final_blocker_dmg = blocker_dmg * blocker_mult;
                    damage_events.push((group.attacker_id, final_blocker_dmg, false, *blocker_id));
                    if blocker_perm.has_lifelink() {
                        lifelink_sources.push((blocker_perm.controller, final_blocker_dmg));
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

                // Check if the player can pay the mana cost (with cost reduction)
                let base_mana_cost = card.mana_cost.to_mana();
                let reduction = self.calculate_cost_reduction(player_id, card);
                let mana_cost = base_mana_cost.reduce_generic(reduction);
                let available = player.mana_pool.available();

                let can_afford = if self.spell_has_convoke(player_id, card) {
                    let convoke_mana = self.calculate_convoke_mana(player_id);
                    available.can_pay_with_convoke(&mana_cost, &convoke_mana)
                } else {
                    available.can_pay(&mana_cost)
                };

                if can_afford {
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
            if !self.state.exile.contains(impulse.card_id) {
                continue;
            }
            if let crate::state::ImpulseDuration::WhileSourceControlled { source_id, controller } = &impulse.duration {
                let still_valid = self.state.battlefield.get(*source_id)
                    .map(|p| p.controller == *controller)
                    .unwrap_or(false);
                if !still_valid {
                    continue;
                }
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
                        let base_cost = card.mana_cost.to_mana();
                        let reduction = self.calculate_cost_reduction(player_id, card);
                        let mana_cost = base_cost.reduce_generic(reduction);
                        let available = player.mana_pool.available();
                        let can_afford = if self.spell_has_convoke(player_id, card) {
                            let convoke_mana = self.calculate_convoke_mana(player_id);
                            available.can_pay_with_convoke(&mana_cost, &convoke_mana)
                        } else {
                            available.can_pay(&mana_cost)
                        };
                        if can_afford {
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

        // Check for CastFromExileWithCounterCost static effects
        if can_sorcery && self.state.active_player == player_id {
            let mut exile_castable: Vec<(ObjectId, u32)> = Vec::new();
            for perm in self.state.battlefield.iter() {
                if perm.controller != player_id { continue; }
                let source_id = perm.id();
                let abilities = self.state.ability_store.for_source(source_id);
                for ability in abilities {
                    if ability.ability_type != crate::constants::AbilityType::Static { continue; }
                    for effect in &ability.static_effects {
                        if let crate::abilities::StaticEffect::CastFromExileWithCounterCost { counter_count } = effect {
                            if let Some(zone) = self.state.exile.get_zone(source_id) {
                                for &card_id in &zone.cards {
                                    exile_castable.push((card_id, *counter_count));
                                }
                            }
                        }
                    }
                }
            }
            let total_counters: u32 = self.state.battlefield.iter()
                .filter(|p| p.controller == player_id && p.is_creature())
                .map(|p| p.counters.total_count())
                .sum();
            for (card_id, counter_cost) in exile_castable {
                if total_counters < counter_cost { continue; }
                if !self.state.exile.contains(card_id) { continue; }
                if let Some(card) = self.state.card_store.get(card_id) {
                    if card.owner != player_id { continue; }
                    if !card.is_creature() { continue; }
                    let base_cost = card.mana_cost.to_mana();
                    let reduction = self.calculate_cost_reduction(player_id, card);
                    let mana_cost = base_cost.reduce_generic(reduction);
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

        // Check for CastExiledOncePerTurn static effects
        if can_sorcery && self.state.active_player == player_id {
            let mut once_castable: Vec<(ObjectId, ObjectId, String)> = Vec::new();
            for perm in self.state.battlefield.iter() {
                if perm.controller != player_id { continue; }
                let source_id = perm.id();
                if self.state.cast_from_exile_once_used.contains(&source_id) { continue; }
                let abilities = self.state.ability_store.for_source(source_id);
                for ability in abilities {
                    if ability.ability_type != crate::constants::AbilityType::Static { continue; }
                    for effect in &ability.static_effects {
                        if let crate::abilities::StaticEffect::CastExiledOncePerTurn { mv_count_filter } = effect {
                            if let Some(zone) = self.state.exile.get_zone(source_id) {
                                for &card_id in &zone.cards {
                                    once_castable.push((card_id, source_id, mv_count_filter.clone()));
                                }
                            }
                        }
                    }
                }
            }
            for (card_id, _source_id, mv_count_filter) in once_castable {
                if !self.state.exile.contains(card_id) { continue; }
                if let Some(card) = self.state.card_store.get(card_id) {
                    if card.is_land() { continue; }
                    let max_mv = self.evaluate_count_filter(&mv_count_filter, player_id);
                    if card.mana_value() <= max_mv {
                        actions.push(crate::decision::PlayerAction::CastSpell {
                            card_id,
                            targets: vec![],
                            mode: None,
                            without_mana: true,
                        });
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
            let Some(player) = self.state.players.get_mut(&player_id) else {
                return;
            };
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
            self.check_enters_with_counters(card_id);

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
        let mut without_mana = from_exile && self.state.impulse_playable.iter()
            .any(|ip| ip.card_id == card_id && ip.without_mana);

        // Check if this is a flashback cast from graveyard
        let from_graveyard = !from_exile && card_data.flashback_cost.is_some()
            && self.state.players.get(&player_id)
                .map(|p| p.graveyard.contains(card_id))
                .unwrap_or(false);

        // Check if this is cast from exile via CastFromExileWithCounterCost
        let exile_counter_cost = if !from_exile && !from_graveyard && self.state.exile.contains(card_id) {
            let mut cost = None;
            for perm in self.state.battlefield.iter() {
                if perm.controller != player_id { continue; }
                let source_id = perm.id();
                if let Some(zone) = self.state.exile.get_zone(source_id) {
                    if !zone.cards.contains(&card_id) { continue; }
                }  else { continue; }
                let abilities = self.state.ability_store.for_source(source_id);
                for ability in abilities {
                    if ability.ability_type != crate::constants::AbilityType::Static { continue; }
                    for effect in &ability.static_effects {
                        if let crate::abilities::StaticEffect::CastFromExileWithCounterCost { counter_count } = effect {
                            cost = Some(*counter_count);
                        }
                    }
                }
            }
            cost
        } else {
            None
        };

        // Check if this is cast from exile via CastExiledOncePerTurn
        let exile_once_source = if !from_exile && !from_graveyard && exile_counter_cost.is_none()
            && self.state.exile.contains(card_id) {
            let mut found_source = None;
            for perm in self.state.battlefield.iter() {
                if perm.controller != player_id { continue; }
                let source_id = perm.id();
                if self.state.cast_from_exile_once_used.contains(&source_id) { continue; }
                if let Some(zone) = self.state.exile.get_zone(source_id) {
                    if !zone.cards.contains(&card_id) { continue; }
                } else { continue; }
                let abilities = self.state.ability_store.for_source(source_id);
                for ability in abilities {
                    if ability.ability_type != crate::constants::AbilityType::Static { continue; }
                    for effect in &ability.static_effects {
                        if let crate::abilities::StaticEffect::CastExiledOncePerTurn { .. } = effect {
                            found_source = Some(source_id);
                        }
                    }
                }
            }
            found_source
        } else {
            None
        };

        // Remove from hand, exile, or graveyard
        if from_exile {
            self.state.exile.remove(card_id);
            self.state.impulse_playable.retain(|ip| ip.card_id != card_id);
        } else if from_graveyard {
            if let Some(player) = self.state.players.get_mut(&player_id) {
                player.graveyard.remove(card_id);
            }
        } else if exile_counter_cost.is_some() {
            self.state.exile.remove(card_id);
        } else if let Some(src_id) = exile_once_source {
            self.state.exile.remove(card_id);
            self.state.cast_from_exile_once_used.insert(src_id);
            without_mana = true;
        } else if let Some(player) = self.state.players.get_mut(&player_id) {
            if !player.hand.remove(card_id) {
                return;
            }
        }

        // Pay additional counter removal cost for exile-with-counter casting
        if let Some(counter_cost) = exile_counter_cost {
            let mut remaining = counter_cost;
            let creature_ids: Vec<ObjectId> = self.state.battlefield.iter()
                .filter(|p| p.controller == player_id && p.is_creature())
                .map(|p| p.id())
                .collect();
            for cid in creature_ids {
                if remaining == 0 { break; }
                if let Some(perm) = self.state.battlefield.get_mut(cid) {
                    let available = perm.counters.total_count();
                    let to_remove = remaining.min(available);
                    if to_remove > 0 {
                        let types: Vec<_> = perm.counters.iter()
                            .filter(|(_, &c)| c > 0)
                            .map(|(ct, _)| ct.clone())
                            .collect();
                        for ct in types {
                            if remaining == 0 { break; }
                            let avail = perm.counters.get(&ct);
                            let remove = remaining.min(avail);
                            perm.counters.remove(&ct, remove);
                            remaining -= remove;
                        }
                    }
                }
            }
        }

        // Pay mana cost (with X substituted if applicable), unless free cast
        if !without_mana {
            let reduction = self.calculate_cost_reduction(player_id, &card_data);
            let has_convoke = self.spell_has_convoke(player_id, &card_data);
            let Some(base_cost) = (if from_graveyard {
                card_data.flashback_cost.as_ref().map(|fc| fc.to_mana())
            } else {
                Some(match x_value {
                    Some(x) => card_data.mana_cost.to_mana_with_x(x),
                    None => card_data.mana_cost.to_mana(),
                })
            }) else {
                return;
            };
            let mana_cost = base_cost.reduce_generic(reduction);

            if has_convoke {
                let available = self.state.players.get(&player_id)
                    .map(|p| p.mana_pool.available())
                    .unwrap_or_default();
                if !available.can_pay(&mana_cost) {
                    let shortfall = mana_cost - available;
                    let produced = self.pay_convoke_cost(player_id, &shortfall);
                    if !produced.is_empty() {
                        if let Some(player) = self.state.players.get_mut(&player_id) {
                            player.mana_pool.add(produced, None, false);
                        }
                    }
                }
            }

            if let Some(player) = self.state.players.get_mut(&player_id) {
                if !player.mana_pool.try_pay(&mana_cost) {
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
        if !card_data.additional_costs.is_empty()
            && !self.pay_costs(player_id, card_id, &card_data.additional_costs)
        {
            // Cannot pay additional costs — put card back
            if from_graveyard {
                if let Some(player) = self.state.players.get_mut(&player_id) {
                    player.graveyard.add(card_id);
                }
            } else if from_exile {
                self.state.exile.exile(card_id);
            } else if let Some(player) = self.state.players.get_mut(&player_id) {
                player.hand.add(card_id);
            }
            return;
        }

        let x_value = if let Some(blight_x) = self.variable_blight_amount.take() {
            Some(blight_x)
        } else {
            x_value
        };

        // Select targets based on the spell's TargetSpec
        let target_spec = card_data
            .abilities
            .iter()
            .find(|a| a.ability_type == AbilityType::Spell)
            .map(|a| a.targets.clone())
            .unwrap_or(crate::abilities::TargetSpec::None);
        let spell_colors = card_data.colors();
        let targets = self.select_targets_for_spec(&target_spec, player_id, &spell_colors);

        // Put on the stack
        let stack_item = crate::zones::StackItem {
            id: card_id,
            kind: crate::zones::StackItemKind::Spell { card: Box::new(card_data.clone()) },
            controller: player_id,
            targets,
            countered: false,
            x_value,
            exile_on_resolve: from_graveyard,
        };
        self.state.stack.push(stack_item);
        self.state.set_zone(card_id, crate::constants::Zone::Stack, None);

        // Conspire: if the spell has conspire and the player has 2+ eligible creatures,
        // ask if they want to pay the conspire cost (tap 2 creatures sharing a color).
        // If paid, copy the spell on the stack.
        if self.spell_has_conspire(player_id, &card_data) && !card_data.is_creature() {
            let spell_colors = card_data.colors();
            if self.count_conspire_eligible_creatures(player_id, &spell_colors) >= 2 {
                let view = crate::decision::GameView::placeholder();
                let use_conspire = if let Some(dm) = self.decision_makers.get_mut(&player_id) {
                    dm.choose_use(&view, crate::constants::Outcome::Benefit, "Pay conspire cost (tap two creatures)?")
                } else {
                    false
                };
                if use_conspire {
                    self.pay_conspire_cost(player_id, &spell_colors);
                    self.copy_spell_on_stack(card_id, player_id);
                }
            }
        }

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
            let card_id = self.state.players.get(&payer)
                .and_then(|p| p.hand.iter().next().copied());
            if let Some(card_id) = card_id {
                if let Some(player) = self.state.players.get_mut(&payer) {
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
            let has_explicit_targets = match &item.kind {
                crate::zones::StackItemKind::Ability { ability_id, .. } => {
                    self.state.ability_store.get(*ability_id)
                        .map(|ab| !matches!(ab.targets, TargetSpec::None))
                        .unwrap_or(true)
                }
                _ => true,
            };
            if has_explicit_targets {
                let any_legal = item.targets.iter().any(|&target_id| {
                    self.state.battlefield.contains(target_id)
                        || self.state.stack.get(target_id).is_some()
                });
                if !any_legal {
                    match &item.kind {
                        crate::zones::StackItemKind::Spell { .. } => {
                            self.move_card_to_graveyard(item.id, item.controller);
                        }
                        crate::zones::StackItemKind::Ability { .. } => {
                        }
                    }
                    return;
                }
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
                    let perm = Permanent::new((**card).clone(), item.controller);
                    self.state.battlefield.add(perm);
                    self.state.set_zone(item.id, crate::constants::Zone::Battlefield, None);
                    self.check_enter_as_copy(item.id);
                    self.check_enters_tapped(item.id);
                    self.check_enters_with_counters(item.id);

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
                        self.state.exile.exile(item.id);
                        self.state.set_zone(item.id, crate::constants::Zone::Exile, None);
                    } else if let Some(pos) = self.state.pending_dream_exile.iter().position(|&id| id == item.id) {
                        self.state.pending_dream_exile.remove(pos);
                        self.state.exile.exile(item.id);
                        self.state.set_zone(item.id, crate::constants::Zone::Exile, None);
                        self.state.dream_countered_cards.push(item.id);
                    } else {
                        self.move_card_to_graveyard(item.id, item.controller);
                    }
                }
            }
            crate::zones::StackItemKind::Ability { ability_id, source_id, .. } => {
                let ab_id = *ability_id;
                *self.state.ability_resolution_counts_this_turn.entry(ab_id).or_insert(0) += 1;
                let source = *source_id;
                let ability_data = self.state.ability_store.get(ab_id).cloned();
                if let Some(ability) = ability_data {
                    let targets = item.targets.clone();
                    self.resolving_ability_id = Some(ab_id);
                    self.execute_effects(&ability.effects, item.controller, &targets, Some(source), item.x_value);
                    self.resolving_ability_id = None;
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
        use crate::constants::KeywordAbilities;

        // Players losing the game
        for &pid in &sba.players_losing {
            if let Some(player) = self.state.players.get_mut(&pid) {
                player.lost = true;
            }
        }

        // Track IDs of permanents that die (for deferred ability cleanup)
        let mut died_sources: Vec<ObjectId> = Vec::new();
        let mut persist_returns: Vec<(ObjectId, PlayerId)> = Vec::new();
        let mut undying_returns: Vec<(ObjectId, PlayerId)> = Vec::new();

        // Permanents going to graveyard (0 toughness)
        for &perm_id in &sba.permanents_to_graveyard {
            if let Some(perm) = self.state.battlefield.remove(perm_id) {
                let owner = perm.owner();
                let controller = perm.controller;
                let was_creature = perm.is_creature();
                let counter_count = perm.counters.total_count();
                let has_persist = was_creature && perm.has_keyword(KeywordAbilities::PERSIST)
                    && perm.counters.get(&CounterType::M1M1) == 0;
                let has_undying = was_creature && !has_persist
                    && perm.has_keyword(KeywordAbilities::UNDYING)
                    && perm.counters.get(&CounterType::P1P1) == 0;
                self.move_card_to_graveyard(perm_id, owner);
                if was_creature {
                    self.emit_event(GameEvent::dies(perm_id, controller, counter_count));
                    died_sources.push(perm_id);
                    if has_persist { persist_returns.push((perm_id, owner)); }
                    if has_undying { undying_returns.push((perm_id, owner)); }
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
                let counter_count = perm.counters.total_count();
                let has_persist = was_creature && perm.has_keyword(KeywordAbilities::PERSIST)
                    && perm.counters.get(&CounterType::M1M1) == 0;
                let has_undying = was_creature && !has_persist
                    && perm.has_keyword(KeywordAbilities::UNDYING)
                    && perm.counters.get(&CounterType::P1P1) == 0;
                self.move_card_to_graveyard(perm_id, owner);
                if was_creature {
                    self.emit_event(GameEvent::dies(perm_id, controller, counter_count));
                    died_sources.push(perm_id);
                    if has_persist { persist_returns.push((perm_id, owner)); }
                    if has_undying { undying_returns.push((perm_id, owner)); }
                } else {
                    self.state.ability_store.remove_source(perm_id);
                }
            }
        }

        // Persist: return creature from graveyard to battlefield with a -1/-1 counter
        for (card_id, owner) in persist_returns {
            self.return_from_graveyard_with_counter(card_id, owner, CounterType::M1M1);
        }

        // Undying: return creature from graveyard to battlefield with a +1/+1 counter
        for (card_id, owner) in undying_returns {
            self.return_from_graveyard_with_counter(card_id, owner, CounterType::P1P1);
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
            let mut produced = mana;
            if self.state.mana_doubling_basic_lands > 0 {
                if let Some(perm) = self.state.battlefield.get(source_id) {
                    if perm.is_land() && perm.has_supertype(crate::constants::SuperType::Basic) {
                        for _ in 0..self.state.mana_doubling_basic_lands {
                            produced += mana;
                        }
                    }
                }
            }
            for &(_aura_id, land_id, color) in &self.state.enhanced_mana_productions {
                if land_id == source_id {
                    let bonus = crate::mana::Mana::of_color(color, 1);
                    produced += bonus;
                }
            }
            if let Some(player) = self.state.players.get_mut(&player_id) {
                player.mana_pool.add(produced, None, false);
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
            if perm.all_colors_until_eot {
                return 5;
            }
            if perm.colorless_override {
                continue;
            }
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
                            self.state.card_store.get(perm.id()).is_some_and(|card| {
                                card.subtypes.iter().any(|st| st.to_string().to_lowercase() == ct_lower)
                                || card.keywords.contains(crate::constants::KeywordAbilities::CHANGELING)
                            })
                        })
                        || self.state.players.get(&player_id).is_some_and(|p| {
                            p.hand.iter().any(|&cid| {
                                cid != source_id &&
                                self.state.card_store.get(cid).is_some_and(|card| {
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
                            self.state.card_store.get(perm.id()).is_some_and(|card| {
                                card.subtypes.iter().any(|st| st.to_string().to_lowercase() == ct_lower)
                                || card.keywords.contains(crate::constants::KeywordAbilities::CHANGELING)
                            })
                        })
                        || self.state.players.get(&player_id).is_some_and(|p| {
                            p.hand.iter().any(|&cid| {
                                cid != source_id &&
                                self.state.card_store.get(cid).is_some_and(|card| {
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
                Cost::VariableBlight => {
                    let has_creature = self.state.battlefield.controlled_by(player_id)
                        .any(|p| p.is_creature());
                    if !has_creature { return false; }
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
                    if counter_type_name == "any" {
                        if let Some(perm) = self.state.battlefield.get_mut(source_id) {
                            if perm.counters.total_count() < *count {
                                return false;
                            }
                            let mut remaining = *count;
                            let types: Vec<_> = perm.counters.iter()
                                .filter(|(_, &c)| c > 0)
                                .map(|(ct, _)| ct.clone())
                                .collect();
                            for ct in types {
                                if remaining == 0 { break; }
                                let available = perm.counters.get(&ct);
                                let to_remove = remaining.min(available);
                                perm.counters.remove(&ct, to_remove);
                                remaining -= to_remove;
                            }
                        } else {
                            return false;
                        }
                    } else {
                        let ct = crate::counters::CounterType::from_name(counter_type_name);
                        if let Some(perm) = self.state.battlefield.get_mut(source_id) {
                            let current = perm.counters.get(&ct);
                            if current < *count {
                                return false;
                            }
                            perm.counters.remove(&ct, *count);
                        } else {
                            return false;
                        }
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
                Cost::VariableBlight => {
                    let creatures: Vec<ObjectId> = self.state.battlefield.iter()
                        .filter(|p| p.controller == player_id && p.is_creature())
                        .map(|p| p.id())
                        .collect();
                    if creatures.is_empty() {
                        return false;
                    }
                    let max_x = self.state.battlefield.iter()
                        .filter(|p| p.controller == player_id && p.is_creature())
                        .map(|p| p.toughness().max(0) as u32)
                        .max()
                        .unwrap_or(0);
                    let view = crate::decision::GameView::placeholder();
                    let x = if let Some(dm) = self.decision_makers.get_mut(&player_id) {
                        dm.choose_amount(&view, "Choose X (blight amount)", 0, max_x)
                    } else {
                        max_x
                    };
                    if x > 0 {
                        let chosen = if let Some(dm) = self.decision_makers.get_mut(&player_id) {
                            let targets = dm.choose_targets(&view, crate::constants::Outcome::Detriment,
                                &crate::decision::TargetRequirement {
                                    description: format!("Put {x} -1/-1 counters on creature you control"),
                                    legal_targets: creatures.clone(),
                                    min_targets: 1, max_targets: 1,
                                    required: true,
                                });
                            targets.into_iter().next().unwrap_or(creatures[0])
                        } else {
                            creatures[0]
                        };
                        if let Some(perm) = self.state.battlefield.get_mut(chosen) {
                            perm.counters.add(crate::counters::CounterType::M1M1, x);
                        }
                    }
                    self.variable_blight_amount = Some(x);
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
                    } else if let Some(player) = self.state.players.get_mut(&player_id) {
                        if !player.mana_pool.try_pay(mana) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                Cost::TapCreatures { filter, count } => {
                    let f_lower = filter.to_lowercase();
                    let candidates: Vec<ObjectId> = self.state.battlefield.iter()
                        .filter(|perm| perm.controller == player_id && !perm.tapped && perm.id() != source_id && perm.is_creature())
                        .filter(|perm| {
                            if f_lower.contains("elf") {
                                self.state.card_store.get(perm.id()).is_some_and(|c| c.subtypes.iter().any(|st| st.to_string().to_lowercase() == "elf") || c.keywords.contains(crate::constants::KeywordAbilities::CHANGELING))
                            } else {
                                true
                            }
                        })
                        .map(|perm| perm.id())
                        .collect();
                    if (candidates.len() as u32) < *count {
                        return false;
                    }
                    for candidate in candidates.iter().take(*count as usize) {
                        if let Some(perm) = self.state.battlefield.get_mut(*candidate) {
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
                    let base_dmg = resolve_x(*amount);
                    let mult = source.map(|s| self.get_damage_multiplier(s)).unwrap_or(1);
                    let dmg = base_dmg * mult;
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
                                let counter_count = perm.counters.total_count();
                                if let Some(perm) = self.state.battlefield.remove(target_id) {
                                    self.move_card_to_graveyard_inner(target_id, perm.owner());
                                    if was_creature {
                                        self.emit_event(GameEvent::dies(target_id, perm_controller, counter_count));
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
                Effect::BounceAll { filter } => {
                    let to_bounce: Vec<(ObjectId, PlayerId)> = self.state.battlefield.iter()
                        .filter(|p| Self::matches_filter(p, filter))
                        .map(|p| (p.id(), p.owner()))
                        .collect();
                    for (id, owner) in &to_bounce {
                        if let Some(_perm) = self.state.battlefield.remove(*id) {
                            self.state.ability_store.remove_source(*id);
                            if let Some(player) = self.state.players.get_mut(owner) {
                                player.hand.add(*id);
                            }
                            self.state.set_zone(*id, crate::constants::Zone::Hand, Some(*owner));
                        }
                    }
                }
                Effect::ExileFromOpponentLibrary { count } => {
                    let exile_count = resolve_x(*count);
                    let opponents: Vec<PlayerId> = self.state.turn_order.iter()
                        .filter(|&&id| id != controller)
                        .copied()
                        .collect();
                    let mut exiled: Vec<(ObjectId, PlayerId)> = Vec::new();
                    for opp_id in opponents {
                        if let Some(player) = self.state.players.get_mut(&opp_id) {
                            for _ in 0..exile_count {
                                if let Some(card_id) = player.library.draw() {
                                    exiled.push((card_id, opp_id));
                                }
                            }
                        }
                    }
                    for (card_id, opp_id) in exiled {
                        self.state.exile.exile(card_id);
                        self.state.set_zone(card_id, crate::constants::Zone::Exile, Some(opp_id));
                    }
                }
                Effect::ExileFromOpponentLibraryToSourceZone { count } => {
                    let exile_count = resolve_x(*count);
                    let src_id = source.unwrap_or_default();
                    let source_name = self.state.battlefield.get(src_id)
                        .map(|p| p.name().to_string())
                        .unwrap_or_else(|| "Unknown".to_string());
                    let opponents: Vec<PlayerId> = self.state.turn_order.iter()
                        .filter(|&&id| id != controller)
                        .copied()
                        .collect();
                    let mut exiled: Vec<ObjectId> = Vec::new();
                    for opp_id in opponents {
                        if let Some(player) = self.state.players.get_mut(&opp_id) {
                            for _ in 0..exile_count {
                                if let Some(card_id) = player.library.draw() {
                                    exiled.push(card_id);
                                }
                            }
                        }
                    }
                    let zone_name = format!("Exiled with {source_name}");
                    for card_id in exiled {
                        self.state.exile.exile_to_zone(card_id, src_id, &zone_name);
                        self.state.set_zone(card_id, crate::constants::Zone::Exile, None);
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
                    let base_dmg = resolve_x(*amount);
                    let mult = source.map(|s| self.get_damage_multiplier(s)).unwrap_or(1);
                    let dmg = base_dmg * mult;
                    let opponents: Vec<PlayerId> = self.state.turn_order.iter()
                        .filter(|&&id| id != controller)
                        .copied()
                        .collect();
                    for opp in opponents {
                        if let Some(player) = self.state.players.get_mut(&opp) {
                            player.life -= dmg as i32;
                        }
                    }
                }
                Effect::DealDamageOpponentsCreatures { amount } => {
                    let base_dmg = resolve_x(*amount);
                    let mult = source.map(|s| self.get_damage_multiplier(s)).unwrap_or(1);
                    let dmg = base_dmg * mult;
                    let matching: Vec<ObjectId> = self.state.battlefield.iter()
                        .filter(|p| p.is_creature() && p.controller != controller)
                        .map(|p| p.id())
                        .collect();
                    for id in matching {
                        if let Some(perm) = self.state.battlefield.get_mut(id) {
                            perm.apply_damage(dmg);
                        }
                    }
                }
                Effect::AddCounters { counter_type, count: raw_count } => {
                    let count = resolve_x(*raw_count);
                    let ct = crate::counters::CounterType::from_name(counter_type);
                    let target_id = if targets.is_empty() {
                        source
                    } else {
                        Some(targets[0])
                    };
                    if let Some(tid) = target_id {
                        if let Some(perm) = self.state.battlefield.get_mut(tid) {
                            perm.add_counters(ct, count);
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
                Effect::BoostByToughnessMinusPower => {
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            let p = perm.power();
                            let t = perm.toughness();
                            let diff = (t - p).max(0);
                            if diff > 0 {
                                perm.add_counters(CounterType::P1P1, diff as u32);
                            }
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
                            // Countered abilities just vanish (only spells go to graveyard)
                            if let crate::zones::StackItemKind::Spell { .. } = &stack_item.kind {
                                self.move_card_to_graveyard_inner(stack_item.id, stack_item.controller);
                            }
                        }
                    }
                }
                Effect::CounterAllOpponentSpellsAndAbilities { token_name } => {
                    let opponent_ids: Vec<PlayerId> = self.state.turn_order.iter()
                        .filter(|&&pid| pid != controller)
                        .copied().collect();
                    let opp_stack_ids: Vec<ObjectId> = self.state.stack.iter()
                        .filter(|item| opponent_ids.contains(&item.controller))
                        .map(|item| item.id)
                        .collect();
                    let mut countered_count = 0u32;
                    for stack_id in opp_stack_ids {
                        let cant_counter = if let Some(item) = self.state.stack.get(stack_id) {
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
                        let cant_counter_perm = if let Some(item) = self.state.stack.get(stack_id) {
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
                        if cant_counter || cant_counter_perm {
                            continue;
                        }
                        if let Some(stack_item) = self.state.stack.remove(stack_id) {
                            if let crate::zones::StackItemKind::Spell { .. } = &stack_item.kind {
                                self.move_card_to_graveyard_inner(stack_item.id, stack_item.controller);
                            }
                            countered_count += 1;
                        }
                    }
                    if countered_count > 0 {
                        for _ in 0..countered_count {
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
                Effect::MillAndSelect { count, filter, destination } => {
                    let mut milled = Vec::new();
                    for _ in 0..resolve_x(*count) {
                        let card_id = self.state.players.get_mut(&controller)
                            .and_then(|p| p.library.draw());
                        if let Some(card_id) = card_id {
                            self.move_card_to_graveyard_inner(card_id, controller);
                            milled.push(card_id);
                        }
                    }
                    let picked = milled.iter().find(|&&card_id| {
                        self.state.card_store.get(card_id)
                            .map(|c| Self::card_matches_filter(c, filter))
                            .unwrap_or(false)
                    }).copied();
                    if let Some(card_id) = picked {
                        if let Some(player) = self.state.players.get_mut(&controller) {
                            player.graveyard.remove(card_id);
                        }
                        if destination == "hand" {
                            if let Some(player) = self.state.players.get_mut(&controller) {
                                player.hand.add(card_id);
                            }
                            self.state.set_zone(card_id, crate::constants::Zone::Hand, Some(controller));
                        } else {
                            if let Some(player) = self.state.players.get_mut(&controller) {
                                player.library.put_on_top(card_id);
                            }
                            self.state.set_zone(card_id, crate::constants::Zone::Library, Some(controller));
                        }
                    }
                }
                Effect::MillAndReturnAll { count, filter } => {
                    let mut milled = Vec::new();
                    for _ in 0..resolve_x(*count) {
                        let card_id = self.state.players.get_mut(&controller)
                            .and_then(|p| p.library.draw());
                        if let Some(card_id) = card_id {
                            self.move_card_to_graveyard_inner(card_id, controller);
                            milled.push(card_id);
                        }
                    }
                    let matched: Vec<ObjectId> = milled.iter().filter(|&&card_id| {
                        self.state.card_store.get(card_id)
                            .map(|c| Self::card_matches_filter(c, filter))
                            .unwrap_or(false)
                    }).copied().collect();
                    for card_id in matched {
                        if let Some(player) = self.state.players.get_mut(&controller) {
                            player.graveyard.remove(card_id);
                        }
                        if let Some(player) = self.state.players.get_mut(&controller) {
                            player.hand.add(card_id);
                        }
                        self.state.set_zone(card_id, crate::constants::Zone::Hand, Some(controller));
                    }
                }
                Effect::CreateToken { token_name, count } => {
                    let actual_count = resolve_x(*count);
                    if !self.try_replace_token_creation(controller, actual_count) {
                        self.mark_tokens_created(controller);
                        for _ in 0..actual_count {
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
                    for &target_id in targets {
                        let owner = self.state.find_card_owner_in_graveyard(target_id);
                        if let Some(owner_id) = owner {
                            if let Some(player) = self.state.players.get_mut(&owner_id) {
                                player.graveyard.remove(target_id);
                            }
                            if let Some(card_data) = self.state.card_store.remove(target_id) {
                                for ability in &card_data.abilities {
                                    self.state.ability_store.add(ability.clone());
                                }
                                let perm = Permanent::new(card_data, controller);
                                self.state.battlefield.add(perm);
                                self.state.set_zone(target_id, crate::constants::Zone::Battlefield, None);
                                self.check_enters_tapped(target_id);
                                self.check_enters_with_counters(target_id);
                                self.emit_event(GameEvent::enters_battlefield_from(target_id, controller, crate::constants::Zone::Graveyard));
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
                    if let Some(kw) = crate::constants::KeywordAbilities::keyword_from_name(keyword) {
                        let effective_targets: Vec<ObjectId> = if targets.is_empty() {
                            source.into_iter().collect()
                        } else {
                            targets.to_vec()
                        };
                        for target_id in effective_targets {
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
                            let ctr_count = self.state.battlefield.get(victim_id)
                                .map(|p| p.counters.total_count()).unwrap_or(0);
                            if let Some(perm) = self.state.battlefield.remove(victim_id) {
                                self.move_card_to_graveyard_inner(victim_id, perm.owner());
                                if was_creature {
                                    self.emit_event(GameEvent::dies(victim_id, opp, ctr_count));
                                }
                                self.state.ability_store.remove_source(victim_id);
                            }
                        }
                    }
                }
                Effect::DestroyAll { filter } => {
                    let to_destroy: Vec<(ObjectId, PlayerId, bool, u32)> = self.state.battlefield.iter()
                        .filter(|p| Self::matches_filter(p, filter) && !p.has_indestructible())
                        .map(|p| (p.id(), p.owner(), p.is_creature(), p.counters.total_count()))
                        .collect();
                    for (id, owner, was_creature, ctr_count) in &to_destroy {
                        if let Some(perm) = self.state.battlefield.remove(*id) {
                            self.move_card_to_graveyard_inner(*id, *owner);
                            if *was_creature {
                                self.emit_event(GameEvent::dies(*id, perm.controller, *ctr_count));
                            }
                        }
                    }
                    for (id, _, _, _) in &to_destroy {
                        self.state.ability_store.remove_source(*id);
                    }
                }
                Effect::DealDamageAll { amount, filter } => {
                    let base_dmg = resolve_x(*amount);
                    let mult = source.map(|s| self.get_damage_multiplier(s)).unwrap_or(1);
                    let dmg = base_dmg * mult;
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
                    self.mark_tokens_created(controller);
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
                        perm.summoning_sick = false;
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
                    let effective_targets: Vec<ObjectId> = if targets.is_empty() {
                        source.into_iter().collect()
                    } else {
                        targets.to_vec()
                    };
                    for target_id in effective_targets {
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            perm.card.power = Some(*power);
                            perm.card.toughness = Some(*toughness);
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
                Effect::LoseAllAbilities => {
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            perm.removed_keywords = crate::constants::KeywordAbilities::all();
                            perm.abilities_lost = true;
                        }
                        self.state.ability_store.remove_source(target_id);
                    }
                }
                Effect::LoseAllAbilitiesAll { filter } => {
                    let matching: Vec<ObjectId> = self.state.battlefield.iter()
                        .filter(|p| p.is_creature()
                            && (filter.to_lowercase().contains("opponent") && p.controller != controller
                                || !filter.to_lowercase().contains("opponent") && Self::matches_filter(p, filter)))
                        .map(|p| p.id())
                        .collect();
                    for id in matching {
                        if let Some(perm) = self.state.battlefield.get_mut(id) {
                            perm.removed_keywords = crate::constants::KeywordAbilities::all();
                            perm.abilities_lost = true;
                        }
                        self.state.ability_store.remove_source(id);
                    }
                }
                Effect::AddSubtypeAll { subtype, filter } => {
                    let st = crate::constants::SubType::by_description(subtype);
                    let matching: Vec<ObjectId> = self.state.battlefield.iter()
                        .filter(|p| p.is_creature()
                            && (filter.to_lowercase().contains("opponent") && p.controller != controller
                                || !filter.to_lowercase().contains("opponent") && Self::matches_filter(p, filter)))
                        .map(|p| p.id())
                        .collect();
                    for id in matching {
                        if let Some(perm) = self.state.battlefield.get_mut(id) {
                            if !perm.card.subtypes.contains(&st) {
                                perm.card.subtypes.push(st.clone());
                            }
                        }
                    }
                }
                Effect::SetSubtypesSelf { subtypes } => {
                    if let Some(src) = source {
                        if let Some(perm) = self.state.battlefield.get_mut(src) {
                            perm.card.subtypes = subtypes.iter()
                                .map(|s| crate::constants::SubType::by_description(s))
                                .collect();
                        }
                    }
                }
                Effect::SetBasePowerToughnessAll { power, toughness, filter } => {
                    let matching: Vec<ObjectId> = self.state.battlefield.iter()
                        .filter(|p| p.is_creature()
                            && (filter.to_lowercase().contains("opponent") && p.controller != controller
                                || !filter.to_lowercase().contains("opponent") && Self::matches_filter(p, filter)))
                        .map(|p| p.id())
                        .collect();
                    for id in matching {
                        if let Some(perm) = self.state.battlefield.get_mut(id) {
                            perm.card.power = Some(*power);
                            perm.card.toughness = Some(*toughness);
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
                            let fighter_mult = self.get_damage_multiplier(fid);
                            let target_mult = self.get_damage_multiplier(tid);
                            if let Some(target_perm) = self.state.battlefield.get_mut(tid) {
                                target_perm.apply_damage(fighter_power * fighter_mult);
                            }
                            if let Some(fighter_perm) = self.state.battlefield.get_mut(fid) {
                                fighter_perm.apply_damage(target_power * target_mult);
                            }
                        }
                    }
                }
                Effect::Bite => {
                    let (biter_id, target_id) = Self::resolve_fight_pair(
                        &self.state, targets, source, controller,
                    );

                    if let (Some(bid), Some(tid)) = (biter_id, target_id) {
                        if bid != tid {
                            let biter_power = self.state.battlefield.get(bid)
                                .map(|p| p.power().max(0) as u32).unwrap_or(0);
                            let biter_mult = self.get_damage_multiplier(bid);
                            if let Some(target_perm) = self.state.battlefield.get_mut(tid) {
                                target_perm.apply_damage(biter_power * biter_mult);
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
                        let mult = source.map(|s| self.get_damage_multiplier(s)).unwrap_or(1);
                        let dmg = x * mult;
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
                    if !self.try_replace_token_creation(controller, x) {
                        self.mark_tokens_created(controller);
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
                }
                Effect::SearchLibraryVivid => {
                    let x = self.count_colors_among_permanents(controller);
                    let found = if x > 0 {
                        if let Some(player) = self.state.players.get(&controller) {
                            let lib_cards: Vec<ObjectId> = player.library.iter().copied().collect();
                            let mut result: Vec<ObjectId> = Vec::new();
                            for &card_id in &lib_cards {
                                if result.len() >= x {
                                    break;
                                }
                                if let Some(c) = self.state.card_store.get(card_id) {
                                    if Self::card_matches_filter(c, "basic land") {
                                        result.push(card_id);
                                    }
                                }
                            }
                            result
                        } else { vec![] }
                    } else { vec![] };
                    if let Some(player) = self.state.players.get_mut(&controller) {
                        for card_id in &found {
                            player.library.remove(*card_id);
                            player.hand.add(*card_id);
                        }
                    }
                    for card_id in found {
                        self.state.set_zone(card_id, crate::constants::Zone::Hand, Some(controller));
                    }
                }
                Effect::RevealFromLibraryVivid => {
                    let x = self.count_colors_among_permanents(controller);
                    if x > 0 {
                        let (permanents, rest) = if let Some(player) = self.state.players.get(&controller) {
                            let lib_cards: Vec<ObjectId> = player.library.iter().copied().collect();
                            let mut permanents: Vec<ObjectId> = Vec::new();
                            let mut rest: Vec<ObjectId> = Vec::new();
                            let mut found_count = 0usize;
                            for &card_id in &lib_cards {
                                if found_count >= x {
                                    break;
                                }
                                if let Some(c) = self.state.card_store.get(card_id) {
                                    if Self::card_matches_filter(c, "permanent") {
                                        permanents.push(card_id);
                                        found_count += 1;
                                    } else {
                                        rest.push(card_id);
                                    }
                                }
                            }
                            (permanents, rest)
                        } else {
                            (vec![], vec![])
                        };
                        for &card_id in &permanents {
                            let card = self.state.card_store.get(card_id).cloned();
                            if let Some(card) = card {
                                if let Some(player) = self.state.players.get_mut(&controller) {
                                    player.library.remove(card_id);
                                }
                                let perm = crate::permanent::Permanent::new(card, controller);
                                self.state.battlefield.add(perm);
                                self.state.set_zone(card_id, crate::constants::Zone::Battlefield, None);
                                self.emit_event(GameEvent::enters_battlefield(card_id, controller));
                            }
                        }
                        if let Some(player) = self.state.players.get_mut(&controller) {
                            for &card_id in &rest {
                                player.library.remove(card_id);
                            }
                        }
                        use rand::seq::SliceRandom;
                        let mut rng = rand::thread_rng();
                        let mut rest = rest;
                        rest.shuffle(&mut rng);
                        if let Some(player) = self.state.players.get_mut(&controller) {
                            for card_id in rest {
                                player.library.put_on_bottom(card_id);
                            }
                        }
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
                    let source_id = source.unwrap_or_default();
                    if wants_to_pay && self.pay_costs(controller, source_id, &[cost.clone()]) {
                        self.execute_effects(if_paid, controller, targets, source, None);
                    } else {
                        self.execute_effects(if_not_paid, controller, targets, source, None);
                    }
                }
                Effect::Conditional { condition, if_true, if_false } => {
                    let source_id = source.unwrap_or_default();
                    if self.evaluate_condition_with_targets(source_id, controller, condition, targets) {
                        self.execute_effects(if_true, controller, targets, source, None);
                    } else {
                        self.execute_effects(if_false, controller, targets, source, None);
                    }
                }
                Effect::IfAbilityResolvedNTimes { resolution_number, effects: sub_effects } => {
                    if let Some(ab_id) = self.resolving_ability_id {
                        let count = self.state.ability_resolution_counts_this_turn
                            .get(&ab_id).copied().unwrap_or(0);
                        if count >= *resolution_number {
                            self.execute_effects(sub_effects, controller, targets, source, None);
                        }
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
                        let subtype = crate::constants::SubType::Custom(chosen.description.clone());
                        if let Some(source_id) = source {
                            if let Some(perm) = self.state.battlefield.get_mut(source_id) {
                                perm.chosen_type = Some(subtype);
                            }
                        }
                    }
                }
                Effect::ChooseColor => {
                    let color_names = ["White", "Blue", "Black", "Red", "Green"];
                    let options: Vec<crate::decision::NamedChoice> = color_names.iter().enumerate()
                        .map(|(i, s)| crate::decision::NamedChoice { index: i, description: s.to_string() })
                        .collect();
                    let view = crate::decision::GameView::placeholder();
                    let choice_idx = if let Some(dm) = self.decision_makers.get_mut(&controller) {
                        dm.choose_option(&view, crate::constants::Outcome::Benefit, "Choose a color", &options)
                    } else {
                        0
                    };
                    let color = match choice_idx {
                        0 => crate::constants::ManaColor::White,
                        1 => crate::constants::ManaColor::Blue,
                        2 => crate::constants::ManaColor::Black,
                        3 => crate::constants::ManaColor::Red,
                        _ => crate::constants::ManaColor::Green,
                    };
                    if let Some(source_id) = source {
                        if let Some(perm) = self.state.battlefield.get_mut(source_id) {
                            perm.chosen_color = Some(color);
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
                                        other => format!("{other:?}") == *type_name,
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
                Effect::ChooseTypeAndReturnFromGraveyard => {
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
                        let target_subtype = crate::constants::SubType::by_description(type_name);
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
                                    self.check_enters_with_counters(card_id);
                                    self.emit_event(GameEvent::enters_battlefield_from(card_id, controller, crate::constants::Zone::Graveyard));
                                }
                            }
                        }
                    }
                }
                Effect::ChooseTypeAndGrantKeywords { keywords, other_only } => {
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
                        let target_subtype = crate::constants::SubType::by_description(&chosen.description);
                        let mut combined_kw = crate::constants::KeywordAbilities::empty();
                        for kw_name in keywords {
                            if let Some(kw) = crate::constants::KeywordAbilities::keyword_from_name(kw_name) {
                                combined_kw |= kw;
                            }
                        }
                        let matching: Vec<ObjectId> = self.state.battlefield.iter()
                            .filter(|p| {
                                if *other_only && source.is_some_and(|s| p.id() == s) {
                                    return false;
                                }
                                if p.controller != controller {
                                    return false;
                                }
                                p.card.subtypes.contains(&target_subtype)
                                    || (p.is_creature() && p.has_keyword(crate::constants::KeywordAbilities::CHANGELING))
                            })
                            .map(|p| p.id())
                            .collect();
                        for id in matching {
                            if let Some(perm) = self.state.battlefield.get_mut(id) {
                                perm.granted_keywords |= combined_kw;
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
                        controller_filter: None,
                        copy_spell: false,
                        stored_value: None,
                    });
                }
                Effect::GrantTriggeredAbilityUntilEOT { event_type, filter, trigger_effects } => {
                    let evt = crate::events::EventType::from_name(event_type);
                    self.state.delayed_triggers.push(crate::state::DelayedTrigger {
                        event_type: evt,
                        watching: None,
                        effects: trigger_effects.clone(),
                        controller,
                        source,
                        targets: vec![],
                        duration: crate::state::DelayedDuration::EndOfTurn,
                        trigger_only_once: false,
                        created_turn: self.state.turn_number,
                        controller_filter: Some(filter.clone()),
                        copy_spell: false,
                        stored_value: None,
                    });
                }
                Effect::CopyNextSpell => {
                    self.state.delayed_triggers.push(crate::state::DelayedTrigger {
                        event_type: EventType::SpellCast,
                        watching: None,
                        effects: vec![],
                        controller,
                        source,
                        targets: vec![],
                        duration: crate::state::DelayedDuration::EndOfTurn,
                        trigger_only_once: true,
                        created_turn: self.state.turn_number,
                        controller_filter: None,
                        copy_spell: true,
                        stored_value: None,
                    });
                }
                Effect::CopyTriggeringSpell { keywords, single_target_only } => {
                    let spell_id = self.find_triggering_spell(controller);
                    if let Some(sid) = spell_id {
                        let should_copy = if *single_target_only {
                            self.state.stack.get(sid)
                                .map(|item| item.targets.len() == 1)
                                .unwrap_or(false)
                        } else {
                            true
                        };
                        if should_copy {
                            if !keywords.is_empty() {
                                self.grant_keywords_to_spell(sid, keywords);
                            }
                            self.copy_spell_on_stack(sid, controller);
                            if !keywords.is_empty() {
                                if let Some(top) = self.state.stack.top() {
                                    let copy_id = top.id;
                                    self.grant_keywords_to_spell(copy_id, keywords);
                                }
                            }
                        }
                    }
                }
                Effect::OpponentRevealsFromHandExileCast { count_source, instant_sorcery_only } => {
                    let x = self.evaluate_count_filter(count_source, controller);
                    let opp_opt = self.state.turn_order.iter()
                        .find(|&&id| id != controller)
                        .copied();
                    if let Some(opp) = opp_opt {
                        let hand: Vec<ObjectId> = self.state.players.get(&opp)
                            .map(|p| p.hand.iter().copied().collect())
                            .unwrap_or_default();
                        let reveal_count = (x as usize).min(hand.len());
                        if reveal_count > 0 {
                            let revealed = if hand.len() <= reveal_count {
                                hand.clone()
                            } else {
                                let view = crate::decision::GameView::placeholder();
                                if let Some(dm) = self.decision_makers.get_mut(&opp) {
                                    dm.choose_discard(&view, &hand, reveal_count)
                                } else {
                                    hand.iter().rev().take(reveal_count).copied().collect()
                                }
                            };
                            let chosen = if revealed.len() == 1 {
                                revealed[0]
                            } else {
                                let view = crate::decision::GameView::placeholder();
                                if let Some(dm) = self.decision_makers.get_mut(&controller) {
                                    let req = crate::decision::TargetRequirement {
                                        description: "Choose a revealed card to exile".to_string(),
                                        legal_targets: revealed.clone(),
                                        min_targets: 1,
                                        max_targets: 1,
                                        required: true,
                                    };
                                    let picks = dm.choose_targets(&view, crate::constants::Outcome::Benefit, &req);
                                    picks.into_iter().next().unwrap_or(revealed[0])
                                } else {
                                    revealed[0]
                                }
                            };
                            if let Some(player) = self.state.players.get_mut(&opp) {
                                player.hand.remove(chosen);
                            }
                            self.state.exile.exile(chosen);
                            self.state.set_zone(chosen, crate::constants::Zone::Exile, Some(opp));
                            let is_instant_or_sorcery = self.state.card_store.get(chosen)
                                .map(|c| c.is_instant() || c.is_sorcery())
                                .unwrap_or(false);
                            let should_make_playable = if *instant_sorcery_only {
                                is_instant_or_sorcery
                            } else {
                                true
                            };
                            if should_make_playable {
                                if let Some(&src_id) = source.as_ref() {
                                    self.state.impulse_playable.push(crate::state::ImpulsePlayable {
                                        card_id: chosen,
                                        player_id: controller,
                                        duration: crate::state::ImpulseDuration::WhileSourceControlled {
                                            source_id: src_id,
                                            controller,
                                        },
                                        created_turn: self.state.turn_number,
                                        without_mana: false,
                                    });
                                }
                            }
                        }
                    }
                }
                Effect::OpponentsExileUntilMVAndCast { mv_threshold } => {
                    let threshold = resolve_x(*mv_threshold);
                    let opponents: Vec<PlayerId> = self.state.turn_order.iter()
                        .filter(|&&id| id != controller)
                        .copied()
                        .collect();
                    let mut all_exiled = Vec::new();
                    for opp_id in opponents {
                        let mut total_mv: u32 = 0;
                        loop {
                            let card_id = self.state.players.get_mut(&opp_id)
                                .and_then(|p| p.library.draw());
                            if let Some(id) = card_id {
                                let mv = self.state.card_store.get(id)
                                    .map(|c| c.mana_value())
                                    .unwrap_or(0);
                                self.state.exile.exile(id);
                                self.state.set_zone(id, crate::constants::Zone::Exile, Some(opp_id));
                                all_exiled.push(id);
                                total_mv += mv;
                                if total_mv >= threshold {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                    let turn = self.state.turn_number;
                    for id in all_exiled {
                        self.state.impulse_playable.push(crate::state::ImpulsePlayable {
                            card_id: id,
                            player_id: controller,
                            duration: crate::state::ImpulseDuration::EndOfTurn,
                            created_turn: turn,
                            without_mana: true,
                        });
                    }
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
                    if let Some(_src) = source {
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
                    let src_id = source.unwrap_or_default();
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
                Effect::ExileTargetToSourceZone => {
                    let source_id = source.unwrap_or_default();
                    let source_name = self.state.battlefield.get(source_id)
                        .map(|p| p.name().to_string())
                        .unwrap_or_else(|| "Unknown".to_string());
                    for &target_id in targets {
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
                            }
                            let zone_name = format!("Exiled with {source_name}");
                            self.state.exile.exile_to_zone(target_id, source_id, &zone_name);
                            self.state.set_zone(target_id, crate::constants::Zone::Exile, None);
                        } else if self.state.battlefield.remove(target_id).is_some() {
                            self.state.ability_store.remove_source(target_id);
                            let zone_name = format!("Exiled with {source_name}");
                            self.state.exile.exile_to_zone(target_id, source_id, &zone_name);
                            self.state.set_zone(target_id, crate::constants::Zone::Exile, None);
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
                                self.check_enters_with_counters(target_id);
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
                            controller_filter: None,
                            copy_spell: false,
                            stored_value: None,
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
                                self.check_enters_with_counters(target_id);
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
                                        description: format!("Blight {count} (put -1/-1 counters on creature you control)"),
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
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            perm.granted_keywords |= crate::constants::KeywordAbilities::CHANGELING;
                        }
                    }
                }
                Effect::BecomeAllColors => {
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            perm.all_colors_until_eot = true;
                        }
                    }
                }
                Effect::BecomesCreature { power, toughness } => {
                    if let Some(source_id) = source {
                        if let Some(perm) = self.state.battlefield.get_mut(source_id) {
                            if !perm.has_card_type(crate::constants::CardType::Creature) {
                                perm.added_card_types.push(crate::constants::CardType::Creature);
                            }
                            perm.base_power_eot = Some(*power);
                            perm.base_toughness_eot = Some(*toughness);
                        }
                    }
                }
                Effect::TransformSelf => {
                    if let Some(source_id) = source {
                        let did_transform = if let Some(perm) = self.state.battlefield.get_mut(source_id) {
                            if perm.can_transform() {
                                let old_name = perm.name().to_string();
                                if perm.transform() {
                                    let new_name = perm.name().to_string();
                                    self.state.ability_store.remove_source(source_id);
                                    let abilities = perm.card.abilities.clone();
                                    Some((old_name, new_name, abilities))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                        if let Some((_old_name, _new_name, abilities)) = did_transform {
                            for ability in &abilities {
                                let mut ab = ability.clone();
                                ab.source_id = source_id;
                                ab.id = crate::types::AbilityId::new();
                                self.state.ability_store.add(ab);
                            }
                            self.emit_event(
                                crate::events::GameEvent::new(crate::events::EventType::Transformed)
                                    .target(source_id)
                                    .player(controller),
                            );
                        }
                    }
                }
                Effect::CreateTokenCopy { count, modifications } => {
                    self.mark_tokens_created(controller);
                    let count = resolve_x(*count);
                    for &target_id in targets {
                        // Get the source permanent's card data to copy
                        let source_card = self.state.battlefield.get(target_id).map(|perm| perm.card.clone());
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
                                        controller_filter: None,
                                        copy_spell: false,
                                        stored_value: None,
                                    });
                                }
                            }
                        }
                    }
                }
                Effect::CreateTokenCopyOfTriggering => {
                    self.mark_tokens_created(controller);
                    for &target_id in targets {
                        let source_card = self.state.battlefield.get(target_id).map(|perm| perm.card.clone());
                        if let Some(src) = source_card {
                            let token_id = ObjectId::new();
                            let mut token_card = src.clone();
                            token_card.id = token_id;
                            token_card.owner = controller;
                            token_card.is_token = true;
                            token_card.abilities = token_card.abilities.iter().map(|ab| {
                                let mut new_ab = ab.clone();
                                new_ab.id = crate::types::AbilityId::new();
                                new_ab.source_id = token_id;
                                new_ab
                            }).collect();
                            for ab in &token_card.abilities {
                                self.state.ability_store.add(ab.clone());
                            }
                            let perm = Permanent::new(token_card, controller);
                            self.state.battlefield.add(perm);
                            self.state.set_zone(token_id, crate::constants::Zone::Battlefield, None);
                            self.emit_event(GameEvent::enters_battlefield(token_id, controller));
                        }
                    }
                }
                Effect::TapSelf => {
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
                                self.check_enters_with_counters(card_id);
                                self.emit_event(GameEvent::enters_battlefield_from(card_id, controller, crate::constants::Zone::Graveyard));
                            }
                        }
                    }
                }
                Effect::CreateTokenDynamic { token_name, count_filter } => {
                    let count = self.evaluate_count_filter(count_filter, controller);
                    if !self.try_replace_token_creation(controller, count) {
                        self.mark_tokens_created(controller);
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
                }
                Effect::GainLifeDynamic { value_source } => {
                    let amount = self.evaluate_count_filter(value_source, controller);
                    if amount > 0 {
                        if let Some(player) = self.state.players.get_mut(&controller) {
                            player.life += amount as i32;
                        }
                        self.emit_event(GameEvent::gain_life(controller, amount));
                    }
                }
                Effect::BoostTargetDynamic { value_source } => {
                    let amount = self.evaluate_count_filter(value_source, controller) as i32;
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            perm.continuous_boost_power += amount;
                            perm.continuous_boost_toughness += amount;
                        }
                    }
                }
                Effect::BoostDualTargetDynamic { value_source } => {
                    let amount = self.evaluate_count_filter(value_source, controller) as i32;
                    if !targets.is_empty() {
                        if let Some(perm) = self.state.battlefield.get_mut(targets[0]) {
                            perm.continuous_boost_power += amount;
                        }
                    }
                    if targets.len() >= 2 {
                        if let Some(perm) = self.state.battlefield.get_mut(targets[1]) {
                            perm.continuous_boost_toughness -= amount;
                        }
                    }
                }
                Effect::CompareAndBoost => {
                    if targets.len() >= 2 {
                        let power_a = self.state.battlefield.get(targets[0]).map(|p| p.power()).unwrap_or(0);
                        let power_b = self.state.battlefield.get(targets[1]).map(|p| p.power()).unwrap_or(0);
                        let x = (power_a - power_b).unsigned_abs();
                        if x > 0 {
                            self.draw_cards(controller, x);
                            for &tid in &targets[..2] {
                                if let Some(perm) = self.state.battlefield.get_mut(tid) {
                                    perm.add_counters(CounterType::P1P1, x);
                                }
                            }
                        }
                        for &tid in &targets[..2] {
                            if let Some(perm) = self.state.battlefield.get_mut(tid) {
                                perm.granted_keywords |= crate::constants::KeywordAbilities::TRAMPLE;
                            }
                        }
                    }
                }
                Effect::TargetControllerDraws { count } => {
                    // Target's controller draws cards (not the ability's controller)
                    for &target_id in targets {
                        let target_controller = self.state.battlefield.get(target_id)
                            .map(|p| p.controller);
                        if let Some(tc) = target_controller {
                            self.draw_cards(tc, *count);
                        }
                    }
                }
                Effect::TargetControllerCreatesToken { token_name } => {
                    // Target's controller creates a token
                    for &target_id in targets {
                        let target_controller = self.state.battlefield.get(target_id)
                            .map(|p| p.controller);
                        if let Some(tc) = target_controller {
                            let token_id = ObjectId::new();
                            let mut card = CardData::new(token_id, tc, token_name);
                            card.card_types = vec![crate::constants::CardType::Creature];
                            let (p, t, kw) = Self::parse_token_stats(token_name);
                            card.power = Some(p);
                            card.toughness = Some(t);
                            card.keywords = kw;
                            card.is_token = true;
                            let perm = Permanent::new(card, tc);
                            self.state.battlefield.add(perm);
                            self.state.set_zone(token_id, crate::constants::Zone::Battlefield, None);
                            self.emit_event(GameEvent::enters_battlefield(token_id, tc));
                        }
                    }
                }
                Effect::PutFromHandToBattlefield { max_mana_value, max_mv_dynamic, tapped, attacking, haste, sacrifice_eot } => {
                    let max_mv = if let Some(dynamic_source) = max_mv_dynamic {
                        self.evaluate_count_filter(dynamic_source, controller)
                    } else {
                        resolve_x(*max_mana_value)
                    };
                    let hand: Vec<ObjectId> = self.state.players.get(&controller)
                        .map(|p| p.hand.iter().copied().collect())
                        .unwrap_or_default();
                    let eligible: Vec<ObjectId> = hand.iter().copied()
                        .filter(|&cid| {
                            self.state.card_store.get(cid).is_some_and(|card| {
                                card.card_types.contains(&crate::constants::CardType::Creature)
                                    && card.mana_value() <= max_mv
                            })
                        })
                        .collect();
                    if !eligible.is_empty() {
                        let view = crate::decision::GameView::placeholder();
                        let wants = if let Some(dm) = self.decision_makers.get_mut(&controller) {
                            dm.choose_use(&view, crate::constants::Outcome::Benefit,
                                "Put a creature card from your hand onto the battlefield?")
                        } else {
                            true
                        };
                        if wants {
                            let chosen = if let Some(dm) = self.decision_makers.get_mut(&controller) {
                                let picked = dm.choose_discard(&view, &eligible, 1);
                                picked.into_iter().next()
                            } else {
                                eligible.first().copied()
                            };
                            if let Some(card_id) = chosen {
                                if let Some(player) = self.state.players.get_mut(&controller) {
                                    player.hand.remove(card_id);
                                }
                                if let Some(card_data) = self.state.card_store.remove(card_id) {
                                    for ability in &card_data.abilities {
                                        self.state.ability_store.add(ability.clone());
                                    }
                                    let mut perm = Permanent::new(card_data, controller);
                                    if *tapped {
                                        perm.tapped = true;
                                    }
                                    if *attacking {
                                        perm.summoning_sick = false;
                                    }
                                    if *haste {
                                        perm.granted_keywords |= crate::constants::KeywordAbilities::HASTE;
                                    }
                                    self.state.battlefield.add(perm);
                                    self.state.set_zone(card_id, crate::constants::Zone::Battlefield, None);
                                    self.check_enters_with_counters(card_id);
                                    self.emit_event(GameEvent::enters_battlefield(card_id, controller));
                                    if *sacrifice_eot {
                                        self.state.delayed_triggers.push(crate::state::DelayedTrigger {
                                            event_type: crate::events::EventType::EndStep,
                                            watching: None,
                                            effects: vec![Effect::Sacrifice { filter: "self".into() }],
                                            controller,
                                            source: Some(card_id),
                                            targets: vec![card_id],
                                            duration: crate::state::DelayedDuration::UntilTriggered,
                                            trigger_only_once: true,
                                            created_turn: self.state.turn_number,
                                            controller_filter: None,
                                            copy_spell: false,
                                            stored_value: None,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
                Effect::DealDamageWithDelayedExile => {
                    let base_dmg = resolve_x(crate::abilities::X_VALUE);
                    let mult = source.map(|s| self.get_damage_multiplier(s)).unwrap_or(1);
                    let dmg = base_dmg * mult;
                    for &target_id in targets {
                        let creature_power = self.state.battlefield.get(target_id)
                            .map(|p| p.power())
                            .unwrap_or(0);
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            perm.apply_damage(dmg);
                        }
                        self.state.delayed_triggers.push(crate::state::DelayedTrigger {
                            event_type: EventType::Dies,
                            watching: Some(target_id),
                            effects: vec![Effect::ExileTopChooseOneAndPlay {
                                count: crate::abilities::X_VALUE,
                                duration: "until_end_of_next_turn".into(),
                            }],
                            controller,
                            source,
                            targets: vec![],
                            duration: crate::state::DelayedDuration::EndOfTurn,
                            trigger_only_once: true,
                            created_turn: self.state.turn_number,
                            controller_filter: None,
                            copy_spell: false,
                            stored_value: Some(creature_power),
                        });
                    }
                }
                Effect::ExileTopChooseOneAndPlay { count, duration } => {
                    let n = resolve_x(*count) as usize;
                    let dur = match duration.as_str() {
                        "until_end_of_next_turn" => crate::state::ImpulseDuration::UntilEndOfNextTurn,
                        _ => crate::state::ImpulseDuration::EndOfTurn,
                    };
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
                    if !exiled.is_empty() {
                        let chosen = exiled.iter().copied()
                            .max_by_key(|&id| {
                                self.state.card_store.get(id)
                                    .map(|c| c.mana_value())
                                    .unwrap_or(0)
                            })
                            .unwrap_or(exiled[0]);
                        let turn = self.state.turn_number;
                        self.state.impulse_playable.push(crate::state::ImpulsePlayable {
                            card_id: chosen,
                            player_id: controller,
                            duration: dur,
                            created_turn: turn,
                            without_mana: false,
                        });
                    }
                }
                Effect::Winnowing => {
                    let all_players: Vec<PlayerId> = self.state.turn_order.to_vec();
                    let mut chosen_per_player: Vec<(PlayerId, ObjectId)> = Vec::new();
                    for &pid in &all_players {
                        let creatures: Vec<ObjectId> = self.state.battlefield.iter()
                            .filter(|p| p.controller == pid && p.is_creature())
                            .map(|p| p.id())
                            .collect();
                        if creatures.is_empty() {
                            continue;
                        }
                        let view = crate::decision::GameView::placeholder();
                        let chosen_id = if let Some(dm) = self.decision_makers.get_mut(&controller) {
                            let targets = dm.choose_targets(&view, crate::constants::Outcome::Benefit,
                                &crate::decision::TargetRequirement {
                                    description: "Choose a creature controlled by player".to_string(),
                                    legal_targets: creatures.clone(),
                                    min_targets: 1, max_targets: 1,
                                    required: true,
                                });
                            targets.into_iter().next().unwrap_or(creatures[0])
                        } else {
                            creatures[0]
                        };
                        chosen_per_player.push((pid, chosen_id));
                    }
                    let mut to_sacrifice: Vec<(ObjectId, PlayerId, bool, u32)> = Vec::new();
                    for &(pid, chosen_id) in &chosen_per_player {
                        let chosen_subtypes: Vec<crate::constants::SubType> = self.state.battlefield.get(chosen_id)
                            .map(|p| p.card.subtypes.clone())
                            .unwrap_or_default();
                        let chosen_is_changeling = self.state.battlefield.get(chosen_id)
                            .map(|p| p.is_creature() && p.has_keyword(crate::constants::KeywordAbilities::CHANGELING))
                            .unwrap_or(false);
                        for perm in self.state.battlefield.iter() {
                            if perm.controller != pid || !perm.is_creature() || perm.id() == chosen_id {
                                continue;
                            }
                            let perm_is_changeling = perm.is_creature()
                                && perm.has_keyword(crate::constants::KeywordAbilities::CHANGELING);
                            if chosen_is_changeling || perm_is_changeling {
                                continue;
                            }
                            let shares = chosen_subtypes.iter().any(|st| perm.has_subtype(st));
                            if !shares {
                                to_sacrifice.push((perm.id(), perm.owner(), perm.is_creature(), perm.counters.total_count()));
                            }
                        }
                    }
                    for (id, owner, was_creature, ctr_count) in &to_sacrifice {
                        if let Some(perm) = self.state.battlefield.remove(*id) {
                            self.move_card_to_graveyard_inner(*id, *owner);
                            if *was_creature {
                                self.emit_event(GameEvent::dies(*id, perm.controller, *ctr_count));
                            }
                            self.state.ability_store.remove_source(*id);
                        }
                    }
                }
                Effect::MassBecomeCopy => {
                    for &target_id in targets {
                        let source_card = match self.state.battlefield.get(target_id) {
                            Some(p) => p.card.clone(),
                            None => continue,
                        };
                        let my_nonland_ids: Vec<ObjectId> = self.state.battlefield.iter()
                            .filter(|p| p.controller == controller
                                && !p.card.card_types.contains(&crate::constants::CardType::Land)
                                && p.id() != target_id)
                            .map(|p| p.id())
                            .collect();
                        for perm_id in my_nonland_ids {
                            self.state.ability_store.remove_source(perm_id);
                            if let Some(perm) = self.state.battlefield.get_mut(perm_id) {
                                let original_id = perm.card.id;
                                let original_owner = perm.card.owner;
                                let was_token = perm.card.is_token;
                                perm.card = source_card.clone();
                                perm.card.id = original_id;
                                perm.card.owner = original_owner;
                                perm.card.is_token = was_token;
                                perm.card.abilities = perm.card.abilities.iter().map(|ab| {
                                    let mut new_ab = ab.clone();
                                    new_ab.id = crate::types::AbilityId::new();
                                    new_ab.source_id = original_id;
                                    new_ab
                                }).collect();
                                perm.summoning_sick = perm.card.is_creature();
                            }
                            if let Some(perm) = self.state.battlefield.get(perm_id) {
                                for ab in &perm.card.abilities {
                                    self.state.ability_store.add(ab.clone());
                                }
                            }
                        }
                    }
                }
                Effect::ExileWithDreamCounterInsteadOfGraveyard => {
                    let spell_id = self.state.stack.iter()
                        .find(|item| {
                            if item.controller != controller {
                                return false;
                            }
                            if let crate::zones::StackItemKind::Spell { card } = &item.kind {
                                (card.is_instant() || card.is_sorcery())
                                    && !self.state.pending_dream_exile.contains(&item.id)
                            } else {
                                false
                            }
                        })
                        .map(|item| item.id);
                    if let Some(id) = spell_id {
                        self.state.pending_dream_exile.push(id);
                    }
                }
                Effect::CastFromExileWithDreamCounters => {
                    let eligible: Vec<ObjectId> = self.state.dream_countered_cards.iter()
                        .filter(|&&id| self.state.exile.contains(id))
                        .copied()
                        .collect();
                    let turn = self.state.turn_number;
                    for id in eligible {
                        if !self.state.impulse_playable.iter().any(|ip| ip.card_id == id) {
                            self.state.impulse_playable.push(crate::state::ImpulsePlayable {
                                card_id: id,
                                player_id: controller,
                                duration: crate::state::ImpulseDuration::EndOfTurn,
                                created_turn: turn,
                                without_mana: true,
                            });
                        }
                    }
                }
                _ => {
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

    fn return_from_graveyard_with_counter(&mut self, card_id: ObjectId, owner: PlayerId, counter: CounterType) {
        if let Some(player) = self.state.players.get_mut(&owner) {
            if !player.graveyard.remove(card_id) {
                return;
            }
        }
        if let Some(card_data) = self.state.card_store.remove(card_id) {
            for ability in &card_data.abilities {
                self.state.ability_store.add(ability.clone());
            }
            let mut perm = Permanent::new(card_data, owner);
            perm.add_counters(counter, 1);
            self.state.battlefield.add(perm);
            self.state.set_zone(card_id, crate::constants::Zone::Battlefield, None);
            self.emit_event(GameEvent::enters_battlefield_from(card_id, owner, crate::constants::Zone::Graveyard));
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
    fn try_replace_token_creation(&mut self, controller: PlayerId, count: u32) -> bool {
        if self.state.tokens_created_this_turn.contains(&controller) {
            return false;
        }
        let replacement = self.state.token_replacement_effects.iter()
            .find(|(_, ctrl, _)| *ctrl == controller)
            .map(|(_, _, equipped_id)| *equipped_id);
        let equipped_id = match replacement {
            Some(id) => id,
            None => return false,
        };
        let source_card = match self.state.battlefield.get(equipped_id) {
            Some(perm) => perm.card.clone(),
            None => return false,
        };
        self.state.tokens_created_this_turn.insert(controller);
        for _ in 0..count {
            let token_id = ObjectId::new();
            let mut token_card = source_card.clone();
            token_card.id = token_id;
            token_card.owner = controller;
            token_card.is_token = true;
            token_card.abilities = token_card.abilities.iter().map(|ab| {
                let mut new_ab = ab.clone();
                new_ab.id = crate::types::AbilityId::new();
                new_ab.source_id = token_id;
                new_ab
            }).collect();
            for ab in &token_card.abilities {
                self.state.ability_store.add(ab.clone());
            }
            let perm = Permanent::new(token_card, controller);
            self.state.battlefield.add(perm);
            self.state.set_zone(token_id, crate::constants::Zone::Battlefield, None);
            self.emit_event(GameEvent::enters_battlefield(token_id, controller));
        }
        true
    }

    fn mark_tokens_created(&mut self, controller: PlayerId) {
        self.state.tokens_created_this_turn.insert(controller);
    }

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
            for part in kw_str.split([',', '&']) {
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
        if f.is_empty() || f == "all" {
            return true;
        }
        let is_changeling = perm.is_creature()
            && perm.has_keyword(crate::constants::KeywordAbilities::CHANGELING);

        if f.starts_with("non-") && f.ends_with("creatures") {
            let type_part = &f[4..f.len() - 1]; // "non-elemental creatures" -> "elemental creature"
            let type_str = type_part.trim_end_matches(" creature").trim_end_matches('s');
            if !perm.is_creature() {
                return false;
            }
            if is_changeling {
                return false;
            }
            for st in &perm.card.subtypes {
                if st.to_string().to_lowercase() == type_str {
                    return false;
                }
            }
            return true;
        }

        for st in &perm.card.subtypes {
            if f.contains(&st.to_string().to_lowercase()) {
                return true;
            }
        }
        for ct in &perm.card.card_types {
            let ct_name = format!("{ct:?}").to_lowercase();
            if f.contains(&ct_name) {
                return true;
            }
        }
        if is_changeling {
            let is_card_type = f.contains("creature") || f.contains("land")
                || f.contains("artifact") || f.contains("enchantment")
                || f.contains("planeswalker") || f.contains("instant")
                || f.contains("sorcery") || f.contains("nonland");
            if !is_card_type {
                return true;
            }
        }
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
        // Check "permanent" (any permanent card type)
        if f == "permanent" {
            return card.card_types.iter().any(|ct| ct.is_permanent());
        }
        // Check "basic land"
        if f.contains("basic") && f.contains("land") {
            return card.supertypes.contains(&crate::constants::SuperType::Basic)
                && card.card_types.contains(&crate::constants::CardType::Land);
        }
        // Check card types
        for ct in &card.card_types {
            let ct_name = format!("{ct:?}").to_lowercase();
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
        source_colors: &[crate::constants::Color],
    ) -> Vec<ObjectId> {
        use crate::abilities::TargetSpec;

        match spec {
            TargetSpec::None => vec![],
            TargetSpec::Pair { first, second } => {
                let mut result = Vec::new();
                let first_targets = self.select_targets_for_spec(first, controller, source_colors);
                result.extend(&first_targets);
                let second_targets = self.select_targets_for_spec(second, controller, source_colors);
                result.extend(&second_targets);
                result
            }
            _ => {
                let legal = self.legal_targets_for_spec(spec, controller, source_colors);
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
                    requirement.legal_targets.into_iter().take(1).collect()
                };
                if chosen.is_empty() {
                    let legal = self.legal_targets_for_spec(spec, controller, source_colors);
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
        source_colors: &[crate::constants::Color],
    ) -> Vec<ObjectId> {
        use crate::abilities::TargetSpec;
        match spec {
            TargetSpec::Creature => self
                .state
                .battlefield
                .iter()
                .filter(|p| p.is_creature() && !Self::is_untargetable(p, controller, source_colors))
                .map(|p| p.id())
                .collect(),
            TargetSpec::CreatureYouControl => self
                .state
                .battlefield
                .iter()
                .filter(|p| p.is_creature() && p.controller == controller)
                .map(|p| p.id())
                .collect(),
            TargetSpec::OpponentCreature => self
                .state
                .battlefield
                .iter()
                .filter(|p| p.is_creature() && p.controller != controller
                    && !Self::is_untargetable(p, controller, source_colors))
                .map(|p| p.id())
                .collect(),
            TargetSpec::CreatureOrPlayer => {
                let mut targets: Vec<ObjectId> = self
                    .state
                    .battlefield
                    .iter()
                    .filter(|p| p.is_creature() && !Self::is_untargetable(p, controller, source_colors))
                    .map(|p| p.id())
                    .collect();
                targets.sort();
                targets
            }
            TargetSpec::Permanent => self
                .state
                .battlefield
                .iter()
                .filter(|p| !Self::is_untargetable(p, controller, source_colors))
                .map(|p| p.id())
                .collect(),
            TargetSpec::PermanentFiltered(filter) => self
                .state
                .battlefield
                .iter()
                .filter(|p| Self::matches_filter(p, filter)
                    && !Self::is_untargetable(p, controller, source_colors))
                .map(|p| p.id())
                .collect(),
            TargetSpec::Spell => self
                .state
                .stack
                .iter()
                .map(|item| item.id)
                .collect(),
            _ => vec![],
        }
    }

    /// Check if a permanent is untargetable by a given controller and source colors.
    /// Returns true for shroud (can't be targeted by anyone),
    /// hexproof (can't be targeted by opponents), or
    /// hexproof from colors (can't be targeted by spells/abilities of matching colors from opponents).
    fn is_untargetable(perm: &Permanent, targeting_controller: PlayerId, source_colors: &[crate::constants::Color]) -> bool {
        if perm.has_keyword(crate::constants::KeywordAbilities::SHROUD) {
            return true;
        }
        if perm.has_hexproof() && perm.controller != targeting_controller {
            return true;
        }
        if perm.controller != targeting_controller && !perm.hexproof_from_colors.is_empty() {
            for sc in source_colors {
                if perm.hexproof_from_colors.contains(sc) {
                    return true;
                }
            }
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
            TargetSpec::PermanentFiltered(f) => format!("target {f}"),
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
                if state.battlefield.get(sid).is_some_and(|p| p.is_creature()) {
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
mod tests;
