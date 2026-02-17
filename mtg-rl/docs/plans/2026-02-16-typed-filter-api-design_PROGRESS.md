# Progress: 2026-02-16-typed-filter-api-design

## Status
IN_PROGRESS

## Task List
- [x] Phase 1: Update find_matching_permanents to use bool flags instead of string checks
- [x] Phase 2: Migrate common Filter::parse patterns to factory methods (~20 patterns)
- [x] Phase 3: Migrate long-tail Filter::parse patterns to typed builder signatures
- [ ] Phase 4: Delete the parser (Filter::parse, parse_filter_string, message_lower, helpers)
- [ ] Phase 5: Update Python code-gen scripts to emit typed constructors

## Completed This Iteration
- Phase 3: Changed 36 builder method signatures in abilities.rs from `filter: &str` to `filter: Filter`. Updated ~120 call sites in set files (fdn, tla, tdm, ecl) and test files (effects, combat, continuous_effects, keywords, special_mechanics, triggers, costs, abilities) to pass `Filter::parse(...)` instead of bare strings. Fixed clippy warnings (redundant field names). All 638 tests pass, clippy clean.

## Prior Work (already completed before this loop)
- Filter struct has &'static str message field (was Arc<str>)
- Bool flags added to struct: is_self_referential, excludes_source, requires_attacking
- Filter::new() constructor and flag setters (.excludes_source(), .requires_attacking(), .self_referential())
- Pre-built factory methods: any_permanent, any_creature, any_nonland_permanent, creature_you_control, permanent_you_control, permanent_opponent_controls, any_card, creature_card
- Phase 1 factory methods: self_reference, enchanted_creature, equipped_creature, creature_opponent_controls, artifact_or_enchantment, other_creature_you_control
- Phase 2: Migrated ~124 Filter::parse call sites to factory methods, added 8 new factory methods
- Predicate builder methods exist (creature(), land(), nonland(), etc.)

## Notes
- Remaining Filter::parse calls: ~228 across all files (at call sites, not hidden in builders)
  - 10 in filters.rs (parser definition + tests - delete in Phase 4)
  - 1 in game.rs (dynamic runtime evaluation - needs parser or refactor)
  - 6 in abilities.rs (tests at bottom of file)
  - ~156 in set files (fdn: 34, tla: 1, tdm: 47, ecl: 75)
  - ~55 in test files
- Builder methods now all accept `Filter` instead of `&str` — new code must construct Filter explicitly
- 22 factory methods exist on Filter; callers still using Filter::parse() could be converted to Filter::new() or factories for additional type safety
- 638 tests passing, cargo check + clippy clean
