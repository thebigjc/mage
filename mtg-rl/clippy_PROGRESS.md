# Progress: clippy

Started: Mon Feb 16 10:17:39 AM EST 2026

## Status

IN_PROGRESS

## Analysis

### Scope
Run `cargo clippy --workspace` and remediate all warnings and errors until clippy passes cleanly.

### Current State
- **0 errors** (7 eq_op errors fixed in iteration 1)
- **~68 warnings** in library code across mtg-engine and mtg-cards
- **~21 warnings** in test code (mtg-engine tests)
- mtg-ai, mtg-python, mtg-tests crates have zero issues of their own

## Task List

### Phase 1: Fix Errors (blocks all downstream crates)
- [x] Task 1: Fix 7 `eq_op` errors — duplicate keyword flags in card definitions (fdn.rs:3893, tla.rs:2204/2357/2532/2563/3338, ecl.rs:3826). Look up correct keywords for each card.

### Phase 2: mtg-engine/src/game.rs (53 warnings — biggest file)
- [x] Task 2: Fix ~20 `uninlined_format_args` warnings in game.rs — inline variables into format strings
- [x] Task 3: Fix ~13 `map_or(false, ...)` → `is_some_and(...)` warnings in game.rs
- [x] Task 4: Fix ~5 `unwrap_or_default` warnings in game.rs
- [x] Task 5: Fix 4 `unnecessary_cast` warnings in game.rs (u32→u32, usize→usize)
- [x] Task 6: Fix 2 `collapsible_if` + 2 `collapsible_else_if` warnings in game.rs
- [x] Task 7: Fix 2 `single_match` → `if let` warnings in game.rs
- [x] Task 8: Fix 2 `assign_op_pattern` → `+=` warnings in game.rs
- [x] Task 9: Fix remaining one-off warnings in game.rs: needless_borrow, len_zero, needless_range_loop, let_and_return, manual_map, iter_cloned_collect, useless_format, useless_conversion, manual_pattern_char_comparison
- [x] Task 10: Review `if_same_then_else` at game.rs:684 — was a logic bug: non-creature cards were incorrectly counted when filter contained both "creature" and "card". Fixed by restructuring conditionals.
- [x] Task 11: Fix `only_used_in_recursion` in filters.rs:311 — prefixed `you` → `_you` in `predicate_matches_card` since the parameter is only passed through recursive And/Or/Not arms and never directly used (cards don't have controllers).
- [x] Task 12: Fix `collapsible_if` in combat.rs:234 — collapsed nested `if` for SKULK check into single `&&` condition

### Phase 3: Other mtg-engine files (10 warnings)
- [x] Task 13: Fix `large_enum_variant` in zones.rs:451 — boxed CardData in StackItemKind::Spell (320→~8 bytes)
- [x] Task 14: Fix `uninlined_format_args` in constants.rs, mana.rs, events.rs
- [x] Task 15: Fix `should_implement_trait` in filters.rs:178 — implemented `std::ops::Not` trait for `Predicate`, replaced `.not()` call with `!` operator
- [x] Task 16: Fix remaining filters.rs warnings (map_or → is_some_and)
- [x] Task 17: Fix `unnecessary_lazy_evaluations` in watchers.rs:181 — replaced `.and_then(|_| expr)` with `.and(expr)`

### Phase 4: mtg-cards crate (13 warnings)
- [x] Task 18: Fix `uninlined_format_args` and other warnings in keywords/behold.rs, keywords/blight.rs, keywords/mobilize.rs
- [x] Task 19: Fix `manual_contains` in registry.rs:87
- [x] Task 20: Fix `useless_conversion` in sets/fdn.rs:1643 — removed unnecessary `.into()` on string literal passed to `boost_controlled(&str)`

### Phase 5: Test code (21 warnings)
- [x] Task 21: Fix `uninlined_format_args` warnings in test files — fixed 16 instances across 6 files (mtg-engine tests: abilities.rs, effects.rs, keywords.rs, special_mechanics.rs; mtg-tests: framework.rs, concurrency.rs)
- [x] Task 22: Fix 8 `empty_line_after_outer_attribute` warnings in test files + duplicate `#[cfg(test)]` in game.rs
- [x] Task 23: Fix 2 `map_or` → `is_some_and` warnings in test files (keywords.rs lines 1457, 1725)
- [x] Task 24: Fix `len_zero` warning in test special_mechanics.rs — replaced `.len() >= 1` with `!.is_empty()`
- [x] Task 25: Suppressed `too_many_arguments` on `add_lord_with_boost` test helper in continuous_effects.rs with `#[allow(clippy::too_many_arguments)]`

### Phase 5b: Newly discovered warnings (mtg-cards tests, mtg-ai, mtg-python, mtg-tests)
- [x] Task 29: Fix 6 `uninlined_format_args` in mtg-cards test code (behold.rs, blight.rs, mobilize.rs) — inlined `other` variable in panic! format strings
- [x] Task 30: Fix 3 `manual_repeat_n` warnings in mtg-ai (repeat().take() → repeat_n()) — observation.rs lines 199, 208, 245
- [x] Task 31: Fix 2 `redundant_closure` warnings in mtg-tests — replaced `.map(|seed| run_game(seed))` with `.map(run_game)` in concurrency.rs lines 118, 260
- [x] Task 32: Fix `type_complexity` warning in mtg-python — extracted `StepResult<'py>` type alias for the step return tuple
- [ ] Task 33: Fix `useless_conversion` warning in mtg-python

### Phase 6: Final Verification
- [ ] Task 26: Run `cargo clippy --workspace` and confirm zero errors + zero warnings
- [ ] Task 27: Run `cargo clippy --workspace --tests` and confirm zero errors + zero warnings
- [ ] Task 28: Run `cargo test --lib` and `cargo test --release` to confirm all tests still pass

## Completed This Iteration
- Task 32: Fixed `type_complexity` warning in mtg-python/src/lib.rs — extracted `StepResult<'py>` type alias for the Gymnasium step return tuple `(Vec<f32>, f32, bool, bool, Bound<'py, PyDict>)`. Clippy now shows only 1 warning (useless_conversion, Task 33).

## Notes

1. All 576 engine tests still pass after fixes.
2. Task 2 found 13 uninlined_format_args in game.rs (not ~20 as estimated). Includes 4 with `:?` debug format which inline as `{var:?}`.
5. Task 3 found 11 `map_or(false, ...)` in game.rs (not ~13 as estimated). 2 more in filters.rs covered by Task 16.
3. The `only_used_in_recursion` warning was in filters.rs:311, not combat.rs:234 as originally listed. combat.rs:234 is a `collapsible_if`.
4. mtg-ai has 3 `manual_repeat_n` warnings and mtg-python has 2 warnings (type_complexity, useless_conversion) — these were not in the original task list and should be added.
5. Task 18 only fixed library code in mtg-cards keyword files; the test functions in those same files still have 6 uninlined_format_args warnings.
6. Remaining warnings after Task 24: 14 actual warnings across mtg-cards tests (6), mtg-ai (3), mtg-tests (2), mtg-python (2), mtg-engine tests (1 too_many_arguments).
