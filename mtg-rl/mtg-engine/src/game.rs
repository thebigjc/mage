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

use crate::abilities::{Cost, Effect};
use crate::constants::AbilityType;
use crate::card::CardData;
use crate::constants::PhaseStep;
use crate::counters::CounterType;
use crate::decision::PlayerDecisionMaker;
use crate::permanent::Permanent;
use crate::state::{GameState, StateBasedActions};
use crate::turn::{has_priority, PriorityTracker, TurnManager};
use crate::types::{AbilityId, ObjectId, PlayerId};
use crate::watchers::WatcherManager;
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

        // -- Check state-based actions (loop until stable) --
        self.process_state_based_actions();

        // -- Handle triggered abilities --
        // TODO: Put triggered abilities on the stack (task #13)

        // -- Priority loop --
        if has_priority(step) {
            self.priority_loop();
        }
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
                }
                // Empty mana pools
                for player in self.state.players.values_mut() {
                    player.mana_pool.clear();
                }
            }
            _ => {
                // Other steps: empty mana pool at step transition (simplified)
                // In full rules, mana empties at end of each step/phase.
            }
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

                    actions.push(crate::decision::PlayerAction::CastSpell {
                        card_id,
                        targets: vec![],
                        mode: None,
                        without_mana: false,
                    });
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

        if !player.hand.remove(card_id) {
            return;
        }

        player.play_land();

        // Create permanent from card data
        if let Some(card_data) = self.state.card_store.get(card_id).cloned() {
            // Register abilities from the card
            for ability in &card_data.abilities {
                self.state.ability_store.add(ability.clone());
            }
            let perm = Permanent::new(card_data, player_id);
            self.state.battlefield.add(perm);
            self.state.set_zone(card_id, crate::constants::Zone::Battlefield, None);
        }
    }

    /// Cast a spell (simplified: pay mana, move to stack, then resolve immediately
    /// for now since the full stack resolution needs the ability framework).
    fn cast_spell(&mut self, player_id: PlayerId, card_id: ObjectId) {
        let card_data = match self.state.card_store.get(card_id).cloned() {
            Some(c) => c,
            None => return,
        };

        // Remove from hand
        if let Some(player) = self.state.players.get_mut(&player_id) {
            if !player.hand.remove(card_id) {
                return;
            }

            // Pay mana cost
            let mana_cost = card_data.mana_cost.to_mana();
            if !player.mana_pool.try_pay(&mana_cost) {
                // Can't pay — put card back in hand
                player.hand.add(card_id);
                return;
            }
        }

        // Put on the stack
        let stack_item = crate::zones::StackItem {
            id: card_id,
            kind: crate::zones::StackItemKind::Spell { card: card_data.clone() },
            controller: player_id,
            targets: vec![],
            countered: false,
        };
        self.state.stack.push(stack_item);
        self.state.set_zone(card_id, crate::constants::Zone::Stack, None);
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
                } else {
                    // Non-permanent spells: execute effects then go to graveyard
                    let effects: Vec<Effect> = card.abilities.iter()
                        .flat_map(|a| a.effects.clone())
                        .collect();
                    let targets = item.targets.clone();
                    self.execute_effects(&effects, item.controller, &targets);
                    self.move_card_to_graveyard(item.id, item.controller);
                }
            }
            crate::zones::StackItemKind::Ability { ability_id, .. } => {
                // Resolve ability: find its effects and execute them
                let ability_data = self.state.ability_store.get(*ability_id).cloned();
                if let Some(ability) = ability_data {
                    let targets = item.targets.clone();
                    self.execute_effects(&ability.effects, item.controller, &targets);
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
    fn apply_state_based_actions(&mut self, sba: &StateBasedActions) {
        // Players losing the game
        for &pid in &sba.players_losing {
            if let Some(player) = self.state.players.get_mut(&pid) {
                player.lost = true;
            }
        }

        // Permanents going to graveyard (0 toughness)
        for &perm_id in &sba.permanents_to_graveyard {
            if let Some(perm) = self.state.battlefield.remove(perm_id) {
                let owner = perm.owner();
                self.state.ability_store.remove_source(perm_id);
                self.move_card_to_graveyard(perm_id, owner);
            }
        }

        // Permanents being destroyed (lethal damage)
        for &perm_id in &sba.permanents_to_destroy {
            if let Some(perm) = self.state.battlefield.remove(perm_id) {
                let owner = perm.owner();
                self.state.ability_store.remove_source(perm_id);
                self.move_card_to_graveyard(perm_id, owner);
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

    /// Pay the costs for an ability or spell. Returns false if costs can't be paid.
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
                _ => {
                    // Other costs (exile from hand, remove counters, sacrifice other, etc.)
                    // will be implemented as cards need them
                }
            }
        }
        true
    }

    /// Execute a list of effects for a controller with given targets.
    pub fn execute_effects(&mut self, effects: &[Effect], controller: PlayerId, targets: &[ObjectId]) {
        for effect in effects {
            match effect {
                Effect::DealDamage { amount } => {
                    // Deal damage to target permanents.
                    // Player targeting is handled separately via SelectedTargets.
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            perm.apply_damage(*amount);
                        }
                    }
                    // If no permanent targets, deal damage to opponents
                    // (simplified for "deal N damage to target opponent" effects)
                    if targets.is_empty() {
                        if let Some(opp_id) = self.state.opponent_of(controller) {
                            if let Some(opp) = self.state.players.get_mut(&opp_id) {
                                opp.life -= *amount as i32;
                            }
                        }
                    }
                }
                Effect::Destroy => {
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.get(target_id) {
                            if !perm.has_indestructible() {
                                if let Some(perm) = self.state.battlefield.remove(target_id) {
                                    self.state.ability_store.remove_source(target_id);
                                    self.move_card_to_graveyard_inner(target_id, perm.owner());
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
                Effect::DrawCards { count } => {
                    self.draw_cards(controller, *count);
                }
                Effect::GainLife { amount } => {
                    if let Some(player) = self.state.players.get_mut(&controller) {
                        player.life += *amount as i32;
                    }
                }
                Effect::LoseLife { amount } => {
                    // Controller loses life (target player effects will use
                    // SelectedTargets for proper player targeting)
                    if let Some(player) = self.state.players.get_mut(&controller) {
                        player.life -= *amount as i32;
                    }
                }
                Effect::DealDamageOpponents { amount } => {
                    let opponents: Vec<PlayerId> = self.state.turn_order.iter()
                        .filter(|&&id| id != controller)
                        .copied()
                        .collect();
                    for opp in opponents {
                        if let Some(player) = self.state.players.get_mut(&opp) {
                            player.life -= *amount as i32;
                        }
                    }
                }
                Effect::AddCounters { counter_type, count } => {
                    let ct = crate::counters::CounterType::from_name(counter_type);
                    for &target_id in targets {
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            perm.add_counters(ct.clone(), *count);
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
                Effect::Mill { count } => {
                    for _ in 0..*count {
                        let card_id = self.state.players.get_mut(&controller)
                            .and_then(|p| p.library.draw());
                        if let Some(card_id) = card_id {
                            self.move_card_to_graveyard_inner(card_id, controller);
                        }
                    }
                }
                Effect::CreateToken { token_name, count } => {
                    for _ in 0..*count {
                        // Create a minimal token permanent
                        let token_id = ObjectId::new();
                        let mut card = CardData::new(token_id, controller, token_name);
                        card.card_types = vec![crate::constants::CardType::Creature];
                        // Parse token stats from name (e.g. "4/4 Dragon with flying")
                        let (p, t, kw) = Self::parse_token_stats(token_name);
                        card.power = Some(p);
                        card.toughness = Some(t);
                        card.keywords = kw;
                        let perm = Permanent::new(card, controller);
                        self.state.battlefield.add(perm);
                        self.state.set_zone(token_id, crate::constants::Zone::Battlefield, None);
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
                                let perm = Permanent::new(card_data, controller);
                                self.state.battlefield.add(perm);
                                self.state.set_zone(target_id, crate::constants::Zone::Battlefield, None);
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
                    // For "target player sacrifices" effects, this targets the opponent.
                    let opponents: Vec<PlayerId> = self.state.turn_order.iter()
                        .filter(|&&id| id != controller)
                        .copied()
                        .collect();
                    for opp in opponents {
                        // Find permanents controlled by opponent matching filter
                        let matching: Vec<ObjectId> = self.state.battlefield.iter()
                            .filter(|p| p.controller == opp && Self::matches_filter(p, filter))
                            .map(|p| p.id())
                            .collect();
                        if let Some(&victim_id) = matching.first() {
                            // Simplified: sacrifice the first matching permanent
                            // (proper implementation would let opponent choose)
                            if let Some(perm) = self.state.battlefield.remove(victim_id) {
                                self.state.ability_store.remove_source(victim_id);
                                self.move_card_to_graveyard_inner(victim_id, perm.owner());
                            }
                        }
                    }
                }
                Effect::DestroyAll { filter } => {
                    // Destroy all permanents matching filter
                    let to_destroy: Vec<(ObjectId, PlayerId)> = self.state.battlefield.iter()
                        .filter(|p| Self::matches_filter(p, filter) && !p.has_indestructible())
                        .map(|p| (p.id(), p.owner()))
                        .collect();
                    for (id, owner) in to_destroy {
                        if self.state.battlefield.remove(id).is_some() {
                            self.state.ability_store.remove_source(id);
                            self.move_card_to_graveyard_inner(id, owner);
                        }
                    }
                }
                Effect::DealDamageAll { amount, filter } => {
                    // Deal damage to all creatures matching filter
                    let matching: Vec<ObjectId> = self.state.battlefield.iter()
                        .filter(|p| p.is_creature() && Self::matches_filter(p, filter))
                        .map(|p| p.id())
                        .collect();
                    for id in matching {
                        if let Some(perm) = self.state.battlefield.get_mut(id) {
                            perm.apply_damage(*amount);
                        }
                    }
                }
                Effect::RemoveCounters { counter_type, count } => {
                    let ct = crate::counters::CounterType::from_name(counter_type);
                    for &target_id in targets {
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
                _ => {
                    // Remaining effects not yet implemented (gain control, protection, etc.)
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
        // Check creature types
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
    use crate::constants::{CardType, KeywordAbilities, Outcome};
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
        game.execute_effects(&[Effect::DrawCards { count: 2 }], p1, &[]);

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

        game.execute_effects(&[Effect::GainLife { amount: 5 }], p1, &[]);
        assert_eq!(game.state.players.get(&p1).unwrap().life, 25);
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
        game.execute_effects(&[Effect::Exile], p1, &[bear_id]);

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
        game.execute_effects(&[Effect::Bounce], p1, &[bear_id]);

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
}
