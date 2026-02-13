// Concurrency validation — compile-time Send+Sync checks, parallel game tests.
//
// Ensures all core types can be safely shared across threads (required for
// rayon-based parallel game simulation and RL training).

// ---------------------------------------------------------------------------
// Compile-time Send + Sync assertions
// ---------------------------------------------------------------------------

// Core types
static_assertions::assert_impl_all!(mtg_engine::types::ObjectId: Send, Sync);
static_assertions::assert_impl_all!(mtg_engine::types::PlayerId: Send, Sync);
static_assertions::assert_impl_all!(mtg_engine::types::AbilityId: Send, Sync);

// Game data types
static_assertions::assert_impl_all!(mtg_engine::card::CardData: Send, Sync);
static_assertions::assert_impl_all!(mtg_engine::permanent::Permanent: Send, Sync);
static_assertions::assert_impl_all!(mtg_engine::mana::Mana: Send, Sync);
static_assertions::assert_impl_all!(mtg_engine::mana::ManaCost: Send, Sync);
static_assertions::assert_impl_all!(mtg_engine::counters::Counters: Send, Sync);
static_assertions::assert_impl_all!(mtg_engine::player::Player: Send, Sync);
static_assertions::assert_impl_all!(mtg_engine::mana_pool::ManaPool: Send, Sync);

// Zone types
static_assertions::assert_impl_all!(mtg_engine::zones::Battlefield: Send, Sync);
static_assertions::assert_impl_all!(mtg_engine::zones::Stack: Send, Sync);
static_assertions::assert_impl_all!(mtg_engine::zones::Exile: Send, Sync);
static_assertions::assert_impl_all!(mtg_engine::zones::CardStore: Send, Sync);

// State
static_assertions::assert_impl_all!(mtg_engine::state::GameState: Send, Sync);

// Ability system
static_assertions::assert_impl_all!(mtg_engine::abilities::Ability: Send, Sync);
static_assertions::assert_impl_all!(mtg_engine::abilities::AbilityStore: Send, Sync);
static_assertions::assert_impl_all!(mtg_engine::abilities::Cost: Send, Sync);
static_assertions::assert_impl_all!(mtg_engine::abilities::Effect: Send, Sync);

// Events
static_assertions::assert_impl_all!(mtg_engine::events::GameEvent: Send, Sync);

// Combat
static_assertions::assert_impl_all!(mtg_engine::combat::CombatState: Send, Sync);

// AI players
static_assertions::assert_impl_all!(mtg_ai::random_player::RandomPlayer: Send, Sync);
static_assertions::assert_impl_all!(mtg_ai::heuristic_player::HeuristicPlayer: Send, Sync);

// Gymnasium environment
static_assertions::assert_impl_all!(mtg_ai::gym::MtgGymEnv: Send, Sync);

// ---------------------------------------------------------------------------
// Runtime concurrency tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use mtg_engine::card::CardData;
    use mtg_engine::constants::{CardType, KeywordAbilities};
    use mtg_engine::game::{Game, GameConfig, PlayerConfig};
    use mtg_engine::types::{ObjectId, PlayerId};
    use mtg_ai::random_player::RandomPlayer;
    use rayon::prelude::*;

    fn make_basic_land(name: &str, owner: PlayerId) -> CardData {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Land];
        card.keywords = KeywordAbilities::empty();
        card
    }

    fn make_creature(name: &str, owner: PlayerId, power: i32, toughness: i32) -> CardData {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(power);
        card.toughness = Some(toughness);
        card.keywords = KeywordAbilities::empty();
        card
    }

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        let mut deck = Vec::new();
        for _ in 0..20 {
            deck.push(make_basic_land("Forest", owner));
        }
        for _ in 0..20 {
            deck.push(make_creature("Grizzly Bears", owner, 2, 2));
        }
        deck
    }

    fn run_game(seed: u64) -> mtg_engine::game::GameResult {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "A".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "B".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(RandomPlayer::with_seed(seed))),
                (p2, Box::new(RandomPlayer::with_seed(seed + 1000))),
            ],
        );
        game.run()
    }

    #[test]
    fn parallel_games_complete_without_panic() {
        // Run 20 games in parallel using rayon. If any game panics
        // (infinite loop, data race, etc.), this test will fail.
        let results: Vec<_> = (0..20u64)
            .into_par_iter()
            .map(|seed| run_game(seed))
            .collect();

        // All games should have completed
        assert_eq!(results.len(), 20);

        // Each game should have a result (either a winner or max turns reached)
        for result in &results {
            assert!(result.turn_number >= 1);
        }
    }

    #[test]
    fn parallel_games_produce_varied_results() {
        // Run games with different seeds to verify they produce different outcomes
        let results: Vec<_> = (0..10u64)
            .into_par_iter()
            .map(|seed| run_game(seed * 17))
            .collect();

        // Check that not all games end on the same turn (would indicate determinism bug)
        let turns: Vec<u32> = results.iter().map(|r| r.turn_number).collect();
        let all_same = turns.windows(2).all(|w| w[0] == w[1]);
        // With random players, it's extremely unlikely all 10 games have the same length
        assert!(
            !all_same,
            "All 10 games ended on the same turn ({}), suggesting randomness issues",
            turns[0]
        );
    }

    #[test]
    fn game_state_clone_is_independent() {
        // Verify that cloning GameState produces an independent copy
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "A".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "B".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(RandomPlayer::with_seed(42))),
                (p2, Box::new(RandomPlayer::with_seed(43))),
            ],
        );

        let original_life = game.state.player(p1).unwrap().life;
        let mut cloned_state = game.state.clone();

        // Modify the cloned state
        cloned_state.player_mut(p1).unwrap().life = 5;

        // Original should be unchanged
        assert_eq!(game.state.player(p1).unwrap().life, original_life);
        assert_eq!(cloned_state.player(p1).unwrap().life, 5);
    }

    #[test]
    fn gymnasium_env_runs_end_to_end() {
        use mtg_ai::gym::{GymConfig, MtgGymEnv};

        let mut env = MtgGymEnv::new(GymConfig {
            max_turns: 50,
            seed: Some(42),
            intermediate_reward_scale: 0.01,
        });

        // Reset
        let obs = env.reset(Some(42));
        assert!(!obs.features.is_empty());

        // Run a full episode: step until terminated or truncated
        let mut steps = 0;
        loop {
            let mask = env.action_mask();
            // Pick the first legal action
            let action = mask.mask.iter().position(|&m| m).unwrap_or(0);
            let result = env.step(action);
            steps += 1;

            if result.terminated || result.truncated {
                break;
            }
            if steps > 200 {
                panic!("Gymnasium env did not terminate within 200 steps");
            }
        }

        assert!(steps > 0, "Episode should have at least one step");
    }

    #[test]
    fn parallel_gymnasium_envs() {
        use mtg_ai::gym::{GymConfig, MtgGymEnv};

        // Run multiple Gymnasium environments in parallel
        let episode_lengths: Vec<u32> = (0..8u64)
            .into_par_iter()
            .map(|seed| {
                let mut env = MtgGymEnv::new(GymConfig {
                    max_turns: 50,
                    seed: Some(seed),
                    intermediate_reward_scale: 0.01,
                });
                env.reset(Some(seed));

                let mut steps = 0u32;
                loop {
                    let result = env.step(0); // Always pass
                    steps += 1;
                    if result.terminated || result.truncated || steps > 200 {
                        break;
                    }
                }
                steps
            })
            .collect();

        assert_eq!(episode_lengths.len(), 8);
        // All episodes should have completed
        for &len in &episode_lengths {
            assert!(len > 0);
        }
    }

    #[test]
    fn throughput_baseline() {
        // Run a batch of games and measure basic throughput.
        // This is not a proper benchmark (use criterion for that), but
        // verifies we can measure games/second and establishes a baseline.
        use std::time::Instant;

        let n_games = 10u64;
        let start = Instant::now();

        let results: Vec<_> = (0..n_games)
            .into_par_iter()
            .map(|seed| run_game(seed))
            .collect();

        let elapsed = start.elapsed();
        let games_per_sec = n_games as f64 / elapsed.as_secs_f64();

        // Just verify it completes and print throughput
        assert_eq!(results.len(), n_games as usize);

        // We expect at least 1 game/second even on slow hardware
        assert!(
            games_per_sec > 1.0,
            "Throughput too low: {:.1} games/sec",
            games_per_sec
        );
    }
}
