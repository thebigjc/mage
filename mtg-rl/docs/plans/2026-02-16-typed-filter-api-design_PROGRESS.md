# Progress: 2026-02-16-typed-filter-api-design

## Status
IN_PROGRESS

## Task List
- [x] Phase 1: Update find_matching_permanents to use bool flags instead of string checks
- [x] Phase 2: Migrate common Filter::parse patterns to factory methods (~20 patterns)
- [ ] Phase 3: Migrate long-tail Filter::parse patterns to Filter::new() calls
- [ ] Phase 4: Delete the parser (Filter::parse, parse_filter_string, message_lower, helpers)
- [ ] Phase 5: Update Python code-gen scripts to emit typed constructors

## Completed This Iteration
- Phase 2: Migrated ~124 Filter::parse call sites to factory methods. Added 8 new factory methods: nonland_permanent_opponent_controls(), other_nonland_permanent(), any_land(), any_enchantment(), land_card(), creatures_opponents_control(), basic_land(), attacking_or_blocking_creature(). Migrated all instances of: "self", "enchanted creature", "creature an opponent controls", "nonland permanent", "artifact or enchantment", "equipped creature", "creature you control", "creature", "another creature you control", "other creature(s) you control", "nonland permanent an opponent controls", "other nonland permanent", "creatures opponents control", "permanent", "land", "enchantment", "land card", "creature card", "basic land", "attacking or blocking creature". Down from ~241 to ~117 remaining Filter::parse calls.

## Prior Work (already completed before this loop)
- Filter struct has &'static str message field (was Arc<str>)
- Bool flags added to struct: is_self_referential, excludes_source, requires_attacking
- Filter::new() constructor and flag setters (.excludes_source(), .requires_attacking(), .self_referential())
- Pre-built factory methods: any_permanent, any_creature, any_nonland_permanent, creature_you_control, permanent_you_control, permanent_opponent_controls, any_card, creature_card
- Phase 1 factory methods: self_reference, enchanted_creature, equipped_creature, creature_opponent_controls, artifact_or_enchantment, other_creature_you_control
- Predicate builder methods exist (creature(), land(), nonland(), etc.)

## Notes
- Remaining ~117 Filter::parse calls: 36 in abilities.rs (dynamic filter params), 63 in set files (long-tail), 17 in test/engine files
- abilities.rs calls take `filter: &str` parameter — need function signature changes (Phase 3)
- 22 factory methods now exist on Filter
- 618 tests passing, cargo check clean
