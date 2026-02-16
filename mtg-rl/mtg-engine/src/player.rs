// Player state — tracks a player's life, zones, counters, and game flags.
//
// Ported from mage.players.PlayerImpl. This is the game-engine-side player
// state. Decision-making is delegated to PlayerDecisionMaker (in decision.rs).
//
// The Player struct owns the player's private zones (library, hand) and
// graveyard. The battlefield and stack are shared and live in GameState.

use crate::constants::Zone;
use crate::counters::{CounterType, Counters};
use crate::mana_pool::ManaPool;
use crate::types::{Life, ObjectId, PlayerId};
use crate::zones::{CommandZone, Graveyard, Hand, Library};
use serde::{Deserialize, Serialize};

/// Player state within a game.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    /// Unique player ID.
    pub id: PlayerId,
    /// Player name (for display).
    pub name: String,

    // ── Life and counters ────────────────────────────────────────────────
    /// Current life total.
    pub life: Life,
    /// Counters on this player (poison, energy, experience, etc.).
    pub counters: Counters,

    // ── Zones ────────────────────────────────────────────────────────────
    /// The player's library (deck).
    pub library: Library,
    /// The player's hand.
    pub hand: Hand,
    /// The player's graveyard.
    pub graveyard: Graveyard,
    /// The player's command zone (commanders, companions).
    pub command_zone: CommandZone,

    // ── Mana ─────────────────────────────────────────────────────────────
    /// The player's mana pool.
    pub mana_pool: ManaPool,

    // ── Land plays ───────────────────────────────────────────────────────
    /// How many lands the player has played this turn.
    pub lands_played_this_turn: u32,
    /// Maximum land plays per turn (normally 1, increased by effects).
    pub lands_per_turn: u32,

    // ── Hand size ────────────────────────────────────────────────────────
    /// Maximum hand size (normally 7). Set to u32::MAX for "no maximum hand size".
    pub max_hand_size: u32,

    // ── Game-over flags ──────────────────────────────────────────────────
    /// Whether this player has lost the game.
    pub lost: bool,
    /// Whether this player has won the game.
    pub won: bool,
    /// Whether this player has drawn (tied).
    pub drawn: bool,
    /// Whether this player has conceded.
    pub conceded: bool,

    // ── Priority tracking ────────────────────────────────────────────────
    /// Whether the player has passed priority since last stack change.
    pub passed: bool,
    /// Whether the player wants to pass until end of turn (F5-like).
    pub passed_until_end_of_turn: bool,
    /// Whether the player wants to skip to the next main phase.
    pub passed_until_next_main: bool,

    // ── Combat tracking ──────────────────────────────────────────────────
    /// Commanders' zone change counts (for commander tax).
    pub commander_cast_count: u32,

    // ── Miscellaneous ────────────────────────────────────────────────────
    /// The IDs of this player's commander card(s).
    pub commander_ids: Vec<ObjectId>,
    /// Whether the player has played a land this turn (shorthand for lands_played > 0).
    pub played_land_this_turn: bool,
}

impl Player {
    /// Create a new player with default starting values.
    pub fn new(id: PlayerId, name: &str) -> Self {
        Player {
            id,
            name: name.to_string(),
            life: Life::new(20),
            counters: Counters::new(),
            library: Library::new(),
            hand: Hand::new(),
            graveyard: Graveyard::new(),
            command_zone: CommandZone::new(),
            mana_pool: ManaPool::new(),
            lands_played_this_turn: 0,
            lands_per_turn: 1,
            max_hand_size: 7,
            lost: false,
            won: false,
            drawn: false,
            conceded: false,
            passed: false,
            passed_until_end_of_turn: false,
            passed_until_next_main: false,
            commander_cast_count: 0,
            commander_ids: Vec::new(),
            played_land_this_turn: false,
        }
    }

    // ── Life ─────────────────────────────────────────────────────────────

    /// Gain life. Returns the new life total.
    pub fn gain_life(&mut self, amount: u32) -> Life {
        self.life += amount as i32;
        self.life
    }

    /// Lose life. Returns the new life total.
    pub fn lose_life(&mut self, amount: u32) -> Life {
        self.life -= amount as i32;
        self.life
    }

    /// Set life total directly (for effects like "your life total becomes N").
    pub fn set_life(&mut self, amount: Life) -> Life {
        self.life = amount;
        self.life
    }

    /// Deal damage to this player.
    pub fn damage(&mut self, amount: u32) -> u32 {
        if amount == 0 {
            return 0;
        }
        self.life -= amount as i32;
        amount
    }

    // ── Counters ─────────────────────────────────────────────────────────

    /// Get poison counter count.
    pub fn poison_counters(&self) -> u32 {
        self.counters.get(&CounterType::Poison)
    }

    /// Get energy counter count.
    pub fn energy_counters(&self) -> u32 {
        self.counters.get(&CounterType::Energy)
    }

    /// Get experience counter count.
    pub fn experience_counters(&self) -> u32 {
        self.counters.get(&CounterType::Experience)
    }

    /// Add poison counters.
    pub fn add_poison(&mut self, count: u32) {
        self.counters.add(CounterType::Poison, count);
    }

    /// Add energy counters.
    pub fn add_energy(&mut self, count: u32) {
        self.counters.add(CounterType::Energy, count);
    }

    // ── Land plays ───────────────────────────────────────────────────────

    /// Check if the player can play another land this turn.
    pub fn can_play_land(&self) -> bool {
        self.lands_played_this_turn < self.lands_per_turn
    }

    /// Record that a land was played.
    pub fn play_land(&mut self) {
        self.lands_played_this_turn += 1;
        self.played_land_this_turn = true;
    }

    // ── Hand size ────────────────────────────────────────────────────────

    /// Check if the player needs to discard (hand size > max).
    pub fn needs_discard(&self) -> bool {
        self.max_hand_size < u32::MAX && self.hand.len() as u32 > self.max_hand_size
    }

    /// How many cards the player needs to discard.
    pub fn discard_count(&self) -> u32 {
        if self.needs_discard() {
            self.hand.len() as u32 - self.max_hand_size
        } else {
            0
        }
    }

    // ── Game state checks ────────────────────────────────────────────────

    /// Check if the player has lost (life <= 0, 10+ poison, etc.).
    /// Note: actual SBA checking is done by the game loop; this just checks
    /// the `lost` flag which is set by the game.
    pub fn has_lost(&self) -> bool {
        self.lost || self.conceded
    }

    /// Whether the player is still in the game.
    pub fn is_in_game(&self) -> bool {
        !self.lost && !self.won && !self.drawn && !self.conceded
    }

    // ── Turn reset ───────────────────────────────────────────────────────

    /// Reset per-turn state at the start of a new turn.
    pub fn begin_turn(&mut self) {
        self.lands_played_this_turn = 0;
        self.played_land_this_turn = false;
        self.passed = false;
        self.passed_until_end_of_turn = false;
        self.passed_until_next_main = false;
    }

    /// Reset priority pass flag.
    pub fn reset_passed(&mut self) {
        self.passed = false;
    }

    // ── Zone queries ─────────────────────────────────────────────────────

    /// Find which zone a card is in for this player.
    pub fn find_card_zone(&self, card_id: ObjectId) -> Option<Zone> {
        if self.hand.contains(card_id) {
            Some(Zone::Hand)
        } else if self.library.contains(card_id) {
            Some(Zone::Library)
        } else if self.graveyard.contains(card_id) {
            Some(Zone::Graveyard)
        } else if self.command_zone.contains(card_id) {
            Some(Zone::Command)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_player() -> Player {
        Player::new(PlayerId::new(), "Test Player")
    }

    #[test]
    fn starting_values() {
        let p = make_player();
        assert_eq!(p.life, Life::new(20));
        assert_eq!(p.lands_per_turn, 1);
        assert_eq!(p.max_hand_size, 7);
        assert!(!p.has_lost());
        assert!(p.is_in_game());
    }

    #[test]
    fn life_changes() {
        let mut p = make_player();
        assert_eq!(p.gain_life(5), Life::new(25));
        assert_eq!(p.lose_life(10), Life::new(15));
        assert_eq!(p.damage(3), 3);
        assert_eq!(p.life, Life::new(12));
        assert_eq!(p.set_life(Life::new(1)), Life::new(1));
    }

    #[test]
    fn land_plays() {
        let mut p = make_player();
        assert!(p.can_play_land());
        p.play_land();
        assert!(!p.can_play_land());
        assert!(p.played_land_this_turn);

        p.begin_turn();
        assert!(p.can_play_land());
        assert!(!p.played_land_this_turn);
    }

    #[test]
    fn discard_check() {
        let mut p = make_player();
        // Add 9 cards to hand
        for _ in 0..9 {
            p.hand.add(ObjectId::new());
        }
        assert!(p.needs_discard());
        assert_eq!(p.discard_count(), 2);
    }

    #[test]
    fn poison_counters() {
        let mut p = make_player();
        assert_eq!(p.poison_counters(), 0);
        p.add_poison(5);
        assert_eq!(p.poison_counters(), 5);
    }

    #[test]
    fn find_card_in_zones() {
        let mut p = make_player();
        let hand_card = ObjectId::new();
        let lib_card = ObjectId::new();
        let gy_card = ObjectId::new();
        let nowhere = ObjectId::new();

        p.hand.add(hand_card);
        p.library.put_on_top(lib_card);
        p.graveyard.add(gy_card);

        assert_eq!(p.find_card_zone(hand_card), Some(Zone::Hand));
        assert_eq!(p.find_card_zone(lib_card), Some(Zone::Library));
        assert_eq!(p.find_card_zone(gy_card), Some(Zone::Graveyard));
        assert_eq!(p.find_card_zone(nowhere), None);
    }

    #[test]
    fn game_over_states() {
        let mut p = make_player();
        assert!(p.is_in_game());

        p.conceded = true;
        assert!(p.has_lost());
        assert!(!p.is_in_game());
    }
}
