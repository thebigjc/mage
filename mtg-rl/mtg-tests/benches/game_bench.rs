// Criterion benchmarks for the MTG engine.
//
// Measures:
// - GameState clone speed
// - Full game simulation throughput (games/second)
// - Gymnasium environment step throughput
// - SBA check speed
// - Parallel game throughput via rayon

use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};

use mtg_engine::card::CardData;
use mtg_engine::constants::{CardType, KeywordAbilities};
use mtg_engine::game::{Game, GameConfig, PlayerConfig};
use mtg_engine::permanent::Permanent;
use mtg_engine::types::{ObjectId, PlayerId};

use mtg_ai::gym::{GymConfig, MtgGymEnv};
use mtg_ai::random_player::RandomPlayer;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

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

fn make_game() -> Game {
    let p1 = PlayerId::new();
    let p2 = PlayerId::new();
    let config = GameConfig {
        players: vec![
            PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
            PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
        ],
        starting_life: 20,
    };
    Game::new_two_player(
        config,
        vec![
            (p1, Box::new(RandomPlayer::with_seed(42))),
            (p2, Box::new(RandomPlayer::with_seed(43))),
        ],
    )
}

fn make_populated_state() -> Game {
    let p1 = PlayerId::new();
    let p2 = PlayerId::new();
    let config = GameConfig {
        players: vec![
            PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
            PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
        ],
        starting_life: 20,
    };
    let mut game = Game::new_two_player(
        config,
        vec![
            (p1, Box::new(RandomPlayer::with_seed(42))),
            (p2, Box::new(RandomPlayer::with_seed(43))),
        ],
    );

    // Populate the battlefield with permanents to make clone/SBA more realistic
    for i in 0..5 {
        let card = make_creature(&format!("Bear_{}", i), p1, 2, 2);
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);
    }
    for i in 0..5 {
        let card = make_creature(&format!("Bear_{}", i), p2, 2, 2);
        let perm = Permanent::new(card, p2);
        game.state.battlefield.add(perm);
    }
    game
}

// ---------------------------------------------------------------------------
// Benchmarks
// ---------------------------------------------------------------------------

fn bench_game_state_clone(c: &mut Criterion) {
    let game = make_populated_state();
    c.bench_function("game_state_clone", |b| {
        b.iter(|| {
            let _cloned = black_box(game.state.clone());
        });
    });
}

fn bench_game_state_clone_empty(c: &mut Criterion) {
    let game = make_game();
    c.bench_function("game_state_clone_empty", |b| {
        b.iter(|| {
            let _cloned = black_box(game.state.clone());
        });
    });
}

fn bench_sba_check(c: &mut Criterion) {
    let game = make_populated_state();
    c.bench_function("sba_check", |b| {
        b.iter(|| {
            let _sba = black_box(game.state.check_state_based_actions());
        });
    });
}

fn bench_full_game_simulation(c: &mut Criterion) {
    c.bench_function("full_game_simulation", |b| {
        b.iter_batched(
            || make_game(),
            |mut game| {
                let result = game.run();
                black_box(result);
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_gym_env_step(c: &mut Criterion) {
    c.bench_function("gym_env_step", |b| {
        let mut env = MtgGymEnv::new(GymConfig {
            max_turns: 50,
            seed: Some(42),
            intermediate_reward_scale: 0.01,
        });
        env.reset(Some(42));
        b.iter(|| {
            let result = env.step(0); // Pass action
            if result.terminated || result.truncated {
                env.reset(Some(42));
            }
            black_box(result);
        });
    });
}

fn bench_gym_env_reset(c: &mut Criterion) {
    c.bench_function("gym_env_reset", |b| {
        let mut env = MtgGymEnv::new(GymConfig::default());
        b.iter(|| {
            let obs = env.reset(Some(42));
            black_box(obs);
        });
    });
}

fn bench_parallel_games(c: &mut Criterion) {
    use rayon::prelude::*;

    let mut group = c.benchmark_group("parallel_games");

    // Sequential baseline: run N games one at a time
    group.bench_function("sequential_10", |b| {
        b.iter(|| {
            let results: Vec<_> = (0..10)
                .map(|seed| {
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
                            (p2, Box::new(RandomPlayer::with_seed(seed + 100))),
                        ],
                    );
                    game.run()
                })
                .collect();
            black_box(results);
        });
    });

    // Parallel: run N games via rayon
    group.bench_function("parallel_10", |b| {
        b.iter(|| {
            let results: Vec<_> = (0..10u64)
                .into_par_iter()
                .map(|seed| {
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
                            (p2, Box::new(RandomPlayer::with_seed(seed + 100))),
                        ],
                    );
                    game.run()
                })
                .collect();
            black_box(results);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_game_state_clone,
    bench_game_state_clone_empty,
    bench_sba_check,
    bench_full_game_simulation,
    bench_gym_env_step,
    bench_gym_env_reset,
    bench_parallel_games,
);
criterion_main!(benches);
