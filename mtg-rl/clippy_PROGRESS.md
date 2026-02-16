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
- [ ] Task 3: Fix ~13 `map_or(false, ...)` → `is_some_and(...)` warnings in game.rs
- [ ] Task 4: Fix ~5 `unwrap_or_default` warnings in game.rs
- [ ] Task 5: Fix 4 `unnecessary_cast` warnings in game.rs (u32→u32, usize→usize)
- [ ] Task 6: Fix 2 `collapsible_if` + 2 `collapsible_else_if` warnings in game.rs
- [ ] Task 7: Fix 2 `single_match` → `if let` warnings in game.rs
- [ ] Task 8: Fix 2 `assign_op_pattern` → `+=` warnings in game.rs
- [ ] Task 9: Fix remaining one-off warnings in game.rs: needless_borrow, len_zero, needless_range_loop, let_and_return, manual_map, iter_cloned_collect, useless_format, useless_conversion, manual_pattern_char_comparison, unnecessary_lazy_evaluations
- [ ] Task 10: Review `if_same_then_else` at game.rs:684 — determine if it's a logic bug or just duplicate code, fix accordingly
- [ ] Task 11: Review `only_used_in_recursion` at combat.rs:234 — determine if parameter is needed, fix or suppress
- [ ] Task 12: Run `cargo test --lib` after game.rs fixes to verify no regressions

### Phase 3: Other mtg-engine files (10 warnings)
- [ ] Task 13: Fix `large_enum_variant` in zones.rs:451 — either Box<CardData> in StackItemKind::Spell or `#[allow]` with justification
- [ ] Task 14: Fix `uninlined_format_args` in constants.rs, mana.rs, events.rs
- [ ] Task 15: Fix `should_implement_trait` in filters.rs:178 — rename `not()` or add `#[allow]` with justification
- [ ] Task 16: Fix remaining filters.rs warnings (format args, map_or, only_used_in_recursion)
- [ ] Task 17: Fix `unnecessary_lazy_evaluations` in watchers.rs:181

### Phase 4: mtg-cards crate (13 warnings)
- [ ] Task 18: Fix `uninlined_format_args` and other warnings in keywords/behold.rs, keywords/blight.rs, keywords/mobilize.rs
- [ ] Task 19: Fix `manual_contains` in registry.rs:87
- [ ] Task 20: Fix `useless_conversion` in sets/fdn.rs:1643

### Phase 5: Test code (21 warnings)
- [ ] Task 21: Fix 9 `uninlined_format_args` warnings in test files
- [ ] Task 22: Fix 8 `empty_line_after_outer_attribute` warnings in test files
- [ ] Task 23: Fix 2 `map_or` → `is_some_and` warnings in test files
- [ ] Task 24: Fix `len_one` warning in test files
- [ ] Task 25: Decide on `too_many_arguments` in test files — suppress with `#[allow]` if test helper

### Phase 6: Final Verification
- [ ] Task 26: Run `cargo clippy --workspace` and confirm zero errors + zero warnings
- [ ] Task 27: Run `cargo clippy --workspace --tests` and confirm zero errors + zero warnings
- [ ] Task 28: Run `cargo test --lib` and `cargo test --release` to confirm all tests still pass

## Completed This Iteration
- Task 2: Fixed 13 `uninlined_format_args` warnings in game.rs — inlined variables into format strings (e.g. `format!("{}", x)` → `format!("{x}")`). All 576 tests still pass.

## Notes

1. All 576 engine tests still pass after fixes.
2. Task 2 found 13 uninlined_format_args in game.rs (not ~20 as estimated). Includes 4 with `:?` debug format which inline as `{var:?}`.
3. The `only_used_in_recursion` warning is in filters.rs:311 — `predicate_matches_card` function where `you: PlayerId` is only passed through recursive calls but never used directly.
4. mtg-ai has 3 `manual_repeat_n` warnings and mtg-python has 2 warnings (type_complexity, useless_conversion) — these were not in the original task list and should be added.
