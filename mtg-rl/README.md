# mtg-rl

A Rust reimplementation of the Magic: The Gathering game engine, designed for reinforcement learning research. Built as a companion to the [XMage](https://github.com/magefree/mage) Java engine in the parent repository.

## Architecture

The workspace contains 5 crates with ~36K lines of Rust across 59 source files:

```
mtg-engine   Core game engine: game loop, state, combat, abilities, effects, zones, mana
mtg-cards    Card factory registry + 1,333 cards across 4 sets
mtg-ai       AI players (random, heuristic, minimax) + Gymnasium environment
mtg-tests    Integration test framework + Criterion benchmarks
mtg-python   PyO3 bindings for Python/Gymnasium RL training
```

**Dependency flow:** `mtg-engine` <- `mtg-cards` <- `mtg-ai` <- `mtg-tests` / `mtg-python`

### mtg-engine

The core game simulation. Key modules:

| Module | Purpose |
|--------|---------|
| `game.rs` | Main `Game` struct with `execute()` game loop and `execute_effects()` effect resolution |
| `abilities.rs` | `Ability`, `Effect` (~35 variants), `StaticEffect`, `Cost`, `TargetSpec` |
| `effects.rs` | `ContinuousEffect`, `ReplacementEffect`, 7-layer effect system |
| `constants.rs` | `Zone`, `PhaseStep`, `CardType`, `SubType`, `KeywordAbilities` (47 keywords as bitflags) |
| `state.rs` | `GameState` snapshots for decision points |
| `combat.rs` | Combat assignment and damage resolution |
| `mana.rs` | `Mana` enum, `ManaCost` parsed from strings like `"{1}{U}{B}"` |
| `permanent.rs` | `Permanent` struct (battlefield objects) |
| `player.rs` | `Player` struct (hand, library, graveyard, mana pool, life) |
| `zones.rs` | Zone management (Library, Hand, Battlefield, Graveyard, Stack, Exile) |
| `decision.rs` | `PlayerDecisionMaker` trait -- the interface AI players implement |

### mtg-cards

Card factory functions registered in `CardRegistry`. Each card is a function `fn(ObjectId, PlayerId) -> CardData` in its set file:

| Set | File | Cards | Description |
|-----|------|-------|-------------|
| FDN | `sets/fdn.rs` | 512 | Foundations (core set) |
| TLA | `sets/tla.rs` | 280 | Avatar: The Last Airbender |
| TDM | `sets/tdm.rs` | 273 | Tarkir: Dragonstorm |
| ECL | `sets/ecl.rs` | 268 | Lorwyn Eclipsed |

`CardRegistry::with_all_sets()` loads all 1,333 cards. Basic lands are registered separately via `basic_lands::register`.

### mtg-ai

| Module | Purpose |
|--------|---------|
| `random_player.rs` | Picks random legal actions |
| `heuristic_player.rs` | Board-presence + life-total evaluation |
| `minimax_player.rs` | Minimax with alpha-beta pruning (adjustable depth) |
| `gym.rs` | Gymnasium-compatible RL environment (`MtgGymEnv`) |
| `action_space.rs` | Action encoding for neural network input |
| `observation.rs` | Game state observation tensor |

### mtg-tests

Declarative test framework (`GameTest` builder) that mirrors XMage's Java `CardTestPlayerBase`. Uses `ScriptedPlayer` for deterministic action sequences.

## Build & Test

```bash
# Type-check (fast feedback loop)
cargo check
cargo check -p mtg-cards

# Build
cargo build
cargo build --release

# Run tests
cargo test --lib

# Benchmarks
cargo bench --bench game_bench
```

## Performance

- ~88 games/sec single-threaded
- ~585 games/sec with rayon parallelism
- All types are `Send + Sync` (verified at compile-time via `static_assertions`)

## Implementation Status

Cards exist at three levels of implementation:

| Level | Description |
|-------|-------------|
| **Complete** | All effects use implemented `Effect` variants. Card works correctly in gameplay. |
| **Partial** | Some effects work but uses `Effect::Custom(...)` or no-op variants for others. Working parts function; broken parts are silently skipped. |
| **Stub** | Stats/keywords only. Special abilities are placeholder `Custom(...)` strings or missing entirely. Card exists as a permanent but abilities don't function. |

Summary across all sets:

| Set | Complete | Partial | Stub | Total |
|-----|----------|---------|------|-------|
| FDN | 95 | 126 | 267 | 512* |
| TLA | 39 | 22 | 219 | 280 |
| TDM | 97 | 115 | 59 | 271 |
| ECL | 56 | 69 | 105 | 230 |
| **Total** | **287** | **332** | **650** | **1,293** |

*FDN has 512 non-basic-land cards + 5 basic lands registered separately.

See `docs/*-remediation.md` for per-card breakdowns and fix instructions.

## Key Dependencies

- **im** -- Immutable data structures for safe state snapshots
- **bitflags** -- Keyword abilities as efficient bit sets
- **rayon** -- Data parallelism for batch game simulation
- **pyo3** -- Python bindings for Gymnasium RL training
- **criterion** -- Benchmarking with HTML reports
- **uuid** -- Object and player identity
- **rand** -- Randomized game elements (shuffling, AI)
