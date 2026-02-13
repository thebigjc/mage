// Action space for RL — two-phase action encoding.
//
// MTG has a combinatorially large action space (cast spell X targeting Y with
// mode Z paying mana from sources A, B, C...). Encoding every combination as
// a discrete action is infeasible. Instead, we use a two-phase approach:
//
// Phase 1: Choose an action TYPE from a fixed-size vocabulary.
//   - Pass priority
//   - Cast spell (indexed by hand position 0..MAX_HAND)
//   - Activate ability (indexed by permanent position 0..MAX_PERMANENTS,
//     ability index 0..MAX_ABILITIES_PER_PERM)
//   - Play land (indexed by hand position 0..MAX_HAND)
//
// Phase 2: Choose action PARAMETERS (targets, modes, etc.)
//   - Target selection from legal targets (indexed 0..MAX_TARGETS)
//   - Mode selection for modal spells
//
// Each phase uses an action mask to filter to legal options only.

use mtg_engine::decision::PlayerAction;
use mtg_engine::types::{AbilityId, ObjectId};

// ---------------------------------------------------------------------------
// Dimension constants
// ---------------------------------------------------------------------------

/// Max cards in hand for action indexing.
pub const MAX_HAND_ACTIONS: usize = 15;
/// Max permanents for ability activation indexing.
pub const MAX_PERMANENT_ACTIONS: usize = 50;
/// Max abilities per permanent.
pub const MAX_ABILITIES_PER_PERMANENT: usize = 4;
/// Max targets to select from.
pub const MAX_TARGET_ACTIONS: usize = 50;
/// Max modes for modal spells.
pub const MAX_MODE_ACTIONS: usize = 5;

/// Total Phase 1 action space size.
/// Layout: [Pass(1), CastSpell(MAX_HAND), PlayLand(MAX_HAND),
///          ActivateAbility(MAX_PERMANENT * MAX_ABILITIES)]
pub const PHASE1_ACTION_SIZE: usize =
    1 + MAX_HAND_ACTIONS + MAX_HAND_ACTIONS + MAX_PERMANENT_ACTIONS * MAX_ABILITIES_PER_PERMANENT;

/// Total Phase 2 action space size (target/mode selection).
/// Layout: [targets(MAX_TARGET), modes(MAX_MODE), done(1)]
pub const PHASE2_ACTION_SIZE: usize = MAX_TARGET_ACTIONS + MAX_MODE_ACTIONS + 1;

// ---------------------------------------------------------------------------
// Phase 1 action type
// ---------------------------------------------------------------------------

/// An action in Phase 1 (what to do).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase1Action {
    /// Index 0: pass priority.
    Pass,
    /// Indices 1..=MAX_HAND: cast spell from hand position.
    CastSpell { hand_index: usize },
    /// Indices after CastSpell: play land from hand position.
    PlayLand { hand_index: usize },
    /// Indices after PlayLand: activate ability on permanent.
    ActivateAbility {
        permanent_index: usize,
        ability_index: usize,
    },
}

impl Phase1Action {
    /// Convert to a flat action index (0..PHASE1_ACTION_SIZE).
    pub fn to_index(self) -> usize {
        match self {
            Phase1Action::Pass => 0,
            Phase1Action::CastSpell { hand_index } => 1 + hand_index,
            Phase1Action::PlayLand { hand_index } => 1 + MAX_HAND_ACTIONS + hand_index,
            Phase1Action::ActivateAbility {
                permanent_index,
                ability_index,
            } => {
                1 + MAX_HAND_ACTIONS
                    + MAX_HAND_ACTIONS
                    + permanent_index * MAX_ABILITIES_PER_PERMANENT
                    + ability_index
            }
        }
    }

    /// Convert from a flat action index back to a Phase1Action.
    pub fn from_index(index: usize) -> Option<Phase1Action> {
        if index == 0 {
            return Some(Phase1Action::Pass);
        }
        let index = index - 1;
        if index < MAX_HAND_ACTIONS {
            return Some(Phase1Action::CastSpell { hand_index: index });
        }
        let index = index - MAX_HAND_ACTIONS;
        if index < MAX_HAND_ACTIONS {
            return Some(Phase1Action::PlayLand { hand_index: index });
        }
        let index = index - MAX_HAND_ACTIONS;
        if index < MAX_PERMANENT_ACTIONS * MAX_ABILITIES_PER_PERMANENT {
            let permanent_index = index / MAX_ABILITIES_PER_PERMANENT;
            let ability_index = index % MAX_ABILITIES_PER_PERMANENT;
            return Some(Phase1Action::ActivateAbility {
                permanent_index,
                ability_index,
            });
        }
        None
    }
}

// ---------------------------------------------------------------------------
// Phase 2 action type
// ---------------------------------------------------------------------------

/// An action in Phase 2 (parameterization: target/mode selection).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase2Action {
    /// Select target at index 0..MAX_TARGET.
    SelectTarget { target_index: usize },
    /// Select mode at index 0..MAX_MODE.
    SelectMode { mode_index: usize },
    /// Done selecting (confirm choices).
    Done,
}

impl Phase2Action {
    pub fn to_index(self) -> usize {
        match self {
            Phase2Action::SelectTarget { target_index } => target_index,
            Phase2Action::SelectMode { mode_index } => MAX_TARGET_ACTIONS + mode_index,
            Phase2Action::Done => MAX_TARGET_ACTIONS + MAX_MODE_ACTIONS,
        }
    }

    pub fn from_index(index: usize) -> Option<Phase2Action> {
        if index < MAX_TARGET_ACTIONS {
            return Some(Phase2Action::SelectTarget {
                target_index: index,
            });
        }
        let index = index - MAX_TARGET_ACTIONS;
        if index < MAX_MODE_ACTIONS {
            return Some(Phase2Action::SelectMode { mode_index: index });
        }
        if index == MAX_MODE_ACTIONS {
            return Some(Phase2Action::Done);
        }
        None
    }
}

// ---------------------------------------------------------------------------
// Action mask
// ---------------------------------------------------------------------------

/// A boolean mask over the action space. Used to filter out illegal actions.
///
/// Neural networks output logits for all actions; the mask sets illegal
/// actions to -inf before softmax to ensure only legal actions are selected.
#[derive(Clone, Debug)]
pub struct ActionMask {
    pub mask: Vec<bool>,
}

impl ActionMask {
    /// Create a mask where all actions are illegal (all false).
    pub fn new(size: usize) -> Self {
        ActionMask {
            mask: vec![false; size],
        }
    }

    /// Create a Phase 1 mask from a list of legal PlayerActions.
    ///
    /// `hand_ids` maps hand position index -> ObjectId for cards in hand.
    /// `permanent_ids` maps permanent position index -> (ObjectId, ability_count).
    /// These mappings are needed to translate between the engine's ObjectId-based
    /// actions and the RL agent's index-based actions.
    pub fn phase1_from_legal_actions(
        legal_actions: &[PlayerAction],
        hand_ids: &[ObjectId],
        permanent_abilities: &[(ObjectId, Vec<AbilityId>)], // (perm_id, [ability_ids])
    ) -> Self {
        let mut mask = ActionMask::new(PHASE1_ACTION_SIZE);

        for action in legal_actions {
            match action {
                PlayerAction::Pass => {
                    mask.mask[0] = true;
                }
                PlayerAction::CastSpell { card_id, .. } => {
                    if let Some(idx) = hand_ids.iter().position(|id| id == card_id) {
                        if idx < MAX_HAND_ACTIONS {
                            mask.mask[Phase1Action::CastSpell { hand_index: idx }.to_index()] =
                                true;
                        }
                    }
                }
                PlayerAction::PlayLand { card_id } => {
                    if let Some(idx) = hand_ids.iter().position(|id| id == card_id) {
                        if idx < MAX_HAND_ACTIONS {
                            mask.mask[Phase1Action::PlayLand { hand_index: idx }.to_index()] = true;
                        }
                    }
                }
                PlayerAction::ActivateAbility {
                    source_id,
                    ability_id,
                    ..
                } => {
                    if let Some(perm_idx) = permanent_abilities
                        .iter()
                        .position(|(pid, _)| pid == source_id)
                    {
                        if perm_idx < MAX_PERMANENT_ACTIONS {
                            let (_, abilities) = &permanent_abilities[perm_idx];
                            if let Some(ab_idx) =
                                abilities.iter().position(|aid| aid == ability_id)
                            {
                                if ab_idx < MAX_ABILITIES_PER_PERMANENT {
                                    mask.mask[Phase1Action::ActivateAbility {
                                        permanent_index: perm_idx,
                                        ability_index: ab_idx,
                                    }
                                    .to_index()] = true;
                                }
                            }
                        }
                    }
                }
                PlayerAction::ActivateManaAbility { .. }
                | PlayerAction::SpecialAction { .. } => {
                    // Mana abilities and special actions are handled separately
                    // (Phase 2 or special decision methods), not in Phase 1.
                }
            }
        }

        mask
    }

    /// Create a Phase 2 target selection mask.
    pub fn phase2_target_mask(legal_target_indices: &[usize], allow_done: bool) -> Self {
        let mut mask = ActionMask::new(PHASE2_ACTION_SIZE);
        for &idx in legal_target_indices {
            if idx < MAX_TARGET_ACTIONS {
                mask.mask[idx] = true;
            }
        }
        if allow_done {
            mask.mask[Phase2Action::Done.to_index()] = true;
        }
        mask
    }

    /// Create a Phase 2 mode selection mask.
    pub fn phase2_mode_mask(legal_mode_indices: &[usize]) -> Self {
        let mut mask = ActionMask::new(PHASE2_ACTION_SIZE);
        for &idx in legal_mode_indices {
            if idx < MAX_MODE_ACTIONS {
                mask.mask[MAX_TARGET_ACTIONS + idx] = true;
            }
        }
        mask
    }

    /// Convert mask to f32 for neural network input (1.0 = legal, 0.0 = illegal).
    pub fn as_f32(&self) -> Vec<f32> {
        self.mask.iter().map(|&b| if b { 1.0 } else { 0.0 }).collect()
    }

    /// Number of legal (true) actions.
    pub fn count_legal(&self) -> usize {
        self.mask.iter().filter(|&&b| b).count()
    }

    /// Whether any action is legal.
    pub fn has_legal_action(&self) -> bool {
        self.mask.iter().any(|&b| b)
    }
}

// ---------------------------------------------------------------------------
// Reward signal
// ---------------------------------------------------------------------------

/// Compute the reward for the RL agent.
///
/// Terminal reward: +1.0 for winning, -1.0 for losing.
/// Intermediate reward: small signal from life differential to encourage
/// the agent to maintain board advantage, scaled down to avoid dominating
/// the terminal reward.
pub fn compute_reward(
    game_over: bool,
    won: bool,
    own_life: i32,
    opponent_life: i32,
    prev_own_life: i32,
    prev_opponent_life: i32,
) -> f32 {
    if game_over {
        return if won { 1.0 } else { -1.0 };
    }

    // Small intermediate reward from life differential change.
    // Positive when we gained relative advantage, negative when we lost it.
    let prev_diff = prev_own_life - prev_opponent_life;
    let curr_diff = own_life - opponent_life;
    let delta = (curr_diff - prev_diff) as f32;
    // Scale to keep intermediate rewards small relative to terminal reward.
    delta * 0.01
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase1_pass_roundtrip() {
        let action = Phase1Action::Pass;
        let idx = action.to_index();
        assert_eq!(idx, 0);
        assert_eq!(Phase1Action::from_index(idx), Some(Phase1Action::Pass));
    }

    #[test]
    fn phase1_cast_spell_roundtrip() {
        let action = Phase1Action::CastSpell { hand_index: 5 };
        let idx = action.to_index();
        assert_eq!(Phase1Action::from_index(idx), Some(action));
    }

    #[test]
    fn phase1_play_land_roundtrip() {
        let action = Phase1Action::PlayLand { hand_index: 3 };
        let idx = action.to_index();
        assert_eq!(Phase1Action::from_index(idx), Some(action));
    }

    #[test]
    fn phase1_activate_ability_roundtrip() {
        let action = Phase1Action::ActivateAbility {
            permanent_index: 10,
            ability_index: 2,
        };
        let idx = action.to_index();
        assert_eq!(Phase1Action::from_index(idx), Some(action));
    }

    #[test]
    fn phase1_all_indices_are_unique() {
        let mut indices = std::collections::HashSet::new();
        indices.insert(Phase1Action::Pass.to_index());
        for i in 0..MAX_HAND_ACTIONS {
            assert!(indices.insert(Phase1Action::CastSpell { hand_index: i }.to_index()));
        }
        for i in 0..MAX_HAND_ACTIONS {
            assert!(indices.insert(Phase1Action::PlayLand { hand_index: i }.to_index()));
        }
        for p in 0..MAX_PERMANENT_ACTIONS {
            for a in 0..MAX_ABILITIES_PER_PERMANENT {
                assert!(indices.insert(
                    Phase1Action::ActivateAbility {
                        permanent_index: p,
                        ability_index: a,
                    }
                    .to_index()
                ));
            }
        }
        assert_eq!(indices.len(), PHASE1_ACTION_SIZE);
    }

    #[test]
    fn phase1_out_of_range_returns_none() {
        assert_eq!(Phase1Action::from_index(PHASE1_ACTION_SIZE), None);
        assert_eq!(Phase1Action::from_index(usize::MAX), None);
    }

    #[test]
    fn phase2_roundtrip() {
        let actions = [
            Phase2Action::SelectTarget { target_index: 0 },
            Phase2Action::SelectTarget { target_index: 49 },
            Phase2Action::SelectMode { mode_index: 0 },
            Phase2Action::SelectMode { mode_index: 4 },
            Phase2Action::Done,
        ];
        for action in actions {
            assert_eq!(Phase2Action::from_index(action.to_index()), Some(action));
        }
    }

    #[test]
    fn action_mask_pass_only() {
        let legal = vec![PlayerAction::Pass];
        let mask =
            ActionMask::phase1_from_legal_actions(&legal, &[], &[]);
        assert!(mask.mask[0]); // Pass is legal
        assert_eq!(mask.count_legal(), 1);
    }

    #[test]
    fn action_mask_with_castable_spell() {
        let card_id = ObjectId::new();
        let legal = vec![
            PlayerAction::Pass,
            PlayerAction::CastSpell {
                card_id,
                targets: vec![],
                mode: None,
                without_mana: false,
            },
        ];
        let hand_ids = vec![card_id];
        let mask = ActionMask::phase1_from_legal_actions(&legal, &hand_ids, &[]);
        assert!(mask.mask[0]); // Pass
        assert!(mask.mask[1]); // CastSpell at hand index 0
        assert_eq!(mask.count_legal(), 2);
    }

    #[test]
    fn reward_win_is_positive() {
        let r = compute_reward(true, true, 20, 0, 20, 5);
        assert!((r - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn reward_loss_is_negative() {
        let r = compute_reward(true, false, 0, 20, 5, 20);
        assert!((r - (-1.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn reward_intermediate_positive_when_gaining_advantage() {
        let r = compute_reward(false, false, 20, 15, 20, 20);
        assert!(r > 0.0, "Should get positive reward when gaining life advantage");
    }

    #[test]
    fn reward_intermediate_negative_when_losing_advantage() {
        let r = compute_reward(false, false, 15, 20, 20, 20);
        assert!(r < 0.0, "Should get negative reward when losing life advantage");
    }
}
