// Watchers — track game events for triggered abilities and conditions.
//
// Watchers observe game events and accumulate state that triggered abilities
// and other game mechanics query. They are reset at appropriate points
// (typically at the start of each turn).
//
// Replaces Java's Watcher abstract class and the 80+ common watchers
// with a data-oriented approach: a central WatcherManager that tracks
// commonly needed per-turn statistics, plus a registry for custom watchers.

use crate::events::{EventType, GameEvent};
use crate::types::{ObjectId, PlayerId};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

// ---------------------------------------------------------------------------
// WatcherScope — how a watcher instance is keyed
// ---------------------------------------------------------------------------

pub use crate::constants::WatcherScope;

// ---------------------------------------------------------------------------
// Built-in watcher data (common per-turn tracking)
// ---------------------------------------------------------------------------

/// Per-player statistics tracked each turn.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PlayerTurnStats {
    /// Number of spells cast this turn.
    pub spells_cast: u32,
    /// Number of creature spells cast this turn.
    pub creature_spells_cast: u32,
    /// Number of noncreature spells cast this turn.
    pub noncreature_spells_cast: u32,
    /// Number of lands played this turn.
    pub lands_played: u32,
    /// Number of cards drawn this turn.
    pub cards_drawn: u32,
    /// Total life gained this turn.
    pub life_gained: i32,
    /// Total life lost this turn.
    pub life_lost: i32,
    /// Total damage dealt this turn (by sources controlled by this player).
    pub damage_dealt: u32,
    /// Number of creatures that died (controlled by this player) this turn.
    pub creatures_died: u32,
    /// Number of creatures that entered the battlefield this turn.
    pub creatures_entered: u32,
    /// Set of permanents that entered the battlefield this turn (for ETB triggers).
    pub entered_battlefield: HashSet<ObjectId>,
    /// Set of permanents that left the battlefield this turn.
    pub left_battlefield: HashSet<ObjectId>,
    /// Whether this player was dealt combat damage this turn.
    pub took_combat_damage: bool,
    /// IDs of spells cast from graveyard this turn.
    pub spells_cast_from_graveyard: Vec<ObjectId>,
    /// Number of permanents sacrificed this turn.
    pub permanents_sacrificed: u32,
    /// Attackers declared this turn (by this player).
    pub attackers_declared: Vec<ObjectId>,
    /// Whether this player attacked this turn.
    pub attacked_this_turn: bool,
}

/// Global (game-level) statistics tracked each turn.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GameTurnStats {
    /// Total creatures that died this turn (all players).
    pub total_creatures_died: u32,
    /// Total spells cast this turn (all players).
    pub total_spells_cast: u32,
    /// IDs of all permanents that entered the battlefield this turn.
    pub all_entered_battlefield: HashSet<ObjectId>,
    /// IDs of all permanents that died this turn.
    pub all_died: HashSet<ObjectId>,
    /// Whether any player gained life this turn.
    pub any_life_gained: bool,
    /// Whether combat damage was dealt this turn.
    pub combat_damage_dealt: bool,
}

// ---------------------------------------------------------------------------
// WatcherManager — central tracking for all game events
// ---------------------------------------------------------------------------

/// Central manager for all watchers in the game.
///
/// Provides both built-in per-turn statistics and a registry for
/// custom watchers needed by specific cards.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WatcherManager {
    /// Per-player stats for the current turn.
    player_stats: HashMap<PlayerId, PlayerTurnStats>,
    /// Game-level stats for the current turn.
    game_stats: GameTurnStats,
    /// Custom watchers registered by cards/abilities.
    custom_watchers: HashMap<String, CustomWatcher>,
}

impl WatcherManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Process a game event, updating all relevant tracking.
    pub fn watch(&mut self, event: &GameEvent) {
        // Update player stats.
        if let Some(pid) = event.player_id {
            let stats = self.player_stats.entry(pid).or_default();

            match event.event_type {
                EventType::SpellCast => {
                    stats.spells_cast += 1;
                    self.game_stats.total_spells_cast += 1;
                }
                // Note: creature spell detection requires card type info.
                // For now, tracked via SpellCast only. Cards that need
                // creature_spells_cast should use a custom watcher.
                EventType::DrawCard => {
                    stats.cards_drawn += 1;
                }
                EventType::GainLife => {
                    stats.life_gained += event.amount;
                    self.game_stats.any_life_gained = true;
                }
                EventType::LoseLife => {
                    stats.life_lost += event.amount;
                }
                EventType::PlayLand => {
                    stats.lands_played += 1;
                }
                EventType::DeclareAttacker => {
                    if let Some(id) = event.source_id {
                        stats.attackers_declared.push(id);
                    }
                    stats.attacked_this_turn = true;
                }
                EventType::SacrificedPermanent => {
                    stats.permanents_sacrificed += 1;
                }
                _ => {}
            }
        }

        // Handle zone-change events that track dying, entering, leaving.
        match event.event_type {
            EventType::EntersTheBattlefield => {
                if let Some(target_id) = event.target_id {
                    self.game_stats.all_entered_battlefield.insert(target_id);
                    if let Some(pid) = event.player_id {
                        let stats = self.player_stats.entry(pid).or_default();
                        stats.entered_battlefield.insert(target_id);
                        // TODO: check if it's a creature for creatures_entered
                        stats.creatures_entered += 1;
                    }
                }
            }
            EventType::Dies => {
                if let Some(target_id) = event.target_id {
                    self.game_stats.all_died.insert(target_id);
                    self.game_stats.total_creatures_died += 1;
                    if let Some(pid) = event.player_id {
                        let stats = self.player_stats.entry(pid).or_default();
                        stats.creatures_died += 1;
                        stats.left_battlefield.insert(target_id);
                    }
                }
            }
            EventType::ZoneChanged => {
                if let Some(target_id) = event.target_id {
                    if let Some(pid) = event.player_id {
                        let stats = self.player_stats.entry(pid).or_default();
                        stats.left_battlefield.insert(target_id);
                    }
                }
            }
            EventType::DamagePlayer => {
                if event.flag {
                    // flag indicates combat damage
                    self.game_stats.combat_damage_dealt = true;
                    if let Some(pid) = event.target_id.and_then(|_| event.player_id) {
                        let stats = self.player_stats.entry(pid).or_default();
                        stats.took_combat_damage = true;
                    }
                }
                // Track damage dealt by source controller
                if let Some(source_controller) = event.player_id {
                    let stats = self.player_stats.entry(source_controller).or_default();
                    stats.damage_dealt += event.amount as u32;
                }
            }
            _ => {}
        }

        // Update custom watchers.
        for watcher in self.custom_watchers.values_mut() {
            watcher.watch(event);
        }
    }

    /// Reset all turn-level tracking (called at the start of each turn).
    pub fn reset_turn(&mut self) {
        self.player_stats.clear();
        self.game_stats = GameTurnStats::default();
        for watcher in self.custom_watchers.values_mut() {
            watcher.reset();
        }
    }

    // ── Query methods ───────────────────────────────────────────────────

    /// Get per-player stats for a player. Returns None if no events were
    /// recorded for the player this turn.
    pub fn player_stats(&self, player: PlayerId) -> PlayerTurnStats {
        self.player_stats
            .get(&player)
            .cloned()
            .unwrap_or_default()
    }

    /// Get game-level stats.
    pub fn game_stats(&self) -> &GameTurnStats {
        &self.game_stats
    }

    /// Number of spells a player has cast this turn.
    pub fn spells_cast_this_turn(&self, player: PlayerId) -> u32 {
        self.player_stats(player).spells_cast
    }

    /// Number of cards drawn by a player this turn.
    pub fn cards_drawn_this_turn(&self, player: PlayerId) -> u32 {
        self.player_stats(player).cards_drawn
    }

    /// Total life gained by a player this turn.
    pub fn life_gained_this_turn(&self, player: PlayerId) -> i32 {
        self.player_stats(player).life_gained
    }

    /// Total creatures that died this turn (all players).
    pub fn total_creatures_died_this_turn(&self) -> u32 {
        self.game_stats.total_creatures_died
    }

    /// Whether a player attacked this turn.
    pub fn player_attacked_this_turn(&self, player: PlayerId) -> bool {
        self.player_stats(player).attacked_this_turn
    }

    /// Whether a permanent entered the battlefield this turn.
    pub fn entered_battlefield_this_turn(&self, id: ObjectId) -> bool {
        self.game_stats.all_entered_battlefield.contains(&id)
    }

    /// Whether a permanent died this turn.
    pub fn died_this_turn(&self, id: ObjectId) -> bool {
        self.game_stats.all_died.contains(&id)
    }

    // ── Custom watcher management ───────────────────────────────────────

    /// Register a custom watcher.
    pub fn register_custom(&mut self, name: &str, watcher: CustomWatcher) {
        self.custom_watchers.insert(name.to_string(), watcher);
    }

    /// Get a custom watcher by name.
    pub fn get_custom(&self, name: &str) -> Option<&CustomWatcher> {
        self.custom_watchers.get(name)
    }

    /// Get a mutable custom watcher by name.
    pub fn get_custom_mut(&mut self, name: &str) -> Option<&mut CustomWatcher> {
        self.custom_watchers.get_mut(name)
    }
}

// ---------------------------------------------------------------------------
// Custom watcher — for card-specific tracking
// ---------------------------------------------------------------------------

/// A custom watcher for card-specific event tracking.
///
/// Custom watchers listen for specific event types and maintain
/// a condition flag plus a counter.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomWatcher {
    /// The event types this watcher cares about.
    pub watched_events: Vec<EventType>,
    /// The scope of this watcher.
    pub scope: WatcherScope,
    /// Source ID (for Card-scoped watchers).
    pub source_id: Option<ObjectId>,
    /// Controller ID (for Player-scoped watchers).
    pub controller_id: Option<PlayerId>,
    /// Whether the condition has been met.
    pub condition: bool,
    /// A general-purpose counter.
    pub count: u32,
    /// Set of object IDs that triggered the condition.
    pub tracked_ids: HashSet<ObjectId>,
    /// Set of player IDs that triggered the condition.
    pub tracked_players: HashSet<PlayerId>,
}

impl CustomWatcher {
    pub fn new(watched_events: Vec<EventType>, scope: WatcherScope) -> Self {
        CustomWatcher {
            watched_events,
            scope,
            source_id: None,
            controller_id: None,
            condition: false,
            count: 0,
            tracked_ids: HashSet::new(),
            tracked_players: HashSet::new(),
        }
    }

    /// Process a game event.
    pub fn watch(&mut self, event: &GameEvent) {
        if !self.watched_events.contains(&event.event_type) {
            return;
        }

        // Check scope filtering.
        match self.scope {
            WatcherScope::Card => {
                if let Some(src) = self.source_id {
                    if event.source_id != Some(src) {
                        return;
                    }
                }
            }
            WatcherScope::Player => {
                if let Some(ctrl) = self.controller_id {
                    if event.player_id != Some(ctrl) {
                        return;
                    }
                }
            }
            WatcherScope::Game => {
                // No scope filtering for game-wide watchers.
            }
        }

        self.condition = true;
        self.count += 1;
        if let Some(target_id) = event.target_id {
            self.tracked_ids.insert(target_id);
        }
        if let Some(player_id) = event.player_id {
            self.tracked_players.insert(player_id);
        }
    }

    /// Reset the watcher (typically at the start of each turn).
    pub fn reset(&mut self) {
        self.condition = false;
        self.count = 0;
        self.tracked_ids.clear();
        self.tracked_players.clear();
    }

    pub fn condition_met(&self) -> bool {
        self.condition
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{EventType, GameEvent};
    use crate::types::{ObjectId, PlayerId};

    #[test]
    fn track_spells_cast() {
        let mut mgr = WatcherManager::new();
        let player = PlayerId::new();

        let event = GameEvent::new(EventType::SpellCast).player(player);
        mgr.watch(&event);
        mgr.watch(&event);

        assert_eq!(mgr.spells_cast_this_turn(player), 2);
        assert_eq!(mgr.game_stats().total_spells_cast, 2);
    }

    #[test]
    fn track_cards_drawn() {
        let mut mgr = WatcherManager::new();
        let player = PlayerId::new();

        let event = GameEvent::new(EventType::DrawCard).player(player);
        mgr.watch(&event);
        mgr.watch(&event);
        mgr.watch(&event);

        assert_eq!(mgr.cards_drawn_this_turn(player), 3);
    }

    #[test]
    fn track_creatures_died() {
        let mut mgr = WatcherManager::new();
        let player = PlayerId::new();
        let creature_id = ObjectId::new();

        let event = GameEvent::new(EventType::Dies)
            .target(creature_id)
            .player(player);
        mgr.watch(&event);

        assert_eq!(mgr.total_creatures_died_this_turn(), 1);
        assert!(mgr.died_this_turn(creature_id));
        assert_eq!(mgr.player_stats(player).creatures_died, 1);
    }

    #[test]
    fn track_life_gained() {
        let mut mgr = WatcherManager::new();
        let player = PlayerId::new();

        let event = GameEvent::new(EventType::GainLife)
            .player(player)
            .amount(3);
        mgr.watch(&event);

        assert_eq!(mgr.life_gained_this_turn(player), 3);
        assert!(mgr.game_stats().any_life_gained);
    }

    #[test]
    fn track_enters_battlefield() {
        let mut mgr = WatcherManager::new();
        let player = PlayerId::new();
        let perm_id = ObjectId::new();

        let event = GameEvent::new(EventType::EntersTheBattlefield)
            .target(perm_id)
            .player(player);
        mgr.watch(&event);

        assert!(mgr.entered_battlefield_this_turn(perm_id));
        assert!(mgr.player_stats(player).entered_battlefield.contains(&perm_id));
    }

    #[test]
    fn reset_clears_stats() {
        let mut mgr = WatcherManager::new();
        let player = PlayerId::new();

        let event = GameEvent::new(EventType::SpellCast).player(player);
        mgr.watch(&event);
        assert_eq!(mgr.spells_cast_this_turn(player), 1);

        mgr.reset_turn();
        assert_eq!(mgr.spells_cast_this_turn(player), 0);
        assert_eq!(mgr.game_stats().total_spells_cast, 0);
    }

    #[test]
    fn custom_watcher_game_scope() {
        let mut mgr = WatcherManager::new();

        let watcher = CustomWatcher::new(
            vec![EventType::SpellCast],
            WatcherScope::Game,
        );
        mgr.register_custom("storm_count", watcher);

        let player = PlayerId::new();
        let event = GameEvent::new(EventType::SpellCast).player(player);
        mgr.watch(&event);
        mgr.watch(&event);
        mgr.watch(&event);

        let w = mgr.get_custom("storm_count").unwrap();
        assert!(w.condition_met());
        assert_eq!(w.count, 3);
    }

    #[test]
    fn custom_watcher_card_scope() {
        let mut mgr = WatcherManager::new();
        let source = ObjectId::new();
        let other = ObjectId::new();

        let mut watcher = CustomWatcher::new(
            vec![EventType::DamagePermanent],
            WatcherScope::Card,
        );
        watcher.source_id = Some(source);
        mgr.register_custom("damage_tracker", watcher);

        // Event from the tracked source
        let event = GameEvent::new(EventType::DamagePermanent)
            .source(source)
            .amount(3);
        mgr.watch(&event);

        // Event from a different source
        let event2 = GameEvent::new(EventType::DamagePermanent)
            .source(other)
            .amount(2);
        mgr.watch(&event2);

        let w = mgr.get_custom("damage_tracker").unwrap();
        assert!(w.condition_met());
        assert_eq!(w.count, 1); // only the matching source
    }

    #[test]
    fn custom_watcher_player_scope() {
        let mut mgr = WatcherManager::new();
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let mut watcher = CustomWatcher::new(
            vec![EventType::DrawCard],
            WatcherScope::Player,
        );
        watcher.controller_id = Some(p1);
        mgr.register_custom("draw_watcher", watcher);

        mgr.watch(&GameEvent::new(EventType::DrawCard).player(p1));
        mgr.watch(&GameEvent::new(EventType::DrawCard).player(p2));
        mgr.watch(&GameEvent::new(EventType::DrawCard).player(p1));

        let w = mgr.get_custom("draw_watcher").unwrap();
        assert_eq!(w.count, 2); // only p1's draws
    }

    #[test]
    fn track_attackers() {
        let mut mgr = WatcherManager::new();
        let player = PlayerId::new();
        let attacker_id = ObjectId::new();

        let event = GameEvent::new(EventType::DeclareAttacker)
            .source(attacker_id)
            .player(player);
        mgr.watch(&event);

        assert!(mgr.player_attacked_this_turn(player));
        assert_eq!(mgr.player_stats(player).attackers_declared.len(), 1);
    }

    #[test]
    fn custom_watcher_reset() {
        let mut watcher = CustomWatcher::new(
            vec![EventType::SpellCast],
            WatcherScope::Game,
        );

        let event = GameEvent::new(EventType::SpellCast).player(PlayerId::new());
        watcher.watch(&event);
        assert!(watcher.condition_met());
        assert_eq!(watcher.count, 1);

        watcher.reset();
        assert!(!watcher.condition_met());
        assert_eq!(watcher.count, 0);
    }
}
