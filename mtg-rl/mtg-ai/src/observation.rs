// Observation space for RL — fixed-size tensor encoding of game state.
//
// The observation encodes the game state as a flat f32 vector suitable for
// neural network input. Features are organized into sections:
//
//   [player_info | opponent_info | mana | phase | permanents | hand | stack | action_mask]
//
// All sections are fixed-size to enable batched training. Permanents, hand
// cards, and stack items use a fixed maximum count with zero-padding.

use mtg_engine::constants::PhaseStep;
use mtg_engine::types::ObjectId;

// ---------------------------------------------------------------------------
// Dimension constants
// ---------------------------------------------------------------------------

/// Max permanents per player to encode.
pub const MAX_PERMANENTS_PER_SIDE: usize = 50;
/// Features per permanent: [power, toughness, tapped, summoning_sickness,
///   is_creature, is_land, is_artifact, is_enchantment, is_planeswalker,
///   is_attacking, is_blocking, damage_marked, keywords_bits(16)]
pub const PERMANENT_FEATURES: usize = 28;

/// Max cards in hand to encode.
pub const MAX_HAND_SIZE: usize = 15;
/// Features per hand card: [mana_value, is_creature, is_instant, is_sorcery,
///   is_land, is_artifact, is_enchantment, is_planeswalker, power, toughness,
///   castable(bool)]
pub const HAND_CARD_FEATURES: usize = 11;

/// Max stack items to encode.
pub const MAX_STACK_SIZE: usize = 10;
/// Features per stack item: [is_own, mana_value, is_creature_spell,
///   is_instant, is_sorcery, is_ability, targets_own_permanent,
///   targets_opponent_permanent]
pub const STACK_ITEM_FEATURES: usize = 8;

/// Player info features: [life, poison_counters, energy_counters,
///   land_count, creature_count, hand_size, library_size, graveyard_size]
pub const PLAYER_INFO_FEATURES: usize = 8;

/// Mana available: [W, U, B, R, G, C] per player.
pub const MANA_FEATURES: usize = 6;

/// Phase step one-hot encoding (13 steps).
pub const PHASE_FEATURES: usize = 13;

/// Total observation vector size.
pub const OBSERVATION_SIZE: usize = PLAYER_INFO_FEATURES * 2  // both players
    + MANA_FEATURES * 2                                        // mana for both
    + PHASE_FEATURES                                           // current phase
    + 1                                                        // is_active_player (bool)
    + 1                                                        // turn_number (normalized)
    + MAX_PERMANENTS_PER_SIDE * PERMANENT_FEATURES * 2         // permanents both sides
    + MAX_HAND_SIZE * HAND_CARD_FEATURES                       // own hand
    + MAX_STACK_SIZE * STACK_ITEM_FEATURES;                    // stack

// ---------------------------------------------------------------------------
// Input data types (game-state snapshots for encoding)
// ---------------------------------------------------------------------------

/// Snapshot of a player's state for observation encoding.
#[derive(Clone, Debug, Default)]
pub struct PlayerSnapshot {
    pub life: i32,
    pub poison_counters: i32,
    pub energy_counters: i32,
    pub land_count: u32,
    pub creature_count: u32,
    pub hand_size: u32,
    pub library_size: u32,
    pub graveyard_size: u32,
    /// Available mana: [W, U, B, R, G, C].
    pub mana_available: [u32; 6],
}

/// Snapshot of a permanent for observation encoding.
#[derive(Clone, Debug, Default)]
pub struct PermanentSnapshot {
    pub id: ObjectId,
    pub power: i32,
    pub toughness: i32,
    pub tapped: bool,
    pub summoning_sickness: bool,
    pub is_creature: bool,
    pub is_land: bool,
    pub is_artifact: bool,
    pub is_enchantment: bool,
    pub is_planeswalker: bool,
    pub is_attacking: bool,
    pub is_blocking: bool,
    pub damage_marked: i32,
    /// Keyword abilities as a 16-bit bitfield (maps to the most common 16
    /// keywords: flying, first_strike, double_strike, trample, haste,
    /// vigilance, lifelink, deathtouch, reach, defender, menace, flash,
    /// indestructible, hexproof, shroud, fear).
    pub keyword_bits: u16,
}

/// Snapshot of a hand card for observation encoding.
#[derive(Clone, Debug, Default)]
pub struct HandCardSnapshot {
    pub id: ObjectId,
    pub mana_value: u32,
    pub is_creature: bool,
    pub is_instant: bool,
    pub is_sorcery: bool,
    pub is_land: bool,
    pub is_artifact: bool,
    pub is_enchantment: bool,
    pub is_planeswalker: bool,
    pub power: i32,
    pub toughness: i32,
    /// Whether this card can currently be cast/played.
    pub castable: bool,
}

/// Snapshot of a stack item for observation encoding.
#[derive(Clone, Debug, Default)]
pub struct StackItemSnapshot {
    /// True if controlled by the observing player.
    pub is_own: bool,
    pub mana_value: u32,
    pub is_creature_spell: bool,
    pub is_instant: bool,
    pub is_sorcery: bool,
    pub is_ability: bool,
    pub targets_own_permanent: bool,
    pub targets_opponent_permanent: bool,
}

/// Full game snapshot for observation encoding.
#[derive(Clone, Debug)]
pub struct GameSnapshot {
    pub player: PlayerSnapshot,
    pub opponent: PlayerSnapshot,
    pub own_permanents: Vec<PermanentSnapshot>,
    pub opponent_permanents: Vec<PermanentSnapshot>,
    pub hand: Vec<HandCardSnapshot>,
    pub stack: Vec<StackItemSnapshot>,
    pub current_step: PhaseStep,
    pub is_active_player: bool,
    pub turn_number: u32,
}

// ---------------------------------------------------------------------------
// Observation encoding
// ---------------------------------------------------------------------------

/// A fixed-size observation vector for RL input.
#[derive(Clone, Debug)]
pub struct Observation {
    /// The encoded feature vector. Length = OBSERVATION_SIZE.
    pub features: Vec<f32>,
}

impl Observation {
    /// Encode a game snapshot into a fixed-size observation vector.
    pub fn encode(snapshot: &GameSnapshot) -> Self {
        let mut features = Vec::with_capacity(OBSERVATION_SIZE);

        // -- Player info (own) --
        encode_player_info(&mut features, &snapshot.player);
        // -- Player info (opponent) --
        encode_player_info(&mut features, &snapshot.opponent);

        // -- Mana (own) --
        for &m in &snapshot.player.mana_available {
            features.push(m as f32 / 10.0); // normalize
        }
        // -- Mana (opponent) --
        for &m in &snapshot.opponent.mana_available {
            features.push(m as f32 / 10.0);
        }

        // -- Phase one-hot --
        let step_idx = snapshot.current_step.index() as usize;
        for i in 0..PHASE_FEATURES {
            features.push(if i == step_idx { 1.0 } else { 0.0 });
        }

        // -- Active player flag --
        features.push(if snapshot.is_active_player { 1.0 } else { 0.0 });

        // -- Turn number (normalized by /50 so early turns are ~0.02-0.4) --
        features.push(snapshot.turn_number as f32 / 50.0);

        // -- Own permanents (padded to MAX_PERMANENTS_PER_SIDE) --
        encode_permanents(&mut features, &snapshot.own_permanents);
        // -- Opponent permanents --
        encode_permanents(&mut features, &snapshot.opponent_permanents);

        // -- Hand cards (padded to MAX_HAND_SIZE) --
        for i in 0..MAX_HAND_SIZE {
            if i < snapshot.hand.len() {
                encode_hand_card(&mut features, &snapshot.hand[i]);
            } else {
                features.extend(std::iter::repeat_n(0.0, HAND_CARD_FEATURES));
            }
        }

        // -- Stack items (padded to MAX_STACK_SIZE) --
        for i in 0..MAX_STACK_SIZE {
            if i < snapshot.stack.len() {
                encode_stack_item(&mut features, &snapshot.stack[i]);
            } else {
                features.extend(std::iter::repeat_n(0.0, STACK_ITEM_FEATURES));
            }
        }

        debug_assert_eq!(
            features.len(),
            OBSERVATION_SIZE,
            "Observation size mismatch: expected {}, got {}",
            OBSERVATION_SIZE,
            features.len()
        );

        Observation { features }
    }

    /// Return the observation as a slice for passing to neural networks.
    pub fn as_slice(&self) -> &[f32] {
        &self.features
    }
}

fn encode_player_info(out: &mut Vec<f32>, player: &PlayerSnapshot) {
    out.push(player.life as f32 / 20.0); // normalize around starting life
    out.push(player.poison_counters as f32 / 10.0);
    out.push(player.energy_counters as f32 / 10.0);
    out.push(player.land_count as f32 / 10.0);
    out.push(player.creature_count as f32 / 10.0);
    out.push(player.hand_size as f32 / 7.0); // normalize around starting hand
    out.push(player.library_size as f32 / 60.0); // normalize around starting library
    out.push(player.graveyard_size as f32 / 20.0);
}

fn encode_permanents(out: &mut Vec<f32>, permanents: &[PermanentSnapshot]) {
    for i in 0..MAX_PERMANENTS_PER_SIDE {
        if i < permanents.len() {
            encode_permanent(out, &permanents[i]);
        } else {
            out.extend(std::iter::repeat_n(0.0, PERMANENT_FEATURES));
        }
    }
}

fn encode_permanent(out: &mut Vec<f32>, perm: &PermanentSnapshot) {
    out.push(perm.power as f32 / 10.0);
    out.push(perm.toughness as f32 / 10.0);
    out.push(if perm.tapped { 1.0 } else { 0.0 });
    out.push(if perm.summoning_sickness { 1.0 } else { 0.0 });
    out.push(if perm.is_creature { 1.0 } else { 0.0 });
    out.push(if perm.is_land { 1.0 } else { 0.0 });
    out.push(if perm.is_artifact { 1.0 } else { 0.0 });
    out.push(if perm.is_enchantment { 1.0 } else { 0.0 });
    out.push(if perm.is_planeswalker { 1.0 } else { 0.0 });
    out.push(if perm.is_attacking { 1.0 } else { 0.0 });
    out.push(if perm.is_blocking { 1.0 } else { 0.0 });
    out.push(perm.damage_marked as f32 / 10.0);
    // Keyword bits: 16 bits encoded as 16 individual floats.
    for bit in 0..16 {
        out.push(if (perm.keyword_bits >> bit) & 1 == 1 {
            1.0
        } else {
            0.0
        });
    }
}

fn encode_hand_card(out: &mut Vec<f32>, card: &HandCardSnapshot) {
    out.push(card.mana_value as f32 / 10.0);
    out.push(if card.is_creature { 1.0 } else { 0.0 });
    out.push(if card.is_instant { 1.0 } else { 0.0 });
    out.push(if card.is_sorcery { 1.0 } else { 0.0 });
    out.push(if card.is_land { 1.0 } else { 0.0 });
    out.push(if card.is_artifact { 1.0 } else { 0.0 });
    out.push(if card.is_enchantment { 1.0 } else { 0.0 });
    out.push(if card.is_planeswalker { 1.0 } else { 0.0 });
    out.push(card.power as f32 / 10.0);
    out.push(card.toughness as f32 / 10.0);
    out.push(if card.castable { 1.0 } else { 0.0 });
}

fn encode_stack_item(out: &mut Vec<f32>, item: &StackItemSnapshot) {
    out.push(if item.is_own { 1.0 } else { 0.0 });
    out.push(item.mana_value as f32 / 10.0);
    out.push(if item.is_creature_spell { 1.0 } else { 0.0 });
    out.push(if item.is_instant { 1.0 } else { 0.0 });
    out.push(if item.is_sorcery { 1.0 } else { 0.0 });
    out.push(if item.is_ability { 1.0 } else { 0.0 });
    out.push(if item.targets_own_permanent { 1.0 } else { 0.0 });
    out.push(if item.targets_opponent_permanent { 1.0 } else { 0.0 });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_snapshot() -> GameSnapshot {
        GameSnapshot {
            player: PlayerSnapshot {
                life: 20,
                hand_size: 7,
                library_size: 53,
                ..Default::default()
            },
            opponent: PlayerSnapshot {
                life: 20,
                hand_size: 7,
                library_size: 53,
                ..Default::default()
            },
            own_permanents: Vec::new(),
            opponent_permanents: Vec::new(),
            hand: Vec::new(),
            stack: Vec::new(),
            current_step: PhaseStep::PrecombatMain,
            is_active_player: true,
            turn_number: 1,
        }
    }

    #[test]
    fn observation_has_correct_size() {
        let obs = Observation::encode(&empty_snapshot());
        assert_eq!(obs.features.len(), OBSERVATION_SIZE);
    }

    #[test]
    fn observation_size_constant_matches() {
        // Manually compute expected size:
        let expected = PLAYER_INFO_FEATURES * 2
            + MANA_FEATURES * 2
            + PHASE_FEATURES
            + 1 // is_active_player
            + 1 // turn_number
            + MAX_PERMANENTS_PER_SIDE * PERMANENT_FEATURES * 2
            + MAX_HAND_SIZE * HAND_CARD_FEATURES
            + MAX_STACK_SIZE * STACK_ITEM_FEATURES;
        assert_eq!(OBSERVATION_SIZE, expected);
    }

    #[test]
    fn life_normalized_correctly() {
        let obs = Observation::encode(&empty_snapshot());
        // First feature is own life/20 = 20/20 = 1.0
        assert!((obs.features[0] - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn phase_one_hot_encoding() {
        let snapshot = empty_snapshot();
        let obs = Observation::encode(&snapshot);
        // Phase starts at offset: 2*PLAYER_INFO + 2*MANA = 2*8 + 2*6 = 28
        let phase_offset = PLAYER_INFO_FEATURES * 2 + MANA_FEATURES * 2;
        let step_idx = PhaseStep::PrecombatMain.index() as usize; // 3
        for i in 0..PHASE_FEATURES {
            let expected = if i == step_idx { 1.0 } else { 0.0 };
            assert!(
                (obs.features[phase_offset + i] - expected).abs() < f32::EPSILON,
                "Phase bit {} expected {} got {}",
                i,
                expected,
                obs.features[phase_offset + i]
            );
        }
    }

    #[test]
    fn permanents_are_padded() {
        let mut snapshot = empty_snapshot();
        snapshot.own_permanents.push(PermanentSnapshot {
            is_creature: true,
            power: 3,
            toughness: 3,
            ..Default::default()
        });
        let obs = Observation::encode(&snapshot);
        assert_eq!(obs.features.len(), OBSERVATION_SIZE);
    }
}
