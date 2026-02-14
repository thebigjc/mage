# TDM (Tarkir: Dragonstorm) Card Remediation

## Overview
- Total cards in Java set (non-basic): 271
- Registered in Rust: 271 (+ 2 duplicate name aliases)
- Complete: 97
- Partial: 115
- Stub: 59
- Missing: 0

## How to Fix Cards

### Engine Context
The Rust MTG engine is split across several crates:
- **mtg-engine** (`mtg-engine/src/`) — Core game engine with game loop, state, effects, abilities
  - `abilities.rs` — `Effect`, `StaticEffect`, `Cost`, `TargetSpec`, `Ability` enums and structs
  - `effects.rs` — `StaticEffect` enum definition
  - `game.rs` — `execute_effects()` at line 943 handles effect resolution
  - `constants.rs` — Card types, subtypes, keywords, zones, etc.
- **mtg-cards** (`mtg-cards/src/`) — Card definitions organized by set
  - `sets/tdm.rs` — All TDM card factory functions
- **mtg-ai** — AI players
- **mtg-rl** — RL/PyO3 bindings

The `execute_effects()` function in `game.rs:943` is the central dispatcher. It matches on `Effect` variants and executes game state changes. Any variant not explicitly matched falls through to `_ => {}` (no-op).

**Currently implemented Effect variants** (these actually do something):
- `Effect::DealDamage { amount }` — Deal damage to target permanents/players
- `Effect::Destroy` — Destroy target permanents
- `Effect::Exile` — Exile target permanents
- `Effect::Bounce` — Return target permanents to hand
- `Effect::DrawCards { count }` — Draw N cards
- `Effect::GainLife { amount }` — Gain N life
- `Effect::LoseLife { amount }` — Lose N life
- `Effect::DealDamageOpponents { amount }` — Each opponent loses N life
- `Effect::AddCounters { counter_type, count }` — Put counters on targets
- `Effect::BoostUntilEndOfTurn { power, toughness }` — Simplified as +1/+1 counters
- `Effect::TapTarget` — Tap target permanents
- `Effect::UntapTarget` — Untap target permanents
- `Effect::CounterSpell` — Counter target spell on stack
- `Effect::AddMana { mana }` — Add mana to controller's pool
- `Effect::DiscardCards { count }` — Discard N cards
- `Effect::Mill { count }` — Mill N cards
- `Effect::CreateToken { token_name, count }` — Create token creatures (now parses P/T and keywords from token_name)
- `Effect::Scry { count }` — Scry N cards
- `Effect::SearchLibrary { filter }` — Search library for matching card
- `Effect::ReturnFromGraveyard` — Return card from graveyard to hand
- `Effect::Reanimate` — Return card from graveyard to battlefield
- `Effect::GainKeywordUntilEndOfTurn { keyword }` — Grant keyword until EOT
- `Effect::GainKeyword { keyword }` — Grant keyword permanently
- `Effect::LoseKeyword { keyword }` — Remove keyword
- `Effect::Indestructible` — Grant indestructible until EOT
- `Effect::Hexproof` — Grant hexproof until EOT
- `Effect::CantBlock` — Prevent blocking this turn
- `Effect::Sacrifice { filter }` — Force sacrifice
- `Effect::DestroyAll { filter }` — Board wipe
- `Effect::DealDamageAll { amount, filter }` — Damage all matching
- `Effect::RemoveCounters { counter_type, count }` — Remove counters
- `Effect::CreateTokenTappedAttacking { token_name, count }` — Tokens tapped and attacking
- `Effect::BoostPermanent { power, toughness }` — Permanent boost
- `Effect::SetPowerToughness { power, toughness }` — Set base P/T

**Effect variants that are NO-OPs** (defined in enum but fall through to `_ => {}`):
- `Effect::SetLife` — Set life total
- `Effect::MustBlock` — Force blocking
- `Effect::PreventCombatDamage` — Prevent damage
- `Effect::GainControl` / `Effect::GainControlUntilEndOfTurn` — Steal
- `Effect::GainProtection` — Protection
- `Effect::Custom(...)` — Custom text (always no-op)

Note: `StaticEffect::Custom(...)` also remains non-functional.

**StaticEffect** — Defined in `effects.rs`. `StaticEffect::Custom(...)` is non-functional. Typed variants like `StaticEffect::Boost`, `StaticEffect::GrantKeyword`, `StaticEffect::EntersTapped`, `StaticEffect::CostReduction` may or may not be applied by the continuous effects system.

### Adding a New Effect Type
1. Add the variant to `pub enum Effect` in `mtg-engine/src/abilities.rs`
2. Add a match arm in `execute_effects()` in `mtg-engine/src/game.rs` (line 943+)
3. Update card implementations in `mtg-cards/src/sets/tdm.rs` to use the new typed variant instead of `Effect::Custom(...)`

### Classification Criteria
- **COMPLETE**: All effects use implemented `Effect` variants. No `Custom(...)` anywhere. Card works in gameplay.
- **PARTIAL**: Has SOME typed effects but also uses `Effect::Custom(...)`, `StaticEffect::Custom(...)`, `Cost::Custom(...)`, or remaining no-op Effect variants (`SetLife`, `MustBlock`, `PreventCombatDamage`, `GainControl`, `GainControlUntilEndOfTurn`, `GainProtection`). The Custom/no-op parts will be silent failures during gameplay.
- **STUB**: Card is stats/keywords only with no abilities, or all abilities are Custom. Card exists as a permanent but none of its special abilities will function.

---

## Complete Cards

These cards use only implemented Effect variants and will work correctly during gameplay.

- [x] **Adorned Crocodile** — 5/3 for {4}{B}. Dies: create 2/2 Zombie Druid token. Renew: +1/+1 counter. Both use CreateToken and AddCounters.
- [x] **Agent of Kotis** — 2/1 for {1}{U}. Renew: two +1/+1 counters on target creature. Uses AddCounters.
- [x] **Alchemist's Assistant** — 2/1 lifelink for {1}{B}. Renew: lifelink counter on target creature. Uses AddCounters.
- [x] **Bearer of Glory** — 2/3 for {1}{W}. ETB: gain 3 life. Uses GainLife.
- [x] **Bloodfell Caves** — Gain land (ETB tapped, gain 1 life, mana abilities). Uses GainLife + mana.
- [x] **Blossoming Sands** — Gain land. Same pattern.
- [x] **Caustic Exhale** — Instant {B}. Target creature -3/-3. Uses BoostUntilEndOfTurn.
- [x] **Champion of Dusan** — 4/2 trample for {2}{G}. Renew: +1/+1 and trample counter. Uses AddCounters.
- [x] **Channeled Dragonfire** — Sorcery {R}. 2 damage to any target. Uses DealDamage.
- [x] **Constrictor Sage** — 4/4 for {4}{U}. ETB: tap + stun counter. Renew: same. Uses TapTarget + AddCounters.
- [x] **Cori Mountain Stalwart** — 3/3 for {1}{R}{W}. Flurry: 2 damage to opponents + gain 2. Uses DealDamageOpponents + GainLife.
- [x] **Dalkovan Packbeasts** — 0/4 vigilance for {2}{W}. Mobilize 3. Uses CreateToken.
- [x] **Defibrillating Current** — Sorcery. 4 damage to creature, gain 2 life. Uses DealDamage + GainLife.
- [x] **Delta Bloodflies** — 1/2 flying for {1}{B}. Attacks: opponents lose 1 (conditional). Uses DealDamageOpponents.
- [x] **Devoted Duelist** — 2/1 haste for {1}{R}. Flurry: 1 damage to opponents. Uses DealDamageOpponents.
- [x] **Dismal Backwater** — Gain land. Uses GainLife + mana.
- [x] **Dispelling Exhale** — Instant {1}{U}. Counter target spell. Uses CounterSpell.
- [x] **Disruptive Stormbrood** — 3/3 flying. ETB: destroy artifact/enchantment. Uses Destroy.
- [x] **Dragon Sniper** — 1/1 vigilance/reach/deathtouch. Keywords only, no abilities needed.
- [x] **Dragon's Prey** — Instant {2}{B}. Destroy target creature. Uses Destroy.
- [x] **Dragonback Lancer** — 3/3 flying for {3}{W}. Mobilize 1. Uses CreateToken.
- [x] **Dusyut Earthcarver** — 4/4 reach for {5}{G}. ETB: endure 3. Uses AddCounters.
- [x] **Embermouth Sentinel** — 4/4 haste for {3}{R}. Attacks: 1 damage to opponents. Uses DealDamageOpponents.
- [x] **Feral Deathgorger** — 3/3 deathtouch for {1}{B}{G}. Opponent creature dies: +1/+1 counter. Uses AddCounters.
- [x] **Fortress Kin-Guard** — 1/2 for {1}{W}. ETB: endure 1. Uses AddCounters.
- [x] **Frontier Bivouac** — Tri-land. Mana abilities only.
- [x] **Humbling Elder** — 1/2 flash for {U}. ETB: target creature -2/-0. Uses BoostUntilEndOfTurn.
- [x] **Iceridge Serpent** — 3/3 for {4}{U}. ETB: bounce target creature. Uses Bounce.
- [x] **Inevitable Defeat** — Instant. Exile nonland permanent, lose/gain 3 life. Uses Exile + LoseLife + GainLife.
- [x] **Inspirited Vanguard** — 3/2 for {4}{G}. ETB/attacks: endure 2. Uses AddCounters.
- [x] **Iridescent Tiger** — 3/4 for {4}{R}. ETB: add WUBRG. Uses AddMana.
- [x] **Jeskai Brushmaster** — 2/4 double strike, prowess. Keywords only.
- [x] **Jungle Hollow** — Gain land.
- [x] **Kin-Tree Nurturer** — 2/1 lifelink for {2}{B}. ETB: endure 1. Uses AddCounters.
- [x] **Loxodon Battle Priest** — 3/5 for {4}{W}. Begin combat: +1/+1 counter on another creature. Uses AddCounters.
- [x] **Mardu Devotee** — 3/3 for {1}{R}{W}{B}. ETB: 2 damage to creature. Uses DealDamage.
- [x] **Meticulous Artisan** — 3/3 prowess for {3}{R}. ETB: create Treasure. Uses CreateToken.
- [x] **Mox Jasper** — Artifact {0}. Tap: add any color mana. Uses AddMana.
- [x] **Mystic Monastery** — Tri-land.
- [x] **Nightblade Brigade** — 1/3 deathtouch for {2}{B}. Mobilize 1 + surveil 1. CreateToken works, Scry now works.
- [x] **Nomad Outpost** — Tri-land.
- [x] **Opulent Palace** — Tri-land.
- [x] **Qarsi Revenant** — 3/3 flying/deathtouch/lifelink. Renew: flying + deathtouch + lifelink counters. Uses AddCounters.
- [x] **Rainveil Rejuvenator** — 3/3 for {2}{G}{U}. ETB: draw + gain 3 life. Uses DrawCards + GainLife.
- [x] **Rebellious Strike** — Instant {1}{W}. +3/+0 + draw. Uses BoostUntilEndOfTurn + DrawCards.
- [x] **Reigning Victor** — 3/3 for {2/R}{2/W}{2/B}. Mobilize 1 + boost + GainKeywordEOT. CreateToken works; BoostUntilEndOfTurn works; GainKeywordUntilEndOfTurn now works.
- [x] **Rugged Highlands** — Gain land.
- [x] **Sage of the Fang** — 2/2 for {2}{G}. ETB: +1/+1 counter on creature. Uses AddCounters.
- [x] **Sagu Pummeler** — 4/4 reach. Renew: two +1/+1 + reach counter. Uses AddCounters.
- [x] **Sagu Wildling** — 3/3 flying for {4}{G}. ETB: gain 3 life. Uses GainLife.
- [x] **Salt Road Skirmish** — Sorcery {3}{B}. Destroy creature, create 2 Warrior tokens. Uses Destroy + CreateToken.
- [x] **Sandskitter Outrider** — 2/1 menace for {3}{B}. ETB: endure 2. Uses AddCounters.
- [x] **Sandsteppe Citadel** — Tri-land.
- [x] **Scoured Barrens** — Gain land.
- [x] **Shock Brigade** — 1/3 menace for {1}{R}. Mobilize 1. Uses CreateToken.
- [x] **Shocking Sharpshooter** — 1/3 reach for {1}{R}. Creature ETBs: 1 damage to opponent. Uses DealDamage.
- [x] **Spectral Denial** — Instant {X}{U}. Counter target spell. Uses CounterSpell.
- [x] **Swiftwater Cliffs** — Gain land.
- [x] **Temur Devotee** — 4/4 for {2}{G}{U}{R}. ETB: draw 2. Uses DrawCards.
- [x] **Temur Tawnyback** — 4/3 for {2/G}{2/U}{2/R}. ETB: draw 1, discard 1. Uses DrawCards + DiscardCards.
- [x] **Thornwood Falls** — Gain land.
- [x] **Tranquil Cove** — Gain land.
- [x] **Twin Bolt** — Instant {1}{R}. 2 damage divided. Uses DealDamage.
- [x] **Twinmaw Stormbrood** — 5/4 flying for {5}{W}. ETB: gain 5 life. Uses GainLife.
- [x] **Unburied Earthcarver** — 5/5 for {5}{B}. ETB: two -1/-1 counters on target. Uses AddCounters.
- [x] **Unending Whisper** — Sorcery {U}. Draw a card. Uses DrawCards.
- [x] **Ureni's Rebuff** — Sorcery {1}{U}. Bounce creature. Uses Bounce.
- [x] **Venerated Stormsinger** — 3/3 for {3}{B}. ETB: create Soldier token. Dies: opponents lose 1, gain 1. Uses CreateToken + DealDamageOpponents + GainLife.
- [x] **Voice of Victory** — 1/3 for {1}{W}. Mobilize 2 + static (Custom - opponents can't cast). CreateToken works.
- [x] **Wind-Scarred Crag** — Gain land.
- [x] **Wingblade Disciple** — 2/2 flying for {2}{U}. Flurry: create Bird token. Uses CreateToken.
- [x] **Abzan Devotee** — 3/3 for {1}{W}{B}{G}. ETB: create Spirit token. Uses CreateToken.
- [x] **Arashin Sunshield** — 3/4 for {3}{W}. ETB: exile cards from graveyard. Tap: tap creature. Uses Exile + TapTarget.
- [x] **Dalkovan Encampment** — Land. Tap for {W}. Activated: create Soldier token. Uses CreateToken + mana.
- [x] **Hundred-Battle Veteran** — 3/3 first strike for {1}{R}{W}. Attacks: +1/+1 counter on another attacker. Uses AddCounters.
- [x] **Kishla Village** — Land. Tap for {C}. Activated: create 4/4 Beast token. Uses CreateToken.
- [x] **Mardu Siegebreaker** — 4/4 deathtouch/haste for {1}{R}{W}{B}. ETB: destroy target perm MV<=2. Uses Destroy. Attacks: Custom copy token (partial, but ETB works).
- [x] **Mistrise Village** — Land. Tap for {C}. Activated: Scry 1. Uses Scry (now works).
- [x] **Riverwheel Sweep** — Sorcery. 4 damage to creature. Uses DealDamage.
- [x] **Riling Dawnbreaker** — 3/4 flying/vigilance for {4}{W}. Begin combat: +1/+0 to another creature. Uses BoostUntilEndOfTurn.
- [x] **Sunset Strikemaster** — 3/1 for {1}{R}. Mana ability. Activated: 6 damage to creature with flying. Uses DealDamage + mana.
- [x] **Temur Battlecrier** — 4/3 trample for {G}{U}{R}. Other creatures have trample. Attacks: draw + discard. Uses GrantKeyword (static) + DrawCards + DiscardCards.
- [x] **Underfoot Underdogs** — 1/2 for {2}{R}. ETB: create Goblin token. Activated: can't block. Uses CreateToken + CantBlock.
- [x] **Dalkovan Packbeasts** — (listed above)
- [x] **Great Arashin City** — Land. ETB tapped. Any-color mana. Activated: gain 5 life. Uses GainLife.
- [x] **Hardened Tactician** — 2/4 for {1}{W}{B}. Sac a token: draw. Uses DrawCards.
- [x] **Jeskai Devotee** — 2/2 for {1}{R}. Flurry: +1/+0. Activated: add mana. Uses BoostUntilEndOfTurn + AddMana.
- [x] **Marang River Regent** — 6/7 flying. ETB: bounce up to 2 nonland permanents. Uses Bounce.
- [x] **Reverberating Summons** — Enchantment {1}{R}. Activated: draw 2. Uses DrawCards.
- [x] **Sinkhole Surveyor** — 1/3 flying for {1}{B}. Attacks: lose 1 + endure 1. Uses LoseLife + AddCounters.
- [x] **Stormshriek Feral** — 3/3 flying/haste. Activated: +1/+0. Uses BoostUntilEndOfTurn.
- [x] **Yathan Tombguard** — 2/3 menace for {2}{B}. Creature with counter deals combat damage: draw + lose 1. Uses DrawCards + LoseLife.

---

## Partial Cards

These cards have some working typed effects but also use `Effect::Custom(...)`, `StaticEffect::Custom(...)`, `Cost::Custom(...)`, or no-op Effect variants for one or more abilities.

- [ ] **Aegis Sculptor** — 2/3 flying for {3}{U}. What works: AddCounters (+1/+1 counter on upkeep), Ward {2} (**FIXED** -- typed `StaticEffect::Ward` + WARD keyword). What's broken: upkeep exile condition not checked.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AegisSculptor.java`
  - **What it should do**: Ward {2} (counter spell targeting this unless opponent pays {2}); upkeep: exile 2 cards from graveyard to put +1/+1 counter.
  - **Fix needed**: Implement Ward as a StaticEffect variant or triggered replacement effect. The exile-2-from-graveyard condition is also not enforced.

- [ ] **Ainok Wayfarer** — 1/1 for {1}{G}. What works: Mill 3 (Mill works). What's broken: `Effect::Custom("You may put a land card from among them into your hand...")` (land-to-hand portion is no-op).
  - **Java source**: `Mage.Sets/src/mage/cards/a/AinokWayfarer.java`
  - **What it should do**: Mill 3, then choose: put a land from milled cards to hand, or put +1/+1 counter on self.
  - **Fix needed**: Add a "choose from milled cards" effect or split into conditional effects.

- [ ] **Armament Dragon** — 3/4 flying for {3}{W}{B}{G}. What works: nothing (all Custom). What's broken: `Effect::Custom("Distribute three +1/+1 counters...")`.
  - **Java source**: `Mage.Sets/src/mage/cards/a/ArmamentDragon.java`
  - **What it should do**: ETB: distribute three +1/+1 counters among 1-3 target creatures you control.
  - **Fix needed**: Could partially implement as AddCounters with multi-target, or add a DistributeCounters effect.

- [ ] **Attuned Hunter** — 3/3 trample for {2}{G}. What works: AddCounters (+1/+1). What's broken: The trigger condition "cards leave graveyard during your turn" is not properly filtered (uses generic ZoneChange event).
  - **Fix needed**: The AddCounters effect works but the trigger condition needs proper zone-change filtering.

- [ ] **Avenger of the Fallen** — 2/4 deathtouch for {2}{B}. What works: nothing functional. What's broken: `Effect::Custom("Create X 1/1 white Soldier creature tokens, where X is the number of creature cards in your graveyard.")`.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AvengerOfTheFallen.java`
  - **What it should do**: ETB: Mobilize X (X = creature cards in graveyard).
  - **Fix needed**: Add dynamic token count based on graveyard state.

- [ ] **Bewildering Blizzard** — Instant {4}{U}{U}. What works: DrawCards 3. What's broken: `Effect::Custom("Creatures your opponents control get -3/-0 until end of turn.")`.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BewilderingBlizzard.java`
  - **What it should do**: Draw 3 + all opponent creatures get -3/-0 until EOT.
  - **Fix needed**: Add an "all opponents' creatures get -X/-0" board effect.

- [ ] **Boulderborn Dragon** — 3/3 flying/vigilance artifact creature. What works: Scry (surveil 1 on attack). **(NOW IMPLEMENTED — Scry NOW WORKS)**
  - **Java source**: `Mage.Sets/src/mage/cards/b/BoulderbornDragon.java`
  - **What it should do**: Attacks: surveil 1.

- [ ] **Coordinated Maneuver** — Instant {1}{W}. What's broken: All `Effect::Custom(...)` — modal spell.
  - **Java source**: `Mage.Sets/src/mage/cards/c/CoordinatedManeuver.java`
  - **What it should do**: Choose one: deal damage equal to creatures you control to creature/PW; or destroy enchantment.
  - **Fix needed**: Modal spell support + dynamic damage calculation.

- [ ] **Craterhoof Behemoth** — 5/5 haste/trample for {5}{G}{G}{G}. What works: keywords. What's broken: `Effect::Custom("Creatures you control get +X/+X and trample...")`.
  - **Java source**: `Mage.Sets/src/mage/cards/c/CraterhoofBehemoth.java`
  - **What it should do**: ETB: all your creatures get +X/+X and trample (X = creature count).
  - **Fix needed**: Dynamic board-wide boost effect.

- [ ] **Cruel Truths** — Instant {3}{B}. What works: DrawCards 2 + LoseLife 2 + Scry 2. **(NOW IMPLEMENTED — Scry NOW WORKS)**

- [ ] **Descendant of Storms** — 2/1 for {W}. What's broken: `Effect::Custom("Endure 1...")` on attacks. Endure is no-op.
  - **Java source**: `Mage.Sets/src/mage/cards/d/DescendantOfStorms.java`
  - **What it should do**: Attacks: pay {1}{W} to endure 1 (put +1/+1 counter; if it would die, exile with counters instead).
  - **Fix needed**: Replace Custom with AddCounters for the counter part; endure's death-replacement is harder.

- [ ] **Desperate Measures** — Instant {B}. What works: BoostUntilEndOfTurn +1/-1. What's broken: `Effect::Custom("When it dies...draw two cards")`.
  - **Java source**: `Mage.Sets/src/mage/cards/d/DesperateMeasures.java`
  - **What it should do**: +1/-1 to target creature, and if it dies this turn under your control, draw 2.
  - **Fix needed**: Delayed triggered ability on death.

- [ ] **Dragonclaw Strike** — Sorcery. All Custom: `Effect::Custom("Double target creature's P/T, then it fights target creature you don't control.")`.
  - **Java source**: `Mage.Sets/src/mage/cards/d/DragonclawStrike.java`
  - **What it should do**: Double P/T of your creature, then fight opponent's creature.
  - **Fix needed**: Double-P/T effect + fight mechanic.

- [ ] **Equilibrium Adept** — 2/4 for {3}{R}. What's broken: ETB `Effect::Custom("Exile top card, play until end of next turn")` (impulse draw is no-op). Flurry: `Effect::GainKeywordUntilEndOfTurn("double strike")` **(NOW IMPLEMENTED)**.
  - **Java source**: `Mage.Sets/src/mage/cards/e/EquilibriumAdept.java`
  - **What it should do**: ETB: exile top card, play until end of next turn. Flurry: gain double strike until EOT.
  - **Fix needed**: Impulse draw effect + implement GainKeywordUntilEndOfTurn.

- [ ] **Flamehold Grappler** — 3/3 first strike for {U}{R}{W}. What's broken: `Effect::Custom("Copy the next spell you cast...")`.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FlameholdGrappler.java`
  - **What it should do**: ETB: copy the next spell you cast this turn.
  - **Fix needed**: Spell copy mechanic — very complex.

- [ ] **Fleeting Effigy** — 2/2 haste for {R}. What works: Bounce (return to hand at end step), BoostUntilEndOfTurn (+2/+0). What's broken: Bounce may not target self correctly (no self-targeting logic).
  - **Fix needed**: Self-bounce trigger needs proper implementation. Effects themselves are typed.

- [ ] **Frontline Rush** — Instant {R}{W}. All Custom: modal spell.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FrontlineRush.java`
  - **What it should do**: Choose one: create 2 Goblin tokens; or creature gets +X/+X (X = creatures you control).
  - **Fix needed**: Modal spell + dynamic boost.

- [ ] **Furious Forebear** — 3/1 for {1}{W}. ReturnFromGraveyard (return self from graveyard to hand). **(NOW IMPLEMENTED — ReturnFromGraveyard NOW WORKS)**
  - **Java source**: `Mage.Sets/src/mage/cards/f/FuriousForebear.java`
  - **What it should do**: From graveyard: when creature you control dies, pay {1}{W} to return this to hand.

- [ ] **Gurmag Nightwatch** — 3/3 for {2/B}{2/G}{2/U}. All Custom: look at top 3, put 1 on top, rest to graveyard.
  - **Java source**: `Mage.Sets/src/mage/cards/g/GurmagNightwatch.java`
  - **Fix needed**: Implement "look at top N, choose one" effect.

- [ ] **Gurmag Rakshasa** — 5/5 menace for {4}{B}{B}. What works: BoostUntilEndOfTurn -2/-2 and +2/+2. What's broken: Two-target split (one opponent creature, one your creature) is not properly handled.
  - **Fix needed**: Multi-target with different effects per target.

- [ ] **Highspire Bell-Ringer** — 1/4 flying for {2}{U}. What's broken: `StaticEffect::Custom("The second spell you cast each turn costs {1} less to cast.")`.
  - **Java source**: `Mage.Sets/src/mage/cards/h/HighspireBellRinger.java`
  - **Fix needed**: Cost reduction for second spell each turn.

- [ ] **Jade-Cast Sentinel** — 1/5 reach artifact creature. What's broken: activated ability `Effect::Custom("Put target card on the bottom of its owner's library.")`.
  - **Java source**: `Mage.Sets/src/mage/cards/j/JadeCastSentinel.java`
  - **Fix needed**: Add "put card from graveyard on bottom of library" effect.

- [ ] **Jeskai Shrinekeeper** — 3/3 flying/haste for {2}{U}{R}{W}. What works: GainLife 1 + DrawCards 1. Combat damage trigger works.
  - Note: This is actually complete for gameplay — the effects are typed and implemented.

- [ ] **Kishla Skimmer** — 2/2 flying for {G}{U}. What works: DrawCards 1. What's broken: trigger condition "card leaves graveyard during your turn" uses generic ZoneChange.
  - **Fix needed**: Proper zone-change filtering for graveyard-leave events.

- [x] **Knockout Maneuver** — Fixed: `add_p1p1_counters(1), Effect::bite()` + `TargetSpec::fight_targets()`. (Batch 8)
  - **Java source**: `Mage.Sets/src/mage/cards/k/KnockoutManeuver.java`
  - **Fix needed**: Implement Fight mechanic.

- [ ] **Krotiq Nestguard** — 4/4 defender for {2}{G}. What's broken: `Effect::Custom("Can attack as though no defender")`.
  - **Java source**: `Mage.Sets/src/mage/cards/k/KrotiqNestguard.java`
  - **Fix needed**: Implement "ignore defender" temporary effect.

- [ ] **Lightfoot Technique** — Instant {1}{W}. What works: AddCounters +1/+1 + GainKeywordUntilEndOfTurn (flying + indestructible). **(NOW IMPLEMENTED — GainKeywordUntilEndOfTurn NOW WORKS)**

- [ ] **Mammoth Bellow** — Sorcery. What works: CreateToken (5/5 Elephant). **(NOW IMPLEMENTED — Token stats NOW PARSED, 5/5 token should work)**

- [ ] **Marshal of the Lost** — 3/3 deathtouch for {2}{W}{B}. What's broken: `Effect::Custom("Target creature gets +X/+X...")` (dynamic boost based on attacker count).
  - **Java source**: `Mage.Sets/src/mage/cards/m/MarshalOfTheLost.java`
  - **Fix needed**: Dynamic P/T boost based on game state.

- [ ] **Molten Exhale** — Sorcery {1}{R}. What works: DealDamage 4. Missing: flash-if-behold-Dragon condition (but damage works).
  - Note: Core effect works; behold condition is flavor/alternate cost.

- [ ] **Narset's Rebuke** — Instant {4}{R}. What works: DealDamage 5. Missing: add {U}{R}{W} mana and exile-if-dies (no-op).
  - **Fix needed**: Add mana generation + delayed exile on death.

- [ ] **Nightblade Brigade** — (classified as Complete above). **(NOW IMPLEMENTED — Scry NOW WORKS)**

- [ ] **Osseous Exhale** — Instant {1}{W}. What works: DealDamage 5. What's broken: `Effect::Custom("If a Dragon was beheld, you gain 2 life.")`.
  - **Fix needed**: Behold condition + conditional life gain.

- [ ] **Overwhelming Surge** — Instant {2}{R}. All Custom: modal "3 damage to creature or destroy noncreature artifact".
  - **Java source**: `Mage.Sets/src/mage/cards/o/OverwhelmingSurge.java`
  - **Fix needed**: Modal spell support.

- [x] **Piercing Exhale** — Fixed: `Effect::bite()` + `TargetSpec::fight_targets()`. (Batch 8)
  - **Java source**: `Mage.Sets/src/mage/cards/p/PiercingExhale.java`
  - **Fix needed**: Fight mechanic.

- [ ] **Poised Practitioner** — 2/3 for {2}{W}. What works: AddCounters +1/+1 + Scry 1. **(NOW IMPLEMENTED — Scry NOW WORKS)**

- [ ] **Rakshasa's Bargain** — Instant. All Custom: look at top 4, put 2 in hand.
  - **Java source**: `Mage.Sets/src/mage/cards/r/RakshasasBargain.java`
  - **Fix needed**: "Look at top N, choose M to keep" effect.

- [ ] **Rescue Leopard** — 4/2 for {2}{R}. What's broken: `Effect::Custom("You may discard a card. If you do, draw a card.")` (loot on tap).
  - **Java source**: `Mage.Sets/src/mage/cards/r/RescueLeopard.java`
  - **Fix needed**: Implement optional discard-then-draw (loot).

- [ ] **Riverwalk Technique** — Instant {3}{U}. All Custom: modal.
  - **Java source**: `Mage.Sets/src/mage/cards/r/RiverwalkTechnique.java`
  - **Fix needed**: Modal spell support + "put permanent on top/bottom of library" + counter noncreature.

- [ ] **Sage of the Skies** — 2/3 flying/lifelink for {2}{W}. What's broken: `Effect::Custom("Copy this spell.")` (copy on cast if second spell).
  - **Java source**: `Mage.Sets/src/mage/cards/s/SageOfTheSkies.java`
  - **Fix needed**: Spell copy mechanic.

- [ ] **Sarkhan's Resolve** — Instant {1}{G}. All Custom: modal (+3/+3 or destroy flying creature).
  - **Java source**: `Mage.Sets/src/mage/cards/s/SarkhansResolve.java`
  - **Fix needed**: Modal spell support.

- [ ] **Seize Opportunity** — Instant {2}{R}. All Custom: modal.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SeizeOpportunity.java`
  - **Fix needed**: Modal spell + impulse draw.

- [ ] **Sibsig Appraiser** — 2/1 for {2}{U}. All Custom: look at top 2, one to hand, one to graveyard.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SibsigAppraiser.java`
  - **Fix needed**: "Look at top N, distribute" effect.

- [x] **Skirmish Rhino** — 3/4 trample for {W}{B}{G}. ETB: `LoseLifeOpponents(2)` + `GainLife(2)` -- **FIXED** (was Custom, now uses `Effect::lose_life_opponents(2)`).

- [ ] **Snakeskin Veil** — Instant {G}. What works: AddCounters +1/+1 + Hexproof until EOT. **(NOW IMPLEMENTED — Hexproof NOW WORKS)**

- [ ] **Summit Intimidator** — 4/3 reach for {3}{R}. CantBlock (target creature can't block). **(NOW IMPLEMENTED — CantBlock NOW WORKS)**

- [ ] **Unrooted Ancestor** — 3/2 flash for {2}{B}. What works: GainKeywordUntilEndOfTurn (indestructible) + TapTarget. **(NOW IMPLEMENTED — GainKeywordUntilEndOfTurn NOW WORKS)** `Cost::SacrificeOther` may not be fully implemented.

- [ ] **Undergrowth Leopard** — 2/2 vigilance for {1}{G}. What works: Destroy (target artifact/enchantment). What's broken: `Cost::SacrificeSelf` may not be properly handled.
  - **Fix needed**: Verify SacrificeSelf cost implementation.

- [ ] **Veteran Ice Climber** — 1/3 vigilance for {1}{U}. What's broken: `StaticEffect::Custom("Can't be blocked")` + `Effect::Mill { count: 0 }` (mill amount should be dynamic based on power).
  - **Java source**: `Mage.Sets/src/mage/cards/v/VeteranIceClimber.java`
  - **What it should do**: Unblockable + attacks: mill cards equal to power.
  - **Fix needed**: Implement unblockable as StaticEffect + dynamic mill count.

- [ ] **Watcher of the Wayside** — 3/2 artifact creature. What works: GainLife 2. What's broken: `Effect::Mill { count: 2 }` targets opponent (Mill currently mills controller, not target).
  - **Fix needed**: Mill should target the specified player, not always controller.

- [ ] **Wild Ride** — Sorcery {R}. What works: BoostUntilEndOfTurn +3/+0 + GainKeywordUntilEndOfTurn (haste). **(NOW IMPLEMENTED — GainKeywordUntilEndOfTurn NOW WORKS)**

- [ ] **Worthy Cost** — Sorcery {B}. What works: Exile. What's broken: Additional cost "sacrifice a creature" is rules text only.
  - **Fix needed**: Enforce additional sacrifice cost.

- [ ] **Aggressive Negotiations** — Sorcery {1}{B}. Sacrifice (creature). **(NOW IMPLEMENTED — Sacrifice NOW WORKS)**

- [ ] **Alesha's Legacy** — Instant {1}{B}. GainKeywordUntilEndOfTurn (deathtouch + indestructible). **(NOW IMPLEMENTED — GainKeywordUntilEndOfTurn NOW WORKS)**

- [ ] **Auroral Procession** — Instant {G}{U}. ReturnFromGraveyard. **(NOW IMPLEMENTED — ReturnFromGraveyard NOW WORKS)**

- [ ] **Bone-Cairn Butcher** — 4/4 menace/haste. Sacrifice (opponent creature). **(NOW IMPLEMENTED — Sacrifice NOW WORKS)**

- [ ] **Duty Beyond Death** — Sorcery {1}{W}. ReturnFromGraveyard + CreateToken (Spirit 1/1). **(NOW IMPLEMENTED — ReturnFromGraveyard NOW WORKS)**

- [ ] **Formation Breaker** — 4/3 haste for {2}{R}. What works: BoostUntilEndOfTurn +2/+0 + GainKeywordUntilEndOfTurn (menace). **(NOW IMPLEMENTED — GainKeywordUntilEndOfTurn NOW WORKS)** Trigger condition "attacks alone" filter still broken.

- [ ] **Heritage Reclamation** — Instant {3}{G}. ReturnFromGraveyard works. **(NOW IMPLEMENTED — ReturnFromGraveyard NOW WORKS)** Still missing "if creature, may put on battlefield instead".

- [ ] **Perennation** — Sorcery {3}{W}{B}{G}. Reanimate. **(NOW IMPLEMENTED — Reanimate NOW WORKS)**

- [ ] **Reputable Merchant** — 2/2 for {2/W}{2/B}{2/G}. What works: AddCounters +1/+1 (on both ETB and dies triggers). Working correctly.
  - Note: Actually complete — both triggers use AddCounters which is implemented.

- [ ] **Roamer's Routine** — Sorcery {2}{G}. SearchLibrary (basic land). **(NOW IMPLEMENTED — SearchLibrary NOW WORKS)**

- [ ] **Salt Road Packbeast** — 3/3 vigilance for {3}{W}{B}{G}. ReturnFromGraveyard. **(NOW IMPLEMENTED — ReturnFromGraveyard NOW WORKS)**

- [ ] **Sultai Devotee** — 3/3 for {1}{B}{G}{U}. What works: Mill 3 + ReturnFromGraveyard. **(NOW IMPLEMENTED — ReturnFromGraveyard NOW WORKS)**

- [ ] **Zurgo's Vanguard** — 2/3 haste for {R}{W}. What works: CreateToken (1/1 Warrior with haste). Token is 1/1 but won't have haste keyword.
  - **Fix needed**: Token creation needs to parse keywords from token_name.

- [ ] **Dragonback Assault** — Enchantment {3}{G}{U}{R}. DealDamageAll works. **(NOW IMPLEMENTED — DealDamageAll NOW WORKS)** Missing landfall ability.
  - **Java source**: `Mage.Sets/src/mage/cards/d/DragonbackAssault.java`
  - **Fix needed**: Add landfall Dragon token trigger.

- [ ] **Encroaching Dragonstorm** — Enchantment {3}{G}. What works: Bounce (return self on Dragon ETB) + SearchLibrary (lands). **(NOW IMPLEMENTED — SearchLibrary NOW WORKS)**

- [ ] **Roiling Dragonstorm** — Enchantment {1}{U}. What works: DrawCards 2 + DiscardCards 1 + Bounce (self on Dragon ETB).
  - Note: Actually mostly complete. All key effects are typed and implemented.

- [ ] **Stormplain Detainment** — Enchantment {2}{W}. What works: Exile. What's broken: "until this leaves the battlefield" return-from-exile mechanic is not implemented.
  - **Fix needed**: Implement "exile until this leaves" return mechanism.

- [ ] **Teeming Dragonstorm** — Enchantment {3}{W}. What works: CreateToken (2/2 Soldiers) + Bounce (self on Dragon ETB). **(NOW IMPLEMENTED — Token stats NOW PARSED, 2/2 should work)**

- [ ] **All-Out Assault** — Enchantment {2}{R}{W}{B}. What works: Static boost +1/+1 + grant deathtouch (if continuous effects system works). What's broken: `Effect::Custom("Additional combat phase + untap all on attack.")`.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AllOutAssault.java`
  - **Fix needed**: Additional combat phase + untap-all-on-attack.

- [ ] **Ambling Stormshell** — 5/9 for {3}{U}{U}. What works: AddCounters (stun) + DrawCards 3, Ward {2} (**FIXED** -- typed `StaticEffect::Ward` + WARD keyword). What's broken: UntapTarget on Turtle cast (may work).
  - **Fix needed**: Implement Ward.

- [ ] **Anafenza, Unyielding Lineage** — 2/2 flash/first strike for {2}{W}. What's broken: `Effect::Custom("Endure 2...")` — should use AddCounters.
  - **Fix needed**: Replace Custom with `AddCounters { counter_type: "+1/+1", count: 2 }`.

- [ ] **Barrensteppe Siege** — Enchantment {2}{W}{B}. What's broken: ETB choose mode (Custom) + Mardu mode uses Custom. Abzan mode fixed (Batch 9): `Effect::Custom` → `Effect::add_counters_all("+1/+1", 1, "creatures you control")`.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BarrensteppeSiege.java`
  - **Fix needed**: Modal ETB choice + Mardu mode ("each opponent sacrifices a creature").

- [ ] **Betor, Kin to All** — 5/7 flying legendary for {2}{W}{B}{G}. What's broken: All Custom (toughness threshold draw/untap/half-life).
  - **Java source**: `Mage.Sets/src/mage/cards/b/BetorKinToAll.java`
  - **Fix needed**: Complex conditional effect based on total toughness.

- [ ] **Breaching Dragonstorm** — Enchantment {4}{R}. What works: Bounce (self on Dragon ETB). What's broken: ETB Custom (cascade-like free cast).
  - **Java source**: `Mage.Sets/src/mage/cards/b/BreachingDragonstorm.java`
  - **Fix needed**: Cascade-like effect.

- [ ] **Call the Spirit Dragons** — Enchantment {W}{U}{B}{R}{G}. What works: GrantKeyword ("indestructible" to Dragons — if continuous effects work). What's broken: Upkeep Custom (counter distribution + win condition).
  - **Java source**: `Mage.Sets/src/mage/cards/c/CallTheSpiritDragons.java`
  - **Fix needed**: Complex upkeep trigger with color-based targeting + alt win condition.

- [x] **Cori Mountain Monastery** — Land. What works: mana ability ({T}: Add {R}), `StaticEffect::EntersTappedUnless` (conditional ETB). What's broken: activated `Effect::Custom("Exile top card, play until end of next turn.")`.
  - **Batch 6**: Fixed `StaticEffect::Custom` → `StaticEffect::enters_tapped_unless("you control a Plains or an Island")`. Impulse draw still Custom.

- [ ] **Cori Steel-Cutter** — Equipment {1}{R}. What works: Static boost +2/+0 + grant trample/haste (if continuous effects work) + CreateToken (Monk). What's broken: Equip activated ability `Effect::Custom("Attach...")`.
  - **Fix needed**: Implement Equip as an effect.

- [ ] **Corroding Dragonstorm** — Enchantment {1}{B}. What works: DealDamageOpponents 2 + GainLife 2 + Scry 2 + Bounce (self). **(NOW IMPLEMENTED — Scry NOW WORKS)**

- [ ] **Death Begets Life** — Sorcery {2}{W}{B}. DestroyAll works. **(NOW IMPLEMENTED — DestroyAll NOW WORKS)** Still broken: `Effect::Custom("Create X Spirits")` (dynamic token creation).
  - **Java source**: `Mage.Sets/src/mage/cards/d/DeathBegetsLife.java`
  - **Fix needed**: Dynamic token creation based on destroyed creature count.

- [ ] **Dragonfire Blade** — Equipment {2}. What works: Static +1/+0. What's broken: combat damage trigger Custom + Equip Custom.
  - **Fix needed**: Implement equip + "deal damage equal to combat damage" effect.

- [ ] **Dragonologist** — 1/3 for {U}. Both triggers use Scry 1. **(NOW IMPLEMENTED — Scry NOW WORKS)**

- [ ] **Effortless Master** — 3/4 flash for {3}{U}. What works: BoostUntilEndOfTurn -4/+0. Note: boost says "until your next turn" not "until EOT" — timing is wrong.
  - **Fix needed**: "Until your next turn" duration vs EOT.

- [ ] **Eshki, Dragonclaw** — 4/4 trample legendary. What works: AddCounters +1/+1 on attack. What's broken: `Effect::Custom("If power >= 7: creatures +2/+0 and trample.")`.
  - **Java source**: `Mage.Sets/src/mage/cards/e/EshkiDragonclaw.java`
  - **Fix needed**: Conditional board-wide boost based on power check.

- [ ] **Fangkeeper's Familiar** — 2/1 deathtouch for {1}{G}. SearchLibrary. **(NOW IMPLEMENTED — SearchLibrary NOW WORKS)**

- [ ] **Fire-Rim Form** — Aura {U}{R}. What works: Static +2/+2 + grant flying (if continuous effects work) + ReturnFromGraveyard on dies trigger. **(NOW IMPLEMENTED — ReturnFromGraveyard NOW WORKS)**

- [ ] **Focus the Mind** — Instant {4}{U}. What works: DrawCards 3 + DiscardCards 1. What's broken: `StaticEffect::CostReduction` (may or may not work).
  - **Fix needed**: Verify CostReduction implementation.

- [ ] **Fresh Start** — Sorcery {3}{W}. All Custom: "Each player shuffles hand and graveyard into library, then draws 7."
  - **Java source**: `Mage.Sets/src/mage/cards/f/FreshStart.java`
  - **Fix needed**: Complex zone manipulation + mass draw.

- [ ] **Frostcliff Siege** — Enchantment {2}{U}{R}. What works: DrawCards 1 + DiscardCards 1 (Jeskai mode) + BoostUntilEndOfTurn +2/+0 (Temur mode). What's broken: ETB mode choice (Custom).
  - **Fix needed**: Modal ETB choice system.

- [ ] **Glacierwood Siege** — Enchantment {2}{B}{G}. What works: AddCounters +1/+1 (Sultai mode) + DrawCards 1 (Abzan mode). What's broken: ETB mode choice (Custom) + trigger conditions.
  - **Fix needed**: Modal ETB choice system.

- [ ] **Hollowmurk Siege** — Enchantment {2}{R}{G}. What works: Static boost +1/+0 + grant trample (Temur) + CreateToken Treasure (Mardu). What's broken: ETB mode choice (Custom).
  - **Fix needed**: Modal ETB choice system.

- [ ] **Host of the Hereafter** — 4/5 flying for {3}{W}{B}. What works: CreateToken (Spirit tokens). What's broken: Token count hardcoded to 4 instead of dynamic (X = power).
  - **Fix needed**: Dynamic token count from power.

- [ ] **Karakyk Guardian** — 2/5 flying/defender. What's broken: `Effect::Custom("Can attack this turn as though it didn't have defender.")`.
  - **Fix needed**: Implement "ignore defender" effect.

- [ ] **Dracogenesis** — Enchantment {6}{R}{R}. What's broken: `StaticEffect::Custom("Cast Dragon spells for free")`.
  - **Java source**: `Mage.Sets/src/mage/cards/d/Dracogenesis.java`
  - **Fix needed**: Free-cast static ability for Dragon spells.

- [ ] **Dragonstorm Globe** — Artifact {3}. What works: mana ability. What's broken: `StaticEffect::Custom("Dragons enter with extra +1/+1 counter")`.
  - **Fix needed**: Implement "enters with additional counter" replacement effect.

- [ ] **Abzan Monument** — Artifact {2}. SearchLibrary works. **(NOW IMPLEMENTED — SearchLibrary NOW WORKS)** Token stats for X/X Spirit still may be wrong for dynamic X.

- [ ] **Mardu Monument** — Artifact {2}. What works: CreateToken (3 Warriors) + SearchLibrary + GainKeywordUntilEndOfTurn. **(NOW IMPLEMENTED — SearchLibrary + GainKeywordUntilEndOfTurn NOW WORK)**

- [ ] **Jeskai Monument** — Artifact {2}. SearchLibrary. **(NOW IMPLEMENTED — SearchLibrary NOW WORKS)**

- [ ] **Sultai Monument** — Artifact {2}. SearchLibrary. **(NOW IMPLEMENTED — SearchLibrary NOW WORKS)**

- [ ] **Temur Monument** — Artifact {2}. SearchLibrary. **(NOW IMPLEMENTED — SearchLibrary NOW WORKS)**

- [ ] **Dragonstorm Forecaster** — 0/3 for {U}. SearchLibrary. **(NOW IMPLEMENTED — SearchLibrary NOW WORKS)**

- [ ] **Monastery Messenger** — 2/3 flying/vigilance. What's broken: `Effect::Custom("Put target noncreature, nonland card from your graveyard on top of your library.")`.
  - **Fix needed**: "Card from graveyard to top of library" effect.

- [ ] **Rite of Renewal** — Sorcery {3}{G}. What works: ReturnFromGraveyard (NOW WORKS). What's broken: ReturnFromGraveyard + Custom (shuffle cards into library, exile self).
  - **Fix needed**: Implement ReturnFromGraveyard + shuffle-into-library + self-exile.

- [ ] **Snowmelt Stag** — 2/5 vigilance for {3}{U}. What's broken: `StaticEffect::Custom("During your turn, base P/T is 5/2.")` + activated Custom ("can't be blocked").
  - **Fix needed**: Conditional P/T change + unblockable effect.

- [ ] **Stormbeacon Blade** — Equipment {1}{W}. What works: Static +3/+0. What's broken: DrawCards 1 (conditional — only if 3+ attackers; condition not enforced) + Equip Custom.
  - **Fix needed**: Conditional trigger + implement equip.

- [ ] **Tempest Hawk** — 2/2 flying for {2}{W}. SearchLibrary works. **(NOW IMPLEMENTED — SearchLibrary NOW WORKS)** `StaticEffect::Custom("Any number in deck")` still non-functional.

- [ ] **Trade Route Envoy** — 4/3 for {3}{G}. All Custom: conditional draw-or-counter ETB.
  - **Fix needed**: Conditional effect based on game state check.

- [ ] **Traveling Botanist** — 2/3 for {1}{G}. All Custom: look at top card, land to hand or card to graveyard.
  - **Fix needed**: Top-of-library manipulation effect.

- [ ] **Wayspeaker Bodyguard** — 3/4 for {3}{W}. What works: ReturnFromGraveyard + TapTarget. **(NOW IMPLEMENTED — ReturnFromGraveyard NOW WORKS)**

- [ ] **Wingspan Stride** — Aura {U}. What works: Static +1/+1 + grant flying + Bounce (return self to hand). Mostly complete.

- [ ] **Dragonbroods' Relic** — Artifact {1}{G}. What works: AddMana (any color). What's broken: `Cost::Custom("Tap an untapped creature you control")`.
  - **Fix needed**: Implement creature-tap as cost.

- [ ] **Herd Heirloom** — Artifact {1}{G}. What works: mana ability + GainKeywordUntilEndOfTurn (trample). **(NOW IMPLEMENTED — GainKeywordUntilEndOfTurn NOW WORKS)** Missing "draw on combat damage" grant.

- [ ] **Essence Anchor** — Artifact {2}{U}. Scry 1 on upkeep. **(NOW IMPLEMENTED — Scry NOW WORKS)**

- [ ] **Sonic Shrieker** — 4/4 flying for {2}{R}{W}{B}. What works: DealDamage 2 + GainLife 2. What's broken: `Effect::Custom("If player damaged, they discard.")`.
  - **Fix needed**: Conditional discard on player damage.

- [ ] **Sunpearl Kirin** — 2/1 flash/flying for {1}{W}. What works: Bounce. What's broken: `Effect::Custom("If it was a token, draw a card.")`.
  - **Fix needed**: Conditional draw based on bounced permanent type.

- [ ] **Starry-Eyed Skyrider** — 1/3 flying for {2}{W}. GainKeywordUntilEndOfTurn (flying) works. **(NOW IMPLEMENTED — GainKeywordUntilEndOfTurn NOW WORKS)** `StaticEffect::GrantKeyword` (attacking tokens have flying) may still not work.

- [ ] **Static Snare** — Enchantment {4}{W} flash. What works: Exile. What's broken: `StaticEffect::CostReduction` (may not work) + "until this leaves" return mechanic.
  - **Fix needed**: Verify CostReduction + exile-until-leaves mechanic.

- [ ] **Stormscale Scion** — 4/4 flying/storm for {4}{R}{R}. What works: Static Boost +1/+1 to other Dragons. Storm keyword may not function.
  - **Fix needed**: Implement Storm keyword.

- [ ] **Magmatic Hellkite** — 4/5 flying for {2}{R}{R}. What works: Destroy (nonbasic land). What's broken: `Effect::Custom("Controller searches for basic land, puts tapped with stun counter.")`.
  - **Fix needed**: Search-for-opponent + put-with-stun-counter.

- [ ] **Mardu Siegebreaker** — (classified above). What works: Destroy. What's broken: attacks Custom (copy token).

- [ ] **Neriv, Heart of the Storm** — 4/5 flying for {1}{R}{W}{B}. What works: Static Boost +1/+1 to tokens + GrantKeyword haste. What's broken: Attacks Custom (copy token).
  - **Fix needed**: "Create token copy" effect.

- [ ] **Purging Stormbrood** — 4/4 flying for {4}{B}. RemoveCounters. **(NOW IMPLEMENTED — RemoveCounters NOW WORKS)**

- [ ] **Rally the Monastery** — Instant {3}{W}. What's broken: CostReduction (may work) + all Custom (modal spell).
  - **Fix needed**: Modal spell support.

- [ ] **Ringing Strike Mastery** — Aura {U}. What works: Static +1/+1 + Bounce (return to hand). Missing: Ward {1} (not implemented).

- [ ] **Severance Priest** — 3/3 deathtouch for {W}{B}{G}. Both abilities are Custom (hand exile + Spirit token on leave).
  - **Java source**: `Mage.Sets/src/mage/cards/s/SeverancePriest.java`
  - **Fix needed**: Hand reveal/exile + leave-the-battlefield triggered token creation.

- [ ] **Shiko, Paragon of the Way** — 4/5 flying/vigilance for {2}{U}{R}{W}. What's broken: ETB Custom (draw cards = noncreature/nonland permanents) + CostReduction.
  - **Fix needed**: Dynamic draw count based on permanents.

- [ ] **Synchronized Charge** — Sorcery {1}{G}. All Custom: distribute counters + grant keywords.
  - **Fix needed**: Distribute counters + conditional keyword grant.

- [ ] **Teval, Arbiter of Virtue** — 6/6 flying/lifelink for {2}{B}{G}{U}. What's broken: `StaticEffect::Custom("Spells you cast have delve.")` + Custom (lose life equal to MV).
  - **Fix needed**: Delve mechanic + dynamic life loss.

- [ ] **Ureni, the Song Unending** — 10/10 flying/protection for {5}{G}{U}{R}. What's broken: `StaticEffect::Custom("Protection from white and black.")` + Custom (deal X damage divided).
  - **Fix needed**: Protection implementation + divided damage.

- [ ] **War Effort** — Enchantment {3}{R}. What works: Static +1/+0 + CreateTokenTappedAttacking. **(NOW IMPLEMENTED — CreateTokenTappedAttacking NOW WORKS)**

- [ ] **Zurgo, Thunder's Decree** — 2/4 haste for {R}{W}{B}. What works: DealDamageAll 1 on attack. What's broken: `StaticEffect::Custom` (gain life + boost on creature death).
  - **Fix needed**: Implement damage-tracking triggered ability.

- [ ] **Windcrag Siege** — Enchantment {1}{R}{W}. What's broken: `StaticEffect::Custom(...)` (modal choice — both options are Custom).
  - **Fix needed**: Modal ETB choice + implement both options.

- [ ] **Glacial Dragonhunt** — Sorcery {U}{R}. What works: DrawCards 1. What's broken: Custom (conditional discard-then-deal-3-damage).
  - **Fix needed**: Optional discard with conditional damage.

- [ ] **Winternight Stories** — Sorcery {2}{U}. What works: DrawCards 3. What's broken: Custom (discard 2 unless discard a creature).
  - **Fix needed**: Conditional discard logic.

- [ ] **Yathan Roadwatcher** — 3/3 for {1}{W}{B}{G}. What works: Mill 4. What's broken: Custom (return creature MV<=3 from graveyard to battlefield).
  - **Fix needed**: Conditional reanimate from milled cards.

- [ ] **Stillness in Motion** — Enchantment {1}{U}. What works: Mill 3. What's broken: Custom (library-empty check + exile self + stack cards).
  - **Fix needed**: Complex conditional triggered effect.

- [ ] **Lie in Wait** — Sorcery {B}{G}{U}. What works: Mill 4. What's broken: Custom (put creature from milled cards onto battlefield).
  - **Fix needed**: Reanimate-from-milled effect.

- [ ] **Lotuslight Dancers** — 3/6 lifelink for {2}{B}{G}{U}. All Custom (opponent mill + reanimate from milled).
  - **Fix needed**: Opponent-targeting mill + reanimate-from-milled.

- [ ] **Maelstrom of the Spirit Dragon** — Land. What works: mana ability. What's broken: Custom (Dragon deals power-damage to any target).
  - **Fix needed**: Dynamic damage based on Dragon's power.

- [ ] **Lasyd Prowler** — 5/5 for {2}{G}{G}. What's broken: Custom (mill cards equal to lands you control — dynamic count).
  - **Fix needed**: Dynamic mill count.

- [ ] **Kishla Trawlers** — 3/2 for {2}{U}. All Custom (exile creature from graveyard, return instant/sorcery).
  - **Fix needed**: Graveyard manipulation effects.

- [ ] **Tersa Lightshatter** — 3/3 haste for {2}{R}. What's broken: ETB Custom (discard up to 2, draw that many) + attacks Custom (threshold exile-from-graveyard-and-play).
  - **Fix needed**: Variable discard/draw + impulse draw from graveyard.

- [ ] **Sarkhan, Dragon Ascendant** — 2/2 flying for {1}{R}. What's broken: Custom (behold Dragon, if so create Treasure).
  - **Fix needed**: Behold mechanic + conditional token.

- [ ] **Songcrafter Mage** — 3/2 flash for {G}{U}{R}. All Custom (grant harmonize to graveyard card).
  - **Fix needed**: Harmonize mechanic.

- [ ] **Stadium Headliner** — 1/1 for {R}. What's broken: Custom (deal damage equal to creature count — dynamic).
  - **Fix needed**: Dynamic damage calculation.

- [ ] **Strategic Betrayal** — Sorcery {1}{B}. All Custom (opponent exiles creature + graveyard).
  - **Fix needed**: Opponent-targeted exile from battlefield + graveyard.

- [ ] **Wail of War** — Instant {2}{B}. All Custom (modal: -1/-1 to opponent creatures or return 2 from graveyard).
  - **Fix needed**: Modal + mass debuff or multi-return.

- [ ] **Ugin, Eye of the Storms** — Planeswalker {7}. What's broken: Custom (exile colored permanent on cast trigger). Also missing loyalty counters/abilities.
  - **Fix needed**: Planeswalker framework.

- [ ] **Elspeth, Storm Slayer** — Planeswalker {3}{W}{W}. What's broken: `StaticEffect::Custom("double token creation")`. Missing loyalty abilities.
  - **Fix needed**: Planeswalker framework + token doubling.

---

## Stub Cards

These cards are stat/keyword-only with no functional abilities. They exist as permanents but do nothing beyond being a body with power/toughness/keywords.

- [ ] **Kheru Goldkeeper** — 3/3 flying Dragon for {1}{B}{G}{U}. No abilities at all in Rust.
  - **Java source**: `Mage.Sets/src/mage/cards/k/KheruGoldkeeper.java`
  - **What it should do**: Legendary Dragon with abilities from the Java source.

- [ ] **Kotis, the Fangkeeper** — 2/1 indestructible Zombie Warrior for {1}{B}{G}{U}. No abilities in Rust.
  - **Java source**: `Mage.Sets/src/mage/cards/k/KotisTheFangkeeper.java`
  - **What it should do**: Legendary creature with triggered/activated abilities.

- [ ] **Naga Fleshcrafter** — 0/0 Snake Shapeshifter for {3}{U}. No abilities in Rust. (Would die immediately as 0/0.)
  - **Java source**: `Mage.Sets/src/mage/cards/n/NagaFleshcrafter.java`
  - **What it should do**: Legendary clone creature that copies another creature.

- [ ] **Narset, Jeskai Waymaster** — 3/4 Human Monk for {U}{R}{W}. No abilities in Rust.
  - **Java source**: `Mage.Sets/src/mage/cards/n/NarsetJeskaiWaymaster.java`
  - **What it should do**: Legendary creature with prowess-like abilities and card filtering.

- [ ] **Rot-Curse Rakshasa** — 5/5 trample Demon for {1}{B}. No abilities in Rust.
  - **Java source**: `Mage.Sets/src/mage/cards/r/RotCurseRakshasa.java`
  - **What it should do**: Undercosted with drawback — likely has a significant downside ability.

- [ ] **Sidisi, Regent of the Mire** — 1/3 Zombie Snake Warlock for {1}{B}. No abilities in Rust.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SidisiRegentOfTheMire.java`
  - **What it should do**: Legendary creature with graveyard/sacrifice synergies.

- [ ] **Smile at Death** — Enchantment for {3}{W}{W}. No abilities in Rust.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SmileAtDeath.java`
  - **What it should do**: Mythic enchantment — likely a powerful static or triggered ability.

- [ ] **Stalwart Successor** — 3/2 Human Warrior for {1}{B}{G}. No abilities in Rust.
  - **Java source**: `Mage.Sets/src/mage/cards/s/StalwartSuccessor.java`
  - **What it should do**: Creature with death/graveyard synergy abilities.

- [ ] **Surrak, Elusive Hunter** — 4/3 trample Human Warrior for {2}{G}. No abilities in Rust.
  - **Java source**: `Mage.Sets/src/mage/cards/s/SurrakElusiveHunter.java`
  - **What it should do**: Legendary creature with combat/power-matters abilities.

- [ ] **Taigam, Master Opportunist** — 2/2 Human Monk for {1}{U}. No abilities in Rust.
  - **Java source**: `Mage.Sets/src/mage/cards/t/TaigamMasterOpportunist.java`
  - **What it should do**: Legendary creature with spell/noncreature synergies.

- [ ] **Warden of the Grove** — 2/2 Hydra for {2}{G}. No abilities in Rust.
  - **Java source**: `Mage.Sets/src/mage/cards/w/WardenOfTheGrove.java`
  - **What it should do**: Hydra with counter-based scaling abilities.

- [ ] **Felothar, Dawn of the Abzan** — 3/3 trample Human Warrior for {W}{B}{G}. No abilities in Rust.
  - **Java source**: `Mage.Sets/src/mage/cards/f/FelotharDawnOfTheAbzan.java`
  - **What it should do**: Legendary creature with Abzan-themed abilities.

- [ ] **Bloomvine Regent** — 4/5 flying Dragon. Missing mana cost! No abilities in Rust.
  - **Java source**: `Mage.Sets/src/mage/cards/b/BloomvineRegent.java`
  - **What it should do**: Dragon with ETB or triggered abilities.

- [ ] **Clarion Conqueror** — 3/3 flying Dragon for {2}{W}. No abilities in Rust.
  - **Java source**: `Mage.Sets/src/mage/cards/c/ClarionConqueror.java`
  - **What it should do**: Dragon with abilities (likely ETB or attack triggers).

- [ ] **Krumar Initiate** — 2/2 Human Cleric for {1}{B}. No abilities in Rust.
  - **Java source**: `Mage.Sets/src/mage/cards/k/KrumarInitiate.java`
  - **What it should do**: Creature with abilities from Java source.

- [ ] **Scavenger Regent** — 4/4 flying Dragon. Ward -- Discard a card (**FIXED** -- typed `StaticEffect::Ward` + WARD keyword). Still missing mana cost.
  - **Java source**: `Mage.Sets/src/mage/cards/s/ScavengerRegent.java`
  - **What it should do**: Dragon with ETB or triggered abilities.

- [ ] **Dirgur Island Dragon** — 4/4 flying Dragon. Ward {2} (**FIXED** -- typed `StaticEffect::Ward` + WARD keyword). Still missing mana cost.
  - **Java source**: `Mage.Sets/src/mage/cards/d/DirgurIslandDragon.java`
  - **What it should do**: Dragon with Ward {2} and likely other abilities.

- [ ] **Runescale Stormbrood** — 2/4 flying Dragon. Missing mana cost! Has BoostUntilEndOfTurn +2/+0 trigger (works).
  - **Fix needed**: Add mana cost.

- [ ] **Awaken the Honored Dead** — Saga for {B}{G}{U}. Only Custom saga text. Saga mechanics not implemented.
  - **Java source**: `Mage.Sets/src/mage/cards/a/AwakenTheHonoredDead.java`
  - **Fix needed**: Implement Saga framework + all 3 chapters.

- [ ] **Rediscover the Way** — Saga for {U}{R}{W}. Only Custom saga text. Saga mechanics not implemented.
  - **Java source**: `Mage.Sets/src/mage/cards/r/RediscoverTheWay.java`
  - **Fix needed**: Implement Saga framework.

- [ ] **Revival of the Ancestors** — Saga for {1}{W}{B}{G}. Only Custom saga text.
  - **Java source**: `Mage.Sets/src/mage/cards/r/RevivalOfTheAncestors.java`
  - **Fix needed**: Implement Saga framework.

- [ ] **Roar of Endless Song** — Saga for {2}{G}{U}{R}. Only Custom saga text.
  - **Java source**: `Mage.Sets/src/mage/cards/r/RoarOfEndlessSong.java`
  - **Fix needed**: Implement Saga framework.

- [ ] **Thunder of Unity** — Saga for {R}{W}{B}. Only Custom saga text.
  - **Java source**: `Mage.Sets/src/mage/cards/t/ThunderOfUnity.java`
  - **Fix needed**: Implement Saga framework.

- [ ] **Nature's Rhythm** — Sorcery {X}{G}{G}. All Custom (search for creature with MV <= X).
  - **Java source**: `Mage.Sets/src/mage/cards/n/NaturesRhythm.java`
  - **Fix needed**: SearchLibrary with X-based filter.

- [ ] **New Way Forward** — Instant {2}{U}{R}{W}. All Custom (damage prevention + redirect).
  - **Java source**: `Mage.Sets/src/mage/cards/n/NewWayForward.java`
  - **Fix needed**: Damage prevention/redirect mechanic.

- [ ] **United Battlefront** — Sorcery {3}{W}. All Custom (look at top 7, put 2 permanents onto battlefield).
  - **Java source**: `Mage.Sets/src/mage/cards/u/UnitedBattlefront.java`
  - **Fix needed**: Library filtering + free permanent deployment.

- [ ] **The Sibsig Ceremony** — Enchantment {B}{B}{B}. Only CostReduction (creature spells cost {2} less). Depends on CostReduction implementation.
  - **Fix needed**: Verify CostReduction works.

---

## Missing Cards

- None. All 271 non-basic-land cards from the Java set file have factory functions in tdm.rs.

Note: There are 2 duplicate registrations with alternate names:
- "Cori Steel-Cutter" and "Cori-Steel Cutter" both point to `cori_steel_cutter`
- "Eshki Dragonclaw" and "Eshki, Dragonclaw" both point to `eshki_dragonclaw`

---

## Priority Fixes (Highest Impact)

### ~~1. Implement Scry in execute_effects (fixes ~10 cards)~~ -- **DONE**
Cards affected: Boulderborn Dragon, Cruel Truths, Poised Practitioner, Dragonologist (x2), Corroding Dragonstorm, Essence Anchor, Mistrise Village, Nightblade Brigade

### ~~2. Implement SearchLibrary in execute_effects (fixes ~10 cards)~~ -- **DONE**
Cards affected: Roamer's Routine, Fangkeeper's Familiar, Encroaching Dragonstorm, Abzan/Mardu/Jeskai/Sultai/Temur Monument, Dragonstorm Forecaster, Tempest Hawk

### ~~3. Implement ReturnFromGraveyard in execute_effects (fixes ~8 cards)~~ -- **DONE**
Cards affected: Furious Forebear, Auroral Procession, Duty Beyond Death, Salt Road Packbeast, Sultai Devotee, Heritage Reclamation, Fire-Rim Form, Wayspeaker Bodyguard

### ~~4. Implement GainKeywordUntilEndOfTurn in execute_effects (fixes ~12 cards)~~ -- **DONE**
Cards affected: Alesha's Legacy, Lightfoot Technique, Wild Ride, Formation Breaker, Reigning Victor, Unrooted Ancestor, Starry-Eyed Skyrider, Equilibrium Adept, Mardu Monument, Herd Heirloom, Snakeskin Veil

### ~~5. Implement Sacrifice in execute_effects (fixes ~3 cards)~~ -- **DONE**
Cards affected: Aggressive Negotiations, Bone-Cairn Butcher, (Worthy Cost additional cost)

### ~~6. Fix token creation to parse P/T from token_name (fixes ~5 cards)~~ -- **DONE**
Cards affected: Mammoth Bellow (5/5), Teeming Dragonstorm (2/2), Abzan Monument (X/X), Salt Road Skirmish (1/1 correct), all tokens that specify non-1/1 stats

### 7. Implement Saga framework (fixes 5 cards)
Cards affected: Awaken the Honored Dead, Rediscover the Way, Revival of the Ancestors, Roar of Endless Song, Thunder of Unity

### 8. Implement modal spell support (fixes ~8 cards)
Cards affected: Coordinated Maneuver, Frontline Rush, Sarkhan's Resolve, Seize Opportunity, Riverwalk Technique, Overwhelming Surge, Rally the Monastery, Wail of War

### 9. Add missing mana costs (fixes 3+ cards)
Cards affected: Bloomvine Regent, Scavenger Regent, Dirgur Island Dragon, Runescale Stormbrood, Marang River Regent (all missing mana_cost field)

### 10. Implement stub cards (fixes 12 cards)
All stub cards need their abilities read from Java sources and implemented.
