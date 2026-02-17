# Progress: 2026-02-16-typed-filter-api-design

## Status
DONE

## Task List
- [x] Phase 1: Update find_matching_permanents to use bool flags instead of string checks
- [x] Phase 2: Migrate common Filter::parse patterns to factory methods (~20 patterns)
- [x] Phase 3: Migrate long-tail Filter::parse patterns to typed builder signatures
- [x] Phase 4: Delete the parser (Filter::parse, parse_filter_string, message_lower, helpers)
- [x] Phase 5: Update Python code-gen scripts to emit typed constructors

## Completed This Iteration
- Phase 4: Converted ~228 remaining Filter::parse calls across 15 files to typed constructors using Predicate combinators. Deleted ~340 lines of parser code (parse_filter_string, depluralize, capitalize, helpers). Removed message_lower field from Filter struct. Replaced string-based logic in game.rs with bool flags. Rewrote permanent_matches_filter_part to use inline type matching. Created Python migration script (scripts/migrate_filter_parse.py) for bulk mechanical conversion. Fixed evaluate_count_filter compatibility by preserving descriptive message strings for BoostPerCount/CreateTokenDynamic filters. 709 tests pass, clippy clean.
- Phase 5: Updated /tmp/gen_cards.py (the only ongoing code-gen script) to emit `Filter::self_reference()` instead of `"self".into()` for EntersTapped static effects. The one-time migration scripts (/tmp/migrate_filters.py, /tmp/fix_callers.py, /tmp/ajani_fix.py) are obsolete and were not updated. No repo changes needed since gen_cards.py lives in /tmp/.

## Prior Work (already completed before this loop)
- Filter struct has &'static str message field (was Arc<str>)
- Bool flags added to struct: is_self_referential, excludes_source, requires_attacking
- Filter::new() constructor and flag setters (.excludes_source(), .requires_attacking(), .self_referential())
- Pre-built factory methods: any_permanent, any_creature, any_nonland_permanent, creature_you_control, permanent_you_control, permanent_opponent_controls, any_card, creature_card
- Phase 1 factory methods: self_reference, enchanted_creature, equipped_creature, creature_opponent_controls, artifact_or_enchantment, other_creature_you_control
- Phase 2: Migrated ~124 Filter::parse call sites to factory methods, added 8 new factory methods
- Predicate builder methods exist (creature(), land(), nonland(), etc.)
- Phase 3: Changed 36 builder method signatures from `filter: &str` to `filter: Filter`. Updated ~120 call sites.

## Notes
- Filter::parse() is now completely deleted — no string-based filter construction remains
- evaluate_count_filter in game.rs still parses filter.message text at runtime for dynamic counting — messages for filters used with BoostPerCount/CreateTokenDynamic must preserve descriptive patterns like "Elf cards in your graveyard"
- scripts/migrate_filter_parse.py maps ~114 unique Filter::parse patterns to typed replacements
- 709 tests passing, cargo check + clippy clean
- gen_cards.py lives at /tmp/gen_cards.py (not tracked in git)
