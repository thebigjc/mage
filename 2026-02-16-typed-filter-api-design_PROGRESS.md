# Progress: 2026-02-16-typed-filter-api-design

Started: Mon Feb 16 09:19:58 PM EST 2026

## Status

IN_PROGRESS

## Analysis

### Current State

**Filter struct** (`mtg-engine/src/filters.rs:196-243`):
- Fields: `message: Arc<str>`, `message_lower: Arc<str>`, `predicate: Arc<Predicate>`
- Derives: `Clone, Debug, Serialize, Deserialize`
- Implements `PartialEq<&str>` and `PartialEq<str>` (compares on `message`)
- Has 8 pre-built factory methods (lines 245-290): `any_permanent`, `any_creature`, `any_nonland_permanent`, `creature_you_control`, `permanent_you_control`, `permanent_opponent_controls`, `any_card`, `creature_card`
- 4 of those 8 factories are **unused** in the codebase

**Filter::parse()** (`filters.rs:292-296`): Wraps `parse_filter_string()` (~266 lines, lines 319-584).

**Call sites**: 231 total — 89 in mtg-engine, 142 in mtg-cards (4 set files).

**String-based logic in game.rs** that must be replaced by bool flags:
1. `find_matching_permanents` (lines 1314-1341): checks "self", "enchanted", "equipped", "other", "attacking"
2. `BoostPerCount` handler (lines 676-702): checks "graveyard", "creature" on `count_filter.message`
3. Trigger doubling (line 1845): checks `filter.message_lower().contains("other")`

**Other string-based logic** (plain strings, NOT Filter objects — out of scope for this plan):
- `evaluate_condition()` (lines 836-920): condition strings
- `count_permanents_matching()` (lines 922-938): condition strings
- `evaluate_count_filter()` (lines 949-1037): dynamic value sources
- `Cost::SacrificeOther` (lines 3818-3831): cost filter strings

### Key Design Decision: `&'static str` vs `Arc<str>`

The plan specifies `message: &'static str`. However, investigation reveals:
- `Deserialize` cannot reconstruct `&'static str` from serialized data
- Filter is embedded in GameState → Battlefield → Effects chains that all derive Serialize/Deserialize
- Serialization is **never actually called** anywhere in the codebase
- Plan already says "drop Deserialize, keep Serialize" — this is necessary and sufficient
- Serde can serialize `&'static str` (via the `Serialize for &str` impl), so keeping `Serialize` works
- Since Deserialize is dropped, the `&'static str` approach works as planned
- All filter messages are string literals known at compile time (either from factories or from card code like `Filter::new("enchanted creature", ...)`)

### Bool Flags Coverage

The plan proposes 3 bool flags: `is_self_referential`, `excludes_source`, `requires_attacking`.

After analysis, the "enchanted"/"equipped" case is covered by `is_self_referential` (the plan groups "self", "enchanted creature", "equipped creature" under this flag). The `BoostPerCount` graveyard string checks are **not** covered by the 3 flags — but those are on `count_filter` (a different usage context that can be addressed separately or with the predicate tree).

### Existing Tests

- 19 unit tests in `filters.rs` (lines 761-1027) covering parsing, matching, and combinators
- 50+ integration tests in `mtg-engine/src/tests/` that use `Filter::parse()` extensively
- `PartialEq<&str>` is used in test assertions like `assert_eq!(filter, "enchanted creature")`

### Unique Filter Strings (90 total)

**Top patterns for factory methods** (covering ~60% of 231 call sites):
- "self" (20x), "enchanted creature" (20x), "creature an opponent controls" (10x)
- "nonland permanent" (7x), "artifact or enchantment" (7x), "equipped creature" (6x)
- "creature you control" (6x), "creature" (6x), "another creature you control" (6x)
- "nonland permanent an opponent controls" (4x), "other nonland permanent" (3x)
- "other creature you control" / "other creatures you control" (3x each)
- "creatures opponents control" (3x), "Elf" (3x)

**Singleton patterns** (79 strings, each appearing once): require `Filter::new()` with typed predicates.

## Task List

### Phase 1: Infrastructure (Filter struct + find_matching_permanents)

- [ ] Task 1.1: Add bool flags to Filter struct — add `is_self_referential: bool`, `excludes_source: bool`, `requires_attacking: bool` fields to Filter struct. Update `Filter::new()` to initialize all three to `false`. Add builder methods `excludes_source(mut self) -> Self`, `requires_attacking(mut self) -> Self`. Keep all existing fields for now.

- [ ] Task 1.2: Change `message` field from `Arc<str>` to `&'static str` — update Filter struct, update `Filter::new()` signature to take `&'static str`, update `PartialEq` impls. Drop `Deserialize` derive (keep `Serialize`). This requires a custom `Deserialize` removal — check that nothing actually deserializes Filter (confirmed: nothing does). Note: `Filter::parse()` will need to keep `Arc<str>` temporarily or use `Box::leak` — simplest approach is to leave `parse()` using a leaked `&'static str` since parse is only called at card-creation time (not hot path), and the leaked strings are the same ~90 unique strings reused across game instances.

- [ ] Task 1.3: Remove `message_lower` field — delete the field, delete the `message_lower()` accessor method. Since we're adding bool flags, runtime lowercase string checks are no longer needed. Any remaining callers of `message_lower()` outside of `find_matching_permanents` must be updated (BoostPerCount handler at line 686, trigger doubling at line 1845).

- [ ] Task 1.4: Set bool flags in `Filter::parse()` — update `parse_filter_string()` / `Filter::parse()` so that the returned Filter has correct bool flags set based on the parsed string (e.g., if string is "self" or "enchanted creature" or "equipped creature", set `is_self_referential = true`; if string contains "other", set `excludes_source = true`; if string contains "attacking", set `requires_attacking = true`).

- [ ] Task 1.5: Update `find_matching_permanents` to use bool flags — replace all `msg.contains(...)` and `msg == "self"` checks with `filter.is_self_referential`, `filter.excludes_source`, `filter.requires_attacking`. Remove the `let msg = filter.message_lower();` line entirely.

- [ ] Task 1.6: Update BoostPerCount handler to avoid string checks — the `count_filter.message.contains("graveyard")` and `count_filter.message_lower().contains("creature")` checks (game.rs lines 681-686) need a different approach. Options: (a) add a `counts_graveyard: bool` flag to Filter, (b) use the predicate tree to detect graveyard counting, (c) restructure the BoostPerCount effect to carry explicit zone info. Choose the simplest approach that fits the plan's spirit.

- [ ] Task 1.7: Update trigger doubling to use `excludes_source` flag — replace `filter.message_lower().contains("other")` at game.rs line 1845 with `filter.excludes_source`.

- [ ] Task 1.8: Update existing factory methods — update the 8 existing factory methods (any_permanent, any_creature, etc.) to set appropriate bool flags and use `&'static str` messages. Add new factory methods per the plan: `self_reference()`, `enchanted_creature()`, `creature_opponent_controls()`, `artifact_or_enchantment()`, `other_creature_you_control()`, `nonland_permanent()`, `equipped_creature()`. Some overlap with existing methods (e.g., `creature_you_control()` already exists).

- [ ] Task 1.9: Add equivalence tests — for each new factory method, add a test that creates the filter both via the factory and via `Filter::parse()`, then verifies both match the same set of permanents. These are temporary tests deleted in Phase 4.

- [ ] Task 1.10: Add bool flag unit tests — test that `find_matching_permanents` correctly respects `excludes_source`, `requires_attacking`, and `is_self_referential` flags.

- [ ] Task 1.11: Run gate — `cargo check && cargo clippy && cargo test --lib` must all pass.

### Phase 2: Migrate common patterns (~60% of call sites)

- [ ] Task 2.1: Migrate "self" (20 sites) — replace `Filter::parse("self")` with `Filter::self_reference()` across all files.

- [ ] Task 2.2: Migrate "enchanted creature" (20 sites) — replace `Filter::parse("enchanted creature")` with `Filter::enchanted_creature()`.

- [ ] Task 2.3: Migrate "creature an opponent controls" (10 sites) — replace with `Filter::creature_opponent_controls()`.

- [ ] Task 2.4: Migrate "nonland permanent" (7 sites) — replace with `Filter::nonland_permanent()`. Note: existing `Filter::any_nonland_permanent()` is similar but check if semantics differ.

- [ ] Task 2.5: Migrate "artifact or enchantment" (7 sites) — replace with `Filter::artifact_or_enchantment()`.

- [ ] Task 2.6: Migrate "equipped creature" (6 sites) — replace with `Filter::equipped_creature()`.

- [ ] Task 2.7: Migrate "creature you control" (6 sites) — replace with existing `Filter::creature_you_control()`, verifying it sets correct bool flags.

- [ ] Task 2.8: Migrate "creature" (6 sites) — replace with existing `Filter::any_creature()` or a new `Filter::creature()` factory.

- [ ] Task 2.9: Migrate "another creature you control" (6 sites) — replace with `Filter::other_creature_you_control()`.

- [ ] Task 2.10: Migrate remaining medium-frequency patterns — "nonland permanent an opponent controls" (4x), "other nonland permanent" (3x), "other creature you control" (3x), "other creatures you control" (3x), "creatures opponents control" (3x), "Elf" (3x). Create factory methods where warranted (3+ uses) or use `Filter::new()`.

- [ ] Task 2.11: Run gate — `cargo check && cargo clippy && cargo test --lib`.

### Phase 3: Migrate long-tail patterns (~40% of call sites)

- [ ] Task 3.1: Migrate 2-use patterns (9 strings, 18 sites) — "Skeleton you control", "other Zombie you control", "other Spirit you control", "other Elf you control", "land", "creature token you control", "creature or land", "basic land", "attacking or blocking creature". Use `Filter::new()` with typed predicate chains.

- [ ] Task 3.2: Migrate singleton patterns in mtg-engine/src/abilities.rs (37 sites) — these are effect/ability definitions. Convert each `Filter::parse("...")` to `Filter::new("...", predicate_chain)` with appropriate bool flags.

- [ ] Task 3.3: Migrate singleton patterns in mtg-engine/src/tests/ (~24 sites across 6 test files) — convert test code Filter::parse calls to typed constructors.

- [ ] Task 3.4: Migrate singleton patterns in mtg-engine/src/game.rs (3 sites) — convert inline filter constructions.

- [ ] Task 3.5: Migrate singleton patterns in mtg-cards/src/sets/fdn.rs (39 sites) — convert Foundations set card factories.

- [ ] Task 3.6: Migrate singleton patterns in mtg-cards/src/sets/tdm.rs (60 sites) — convert Tolkien set card factories.

- [ ] Task 3.7: Migrate singleton patterns in mtg-cards/src/sets/ecl.rs (42 sites) — convert Eclogue set card factories.

- [ ] Task 3.8: Migrate singleton patterns in mtg-cards/src/sets/tla.rs (1 site) — convert Avatar set card factory.

- [ ] Task 3.9: Run gate — `cargo check && cargo clippy && cargo test --lib`.

### Phase 4: Delete the parser

- [ ] Task 4.1: Remove `Filter::parse()` method — delete the method from filters.rs.

- [ ] Task 4.2: Remove `parse_filter_string()` and helpers — delete `parse_filter_string()` (~266 lines), `depluralize()`, `capitalize()`, `strip_suffix_ignore_case()`, `strip_prefix_ignore_case()`, `parse_mana_value_suffix()`, `extract_mana_value_comparison()`, `parse_power_suffix()`, `extract_power_comparison()`, `parse_subtype()`.

- [ ] Task 4.3: Remove equivalence tests — delete the temporary equivalence tests added in Task 1.9.

- [ ] Task 4.4: Remove parser unit tests — delete tests that test `Filter::parse()` behavior (e.g., `parse_basic_types`, `parse_nonland_permanent`, `parse_or_combinator`, `parse_controller_suffix`, `parse_card_suffix`, `parse_basic_land_with_subtypes`, `parse_subtype_filter`, `parse_ignore_controller_with_controller_filter`).

- [ ] Task 4.5: Clean up unused imports — remove any imports that were only needed by the parser.

- [ ] Task 4.6: Run gate — `cargo check && cargo clippy && cargo test --lib`.

### Phase 5: Update Python code-gen scripts

- [ ] Task 5.1: Identify codegen scripts — locate any Python scripts used for batch card generation that emit `Filter::parse("...")`. Check `/tmp/` scripts, `docs/` scripts, and memory notes about batch card generation.

- [ ] Task 5.2: Update codegen to emit typed constructors — modify scripts to emit `Filter::new("...", predicate_chain)` or factory method calls instead of `Filter::parse("...")`.

- [ ] Task 5.3: Verify generated code compiles — run a test generation and verify output with `cargo check`.

## Notes

### Serialization Approach
- Drop `Deserialize` from Filter (nothing deserializes it)
- Keep `Serialize` (works with `&'static str` via serde's `Serialize for &str`)
- Drop `Deserialize` from `Predicate` too if it's no longer needed, or keep it for other uses
- The `"rc"` serde feature in workspace Cargo.toml can remain (used by other Arc types)

### `Filter::parse()` Leak Strategy for Transition
During Phases 1-3, `Filter::parse()` must return a `Filter` with `message: &'static str`. Since parse takes a `&str`, use `Box::leak(s.to_string().into_boxed_str())` to create a `&'static str`. This leaks ~90 unique strings (a few KB total, negligible). The leaked strings are freed when the program exits. This is deleted in Phase 4 when parse is removed.

### `BoostPerCount` Graveyard Counting
The `count_filter.message.contains("graveyard")` check is a broader issue. Options:
1. **Quick fix**: Add a `counts_graveyard: bool` flag to Filter (not in plan but pragmatic)
2. **Better fix**: Restructure `BoostPerCount`/`StaticEffect::BoostPerCount` to carry an explicit zone enum
3. **Defer**: Leave the `.message.contains("graveyard")` check (it works on `&'static str` too, just not as clean)

Recommend option 3 (defer) since it still works and keeps the plan focused.

### PartialEq Impact
`PartialEq<&str>` currently compares `&*self.message` (Arc<str> deref). With `&'static str`, it becomes `self.message == *other` — simpler. Tests using `assert_eq!(filter, "enchanted creature")` will continue to work.

### Predicate Ownership Change
Plan says replace `Arc<Predicate>` with owned `Predicate`. This means Filter's `Clone` impl deep-copies the predicate tree. Predicate trees are small (1-5 nodes), so clone cost is ~50-200ns — negligible. The `Serialize` derive works the same with owned vs Arc-wrapped.

## Tasks Completed

