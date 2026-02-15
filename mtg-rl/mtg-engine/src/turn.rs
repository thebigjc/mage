// Turn structure and phase/step state machine.
//
// Implements the full MTG turn structure:
//   Beginning Phase: Untap → Upkeep → Draw
//   Pre-combat Main Phase
//   Combat Phase: Beginning of Combat → Declare Attackers → Declare Blockers
//                 → First Strike Damage → Combat Damage → End of Combat
//   Post-combat Main Phase
//   Ending Phase: End Step → Cleanup
//
// Ported from mage.game.turn.Turn / Phase / Step.

use crate::constants::{PhaseStep, TurnPhase};
use crate::types::PlayerId;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// The ordered list of steps in a normal turn.
pub const TURN_STEPS: &[PhaseStep] = &[
    PhaseStep::Untap,
    PhaseStep::Upkeep,
    PhaseStep::Draw,
    PhaseStep::PrecombatMain,
    PhaseStep::BeginCombat,
    PhaseStep::DeclareAttackers,
    PhaseStep::DeclareBlockers,
    PhaseStep::FirstStrikeDamage,
    PhaseStep::CombatDamage,
    PhaseStep::EndCombat,
    PhaseStep::PostcombatMain,
    PhaseStep::EndStep,
    PhaseStep::Cleanup,
];

/// Map a PhaseStep to its parent TurnPhase.
pub fn step_to_phase(step: PhaseStep) -> TurnPhase {
    match step {
        PhaseStep::Untap | PhaseStep::Upkeep | PhaseStep::Draw => TurnPhase::Beginning,
        PhaseStep::PrecombatMain => TurnPhase::PrecombatMain,
        PhaseStep::BeginCombat
        | PhaseStep::DeclareAttackers
        | PhaseStep::DeclareBlockers
        | PhaseStep::FirstStrikeDamage
        | PhaseStep::CombatDamage
        | PhaseStep::EndCombat => TurnPhase::Combat,
        PhaseStep::PostcombatMain => TurnPhase::PostcombatMain,
        PhaseStep::EndStep | PhaseStep::Cleanup => TurnPhase::Ending,
    }
}

/// Steps where players normally receive priority.
/// No priority during Untap or Cleanup (unless triggers fire).
pub fn has_priority(step: PhaseStep) -> bool {
    !matches!(step, PhaseStep::Untap | PhaseStep::Cleanup)
}

/// Steps where the active player can play spells at sorcery speed.
pub fn is_sorcery_speed(step: PhaseStep) -> bool {
    matches!(step, PhaseStep::PrecombatMain | PhaseStep::PostcombatMain)
}

/// Manages turn state, tracking the current position in the turn sequence
/// and managing extra turns.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TurnManager {
    /// The current step index into TURN_STEPS (0-12).
    current_step_index: usize,

    /// Turn order (player IDs). This rotates as turns pass.
    turn_order: Vec<PlayerId>,

    /// Index into turn_order for the *next* normal turn. Extra turns don't
    /// advance this; only normal turn rotation does.
    next_normal_turn_index: usize,

    /// The current active player (may differ from turn order during extra turns).
    current_active_player: PlayerId,

    /// The current turn number (1-based).
    pub turn_number: u32,

    /// Queue of extra turns. Each entry is the player who gets the extra turn.
    /// Extra turns are LIFO (most recently added goes first).
    extra_turns: VecDeque<PlayerId>,

    /// Whether the current turn should be ended early (e.g. "end the turn" effect).
    pub end_turn_requested: bool,

    /// Whether combat happened this turn (for "at end of combat" triggers).
    pub had_combat: bool,

    /// Whether first strike damage step should be included.
    /// Only included if any creature has first strike or double strike.
    pub has_first_strike: bool,
}

impl TurnManager {
    /// Create a new TurnManager for the given players.
    pub fn new(turn_order: Vec<PlayerId>) -> Self {
        let first = turn_order[0];
        TurnManager {
            current_step_index: 0,
            current_active_player: first,
            // Next normal turn goes to player index 1 (the second player).
            next_normal_turn_index: 1 % turn_order.len(),
            turn_number: 1,
            turn_order,
            extra_turns: VecDeque::new(),
            end_turn_requested: false,
            had_combat: false,
            has_first_strike: false,
        }
    }

    /// Get the current phase step.
    pub fn current_step(&self) -> PhaseStep {
        TURN_STEPS[self.current_step_index]
    }

    /// Get the current turn phase.
    pub fn current_phase(&self) -> TurnPhase {
        step_to_phase(self.current_step())
    }

    /// Get the active player (whose turn it is).
    pub fn active_player(&self) -> PlayerId {
        self.current_active_player
    }

    /// Advance to the next step. Returns the new step, or None if the turn is over.
    ///
    /// When the turn ends, the caller should call `next_turn()`.
    pub fn advance_step(&mut self) -> Option<PhaseStep> {
        loop {
            self.current_step_index += 1;

            if self.current_step_index >= TURN_STEPS.len() {
                // Turn is over
                return None;
            }

            let step = TURN_STEPS[self.current_step_index];

            // Skip first strike damage if no creature has first/double strike
            if step == PhaseStep::FirstStrikeDamage && !self.has_first_strike {
                continue;
            }

            // If "end the turn" was requested, skip to cleanup
            if self.end_turn_requested && step != PhaseStep::Cleanup {
                continue;
            }

            return Some(step);
        }
    }

    /// Move to the next turn. Handles extra turns and turn order rotation.
    /// Returns the new active player.
    pub fn next_turn(&mut self) -> PlayerId {
        self.current_step_index = 0;
        self.end_turn_requested = false;
        self.had_combat = false;
        self.has_first_strike = false;
        self.turn_number += 1;

        if let Some(extra_player) = self.extra_turns.pop_front() {
            // Extra turn — active player changes to the extra turn player.
            // next_normal_turn_index does NOT advance (extra turns are inserted).
            self.current_active_player = extra_player;
        } else {
            // Normal turn rotation
            self.current_active_player = self.turn_order[self.next_normal_turn_index];
            self.next_normal_turn_index =
                (self.next_normal_turn_index + 1) % self.turn_order.len();
        }

        self.current_active_player
    }

    /// Add an extra turn for a player. The most recently added extra turn
    /// happens first (LIFO behavior per MTG rules).
    pub fn add_extra_turn(&mut self, player: PlayerId) {
        self.extra_turns.push_front(player);
    }

    /// Request that the current turn end immediately (skip to cleanup).
    /// Used by effects like "End the turn" (Sundial of the Infinite, etc.).
    pub fn request_end_turn(&mut self) {
        self.end_turn_requested = true;
    }

    /// Get the turn order.
    pub fn turn_order(&self) -> &[PlayerId] {
        &self.turn_order
    }

    /// Check if there are pending extra turns.
    pub fn has_extra_turns(&self) -> bool {
        !self.extra_turns.is_empty()
    }
}

#[cfg(test)]
impl TurnManager {
    pub fn set_phase_step(&mut self, step: PhaseStep) {
        for (i, &s) in TURN_STEPS.iter().enumerate() {
            if s == step {
                self.current_step_index = i;
                return;
            }
        }
    }
}

/// Tracks priority passing within a step.
#[derive(Clone, Debug)]
pub struct PriorityTracker {
    /// The player who currently has priority.
    pub current: PlayerId,
    /// Number of consecutive passes.
    pub passes: u32,
    /// Total number of players in the game.
    pub player_count: u32,
}

impl PriorityTracker {
    pub fn new(starting_player: PlayerId, player_count: u32) -> Self {
        PriorityTracker {
            current: starting_player,
            passes: 0,
            player_count,
        }
    }

    /// Record a pass. Returns true if all players have passed in succession
    /// (meaning the step can advance or the top of stack resolves).
    pub fn pass(&mut self) -> bool {
        self.passes += 1;
        self.passes >= self.player_count
    }

    /// Reset the pass count (called when a player takes an action).
    pub fn reset(&mut self) {
        self.passes = 0;
    }

    /// Check if all players have passed.
    pub fn all_passed(&self) -> bool {
        self.passes >= self.player_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_players() -> (PlayerId, PlayerId) {
        (PlayerId::new(), PlayerId::new())
    }

    #[test]
    fn turn_step_sequence() {
        let (p1, p2) = make_players();
        let mut tm = TurnManager::new(vec![p1, p2]);

        assert_eq!(tm.current_step(), PhaseStep::Untap);
        assert_eq!(tm.active_player(), p1);
        assert_eq!(tm.turn_number, 1);

        // Advance through all steps
        let mut steps = vec![PhaseStep::Untap]; // starting step
        while let Some(step) = tm.advance_step() {
            steps.push(step);
        }

        // First strike damage is skipped by default (has_first_strike = false)
        assert!(!steps.contains(&PhaseStep::FirstStrikeDamage));
        assert_eq!(steps.len(), 12); // 13 steps minus FirstStrikeDamage
    }

    #[test]
    fn turn_rotation() {
        let (p1, p2) = make_players();
        let mut tm = TurnManager::new(vec![p1, p2]);

        assert_eq!(tm.active_player(), p1);
        let next = tm.next_turn();
        assert_eq!(next, p2);
        assert_eq!(tm.turn_number, 2);

        let next = tm.next_turn();
        assert_eq!(next, p1);
        assert_eq!(tm.turn_number, 3);
    }

    #[test]
    fn extra_turns() {
        let (p1, p2) = make_players();
        let mut tm = TurnManager::new(vec![p1, p2]);

        // P1's turn. P1 adds an extra turn.
        tm.add_extra_turn(p1);

        // End of p1's turn → extra turn for p1
        let next = tm.next_turn();
        assert_eq!(next, p1);
        assert_eq!(tm.turn_number, 2);

        // After extra turn, normal rotation to p2
        let next = tm.next_turn();
        assert_eq!(next, p2);
        assert_eq!(tm.turn_number, 3);
    }

    #[test]
    fn extra_turns_lifo() {
        let (p1, p2) = make_players();
        let mut tm = TurnManager::new(vec![p1, p2]);

        // Both players get extra turns
        tm.add_extra_turn(p2); // added first
        tm.add_extra_turn(p1); // added second (LIFO, goes first)

        let next = tm.next_turn();
        assert_eq!(next, p1); // most recent extra turn

        let next = tm.next_turn();
        assert_eq!(next, p2); // earlier extra turn

        let next = tm.next_turn();
        assert_eq!(next, p2); // normal rotation
    }

    #[test]
    fn first_strike_step() {
        let (p1, p2) = make_players();
        let mut tm = TurnManager::new(vec![p1, p2]);
        tm.has_first_strike = true;

        let mut steps = vec![PhaseStep::Untap];
        while let Some(step) = tm.advance_step() {
            steps.push(step);
        }

        assert!(steps.contains(&PhaseStep::FirstStrikeDamage));
        assert_eq!(steps.len(), 13); // all 13 steps
    }

    #[test]
    fn end_turn_skips_to_cleanup() {
        let (p1, p2) = make_players();
        let mut tm = TurnManager::new(vec![p1, p2]);

        // Advance to upkeep
        tm.advance_step();
        assert_eq!(tm.current_step(), PhaseStep::Upkeep);

        // Request end turn
        tm.request_end_turn();

        // Next step should be cleanup
        let step = tm.advance_step().unwrap();
        assert_eq!(step, PhaseStep::Cleanup);

        // After cleanup, turn is over
        assert!(tm.advance_step().is_none());
    }

    #[test]
    fn priority_tracking() {
        let (p1, _p2) = make_players();
        let mut tracker = PriorityTracker::new(p1, 2);

        assert!(!tracker.pass()); // One player passed
        assert!(tracker.pass());  // Both players passed

        tracker.reset();
        assert!(!tracker.all_passed());
    }

    #[test]
    fn step_to_phase_mapping() {
        assert_eq!(step_to_phase(PhaseStep::Untap), TurnPhase::Beginning);
        assert_eq!(step_to_phase(PhaseStep::Draw), TurnPhase::Beginning);
        assert_eq!(step_to_phase(PhaseStep::PrecombatMain), TurnPhase::PrecombatMain);
        assert_eq!(step_to_phase(PhaseStep::DeclareAttackers), TurnPhase::Combat);
        assert_eq!(step_to_phase(PhaseStep::PostcombatMain), TurnPhase::PostcombatMain);
        assert_eq!(step_to_phase(PhaseStep::Cleanup), TurnPhase::Ending);
    }

    #[test]
    fn priority_rules() {
        // No priority during untap
        assert!(!has_priority(PhaseStep::Untap));
        // No priority during cleanup (normally)
        assert!(!has_priority(PhaseStep::Cleanup));
        // Priority during upkeep
        assert!(has_priority(PhaseStep::Upkeep));
        // Priority during main phases
        assert!(has_priority(PhaseStep::PrecombatMain));
    }
}
