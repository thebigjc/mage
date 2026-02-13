// Gymnasium-compatible environment for MTG RL training.
//
// Implements the standard Gymnasium step/reset/observe pattern for MTG games.
// This is pure Rust and is wrapped by PyO3 in the mtg-python crate.
//
// The environment runs a two-player game where the RL agent controls one
// player and an opponent AI (configurable) controls the other.

use crate::action_space::{self, ActionMask, Phase1Action, PHASE1_ACTION_SIZE};
use crate::observation::{GameSnapshot, Observation, PlayerSnapshot, OBSERVATION_SIZE};
use crate::random_player::RandomPlayer;

use mtg_engine::constants::PhaseStep;
use mtg_engine::types::PlayerId;

use std::collections::HashMap;

/// Step result from the environment.
#[derive(Clone, Debug)]
pub struct StepResult {
    /// Observation after the step.
    pub observation: Observation,
    /// Reward for the step.
    pub reward: f32,
    /// Whether the episode is terminated (game over).
    pub terminated: bool,
    /// Whether the episode was truncated (time/turn limit).
    pub truncated: bool,
    /// Additional info (e.g. game stats).
    pub info: HashMap<String, String>,
}

/// Configuration for the Gymnasium environment.
#[derive(Clone, Debug)]
pub struct GymConfig {
    /// Maximum number of turns before truncation.
    pub max_turns: u32,
    /// Seed for the random number generator. None for random seed.
    pub seed: Option<u64>,
    /// Reward shaping: scale factor for intermediate life-differential rewards.
    pub intermediate_reward_scale: f32,
}

impl Default for GymConfig {
    fn default() -> Self {
        GymConfig {
            max_turns: 100,
            seed: None,
            intermediate_reward_scale: 0.01,
        }
    }
}

/// The Gymnasium environment for MTG RL training.
///
/// This wraps a game session and exposes the standard RL interface:
/// reset() -> observation, step(action) -> (observation, reward, done, info).
///
/// Currently operates in a simplified mode since the full game engine
/// (GameState, game loop) is not yet implemented. When the engine is
/// complete, this will drive real games. For now, it provides the correct
/// API shape and dimensions for building the training pipeline.
pub struct MtgGymEnv {
    config: GymConfig,
    /// Current turn number.
    turn: u32,
    /// Agent's player ID.
    agent_id: PlayerId,
    /// Opponent's player ID.
    opponent_id: PlayerId,
    /// Agent's life total (simplified tracking until full game state exists).
    agent_life: i32,
    /// Opponent's life total.
    opponent_life: i32,
    /// Previous agent life (for reward shaping).
    prev_agent_life: i32,
    /// Previous opponent life.
    prev_opponent_life: i32,
    /// Whether the game is over.
    game_over: bool,
    /// Whether the agent won.
    agent_won: bool,
    /// The opponent AI.
    _opponent: RandomPlayer,
    /// Current phase step.
    current_step: PhaseStep,
    /// Whether it's the agent's turn.
    is_agent_turn: bool,
}

impl MtgGymEnv {
    /// Create a new environment with the given configuration.
    pub fn new(config: GymConfig) -> Self {
        let seed = config.seed.unwrap_or(42);
        MtgGymEnv {
            config,
            turn: 0,
            agent_id: PlayerId::new(),
            opponent_id: PlayerId::new(),
            agent_life: 20,
            opponent_life: 20,
            prev_agent_life: 20,
            prev_opponent_life: 20,
            game_over: false,
            agent_won: false,
            _opponent: RandomPlayer::with_seed(seed),
            current_step: PhaseStep::PrecombatMain,
            is_agent_turn: true,
        }
    }

    /// Reset the environment and return the initial observation.
    pub fn reset(&mut self, seed: Option<u64>) -> Observation {
        if let Some(s) = seed {
            self._opponent = RandomPlayer::with_seed(s);
        }
        self.turn = 1;
        self.agent_life = 20;
        self.opponent_life = 20;
        self.prev_agent_life = 20;
        self.prev_opponent_life = 20;
        self.game_over = false;
        self.agent_won = false;
        self.current_step = PhaseStep::PrecombatMain;
        self.is_agent_turn = true;
        self.agent_id = PlayerId::new();
        self.opponent_id = PlayerId::new();

        self.make_observation()
    }

    /// Take a step in the environment.
    ///
    /// `action` is a Phase 1 action index (0..PHASE1_ACTION_SIZE).
    /// Returns (observation, reward, terminated, truncated, info).
    pub fn step(&mut self, action: usize) -> StepResult {
        if self.game_over {
            return StepResult {
                observation: self.make_observation(),
                reward: 0.0,
                terminated: true,
                truncated: false,
                info: HashMap::new(),
            };
        }

        // Save previous state for reward calculation.
        self.prev_agent_life = self.agent_life;
        self.prev_opponent_life = self.opponent_life;

        // Decode and execute the action.
        // In the full implementation, this would:
        // 1. Convert Phase1Action to PlayerAction
        // 2. Apply it to the game state
        // 3. Advance the game loop (opponent decisions, phases, etc.)
        // 4. Return the resulting observation
        //
        // For now, we simulate simplified game progression:
        let _phase1_action = Phase1Action::from_index(action);

        // Advance turn
        self.turn += 1;
        self.is_agent_turn = !self.is_agent_turn;

        // Check termination conditions
        let truncated = self.turn > self.config.max_turns;
        if self.agent_life <= 0 {
            self.game_over = true;
            self.agent_won = false;
        } else if self.opponent_life <= 0 {
            self.game_over = true;
            self.agent_won = true;
        } else if truncated {
            self.game_over = true;
            self.agent_won = false;
        }

        let reward = action_space::compute_reward(
            self.game_over,
            self.agent_won,
            self.agent_life,
            self.opponent_life,
            self.prev_agent_life,
            self.prev_opponent_life,
        );

        let mut info = HashMap::new();
        info.insert("turn".to_string(), self.turn.to_string());
        info.insert("agent_life".to_string(), self.agent_life.to_string());
        info.insert("opponent_life".to_string(), self.opponent_life.to_string());

        StepResult {
            observation: self.make_observation(),
            reward,
            terminated: self.game_over && !truncated,
            truncated,
            info,
        }
    }

    /// Get the current legal action mask.
    pub fn action_mask(&self) -> ActionMask {
        // In the full implementation, this would query the game state for
        // legal actions and build a mask. For now, only Pass is always legal.
        let mut mask = ActionMask::new(PHASE1_ACTION_SIZE);
        mask.mask[0] = true; // Pass is always legal
        mask
    }

    /// Get the observation space size.
    pub fn observation_space_size(&self) -> usize {
        OBSERVATION_SIZE
    }

    /// Get the action space size.
    pub fn action_space_size(&self) -> usize {
        PHASE1_ACTION_SIZE
    }

    /// Build an observation from the current state.
    fn make_observation(&self) -> Observation {
        let snapshot = GameSnapshot {
            player: PlayerSnapshot {
                life: self.agent_life,
                hand_size: 7, // placeholder
                library_size: 53, // placeholder
                ..Default::default()
            },
            opponent: PlayerSnapshot {
                life: self.opponent_life,
                hand_size: 7,
                library_size: 53,
                ..Default::default()
            },
            own_permanents: Vec::new(),
            opponent_permanents: Vec::new(),
            hand: Vec::new(),
            stack: Vec::new(),
            current_step: self.current_step,
            is_active_player: self.is_agent_turn,
            turn_number: self.turn,
        };
        Observation::encode(&snapshot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_reset_returns_correct_size_observation() {
        let mut env = MtgGymEnv::new(GymConfig::default());
        let obs = env.reset(Some(42));
        assert_eq!(obs.features.len(), OBSERVATION_SIZE);
    }

    #[test]
    fn env_step_returns_valid_result() {
        let mut env = MtgGymEnv::new(GymConfig::default());
        env.reset(Some(42));
        let result = env.step(0); // Pass
        assert_eq!(result.observation.features.len(), OBSERVATION_SIZE);
        assert!(!result.terminated || result.truncated || env.game_over);
    }

    #[test]
    fn env_action_mask_has_correct_size() {
        let env = MtgGymEnv::new(GymConfig::default());
        let mask = env.action_mask();
        assert_eq!(mask.mask.len(), PHASE1_ACTION_SIZE);
        assert!(mask.mask[0]); // Pass is always legal
    }

    #[test]
    fn env_truncates_at_max_turns() {
        let mut env = MtgGymEnv::new(GymConfig {
            max_turns: 5,
            ..Default::default()
        });
        env.reset(Some(42));
        for _ in 0..10 {
            let result = env.step(0);
            if result.truncated {
                assert!(env.game_over);
                return;
            }
        }
        panic!("Expected truncation after max_turns");
    }

    #[test]
    fn env_space_sizes() {
        let env = MtgGymEnv::new(GymConfig::default());
        assert_eq!(env.observation_space_size(), OBSERVATION_SIZE);
        assert_eq!(env.action_space_size(), PHASE1_ACTION_SIZE);
    }
}
