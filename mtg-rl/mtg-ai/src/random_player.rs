// RandomPlayer — makes uniformly random legal choices for every decision.
//
// This is the simplest possible AI and serves as a testing baseline.
// Every decision method picks uniformly at random from the legal options.

use mtg_engine::constants::Outcome;
use mtg_engine::decision::{
    AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
    PlayerDecisionMaker, ReplacementEffectChoice, TargetRequirement, UnpaidMana,
};
use mtg_engine::types::{ObjectId, PlayerId};
use rand::prelude::*;

/// A player that makes every decision uniformly at random.
///
/// This is useful for:
/// - Baseline measurement: any smart AI should beat RandomPlayer handily
/// - Stress testing: random actions explore unusual game states
/// - Monte Carlo rollouts: used inside MCTS or evaluation functions
pub struct RandomPlayer {
    rng: StdRng,
}

impl RandomPlayer {
    /// Create a new RandomPlayer with a random seed.
    pub fn new() -> Self {
        RandomPlayer {
            rng: StdRng::from_entropy(),
        }
    }

    /// Create a new RandomPlayer with a deterministic seed for reproducibility.
    pub fn with_seed(seed: u64) -> Self {
        RandomPlayer {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    /// Pick a random element from a non-empty slice.
    fn pick<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.rng.gen_range(0..items.len())]
    }

    /// Pick `count` random elements from a slice without replacement.
    fn pick_n<T: Clone>(&mut self, items: &[T], count: usize) -> Vec<T> {
        let mut indices: Vec<usize> = (0..items.len()).collect();
        indices.shuffle(&mut self.rng);
        indices.truncate(count);
        indices.iter().map(|&i| items[i].clone()).collect()
    }
}

impl Default for RandomPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayerDecisionMaker for RandomPlayer {
    fn priority(
        &mut self,
        _game: &GameView<'_>,
        legal_actions: &[PlayerAction],
    ) -> PlayerAction {
        self.pick(legal_actions).clone()
    }

    fn choose_targets(
        &mut self,
        _game: &GameView<'_>,
        _outcome: Outcome,
        requirement: &TargetRequirement,
    ) -> Vec<ObjectId> {
        // Pick a random count between min and max, then choose that many targets.
        let count = if requirement.min_targets == requirement.max_targets {
            requirement.min_targets
        } else {
            self.rng
                .gen_range(requirement.min_targets..=requirement.max_targets)
        };
        let count = count.min(requirement.legal_targets.len());
        self.pick_n(&requirement.legal_targets, count)
    }

    fn choose_use(
        &mut self,
        _game: &GameView<'_>,
        _outcome: Outcome,
        _message: &str,
    ) -> bool {
        self.rng.gen_bool(0.5)
    }

    fn choose_mode(
        &mut self,
        _game: &GameView<'_>,
        modes: &[NamedChoice],
    ) -> usize {
        self.pick(modes).index
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
        // Randomly decide how many creatures to attack with (0 to all).
        let count = self.rng.gen_range(0..=possible_attackers.len());
        let attackers = self.pick_n(possible_attackers, count);
        // Each attacker attacks a random defender.
        attackers
            .into_iter()
            .map(|a| {
                let d = *self.pick(possible_defenders);
                (a, d)
            })
            .collect()
    }

    fn select_blockers(
        &mut self,
        _game: &GameView<'_>,
        attackers: &[AttackerInfo],
    ) -> Vec<(ObjectId, ObjectId)> {
        let mut blocks = Vec::new();
        // Collect all available blockers across all attackers.
        // Each blocker can only block one attacker, so track used blockers.
        let mut used_blockers = std::collections::HashSet::new();
        for attacker_info in attackers {
            let available: Vec<&ObjectId> = attacker_info
                .legal_blockers
                .iter()
                .filter(|b| !used_blockers.contains(*b))
                .collect();
            if available.is_empty() {
                continue;
            }
            // Randomly decide whether to block this attacker.
            if self.rng.gen_bool(0.5) {
                let blocker = **self.pick(&available);
                used_blockers.insert(blocker);
                blocks.push((blocker, attacker_info.attacker_id));
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
        // First assign minimum damage to each target.
        let mut result: Vec<(ObjectId, u32)> = assignment
            .targets
            .iter()
            .zip(assignment.minimum_per_target.iter())
            .map(|(&id, &min)| (id, min))
            .collect();
        let assigned: u32 = result.iter().map(|(_, d)| d).sum();
        let mut remaining = assignment.total_damage.saturating_sub(assigned);
        // Distribute remaining damage randomly.
        while remaining > 0 {
            let idx = self.rng.gen_range(0..result.len());
            let give = self.rng.gen_range(1..=remaining);
            result[idx].1 += give;
            remaining -= give;
        }
        result
    }

    fn choose_mulligan(
        &mut self,
        _game: &GameView<'_>,
        hand: &[ObjectId],
    ) -> bool {
        // Mulligan if hand has 0-1 lands (rough heuristic even for random).
        // Actually for a truly random player, just flip a coin.
        // But we'll slightly bias toward keeping 7-card hands.
        if hand.len() <= 4 {
            false // Don't mulligan below 5 cards
        } else {
            self.rng.gen_bool(0.3) // 30% chance to mulligan
        }
    }

    fn choose_cards_to_put_back(
        &mut self,
        _game: &GameView<'_>,
        hand: &[ObjectId],
        count: usize,
    ) -> Vec<ObjectId> {
        self.pick_n(hand, count)
    }

    fn choose_discard(
        &mut self,
        _game: &GameView<'_>,
        hand: &[ObjectId],
        count: usize,
    ) -> Vec<ObjectId> {
        self.pick_n(hand, count)
    }

    fn choose_amount(
        &mut self,
        _game: &GameView<'_>,
        _message: &str,
        min: u32,
        max: u32,
    ) -> u32 {
        self.rng.gen_range(min..=max)
    }

    fn choose_mana_payment(
        &mut self,
        _game: &GameView<'_>,
        _unpaid: &UnpaidMana,
        mana_abilities: &[PlayerAction],
    ) -> Option<PlayerAction> {
        if mana_abilities.is_empty() {
            None
        } else {
            Some(self.pick(mana_abilities).clone())
        }
    }

    fn choose_replacement_effect(
        &mut self,
        _game: &GameView<'_>,
        effects: &[ReplacementEffectChoice],
    ) -> usize {
        self.pick(effects).index
    }

    fn choose_pile(
        &mut self,
        _game: &GameView<'_>,
        _outcome: Outcome,
        _message: &str,
        _pile1: &[ObjectId],
        _pile2: &[ObjectId],
    ) -> bool {
        self.rng.gen_bool(0.5)
    }

    fn choose_option(
        &mut self,
        _game: &GameView<'_>,
        _outcome: Outcome,
        _message: &str,
        options: &[NamedChoice],
    ) -> usize {
        self.pick(options).index
    }

    fn on_game_start(&mut self, _game: &GameView<'_>, _player_id: PlayerId) {}

    fn on_game_end(&mut self, _game: &GameView<'_>, _won: bool) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_player_passes_when_only_option() {
        let mut player = RandomPlayer::with_seed(42);
        let game = GameView::placeholder();
        let actions = vec![PlayerAction::Pass];
        let action = player.priority(&game, &actions);
        assert_eq!(action, PlayerAction::Pass);
    }

    #[test]
    fn random_player_chooses_from_legal_actions() {
        let mut player = RandomPlayer::with_seed(42);
        let game = GameView::placeholder();
        let card_id = ObjectId::new();
        let actions = vec![
            PlayerAction::Pass,
            PlayerAction::PlayLand { card_id },
        ];
        // Run multiple times to verify it doesn't crash and picks from the list.
        for _ in 0..20 {
            let action = player.priority(&game, &actions);
            assert!(actions.contains(&action));
        }
    }

    #[test]
    fn random_player_choose_use_returns_bool() {
        let mut player = RandomPlayer::with_seed(42);
        let game = GameView::placeholder();
        let mut saw_true = false;
        let mut saw_false = false;
        for _ in 0..100 {
            let result = player.choose_use(&game, Outcome::Benefit, "test?");
            if result {
                saw_true = true;
            } else {
                saw_false = true;
            }
        }
        assert!(saw_true && saw_false, "should produce both true and false");
    }

    #[test]
    fn random_player_damage_assignment_sums_correctly() {
        let mut player = RandomPlayer::with_seed(42);
        let game = GameView::placeholder();
        let t1 = ObjectId::new();
        let t2 = ObjectId::new();
        let assignment = DamageAssignment {
            total_damage: 5,
            targets: vec![t1, t2],
            minimum_per_target: vec![1, 1],
        };
        for _ in 0..20 {
            let result = player.assign_damage(&game, &assignment);
            let total: u32 = result.iter().map(|(_, d)| d).sum();
            assert_eq!(total, 5);
            for &(_, d) in &result {
                assert!(d >= 1);
            }
        }
    }
}
