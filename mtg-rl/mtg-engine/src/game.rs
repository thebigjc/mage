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
                    // Revert temporary control changes (GainControlUntilEndOfTurn)
                    if let Some(orig) = perm.original_controller.take() {
                        perm.controller = orig;
                    }
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
                    self.execute_effects(&effects, item.controller, &targets, Some(item.id));
                    self.move_card_to_graveyard(item.id, item.controller);
                }
            }
            crate::zones::StackItemKind::Ability { ability_id, source_id, .. } => {
                // Resolve ability: find its effects and execute them
                let source = *source_id;
                let ability_data = self.state.ability_store.get(*ability_id).cloned();
                if let Some(ability) = ability_data {
                    let targets = item.targets.clone();
                    self.execute_effects(&ability.effects, item.controller, &targets, Some(source));
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
                Cost::Custom(_) => {
                    // Custom costs: no-op (annotation only)
                }
            }
        }
        true
    }

    /// Execute a list of effects for a controller with given targets.
    pub fn execute_effects(&mut self, effects: &[Effect], controller: PlayerId, all_targets: &[ObjectId], source: Option<ObjectId>) {
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
                Effect::LoseLifeOpponents { amount } => {
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
                    // If no targets, fall back to source (self-targeting counters)
                    let effective_targets: Vec<ObjectId> = if targets.is_empty() {
                        source.into_iter().collect()
                    } else {
                        targets.to_vec()
                    };
                    for target_id in effective_targets {
                        if let Some(perm) = self.state.battlefield.get_mut(target_id) {
                            perm.add_counters(ct.clone(), *count);
                        }
                    }
                }
                Effect::AddCountersSelf { counter_type, count } => {
                    // Always add counters to the source permanent, even when the
                    // ability has other targets (e.g. blight self + grant haste to target).
                    if let Some(source_id) = source {
                        let ct = crate::counters::CounterType::from_name(counter_type);
                        if let Some(perm) = self.state.battlefield.get_mut(source_id) {
                            perm.add_counters(ct, *count);
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
                            dm.choose_discard(&view, &hand, *count as usize)
                        } else {
                            hand.iter().rev().take(*count as usize).copied().collect()
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
                            self.execute_effects(&mode.effects, controller, targets, source);
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
                        self.execute_effects(if_paid, controller, targets, source);
                    } else {
                        self.execute_effects(if_not_paid, controller, targets, source);
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
                .filter(|p| p.is_creature())
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
                .filter(|p| p.is_creature() && p.controller != controller)
                .map(|p| p.id())
                .collect(),
            TargetSpec::CreatureOrPlayer => {
                let mut targets: Vec<ObjectId> = self
                    .state
                    .battlefield
                    .iter()
                    .filter(|p| p.is_creature())
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
                .map(|p| p.id())
                .collect(),
            TargetSpec::PermanentFiltered(filter) => self
                .state
                .battlefield
                .iter()
                .filter(|p| Self::matches_filter(p, filter))
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
        game.execute_effects(&[Effect::DrawCards { count: 2 }], p1, &[], None);

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

        game.execute_effects(&[Effect::GainLife { amount: 5 }], p1, &[], None);
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

        game.execute_effects(&[Effect::lose_life_opponents(3)], p1, &[], None);
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
        game.execute_effects(&[Effect::Exile], p1, &[bear_id], None);

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
        game.execute_effects(&[Effect::Bounce], p1, &[bear_id], None);

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
        );

        let perm = game.state.battlefield.get(source_id).unwrap();
        assert_eq!(perm.counters.get(&CounterType::M1M1), 2);

        // Execute RemoveCounters with no targets but with source — should remove from self
        game.execute_effects(
            &[Effect::RemoveCounters { counter_type: "-1/-1".into(), count: 1 }],
            p1,
            &[],
            Some(source_id),
        );

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
        );

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
        game.execute_effects(&[Effect::discard_opponents(1)], p1, &[], None);

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
            p1, &[], None,
        );

        // P1's creatures should be 3/x, opponent's should remain 2/x
        assert_eq!(game.state.battlefield.get(bear1_id).unwrap().power(), 3);
        assert_eq!(game.state.battlefield.get(bear2_id).unwrap().power(), 3);
        assert_eq!(game.state.battlefield.get(opp_bear_id).unwrap().power(), 2);

        // Grant trample to all creatures P1 controls
        game.execute_effects(
            &[Effect::grant_keyword_all_eot("creatures you control", "trample")],
            p1, &[], None,
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
        );

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
        );

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
        game.execute_effects(&[Effect::fight()], p1, &[], None);

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
            None,
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
            None,
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
            None,
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
            None,
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
        game.execute_effects(&effects, p1, &targets, None);

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
        game.execute_effects(&[modal], p1, &[], None);

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
        game.execute_effects(&[modal], p1, &[], None);

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

        game.execute_effects(&[modal], p1, &[], None);

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
    use crate::permanent::Permanent;
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
    use crate::permanent::Permanent;
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

        game.execute_effects(&[Effect::GainLifeVivid], p1, &[], None);
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

        game.execute_effects(&[Effect::DealDamageVivid], p1, &[target], None);
        assert_eq!(game.state.battlefield.get(target).unwrap().damage, 2);
    }

    #[test]
    fn vivid_boost_until_eot() {
        let (mut game, p1, _) = setup();
        // p1 has 3 colors
        add_colored_creature(&mut game, p1, "R", "{R}");
        add_colored_creature(&mut game, p1, "G", "{G}");
        let target = add_colored_creature(&mut game, p1, "B", "{B}");

        game.execute_effects(&[Effect::BoostUntilEotVivid], p1, &[target], None);
        let perm = game.state.battlefield.get(target).unwrap();
        assert_eq!(perm.power(), 5); // 2 base + 3 vivid
        assert_eq!(perm.toughness(), 5);
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
    use crate::permanent::Permanent;
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

        game.execute_effects(&[effect], p1, &[], Some(src_id));

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

        game.execute_effects(&[effect], p1, &[], Some(src_id));

        // NeverPayPlayer says no, so blight 2 happens
        assert_eq!(game.state.players[&p1].life, 20); // no life paid
        assert_eq!(game.state.battlefield.get(src_id).unwrap().counters.get(&CounterType::M1M1), 2);
    }
}
