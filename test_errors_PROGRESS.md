# Progress: test_errors

Started: Sun Feb 15 06:17:48 PM EST 2026

## Status

IN_PROGRESS

## Analysis

Running `cargo test --lib -p mtg-engine --no-run 2>&1` produces **137 warnings** (136 individual + 1 summary). They break down into these categories:

### Warning Categories

1. **Unused imports in test files** (~114 warnings): Each of the 12 test files has a copy-pasted block of ~15 imports, many of which are unused in that specific file. These can be safely removed by trimming each file's imports to only what it actually uses.

2. **Unused `pub use` re-exports in tests.rs** (12 warnings): `tests.rs` does `pub use abilities::*;` etc. for all 12 submodules. Since these are test modules that don't need to export anything publicly, these should be removed.

3. **Dead code in test files** (19 warnings): Unused structs (PlayerDecisionMaker implementations) and helper functions that were never wired to any test. All 19 are genuinely unused duplicates and can be safely removed.

4. **Unused variables in game.rs** (2 warnings): `mut candidates` (line 2962, remove `mut`) and `src` (line 4117, prefix with `_`).

5. **Unused variables in test files** (3 warnings): `modes` in abilities.rs:31, `land_id` in continuous_effects.rs:1297, `lib_ids` in special_mechanics.rs:588 — prefix with `_`.

### Files Affected (sorted by warning count)

| File | Warnings |
|------|----------|
| tests/keywords.rs | 14 |
| tests/special_mechanics.rs | 13 |
| tests/modal.rs | 13 |
| tests/continuous_effects.rs | 13 |
| game/tests.rs | 12 |
| tests/equipment_auras.rs | 12 |
| tests/combat.rs | 12 |
| tests/abilities.rs | 12 |
| tests/triggers.rs | 11 |
| tests/tokens.rs | 11 |
| tests/costs.rs | 8 |
| tests/game_basics.rs | 2 |
| tests/effects.rs | 2 |
| game.rs | 2 |

### Approach

The plan says: "If they require implementing new functionality, we can skip them (such as 'never used' warnings), but otherwise we should do our best to clean them up."

All warnings here are straightforward cleanups (remove unused imports, remove dead code, fix variables). None require new functionality. We should fix them all.

The cleanest approach is to fix file-by-file, starting with game.rs (production code), then tests.rs, then each test file alphabetically. After each file, verify warning count decreases and no tests break.

## Task List

### Production code fixes
- [x] Task 1: Fix 2 warnings in `mtg-engine/src/game.rs` — remove `mut` from `candidates` (line 2962), prefix `src` with `_` (line 4117)

### Test infrastructure fix
- [x] Task 2: Fix 12 warnings in `mtg-engine/src/game/tests.rs` — remove `pub use module::*` re-exports (these are test modules, nothing should import from them)

### Test file import + dead code cleanup (one task per file)
- [x] Task 3: Fix 12 warnings in `tests/abilities.rs` — remove unused imports (`StaticEffect`, `ModalMode`, `CombatState`, `Color`, `KeywordAbilities`, `Outcome`, `PhaseStep`, `SuperType`, `Zone`, `CounterType`, `EventType`, `GameEvent`, `Mana`, `ManaCost`, `Permanent`, `StateBasedActions`, `AbilityId`, `WatcherManager`, `super::*`), prefix `modes` variable with `_`
- [x] Task 4: Fix 12 warnings in `tests/combat.rs` — remove unused imports (`Cost`, `Effect`, `ModalMode`, `TargetSpec`, `CombatState`, `Color`, `PhaseStep`, `SuperType`, `Zone`, `CounterType`, `EventType`, `GameEvent`, `Mana`, `ManaCost`, `StateBasedActions`, `AbilityId`, `WatcherManager`, `super::*`), remove dead `AttackAllPlayer2` struct + impl (~30 lines), remove dead `add_creature2` function
- [x] Task 5: Fix 13 warnings in `tests/continuous_effects.rs` — remove unused imports (`Cost`, `ModalMode`, `CombatState`, `PhaseStep`, `SuperType`, `Zone`, `CounterType`, `EventType`, `GameEvent`, `ManaCost`, `StateBasedActions`, `AbilityId`, `WatcherManager`, `super::*`), remove dead `AlwaysPassPlayer2` struct + impl, remove dead `PassPlayer2` struct + impl, prefix `land_id` with `_`
- [x] Task 6: Fix 8 warnings in `tests/costs.rs` — remove unused imports (`ModalMode`, `CombatState`, `Color`, `PhaseStep`, `SuperType`, `Zone`, `EventType`, `GameEvent`, `StateBasedActions`, `AbilityId`, `WatcherManager`, `super::*`)
- [x] Task 7: Fix 2 warnings in `tests/effects.rs` — remove unused `super::*` import, remove unused `PhaseStep` import
- [x] Task 8: Fix 12 warnings in `tests/equipment_auras.rs` — remove unused imports (`ModalMode`, `CombatState`, `Color`, `KeywordAbilities`, `PhaseStep`, `SuperType`, `Zone`, `CounterType`, `EventType`, `GameEvent`, `Mana`, `StateBasedActions`, `AbilityId`, `WatcherManager`, `super::*`), remove dead `PassivePlayer2` struct + impl, remove dead `setup2` function
- [x] Task 9: Fix 2 warnings in `tests/game_basics.rs` — remove unused `super::*` import, remove unused `TurnPhase` import
- [x] Task 10: Fix 14 warnings in `tests/keywords.rs` — remove unused imports (`Cost`, `ModalMode`, `CombatState`, `SuperType`, `Zone`, `CounterType`, `EventType`, `StateBasedActions`, `AbilityId`, `WatcherManager`, `super::*`), remove dead `PassivePlayer2` struct + impl, remove dead `PassivePlayer3` struct + impl, remove dead `PassivePlayer4` struct + impl, remove dead `PassivePlayer5` struct + impl, remove dead `PassivePlayer6` struct + impl
- [x] Task 11: Fix 13 warnings in `tests/modal.rs` — remove unused imports (`Ability`, `Cost`, `StaticEffect`, `TargetSpec`, `CombatState`, `Color`, `KeywordAbilities`, `PhaseStep`, `SubType`, `SuperType`, `Zone`, `CounterType`, `EventType`, `GameEvent`, `Mana`, `ManaCost`, `Permanent`, `StateBasedActions`, `AbilityId`, `WatcherManager`, `super::*`), remove dead `make_deck` function, remove dead `AlwaysPassPlayer` struct + impl
- [x] Task 12: Fix 13 warnings in `tests/special_mechanics.rs` — remove unused imports (`ModalMode`, `TargetSpec`, `CombatState`, `Color`, `SuperType`, `Zone`, `EventType`, `GameEvent`, `StateBasedActions`, `AbilityId`, `WatcherManager`, `super::*`), remove dead `PassivePlayer2` struct + impl, remove dead `PassivePlayer3` struct + impl, remove dead `OptionPicker2` struct + impl, remove dead `AlwaysPassPlayer2` struct + impl, prefix `lib_ids` with `_`
- [ ] Task 13: Fix 11 warnings in `tests/tokens.rs` — remove unused imports (`Ability`, `Cost`, `ModalMode`, `StaticEffect`, `TargetSpec`, `CombatState`, `CardType`, `Color`, `KeywordAbilities`, `Outcome`, `PhaseStep`, `SubType`, `SuperType`, `Zone`, `CounterType`, `AttackerInfo`, `DamageAssignment`, `GameView`, `NamedChoice`, `PlayerAction`, `PlayerDecisionMaker`, `ReplacementEffectChoice`, `TargetRequirement`, `UnpaidMana`, `EventType`, `GameEvent`, `Mana`, `ManaCost`, `Permanent`, `StateBasedActions`, `AbilityId`, `WatcherManager`, `super::*`)
- [ ] Task 14: Fix 11 warnings in `tests/triggers.rs` — remove unused imports, remove dead `PassivePlayer2` struct + impl, remove dead `PassivePlayer3` struct + impl

### Verification
- [ ] Task 15: Run `cargo test --lib -p mtg-engine --no-run 2>&1` and verify 0 warnings remain
- [ ] Task 16: Run `cargo test --lib -p mtg-engine` and verify all tests still pass

## Completed This Iteration
- Task 12: Removed 8 unused import lines, `super::*`, dead `PassivePlayer2`/`PassivePlayer3`/`OptionPicker2`/`AlwaysPassPlayer2` structs+impls, prefixed `lib_ids` with `_`. Warnings dropped from 22 to 22 (remaining are tokens.rs and triggers.rs). All 47 special_mechanics tests pass.

## Notes

- The `super::*` import in each test file is redundant because `super` refers to the `tests` module in tests.rs, which re-exports everything — but since each test file already imports what it needs via `crate::`, the `super::*` is unnecessary.
- All 19 dead code items (structs and functions) are genuinely unused duplicates of patterns used elsewhere in the test suite. They can be safely deleted.
- `cargo fix --lib -p mtg-engine --tests` could auto-fix 114 of the import warnings, but manual cleanup is preferred to also handle the dead code and ensure correctness.
- Task 2 done: Removed `pub use` from tests.rs. This also means `super::*` in each test file now imports nothing (since tests.rs no longer re-exports). The `super::*` warnings should still show up in each test file; they'll be cleaned up as part of Tasks 3–14.
