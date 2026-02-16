# Progress: plan

Started: Mon Feb 16 12:34:17 AM EST 2026

## Status

IN_PROGRESS

## Analysis

### Current State
- **Engine tests**: 564 (mtg-engine), all passing
- **ECL Custom fallbacks**: 28 Effect::Custom + 7 StaticEffect::Custom + 0 Cost::Custom = 35 total (now 3 Effect::Custom + 3 StaticEffect::Custom remaining)
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

- [x] Task 16: Add `OpponentRevealsExileCast` effect — Added `Effect::OpponentRevealsFromHandExileCast { count_source, instant_sorcery_only }` variant. Added `ImpulseDuration::WhileSourceControlled { source_id, controller }` to track playability tied to permanent control. Opponent reveals X cards from hand (X = dynamic count via evaluate_count_filter), controller picks one to exile, instant/sorcery cards become impulse-playable while source is controlled. Updated WhileSourceControlled check in compute_legal_actions and end-of-turn cleanup. Updated **taster_of_wares**: fixed oracle text (hand not library), replaced Effect::Custom with `Effect::opponent_reveals_from_hand_exile_cast("Goblins you control", true)`. 3 new tests (534 engine total). 1 Effect::Custom eliminated.

- [x] Task 17: Add `OpponentsExileUntilMVAndCast` effect — Each opponent exiles from library until total MV >= threshold, controller may cast exiled cards without mana until EOT. Updated: **dream_harvest** (opponents exile until MV 5+, free cast). Added Effect::OpponentsExileUntilMVAndCast variant, builder, game.rs resolution handler. 3 new tests (537 engine total). 1 Effect::Custom eliminated.

- [x] Task 18: Add `DealDamageWithDelayedExile` effect — Deal X damage to target creature, create delayed trigger watching for death. When creature dies, exile cards from library equal to its power, choose one to play until end of next turn. Added `Effect::DealDamageWithDelayedExile`, `Effect::ExileTopChooseOneAndPlay`, `stored_value` field on `DelayedTrigger` for passing last-known info through triggers. Updated **end_blaze_epiphany** (replaced Effect::Custom, fixed rarity to Rare, added TargetSpec::Creature). 3 new tests (540 engine total). 1 Effect::Custom eliminated.

### Tier 4: Hard — requires new engine subsystems

- [x] Task 19: Add `FigureOfFableTransform` / `LevelUp` effect — Added `Effect::SetSubtypesSelf { subtypes }` to replace source's subtypes. Added `"source is a {Type}"` condition to `evaluate_condition_with_targets`. Fixed `SetPowerToughness` and `GainKeyword` to fall back to source when targets empty. Updated **figure_of_fable**: replaced 2 Effect::Custom with `Effect::conditional("source is a Scout/Soldier", ...)` + `Effect::set_subtypes_self()` + `Effect::set_pt()` + `Effect::GainKeyword`. Also fixed first ability to set subtypes. 3 new tests (543 engine total). 2 Effect::Custom eliminated.

- [x] Task 20: Add `WinnowingEffect` — Added `Effect::Winnowing` variant. For each player, the spell's controller chooses a creature that player controls; then each player sacrifices all other creatures they control that don't share a creature type with the chosen creature. Handles Changeling (shares all types). Updated **winnowing**: replaced Effect::Custom with Effect::winnowing(). 3 new tests (546 engine total). 1 Effect::Custom eliminated.

- [x] Task 21: Add `CounterAllOpponentSpellsAndAbilities` effect — Added `Effect::CounterAllOpponentSpellsAndAbilities { token_name }` variant. Counters all opponent spells and abilities on the stack (respecting can't-be-countered), creates tokens equal to the number countered. Updated **glen_elendras_answer**: replaced Effect::Custom, fixed rarity to Mythic. 3 new tests (549 engine total). 1 Effect::Custom eliminated.

- [x] Task 22: Add `MassBecomeCopy` effect — Added `Effect::MassBecomeCopy` variant. Each nonland permanent controller controls becomes a copy of the target permanent. Preserves original object ID, owner, and token status. Re-keys abilities with new IDs. Updated **mirrorform**: replaced Effect::Custom with `Effect::mass_become_copy()`, added `TargetSpec::PermanentFiltered("non-Aura permanent")`, fixed rarity from Common to Rare. 3 new tests (552 engine total). 1 Effect::Custom eliminated.

- [x] Task 23: Add exile-with-dream-counters system — Added `Effect::ExileWithDreamCounterInsteadOfGraveyard` and `Effect::CastFromExileWithDreamCounters` variants. Added `pending_dream_exile` and `dream_countered_cards` to GameState. Modified `resolve_top_of_stack` to check pending dream exiles and route to exile zone instead of graveyard. Attack trigger makes dream-countered exiled cards impulse-playable for free. Updated **goliath_daydreamer**: replaced 2 Effect::Custom with `Effect::exile_with_dream_counter()` and `Effect::cast_from_exile_with_dream_counters()`. 3 new tests (555 engine total). 2 Effect::Custom eliminated.

### Tier 5: Hard — StaticEffect::Custom eliminations

- [x] Task 24: Add `CastFromExileWithCounterCost` static effect — Added `Effect::ExileTargetToSourceZone` to exile cards to a named zone linked to source. Added `StaticEffect::CastFromExileWithCounterCost { counter_count }` to allow casting creature spells from source's exile zone during your turn by removing N counters from creatures you control. Updated `compute_legal_actions` to check for castable exiled creatures and `cast_spell` to handle counter removal cost. Updated **dawnhand_dissident**: replaced `StaticEffect::Custom` with `StaticEffect::cast_from_exile_with_counter_cost(3)`, replaced `Effect::exile()` with `Effect::exile_target_to_source_zone()`, added missing `Cost::Blight` to both activated abilities. 3 new tests (558 engine total). 1 StaticEffect::Custom eliminated.

- [x] Task 25: Add persist/undying keyword mechanics — Implemented persist and undying in `apply_state_based_actions`. Creatures with persist (no -1/-1 counters) return from graveyard to battlefield with a -1/-1 counter. Creatures with undying (no +1/+1 counters) return with a +1/+1 counter. Added `return_from_graveyard_with_counter` helper. Added "nontoken" filter support to `find_matching_permanents`. Updated **eirdu_carrier_of_dawn** (back face): replaced `StaticEffect::Custom` with `StaticEffect::grant_keyword_controlled("other nontoken creatures you control", "persist")`. 3 new tests (561 engine total). 1 StaticEffect::Custom eliminated.

- [x] Task 26: Add `BoostPerTurnEvent` static effect — Added `StaticEffect::BoostPerTurnEvent { filter, event, power_per, toughness_per }` variant. Wired `WatcherManager` into `emit_event` so per-turn stats (creatures_entered etc.) are tracked during gameplay. Fixed watcher to handle both `EntersTheBattlefield` and `EnteredTheBattlefield` event types. Updated **kinbinding**: replaced `StaticEffect::Custom` with `StaticEffect::boost_per_turn_event("creatures you control", "creatures_entered", 1, 1)`. 3 new tests (564 engine total). 1 StaticEffect::Custom eliminated.

- [x] Task 27: Add `CastExiledOncePerTurn` static effect — Added `StaticEffect::CastExiledOncePerTurn { mv_count_filter }` variant. Added `Effect::ExileFromOpponentLibraryToSourceZone` to exile cards into source-specific exile zone. Added `cast_from_exile_once_used: HashSet<ObjectId>` to GameState for once-per-turn tracking. Added "and" multi-type OR counting to `evaluate_count_filter`. Updated **maralen_fae_ascendant**: replaced `StaticEffect::Custom` with `StaticEffect::cast_exiled_once_per_turn("Elves and Faeries you control")`, replaced `Effect::exile_from_opponent_library` with `Effect::exile_from_opponent_library_to_source_zone`. 3 new tests (567 engine total). 1 StaticEffect::Custom eliminated.

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
- Task 27: Added `StaticEffect::CastExiledOncePerTurn` for once-per-turn free casting from source exile zone with MV restriction. Added `Effect::ExileFromOpponentLibraryToSourceZone` and multi-type OR counting in `evaluate_count_filter`. Updated maralen_fae_ascendant card. 3 new tests (567 engine total). 1 StaticEffect::Custom eliminated.

## Notes

- All 44 ECL Custom fallbacks have corresponding Java XMage implementations that can be used as reference
- The plan explicitly says "do not implement any new sets" — focus only on engine features + ECL card updates
- Each task should: read Java source -> add engine feature with tests -> update ECL card(s) -> commit
- The guardrails warn about `rg -c "Effect::Custom"` double-counting StaticEffect::Custom
- Always verify with `cargo check -p mtg-cards` and `cargo test --lib -p mtg-engine`
- Card oracle text should be verified against scryfall.com before changes
- Some tasks may be combined if the engine feature serves multiple cards
- Hardest tasks (19-30) each fix only 1 card — consider if all are worth implementing vs. keeping as Custom
