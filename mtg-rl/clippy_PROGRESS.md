# Progress: clippy

Started: Mon Feb 16 10:17:39 AM EST 2026

## Status

IN_PROGRESS

## Analysis

### Scope
Run `cargo clippy --workspace` and remediate all warnings and errors until clippy passes cleanly.

### Current State
- **7 errors** (all `clippy::eq_op` — duplicate keyword flags in card definitions)
- **68 warnings** in library code across mtg-engine and mtg-cards
- **21 warnings** in test code (mtg-engine tests)
- **Total: 96 issues** across ~60K lines of Rust
- mtg-ai, mtg-python, mtg-tests crates have zero issues of their own
- No clippy.toml or lint configuration exists yet

### Issues by File (lib code only)
| File | Count | Primary Issues |
|------|-------|----------------|
| mtg-engine/src/game.rs | 53 | uninlined_format_args, map_or→is_some_and, collapsible_if, single_match, casts, etc. |
| mtg-engine/src/filters.rs | 5 | should_implement_trait, format args |
| mtg-cards/src/sets/tla.rs | 5 | eq_op (duplicate keywords) |
| mtg-cards/src/sets/fdn.rs | 2 | eq_op, useless_conversion |
| mtg-cards/src/keywords/behold.rs | 2 | format args, manual_map |
| mtg-engine/src/zones.rs | 1 | large_enum_variant |
| mtg-engine/src/watchers.rs | 1 | unnecessary_lazy_evaluations |
| mtg-engine/src/mana.rs | 1 | uninlined_format_args |
| mtg-engine/src/events.rs | 1 | uninlined_format_args |
| mtg-engine/src/constants.rs | 1 | uninlined_format_args |
| mtg-engine/src/combat.rs | 1 | only_used_in_recursion |
| mtg-cards/src/sets/ecl.rs | 1 | eq_op |
| mtg-cards/src/registry.rs | 1 | manual_contains |
| mtg-cards/src/keywords/mobilize.rs | 1 | uninlined_format_args |
| mtg-cards/src/keywords/blight.rs | 1 | format args |

### Issues by Category (all 96)
| Lint | Count | Auto-fixable? | Risk |
|------|-------|---------------|------|
| uninlined_format_args | ~29 | Yes (trivial) | None |
| map_or→is_some_and | ~15 | Yes (mechanical) | None |
| eq_op (ERRORS) | 7 | Yes (remove duplicate keyword) | Low — need to check correct keywords |
| empty_line_after_outer_attribute | 8 | Yes (remove blank line) | None |
| unwrap_or_default | 5 | Yes (mechanical) | None |
| unnecessary_cast | 4 | Yes (remove cast) | None |
| collapsible_if / collapsible_else_if | 4 | Yes (merge blocks) | Low |
| single_match→if let | 2 | Yes (mechanical) | None |
| manual_map | 2 | Yes (use .map()) | None |
| assign_op_pattern | 2 | Yes (use +=) | None |
| manual_implementation (Option::map) | 2 | Yes | None |
| useless_conversion | 2 | Yes (remove .into()) | None |
| large_enum_variant | 1 | Needs design thought | Medium — Box<CardData> changes API |
| should_implement_trait | 1 | Suppress or rename | Low |
| only_used_in_recursion | 1 | Needs investigation | Medium |
| needless_range_loop | 1 | Yes (use iterator) | Low |
| needless_borrow | 1 | Yes (remove &) | None |
| if_same_then_else | 1 | Needs logic review | Medium |
| len_zero / len_one | 2 | Yes (use is_empty) | None |
| let_and_return | 1 | Yes (inline) | None |
| iter_cloned_collect | 1 | Yes (use to_vec()) | None |
| useless_format | 1 | Yes (remove format!) | None |
| manual_pattern_char_comparison | 1 | Yes | None |
| unnecessary_lazy_evaluations | 1 | Yes | None |
| manual_contains | 1 | Yes | None |
| too_many_arguments | 1 | Suppress (test helper) | None |

### Strategy
1. Fix errors first (eq_op) — these block compilation under clippy
2. Fix mechanical/trivial warnings in bulk by file
3. Handle the few "needs thought" items individually
4. Run tests after each batch to ensure no regressions
5. Final clean pass with `cargo clippy --workspace --tests`

### Risk Assessment
- **Low risk overall** — almost all fixes are mechanical transformations
- **Box<CardData>** for large_enum_variant needs careful thought — it changes how StackItemKind::Spell is constructed/destructured everywhere. May be better to `#[allow]` if the perf impact is negligible.
- **only_used_in_recursion** in combat.rs needs investigation — parameter may actually be needed
- **if_same_then_else** in game.rs:684 — the identical blocks suggest a logic issue that should be reviewed, not just suppressed

## Task List

### Phase 1: Fix Errors (blocks all downstream crates)
- [ ] Task 1: Fix 7 `eq_op` errors — duplicate keyword flags in card definitions (fdn.rs:3893, tla.rs:2204/2357/2532/2563/3338, ecl.rs:3826). Look up correct keywords for each card.

### Phase 2: mtg-engine/src/game.rs (53 warnings — biggest file)
- [ ] Task 2: Fix ~20 `uninlined_format_args` warnings in game.rs — inline variables into format strings
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
- [ ] Task 16: Fix remaining filters.rs warnings (format args, etc.)
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

## Notes

1. **eq_op errors are blocking** — clippy treats these as errors (deny by default), so mtg-cards won't compile under clippy until fixed. Need to look up the correct second keyword for each card.
2. **game.rs dominates** — 53 of 75 lib warnings are in this one 7000+ line file. Batch the fixes by lint type for efficiency.
3. **All fixes are in mtg-engine and mtg-cards** — mtg-ai, mtg-python, mtg-tests have zero clippy issues of their own.
4. **large_enum_variant** is the only structural change — everything else is a surface-level code transformation. Boxing CardData would change how the Spell variant is constructed and destructured in ~10-20 places. Consider `#[allow(clippy::large_enum_variant)]` if profiling doesn't show this as an issue.
5. **Use `sed -i` for bulk mechanical fixes** if the Edit tool races with the linter hook (per MEMORY.md).
6. **Test after each phase** to catch regressions early.
