# ECL (Lorwyn Eclipsed) Card Remediation

## Overview
- Total cards: 230 (excluding basic lands)
- Complete: 56
- Partial: 69
- Stub: 105
- Missing: 0

## How to Fix Cards

### Engine Context
The Rust MTG engine is structured as a Cargo workspace with these key crates:

- **mtg-engine** (`mtg-engine/src/`) — Core game logic
  - `game.rs` — Game loop and `execute_effects()` dispatcher
  - `abilities.rs` — `Effect`, `StaticEffect`, `Cost`, `Ability`, `TargetSpec` enums and constructors
  - `effects.rs` — `StaticEffect` variant definitions
  - `constants.rs` — `CardType`, `SubType`, `KeywordAbilities`, `Rarity`, etc.
  - `events.rs` — `EventType` enum for triggers
- **mtg-cards** (`mtg-cards/src/`) — Card definitions
  - `sets/ecl.rs` — All ECL card factory functions
  - `registry.rs` — `CardRegistry` for name-to-factory mapping

### Working Effects (handled in `execute_effects()`)
These `Effect` variants have actual implementations in the game engine:
- `Effect::DealDamage { amount }` / `deal_damage(n)`
- `Effect::Destroy` / `destroy()`
- `Effect::Exile` / `exile()`
- `Effect::Bounce` / `bounce()` (return to hand)
- `Effect::DrawCards { count }` / `draw_cards(n)`
- `Effect::GainLife { amount }` / `gain_life(n)`
- `Effect::LoseLife { amount }` / `lose_life(n)`
- `Effect::DealDamageOpponents { amount }` / `damage_opponents(n)`
- `Effect::AddCounters { counter_type, count }` / `add_counters(type, n)`
- `Effect::BoostUntilEndOfTurn { power, toughness }` / `boost_until_eot(p, t)`
- `Effect::TapTarget` / (via tap effects)
- `Effect::UntapTarget` / (via untap effects)
- `Effect::CounterSpell` / `counter_spell()`
- `Effect::AddMana { mana }` / (via mana abilities)
- `Effect::DiscardCards { count }` / `discard_cards(n)`
- `Effect::Mill { count }` / `mill(n)`
- `Effect::CreateToken { token_name, count }` / `create_token(name, n)` — now parses P/T and keywords from token_name
- `Effect::Scry { count }` / `scry(n)`
- `Effect::SearchLibrary { filter }` / `search_library(filter)`
- `Effect::ReturnFromGraveyard` / `return_from_graveyard()`
- `Effect::Reanimate` / `reanimate()`
- `Effect::GainKeywordUntilEndOfTurn { keyword }` / `gain_keyword_eot(kw)`
- `Effect::GainKeyword { keyword }` / `gain_keyword(kw)`
- `Effect::LoseKeyword { keyword }` / `lose_keyword(kw)`
- `Effect::Indestructible` / `indestructible()`
- `Effect::Hexproof` / `hexproof()`
- `Effect::CantBlock` / `cant_block()`
- `Effect::Sacrifice { filter }` / `sacrifice(filter)`
- `Effect::DestroyAll { filter }` / `destroy_all(filter)`
- `Effect::DealDamageAll { amount, filter }` / `deal_damage_all(n, filter)`
- `Effect::RemoveCounters { counter_type, count }` / `remove_counters(type, n)`
- `Effect::CreateTokenTappedAttacking { token_name, count }` / `create_token_tapped_attacking(name, n)`
- `Effect::BoostPermanent { power, toughness }` / `boost_permanent(p, t)`
- `Effect::SetPowerToughness { power, toughness }` / `set_power_toughness(p, t)`

### Non-functional Effects (NO-OP at runtime)
These fall through to `_ => {}` in `execute_effects()`:
- `Effect::SetLife` — Set life total
- `Effect::MustBlock` — Force blocking
- `Effect::PreventCombatDamage` — Prevent damage
- `Effect::GainControl` / `GainControlUntilEndOfTurn` — Steal
- `Effect::GainProtection` — Protection
- `Effect::Custom(...)` — always a no-op
- `StaticEffect::Custom(...)` — non-functional in continuous effects system

### Adding a New Effect Type
1. Add the variant to the `Effect` enum in `mtg-engine/src/abilities.rs`
2. Add a convenience constructor (e.g., `pub fn my_effect() -> Self { Effect::MyEffect }`)
3. Handle the variant in the `execute_effects()` match in `mtg-engine/src/game.rs`
4. Update card code in `mtg-cards/src/sets/ecl.rs` to use the new typed variant instead of `Effect::Custom`

---

## Complete Cards
These cards use only working typed effects and will function correctly in gameplay.

- [x] **Bitterbloom Bearer** — Flash, flying. Upkeep: `lose_life(1)` + `create_token`. All effects work.
- [x] **Boggart Cursecrafter** — Deathtouch. Goblin dies: `damage_opponents(1)`. Works.
- [x] **Boggart Prankster** — Attack trigger: `boost_until_eot(1, 0)`. Works.
- [x] **Boldwyr Aggressor** — Double strike. Static: `grant_keyword_controlled`. Works (static may need validation).
- [x] **Chitinous Graspling** — Changeling, reach. Keywords only, no abilities. Works.
- [x] **Chomping Changeling** — Changeling. ETB: `destroy()` targeting artifact/enchantment. Works.
- [x] **Crossroads Watcher** — Trample. Creature ETB: `boost_until_eot(1, 0)`. Works.
- [x] **Dawn's Light Archer** — Flash, reach. Keywords only. Works.
- [x] **Deepchannel Duelist** — Static: `boost_controlled("other Merfolk", 1, 1)`. Works.
- [x] **Elder Auntie** — ETB: `create_token("1/1 Goblin", 1)`. Works.
- [x] **Enraged Flamecaster** — Reach. Spell cast: `damage_opponents(2)`. Works.
- [x] **Flaring Cinder** — ETB + spell trigger: `draw_cards(1)` + `discard_cards(1)`. Works.
- [x] **Flock Impostor** — Changeling, flash, flying. ETB: `bounce()`. Works.
- [x] **Gangly Stompling** — Changeling, trample. Keywords only. Works.
- [x] **Kinsbaile Aspirant** — Creature ETB: `add_counters("+1/+1", 1)`. Works.
- [x] **Kulrath Mystic** — Spell cast: `boost_until_eot(2, 0)` + `gain_keyword_eot("vigilance")`. Fully works (`gain_keyword_eot` NOW IMPLEMENTED).
- [x] **Merrow Skyswimmer** — Convoke, flying, vigilance. ETB: `create_token("1/1 Merfolk", 1)`. Works.
- [x] **Mischievous Sneakling** — Changeling, flash. Keywords only. Works.
- [x] **Moonglove Extractor** — Attacks: `draw_cards(1)` + `lose_life(1)`. Works.
- [x] **Nightmare Sower** — Flying, lifelink. Trigger: `add_counters("-1/-1", 1)`. Works.
- [x] **Noggle Robber** — ETB + dies: `create_token("Treasure", 1)`. Works.
- [x] **Pestered Wellguard** — Tapped trigger: `create_token("1/1 Faerie", 1)`. Works.
- [x] **Prideful Feastling** — Changeling, lifelink. Keywords only. Works.
- [x] **Rimekin Recluse** — ETB: `bounce()`. Works.
- [x] **Scarblade Scout** — Lifelink. ETB: `mill(2)`. Works.
- [x] **Silvergill Mentor** — ETB: `create_token("1/1 Merfolk", 1)`. Works.
- [x] **Silvergill Peddler** — Tapped trigger: `draw_cards(1)` + `discard_cards(1)`. Works.
- [x] **Summit Sentinel** — Dies: `draw_cards(1)`. Works.
- [x] **Sun-Dappled Celebrant** — Convoke, vigilance. Keywords only. Works.
- [x] **Tanufel Rimespeaker** — Spell cast: `draw_cards(1)`. Works.
- [x] **Virulent Emissary** — Deathtouch. Creature ETB: `gain_life(1)`. Works.
- [x] **Wanderbrine Preacher** — Tapped: `gain_life(2)`. Works.
- [x] **Wanderwine Distracter** — Tapped: `boost_until_eot(-3, 0)`. Works.
- [x] **Appeal to Eirdu** — Convoke. Spell: `boost_until_eot(2, 1)`. Works.
- [x] **Blight Rot** — Spell: `add_counters("-1/-1", 4)`. Works.
- [x] **Blossoming Defense** — Spell: `boost_until_eot(2, 2)` + `hexproof()`. Fully works (`hexproof()` NOW IMPLEMENTED).
- [x] **Cinder Strike** — Spell: `deal_damage(2)`. Works (misses blighted bonus).
- [x] **Feed the Flames** — Spell: `deal_damage(5)`. Works (misses exile-if-dies clause).
- [x] **Nameless Inversion** — Changeling. Spell: `boost_until_eot(3, -3)`. Works.
- [x] **Protective Response** — Convoke. Spell: `destroy()`. Works.
- [x] **Reckless Ransacking** — Spell: `boost_until_eot(3, 2)` + `create_token("Treasure", 1)`. Works.
- [x] **Run Away Together** — Spell: `bounce()` x2. Works.
- [x] **Sear** — Spell: `deal_damage(4)`. Works.
- [x] **Spell Snare** — Spell: `counter_spell()`. Works (misses MV 2 restriction).
- [x] **Thoughtweft Charge** — Spell: `boost_until_eot(3, 3)`. Works (misses conditional draw).
- [x] **Tweeze** — Spell: `deal_damage(3)`. Works (misses optional loot).
- [x] **Wild Unraveling** — Spell: `counter_spell()`. Works (misses additional cost).
- [x] **Unexpected Assistance** — Convoke. Spell: `draw_cards(3)` + `discard_cards(1)`. Works.
- [x] **Liminal Hold** — ETB: `exile()` + `gain_life(2)`. Works.
- [x] **Disruptor of Currents** — Flash, convoke. ETB: `bounce()`. Works.
- [x] **Thoughtweft Lieutenant** — Kithkin ETB: `boost_until_eot(1, 1)` + `gain_keyword_eot("trample")`. Fully works (`gain_keyword_eot` NOW IMPLEMENTED).
- [x] **Surly Farrier** — Activated: `boost_until_eot(1, 1)` + `gain_keyword_eot("vigilance")`. Fully works (`gain_keyword_eot` NOW IMPLEMENTED).
- [x] **Giantfall** — Spell: `destroy()`. Works.
- [x] **Keep Out** — Spell: `destroy()`. Works.
- [x] **Pyrrhic Strike** — Spell: `destroy()`. Works.
- [x] **Unforgiving Aim** — Spell: `destroy()`. Works.
- [x] **Trystan's Command** — Spell: `destroy()`. Works (misses modal choices).

Note: `gain_keyword_eot()`, `scry()`, `hexproof()`, and other previously no-op effects are NOW IMPLEMENTED. Cards in this section are now truly complete with all effects functional.

## Partial Cards
These cards have some typed effects that work but also use `Effect::Custom`, `StaticEffect::Custom`, or `Cost::Custom` for one or more abilities. The Custom parts are no-ops.

- [ ] **Adept Watershaper** — What works: stats (3/4). What's broken: `StaticEffect::grant_keyword_controlled` (may or may not be implemented).
  - **Java source**: `Mage.Sets/src/mage/cards/a/AdeptWatershaper.java`
  - **What it should do**: Other tapped creatures you control have indestructible.
  - **Fix needed**: Verify `grant_keyword_controlled` is handled in the continuous effects system. If not, implement it.

- [ ] **Bile-Vial Boggart** — What works: `add_counters("-1/-1", 1)`. What's broken: dies trigger targeting may not work (needs creature target on death).
  - **Java source**: `Mage.Sets/src/mage/cards/b/BileVialBoggart.java`
  - **What it should do**: Dies: put a -1/-1 counter on target creature.
  - **Fix needed**: Verify targeting works for dies triggers.

- [x] **Blighted Blackthorn** — ETB/attacks: `add_counters("-1/-1", 2)` + `draw_cards(1)` + `lose_life(1)` — **FIXED** (was `Effect::Custom("Put two -1/-1 counters on Blighted Blackthorn")`, now uses typed `add_counters` with source fallback)

- [x] **Brambleback Brute** — ETB `add_counters("-1/-1", 2)` + activated: `Effect::CantBlock` — **FIXED** (was `Effect::Custom("Target creature can't block this turn.")`, now uses typed `CantBlock` variant)

- [ ] **Burdened Stoneback** — What works: ETB `add_counters("-1/-1", 2)`. `gain_keyword_eot("indestructible")` NOW WORKS.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BurdenedStoneback.java`
  - **What it should do**: {2}{W}: Gains indestructible until end of turn.
  - **Status**: Keyword grant now implemented. Card may be fully functional.

- [ ] **Champion of the Weird** — What works: `LoseLifeOpponents(2)` on activated ability. What's broken: `Cost::Custom` (blight counter), behold mechanic, and return-exiled-card effect.
  - **Java source**: `Mage.Sets/src/mage/cards/c/ChampionOfTheWeird.java`
  - **What it should do**: Behold a Goblin + exile it. {1}{B}, blight 1: each opponent loses 2 life. Leaves: return exiled card.
  - **Fix needed**: Implement behold mechanic, blight cost, and return-exiled-card effect. (`LoseLifeOpponents` now works.)

- [ ] **Champions of the Perfect** — What works: `draw_cards(1)` on spell cast. What's broken: LTB returns exiled card (Custom).
  - **Java source**: `Mage.Sets/src/mage/cards/c/ChampionsOfThePerfect.java`
  - **What it should do**: Behold+exile Elf. Creature spell: draw. Leaves: return exiled card.
  - **Fix needed**: Implement behold mechanic and return-exiled-card.

- [ ] **Changeling Wayfinder** — What works: stats. `search_library("basic land")` NOW WORKS.
  - **Java source**: `Mage.Sets/src/mage/cards/c/ChangelingWayfinder.java`
  - **What it should do**: ETB: search library for basic land to hand.
  - **Status**: SearchLibrary now implemented. Card may be fully functional.

- [ ] **Chaos Spewer** — What works: stats (5/4). What's broken: ETB `Effect::Custom("Pay {2} or blight 2")`.
  - **Java source**: `Mage.Sets/src/mage/cards/c/ChaosSpewer.java`
  - **What it should do**: ETB: pay {2} or put two -1/-1 counters on it.
  - **Fix needed**: Need player choice + conditional self-counter placement.

- [x] **Dream Seizer** — FIXED (Batch 3). ETB now uses `add_counters("-1/-1", 1), discard_opponents(1)`.

- [ ] **Dundoolin Weaver** — What works: stats. `return_from_graveyard()` NOW WORKS.
  - **Java source**: `Mage.Sets/src/mage/cards/d/DundoolinWeaver.java`
  - **What it should do**: ETB (if 3+ creatures): return target permanent card from graveyard to hand.
  - **Status**: ReturnFromGraveyard now implemented. Card may be fully functional.

- [ ] **Eclipsed Boggart** — What works: stats (2/3). What's broken: ETB `Effect::Custom("look at top 4")`.
  - **Java source**: `Mage.Sets/src/mage/cards/e/EclipsedBoggart.java`
  - **What it should do**: Look at top 4, may reveal Goblin/Swamp/Mountain to hand.
  - **Fix needed**: New effect for look-at-top-N-reveal-filtered.

- [ ] **Eclipsed Elf** — What works: stats (3/2). What's broken: ETB `Effect::Custom("look at top 4")`.
  - **Java source**: `Mage.Sets/src/mage/cards/e/EclipsedElf.java`
  - **What it should do**: Look at top 4, may reveal Elf/Swamp/Forest to hand.
  - **Fix needed**: Same as Eclipsed Boggart.

- [ ] **Eclipsed Flamekin** — What works: stats (1/4). What's broken: ETB `Effect::Custom("look at top 4")`.
  - **Java source**: `Mage.Sets/src/mage/cards/e/EclipsedFlamekin.java`
  - **What it should do**: Look at top 4, may reveal Elemental/Island/Mountain to hand.
  - **Fix needed**: Same as above.

- [ ] **Eclipsed Kithkin** — What works: stats (2/1). What's broken: ETB `Effect::Custom("look at top 4")`.
  - **Java source**: `Mage.Sets/src/mage/cards/e/EclipsedKithkin.java`
  - **What it should do**: Look at top 4, may reveal Kithkin/Forest/Plains to hand.
  - **Fix needed**: Same as above.

- [ ] **Eclipsed Merrow** — What works: stats (2/3). What's broken: ETB `Effect::Custom("look at top 4")`.
  - **Java source**: `Mage.Sets/src/mage/cards/e/EclipsedMerrow.java`
  - **What it should do**: Look at top 4, may reveal Merfolk/Plains/Island to hand.
  - **Fix needed**: Same as above.

- [x] **Encumbered Reejerey** — ETB `add_counters("-1/-1", 3)` + tapped trigger: `RemoveCounters { counter_type: "-1/-1", count: 1 }` — **FIXED** (was `Effect::Custom("Remove a -1/-1 counter from Encumbered Reejerey.")`, now uses typed `RemoveCounters` with source fallback)

- [ ] **Explosive Prodigy** — What works: stats. What's broken: ETB `Effect::Custom("X damage where X = colors")`.
  - **Java source**: `Mage.Sets/src/mage/cards/e/ExplosiveProdigy.java`
  - **What it should do**: Vivid: ETB deals X damage where X = colors among your permanents.
  - **Fix needed**: Implement Vivid damage calculation.

- [ ] **Feisty Spikeling** — What works: changeling, stats. What's broken: `StaticEffect::Custom("your turn: first strike")`.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FeistySpikeling.java`
  - **What it should do**: Has first strike on your turn.
  - **Fix needed**: Implement conditional keyword granting in continuous effects.

- [ ] **Flame-Chain Mauler** — What works: `boost_until_eot(1, 0)`. `gain_keyword_eot("menace")` NOW WORKS.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FlameChainMauler.java`
  - **What it should do**: {1}{R}: +1/+0 and gains menace until EOT.
  - **Status**: Both effects now functional. Card may be fully functional.

- [ ] **Gallant Fowlknight** — What works: stats. What's broken: ETB `Effect::Custom("creatures +1/+0, Kithkin first strike")`.
  - **Java source**: `Mage.Sets/src/mage/cards/g/GallantFowlknight.java`
  - **What it should do**: ETB: all creatures +1/+0, Kithkin also gain first strike.
  - **Fix needed**: Implement mass boost + conditional keyword granting.

- [ ] **Glamermite** — What works: flash, flying, stats. What's broken: ETB `Effect::Custom("tap or untap target creature")`.
  - **Java source**: `Mage.Sets/src/mage/cards/g/Glamermite.java`
  - **What it should do**: ETB: tap or untap target creature.
  - **Fix needed**: Implement `TapTarget`/`UntapTarget` selection or use existing variants.

- [ ] **Glister Bairn** — What works: stats. What's broken: `Effect::Custom("+X/+X where X = colors")`.
  - **Java source**: `Mage.Sets/src/mage/cards/g/GlisterBairn.java`
  - **What it should do**: Vivid: begin combat, target creature gets +X/+X.
  - **Fix needed**: Implement Vivid boost calculation.

- [ ] **Gnarlbark Elm** — What works: ETB `add_counters("-1/-1", 2)`, activated `boost_until_eot(-2, -2)`. What's broken: `Cost::RemoveCounters` may not be implemented.
  - **Java source**: `Mage.Sets/src/mage/cards/g/GnarlbarkElm.java`
  - **What it should do**: Remove two -1/-1 counters: target creature gets -2/-2.
  - **Fix needed**: Verify `Cost::RemoveCounters` is handled in cost payment.

- [ ] **Goldmeadow Nomad** — What works: `create_token("1/1 Kithkin", 1)`. What's broken: `Cost::Custom("Exile from graveyard")` means the ability can't be activated.
  - **Java source**: `Mage.Sets/src/mage/cards/g/GoldmeadowNomad.java`
  - **What it should do**: {3}{W}, exile from graveyard: create 1/1 Kithkin token.
  - **Fix needed**: Implement `Cost::ExileFromGraveyard`.

- [ ] **Graveshifter** — What works: stats, changeling. `return_from_graveyard()` NOW WORKS.
  - **Java source**: `Mage.Sets/src/mage/cards/g/Graveshifter.java`
  - **What it should do**: ETB: return target creature from graveyard to hand.
  - **Status**: ReturnFromGraveyard now implemented. Card may be fully functional.

- [ ] **Gristle Glutton** — What works: `draw_cards(1)` + `discard_cards(1)`. What's broken: `Cost::Blight(1)` may not be implemented.
  - **Java source**: `Mage.Sets/src/mage/cards/g/GristleGlutton.java`
  - **What it should do**: {T}, blight 1: loot.
  - **Fix needed**: Verify `Cost::Blight` is handled in cost payment.

- [ ] **Gutsplitter Gang** — What works: stats (6/6). What's broken: triggered `Effect::Custom("blight 2 or lose 3 life")`.
  - **Java source**: `Mage.Sets/src/mage/cards/g/GutsplitterGang.java`
  - **What it should do**: Precombat main: blight 2 or lose 3 life.
  - **Fix needed**: Implement player choice mechanic.

- [x] **Heirloom Auntie** — ETB `add_counters("-1/-1", 2)` + creature dies: `scry(1)` + `RemoveCounters { counter_type: "-1/-1", count: 1 }` — **FIXED** (was `Effect::Custom("Remove a -1/-1 counter from Heirloom Auntie.")`, now uses typed `RemoveCounters` with source fallback)

- [ ] **Iron-Shield Elf** — `gain_keyword_eot("indestructible")` NOW WORKS, `Cost::Discard(1)` may work. What's broken: `Effect::Custom("Tap Iron-Shield Elf")` self-tap is still a no-op.
  - **Java source**: `Mage.Sets/src/mage/cards/i/IronShieldElf.java`
  - **What it should do**: Discard: gains indestructible, tap it.
  - **Fix needed**: Replace Custom self-tap with typed effect.

- [ ] **Kulrath Zealot** — What works: stats (6/5). `search_library()` NOW WORKS (landcycling). What's broken: ETB `Effect::Custom("exile top card, play until next end")` impulse draw is still a no-op.
  - **Java source**: `Mage.Sets/src/mage/cards/k/KulrathZealot.java`
  - **What it should do**: ETB: exile top card to play. Landcycling {2}.
  - **Fix needed**: Implement impulse draw effect.

- [ ] **Luminollusk** — What works: deathtouch. What's broken: ETB `Effect::Custom("gain X life where X = colors")`.
  - **Java source**: `Mage.Sets/src/mage/cards/l/Luminollusk.java`
  - **What it should do**: Vivid: ETB gain X life.
  - **Fix needed**: Implement Vivid life gain.

- [ ] **Lys Alana Informant** — `scry(1)` (used for surveil) NOW WORKS on both ETB and dies triggers.
  - **Java source**: `Mage.Sets/src/mage/cards/l/LysAlanaInformant.java`
  - **What it should do**: ETB + dies: surveil 1.
  - **Status**: Scry now implemented. Card may be fully functional (surveil approximated as scry).

- [ ] **Moonlit Lamenter** — What works: ETB `add_counters("-1/-1", 1)`, activated `draw_cards(1)`. What's broken: `Cost::RemoveCounters("-1/-1", 1)` may not be implemented.
  - **Java source**: `Mage.Sets/src/mage/cards/m/MoonlitLamenter.java`
  - **What it should do**: Remove -1/-1 counter: draw a card.
  - **Fix needed**: Verify `Cost::RemoveCounters` works.

- [ ] **Mutable Explorer** — What works: changeling, stats. What's broken: ETB `Effect::Custom("create tapped Mutavault token")`.
  - **Java source**: `Mage.Sets/src/mage/cards/m/MutableExplorer.java`
  - **What it should do**: ETB: create tapped Mutavault land token.
  - **Fix needed**: Implement land token creation.

- [ ] **Prismabasher** — What works: trample (6/6). What's broken: ETB `Effect::Custom("other creatures +X/+X")`.
  - **Java source**: `Mage.Sets/src/mage/cards/p/Prismabasher.java`
  - **What it should do**: Vivid: ETB other creatures get +X/+X.
  - **Fix needed**: Implement Vivid mass boost.

- [x] **Reluctant Dounguard** — ETB `add_counters("-1/-1", 2)` + other creature ETB: `RemoveCounters { counter_type: "-1/-1", count: 1 }` — **FIXED** (was `Effect::Custom("Remove a -1/-1 counter from Reluctant Dounguard.")`, now uses typed `RemoveCounters` with source fallback)

- [ ] **Rooftop Percher** — What works: changeling, flying, `gain_life(3)`. What's broken: `Effect::Custom("exile two cards from graveyards")`.
  - **Java source**: `Mage.Sets/src/mage/cards/r/RooftopPercher.java`
  - **What it should do**: ETB: exile up to 2 cards from graveyards, gain 3 life.
  - **Fix needed**: Implement targeted graveyard exile.

- [ ] **Safewright Cavalry** — What works: activated `boost_until_eot(2, 2)`. What's broken: `StaticEffect::Custom("can't be blocked by more than one")`.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SafewrightCavalry.java`
  - **What it should do**: Can't be blocked by more than one creature. {5}: Elf +2/+2.
  - **Fix needed**: Implement blocking restriction in static effects.

- [ ] **Scuzzback Scrounger** — What works: stats. What's broken: triggered `Effect::Custom("blight 1, create Treasure")`.
  - **Java source**: `Mage.Sets/src/mage/cards/s/ScuzzbackScrounger.java`
  - **What it should do**: Main phase: may blight 1, if so create Treasure.
  - **Fix needed**: Implement self-counter + conditional token creation.

- [ ] **Shimmercreep** — What works: menace (3/5). What's broken: ETB `Effect::Custom("opponents lose X, gain X life")`.
  - **Java source**: `Mage.Sets/src/mage/cards/s/Shimmercreep.java`
  - **What it should do**: Vivid: opponents lose X life, you gain X life.
  - **Fix needed**: Implement Vivid drain.

- [ ] **Shinestriker** — What works: flying (3/3). What's broken: ETB `Effect::Custom("draw X cards")`.
  - **Java source**: `Mage.Sets/src/mage/cards/s/Shinestriker.java`
  - **What it should do**: Vivid: draw X cards.
  - **Fix needed**: Implement Vivid draw.

- [ ] **Shore Lurker** — What works: flying, stats. `scry(1)` NOW WORKS.
  - **Java source**: `Mage.Sets/src/mage/cards/s/ShoreLurker.java`
  - **What it should do**: ETB: surveil 1.
  - **Status**: Scry now implemented. Card may be fully functional (surveil approximated as scry).

- [ ] **Sizzling Changeling** — What works: changeling, stats. What's broken: dies `Effect::Custom("exile top card, play until next end")`.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SizzlingChangeling.java`
  - **What it should do**: Dies: exile top card to play.
  - **Fix needed**: Implement impulse draw from death trigger.

- [x] **Sourbread Auntie** — ETB: `add_counters("-1/-1", 2)` + `create_token("1/1 Goblin", 2)` — **FIXED** (was `Effect::Custom("Put two -1/-1 counters on Sourbread Auntie.")`, now uses typed `add_counters` with source fallback)

- [ ] **Squawkroaster** — What works: double strike. What's broken: `StaticEffect::Custom("power = colors among permanents")`.
  - **Java source**: `Mage.Sets/src/mage/cards/s/Squawkroaster.java`
  - **What it should do**: Vivid: power equals colors among your permanents.
  - **Fix needed**: Implement Vivid P/T-setting static effect.

- [x] **Sting-Slinger** — Activated: `add_counters("-1/-1", 1)` + `damage_opponents(2)` — **FIXED** (was `Effect::Custom("Put a -1/-1 counter on Sting-Slinger.")`, now uses typed `add_counters` with source fallback)

- [ ] **Stoic Grove-Guide** — What works: `create_token("2/2 Elf Warrior", 1)`. What's broken: `Cost::Custom("exile from graveyard")`.
  - **Java source**: `Mage.Sets/src/mage/cards/s/StoicGroveGuide.java`
  - **What it should do**: Exile from graveyard: create 2/2 Elf Warrior.
  - **Fix needed**: Implement `Cost::ExileFromGraveyard`.

- [ ] **Stratosoarer** — What works: flying. `gain_keyword_eot("flying")` NOW WORKS. What's broken: missing landcycling.
  - **Java source**: `Mage.Sets/src/mage/cards/s/Stratosoarer.java`
  - **What it should do**: ETB: give flying. Landcycling {2}.
  - **Fix needed**: Implement landcycling. Keyword grant is now functional.

- [ ] **Thoughtweft Imbuer** — What works: stats. What's broken: triggered `Effect::Custom("+X/+X where X = Kithkin")`.
  - **Java source**: `Mage.Sets/src/mage/cards/t/ThoughtweftImbuer.java`
  - **What it should do**: Attacks alone: gets +X/+X where X = Kithkin count.
  - **Fix needed**: Dynamic P/T boost based on creature count.

- [x] **Timid Shieldbearer** — Activated: boost_all_eot(+1/+1). Fixed in Batch 7.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TimidShieldbearer.java`
  - **What it should do**: {4}{W}: creatures you control get +1/+1 until EOT.
  - **Fix needed**: Implement mass boost effect.

- [ ] **Unwelcome Sprite** — What works: flying. `scry(2)` NOW WORKS.
  - **Java source**: `Mage.Sets/src/mage/cards/u/UnwelcomeSprite.java`
  - **What it should do**: Opponent's turn spell: surveil 2.
  - **Status**: Scry now implemented. Card may be fully functional (surveil approximated as scry).

- [ ] **Voracious Tome-Skimmer** — What works: flying. What's broken: `Effect::Custom("pay 1 life to draw")`.
  - **Java source**: `Mage.Sets/src/mage/cards/v/VoraciousTomeSkimmer.java`
  - **What it should do**: Opponent's turn spell: may pay 1 life, draw a card.
  - **Fix needed**: Implement optional life-payment + draw.

- [x] **Warren Torchmaster** — FIXED (Batch 10). `Effect::Custom` → `add_counters_self("-1/-1", 1)` + `gain_keyword_eot("haste")` with `TargetSpec::Creature` + `.set_optional()`. Uses new `AddCountersSelf` variant that always targets source.
  - **Java source**: `Mage.Sets/src/mage/cards/w/WarrenTorchmaster.java`

- [ ] **Scarblades Malice** — `gain_keyword_eot("deathtouch")` + `gain_keyword_eot("lifelink")` NOW WORK. What's broken: delayed death trigger `Effect::Custom` for creating 2/2 Elf on death is still a no-op.
  - **Java source**: `Mage.Sets/src/mage/cards/s/ScarbladesMalice.java`
  - **What it should do**: Creature gains deathtouch + lifelink. If it dies, create 2/2 Elf.
  - **Fix needed**: Implement delayed death trigger for token creation.

- [x] **Assert Perfection** — Fixed: `boost_until_eot(1, 0), Effect::bite()` + `TargetSpec::fight_targets()`. (Batch 8)
  - **Java source**: `Mage.Sets/src/mage/cards/a/AssertPerfection.java`
  - **What it should do**: +1/+0, then fights opponent's creature.
  - **Fix needed**: Implement `Effect::Fight`.

- [ ] **Boggart Mischief** — What works: dies trigger `LoseLifeOpponents(1)` + `GainLife(1)`. What's broken: ETB still `Effect::Custom` (blight choice + token creation).
  - **Java source**: `Mage.Sets/src/mage/cards/b/BoggartMischief.java`
  - **What it should do**: ETB: blight 1 to create 2 Goblin tokens. Goblin dies: opponents lose 1, you gain 1.
  - **Fix needed**: Blight choice + conditional token creation on ETB. (Dies trigger now works.)

- [ ] **Burning Curiosity** — What works: nothing. What's broken: `Effect::Custom("exile top 3, play until next end")`.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BurningCuriosity.java`
  - **What it should do**: Exile top 3, play until next end step.
  - **Fix needed**: Implement impulse draw effect.

- [ ] **Crib Swap** — What works: `exile()`. What's broken: `Effect::Custom("controller creates 1/1 Shapeshifter token")`.
  - **Java source**: `Mage.Sets/src/mage/cards/c/CribSwap.java`
  - **What it should do**: Exile creature, its controller creates 1/1 Shapeshifter with changeling.
  - **Fix needed**: Add token creation for target's controller.

- [ ] **Temporal Cleansing** — What works: convoke. What's broken: `Effect::Custom("put nonland permanent on library")`.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TemporalCleansing.java`
  - **What it should do**: Put nonland permanent second from top or on bottom.
  - **Fix needed**: Implement library-tuck effect.

- [x] **Darkness Descends** — Fixed: `Effect::Custom` → `Effect::add_counters_all("-1/-1", 2, "creatures")` (Batch 9).
  - **Java source**: `Mage.Sets/src/mage/cards/d/DarknessDescends.java`
  - **What it should do**: Put two -1/-1 counters on each creature.
  - **Fix needed**: Implement mass counter placement.

- [ ] **Dose of Dawnglow** — What works: targeting. `reanimate()` NOW WORKS.
  - **Java source**: `Mage.Sets/src/mage/cards/d/DoseOfDawnglow.java`
  - **What it should do**: Return creature from graveyard to battlefield.
  - **Status**: Reanimate now implemented. Card may be fully functional.

- [ ] **Midnight Tilling** — What works: `mill(4)`. `return_from_graveyard()` NOW WORKS.
  - **Java source**: `Mage.Sets/src/mage/cards/m/MidnightTilling.java`
  - **What it should do**: Mill 4, then return a permanent card from graveyard to hand.
  - **Status**: ReturnFromGraveyard now implemented. Card may be fully functional.

- [ ] **Rime Chill** — What works: targeting. What's broken: `Effect::Custom("Vivid cost reduction, tap + stun + draw")`.
  - **Java source**: `Mage.Sets/src/mage/cards/r/RimeChill.java`
  - **What it should do**: Vivid cost reduction. Tap up to 2 creatures, stun counters, draw.
  - **Fix needed**: Multiple new effects: Vivid cost reduction, tap, stun counters, draw.

- [ ] **Soul Immolation** — What works: nothing. What's broken: `Effect::Custom("blight X, deal X damage to opponents and their creatures")`.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SoulImmolation.java`
  - **What it should do**: Additional cost: blight any number. Deal that much damage to opponents and their creatures.
  - **Fix needed**: Complex — variable blight cost + mass damage.

- [ ] **Blossoming Defense** — What works: `boost_until_eot(2, 2)`. `hexproof()` NOW WORKS.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BlossomingDefense.java`
  - **What it should do**: +2/+2 and hexproof until EOT.
  - **Status**: Both effects now functional. Card may be fully functional.

- [x] **Kulrath Mystic** — `boost_until_eot(2, 0)` + `gain_keyword_eot("vigilance")` both NOW WORK. Truly complete (also listed in Complete section above).
  - **Java source**: `Mage.Sets/src/mage/cards/k/KulrathMystic.java`
  - **Status**: Fully functional. Should be moved to Complete section.

- [ ] **Flamekin Gildweaver** — What works: trample. ETB `create_token("Treasure", 1)` works.
  - **Note**: This card is actually mostly complete. The token creation works. But if Treasure tokens don't have proper sacrifice-for-mana ability, it's partial.

- [ ] **Bloom Tender** — What works: mana ability. What's broken: Should add one mana of each color you control, but produces only green.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BloomTender.java`
  - **What it should do**: T: For each color among your permanents, add one mana of that color.
  - **Fix needed**: Dynamic mana production based on permanent colors.

- [ ] **Great Forest Druid** — What works: mana ability. What's broken: Produces green but should produce any color.
  - **Java source**: `Mage.Sets/src/mage/cards/g/GreatForestDruid.java`
  - **What it should do**: T: Add one mana of any color.
  - **Fix needed**: Implement color choice for mana production.

- [ ] **Springleaf Drum** — What works: mana ability shell. What's broken: Needs tap-creature cost, produces green but should produce any color.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SpringleafDrum.java`
  - **What it should do**: T, tap creature: add any color.
  - **Fix needed**: Implement tap-creature additional cost + color choice.

- [ ] **Champion of the Clachan** — What works: `StaticEffect::boost_controlled` for Kithkin +1/+1. What's broken: LTB `Effect::Custom("return exiled card")`, missing behold mechanic.
  - **Java source**: `Mage.Sets/src/mage/cards/c/ChampionOfTheClachan.java`
  - **What it should do**: Behold+exile Kithkin. Other Kithkin +1/+1. Leaves: return exiled.
  - **Fix needed**: Implement behold + return-exiled-card.

- [ ] **Champion of the Path** — What works: `damage_opponents(0)` (hardcoded 0 instead of dynamic). What's broken: damage should equal entering creature's power; LTB Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/c/ChampionOfThePath.java`
  - **What it should do**: When another Elemental ETBs: deal damage equal to its power. Leaves: return exiled.
  - **Fix needed**: Dynamic damage calculation + behold + return-exiled.

- [x] **Catharsis** — ETB: create_token + boost_all_eot + grant_keyword_all_eot(haste). Evoke still Custom. Fixed in Batch 7.
  - **Java source**: `Mage.Sets/src/mage/cards/c/Catharsis.java`
  - **What it should do**: Incarnation with color-spent ETBs and evoke.
  - **Fix needed**: Color-spent detection, mass boost+haste, evoke mechanic.

- [ ] **Deceit** — What works: UU ETB `bounce()`, BB ETB `discard_cards(1)`. What's broken: `StaticEffect::Custom("Evoke")`, mana color-spent detection missing.
  - **Java source**: `Mage.Sets/src/mage/cards/d/Deceit.java`
  - **What it should do**: Incarnation with color-spent ETBs (bounce or forced discard) and evoke.
  - **Fix needed**: Color-spent detection + evoke mechanic.

- [ ] **Emptiness** — What works: BB ETB `add_counters("-1/-1", 3)`. `reanimate()` NOW WORKS (WW ETB). What's broken: evoke Custom, mana color-spent detection missing.
  - **Java source**: `Mage.Sets/src/mage/cards/e/Emptiness.java`
  - **What it should do**: Incarnation: WW reanimates MV 3 or less. BB puts 3 -1/-1 counters. Evoke.
  - **Fix needed**: Implement color-spent detection + evoke mechanic.

- [ ] **Deepway Navigator** — What works: flash, `StaticEffect::boost_controlled("Merfolk", 1, 0)`. What's broken: ETB `Effect::Custom("untap each other Merfolk")`, conditional static.
  - **Java source**: `Mage.Sets/src/mage/cards/d/DeepwayNavigator.java`
  - **What it should do**: ETB: untap Merfolk. 3+ Merfolk attacked: Merfolk get +1/+0.
  - **Fix needed**: Mass untap effect + conditional static boost.

- [ ] **Bristlebane Battler** — What works: trample, ETB `add_counters("-1/-1", 5)`, creature ETB: `RemoveCounters` (**FIXED**), Ward {2} (**FIXED** -- typed `StaticEffect::Ward` + WARD keyword). What's broken: nothing major remaining.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BristlebaneBattler.java`
  - **What it should do**: Ward {2}. ETB with 5 -1/-1 counters. Creature ETB: remove counter.
  - **Fix needed**: Implement Ward (remove-counter trigger now fixed).

- [ ] **Bristlebane Outrider** — What works: stats. What's broken: Both `StaticEffect::Custom` (daunt + conditional boost).
  - **Java source**: `Mage.Sets/src/mage/cards/b/BristlebaneOutrider.java`
  - **What it should do**: Can't be blocked by power 2 or less. +2/+0 if another creature ETB'd this turn.
  - **Fix needed**: Implement daunt + conditional P/T boost.

- [ ] **Dawnhand Dissident** — What works: `scry(1)` NOW WORKS, `exile()`. What's broken: `StaticEffect::Custom("cast exiled creatures")`.
  - **Java source**: `Mage.Sets/src/mage/cards/d/DawnhandDissident.java`
  - **What it should do**: T, blight 1: surveil 1. T, blight 2: exile from graveyard. Cast exiled creatures by removing counters.
  - **Fix needed**: Implement cast-from-exile mechanic.

- [ ] **Barbed Bloodletter** — What works: `StaticEffect::boost_controlled("equipped creature", 1, 2)`. What's broken: ETB `Effect::Custom("attach and grant wither")`, equip `Effect::Custom("attach")`.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BarbedBloodletter.java`
  - **What it should do**: Flash equipment. ETB: attach, grant wither. +1/+2. Equip {2}.
  - **Fix needed**: Implement equipment attach + wither keyword.

- [ ] **Bark of Doran** — What works: `StaticEffect::boost_controlled("equipped", 0, 1)`. What's broken: `StaticEffect::Custom("damage equal to toughness")`, equip `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BarkOfDoran.java`
  - **What it should do**: +0/+1. If toughness > power, assigns damage equal to toughness. Equip {1}.
  - **Fix needed**: Implement toughness-as-damage + equip action.

- [ ] **Blood Crypt** — What works: mana abilities for {B} and {R}. What's broken: `StaticEffect::Custom("pay 2 life or enters tapped")`.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BloodCrypt.java`
  - **What it should do**: Shock land — pay 2 life or enters tapped.
  - **Fix needed**: Implement ETB replacement effect for shock lands.

- [ ] **Chronicle of Victory** — What works: `StaticEffect::boost_controlled` + `grant_keyword_controlled`, spell cast `draw_cards(1)`. What's broken: ETB `Effect::Custom("choose creature type")` means type choice never happens.
  - **Java source**: `Mage.Sets/src/mage/cards/c/ChronicleOfVictory.java`
  - **What it should do**: Choose type. Chosen-type creatures get +2/+2, first strike, trample. Cast chosen type: draw.
  - **Fix needed**: Implement creature type choice + conditional application.

- [ ] **Bre of Clan Stoutarm** — `gain_keyword_eot("flying")` + `gain_keyword_eot("lifelink")` + `reanimate()` all NOW WORK. What's broken: life-gain tracking for conditional reanimate.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BreOfClanStoutarm.java`
  - **What it should do**: T: give flying+lifelink. End step if gained life: reanimate creature with MV <= life gained.
  - **Fix needed**: Implement life-gain tracking for conditional reanimate trigger.

## Stub Cards
These cards exist as permanents with correct stats/types but none of their special abilities function. They are either keyword-only with Custom statics, have only Custom effects, or are stat-only placeholders.

- [ ] **Ashling, Rekindled** — Legendary creature, no mana cost, no power/toughness, no abilities. Pure placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AshlingRekindled.java`

- [ ] **Brigid, Clachan's Heart** — Legendary creature, no mana cost, no power/toughness, no abilities. Pure placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BrigidClachansHeart.java`

- [ ] **Evershrike's Gift** — Aura with `StaticEffect::Custom("Static effect.")`. No real abilities.
  - **Java source**: `Mage.Sets/src/mage/cards/e/EvershrikesGift.java`

- [ ] **Figure of Fable** — 1/1 Kithkin with `Effect::Custom("Activated effect.")`. Placeholder activated ability.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FigureOfFable.java`

- [ ] **Firdoch Core** — Kindred artifact with `Effect::Custom("Activated effect.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FirdochCore.java`

- [ ] **Flitterwing Nuisance** — 2/2 Faerie Rogue flying with `Effect::Custom("Activated effect.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FlitterwingNuisance.java`

- [ ] **Foraging Wickermaw** — 1/3 Scarecrow with `Effect::Custom("ETB effect.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/f/ForagingWickermaw.java`

- [ ] **Gathering Stone** — Artifact with `Effect::Custom("ETB effect.")` + `StaticEffect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/g/GatheringStone.java`

- [ ] **Gilt-Leaf's Embrace** — Aura with `Effect::Custom("ETB effect.")` + `StaticEffect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/g/GiltLeafsEmbrace.java`

- [ ] **Glamer Gifter** — 1/2 Faerie Wizard, flash, flying with `Effect::Custom("ETB effect.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/g/GlamerGifter.java`

- [ ] **Glen Elendra Guardian** — 3/4 Faerie Wizard, flash, flying with `Effect::Custom("Activated effect.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/g/GlenElendraGuardian.java`

- [ ] **Goliath Daydreamer** — 4/4 Giant Wizard with `Effect::Custom` on spell cast + attack. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/g/GoliathDaydreamer.java`

- [ ] **Grub, Storied Matriarch** — Legendary creature, no stats, `Effect::Custom("Attack trigger.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/g/GrubStoriedMatriarch.java`

- [ ] **Grub's Command** — Kindred sorcery. Has `destroy()` but is actually a modal command — the destroy is only one mode. Effectively partial but classified as stub because the card identity (modal command) is completely broken.
  - **Java source**: `Mage.Sets/src/mage/cards/g/GrubsCommand.java`

- [ ] **Hallowed Fountain** — Shock land with subtypes but no abilities (no mana abilities, no pay-2-life-or-tapped). Pure placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/h/HallowedFountain.java`

- [ ] **Harmonized Crescendo** — Instant with `Effect::Custom("Spell effect.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/h/HarmonizedCrescendo.java`

- [ ] **Hexing Squelcher** — 2/2 Goblin Sorcerer with `StaticEffect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/h/HexingSquelcher.java`

- [ ] **High Perfect Morcant** — Legendary 4/4 Elf Noble. No abilities at all. Pure placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/h/HighPerfectMorcant.java`

- [ ] **Illusion Spinners** — 4/3 Faerie Wizard, flying, hexproof with `StaticEffect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/i/IllusionSpinners.java`

- [ ] **Kinbinding** — Enchantment with `StaticEffect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/k/Kinbinding.java`

- [ ] **Kinscaer Sentry** — 2/2 Kithkin Soldier, first strike, lifelink with `Effect::Custom("Attack trigger.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/k/KinscaerSentry.java`

- [ ] **Lasting Tarfire** — Enchantment. No abilities at all. Pure placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/l/LastingTarfire.java`

- [ ] **Lluwen, Imperfect Naturalist** — Legendary 1/3 Elf Druid with all `Effect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/l/LluwenImperfectNaturalist.java`

- [ ] **Loch Mare** — 4/5 Horse Serpent with `Effect::Custom("Activated effect.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/l/LochMare.java`

- [ ] **Lofty Dreams** — Aura with `Effect::Custom("ETB effect.")` + `StaticEffect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/l/LoftyDreams.java`

- [ ] **Maralen, Fae Ascendant** — Legendary 4/5, flying with `StaticEffect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/m/MaralenFaeAscendant.java`

- [ ] **Mirrormind Crown** — Equipment with `StaticEffect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/m/MirrormindCrown.java`

- [ ] **Moonshadow** — 7/7 Elemental. No abilities. Pure placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/m/Moonshadow.java`

- [ ] **Morcant's Loyalist** — 3/2 Elf Warrior with `Effect::Custom("Dies effect.")` + `StaticEffect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/m/MorcantsLoyalist.java`

- [ ] **Mornsong Aria** — Legendary enchantment with `StaticEffect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/m/MornsongAria.java`

- [ ] **Noggle the Mind** — Aura, flash with `StaticEffect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/n/NoggleTheMind.java`

- [ ] **Omni-Changeling** — 0/0 Shapeshifter with `StaticEffect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/o/OmniChangeling.java`

- [ ] **Overgrown Tomb** — Shock land with subtypes but no abilities. Pure placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/o/OvergrownTomb.java`

- [ ] **Perfect Intimidation** — Sorcery with `Effect::Custom("Spell effect.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/p/PerfectIntimidation.java`

- [ ] **Pitiless Fists** — Aura with `Effect::Custom("ETB effect.")` + `StaticEffect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/p/PitilessFists.java`

- [ ] **Prismatic Undercurrents** — Enchantment with `Effect::Custom` + `StaticEffect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/p/PrismaticUndercurrents.java`

- [ ] **Puca's Eye** — Artifact with `Effect::Custom("ETB effect.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/p/PucasEye.java`

- [ ] **Pummeler for Hire** — 4/4 Giant, vigilance, reach with `Effect::Custom("ETB effect.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/p/PummelerForHire.java`

- [ ] **Reaping Willow** — 3/6 Treefolk Cleric, lifelink. Keywords only. No special abilities defined.
  - **Java source**: `Mage.Sets/src/mage/cards/r/ReapingWillow.java`

- [ ] **Retched Wretch** — 4/2 Goblin with `Effect::Custom("Dies effect.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/r/RetchedWretch.java`

- [ ] **Rhys, the Evermore** — Legendary 2/2, flash with `Effect::Custom("ETB effect.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/r/RhysTheEvermore.java`

- [ ] **Rimefire Torque** — Artifact with `Effect::Custom("Activated effect.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/r/RimefireTorque.java`

- [ ] **Sanar, Innovative First-Year** — Legendary 2/4 with no abilities at all. Pure placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SanarInnovativeFirstYear.java`

- [ ] **Sapling Nursery** — Enchantment, indestructible with `Effect::Custom("Activated effect.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SaplingNursery.java`

- [ ] **Selfless Safewright** — 4/2 Elf Warrior, flash, hexproof, indestructible with `Effect::Custom("ETB effect.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SelflessSafewright.java`

- [ ] **Shadow Urchin** — 3/4 Ouphe with `Effect::Custom("Attack trigger.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/s/ShadowUrchin.java`

- [ ] **Shimmerwilds Growth** — Aura with `StaticEffect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/s/ShimmerwildsGrowth.java`

- [ ] **Spinerock Tyrant** — 6/6 Dragon, flying with `Effect::Custom("Spell cast trigger.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SpinerockTyrant.java`

- [ ] **Spiral into Solitude** — Aura with `StaticEffect::Custom` + `Effect::Custom("Activated effect.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SpiralIntoSolitude.java`

- [ ] **Spry and Mighty** — Sorcery with `Effect::Custom("Spell effect.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SpryAndMighty.java`

- [ ] **Stalactite Dagger** — Equipment with `Effect::Custom("ETB effect.")` + `StaticEffect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/s/StalactiteDagger.java`

- [ ] **Steam Vents** — Shock land with subtypes but no abilities. Pure placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SteamVents.java`

- [ ] **Sunderflock** — 5/5 Elemental, flying with `Effect::Custom("ETB effect.")` + `StaticEffect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/s/Sunderflock.java`

- [ ] **Sygg, Wanderwine Wisdom** — Legendary creature. No stats, no abilities. Pure placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SyggWanderwineWisdom.java`

- [ ] **Sygg's Command** — Kindred sorcery with `Effect::Custom("Spell effect.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SyggsCommand.java`

- [ ] **Tam, Mindful First-Year** — Legendary 2/2 with `StaticEffect::Custom` + `Effect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TamMindfulFirstYear.java`

- [ ] **Taster of Wares** — 3/2 Goblin Warlock with `Effect::Custom("ETB effect.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TasterOfWares.java`

- [ ] **Temple Garden** — Shock land with subtypes but no abilities. Pure placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TempleGarden.java`

- [ ] **Trystan, Callous Cultivator** — Legendary creature with deathtouch. No other abilities. Mostly placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TrystanCallousCultivator.java`

- [ ] **Twilight Diviner** — 3/3 Elf Cleric with `Effect::Custom("ETB effect.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TwilightDiviner.java`

- [ ] **Twinflame Travelers** — 3/3 Elemental Sorcerer, flying with `StaticEffect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TwinflameTravelers.java`

- [ ] **Unbury** — Instant with `Effect::Custom("Spell effect.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/u/Unbury.java`

- [ ] **Vibrance** — 4/4 creature with `Effect::Custom("ETB effect.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/v/Vibrance.java`

- [ ] **Wanderwine Farewell** — Kindred sorcery with `Effect::Custom("Spell effect.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/w/WanderwineFarewell.java`

- [ ] **Wary Farmer** — 3/3 Kithkin Citizen. No abilities at all. Pure placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/w/WaryFarmer.java`

- [ ] **Wildvine Pummeler** — 6/5 Giant, reach, trample with `StaticEffect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/w/WildvinePummeler.java`

- [ ] **Winnowing** — Sorcery with `Effect::Custom("Spell effect.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/w/Winnowing.java`

- [ ] **Wistfulness** — 6/5 Elemental Incarnation with `Effect::Custom("ETB effect.")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/w/Wistfulness.java`

- [ ] **Abigale, Eloquent First-Year** — 1/1 Bird Bard, flying, first strike, lifelink with `Effect::Custom("target loses abilities, gets counters")`. Placeholder for ETB.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AbigaleEloquentFirstYear.java`

- [ ] **Ashling's Command** — Kindred instant with `Effect::Custom("choose two modes")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AshlingsCommand.java`

- [ ] **Auntie's Sentence** — Sorcery with `Effect::Custom("choose one mode")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AuntiesSentence.java`

- [ ] **Aurora Awakener** — 7/7 Giant, trample with `Effect::Custom("Vivid ETB reveal")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AuroraAwakener.java`

- [ ] **Bloodline Bidding** — Sorcery, convoke with `Effect::Custom("mass reanimate by type")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BloodlineBidding.java`

- [ ] **Blossombind** — Aura with `Effect::Custom("tap enchanted creature")` + `StaticEffect::Custom("can't untap/counters")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/b/Blossombind.java`

- [ ] **Brigid's Command** — Kindred sorcery with `Effect::Custom("choose two modes")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BrigidsCommand.java`

- [ ] **Celestial Reunion** — Sorcery with `Effect::search_library()` NOW WORKS. May be more functional now.
  - **Java source**: `Mage.Sets/src/mage/cards/c/CelestialReunion.java`

- [ ] **Collective Inferno** — Enchantment, convoke with `Effect::Custom("choose type")` + `StaticEffect::Custom("double damage")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/c/CollectiveInferno.java`

- [ ] **Curious Colossus** — 7/7 Giant with `Effect::Custom("opponent creatures become 1/1 Cowards")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/c/CuriousColossus.java`

- [ ] **Dawn-Blessed Pennant** — Artifact with `Effect::Custom("choose type")`, gain_life trigger, return_from_graveyard NOW WORKS. Partially works.
  - **Java source**: `Mage.Sets/src/mage/cards/d/DawnBlessedPennant.java`

- [ ] **Doran, Besieged by Time** — Legendary 0/5. `StaticEffect::Custom("cost reduction")` + `Effect::Custom("gets +X/+X")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/d/DoranBesiegedByTime.java`

- [ ] **Eclipsed Realms** — Land with `StaticEffect::Custom("choose type")`, mana abilities work partially.
  - **Java source**: `Mage.Sets/src/mage/cards/e/EclipsedRealms.java`

- [ ] **Eirdu, Carrier of Dawn** — Legendary 5/5, flying, lifelink with `StaticEffect::Custom` (convoke for creatures + transform). Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/e/EirduCarrierOfDawn.java`

- [ ] **Aquitect's Defenses** — Aura, flash. ETB `gain_keyword_eot("hexproof")` NOW WORKS + `StaticEffect::boost_controlled("enchanted creature", 1, 2)`. The +1/+2 may work. More functional now.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AquitectsDefenses.java`

- [ ] **Meek Attack** — Enchantment with `Effect::Custom("cheat creature from hand")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/m/MeekAttack.java`

- [ ] **Ajani, Outland Chaperone** — Planeswalker with `Effect::Custom("+1 ability")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AjaniOutlandChaperone.java`

- [ ] **Oko, Lorwyn Liege** — Legendary creature with `Effect::Custom("transform")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/o/OkoLorwynLiege.java`

- [ ] **Bogslither's Embrace** — Sorcery with `Effect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BogslithersEmbrace.java`

- [ ] **Boneclub Berserker** — 2/4 Goblin with `StaticEffect::Custom("+2/+0 per Goblin")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BoneclubBerserker.java`

- [ ] **Boulder Dash** — Sorcery with `Effect::Custom("2 damage + 1 damage split")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BoulderDash.java`

- [ ] **Champions of the Shoal** — 4/6 Merfolk with `Effect::Custom("tap + stun counter")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/c/ChampionsOfTheShoal.java`

- [x] **Clachan Festival** — **FIXED** (ETB: `create_token("1/1 Kithkin", 2)`, Activated: `Cost::pay_mana("{4}{W}")` + `create_token("1/1 Kithkin", 1)`)
  - **Java source**: `Mage.Sets/src/mage/cards/c/ClachanFestival.java`

- [ ] **Creakwood Safewright** — 5/5 Elf with `Effect::Custom("end step remove counter")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/c/CreakwoodSafewright.java`

- [ ] **Dawnhand Eulogist** — 3/3 Elf, menace with `Effect::Custom("mill 3, conditional drain")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/d/DawnhandEulogist.java`

- [ ] **Dream Harvest** — Sorcery with `Effect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/d/DreamHarvest.java`

- [ ] **End-Blaze Epiphany** — Instant with `Effect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/e/EndBlazeEpiphany.java`

- [ ] **Flamebraider** — 2/2 Elemental Bard. No abilities at all. Pure placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/f/Flamebraider.java`

- [ ] **Formidable Speaker** — 2/4 Elf Druid with all `Effect::Custom` + `Cost::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FormidableSpeaker.java`

- [ ] **Glen Elendra's Answer** — Instant with `Effect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/g/GlenElendrasAnswer.java`

- [ ] **Gloom Ripper** — 4/4 Elf Assassin with `Effect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/g/GloomRipper.java`

- [ ] **Goatnap** — Sorcery with `Effect::Custom("gain control")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/g/Goatnap.java`

- [ ] **Gravelgill Scoundrel** — 1/3 Merfolk, vigilance with `Effect::Custom("tap creature, unblockable")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/g/GravelgillScoundrel.java`

- [ ] **Hovel Hurler** — 6/7 Giant, flying. Keywords only, no special abilities. Stat stick.
  - **Java source**: `Mage.Sets/src/mage/cards/h/HovelHurler.java`

- [ ] **Impolite Entrance** — Sorcery with `Effect::Custom("gain trample+haste")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/i/ImpoliteEntrance.java`

- [ ] **Kindle the Inner Flame** — Kindred sorcery with all `Effect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/k/KindleTheInnerFlame.java`

- [ ] **Kirol, Attentive First-Year** — Legendary 3/3. No abilities at all. Pure placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/k/KirolAttentiveFirstYear.java`

- [ ] **Kithkeeper** — 3/3 Elemental, flying. No special abilities. Stat stick.
  - **Java source**: `Mage.Sets/src/mage/cards/k/Kithkeeper.java`

- [ ] **Lavaleaper** — 4/4 Elemental, haste. Keywords only. Stat stick.
  - **Java source**: `Mage.Sets/src/mage/cards/l/Lavaleaper.java`

- [ ] **Lys Alana Dignitary** — 2/3 Elf Advisor. No abilities. Pure placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/l/LysAlanaDignitary.java`

- [ ] **Meanders Guide** — 3/2 Merfolk with `Effect::Custom("tap Merfolk")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/m/MeandersGuide.java`

- [ ] **Mirrorform** — Instant with `Effect::Custom("mass copy")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/m/Mirrorform.java`

- [x] **Mistmeadow Council** — 4/3 Kithkin, ETB: `draw_cards(1)` — **FIXED** (was `Effect::Custom("When this creature enters, draw a card.")`, now uses typed `draw_cards(1)`)

- [ ] **Moon-Vigil Adherents** — 0/0 Elf, trample with `StaticEffect::Custom("+1/+1 per creature")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/m/MoonVigilAdherents.java`

- [ ] **Morcant's Eyes** — Kindred enchantment with `Effect::Custom("surveil 1")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/m/MorcantsEyes.java`

- [ ] **Morningtide's Light** — Sorcery with `Effect::Custom("exile+return creatures")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/m/MorningtidesLight.java`

- [ ] **Mudbutton Cursetosser** — 2/1 Goblin with `Effect::Custom("dies: destroy creature")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/m/MudbuttonCursetosser.java`

- [ ] **Personify** — Instant with `Effect::Custom("flicker + create token")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/p/Personify.java`

- [ ] **Raiding Schemes** — Enchantment with `StaticEffect::Custom("Conspire")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/r/RaidingSchemes.java`

- [ ] **Requiting Hex** — Instant with `Effect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/r/RequitingHex.java`

- [ ] **Riverguard's Reflexes** — Instant with `Effect::Custom("+2/+2 first strike untap")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/r/RiverguardsReflexes.java`

- [ ] **Slumbering Walker** — 4/7 Giant with `Effect::Custom("end step remove counter, reanimate")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SlumberingWalker.java`

- [ ] **Soulbright Seeker** — 2/1 Elemental, trample with all `Effect::Custom` + `Cost::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SoulbrightSeeker.java`

- [ ] **Swat Away** — Instant with `Effect::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SwatAway.java`

- [ ] **Tend the Sprigs** — Sorcery with `Effect::Custom("search land + conditional token")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TendTheSprigs.java`

- [ ] **Thirst for Identity** — Instant with `Effect::Custom("draw 3, conditional discard")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/t/ThirstForIdentity.java`

- [ ] **Tributary Vaulter** — 1/3 Merfolk, flying with `Effect::Custom("tapped: Merfolk +2/+0")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TributaryVaulter.java`

- [ ] **Vinebred Brawler** — 4/2 Elf with `Effect::Custom("attacks: Elf +2/+1")`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/v/VinebredBrawler.java`

- [ ] **Wanderbrine Trapper** — 2/1 Merfolk with all `Effect::Custom` + `Cost::Custom`. Placeholder.
  - **Java source**: `Mage.Sets/src/mage/cards/w/WanderbrineTrapper.java`

## Missing Cards
No missing cards. All 230 non-basic-land cards from `LorwynEclipsed.java` have factory functions in `ecl.rs`.

---

## Priority Fixes (Highest Impact)

### ~~1. Implement `GainKeywordUntilEndOfTurn` in execute_effects()~~ DONE
~~Unblocks: Burdened Stoneback, Flame-Chain Mauler, Iron-Shield Elf, Kulrath Mystic, Scarblades Malice, Stratosoarer, and many more.~~
Now implemented. Cards using `gain_keyword_eot()` are functional.

### ~~2. Implement `Scry`/`Surveil` in execute_effects()~~ DONE
~~Unblocks: Lys Alana Informant, Shore Lurker, Unwelcome Sprite, Heirloom Auntie, Dawnhand Dissident, Morcant's Eyes.~~
Now implemented. Surveil approximated as scry.

### ~~3. Update cards using `Effect::Custom` for `RemoveCounters`~~ MOSTLY DONE
The `RemoveCounters` effect is implemented, and the engine now falls back to source permanent when no targets are present.
~~Cards needing card-code updates: Encumbered Reejerey, Reluctant Dounguard, Heirloom Auntie, Bristlebane Battler~~ -- **DONE** (Batch 2).
Cards still needing updates: Creakwood Safewright, Slumbering Walker.

### ~~4. Implement `ReturnFromGraveyard` and `Reanimate` in execute_effects()~~ DONE
~~Unblocks: Dundoolin Weaver, Graveshifter, Midnight Tilling, Dose of Dawnglow, Emptiness, Bre of Clan Stoutarm, Dawn-Blessed Pennant.~~
Now implemented. Cards using `return_from_graveyard()` and `reanimate()` are functional.

### ~~5. Implement `SearchLibrary` in execute_effects()~~ DONE
~~Unblocks: Changeling Wayfinder, Kulrath Zealot, Celestial Reunion, Tend the Sprigs.~~
Now implemented. Cards using `search_library(filter)` are functional.

### 6. Implement Vivid mechanic (count colors among permanents)
Unblocks: Explosive Prodigy, Glister Bairn, Luminollusk, Prismabasher, Shimmercreep, Shinestriker, Squawkroaster, Rime Chill.

### ~~7. Fix easy Custom-to-typed replacements~~ DONE (Batch 2)
~~Several cards use `Effect::Custom` for effects that already have typed variants:~~
- ~~Mistmeadow Council: replace Custom with `draw_cards(1)`~~ **DONE**
- ~~Sourbread Auntie: replace Custom blight with `add_counters("-1/-1", 2)` (targeting self)~~ **DONE**
- ~~Blighted Blackthorn: same self-counter replacement~~ **DONE**
- ~~Sting-Slinger: same self-counter replacement~~ **DONE**
- ~~Encumbered Reejerey: replace Custom with `remove_counters("-1/-1", 1)`~~ **DONE**
- ~~Reluctant Dounguard: replace Custom with `remove_counters("-1/-1", 1)`~~ **DONE**
- ~~Heirloom Auntie: replace Custom with `remove_counters("-1/-1", 1)`~~ **DONE**

### 8. Implement shock land ETB replacement effect
Unblocks: Blood Crypt, Hallowed Fountain, Overgrown Tomb, Steam Vents, Temple Garden (5 lands).

### 9. Implement Equipment attach/equip mechanics
Unblocks: Barbed Bloodletter, Bark of Doran, Stalactite Dagger, Mirrormind Crown.
