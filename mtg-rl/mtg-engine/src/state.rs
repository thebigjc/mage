// GameState — the complete game state snapshot.
//
// Ported from mage.game.GameState. The GameState struct holds everything
// needed to describe a game in progress: players, zones, turn info, the
// stack, battlefield, exile, combat state, and continuous effects.
//
// The state is designed to be cheaply cloneable for AI search (minimax,
// MCTS). Use the `im` crate for persistent data structures in the future
// if clone performance becomes a bottleneck.

use crate::abilities::AbilityStore;
use crate::combat::CombatState;
use crate::constants::{ManaColor, PhaseStep, SubType, TurnPhase, Zone};
use crate::player::Player;
use crate::types::{AbilityId, ObjectId, PlayerId};
use crate::zones::{Battlefield, CardStore, Exile, Stack};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The complete game state at any point in time.
///
/// This is the "ground truth" that the game engine operates on. All game
/// actions modify a GameState, and the decision-making interfaces receive
/// a read-only reference to it (via GameView).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    // ── Players ──────────────────────────────────────────────────────────
    /// All players, keyed by PlayerId. Each player owns their library,
    /// hand, graveyard, mana pool, counters, and life total.
    pub players: HashMap<PlayerId, Player>,

    /// Turn order (player IDs in APNAP order).
    pub turn_order: Vec<PlayerId>,

    // ── Shared zones ─────────────────────────────────────────────────────
    /// The battlefield (all permanents in play).
    pub battlefield: Battlefield,

    /// The stack (spells and abilities waiting to resolve).
    pub stack: Stack,

    /// Exile zones.
    pub exile: Exile,

    // ── Card store ───────────────────────────────────────────────────────
    /// Central storage for all card data. Cards keep their CardData here
    /// regardless of which zone they're in; zones only track ObjectIds.
    pub card_store: CardStore,

    // ── Ability store ─────────────────────────────────────────────────
    /// Central registry of all abilities currently in the game.
    /// Abilities are registered when permanents enter the battlefield
    /// and removed when they leave.
    pub ability_store: AbilityStore,

    // ── Zone tracking ────────────────────────────────────────────────────
    /// Tracks which zone each object is currently in.
    /// Updated whenever an object changes zones.
    pub object_zones: HashMap<ObjectId, ZoneLocation>,

    // ── Turn state ───────────────────────────────────────────────────────
    /// Current turn number (1-based).
    pub turn_number: u32,

    /// Which player's turn it is.
    pub active_player: PlayerId,

    /// Who currently has priority.
    pub priority_player: PlayerId,

    /// Current turn phase.
    pub current_phase: TurnPhase,

    /// Current step within the phase.
    pub current_step: PhaseStep,

    // ── Game flags ───────────────────────────────────────────────────────
    /// Whether the game has ended.
    pub game_over: bool,

    /// The winner (if any).
    pub winner: Option<PlayerId>,

    /// Whether we are currently resolving a spell/ability (re-entrancy guard).
    pub resolving: bool,

    /// Counter for how many times all players have passed priority in
    /// succession (both pass = stack resolves or step ends).
    pub consecutive_passes: u32,

    // ── Day/Night tracking (Innistrad mechanics) ─────────────────────────
    pub has_day_night: bool,
    pub is_daytime: bool,

    // ── Monarch / Initiative ──────────────────────────────────────────────
    pub monarch: Option<PlayerId>,
    pub initiative: Option<PlayerId>,

    // ── Combat state ──────────────────────────────────────────────────────
    /// Current combat phase state (attackers, blockers, damage assignment).
    pub combat: CombatState,

    // ── Values map (for tracking miscellaneous game state) ───────────────
    /// Generic key-value store for effects that need to track state across
    /// turns (e.g. "did a creature die this turn", "total damage dealt").
    pub values: HashMap<String, i64>,

    // ── Impulse draw tracking ────────────────────────────────────────────
    /// Cards exiled with "you may play until ..." permission.
    pub impulse_playable: Vec<ImpulsePlayable>,

    // ── Delayed triggers ─────────────────────────────────────────────────
    /// One-shot triggered abilities registered by effects (e.g. "when this
    /// creature dies this turn, draw a card").
    pub delayed_triggers: Vec<DelayedTrigger>,

    // ── Damage doubling ────────────────────────────────────────────────
    /// Active damage doubling effects: (controller, chosen_creature_type).
    /// Rebuilt each apply_continuous_effects call.
    #[serde(skip)]
    pub damage_doublings: Vec<(PlayerId, SubType)>,

    // ── Mana doubling ───────────────────────────────────────────────
    /// Number of "basic lands produce double mana" effects currently active.
    /// Rebuilt each apply_continuous_effects call.
    #[serde(skip)]
    pub mana_doubling_basic_lands: u32,

    // ── Enhanced mana production ──────────────────────────────────
    /// Auras with EnhancedManaProduction: (aura_source_id, attached_to_land_id, chosen_color).
    /// Rebuilt each apply_continuous_effects call.
    #[serde(skip)]
    pub enhanced_mana_productions: Vec<(ObjectId, ObjectId, ManaColor)>,

    // ── Trigger doubling ──────────────────────────────────────────
    /// Active trigger doubling effects: (source_id, controller, filter).
    /// Rebuilt each apply_continuous_effects call.
    #[serde(skip)]
    pub trigger_doublings: Vec<(ObjectId, PlayerId, String)>,

    pub trigger_counts_this_turn: HashMap<AbilityId, u32>,

    pub ability_resolution_counts_this_turn: HashMap<AbilityId, u32>,
}

/// Duration for impulse draw effects (how long the exiled card remains playable).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImpulseDuration {
    /// Playable until end of the current turn.
    EndOfTurn,
    /// Playable until end of the controller's next turn.
    UntilEndOfNextTurn,
}

/// Tracks an exiled card that can be played by a specific player.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImpulsePlayable {
    /// The exiled card that can be played.
    pub card_id: ObjectId,
    /// Who can play this card.
    pub player_id: PlayerId,
    /// When the permission expires.
    pub duration: ImpulseDuration,
    /// Turn number when the effect was created (for expiration tracking).
    pub created_turn: u32,
    /// Whether to play without paying mana cost.
    pub without_mana: bool,
}

/// Duration for delayed triggers.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DelayedDuration {
    /// Expires at end of the current turn.
    EndOfTurn,
    /// Never expires on its own (must be explicitly removed or fire once).
    UntilTriggered,
}

/// A delayed triggered ability registered by a resolving effect.
/// Example: "When this creature dies this turn, draw a card."
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DelayedTrigger {
    /// Event type that causes this trigger to fire.
    pub event_type: crate::events::EventType,
    /// The specific object this trigger watches (e.g. the creature that must die).
    /// None means any matching event fires it.
    pub watching: Option<ObjectId>,
    /// Effects to execute when the trigger fires.
    pub effects: Vec<crate::abilities::Effect>,
    /// Who controls the trigger (and its effects).
    pub controller: PlayerId,
    /// Source permanent that created this trigger.
    pub source: Option<ObjectId>,
    /// Targets for the effects (captured at creation time, if any).
    pub targets: Vec<ObjectId>,
    /// How long this trigger persists.
    pub duration: DelayedDuration,
    /// If true, trigger fires at most once then is removed.
    pub trigger_only_once: bool,
    /// Turn number when created (for expiration).
    pub created_turn: u32,
    /// If set, the event's source (target_id) must be a permanent controlled
    /// by this trigger's controller matching the given filter (e.g. "creatures you control").
    pub controller_filter: Option<String>,
}

/// Describes where a specific game object currently exists.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZoneLocation {
    /// Which zone the object is in.
    pub zone: Zone,
    /// Which player controls/owns the zone (e.g. which player's hand).
    /// None for shared zones like the stack or battlefield.
    pub controller: Option<PlayerId>,
}

impl GameState {
    /// Create a new game state for the given players (in turn order).
    pub fn new(player_names: &[(&str, PlayerId)]) -> Self {
        let mut players = HashMap::new();
        let mut turn_order = Vec::new();

        for &(name, id) in player_names {
            players.insert(id, Player::new(id, name));
            turn_order.push(id);
        }

        let active = turn_order[0];

        GameState {
            players,
            turn_order,
            battlefield: Battlefield::new(),
            stack: Stack::new(),
            exile: Exile::new(),
            card_store: CardStore::new(),
            ability_store: AbilityStore::new(),
            object_zones: HashMap::new(),
            turn_number: 1,
            active_player: active,
            priority_player: active,
            current_phase: TurnPhase::Beginning,
            current_step: PhaseStep::Untap,
            game_over: false,
            winner: None,
            resolving: false,
            consecutive_passes: 0,
            has_day_night: false,
            is_daytime: true,
            monarch: None,
            initiative: None,
            combat: CombatState::new(),
            values: HashMap::new(),
            impulse_playable: Vec::new(),
            delayed_triggers: Vec::new(),
            damage_doublings: Vec::new(),
            mana_doubling_basic_lands: 0,
            enhanced_mana_productions: Vec::new(),
            trigger_doublings: Vec::new(),
            trigger_counts_this_turn: HashMap::new(),
            ability_resolution_counts_this_turn: HashMap::new(),
        }
    }

    // ── Player access ────────────────────────────────────────────────────

    /// Get a reference to a player by ID.
    pub fn player(&self, id: PlayerId) -> Option<&Player> {
        self.players.get(&id)
    }

    /// Get a mutable reference to a player by ID.
    pub fn player_mut(&mut self, id: PlayerId) -> Option<&mut Player> {
        self.players.get_mut(&id)
    }

    /// Get the active player (whose turn it is).
    pub fn active_player(&self) -> &Player {
        self.players.get(&self.active_player).expect("active player not found")
    }

    /// Get the active player mutably.
    pub fn active_player_mut(&mut self) -> &mut Player {
        let id = self.active_player;
        self.players.get_mut(&id).expect("active player not found")
    }

    /// Get the priority player.
    pub fn priority_player(&self) -> &Player {
        self.players.get(&self.priority_player).expect("priority player not found")
    }

    /// Get the opponent of a player (for two-player games).
    pub fn opponent_of(&self, player_id: PlayerId) -> Option<PlayerId> {
        self.turn_order.iter().find(|&&id| id != player_id).copied()
    }

    /// Get all players still in the game.
    pub fn active_players(&self) -> Vec<PlayerId> {
        self.turn_order
            .iter()
            .filter(|&&id| {
                self.players
                    .get(&id)
                    .map(|p| p.is_in_game())
                    .unwrap_or(false)
            })
            .copied()
            .collect()
    }

    /// Get the next player in turn order after the given player.
    pub fn next_player(&self, after: PlayerId) -> PlayerId {
        let pos = self
            .turn_order
            .iter()
            .position(|&id| id == after)
            .expect("player not in turn order");
        let next_pos = (pos + 1) % self.turn_order.len();
        self.turn_order[next_pos]
    }

    // ── Zone tracking ────────────────────────────────────────────────────

    /// Record an object's current zone location.
    pub fn set_zone(&mut self, object_id: ObjectId, zone: Zone, controller: Option<PlayerId>) {
        self.object_zones.insert(
            object_id,
            ZoneLocation { zone, controller },
        );
    }

    /// Get the current zone of an object.
    pub fn get_zone(&self, object_id: ObjectId) -> Option<&ZoneLocation> {
        self.object_zones.get(&object_id)
    }

    /// Find which zone an object is in (simplified version).
    pub fn zone_of(&self, object_id: ObjectId) -> Option<Zone> {
        self.object_zones.get(&object_id).map(|loc| loc.zone)
    }

    // ── Graveyard helpers ──────────────────────────────────────────────

    /// Find which player's graveyard contains the given card.
    pub fn find_card_owner_in_graveyard(&self, card_id: ObjectId) -> Option<PlayerId> {
        for (&player_id, player) in &self.players {
            if player.graveyard.contains(card_id) {
                return Some(player_id);
            }
        }
        None
    }

    // ── Phase/step queries ───────────────────────────────────────────────

    /// Whether we are in a main phase (can play sorcery-speed spells/abilities).
    pub fn is_main_phase(&self) -> bool {
        self.current_phase.is_main()
    }

    /// Whether the stack is empty.
    pub fn stack_is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Whether a player can cast sorcery-speed spells (main phase, stack empty,
    /// active player has priority).
    pub fn can_cast_sorcery(&self, player_id: PlayerId) -> bool {
        self.is_main_phase()
            && self.stack_is_empty()
            && self.active_player == player_id
            && self.priority_player == player_id
    }

    // ── Game state checks ────────────────────────────────────────────────

    /// Check state-based actions (SBAs). Returns the IDs of players/objects
    /// that need action (dead players, lethal damage, etc.).
    ///
    /// The actual SBA processing is done by the game loop — this just detects
    /// what needs attention.
    pub fn check_state_based_actions(&self) -> StateBasedActions {
        let mut sba = StateBasedActions::new();

        for (&player_id, player) in &self.players {
            if !player.is_in_game() {
                continue;
            }

            // Rule 704.5a: Player at 0 or less life loses
            if player.life <= 0 {
                sba.players_losing.push(player_id);
            }

            // Rule 704.5c: Player with 10+ poison counters loses
            if player.poison_counters() >= 10 {
                sba.players_losing.push(player_id);
            }
        }

        // Rule 704.5f: Creature with 0 or less toughness goes to graveyard
        // Rule 704.5g: Creature with lethal damage marked on it is destroyed
        for perm in self.battlefield.iter() {
            if perm.is_creature() {
                if perm.toughness() <= 0 {
                    sba.permanents_to_graveyard.push(perm.id());
                } else if perm.has_lethal_damage() && !perm.has_indestructible() {
                    sba.permanents_to_destroy.push(perm.id());
                }
            }
        }

        // Rule 704.5j: Planeswalker with 0 or less loyalty goes to graveyard
        for perm in self.battlefield.iter() {
            if perm.is_planeswalker() {
                let loyalty = perm.counters.get(&crate::counters::CounterType::Loyalty);
                if loyalty == 0 {
                    sba.permanents_to_graveyard.push(perm.id());
                }
            }
        }

        // Rule 704.5k: Legend rule — if a player controls two or more legendary
        // permanents with the same name, they put all but one into the graveyard.
        {
            let mut legend_names: std::collections::HashMap<
                (PlayerId, String),
                Vec<ObjectId>,
            > = std::collections::HashMap::new();
            for perm in self.battlefield.iter() {
                if perm.is_legendary() {
                    legend_names
                        .entry((perm.controller, perm.name().to_string()))
                        .or_default()
                        .push(perm.id());
                }
            }
            for ((_controller, _name), ids) in &legend_names {
                if ids.len() > 1 {
                    // Keep the first (oldest by timestamp), put the rest in graveyard.
                    // TODO: Let the controller choose which to keep.
                    for &id in &ids[1..] {
                        if !sba.permanents_to_graveyard.contains(&id) {
                            sba.permanents_to_graveyard.push(id);
                        }
                    }
                }
            }
        }

        // Rule 704.5r: +1/+1 and -1/-1 counter pairs annihilate.
        for perm in self.battlefield.iter() {
            let p1p1 = perm.counters.get(&crate::counters::CounterType::P1P1);
            let m1m1 = perm.counters.get(&crate::counters::CounterType::M1M1);
            if p1p1 > 0 && m1m1 > 0 {
                sba.counters_to_annihilate.push(perm.id());
            }
        }

        // Rule 704.5n: Aura attached to illegal/missing permanent → graveyard.
        // Rule 704.5p: Equipment attached to illegal/missing permanent → unattach.
        for perm in self.battlefield.iter() {
            if let Some(attached_to) = perm.attached_to {
                if !self.battlefield.contains(attached_to) {
                    if perm.is_aura() {
                        sba.auras_to_graveyard.push(perm.id());
                    } else {
                        sba.attachments_to_detach.push(perm.id());
                    }
                }
            }
        }

        // Rule 704.5d: Tokens not on the battlefield cease to exist.
        for (&player_id, player) in &self.players {
            for &card_id in player.graveyard.iter() {
                if let Some(card) = self.card_store.get(card_id) {
                    if card.is_token {
                        sba.tokens_to_remove.push((player_id, card_id));
                    }
                }
            }
            for &card_id in player.hand.iter() {
                if let Some(card) = self.card_store.get(card_id) {
                    if card.is_token {
                        sba.tokens_to_remove.push((player_id, card_id));
                    }
                }
            }
        }
        // Also check exile zone for tokens
        for &card_id in self.exile.iter_all() {
            if let Some(card) = self.card_store.get(card_id) {
                if card.is_token {
                    // Find owner for removal
                    sba.tokens_to_remove.push((card.owner, card_id));
                }
            }
        }

        sba
    }

    /// Whether the game should end (all but one player has lost, or game_over flag set).
    pub fn should_end(&self) -> bool {
        if self.game_over {
            return true;
        }
        let alive: Vec<_> = self.active_players();
        alive.len() <= 1
    }
}

/// Results of checking state-based actions.
#[derive(Clone, Debug, Default)]
pub struct StateBasedActions {
    /// Players that should lose the game.
    pub players_losing: Vec<PlayerId>,
    /// Permanents that should be put into the graveyard (0 toughness, etc.).
    pub permanents_to_graveyard: Vec<ObjectId>,
    /// Permanents that should be destroyed (lethal damage, not indestructible).
    pub permanents_to_destroy: Vec<ObjectId>,
    /// Permanents with +1/+1 and -1/-1 counters that need annihilation.
    pub counters_to_annihilate: Vec<ObjectId>,
    /// Equipment that needs to be detached (attached target left battlefield).
    pub attachments_to_detach: Vec<ObjectId>,
    /// Auras that need to go to graveyard (enchanted permanent left battlefield).
    pub auras_to_graveyard: Vec<ObjectId>,
    /// Tokens in non-battlefield zones that should cease to exist (704.5d).
    pub tokens_to_remove: Vec<(PlayerId, ObjectId)>,
}

impl StateBasedActions {
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether any state-based actions need to be performed.
    pub fn has_actions(&self) -> bool {
        !self.players_losing.is_empty()
            || !self.permanents_to_graveyard.is_empty()
            || !self.attachments_to_detach.is_empty()
            || !self.auras_to_graveyard.is_empty()
            || !self.permanents_to_destroy.is_empty()
            || !self.counters_to_annihilate.is_empty()
            || !self.tokens_to_remove.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities};
    use crate::counters::CounterType;
    use crate::permanent::Permanent;

    fn two_player_state() -> (GameState, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let state = GameState::new(&[("Alice", p1), ("Bob", p2)]);
        (state, p1, p2)
    }

    #[test]
    fn initial_state() {
        let (state, p1, p2) = two_player_state();
        assert_eq!(state.turn_number, 1);
        assert_eq!(state.active_player, p1);
        assert_eq!(state.players.len(), 2);
        assert!(state.battlefield.is_empty());
        assert!(state.stack_is_empty());
        assert!(!state.game_over);

        let player1 = state.player(p1).unwrap();
        assert_eq!(player1.life, 20);

        assert_eq!(state.opponent_of(p1), Some(p2));
        assert_eq!(state.opponent_of(p2), Some(p1));
    }

    #[test]
    fn next_player_wraps() {
        let (state, p1, p2) = two_player_state();
        assert_eq!(state.next_player(p1), p2);
        assert_eq!(state.next_player(p2), p1);
    }

    #[test]
    fn sba_life_loss() {
        let (mut state, p1, _p2) = two_player_state();
        state.player_mut(p1).unwrap().life = 0;
        let sba = state.check_state_based_actions();
        assert!(sba.has_actions());
        assert!(sba.players_losing.contains(&p1));
    }

    #[test]
    fn sba_poison() {
        let (mut state, _p1, p2) = two_player_state();
        state.player_mut(p2).unwrap().add_poison(10);
        let sba = state.check_state_based_actions();
        assert!(sba.players_losing.contains(&p2));
    }

    #[test]
    fn sba_lethal_damage() {
        let (mut state, p1, _p2) = two_player_state();

        let mut card = CardData::new(ObjectId::new(), p1, "Bear");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        card.keywords = KeywordAbilities::empty();

        let mut perm = Permanent::new(card, p1);
        let perm_id = perm.id();
        perm.apply_damage(3);
        state.battlefield.add(perm);

        let sba = state.check_state_based_actions();
        assert!(sba.permanents_to_destroy.contains(&perm_id));
    }

    #[test]
    fn sba_zero_toughness() {
        let (mut state, p1, _p2) = two_player_state();

        let mut card = CardData::new(ObjectId::new(), p1, "Weird");
        card.card_types = vec![CardType::Creature];
        card.power = Some(3);
        card.toughness = Some(1);
        card.keywords = KeywordAbilities::empty();

        let mut perm = Permanent::new(card, p1);
        let perm_id = perm.id();
        // Simulate -1/-1 counter making toughness 0
        perm.add_counters(CounterType::M1M1, 1);
        state.battlefield.add(perm);

        let sba = state.check_state_based_actions();
        assert!(sba.permanents_to_graveyard.contains(&perm_id));
    }

    #[test]
    fn sba_indestructible_survives_damage() {
        let (mut state, p1, _p2) = two_player_state();

        let mut card = CardData::new(ObjectId::new(), p1, "Stuffy Doll");
        card.card_types = vec![CardType::Creature];
        card.power = Some(0);
        card.toughness = Some(1);
        card.keywords = KeywordAbilities::INDESTRUCTIBLE;

        let mut perm = Permanent::new(card, p1);
        perm.apply_damage(10);
        state.battlefield.add(perm);

        let sba = state.check_state_based_actions();
        // Should NOT be in the destroy list because indestructible
        assert!(sba.permanents_to_destroy.is_empty());
    }

    #[test]
    fn can_cast_sorcery() {
        let (mut state, p1, p2) = two_player_state();
        state.current_phase = TurnPhase::PrecombatMain;
        state.current_step = PhaseStep::PrecombatMain;
        state.active_player = p1;
        state.priority_player = p1;

        assert!(state.can_cast_sorcery(p1));
        assert!(!state.can_cast_sorcery(p2)); // Not active player
    }

    #[test]
    fn should_end_one_player_left() {
        let (mut state, _p1, p2) = two_player_state();
        state.player_mut(p2).unwrap().lost = true;
        assert!(state.should_end());
    }

    #[test]
    fn zone_tracking() {
        let (mut state, p1, _p2) = two_player_state();
        let card = ObjectId::new();
        state.set_zone(card, Zone::Hand, Some(p1));
        assert_eq!(state.zone_of(card), Some(Zone::Hand));

        state.set_zone(card, Zone::Battlefield, None);
        assert_eq!(state.zone_of(card), Some(Zone::Battlefield));
    }

    #[test]
    fn sba_legend_rule() {
        let (mut state, p1, _p2) = two_player_state();

        // Create two legendary creatures with the same name
        let mut card1 = CardData::new(ObjectId::new(), p1, "Thalia");
        card1.card_types = vec![CardType::Creature];
        card1.supertypes = vec![crate::constants::SuperType::Legendary];
        card1.power = Some(2);
        card1.toughness = Some(1);
        card1.keywords = KeywordAbilities::empty();

        let mut card2 = CardData::new(ObjectId::new(), p1, "Thalia");
        card2.card_types = vec![CardType::Creature];
        card2.supertypes = vec![crate::constants::SuperType::Legendary];
        card2.power = Some(2);
        card2.toughness = Some(1);
        card2.keywords = KeywordAbilities::empty();

        let id1 = card1.id;
        let id2 = card2.id;

        state.battlefield.add(Permanent::new(card1, p1));
        state.battlefield.add(Permanent::new(card2, p1));

        let sba = state.check_state_based_actions();
        assert!(sba.has_actions());
        // One of them should be in the graveyard list
        assert_eq!(sba.permanents_to_graveyard.len(), 1);
        // The first one (oldest) should be kept
        assert!(sba.permanents_to_graveyard.contains(&id2));
        assert!(!sba.permanents_to_graveyard.contains(&id1));
    }

    #[test]
    fn sba_legend_rule_different_controllers() {
        let (mut state, p1, p2) = two_player_state();

        // Two legendary creatures with the same name but controlled by different players
        // should NOT trigger the legend rule
        let mut card1 = CardData::new(ObjectId::new(), p1, "Thalia");
        card1.card_types = vec![CardType::Creature];
        card1.supertypes = vec![crate::constants::SuperType::Legendary];
        card1.power = Some(2);
        card1.toughness = Some(1);
        card1.keywords = KeywordAbilities::empty();

        let mut card2 = CardData::new(ObjectId::new(), p2, "Thalia");
        card2.card_types = vec![CardType::Creature];
        card2.supertypes = vec![crate::constants::SuperType::Legendary];
        card2.power = Some(2);
        card2.toughness = Some(1);
        card2.keywords = KeywordAbilities::empty();

        state.battlefield.add(Permanent::new(card1, p1));
        state.battlefield.add(Permanent::new(card2, p2));

        let sba = state.check_state_based_actions();
        // No legend rule violation since different controllers
        assert!(sba.permanents_to_graveyard.is_empty());
    }

    #[test]
    fn sba_counter_annihilation() {
        let (mut state, p1, _p2) = two_player_state();

        let mut card = CardData::new(ObjectId::new(), p1, "Bear");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        card.keywords = KeywordAbilities::empty();

        let mut perm = Permanent::new(card, p1);
        let perm_id = perm.id();
        perm.add_counters(CounterType::P1P1, 3);
        perm.add_counters(CounterType::M1M1, 2);
        state.battlefield.add(perm);

        let sba = state.check_state_based_actions();
        assert!(sba.has_actions());
        assert!(sba.counters_to_annihilate.contains(&perm_id));
    }

    #[test]
    fn sba_no_counter_annihilation_when_only_one_type() {
        let (mut state, p1, _p2) = two_player_state();

        let mut card = CardData::new(ObjectId::new(), p1, "Bear");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        card.keywords = KeywordAbilities::empty();

        let mut perm = Permanent::new(card, p1);
        perm.add_counters(CounterType::P1P1, 3);
        // No -1/-1 counters
        state.battlefield.add(perm);

        let sba = state.check_state_based_actions();
        assert!(sba.counters_to_annihilate.is_empty());
    }

    #[test]
    fn sba_planeswalker_zero_loyalty() {
        let (mut state, p1, _p2) = two_player_state();

        let mut card = CardData::new(ObjectId::new(), p1, "Jace");
        card.card_types = vec![CardType::Planeswalker];
        card.keywords = KeywordAbilities::empty();

        let perm = Permanent::new(card, p1);
        let perm_id = perm.id();
        // No loyalty counters added = 0 loyalty
        state.battlefield.add(perm);

        let sba = state.check_state_based_actions();
        assert!(sba.permanents_to_graveyard.contains(&perm_id));
    }
}

#[cfg(test)]
mod token_cleanup_tests {
    use super::*;
    use crate::card::CardData;
    use crate::constants::CardType;
    use crate::types::{ObjectId, PlayerId};

    fn two_player_state() -> (GameState, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let state = GameState::new(&[("Alice", p1), ("Bob", p2)]);
        (state, p1, p2)
    }

    #[test]
    fn token_in_graveyard_triggers_sba() {
        let (mut state, p1, _p2) = two_player_state();

        // Create a token card and put it in the graveyard
        let token_id = ObjectId::new();
        let mut card = CardData::new(token_id, p1, "Soldier Token");
        card.card_types = vec![CardType::Creature];
        card.is_token = true;
        state.card_store.insert(card);
        state.players.get_mut(&p1).unwrap().graveyard.add(token_id);

        let sba = state.check_state_based_actions();
        assert!(sba.tokens_to_remove.iter().any(|(_, id)| *id == token_id));
        assert!(sba.has_actions());
    }

    #[test]
    fn non_token_in_graveyard_not_removed() {
        let (mut state, p1, _p2) = two_player_state();

        // Create a normal card in the graveyard
        let card_id = ObjectId::new();
        let mut card = CardData::new(card_id, p1, "Grizzly Bears");
        card.card_types = vec![CardType::Creature];
        state.card_store.insert(card);
        state.players.get_mut(&p1).unwrap().graveyard.add(card_id);

        let sba = state.check_state_based_actions();
        assert!(sba.tokens_to_remove.is_empty());
    }

    #[test]
    fn token_on_battlefield_not_removed() {
        let (mut state, p1, _p2) = two_player_state();

        // Create a token on the battlefield - should NOT be flagged
        let token_id = ObjectId::new();
        let mut card = CardData::new(token_id, p1, "Soldier Token");
        card.card_types = vec![CardType::Creature];
        card.is_token = true;
        card.power = Some(1);
        card.toughness = Some(1);
        let perm = crate::permanent::Permanent::new(card, p1);
        state.battlefield.add(perm);

        let sba = state.check_state_based_actions();
        assert!(sba.tokens_to_remove.is_empty());
    }
}
