// Game event system.
//
// Events are the backbone of the rules engine. Every game action generates
// events that triggered abilities and replacement effects can respond to.
//
// The event flow:
// 1. An action creates a GameEvent (e.g. DAMAGE_PLAYER)
// 2. Replacement effects check if they want to modify/prevent the event
// 3. The event is executed (game state changes)
// 4. Triggered abilities check if they should trigger
//
// Ported from mage.game.events.GameEvent. The Java version has 600+ event
// types; we implement the core subset needed for initial gameplay and
// extend as needed for specific card implementations.

use crate::constants::Zone;
use crate::types::{ObjectId, PlayerId};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Event types — grouped by category
// ---------------------------------------------------------------------------

/// All game event types. Each represents something that is about to happen
/// (pre-event) or has happened (post-event) in the game.
///
/// Convention: `FOO` = the event is about to happen (can be replaced/prevented),
/// `FOOD` / `FOO_DONE` = it already happened (triggers can respond).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventType {
    // -- Turn structure events --
    BeginTurn,
    EndTurn,
    ChangePhase,
    ChangeStep,

    // Step pre/post events (for "at the beginning of..." triggers)
    UntapStepPre,
    UntapStep,
    UpkeepStepPre,
    UpkeepStep,
    DrawStepPre,
    DrawStep,
    PrecombatMainPre,
    PrecombatMain,
    BeginCombatPre,
    BeginCombat,
    DeclareAttackersPre,
    DeclareAttackers,
    DeclareBlockersPre,
    DeclareBlockers,
    CombatDamageStepPre,
    CombatDamageStep,
    EndCombatPre,
    EndCombat,
    PostcombatMainPre,
    PostcombatMain,
    EndStepPre,
    EndStep,
    CleanupStepPre,
    CleanupStep,
    AtEndOfTurn,

    // -- Zone change events --
    /// A card is about to change zones.
    ZoneChange,
    /// A card has changed zones.
    ZoneChanged,

    /// A player is about to draw a card.
    DrawCard,
    /// A player drew a card.
    DrewCard,

    /// A card is about to be discarded.
    DiscardCard,
    /// A card was discarded.
    DiscardedCard,

    /// A card is about to enter the battlefield.
    EntersTheBattlefield,
    /// A card entered the battlefield.
    EnteredTheBattlefield,

    /// A creature died (moved from battlefield to graveyard).
    Dies,

    /// A card is about to be exiled.
    ExileCard,
    /// A card was exiled.
    ExiledCard,

    /// Mill cards (library to graveyard).
    MillCards,
    MilledCard,

    // -- Spell and ability events --
    /// A spell is about to be cast.
    CastSpell,
    /// A spell was cast.
    SpellCast,

    /// An activated ability is about to be activated.
    ActivateAbility,
    /// An activated ability was activated.
    ActivatedAbility,

    /// A triggered ability triggered.
    TriggeredAbility,

    /// A stack object is about to resolve.
    ResolveSpell,
    /// A stack object resolved.
    SpellResolved,

    /// A spell/ability is about to be countered.
    Counter,
    /// A spell/ability was countered.
    Countered,

    // -- Mana events --
    /// Mana is about to be added to a player's pool.
    AddMana,
    /// Mana was added to a player's pool.
    ManaAdded,
    /// Mana was paid for a cost.
    ManaPaid,
    /// Mana pool is being emptied.
    EmptyManaPool,

    // -- Damage events --
    /// Damage is about to be dealt to a player.
    DamagePlayer,
    /// Damage was dealt to a player.
    DamagedPlayer,

    /// Damage is about to be dealt to a permanent.
    DamagePermanent,
    /// Damage was dealt to a permanent.
    DamagedPermanent,

    /// Combat damage was applied.
    CombatDamageApplied,

    /// Damage is about to be prevented.
    PreventDamage,
    /// Damage was prevented.
    PreventedDamage,

    // -- Life events --
    /// A player is about to gain life.
    GainLife,
    /// A player gained life.
    GainedLife,

    /// A player is about to lose life.
    LoseLife,
    /// A player lost life.
    LostLife,

    /// A player's life total changed.
    PlayerLifeChange,

    /// A player is about to pay life.
    PayLife,
    /// A player paid life.
    LifePaid,

    // -- Combat events --
    /// An attacker is being declared.
    DeclareAttacker,
    /// An attacker was declared.
    AttackerDeclared,
    /// Attackers were all declared (batch).
    DeclaredAttackers,

    /// A blocker is being declared.
    DeclareBlocker,
    /// A blocker was declared.
    BlockerDeclared,
    /// Blockers were all declared (batch).
    DeclaredBlockers,

    /// A creature became blocked.
    CreatureBlocked,
    /// A creature is unblocked.
    UnblockedAttacker,

    // -- Permanent events --
    /// A permanent is about to be tapped.
    Tap,
    /// A permanent was tapped.
    Tapped,

    /// A permanent is about to be untapped.
    Untap,
    /// A permanent was untapped.
    Untapped,

    /// A permanent is about to be destroyed.
    DestroyPermanent,
    /// A permanent was destroyed.
    DestroyedPermanent,

    /// A permanent is about to be sacrificed.
    SacrificePermanent,
    /// A permanent was sacrificed.
    SacrificedPermanent,

    /// A permanent is about to be regenerated.
    Regenerate,
    /// A permanent was regenerated.
    Regenerated,

    // -- Counter events --
    /// Counters are about to be added to an object.
    AddCounters,
    /// Counters were added to an object.
    CountersAdded,

    /// Counters are about to be removed from an object.
    RemoveCounters,
    /// Counters were removed from an object.
    CountersRemoved,

    // -- Token events --
    /// A token is about to be created.
    CreateToken,
    /// A token was created.
    CreatedToken,

    // -- Land events --
    /// A land is about to be played.
    PlayLand,
    /// A land was played.
    LandPlayed,

    // -- Player events --
    /// A player is about to lose the game.
    Loses,
    /// A player lost the game.
    Lost,

    /// A player is about to win the game.
    Wins,

    /// A player is about to search their library.
    SearchLibrary,
    /// A player searched their library.
    LibrarySearched,

    /// A player's library is about to be shuffled.
    ShuffleLibrary,
    /// A player's library was shuffled.
    LibraryShuffled,

    // -- Control events --
    /// Control of a permanent is about to change.
    GainControl,
    /// Control of a permanent changed.
    GainedControl,
    /// Control of a permanent was lost.
    LostControl,

    // -- Targeting events --
    /// A spell/ability is choosing targets.
    Target,
    /// Targets were chosen.
    Targeted,

    // -- Misc --
    /// An attachment (Aura, Equipment) is being attached.
    Attach,
    /// An attachment was attached.
    Attached,
    /// An attachment is being detached.
    Unattach,
    /// An attachment was detached.
    Unattached,

    /// A player is about to take a mulligan.
    Mulligan,
    /// Scry event.
    Scry,
    /// Scried.
    Scried,

    /// Monarch gained.
    BecomeMonarch,

    /// A card is transforming (DFC).
    Transforming,
    /// A card transformed.
    Transformed,
}

impl EventType {
    /// Parse an event type from a string name (case-insensitive).
    pub fn from_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "dies" => EventType::Dies,
            "end_step" | "endstep" => EventType::EndStep,
            "upkeep" | "upkeep_step" => EventType::UpkeepStep,
            "entered_the_battlefield" | "etb" => EventType::EnteredTheBattlefield,
            "spell_cast" => EventType::SpellCast,
            "attacker_declared" | "attack" => EventType::AttackerDeclared,
            "gain_life" | "gained_life" => EventType::GainedLife,
            "damaged_player" => EventType::DamagedPlayer,
            "damaged_permanent" => EventType::DamagedPermanent,
            "created_token" => EventType::CreatedToken,
            "land_played" => EventType::LandPlayed,
            "counters_added" => EventType::CountersAdded,
            "precombat_main" | "first_main_phase" => EventType::PrecombatMainPre,
            "transformed" => EventType::Transformed,
            _ => EventType::EnteredTheBattlefield, // fallback
        }
    }
}

// ---------------------------------------------------------------------------
// GameEvent struct
// ---------------------------------------------------------------------------

/// A game event carrying all the information about what happened.
///
/// Most fields are optional; their usage depends on the event type.
/// See the Java GameEvent documentation for per-event-type field semantics.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameEvent {
    /// What kind of event this is.
    pub event_type: EventType,

    /// The target of the event (e.g. damaged creature, spell being countered).
    pub target_id: Option<ObjectId>,

    /// The source of the event (e.g. ability or spell causing it).
    pub source_id: Option<ObjectId>,

    /// The player associated with the event (e.g. player casting, player damaged).
    pub player_id: Option<PlayerId>,

    /// Numeric amount (e.g. damage amount, life gained, number of counters).
    pub amount: i32,

    /// Boolean flag with event-type-specific meaning.
    pub flag: bool,

    /// String data with event-type-specific meaning.
    pub data: Option<String>,

    /// The zone involved (e.g. zone a card is moving from/to).
    pub zone: Option<Zone>,

    /// Whether this event was prevented by a replacement effect.
    pub prevented: bool,

    /// IDs of effects that have already been applied to this event
    /// (to prevent infinite loops of replacement effects).
    pub applied_effects: Vec<ObjectId>,
}

impl GameEvent {
    /// Create a new event with minimal required fields.
    pub fn new(event_type: EventType) -> Self {
        GameEvent {
            event_type,
            target_id: None,
            source_id: None,
            player_id: None,
            amount: 0,
            flag: false,
            data: None,
            zone: None,
            prevented: false,
            applied_effects: Vec::new(),
        }
    }

    /// Builder: set target.
    pub fn target(mut self, id: ObjectId) -> Self {
        self.target_id = Some(id);
        self
    }

    /// Builder: set source.
    pub fn source(mut self, id: ObjectId) -> Self {
        self.source_id = Some(id);
        self
    }

    /// Builder: set player.
    pub fn player(mut self, id: PlayerId) -> Self {
        self.player_id = Some(id);
        self
    }

    /// Builder: set amount.
    pub fn amount(mut self, amount: i32) -> Self {
        self.amount = amount;
        self
    }

    /// Builder: set flag.
    pub fn flag(mut self, flag: bool) -> Self {
        self.flag = flag;
        self
    }

    /// Builder: set data.
    pub fn data(mut self, data: &str) -> Self {
        self.data = Some(data.to_string());
        self
    }

    /// Builder: set zone.
    pub fn zone(mut self, zone: Zone) -> Self {
        self.zone = Some(zone);
        self
    }

    /// Mark this event as prevented.
    pub fn prevent(&mut self) {
        self.prevented = true;
    }

    /// Record that an effect has been applied to this event.
    pub fn mark_applied(&mut self, effect_id: ObjectId) {
        self.applied_effects.push(effect_id);
    }

    /// Check if a specific effect has already been applied.
    pub fn was_applied(&self, effect_id: ObjectId) -> bool {
        self.applied_effects.contains(&effect_id)
    }
}

// ---------------------------------------------------------------------------
// Convenience constructors for common events
// ---------------------------------------------------------------------------

impl GameEvent {
    /// Create a damage-to-player event.
    pub fn damage_player(
        target_player: PlayerId,
        source: ObjectId,
        amount: u32,
        is_combat: bool,
    ) -> Self {
        GameEvent::new(EventType::DamagePlayer)
            .player(target_player)
            .source(source)
            .amount(amount as i32)
            .flag(is_combat)
    }

    /// Create a damage-to-permanent event.
    pub fn damage_permanent(
        target_permanent: ObjectId,
        source: ObjectId,
        amount: u32,
        is_combat: bool,
    ) -> Self {
        GameEvent::new(EventType::DamagePermanent)
            .target(target_permanent)
            .source(source)
            .amount(amount as i32)
            .flag(is_combat)
    }

    /// Create a gain-life event.
    pub fn gain_life(player: PlayerId, amount: u32) -> Self {
        GameEvent::new(EventType::GainLife)
            .player(player)
            .amount(amount as i32)
    }

    /// Create a lose-life event.
    pub fn lose_life(player: PlayerId, source: ObjectId, amount: u32, is_combat: bool) -> Self {
        GameEvent::new(EventType::LoseLife)
            .player(player)
            .source(source)
            .amount(amount as i32)
            .flag(is_combat)
    }

    /// Create a draw-card event.
    pub fn draw_card(player: PlayerId) -> Self {
        GameEvent::new(EventType::DrawCard).player(player)
    }

    /// Create a discard event.
    pub fn discard_card(card_id: ObjectId, player: PlayerId, is_effect: bool) -> Self {
        GameEvent::new(EventType::DiscardCard)
            .target(card_id)
            .player(player)
            .flag(is_effect)
    }

    /// Create a zone change event.
    pub fn zone_change(
        card_id: ObjectId,
        source: Option<ObjectId>,
        player: PlayerId,
        from_zone: Zone,
        to_zone: Zone,
    ) -> Self {
        let mut event = GameEvent::new(EventType::ZoneChange)
            .target(card_id)
            .player(player)
            .zone(from_zone);
        if let Some(src) = source {
            event = event.source(src);
        }
        event.data = Some(format!("{} -> {}", from_zone, to_zone));
        event
    }

    /// Create a spell-cast event.
    pub fn spell_cast(spell_id: ObjectId, player: PlayerId, from_zone: Zone) -> Self {
        GameEvent::new(EventType::SpellCast)
            .target(spell_id)
            .source(spell_id)
            .player(player)
            .zone(from_zone)
    }

    /// Create an enters-the-battlefield event.
    pub fn enters_battlefield(permanent_id: ObjectId, player: PlayerId) -> Self {
        GameEvent::new(EventType::EnteredTheBattlefield)
            .target(permanent_id)
            .player(player)
    }

    pub fn enters_battlefield_from(permanent_id: ObjectId, player: PlayerId, from: crate::constants::Zone) -> Self {
        GameEvent::new(EventType::EnteredTheBattlefield)
            .target(permanent_id)
            .player(player)
            .zone(from)
    }

    /// Create a dies event (creature to graveyard from battlefield).
    pub fn dies(permanent_id: ObjectId, player: PlayerId) -> Self {
        GameEvent::new(EventType::Dies)
            .target(permanent_id)
            .player(player)
    }

    /// Create a destroy-permanent event.
    pub fn destroy_permanent(permanent_id: ObjectId, source: ObjectId) -> Self {
        GameEvent::new(EventType::DestroyPermanent)
            .target(permanent_id)
            .source(source)
    }

    /// Create an add-counters event.
    pub fn add_counters(
        target_id: ObjectId,
        source: ObjectId,
        player: PlayerId,
        counter_name: &str,
        count: u32,
    ) -> Self {
        GameEvent::new(EventType::AddCounters)
            .target(target_id)
            .source(source)
            .player(player)
            .amount(count as i32)
            .data(counter_name)
    }

    /// Create a tap event.
    pub fn tap(permanent_id: ObjectId, is_combat: bool) -> Self {
        GameEvent::new(EventType::Tap)
            .target(permanent_id)
            .flag(is_combat)
    }

    /// Create a declare-attacker event.
    pub fn declare_attacker(
        attacker_id: ObjectId,
        defender_id: ObjectId,
        player: PlayerId,
    ) -> Self {
        GameEvent::new(EventType::DeclareAttacker)
            .target(defender_id)
            .source(attacker_id)
            .player(player)
    }

    /// Create a declare-blocker event.
    pub fn declare_blocker(
        blocker_id: ObjectId,
        attacker_id: ObjectId,
        player: PlayerId,
    ) -> Self {
        GameEvent::new(EventType::DeclareBlocker)
            .target(attacker_id)
            .source(blocker_id)
            .player(player)
    }
}

// ---------------------------------------------------------------------------
// Event log — records events for triggered ability checking and replay
// ---------------------------------------------------------------------------

/// A log of events that occurred during a game action. Used to check for
/// triggered abilities after an action resolves.
#[derive(Clone, Debug, Default)]
pub struct EventLog {
    events: Vec<GameEvent>,
}

impl EventLog {
    pub fn new() -> Self {
        EventLog { events: Vec::new() }
    }

    /// Record an event.
    pub fn push(&mut self, event: GameEvent) {
        self.events.push(event);
    }

    /// Get all events of a specific type.
    pub fn events_of_type(&self, event_type: EventType) -> impl Iterator<Item = &GameEvent> {
        self.events.iter().filter(move |e| e.event_type == event_type)
    }

    /// Get all events.
    pub fn iter(&self) -> impl Iterator<Item = &GameEvent> {
        self.events.iter()
    }

    /// Number of events logged.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Clear the event log.
    pub fn clear(&mut self) {
        self.events.clear();
    }

    /// Check if any event of the given type occurred.
    pub fn has_event(&self, event_type: EventType) -> bool {
        self.events.iter().any(|e| e.event_type == event_type)
    }

    /// Count events of a specific type.
    pub fn count_events(&self, event_type: EventType) -> usize {
        self.events.iter().filter(|e| e.event_type == event_type).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_builder() {
        let target = ObjectId::new();
        let source = ObjectId::new();
        let player = PlayerId::new();

        let event = GameEvent::new(EventType::DamagePlayer)
            .target(target)
            .source(source)
            .player(player)
            .amount(3)
            .flag(true);

        assert_eq!(event.event_type, EventType::DamagePlayer);
        assert_eq!(event.target_id, Some(target));
        assert_eq!(event.source_id, Some(source));
        assert_eq!(event.player_id, Some(player));
        assert_eq!(event.amount, 3);
        assert!(event.flag);
        assert!(!event.prevented);
    }

    #[test]
    fn damage_player_convenience() {
        let player = PlayerId::new();
        let source = ObjectId::new();

        let event = GameEvent::damage_player(player, source, 5, true);
        assert_eq!(event.event_type, EventType::DamagePlayer);
        assert_eq!(event.amount, 5);
        assert!(event.flag); // is combat damage
    }

    #[test]
    fn prevention() {
        let mut event = GameEvent::new(EventType::DamagePlayer).amount(3);
        assert!(!event.prevented);
        event.prevent();
        assert!(event.prevented);
    }

    #[test]
    fn applied_effects_tracking() {
        let mut event = GameEvent::new(EventType::DamagePlayer);
        let effect1 = ObjectId::new();
        let effect2 = ObjectId::new();

        assert!(!event.was_applied(effect1));
        event.mark_applied(effect1);
        assert!(event.was_applied(effect1));
        assert!(!event.was_applied(effect2));
    }

    #[test]
    fn event_log() {
        let mut log = EventLog::new();
        let player = PlayerId::new();

        log.push(GameEvent::draw_card(player));
        log.push(GameEvent::draw_card(player));
        log.push(GameEvent::gain_life(player, 3));

        assert_eq!(log.len(), 3);
        assert!(log.has_event(EventType::DrawCard));
        assert!(log.has_event(EventType::GainLife));
        assert!(!log.has_event(EventType::LoseLife));
        assert_eq!(log.count_events(EventType::DrawCard), 2);
        assert_eq!(log.count_events(EventType::GainLife), 1);
    }

    #[test]
    fn zone_change_event() {
        let card = ObjectId::new();
        let source = ObjectId::new();
        let player = PlayerId::new();

        let event = GameEvent::zone_change(
            card,
            Some(source),
            player,
            Zone::Hand,
            Zone::Battlefield,
        );

        assert_eq!(event.event_type, EventType::ZoneChange);
        assert_eq!(event.target_id, Some(card));
        assert_eq!(event.source_id, Some(source));
        assert_eq!(event.zone, Some(Zone::Hand));
        assert!(event.data.as_ref().unwrap().contains("hand"));
        assert!(event.data.as_ref().unwrap().contains("battlefield"));
    }

    #[test]
    fn spell_cast_event() {
        let spell = ObjectId::new();
        let player = PlayerId::new();

        let event = GameEvent::spell_cast(spell, player, Zone::Hand);
        assert_eq!(event.event_type, EventType::SpellCast);
        assert_eq!(event.zone, Some(Zone::Hand));
    }

    #[test]
    fn combat_events() {
        let attacker = ObjectId::new();
        let defender = ObjectId::new();
        let blocker = ObjectId::new();
        let player = PlayerId::new();

        let attack_event = GameEvent::declare_attacker(attacker, defender, player);
        assert_eq!(attack_event.event_type, EventType::DeclareAttacker);
        assert_eq!(attack_event.source_id, Some(attacker));
        assert_eq!(attack_event.target_id, Some(defender));

        let block_event = GameEvent::declare_blocker(blocker, attacker, player);
        assert_eq!(block_event.event_type, EventType::DeclareBlocker);
        assert_eq!(block_event.source_id, Some(blocker));
        assert_eq!(block_event.target_id, Some(attacker));
    }
}
