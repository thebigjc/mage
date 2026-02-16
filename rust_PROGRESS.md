# Progress: rust

Started: Mon Feb 16 10:19:49 AM EST 2026

## Status

IN_PROGRESS

## Analysis

The plan is to convert the mtg-rl Rust workspace from a "Java port wearing Rust syntax" to idiomatic Rust, guided by RUST_REVIEW.md. The review identifies 6 critical issues organized into 4 phases.

### Current State (Baseline)

- **576 engine tests passing**, 19 mtg-tests passing (595 total)
- **~34,000 lines** in mtg-engine, **~21,000 lines** in mtg-cards, **~3,000 lines** in mtg-ai
- **1,328 card implementations** across 4 sets (FDN, TLA, TDM, ECL)
- **Performance**: ~599 games/sec single-threaded, ~2,119/sec parallel (updated from benchmarks)

### String-Based Fallback Scope

| Metric | Count |
|--------|-------|
| `Effect::Custom` | 496 (was 886 before Filter migration) |
| `StaticEffect::Custom` | 126 (was 188 before Filter migration) |
| `SubType::Custom` | 15 |
| String filter fields | 39 definitions, ~250 unique values |
| `TargetSpec::PermanentFiltered` | 62 unique targets |

### Error Handling Scope

| Metric | Count |
|--------|-------|
| `.unwrap()` in production code | 0 (engine) + 38 (cards) = 38 |
| `.unwrap()` in test code | ~507 |
| `.expect()` total | 5 |
| `panic!`/`unreachable!` | 57 |
| `thiserror` dependency | Present but unused |
| `Result<>` return types | 2 (serde only) |

### Architecture Strengths (Don't Break These)

- `im` crate for persistent data structures
- `bitflags` for keyword abilities (48+ keywords)
- `Send + Sync` on all types
- `rayon` parallelization
- `serde` serialization
- No `unsafe` code
- Comprehensive test suite

### Key Constraints

1. **All 576 engine tests must keep passing** after every task
2. **Card factory signature** `fn(ObjectId, PlayerId) -> CardData` is used 1,328 times — changing it is extremely high-impact
3. **PlayerDecisionMaker trait** has 4 implementations — changes propagate widely
4. **GameState has 35 fields** — gradual refactoring, not wholesale replacement
5. **Performance must not regress** — benchmark baseline needed before changes

## Task List

### Phase 0: Infrastructure & Safety Net

- [x] Task 0.1: Create a benchmark baseline snapshot (run `cargo bench` and record numbers for regression testing)
- [x] Task 0.2: Run `cargo clippy` on entire workspace, fix all warnings (establishes clean lint baseline)

### Phase 1: Type System Reform (Highest Impact)

#### 1A: Error Handling Foundation
- [x] Task 1.1: Define `GameError` enum in mtg-engine using `thiserror` (already a dependency). Variants: `InvalidPlayer`, `InvalidObject`, `InvalidZone`, `InvalidTarget`, `InvalidAction`, `GameStateCorruption`, `AbilityResolutionError`
- [x] Task 1.2: Add `EngineResult<T> = Result<T, GameError>` type alias (renamed from `GameResult` to avoid conflict with existing `GameResult` struct)
- [x] Task 1.3: Replace `.unwrap()` calls in game.rs (7 calls) with proper error handling using `?` or `.ok_or(GameError::...)`
- [x] Task 1.4: Replace `.unwrap()` calls in combat.rs, state.rs, and other engine files (~17 calls)
- [x] Task 1.5: Replace `.unwrap()` calls in mtg-cards production code (~38 calls, mostly in registry)
- [x] Task 1.6: Audit and replace `panic!`/`unreachable!` in production code with proper error returns where feasible

#### 1B: Typed Filter System
- [x] Task 1.7: Design and implement `Filter` enum in mtg-engine to replace string-based filters. Start with the most common patterns: creature/permanent type filters, controller filters, power/toughness comparisons
- [x] Task 1.8: Implement `Filter::matches_permanent(&self, perm: &Permanent, state: &GameState) -> bool` evaluation
- [x] Task 1.9: Migrate `matches_filter()` string parsing logic to `Filter` enum evaluation
- [x] Task 1.10: Migrate `DealDamageAll`, `DestroyAll`, `BoostAllUntilEndOfTurn` filter fields from `String` to `Filter`
- [x] Task 1.11: Migrate `Sacrifice`, `SearchLibrary`, `TapCreatures` filter fields from `String` to `Filter`
- [x] Task 1.12: Migrate `TargetSpec::PermanentFiltered(String)` to `TargetSpec::PermanentFiltered(Filter)` (~62 usages)
- [x] Task 1.13: Migrate `CostReduction { filter: String }` to use `Filter` enum (~14 usages)
- [x] Task 1.14: Update card implementations in all 4 set files to use `Filter` enum instead of strings
- [x] Task 1.15: Remove `matches_filter()` string parsing function once all callers migrated

#### 1C: Reduce Custom Effect Fallbacks
- [x] Task 1.16: Audit `Effect::Custom` usages — categorize 496 Effect::Custom + 126 StaticEffect::Custom into groups (see audit below)
- [x] Task 1.17: Add new `Effect` variants for the top 5-10 most common Custom patterns
- [ ] Task 1.18: Migrate card implementations to use new Effect variants, reducing Custom count
- [ ] Task 1.19: Audit `StaticEffect::Custom` usages (126) — categorize and add specific variants for top patterns
- [ ] Task 1.20: Migrate card implementations to use new StaticEffect variants

### Phase 2: Ownership & Patterns

#### 2A: Iterator/Functional Patterns
- [ ] Task 2.1: Replace imperative loops in game.rs with iterator chains where it improves clarity (`.iter().filter().map()` patterns)
- [ ] Task 2.2: Replace imperative loops in combat.rs with iterator chains
- [ ] Task 2.3: Use `Option` and `Result` combinators (`.map()`, `.and_then()`, `.unwrap_or()`) to replace manual match/if-let chains

#### 2B: Newtype Validation
- [ ] Task 2.4: Add validation to `ObjectId`, `PlayerId` constructors (ensure non-nil UUIDs)
- [ ] Task 2.5: Create `Power(i32)`, `Toughness(i32)`, `Life(i32)` newtypes for game values with appropriate `impl`s
- [ ] Task 2.6: Propagate newtypes through CardData, Permanent, and game logic

#### 2C: Enum-Based Dispatch
- [ ] Task 2.7: Replace `Box<dyn PlayerDecisionMaker>` with `PlayerAgent` enum wrapping the 4 known implementations (RandomPlayer, HeuristicPlayer, MinimaxPlayer, ScriptedPlayer) — eliminates vtable overhead
- [ ] Task 2.8: Update game loop and test framework to use `PlayerAgent` enum

### Phase 3: Code Quality

- [ ] Task 3.1: Replace `SubType::Custom(String)` with concrete enum variants for all 15 usages
- [ ] Task 3.2: Add `#[must_use]` annotations to functions returning important values
- [ ] Task 3.3: Use `const` for compile-time card data where possible (mana costs, static strings)
- [ ] Task 3.4: Convert string-based card name lookups in registry to use `&'static str` or interned strings

### Phase 4: Performance

- [ ] Task 4.1: Profile with `cargo flamegraph` to identify hot paths
- [ ] Task 4.2: Replace `HashMap` lookups with array indexing where keys are small integers (player indices)
- [ ] Task 4.3: Reduce unnecessary `.clone()` calls identified by profiling
- [ ] Task 4.4: Run benchmarks and compare against Phase 0 baseline to verify no regression

## Dependencies

- Phase 0 must complete first (baseline + clean lints)
- Task 1.1-1.2 before 1.3-1.6 (need error types before using them)
- Task 1.7-1.8 before 1.9-1.15 (need Filter enum before migration)
- Task 1.16 (audit) before 1.17-1.18 (need to know patterns before adding variants)
- Phase 2 can start after Phase 1A (error handling)
- Phase 3 can proceed in parallel with Phase 2
- Phase 4 should be last (needs stable codebase to profile)

## Notes

- **thiserror** is already a workspace dependency but completely unused — Task 1.1 can start immediately
- **mtg-ai has 0 unwraps** — it's already the most idiomatic crate, can serve as reference
- **ECL set has only 3 Effect::Custom usages** — it's the most recently written and best-implemented set
- The review's Phase 2 "event sourcing" recommendation is too risky for this iteration — it would require rewriting the entire game loop. Focusing on typed filters and error handling gives the biggest idiomaticity improvement with lowest risk.
- **Do not change the card factory signature** — 1,328 call sites is too disruptive. Instead, improve what CardData contains and how effects/filters are expressed.
- The `GameState` 35-field struct could benefit from sub-structs (TurnState, CombatState, etc.) but this is lower priority than type safety improvements.

## Tasks Completed

### Iteration 1 — Task 0.1: Benchmark Baseline
- Ran `cargo bench --bench game_bench` and recorded all results
- Created `mtg-rl/BENCHMARK_BASELINE.md` with full table of results
- Key metrics: 1.67ms/game single-threaded (~599 games/sec), 4.72ms/10 parallel (~2,119 games/sec)
- All 595 tests confirmed passing (576 engine + 19 integration)

### Iteration 2 — Task 0.2: Clippy Clean Baseline
- Fixed 3 remaining warnings in `mtg-tests/benches/game_bench.rs`:
  - 2x `uninlined_format_args` (`format!("Bear_{}", i)` → `format!("Bear_{i}")`)
  - 1x `redundant_closure` (`|| make_game()` → `make_game`)
- Workspace now has zero clippy warnings across all targets
- All 595 tests still passing

### Iteration 3 — Tasks 1.1 + 1.2: GameError Enum & EngineResult Type Alias
- Created `mtg-engine/src/error.rs` with `GameError` enum (7 variants) using `thiserror`
- Added `EngineResult<T> = Result<T, GameError>` type alias (not `GameResult` — that name is taken by the game-outcome struct)
- Added `Display` impl for `AbilityId` (needed by `thiserror` `#[error]` formatting)
- Added `pub mod error` to `lib.rs`
- All 576 engine tests passing, zero clippy warnings, full workspace compiles clean

### Iteration 4 — Task 1.3: Replace .unwrap() in game.rs
- Replaced all 7 `.unwrap()` calls in game.rs with proper error handling:
  - Constructor (`new_two_player`): `.expect("player just inserted into state")` for invariant
  - Block validation (`pop()` after length checks): `.expect()` with clear invariant messages
  - `play_land`: Restructured exile path to use `let Some(...) = ... else { return }` instead of `.unwrap()`
  - `cast_spell`: Changed flashback cost access to use `.map()` + `let Some(...) else { return }`
  - `try_pay_ward_cost`: Restructured discard path to extract card_id via `.and_then().copied()` before mutable borrow
- game.rs now has zero `.unwrap()` calls
- All 576 engine tests passing, zero clippy warnings

### Iteration 5 — Task 1.4: Verify zero .unwrap() in other engine files
- Scanned all 22 production source files in mtg-engine/src/
- Found **zero `.unwrap()` calls in production code** — all remaining calls are exclusively in `#[cfg(test)]` modules
- combat.rs: 0 unwrap calls (never had any in production code)
- state.rs: 6 calls, all in test functions; 4 `.expect()` calls in production code for true invariants (active_player, priority_player, next_player)
- watchers.rs, zones.rs, events.rs, turn.rs: all `.unwrap()` calls only in test modules
- The original estimate of "~17 production calls" was inflated — many were test code all along
- No code changes needed; task confirmed complete by verification
- All 576 engine tests passing, zero clippy warnings

### Iteration 6 — Task 1.5: Verify zero .unwrap() in mtg-cards production code
- Scanned all source files in mtg-cards/src/ (registry.rs, lib.rs, sets/, keywords/)
- Found **zero `.unwrap()` calls in production code** — all 38 calls are exclusively in `#[cfg(test)]` modules (registry tests) or doc comments
- Zero `.expect()` calls in mtg-cards production code
- All 6 `panic!` calls in keywords/ (behold.rs, blight.rs, mobilize.rs) are in `#[cfg(test)]` modules
- The original estimate of "~38 calls, mostly in registry" was inflated — all were test code
- No code changes needed; task confirmed complete by verification
- All 20 mtg-cards tests passing

### Iteration 7 — Task 1.6: Audit panic!/unreachable! in production code
- Comprehensive audit of all `panic!`, `unreachable!`, `unimplemented!`, `todo!` macros across mtg-engine, mtg-cards, and mtg-ai
- Found **zero occurrences in production code** — all instances are exclusively in `#[cfg(test)]` modules
- Remaining `.expect()` calls (7 in production) are for true invariants:
  - `game.rs:140` — player just inserted into state map
  - `game.rs:2413,2420` — `pop()` after explicit `len` checks
  - `state.rs:304,310,315` — active/priority player must exist
  - `state.rs:343` — player must be in turn_order
- These `.expect()` calls are appropriate: they document genuine invariants, not error-prone patterns
- The original estimate of "57 panic!/unreachable!" was inflated — all were test code
- No code changes needed; task confirmed complete by verification
- All 576 engine tests passing, zero clippy warnings

## Completed This Iteration
- Task 1.17: Added 6 new Effect variants and 1 new StaticEffect variant with engine implementations

### Effect::Custom Audit Results (496 usages, 323 unique messages)

**Per-set breakdown:**
| Set | Effect::Custom | StaticEffect::Custom |
|-----|---------------|---------------------|
| FDN | 258 | 57 |
| TLA | 143 | 54 |
| TDM | 92 | 15 |
| ECL | 3 | 0 |

**Categorized groups (by total usages):**

| Category | Usages | Unique | Priority for new variants |
|----------|--------|--------|--------------------------|
| **PLACEHOLDER (no-op descriptions)** | 294 | 10 | HIGHEST — these are stub text like "ETB effect.", "Static effect.", "Activated effect." that carry zero semantic info. Should be eliminated by implementing the actual effect or removing them. |
| **DRAW/DISCARD/MILL/SURVEIL** | 27 | 26 | Medium — many are complex multi-step (surveil, loot, wheel), but some are simple (e.g., "each opponent discards a card") |
| **TOKEN CREATION (complex)** | 25 | 25 | Medium — complex conditions (X tokens, conditional creation), but many could use existing `CreateToken` with compound effects |
| **DEAL DAMAGE (complex)** | 22 | 22 | Medium — many involve conditional amounts or unusual targets |
| **LIFE GAIN/LOSS (complex)** | 19 | 18 | Low — most are compound effects (damage + life gain) |
| **COUNTER MANIPULATION** | 19 | 19 | Medium — distribute counters, conditional counters |
| **EXILE (complex)** | 18 | 18 | Low — most involve temporary exile, or exile-until-leaves |
| **MODAL (choose one/both)** | 15 | 15 | HIGH — already has `Effect::Modal` variant; these should be migrated to use it |
| **LOOK AT TOP / LIBRARY** | 13 | 13 | Low — complex library manipulation |
| **COPY SPELL/TOKEN** | 11 | 11 | Low — very complex game mechanics |
| **DESTROY (complex)** | 10 | 10 | Low — multi-target or conditional destroy |
| **P/T BOOST (team/conditional)** | 8 | 8 | Medium — `BoostAllUntilEndOfTurn` exists but some need "+X/+X where X = count" |
| **RETURN FROM GRAVEYARD** | 8 | 8 | Medium — `Reanimate`/`ReturnFromGraveyard` exist but some need filtering |
| **EVASION / BLOCKING** | 8 | 8 | HIGH — simple variants like "can't be blocked this turn" are easy to add |
| **BOUNCE / RETURN TO HAND** | 7 | 7 | Low — `Bounce`/`BounceAll` exist; complex cases need conditional targeting |
| **SAGA mechanics** | 7 | 2 | Medium — could be a first-class mechanic |
| **GAIN CONTROL** | 7 | 7 | Low — most are complex compound effects |
| **SEARCH LIBRARY (complex)** | 6 | 6 | Low — `SearchLibrary` exists; complex cases involve specific battlefield placement |
| **THRESHOLD condition** | 6 | 6 | Medium — recurring conditional pattern |
| **TAP/UNTAP effects** | 6 | 6 | Low — `TapTarget`/`UntapTarget` exist; complex cases involve "doesn't untap" |
| **CHOOSE CLAN** | 5 | 5 | Low — TDM-specific mechanic |
| **EARTHBEND** | 5 | 2 | Low — TLA-specific mechanic |
| **CAST FROM GRAVEYARD** | 5 | 2 | Medium — flashback-like, recurring pattern |
| **KEYWORD GRANT (team)** | 5 | 4 | HIGH — `GrantKeywordAllUntilEndOfTurn` exists; simple migration |
| **COST REDUCTION** | 5 | 5 | Low — `CostReduction` StaticEffect exists |
| **ENDURE mechanic** | 4 | 4 | Medium — TDM keyword, could be first-class |
| **CHOOSE TYPE** | 4 | 4 | Low — `ChooseCreatureType` exists |
| **ENCHANT / AURA** | 4 | 3 | Low — targeting constraint, not really an effect |
| **COMBAT effects** | 4 | 4 | Low — extra combat phases, complex |
| **KICKER** | 3 | 3 | Low — alternative cost mechanic |
| **MORBID condition** | 3 | 3 | Medium — recurring conditional pattern |
| **PROTECTION** | 2 | 2 | Low — `GainProtection` exists |
| **RAID condition** | 2 | 2 | Low — TLA-specific |
| **SACRIFICE** | 2 | 2 | Low — `Sacrifice` exists |
| **UNCATEGORIZED** | 27 | 27 | Varies — unique one-off effects |

### StaticEffect::Custom Audit Results (126 usages, 39 unique messages)

| Category | Usages | Notes |
|----------|--------|-------|
| **"Static effect." placeholder** | 86 | No-op stub text |
| **"Conditional continuous effect." placeholder** | 4 | No-op stub text |
| **Specific static effects** | 36 | 37 unique messages, 1 usage each — protection, P/T setting, cost reduction, etc. |

### Recommended New Effect Variants for Task 1.17 (highest impact)

1. **Remove placeholders** (294 Effect::Custom + 90 StaticEffect::Custom = 384 total) — These "ETB effect.", "Static effect.", etc. carry no semantic information. The card implementations that use them are essentially stubs. Either:
   - Remove the Custom entirely (if the card is meant to be a stub)
   - Or implement the actual effect using existing variants

2. **`Effect::CantBeBlockedThisTurn`** — 8 usages of "can't be blocked" variants, easy to implement

3. **Migrate 15 modal spells to `Effect::Modal`** — Already has the variant, just needs card-by-card migration

4. **`Effect::BoostAllUntilEndOfTurnDynamic { filter, power_source, toughness_source }`** — For "+X/+X where X = count of something" patterns (~8 usages)

5. **`Effect::EachOpponentDiscards { count }`** — Simple variant, ~5 usages

6. **`Effect::EachOpponentSacrifices { filter }`** — ~3 usages

7. **`Effect::Surveil { count }`** — ~3 usages

8. **`Effect::CantBeBlockedByFilter { filter }`** — For "can't be blocked by Humans" etc.

9. **`Effect::ExileUntilLeaves`** — For Oblivion Ring / Stasis Snare pattern (~5 usages)

10. **`Effect::ReturnFromGraveyardFiltered { filter }`** — For "return creature card with MV <= 3" (~5 usages)

