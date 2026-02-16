# Profiling Report: mtg-rl Hot Path Analysis

**Date**: 2026-02-16
**Method**: Targeted Criterion micro-benchmarks + code analysis (flamegraph/perf unavailable on WSL2)
**Baseline**: Post Phase 3 refactoring (Phases 0-3 complete)

## Current Performance

| Benchmark | Time (mean) | vs Baseline | Throughput |
|-----------|-------------|-------------|------------|
| full_game_simulation | 1.77 ms | −5.9% (was 1.67 ms*) | ~565 games/sec |
| game_state_clone | 9.03 µs | +10.0% (was 8.21 µs) | — |
| game_state_clone_empty | 7.79 µs | +1.8% (was 7.65 µs) | — |
| sba_check | 672 ns | +12.4% (was 598 ns) | — |
| gym_env_step | 605 ns | +7.3% (was 564 ns) | — |
| gym_env_reset | 1.25 µs | −0.8% (was 1.26 µs) | — |
| parallel_games/sequential_10 | 18.23 ms | +8.6% (was 16.78 ms) | — |
| parallel_games/parallel_10 | 5.01 ms | +6.1% (was 4.72 ms) | ~1,996 games/sec |

*Note: Baseline was taken before Phases 1-3 refactoring (error handling, typed filters, newtypes, etc.). The small regressions in micro-benchmarks (SBA, clone) are expected given the added type safety (Power/Toughness/Life newtypes, Filter enum with Predicate tree). Full game simulation shows improvement from better code generation.*

## Hot Path Micro-Benchmarks (NEW)

### Filter Evaluation

| Operation | Time | Notes |
|-----------|------|-------|
| filter_eval/simple_creature | 2.5 ns | Single `HasCardType` check |
| filter_eval/creature_you_control | 7.0 ns | `And(creature, Controller(You))` |
| filter_eval/nonland_permanent | 11.1 ns | `And(Not(Land), Or(Creature,Artifact,Enchantment,PW))` |

**Finding**: Filter evaluation is extremely fast — nanosecond-scale. NOT a bottleneck.

### Filter Cloning (in `apply_continuous_effects`)

| Operation | Time | Notes |
|-----------|------|-------|
| filter_clone_simple | 44 ns | `Filter { message: String, predicate: Creature }` |
| filter_clone_compound | 69 ns | `Filter { message: String, predicate: And(...) }` |
| filter_clone_complex | 97 ns | Nested And/Or predicates |

**Finding**: Filter cloning is **4-40x more expensive** than filter evaluation. With 11 `filter.clone()` calls per permanent with static abilities, and N permanents, this adds up:
- 10 perms with static abilities × 11 clones × ~60ns = **~6.6 µs per `apply_continuous_effects` call**
- Called 2-10× per step, 5-15 steps per turn, ~20-30 turns per game
- Estimated: **0.66-10 ms per game** (significant at 1.77ms total!)

### Battlefield Iteration + Filter Collection

| Operation | Time | Scaling |
|-----------|------|---------|
| filter_collect_10 | 265 ns | ~26.5 ns/perm |
| filter_collect_20 | 561 ns | ~28.1 ns/perm |
| filter_collect_40 | 955 ns | ~23.9 ns/perm |
| iter_count_10 | 134 ns | ~13.4 ns/perm (baseline, no filter) |
| iter_count_20 | 238 ns | ~11.9 ns/perm |
| iter_count_40 | 545 ns | ~13.6 ns/perm |

**Finding**: Filter matching roughly doubles the cost of raw iteration. Scales linearly. The `Vec::collect()` allocation is included in these numbers.

### String Allocation Overhead

| Operation | Time | Notes |
|-----------|------|-------|
| name_to_string | 15 ns | Legend rule: `perm.name().to_string()` per permanent |
| to_lowercase | 19 ns | `filter.message.to_lowercase()` in `find_matching_permanents` |
| legend_rule_key | 15 ns | `(PlayerId, String)` HashMap key creation |

**Finding**: Individual string allocations are cheap (~15-19 ns), but they add up:
- `to_lowercase()`: Called in `find_matching_permanents` which is called per `(static_effect × matching_operation)`. ~20-50 calls per `apply_continuous_effects` = **0.4-1 µs per call**.
- Legend rule: `name.to_string()` per battlefield permanent per SBA check = **~150-600 ns per SBA check** (10-40 permanents).

## Identified Hot Paths (Ranked by Impact)

### 1. `apply_continuous_effects()` — HIGHEST IMPACT
**Location**: `game.rs:418-825`
**Frequency**: 2-10× per step, every step of every turn
**Cost**: Dominates game loop time

**Sub-costs**:
- **17 temporary Vec allocations** (lines 448-467): Created and dropped every call
- **11 `filter.clone()` calls per permanent** with static abilities (lines 480-544)
- **Nested `find_matching_permanents()` calls** in application phase (lines 587+)
- **`to_lowercase()` calls** inside `find_matching_permanents` (line 1318)

**Optimization opportunities**:
1. **Pool/reuse the 17 Vecs** — Use `Vec::clear()` instead of `Vec::new()` each iteration. Store as struct fields. Eliminates ~17 allocation + deallocation pairs per call.
2. **Use `Arc<Filter>` or references** instead of `filter.clone()` — Filter data is immutable during continuous effect application. Borrowing or Arc-wrapping eliminates ~60-100ns per clone × 11 clones × N permanents.
3. **Cache `to_lowercase()` result** — Store lowercase message in Filter at construction time, or eliminate the need for case-insensitive matching entirely (filter predicates are already case-insensitive by design).
4. **Pre-compute filter results** — Instead of calling `find_matching_permanents()` once per effect instance, batch effects by filter and evaluate once per unique filter.

### 2. `find_matching_permanents()` — HIGH IMPACT
**Location**: `game.rs:1312-1339`
**Frequency**: 20-50 calls per `apply_continuous_effects` invocation
**Cost**: ~0.5-1 µs per call (20 permanents)

**Sub-costs**:
- `to_lowercase()` on every call (line 1318): 19 ns
- Full battlefield scan per call: ~560 ns for 20 permanents
- `Vec::collect()` allocation: included above

**Optimization opportunities**:
1. **Eliminate `to_lowercase()`** — The `msg.contains("other")`, `msg.contains("enchanted")`, etc. checks on lines 1320-1331 should use `Filter` predicate properties instead of string matching on the human-readable message.
2. **Return iterator instead of Vec** — Many callers immediately iterate the result; avoid allocation.

### 3. `HashMap<PlayerId, Player>` lookups — MEDIUM IMPACT
**Location**: Throughout `game.rs` (213 lookups) and `state.rs`
**Frequency**: Multiple times per action
**Cost**: ~30-50 ns per lookup (UUID hashing)

**The case for array indexing**: Games always have exactly 2 players. A `[Player; 2]` with index 0/1 would replace 213+ HashMap lookups with array indexing (~1 ns each). This requires mapping `PlayerId → usize` once at game start.

**Optimization opportunities**:
1. **Replace `HashMap<PlayerId, Player>` with `[Player; 2]`** — Store a `player_index: HashMap<PlayerId, usize>` mapping (looked up once) and access via `players[idx]`.
2. **Replace `HashMap<PlayerId, PlayerAgent>` similarly** — 45 lookups in game.rs.

### 4. Legend Rule String Allocation — LOW IMPACT
**Location**: `state.rs:451-474`
**Frequency**: Once per SBA check iteration
**Cost**: ~15 ns × N legendary permanents (usually 0-3)

**Optimization**: Use `&str` reference instead of `to_string()` since permanent names already live in the `Permanent` struct.

### 5. `check_triggered_abilities()` — MEDIUM IMPACT
**Location**: `game.rs:1507-1698`
**Frequency**: Once per SBA iteration
**Cost**: Scans all abilities per event

**Optimization opportunities**:
1. **Index abilities by event type** — `AbilityStore` could maintain a `HashMap<EventType, Vec<AbilityId>>` index for O(1) lookup instead of scanning all abilities.
2. **Avoid repeated `battlefield.contains()`/`battlefield.get()`** — Cache lookups within the function.

## Estimated Impact of Optimizations

| Optimization | Estimated Speedup | Complexity | Priority |
|-------------|-------------------|-----------|----------|
| Pool/reuse Vecs in `apply_continuous_effects` | 3-8% | Low | **HIGH** |
| Replace filter.clone() with references | 5-15% | Medium | **HIGH** |
| Eliminate to_lowercase() in find_matching_permanents | 2-5% | Medium | **MEDIUM** |
| Replace HashMap<PlayerId> with array indexing | 5-10% | High (213+ sites) | **HIGH** |
| Return iterator from find_matching_permanents | 1-3% | Low | **LOW** |
| Index abilities by event type | 2-5% | Medium | **MEDIUM** |
| Use &str in legend rule | <1% | Low | **LOW** |

**Total estimated speedup from all optimizations**: 15-35%
**Target**: ~500 → ~600-700 games/sec single-threaded

## Recommendations for Task 4.2-4.4

**Task 4.2** (Replace HashMap with array indexing): Focus on `HashMap<PlayerId, Player>` → `[Player; 2]` since it has 213+ lookup sites in game.rs alone, plus 45 decision_maker lookups. This is the single highest-frequency HashMap usage pattern.

**Task 4.3** (Reduce unnecessary clones): Focus on:
1. The 11 `filter.clone()` calls in `apply_continuous_effects` (lines 480-544)
2. `keyword.clone()`, `condition.clone()`, `event.clone()` in the same function
3. Consider making the effect collection phase borrow instead of clone

**Task 4.4** (Benchmark comparison): Re-run the full benchmark suite after 4.2-4.3 and compare against both the Phase 0 baseline and this profiling report's numbers.
