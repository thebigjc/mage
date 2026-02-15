# Roadmap

This document describes implementation gaps between the Rust mtg-rl engine and the Java XMage reference engine, organized by impact on the 1,333 cards across our 4 sets (FDN, TLA, TDM, ECL).

**Last audit: 2026-02-14** — Compared Rust engine (~14.6K lines, 21 source files) against Java XMage engine (full rules implementation).

## Summary

| Metric | Value |
|--------|-------|
| Cards registered | 1,333 (FDN 512, TLA 280, TDM 273, ECL 268) |
| `Effect::Custom` fallbacks | 747 |
| `StaticEffect::Custom` fallbacks | 160 |
| `Cost::Custom` fallbacks | 33 |
| **Total Custom fallbacks** | **940** |
| Keywords defined | 47 |
| Keywords mechanically enforced | 20 (combat active, plus hexproof, shroud, prowess, landwalk, ward) |
| State-based actions | 8 of ~20 rules implemented |
| Triggered abilities | Events emitted, triggers stacked (ETB, attack, life gain, dies, upkeep, end step, combat damage) |
| Replacement effects | Data structures defined but not integrated |
| Continuous effect layers | Layer 6 (keywords) + Layer 7 (P/T) applied; others pending |

---

## I. Critical Game Loop Gaps

These are structural deficiencies in `game.rs` that affect ALL cards, not just specific ones.

### ~~A. Combat Phase Not Connected~~ (DONE)

**Completed 2026-02-14.** Combat is now fully wired into the game loop:
- `DeclareAttackers`: prompts active player via `select_attackers()`, taps attackers (respects vigilance), registers in `CombatState` (added to `GameState`)
- `DeclareBlockers`: prompts defending player via `select_blockers()`, validates flying/reach restrictions, registers blocks
- `FirstStrikeDamage` / `CombatDamage`: assigns damage via `combat.rs` functions, applies to permanents and players, handles lifelink life gain
- `EndCombat`: clears combat state
- 13 unit tests covering: unblocked damage, blocked damage, vigilance, lifelink, first strike, trample, defender restriction, summoning sickness, haste, flying/reach, multiple attackers

### ~~B. Triggered Abilities Not Stacked~~ (DONE)

**Completed 2026-02-14.** Triggered abilities are now detected and stacked:
- Added `EventLog` to `Game` for tracking game events
- Events emitted at key game actions: ETB, attack declared, life gain, token creation, land play
- `check_triggered_abilities()` scans `AbilityStore` for matching triggers, validates source still on battlefield, handles APNAP ordering
- Supports optional ("may") triggers via `choose_use()`
- Trigger ownership validation: attack triggers only fire for the attacking creature, ETB triggers only for the entering permanent, life gain triggers only for the controller
- `process_sba_and_triggers()` implements MTG rules 117.5 SBA+trigger loop until stable
- 5 unit tests covering ETB triggers, attack triggers, life gain triggers, optional triggers, ownership validation
- **Dies triggers added 2026-02-14:** `check_triggered_abilities()` now handles `EventType::Dies` events. Ability cleanup is deferred until after trigger checking so dies triggers can fire. `apply_state_based_actions()` returns died source IDs for post-trigger cleanup. 4 unit tests (lethal damage, destroy effect, ownership filtering).

### ~~C. Continuous Effect Layers Not Applied~~ (DONE)

**Completed 2026-02-14.** Continuous effects (Layer 6 + Layer 7) are now recalculated on every SBA iteration:
- `apply_continuous_effects()` clears and recalculates `continuous_boost_power`, `continuous_boost_toughness`, and `continuous_keywords` on all permanents
- Handles `StaticEffect::Boost` (Layer 7c — P/T modify) and `StaticEffect::GrantKeyword` (Layer 6 — ability adding)
- `find_matching_permanents()` handles filter patterns: "other X you control" (excludes self), "X you control" (controller check), "self" (source only), "enchanted/equipped creature" (attached target), "token" (token-only), "attacking" (combat state check), plus type/subtype matching
- Handles comma-separated keyword strings (e.g. "deathtouch, lifelink")
- Added `is_token` field to `CardData` for token identification
- Called in `process_sba_and_triggers()` before each SBA check
- 11 unit tests: lord boost, anthem, keyword grant, comma keywords, recalculation, stacking lords, mutual lords, self filter, token filter, opponent isolation, boost+keyword combo

### D. Replacement Effects Not Integrated (PARTIAL)

`ReplacementEffect` and `ReplacementKind` are defined in `effects.rs` with variants like `Prevent`, `ExileInstead`, `ModifyAmount`, `RedirectTarget`, `EnterTapped`, `EnterWithCounters`, `Custom`. The full event interception pipeline is not yet connected, but specific replacement patterns have been implemented:

**Completed 2026-02-14:**
- `EntersTapped { filter: "self" }` — lands/permanents with "enters tapped" now correctly enter tapped via `check_enters_tapped()`. Called at all ETB points: land play, spell resolve, reanimate. 3 unit tests. Affects 7 guildgate/tapland cards across FDN and TDM.

**Still missing:** General replacement effect pipeline (damage prevention, death replacement, Doubling Season, counter modification, enters-with-counters). Affects ~20+ additional cards.

---

## II. Keyword Enforcement

47 keywords are defined as bitflags in `KeywordAbilities`. Most are decorative.

### Mechanically Enforced (10 keywords — in combat.rs, but combat doesn't run)

| Keyword | Where | How |
|---------|-------|-----|
| FLYING | `combat.rs:205` | Flyers can only be blocked by flying/reach |
| REACH | `combat.rs:205` | Can block flyers |
| DEFENDER | `permanent.rs:249` | Cannot attack |
| HASTE | `permanent.rs:250` | Bypasses summoning sickness |
| FIRST_STRIKE | `combat.rs:241-248` | Damage in first-strike step |
| DOUBLE_STRIKE | `combat.rs:241-248` | Damage in both steps |
| TRAMPLE | `combat.rs:296-298` | Excess damage to defending player |
| DEATHTOUCH | `combat.rs:280-286` | 1 damage = lethal |
| MENACE | `combat.rs:216-224` | Must be blocked by 2+ creatures |
| INDESTRUCTIBLE | `state.rs:296` | Survives lethal damage (SBA) |

All 10 are now active in practice via combat integration (2026-02-14). Additionally, vigilance and lifelink are now enforced. Menace is now enforced during declare blockers validation (2026-02-14).

### Not Enforced (35 keywords)

| Keyword | Java Behavior | Rust Status |
|---------|--------------|-------------|
| FLASH | Cast at instant speed | Partially (blocks sorcery-speed only) |
| HEXPROOF | Can't be targeted by opponents | **Enforced** in `legal_targets_for_spec()` |
| SHROUD | Can't be targeted at all | **Enforced** in `legal_targets_for_spec()` |
| PROTECTION | Prevents damage/targeting/blocking/enchanting | Not checked |
| WARD | Counter unless cost paid | **Enforced** in `check_ward_on_targets()` |
| FEAR | Only blocked by black/artifact | **Enforced** in `combat.rs:can_block()` |
| INTIMIDATE | Only blocked by same color/artifact | **Enforced** in `combat.rs:can_block()` |
| SHADOW | Only blocked by/blocks shadow | Not checked |
| PROWESS | +1/+1 when noncreature spell cast | **Enforced** in `check_triggered_abilities()` |
| UNDYING | Return with +1/+1 counter on death | No death replacement |
| PERSIST | Return with -1/-1 counter on death | No death replacement |
| WITHER | Damage as -1/-1 counters | Not checked |
| INFECT | Damage as -1/-1 counters + poison | Not checked |
| TOXIC | Combat damage → poison counters | Not checked |
| UNBLOCKABLE | Can't be blocked | **Enforced** in `combat.rs:can_block()` |
| CHANGELING | All creature types | **Enforced** in `permanent.rs:has_subtype()` + `matches_filter()` |
| CASCADE | Exile-and-cast on cast | No trigger |
| CONVOKE | Tap creatures to pay | Not checked in cost payment |
| DELVE | Exile graveyard to pay | Not checked in cost payment |
| EVOLVE | +1/+1 counter on bigger ETB | No trigger |
| EXALTED | +1/+1 when attacking alone | No trigger |
| EXPLOIT | Sacrifice creature on ETB | No trigger |
| FLANKING | Blockers get -1/-1 | Not checked |
| FORESTWALK | Unblockable vs forest controller | **Enforced** in blocker selection |
| ISLANDWALK | Unblockable vs island controller | **Enforced** in blocker selection |
| MOUNTAINWALK | Unblockable vs mountain controller | **Enforced** in blocker selection |
| PLAINSWALK | Unblockable vs plains controller | **Enforced** in blocker selection |
| SWAMPWALK | Unblockable vs swamp controller | **Enforced** in blocker selection |
| TOTEM_ARMOR | Prevents enchanted creature death | No replacement |
| AFFLICT | Life loss when blocked | No trigger |
| BATTLE_CRY | +1/+0 to other attackers | No trigger |
| SKULK | Can't be blocked by greater power | **Enforced** in `combat.rs:can_block()` |
| FABRICATE | Counters or tokens on ETB | No choice/trigger |
| STORM | Copy for each prior spell | No trigger |
| PARTNER | Commander pairing | Not relevant |

---

## III. State-Based Actions

Checked in `state.rs:check_state_based_actions()`:

| Rule | Description | Status |
|------|-------------|--------|
| 704.5a | Player at 0 or less life loses | **Implemented** |
| 704.5b | Player draws from empty library loses | **Implemented** (in draw_cards()) |
| 704.5c | 10+ poison counters = loss | **Implemented** |
| 704.5d | Token not on battlefield ceases to exist | **Implemented** |
| 704.5e | 0-cost copy on stack/BF ceases to exist | **Not implemented** |
| 704.5f | Creature with 0 toughness → graveyard | **Implemented** |
| 704.5g | Lethal damage → destroy (if not indestructible) | **Implemented** |
| 704.5i | Planeswalker with 0 loyalty → graveyard | **Implemented** |
| 704.5j | Legend rule (same name) | **Implemented** |
| 704.5n | Aura not attached → graveyard | **Implemented** |
| 704.5p | Equipment/Fortification illegal attach → unattach | **Implemented** |
| 704.5r | +1/+1 and -1/-1 counter annihilation | **Implemented** |
| 704.5s | Saga with lore counters ≥ chapters → sacrifice | **Not implemented** |

**Missing SBAs:** Saga sacrifice. These affect ~40+ cards.

---

## IV. Missing Engine Systems

These require new engine architecture beyond adding match arms to existing functions.

### Tier 1: Foundational (affect 100+ cards each)

#### 1. Combat Integration
- Wire `combat.rs` functions into `turn_based_actions()` for DeclareAttackers, DeclareBlockers, CombatDamage
- Add `choose_attackers()` / `choose_blockers()` to `PlayerDecisionMaker`
- Connect lifelink (damage → life gain), vigilance (no tap), and other combat keywords
- **Cards affected:** Every creature in all 4 sets (~800+ cards)
- **Java reference:** `GameImpl.combat`, `Combat.java`, `CombatGroup.java`

#### 2. Triggered Ability Stacking
- After each game action, scan for triggered abilities whose conditions match recent events
- Push triggers onto stack in APNAP order
- Resolve via existing priority loop
- **Cards affected:** Every card with ETB, attack, death, damage, upkeep, or endstep triggers (~400+ cards)
- **Java reference:** `GameImpl.checkStateAndTriggered()`, 84+ `Watcher` classes

#### 3. Continuous Effect Layer Application
- Recalculate permanent characteristics after each game action
- Apply StaticEffect variants (Boost, GrantKeyword, CantAttack, CantBlock, etc.) in layer order
- Duration tracking (while-on-battlefield, until-end-of-turn, etc.)
- **Cards affected:** All lord/anthem effects, keyword-granting permanents (~50+ cards)
- **Java reference:** `ContinuousEffects.java`, 7-layer system

### Tier 2: Key Mechanics (affect 10-30 cards each)

#### ~~4. Equipment System~~ (DONE)

**Completed 2026-02-14.** Equipment is now fully functional:
- `Effect::Equip` variant handles attaching equipment to target creature
- Detach from previous creature when re-equipping
- Continuous effects ("equipped creature" filter) already handled by `find_matching_permanents()`
- SBA 704.5p: Equipment auto-detaches when attached creature leaves battlefield
- 12 card factories updated from `Effect::Custom` to `Effect::equip()`
- 5 unit tests: attach, stat boost, detach on death, re-equip, keyword grant

#### ~~5. Aura/Enchant System~~ (DONE)

**Completed 2026-02-14.** Aura enchantments are now functional:
- Auras auto-attach to their target on spell resolution (ETB)
- Continuous effects ("enchanted creature" filter) handle P/T boosts and keyword grants
- `CantAttack`/`CantBlock` static effects now enforced via continuous effects layer
  (added `cant_attack` and `cant_block_from_effect` flags to Permanent)
- SBA 704.5n: Auras go to graveyard when enchanted permanent leaves
- SBA 704.5p: Equipment just detaches (stays on battlefield)
- 3 unit tests: boost, fall-off, Pacifism can't-attack

#### 6. Replacement Effect Pipeline
- Before each event, check registered replacement effects
- `applies()` filter + `replaceEvent()` modification
- Support common patterns: exile-instead-of-die, enters-tapped, enters-with-counters, damage prevention
- Prevent infinite loops (each replacement applies once per event)
- **Blocked features:** Damage prevention, death replacement, Doubling Season, Undying, Persist (~30+ cards)
- **Java reference:** `ReplacementEffectImpl.java`, `ContinuousEffects.getReplacementEffects()`

#### ~~7. X-Cost Spells~~ (DONE)

**Completed 2026-02-14.** X-cost spells are now functional:
- `ManaCost::has_x_cost()`, `x_count()`, `to_mana_with_x(x)` for X detection and mana calculation
- `X_VALUE` sentinel constant (u32::MAX) used in effect amounts to indicate "use X"
- `StackItem.x_value: Option<u32>` tracks chosen X on the stack
- `cast_spell()` detects X costs, calls `choose_amount()` for X value, pays `to_mana_with_x(x)`
- `execute_effects()` receives x_value and uses `resolve_x()` closure to substitute X_VALUE with actual X
- All numeric effect handlers updated: DealDamage, DrawCards, GainLife, LoseLife, LoseLifeOpponents, DealDamageOpponents, DealDamageAll, Mill, AddCounters, AddCountersSelf, DiscardOpponents, CreateToken
- 4 unit tests: X damage, X draw, X=0, mana payment verification

#### ~~8. Impulse Draw (Exile-and-Play)~~ (DONE)

**Completed 2026-02-14.** Impulse draw is now functional:
- `ImpulsePlayable` struct tracks exiled cards with player, duration, and without-mana flag
- `ImpulseDuration::EndOfTurn` and `UntilEndOfNextTurn` with proper per-player turn tracking
- `Effect::ExileTopAndPlay { count, duration, without_mana }` exiles from library and registers playability
- `compute_legal_actions()` includes impulse-playable cards as castable/playable
- `cast_spell()` and `play_land()` handle cards from exile (removing from exile zone and impulse list)
- Cleanup step expires `EndOfTurn` entries immediately and `UntilEndOfNextTurn` at controller's next turn
- Convenience builders: `exile_top_and_play(n)`, `exile_top_and_play_next_turn(n)`, `exile_top_and_play_free(n)`
- 6 unit tests: creation, legal actions, resolve, expiration, next-turn persistence, free cast

#### ~~9. Graveyard Casting (Flashback/Escape)~~ (DONE)

**Completed 2026-02-14.** Flashback casting is now implemented:
- `flashback_cost: Option<ManaCost>` field on `CardData` for alternative graveyard cast cost
- `compute_legal_actions()` checks graveyard for cards with flashback_cost, validates mana
- `cast_spell()` detects graveyard-origin spells, uses flashback cost, sets `exile_on_resolve` flag on StackItem
- `resolve_top_of_stack()` exiles flashback spells instead of sending to graveyard
- SpellCast event correctly reports Zone::Graveyard as source zone
- 4 unit tests: legal actions, mana validation, exile-after-resolution, normal-cast-graveyard

#### 10. Planeswalker System
- Loyalty counters as activation resource
- Loyalty abilities: `PayLoyaltyCost(amount)` — add or remove loyalty counters
- One loyalty ability per turn, sorcery speed
- Can be attacked (defender selection during declare attackers)
- Damage redirected from player to planeswalker (or direct attack)
- SBA: 0 loyalty → graveyard (already implemented)
- **Blocked cards:** Ajani, Chandra, Kaito, Liliana, Vivien, all planeswalkers (~10+ cards)
- **Java reference:** `LoyaltyAbility.java`, `PayLoyaltyCost.java`

### Tier 3: Advanced Systems (affect 5-10 cards each)

#### 11. Spell/Permanent Copy (PARTIAL)

**Token copy completed 2026-02-15.** `Effect::CreateTokenCopy { count, modifications }` creates token copies of target permanents:
- Clones CardData (name, types, P/T, abilities, keywords), re-keys ability IDs
- `TokenModification` enum: `AddKeyword`, `AddChangeling`, `SacrificeAtEndStep`, `EnterTappedAttacking`
- Convenience builders: `create_token_copy()`, `create_token_copy_with_haste()`, `create_token_copy_with_changeling()`, `create_token_copy_haste_sacrifice()`
- 4 unit tests; 5 ECL cards updated
- **Still missing:** Copy spell on stack (Fork, Reverberate), enter-as-copy (clone creatures)
- **Remaining blocked cards:** Mirrorform (mass copy), enter-as-copy-with-changeling, graveyard-ETB copy trigger (~3 cards)

#### ~~12. Delayed Triggers~~ (DONE)

**Completed 2026-02-14.** Delayed triggered abilities are now functional:
- `DelayedTrigger` struct in `GameState` tracks: event type, watched object, effects, controller, duration, trigger-only-once
- `DelayedDuration::EndOfTurn` (removed at cleanup) and `UntilTriggered` (persists until fired)
- `Effect::CreateDelayedTrigger` registers a delayed trigger during effect resolution
- `check_triggered_abilities()` checks delayed triggers against events, fires matching ones
- Watched object filtering: only fires when the specific watched permanent/creature matches the event
- `EventType::from_name()` parses string event types for flexible card authoring
- Convenience builders: `delayed_on_death(effects)`, `at_next_end_step(effects)`
- 4 unit tests: death trigger fires, wrong creature doesn't fire, expiration, end step trigger

#### 13. Saga Enchantments
- Lore counters added on ETB and after draw step
- Chapter abilities trigger when lore counter matches chapter number
- Sacrifice after final chapter (SBA)
- **Blocked cards:** 6+ Saga cards in TDM/TLA
- **Java reference:** `SagaAbility.java`

#### 14. Additional Combat Phases
- "Untap all creatures, there is an additional combat phase"
- Insert extra combat steps into the turn sequence
- **Blocked cards:** Aurelia the Warleader, All-Out Assault (~3 cards)

#### 15. Conditional Cost Modifications
- `CostReduction` stored but not applied during cost calculation
- "Second spell costs {1} less", Affinity, Convoke, Delve
- Need cost-modification pass before mana payment
- **Blocked cards:** Highspire Bell-Ringer, Allies at Last, Ghalta Primal Hunger (~5+ cards)
- **Java reference:** `CostModificationEffect.java`, `SpellAbility.adjustCosts()`

### Tier 4: Set-Specific Mechanics

#### 16. Earthbend (TLA)
- "Look at top N, put a land to hand, rest on bottom"
- Similar to Explore/Impulse — top-of-library selection
- **Blocked cards:** Badgermole, Badgermole Cub, Ostrich-Horse, Dai Li Agents, many TLA cards (~20+ cards)

#### ~~17. Behold (ECL)~~ (DONE)

**Completed 2026-02-14.** Behold mechanic implemented with 3 cost variants:
- `Cost::Behold(type)` — Mandatory: choose matching creature on battlefield or reveal from hand
- `Cost::BeholdAndExile(type)` — Mandatory behold + exile the chosen card/permanent
- `Cost::BeholdOrPay { creature_type, mana }` — Behold or pay alternative mana cost
- `can_pay_additional_costs()` validates behold feasibility in legal action computation
- `additional_costs: Vec<Cost>` field on CardData for spell additional costs
- Changeling creatures match any behold type requirement
- 9 unit tests; 10 ECL cards updated to use typed behold costs

#### 18. ~~Vivid (ECL)~~ (DONE)
Color-count calculation implemented. 6 Vivid effect variants added. 6 cards fixed.

#### 19. Renew (TDM)
- Counter-based death replacement (exile with counters, return later)
- Requires replacement effect pipeline (Tier 2, item 6)
- **Blocked cards:** ~5+ TDM cards

#### 20. Endure (TDM)
- Put +1/+1 counters; if would die, exile with counters instead
- Requires replacement effect pipeline
- **Blocked cards:** ~3+ TDM cards

---

## V. Effect System Gaps

### Implemented Effect Variants (~55 of 62)

The following Effect variants have working `execute_effects()` match arms:

**Damage:** DealDamage, DealDamageAll, DealDamageOpponents, DealDamageVivid
**Life:** GainLife, GainLifeVivid, LoseLife, LoseLifeOpponents, LoseLifeOpponentsVivid, SetLife
**Removal:** Destroy, DestroyAll, Exile, Sacrifice, PutOnLibrary
**Card Movement:** Bounce, ReturnFromGraveyard, Reanimate, DrawCards, DrawCardsVivid, DiscardCards, DiscardOpponents, Mill, SearchLibrary, LookTopAndPick
**Counters:** AddCounters, AddCountersSelf, AddCountersAll, RemoveCounters
**Tokens:** CreateToken, CreateTokenTappedAttacking, CreateTokenVivid
**Combat:** CantBlock, Fight, Bite, MustBlock
**Stats:** BoostUntilEndOfTurn, BoostPermanent, BoostAllUntilEndOfTurn, BoostUntilEotVivid, BoostAllUntilEotVivid, SetPowerToughness
**Keywords:** GainKeywordUntilEndOfTurn, GainKeyword, LoseKeyword, GrantKeywordAllUntilEndOfTurn, Indestructible, Hexproof
**Control:** GainControl, GainControlUntilEndOfTurn
**Utility:** TapTarget, UntapTarget, CounterSpell, Scry, AddMana, Modal, DoIfCostPaid, ChooseCreatureType, ChooseTypeAndDrawPerPermanent

### Unimplemented Effect Variants

| Variant | Description | Cards Blocked |
|---------|-------------|---------------|
| `GainProtection` | Target gains protection from quality | ~5 |
| `PreventCombatDamage` | Fog / damage prevention | ~5 |
| `Custom(String)` | Catch-all for untyped effects | 747 instances |

### Custom Effect Fallback Analysis (747 Effect::Custom)

These are effects where no typed variant exists. Grouped by what engine feature would replace them:

| Category | Count | Sets | Engine Feature Needed |
|----------|-------|------|----------------------|
| Generic ETB stubs ("ETB effect.") | 79 | All | Triggered ability stacking |
| Generic activated ability stubs ("Activated effect.") | 67 | All | Proper cost+effect binding on abilities |
| Attack/combat triggers | 45 | All | Combat integration + triggered abilities |
| Cast/spell triggers | 47 | All | Triggered abilities + cost modification |
| Aura/equipment attachment | 28 | FDN,TDM,ECL | Equipment/Aura system |
| Exile-and-play effects | 25 | All | Impulse draw |
| Generic spell stubs ("Spell effect.") | 21 | All | Per-card typing with existing variants |
| Dies/sacrifice triggers | 18 | FDN,TLA | Triggered abilities |
| Conditional complex effects | 30+ | All | Per-card analysis; many are unique |
| Tapped/untap mechanics | 10 | FDN,TLA | Minor — mostly per-card fixes |
| Saga mechanics | 6 | TDM,TLA | Saga system |
| Earthbend keyword | 5 | TLA | Earthbend mechanic |
| Copy/clone effects | 8+ | TDM,ECL | Spell/permanent copy |
| Cost modifiers | 4 | FDN,ECL | Cost modification system |
| X-cost effects | 5+ | All | X-cost system |

### StaticEffect::Custom Analysis (160 instances)

| Category | Count | Engine Feature Needed |
|----------|-------|-----------------------|
| Generic placeholder ("Static effect.") | 90 | Per-card analysis; diverse |
| Conditional continuous ("Conditional continuous effect.") | 4 | Layer system + conditions |
| Dynamic P/T ("P/T = X", "+X/+X where X = ...") | 11 | Characteristic-defining abilities (Layer 7a) |
| Evasion/block restrictions | 5 | Restriction effects in combat |
| Protection effects | 4 | Protection keyword enforcement |
| Counter/spell protection ("Can't be countered") | 8 | Uncounterable flag or replacement effect |
| Keyword grants (conditional) | 4 | Layer 6 + conditions |
| Damage modification | 4 | Replacement effects |
| Transform/copy | 3 | Copy layer + transform |
| Mana/land effects | 3 | Mana ability modification |
| Cost reduction | 2 | Cost modification system |
| Keyword abilities (Kicker, Convoke, Delve) | 4 | Alternative/additional costs |
| Token doubling | 1 | Replacement effect |
| Trigger multiplier | 1 | Triggered ability system |
| Other unique effects | 16 | Per-card analysis |

### Cost::Custom Analysis (33 instances)

| Category | Count | Engine Feature Needed |
|----------|-------|-----------------------|
| Complex mana+tap activated abilities | 13 | Better activated ability cost parsing |
| Sacrifice-based activated abilities | 9 | Sacrifice-other as part of compound costs |
| Tap-creature costs (convoke-like) | 5 | Tap-other-creature cost variant |
| Exile costs (self, enchantment, graveyard) | 3 | Exile-self / exile-zone cost variants |
| Complex multi-part costs | 2 | Compound cost support |
| Discard hand | 1 | Discard-hand cost variant |

---

## VI. Per-Set Custom Fallback Counts

| Set | Effect::Custom | StaticEffect::Custom | Cost::Custom | Total |
|-----|---------------|---------------------|-------------|-------|
| FDN (Foundations) | 322 | 58 | 21 | 401 |
| TLA (Avatar: TLA) | 197 | 54 | 2 | 253 |
| TDM (Tarkir: Dragonstorm) | 111 | 16 | 3 | 130 |
| ECL (Lorwyn Eclipsed) | 77 | 20 | 0 | 97 |
| **Total** | **747** | **160** | **33** | **940** |

Detailed per-card breakdowns in `docs/{fdn,tla,tdm,ecl}-remediation.md`.

---

## VII. Comparison with Java XMage

Features the Java engine has that the Rust engine lacks entirely:

| Java Feature | Java Location | Rust Status |
|-------------|--------------|-------------|
| **84+ Watcher classes** | `mage.watchers.common/` | Basic `WatcherManager` only |
| **Replacement effect pipeline** | `ContinuousEffects.getReplacementEffects()` | Structs defined, not integrated |
| **7-layer continuous effect application** | `ContinuousEffects.apply()` | Layers defined, never applied |
| **RequirementEffect** (must attack/block) | `mage.abilities.effects.RequirementEffect` | **Partial** (`MustBeBlocked` static effect, flag on Permanent) |
| **RestrictionEffect** (can't attack/block) | `mage.abilities.effects.RestrictionEffect` | **Partial** (CantAttack/CantBlock, CantBeBlockedByMoreThan, CantBeBlockedByPowerLessOrEqual) |
| **AsThoughEffect** (play from other zones) | `mage.abilities.effects.AsThoughEffect` | **Partial** (`ImpulsePlayable` for exile-and-play) |
| **CostModificationEffect** | `mage.abilities.effects.CostModificationEffect` | CostReduction stored but not applied |
| **PreventionEffect** (damage prevention) | `mage.abilities.effects.PreventionEffect` | No equivalent |
| **Equipment attachment** | `EquipAbility`, `AttachEffect` | No equivalent |
| **Aura attachment** | `AuraReplacementEffect` | No equivalent |
| **Planeswalker loyalty abilities** | `LoyaltyAbility`, `PayLoyaltyCost` | No equivalent |
| **X-cost system** | `VariableManaCost`, `ManaCostsImpl.getX()` | **Implemented** (`X_VALUE`, `StackItem.x_value`, `resolve_x()`) |
| **Spell copying** | `CopyEffect`, `CopySpellForEachItCouldTargetEffect` | No equivalent |
| **Delayed triggered abilities** | `DelayedTriggeredAbility` | **Implemented** (`DelayedTrigger`, `CreateDelayedTrigger`) |
| **Alternative costs** (Flashback, Evoke, etc.) | `AlternativeCostSourceAbility` | Evoke stored as StaticEffect, not enforced |
| **Additional costs** (Kicker, Buyback, etc.) | `OptionalAdditionalCostImpl` | No equivalent |
| **Combat damage assignment order** | `CombatGroup.pickBlockerOrder()` | Simplified (first blocker takes all) |
| **Spell target legality check on resolution** | `Spell.checkTargets()` | Targets checked at cast, not re-validated |
| **Mana restriction** ("spend only on creatures") | `ManaPool.conditionalMana` | Not tracked |
| **Hybrid mana** ({B/R}, {2/G}) | `HybridManaCost` | Not modeled |
| **Zone change tracking** (LKI) | `getLKIBattlefield()` | No last-known-information |

---

## VIII. Phased Implementation Plan

Priority ordered by cards-unblocked per effort.

### Phase 1: Make the Engine Functional (combat + triggers)

1. ~~**Combat integration**~~ — **DONE (2026-02-14).** Wired `combat.rs` into `turn_based_actions()` for DeclareAttackers, DeclareBlockers, FirstStrikeDamage, CombatDamage, EndCombat. Connected lifelink, vigilance, flying/reach. Added `CombatState` to `GameState`. 13 unit tests.

2. ~~**Triggered ability stacking**~~ — **DONE (2026-02-14).** Events emitted at game actions, triggered abilities detected and stacked in APNAP order. ETB, attack, and life gain triggers functional. 5 unit tests.

3. ~~**Continuous effect layer application**~~ — **DONE (2026-02-14).** `apply_continuous_effects()` recalculates P/T boosts and keyword grants from static abilities. `find_matching_permanents()` handles "other", "you control", "self", "enchanted/equipped creature", "token", "attacking" filter patterns. Added `is_token` to `CardData`. 11 unit tests.

### Phase 2: Core Missing Mechanics

4. **Replacement effect pipeline** — Event interception. Enters-tapped enforcement done (2026-02-14). Still needed: damage prevention, death replacement, Undying/Persist, enters-with-counters. **~20+ remaining cards.**

5. ~~**Equipment system**~~ — **DONE (2026-02-14).** `Effect::Equip`, detachment SBA, card updates.

6. ~~**Aura/enchant system**~~ — **DONE (2026-02-14).** Auto-attach, fall-off SBA, CantAttack/CantBlock.

7. ~~**X-cost spells**~~ — **DONE (2026-02-14).** `X_VALUE` sentinel, `StackItem.x_value`, `resolve_x()` closure in execute_effects. 4 unit tests.

8. ~~**Impulse draw**~~ — **DONE (2026-02-14).** `ImpulsePlayable` tracking, `ExileTopAndPlay` effect, cast/play from exile, duration expiration. 6 unit tests.

### Phase 3: Advanced Systems

9. **Planeswalker system** — Loyalty abilities, can-be-attacked, damage redirection. **~10+ cards.**

10. **Spell/permanent copy** — **PARTIAL (2026-02-15).** Token copy done (CreateTokenCopy + TokenModification). Still needs spell copy on stack and enter-as-copy. **~3 remaining cards.**

11. ~~**Delayed triggers**~~ — **DONE (2026-02-14).** `DelayedTrigger` struct, `CreateDelayedTrigger` effect, event-driven firing, duration expiration. 4 unit tests.

12. ~~**Graveyard casting**~~ — **DONE (2026-02-14).** Flashback casting from graveyard, exile after resolution.

13. **Saga enchantments** — Lore counters, chapter abilities. **~6+ cards.**

14. **Cost modification** — Apply CostReduction during cost calculation. **~5+ cards.**

15. **Additional combat phases** — Extra attack steps. **~3 cards.**

### Phase 4: Set-Specific Mechanics

16. **Earthbend** (TLA) — Top-N selection, land to hand. **~20+ cards.**

17. ~~**Behold**~~ (ECL) — **DONE (2026-02-14).** 3 behold cost variants, additional_costs field, 10 cards updated.

18. **Renew/Endure** (TDM) — Counter-based death replacement (needs replacement pipeline). **~8+ cards.**

19. ~~**Vivid** (ECL)~~ — **DONE.** Color-count calculation + 6 effect variants.

20. ~~**Modal spells**~~ — **DONE.** `Effect::Modal` + `choose_mode()`.

21. ~~**Fight/Bite**~~ — **DONE.** `Effect::Fight` + `Effect::Bite` with dual targeting.

22. ~~**Token stat parsing**~~ — **DONE.** `CreateToken` parses P/T and keywords from name string.

### Phase 5: Eliminate Custom Fallbacks

After the above systems are in place, systematically replace remaining `Custom(String)` fallbacks with typed variants:

- **Easy wins (~100 cards):** Generic stubs ("ETB effect", "Activated effect", "Spell effect") where the effect is a simple combination of existing typed variants
- **Medium (~200 cards):** Cards needing one of the systems above (triggers, layers, combat) plus per-card typing
- **Hard (~50+ cards):** Cards with truly unique mechanics needing new Effect variants

---

## IX. Previously Completed Work

**Batch 1-10 remediation** (2026-02-13 to 2026-02-14): Fixed ~60 cards by replacing Custom effects with typed variants. Added engine features: source-fallback for counters, Ward variant, EntersTappedUnless variant, mass-buff effects (BoostAllUntilEndOfTurn, GrantKeywordAllUntilEndOfTurn), AddCountersAll, AddCountersSelf, 7 cost implementations (RemoveCounters, Blight, ExileFromGraveyard, ExileFromHand, SacrificeOther, UntapSelf, Custom), Vivid mechanic (6 effect variants + color counting), Modal spells (Effect::Modal + ModalMode), Fight/Bite mechanics.

**Session 2026-02-15:** Added BoostPerCount (dynamic P/T from counting permanents/graveyard), Flicker/FlickerEndStep/ReturnFromExileTapped, AdditionalLandPlays, OpponentExilesFromHand, ConditionalKeyword/ConditionalBoostSelf (evaluate_condition), BlightOpponents, GainAllCreatureTypes, CreateTokenCopy with TokenModification. Updated ~12 ECL cards. ECL now has 77 Effect::Custom and 20 StaticEffect::Custom.

See `docs/work-queue.md` for the batch-fix loop and per-set remediation docs for card-level details.
