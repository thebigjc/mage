# Progress: plan

Started: Sun Feb 15 11:47:37 AM EST 2026

## Status

IN_PROGRESS

## Analysis

### Current State
- 1,333 cards across 4 sets (FDN 512, TLA 280, TDM 273, ECL 268)
- **940 total Custom fallbacks** (747 Effect::Custom, 160 StaticEffect::Custom, 33 Cost::Custom)
- ECL has the fewest customs: 68 Effect::Custom, 20 StaticEffect::Custom, 0 Cost::Custom = **88 total**
- All tests pass (318 engine, 20 cards, 52 AI)
- Build compiles cleanly (warnings only)
- **Cost reduction system**: committed (cf3ba09)

### Goal
Complete ECL set by implementing missing engine capabilities and updating card factories to use typed Effect/StaticEffect variants instead of Custom strings. Work one ROADMAP item at a time, add tests, commit after each.

### What Already Exists (Engine Features)
- Combat system (full), triggered abilities (ETB, attack, dies, upkeep, end step, spell cast, life gain)
- Continuous effects (Layer 6 keywords + Layer 7 P/T), equipment/aura systems
- X-cost spells, impulse draw, graveyard casting (flashback), delayed triggers
- Behold mechanic, Vivid mechanic, Modal spells, Fight/Bite
- Token creation (regular, tapped-attacking, copy, dynamic), Flicker/FlickerEndStep
- Cost reduction (uncommitted), BoostPerCount, ConditionalKeyword/ConditionalBoostSelf
- GainAllCreatureTypes, BlightOpponents, AdditionalLandPlays

### What's Missing for ECL Parity (grouped by engine feature)

**Tier 1 - High Impact (reduce many Custom fallbacks):**
1. Generic stub replacement - 48 cards have generic placeholders ("ETB effect.", "Activated effect.", "Spell effect.", "Static effect.") that can be replaced by composing existing Effect variants per-card
2. Replacement effect pipeline - needed for enters-with-counters, death replacement, damage prevention (~20+ cards across all sets, ~3-5 in ECL)

**Tier 2 - Medium Impact (new engine features for ECL):**
3. "Loses all abilities" effect - 4 ECL cards (Abigale, Curious Colossus, Noggle the Mind, Retched Wretch)
4. SetPowerToughnessAll / mass type change - 2 ECL cards (Curious Colossus, Noggle the Mind)
5. "Put creature from hand onto battlefield" effect - 2 ECL cards (Meek Attack, Kinscaer Sentry)
6. Transform/DFC system - 2 ECL cards (Eirdu/Isilu, Figure of Fable)
7. Animate artifact ("becomes a creature") - 1 ECL card (Firdoch Core)
8. Convoke keyword enforcement - 2 ECL cards (Omni-Changeling, Eirdu)
9. Conspire keyword - 1 ECL card (Raiding Schemes)
10. Mass reanimation by creature type - 1 ECL card (Bloodline Bidding)
11. Dynamic P/T based on game state conditions - 3 ECL cards (Squawkroaster, Doran, Kinbinding)
12. Toughness-based damage assignment - 1 ECL card (Bark of Doran)
13. Damage doubling from chosen type - 1 ECL card (Collective Inferno)
14. Mana doubling for basic lands - 1 ECL card (Lavaleaper)
15. Enhanced mana production (enchanted land) - 1 ECL card (Shimmerwilds Growth)
16. Triggered ability doubling - 1 ECL card (Twinflame Travelers)
17. "Can't untap" static effect - 1 ECL card (Blossombind)
18. Mass hexproof+indestructible by type - 1 ECL card (Selfless Safewright)
19. Enter-as-copy with changeling - 1 ECL card (Omni-Changeling)
20. Graveyard-to-battlefield copy trigger - 1 ECL card (Twilight Diviner)

**Tier 3 - Simple per-card fixes (compose existing effects):**
21. Cards where Custom strings can be directly replaced by existing typed Effect variants (e.g., "Each opponent loses 1 life" -> LoseLifeOpponents)

## Task List

### Phase 0: Commit Existing Work
- [x] Task 0.1: Commit uncommitted cost reduction system (game.rs + mana.rs changes with tests)

### Phase 1: Generic Stub Replacement (Easy Wins)
Replace generic Custom placeholders with compositions of existing typed effects. Each sub-task is one card or small batch of related cards. Analyze each card's Oracle text (check Java reference if needed), then replace Custom with the correct combination of existing Effect variants.

- [x] Task 1.1: Audit all generic-placeholder ECL cards and all 88 Custom fallbacks
- [x] Task 1.2: Replace generic placeholders on 14 cards with descriptive Custom strings matching Oracle text. Also added missing card data and typed variants where possible.
- [x] Task 1.3: Replace simple Custom strings with existing Effect variants where possible (4 cards updated)

### Phase 2: New Engine Features (Ordered by Dependency + Impact)

Each task: add engine feature, add tests, update ECL cards to use it, commit.

- [x] Task 2.1: Add `Effect::LoseAllAbilities` - remove all abilities from target creature (Java ref: `LoseAllAbilitiesTargetEffect.java`). Update 4 ECL cards: Abigale, Curious Colossus, Noggle the Mind, Retched Wretch. Tests + commit.

- [x] Task 2.2: Add `Effect::SetBasePowerToughnessAll`, `Effect::LoseAllAbilitiesAll`, `StaticEffect::SetBasePowerToughness` - mass P/T setting and continuous base P/T override. Update Curious Colossus, Noggle the Mind. Tests + commit.

- [x] Task 2.3: Add `Effect::PutFromHandToBattlefield { max_mana_value, max_mv_dynamic, tapped, attacking, haste, sacrifice_eot }` - put creature from hand onto battlefield. Update Meek Attack, Kinscaer Sentry. Tests + commit.

- [x] Task 2.4: Add `StaticEffect::CantUntap { filter }` - prevents permanents from untapping. Update Blossombind. Tests + commit.

- [x] Task 2.5: Add `Effect::ChooseTypeAndReturnFromGraveyard` - compound effect that chooses a creature type then returns all matching creatures from graveyard to battlefield (needed because sorceries can't chain ChooseCreatureType → ReturnAllTypeFromGraveyard). Update Bloodline Bidding. Tests + commit.

- [x] Task 2.6: Add `Effect::ChooseTypeAndGrantKeywords { keywords, other_only }` - compound effect choosing creature type then granting keywords to matching permanents until EOT. Update Selfless Safewright. Tests + commit.

- [x] Task 2.7: Add `StaticEffect::SetPowerToColorCount` — continuous Vivid power-setting effect. Update Squawkroaster. Tests + commit.

- [x] Task 2.8: Add `StaticEffect::AssignDamageWithToughness` - creature assigns combat damage equal to toughness. Update Bark of Doran. Java ref: `CombatDamageByToughnessAllEffect` / custom. Tests + commit.

- [x] Task 2.9: Add `Effect::BecomesCreature { power, toughness }` - make artifact become a creature temporarily until end of turn. Update Firdoch Core. Tests + commit.

- [x] Task 2.10: Implement Convoke cost payment - tap creatures to reduce mana cost. Update Omni-Changeling, Eirdu. Java ref: `ConvokeAbility.java`. Tests + commit.

- [x] Task 2.11: Add `StaticEffect::DamageDoublingFromType` - double damage from chosen type. Update Collective Inferno. Tests + commit.

- [x] Task 2.12: Add `StaticEffect::ManaDoublingBasicLands` - basic lands produce double mana. Update Lavaleaper. Tests + commit.

- [x] Task 2.13: Add `StaticEffect::EnhancedManaProduction { filter, mana }` - enchanted land produces additional mana. Update Shimmerwilds Growth. Tests + commit.

- [x] Task 2.14: Add `StaticEffect::TriggerDoubling { filter }` - triggered abilities of matching permanents trigger additional time. Update Twinflame Travelers. Tests + commit.

- [x] Task 2.15: Implement Conspire mechanic - tap two creatures sharing a color to copy spell. Update Raiding Schemes. Tests + commit.

### Phase 3: Complex Engine Systems (Higher Effort)

- [x] Task 3.1: Implement basic replacement effect pipeline - hook into event system for enters-with-counters and death replacement. Java ref: `ReplacementEffectImpl.java`. Tests + commit.

- [x] Task 3.2: Implement Transform/DFC basics - card can transform into back face. Update Eirdu/Isilu. Tests + commit.

- [ ] Task 3.3: Implement enter-as-copy - clone creature on ETB. Update Omni-Changeling. Java ref: `CopyEffect.java`. Tests + commit.

- [ ] Task 3.4: Implement graveyard-ETB copy trigger (once per turn). Update Twilight Diviner. Tests + commit.

- [ ] Task 3.5: Add dynamic boost based on toughness-power difference. Update Doran Besieged by Time. Tests + commit.

- [ ] Task 3.6: Add conditional cost reduction for toughness > power creatures. Update Doran. Tests + commit.

- [ ] Task 3.7: Implement complex multi-part effects for remaining ECL cards with unique Custom strings. Tests + commit.

### Phase 4: Verification & Cleanup

- [ ] Task 4.1: Run full test suite, fix any failures
- [ ] Task 4.2: Run `cargo check` on all crates, fix warnings
- [ ] Task 4.3: Audit ECL set - count remaining Custom fallbacks, verify reduction
- [ ] Task 4.4: Update ROADMAP.md with completed items and new counts

## Notes

### Key Decisions
1. **ECL focus first** - ECL has the fewest Custom fallbacks (88 vs 401 for FDN), making it the best target for "complete a set" milestone
2. **Engine features before card fixes** - Adding typed Effect variants first, then batch-updating cards that use them
3. **Commit after each item** - Per the plan, commit after each task to maintain clean history
4. **Tests before implementation** - Per the plan, write tests before implementing changes (TDD)
5. **Read Java source** - For each new engine feature, read the corresponding Java XMage implementation to shape the Rust version

### Dependencies
- Task 0.1 (commit existing) has no dependencies
- Phase 1 tasks are independent of each other
- Task 2.1 (LoseAllAbilities) should come before 2.2 (SetPowerToughnessAll) since Curious Colossus needs both
- Task 3.1 (replacement effects) blocks Renew/Endure mechanics (TDM, not ECL)
- Task 3.2 (Transform) is independent but complex
- Task 3.3 (enter-as-copy) depends on CreateTokenCopy infrastructure (already done)

### Risk Areas
- **Replacement effect pipeline** (Task 3.1) is architecturally complex - may need to be broken into sub-tasks
- **Transform/DFC** (Task 3.2) requires significant engine changes (dual-faced card data, transform action)
- **Convoke** (Task 2.10) requires changes to mana payment system which is delicate
- **Generic stubs** (Phase 1) require manual Oracle text lookup per card - time-consuming but straightforward
- **Some Custom strings may be truly unique** and need new one-off Effect variants

### Metrics to Track
- ECL Custom fallback count: 68 Effect::Custom + 20 StaticEffect::Custom = 88 (starting point)
- Target: 0 Custom fallbacks in ECL
- Tests passing: 390 (current) -> should increase with each task

## Tasks Completed

- Task 0.1: Committed cost reduction system (game.rs + mana.rs) — Mana::reduce_generic(), Game::calculate_cost_reduction(), integration into legal actions and spell payment. 3 tests.
- Task 1.1: Audited all 88 ECL Custom fallbacks. Found: 18 generic placeholders (14 unique cards), ~3 simple replacements, ~8 partial replacements, ~22 need new engine features. Key finding: ALL 14 generic-placeholder cards need complex new engine features (replacement effects, transform, delayed triggers, watcher patterns, etc.), not simple stub replacement.
- Task 1.2: Replaced all 18 generic placeholders across 14 ECL cards with descriptive Oracle-text Custom strings. Added missing card data for Grub and Spinerock. Replaced some effects with typed variants (Mill, ChooseCreatureType, AddCountersSelf, RemoveCounters, BlightOpponents, Equip). 0 generic placeholders remain.
- Task 1.3: Replaced Custom strings with typed Effect variants on 4 cards: (1) champions_of_the_shoal: Custom→TapTarget+AddCounters("stun") with proper TargetSpec; (2) ajani_outland_chaperone: Custom→CreateToken("1/1 Kithkin"), added 2 missing loyalty abilities; (3) swat_away: restructured from misplaced Custom to CostReduction static+PutOnLibrary spell; (4) goatnap: added missing UntapTarget+GainKeywordEot("haste"). Net: +7 typed effects, -1 Custom. ECL now at 48 Effect::Custom + 20 StaticEffect::Custom = 68 unique Custom lines.
- Task 2.1: Added Effect::LoseAllAbilities (one-shot) and StaticEffect::LoseAllAbilities (continuous) to engine. One-shot version sets removed_keywords=all() and clears ability store; continuous version reapplied each recalculation cycle. Added abilities_lost field to Permanent. Updated 4 ECL cards: Abigale (ETB), Curious Colossus (ETB), Noggle the Mind (static aura), Retched Wretch (dies). 7 new tests. Net: -3 Effect::Custom, -1 StaticEffect::Custom (but +1 Custom kept for type/P/T changes on Curious Colossus and Noggle). 325 total engine tests passing.
- Task 2.2: Added 3 new effect variants: (1) Effect::SetBasePowerToughnessAll — one-shot mass base P/T setting for all creatures matching filter (modifies card.power/toughness directly); (2) Effect::LoseAllAbilitiesAll — one-shot mass ability removal for all matching creatures; (3) StaticEffect::SetBasePowerToughness — continuous base P/T override (Layer 7b) using new base_power_override/base_toughness_override fields on Permanent. Updated Curious Colossus to use lose_all_abilities_all + set_base_pt_all instead of old broken LoseAllAbilities with player targeting. Updated Noggle the Mind to use StaticEffect::set_base_pt for continuous 1/1 override. 7 new tests covering mass effects, continuous aura, layer interaction with boosts, and aura removal reset. 332 engine tests passing.
- Task 2.3: Added Effect::PutFromHandToBattlefield with fields: max_mana_value (fixed/X_VALUE), max_mv_dynamic (evaluate_count_filter for dynamic MV), tapped, attacking, haste, sacrifice_eot. Three helper constructors: put_from_hand_with_haste_sacrifice(), put_from_hand_tapped_attacking(), put_from_hand_tapped_attacking_dynamic(). Added "attacking creatures you control" pattern to evaluate_count_filter. Updated Meek Attack (Custom→put_from_hand_with_haste_sacrifice(2)) and Kinscaer Sentry (Custom→put_from_hand_tapped_attacking_dynamic("attacking creatures you control")). 11 new tests. Net: -2 Effect::Custom. 343 engine tests passing.
- Task 2.4: Added StaticEffect::CantUntap { filter } to prevent permanents from untapping during untap step. Added cant_untap field to Permanent struct, CantUntap variant to StaticEffect enum with cant_untap() helper, integration into apply_continuous_effects (Step 2d) and turn_based_actions (Untap). Updated Blossombind ECL card (Custom→cant_untap("enchanted creature")). 8 new tests covering: direct flag, aura enchantment, self filter, aura removal reset, selective targeting. Net: -1 StaticEffect::Custom. 351 engine tests passing.
- Task 2.5: Added Effect::ChooseTypeAndReturnFromGraveyard — compound effect combining creature type choice with mass graveyard reanimation. Needed because sorceries (like Bloodline Bidding) don't have a permanent on the battlefield, so ChooseCreatureType can't store chosen_type for a separate ReturnAllTypeFromGraveyard to read. Follows the ChooseTypeAndDrawPerPermanent pattern: choice + action in one effect resolution. Updated Bloodline Bidding ECL card (removed choose_creature_type() + Custom pair, replaced with single choose_type_and_return_from_graveyard()). 7 new tests covering: multiple matching creatures, non-matching type filtering, non-creature card filtering, empty graveyard, different type index selection, helper constructor, controller-only scope. Net: -1 Effect::Custom, -1 ChooseCreatureType. 358 engine tests passing.
- Task 2.6: Added Effect::ChooseTypeAndGrantKeywords { keywords, other_only } — compound effect that chooses a creature type then grants specified keywords to matching permanents you control until end of turn. Uses granted_keywords field (cleared at EOT). Supports other_only flag to exclude source permanent (for "other permanents you control" wording). Changeling creatures also match any chosen type. Updated Selfless Safewright ECL card (removed choose_creature_type() + Custom pair, replaced with single choose_type_and_grant_keywords(["hexproof", "indestructible"], true)). 8 new tests covering: matching creatures granted both keywords, source exclusion with other_only, source inclusion without other_only, non-matching type filtering, opponent creature filtering, single keyword granting, helper constructor, end-of-turn cleanup. Net: -1 Effect::Custom, -1 ChooseCreatureType. 366 engine tests passing.
- Task 2.7: Added StaticEffect::SetPowerToColorCount — continuous static effect (Layer 7b) that sets a creature's base power to the number of distinct colors among permanents its controller controls. Uses existing count_colors_among_permanents() function and base_power_override mechanism. Added helper constructor set_power_to_color_count(). Updated Squawkroaster ECL card (Custom→set_power_to_color_count()). 8 new tests covering: zero colors, single color, three colors, all five colors, multicolored permanents, duplicate color deduplication, opponent permanents excluded, helper constructor. Net: -1 StaticEffect::Custom. 374 engine tests passing.
- Task 2.8: Added StaticEffect::AssignDamageWithToughness { filter, condition } — creature assigns combat damage equal to toughness rather than power. Supports unconditional variant and conditional "toughness_greater_than_power" variant. Added assign_damage_with_toughness bool field to Permanent struct. Modified assign_combat_damage() and assign_blocker_damage() in combat.rs to use toughness when flag is set. Applied after all P/T effects in apply_continuous_effects (Step 8) so condition checks use final P/T. Added helper constructors: assign_damage_with_toughness() (unconditional) and assign_damage_with_toughness_if_greater() (conditional). Updated Bark of Doran ECL card (Custom→assign_damage_with_toughness_if_greater("equipped creature")). 13 new tests: 5 combat unit tests (attacker/blocker with/without flag, trample interaction) + 8 integration tests (unconditional flag, conditional T>P/P>T/equal, flag cleared on recalc, unequipped creature, both helper constructors). Net: -1 StaticEffect::Custom. 387 engine tests passing.
- Task 2.9: Added Effect::BecomesCreature { power, toughness } — one-shot effect that temporarily makes the source permanent become a creature until end of turn. Added added_card_types field to Permanent for temporary type additions, and base_power_eot/base_toughness_eot for temporary P/T setting (takes priority over base_*_override from continuous effects). Updated is_creature(), is_artifact(), is_enchantment(), has_card_type() to check added_card_types. Cleanup step clears all temporary type/P/T changes. Added helper constructor becomes_creature(). Updated Firdoch Core ECL card (Custom→becomes_creature(4, 4)). 9 new tests covering: type addition, P/T setting, cleanup revert, counter interaction, no-double-type for existing creatures, continuous boost interaction, has_card_type check, helper constructor, EOT priority over override. Net: -1 Effect::Custom. 396 engine tests passing.
- Task 2.10: Implemented Convoke cost payment mechanic. Added 3 new methods to Game: spell_has_convoke() (checks keyword + GrantConvoke static effects), calculate_convoke_mana() (counts untapped creatures as virtual mana — colored creatures produce any, colorless produce generic), pay_convoke_cost() (taps creatures to produce mana, prioritizing colored needs then generic). Added Mana::can_pay_with_convoke() for affordability checks. Added StaticEffect::GrantConvoke { filter } with grant_convoke() helper for "creature spells you cast have convoke" effects (Eirdu). Modified compute_legal_actions to check convoke affordability (hand + exile). Modified cast_spell to tap creatures and add convoke mana to pool before try_pay. Updated 5 ECL cards: Omni-Changeling (Custom→CONVOKE keyword), Eirdu (Custom→grant_convoke("creature spells")), Harmonized Crescendo (added missing CONVOKE keyword), Lofty Dreams (added missing CONVOKE keyword), Winnowing (added CONVOKE keyword, removed "Convoke." prefix from Custom text). 14 new tests covering: keyword detection, grant detection, mana calculation (untapped/tapped/colorless/opponent exclusion), affordability checks, legal actions with convoke, spell casting with creature tapping, colored mana payment, helper constructor. Net: -1 StaticEffect::Custom, +CONVOKE keyword on 3 cards. 410 engine tests passing.
- Task 2.11: Added StaticEffect::DamageDoublingFromType — doubles all damage dealt by sources the controller controls of the chosen creature type. Stored as (PlayerId, SubType) pairs on GameState.damage_doublings, rebuilt each apply_continuous_effects cycle (Step 9). Added get_damage_multiplier() helper to Game. Applied multiplier to all damage pathways: combat (attacker + blocker), DealDamage, DealDamageAll, DealDamageOpponents, DealDamageVivid, Fight, Bite. Multiple doublings stack multiplicatively. Changeling creatures match any chosen type. Updated Collective Inferno ECL card (Custom→damage_doubling_from_type()). 9 new tests covering: matching type doubling, non-matching type, opponent creature exclusion, changeling matching, enchantment removal revert, multiplicative stacking, no chosen type, player damage doubling, helper constructor. Net: -1 StaticEffect::Custom. 395+9=404 engine tests passing (486 total across all crates).
- Task 2.12: Added StaticEffect::ManaDoublingBasicLands — basic lands produce one additional mana of the same type when tapped. Tracked as u32 counter on GameState.mana_doubling_basic_lands, rebuilt each apply_continuous_effects cycle. Applied in activate_mana_ability: checks if source permanent is a basic land (has SuperType::Basic + is_land()), then adds extra copies of produced mana for each active doubler. Multiple doublers stack additively (N doublers = base + N extra copies). Applies to ALL players' basic lands (matching Lavaleaper's Oracle text). Added mana_doubling_basic_lands() helper constructor. Updated Lavaleaper ECL card (Custom→mana_doubling_basic_lands()). 8 new tests covering: basic land doubled, nonbasic not doubled, opponent basic lands also doubled, multiple doublers stacking, removal reverts, no effect without doubler, different colored mana, helper constructor. Net: -1 StaticEffect::Custom. 403 engine tests passing (494 total across all crates).
- Task 2.13: Added StaticEffect::EnhancedManaProduction — enchanted land produces one additional mana of the aura's chosen color when tapped. Added chosen_color: Option<ManaColor> field to Permanent for "choose a color" effects. Added Effect::ChooseColor for ETB color selection (5-color menu via choose_option). Added Mana::of_color() helper. Tracked as Vec<(aura_id, land_id, color)> on GameState.enhanced_mana_productions, rebuilt each apply_continuous_effects cycle. Applied in activate_mana_ability: for each aura pointing at the tapped land, adds 1 mana of the chosen color. Multiple auras on the same land stack (each adds its own chosen color). Updated Shimmerwilds Growth ECL card (Custom→enters_battlefield_triggered ChooseColor + static EnhancedManaProduction). 10 new tests covering: enchanted land produces additional mana, non-enchanted land unaffected, works on nonbasic lands, removal reverts, multiple auras on same land, same-color stacking, no effect without aura, helper constructors (StaticEffect + Effect + Mana::of_color). Net: -1 StaticEffect::Custom. 413 engine tests passing (504 total across all crates).
- Task 2.14: Added StaticEffect::TriggerDoubling { filter } — triggered abilities of matching permanents the controller controls trigger an additional time. Added trigger_doubling() helper constructor. Tracked as Vec<(source_id, controller, filter)> on GameState.trigger_doublings, rebuilt each apply_continuous_effects cycle. Applied in check_triggered_abilities after collecting triggers but before pushing to stack: iterates over collected triggers, checks if source permanent matches any doubling filter (using find_matching_permanents/matches_filter), duplicates matching triggers. Supports "other" exclusion (doubler's own triggers not doubled), controller scoping (only controller's permanents matched), subtype matching, and Changeling. Multiple doublers stack additively. Updated Twinflame Travelers ECL card (Custom→trigger_doubling("other Elementals you control")). 9 new tests covering: matching Elemental ETB doubled, non-matching type, self-exclusion via "other", opponent exclusion, removal reverts, multiple doublers stacking, changeling matching, no doubling without doubler, helper constructor. Net: -1 StaticEffect::Custom. 422 engine tests passing (513 total across all crates).
- Task 2.15: Implemented Conspire mechanic — tap two untapped creatures sharing a color with the spell to copy it on the stack. Added CONSPIRE keyword flag (bit 47), StaticEffect::GrantConspire { filter } with grant_conspire() helper, spell_has_conspire() (checks keyword + GrantConspire static effects), count_conspire_eligible_creatures() (counts untapped creatures sharing color with spell), pay_conspire_cost() (taps 2 eligible creatures), copy_spell_on_stack() (basic spell copy on stack with same targets/X). Added "noncreature spells" filter to spell_matches_cost_filter. Integrated into cast_spell: after putting spell on stack, checks conspire eligibility and asks player via choose_use, then pays cost and copies. Updated Raiding Schemes ECL card (Custom→grant_conspire("noncreature spells")). Updated ROADMAP.md spell copying + additional costs rows. 10 new tests covering: keyword detection, no-keyword detection, eligible creature color matching, tapped creature exclusion, opponent creature exclusion, no-matching-color, grant via static effect (noncreature only), copy_spell_on_stack creates copy, helper constructor, pay_conspire taps exactly 2. Net: -1 StaticEffect::Custom. 432 engine tests passing (523 total across all crates).
- Task 3.1: Implemented enters-with-counters replacement effect. Added StaticEffect::EntersWithCounters { counter_type, count } variant with enters_with_counters() helper constructor. Added check_enters_with_counters() method to Game (mirrors check_enters_tapped pattern). Hooked into all 9 ETB paths in game.rs: play_land, resolve_top_of_stack (permanent spells), Reanimate, Flicker, ReturnFromExileTapped, ReturnFromGraveyardToBattlefield, PutCardFromHandOntoBattlefield, FlickerUntilEndOfTurn, FlickerEndStep return. Updated 15 ECL cards from incorrect enters_battlefield_triggered+add_counters pattern to correct static_ability+EntersWithCounters: Brambleback Brute, Burdened Stoneback, Encumbered Reejerey, Gnarlbark Elm, Heirloom Auntie, Moonlit Lamenter, Reluctant Dounguard, Bristlebane Battler, Glen Elendra Guardian, Loch Mare, Moonshadow, Reaping Willow, Creakwood Safewright, Hovel Hurler, Slumbering Walker. Updated 5 registry test assertions. 7 new tests covering: -1/-1 counters with P/T verification, +1/+1 counters, no-op without ability, multiple counter types, helper constructor, 6 M1M1 on large creature, stun counters. Net: -15 incorrect ETB triggers converted to proper replacement effects. 439 engine tests passing (530 total across all crates).
- Task 3.2: Implemented Transform/DFC system. Added back_face: Option<Box<CardData>> field to CardData for double-faced cards. Added front_face: Option<Box<CardData>> field to Permanent for storing original face when transformed. Added Permanent::can_transform() and Permanent::transform() methods that swap card characteristics while preserving counters, damage, tapped state, and controller. Added Effect::TransformSelf variant with transform_self() helper. Implemented transform logic in game.rs: swaps card data, re-registers abilities in ability store, emits Transformed event. Added PrecombatMainPre event emission at beginning of first main phase for "at the beginning of your first main phase" triggers. Updated Eirdu/Isilu ECL card: Eirdu now has full back face (Isilu, Carrier of Twilight with persist static + transform trigger), both faces use DoIfCostPaid({B}/{W}) + TransformSelf for the "you may pay to transform" trigger. Note: Figure of Fable is NOT a DFC — it's a level-up style card using activated abilities for type changes (already implemented). 9 new tests covering: swap characteristics, round-trip transform, counter preservation, tapped state preservation, no-back-face no-op, object ID preservation, ability re-registration, helper constructor, can_transform check. 448 engine tests passing (539 total across all crates).
