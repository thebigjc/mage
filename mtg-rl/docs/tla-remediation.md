# TLA (Avatar: The Last Airbender) Card Remediation

## Overview
- Total cards in Rust: 236 (231 unique non-basic-land cards + 5 basic lands)
- Total unique non-basic cards in Java set: 231
- Complete: 33
- Partial: 34
- Stub: 164
- Missing: 0

## How to Fix Cards

### Engine Context

The Rust MTG engine lives in the `mtg-rl/` workspace with these key crates:

| Crate | Purpose |
|---|---|
| `mtg-engine` | Core game engine: game loop, state, effects, abilities, combat |
| `mtg-cards` | Card data factories organized by set (fdn.rs, tla.rs, tdm.rs, ecl.rs) |
| `mtg-ai` | AI players (random, heuristic, minimax) |
| `mtg-py` | PyO3 bindings for Python/Gymnasium |

Key files:
- **`mtg-engine/src/abilities.rs`** — Defines `Effect`, `StaticEffect`, `Cost`, `Ability`, and `TargetSpec` enums
- **`mtg-engine/src/game.rs`** — Contains `execute_effects()` (line ~943) which is the match statement that actually resolves effects
- **`mtg-cards/src/sets/tla.rs`** — All TLA card factory functions

The `execute_effects()` function in `game.rs` handles these **Effect** variants that ACTUALLY WORK:

| Effect Variant | What it does |
|---|---|
| `Effect::DealDamage { amount }` | Deal damage to target creature(s) or player |
| `Effect::Destroy` | Destroy target permanent (respects indestructible) |
| `Effect::Exile` | Exile target permanent |
| `Effect::Bounce` | Return target permanent to owner's hand |
| `Effect::DrawCards { count }` | Controller draws N cards |
| `Effect::GainLife { amount }` | Controller gains N life |
| `Effect::LoseLife { amount }` | Controller loses N life |
| `Effect::DealDamageOpponents { amount }` | Deal N damage to each opponent |
| `Effect::AddCounters { counter_type, count }` | Add counters to target permanent |
| `Effect::BoostUntilEndOfTurn { power, toughness }` | Boost target (simplified: uses counters) |
| `Effect::TapTarget` | Tap target permanent |
| `Effect::UntapTarget` | Untap target permanent |
| `Effect::CounterSpell` | Counter target spell on the stack |
| `Effect::AddMana { mana }` | Add mana to controller's pool |
| `Effect::DiscardCards { count }` | Controller discards N cards |
| `Effect::Mill { count }` | Controller mills N cards |
| `Effect::CreateToken { token_name, count }` | Create N token creatures (always 1/1) |

Everything else (`Effect::Custom(...)`, `Effect::Sacrifice`, `Effect::SearchLibrary`, `Effect::GainKeywordUntilEndOfTurn`, `Effect::DestroyAll`, `Effect::Scry`, `Effect::ReturnFromGraveyard`, `Effect::Reanimate`, etc.) falls through to a catch-all `_ => {}` and is a **NO-OP**.

`StaticEffect::Custom(...)` is also non-functional. `Cost::Custom(...)` will prevent activation since the engine cannot determine how to pay it.

Keywords set on the `keywords` field (Flying, Trample, Lifelink, etc.) DO work via the combat system and damage resolution — they are not effects.

### Adding a New Effect Type

To make a currently-Custom effect functional, follow this pattern:

1. **Add variant to `Effect` enum** in `mtg-engine/src/abilities.rs`
   ```rust
   pub enum Effect {
       // ... existing variants ...
       /// New: Scry N cards.
       Scry { count: u32 },
   }
   ```

2. **Add match arm in `execute_effects()`** in `mtg-engine/src/game.rs` (~line 943)
   ```rust
   Effect::Scry { count } => {
       // Look at top N cards, put any number on bottom in any order
       // (simplified: just reorder top of library)
       // ... implementation ...
   }
   ```

3. **Update card factory** in `mtg-cards/src/sets/tla.rs` to use the new typed variant instead of `Effect::Custom(...)`
   ```rust
   // Before:
   vec![Effect::Custom("Scry 2.".into())]
   // After:
   vec![Effect::scry(2)]
   ```

4. **Add convenience constructor** (optional) in abilities.rs:
   ```rust
   impl Effect {
       pub fn scry(count: u32) -> Self { Effect::Scry { count } }
   }
   ```

### Earthbend Mechanic

Many TLA cards reference "Earthbend N" which is a set-specific keyword action meaning "Look at the top N cards of your library, put a land card from among them into your hand, and the rest on the bottom." This requires a `SearchTopN` or `Earthbend` effect variant to be added to the engine.

---

## Complete Cards

These cards use ONLY typed Effect variants (or have no abilities beyond keywords/stats). They will function correctly in gameplay.

- [x] **Avatar Enthusiasts** — 2/2 vanilla creature with no abilities
- [x] **Barrels of Blasting Jelly** — Activated: pay {1}, deal 1 damage to any target. Uses `Effect::deal_damage(1)`
- [x] **Beetle-Headed Merchants** — Attack trigger: `Effect::add_p1p1_counters(1)`
- [x] **Boar-q-pine** — Spell cast trigger: `Effect::add_p1p1_counters(1)`
- [x] **Callous Inspector** — Dies trigger: `Effect::create_token("Clue Artifact", 1)`
- [x] **Corrupt Court Official** — ETB: `Effect::discard_cards(1)` targeting player
- [x] **Crescent Island Temple** — ETB: `Effect::create_token("Monk Red", 1)`
- [x] **Cruel Administrator** — Attack trigger: `Effect::add_p1p1_counters(1)`
- [x] **Curious Farm Animals** — Dies: `Effect::gain_life(3)`, Activated: `Effect::destroy()` targeting permanent. Fully typed
- [x] **Earth Kingdom Soldier** — ETB: `Effect::add_p1p1_counters(1)` targeting creature. Keywords: Vigilance
- [x] **Fire Nation Raider** — ETB: `Effect::create_token("Clue Artifact", 1)`
- [x] **Foggy Swamp Vinebender** — 4/3 vanilla creature
- [x] **Forecasting Fortune Teller** — ETB: `Effect::create_token("Clue Artifact", 1)`
- [x] **Hog-Monkey** — 3/2 vanilla creature
- [x] **Iguana Parrot** — 2/2 flying vigilance vanilla creature
- [x] **Invasion Reinforcements** — Flash, ETB: `Effect::create_token("Ally", 1)`
- [x] **Jeong Jeong's Deserters** — ETB: `Effect::add_p1p1_counters(1)` targeting creature
- [x] **Katara, Bending Prodigy** — Activated: tap, `Effect::draw_cards(1)`
- [x] **Kyoshi Warriors** — ETB: `Effect::create_token("Ally", 1)`
- [x] **Momo, Playful Pet** — 1/1 flying vigilance vanilla
- [x] **Northern Air Temple** — ETB: `Effect::gain_life(1)`
- [x] **Pirate Peddlers** — 2/2 deathtouch vanilla
- [x] **Pretending Poxbearers** — Dies: `Effect::create_token("Ally", 1)`
- [x] **Professor Zei, Anthropologist** — Activated: pay {1}, `Effect::draw_cards(1)`
- [x] **Rabaroo Troop** — 3/5 flying vanilla
- [x] **Raucous Audience** — 2/1 vanilla
- [x] **Rebellious Captives** — 2/2 vanilla
- [x] **Rough Rhino Cavalry** — 5/5 trample vanilla
- [x] **Rowdy Snowballers** — ETB: `Effect::add_p1p1_counters(1)` targeting creature
- [x] **Saber-Tooth Moose-Lion** — 7/7 reach vanilla
- [x] **The Spirit Oasis** — ETB: `Effect::draw_cards(1)`
- [x] **Tolls of War** — ETB: `Effect::create_token("Clue Artifact", 1)`
- [x] **Treetop Freedom Fighters** — Haste, ETB: `Effect::create_token("Ally", 1)`

## Partial Cards

These cards have SOME typed effects but also use `Effect::Custom(...)`, `StaticEffect::Custom(...)`, or `Cost::Custom(...)` for one or more abilities. The Custom parts will be no-ops during gameplay.

- [ ] **Allies at Last** — What works: nothing (instant shell). What's broken: `StaticEffect::Custom("Affinity for allies.")` — cost reduction doesn't work.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AlliesAtLast.java`
  - **What it should do**: Affinity for creatures with "Ally" subtype; put two +1/+1 counters on target creature.
  - **Fix needed**: Add typed effect for Affinity cost reduction + `Effect::AddCounters` for the spell effect

- [ ] **Azula, On the Hunt** — What works: attack trigger fires. What's broken: `Effect::create_token("token", 1)` creates a generic 1/1 instead of the correct token.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AzulaOnTheHunt.java`
  - **What it should do**: When attacks, create a 1/1 black Human Rogue token tapped and attacking.
  - **Fix needed**: Improve token creation to support specific token types and "tapped and attacking" modifier

- [ ] **Badgermole** — What works: trample keyword. What's broken: ETB `Effect::Custom("Earthbend 2.")` and `StaticEffect::Custom` (power/toughness boost based on lands).
  - **Java source**: `Mage.Sets/src/mage/cards/b/Badgermole.java`
  - **What it should do**: ETB Earthbend 2 (look at top 2, put land to hand). Gets +2/+2 as long as you control 6+ lands.
  - **Fix needed**: Implement Earthbend effect variant; implement conditional P/T boost static effect

- [ ] **Badgermole Cub** — What works: nothing beyond stats. What's broken: ETB `Effect::Custom("Earthbend 1.")`
  - **Java source**: `Mage.Sets/src/mage/cards/b/BadgermoleCub.java`
  - **What it should do**: ETB Earthbend 1.
  - **Fix needed**: Implement Earthbend effect variant

- [ ] **Cat-Gator** — What works: lifelink keyword, ETB `Effect::deal_damage(1)`. What's broken: nothing — WAIT, this actually looks complete. Reclassifying...
  - Actually COMPLETE. ETB deals 1 damage, has lifelink.

- [ ] **Cat-Owl** — What works: flying keyword. What's broken: attack trigger `Effect::Custom("Attack trigger.")`
  - **Java source**: `Mage.Sets/src/mage/cards/c/CatOwl.java`
  - **What it should do**: Whenever attacks, learn (may discard a card; if you do, draw a card).
  - **Fix needed**: Implement "Learn" keyword action as a new effect variant

- [ ] **Compassionate Healer** — What works: `Effect::gain_life(1)`. What's broken: `Effect::scry(1)` is a no-op.
  - **Java source**: `Mage.Sets/src/mage/cards/c/CompassionateHealer.java`
  - **What it should do**: Whenever tapped, gain 1 life and scry 1.
  - **Fix needed**: Implement `Effect::Scry` in execute_effects

- [ ] **Dai Li Agents** — What works: `Effect::gain_life(1)`. What's broken: ETB `Effect::Custom("Earthbend 1.")` and attack trigger `Effect::Custom("Attack trigger.")`
  - **Java source**: `Mage.Sets/src/mage/cards/d/DaiLiAgents.java`
  - **What it should do**: ETB gain 1 life and Earthbend 1. Whenever attacks, each opponent loses 1 life.
  - **Fix needed**: Implement Earthbend; replace attack trigger Custom with `Effect::LoseLife` targeting opponents (or `Effect::DealDamageOpponents`)

- [ ] **Earth Kingdom General** — What works: `Effect::gain_life(1)`. What's broken: ETB `Effect::Custom("Earthbend 2.")`
  - **Java source**: `Mage.Sets/src/mage/cards/e/EarthKingdomGeneral.java`
  - **What it should do**: ETB gain 1 life and Earthbend 2.
  - **Fix needed**: Implement Earthbend effect variant

- [ ] **Earth Village Ruffians** — What works: nothing beyond stats. What's broken: dies trigger `Effect::Custom("Earthbend 2.")`
  - **Java source**: `Mage.Sets/src/mage/cards/e/EarthVillageRuffians.java`
  - **What it should do**: When dies, Earthbend 2.
  - **Fix needed**: Implement Earthbend effect variant

- [ ] **Glider Kids** — What works: flying keyword. What's broken: ETB `Effect::scry(1)` is a no-op.
  - **Java source**: `Mage.Sets/src/mage/cards/g/GliderKids.java`
  - **What it should do**: ETB scry 1.
  - **Fix needed**: Implement `Effect::Scry` in execute_effects

- [ ] **Guru Pathik** — What works: ETB `Effect::add_p1p1_counters(1)`. What's broken: spell cast trigger `Effect::Custom("Spell cast trigger.")`
  - **Java source**: `Mage.Sets/src/mage/cards/g/GuruPathik.java`
  - **What it should do**: ETB put a +1/+1 counter on target creature. Whenever you cast a Lesson spell, scry 1 then draw a card.
  - **Fix needed**: Replace Custom with `Effect::Scry` + `Effect::DrawCards` (with Lesson filter condition)

- [ ] **Invasion Submersible** — What works: ETB `Effect::add_p1p1_counters(1)` and `Effect::bounce()`. What's broken: the two effects don't properly target different things (counters on self, bounce on opponent's creature). Also vehicle Crew mechanic is missing.
  - **Java source**: `Mage.Sets/src/mage/cards/i/InvasionSubmersible.java`
  - **What it should do**: ETB put a +1/+1 counter on itself. Return target nonland permanent opponent controls to hand. Crew 2.
  - **Fix needed**: Separate self-targeting counter from opponent-targeting bounce; implement Crew

- [ ] **Jet, Freedom Fighter** — What works: ETB `Effect::add_p1p1_counters(1)` + `Effect::deal_damage(1)`. What's broken: dies `Effect::Custom("Dies effect.")`
  - **Java source**: `Mage.Sets/src/mage/cards/j/JetFreedomFighter.java`
  - **What it should do**: ETB put a +1/+1 counter and deal 1 damage to any target. When dies, return target Rebel card from graveyard to hand.
  - **Fix needed**: Implement `Effect::ReturnFromGraveyard` with type filter (or make existing variant functional)

- [ ] **Obsessive Pursuit** — What works: ETB `Effect::create_token("Clue Artifact", 1)` + `Effect::add_p1p1_counters(1)`. What's broken: lifelink keyword on an enchantment is meaningless (should be an ability that enchanted creature has lifelink).
  - **Java source**: `Mage.Sets/src/mage/cards/o/ObsessivePursuit.java`
  - **What it should do**: Aura — enchanted creature gets +2/+1 and has lifelink. When ETB, investigate (create Clue).
  - **Fix needed**: This is an Aura needing attach-to-creature + static P/T boost + granted lifelink

- [ ] **Ostrich-Horse** — What works: `Effect::add_p1p1_counters(1)` and `Effect::mill(3)`. What's broken: `Effect::Custom("Put a land card from among them...")` — the Earthbend-like selection is missing.
  - **Java source**: `Mage.Sets/src/mage/cards/o/OstrichHorse.java`
  - **What it should do**: ETB Earthbend 3 (look at top 3, put a land to hand, rest on bottom).
  - **Fix needed**: Implement Earthbend; the mill(3) is incorrect (mills to graveyard instead of bottom of library)

- [ ] **Otter-Penguin** — What works: `Effect::boost_until_eot(1, 2)`. What's broken: `Effect::Custom("Can't be blocked this turn.")`
  - **Java source**: `Mage.Sets/src/mage/cards/o/OtterPenguin.java`
  - **What it should do**: When you draw second card each turn, gets +1/+2 and can't be blocked this turn.
  - **Fix needed**: Implement "can't be blocked" as `Effect::GainKeywordUntilEndOfTurn` (or evasion variant) and make it functional

- [ ] **Sokka, Bold Boomeranger** — What works: ETB `Effect::add_p1p1_counters(1)`, spell cast trigger `Effect::add_p1p1_counters(1)`. What's broken: these are correct for a simplified version; however the ETB should actually create an Equipment token and attach it, not add counters.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SokkaBoldBoomeranger.java`
  - **What it should do**: ETB create a Boomerang Equipment token and attach it. Whenever you cast a noncreature spell, put a +1/+1 counter on this.
  - **Fix needed**: Fix ETB to create and attach equipment token instead of adding counters

- [ ] **Suki, Kyoshi Warrior** — What works: attack trigger `Effect::create_token("token", 1)`. What's broken: `StaticEffect::Custom("Static effect.")` and token is generic 1/1.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SukiKyoshiWarrior.java`
  - **What it should do**: Suki gets +1/+1 for each Ally you control. When attacks, create 1/1 white Human Ally token tapped and attacking.
  - **Fix needed**: Implement "lord" static effect counting Allies; improve token specification

- [ ] **Team Avatar** — What works: activated `Effect::deal_damage(1)`. What's broken: enchantment shell is there but the full card has more going on.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TeamAvatar.java`
  - **What it should do**: Enchantment with multiple activated/triggered abilities involving Ally creatures.
  - **Fix needed**: Implement remaining abilities from Java source

- [ ] **The Legend of Roku** — What works: first activated ability `Effect::boost_until_eot(4, 0)` (Firebending 4). What's broken: second activated ability uses `Cost::Custom` and `Effect::Custom` for dragon token creation.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TheLegendOfRoku.java`
  - **What it should do**: Firebending 4 works. Also has {8}: Create a 4/4 red Dragon token with flying and firebending 4.
  - **Fix needed**: Replace Custom cost/effect with proper mana cost + CreateToken with specific stats

- [ ] **The Rise of Sozin** — What works: first activated ability `Effect::boost_until_eot(3, 0)` (Firebending 3), menace keyword. What's broken: combat damage trigger `Effect::Custom(...)` for reanimation.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TheRiseOfSozin.java`
  - **What it should do**: Firebending 3. Menace. When deals combat damage to player, pay {X} to reanimate creatures from their graveyard.
  - **Fix needed**: Implement reanimate-from-opponent's-graveyard effect

## Stub Cards

These cards are either stat-only (vanilla with keywords at most) or have ALL abilities as Custom. They exist as permanents but none of their special abilities function.

### Spells with no effects (cast does nothing beyond going to graveyard)

- [ ] **Aang's Journey** — Sorcery, Lesson. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AangsJourney.java`
  - **What it should do**: Search library for a basic land card, put it onto the battlefield tapped.
  - **Fix needed**: Implement `Effect::SearchLibrary` for basic land fetch

- [ ] **Abandon Attachments** — Instant, Lesson. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AbandonAttachments.java`
  - **What it should do**: Return target nonland permanent to its owner's hand.
  - **Fix needed**: Add `Ability::spell` with `Effect::Bounce` targeting nonland permanent

- [ ] **Accumulate Wisdom** — Instant, Lesson. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AccumulateWisdom.java`
  - **What it should do**: Draw cards equal to the number of cards in your graveyard, up to 3.
  - **Fix needed**: Implement conditional draw count effect

- [ ] **Airbending Lesson** — Instant, Lesson. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AirbendingLesson.java`
  - **What it should do**: Put target creature on top of its owner's library.
  - **Fix needed**: Implement "put on top of library" effect

- [ ] **Bitter Work** — Enchantment. No abilities.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BitterWork.java`
  - **What it should do**: Enchantment with triggered abilities for casting creature/noncreature spells (Earthbend/damage).
  - **Fix needed**: Implement dual triggered abilities with Earthbend and damage

- [ ] **Boomerang Basics** — Sorcery, Lesson. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BoomerangBasics.java`
  - **What it should do**: Return target nonland permanent to hand. Learn.
  - **Fix needed**: Add `Effect::Bounce` + Learn keyword action

- [ ] **Combustion Technique** — Instant, Lesson. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/c/CombustionTechnique.java`
  - **What it should do**: Deal 2 damage to any target. Learn.
  - **Fix needed**: Add `Effect::DealDamage { amount: 2 }` + Learn

- [ ] **Cunning Maneuver** — Instant. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/c/CunningManeuver.java`
  - **What it should do**: Target creature gets +2/+0 and gains first strike until end of turn.
  - **Fix needed**: Add `Effect::BoostUntilEndOfTurn` + grant first strike

- [ ] **Cycle of Renewal** — Instant, Lesson. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/c/CycleOfRenewal.java`
  - **What it should do**: Return target permanent card from graveyard to hand.
  - **Fix needed**: Implement `Effect::ReturnFromGraveyard`

- [ ] **Day of Black Sun** — Sorcery. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/d/DayOfBlackSun.java`
  - **What it should do**: Each creature gets -X/-X until end of turn.
  - **Fix needed**: Implement mass -X/-X effect variant

- [ ] **Deadly Precision** — Sorcery. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/d/DeadlyPrecision.java`
  - **What it should do**: Target creature gets -2/-2 until end of turn.
  - **Fix needed**: Add `Effect::BoostUntilEndOfTurn { power: -2, toughness: -2 }` (need signed version)

- [ ] **Earth Rumble** — Sorcery. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/e/EarthRumble.java`
  - **What it should do**: Fight — target creature you control fights target creature you don't control. Earthbend 2.
  - **Fix needed**: Implement Fight effect + Earthbend

- [ ] **Earthbending Lesson** — Sorcery, Lesson. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/e/EarthbendingLesson.java`
  - **What it should do**: Earthbend 4.
  - **Fix needed**: Implement Earthbend

- [ ] **Elemental Teachings** — Instant, Lesson. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/e/ElementalTeachings.java`
  - **What it should do**: Put two +1/+1 counters on target creature, it gains hexproof until end of turn.
  - **Fix needed**: Add `Effect::AddCounters` + grant hexproof

- [ ] **Energybending** — Instant, Lesson. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/e/Energybending.java`
  - **What it should do**: Counter target noncreature spell.
  - **Fix needed**: Add `Effect::CounterSpell` (with noncreature filter)

- [ ] **Enter the Avatar State** — Instant, Lesson. Keywords (flying, first strike, lifelink, hexproof) are on the card itself, not granted to a target.
  - **Java source**: `Mage.Sets/src/mage/cards/e/EnterTheAvatarState.java`
  - **What it should do**: Target creature gains flying, first strike, lifelink, and hexproof until end of turn.
  - **Fix needed**: Keywords should be granted via effect, not set on the instant itself

- [ ] **Epic Downfall** — Sorcery. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/e/EpicDownfall.java`
  - **What it should do**: Exile target creature with mana value 3 or greater.
  - **Fix needed**: Add `Effect::Exile` with MV filter

- [ ] **Fancy Footwork** — Instant, Lesson. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FancyFootwork.java`
  - **What it should do**: Up to two target creatures each get +1/+1 until end of turn.
  - **Fix needed**: Add `Effect::BoostUntilEndOfTurn` with multi-target

- [ ] **Fatal Fissure** — Instant. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FatalFissure.java`
  - **What it should do**: Target creature gets -3/-3 until end of turn. If you control a land with 6+ lands, instead -6/-6.
  - **Fix needed**: Implement conditional P/T reduction

- [ ] **Firebending Lesson** — Instant, Lesson. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FirebendingLesson.java`
  - **What it should do**: Deal 2 damage to target creature or planeswalker.
  - **Fix needed**: Add `Effect::DealDamage { amount: 2 }`

- [ ] **Gather the White Lotus** — Sorcery. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/g/GatherTheWhiteLotus.java`
  - **What it should do**: Create two 1/1 white Human Ally tokens. Put a +1/+1 counter on each creature you control.
  - **Fix needed**: Add `Effect::CreateToken` + mass `Effect::AddCounters`

- [ ] **It'll Quench Ya!** — Instant, Lesson. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/i/ItllQuenchYa.java`
  - **What it should do**: Counter target spell unless its controller pays {2}.
  - **Fix needed**: Implement conditional counter (tax counter)

- [ ] **Jet's Brainwashing** — Sorcery. Keywords (haste) on sorcery itself.
  - **Java source**: `Mage.Sets/src/mage/cards/j/JetsBrainwashing.java`
  - **What it should do**: Gain control of target creature until end of turn. Untap it. It gains haste.
  - **Fix needed**: Implement gain-control-until-EOT + untap + grant haste

- [ ] **Lightning Strike** — Instant. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/l/LightningStrike.java`
  - **What it should do**: Deal 3 damage to any target.
  - **Fix needed**: Add `Effect::DealDamage { amount: 3 }` — this is a very easy fix

- [ ] **Lost Days** — Instant, Lesson. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/l/LostDays.java`
  - **What it should do**: Draw 2 cards. Learn.
  - **Fix needed**: Add `Effect::DrawCards { count: 2 }` + Learn

- [ ] **Octopus Form** — Instant, Lesson. Keywords (hexproof) on spell itself.
  - **Java source**: `Mage.Sets/src/mage/cards/o/OctopusForm.java`
  - **What it should do**: Target creature you control gets +2/+2 and gains hexproof until end of turn.
  - **Fix needed**: Add boost + grant hexproof as effects targeting creature

- [ ] **Ozai's Cruelty** — Sorcery, Lesson. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/o/OzaisCruelty.java`
  - **What it should do**: Target opponent discards 2 cards.
  - **Fix needed**: Add `Effect::DiscardCards { count: 2 }` targeting opponent

- [ ] **Pillar Launch** — Instant. Keywords (reach) on spell itself.
  - **Java source**: `Mage.Sets/src/mage/cards/p/PillarLaunch.java`
  - **What it should do**: Target creature gets +3/+3 and gains reach until end of turn.
  - **Fix needed**: Add `Effect::BoostUntilEndOfTurn` + grant reach

- [ ] **Price of Freedom** — Sorcery, Lesson. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/p/PriceOfFreedom.java`
  - **What it should do**: Target creature gets +2/+0 and gains haste until end of turn. Learn.
  - **Fix needed**: Add boost + grant haste + Learn

- [ ] **Razor Rings** — Instant. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/r/RazorRings.java`
  - **What it should do**: Target creature gets +2/+2 until end of turn.
  - **Fix needed**: Add `Effect::BoostUntilEndOfTurn { power: 2, toughness: 2 }`

- [ ] **Redirect Lightning** — Instant, Lesson. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/r/RedirectLightning.java`
  - **What it should do**: Change the target of target spell or ability with a single target.
  - **Fix needed**: Implement redirect effect (complex)

- [ ] **Rocky Rebuke** — Instant. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/r/RockyRebuke.java`
  - **What it should do**: Target creature you control deals damage equal to its power to target creature you don't control (bite).
  - **Fix needed**: Implement Bite effect

- [ ] **Seismic Sense** — Sorcery, Lesson. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SeismicSense.java`
  - **What it should do**: Earthbend 2.
  - **Fix needed**: Implement Earthbend

- [ ] **Shared Roots** — Sorcery, Lesson. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SharedRoots.java`
  - **What it should do**: Search library for a basic land, put onto battlefield tapped.
  - **Fix needed**: Implement `Effect::SearchLibrary` for basic land

- [ ] **Sokka's Haiku** — Instant, Lesson. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SokkasHaiku.java`
  - **What it should do**: Draw 3 cards, then discard a card. Learn.
  - **Fix needed**: Add `Effect::DrawCards { count: 3 }` + `Effect::DiscardCards { count: 1 }` + Learn

- [ ] **Sold Out** — Instant. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SoldOut.java`
  - **What it should do**: Destroy target creature or planeswalker.
  - **Fix needed**: Add `Effect::Destroy` targeting creature/planeswalker

- [ ] **Spirit Water Revival** — Sorcery. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SpiritWaterRevival.java`
  - **What it should do**: Return target creature card from graveyard to battlefield.
  - **Fix needed**: Implement `Effect::Reanimate` (make existing variant functional)

- [ ] **True Ancestry** — Sorcery, Lesson. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TrueAncestry.java`
  - **What it should do**: Search library for a basic land card, put onto battlefield.
  - **Fix needed**: Implement `Effect::SearchLibrary`

- [ ] **United Front** — Sorcery. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/u/UnitedFront.java`
  - **What it should do**: Create X 1/1 white Human Ally tokens.
  - **Fix needed**: Add `Effect::CreateToken` with X count

- [ ] **Waterbending Lesson** — Sorcery, Lesson. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/w/WaterbendingLesson.java`
  - **What it should do**: Return target creature to its owner's hand.
  - **Fix needed**: Add `Effect::Bounce` targeting creature

- [ ] **Yip Yip!** — Instant, Lesson. Keywords (flying) on spell itself.
  - **Java source**: `Mage.Sets/src/mage/cards/y/YipYip.java`
  - **What it should do**: Target creature gets +1/+3 and gains flying until end of turn.
  - **Fix needed**: Add boost + grant flying

- [ ] **Zuko's Conviction** — Instant. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/z/ZukosConviction.java`
  - **What it should do**: Target creature gets +2/+1 until end of turn. If you control a legendary creature, also first strike.
  - **Fix needed**: Add conditional boost + grant first strike

- [ ] **Zuko's Exile** — Instant, Lesson. No spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/z/ZukosExile.java`
  - **What it should do**: Exile target nonland permanent.
  - **Fix needed**: Add `Effect::Exile` targeting nonland permanent

### Creatures/Permanents with all-Custom abilities

- [ ] **Bender's Waterskin** — Artifact. Only `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BendersWaterskin.java`
  - **What it should do**: Equipped creature gets +1/+1 and has "{T}: Tap target creature." Equip {2}.
  - **Fix needed**: Implement Equipment attach, P/T boost static, granted tap ability

- [ ] **Deserter's Disciple** — Creature 2/2. Activated `Effect::Custom("Activated effect.")`
  - **Java source**: `Mage.Sets/src/mage/cards/d/DesertersDisciple.java`
  - **What it should do**: {T}: Add one mana of any color. (Mana ability.)
  - **Fix needed**: Replace with `Effect::AddMana` activated ability

- [ ] **Earth Kingdom Jailer** — Creature 3/3. ETB `Effect::Custom("ETB effect.")`
  - **Java source**: `Mage.Sets/src/mage/cards/e/EarthKingdomJailer.java`
  - **What it should do**: ETB exile target creature an opponent controls until this leaves the battlefield.
  - **Fix needed**: Implement "exile until leaves" effect (Banisher Priest pattern)

- [ ] **Earth Kingdom Protectors** — Creature 1/1. Vigilance, Indestructible. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/e/EarthKingdomProtectors.java`
  - **What it should do**: {T}: Another target creature gains indestructible until end of turn.
  - **Fix needed**: Implement grant-keyword-until-EOT targeting another creature

- [ ] **Earth Rumble Wrestlers** — Creature 3/4. Reach, Trample. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/e/EarthRumbleWrestlers.java`
  - **What it should do**: Gets +2/+0 as long as it's your turn.
  - **Fix needed**: Implement conditional P/T boost static effect (your-turn check)

- [ ] **Earthen Ally** — Creature 0/2. `StaticEffect::Custom` + activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/e/EarthenAlly.java`
  - **What it should do**: Has power equal to the number of Allies you control. {WUBRG}: Put five +1/+1 counters on Earthen Ally.
  - **Fix needed**: Implement "power equals X" static + replace activated with `Effect::AddCounters`

- [ ] **Fire Lord Azula** — Creature 4/4. Spell cast trigger `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FireLordAzula.java`
  - **What it should do**: Whenever you cast a noncreature spell, create a 1/1 Lightning token and deal 1 damage to each opponent.
  - **Fix needed**: Replace Custom with `Effect::CreateToken` + `Effect::DealDamageOpponents`

- [ ] **Fire Nation Attacks** — Instant. Activated "flashback" `Effect::Custom("Cast from graveyard.")`
  - **Java source**: `Mage.Sets/src/mage/cards/f/FireNationAttacks.java`
  - **What it should do**: Deal 4 damage to target creature. Flashback {8}{R}.
  - **Fix needed**: Add `Effect::DealDamage { amount: 4 }` as spell effect; implement Flashback keyword

- [ ] **Fire Nation Cadets** — Creature 1/2. `StaticEffect::Custom` + activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FireNationCadets.java`
  - **What it should do**: Gets +1/+0 as long as it's your turn. {2}: Gains first strike until end of turn.
  - **Fix needed**: Implement conditional P/T boost + grant first strike until EOT

- [ ] **Fire Nation Engineer** — Creature 2/3. No abilities.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FireNationEngineer.java`
  - **What it should do**: When ETB, create a Clue token. Sacrifice an artifact: Target creature gets -2/-2 until end of turn.
  - **Fix needed**: Add ETB create Clue + sacrifice-artifact activated ability with -2/-2 boost

- [ ] **Fire Sages** — Creature 2/2. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FireSages.java`
  - **What it should do**: {1}{R}{R}: Deal 3 damage to any target.
  - **Fix needed**: Replace Custom with `Effect::DealDamage { amount: 3 }`

- [ ] **Firebending Student** — Creature 1/2. No abilities.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FirebendingStudent.java`
  - **What it should do**: Prowess (whenever you cast a noncreature spell, gets +1/+1 until end of turn). When it deals combat damage to a player, Learn.
  - **Fix needed**: Implement Prowess trigger + combat damage Learn trigger

- [ ] **First-Time Flyer** — Creature 1/2. Flying. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FirstTimeFlyer.java`
  - **What it should do**: Gets +1/+0 as long as you control another Ally.
  - **Fix needed**: Implement conditional P/T boost (Ally count)

- [ ] **Flexible Waterbender** — Creature 2/5. Vigilance. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FlexibleWaterbender.java`
  - **What it should do**: {T}: Tap target creature.
  - **Fix needed**: Replace Custom with `Effect::TapTarget`

- [ ] **Flopsie, Bumi's Buddy** — Creature 4/4. ETB `Effect::Custom` + `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FlopsieBumisBuddy.java`
  - **What it should do**: ETB Earthbend 3. Other creatures you control get +1/+1.
  - **Fix needed**: Implement Earthbend + lord P/T boost

- [ ] **Foggy Swamp Hunters** — Creature 3/4. Lifelink. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FoggySwampHunters.java`
  - **What it should do**: Gets +2/+0 as long as a card was put into your graveyard this turn.
  - **Fix needed**: Implement conditional P/T boost with graveyard condition

- [ ] **Foggy Swamp Spirit Keeper** — Creature 2/4. Lifelink. Triggered `Effect::Custom("Draw second card trigger.")`
  - **Java source**: `Mage.Sets/src/mage/cards/f/FoggySwampSpiritKeeper.java`
  - **What it should do**: Whenever you draw your second card each turn, gets +1/+2 and can't be blocked this turn.
  - **Fix needed**: Implement second-draw trigger with boost + evasion

- [ ] **Geyser Leaper** — Creature 4/3. Flying. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/g/GeyserLeaper.java`
  - **What it should do**: {T}: Tap target creature. Untap this during each opponent's untap step.
  - **Fix needed**: Replace Custom with `Effect::TapTarget` + implement untap-during-opponent's-turn static

- [ ] **Giant Koi** — Creature 5/7. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/g/GiantKoi.java`
  - **What it should do**: {2}: This creature can't be blocked this turn.
  - **Fix needed**: Implement "can't be blocked" evasion effect

- [ ] **Gran-Gran** — Creature 1/2. Triggered `Effect::Custom("Tapped trigger.")`
  - **Java source**: `Mage.Sets/src/mage/cards/g/GranGran.java`
  - **What it should do**: Whenever becomes tapped, look at top 2, put 1 in hand and rest on bottom.
  - **Fix needed**: Implement "impulse" / card selection effect

- [ ] **Haru, Hidden Talent** — Creature 1/1. No abilities.
  - **Java source**: `Mage.Sets/src/mage/cards/h/HaruHiddenTalent.java`
  - **What it should do**: ETB Earthbend 1. {T}: Add {G}.
  - **Fix needed**: Add ETB Earthbend + mana ability

- [ ] **Hei Bai, Spirit of Balance** — Creature 3/3. No abilities.
  - **Java source**: `Mage.Sets/src/mage/cards/h/HeiBaiSpiritOfBalance.java`
  - **What it should do**: ETB exile target nonland permanent until this leaves. When leaves, opponent creates tokens.
  - **Fix needed**: Implement exile-until-leaves + leave-the-battlefield trigger

- [ ] **Hermitic Herbalist** — Creature 2/3. No abilities.
  - **Java source**: `Mage.Sets/src/mage/cards/h/HermiticHerbalist.java`
  - **What it should do**: ETB gain 3 life if you control 2+ creatures. {T}: Add {G} or {U}.
  - **Fix needed**: Add conditional ETB life gain + mana ability

- [ ] **Joo Dee, One of Many** — Creature 2/2. No abilities.
  - **Java source**: `Mage.Sets/src/mage/cards/j/JooDeeOneOfMany.java`
  - **What it should do**: ETB exile target creature until this leaves (or similar control effect).
  - **Fix needed**: Implement exile-until-leaves

- [ ] **June, Bounty Hunter** — Creature 2/2. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/j/JuneBountyHunter.java`
  - **What it should do**: Creatures you control with bounty counters get -2/-0. Whenever this deals combat damage, put a bounty counter on target creature.
  - **Fix needed**: Implement counter-based static debuff + combat damage trigger

- [ ] **Katara, the Fearless** — Creature 3/3. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/k/KataraTheFearless.java`
  - **What it should do**: Allies you control get +1/+1. Whenever an Ally enters, scry 1.
  - **Fix needed**: Implement lord boost + Ally ETB trigger with scry

- [ ] **Kyoshi Island Plaza** — Enchantment Shrine. ETB `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/k/KyoshiIslandPlaza.java`
  - **What it should do**: ETB create a 1/1 white Human Ally token. Allies you control get +0/+1 for each Shrine.
  - **Fix needed**: Replace Custom with `Effect::CreateToken` + implement Shrine-counting boost

- [ ] **Long Feng, Grand Secretariat** — Creature 2/3. No abilities.
  - **Java source**: `Mage.Sets/src/mage/cards/l/LongFengGrandSecretariat.java`
  - **What it should do**: Flash. ETB exile target creature until this leaves.
  - **Fix needed**: Add flash keyword + exile-until-leaves ETB

- [ ] **Mai, Jaded Edge** — Creature 1/3. No abilities.
  - **Java source**: `Mage.Sets/src/mage/cards/m/MaiJadedEdge.java`
  - **What it should do**: First strike. Whenever attacks, deal 1 damage to any target.
  - **Fix needed**: Add first strike keyword + attack trigger with `Effect::DealDamage`

- [ ] **Mai, Scornful Striker** — Creature 2/2. First strike.
  - **Java source**: `Mage.Sets/src/mage/cards/m/MaiScornfulStriker.java`
  - **What it should do**: First strike. Deathtouch. Whenever this deals damage to a creature, exile that creature.
  - **Fix needed**: Add deathtouch + damage trigger with exile

- [ ] **Master Pakku** — Creature 1/3. Triggered `Effect::Custom("Tapped trigger.")`
  - **Java source**: `Mage.Sets/src/mage/cards/m/MasterPakku.java`
  - **What it should do**: Whenever becomes tapped, tap target creature an opponent controls.
  - **Fix needed**: Replace Custom with `Effect::TapTarget` targeting opponent's creature

- [ ] **Master Piandao** — Creature 4/4. First strike. Attack trigger `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/m/MasterPiandao.java`
  - **What it should do**: First strike. When attacks, create an Equipment token and attach to this.
  - **Fix needed**: Implement equipment token creation + auto-attach

- [ ] **Merchant of Many Hats** — Creature 2/2. Activated uses `Effect::return_from_graveyard()` which is a no-op.
  - **Java source**: `Mage.Sets/src/mage/cards/m/MerchantOfManyHats.java`
  - **What it should do**: {2}{B}: Return this from graveyard to hand.
  - **Fix needed**: Make `Effect::ReturnFromGraveyard` functional in execute_effects

- [ ] **North Pole Patrol** — Creature 2/3. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/n/NorthPolePatrol.java`
  - **What it should do**: {T}: Tap target creature.
  - **Fix needed**: Replace Custom with `Effect::TapTarget`

- [ ] **Serpent of the Pass** — Creature 6/5. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SerpentOfThePass.java`
  - **What it should do**: Can't be blocked. Ward {2}.
  - **Fix needed**: Implement "can't be blocked" + ward static effects

- [ ] **Sokka, Lateral Strategist** — Creature 2/4. Vigilance. No special abilities.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SokkaLateralStrategist.java`
  - **What it should do**: Vigilance. Whenever an Ally ETB, investigate (create Clue).
  - **Fix needed**: Add Ally-ETB trigger creating Clue token

- [ ] **South Pole Voyager** — Creature 2/2. No abilities.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SouthPoleVoyager.java`
  - **What it should do**: When ETB, search library for basic land, put tapped.
  - **Fix needed**: Implement `Effect::SearchLibrary`

- [ ] **Southern Air Temple** — Enchantment Shrine. ETB `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SouthernAirTemple.java`
  - **What it should do**: ETB create a 1/1 white Human Monk Ally token. Creatures you control with flying get +1/+0 for each Shrine.
  - **Fix needed**: Replace Custom with CreateToken + Shrine-counting conditional boost

- [ ] **Sun Warriors** — Creature 3/5. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SunWarriors.java`
  - **What it should do**: {5}: Each creature you control gets +1/+1 and gains trample until end of turn.
  - **Fix needed**: Implement mass boost + grant trample effect

- [ ] **The Boulder, Ready to Rumble** — Creature 4/4. Attack trigger `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TheBoulderReadyToRumble.java`
  - **What it should do**: When attacks, Earthbend 2.
  - **Fix needed**: Implement Earthbend

- [ ] **Tiger-Dillo** — Creature 4/3. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TigerDillo.java`
  - **What it should do**: Can only attack or block alone. Tiger-Dillo attacks each combat if able.
  - **Fix needed**: Implement "attacks alone" restriction + forced attack

- [ ] **Turtle-Duck** — Creature 0/4. Trample. Activated `Effect::Custom("Set base power to 4 until end of turn.")`
  - **Java source**: `Mage.Sets/src/mage/cards/t/TurtleDuck.java`
  - **What it should do**: Trample. {3}: Base power becomes 4 until end of turn.
  - **Fix needed**: Implement "set base power" effect variant

- [ ] **Ty Lee, Artful Acrobat** — Creature 3/2. Attack trigger `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TyLeeArtfulAcrobat.java`
  - **What it should do**: When attacks, tap target creature an opponent controls.
  - **Fix needed**: Replace Custom with `Effect::TapTarget` targeting opponent's creature

- [ ] **Uncle Iroh** — Creature 4/2. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/u/UncleIroh.java`
  - **What it should do**: Other Allies you control get +1/+1. When this ETB, create a Food token.
  - **Fix needed**: Implement lord boost + ETB create Food token

- [ ] **Vengeful Villagers** — Creature 3/3. Attack trigger `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/v/VengefulVillagers.java`
  - **What it should do**: When attacks, target creature can't block this turn.
  - **Fix needed**: Implement "target can't block" until EOT effect

- [ ] **Wartime Protestors** — Creature 4/4. Haste. No special abilities beyond haste.
  - **Java source**: `Mage.Sets/src/mage/cards/w/WartimeProtestors.java`
  - **What it should do**: Haste. ETB deal 3 damage to any target.
  - **Fix needed**: Add ETB `Effect::DealDamage { amount: 3 }`

- [ ] **Water Tribe Rallier** — Creature 2/2. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/w/WaterTribeRallier.java`
  - **What it should do**: {T}: Untap target Ally.
  - **Fix needed**: Replace Custom with `Effect::UntapTarget` (with Ally filter)

- [ ] **Waterbender Ascension** — Enchantment. Activated `Effect::draw_cards(1)` — this is actually COMPLETE as an activated draw ability. But the real card is more complex.
  - **Java source**: `Mage.Sets/src/mage/cards/w/WaterbenderAscension.java`
  - **What it should do**: Enchantment with Ascension counters mechanic — gets counters from card draw, then gains additional abilities at threshold.
  - **Fix needed**: Implement Ascension counter mechanic (complex)

- [ ] **Waterbending Scroll** — Artifact. Activated draw for {6} works.
  - **Java source**: `Mage.Sets/src/mage/cards/w/WaterbendingScroll.java`
  - **What it should do**: {6}, {T}: Draw two cards. But also has a tap-to-tap-creature ability.
  - **Fix needed**: Fix draw count (should be 2, not 1) and add tap ability. Actually wait — the draw is `Effect::draw_cards(1)` but should be 2. Partial.

- [ ] **White Lotus Tile** — Artifact. No abilities.
  - **Java source**: `Mage.Sets/src/mage/cards/w/WhiteLotusTile.java`
  - **What it should do**: {T}: Add one mana of any color. Complex activated for searching legendaries.
  - **Fix needed**: Add mana ability + search effect

- [ ] **Wolfbat** — Creature 2/2. Flying. Triggered `Effect::Custom("Draw second card trigger.")`
  - **Java source**: `Mage.Sets/src/mage/cards/w/Wolfbat.java`
  - **What it should do**: Whenever you draw your second card each turn, gets +1/+2 and can't be blocked this turn.
  - **Fix needed**: Implement second-draw trigger with boost + evasion

- [ ] **Yuyan Archers** — Creature 3/1. Reach. ETB `Effect::draw_cards(1)`. WAIT — this might actually be wrong; let me check Java...
  - **Java source**: `Mage.Sets/src/mage/cards/y/YuyanArchers.java`
  - **What it should do**: Reach. ETB deal 1 damage to target creature. (Not draw a card.)
  - **Fix needed**: Replace `Effect::draw_cards(1)` with `Effect::deal_damage(1)` — currently gives wrong effect

- [ ] **Zuko, Exiled Prince** — Creature 4/3. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/z/ZukoExiledPrince.java`
  - **What it should do**: {3}: Zuko gets +2/+0 and gains menace until end of turn.
  - **Fix needed**: Replace Custom with `Effect::BoostUntilEndOfTurn` + grant menace

### Second-wave cards (batch 2, lines 2350+) — all Custom or missing abilities

- [ ] **Aang, at the Crossroads** — Creature. Flying, Vigilance. ETB + Static both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AangAtTheCrossroads.java`
  - **What it should do**: Complex multicolor mythic with ETB and static abilities for Aura/counters.
  - **Fix needed**: Read Java source for full ability set

- [ ] **Aang, Swift Savior** — Creature. Flash, Flying, Reach, Trample. ETB + Attack + Activated all Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AangSwiftSavior.java`
  - **What it should do**: Complex legendary with multiple triggered/activated abilities.
  - **Fix needed**: Read Java source for full ability set

- [ ] **Aang, the Last Airbender** — Creature 3/2. Flying, Lifelink. ETB + Spell-cast trigger both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AangTheLastAirbender.java`
  - **What it should do**: ETB return target creature to hand. Whenever you cast a Lesson, learn.
  - **Fix needed**: Replace ETB Custom with `Effect::Bounce`; implement learn trigger

- [ ] **Aang's Iceberg** — Enchantment. Flash. ETB + Activated both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AangsIceberg.java`
  - **What it should do**: ETB exile target creature until this leaves. Activated to protect.
  - **Fix needed**: Implement exile-until-leaves + protective activated

- [ ] **Abandoned Air Temple** — Land. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AbandonedAirTemple.java`
  - **What it should do**: {T}: Add {W}. Activated to create tokens or animate.
  - **Fix needed**: Add mana ability + implement creature-land activation

- [ ] **Agna Qel'a** — Land. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AgnaQela.java`
  - **What it should do**: {T}: Add {U}. Channel activated ability.
  - **Fix needed**: Add mana ability + channel effect

- [ ] **Air Nomad Legacy** — Enchantment. ETB + Static both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AirNomadLegacy.java`
  - **What it should do**: ETB create token. Static buff to flying creatures.
  - **Fix needed**: Replace Custom ETB with `Effect::CreateToken`; implement conditional boost

- [ ] **Airbender Ascension** — Enchantment. ETB `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AirbenderAscension.java`
  - **What it should do**: Ascension mechanic — gains counters, unlocks abilities at threshold.
  - **Fix needed**: Implement Ascension counter system

- [ ] **Airbender's Reversal** — Instant, Lesson. Spell `Effect::destroy()` works (destroys target creature).
  - Actually this IS functional — the spell destroys a target creature. Reclassifying as COMPLETE.

- [ ] **Airship Engine Room** — Land. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AirshipEngineRoom.java`
  - **What it should do**: Enters tapped. {T}: Add {R}. Activated for bonus.
  - **Fix needed**: Add mana ability + ETB tapped

- [ ] **Appa, Loyal Sky Bison** — Creature 4/4. Flying. No special abilities beyond keywords.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AppaLoyalSkyBison.java`
  - **What it should do**: Flying, Vigilance. When attacks, other attacking Allies get +1/+1 until EOT.
  - **Fix needed**: Add attack trigger with mass Ally boost

- [ ] **Appa, Steadfast Guardian** — Creature 3/4. Flash, Flying. ETB + Spell-cast trigger both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AppaSteadfastGuardian.java`
  - **What it should do**: Flash, Flying. ETB protect creature (hexproof/indestructible). Spellcast trigger.
  - **Fix needed**: Implement protection ETB + spell trigger

- [ ] **Avatar Aang** — Creature. Flying. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AvatarAang.java`
  - **What it should do**: Complex mythic Avatar with multiple abilities.
  - **Fix needed**: Read Java source for full abilities

- [ ] **Avatar Destiny** — Enchantment Aura. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AvatarDestiny.java`
  - **What it should do**: Aura granting abilities to enchanted creature.
  - **Fix needed**: Implement Aura attach + granted abilities

- [ ] **Avatar's Wrath** — Sorcery. Spell `Effect::Custom("Spell effect.")`
  - **Java source**: `Mage.Sets/src/mage/cards/a/AvatarsWrath.java`
  - **What it should do**: Board wipe or mass removal.
  - **Fix needed**: Replace Custom with appropriate mass effect

- [ ] **Azula Always Lies** — Instant, Lesson. Spell `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AzulaAlwaysLies.java`
  - **What it should do**: Target creature gets -3/-0 until end of turn. Learn.
  - **Fix needed**: Replace Custom with `Effect::BoostUntilEndOfTurn { power: -3, toughness: 0 }`

- [ ] **Azula, Cunning Usurper** — Creature 4/4. ETB + Static both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AzulaCunningUsurper.java`
  - **What it should do**: Complex legendary with control/steal effects.
  - **Fix needed**: Read Java source for full abilities

- [ ] **Ba Sing Se** — Land. No abilities.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BaSingSe.java`
  - **What it should do**: Legendary land with mana ability + activated.
  - **Fix needed**: Add mana ability + other activations

- [ ] **Beifong's Bounty Hunters** — Creature 4/4. No abilities.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BeifongsBountyHunters.java`
  - **What it should do**: ETB destroy target creature opponent controls with a bounty counter.
  - **Fix needed**: Implement conditional destruction

- [ ] **Benevolent River Spirit** — Creature 4/5. Flying. ETB + Static both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BenevolentRiverSpirit.java`
  - **What it should do**: ETB gain life. Static: other creatures get ability.
  - **Fix needed**: Replace ETB Custom + implement static

- [ ] **Boiling Rock Prison** — Land. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BoilingRockPrison.java`
  - **What it should do**: Enters tapped. {T}: Add {B}. Activated bonus.
  - **Fix needed**: Add mana ability + ETB tapped

- [ ] **Boiling Rock Rioter** — Creature 3/3. Attack + Activated both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BoilingRockRioter.java`
  - **What it should do**: Attack trigger + activated ability for card advantage.
  - **Fix needed**: Read Java source; implement typed effects

- [ ] **Bumi Bash** — Sorcery. Spell `Effect::destroy()` targeting creature. COMPLETE.
  - Reclassifying as COMPLETE.

- [ ] **Bumi, King of Three Trials** — Creature 4/4. ETB `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BumiKingOfThreeTrials.java`
  - **What it should do**: ETB Earthbend 3.
  - **Fix needed**: Implement Earthbend

- [ ] **Bumi, Unleashed** — Creature 5/4. Trample. ETB `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BumiUnleashed.java`
  - **What it should do**: Trample. ETB Earthbend 4. Whenever a land enters, gets +2/+2 until EOT.
  - **Fix needed**: Implement Earthbend + landfall trigger

- [ ] **Buzzard-Wasp Colony** — Creature 2/2. Flying. ETB `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BuzzardWaspColony.java`
  - **What it should do**: Flying. ETB create two 1/1 Insect tokens with flying.
  - **Fix needed**: Replace Custom with `Effect::CreateToken`

- [ ] **Canyon Crawler** — Creature 6/6. Deathtouch. ETB `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/c/CanyonCrawler.java`
  - **What it should do**: Deathtouch. ETB each player sacrifices a creature.
  - **Fix needed**: Implement mass sacrifice effect

- [ ] **Combustion Man** — Creature 4/6. Attack trigger `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/c/CombustionMan.java`
  - **What it should do**: When attacks, deal 3 damage to any target.
  - **Fix needed**: Replace Custom with `Effect::DealDamage { amount: 3 }`

- [ ] **Crashing Wave** — Sorcery. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/c/CrashingWave.java`
  - **What it should do**: Return all nonland permanents to owners' hands.
  - **Fix needed**: Implement mass bounce effect

- [ ] **Dai Li Indoctrination** — Sorcery, Lesson. Spell `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/d/DaiLiIndoctrination.java`
  - **What it should do**: Target creature gets -2/-2 until end of turn. Learn.
  - **Fix needed**: Replace Custom with `Effect::BoostUntilEndOfTurn { power: -2, toughness: -2 }`

- [ ] **Destined Confrontation** — Sorcery. Spell `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/d/DestinedConfrontation.java`
  - **What it should do**: Exile target creature. Controller creates a token copy of another creature they control.
  - **Fix needed**: Implement exile + token copy (complex)

- [ ] **Diligent Zookeeper** — Creature 4/4. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/d/DiligentZookeeper.java`
  - **What it should do**: Creatures you control with +1/+1 counters get trample.
  - **Fix needed**: Implement conditional keyword-granting static

- [ ] **Dragonfly Swarm** — Creature 0/3. Flying. Dies + Static both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/d/DragonflySwarm.java`
  - **What it should do**: Flying. Gets +1/+0 for each noncreature spell cast this turn. When dies, create tokens.
  - **Fix needed**: Implement spell-count P/T boost + dies token creation

- [ ] **Earth King's Lieutenant** — Creature 1/1. Trample. ETB `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/e/EarthKingsLieutenant.java`
  - **What it should do**: ETB Earthbend 2. Whenever a land enters, put a +1/+1 counter on this.
  - **Fix needed**: Implement Earthbend + landfall counters

- [ ] **Earthbender Ascension** — Enchantment. Trample keyword on enchantment (wrong). ETB `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/e/EarthbenderAscension.java`
  - **What it should do**: Ascension mechanic with Earthbend triggers.
  - **Fix needed**: Implement Ascension counter system with Earthbend

- [ ] **Ember Island Production** — Sorcery. Spell `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/e/EmberIslandProduction.java`
  - **What it should do**: Create token copies of creatures.
  - **Fix needed**: Implement token copy effect

- [ ] **Fated Firepower** — Enchantment. Flash. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FatedFirepower.java`
  - **What it should do**: Complex enchantment with damage-based triggers.
  - **Fix needed**: Read Java source for full abilities

- [ ] **Fire Lord Zuko** — Creature 2/4. No abilities.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FireLordZuko.java`
  - **What it should do**: Complex legendary with multiple triggered abilities.
  - **Fix needed**: Read Java source for full abilities

- [ ] **Fire Nation Palace** — Land. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FireNationPalace.java`
  - **What it should do**: {T}: Add {R}. Activated bonus.
  - **Fix needed**: Add mana ability

- [ ] **Fire Nation Warship** — Vehicle 4/4. Reach. Dies `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FireNationWarship.java`
  - **What it should do**: Crew 2. When dies, create Treasure tokens.
  - **Fix needed**: Implement Crew + dies create Treasure

- [ ] **Fire Navy Trebuchet** — Artifact Creature Wall 0/4. Defender, Reach. No special abilities.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FireNavyTrebuchet.java`
  - **What it should do**: Defender, Reach. {T}: Deal 2 damage to any target.
  - **Fix needed**: Add activated `Effect::DealDamage { amount: 2 }`

- [ ] **Firebender Ascension** — Enchantment. ETB `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FirebenderAscension.java`
  - **What it should do**: Ascension mechanic with fire/damage abilities.
  - **Fix needed**: Implement Ascension

- [ ] **Foggy Bottom Swamp** — Land. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FoggyBottomSwamp.java`
  - **What it should do**: Enters tapped. {T}: Add {B} or {G}. Activated bonus.
  - **Fix needed**: Add dual mana ability + ETB tapped

- [ ] **Foggy Swamp Visions** — Sorcery. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FoggySwampVisions.java`
  - **What it should do**: Return up to 2 creature cards from graveyard to hand.
  - **Fix needed**: Implement graveyard-to-hand return

- [ ] **Glider Staff** — Equipment. ETB + Static both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/g/GliderStaff.java`
  - **What it should do**: ETB attach to creature. Equipped creature gets +1/+1 and flying. Equip cost.
  - **Fix needed**: Implement Equipment attach/equip + P/T boost + keyword grant

- [ ] **Great Divide Guide** — Creature 2/3. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/g/GreatDivideGuide.java`
  - **What it should do**: Creatures you control have Earthbend 1.
  - **Fix needed**: Grant Earthbend to all creatures (complex)

- [ ] **Hakoda, Selfless Commander** — Creature 3/5. Vigilance, Indestructible. Static + Activated both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/h/HakodaSelflessCommander.java`
  - **What it should do**: Other Allies get +1/+1. {T}: Allies gain indestructible until EOT.
  - **Fix needed**: Implement lord boost + mass keyword grant

- [ ] **Hama, the Bloodbender** — Creature 3/3. ETB `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/h/HamaTheBloodbender.java`
  - **What it should do**: ETB gain control of target creature until end of turn, untap it, it gains haste.
  - **Fix needed**: Implement gain-control-until-EOT

- [ ] **Heartless Act** — Instant. Spell `Effect::destroy()` targeting creature. COMPLETE.
  - Reclassifying as COMPLETE.

- [ ] **Honest Work** — Enchantment Aura. ETB + Static both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/h/HonestWork.java`
  - **What it should do**: Aura that taps enchanted creature and gives controller a benefit.
  - **Fix needed**: Implement Aura mechanics

- [ ] **How to Start a Riot** — Instant, Lesson. Spell `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/h/HowToStartARiot.java`
  - **What it should do**: Creatures you control get +1/+0 and gain haste until EOT. Learn.
  - **Fix needed**: Implement mass boost + grant haste

- [ ] **Invasion Tactics** — Enchantment. ETB `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/i/InvasionTactics.java`
  - **What it should do**: ETB create tokens. Has ongoing effects.
  - **Fix needed**: Read Java source; implement typed effects

- [ ] **Iroh, Grand Lotus** — Creature 5/5. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/i/IrohGrandLotus.java`
  - **What it should do**: Complex legendary with multiple abilities.
  - **Fix needed**: Read Java source for full abilities

- [ ] **Iroh, Tea Master** — Creature 2/2. ETB `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/i/IrohTeaMaster.java`
  - **What it should do**: ETB create Food token. Other abilities.
  - **Fix needed**: Replace Custom with `Effect::CreateToken` + add other abilities

- [ ] **Iroh's Demonstration** — Sorcery, Lesson. Spell `Effect::deal_damage(4)`. COMPLETE.
  - Reclassifying as COMPLETE.

- [ ] **Jasmine Dragon Tea Shop** — Land. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/j/JasmineDragonTeaShop.java`
  - **What it should do**: Legendary land with mana + activated abilities.
  - **Fix needed**: Add mana ability + implement activated

- [ ] **Katara, Water Tribe's Hope** — Creature 3/3. Vigilance. ETB `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/k/KataraWaterTribesHope.java`
  - **What it should do**: Complex legendary with multiple abilities.
  - **Fix needed**: Read Java source for full abilities

- [ ] **Knowledge Seeker** — Creature 2/1. Vigilance. Dies `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/k/KnowledgeSeeker.java`
  - **What it should do**: When dies, draw a card.
  - **Fix needed**: Replace Custom with `Effect::DrawCards { count: 1 }`

- [ ] **Koh, the Face Stealer** — Creature 6/6. ETB + Static + Activated all Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/k/KohTheFaceStealer.java`
  - **What it should do**: Complex mythic with steal/exile/copy abilities.
  - **Fix needed**: Read Java source for full abilities

- [ ] **Kyoshi Battle Fan** — Equipment. ETB + Static both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/k/KyoshiBattleFan.java`
  - **What it should do**: Equipment with P/T boost + keyword. Auto-attach on ETB.
  - **Fix needed**: Implement Equipment mechanics

- [ ] **Kyoshi Village** — Land. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/k/KyoshiVillage.java`
  - **What it should do**: Enters tapped. {T}: Add {G} or {W}. Activated bonus.
  - **Fix needed**: Add dual mana ability

- [ ] **Lo and Li, Twin Tutors** — Creature 2/2. Lifelink. ETB + Static both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/l/LoAndLiTwinTutors.java`
  - **What it should do**: ETB tutor. Static ability.
  - **Fix needed**: Read Java source for full abilities

- [ ] **Meditation Pools** — Land. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/m/MeditationPools.java`
  - **What it should do**: Enters tapped. {T}: Add {W} or {U}. Activated bonus.
  - **Fix needed**: Add dual mana ability

- [ ] **Messenger Hawk** — Creature 1/2. Flying. ETB + Static both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/m/MessengerHawk.java`
  - **What it should do**: ETB investigate (create Clue). Static: can't be blocked by non-fliers.
  - **Fix needed**: Replace ETB Custom with `Effect::CreateToken("Clue", 1)` + implement evasion

- [ ] **Meteor Sword** — Equipment. ETB + Static both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/m/MeteorSword.java`
  - **What it should do**: Equipment with significant P/T boost. Auto-attach ETB.
  - **Fix needed**: Implement Equipment mechanics

- [ ] **Misty Palms Oasis** — Land. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/m/MistyPalmsOasis.java`
  - **What it should do**: Enters tapped. {T}: Add {U} or {R}. Activated bonus.
  - **Fix needed**: Add dual mana ability

- [ ] **Momo, Friendly Flier** — Creature 1/1. Flying. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/m/MomoFriendlyFlier.java`
  - **What it should do**: Flying. Whenever an Ally ETB, draw a card.
  - **Fix needed**: Implement Ally ETB trigger with draw

- [ ] **Mongoose Lizard** — Creature 5/6. ETB `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/m/MongooseLizard.java`
  - **What it should do**: ETB fight target creature opponent controls.
  - **Fix needed**: Implement Fight effect

- [ ] **North Pole Gates** — Land. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/n/NorthPoleGates.java`
  - **What it should do**: Enters tapped. {T}: Add {W} or {U}. Activated bonus.
  - **Fix needed**: Add dual mana ability

- [ ] **Omashu City** — Land. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/o/OmashuCity.java`
  - **What it should do**: Enters tapped. {T}: Add {R} or {G}. Activated bonus.
  - **Fix needed**: Add dual mana ability

- [ ] **Origin of Metalbending** — Instant, Lesson. Spell `Effect::destroy()` works. Indestructible keyword on instant (wrong).
  - **Java source**: `Mage.Sets/src/mage/cards/o/OriginOfMetalbending.java`
  - **What it should do**: Target creature gains indestructible until EOT. Destroy target artifact.
  - **Fix needed**: Fix: grant indestructible to creature + destroy artifact (not creature)

- [ ] **Ozai, the Phoenix King** — Creature 7/7. Trample, Haste, Flying, Indestructible. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/o/OzaiThePhoenixKing.java`
  - **What it should do**: Keywords work. Has additional abilities (probably damage triggers, opponent control).
  - **Fix needed**: Read Java source for additional abilities

- [ ] **Path to Redemption** — Enchantment Aura. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/p/PathToRedemption.java`
  - **What it should do**: Aura with pacifism-like effect (enchanted creature can't attack/block).
  - **Fix needed**: Implement Aura mechanics with attack/block restriction

- [ ] **Phoenix Fleet Airship** — Vehicle 4/4. Flying. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/p/PhoenixFleetAirship.java`
  - **What it should do**: Flying. Crew. Additional abilities when crewed.
  - **Fix needed**: Implement Crew + additional abilities

- [ ] **Planetarium of Wan Shi Tong** — Artifact. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/p/PlanetariumOfWanShiTong.java`
  - **What it should do**: Complex artifact with scry/draw abilities.
  - **Fix needed**: Replace Custom with typed effects

- [ ] **Platypus-Bear** — Creature 2/3. Defender. ETB + Static both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/p/PlatypusBear.java`
  - **What it should do**: Defender. ETB create Food token. Can attack as though it didn't have defender if you control 3+ creatures.
  - **Fix needed**: Implement conditional defender removal + ETB create Food

- [ ] **Ran and Shaw** — Creature 4/4. Flying. ETB + Activated both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/r/RanAndShaw.java`
  - **What it should do**: Flying. ETB deal damage. Activated for dragon token.
  - **Fix needed**: Replace ETB Custom with `Effect::DealDamage` + implement token creation

- [ ] **Raven Eagle** — Creature 2/3. Flying. No special abilities.
  - **Java source**: `Mage.Sets/src/mage/cards/r/RavenEagle.java`
  - **What it should do**: Flying. When deals combat damage to player, create Clue. Has flashback-like graveyard ability.
  - **Fix needed**: Implement combat damage trigger + graveyard activation

- [ ] **Realm of Koh** — Land. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/r/RealmOfKoh.java`
  - **What it should do**: {T}: Add {B}. Channel/activated ability.
  - **Fix needed**: Add mana ability + implement activated

- [ ] **Rockalanche** — Sorcery, Lesson. "Flashback" activated `Effect::Custom`. No main spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/r/Rockalanche.java`
  - **What it should do**: Each creature gets -2/-2 until end of turn. Flashback {5}{G}.
  - **Fix needed**: Add mass -2/-2 spell effect + implement Flashback

- [ ] **Ruinous Waterbending** — Sorcery, Lesson. Spell `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/r/RuinousWaterbending.java`
  - **What it should do**: Destroy target creature. If it had a +1/+1 counter, draw a card.
  - **Fix needed**: Replace Custom with `Effect::Destroy` + conditional draw

- [ ] **Rumble Arena** — Land. Vigilance keyword on land (wrong). ETB `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/r/RumbleArena.java`
  - **What it should do**: Enters tapped. {T}: Add {R} or {G}. ETB Earthbend 1.
  - **Fix needed**: Add dual mana + ETB tapped + Earthbend

- [ ] **Sandbender Scavengers** — Creature 1/1. Dies `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SandbenderScavengers.java`
  - **What it should do**: When dies, exile target permanent.
  - **Fix needed**: Replace Custom with `Effect::Exile`

- [ ] **Sandbenders' Storm** — Instant. Spell `Effect::destroy()` targeting creature. COMPLETE.
  - Reclassifying as COMPLETE.

- [ ] **Secret Tunnel** — Land Cave. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SecretTunnel.java`
  - **What it should do**: {T}: Add one mana of any color. Channel ability.
  - **Fix needed**: Add any-color mana ability + channel

- [ ] **Serpent's Pass** — Land. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SerpentsPass.java`
  - **What it should do**: Enters tapped. {T}: Add {U} or {B}. Activated bonus.
  - **Fix needed**: Add dual mana ability

- [ ] **Sokka, Tenacious Tactician** — Creature 3/3. Spell-cast trigger + Static both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SokkaTenaciousTactician.java`
  - **What it should do**: Complex legendary with Equipment/noncreature spell synergy.
  - **Fix needed**: Read Java source for full abilities

- [ ] **Solstice Revelations** — Instant, Lesson. "Flashback" `Effect::Custom`. No main spell effect.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SolsticeRevelations.java`
  - **What it should do**: Exile top 3 cards, play them until end of next turn. Flashback.
  - **Fix needed**: Implement impulse draw + Flashback

- [ ] **Sozin's Comet** — Sorcery. Spell `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SozinsComet.java`
  - **What it should do**: Deal 5 damage to each creature and each opponent.
  - **Fix needed**: Replace Custom with `Effect::DealDamageAll` + `Effect::DealDamageOpponents`

- [ ] **Sparring Dummy** — Artifact Creature 1/3. Defender. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SparringDummy.java`
  - **What it should do**: Defender. {T}: Target creature gets +1/+1 until EOT.
  - **Fix needed**: Replace Custom with `Effect::BoostUntilEndOfTurn { power: 1, toughness: 1 }`

- [ ] **Suki, Courageous Rescuer** — Creature 2/4. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SukiCourageousRescuer.java`
  - **What it should do**: Allies you control get +1/+0. When attacks, create token.
  - **Fix needed**: Implement lord boost + attack trigger

- [ ] **Sun-Blessed Peak** — Land. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SunBlessedPeak.java`
  - **What it should do**: Enters tapped. {T}: Add {R} or {W}. Activated bonus.
  - **Fix needed**: Add dual mana ability

- [ ] **Swampsnare Trap** — Enchantment Aura. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SwampsnareTrap.java`
  - **What it should do**: Aura. Enchanted creature doesn't untap during its controller's untap step. Tap enchanted creature.
  - **Fix needed**: Implement Aura + untap prevention

- [ ] **Teo, Spirited Glider** — Creature 1/4. Flying. No special abilities.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TeoSpiritedGlider.java`
  - **What it should do**: Flying. Whenever you cast a noncreature spell, create a Thopter token.
  - **Fix needed**: Add spell-cast trigger with `Effect::CreateToken`

- [ ] **The Earth King** — Creature 2/2. ETB `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TheEarthKing.java`
  - **What it should do**: ETB create Bosco token (bear companion). Other abilities.
  - **Fix needed**: Implement token creation + other abilities

- [ ] **The Fire Nation Drill** — Vehicle 6/3. Trample, Hexproof, Indestructible. ETB + Activated both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TheFireNationDrill.java`
  - **What it should do**: Keywords work. Crew. ETB destroy target creature. Activated to deal damage.
  - **Fix needed**: Implement Crew + replace ETB/activated Custom with typed effects

- [ ] **The Last Agni Kai** — Instant. Spell `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TheLastAgniKai.java`
  - **What it should do**: Target creature you control fights target creature you don't control.
  - **Fix needed**: Implement Fight effect

- [ ] **The Lion-Turtle** — Creature 3/6. Vigilance, Reach. ETB + Static both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TheLionTurtle.java`
  - **What it should do**: Keywords work. ETB and static abilities for energy/mana.
  - **Fix needed**: Read Java source for full abilities

- [ ] **The Unagi of Kyoshi Island** — Creature 5/5. Flash. No special abilities.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TheUnagiOfKyoshiIsland.java`
  - **What it should do**: Flash. ETB fight target creature opponent controls.
  - **Fix needed**: Implement Fight ETB

- [ ] **The Walls of Ba Sing Se** — Artifact Creature Wall 0/30. Defender, Indestructible. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TheWallsOfBaSingSe.java`
  - **What it should do**: Keywords work. Static abilities for board protection.
  - **Fix needed**: Read Java source for static abilities

- [ ] **Tiger-Seal** — Creature 3/3. Vigilance. No special abilities.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TigerSeal.java`
  - **What it should do**: Enters tapped unless you control an Island. Whenever attacks alone, draw a card.
  - **Fix needed**: Implement conditional ETB tapped + lone-attacker trigger

- [ ] **Toph, Hardheaded Teacher** — Creature 3/4. ETB + Spell-cast trigger both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TophHardheadedTeacher.java`
  - **What it should do**: ETB Earthbend. Spell-cast trigger for counters/damage.
  - **Fix needed**: Implement Earthbend + replace spell trigger Custom

- [ ] **Toph, the Blind Bandit** — Creature 0/3. ETB + Static both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TophTheBlindBandit.java`
  - **What it should do**: ETB Earthbend. Static: power equals number of lands.
  - **Fix needed**: Implement Earthbend + "power = X" static

- [ ] **Toph, the First Metalbender** — Creature 3/3. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TophTheFirstMetalbender.java`
  - **What it should do**: Complex legendary with artifact/Equipment synergies.
  - **Fix needed**: Read Java source for full abilities

- [ ] **Trusty Boomerang** — Equipment. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TrustyBoomerang.java`
  - **What it should do**: Equipped creature gets +1/+0. ETB attach. When equipped creature deals combat damage, return this to hand.
  - **Fix needed**: Implement Equipment mechanics

- [ ] **Tundra Tank** — Vehicle 4/4. Indestructible. ETB `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TundraTank.java`
  - **What it should do**: Crew. ETB destroy target creature.
  - **Fix needed**: Implement Crew + replace ETB Custom with `Effect::Destroy`

- [ ] **Twin Blades** — Equipment. Flash, Double Strike keywords on Equipment itself (wrong). `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TwinBlades.java`
  - **What it should do**: Flash. Equipped creature gets +1/+0 and double strike. Auto-attach ETB. Equip cost.
  - **Fix needed**: Move keywords to granted abilities; implement Equipment

- [ ] **Ty Lee, Chi Blocker** — Creature 2/1. Flash. ETB `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TyLeeChiBlocker.java`
  - **What it should do**: Flash. ETB tap target creature, it doesn't untap during next untap step.
  - **Fix needed**: Replace Custom with `Effect::TapTarget` + implement untap prevention

- [ ] **Unlucky Cabbage Merchant** — Creature 2/2. ETB `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/u/UnluckyCabbageMerchant.java`
  - **What it should do**: ETB create a Food token (My Cabbages!).
  - **Fix needed**: Replace Custom with `Effect::CreateToken("Food", 1)`

- [ ] **Vindictive Warden** — Creature 2/3. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/v/VindictiveWarden.java`
  - **What it should do**: {3}: Target creature gets -1/-1 until end of turn.
  - **Fix needed**: Replace Custom with `Effect::BoostUntilEndOfTurn { power: -1, toughness: -1 }`

- [ ] **Walltop Sentries** — Creature 2/3. Reach, Deathtouch. Dies `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/w/WalltopSentries.java`
  - **What it should do**: Reach, Deathtouch. When dies, Earthbend 2.
  - **Fix needed**: Implement Earthbend

- [ ] **Wan Shi Tong, Librarian** — Creature 1/1. Flash, Flying, Vigilance. ETB `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/w/WanShiTongLibrarian.java`
  - **What it should do**: ETB draw X cards. Complex mythic.
  - **Fix needed**: Replace Custom with `Effect::DrawCards` (X-cost linked)

- [ ] **Wandering Musicians** — Creature 2/5. Attack trigger `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/w/WanderingMusicians.java`
  - **What it should do**: When attacks, put a +1/+1 counter on each Ally you control.
  - **Fix needed**: Replace Custom with mass `Effect::AddCounters` targeting Allies

- [ ] **War Balloon** — Vehicle 4/3. Flying. Static + Activated both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/w/WarBalloon.java`
  - **What it should do**: Flying. Crew. {1}: Deal 1 damage to any target.
  - **Fix needed**: Implement Crew + replace activated Custom with `Effect::DealDamage { amount: 1 }`

- [ ] **Water Tribe Captain** — Creature 3/3. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/w/WaterTribeCaptain.java`
  - **What it should do**: {5}: Allies you control get +2/+2 and gain vigilance until EOT.
  - **Fix needed**: Implement mass boost + grant vigilance

- [ ] **Watery Grasp** — Enchantment Aura. Static + Activated both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/w/WateryGrasp.java`
  - **What it should do**: Aura. Enchanted creature doesn't untap. {T}: Tap enchanted creature.
  - **Fix needed**: Implement Aura + untap prevention + tap activation

- [ ] **White Lotus Hideout** — Land. No abilities.
  - **Java source**: `Mage.Sets/src/mage/cards/w/WhiteLotusHideout.java`
  - **What it should do**: {T}: Add {C}. Has channel or other abilities.
  - **Fix needed**: Add mana ability

- [ ] **White Lotus Reinforcements** — Creature 2/3. Vigilance. `StaticEffect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/w/WhiteLotusReinforcements.java`
  - **What it should do**: Vigilance. Other Allies you control get +0/+1.
  - **Fix needed**: Implement toughness lord boost

- [ ] **Yue, the Moon Spirit** — Creature 3/3. Flying, Vigilance. Activated `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/y/YueTheMoonSpirit.java`
  - **What it should do**: Keywords work. {T}: Target creature gets -X/-0.
  - **Fix needed**: Implement variable power reduction activated ability

- [ ] **Zhao, Ruthless Admiral** — Creature 3/4. No abilities.
  - **Java source**: `Mage.Sets/src/mage/cards/z/ZhaoRuthlessAdmiral.java`
  - **What it should do**: Menace. When ETB, each opponent sacrifices a creature.
  - **Fix needed**: Add menace + ETB mass sacrifice

- [ ] **Zhao, the Moon Slayer** — Creature 2/2. Static + Activated both Custom.
  - **Java source**: `Mage.Sets/src/mage/cards/z/ZhaoTheMoonSlayer.java`
  - **What it should do**: Complex legendary with destroy/damage abilities.
  - **Fix needed**: Read Java source for full abilities

- [ ] **Zuko, Conflicted** — Creature 2/3. No abilities.
  - **Java source**: `Mage.Sets/src/mage/cards/z/ZukoConflicted.java`
  - **What it should do**: Complex legendary with multiple modes/abilities.
  - **Fix needed**: Read Java source for full abilities

- [ ] **Leaves from the Vine** — Enchantment Saga. Spell `Effect::Custom` (saga placeholder).
  - **Java source**: `Mage.Sets/src/mage/cards/l/LeavesFromTheVine.java`
  - **What it should do**: Saga with three chapters. Each chapter does something specific.
  - **Fix needed**: Implement Saga framework + chapter effects

- [ ] **The Cave of Two Lovers** — Enchantment Saga. Spell `Effect::Custom` (saga placeholder).
  - **Java source**: `Mage.Sets/src/mage/cards/t/TheCaveOfTwoLovers.java`
  - **What it should do**: Saga with three chapters.
  - **Fix needed**: Implement Saga framework + chapter effects

- [ ] **The Legend of Kuruk** — Creature. Activated `Cost::Custom("Exhaust")` + triggered `Effect::Custom`. All non-functional.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TheLegendOfKuruk.java`
  - **What it should do**: Complex legendary Avatar with Waterbend and token creation.
  - **Fix needed**: Implement Exhaust cost + Waterbend + token creation

- [ ] **The Legend of Kyoshi** — Creature. Trample, Hexproof. Spell `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TheLegendOfKyoshi.java`
  - **What it should do**: Complex legendary Avatar.
  - **Fix needed**: Read Java source for full abilities

- [ ] **The Legend of Yangchen** — Creature. Flying. Spell `Effect::Custom`.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TheLegendOfYangchen.java`
  - **What it should do**: Complex legendary Avatar.
  - **Fix needed**: Read Java source for full abilities

## Missing Cards

No cards are missing. All 231 unique non-basic-land cards from the Java set file `AvatarTheLastAirbender.java` have corresponding factory functions in `tla.rs`. Basic lands are handled via the shared `basic_lands::register()` call.

---

## Priority Fixes (Quick Wins)

These cards can be fixed with minimal effort using existing Effect variants:

1. **Lightning Strike** — Just add `Effect::DealDamage { amount: 3 }` (reprint, trivial)
2. **Firebending Lesson** — Just add `Effect::DealDamage { amount: 2 }`
3. **Sold Out** — Just add `Effect::Destroy` targeting creature/planeswalker
4. **Waterbending Lesson** — Just add `Effect::Bounce` targeting creature
5. **Zuko's Exile** — Just add `Effect::Exile` targeting nonland permanent
6. **Ozai's Cruelty** — Just add `Effect::DiscardCards { count: 2 }` targeting opponent
7. **Combustion Technique** — Just add `Effect::DealDamage { amount: 2 }`
8. **Energybending** — Just add `Effect::CounterSpell` (with noncreature filter)
9. **Fire Sages** — Replace activated Custom with `Effect::DealDamage { amount: 3 }`
10. **Flexible Waterbender** — Replace activated Custom with `Effect::TapTarget`
11. **North Pole Patrol** — Replace activated Custom with `Effect::TapTarget`
12. **Master Pakku** — Replace tapped trigger Custom with `Effect::TapTarget`
13. **Deserter's Disciple** — Replace activated Custom with `Effect::AddMana`
14. **Knowledge Seeker** — Replace dies Custom with `Effect::DrawCards { count: 1 }`
15. **Unlucky Cabbage Merchant** — Replace ETB Custom with `Effect::CreateToken("Food", 1)`
16. **Combustion Man** — Replace attack Custom with `Effect::DealDamage { amount: 3 }`
17. **Fire Navy Trebuchet** — Add activated `Effect::DealDamage { amount: 2 }`
18. **Wartime Protestors** — Add ETB `Effect::DealDamage { amount: 3 }`
19. **Yuyan Archers** — Fix: change `Effect::draw_cards(1)` to `Effect::deal_damage(1)`
20. **Waterbending Scroll** — Fix: change draw count from 1 to 2

## Systemic Fixes Needed

These engine-level changes would unblock many cards at once:

1. **Implement `Effect::Scry`** — Unblocks: Compassionate Healer, Glider Kids, Guru Pathik, Katara the Fearless, and many others
2. **Implement Earthbend mechanic** — Unblocks: Badgermole, Badgermole Cub, Earth Kingdom General, Earth Village Ruffians, Walltop Sentries, The Boulder, Bumi King, Bumi Unleashed, Haru, Earth King's Lieutenant, Toph variants, Seismic Sense, Earthbending Lesson (~15+ cards)
3. **Implement Equipment attach/equip** — Unblocks: Glider Staff, Kyoshi Battle Fan, Meteor Sword, Trusty Boomerang, Twin Blades, Bender's Waterskin
4. **Implement conditional P/T boost static** — Unblocks: Badgermole, Earth Rumble Wrestlers, First-Time Flyer, Foggy Swamp Hunters, and many lords
5. **Implement Aura mechanics** — Unblocks: Path to Redemption, Swampsnare Trap, Watery Grasp, Honest Work, Avatar Destiny
6. **Implement Learn keyword action** — Unblocks: Cat-Owl, Boomerang Basics, and many Lesson spells
7. **Implement Crew (Vehicle)** — Unblocks: Fire Nation Warship, Phoenix Fleet Airship, War Balloon, Tundra Tank, The Fire Nation Drill
8. **Implement Fight effect** — Unblocks: Earth Rumble, The Last Agni Kai, The Unagi, Mongoose Lizard
9. **Add dual-land mana abilities** — Unblocks: All 10+ common dual lands (Foggy Bottom Swamp, North Pole Gates, etc.)
10. **Make `Effect::ReturnFromGraveyard` functional** — Unblocks: Merchant of Many Hats, Cycle of Renewal, Foggy Swamp Visions
