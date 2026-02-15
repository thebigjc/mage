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

- [ ] Task 2.4: Add `StaticEffect::CantUntap { filter }` - prevents permanents from untapping. Update Blossombind. Tests + commit.

- [ ] Task 2.5: Add `Effect::MassReanimate { filter }` - return all cards of type from graveyard to battlefield. Update Bloodline Bidding. Tests + commit.

- [ ] Task 2.6: Add `Effect::GrantKeywordToType { keyword, filter, duration }` - mass keyword granting to specific types. Update Selfless Safewright. Tests + commit.

- [ ] Task 2.7: Extend `evaluate_count_filter` for "colors among permanents you control" pattern. Update Squawkroaster's StaticEffect to use BoostPerCount with color counting. Tests + commit.

- [ ] Task 2.8: Add `StaticEffect::AssignDamageWithToughness` - creature assigns combat damage equal to toughness. Update Bark of Doran. Java ref: `AssignNoCombatDamageSourceEffect` / custom. Tests + commit.

- [ ] Task 2.9: Add `Effect::AnimateArtifact { power, toughness, duration }` - make artifact become a creature temporarily. Update Firdoch Core. Tests + commit.

- [ ] Task 2.10: Implement Convoke cost payment - tap creatures to reduce mana cost. Update Omni-Changeling, Eirdu. Java ref: `ConvokeAbility.java`. Tests + commit.

- [ ] Task 2.11: Add `StaticEffect::DamageDoublingFromType { creature_type }` - double damage from chosen type. Update Collective Inferno. Tests + commit.

- [ ] Task 2.12: Add `StaticEffect::ManaDoublingBasicLands` - basic lands produce double mana. Update Lavaleaper. Tests + commit.

- [ ] Task 2.13: Add `StaticEffect::EnhancedManaProduction { filter, mana }` - enchanted land produces additional mana. Update Shimmerwilds Growth. Tests + commit.

- [ ] Task 2.14: Add `StaticEffect::TriggerDoubling { filter }` - triggered abilities of matching permanents trigger additional time. Update Twinflame Travelers. Tests + commit.

- [ ] Task 2.15: Implement Conspire mechanic - tap two creatures sharing a color to copy spell. Update Raiding Schemes. Tests + commit.

### Phase 3: Complex Engine Systems (Higher Effort)

- [ ] Task 3.1: Implement basic replacement effect pipeline - hook into event system for enters-with-counters and death replacement. Java ref: `ReplacementEffectImpl.java`. Tests + commit.

- [ ] Task 3.2: Implement Transform/DFC basics - card can transform into back face. Update Eirdu/Isilu, Figure of Fable. Tests + commit.

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
