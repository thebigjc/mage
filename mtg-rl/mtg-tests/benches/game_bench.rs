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
use mtg_engine::decision::PlayerAgent;
use mtg_engine::game::{Game, GameConfig, PlayerConfig};
use mtg_engine::permanent::Permanent;
use mtg_engine::types::{Life, ObjectId, PlayerId, Power, Toughness};

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
    card.power = Some(Power::new(power));
    card.toughness = Some(Toughness::new(toughness));
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
        starting_life: Life::new(20),
    };
    Game::new_two_player(
        config,
        vec![
            (p1, PlayerAgent::new(RandomPlayer::with_seed(42))),
            (p2, PlayerAgent::new(RandomPlayer::with_seed(43))),
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
        starting_life: Life::new(20),
    };
    let mut game = Game::new_two_player(
        config,
        vec![
            (p1, PlayerAgent::new(RandomPlayer::with_seed(42))),
            (p2, PlayerAgent::new(RandomPlayer::with_seed(43))),
        ],
    );

    // Populate the battlefield with permanents to make clone/SBA more realistic
    for i in 0..5 {
        let card = make_creature(&format!("Bear_{i}"), p1, 2, 2);
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);
    }
    for i in 0..5 {
        let card = make_creature(&format!("Bear_{i}"), p2, 2, 2);
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
            make_game,
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
                        starting_life: Life::new(20),
                    };
                    let mut game = Game::new_two_player(
                        config,
                        vec![
                            (p1, PlayerAgent::new(RandomPlayer::with_seed(seed))),
                            (p2, PlayerAgent::new(RandomPlayer::with_seed(seed + 100))),
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
                        starting_life: Life::new(20),
                    };
                    let mut game = Game::new_two_player(
                        config,
                        vec![
                            (p1, PlayerAgent::new(RandomPlayer::with_seed(seed))),
                            (p2, PlayerAgent::new(RandomPlayer::with_seed(seed + 100))),
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

// ---------------------------------------------------------------------------
// Hot-path micro-benchmarks for profiling
// ---------------------------------------------------------------------------

fn bench_filter_evaluation(c: &mut Criterion) {
    use mtg_engine::constants::SubType;
    use mtg_engine::filters::{Filter, Predicate};

    let p1 = PlayerId::new();

    // Build a creature permanent for testing
    let mut card = CardData::new(ObjectId::new(), p1, "Elvish Mystic");
    card.card_types = vec![CardType::Creature];
    card.subtypes = vec![SubType::Elf, SubType::Druid];
    card.power = Some(Power::new(1));
    card.toughness = Some(Toughness::new(1));
    card.keywords = KeywordAbilities::empty();
    let perm = Permanent::new(card, p1);

    let mut group = c.benchmark_group("filter_eval");

    // Simple predicate: just check card type
    let simple_filter = Filter::new("creature", Predicate::creature());
    group.bench_function("simple_creature", |b| {
        b.iter(|| black_box(simple_filter.matches_permanent(&perm, p1)));
    });

    // Compound predicate: "creature you control" (And with 2 predicates)
    let compound_filter = Filter::creature_you_control();
    group.bench_function("creature_you_control", |b| {
        b.iter(|| black_box(compound_filter.matches_permanent(&perm, p1)));
    });

    // Complex predicate: nonland permanent (And + nested Or)
    let complex_filter = Filter::any_nonland_permanent();
    group.bench_function("nonland_permanent", |b| {
        b.iter(|| black_box(complex_filter.matches_permanent(&perm, p1)));
    });

    // Filter clone cost (hot in apply_continuous_effects)
    group.bench_function("filter_clone_simple", |b| {
        b.iter(|| black_box(simple_filter.clone()));
    });
    group.bench_function("filter_clone_compound", |b| {
        b.iter(|| black_box(compound_filter.clone()));
    });
    group.bench_function("filter_clone_complex", |b| {
        b.iter(|| black_box(complex_filter.clone()));
    });

    group.finish();
}

fn bench_battlefield_iteration(c: &mut Criterion) {
    use mtg_engine::filters::Filter;

    let p1 = PlayerId::new();
    let p2 = PlayerId::new();

    let mut group = c.benchmark_group("battlefield_iter");

    // Build battlefield with 20 permanents (realistic mid-game)
    for size in [10, 20, 40] {
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "A".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "B".to_string(), deck: make_deck(p2) },
            ],
            starting_life: Life::new(20),
        };
        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, PlayerAgent::new(RandomPlayer::with_seed(42))),
                (p2, PlayerAgent::new(RandomPlayer::with_seed(43))),
            ],
        );
        for i in 0..(size / 2) {
            let card = make_creature(&format!("Bear_{i}"), p1, 2, 2);
            game.state.battlefield.add(Permanent::new(card, p1));
            let card = make_creature(&format!("Lion_{i}"), p2, 3, 3);
            game.state.battlefield.add(Permanent::new(card, p2));
        }

        let filter = Filter::creature_you_control();

        // Measure: iterate + filter + collect IDs (mirrors find_matching_permanents)
        group.bench_function(format!("filter_collect_{size}"), |b| {
            b.iter(|| {
                let ids: Vec<ObjectId> = game.state.battlefield.iter()
                    .filter(|perm| filter.matches_permanent(perm, p1))
                    .map(|perm| perm.id())
                    .collect();
                black_box(ids);
            });
        });

        // Measure: just iteration count (baseline without filter)
        group.bench_function(format!("iter_count_{size}"), |b| {
            b.iter(|| {
                let count = game.state.battlefield.iter().count();
                black_box(count);
            });
        });
    }

    group.finish();
}

fn bench_string_allocation(c: &mut Criterion) {
    // Measures the string allocation overhead in hot paths
    let mut group = c.benchmark_group("string_alloc");

    // Legend rule: perm.name().to_string() for HashMap key
    let name = "Sheoldred, the Apocalypse";
    group.bench_function("name_to_string", |b| {
        b.iter(|| black_box(name.to_string()));
    });

    // to_lowercase() calls in find_matching_permanents
    let filter_msg = "creature you control";
    group.bench_function("to_lowercase", |b| {
        b.iter(|| black_box(filter_msg.to_lowercase()));
    });

    // Combined: what legend rule does per permanent
    let p1 = PlayerId::new();
    group.bench_function("legend_rule_key", |b| {
        b.iter(|| {
            let key: (PlayerId, String) = (p1, name.to_string());
            let _ = black_box(key);
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
    bench_filter_evaluation,
    bench_battlefield_iteration,
    bench_string_allocation,
);
criterion_main!(benches);
