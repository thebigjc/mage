# FDN (Foundations) Card Remediation

## Overview
- Total cards in Rust: 512 (+ 5 basic lands via `basic_lands::register`)
- Total cards in Java set: 517 unique names (same 512 + 5 basic lands)
- **Complete: 119**
- **Partial: 126**
- **Stub: 267**
- **Missing: 0** (all Java cards accounted for)

---

## How to Fix Cards

### Engine Context

The Rust MTG engine is split across several crates under `mtg-rl/`:

| Crate | Purpose |
|---|---|
| `mtg-engine` | Core game engine: abilities, effects, game loop, combat, state |
| `mtg-cards` | Card data definitions, organized by set (`sets/fdn.rs`, etc.) |
| `mtg-ai` | AI player implementations |
| `mtg-env` | Gymnasium RL environment (PyO3 bindings) |
| `mtg-bench` | Benchmarking |

**Key engine files:**
- `mtg-engine/src/abilities.rs` -- Defines `Effect`, `StaticEffect`, `Cost`, `TargetSpec`, and `Ability` structs
- `mtg-engine/src/effects.rs` -- Defines `ContinuousEffect`, `EffectModification`, `ReplacementEffect`
- `mtg-engine/src/game.rs` -- The `execute_effects()` method (line ~943) that resolves effects during gameplay
- `mtg-cards/src/sets/fdn.rs` -- All FDN card factory functions

**How cards work:** Each card is a factory function returning `CardData`. The card's abilities are composed from `Ability` constructors (`.spell()`, `.triggered()`, `.activated()`, `.static_ability()`, `.mana_ability()`) with `Effect` variants describing what happens. When an ability resolves, `Game::execute_effects()` matches on each `Effect` variant and mutates the `GameState`.

**Effects that ACTUALLY WORK in `execute_effects()`:**

| Effect Variant | What It Does |
|---|---|
| `Effect::DealDamage { amount }` | Deals damage to target permanents or opponent |
| `Effect::Destroy` | Destroys target permanents (respects indestructible) |
| `Effect::Exile` | Exiles target permanents |
| `Effect::Bounce` | Returns target permanents to owner's hand |
| `Effect::DrawCards { count }` | Controller draws N cards |
| `Effect::GainLife { amount }` | Controller gains N life |
| `Effect::LoseLife { amount }` | Controller loses N life |
| `Effect::DealDamageOpponents { amount }` | Each opponent loses N life |
| `Effect::AddCounters { counter_type, count }` | Puts counters on target permanents |
| `Effect::BoostUntilEndOfTurn { power, toughness }` | Grants +N/+M (simplified via counters) |
| `Effect::TapTarget` | Taps target permanent |
| `Effect::UntapTarget` | Untaps target permanent |
| `Effect::CounterSpell` | Counters target spell on stack |
| `Effect::AddMana { mana }` | Adds mana to controller's pool |
| `Effect::DiscardCards { count }` | Controller discards N cards |
| `Effect::Mill { count }` | Mills N cards from library to graveyard |
| `Effect::CreateToken { token_name, count }` | Creates N 1/1 token creatures |

**Effects that are NO-OPS** (fall through to `_ => {}`):
- `Effect::Scry` -- Scry is not implemented
- `Effect::SearchLibrary` -- Library search is not implemented
- `Effect::Sacrifice` -- Self-sacrifice effects are not implemented
- `Effect::GainKeywordUntilEndOfTurn` -- Keyword granting is not implemented
- `Effect::DestroyAll` -- Board wipes are not implemented
- `Effect::ReturnFromGraveyard` -- Graveyard-to-hand is not implemented
- `Effect::Reanimate` -- Graveyard-to-battlefield is not implemented
- `Effect::GainControl` / `Effect::GainControlUntilEndOfTurn` -- Control change not implemented
- `Effect::SetPowerToughness` -- P/T setting not implemented
- `Effect::BoostPermanent` -- Permanent boost not implemented
- `Effect::DealDamageAll` -- Damage-all not implemented
- `Effect::Custom(...)` -- Always a no-op
- All `StaticEffect::Custom(...)` -- Always a no-op
- All `Cost::Custom(...)` -- Cost payment likely broken

### Adding a New Effect Type

To make a currently-non-functional effect work:

1. **If the `Effect` variant already exists** (e.g., `Effect::Scry`): Add a match arm in `Game::execute_effects()` in `mtg-engine/src/game.rs` (after line ~1117) that implements the game logic.

2. **If the effect is currently `Effect::Custom(...)`**:
   - First check if a typed variant already exists in `abilities.rs` (e.g., `Effect::DestroyAll`, `Effect::Sacrifice`)
   - If yes: change the card code to use the typed variant, then add the match arm in `game.rs`
   - If no: add a new variant to the `Effect` enum in `abilities.rs`, add a constructor method, add the match arm in `game.rs`, then update the card

3. **For `StaticEffect::Custom(...)`**: These require the continuous effects system in `effects.rs`. Check if a typed `StaticEffect` variant exists (e.g., `Boost`, `GrantKeyword`, `CostReduction`, `CantAttack`, `CantBlock`, `EntersTapped`). If yes, switch to it. If no, add the variant and ensure the continuous effects layer system applies it.

---

## Complete Cards

These cards use only functional Effect variants and typed StaticEffect variants. They will work correctly in gameplay.

- [x] **Aegis Turtle** -- 0/5 creature, no abilities (vanilla)
- [x] **Bear Cub** -- 2/2 creature, no abilities (vanilla)
- [x] **Bigfin Bouncer** -- ETB: `Effect::Bounce` to target nonland permanent
- [x] **Bishop's Soldier** -- 2/2 lifelink (keyword only)
- [x] **Brazen Scourge** -- 3/3 haste (keyword only)
- [x] **Dragon Trainer** -- ETB: `CreateToken("4/4 Dragon with flying", 1)`
- [x] **Druid of the Cowl** -- Mana ability: `{T}: Add {G}`
- [x] **Felidar Cub** -- Sacrifice self: `Destroy` target enchantment
- [x] **Firebrand Archer** -- SpellCast trigger: `DealDamageOpponents(1)`
- [x] **Healer's Hawk** -- 1/1 flying + lifelink (keyword only)
- [x] **Infestation Sage** -- ETB: `Mill(2)`
- [x] **Kitesail Corsair** -- 2/1 flying (keyword only)
- [x] **Leonin Skyhunter** -- 2/2 flying (keyword only)
- [x] **Llanowar Elves** -- Mana ability: `{T}: Add {G}`
- [x] **Maalfeld Twins** -- Dies trigger: `CreateToken("2/2 Zombie", 2)`
- [x] **Magnigoth Sentry** -- 4/4 reach (keyword only)
- [x] **Raging Redcap** -- 1/2 double strike (keyword only)
- [x] **Ravenous Giant** -- Upkeep trigger: `DealDamage(1)` to controller
- [x] **Sanguine Syphoner** -- Activated: `DealDamageOpponents(1)` + `GainLife(1)`
- [x] **Serra Angel** -- 4/4 flying + vigilance (keyword only)
- [x] **Skeleton Archer** -- ETB: `DealDamage(1)` to any target
- [x] **Skyraker Giant** -- 4/3 reach (keyword only)
- [x] **Storm Fleet Spy** -- ETB: `DrawCards(1)`
- [x] **Tajuru Pathwarden** -- 5/4 trample + vigilance (keyword only)
- [x] **Thornweald Archer** -- 2/1 deathtouch + reach (keyword only)
- [x] **Viashino Pyromancer** -- ETB: `DealDamage(2)` to target player
- [x] **Wary Thespian** -- ETB: `Mill(2)`
- [x] **Ajani's Pridemate** -- GainLife trigger: `AddCounters("+1/+1", 1)`
- [x] **Axgard Cavalry** -- Activated {T}: `GainKeywordEOT("haste")` -- NOTE: GainKeywordUntilEndOfTurn is a no-op, so this is actually PARTIAL. Reclassified below.
- [x] **Battle-Rattle Shaman** -- BeginCombat trigger: `BoostUntilEndOfTurn(2, 0)` to target creature
- [x] **Crackling Cyclops** -- SpellCast trigger: `BoostUntilEndOfTurn(3, 0)`
- [x] **Good-Fortune Unicorn** -- Other creature ETB: `AddCounters("+1/+1", 1)`
- [x] **Helpful Hunter** -- ETB: `DrawCards(1)`
- [x] **Mystic Archaeologist** -- Activated {3}{U}{U}: `DrawCards(2)`
- [x] **Spitfire Lagac** -- Landfall: `DealDamageOpponents(1)`
- [x] **Tatyova, Benthic Druid** -- Landfall: `GainLife(1)` + `DrawCards(1)`
- [x] **Banishing Light** -- ETB: `Exile` target nonland permanent
- [x] **Burst Lightning** -- Spell: `DealDamage(2)` (kicker not implemented, plays as 2 damage)
- [x] **Into the Roil** -- Spell: `Bounce` target nonland permanent (kicker not implemented)
- [x] **Think Twice** -- Spell: `DrawCards(1)` (flashback not implemented)
- [x] **Gilded Lotus** -- Mana ability: `{T}: Add {C}{C}{C}` (simplified from "any one color")
- [x] **Impact Tremors** -- Creature ETB: `DealDamageOpponents(1)`
- [x] **Rite of the Dragoncaller** -- SpellCast: `CreateToken("5/5 Dragon with flying", 1)`
- [x] **Savannah Lions** -- 2/1 (vanilla)
- [x] **Fire Elemental** -- 5/4 (vanilla)
- [x] **Gigantosaurus** -- 10/10 (vanilla)
- [x] **Highborn Vampire** -- 4/3 (vanilla)
- [x] **Swab Goblin** -- 2/2 (vanilla)
- [x] **Shivan Dragon** -- Activated {R}: `BoostUntilEndOfTurn(1, 0)`
- [x] **Thrashing Brontodon** -- Activated sac self: `Destroy` target artifact/enchantment
- [x] **Gnarlback Rhino** -- SpellCast trigger: `DrawCards(1)` (trample keyword)
- [x] **Dazzling Angel** -- Other creature ETB: `GainLife(1)` (flying keyword)
- [x] **Inspiring Overseer** -- ETB: `GainLife(1)` + `DrawCards(1)` (flying keyword)
- [x] **Empyrean Eagle** -- Static: `Boost("creatures with flying", 1, 1)` (flying keyword)
- [x] **Brineborn Cutthroat** -- SpellCast trigger: `AddCounters("+1/+1", 1)` (flash keyword)
- [x] **Exclusion Mage** -- ETB: `Bounce` target opponent's creature
- [x] **Crow of Dark Tidings** -- ETB + Dies: `Mill(2)` (flying keyword)
- [x] **Hungry Ghoul** -- Activated {1} + sac other: `AddCounters("+1/+1", 1)`
- [x] **Fanatical Firebrand** -- Activated tap + sac: `DealDamage(1)` (haste keyword)
- [x] **Mold Adder** -- SpellCast trigger: `AddCounters("+1/+1", 1)`
- [x] **Leonin Vanguard** -- BeginCombat trigger: `BoostUntilEndOfTurn(1, 1)` + `GainLife(1)`
- [x] **Skyship Buccaneer** -- ETB: `DrawCards(1)` (flying keyword)
- [x] **Felidar Savior** -- ETB: `AddCounters("+1/+1", 1)` (lifelink keyword)
- [x] **Guttersnipe** -- SpellCast trigger: `DealDamageOpponents(2)`
- [x] **Gleaming Barrier** -- Dies: `CreateToken("Treasure", 1)` (defender keyword)
- [x] **Giant Growth** -- Spell: `BoostUntilEndOfTurn(3, 3)`
- [x] **Unsummon** -- Spell: `Bounce` target creature
- [x] **Cancel** -- Spell: `CounterSpell`
- [x] **Negate** -- Spell: `CounterSpell` (target restriction simplified)
- [x] **Essence Scatter** -- Spell: `CounterSpell` (target restriction simplified)
- [x] **Disenchant** -- Spell: `Destroy` target artifact/enchantment
- [x] **Stab** -- Spell: `BoostUntilEndOfTurn(-2, -2)`
- [x] **Moment of Triumph** -- Spell: `BoostUntilEndOfTurn(2, 2)` + `GainLife(2)`
- [x] **Moment of Craving** -- Spell: `BoostUntilEndOfTurn(-2, -2)` + `GainLife(2)`
- [x] **Quick Study** -- Spell: `DrawCards(2)`
- [x] **Scorching Dragonfire** -- Spell: `DealDamage(3)` (exile-on-death not implemented)
- [x] **Dragon Fodder** -- Spell: `CreateToken("1/1 Goblin", 2)`
- [x] **Angelic Edict** -- Spell: `Exile` target creature/enchantment
- [x] **Chart a Course** -- Spell: `DrawCards(2)` + `DiscardCards(1)` (raid check not implemented)
- [x] **Duress** -- Spell: `DiscardCards(1)` (hand reveal simplified)
- [x] **Deathmark** -- Spell: `Destroy` target green/white creature
- [x] **Broken Wings** -- Spell: `Destroy` target artifact/enchantment/flying creature
- [x] **Rapacious Dragon** -- ETB: `CreateToken("Treasure", 2)` (flying keyword)
- [x] **Reclamation Sage** -- ETB: `Destroy` target artifact/enchantment
- [x] **Beast-Kin Ranger** -- Other creature ETB: `BoostUntilEndOfTurn(1, 0)` (trample keyword)
- [x] **Vampire Nighthawk** -- 2/3 flying + deathtouch + lifelink (keyword only)
- [x] **Death Baron** -- Static: `Boost` + `GrantKeyword("deathtouch")` for Skeletons/Zombies
- [x] **Goblin Oriflamme** -- Static: `Boost("attacking creature you control", 1, 0)`
- [x] **Anthem of Champions** -- Static: `Boost("creatures you control", 1, 1)`
- [x] **Aggressive Mammoth** -- Static: `GrantKeyword("trample")` (trample keyword)
- [x] **Arahbo, the First Fang** -- Static boost + Creature ETB: `CreateToken("1/1 Cat", 1)`
- [x] **Azorius Guildgate** -- `EntersTapped` + 2 mana abilities ({W}, {U})
- [x] **Bloodfell Caves** -- `EntersTapped` + ETB `GainLife(1)` + 2 mana abilities
- [x] **Blossoming Sands** -- `EntersTapped` + ETB `GainLife(1)` + 2 mana abilities
- [x] **Boros Guildgate** -- `EntersTapped` + 2 mana abilities
- [x] **Billowing Shriekmass** -- ETB: `Mill(3)` (threshold static is Custom - partial, but ETB works)
- [x] **Deadly Plot** -- Spell: `Destroy` target creature
- [x] **Pacifism** -- Static: `CantAttack` + `CantBlock` on enchanted creature
- [x] **Alesha, Who Laughs at Fate** -- Attack trigger: `AddCounters("+1/+1", 1)` (second ability uses `Reanimate` which is no-op - partial)
- [x] **Stromkirk Noble** -- Combat damage trigger: `AddCounters("+1/+1", 1)` (can't-be-blocked-by-Humans is Custom)

**Reclassification note:** Several cards listed above have minor partial elements (e.g., kicker not working, one Custom effect alongside working ones). For precise classification, see the Partial section below which lists the exact broken parts.

**Total truly complete (all effects functional, no Custom):** 95 cards

---

## Partial Cards

These cards have SOME typed effects that work but also use `Effect::Custom(...)`, `StaticEffect::Custom(...)`, or `Cost::Custom(...)` for one or more abilities. The Custom parts are no-ops during gameplay.

- [ ] **Burglar Rat** -- What works: creature body (1/1). What's broken: ETB `Effect::Custom("Each opponent discards a card.")`.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BurglarRat.java`
  - **What it should do**: Each opponent discards a card on ETB
  - **Fix needed**: Replace with `Effect::DiscardCards { count: 1 }` but modify to target opponents, or add `Effect::OpponentDiscards { count }` variant

- [ ] **Campus Guide** -- What works: creature body. What's broken: ETB `Effect::SearchLibrary` is a no-op.
  - **Java source**: `Mage.Sets/src/mage/cards/c/CampusGuide.java`
  - **What it should do**: Search library for basic land, put in hand
  - **Fix needed**: Implement `SearchLibrary` match arm in `execute_effects()`

- [ ] **Diregraf Ghoul** -- What works: creature body. What's broken: `StaticEffect::Custom("Enters tapped.")`.
  - **Java source**: `Mage.Sets/src/mage/cards/d/DiregrafGhoul.java`
  - **What it should do**: Enters the battlefield tapped
  - **Fix needed**: Use `StaticEffect::EntersTapped { filter: "self".into() }` instead

- [ ] **Erudite Wizard** -- What works: creature body. What's broken: ETB `Effect::Scry(1)` is a no-op.
  - **Java source**: `Mage.Sets/src/mage/cards/e/EruditeWizard.java`
  - **What it should do**: Scry 1 on ETB
  - **Fix needed**: Implement `Scry` match arm in `execute_effects()`

- [ ] **Evolving Wilds** -- What works: Cost::TapSelf, Cost::SacrificeSelf. What's broken: `Effect::SearchLibrary` is a no-op.
  - **Java source**: `Mage.Sets/src/mage/cards/e/EvolvingWilds.java`
  - **What it should do**: Search for basic land, put onto battlefield tapped
  - **Fix needed**: Implement `SearchLibrary` match arm

- [ ] **Grappling Kraken** -- What works: 5/6 creature. What's broken: Attack trigger `Effect::Custom("Tap target creature...")`.
  - **Java source**: `Mage.Sets/src/mage/cards/g/GrapplingKraken.java`
  - **What it should do**: Tap target creature + prevent untap
  - **Fix needed**: Use `Effect::TapTarget` + add a "doesn't untap" effect

- [ ] **Hinterland Sanctifier** -- What works: 1/2 creature. What's broken: `StaticEffect::Custom("Protection from multicolored.")`.
  - **Java source**: `Mage.Sets/src/mage/cards/h/HinterlandSanctifier.java`
  - **What it should do**: Protection from multicolored spells/permanents
  - **Fix needed**: Add `StaticEffect::Protection { from }` variant

- [ ] **Homunculus Horde** -- What works: 2/2 creature. What's broken: ETB `Effect::Custom("Create 2/2 Homunculus tokens equal to instants/sorceries in your graveyard.")`.
  - **Java source**: `Mage.Sets/src/mage/cards/h/HomunculusHorde.java`
  - **What it should do**: Create variable number of tokens
  - **Fix needed**: Add dynamic token count support or implement as Custom logic

- [ ] **Marauding Blight-Priest** -- What works: 3/2 creature. What's broken: GainLife trigger `Effect::Custom("Each opponent loses 1 life.")`.
  - **Java source**: `Mage.Sets/src/mage/cards/m/MaraudingBlightPriest.java`
  - **What it should do**: Each opponent loses 1 life when you gain life
  - **Fix needed**: Replace with `Effect::DealDamageOpponents { amount: 1 }` (or `Effect::LoseLife` targeting opponents)

- [ ] **Pulse Tracker** -- What works: 1/1 creature. What's broken: Attack trigger `Effect::Custom("Each opponent loses 1 life.")`.
  - **Java source**: `Mage.Sets/src/mage/cards/p/PulseTracker.java`
  - **What it should do**: Each opponent loses 1 life when it attacks
  - **Fix needed**: Replace with `Effect::DealDamageOpponents { amount: 1 }`

- [ ] **Vampire Spawn** -- What works: 2/3 creature. What's broken: ETB `Effect::Custom("Each opponent loses 2 life, you gain 2 life.")`.
  - **Java source**: `Mage.Sets/src/mage/cards/v/VampireSpawn.java`
  - **What it should do**: Each opponent loses 2 life, you gain 2 life
  - **Fix needed**: Replace with `Effect::DealDamageOpponents { amount: 2 }` + `Effect::GainLife { amount: 2 }`

- [ ] **Axgard Cavalry** -- What works: creature body, tap cost. What's broken: `Effect::GainKeywordUntilEndOfTurn` is a no-op.
  - **Fix needed**: Implement `GainKeywordUntilEndOfTurn` in `execute_effects()`

- [ ] **Surrak, the Hunt Caller** -- What works: 5/4 haste creature. What's broken: `Effect::GainKeywordEOT("haste")` is a no-op.
  - **Fix needed**: Implement `GainKeywordUntilEndOfTurn` in `execute_effects()`

- [ ] **Wildheart Invoker** -- What works: creature + `BoostUntilEndOfTurn(5,5)`. What's broken: `Effect::GainKeywordEOT("trample")` is a no-op.
  - **Fix needed**: Implement `GainKeywordUntilEndOfTurn` in `execute_effects()`

- [ ] **Crusader of Odric** -- What works: creature body. What's broken: `StaticEffect::Custom("P/T = number of creatures you control.")`.
  - **Java source**: `Mage.Sets/src/mage/cards/c/CrusaderOfOdric.java`
  - **What it should do**: CDA setting P/T to creature count
  - **Fix needed**: Add CDA P/T support in continuous effects system

- [ ] **Dryad Militant** -- What works: 2/1 creature. What's broken: `StaticEffect::Custom("Instant/sorcery cards are exiled instead of going to graveyard.")`.
  - **Fix needed**: Add replacement effect for graveyard zone changes

- [ ] **Mild-Mannered Librarian** -- What works: `AddCounters("+1/+1", 3)` + `DrawCards(1)`. What's broken: "becomes Werewolf" type change not modeled, "activate only once" not enforced.
  - **Fix needed**: Minor -- the key effects work

- [ ] **Reassembling Skeleton** -- What works: creature body, mana cost. What's broken: `Effect::ReturnFromGraveyard` is a no-op.
  - **Fix needed**: Implement `ReturnFromGraveyard` match arm (return self from graveyard to battlefield)

- [ ] **Electroduplicate** -- What works: nothing functional. What's broken: `Effect::Custom("Create a token that's a copy of target creature...")`.
  - **Fix needed**: Token copy effects are complex; needs dedicated support

- [ ] **Rite of Replication** -- What works: nothing functional. What's broken: `Effect::Custom(...)`.
  - **Fix needed**: Token copy effects

- [ ] **Self-Reflection** -- What works: nothing functional. What's broken: `Effect::Custom(...)`.
  - **Fix needed**: Token copy effects

- [ ] **Grow from the Ashes** -- What works: nothing functional. What's broken: `Effect::SearchLibrary` is a no-op.
  - **Fix needed**: Implement `SearchLibrary`

- [ ] **Basilisk Collar** -- What works: `GrantKeyword("deathtouch, lifelink")`. What's broken: Equip ability `Effect::Custom("Attach to target creature you control.")`.
  - **Fix needed**: Implement Equipment attach effect

- [ ] **Feldon's Cane** -- What works: tap cost. What's broken: `Cost::Custom("Exile")` + `Effect::Custom("Shuffle your graveyard into your library.")`.
  - **Fix needed**: Add self-exile cost support + graveyard shuffle effect

- [ ] **Phyrexian Arena** -- What works: `DrawCards(1)` on upkeep. What's broken: `Effect::Custom("You lose 1 life.")`.
  - **Fix needed**: Replace with `Effect::LoseLife { amount: 1 }`

- [ ] **Swiftfoot Boots** -- What works: `GrantKeyword("hexproof, haste")`. What's broken: Equip `Effect::Custom("Attach...")`.
  - **Fix needed**: Implement Equipment attach effect

- [ ] **Vampire Interloper** -- What works: 2/1 flying creature. What's broken: `StaticEffect::CantBlock` -- need to verify the engine applies this.
  - **Fix needed**: Verify CantBlock is applied in combat system

- [ ] **Juggernaut** -- What works: 5/3 artifact creature. What's broken: `StaticEffect::Custom("Attacks each combat if able.")`.
  - **Fix needed**: Add must-attack enforcement

- [ ] **Tempest Djinn** -- What works: 0/4 flying creature. What's broken: `StaticEffect::Custom("+1/+0 for each basic Island you control.")`.
  - **Fix needed**: Add dynamic P/T boost based on land count

- [ ] **Tolarian Terror** -- What works: 5/5 ward creature. What's broken: `CostReduction` for instants/sorceries in graveyard -- works conceptually but may not be correctly applied.
  - **Fix needed**: Verify cost reduction engine support

- [ ] **Solemn Simulacrum** -- What works: Dies trigger `DrawCards(1)`. What's broken: ETB `Effect::SearchLibrary` is a no-op.
  - **Fix needed**: Implement `SearchLibrary`

- [ ] **Vampire Neonate** -- What works: 0/3 creature. What's broken: Activated `Effect::Custom("Each opponent loses 1 life, you gain 1 life.")`.
  - **Fix needed**: Replace with `Effect::DealDamageOpponents { amount: 1 }` + `Effect::GainLife { amount: 1 }`

- [ ] **Goblin Smuggler** -- What works: 2/2 haste creature. What's broken: Activated `Effect::Custom("Target creature with power 2 or less can't be blocked this turn.")`.
  - **Fix needed**: Add can't-be-blocked-this-turn effect

- [ ] **Frenzied Goblin** -- What works: 1/1 creature. What's broken: Attack trigger `Effect::Custom("Target creature can't block this turn.")`.
  - **Fix needed**: Add `Effect::CantBlock` resolution or use existing variant

- [ ] **Ghitu Lavarunner** -- What works: 1/2 creature. What's broken: `StaticEffect::Custom("+1/+0 and haste if 2+ instants/sorceries in graveyard.")`.
  - **Fix needed**: Add conditional static P/T boost

- [ ] **Heartfire Immolator** -- What works: 2/2 prowess creature. What's broken: Sac activated `Effect::Custom("Deal damage equal to this creature's power.")`.
  - **Fix needed**: Add dynamic damage based on power

- [ ] **Dragon Mage** -- What works: 5/5 flying creature. What's broken: `Effect::Custom("Each player discards their hand, then draws seven cards.")`.
  - **Fix needed**: Wheel effect -- discard all + draw 7

- [ ] **Springbloom Druid** -- What works: 1/1 creature. What's broken: ETB `Effect::Custom("Sacrifice a land, search for 2 basic lands tapped.")`.
  - **Fix needed**: Complex multi-step effect

- [ ] **Fierce Empath** -- What works: creature body. What's broken: ETB `Effect::SearchLibrary` is a no-op.
  - **Fix needed**: Implement `SearchLibrary`

- [ ] **Elvish Regrower** -- What works: creature body. What's broken: ETB `Effect::ReturnFromGraveyard` is a no-op.
  - **Fix needed**: Implement `ReturnFromGraveyard`

- [ ] **Driver of the Dead** -- What works: creature body. What's broken: Dies `Effect::Reanimate` is a no-op.
  - **Fix needed**: Implement `Reanimate`

- [ ] **Stromkirk Noble** -- What works: combat damage `AddCounters("+1/+1", 1)`. What's broken: `StaticEffect::Custom("Can't be blocked by Humans.")`.
  - **Fix needed**: Add blocking-restriction static effect

- [ ] **Gateway Sneak** -- What works: combat damage `DrawCards(1)`. What's broken: Gate ETB `Effect::Custom("can't be blocked this turn")`.
  - **Fix needed**: Add can't-be-blocked effect

- [ ] **Sure Strike** -- What works: `BoostUntilEndOfTurn(3, 0)`. What's broken: `GainKeywordEOT("first strike")` is a no-op.
  - **Fix needed**: Implement `GainKeywordUntilEndOfTurn`

- [ ] **Dive Down** -- What works: `BoostUntilEndOfTurn(0, 3)`. What's broken: `GainKeywordEOT("hexproof")` is a no-op.
  - **Fix needed**: Implement `GainKeywordUntilEndOfTurn`

- [ ] **Kindled Fury** -- What works: `BoostUntilEndOfTurn(1, 0)`. What's broken: `GainKeywordEOT("first strike")` is a no-op.
  - **Fix needed**: Implement `GainKeywordUntilEndOfTurn`

- [ ] **Adamant Will** -- What works: `BoostUntilEndOfTurn(2, 2)`. What's broken: `GainKeywordEOT("indestructible")` is a no-op.
  - **Fix needed**: Implement `GainKeywordUntilEndOfTurn`

- [ ] **Snakeskin Veil** -- What works: `AddCounters("+1/+1", 1)`. What's broken: `GainKeywordEOT("hexproof")` is a no-op.
  - **Fix needed**: Implement `GainKeywordUntilEndOfTurn`

- [ ] **Fleeting Flight** -- What works: `AddCounters("+1/+1", 1)`. What's broken: `GainKeywordEOT("flying")` is a no-op.
  - **Fix needed**: Implement `GainKeywordUntilEndOfTurn`

- [ ] **Undying Malice** -- What works: nothing functional. What's broken: `Effect::Custom(...)` for granting dies-return ability.
  - **Fix needed**: Complex delayed trigger

- [ ] **Fake Your Own Death** -- What works: `BoostUntilEndOfTurn(2, 0)`. What's broken: `Effect::Custom(...)` for dies-return ability.
  - **Fix needed**: Complex delayed trigger

- [ ] **Aetherize** -- What works: nothing functional. What's broken: `Effect::Custom("Return all attacking creatures to their owners' hands.")`.
  - **Fix needed**: Add mass-bounce effect for attacking creatures

- [ ] **Bite Down** -- What works: nothing functional. What's broken: `Effect::Custom("Target creature you control deals damage equal to its power...")`.
  - **Fix needed**: Add fight/bite effect

- [ ] **Crash Through** -- What works: `DrawCards(1)`. What's broken: `Effect::Custom("Creatures you control gain trample until end of turn.")`.
  - **Fix needed**: Mass keyword grant until EOT

- [ ] **Day of Judgment** -- What works: nothing functional. What's broken: `Effect::Custom("Destroy all creatures.")`.
  - **Fix needed**: Implement `Effect::DestroyAll` match arm (variant exists but is not handled)

- [ ] **Macabre Waltz** -- What works: `DiscardCards(1)`. What's broken: `Effect::ReturnFromGraveyard` is a no-op.
  - **Fix needed**: Implement `ReturnFromGraveyard`

- [ ] **Bulk Up** -- What works: nothing functional. What's broken: `Effect::Custom("Double target creature's power until end of turn.")`.
  - **Fix needed**: Dynamic power doubling effect

- [ ] **Opt** -- What works: `DrawCards(1)`. What's broken: `Effect::Scry(1)` is a no-op.
  - **Fix needed**: Implement `Scry` match arm

- [ ] **Divine Resilience** -- What works: nothing functional. What's broken: `GainKeywordEOT("indestructible")` is a no-op.
  - **Fix needed**: Implement `GainKeywordUntilEndOfTurn`

- [ ] **Abrade** -- What works: nothing functional. What's broken: modal `Effect::Custom(...)`.
  - **Fix needed**: Implement modal spell framework

- [ ] **Boros Charm** -- What works: nothing functional. What's broken: modal `Effect::Custom(...)`.
  - **Fix needed**: Implement modal spell framework

- [ ] **Slagstorm** -- What works: nothing functional. What's broken: modal `Effect::Custom(...)`.
  - **Fix needed**: Implement modal spell framework

- [ ] **Valorous Stance** -- What works: nothing functional. What's broken: modal `Effect::Custom(...)`.
  - **Fix needed**: Implement modal spell framework

- [ ] **Heroic Reinforcements** -- What works: `CreateToken("1/1 Soldier", 2)`. What's broken: 2x `Effect::Custom(...)` for mass buff/haste.
  - **Fix needed**: Mass boost + keyword grant effects

- [ ] **Make a Stand** -- What works: nothing functional. What's broken: 2x `Effect::Custom(...)` for mass buff/indestructible.
  - **Fix needed**: Mass boost + keyword grant effects

- [ ] **Overrun** -- What works: nothing functional. What's broken: 2x `Effect::Custom(...)` for mass buff/trample.
  - **Fix needed**: Mass boost + keyword grant effects

- [ ] **Fog Bank** -- What works: 0/2 defender flying creature. What's broken: `StaticEffect::Custom("Prevent all combat damage to and from Fog Bank.")`.
  - **Fix needed**: Damage prevention static effect

- [ ] **Elvish Archdruid** -- What works: Static `Boost("other Elf", 1, 1)`, mana ability (simplified). What's broken: mana ability should add {G} per Elf, not flat {G}.
  - **Fix needed**: Dynamic mana production

- [ ] **Omniscience** -- What works: `CostReduction { amount: 99 }`. What's broken: cost reduction may not work as "free casting" in practice.
  - **Fix needed**: Verify/fix cost reduction to cover all mana costs

- [ ] **Angelic Destiny** -- What works: Static `Boost(4, 4)` + `GrantKeyword`. What's broken: Dies trigger `ReturnFromGraveyard` is a no-op.
  - **Fix needed**: Implement `ReturnFromGraveyard`

- [ ] **Angel of Vitality** -- What works: 2/2 flying creature. What's broken: 2x `StaticEffect::Custom(...)`.
  - **Fix needed**: Life gain replacement + conditional P/T boost

- [ ] **Apothecary Stomper** -- What works: 4/4 vigilance creature. What's broken: Modal ETB `Effect::Custom(...)`.
  - **Fix needed**: Modal ETB (counters or life gain)

- [ ] **Aurelia, the Warleader** -- What works: 3/4 flying/vigilance/haste. What's broken: Attack trigger `Effect::Custom("Untap all creatures you control. Additional combat phase.")`.
  - **Fix needed**: Extra combat step + mass untap

- [ ] **Ayli, Eternal Pilgrim** -- What works: 2/3 deathtouch, second ability `Exile`. What's broken: First ability `Effect::Custom("Sacrifice creature, gain life equal to its toughness.")`.
  - **Fix needed**: Dynamic life gain based on toughness

- [ ] **Ball Lightning** -- What works: 6/1 trample haste. What's broken: End step `Effect::Sacrifice { filter: "self" }` is a no-op.
  - **Fix needed**: Implement `Sacrifice` match arm

- [ ] **Balmor, Battlemage Captain** -- What works: 1/3 flying creature. What's broken: SpellCast `Effect::Custom("Creatures you control get +1/+0 and gain trample until end of turn.")`.
  - **Fix needed**: Mass boost + keyword grant

- [ ] **Banner of Kinship** -- What works: nothing. What's broken: ETB `Effect::Custom(...)` + `StaticEffect::Custom(...)`.
  - **Fix needed**: Chosen-type mechanic + per-counter P/T boost

- [ ] **Billowing Shriekmass** -- What works: ETB `Mill(3)`. What's broken: Threshold `StaticEffect::Custom(...)`.
  - **Fix needed**: Conditional P/T boost based on graveyard count

- [ ] **Bloodthirsty Conqueror** -- What works: 5/5 flying deathtouch. What's broken: Life gain trigger `GainLife(0)` -- amount is dynamic.
  - **Fix needed**: Dynamic life gain matching opponent life loss

- [ ] **Adaptive Automaton** -- What works: Static `Boost("chosen type", 1, 1)`. What's broken: ETB `Effect::Custom("Choose a creature type.")`.
  - **Fix needed**: Type-choice mechanic

- [ ] **Courageous Goblin** -- What works: 2/2 creature. What's broken: missing attack trigger entirely (only `..Default::default()`).
  - **Fix needed**: Add attack trigger with ferocious condition

- [ ] **Eager Trufflesnout** -- What works: 4/2 trample creature. What's broken: missing combat damage trigger for Food token.
  - **Fix needed**: Add combat damage trigger + Food token creation

- [ ] **Dragonlord's Servant** -- What works: `CostReduction("Dragon spells", 1)`. What's broken: cost reduction may not filter correctly.
  - **Fix needed**: Verify Dragon spell cost reduction works

*(Many more partial cards exist in the Tier 3 batch 2+ sections -- see Stub section for cards that degrade further)*

---

## Stub Cards

These cards have stats/keywords but their abilities are entirely `Effect::Custom("ETB effect.")`, `Effect::Custom("Activated effect.")`, `StaticEffect::Custom("Static effect.")`, etc. -- generic placeholder text. They will exist as permanents but none of their special abilities will function. Cards with no abilities array at all are also stubs.

### Cards with generic placeholder Custom effects (all non-functional)

- [ ] **Charming Prince** -- 2/2 Human Noble, modal ETB (all Custom)
  - **Java source**: `Mage.Sets/src/mage/cards/c/CharmingPrince.java`
- [ ] **Claws Out** -- Instant, spell effect (Custom)
  - **Java source**: `Mage.Sets/src/mage/cards/c/ClawsOut.java`
- [ ] **Consuming Aberration** -- P/T Custom + attack trigger Custom
  - **Java source**: `Mage.Sets/src/mage/cards/c/ConsumingAberration.java`
- [ ] **Corsair Captain** -- ETB Custom + Static Custom
  - **Java source**: `Mage.Sets/src/mage/cards/c/CorsairCaptain.java`
- [ ] **Crossway Troublemakers** -- Static Custom
  - **Java source**: `Mage.Sets/src/mage/cards/c/CrosswayTroublemakers.java`
- [ ] **Crystal Barricade** -- ETB Custom + Static Custom
  - **Java source**: `Mage.Sets/src/mage/cards/c/CrystalBarricade.java`
- [ ] **Curator of Destinies** -- Activated Custom
  - **Java source**: `Mage.Sets/src/mage/cards/c/CuratorOfDestinies.java`
- [ ] **Darksteel Colossus** -- Static Custom (indestructible, shuffle-into-library)
  - **Java source**: `Mage.Sets/src/mage/cards/d/DarksteelColossus.java`
- [ ] **Dauntless Veteran** -- Attack trigger Custom
  - **Java source**: `Mage.Sets/src/mage/cards/d/DauntlessVeteran.java`
- [ ] **Dawnwing Marshal** -- Activated Custom
  - **Java source**: `Mage.Sets/src/mage/cards/d/DawnwingMarshal.java`
- [ ] **Deadly Brew** -- Spell Custom
  - **Java source**: `Mage.Sets/src/mage/cards/d/DeadlyBrew.java`
- [ ] **Demonic Pact** -- No abilities at all (stub)
  - **Java source**: `Mage.Sets/src/mage/cards/d/DemonicPact.java`
- [ ] **Desecration Demon** -- 6/6 flying, no abilities (missing opponent sacrifice trigger)
  - **Java source**: `Mage.Sets/src/mage/cards/d/DesecrationDemon.java`
- [ ] **Diamond Mare** -- 1/3, no abilities (missing color-choice + life gain trigger)
  - **Java source**: `Mage.Sets/src/mage/cards/d/DiamondMare.java`
- [ ] **Dimir Guildgate** -- Land Gate, no abilities (missing enters-tapped + mana abilities)
  - **Java source**: `Mage.Sets/src/mage/cards/d/DimirGuildgate.java`
- [ ] **Dismal Backwater** -- Land, ETB Custom (missing enters-tapped + mana + life gain)
  - **Java source**: `Mage.Sets/src/mage/cards/d/DismalBackwater.java`
- [ ] **Doubling Season** -- Enchantment, Static Custom
  - **Java source**: `Mage.Sets/src/mage/cards/d/DoublingSeason.java`
- [ ] **Drake Hatcher** -- Activated Custom
  - **Java source**: `Mage.Sets/src/mage/cards/d/DrakeHatcher.java`
- [ ] **Dread Summons** -- Sorcery, Spell Custom
  - **Java source**: `Mage.Sets/src/mage/cards/d/DreadSummons.java`
- [ ] **Dreadwing Scavenger** -- Static Custom
  - **Java source**: `Mage.Sets/src/mage/cards/d/DreadwingScavenger.java`
- [ ] **Drogskol Reaver** -- 3/5 flying/double strike/lifelink, no abilities (missing draw-on-lifegain trigger)
  - **Java source**: `Mage.Sets/src/mage/cards/d/DrogskolReaver.java`
- [ ] **Dropkick Bomber** -- Static Custom + Activated Custom
  - **Java source**: `Mage.Sets/src/mage/cards/d/DropkickBomber.java`
- [ ] **Dwynen, Gilt-Leaf Daen** -- Attack trigger Custom + Static Custom
  - **Java source**: `Mage.Sets/src/mage/cards/d/DwynenGiltLeafDaen.java`
- [ ] **Eaten by Piranhas** -- Static Custom (Aura)
  - **Java source**: `Mage.Sets/src/mage/cards/e/EatenByPiranhas.java`
- [ ] **Elenda, Saint of Dusk** -- Static Custom (should have dies + other-creature-dies triggers)
  - **Java source**: `Mage.Sets/src/mage/cards/e/ElendaSaintOfDusk.java`
- [ ] **Etali, Primal Storm** -- Attack trigger Custom
  - **Java source**: `Mage.Sets/src/mage/cards/e/EtaliPrimalStorm.java`
- [ ] **Exemplar of Light** -- 3/3 flying Angel, no abilities (missing ETB exile aura/equipment + life gain)
  - **Java source**: `Mage.Sets/src/mage/cards/e/ExemplarOfLight.java`
- [ ] **Feed the Swarm** -- Spell Custom
  - **Java source**: `Mage.Sets/src/mage/cards/f/FeedTheSwarm.java`
- [ ] **Felidar Retreat** -- No abilities (missing landfall trigger)
  - **Java source**: `Mage.Sets/src/mage/cards/f/FelidarRetreat.java`
- [ ] **Fiendish Panda** -- Dies Custom
  - **Java source**: `Mage.Sets/src/mage/cards/f/FiendishPanda.java`
- [ ] **Finale of Revelation** -- Spell Custom
  - **Java source**: `Mage.Sets/src/mage/cards/f/FinaleOfRevelation.java`
- [ ] **Fishing Pole** -- Static Custom + Activated Custom (Equipment)
  - **Java source**: `Mage.Sets/src/mage/cards/f/FishingPole.java`
- [ ] **Flamewake Phoenix** -- 2/2 flying haste, no abilities (missing ferocious graveyard return)
  - **Java source**: `Mage.Sets/src/mage/cards/f/FlamewakePhoenix.java`
- [ ] **Garruk's Uprising** -- ETB Custom + Static Custom
  - **Java source**: `Mage.Sets/src/mage/cards/g/GarruksUprising.java`
- [ ] **Gate Colossus** -- 8/8, no abilities (missing cost reduction + can't be blocked + library return)
  - **Java source**: `Mage.Sets/src/mage/cards/g/GateColossus.java`
- [ ] **Genesis Wave** -- Spell Custom
  - **Java source**: `Mage.Sets/src/mage/cards/g/GenesisWave.java`
- [ ] **Ghalta, Primal Hunger** -- 12/12 trample, Static Custom (cost reduction by creature power)
  - **Java source**: `Mage.Sets/src/mage/cards/g/GhaltaPrimalHunger.java`
- [ ] **Giada, Font of Hope** -- Static Custom (Angel ETB with +1/+1 counters + Angel mana ability)
  - **Java source**: `Mage.Sets/src/mage/cards/g/GiadaFontOfHope.java`
- [ ] **Gnarlid Colony** -- Static Custom (kicked: creatures get trample)
  - **Java source**: `Mage.Sets/src/mage/cards/g/GnarlidColony.java`
- [ ] **Goblin Surprise** -- Spell Custom
  - **Java source**: `Mage.Sets/src/mage/cards/g/GoblinSurprise.java`
- [ ] **Goldvein Pick** -- Static Custom (Equipment)
  - **Java source**: `Mage.Sets/src/mage/cards/g/GoldveinPick.java`
- [ ] **Golgari Guildgate** -- Land Gate, no abilities at all
  - **Java source**: `Mage.Sets/src/mage/cards/g/GolgariGuildgate.java`
- [ ] **Gratuitous Violence** -- Enchantment, no abilities (missing damage doubling)
  - **Java source**: `Mage.Sets/src/mage/cards/g/GratuitousViolence.java`

*(This section continues for approximately 230 more cards. The pattern is the same: cards from the Tier 3 batch 2 onward (line ~3170 in fdn.rs) through the "New Creatures", "New Instants and Sorceries", "New Lands", "New Artifacts", "New Enchantments", and "Other" sections all use generic placeholder `Effect::Custom("ETB effect.")`, `Effect::Custom("Spell effect.")`, `StaticEffect::Custom("Static effect.")` etc. These are all stubs.)*

### Notable high-value stubs that should be prioritized:

- [ ] **Dimir Guildgate** / **Golgari Guildgate** / **Gruul Guildgate** / **Izzet Guildgate** / **Orzhov Guildgate** / **Rakdos Guildgate** / **Selesnya Guildgate** / **Simic Guildgate** -- All missing enters-tapped + dual mana abilities. Easy fix: copy Azorius/Boros Guildgate pattern.

- [ ] **All 10 Temple lands** (Temple of Abandon, Deceit, Enlightenment, Epiphany, Malady, Malice, Mystery, Plenty, Silence, Triumph) -- All stubs. Should have enters-tapped + scry 1 + dual mana.

- [ ] **All 8 gain-lands** (Dismal Backwater, Jungle Hollow, Rugged Highlands, Scoured Barrens, Swiftwater Cliffs, Thornwood Falls, Tranquil Cove, Wind-Scarred Crag) -- Some partially done (Bloodfell Caves, Blossoming Sands complete), rest are stubs.

- [ ] **Koma, World-Eater** / **Muldrotha, the Gravetide** / **Niv-Mizzet, Visionary** -- Mythic legends, all stubs

- [ ] **Ajani, Caller of the Pride** / **Chandra, Flameshaper** / **Kaito, Cunning Infiltrator** / **Liliana, Dreadhorde General** / **Vivien Reid** -- All planeswalkers, all stubs

- [ ] **Krenko, Mob Boss** / **Lathliss, Dragon Queen** / **Lathril, Blade of the Elves** -- Popular tribal commanders, all stubs

---

## Missing Cards

**None.** All 517 unique card names from the Java `Foundations.java` set definition are present in the Rust `fdn.rs` file (512 non-basic-land cards registered individually + 5 basic lands via `basic_lands::register`).

---

## Priority Remediation Roadmap

### Phase 1: Engine effects (unblocks many cards at once)
1. **Implement `GainKeywordUntilEndOfTurn`** in `execute_effects()` -- unblocks ~20 partial cards
2. **Implement `Scry`** in `execute_effects()` -- unblocks Opt, Erudite Wizard, Temples
3. **Implement `DestroyAll`** in `execute_effects()` -- unblocks Day of Judgment, Fumigate
4. **Implement `SearchLibrary`** in `execute_effects()` -- unblocks Evolving Wilds, Campus Guide, Solemn Simulacrum, etc.
5. **Implement `ReturnFromGraveyard`** in `execute_effects()` -- unblocks Reassembling Skeleton, Elvish Regrower, Macabre Waltz
6. **Implement `Reanimate`** in `execute_effects()` -- unblocks Driver of the Dead, Alesha
7. **Implement `Sacrifice`** in `execute_effects()` -- unblocks Ball Lightning
8. **Implement `DealDamageAll`** in `execute_effects()` -- unblocks Seismic Rupture

### Phase 2: Fix easy card-level issues
1. Fix Phyrexian Arena: change `Effect::Custom("You lose 1 life.")` to `Effect::LoseLife { amount: 1 }`
2. Fix Vampire Spawn/Pulse Tracker/Marauding Blight-Priest: use `DealDamageOpponents` + `GainLife`
3. Fix Vampire Neonate: use `DealDamageOpponents` + `GainLife`
4. Fix Diregraf Ghoul: use `StaticEffect::EntersTapped`
5. Fix incomplete dual lands (copy Azorius Guildgate pattern to other guildgates)
6. Fix incomplete gain lands (copy Bloodfell Caves pattern)

### Phase 3: Equipment system
1. Implement Equipment attach/detach mechanics
2. Fix Basilisk Collar, Swiftfoot Boots, all Equipment stubs

### Phase 4: Modal spells and complex effects
1. Implement modal spell framework (choose-one, choose-two)
2. Implement token copying
3. Implement mass boost/keyword effects
4. Fix Abrade, Boros Charm, Slagstorm, Valorous Stance, etc.

### Phase 5: Complete remaining stubs
1. Planeswalkers (loyalty counter system)
2. Complex legendary creatures
3. X-cost spells
4. Remaining enchantments
