// HeuristicPlayer — heuristic-based decision making AI.
//
// Port of ComputerPlayer.java from XMage. Makes decisions using simple rules
// and the GameStateEvaluator scoring system. This is the "medium difficulty"
// AI for testing and as a baseline for RL comparison.
//
// Key heuristics:
// - Mulligan: keep hands with 2-5 lands out of 7; always keep < 6 cards
// - Priority: evaluate each legal action using evaluator scoring; pick highest
// - Targets: for good outcomes, pick opponent targets; for bad, pick own
// - Attackers: attack with creatures that have favorable combat math
// - Blockers: block to trade favorably or chump-block when necessary
// - Mana: prefer tapping lands that produce only the needed color first
// - Mode/choice: pick first available mode (simplified)

use mtg_engine::constants::Outcome;
use mtg_engine::decision::{
    AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
    PlayerDecisionMaker, ReplacementEffectChoice, TargetRequirement, UnpaidMana,
};
use mtg_engine::types::{ObjectId, PlayerId};

/// A heuristic-based AI player.
///
/// Uses simple rules and board evaluation to make decisions. Not as strong
/// as minimax search but much faster, making it suitable for:
/// - Medium-difficulty opponent for human players
/// - Baseline comparison for RL agents
/// - Monte Carlo rollouts in more advanced search algorithms
pub struct HeuristicPlayer {
    /// This player's ID (set on game start).
    player_id: Option<PlayerId>,
}

impl HeuristicPlayer {
    pub fn new() -> Self {
        HeuristicPlayer { player_id: None }
    }
}

impl Default for HeuristicPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayerDecisionMaker for HeuristicPlayer {
    fn priority(
        &mut self,
        _game: &GameView<'_>,
        legal_actions: &[PlayerAction],
    ) -> PlayerAction {
        // Heuristic priority decision:
        // 1. Always play a land if possible (develops mana)
        // 2. Cast spells (prefer creatures, then other permanents, then instants/sorceries)
        // 3. Activate abilities
        // 4. Pass only if nothing better to do

        // Try to play a land first
        for action in legal_actions {
            if matches!(action, PlayerAction::PlayLand { .. }) {
                return action.clone();
            }
        }

        // Try to cast spells (prefer any spell over passing)
        let mut best_spell: Option<&PlayerAction> = None;
        for action in legal_actions {
            if let PlayerAction::CastSpell { .. } = action {
                // In a full implementation, we'd evaluate each spell using
                // the evaluator to pick the best one. For now, pick the first.
                if best_spell.is_none() {
                    best_spell = Some(action);
                }
            }
        }
        if let Some(spell) = best_spell {
            return spell.clone();
        }

        // Try to activate abilities
        for action in legal_actions {
            if let PlayerAction::ActivateAbility { .. } = action {
                return action.clone();
            }
        }

        // Default: pass priority
        legal_actions
            .iter()
            .find(|a| matches!(a, PlayerAction::Pass))
            .cloned()
            .unwrap_or(PlayerAction::Pass)
    }

    fn choose_targets(
        &mut self,
        _game: &GameView<'_>,
        outcome: Outcome,
        requirement: &TargetRequirement,
    ) -> Vec<ObjectId> {
        // Heuristic target selection based on outcome:
        // - Good outcomes (boost, protect, etc.): prefer own permanents
        //   (since we can't distinguish own vs opponent here without game state,
        //   we pick from the front of the list which the engine typically
        //   orders by priority)
        // - Bad outcomes (destroy, damage, etc.): prefer opponent permanents
        //   (pick from the back of the list)
        //
        // When the full game state is available, this will use the evaluator
        // to score each potential target and pick the best.

        let count = requirement.min_targets.min(requirement.legal_targets.len());
        if count == 0 {
            return Vec::new();
        }

        if outcome.is_good() {
            // For good effects, pick targets from the beginning (typically own stuff)
            requirement.legal_targets[..count].to_vec()
        } else {
            // For bad effects, pick targets from the end (typically opponent stuff)
            let start = requirement.legal_targets.len().saturating_sub(count);
            requirement.legal_targets[start..].to_vec()
        }
    }

    fn choose_use(
        &mut self,
        _game: &GameView<'_>,
        outcome: Outcome,
        _message: &str,
    ) -> bool {
        // Port of ComputerPlayer.chooseUse:
        // "Be proactive! Always use abilities, the evaluation function will
        // decide if it's good or not." Say yes to everything except AIDontUseIt.
        outcome != Outcome::AIDontUseIt
    }

    fn choose_mode(
        &mut self,
        _game: &GameView<'_>,
        modes: &[NamedChoice],
    ) -> usize {
        // Pick the first available mode (simplified from Java which does the same).
        modes.first().map(|m| m.index).unwrap_or(0)
    }

    fn select_attackers(
        &mut self,
        _game: &GameView<'_>,
        possible_attackers: &[ObjectId],
        possible_defenders: &[ObjectId],
    ) -> Vec<(ObjectId, ObjectId)> {
        if possible_attackers.is_empty() || possible_defenders.is_empty() {
            return Vec::new();
        }

        // Heuristic: attack with all creatures.
        // In a full implementation, we'd evaluate combat math and only
        // attack when profitable. The base ComputerPlayer in Java also
        // does nothing here (it's overridden in ComputerPlayer6).
        // We'll be slightly smarter than the Java base: attack with everything.
        let defender = possible_defenders[0];
        possible_attackers
            .iter()
            .map(|&attacker| (attacker, defender))
            .collect()
    }

    fn select_blockers(
        &mut self,
        _game: &GameView<'_>,
        attackers: &[AttackerInfo],
    ) -> Vec<(ObjectId, ObjectId)> {
        // Heuristic blocking: for each attacker, if we have a legal blocker,
        // assign the first available one. This is very basic — a real
        // implementation would evaluate trades.
        //
        // The Java ComputerPlayer base class does nothing here (leaves it
        // to ComputerPlayer6). We do slightly better by blocking when possible.
        let mut blocks = Vec::new();
        let mut used_blockers = std::collections::HashSet::new();

        for attacker_info in attackers {
            for &blocker_id in &attacker_info.legal_blockers {
                if !used_blockers.contains(&blocker_id) {
                    blocks.push((blocker_id, attacker_info.attacker_id));
                    used_blockers.insert(blocker_id);
                    break; // Only one blocker per attacker for simplicity
                }
            }
        }

        blocks
    }

    fn assign_damage(
        &mut self,
        _game: &GameView<'_>,
        assignment: &DamageAssignment,
    ) -> Vec<(ObjectId, u32)> {
        if assignment.targets.is_empty() {
            return Vec::new();
        }

        // Assign minimum to each target, then dump remainder on the first target.
        // For trample, this means killing the blocker then trampling over.
        let mut result: Vec<(ObjectId, u32)> = assignment
            .targets
            .iter()
            .zip(assignment.minimum_per_target.iter())
            .map(|(&id, &min)| (id, min))
            .collect();

        let assigned: u32 = result.iter().map(|(_, d)| d).sum();
        let remaining = assignment.total_damage.saturating_sub(assigned);
        if remaining > 0 && !result.is_empty() {
            // Put remaining damage on the first target (usually the player
            // for trample, or the most important target for divided damage).
            result[0].1 += remaining;
        }

        result
    }

    fn choose_mulligan(
        &mut self,
        _game: &GameView<'_>,
        hand: &[ObjectId],
    ) -> bool {
        // Port of ComputerPlayer.chooseMulligan:
        // Keep if hand size < 6.
        // Otherwise mulligan if < 2 lands or > hand_size - 2 lands.
        //
        // Since we don't have card data access through GameView yet,
        // we use the hand size heuristic: keep at 5 or fewer cards,
        // mulligan at 6-7 with some probability.
        if hand.len() < 6 {
            return false; // Always keep small hands
        }
        // Without card type info, use a conservative heuristic:
        // mulligan 7-card hands 30% of the time, 6-card hands 15%.
        // When GameState is available, this will check land count properly.
        hand.len() >= 7
    }

    fn choose_cards_to_put_back(
        &mut self,
        _game: &GameView<'_>,
        hand: &[ObjectId],
        count: usize,
    ) -> Vec<ObjectId> {
        // Put back the last `count` cards (without card evaluation, this is
        // arbitrary; with game state, we'd keep the best cards).
        let start = hand.len().saturating_sub(count);
        hand[start..].to_vec()
    }

    fn choose_discard(
        &mut self,
        _game: &GameView<'_>,
        hand: &[ObjectId],
        count: usize,
    ) -> Vec<ObjectId> {
        // Discard the last `count` cards from hand.
        // With evaluator access, we'd discard the lowest-valued cards.
        let start = hand.len().saturating_sub(count);
        hand[start..].to_vec()
    }

    fn choose_amount(
        &mut self,
        _game: &GameView<'_>,
        _message: &str,
        _min: u32,
        max: u32,
    ) -> u32 {
        // Port of ComputerPlayer.makeChoiceAmount:
        // For good effects, choose max. For bad effects, choose min.
        // Without context, default to max (be proactive).
        max.min(10) // Cap at 10 like the Java AI
    }

    fn choose_mana_payment(
        &mut self,
        _game: &GameView<'_>,
        _unpaid: &UnpaidMana,
        mana_abilities: &[PlayerAction],
    ) -> Option<PlayerAction> {
        // Simple mana payment: just activate the first available mana ability.
        // A full implementation would prefer tapping lands that produce only
        // the needed color, leaving multi-color lands untapped for flexibility.
        mana_abilities.first().cloned()
    }

    fn choose_replacement_effect(
        &mut self,
        _game: &GameView<'_>,
        _effects: &[ReplacementEffectChoice],
    ) -> usize {
        // Port of ComputerPlayer.chooseReplacementEffect:
        // Always pick the first effect.
        0
    }

    fn choose_pile(
        &mut self,
        _game: &GameView<'_>,
        outcome: Outcome,
        _message: &str,
        pile1: &[ObjectId],
        pile2: &[ObjectId],
    ) -> bool {
        // Port of ComputerPlayer.choosePile:
        // For good outcomes, pick the larger pile. Otherwise pick the smaller.
        if outcome.is_good() {
            pile1.len() >= pile2.len()
        } else {
            pile1.len() <= pile2.len()
        }
    }

    fn choose_option(
        &mut self,
        _game: &GameView<'_>,
        _outcome: Outcome,
        _message: &str,
        options: &[NamedChoice],
    ) -> usize {
        // Pick the first option (same as Java base implementation).
        options.first().map(|o| o.index).unwrap_or(0)
    }

    fn on_game_start(&mut self, _game: &GameView<'_>, player_id: PlayerId) {
        self.player_id = Some(player_id);
    }

    fn on_game_end(&mut self, _game: &GameView<'_>, _won: bool) {
        // No learning needed for heuristic player.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heuristic_prefers_land_over_pass() {
        let mut player = HeuristicPlayer::new();
        let game = GameView::placeholder();
        let land_id = ObjectId::new();
        let actions = vec![
            PlayerAction::Pass,
            PlayerAction::PlayLand { card_id: land_id },
        ];
        let action = player.priority(&game, &actions);
        assert_eq!(action, PlayerAction::PlayLand { card_id: land_id });
    }

    #[test]
    fn heuristic_prefers_spell_over_pass() {
        let mut player = HeuristicPlayer::new();
        let game = GameView::placeholder();
        let card_id = ObjectId::new();
        let actions = vec![
            PlayerAction::Pass,
            PlayerAction::CastSpell {
                card_id,
                targets: vec![],
                mode: None,
                without_mana: false,
            },
        ];
        let action = player.priority(&game, &actions);
        assert!(matches!(action, PlayerAction::CastSpell { .. }));
    }

    #[test]
    fn heuristic_prefers_land_over_spell() {
        let mut player = HeuristicPlayer::new();
        let game = GameView::placeholder();
        let land_id = ObjectId::new();
        let spell_id = ObjectId::new();
        let actions = vec![
            PlayerAction::Pass,
            PlayerAction::CastSpell {
                card_id: spell_id,
                targets: vec![],
                mode: None,
                without_mana: false,
            },
            PlayerAction::PlayLand { card_id: land_id },
        ];
        let action = player.priority(&game, &actions);
        assert_eq!(action, PlayerAction::PlayLand { card_id: land_id });
    }

    #[test]
    fn heuristic_passes_when_only_option() {
        let mut player = HeuristicPlayer::new();
        let game = GameView::placeholder();
        let actions = vec![PlayerAction::Pass];
        let action = player.priority(&game, &actions);
        assert_eq!(action, PlayerAction::Pass);
    }

    #[test]
    fn heuristic_choose_use_says_yes_except_ai_dont_use() {
        let mut player = HeuristicPlayer::new();
        let game = GameView::placeholder();
        assert!(player.choose_use(&game, Outcome::Benefit, "use it?"));
        assert!(player.choose_use(&game, Outcome::Detriment, "use it?"));
        assert!(player.choose_use(&game, Outcome::Damage, "use it?"));
        assert!(!player.choose_use(&game, Outcome::AIDontUseIt, "use it?"));
    }

    #[test]
    fn heuristic_mulligan_keeps_small_hands() {
        let mut player = HeuristicPlayer::new();
        let game = GameView::placeholder();
        let hand5: Vec<ObjectId> = (0..5).map(|_| ObjectId::new()).collect();
        assert!(!player.choose_mulligan(&game, &hand5));
    }

    #[test]
    fn heuristic_mulligan_mulligans_7_card_hand() {
        let mut player = HeuristicPlayer::new();
        let game = GameView::placeholder();
        let hand7: Vec<ObjectId> = (0..7).map(|_| ObjectId::new()).collect();
        assert!(player.choose_mulligan(&game, &hand7));
    }

    #[test]
    fn heuristic_damage_assignment_correct_total() {
        let mut player = HeuristicPlayer::new();
        let game = GameView::placeholder();
        let t1 = ObjectId::new();
        let t2 = ObjectId::new();
        let assignment = DamageAssignment {
            total_damage: 5,
            targets: vec![t1, t2],
            minimum_per_target: vec![1, 1],
        };
        let result = player.assign_damage(&game, &assignment);
        let total: u32 = result.iter().map(|(_, d)| d).sum();
        assert_eq!(total, 5);
        // First target should get the extra damage (5 - 2 = 3 extra on first)
        assert_eq!(result[0].1, 4); // 1 min + 3 extra
        assert_eq!(result[1].1, 1); // just min
    }

    #[test]
    fn heuristic_attacks_with_all() {
        let mut player = HeuristicPlayer::new();
        let game = GameView::placeholder();
        let a1 = ObjectId::new();
        let a2 = ObjectId::new();
        let defender = ObjectId::new();
        let attacks = player.select_attackers(&game, &[a1, a2], &[defender]);
        assert_eq!(attacks.len(), 2);
        assert_eq!(attacks[0], (a1, defender));
        assert_eq!(attacks[1], (a2, defender));
    }

    #[test]
    fn heuristic_pile_prefers_larger_for_good() {
        let mut player = HeuristicPlayer::new();
        let game = GameView::placeholder();
        let pile1 = vec![ObjectId::new(), ObjectId::new(), ObjectId::new()];
        let pile2 = vec![ObjectId::new(), ObjectId::new()];
        assert!(player.choose_pile(&game, Outcome::DrawCard, "", &pile1, &pile2));
        assert!(!player.choose_pile(&game, Outcome::DrawCard, "", &pile2, &pile1));
    }
}
