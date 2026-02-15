// PlayerDecisionMaker trait and PlayerAction enum.
//
// This module defines the interface between the game engine and any player
// implementation (human, AI, random, RL agent). It lives in mtg-engine so
// the game loop can call it without depending on mtg-ai.
//
// Ported from mage.players.Player (Java) — only the decision/choice methods.

use crate::constants::{ManaColor, Outcome};
use crate::types::{AbilityId, ObjectId, PlayerId};

// ---------------------------------------------------------------------------
// Supporting types for decisions
// ---------------------------------------------------------------------------

/// A single action a player can take when they have priority.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PlayerAction {
    /// Pass priority (do nothing).
    Pass,

    /// Cast a spell from the given zone.
    CastSpell {
        card_id: ObjectId,
        /// Targets chosen for the spell (may be empty for untargeted spells).
        targets: Vec<ObjectId>,
        /// Mode choice for modal spells (0-indexed). `None` for non-modal spells.
        mode: Option<usize>,
        /// If true, the spell is cast without paying its mana cost
        /// (e.g. alternative cost, free cast effect).
        without_mana: bool,
    },

    /// Activate an activated ability of a permanent or card.
    ActivateAbility {
        source_id: ObjectId,
        ability_id: AbilityId,
        /// Targets for the ability.
        targets: Vec<ObjectId>,
    },

    /// Play a land from hand. Only legal during main phases with priority and
    /// if the player has remaining land plays.
    PlayLand {
        card_id: ObjectId,
    },

    /// Special actions (e.g. unmorph, suspend, companion, foretell).
    SpecialAction {
        source_id: ObjectId,
        action_type: SpecialActionType,
    },

    /// Tap a land or mana source to pay a mana cost. Used during mana payment.
    ActivateManaAbility {
        source_id: ObjectId,
        ability_id: AbilityId,
    },
}

/// Types of special actions that don't use the stack.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpecialActionType {
    TurnFaceUp,
    PlayFromExile,
    CompanionToHand,
    Foretell,
    Other(String),
}

/// Describes a target requirement for a spell or ability being resolved.
#[derive(Clone, Debug)]
pub struct TargetRequirement {
    /// Human-readable description of what is being targeted (e.g. "target creature").
    pub description: String,
    /// Legal target object IDs.
    pub legal_targets: Vec<ObjectId>,
    /// Minimum number of targets to choose.
    pub min_targets: usize,
    /// Maximum number of targets to choose.
    pub max_targets: usize,
    /// Whether this is a mandatory or optional target.
    pub required: bool,
}

/// Describes a choice from a list of named options (e.g. mode selection, pile choice).
#[derive(Clone, Debug)]
pub struct NamedChoice {
    /// The unique key/index for this choice.
    pub index: usize,
    /// Human-readable description of the option.
    pub description: String,
}

/// Describes an amount of damage that must be distributed among targets.
#[derive(Clone, Debug)]
pub struct DamageAssignment {
    /// Total damage to distribute.
    pub total_damage: u32,
    /// Objects that can receive damage.
    pub targets: Vec<ObjectId>,
    /// Minimum damage each target must receive (usually 1 for "divide" effects,
    /// or the target's toughness for trample).
    pub minimum_per_target: Vec<u32>,
}

/// Information about an attacker for blocker assignment.
#[derive(Clone, Debug)]
pub struct AttackerInfo {
    pub attacker_id: ObjectId,
    /// Which player or planeswalker is being attacked.
    pub defending_id: ObjectId,
    /// Creatures that can legally block this attacker.
    pub legal_blockers: Vec<ObjectId>,
    /// Whether this creature must be blocked if able.
    pub must_be_blocked: bool,
    /// Maximum blockers allowed (None = unlimited, Some(1) = can't be blocked by more than 1).
    pub max_blocked_by: Option<u32>,
}

/// Description of an unpaid mana cost component.
#[derive(Clone, Debug)]
pub struct UnpaidMana {
    /// Which colors can satisfy this cost component. Empty means generic.
    pub acceptable_colors: Vec<ManaColor>,
    /// Whether this is a generic mana cost (any color/colorless works).
    pub is_generic: bool,
    /// Whether snow mana is required.
    pub requires_snow: bool,
}

/// Information about a replacement effect that the player must choose between.
#[derive(Clone, Debug)]
pub struct ReplacementEffectChoice {
    pub index: usize,
    pub description: String,
    pub source_name: String,
}

/// Read-only view of the game state for decision-making.
///
/// This is a reference to the actual GameState — implementations should not
/// need to clone or mutate it. The game engine passes this to every decision
/// method so the AI/player can inspect the board, hands, etc.
///
/// This is currently a placeholder. When `state.rs` is implemented (task #5),
/// this will become a proper type alias or wrapper around GameState.
pub struct GameView<'a> {
    _marker: std::marker::PhantomData<&'a ()>,
    // Will contain:
    // pub state: &'a GameState,
    // pub active_player: PlayerId,
    // pub priority_player: PlayerId,
    // pub current_step: PhaseStep,
    // pub turn_number: u32,
    // ... visibility filters per player
}

impl<'a> GameView<'a> {
    /// Temporary constructor while GameState is not yet implemented.
    pub fn placeholder() -> Self {
        GameView {
            _marker: std::marker::PhantomData,
        }
    }
}

// ---------------------------------------------------------------------------
// The trait
// ---------------------------------------------------------------------------

/// The core decision-making interface for all player types.
///
/// The game engine calls these methods whenever a player needs to make a
/// choice. Implementations include:
/// - **RandomPlayer** — picks uniformly at random from legal options
/// - **HeuristicPlayer** — port of ComputerPlayer.java
/// - **MinimaxPlayer** — port of ComputerPlayer6.java with alpha-beta search
/// - **RLAgent** — neural network policy, action selected from observation tensor
/// - **HumanPlayer** — forwards choices to a UI (not in scope for mtg-rl)
///
/// All methods receive a `&GameView` providing read-only access to the full
/// game state. The engine guarantees that only legal options are presented
/// in the choice parameters.
///
/// Methods return the chosen option. If a method returns an invalid choice,
/// the engine will treat it as an illegal action (the exact behavior depends
/// on the game loop — typically the game will choose a default or pass).
pub trait PlayerDecisionMaker: Send + Sync {
    /// Called when the player has priority. Must return one of the legal actions.
    ///
    /// This is the primary decision point. The engine provides the set of
    /// currently legal actions (always includes `PlayerAction::Pass`).
    fn priority(
        &mut self,
        game: &GameView<'_>,
        legal_actions: &[PlayerAction],
    ) -> PlayerAction;

    /// Choose targets for a spell or ability.
    ///
    /// Returns the selected target IDs. The engine validates that the returned
    /// targets are within `requirement.legal_targets` and the count is within
    /// `[min_targets, max_targets]`.
    fn choose_targets(
        &mut self,
        game: &GameView<'_>,
        outcome: Outcome,
        requirement: &TargetRequirement,
    ) -> Vec<ObjectId>;

    /// Yes/no decision (e.g. "Do you want to pay {2}?", "Sacrifice a creature?").
    ///
    /// `outcome` indicates whether saying "yes" is generally good or bad for
    /// the player, helping AI implementations decide without deep evaluation.
    fn choose_use(
        &mut self,
        game: &GameView<'_>,
        outcome: Outcome,
        message: &str,
    ) -> bool;

    /// Choose a mode for a modal spell or ability.
    ///
    /// `modes` lists the available mode descriptions. Returns the 0-based
    /// index of the chosen mode. For spells that choose multiple modes,
    /// this will be called once per mode selection.
    fn choose_mode(
        &mut self,
        game: &GameView<'_>,
        modes: &[NamedChoice],
    ) -> usize;

    /// Choose attackers during the declare attackers step.
    ///
    /// `possible_attackers` lists all creatures that can legally attack.
    /// Returns the IDs of creatures that will attack, along with the
    /// defender each attacks (player or planeswalker).
    fn select_attackers(
        &mut self,
        game: &GameView<'_>,
        possible_attackers: &[ObjectId],
        possible_defenders: &[ObjectId],
    ) -> Vec<(ObjectId, ObjectId)>;

    /// Choose blockers during the declare blockers step.
    ///
    /// `attackers` describes each attacking creature and which creatures
    /// can legally block it. Returns pairs of (blocker_id, attacker_id).
    fn select_blockers(
        &mut self,
        game: &GameView<'_>,
        attackers: &[AttackerInfo],
    ) -> Vec<(ObjectId, ObjectId)>;

    /// Distribute damage among multiple targets.
    ///
    /// Used for effects like "deal 5 damage divided as you choose" and
    /// trample damage assignment. Returns (target_id, damage_amount) pairs
    /// that must sum to `assignment.total_damage`.
    fn assign_damage(
        &mut self,
        game: &GameView<'_>,
        assignment: &DamageAssignment,
    ) -> Vec<(ObjectId, u32)>;

    /// Mulligan decision. Returns `true` to mulligan (shuffle and draw one fewer),
    /// `false` to keep the current hand.
    fn choose_mulligan(
        &mut self,
        game: &GameView<'_>,
        hand: &[ObjectId],
    ) -> bool;

    /// After mulliganing, choose which cards to put on the bottom of the library.
    ///
    /// `hand` is the current hand, `count` is how many cards must be put back.
    /// Returns exactly `count` card IDs from `hand`.
    fn choose_cards_to_put_back(
        &mut self,
        game: &GameView<'_>,
        hand: &[ObjectId],
        count: usize,
    ) -> Vec<ObjectId>;

    /// Choose cards to discard (e.g. for hand size limit or discard effects).
    ///
    /// `hand` is the current hand, `count` is how many must be discarded.
    /// Returns exactly `count` card IDs from `hand`.
    fn choose_discard(
        &mut self,
        game: &GameView<'_>,
        hand: &[ObjectId],
        count: usize,
    ) -> Vec<ObjectId>;

    /// Choose a number within a range (e.g. X in mana costs, number of counters).
    fn choose_amount(
        &mut self,
        game: &GameView<'_>,
        message: &str,
        min: u32,
        max: u32,
    ) -> u32;

    /// Pay a mana cost by activating mana abilities.
    ///
    /// `unpaid` describes the remaining unpaid portion of the cost.
    /// `mana_abilities` lists the mana abilities that can be activated.
    /// Returns the mana ability action to activate, or `None` to stop
    /// paying (which may result in the spell being canceled).
    fn choose_mana_payment(
        &mut self,
        game: &GameView<'_>,
        unpaid: &UnpaidMana,
        mana_abilities: &[PlayerAction],
    ) -> Option<PlayerAction>;

    /// Choose between multiple replacement effects that want to modify the
    /// same event (e.g. multiple "if this would die" effects).
    ///
    /// Returns the index of the chosen replacement effect.
    fn choose_replacement_effect(
        &mut self,
        game: &GameView<'_>,
        effects: &[ReplacementEffectChoice],
    ) -> usize;

    /// Choose which pile to take when a card splits into two piles
    /// (e.g. Fact or Fiction). Returns `true` for pile 1, `false` for pile 2.
    fn choose_pile(
        &mut self,
        game: &GameView<'_>,
        outcome: Outcome,
        message: &str,
        pile1: &[ObjectId],
        pile2: &[ObjectId],
    ) -> bool;

    /// Generic choice from a list of named options. Used for miscellaneous
    /// decisions like choosing a color, creature type, card name, etc.
    fn choose_option(
        &mut self,
        game: &GameView<'_>,
        outcome: Outcome,
        message: &str,
        options: &[NamedChoice],
    ) -> usize;

    /// Called at the start of each game to let the implementation initialize
    /// any per-game state. Default implementation does nothing.
    fn on_game_start(&mut self, _game: &GameView<'_>, _player_id: PlayerId) {}

    /// Called at the end of the game. Implementations can use this for
    /// learning (RL reward signal) or cleanup.
    fn on_game_end(&mut self, _game: &GameView<'_>, _won: bool) {}
}
