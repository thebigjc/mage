# Progress: test_errors

Started: Sun Feb 15 05:11:52 PM EST 2026

## Status

IN_PROGRESS

## Analysis

Running `cargo test --lib -p mtg-engine --no-run 2>&1` produces **42 compilation errors** and **120 warnings** across 6 test files. The errors stem from API mismatches — the tests reference methods/types/fields that have been renamed, removed, or have different signatures than the current engine API.

### Error Categories

| Category | Count | Files Affected |
|---|---|---|
| `pass_priority` method missing | 6 calls | game_basics.rs |
| `set_phase_step` method missing | 2 calls | game_basics.rs |
| `DealDamageAny` variant missing | 2 refs | game_basics.rs |
| `AnyTarget` variant missing | 2 refs | game_basics.rs |
| `as_target()` on PlayerId missing | 2 calls | game_basics.rs |
| `Hand::push()` missing (use `add`) | 2 calls | game_basics.rs |
| `Stack` index `[0]` not supported | 2 refs | game_basics.rs |
| `cast_spell` takes 2 args, not 3 | 2 calls | game_basics.rs |
| `graveyard.contains(&x)` ref issue | 2 calls | game_basics.rs |
| `entered_this_turn` field missing | 1 ref | game_basics.rs |
| `activate_ability` signature mismatch | 1 call | game_basics.rs |
| `Outcome` private import via decision | 2 imports | effects.rs, game_basics.rs |
| `gain_control_until_end_of_turn()` missing | 1 call | effects.rs |
| `Library::top_n()` missing (use `peek`) | 1 call | effects.rs |
| `Hand::to_vec()` missing (use `as_slice`) | 1 call | effects.rs |
| `Graveyard::to_vec()` missing (use `as_slice`) | 1 call | effects.rs |
| `look_top_and_pick` takes 2 args, not 4 | 1 call | effects.rs |
| `TurnPhase` undeclared in triggers | 1 ref | triggers.rs |
| `setup()` takes 2 args, not 0 | 2 calls | triggers.rs |
| `combat` name ambiguous | 1 ref | keywords.rs |
| Missing closing `}` before test fn | 1 | keywords.rs |

### Key API Mappings (current engine → what tests expect)

- `Hand::add()` exists → tests use `Hand::push()` (fix: change to `add`)
- `Library::peek(n)` exists → tests use `Library::top_n(n)` (fix: change to `peek`, returns `&[ObjectId]`)
- `Hand::as_slice()` / `Graveyard::as_slice()` exist → tests use `.to_vec()` (fix: change to `.as_slice().to_vec()`)
- `Stack::top()` / `Stack::iter()` exist → tests use `stack[0]` indexing (fix: use `stack.top().unwrap()` or `stack.iter().next().unwrap()`)
- `Permanent::summoning_sick` exists → tests use `entered_this_turn` (fix: change to `summoning_sick`)
- `Effect::GainControlUntilEndOfTurn` exists → tests use `Effect::gain_control_until_end_of_turn()` (fix: use `Effect::gain_control_eot()`)
- `Effect::DealDamage { amount }` exists → tests use `Effect::DealDamageAny { amount }` (fix: use `DealDamage`)
- `TargetSpec::CreatureOrPlayer` exists → tests use `TargetSpec::AnyTarget` (fix: use `CreatureOrPlayer`)
- `Effect::look_top_and_pick(count, filter)` takes 2 args → tests pass 4 (fix: use 2-arg version)
- `cast_spell(player, card)` takes 2 args → tests pass 3 with targets (fix: remove target arg, targets are chosen by decision maker)
- `activate_ability(player, source, ability, &targets)` takes `&[ObjectId]` → tests pass `vec![p2.as_target()]` (fix: targets must be ObjectId not PlayerId, remove as_target)
- `Outcome` is public in `crate::constants` → tests import from `crate::decision` (fix: import from `crate::constants`)
- `TurnPhase` is public in `crate::constants` → triggers.rs doesn't import it (fix: add import)
- No `pass_priority` or `set_phase_step` public methods exist → tests need rewriting to either (a) add these as test-only helpers in game.rs, or (b) restructure tests to use the decision-maker pattern

### Critical Design Decision

The two tests `activated_ability_puts_on_stack` and `spell_effects_execute_on_resolve` in game_basics.rs fundamentally rely on manual spell casting + priority passing + stack resolution, which the current engine doesn't expose. These tests either need:
1. **Option A**: Add `pub(crate)` test helper methods on Game (e.g., `resolve_top_of_stack()`, `set_step()`)
2. **Option B**: Rewrite tests to use the decision-maker pattern with custom PlayerDecisionMaker impls
3. **Option C**: Simplify/remove these integration-style tests if similar coverage exists elsewhere

Recommendation: **Option A** — add minimal test helpers since these are unit tests for core game mechanics and the decision-maker pattern adds unnecessary complexity.

## Task List

### Phase 1: Simple import/type fixes (no logic changes)

- [x] Task 1: Fix `Outcome` import in `effects.rs:14` — change `crate::decision::...Outcome...` to separate `use crate::constants::Outcome;`
- [x] Task 2: Fix `Outcome` import in `game_basics.rs:14` — same pattern as Task 1
- [ ] Task 3: Add `TurnPhase` import in `triggers.rs` — add `use crate::constants::TurnPhase;` (or add to existing constants import line 7)
- [ ] Task 4: Fix `combat` ambiguity in `keywords.rs:283` — qualify as `crate::combat::can_block(...)` instead of `combat::can_block(...)`
- [ ] Task 5: Fix missing `}` in `keywords.rs` before line 1626 — the `spell_has_convoke_test` function is missing its closing brace

### Phase 2: Simple API renames in test code (mechanical fixes)

- [ ] Task 6: Fix `Hand::push()` → `Hand::add()` in `game_basics.rs:440` and `game_basics.rs:508`
- [ ] Task 7: Fix `graveyard.contains(&spell_id)` → `graveyard.contains(spell_id)` in `game_basics.rs:467` and `game_basics.rs:532` (remove borrow)
- [ ] Task 8: Fix `Library::top_n(3)` → `Library::peek(3).to_vec()` in `effects.rs:747`
- [ ] Task 9: Fix `Hand::to_vec()` → `hand.as_slice().to_vec()` in `effects.rs:759`
- [ ] Task 10: Fix `Graveyard::to_vec()` → `graveyard.as_slice().to_vec()` in `effects.rs:760`
- [ ] Task 11: Fix `Effect::gain_control_until_end_of_turn()` → `Effect::gain_control_eot()` in `effects.rs:801`
- [ ] Task 12: Fix `Effect::look_top_and_pick(3, "land card", "hand", "graveyard")` → `Effect::look_top_and_pick(3, "land card")` in `effects.rs:751`
- [ ] Task 13: Fix `perm.entered_this_turn = false` → `perm.summoning_sick = false` in `game_basics.rs:369`
- [ ] Task 14: Fix `Effect::DealDamageAny { amount: N }` → `Effect::DealDamage { amount: N }` in `game_basics.rs:354,431`
- [ ] Task 15: Fix `TargetSpec::AnyTarget` → `TargetSpec::CreatureOrPlayer` in `game_basics.rs:355,432`

### Phase 3: Test infrastructure — add test helpers to game.rs

- [ ] Task 16: Add `#[cfg(test)] pub(crate) fn resolve_top_of_stack(&mut self)` helper to Game that pops and resolves the top stack item (replaces `pass_priority` x2 pattern)
- [ ] Task 17: Add `#[cfg(test)] pub(crate) fn set_step(&mut self, step: PhaseStep)` helper to Game (replaces `turn_manager.set_phase_step(...)`)
- [ ] Task 18: Make `cast_spell` accessible from tests — add `#[cfg(test)] pub(crate) fn test_cast_spell(&mut self, player_id: PlayerId, card_id: ObjectId)` wrapper or make `cast_spell` `pub(crate)`

### Phase 4: Rewrite affected test functions in game_basics.rs

- [ ] Task 19: Rewrite `activated_ability_puts_on_stack` test (game_basics.rs:325-398):
  - Use `set_step()` instead of `turn_manager.set_phase_step()`
  - Use `DealDamage`/`CreatureOrPlayer` instead of `DealDamageAny`/`AnyTarget`
  - Fix `entered_this_turn` → `summoning_sick`
  - Fix `activate_ability` call: pass `&[ObjectId]` not `vec![PlayerId.as_target()]` — need to convert p2 (PlayerId) to an appropriate target. Since the engine internally handles player targeting, may need to pass empty targets or use a player-as-target approach
  - Use `stack.top().unwrap()` instead of `stack[0]`
  - Use `resolve_top_of_stack()` instead of `pass_priority()` x2
- [ ] Task 20: Rewrite `spell_effects_execute_on_resolve` test (game_basics.rs:400-468):
  - Use `set_step()` instead of `turn_manager.set_phase_step()`
  - Use `DealDamage`/`CreatureOrPlayer`
  - Use `hand.add()` instead of `hand.push()`
  - Use `test_cast_spell()` instead of 3-arg `cast_spell()`
  - Use `stack.top()` instead of `stack[0]`
  - Use `resolve_top_of_stack()` instead of `pass_priority()` x2
  - Fix `graveyard.contains(spell_id)` (remove `&`)
- [ ] Task 21: Rewrite `fizzle_when_target_removed` test (game_basics.rs:470-533):
  - Use `hand.add()` instead of `hand.push()`
  - Use `test_cast_spell()` instead of 3-arg `cast_spell()`
  - Use `resolve_top_of_stack()` instead of `pass_priority()` x2
  - Fix `graveyard.contains(spell_id)` (remove `&`)

### Phase 5: Fix triggers.rs errors

- [ ] Task 22: Fix `setup()` calls in `triggers.rs:414` and `triggers.rs:451` — pass `(Box::new(PassivePlayer), Box::new(PassivePlayer))` arguments to match the 2-arg setup function signature

### Phase 6: Verify compilation

- [ ] Task 23: Run `cargo check -p mtg-engine --lib --tests` and verify 0 errors
- [ ] Task 24: Run `cargo test --lib -p mtg-engine` and verify tests pass
- [ ] Task 25: Address any remaining warnings if they indicate real issues

## Notes

- The test files are compiled via `game.rs` → `mod tests;` → `game/tests.rs` → `#[path = "../tests/X.rs"] mod X;`. This means tests are nested inside the `game` module and have access to private methods.
- `tests/mod.rs` appears to be a parallel module definition that may not be used for compilation (since `game/tests.rs` uses `#[path]` directives). Need to verify this doesn't cause duplicate symbol issues.
- The `combat` ambiguity in keywords.rs arises because `use crate::game::*` brings in module-level items, and `game/tests.rs` has `mod combat;` which creates a local `combat` name that conflicts with `crate::combat`.
- `activate_ability` takes `&[ObjectId]` for targets, but PlayerId is not ObjectId. The test tries to target player 2 with damage — need to investigate how player targeting works in the engine (likely through the decision-maker's `choose_targets` method, not through direct ObjectId passing). May need a custom decision maker for this test.
- The `pass_priority` pattern (call twice = both players pass = resolve) is a conceptual model that doesn't map to the engine's decision-maker architecture. The `resolve_top_of_stack` helper should directly call the internal resolution logic.
- Missing `}` in keywords.rs at ~line 1625 means `spell_has_convoke_test` function isn't closed, causing `convoke_excludes_opponent_creatures` and `convoke_helper_constructor` to be parsed as nested functions.

## Tasks Completed

- Task 1: Moved `Outcome` from `crate::decision` import to `crate::constants` import in effects.rs
- Task 2: Moved `Outcome` from `crate::decision` import to `crate::constants` import in game_basics.rs
