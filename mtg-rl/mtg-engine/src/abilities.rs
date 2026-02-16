// Ability framework — activated, triggered, static, spell, and mana abilities.
//
// In MTG, abilities are the things cards can do. The framework models:
// - **SpellAbility**: The ability a spell has while on the stack (resolve effects)
// - **ActivatedAbility**: "Cost: Effect" abilities that can be activated by a player
// - **TriggeredAbility**: "When/Whenever/At" abilities that trigger from events
// - **StaticAbility**: Abilities that generate continuous effects while in play
// - **ManaAbility**: Special activated abilities that produce mana (don't use the stack)
//
// Ported from mage.abilities.*.

use crate::constants::{AbilityType, Zone};
use crate::events::{EventType, GameEvent};
use crate::filters::Filter;
use crate::mana::Mana;
use crate::types::{AbilityId, ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriggerScope {
    SelfOnly,
    OtherControlled,
    Any,
}

/// Sentinel value for effect amounts that should use the X value from the stack.
/// When an effect has this amount, it will be resolved using the X value chosen at cast time.
pub const X_VALUE: u32 = u32::MAX;

// ---------------------------------------------------------------------------
// Cost types
// ---------------------------------------------------------------------------

/// A cost that must be paid to activate an ability or cast a spell.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Cost {
    /// Pay mana (e.g. "{2}{B}").
    Mana(Mana),
    /// Tap this permanent ("{T}").
    TapSelf,
    /// Untap this permanent ("{Q}").
    UntapSelf,
    /// Pay life.
    PayLife(u32),
    /// Sacrifice this permanent.
    SacrificeSelf,
    /// Exile this permanent (similar to sacrifice but goes to exile).
    ExileSelf,
    /// Sacrifice another permanent (described by text).
    SacrificeOther(String),
    /// Discard a card.
    Discard(u32),
    /// Exile a card from hand.
    ExileFromHand(u32),
    /// Exile a card from graveyard.
    ExileFromGraveyard(u32),
    /// Remove counters from this permanent.
    RemoveCounters(String, u32),
    /// Blight N — put N -1/-1 counters on a creature you control.
    /// (ECL set-specific mechanic.)
    Blight(u32),
    /// Variable Blight X — choose X, put X -1/-1 counters on a creature you control.
    /// Sets the spell's X value for subsequent effects.
    VariableBlight,
    /// Reveal a card of a specific type from hand (used by Behold).
    /// Reveal a card of a specific type from hand (used by Behold).
    RevealFromHand(String),
    /// Behold a creature type: choose a permanent of that type on battlefield or reveal
    /// a card of that type from hand. Mandatory additional cost.
    Behold(String),
    /// Behold a creature type and exile the chosen card/permanent.
    /// Mandatory additional cost used by Champion cards.
    BeholdAndExile(String),
    /// Behold a creature type, or pay alternative mana if unable/unwilling.
    /// Used by cards like "behold a Kithkin or pay {2}".
    BeholdOrPay { creature_type: String, mana: Mana },
    /// Tap N other untapped creatures you control matching a filter.
    TapCreatures { filter: Filter, count: u32 },
    /// A custom/complex cost (described by text).

    Custom(String),
}

// ---------------------------------------------------------------------------
// Effect types
// ---------------------------------------------------------------------------

/// What an effect does when it resolves. These are the building blocks
/// that card implementations compose to create their abilities.
///
/// Each variant describes a specific game action. Complex cards can chain
/// multiple effects. The game engine interprets these to modify the state.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Effect {
    // -- Damage --
    /// Deal damage to target creature or player.
    DealDamage { amount: u32 },
    /// Deal damage to each creature.
    DealDamageAll { amount: u32, filter: Filter },
    /// Deal damage to each opponent.
    DealDamageOpponents { amount: u32 },
    /// Deal damage to each creature opponents control.
    DealDamageOpponentsCreatures { amount: u32 },

    // -- Life --
    /// Gain life.
    GainLife { amount: u32 },
    /// Lose life (target player).
    LoseLife { amount: u32 },
    /// Each opponent loses life.
    LoseLifeOpponents { amount: u32 },
    /// Set life total.
    SetLife { amount: i32 },

    // -- Destroy / Remove --
    /// Destroy target permanent.
    Destroy,
    /// Destroy all permanents matching filter.
    DestroyAll { filter: Filter },
    /// Exile target permanent.
    Exile,
    /// Sacrifice a permanent (owner chooses).
    Sacrifice { filter: Filter },
    /// Return target permanent to hand.
    Bounce,
    /// Return all permanents matching filter to their owners' hands.
    BounceAll { filter: String },
    /// Exile the top N cards of target opponent's library.
    ExileFromOpponentLibrary { count: u32 },
    /// Exile the top N cards of target opponent's library into the source permanent's exile zone.
    ExileFromOpponentLibraryToSourceZone { count: u32 },
    /// Put target permanent on top of its owner's library.
    PutOnLibrary,
    /// Return target card from graveyard to hand.
    ReturnFromGraveyard,
    /// Return target card from graveyard to battlefield.
    Reanimate,

    // -- Cards --
    /// Draw cards.
    DrawCards { count: u32 },
    /// Discard cards.
    DiscardCards { count: u32 },
    /// Each opponent discards N cards.
    DiscardOpponents { count: u32 },
    /// Mill cards (library to graveyard).
    Mill { count: u32 },
    /// Mill N cards, then you may put a card matching filter from among them
    /// on top of your library (destination = "top") or into your hand (destination = "hand").
    MillAndSelect { count: u32, filter: String, destination: String },
    /// Mill N cards, then return ALL cards matching filter from among the milled to hand.
    MillAndReturnAll { count: u32, filter: String },
    /// Scry N (look at top N, put any on bottom in any order).
    Scry { count: u32 },
    /// Search library for a card.
    SearchLibrary { filter: Filter },
    /// Look at the top N cards of your library. You may reveal a card matching
    /// the filter from among them and put it into your hand. Put the rest on
    /// the bottom of your library in a random order.
    /// (Used by Eclipsed cycle, Earthbend, and similar "impulse look" effects.)
    LookTopAndPick { count: u32, filter: String },

    // -- Counters --
    /// Put counters on target.
    AddCounters { counter_type: String, count: u32 },
    /// Put counters on the source permanent (self), regardless of targets.
    /// Used in compound effects where other effects target a different permanent.
    AddCountersSelf { counter_type: String, count: u32 },
    /// Put counters on all permanents matching filter.
    AddCountersAll { counter_type: String, count: u32, filter: String },
    /// Remove counters from target.
    RemoveCounters { counter_type: String, count: u32 },

    // -- Tokens --
    /// Create token creatures.
    CreateToken { token_name: String, count: u32 },
    /// Create tokens that enter tapped and attacking, then sacrifice at next end step.
    /// (Used by TDM Mobilize mechanic.)
    CreateTokenTappedAttacking { token_name: String, count: u32 },

    // -- Mana --
    /// Add mana to controller's pool.
    AddMana { mana: Mana },

    // -- Combat --
    /// Target creature can't block this turn.
    CantBlock,
    /// Target creature must block this turn.
    MustBlock,
    /// Prevent combat damage.
    PreventCombatDamage,
    /// Fight — source creature and target creature each deal damage equal
    /// to their power to each other.
    Fight,
    /// Bite — source creature deals damage equal to its power to target
    /// creature (one-way; the target does not deal damage back).
    Bite,

    // -- Stats --
    /// Give +N/+M until end of turn.
    BoostUntilEndOfTurn { power: i32, toughness: i32 },
    /// Give +N/+M permanently (e.g. from counters, applied differently).
    BoostPermanent { power: i32, toughness: i32 },
    /// Give all matching creatures +N/+M until end of turn.
    BoostAllUntilEndOfTurn { filter: Filter, power: i32, toughness: i32 },
    /// Set power and toughness.
    SetPowerToughness { power: i32, toughness: i32 },
    /// Give target creature +X/+X until end of turn, where X = |toughness - power|.
    BoostByToughnessMinusPower,

    // -- Keywords --
    /// Grant a keyword ability until end of turn.
    GainKeywordUntilEndOfTurn { keyword: String },
    /// Grant a keyword to all matching creatures until end of turn.
    GrantKeywordAllUntilEndOfTurn { filter: String, keyword: String },
    /// Grant a keyword ability permanently.
    GainKeyword { keyword: String },
    /// Remove a keyword ability.
    LoseKeyword { keyword: String },
    /// Target creature loses all abilities (keywords + activated/triggered/static).
    LoseAllAbilities,
    /// Set base power and toughness of all creatures matching filter.
    /// Used for mass P/T setting effects like "each creature target opponent controls has base power and toughness 1/1".
    SetBasePowerToughnessAll { power: i32, toughness: i32, filter: String },
    /// Remove all abilities from all creatures matching filter.
    LoseAllAbilitiesAll { filter: String },
    /// Add a subtype to all creatures matching filter ("becomes X in addition to its other types").
    AddSubtypeAll { subtype: String, filter: String },
    /// Replace the source permanent's subtypes with the given list (for level-up / figure cards).
    SetSubtypesSelf { subtypes: Vec<String> },

    // -- Control --
    /// Gain control of target.
    GainControl,
    /// Gain control of target until end of turn.
    GainControlUntilEndOfTurn,

    // -- Tap --
    /// Tap target permanent.
    TapTarget,
    /// Untap target permanent.
    UntapTarget,

    // -- Counter spells --
    /// Counter target spell.
    CounterSpell,
    /// Counter all spells and abilities opponents control on the stack.
    /// Creates tokens equal to the number of spells/abilities countered.
    CounterAllOpponentSpellsAndAbilities { token_name: String },

    // -- Protection --
    /// Target gains protection from a color/quality until end of turn.
    GainProtection { from: String },
    /// Target becomes indestructible until end of turn.
    Indestructible,
    /// Target gains hexproof until end of turn.
    Hexproof,

    // -- Modal --
    /// Modal spell: choose min_modes to max_modes from the list, then
    /// execute each chosen mode's effects in order. Uses `choose_mode()`
    /// from the player decision maker.
    Modal { modes: Vec<ModalMode>, min_modes: usize, max_modes: usize },

    // -- Vivid (ECL mechanic) --
    /// Vivid -- Deal damage equal to the number of colors among permanents you control.
    DealDamageVivid,
    /// Vivid -- Gain life equal to the number of colors among permanents you control.
    GainLifeVivid,
    /// Vivid -- Target creature gets +X/+X until end of turn where X = colors among permanents you control.
    BoostUntilEotVivid,
    /// Vivid -- Each opponent loses X life where X = colors among permanents you control.
    LoseLifeOpponentsVivid,
    /// Vivid -- Draw X cards where X = colors among permanents you control.
    DrawCardsVivid,
    /// Vivid -- Other creatures you control get +X/+X until EOT where X = colors.
    BoostAllUntilEotVivid,
    /// Vivid -- Create X tokens where X = colors among permanents you control.
    CreateTokenVivid { token_name: String },
    /// Vivid -- Search library for up to X basic land cards where X = colors among permanents you control, put into hand.
    SearchLibraryVivid,
    /// Vivid -- Reveal cards from the top of your library until you reveal X permanent cards
    /// (X = colors among permanents you control). Put any number of those onto the battlefield,
    /// rest on the bottom in random order.
    RevealFromLibraryVivid,

    // -- Conditional cost --
    /// "You may pay [cost]. If you do, [if_paid]. If you don't, [if_not_paid]."
    /// Uses choose_use() for the yes/no decision, then pay_costs() if accepted.
    DoIfCostPaid {
        cost: Cost,
        if_paid: Vec<Effect>,
        if_not_paid: Vec<Effect>,
    },
    // -- Creature type choice --
    /// "As this permanent enters, choose a creature type." Stores the
    /// choice on the source permanent's `chosen_type` field.
    /// `restricted` limits the available types (empty = any type).
    ChooseCreatureType { restricted: Vec<String> },

    /// "As this permanent enters, choose a color." Stores the choice
    /// on the source permanent's `chosen_color` field.
    ChooseColor,

    /// "Choose a creature type. Draw a card for each permanent you control of that type."
    ChooseTypeAndDrawPerPermanent,

    /// "Choose a creature type. Return all creature cards of the chosen type from your graveyard to the battlefield."
    ChooseTypeAndReturnFromGraveyard,

    /// "Choose a creature type. Other permanents you control of the chosen type gain [keywords] until end of turn."
    ChooseTypeAndGrantKeywords { keywords: Vec<String>, other_only: bool },

    // -- Equipment --
    /// Attach source equipment to target creature you control.
    Equip,

    // -- Delayed triggers --
    /// Create a delayed triggered ability that fires when a specific event occurs.
    /// Example: "When this creature dies this turn, draw a card."
    CreateDelayedTrigger {
        /// Event type to watch for.
        event_type: String,
        /// Effects to execute when the trigger fires.
        trigger_effects: Vec<Effect>,
        /// "end_of_turn" or "until_triggered"
        duration: String,
        /// If true, watches the first target or source; if false, any matching event.
        watch_target: bool,
    },

    // -- Impulse draw --
    /// Exile top N cards of your library; you may play them until the specified duration.
    ExileTopAndPlay {
        count: u32,
        /// "end_of_turn" or "until_end_of_next_turn"
        duration: String,
        /// If true, may play without paying mana cost.
        without_mana: bool,
    },


    /// Return all cards exiled by this source to their owners hands.
    ReturnExiledToHand,

    /// Untap all permanents matching a filter.
    UntapAll { filter: String },

    /// Give target "can't be blocked this turn" until end of turn.
    CantBeBlockedUntilEot,

    /// Tap the permanent this source is attached to (aura/equipment ETB).
    TapAttached,

    /// Proliferate — for each permanent with a counter, add one more of each type it already has.
    /// For each player with a counter, do the same.
    Proliferate,

    /// Remove all counters from target creature.
    RemoveAllCounters,

    /// Exile up to N target cards from graveyards.
    ExileTargetCardsFromGraveyards { count: u32 },

    /// Exile target card(s) from graveyards into a named exile zone linked to the source.
    ExileTargetToSourceZone,

    /// Flicker: exile target creature, then immediately return it to the battlefield
    /// under its owner's control (as a new object, triggers ETB).
    Flicker,

    /// Flicker at end step: exile target creatures, then return them at the
    /// beginning of the next end step tapped under their owners' control.
    FlickerEndStep,

    /// Return target cards from exile to the battlefield tapped under their owners' control.
    ReturnFromExileTapped,

    /// Target opponent exiles cards from their hand.
    OpponentExilesFromHand { count: u32 },

    /// Each opponent blights N (puts N -1/-1 counters on a creature they control).
    BlightOpponents { count: u32 },

    /// Target creature gains all creature types until end of turn.
    GainAllCreatureTypes,
    /// Target creature becomes all colors until end of turn.
    BecomeAllColors,

    /// Create a token that is a copy of target creature/permanent.
    /// The token gets all the same characteristics (name, types, subtypes,
    /// abilities, P/T, keywords) plus any specified modifications.
    CreateTokenCopy {
        count: u32,
        modifications: Vec<TokenModification>,
    },

    CreateTokenCopyOfTriggering,

    /// Tap the source permanent (self-tap as part of an effect, not a cost).
    TapSelf,

    /// Return all creature cards of the specified type from your graveyard to the battlefield.
    ReturnAllTypeFromGraveyard { creature_type: String },

    /// Create X tokens where X is dynamically computed from count_filter.
    /// count_filter examples: "Elf cards in your graveyard", "Goblins you control"
    CreateTokenDynamic { token_name: String, count_filter: String },

    /// Gain life equal to a dynamically computed value.
    /// value_source examples: "greatest power among Giants you control"
    GainLifeDynamic { value_source: String },

    /// Target creature gets +X/+X until end of turn where X = a dynamic count.
    /// value_source examples: "Kithkin you control"
    BoostTargetDynamic { value_source: String },

    /// Dual-target dynamic boost: first target gets +X/+0, second target gets -0/-X,
    /// where X = evaluate_count_filter(value_source). Uses TargetSpec::Pair.
    /// targets[0] = creature you control (gets +X/+0)
    /// targets[1] = opponent creature (gets -0/-X)
    BoostDualTargetDynamic { value_source: String },

    /// The controller of the targeted permanent draws N cards.
    /// Used for effects like "Its controller draws a card."
    TargetControllerDraws { count: u32 },

    /// The controller of the targeted permanent creates a token.
    /// Used for effects like "Its controller creates a 1/1 token."
    TargetControllerCreatesToken { token_name: String },

    /// You may put a creature card matching the filter from your hand onto the battlefield.
    /// Supports haste grant, tapped entry, attacking entry, and sacrifice at next end step.
    /// `max_mana_value` is the maximum MV allowed (u32::MAX = no limit, X_VALUE = use X from stack).
    /// `max_mv_dynamic`: if Some, evaluated via evaluate_count_filter to determine MV limit (overrides max_mana_value).
    /// `tapped`: enters tapped. `attacking`: enters attacking. `haste`: gains haste.
    /// `sacrifice_eot`: sacrifice at the beginning of the next end step.
    PutFromHandToBattlefield {
        max_mana_value: u32,
        max_mv_dynamic: Option<String>,
        tapped: bool,
        attacking: bool,
        haste: bool,
        sacrifice_eot: bool,
    },

    /// Make the source permanent become an artifact creature until end of turn.
    /// Sets base power/toughness and adds the Creature card type.
    /// Retains existing types (e.g. artifact stays artifact).
    BecomesCreature {
        power: i32,
        toughness: i32,
    },

    // -- Transform (DFC) --
    /// Transform the source permanent (toggle between front and back face).
    /// Requires the card to have a back_face defined in CardData.
    TransformSelf,

    // -- Conditional --
    /// "If [condition], [if_true effects]. Otherwise, [if_false effects]."
    /// Evaluates a game-state condition string and branches accordingly.
    /// Supported conditions include "target is a {Type}", "you control a {Type}",
    /// "you control N or more {filter}", etc.
    Conditional {
        condition: String,
        if_true: Vec<Effect>,
        if_false: Vec<Effect>,
    },

    /// Choose two target creatures you control. X = abs(power difference).
    /// Draw X cards, both get +X/+X (via P1P1 counters), both gain trample until EOT.
    CompareAndBoost,

    /// "If this is the Nth time this ability has resolved this turn, [effects]."
    /// Checks the resolution count for the current ability and conditionally executes sub-effects.
    IfAbilityResolvedNTimes {
        resolution_number: u32,
        effects: Vec<Effect>,
    },

    /// Grant a triggered ability to permanents matching a filter until end of turn.
    /// Creates a delayed trigger that fires each time the specified event occurs
    /// for a permanent the controller controls matching the filter.
    /// Unlike CreateDelayedTrigger, this can fire multiple times (once per matching event).
    GrantTriggeredAbilityUntilEOT {
        /// Event type name (e.g. "damaged_player").
        event_type: String,
        /// Filter for permanents that can trigger this (e.g. "creatures you control").
        filter: String,
        /// Effects to execute each time the trigger fires.
        trigger_effects: Vec<Effect>,
    },

    /// "When you next cast an instant or sorcery spell this turn, copy that spell.
    /// You may choose new targets for the copy."
    /// Creates a delayed trigger on SpellCast that copies the next matching spell.
    CopyNextSpell,

    /// Copy the spell that triggered this ability (must be on the stack).
    /// Used by permanents with "whenever you cast [spell], copy it" triggers.
    /// `keywords`: keywords to grant to both original and copy (e.g. "wither").
    /// `single_target_only`: if true, only copies spells with exactly one target.
    CopyTriggeringSpell {
        keywords: Vec<String>,
        single_target_only: bool,
    },

    /// Target opponent reveals X cards from their hand (X = dynamic count).
    /// Controller chooses one to exile. If the exiled card is instant/sorcery,
    /// controller may cast it while controlling the source permanent.
    OpponentRevealsFromHandExileCast {
        /// Dynamic count source (e.g. "Goblins you control").
        count_source: String,
        /// Only instant/sorcery cards become playable (others are just exiled).
        instant_sorcery_only: bool,
    },

    /// Each opponent exiles cards from the top of their library until the total
    /// mana value of cards exiled this way is >= `mv_threshold`. Until end of turn,
    /// the controller may cast any of the exiled cards without paying their mana costs.
    OpponentsExileUntilMVAndCast {
        mv_threshold: u32,
    },

    /// Deal X damage to target creature. Create a delayed trigger: when that creature
    /// dies this turn, exile cards from the top of your library equal to its power,
    /// then choose one — you may play it until end of your next turn.
    DealDamageWithDelayedExile,

    /// Exile top N cards, choose one, you may play it until the specified duration.
    /// The rest remain in exile but are not playable.
    ExileTopChooseOneAndPlay {
        count: u32,
        duration: String,
    },

    /// For each player, the spell's controller chooses a creature that player controls.
    /// Then each player sacrifices all other creatures they control that don't share
    /// a creature type with the chosen creature they control.
    Winnowing,

    MassBecomeCopy,

    ExileWithDreamCounterInsteadOfGraveyard,

    CastFromExileWithDreamCounters,

    // -- Misc --
    /// A custom/complex effect described by text. The game engine or card
    /// code handles the specific implementation.

    Custom(String),
}

/// Modifications to apply when creating a token copy of a permanent.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TokenModification {
    /// Add a keyword ability (e.g. "haste", "flying").
    AddKeyword(String),
    /// Add the changeling keyword (all creature types).
    AddChangeling,
    /// Sacrifice the token at the next end step.
    SacrificeAtEndStep,
    /// The token enters tapped and attacking.
    EnterTappedAttacking,
}

/// One mode of a modal spell. Each mode has a description and a set of
/// effects to execute if that mode is chosen.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModalMode {
    /// Human-readable description of this mode (e.g. "Deal 3 damage to any target").
    pub description: String,
    /// The effects to execute when this mode is chosen.
    pub effects: Vec<Effect>,
}

// ---------------------------------------------------------------------------
// Target specification for abilities
// ---------------------------------------------------------------------------

/// Describes what an ability can target.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TargetSpec {
    /// No targets.
    None,
    /// Target creature.
    Creature,
    /// Target creature or player.
    CreatureOrPlayer,
    /// Target player.
    Player,
    /// Target permanent.
    Permanent,
    /// Target permanent matching a filter.
    PermanentFiltered(Filter),
    /// Target spell on the stack.
    Spell,
    /// Target card in a graveyard.
    CardInGraveyard,
    /// Target card in your graveyard.
    CardInYourGraveyard,
    /// Target creature you control.
    CreatureYouControl,
    /// Target creature you don't control (opponent's creature).
    OpponentCreature,
    /// Two targets (e.g. fight spells: your creature + opponent's creature).
    /// targets[0] comes from `first`, targets[1] from `second`.
    Pair { first: Box<TargetSpec>, second: Box<TargetSpec> },
    /// Multiple targets of the same type.
    Multiple { spec: Box<TargetSpec>, count: usize },
    /// Custom targeting (described by text).

    Custom(String),
}

// ---------------------------------------------------------------------------
// Ability struct
// ---------------------------------------------------------------------------

/// A concrete ability instance attached to a card or permanent.
///
/// This is a data-oriented design: each ability is a struct containing
/// its type, costs, effects, targets, and configuration. The game engine
/// interprets these to execute game actions.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ability {
    /// Unique ID for this ability instance.
    pub id: AbilityId,
    /// The source object (card or permanent) this ability belongs to.
    pub source_id: ObjectId,
    /// What kind of ability this is.
    pub ability_type: AbilityType,
    /// Human-readable rules text.
    pub rules_text: String,
    /// The zone(s) this ability functions from (e.g. battlefield, graveyard).
    pub active_zones: Vec<Zone>,
    /// Costs to activate (for activated/mana abilities).
    pub costs: Vec<Cost>,
    /// Effects that happen when this ability resolves.
    pub effects: Vec<Effect>,
    /// Target requirements.
    pub targets: TargetSpec,
    /// For triggered abilities: the event type(s) that trigger it.
    pub trigger_events: Vec<EventType>,
    /// For triggered abilities: whether the trigger is optional ("may").
    pub optional_trigger: bool,
    /// For triggered abilities: scope — self, other, or any.
    pub trigger_scope: TriggerScope,
    /// For triggered abilities: max times this can trigger per turn (0 = unlimited).
    pub triggers_per_turn: u32,
    /// For triggered abilities: filter on from_zone (e.g. Some(Zone::Graveyard) for "from a graveyard").
    pub trigger_from_zone: Option<Zone>,
    /// For mana abilities: the mana produced.
    pub mana_produced: Option<Mana>,
    /// For static abilities: continuous effects applied while in play.
    pub static_effects: Vec<StaticEffect>,
}

impl Ability {
    /// Create a new activated ability.
    pub fn activated(
        source_id: ObjectId,
        rules_text: &str,
        costs: Vec<Cost>,
        effects: Vec<Effect>,
        targets: TargetSpec,
    ) -> Self {
        Ability {
            id: AbilityId::new(),
            source_id,
            ability_type: AbilityType::ActivatedNonMana,
            rules_text: rules_text.to_string(),
            active_zones: vec![Zone::Battlefield],
            costs,
            effects,
            targets,
            trigger_events: vec![],
            optional_trigger: false,
            trigger_scope: TriggerScope::SelfOnly,
            triggers_per_turn: 0,
            trigger_from_zone: None,
            mana_produced: None,
            static_effects: vec![],
        }
    }

    /// Create a new triggered ability.
    pub fn triggered(
        source_id: ObjectId,
        rules_text: &str,
        trigger_events: Vec<EventType>,
        effects: Vec<Effect>,
        targets: TargetSpec,
    ) -> Self {
        Ability {
            id: AbilityId::new(),
            source_id,
            ability_type: AbilityType::TriggeredNonMana,
            rules_text: rules_text.to_string(),
            active_zones: vec![Zone::Battlefield],
            costs: vec![],
            effects,
            targets,
            trigger_events,
            optional_trigger: false,
            trigger_scope: TriggerScope::SelfOnly,
            triggers_per_turn: 0,
            trigger_from_zone: None,
            mana_produced: None,
            static_effects: vec![],
        }
    }

    /// Create a new static ability.
    pub fn static_ability(
        source_id: ObjectId,
        rules_text: &str,
        static_effects: Vec<StaticEffect>,
    ) -> Self {
        Ability {
            id: AbilityId::new(),
            source_id,
            ability_type: AbilityType::Static,
            rules_text: rules_text.to_string(),
            active_zones: vec![Zone::Battlefield],
            costs: vec![],
            effects: vec![],
            targets: TargetSpec::None,
            trigger_events: vec![],
            optional_trigger: false,
            trigger_scope: TriggerScope::SelfOnly,
            triggers_per_turn: 0,
            trigger_from_zone: None,
            mana_produced: None,
            static_effects,
        }
    }

    /// Create a mana ability (tap for mana).
    pub fn mana_ability(source_id: ObjectId, rules_text: &str, mana: Mana) -> Self {
        Ability {
            id: AbilityId::new(),
            source_id,
            ability_type: AbilityType::ActivatedMana,
            rules_text: rules_text.to_string(),
            active_zones: vec![Zone::Battlefield],
            costs: vec![Cost::TapSelf],
            effects: vec![Effect::AddMana { mana }],
            targets: TargetSpec::None,
            trigger_events: vec![],
            optional_trigger: false,
            trigger_scope: TriggerScope::SelfOnly,
            triggers_per_turn: 0,
            trigger_from_zone: None,
            mana_produced: Some(mana),
            static_effects: vec![],
        }
    }

    /// Create a spell ability (the ability a spell has on the stack).
    pub fn spell(source_id: ObjectId, effects: Vec<Effect>, targets: TargetSpec) -> Self {
        Ability {
            id: AbilityId::new(),
            source_id,
            ability_type: AbilityType::Spell,
            rules_text: String::new(),
            active_zones: vec![Zone::Stack],
            costs: vec![], // mana cost is on the card, not the ability
            effects,
            targets,
            trigger_events: vec![],
            optional_trigger: false,
            trigger_scope: TriggerScope::SelfOnly,
            triggers_per_turn: 0,
            trigger_from_zone: None,
            mana_produced: None,
            static_effects: vec![],
        }
    }

    /// Check if this ability is a mana ability.
    pub fn is_mana_ability(&self) -> bool {
        self.ability_type == AbilityType::ActivatedMana
    }

    /// Check if this ability uses the stack.
    pub fn uses_stack(&self) -> bool {
        !self.is_mana_ability()
            && self.ability_type != AbilityType::Static
    }

    /// Check if a triggered ability should trigger from an event.
    pub fn should_trigger(&self, event: &GameEvent) -> bool {
        if self.ability_type != AbilityType::TriggeredNonMana {
            return false;
        }
        self.trigger_events.contains(&event.event_type)
    }

    /// Check if an activated ability can be activated in the given zone.
    pub fn can_activate_in_zone(&self, zone: Zone) -> bool {
        self.active_zones.contains(&zone)
    }

    /// Make this a "may" trigger (optional).
    pub fn set_optional(mut self) -> Self {
        self.optional_trigger = true;
        self
    }

    /// Set the active zones for this ability.
    pub fn in_zones(mut self, zones: Vec<Zone>) -> Self {
        self.active_zones = zones;
        self
    }

    /// Set the rules text.
    pub fn with_rules_text(mut self, text: &str) -> Self {
        self.rules_text = text.to_string();
        self
    }
}

// ---------------------------------------------------------------------------
// Common triggered ability builders
// ---------------------------------------------------------------------------

impl Ability {
    /// "When ~ enters the battlefield, [effect]."
    pub fn enters_battlefield_triggered(
        source_id: ObjectId,
        rules_text: &str,
        effects: Vec<Effect>,
        targets: TargetSpec,
    ) -> Self {
        Ability::triggered(
            source_id,
            rules_text,
            vec![EventType::EnteredTheBattlefield],
            effects,
            targets,
        )
    }

    /// "When ~ dies, [effect]."
    pub fn dies_triggered(
        source_id: ObjectId,
        rules_text: &str,
        effects: Vec<Effect>,
        targets: TargetSpec,
    ) -> Self {
        Ability::triggered(
            source_id,
            rules_text,
            vec![EventType::Dies],
            effects,
            targets,
        )
        .in_zones(vec![Zone::Battlefield, Zone::Graveyard])
    }

    /// "Whenever ~ attacks, [effect]."
    pub fn attacks_triggered(
        source_id: ObjectId,
        rules_text: &str,
        effects: Vec<Effect>,
        targets: TargetSpec,
    ) -> Self {
        Ability::triggered(
            source_id,
            rules_text,
            vec![EventType::AttackerDeclared],
            effects,
            targets,
        )
    }

    /// "Whenever a creature you control attacks or blocks, [effect]."
    pub fn controlled_creature_attacks_or_blocks_triggered(
        source_id: ObjectId,
        rules_text: &str,
        effects: Vec<Effect>,
        targets: TargetSpec,
    ) -> Self {
        let mut ab = Ability::triggered(
            source_id,
            rules_text,
            vec![EventType::AttackerDeclared, EventType::BlockerDeclared],
            effects,
            targets,
        );
        ab.trigger_scope = TriggerScope::Any;
        ab
    }

    /// "Whenever ~ deals combat damage to a player, [effect]."
    pub fn combat_damage_to_player_triggered(
        source_id: ObjectId,
        rules_text: &str,
        effects: Vec<Effect>,
        targets: TargetSpec,
    ) -> Self {
        Ability::triggered(
            source_id,
            rules_text,
            vec![EventType::DamagedPlayer],
            effects,
            targets,
        )
    }

    /// "At the beginning of your upkeep, [effect]."
    pub fn beginning_of_upkeep_triggered(
        source_id: ObjectId,
        rules_text: &str,
        effects: Vec<Effect>,
        targets: TargetSpec,
    ) -> Self {
        Ability::triggered(
            source_id,
            rules_text,
            vec![EventType::UpkeepStep],
            effects,
            targets,
        )
    }

    /// "At the beginning of your end step, [effect]."
    pub fn beginning_of_end_step_triggered(
        source_id: ObjectId,
        rules_text: &str,
        effects: Vec<Effect>,
        targets: TargetSpec,
    ) -> Self {
        Ability::triggered(
            source_id,
            rules_text,
            vec![EventType::EndStep],
            effects,
            targets,
        )
    }

    /// "Whenever you cast a spell, [effect]."
    pub fn spell_cast_triggered(
        source_id: ObjectId,
        rules_text: &str,
        effects: Vec<Effect>,
        targets: TargetSpec,
    ) -> Self {
        Ability::triggered(
            source_id,
            rules_text,
            vec![EventType::SpellCast],
            effects,
            targets,
        )
    }

    /// "Whenever another creature enters the battlefield under your control, [effect]."
    pub fn other_creature_etb_triggered(
        source_id: ObjectId,
        rules_text: &str,
        effects: Vec<Effect>,
        targets: TargetSpec,
    ) -> Self {
        let mut ab = Ability::triggered(
            source_id,
            rules_text,
            vec![EventType::EnteredTheBattlefield],
            effects,
            targets,
        );
        ab.trigger_scope = TriggerScope::OtherControlled;
        ab
    }

    pub fn other_creature_etb_from_graveyard_triggered(
        source_id: ObjectId,
        rules_text: &str,
        effects: Vec<Effect>,
        targets: TargetSpec,
    ) -> Self {
        let mut ab = Ability::triggered(
            source_id,
            rules_text,
            vec![EventType::EnteredTheBattlefield],
            effects,
            targets,
        );
        ab.trigger_scope = TriggerScope::OtherControlled;
        ab.trigger_from_zone = Some(Zone::Graveyard);
        ab
    }

    pub fn set_once_per_turn(mut self) -> Self {
        self.triggers_per_turn = 1;
        self
    }

    /// "Whenever a creature dies, [effect]."
    pub fn any_creature_dies_triggered(
        source_id: ObjectId,
        rules_text: &str,
        effects: Vec<Effect>,
        targets: TargetSpec,
    ) -> Self {
        let mut ab = Ability::triggered(
            source_id,
            rules_text,
            vec![EventType::Dies],
            effects,
            targets,
        );
        ab.trigger_scope = TriggerScope::Any;
        ab
    }
}

// ---------------------------------------------------------------------------
// Common one-shot effect constructors
// ---------------------------------------------------------------------------

impl Effect {
    /// "Destroy target creature/permanent."
    pub fn destroy() -> Self {
        Effect::Destroy
    }

    /// "Exile target."
    pub fn exile() -> Self {
        Effect::Exile
    }

    /// "Deal N damage to target."
    pub fn deal_damage(amount: u32) -> Self {
        Effect::DealDamage { amount }
    }

    /// "Draw N cards."
    pub fn draw_cards(count: u32) -> Self {
        Effect::DrawCards { count }
    }

    /// "Gain N life."
    pub fn gain_life(amount: u32) -> Self {
        Effect::GainLife { amount }
    }

    /// "Lose N life."
    pub fn lose_life(amount: u32) -> Self {
        Effect::LoseLife { amount }
    }

    /// "Each opponent loses N life."
    pub fn lose_life_opponents(amount: u32) -> Self {
        Effect::LoseLifeOpponents { amount }
    }

    /// "Target creature gets +N/+M until end of turn."
    pub fn boost_until_eot(power: i32, toughness: i32) -> Self {
        Effect::BoostUntilEndOfTurn { power, toughness }
    }

    pub fn boost_by_toughness_minus_power() -> Self {
        Effect::BoostByToughnessMinusPower
    }

    /// "Target creature gets +N/+M."
    pub fn boost_permanent(power: i32, toughness: i32) -> Self {
        Effect::BoostPermanent { power, toughness }
    }

    /// "Creatures [matching filter] get +N/+M until end of turn."
    pub fn boost_all_eot(filter: &str, power: i32, toughness: i32) -> Self {
        Effect::BoostAllUntilEndOfTurn {
            filter: Filter::parse(filter),
            power,
            toughness,
        }
    }

    /// "Create N token(s)."
    pub fn create_token(token_name: &str, count: u32) -> Self {
        Effect::CreateToken {
            token_name: token_name.to_string(),
            count,
        }
    }

    /// "Create N token(s) that are tapped and attacking. Sacrifice at next end step."
    /// Used by Mobilize.
    pub fn create_token_tapped_attacking(token_name: &str, count: u32) -> Self {
        Effect::CreateTokenTappedAttacking {
            token_name: token_name.to_string(),
            count,
        }
    }

    /// "Counter target spell."
    pub fn counter_spell() -> Self {
        Effect::CounterSpell
    }

    pub fn counter_all_opponent_spells_and_abilities(token_name: &str) -> Self {
        Effect::CounterAllOpponentSpellsAndAbilities { token_name: token_name.to_string() }
    }

    /// "Scry N."
    pub fn scry(count: u32) -> Self {
        Effect::Scry { count }
    }

    /// "Mill N."
    pub fn mill(count: u32) -> Self {
        Effect::Mill { count }
    }

    pub fn mill_and_select(count: u32, filter: &str, destination: &str) -> Self {
        Effect::MillAndSelect { count, filter: filter.to_string(), destination: destination.to_string() }
    }

    pub fn mill_and_return_all(count: u32, filter: &str) -> Self {
        Effect::MillAndReturnAll { count, filter: filter.to_string() }
    }

    /// "Discard N cards."
    pub fn discard_cards(count: u32) -> Self {
        Effect::DiscardCards { count }
    }

    /// "Each opponent discards N cards."
    pub fn discard_opponents(count: u32) -> Self {
        Effect::DiscardOpponents { count }
    }

    /// "Return target to owner's hand."
    pub fn bounce() -> Self {
        Effect::Bounce
    }

    /// "Put target permanent on top of its owner's library."
    pub fn put_on_library() -> Self {
        Effect::PutOnLibrary
    }

    /// "Return target card from graveyard to hand."
    pub fn return_from_graveyard() -> Self {
        Effect::ReturnFromGraveyard
    }

    /// "Return target card from graveyard to battlefield."
    pub fn reanimate() -> Self {
        Effect::Reanimate
    }

    /// "Put N +1/+1 counters on target."
    pub fn add_p1p1_counters(count: u32) -> Self {
        Effect::AddCounters {
            counter_type: "+1/+1".to_string(),
            count,
        }
    }

    /// "Add counters of specified type."
    pub fn add_counters(counter_type: &str, count: u32) -> Self {
        Effect::AddCounters {
            counter_type: counter_type.to_string(),
            count,
        }
    }

    /// "Put counters on this permanent." Always targets the source, even when
    /// the ability has other targets (e.g. compound blight + target haste).
    pub fn add_counters_self(counter_type: &str, count: u32) -> Self {
        Effect::AddCountersSelf {
            counter_type: counter_type.to_string(),
            count,
        }
    }

    /// "Put N counters on each permanent matching filter."
    pub fn add_counters_all(counter_type: &str, count: u32, filter: &str) -> Self {
        Effect::AddCountersAll {
            counter_type: counter_type.to_string(),
            count,
            filter: filter.to_string(),
        }
    }

    /// "Tap target permanent."
    pub fn tap_target() -> Self {
        Effect::TapTarget
    }

    /// "Untap target permanent."
    pub fn untap_target() -> Self {
        Effect::UntapTarget
    }

    /// "Add mana."
    pub fn add_mana(mana: Mana) -> Self {
        Effect::AddMana { mana }
    }

    /// "Gain keyword until end of turn."
    pub fn gain_keyword_eot(keyword: &str) -> Self {
        Effect::GainKeywordUntilEndOfTurn {
            keyword: keyword.to_string(),
        }
    }

    /// "Creatures [matching filter] gain [keyword] until end of turn."
    pub fn grant_keyword_all_eot(filter: &str, keyword: &str) -> Self {
        Effect::GrantKeywordAllUntilEndOfTurn {
            filter: filter.to_string(),
            keyword: keyword.to_string(),
        }
    }

    /// "This creature fights target creature." (mutual damage)
    pub fn fight() -> Self {
        Effect::Fight
    }

    /// "This creature deals damage equal to its power to target." (one-way)
    pub fn bite() -> Self {
        Effect::Bite
    }

    /// "Set power and toughness."
    pub fn set_pt(power: i32, toughness: i32) -> Self {
        Effect::SetPowerToughness { power, toughness }
    }

    /// "Destroy all creatures" (or other filter).
    pub fn destroy_all(filter: &str) -> Self {
        Effect::DestroyAll {
            filter: Filter::parse(filter),
        }
    }

    /// "Deal N damage to each opponent."
    pub fn damage_opponents(amount: u32) -> Self {
        Effect::DealDamageOpponents { amount }
    }

    pub fn damage_opponents_creatures(amount: u32) -> Self {
        Effect::DealDamageOpponentsCreatures { amount }
    }

    /// "Search library for a card."
    pub fn search_library(filter: &str) -> Self {
        Effect::SearchLibrary {
            filter: Filter::parse(filter),
        }
    }

    pub fn search_library_vivid() -> Self {
        Effect::SearchLibraryVivid
    }

    pub fn reveal_from_library_vivid() -> Self {
        Effect::RevealFromLibraryVivid
    }

    /// "Look at top N, may pick one matching filter to hand, rest to bottom."
    pub fn look_top_and_pick(count: u32, filter: &str) -> Self {
        Effect::LookTopAndPick {
            count,
            filter: filter.to_string(),
        }
    }

    /// "Gain control of target."
    pub fn gain_control() -> Self {
        Effect::GainControl
    }

    /// "Gain control of target until end of turn."
    pub fn gain_control_eot() -> Self {
        Effect::GainControlUntilEndOfTurn
    }

    /// "Target gains protection from [quality] until end of turn."
    pub fn gain_protection(from: &str) -> Self {
        Effect::GainProtection {
            from: from.to_string(),
        }
    }

    /// "Target becomes indestructible until end of turn."
    pub fn indestructible() -> Self {
        Effect::Indestructible
    }

    /// "Target gains hexproof until end of turn."
    pub fn hexproof() -> Self {
        Effect::Hexproof
    }

    /// "Choose N of M modes" — modal spell effect.
    pub fn modal(modes: Vec<ModalMode>, min_modes: usize, max_modes: usize) -> Self {
        Effect::Modal { modes, min_modes, max_modes }
    }

    /// Vivid -- Deal damage equal to colors among permanents you control.
    pub fn deal_damage_vivid() -> Self { Effect::DealDamageVivid }
    /// Vivid -- Gain life equal to colors among permanents you control.
    pub fn gain_life_vivid() -> Self { Effect::GainLifeVivid }
    /// Vivid -- Target gets +X/+X until EOT where X = colors among permanents.
    pub fn boost_until_eot_vivid() -> Self { Effect::BoostUntilEotVivid }
    /// Vivid -- Each opponent loses X life.
    pub fn lose_life_opponents_vivid() -> Self { Effect::LoseLifeOpponentsVivid }
    /// Vivid -- Draw X cards.
    pub fn draw_cards_vivid() -> Self { Effect::DrawCardsVivid }
    /// Vivid -- Other creatures get +X/+X until EOT.
    pub fn boost_all_until_eot_vivid() -> Self { Effect::BoostAllUntilEotVivid }
    pub fn create_token_vivid(token_name: &str) -> Self { Effect::CreateTokenVivid { token_name: token_name.to_string() } }

    /// "You may pay [cost]. If you do, [effects]. Otherwise, [else_effects]."
    pub fn do_if_cost_paid(cost: Cost, if_paid: Vec<Effect>, if_not_paid: Vec<Effect>) -> Self {
        Effect::DoIfCostPaid { cost, if_paid, if_not_paid }
    }

    /// "As this permanent enters, choose a creature type." (any type)
    pub fn choose_creature_type() -> Self {
        Effect::ChooseCreatureType { restricted: vec![] }
    }

    /// "As this permanent enters, choose a color."
    pub fn choose_color() -> Self {
        Effect::ChooseColor
    }

    /// "As this permanent enters, choose [list of types]."
    pub fn choose_creature_type_restricted(types: Vec<&str>) -> Self {
        Effect::ChooseCreatureType { restricted: types.into_iter().map(|s| s.to_string()).collect() }
    }

    /// "Attach this Equipment to target creature you control."
    pub fn equip() -> Self {
        Effect::Equip
    }

    /// "Choose a creature type. Draw a card for each permanent you control of that type."
    pub fn choose_type_and_draw_per_permanent() -> Self {
        Effect::ChooseTypeAndDrawPerPermanent
    }

    /// "Choose a creature type. Return all creature cards of the chosen type from your graveyard to the battlefield."
    pub fn choose_type_and_return_from_graveyard() -> Self {
        Effect::ChooseTypeAndReturnFromGraveyard
    }

    /// "Choose a creature type. Other permanents you control of the chosen type gain [keywords] until end of turn."
    pub fn choose_type_and_grant_keywords(keywords: Vec<&str>, other_only: bool) -> Self {
        Effect::ChooseTypeAndGrantKeywords {
            keywords: keywords.into_iter().map(|s| s.to_string()).collect(),
            other_only,
        }
    }

    /// "When [target/source] dies this turn, [effects]."
    pub fn delayed_on_death(effects: Vec<Effect>) -> Self {
        Effect::CreateDelayedTrigger {
            event_type: "dies".into(),
            trigger_effects: effects,
            duration: "end_of_turn".into(),
            watch_target: true,
        }
    }

    /// "At the beginning of the next end step, [effects]."
    pub fn at_next_end_step(effects: Vec<Effect>) -> Self {
        Effect::CreateDelayedTrigger {
            event_type: "end_step".into(),
            trigger_effects: effects,
            duration: "until_triggered".into(),
            watch_target: false,
        }
    }

    pub fn deal_damage_with_delayed_exile() -> Self {
        Effect::DealDamageWithDelayedExile
    }

    /// "Exile the top N cards. You may play them until end of turn."
    pub fn exile_top_and_play(count: u32) -> Self {
        Effect::ExileTopAndPlay { count, duration: "end_of_turn".into(), without_mana: false }
    }

    /// "Exile the top N cards. You may play them until end of your next turn."
    pub fn exile_top_and_play_next_turn(count: u32) -> Self {
        Effect::ExileTopAndPlay { count, duration: "until_end_of_next_turn".into(), without_mana: false }
    }

    /// "Exile the top N cards. You may play them without paying their mana cost until end of turn."
    pub fn exile_top_and_play_free(count: u32) -> Self {
        Effect::ExileTopAndPlay { count, duration: "end_of_turn".into(), without_mana: true }
    }

    /// Return cards exiled by this source to their owners' hands.
    pub fn return_exiled_to_hand() -> Self {
        Effect::ReturnExiledToHand
    }

    /// Untap all permanents matching a filter.
    pub fn untap_all(filter: &str) -> Self {
        Effect::UntapAll { filter: filter.to_string() }
    }

    /// "This creature can't be blocked this turn."
    pub fn cant_be_blocked_eot() -> Self {
        Effect::CantBeBlockedUntilEot
    }

    pub fn tap_attached() -> Self {
        Effect::TapAttached
    }

    pub fn proliferate() -> Self {
        Effect::Proliferate
    }

    pub fn remove_all_counters() -> Self {
        Effect::RemoveAllCounters
    }

    pub fn exile_from_graveyards(count: u32) -> Self {
        Effect::ExileTargetCardsFromGraveyards { count }
    }

    pub fn exile_target_to_source_zone() -> Self {
        Effect::ExileTargetToSourceZone
    }

    pub fn flicker() -> Self {
        Effect::Flicker
    }

    pub fn flicker_end_step() -> Self {
        Effect::FlickerEndStep
    }

    pub fn opponent_exiles_from_hand(count: u32) -> Self {
        Effect::OpponentExilesFromHand { count }
    }

    pub fn blight_opponents(count: u32) -> Self {
        Effect::BlightOpponents { count }
    }

    pub fn gain_all_creature_types() -> Self {
        Effect::GainAllCreatureTypes
    }

    pub fn become_all_colors() -> Self {
        Effect::BecomeAllColors
    }

    pub fn bounce_all(filter: &str) -> Self {
        Effect::BounceAll { filter: filter.to_string() }
    }

    pub fn exile_from_opponent_library(count: u32) -> Self {
        Effect::ExileFromOpponentLibrary { count }
    }

    pub fn exile_from_opponent_library_to_source_zone(count: u32) -> Self {
        Effect::ExileFromOpponentLibraryToSourceZone { count }
    }

    /// Create a token copy of target creature.
    pub fn create_token_copy(count: u32) -> Self {
        Effect::CreateTokenCopy { count, modifications: vec![] }
    }

    /// Create a token copy with haste.
    pub fn create_token_copy_with_haste(count: u32) -> Self {
        Effect::CreateTokenCopy {
            count,
            modifications: vec![TokenModification::AddKeyword("haste".into())],
        }
    }

    /// Create a token copy with changeling (all creature types).
    pub fn create_token_copy_with_changeling(count: u32) -> Self {
        Effect::CreateTokenCopy {
            count,
            modifications: vec![TokenModification::AddChangeling],
        }
    }

    /// Create a token copy with haste that's sacrificed at end step.
    pub fn create_token_copy_haste_sacrifice(count: u32) -> Self {
        Effect::CreateTokenCopy {
            count,
            modifications: vec![
                TokenModification::AddKeyword("haste".into()),
                TokenModification::SacrificeAtEndStep,
            ],
        }
    }

    pub fn create_token_copy_of_triggering() -> Self {
        Effect::CreateTokenCopyOfTriggering
    }

    /// Tap the source permanent.
    pub fn tap_self() -> Self {
        Effect::TapSelf
    }

    /// Return all creatures of a type from your graveyard to the battlefield.
    pub fn return_all_type_from_graveyard(creature_type: &str) -> Self {
        Effect::ReturnAllTypeFromGraveyard { creature_type: creature_type.to_string() }
    }

    /// Create X tokens where X is dynamically counted from a filter.
    pub fn create_token_dynamic(token_name: &str, count_filter: &str) -> Self {
        Effect::CreateTokenDynamic {
            token_name: token_name.to_string(),
            count_filter: count_filter.to_string(),
        }
    }

    /// Gain life equal to a dynamic value.
    pub fn gain_life_dynamic(value_source: &str) -> Self {
        Effect::GainLifeDynamic { value_source: value_source.to_string() }
    }

    /// Target gets +X/+X until end of turn where X = dynamic value.
    pub fn boost_target_dynamic(value_source: &str) -> Self {
        Effect::BoostTargetDynamic { value_source: value_source.to_string() }
    }

    /// First target gets +X/+0, second target gets -0/-X where X = dynamic value.
    pub fn boost_dual_target_dynamic(value_source: &str) -> Self {
        Effect::BoostDualTargetDynamic { value_source: value_source.to_string() }
    }

    /// Target's controller draws N cards.
    pub fn target_controller_draws(count: u32) -> Self {
        Effect::TargetControllerDraws { count }
    }

    /// Target's controller creates a token.
    pub fn target_controller_creates_token(token_name: &str) -> Self {
        Effect::TargetControllerCreatesToken { token_name: token_name.to_string() }
    }

    /// Target creature loses all abilities.
    pub fn lose_all_abilities() -> Self {
        Effect::LoseAllAbilities
    }

    /// Set base P/T of all creatures matching a filter.
    pub fn set_base_pt_all(power: i32, toughness: i32, filter: &str) -> Self {
        Effect::SetBasePowerToughnessAll {
            power,
            toughness,
            filter: filter.to_string(),
        }
    }

    /// Remove all abilities from all creatures matching a filter.
    pub fn lose_all_abilities_all(filter: &str) -> Self {
        Effect::LoseAllAbilitiesAll {
            filter: filter.to_string(),
        }
    }

    pub fn add_subtype_all(subtype: &str, filter: &str) -> Self {
        Effect::AddSubtypeAll {
            subtype: subtype.to_string(),
            filter: filter.to_string(),
        }
    }

    pub fn set_subtypes_self(subtypes: Vec<&str>) -> Self {
        Effect::SetSubtypesSelf {
            subtypes: subtypes.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn put_from_hand_with_haste_sacrifice(max_mv: u32) -> Self {
        Effect::PutFromHandToBattlefield {
            max_mana_value: max_mv,
            max_mv_dynamic: None,
            tapped: false,
            attacking: false,
            haste: true,
            sacrifice_eot: true,
        }
    }

    pub fn put_from_hand_tapped_attacking(max_mv: u32) -> Self {
        Effect::PutFromHandToBattlefield {
            max_mana_value: max_mv,
            max_mv_dynamic: None,
            tapped: true,
            attacking: true,
            haste: false,
            sacrifice_eot: false,
        }
    }

    pub fn becomes_creature(power: i32, toughness: i32) -> Self {
        Effect::BecomesCreature { power, toughness }
    }

    pub fn put_from_hand_tapped_attacking_dynamic(dynamic_source: &str) -> Self {
        Effect::PutFromHandToBattlefield {
            max_mana_value: 0,
            max_mv_dynamic: Some(dynamic_source.to_string()),
            tapped: true,
            attacking: true,
            haste: false,
            sacrifice_eot: false,
        }
    }

    pub fn transform_self() -> Self {
        Effect::TransformSelf
    }

    pub fn conditional(condition: &str, if_true: Vec<Effect>, if_false: Vec<Effect>) -> Self {
        Effect::Conditional {
            condition: condition.to_string(),
            if_true,
            if_false,
        }
    }

    pub fn compare_and_boost() -> Self {
        Effect::CompareAndBoost
    }

    pub fn if_resolved_n_times(n: u32, effects: Vec<Effect>) -> Self {
        Effect::IfAbilityResolvedNTimes {
            resolution_number: n,
            effects,
        }
    }

    pub fn grant_triggered_ability_eot(event_type: &str, filter: &str, trigger_effects: Vec<Effect>) -> Self {
        Effect::GrantTriggeredAbilityUntilEOT {
            event_type: event_type.to_string(),
            filter: filter.to_string(),
            trigger_effects,
        }
    }

    pub fn copy_next_spell() -> Self {
        Effect::CopyNextSpell
    }

    pub fn copy_triggering_spell(keywords: Vec<&str>, single_target_only: bool) -> Self {
        Effect::CopyTriggeringSpell {
            keywords: keywords.into_iter().map(|s| s.to_string()).collect(),
            single_target_only,
        }
    }

    pub fn opponent_reveals_from_hand_exile_cast(count_source: &str, instant_sorcery_only: bool) -> Self {
        Effect::OpponentRevealsFromHandExileCast {
            count_source: count_source.to_string(),
            instant_sorcery_only,
        }
    }

    pub fn opponents_exile_until_mv_and_cast(mv_threshold: u32) -> Self {
        Effect::OpponentsExileUntilMVAndCast { mv_threshold }
    }

    pub fn winnowing() -> Self {
        Effect::Winnowing
    }

    pub fn mass_become_copy() -> Self {
        Effect::MassBecomeCopy
    }

    pub fn exile_with_dream_counter() -> Self {
        Effect::ExileWithDreamCounterInsteadOfGraveyard
    }

    pub fn cast_from_exile_with_dream_counters() -> Self {
        Effect::CastFromExileWithDreamCounters
    }
}

impl ModalMode {
    /// Create a new modal mode.
    pub fn new(description: &str, effects: Vec<Effect>) -> Self {
        ModalMode {
            description: description.to_string(),
            effects,
        }
    }
}

// ---------------------------------------------------------------------------
// Common static effect builders
// ---------------------------------------------------------------------------

impl StaticEffect {
    /// "Other creatures you control get +N/+M." (Lord effect)
    pub fn boost_controlled(filter: &str, power: i32, toughness: i32) -> Self {
        StaticEffect::Boost {
            filter: filter.to_string(),
            power,
            toughness,
        }
    }

    /// "Creatures you control have [keyword]."
    pub fn grant_keyword_controlled(filter: &str, keyword: &str) -> Self {
        StaticEffect::GrantKeyword {
            filter: filter.to_string(),
            keyword: keyword.to_string(),
        }
    }

    /// "Creatures you control can't be blocked" (or specific CantBlock variant).
    pub fn cant_block(filter: &str) -> Self {
        StaticEffect::CantBlock {
            filter: filter.to_string(),
        }
    }

    /// "Creatures you control can't attack."
    pub fn cant_attack(filter: &str) -> Self {
        StaticEffect::CantAttack {
            filter: filter.to_string(),
        }
    }

    /// "[Spell type] spells you cast cost {N} less."
    pub fn cost_reduction(filter: &str, amount: u32) -> Self {
        StaticEffect::CostReduction {
            filter: filter.to_string(),
            amount,
            condition: None,
        }
    }

    pub fn cost_reduction_dynamic(filter: &str, value_source: &str) -> Self {
        StaticEffect::CostReductionDynamic {
            filter: filter.to_string(),
            value_source: value_source.to_string(),
        }
    }

    /// "Creature spells with toughness > power cost {N} less."
    pub fn cost_reduction_if_toughness_greater(filter: &str, amount: u32) -> Self {
        StaticEffect::CostReduction {
            filter: filter.to_string(),
            amount,
            condition: Some("toughness_greater_than_power".into()),
        }
    }

    /// "Ward {cost}" — counter targeting spells/abilities unless opponent pays cost.
    pub fn ward(cost: &str) -> Self {
        StaticEffect::Ward {
            cost: cost.to_string(),
        }
    }

    /// "This land enters tapped unless [condition]."
    pub fn enters_tapped_unless(condition: &str) -> Self {
        StaticEffect::EntersTappedUnless {
            condition: condition.to_string(),
        }
    }
    pub fn evoke(cost: &str) -> Self {
        StaticEffect::Evoke {
            cost: cost.to_string(),
        }
    }

    /// Enchanted/matching creature loses all abilities (continuous).
    pub fn lose_all_abilities(filter: &str) -> Self {
        StaticEffect::LoseAllAbilities {
            filter: filter.to_string(),
        }
    }

    /// Set base P/T of matching permanents (continuous Layer 7b).
    pub fn set_base_pt(filter: &str, power: i32, toughness: i32) -> Self {
        StaticEffect::SetBasePowerToughness {
            filter: filter.to_string(),
            power,
            toughness,
        }
    }

    pub fn becomes_creature_attached(subtypes: &[&str], colorless: bool) -> Self {
        StaticEffect::BecomesCreatureAttached {
            subtypes: subtypes.iter().map(|s| s.to_string()).collect(),
            colorless,
        }
    }

    /// Matching permanents can't untap during their controller's untap step.
    pub fn cant_untap(filter: &str) -> Self {
        StaticEffect::CantUntap {
            filter: filter.to_string(),
        }
    }

    /// Set this creature's base power to the number of colors among permanents you control (Vivid).
    pub fn set_power_to_color_count() -> Self {
        StaticEffect::SetPowerToColorCount
    }

    /// Matching creature assigns combat damage equal to toughness (unconditional).
    pub fn assign_damage_with_toughness(filter: &str) -> Self {
        StaticEffect::AssignDamageWithToughness {
            filter: filter.to_string(),
            condition: None,
        }
    }

    /// Matching creature assigns combat damage equal to toughness, but only when toughness > power.
    pub fn assign_damage_with_toughness_if_greater(filter: &str) -> Self {
        StaticEffect::AssignDamageWithToughness {
            filter: filter.to_string(),
            condition: Some("toughness_greater_than_power".to_string()),
        }
    }

    pub fn grant_convoke(filter: &str) -> Self {
        StaticEffect::GrantConvoke {
            filter: filter.to_string(),
        }
    }

    pub fn damage_doubling_from_type() -> Self {
        StaticEffect::DamageDoublingFromType
    }

    pub fn mana_doubling_basic_lands() -> Self {
        StaticEffect::ManaDoublingBasicLands
    }

    pub fn enhanced_mana_production() -> Self {
        StaticEffect::EnhancedManaProduction
    }

    pub fn trigger_doubling(filter: &str) -> Self {
        StaticEffect::TriggerDoubling {
            filter: filter.to_string(),
        }
    }

    pub fn grant_conspire(filter: &str) -> Self {
        StaticEffect::GrantConspire {
            filter: filter.to_string(),
        }
    }

    pub fn enters_with_counters(counter_type: &str, count: u32) -> Self {
        StaticEffect::EntersWithCounters {
            counter_type: counter_type.to_string(),
            count,
        }
    }

    pub fn enter_as_a_copy(filter: &str, add_keywords: &[&str]) -> Self {
        StaticEffect::EnterAsACopy {
            filter: filter.to_string(),
            add_keywords: add_keywords.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn cast_from_exile_with_counter_cost(counter_count: u32) -> Self {
        StaticEffect::CastFromExileWithCounterCost { counter_count }
    }

    pub fn boost_per_turn_event(filter: &str, event: &str, power_per: i32, toughness_per: i32) -> Self {
        StaticEffect::BoostPerTurnEvent {
            filter: filter.to_string(),
            event: event.to_string(),
            power_per,
            toughness_per,
        }
    }

    pub fn cast_exiled_once_per_turn(mv_count_filter: &str) -> Self {
        StaticEffect::CastExiledOncePerTurn {
            mv_count_filter: mv_count_filter.to_string(),
        }
    }

    pub fn replace_token_creation() -> Self {
        StaticEffect::ReplaceTokenCreation
    }

    pub fn hexproof_from_own_colors() -> Self {
        StaticEffect::HexproofFromOwnColors
    }
}

// ---------------------------------------------------------------------------
// Common target spec builders
// ---------------------------------------------------------------------------

impl TargetSpec {
    /// Fight/Bite targeting: "target creature you control" + "target creature you don't control".
    pub fn fight_targets() -> Self {
        TargetSpec::Pair {
            first: Box::new(TargetSpec::CreatureYouControl),
            second: Box::new(TargetSpec::OpponentCreature),
        }
    }
}

// ---------------------------------------------------------------------------
// Common cost builders
// ---------------------------------------------------------------------------

impl Cost {
    /// Pay mana cost from a string like "{2}{B}".
    pub fn pay_mana(mana_str: &str) -> Self {
        use crate::mana::ManaCost;
        Cost::Mana(ManaCost::parse(mana_str).to_mana())
    }

    /// Tap this permanent ({T}).
    pub fn tap_self() -> Self {
        Cost::TapSelf
    }

    /// Sacrifice this permanent.
    pub fn sacrifice_self() -> Self {
        Cost::SacrificeSelf
    }

    /// Sacrifice another permanent matching a description.
    pub fn sacrifice_other(filter: &str) -> Self {
        Cost::SacrificeOther(filter.to_string())
    }

    /// Pay N life.
    pub fn pay_life(amount: u32) -> Self {
        Cost::PayLife(amount)
    }

    /// Discard N cards.
    pub fn discard(count: u32) -> Self {
        Cost::Discard(count)
    }

    /// Exile N cards from hand.
    pub fn exile_from_hand(count: u32) -> Self {
        Cost::ExileFromHand(count)
    }

    /// Exile N cards from graveyard.
    pub fn exile_from_graveyard(count: u32) -> Self {
        Cost::ExileFromGraveyard(count)
    }

    /// Remove N counters of a type from this permanent.
    pub fn remove_counters(counter_type: &str, count: u32) -> Self {
        Cost::RemoveCounters(counter_type.to_string(), count)
    }

    /// Blight N — put N -1/-1 counters on a creature you control.
    pub fn blight(count: u32) -> Self {
        Cost::Blight(count)
    }

    /// Variable Blight X — choose X, put X -1/-1 counters on a creature you control.
    pub fn variable_blight() -> Self {
        Cost::VariableBlight
    }

    /// Reveal a card of a specific type from hand.
    pub fn reveal_from_hand(card_type: &str) -> Self {
        Cost::RevealFromHand(card_type.to_string())
    }

    /// Behold a creature type (reveal from hand or choose from battlefield).
    pub fn behold(creature_type: &str) -> Self {
        Cost::Behold(creature_type.to_string())
    }

    /// Behold a creature type and exile the chosen card/permanent.
    pub fn behold_and_exile(creature_type: &str) -> Self {
        Cost::BeholdAndExile(creature_type.to_string())
    }

    /// Behold a creature type, or pay alternative mana.
    pub fn behold_or_pay(creature_type: &str, mana_str: &str) -> Self {
        use crate::mana::ManaCost;
        Cost::BeholdOrPay {
            creature_type: creature_type.to_string(),
            mana: ManaCost::parse(mana_str).to_mana(),
        }
    }

    /// Tap N other creatures matching a filter.
    pub fn tap_creatures(filter: &str, count: u32) -> Self {
        Cost::TapCreatures { filter: Filter::parse(filter), count }
    }
}

// ---------------------------------------------------------------------------
// Static (continuous) effects
// ---------------------------------------------------------------------------

/// A continuous effect generated by a static ability.
///
/// These are applied in the 7-layer system each time the game state is
/// recalculated (see effects.rs for Layer enum).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StaticEffect {
    /// Boost P/T of matching permanents.
    Boost {
        filter: String,
        power: i32,
        toughness: i32,
    },
    /// Grant a keyword to matching permanents.
    GrantKeyword {
        filter: String,
        keyword: String,
    },
    /// Remove a keyword from matching permanents.
    RemoveKeyword {
        filter: String,
        keyword: String,
    },
    /// Prevent matching permanents from attacking.
    CantAttack {
        filter: String,
    },
    /// Prevent matching permanents from blocking.
    CantBlock {
        filter: String,
    },
    /// Reduce cost of matching spells.
    CostReduction {
        filter: String,
        amount: u32,
        condition: Option<String>,
    },
    /// Reduce cost dynamically: "greatest mana value among [type] you control".
    CostReductionDynamic {
        filter: String,
        value_source: String,
    },
    /// Matching permanents enter the battlefield tapped.
    EntersTapped {
        filter: String,
    },
    /// Other players can't gain life.
    CantGainLife,
    /// Other players can't draw extra cards.
    CantDrawExtraCards,
    /// This spell can't be countered.
    CantBeCountered,
    /// Ward — when this becomes the target of a spell or ability an opponent
    /// controls, counter it unless that player pays the specified cost.
    Ward {
        cost: String,
    },
    /// Enters tapped unless a condition is met (e.g. "you control a Plains or an Island").
    EntersTappedUnless {
        condition: String,
    },
    /// Evoke — alternative casting cost. When evoked creature enters, sacrifice it.
    Evoke {
        cost: String,
    },
    /// This creature can't be blocked by more than N creatures.
    CantBeBlockedByMoreThan {
        count: u32,
    },
    /// This creature can't be blocked by creatures with power less than or equal to N (daunt).
    CantBeBlockedByPowerLessOrEqual {
        power: i32,
    },
    /// This creature must be blocked if able.
    MustBeBlocked,
    /// Spells you control can't be countered.
    SpellsCantBeCountered,
    /// Dynamic P/T boost: "gets +P/+T for each [count_filter]".
    /// Counts matching permanents on the battlefield, optionally also counts
    /// matching cards in controller's graveyard (when count_filter contains
    /// "and [type] card in your graveyard").
    /// Grant additional land plays per turn to the controller.
    AdditionalLandPlays { count: u32 },
    /// Conditional keyword: grant a keyword to self only when a condition is met.
    /// Conditions: "your turn", "untapped", "you control a {type}", "creature entered this turn"
    ConditionalKeyword {
        keyword: String,
        condition: String,
    },
    /// Conditional P/T boost on self when a condition is met.
    ConditionalBoostSelf {
        power: i32,
        toughness: i32,
        condition: String,
    },
    BoostPerCount {
        count_filter: String,
        power_per: i32,
        toughness_per: i32,
    },
    /// Target/enchanted creature loses all abilities (continuous version for auras).
    LoseAllAbilities {
        filter: String,
    },
    /// Set base power and toughness of matching permanents (Layer 7b continuous override).
    SetBasePowerToughness {
        filter: String,
        power: i32,
        toughness: i32,
    },
    /// Prevent matching permanents from untapping during their controller's untap step.
    CantUntap {
        filter: String,
    },
    /// Set this creature's base power to the number of colors among permanents you control (Vivid).
    SetPowerToColorCount,
    /// Matching creature assigns combat damage equal to its toughness rather than its power.
    /// If `condition` is set (e.g. "toughness_greater_than_power"), only applies when condition is met.
    AssignDamageWithToughness {
        filter: String,
        condition: Option<String>,
    },
    /// Grant convoke to matching spells the controller casts.
    GrantConvoke {
        filter: String,
    },
    /// Double all damage that sources the controller controls of the chosen creature type would deal.
    DamageDoublingFromType,
    /// Basic lands tapped for mana produce one additional mana of the same type (all players).
    ManaDoublingBasicLands,
    /// Enchanted land tapped for mana produces additional mana of the aura's chosen color.
    EnhancedManaProduction,
    /// Triggered abilities of matching permanents the controller controls trigger an additional time.
    TriggerDoubling {
        filter: String,
    },
    /// Grant conspire to matching spells the controller casts.
    GrantConspire {
        filter: String,
    },
    /// This permanent enters the battlefield with counters on it (replacement effect).
    EntersWithCounters {
        counter_type: String,
        count: u32,
    },
    EnterAsACopy {
        filter: String,
        add_keywords: Vec<String>,
    },
    /// During your turn, you may cast creature spells from cards exiled with this source
    /// by removing N counters from among creatures you control as an additional cost.
    CastFromExileWithCounterCost {
        counter_count: u32,
    },
    BoostPerTurnEvent {
        filter: String,
        event: String,
        power_per: i32,
        toughness_per: i32,
    },
    /// Once each turn, you may cast a spell from this source's exile zone without paying
    /// its mana cost if its mana value is <= the count of permanents matching the filter.
    CastExiledOncePerTurn {
        mv_count_filter: String,
    },
    ReplaceTokenCreation,
    BecomesCreatureAttached {
        subtypes: Vec<String>,
        colorless: bool,
    },
    HexproofFromOwnColors,
    Custom(String),
}

// ---------------------------------------------------------------------------
// AbilityStore — stores abilities by source
// ---------------------------------------------------------------------------

/// Stores all abilities for all objects in the game.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AbilityStore {
    /// All abilities, keyed by their unique AbilityId.
    abilities: std::collections::HashMap<AbilityId, Ability>,
    /// Index: source ObjectId → list of AbilityIds.
    by_source: std::collections::HashMap<ObjectId, Vec<AbilityId>>,
}

impl AbilityStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register an ability for a source object.
    pub fn add(&mut self, ability: Ability) {
        let id = ability.id;
        let source = ability.source_id;
        self.abilities.insert(id, ability);
        self.by_source.entry(source).or_default().push(id);
    }

    /// Get an ability by its ID.
    pub fn get(&self, id: AbilityId) -> Option<&Ability> {
        self.abilities.get(&id)
    }

    /// Get all abilities for a source object.
    pub fn for_source(&self, source_id: ObjectId) -> Vec<&Ability> {
        self.by_source
            .get(&source_id)
            .map(|ids| ids.iter().filter_map(|id| self.abilities.get(id)).collect())
            .unwrap_or_default()
    }

    /// Get all triggered abilities that should fire for an event.
    pub fn triggered_by(&self, event: &GameEvent) -> Vec<&Ability> {
        self.abilities
            .values()
            .filter(|a| a.should_trigger(event))
            .collect()
    }

    /// Get all mana abilities for a source.
    pub fn mana_abilities_for(&self, source_id: ObjectId) -> Vec<&Ability> {
        self.for_source(source_id)
            .into_iter()
            .filter(|a| a.is_mana_ability())
            .collect()
    }

    /// Remove all abilities for a source (e.g. when permanent leaves battlefield).
    pub fn remove_source(&mut self, source_id: ObjectId) {
        if let Some(ids) = self.by_source.remove(&source_id) {
            for id in ids {
                self.abilities.remove(&id);
            }
        }
    }

    /// Total number of registered abilities.
    pub fn len(&self) -> usize {
        self.abilities.len()
    }

    pub fn is_empty(&self) -> bool {
        self.abilities.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EventType;
    use crate::types::PlayerId;

    #[test]
    fn activated_ability() {
        let source = ObjectId::new();
        let ability = Ability::activated(
            source,
            "{2}, {T}: Draw a card.",
            vec![Cost::Mana(Mana::generic(2)), Cost::TapSelf],
            vec![Effect::DrawCards { count: 1 }],
            TargetSpec::None,
        );

        assert_eq!(ability.ability_type, AbilityType::ActivatedNonMana);
        assert_eq!(ability.costs.len(), 2);
        assert_eq!(ability.effects.len(), 1);
        assert!(ability.uses_stack());
    }

    #[test]
    fn triggered_ability() {
        let source = ObjectId::new();
        let ability = Ability::triggered(
            source,
            "When this creature enters the battlefield, draw a card.",
            vec![EventType::EnteredTheBattlefield],
            vec![Effect::DrawCards { count: 1 }],
            TargetSpec::None,
        );

        assert_eq!(ability.ability_type, AbilityType::TriggeredNonMana);
        assert!(ability.uses_stack());

        // Check trigger matching
        let event = GameEvent::enters_battlefield(source, PlayerId::new());
        assert!(ability.should_trigger(&event));

        // Unrelated event should not trigger
        let unrelated = GameEvent::new(EventType::DamagePlayer).target(source);
        assert!(!ability.should_trigger(&unrelated));
    }

    #[test]
    fn mana_ability() {
        let source = ObjectId::new();
        let ability = Ability::mana_ability(
            source,
            "{T}: Add {G}.",
            Mana::green(1),
        );

        assert_eq!(ability.ability_type, AbilityType::ActivatedMana);
        assert!(ability.is_mana_ability());
        assert!(!ability.uses_stack());
        assert_eq!(ability.mana_produced, Some(Mana::green(1)));
    }

    #[test]
    fn static_ability_boost() {
        let source = ObjectId::new();
        let ability = Ability::static_ability(
            source,
            "Other creatures you control get +1/+1.",
            vec![StaticEffect::Boost {
                filter: "other creatures you control".to_string(),
                power: 1,
                toughness: 1,
            }],
        );

        assert_eq!(ability.ability_type, AbilityType::Static);
        assert!(!ability.uses_stack());
        assert_eq!(ability.static_effects.len(), 1);
    }

    #[test]
    fn spell_ability() {
        let source = ObjectId::new();
        let ability = Ability::spell(
            source,
            vec![Effect::DealDamage { amount: 3 }],
            TargetSpec::CreatureOrPlayer,
        );

        assert_eq!(ability.ability_type, AbilityType::Spell);
        assert!(ability.uses_stack());
    }

    #[test]
    fn ability_store() {
        let mut store = AbilityStore::new();
        let source = ObjectId::new();

        let a1 = Ability::mana_ability(source, "{T}: Add {G}.", Mana::green(1));
        let a1_id = a1.id;
        let a2 = Ability::activated(
            source,
            "{1}{G}: +1/+1",
            vec![Cost::Mana(Mana { green: 1, generic: 1, ..Default::default() })],
            vec![Effect::BoostUntilEndOfTurn { power: 1, toughness: 1 }],
            TargetSpec::None,
        );

        store.add(a1);
        store.add(a2);

        assert_eq!(store.len(), 2);
        assert_eq!(store.for_source(source).len(), 2);
        assert_eq!(store.mana_abilities_for(source).len(), 1);
        assert!(store.get(a1_id).is_some());

        store.remove_source(source);
        assert!(store.is_empty());
    }

    #[test]
    fn optional_trigger() {
        let source = ObjectId::new();
        let ability = Ability::triggered(
            source,
            "When ~ enters, you may draw a card.",
            vec![EventType::EnteredTheBattlefield],
            vec![Effect::DrawCards { count: 1 }],
            TargetSpec::None,
        ).set_optional();

        assert!(ability.optional_trigger);
    }

    #[test]
    fn active_zones() {
        let source = ObjectId::new();
        let ability = Ability::activated(
            source,
            "Exile from graveyard: effect",
            vec![],
            vec![],
            TargetSpec::None,
        ).in_zones(vec![Zone::Graveyard]);

        assert!(ability.can_activate_in_zone(Zone::Graveyard));
        assert!(!ability.can_activate_in_zone(Zone::Battlefield));
    }

    // ── Tests for common builders ──────────────────────────────────────

    #[test]
    fn etb_triggered() {
        let source = ObjectId::new();
        let ability = Ability::enters_battlefield_triggered(
            source,
            "When ~ enters, draw a card.",
            vec![Effect::draw_cards(1)],
            TargetSpec::None,
        );

        assert_eq!(ability.ability_type, AbilityType::TriggeredNonMana);
        assert!(ability.trigger_events.contains(&EventType::EnteredTheBattlefield));
        assert_eq!(ability.effects.len(), 1);
    }

    #[test]
    fn dies_triggered() {
        let source = ObjectId::new();
        let ability = Ability::dies_triggered(
            source,
            "When ~ dies, each opponent loses 1 life.",
            vec![Effect::damage_opponents(1)],
            TargetSpec::None,
        );

        assert!(ability.trigger_events.contains(&EventType::Dies));
        // Dies triggers work from battlefield and graveyard
        assert!(ability.active_zones.contains(&Zone::Battlefield));
        assert!(ability.active_zones.contains(&Zone::Graveyard));
    }

    #[test]
    fn attacks_triggered() {
        let source = ObjectId::new();
        let ability = Ability::attacks_triggered(
            source,
            "Whenever ~ attacks, draw a card.",
            vec![Effect::draw_cards(1)],
            TargetSpec::None,
        );

        assert!(ability.trigger_events.contains(&EventType::AttackerDeclared));
    }

    #[test]
    fn combat_damage_triggered() {
        let source = ObjectId::new();
        let ability = Ability::combat_damage_to_player_triggered(
            source,
            "Whenever ~ deals combat damage to a player, draw a card.",
            vec![Effect::draw_cards(1)],
            TargetSpec::None,
        );

        assert!(ability.trigger_events.contains(&EventType::DamagedPlayer));
    }

    #[test]
    fn upkeep_triggered() {
        let source = ObjectId::new();
        let ability = Ability::beginning_of_upkeep_triggered(
            source,
            "At the beginning of your upkeep, gain 1 life.",
            vec![Effect::gain_life(1)],
            TargetSpec::None,
        );

        assert!(ability.trigger_events.contains(&EventType::UpkeepStep));
    }

    #[test]
    fn end_step_triggered() {
        let source = ObjectId::new();
        let ability = Ability::beginning_of_end_step_triggered(
            source,
            "At the beginning of your end step, create a 1/1 token.",
            vec![Effect::create_token("Soldier", 1)],
            TargetSpec::None,
        );

        assert!(ability.trigger_events.contains(&EventType::EndStep));
    }

    #[test]
    fn effect_builders() {
        // Test various effect constructors
        match Effect::deal_damage(3) {
            Effect::DealDamage { amount } => assert_eq!(amount, 3),
            _ => panic!("wrong variant"),
        }

        match Effect::draw_cards(2) {
            Effect::DrawCards { count } => assert_eq!(count, 2),
            _ => panic!("wrong variant"),
        }

        match Effect::gain_life(5) {
            Effect::GainLife { amount } => assert_eq!(amount, 5),
            _ => panic!("wrong variant"),
        }

        match Effect::boost_until_eot(2, 2) {
            Effect::BoostUntilEndOfTurn { power, toughness } => {
                assert_eq!(power, 2);
                assert_eq!(toughness, 2);
            }
            _ => panic!("wrong variant"),
        }

        match Effect::create_token("Zombie", 3) {
            Effect::CreateToken { token_name, count } => {
                assert_eq!(token_name, "Zombie");
                assert_eq!(count, 3);
            }
            _ => panic!("wrong variant"),
        }

        match Effect::add_p1p1_counters(2) {
            Effect::AddCounters { counter_type, count } => {
                assert_eq!(counter_type, "+1/+1");
                assert_eq!(count, 2);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn cost_builders() {
        match Cost::tap_self() {
            Cost::TapSelf => {}
            _ => panic!("wrong variant"),
        }

        match Cost::sacrifice_self() {
            Cost::SacrificeSelf => {}
            _ => panic!("wrong variant"),
        }

        match Cost::pay_life(3) {
            Cost::PayLife(n) => assert_eq!(n, 3),
            _ => panic!("wrong variant"),
        }

        match Cost::discard(1) {
            Cost::Discard(n) => assert_eq!(n, 1),
            _ => panic!("wrong variant"),
        }

        match Cost::sacrifice_other("a creature") {
            Cost::SacrificeOther(desc) => assert_eq!(desc, "a creature"),
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn static_effect_builders() {
        match StaticEffect::boost_controlled("creatures you control", 1, 1) {
            StaticEffect::Boost { filter, power, toughness } => {
                assert_eq!(filter, "creatures you control");
                assert_eq!(power, 1);
                assert_eq!(toughness, 1);
            }
            _ => panic!("wrong variant"),
        }

        match StaticEffect::grant_keyword_controlled("creatures you control", "flying") {
            StaticEffect::GrantKeyword { filter, keyword } => {
                assert_eq!(filter, "creatures you control");
                assert_eq!(keyword, "flying");
            }
            _ => panic!("wrong variant"),
        }

        match StaticEffect::cost_reduction("creature spells", 1) {
            StaticEffect::CostReduction { filter, amount, condition } => {
                assert_eq!(filter, "creature spells");
                assert_eq!(amount, 1);
                assert!(condition.is_none());
            }
            _ => panic!("wrong variant"),
        }

        match StaticEffect::cost_reduction_if_toughness_greater("creature spells", 1) {
            StaticEffect::CostReduction { filter, amount, condition } => {
                assert_eq!(filter, "creature spells");
                assert_eq!(amount, 1);
                assert_eq!(condition.as_deref(), Some("toughness_greater_than_power"));
            }
            _ => panic!("wrong variant"),
        }

        match StaticEffect::ward("{2}") {
            StaticEffect::Ward { cost } => {
                assert_eq!(cost, "{2}");
            }
            _ => panic!("wrong variant"),
        }

        match StaticEffect::ward("Discard a card.") {
            StaticEffect::Ward { cost } => {
                assert_eq!(cost, "Discard a card.");
            }
            _ => panic!("wrong variant"),
        }

        match StaticEffect::enters_tapped_unless("you control a Plains or an Island") {
            StaticEffect::EntersTappedUnless { condition } => {
                assert_eq!(condition, "you control a Plains or an Island");
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn compose_realistic_card_lightning_bolt() {
        // Lightning Bolt: {R} instant, "Deal 3 damage to any target."
        let source = ObjectId::new();
        let ability = Ability::spell(
            source,
            vec![Effect::deal_damage(3)],
            TargetSpec::CreatureOrPlayer,
        );
        assert_eq!(ability.effects.len(), 1);
        assert!(ability.uses_stack());
    }

    #[test]
    fn compose_realistic_card_llanowar_elves() {
        // Llanowar Elves: {G} creature, "{T}: Add {G}."
        let source = ObjectId::new();
        let ability = Ability::mana_ability(source, "{T}: Add {G}.", Mana::green(1));
        assert!(ability.is_mana_ability());
        assert_eq!(ability.costs.len(), 1);
        match &ability.costs[0] {
            Cost::TapSelf => {}
            _ => panic!("expected TapSelf cost"),
        }
    }

    #[test]
    fn compose_realistic_card_mulldrifter() {
        // Mulldrifter: when enters, draw 2 cards
        let source = ObjectId::new();
        let ability = Ability::enters_battlefield_triggered(
            source,
            "When Mulldrifter enters the battlefield, draw two cards.",
            vec![Effect::draw_cards(2)],
            TargetSpec::None,
        );
        assert!(ability.should_trigger(&GameEvent::new(EventType::EnteredTheBattlefield).target(source)));
    }

    #[test]
    fn compose_realistic_lord() {
        // Lord of Atlantis: Other Merfolk get +1/+1 and have islandwalk.
        let source = ObjectId::new();
        let ability = Ability::static_ability(
            source,
            "Other Merfolk you control get +1/+1 and have islandwalk.",
            vec![
                StaticEffect::boost_controlled("other Merfolk you control", 1, 1),
                StaticEffect::grant_keyword_controlled("other Merfolk you control", "islandwalk"),
            ],
        );
        assert_eq!(ability.static_effects.len(), 2);
    }
}
