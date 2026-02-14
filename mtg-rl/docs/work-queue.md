# Card Remediation Work Queue

This document drives the batch card-fix loop. Each batch groups cards that share a common missing implementation. Work top-to-bottom; check off batches as completed.

## Process (for each batch)

1. **Read this file** to find the next unchecked batch
2. **Read the engine files** (`abilities.rs`, `game.rs`) to understand current state
3. **Implement** the engine change (new Effect/StaticEffect variant + match arm), if needed
4. **Add a test** for any new engine effect in `game.rs` `mod tests`
5. **Fix the cards** listed in the batch (grep to find exact lines)
6. **Verify**: `cargo check -p mtg-cards && cargo test --lib`
7. **Update docs**: Mark cards fixed in `docs/{set}-remediation.md`, update ROADMAP.md
8. **Check off** the batch below and note the date

---

## Batch 1: LoseLifeOpponents [DONE 2026-02-13]

**Engine**: Added `Effect::LoseLifeOpponents { amount }` variant + constructor + match arm + test.

**Cards fixed (8):**
- [x] FDN: Marauding Blight-Priest, Pulse Tracker, Vampire Spawn, Phyrexian Arena (LoseLife), Vampire Neonate
- [x] TDM: Skirmish Rhino
- [x] ECL: Champion of the Weird (effect part), Boggart Mischief (dies trigger part)

---

## Batch 2: Easy swaps — existing typed effects [DONE 2026-02-13]

**Engine**: Modified `execute_effects` in `game.rs` to accept an optional `source: Option<ObjectId>` parameter. When `AddCounters` or `RemoveCounters` effects have no targets, they now fall back to the source permanent (for self-targeting counter effects). Added test `add_counters_self_when_no_targets`.

### 2a: DestroyAll — Day of Judgment
- [x] FDN: Day of Judgment — `Effect::Custom("Destroy all creatures.")` → `Effect::destroy_all("creatures")`

### 2b: EntersTapped — Diregraf Ghoul
- [x] FDN: Diregraf Ghoul — `StaticEffect::Custom("Enters tapped.")` → `StaticEffect::EntersTapped { filter: "self".into() }`

### 2c: CantBlock (target) — Frenzied Goblin, Brambleback Brute
- [x] FDN: Frenzied Goblin — `Effect::Custom("Target creature can't block this turn.")` → `Effect::CantBlock`
- [x] ECL: Brambleback Brute — `Effect::Custom("Target creature can't block this turn.")` → `Effect::CantBlock`
- **Note**: The work queue originally listed "Skoa Veteran" for batch 2c, but the actual card in ecl.rs was **Brambleback Brute**.
- Verify: these cards already have `TargetSpec::OpponentCreature` or similar targeting

### 2d: Self -1/-1 counters — ECL blight creatures
These use `Effect::Custom("Put N -1/-1 counter(s) on <self>.")` alongside working effects. Replace with `Effect::AddCounters`:
- [x] ECL: Blighted Blackthorn (x2) — `Custom("Put two -1/-1 counters on Blighted Blackthorn.")` → `Effect::add_counters("-1/-1", 2)` (appears twice, two abilities)
- [x] ECL: Sourbread Auntie — `Custom("Put two -1/-1 counters on Sourbread Auntie.")` → `Effect::add_counters("-1/-1", 2)`
- [x] ECL: Sting-Slinger — `Custom("Put a -1/-1 counter on Sting-Slinger.")` → `Effect::add_counters("-1/-1", 1)`
- [x] ECL: Heirloom Auntie — `Custom("Remove a -1/-1 counter from Heirloom Auntie.")` → `Effect::RemoveCounters { counter_type: "-1/-1", count: 1 }`
- [x] ECL: Encumbered Reejerey — `Custom("Remove a -1/-1 counter from Encumbered Reejerey.")` → `Effect::RemoveCounters { counter_type: "-1/-1", count: 1 }`
- [x] ECL: Reluctant Dounguard — `Custom("Remove a -1/-1 counter from Reluctant Dounguard.")` → `Effect::RemoveCounters { counter_type: "-1/-1", count: 1 }`
- [x] ECL: Bristlebane Battler — `Custom("Remove a -1/-1 counter from this creature.")` → `Effect::RemoveCounters { counter_type: "-1/-1", count: 1 }`
- **Engine tweak**: `execute_effects` now falls back to the source permanent when `AddCounters`/`RemoveCounters` have no targets (self-targeting counter effects).

### 2e: Loot (draw then discard) — Icewind Elemental, Refute
- [x] FDN: Icewind Elemental — `Custom("When this creature enters, draw a card, then discard a card.")` → `Effect::draw_cards(1), Effect::discard_cards(1)`
- [x] FDN: Refute — `Custom("Counter target spell. Draw a card, then discard a card.")` → `Effect::counter_spell(), Effect::draw_cards(1), Effect::discard_cards(1)` + fixed TargetSpec to Spell

### 2f: Simple ETB draw — Mistmeadow Council
- [x] ECL: Mistmeadow Council — `Custom("When this creature enters, draw a card.")` → `Effect::draw_cards(1)`

### Cards NOT fixed in Batch 2 (deferred):
- **Warren Torchmaster** (ECL): compound Custom with self-counter + target haste (needs modal/conditional support)
- **Dream Seizer** (ECL): compound Custom with self-counter + opponent discard (Batch 3 dependency)
- **ECL line ~1843**: "Put two -1/-1 counters on each creature." — needs `AddCountersAll` variant (new engine effect)
- **ECL line ~464**: `Cost::Custom` for -1/-1 counter removal — not an Effect, needs Cost system fix

---

## Batch 3: OpponentDiscards [DONE 2026-02-13]

**Engine**: Added `Effect::DiscardOpponents { count }` variant + `discard_opponents(count)` constructor + match arm in `execute_effects` (iterates opponents, each calls `choose_discard` then moves to graveyard). Added test `discard_opponents_effect`.

**Cards fixed (4):**
- [x] FDN: Burglar Rat — `Custom("Each opponent discards a card.")` → `Effect::discard_opponents(1)`
- [x] FDN: Arbiter of Woe — compound Custom → `discard_opponents(1), lose_life_opponents(2), draw_cards(1), gain_life(2)`
- [x] FDN: Bloodtithe Collector — `Custom(conditional)` → `Effect::discard_opponents(1)` (condition "if an opponent lost life this turn" not modeled; trigger fires unconditionally)
- [x] ECL: Dream Seizer — `Custom(compound)` → `add_counters("-1/-1", 1), discard_opponents(1)` (self-counter via Batch 2 source fallback)

---

## Batch 4: Simple token creation [DONE 2026-02-13]

**Engine**: No changes needed. `CreateToken` already works and parses P/T + keywords from `token_name`.

**Cards fixed (8 cards, 10 effect swaps):**
- [x] FDN: Guarded Heir — ETB `create_token("3/3 Knight", 2)`
- [x] FDN: Prideful Parent — ETB `create_token("1/1 Cat", 1)`
- [x] FDN: Resolute Reinforcements — ETB `create_token("1/1 Soldier", 1)`
- [x] FDN: Release the Dogs — Spell `create_token("1/1 Dog", 4)`
- [x] FDN: Dwynen's Elite — ETB `create_token("1/1 Elf Warrior", 1)` (conditional "if you control another Elf" not modeled)
- [x] FDN: Dragonmaster Outcast — Upkeep `create_token("5/5 Dragon with flying", 1)` (conditional "six or more lands" not modeled)
- [x] FDN: Searslicer Goblin — End step `create_token("1/1 Goblin", 1)` (raid condition not modeled)
- [x] ECL: Clachan Festival — ETB `create_token("1/1 Kithkin", 2)` + Activated `Cost::pay_mana("{4}{W}")` + `create_token("1/1 Kithkin", 1)`

**Tests added:** `framework_create_token_effect`, `framework_create_token_with_keyword`

**Skipped/Deferred:**
- FDN: Cat Collector (Food token) — Food tokens are artifacts, not creatures. `CreateToken` always sets `CardType::Creature`. Needs engine support for artifact tokens (new batch).
- FDN: Faebloom Trick — compound effect: create 2 Faerie tokens + reflexive trigger (tap opponent creature). Needs multi-effect resolution.
- FDN: Mender's Bounty (Food token) — same as Cat Collector.
- All token copy effects (Electroduplicate, Rite of Replication, etc.) — needs Spell Copy system.
- All variable-count tokens (Homunculus Horde, Hare Apparent, etc.) — needs dynamic count support.

---

## Batch 5: Ward keyword [DONE 2026-02-13]

**Engine**: Added `StaticEffect::Ward { cost: String }` variant + `ward()` builder. Ward is stored as structured data; mechanical enforcement (counter-unless-pay during targeting) deferred to future engine work. `KeywordAbilities::WARD` already existed in bitflags.

**Cards fixed (7):**
- [x] TDM: Aegis Sculptor — `StaticEffect::Custom("Ward {2}")` → `StaticEffect::ward("{2}")` + WARD keyword
- [x] TDM: Ambling Stormshell — `StaticEffect::Custom("Ward {2}")` → `StaticEffect::ward("{2}")` + WARD keyword
- [x] TDM: Dirgur Island Dragon — `StaticEffect::Custom("Ward {2}")` → `StaticEffect::ward("{2}")` + WARD keyword
- [x] TDM: Scavenger Regent — `StaticEffect::Custom("Ward -- Discard a card.")` → `StaticEffect::ward("Discard a card.")` + WARD keyword
- [x] ECL: Bristlebane Battler — `StaticEffect::Custom("Ward {2}")` → `StaticEffect::ward("{2}")` + WARD keyword
- [x] FDN: Cackling Prowler — `StaticEffect::Custom("Ward {2}")` → `StaticEffect::ward("{2}")` + WARD keyword
- [x] FDN: Tolarian Terror — already had `KeywordAbilities::WARD` but no Ward ability; added `StaticEffect::ward("{2}")`

**Tests added:** `static_effect_builders` extended with Ward variant checks

---

## Batch 6: Enters-tapped-unless (conditional ETB tapped) [DONE 2026-02-13]

**Engine**: Added `StaticEffect::EntersTappedUnless { condition: String }` variant + `enters_tapped_unless()` builder. Condition is stored as structured data; mechanical enforcement (actually checking land types on ETB) deferred to future engine work.

**Cards fixed (2):**
- [x] TDM: Cori Mountain Monastery — `StaticEffect::Custom("Enters tapped unless you control Plains or Island.")` → `StaticEffect::enters_tapped_unless("you control a Plains or an Island")`
- [x] TDM: Dalkovan Encampment — `StaticEffect::Custom("Enters tapped unless you control Swamp or Mountain.")` → `StaticEffect::enters_tapped_unless("you control a Swamp or a Mountain")`

**Tests added:** `static_effect_builders` extended with EntersTappedUnless variant check

**Note:** ECL Blood Crypt has a related but different pattern ("pay 2 life or enters tapped") — shockland mechanic, not a land-type check. Left as Custom for now.

---

## Batch 7: Mass keyword grant / mass boost until EOT [DONE 2026-02-13]

**Engine**: Added `Effect::BoostAllUntilEndOfTurn { filter, power, toughness }` + `boost_all_eot()` constructor and `Effect::GrantKeywordAllUntilEndOfTurn { filter, keyword }` + `grant_keyword_all_eot()` constructor. Both implemented in `execute_effects()` with filter matching and "you control" controller check. Uses P1P1/M1M1 counters for boost (same simplification as single-target `BoostUntilEndOfTurn`). Keywords use `granted_keywords` which are cleared at EOT cleanup.

**Cards fixed (8, 12 effect swaps):**
- [x] FDN: Crash Through — `Custom` → `grant_keyword_all_eot("creatures you control", "trample")` + `draw_cards(1)`
- [x] FDN: Overrun — 2× `Custom` → `boost_all_eot("creatures you control", 3, 3)` + `grant_keyword_all_eot("creatures you control", "trample")`
- [x] FDN: Make a Stand — 2× `Custom` → `boost_all_eot("creatures you control", 1, 0)` + `grant_keyword_all_eot("creatures you control", "indestructible")`
- [x] FDN: Heroic Reinforcements — 2× `Custom` → `boost_all_eot("creatures you control", 1, 1)` + `grant_keyword_all_eot("creatures you control", "haste")`
- [x] FDN: Balmor, Battlemage Captain — `Custom` → `boost_all_eot("creatures you control", 1, 0)` + `grant_keyword_all_eot("creatures you control", "trample")`
- [x] FDN: Claws Out — `Custom` → `boost_all_eot("creatures you control", 2, 2)`
- [x] ECL: Timid Shieldbearer — `Custom` → `boost_all_eot("creatures you control", 1, 1)`
- [x] ECL: Catharsis — `Custom` → `boost_all_eot("creatures you control", 1, 1)` + `grant_keyword_all_eot("creatures you control", "haste")`

**Tests added:** `boost_all_and_grant_keyword_all_until_eot`

**Skipped/Deferred:**
- TDM: Craterhoof (variable +X/+X) — needs dynamic count support
- ECL: Kithkin conditional ("+1/+0 + first strike for Kithkin only") — compound conditional filter
- TDM: Modal spell with mass buff option — needs modal framework (Batch 11)

---

## Batch 8: Fight/Bite mechanic [DONE 2026-02-13]

**Engine**: Added `Effect::Fight` (mutual damage) and `Effect::Bite` (one-way damage based on power) with proper two-target selection. Added `TargetSpec::Pair { CreatureYouControl, OpponentCreature }` and `TargetSpec::fight_targets()` builder for spell-based fight/bite. `select_targets_for_spec()` handles target selection by asking the decision maker. `resolve_fight_pair()` resolves fighter/target from explicit targets, source+target (ETB), or fallback auto-select. For compound effects (e.g. counter+bite), pre-fight/bite effects only apply to `targets[0]` (your creature), matching Java's per-effect target assignment.

**Cards fixed (6):**
- [x] FDN: Bite Down — `Custom` → `Effect::bite()` + `TargetSpec::fight_targets()`
- [x] FDN: Affectionate Indrik — ETB `Custom` → `Effect::fight()` + `TargetSpec::OpponentCreature`
- [x] FDN: Felling Blow — `Custom` → `Effect::add_p1p1_counters(1), Effect::bite()` + `TargetSpec::fight_targets()`
- [x] ECL: Assert Perfection — `Custom` → `Effect::boost_until_eot(1, 0), Effect::bite()` + `TargetSpec::fight_targets()`
- [x] TDM: Piercing Exhale — `Custom` → `Effect::bite()` + `TargetSpec::fight_targets()`
- [x] TDM: Knockout Maneuver — `Custom` → `Effect::add_p1p1_counters(1), Effect::bite()` + `TargetSpec::fight_targets()`

**Tests added:** `fight_and_bite_effects`, `fight_auto_selects_creatures`, `compound_bite_counters_only_on_your_creature`

**Skipped/Deferred:**
- TDM: Dragonclaw Strike — "Double P/T then fight" — doubling not implementable
- TDM: Dragon land (bite with Dragon-specific targeting) — custom target spec
- FDN: Primal Might — X-cost + fight (Batch 15)
- FDN: Tyvar's Stand (Choose one with fight) — modal (Batch 11)
- ECL: Kithkin Whirlwind (Choose two with fight) — modal (Batch 11)
- TLA: Earth Rumble — stub with no abilities defined

---

## Batch 9: AddCountersAll (mass counter placement) [DONE 2026-02-13]

**Engine**: Added `Effect::AddCountersAll { counter_type, count, filter }` variant + `add_counters_all()` constructor + match arm. Supports "you control" filter for controller-only targeting.

**Cards fixed (2):**
- [x] ECL: Darkness Descends — `Custom("Put two -1/-1 counters on each creature.")` → `Effect::add_counters_all("-1/-1", 2, "creatures")`
- [x] TDM: Barrensteppe Siege (Abzan mode only) — `Custom("Put +1/+1 counter on each creature you control.")` → `Effect::add_counters_all("+1/+1", 1, "creatures you control")`

**Tests added:** `add_counters_all_effect`

**Skipped/Deferred:**
- TDM: Felothar — compound: sacrifice + then put +1/+1 counter on each creature (reflexive trigger)
- TDM: Barrensteppe Siege (Mardu mode) — "each opponent sacrifices a creature" (different effect)
- TDM: Barrensteppe Siege (ETB) — modal choice not modeled

---

## Batch 10: Compound self-counter + target effects [DONE 2026-02-14]

**Engine**: Added `Effect::AddCountersSelf { counter_type, count }` variant + `add_counters_self()` constructor + match arm. Unlike `AddCounters` (which falls back to source only when targets are empty), `AddCountersSelf` always applies to the source permanent regardless of other targets. This enables compound effects where one effect targets self and another targets a chosen creature (e.g. blight self + grant haste to target).

**Cards fixed (1):**
- [x] ECL: Warren Torchmaster — `Custom("Put a -1/-1 counter ... Target creature gains haste ...")` → `add_counters_self("-1/-1", 1)` + `gain_keyword_eot("haste")` with `TargetSpec::Creature` + `.set_optional()`
- [x] ECL: Dream Seizer — already fixed in Batch 3 (`add_counters("-1/-1", 1)` + `discard_opponents(1)`, no separate target needed since both effects are self/global)

**Tests added:** `add_counters_self_with_separate_target`

---

## Batch 11: Modal spells (Choose one/two)

**Engine**: Modal framework — player chooses mode, effects resolve based on mode selection.

**Cards (~15+):**
- [ ] FDN: Abrade, Boros Charm, Slagstorm, Valorous Stance, Charming Prince
- [ ] TDM: multiple
- [ ] ECL: multiple

---

## Batch 12+: Larger systems

These require more significant engine work:
- Equipment attach/detach (Batch 12)
- Planeswalker loyalty (Batch 13)
- Token copy (Batch 14)
- X-cost spells (Batch 15)
- Aura attachment (Batch 16)
- Impulse draw / exile-and-play (Batch 17)
- Sagas / lore counters (Batch 18)
- Set mechanics: Earthbend (TLA), Blight (ECL), Vivid (ECL), Behold (ECL) (Batch 19+)
- Cost system: `Cost::Custom` for counter removal (ECL line ~464), `Cost::RemoveCounters`, `Cost::ExileFromGraveyard`

---

## How to pick the next batch

1. Always do the lowest-numbered unchecked batch first
2. If a batch has a NOTE about engine uncertainty, investigate before committing
3. After finishing a batch, update this file AND the per-set remediation docs
4. Run `cargo test --lib` before and after every batch
