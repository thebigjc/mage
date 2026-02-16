# Progress: plan

Started: Mon Feb 16 12:34:17 AM EST 2026

## Status

IN_PROGRESS

## Analysis

### Current State
- **Engine tests**: 531 (mtg-engine), all passing
- **ECL Custom fallbacks**: 30 Effect::Custom + 7 StaticEffect::Custom + 0 Cost::Custom = 37 total
- **Other sets**: FDN 336, TLA 199, TDM 110 (these are out of scope per the plan)

### Goal
The plan says: "bring our rust engine into parity with Java" and "complete the fully functional implementation of ECL by implementing engine capabilities and updating cards to use them." This means we should systematically eliminate the remaining 44 ECL Custom fallbacks by adding engine features and updating card definitions.

### Approach
Group the 44 fallbacks by what engine feature they need. Implement the engine feature first (with tests), then update all ECL cards that benefit. Work one task at a time. Commit after each item.

### Fallback Categories

**Easy (existing effects can handle with minor extensions):**
- Conditional effects based on game state (blight paid, creature type, permanent count)
- Vivid variants (search, put-onto-battlefield)
- Type-adding effects

**Medium (new Effect variant needed, but uses existing infrastructure):**
- Dynamic exile-and-play (X = counters on creature)
- Mill-and-select (put creature/land from milled on top)
- Dynamic dual-target boost (X = count of type)
- Conditional token creation
- Ability resolution count tracking

**Hard (significant engine work):**
- Spell copy with modification (copy next spell, add wither)
- Granting temporary triggered abilities
- Mass copy (all permanents become copy of target)
- Variable additional cost (blight X, scale damage)
- Cast-from-exile systems (remove counters as cost, MV restriction)
- Type-changing layer effects
- Persist keyword granting
- Color-dependent hexproof

## Task List

### Tier 1: Easy Effect::Custom eliminations (compose from existing effects)

- [x] Task 1: Add `ConditionalEffect` variant — `Effect::Conditional { condition, if_true, if_false }` for cards that do "if X, then Y". Updated: **requiting_hex** (DoIfCostPaid with Blight), **goatnap** (target is a Goat), **tend_the_sprigs** (7+ lands/Treefolk), **wanderwine_farewell** (you control a Merfolk). Added evaluate_condition_with_targets, count_permanents_matching, permanent_matches_filter_part. 6 new tests, 499 engine total. 4 Effect::Custom eliminated.

- [x] Task 2: Add `AddSubtypeAll` effect — `Effect::AddSubtypeAll { subtype: String, filter: String }` for type-adding effects. Updated: **curious_colossus** (add Coward subtype to opponent creatures). Added engine effect variant, builder method, game.rs resolution handler. 1 test added, 500 engine total. 1 Effect::Custom eliminated.

- [x] Task 3: Add `SearchLibraryVivid` effect — Vivid variant of SearchLibrary that searches for X basic lands where X = colors among permanents. Update: **prismatic_undercurrents**. Java uses `ColorsAmongControlledPermanentsCount` with `TargetCardInLibrary`. Add engine test. ~1 card fixed.

### Tier 2: Medium — new Effect variants using existing infrastructure

- [x] Task 4: Add `MillAndSelect` effect — Mill N cards, then pick one matching a filter (creature/land) and put it on top of library or into hand. Update: **lluwen_imperfect_naturalist** (mill 4, put creature/land on top). Java uses a custom OneShotEffect. Add engine test. ~1 card fixed.

- [x] Task 5: Add `MillAndReturnAll` effect — Mill N, return all cards of a specific type from among milled to hand. Update: **grubs_command** mode 4 (mill 5, return Goblins). Java uses custom mill effect. Add engine test. ~1 card fixed.

- [x] Task 6: Add `BoostDualTargetDynamic` effect — Give +X/+0 to one target and -0/-X to another, where X = count of permanents matching filter. Update: **gloom_ripper** (X = Elves you control + Elf cards in graveyard). Added Effect::BoostDualTargetDynamic variant, additive "+" support in evaluate_count_filter, irregular plural handling (depluralize_type). 2 new tests, 508 engine total. 1 Effect::Custom eliminated.

- [x] Task 7: Add `RevealFromLibraryVivid` effect — Reveal cards from library until X permanents found (X = colors among permanents), put some onto battlefield. Update: **aurora_awakener** (Vivid ETB reveal+deploy). Java uses `ColorsAmongControlledPermanentsCount`. Add engine test. ~1 card fixed.

- [x] Task 8: Add `CompareAndBoost` effect — Choose two creatures, compute X = abs(power difference), draw X cards, boost both +X/+X and grant trample. Updated: **spry_and_mighty**. Added Effect::CompareAndBoost variant, builder, game.rs resolution (power diff, draw, P1P1 counters, trample). 2 new tests, 513 engine total. 1 Effect::Custom eliminated.

- [x] Task 9: Add `ExileTopAndPlayDynamic` effect — Exile X cards from library (X = dynamic value like counters on a creature), play until next end step. Update: **shadow_urchin** (X = counters on dying creature). Java uses `ExileTopXMayPlayUntilEffect` with `ShadowUrchinValue`. Add engine test. ~1 card fixed.

- [x] Task 10: Add `ConditionalTokenCreation` — Already covered by Task 1. Both **tend_the_sprigs** and **wanderwine_farewell** already use `Effect::conditional()` + `Effect::create_token()`. No Effect::Custom remains.

### Tier 3: Medium-Hard — new systems needed

- [x] Task 11: Add `IfAbilityResolvedNTimes` effect — Added `Effect::IfAbilityResolvedNTimes { resolution_number, effects }` variant. Added `ability_resolution_counts_this_turn` to GameState, tracked in `resolve_top_of_stack`, reset at turn start. Added `resolving_ability_id` transient field on Game for effect context. Updated **soulbright_seeker** from `Effect::Custom` to `Effect::if_resolved_n_times(3, vec![Effect::add_mana(Mana::red(4))])`. 3 new tests (519 engine total). 1 Effect::Custom eliminated.

- [x] Task 12: Add `GrantTriggeredAbilityUntilEOT` effect — Added `Effect::GrantTriggeredAbilityUntilEOT { event_type, filter, trigger_effects }` variant. Added `controller_filter` field to `DelayedTrigger` for filtering by controller's creatures. Support "any" counter type in `Cost::RemoveCounters`. Updated **flitterwing_nuisance**: enters with -1/-1 counter, remove-any-counter cost, grants combat-damage-draw to creatures you control. 3 new tests (522 engine total). 1 Effect::Custom eliminated.

- [x] Task 13: Add `CopyNextSpell` effect — Added `Effect::CopyNextSpell` variant. Creates a delayed trigger on SpellCast with `copy_spell: true` flag. Added `copy_spell` boolean to `DelayedTrigger` struct. Delayed trigger handler checks that the cast spell is instant/sorcery and was cast by the trigger's controller, then calls `copy_spell_on_stack`. Updated **rimefire_torque**: moved RemoveCounters from effects to costs, replaced Effect::Custom with Effect::copy_next_spell(). 3 new tests (525 engine total). 1 Effect::Custom eliminated.

- [x] Task 14: Add `CopySpellWithModification` effect — Added `Effect::CopyTriggeringSpell { keywords, single_target_only }` variant. Finds triggering instant/sorcery on stack, optionally checks single-target, copies it, grants keywords to both original and copy. Added wither/infect/shadow to `keyword_from_name()`. Updated **spinerock_tyrant**: replaced Effect::Custom with `Effect::copy_triggering_spell(vec!["wither"], true)`. 3 new tests (528 engine total). 1 Effect::Custom eliminated.

- [x] Task 15: Add `VariableBlightCost` — Added `Cost::VariableBlight` variant that lets the player choose X (up to greatest toughness among controlled creatures), puts X -1/-1 counters on a chosen creature, and sets the spell's x_value. Added `Effect::DealDamageOpponentsCreatures` for dealing damage to opponents' creatures. Added `variable_blight_amount` transient field on Game for cost→spell X value propagation. Updated **soul_immolation**: replaced Effect::Custom with `Cost::variable_blight()` + `Effect::damage_opponents(X_VALUE)` + `Effect::damage_opponents_creatures(X_VALUE)`. 3 new tests (531 engine total). 1 Effect::Custom eliminated.

- [ ] Task 16: Add `OpponentRevealsExileCast` effect — Target opponent reveals X cards from library (X = dynamic count), you exile one, may cast it. Update: **taster_of_wares** (X = Goblins you control). Java uses custom effect. Add engine test. ~1 card fixed.

- [ ] Task 17: Add `MassExileAndCast` effect — Each opponent exiles from library until MV threshold, you may cast exiled cards. Update: **dream_harvest** (opponents exile until MV 5+). Java uses custom effect. Add engine test. ~1 card fixed.

- [ ] Task 18: Add `DealDamageWithDelayedExile` effect — Deal X damage to creature, create delayed trigger: when it dies this turn, exile cards = its power, choose one to cast. Update: **end_blaze_epiphany**. Java uses complex custom effect. Add engine test. ~1 card fixed.

### Tier 4: Hard — requires new engine subsystems

- [ ] Task 19: Add `FigureOfFableTransform` / `LevelUp` effect — Conditional self-transformation: check current types, change types/P/T/keywords. Update: **figure_of_fable** (Scout->Soldier 4/5, Soldier->Avatar 7/8 with protection). Java uses class-level system. Add engine test. ~1 card (2 Custom instances) fixed.

- [ ] Task 20: Add `WinnowingEffect` — For each player, choose one creature, sacrifice all others that don't share a creature type. Update: **winnowing**. Java uses `SharesCreatureTypePredicate`. Add engine test. ~1 card fixed.

- [ ] Task 21: Add `GlenElendrasAnswer` — Counter all spells and abilities opponents control on the stack, create tokens. Update: **glen_elendras_answer**. Java uses mass countering. Add engine test. ~1 card fixed.

- [ ] Task 22: Add `MassBecomeCopy` effect — Each nonland permanent you control becomes a copy of target. Update: **mirrorform**. Java uses complex copy effects. Add engine test. ~1 card fixed.

- [ ] Task 23: Add exile-with-dream-counters system — Replacement effect: instants/sorceries you cast go to exile with dream counters instead of graveyard. Static ability: cast exiled spells with dream counters for free. Update: **goliath_daydreamer** (2 Custom instances). Java uses `AsThoughEffectImpl` + replacement. Add engine test. ~1 card (2 Custom instances) fixed.

### Tier 5: Hard — StaticEffect::Custom eliminations

- [ ] Task 24: Add `CastFromExileWithCounterCost` static effect — Allow casting creature spells exiled by this source by removing 3 counters from creatures you control. Update: **dawnhand_dissident**. Java uses `AsThoughEffectImpl` + `RemoveCounterCost`. Add engine test. ~1 card fixed.

- [ ] Task 25: Add `GrantPersist` static effect — "Each other nontoken creature you control has persist." Needs persist keyword enforcement (return with -1/-1 counter on death). Update: **eirdu_carrier_of_dawn**. Java uses `GainAbilityAllEffect` with persist. Add engine test. ~1 card fixed.

- [ ] Task 26: Add `BoostPerTurnEvent` static effect — Dynamic +X/+X where X = creatures that entered the battlefield this turn. Needs per-turn event counting watcher. Update: **kinbinding**. Java uses `KinbindingWatcher` + `DynamicValue`. Add engine test. ~1 card fixed.

- [ ] Task 27: Add `CastExiledOncePerTurn` static effect — Once per turn, you may cast exiled spells with MV <= count of permanents matching filter, without paying mana cost. Update: **maralen_fae_ascendant**. Java uses `AsThoughEffectImpl` + `OnceEachTurnCastWatcher`. Add engine test. ~1 card fixed.

- [ ] Task 28: Add `ReplaceTokenCreation` static effect — First time you would create tokens each turn, instead create token copies of equipped creature. Update: **mirrormind_crown**. Java uses `ReplacementEffectImpl`. Add engine test. ~1 card fixed.

- [ ] Task 29: Add `BecomesCreatureAttached` static effect — Enchanted creature loses all abilities and becomes a colorless 1/1 Noggle. Update: **noggle_the_mind**. Java uses `BecomesCreatureAttachedEffect`. Add engine test. ~1 card fixed.

- [ ] Task 30: Add `HexproofFromOwnColors` static effect — Each other creature you control has hexproof from each of its colors. Update: **tam_mindful_first_year**. Java uses `HexproofBaseAbility.getFromColor()`. Add engine test. ~1 card fixed.

### Verification

- [ ] Task 31: Final verification — Run `cargo check -p mtg-cards`, `cargo test --lib -p mtg-engine`, `cargo test`, verify all pass. Count remaining Custom fallbacks in ECL. Update ROADMAP.md with results.

## Task Dependencies

```
Task 1 (ConditionalEffect) <- Tasks 10 (conditional tokens — may be subsumed)
Task 13 (CopyNextSpell) <- Task 14 (CopySpellWithModification extends it)
Tasks 1-10 are independent of each other
Tasks 11-18 are independent of each other
Tasks 19-23 are independent of each other
Tasks 24-30 are independent of each other
Task 31 depends on all others
```

## Priority Order (recommended implementation sequence)

**Start with highest-impact, lowest-effort tasks:**
1. Task 1 (ConditionalEffect) — fixes 4 cards at once
2. Task 2 (AddSubtype) — simple, fixes curious_colossus
3. Task 3 (SearchLibraryVivid) — extends existing Vivid infra
4. Task 4 (MillAndSelect) — straightforward new effect
5. Task 5 (MillAndReturnType) — similar to Task 4
6. Task 6 (BoostDualTargetDynamic) — new but uses existing patterns
7. Task 7 (RevealFromLibraryVivid) — extends Vivid + library
8. Task 8 (CompareAndBoost) — standalone complex effect
9. Task 9 (ExileTopAndPlayDynamic) — extends impulse draw
10. Task 11 (IfResolvedNTimes) — needs watcher system
11. Task 12 (GrantTemporaryTriggeredAbility) — extends delayed triggers
12. Task 13 (CopyNextSpell) — builds on Conspire
13. Task 14 (CopySpellWithModification) — extends Task 13
14. Task 15 (VariableBlightCost) — variable cost system
15. Tasks 16-18 (opponent library effects) — complex new effects
16. Tasks 19-23 (hard engine work) — significant new subsystems
17. Tasks 24-30 (StaticEffect::Custom) — each needs unique engine feature
18. Task 31 (verification)

## Completed This Iteration
- Task 15: Added `Cost::VariableBlight` for variable blight X additional cost. Added `Effect::DealDamageOpponentsCreatures` for dealing X damage to each creature opponents control. Updated soul_immolation from Effect::Custom to proper Cost + Effects. 3 new tests (531 engine total). 1 Effect::Custom eliminated.

## Notes

- All 44 ECL Custom fallbacks have corresponding Java XMage implementations that can be used as reference
- The plan explicitly says "do not implement any new sets" — focus only on engine features + ECL card updates
- Each task should: read Java source -> add engine feature with tests -> update ECL card(s) -> commit
- The guardrails warn about `rg -c "Effect::Custom"` double-counting StaticEffect::Custom
- Always verify with `cargo check -p mtg-cards` and `cargo test --lib -p mtg-engine`
- Card oracle text should be verified against scryfall.com before changes
- Some tasks may be combined if the engine feature serves multiple cards
- Hardest tasks (19-30) each fix only 1 card — consider if all are worth implementing vs. keeping as Custom
