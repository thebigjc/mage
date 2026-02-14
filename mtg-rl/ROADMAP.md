# Roadmap

This document describes implementation gaps in the mtg-rl engine and cards, organized by priority. Each engine feature lists how many cards it would unblock when implemented.

## Engine Gaps

### Effect Variants (game.rs `execute_effects()`)

These `Effect` enum variants exist in `abilities.rs` but have no implementation in `execute_effects()` -- they fall through to `_ => {}` and silently do nothing at runtime.

**Recently implemented:** The following Effect variants were previously no-ops but now have working implementations in `execute_effects()`: `Scry`, `SearchLibrary`, `ReturnFromGraveyard`, `Reanimate`, `GainKeywordUntilEndOfTurn`, `GainKeyword`, `LoseKeyword`, `Indestructible`, `Hexproof`, `CantBlock`, `Sacrifice`, `DestroyAll`, `DealDamageAll`, `RemoveCounters`, `CreateTokenTappedAttacking`, `BoostPermanent`, `SetPowerToughness`, `LoseLifeOpponents`. Token stat parsing is also implemented (`CreateToken` now parses P/T and keywords from `token_name`).

**Batch 2 engine change (2026-02-13):** Modified `execute_effects` to accept an optional `source: Option<ObjectId>` parameter. When `AddCounters` or `RemoveCounters` effects have no selected targets, they now fall back to the source permanent. This enables self-targeting counter effects (e.g., blight creatures putting -1/-1 counters on themselves) without requiring explicit target selection. Test: `add_counters_self_when_no_targets`.

**Batch 5 (2026-02-13):** Added `StaticEffect::Ward { cost: String }` variant + `ward()` builder. Ward is a triggered ability that counters targeting spells/abilities unless the opponent pays the cost. The engine stores it as structured data (not yet mechanically enforced during targeting). Replaced `StaticEffect::Custom("Ward ...")` on 7 cards across FDN/TDM/ECL and added `KeywordAbilities::WARD` flags.

**Batch 6 (2026-02-13):** Added `StaticEffect::EntersTappedUnless { condition: String }` variant + `enters_tapped_unless()` builder. Stored as structured data; mechanical enforcement deferred. Replaced `StaticEffect::Custom("Enters tapped unless ...")` on 2 TDM lands (Cori Mountain Monastery, Dalkovan Encampment).

**Batch 7 (2026-02-13):** Added `Effect::BoostAllUntilEndOfTurn { filter, power, toughness }` and `Effect::GrantKeywordAllUntilEndOfTurn { filter, keyword }` variants with constructors and match arms. These mass-buff effects apply to all creatures matching the filter controlled by the effect's controller. Replaced Custom effects on 8 cards (6 FDN, 2 ECL) with 12 effect swaps total.

**Batch 9 (2026-02-13):** Added `Effect::AddCountersAll { counter_type, count, filter }` variant + `add_counters_all()` constructor + match arm. Puts N counters of specified type on all creatures matching filter, with "you control" support. Fixed ECL Darkness Descends and partially fixed TDM Barrensteppe Siege (Abzan mode only).

**Batch 10 (2026-02-14):** Added `Effect::AddCountersSelf { counter_type, count }` variant + `add_counters_self()` constructor + match arm. Unlike `AddCounters` (source fallback only when targets empty), `AddCountersSelf` always applies to the source permanent regardless of other targets. Enables compound effects like "blight self + grant haste to target creature" (Warren Torchmaster).

| Effect Variant | Description | Cards Blocked |
|---------------|-------------|---------------|
| `GainControl` | Gain control of target permanent | ~5 |
| `GainControlUntilEndOfTurn` | Threaten/Act of Treason effects | ~5 |
| `GainProtection` | Target gains protection from quality | ~5 |
| `PreventCombatDamage` | Fog / damage prevention | ~5 |
| `MustBlock` | Target creature must block | ~3 |
| `SetLife` | Set a player's life total | ~2 |
| `Custom(String)` | Catch-all for untyped effects | ~400+ |

Note: `Custom(String)` is used when no typed variant exists. Each Custom needs to be individually replaced with a typed variant or a new variant added.

### Static Effect Enforcement

`StaticEffect` variants (`Boost`, `GrantKeyword`, `CantBlock`, `CantAttack`, `CostReduction`, `Ward`, `EntersTappedUnless`, `EntersTapped`, `CantGainLife`, `CantDrawExtraCards`) are currently **structured annotations only** — `game.rs` never reads or applies them. In Java XMage, these are active participants in the game loop via the continuous effects layer system (7 layers: Copy → Control → Text → Type → Color → Ability → P/T).

To make them functional, we need:
1. **Continuous effect application loop** in `game.rs` that iterates battlefield permanents' `static_effects` each time state-based actions are checked
2. **ETB replacement hooks** for `EntersTappedUnless` (ask player to pay, conditionally tap)
3. **Combat restriction enforcement** for `CantAttack` / `CantBlock` during declare attackers/blockers
4. **P/T modification layer** for `Boost` (lord effects) applied as a layer on top of base stats
5. **Keyword granting** for `GrantKeyword` applied during ability checks

This is a significant but high-ROI engine change — it would make ~50+ lord effects, combat restrictions, and ETB conditions functional across all sets.

### Missing Engine Systems

These are features that require new engine architecture, not just new match arms:

#### Equipment System
- No attach/detach mechanics
- Equipment stat bonuses not applied
- Equip cost not evaluated
- **Blocked cards:** Basilisk Collar, Swiftfoot Boots, Goldvein Pick, Fishing Pole, all Equipment (~15+ cards)

#### Planeswalker System
- Loyalty counters not tracked as a resource
- Planeswalker abilities not resolved (loyalty cost/gain)
- Planeswalker damage redirection not enforced
- **Blocked cards:** Ajani, Chandra, Kaito, Liliana, Vivien, all planeswalkers (~10+ cards)

#### Modal Spells
- No mode selection framework
- `choose_mode()` decision interface exists but is unused
- **Blocked cards:** Abrade, Boros Charm, Slagstorm, Valorous Stance, Coordinated Maneuver, Frontline Rush, Sarkhan's Resolve, Seize Opportunity (~20+ cards)

#### X-Cost Spells
- No variable cost determination
- X is not tracked or passed to effects
- **Blocked cards:** Day of Black Sun, Spectral Denial (partially works), Genesis Wave, Finale of Revelation (~10+ cards)

#### ~~Fight/Bite Mechanic~~ (DONE)
`Effect::Fight` (mutual damage) and `Effect::Bite` (one-way damage) implemented with proper two-target selection via `TargetSpec::Pair { CreatureYouControl, OpponentCreature }`. Matches Java's `FightTargetsEffect` / `DamageWithPowerFromOneToAnotherTargetEffect` pattern. Compound effects (e.g. +1/+1 counter then bite) correctly apply pre-fight/bite effects only to `targets[0]` (your creature). Fixed 6 cards (Bite Down, Affectionate Indrik, Felling Blow, Knockout Maneuver, Piercing Exhale, Assert Perfection). ETB triggers (Affectionate Indrik) use `TargetSpec::OpponentCreature` with source as fighter. Remaining fight/bite cards blocked by modal spells (Batch 11), X-cost (Batch 15), or other missing systems.

#### ~~Token Stat Parsing~~ (DONE)
`CreateToken` now parses P/T and keywords from `token_name` strings (e.g., '4/4 Dragon with flying' creates a 4/4 with flying). Cards using correctly-formatted token names now work.

#### Aura/Enchant System
- Auras exist as permanents but don't attach to creatures
- Static P/T boosts from Auras not applied
- Keyword grants from Auras not applied
- **Blocked cards:** Pacifism (partially works via CantAttack/CantBlock), Obsessive Pursuit, Eaten by Piranhas, Angelic Destiny (~10+ cards)

#### Impulse Draw (Exile-and-Play)
- "Exile top card, you may play it until end of [next] turn" has no implementation
- **Blocked cards:** Equilibrium Adept, Kulrath Zealot, Sizzling Changeling, Burning Curiosity, Etali (~10+ cards)

#### Spell Copy
- No mechanism to copy spells on the stack
- **Blocked cards:** Electroduplicate, Rite of Replication, Self-Reflection, Flamehold Grappler, Sage of the Skies (~8+ cards)

#### Replacement Effects
- `ReplacementEffect` struct exists in `effects.rs` but is not integrated into the event pipeline
- Events are not interceptable before they resolve
- **Blocked features:** "If a creature would die, exile it instead", "If you would gain life, gain that much +1", Doubling Season, damage prevention, Dryad Militant graveyard replacement

#### Additional Combat Phases
- No support for extra combat steps
- **Blocked cards:** Aurelia the Warleader, All-Out Assault (~3 cards)

#### "Behold" Mechanic (ECL-specific)
- Reveal-and-exile-from-hand mechanic not implemented
- Many ECL cards reference it as an alternative cost or condition
- **Blocked cards:** Champion of the Weird, Champions of the Perfect, Molten Exhale, Osseous Exhale (~15+ cards)

#### "Earthbend" Mechanic (TLA-specific)
- "Look at top N, put a land to hand, rest on bottom" not implemented
- **Blocked cards:** Badgermole, Badgermole Cub, Ostrich-Horse, Dai Li Agents, Earth Kingdom General, Earth Village Ruffians, many TLA cards (~20+ cards)

#### "Vivid" Mechanic (ECL-specific)
- "X = number of colors among permanents you control" calculation not implemented
- **Blocked cards:** Explosive Prodigy, Glister Bairn, Luminollusk, Prismabasher, Shimmercreep, Shinestriker, Squawkroaster (~10+ cards)

#### Delayed Triggers
- "When this creature dies this turn, draw a card" style effects
- No framework for registering one-shot triggered abilities
- **Blocked cards:** Undying Malice, Fake Your Own Death, Desperate Measures, Scarblades Malice (~5+ cards)

#### Conditional Cost Modifications
- `CostReduction` static effect exists but may not apply correctly
- No support for "second spell costs {1} less" or Affinity
- **Blocked cards:** Highspire Bell-Ringer, Allies at Last, Ghalta Primal Hunger (~5+ cards)

---

## Phased Implementation Plan

### Phase 1: High-Impact Engine Effects

These unblock the most cards per effort invested.

1. **Token stat parsing** -- **DONE** -- Parse power/toughness/keywords from `token_name` string in `CreateToken`. Unblocks ~30 cards that already create tokens but with wrong stats.

2. **Fix easy card-level bugs** -- Many cards use `Effect::Custom(...)` when a typed variant already exists. Examples:
   - ~~Phyrexian Arena: `Custom("You lose 1 life.")` -> `LoseLife { amount: 1 }`~~ **DONE**
   - ~~Pulse Tracker/Marauding Blight-Priest/Vampire Spawn/Vampire Neonate: `Custom("Each opponent loses N life")` -> `LoseLifeOpponents { amount: N }`~~ **DONE**
   - ~~Skirmish Rhino (TDM), Champion of the Weird (ECL), Boggart Mischief (ECL): opponent life loss Custom -> `LoseLifeOpponents`~~ **DONE**
   - ~~Diregraf Ghoul: `StaticEffect::Custom("Enters tapped.")` -> `StaticEffect::EntersTapped`~~ **DONE**
   - Incomplete dual lands: copy the Azorius Guildgate / Bloodfell Caves pattern
   - ~~Sourbread Auntie/Sting-Slinger/Blighted Blackthorn: self-counter Custom -> `AddCounters` targeting self~~ **DONE**
   - ~~Day of Judgment: `Custom("Destroy all creatures.")` -> `DestroyAll`~~ **DONE**
   - ~~Frenzied Goblin/Brambleback Brute: `Custom("can't block")` -> `CantBlock`~~ **DONE**
   - ~~Icewind Elemental/Refute: loot/counter Custom -> typed effects~~ **DONE**
   - ~~ECL RemoveCounters cards (Encumbered Reejerey, Reluctant Dounguard, Heirloom Auntie, Bristlebane Battler): Custom -> `RemoveCounters` with source fallback~~ **DONE**
   - ~~Mistmeadow Council: Custom -> `draw_cards(1)`~~ **DONE**
   - ~~Guarded Heir, Prideful Parent, Resolute Reinforcements, Release the Dogs, Dwynen's Elite, Dragonmaster Outcast, Searslicer Goblin: token creation Custom -> `create_token()`~~ **DONE**
   - ~~Clachan Festival (ECL): token creation Custom + Cost::Custom -> `create_token()` + `Cost::pay_mana()`~~ **DONE**
   - ~~Burglar Rat/Dream Seizer/Arbiter of Woe/Bloodtithe Collector: opponent discard Custom -> `DiscardOpponents { count }`~~ **DONE**
   - ~~Warren Torchmaster: compound blight self + target haste Custom -> `AddCountersSelf` + `GainKeywordUntilEndOfTurn`~~ **DONE**

3. ~~**Fight mechanic** -- New `Effect::Fight` and `Effect::Bite` variants. Fight = mutual damage, Bite = one-way. Fixed 6 cards.~~ **PARTIAL** — auto-selects strongest creature instead of player choice (needs multi-target `TargetSpec`). ETB triggers correct; spell-based is heuristic.

### Phase 2: Key Missing Mechanics

4. **Equipment system** -- Attach/detach, equip cost, stat/keyword application. Unblocks ~15 cards.

5. **Modal spells** -- Mode selection in `PlayerDecisionMaker` trait, mode-conditional effect resolution. Unblocks ~20 cards.

6. **Impulse draw** -- "Exile top N, may play until end of [next] turn." Track exiled-playable cards in game state. Unblocks ~10 cards.

7. **Earthbend** (TLA-specific) -- "Look at top N, put a land to hand, rest on bottom." Unblocks ~20 TLA cards.

### Phase 3: Advanced Systems

8. **Replacement effects** -- Event interception pipeline. Required for damage prevention, death replacement, Doubling Season, "exile instead of dying."

9. **X-cost spells** -- Variable cost determination + passing X to effects.

10. **Aura attachment** -- Auras attach to targets, apply continuous effects while attached.

11. **Spell copy** -- Clone spells on the stack with new targets.

12. **Planeswalker system** -- Loyalty as a resource, planeswalker abilities, damage redirection.

13. **Additional combat phases** -- Extra attack steps.

### Phase 4: Set-Specific Mechanics

14. **Behold** (ECL) -- Reveal-from-hand alternative cost/condition.
15. **Vivid** (ECL) -- Color-count calculation for dynamic X values.
16. **Learn** (TLA) -- May discard to draw keyword action.
17. **Renew** (TDM) -- Counter-based death replacement.
18. **Mobilize** (TDM) -- Create N 1/1 Soldier tokens. (Partially works via `CreateToken` already.)

---

## Per-Set Status

Detailed per-card breakdowns with fix instructions are in `docs/`:

| File | Set | Complete | Partial | Stub |
|------|-----|----------|---------|------|
| `docs/fdn-remediation.md` | Foundations | 95 | 126 | 267 |
| `docs/tla-remediation.md` | Avatar: TLA | 39 | 22 | 219 |
| `docs/tdm-remediation.md` | Tarkir: Dragonstorm | 97 | 115 | 59 |
| `docs/ecl-remediation.md` | Lorwyn Eclipsed | 56 | 69 | 105 |

*Note: These counts are outdated -- see the individual remediation docs for current status.*

Each remediation doc includes:
- Full card-by-card audit with working vs broken effects
- Java source file references for each card
- Specific fix instructions per card
- Priority remediation roadmap for that set
