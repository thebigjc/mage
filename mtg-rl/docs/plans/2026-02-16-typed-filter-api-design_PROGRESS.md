# Progress: 2026-02-16-typed-filter-api-design

## Status
IN_PROGRESS

## Task List
- [ ] Phase 1: Update find_matching_permanents to use bool flags instead of string checks
- [ ] Phase 2: Migrate common Filter::parse patterns to factory methods (~20 patterns)
- [ ] Phase 3: Migrate long-tail Filter::parse patterns to Filter::new() calls
- [ ] Phase 4: Delete the parser (Filter::parse, parse_filter_string, message_lower, helpers)
- [ ] Phase 5: Update Python code-gen scripts to emit typed constructors

## Prior Work (already completed before this loop)
- Filter struct has &'static str message field (was Arc<str>)
- Bool flags added: is_self_referential, excludes_source, requires_attacking
- Filter::new() constructor and flag setters (.excludes_source(), .requires_attacking(), .self_referential())
- Pre-built factory methods: any_permanent, any_creature, any_nonland_permanent, creature_you_control, permanent_you_control, permanent_opponent_controls, any_card, creature_card
- Predicate builder methods exist (creature(), land(), nonland(), etc.)
- Filter::parse() still exists and still used at 241 call sites (including tests and docs)
- find_matching_permanents still uses string-based logic (msg.contains("other"), msg.contains("attacking"), etc.)

## Notes
- 241 Filter::parse call sites total: ~39 fdn, ~60 tdm, ~42 ecl, ~1 tla, ~37 abilities.rs, ~3 game.rs, ~26 tests, ~24 filters.rs tests
- find_matching_permanents at game.rs:1314 needs refactoring to use bool flags
- Filter::parse() doesn't set bool flags — callers that need them must set manually
