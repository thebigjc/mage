# ECL Implementation Todo

Ordered by topological sort of the dependency graph. Engine capabilities are
scheduled greedily: each step picks the capability that unlocks the most new
cards. Multi-dependency cards appear under their last-needed capability.

**126 cards complete** | **141 cards remaining** across **21 engine capabilities**

## 1. Conditional/Dynamic Effects (COND)

**Effort:** Hard | **Cards unlocked:** 51 | **Running total:** 51/181
> Various state-dependent abilities, watchers, replacement effects

- [ ] **Implement Conditional/Dynamic Effects engine support**

### Single-dependency cards (51)

- [ ] Abigale, Eloquent First-Year — LoseAllAbilities effect, keyword counters (flying/first_strike/lifelink)
- [ ] Boneclub Berserker
- [ ] Boulder Dash — Multi-target damage split (2 to one target, 1 to another)
- [ ] Bre of Clan Stoutarm
- [ ] Bristlebane Outrider
- [ ] Curious Colossus — LoseAllAbilities, AddCardSubType (Coward), SetBasePowerToughness 1/1 on all opponent creatures
- [x] Dawnhand Eulogist — mill(3) + lose_life_opponents(2) + gain_life(2) (conditional Elf check not enforced)
- [ ] Doran, Besieged by Time
- [ ] Feisty Spikeling
- [ ] Figure of Fable — Multi-level activated abilities changing type/base P/T, conditional on current subtype
- [ ] Formidable Speaker — ETB may discard to search for creature, activated untap another permanent
- [x] Gallant Fowlknight
- [ ] Glamer Gifter — Set base P/T 4/4 + gain all creature types on target until EOT
- [ ] Glen Elendra's Answer — Can't be countered, counter ALL opponent spells+abilities, create tokens equal to count
- [ ] Gloom Ripper — Dynamic X = Elves you control + Elf cards in GY, +X/+0 to your creature, -0/-X to opponent's
- [ ] Gravelgill Scoundrel — Attacks trigger may tap another creature for unblockable this turn
- [ ] Hexing Squelcher — Can't be countered, Ward-pay 2 life, spells can't be countered static, grant ward to others
- [ ] Illusion Spinners — Conditional flash (if you control Faerie), hexproof while untapped
- [x] Impolite Entrance — Target creature gains trample+haste until EOT, draw a card
- [ ] Kinbinding — Dynamic +X/+X where X=creatures entered this turn (watcher), begin-of-combat token creation
- [ ] Kinscaer Sentry — Attacks then put creature from hand onto battlefield tapped+attacking if MV <= attacking count
- [x] Lasting Tarfire — end step damage_opponents(2) (conditional counter check not enforced)
- [ ] Lluwen, Imperfect Naturalist — Mill 4 + top-of-library manipulation, discard land cost, tokens = lands in GY
- [ ] Meanders Guide — Attacks then may tap Merfolk then return creature MV<=3 from GY to battlefield
- [ ] Meek Attack — Activated ability put creature from hand (P+T<=5), haste, end-step sacrifice
- [ ] Moon-Vigil Adherents
- [x] Morcant's Eyes — upkeep scry(1) typed, activated sac+mana with X tokens is Custom
- [x] Morcant's Loyalist — Other Elves +1/+1 lord, dies then return another Elf card from GY to hand
- [ ] Morningtide's Light — Mass flicker, prevent damage until next turn, exile self
- [ ] Mornsong Aria — Players can't draw or gain life (static), each draw step: lose 3 life + search library
- [x] Personify — flicker Custom + create_token typed, targets CreatureYouControl
- [x] Pummeler for Hire — Ward {2} typed, ETB gain life dynamic is Custom
- [ ] Raiding Schemes — Noncreature spells you cast have conspire (very complex stack manipulation)
- [ ] Retched Wretch — Conditional dies trigger (if had -1/-1 counter), return to battlefield + lose all abilities
- [ ] Rhys, the Evermore — Flash, grant persist until EOT, activated remove any number of counters (sorcery speed)
- [x] Riverguard's Reflexes — +2/+2 + first strike until EOT + untap target creature
- [ ] Safewright Cavalry
- [ ] Sapling Nursery — Affinity for Forests, landfall then Treefolk token, exile self then indestructible until EOT
- [ ] Spry and Mighty — Choose 2 creatures, draw X + +X/+X + trample where X=power difference
- [ ] Sunderflock — Cost reduction by greatest MV among Elementals, if cast then bounce all non-Elemental creatures
- [ ] Swat Away — Cost reduction if creature attacking you, put spell/creature on top/bottom of library
- [ ] Tam, Mindful First-Year — Other creatures hexproof from each of their colors, make creature all colors
- [ ] Taster of Wares — ETB opponent reveals X cards (X=Goblins), choose one to exile, may cast instant/sorcery
- [ ] Tend the Sprigs — Search basic land to battlefield tapped, conditional create Treefolk token if 7+ lands/Treefolk
- [ ] Thoughtweft Imbuer
- [x] Tributary Vaulter — Becomes tapped trigger then another Merfolk gets +2/+0 until EOT
- [ ] Twinflame Travelers — Other Elemental triggered abilities trigger additional time (replacement effect)
- [x] Vinebred Brawler — Must be blocked if able, attacks then another Elf gets +2/+1
- [x] Wanderbrine Trapper — Activated: {1}, T, tap another creature then tap opponent creature
- [ ] Wanderwine Farewell — Convoke, bounce 1-2 nonland permanents, conditional Merfolk tokens
- [x] Wary Farmer — end step scry(1) (conditional creature-entry check not enforced)

## 2. Cost System (COST)

**Effort:** Medium | **Cards unlocked:** 15 | **Running total:** 66/181
> RemoveCounters, Blight, ExileFromGraveyard pay_costs() match arms

- [ ] **Implement Cost System engine support**

### Single-dependency cards (12)

- [x] Bogslither's Embrace — exile target creature (blight-or-pay cost is Custom)
- [x] Creakwood Safewright — ETB 3 -1/-1 counters + end step remove counter
- [x] Glen Elendra Guardian — ETB -1/-1 counter + remove counter to counter spell
- [x] Gnarlbark Elm
- [x] Goldmeadow Nomad
- [x] Gristle Glutton
- [x] Hovel Hurler — ETB 2 -1/-1 counters + remove counter for +1/+0 + flying
- [x] Loch Mare — ETB 3 -1/-1 counters + 2 activated abilities (draw, tap+stun)
- [x] Moonlit Lamenter
- [x] Reaping Willow — ETB 2 -1/-1 counters + remove 2 to reanimate MV<=3
- [x] Requiting Hex — destroy + conditional gain life (optional blight is Custom)
- [x] Stoic Grove-Guide — fixed activated ability cost to {1}{B/G}

### Multi-dependency cards (3) — now fully unblocked

- [ ] High Perfect Morcant _COND + COST_ — ETB this/another Elf then opponents blight 1, tap 3 Elves then proliferate (sorcery speed)
- [ ] Moonshadow _COND + COST_ — ETB with 6 -1/-1 counters, trigger on permanent cards to GY then remove -1/-1 counter
- [ ] Slumbering Walker _COND + COST_

## 3. Vivid Mechanic (VIVID)

**Effort:** Medium | **Cards unlocked:** 11 | **Running total:** 77/181
> Color-counting helper → X, effect variants

- [ ] **Implement Vivid Mechanic engine support**

### Single-dependency cards (10)

- [ ] Aurora Awakener
- [ ] Explosive Prodigy
- [ ] Glister Bairn
- [ ] Luminollusk
- [ ] Prismabasher
- [ ] Prismatic Undercurrents — Vivid (search X basic lands, X=colors), play additional land each turn
- [ ] Shimmercreep
- [ ] Shinestriker
- [ ] Squawkroaster
- [ ] Wildvine Pummeler — Vivid cost reduction, Reach, Trample

### Multi-dependency cards (1) — now fully unblocked

- [ ] Kithkeeper _COND + VIVID_ — Vivid (X = colors among permanents), create X Kithkin tokens, tap 3 creatures then +3/+0 + flying

## 4. Player Choice (CHOICE)

**Effort:** Medium | **Cards unlocked:** 10 | **Running total:** 87/181
> Runtime choice/decision framework

- [ ] **Implement Player Choice engine support**

### Single-dependency cards (7)

- [ ] Boggart Mischief
- [ ] Chaos Spewer
- [ ] Glamermite
- [ ] Gutsplitter Gang
- [ ] Scuzzback Scrounger
- [ ] Thirst for Identity — Draw 3, then discard 2 unless you discard a creature card
- [ ] Voracious Tome-Skimmer

### Multi-dependency cards (3) — now fully unblocked

- [ ] Puca's Eye _CHOICE + VIVID_ — ETB draw + choose color + become that color, activated draw if 5 colors
- [ ] Rime Chill _CHOICE + VIVID_
- [ ] Soul Immolation _CHOICE + COST_

## 5. Creature Type Choice (TYPE)

**Effort:** Medium | **Cards unlocked:** 9 | **Running total:** 96/181
> Choose-type + conditional application, Convoke

- [ ] **Implement Creature Type Choice engine support**

### Single-dependency cards (5)

- [ ] Chronicle of Victory
- [ ] Collective Inferno
- [ ] Dawn-Blessed Pennant
- [ ] Eclipsed Realms
- [ ] Harmonized Crescendo — Convoke, choose creature type, draw cards equal to permanents of that type

### Multi-dependency cards (4) — now fully unblocked

- [ ] Bloodline Bidding _COND + TYPE_ — Convoke, choose creature type, return all creatures of type from GY to battlefield
- [ ] Gathering Stone _CHOICE + TYPE_ — Choose creature type, cost reduction for chosen type, look at top card + conditional reveal
- [ ] Selfless Safewright _COND + TYPE_ — Flash, Convoke, choose creature type, grant hexproof+indestructible until EOT
- [ ] Winnowing _COND + TYPE_ — Convoke, for each player choose creature, sac others not sharing type

## 6. Aura System (AURA)

**Effort:** Hard | **Cards unlocked:** 9 | **Running total:** 105/181
> Enchant creature, attachment, continuous effects

- [ ] **Implement Aura System engine support**

### Single-dependency cards (9)

- [ ] Aquitect's Defenses
- [ ] Blossombind
- [ ] Evershrike's Gift
- [ ] Gilt-Leaf's Embrace
- [ ] Lofty Dreams
- [ ] Noggle the Mind
- [ ] Pitiless Fists
- [ ] Shimmerwilds Growth
- [ ] Spiral into Solitude

## 7. Behold Mechanic (BEHOLD)

**Effort:** Hard | **Cards unlocked:** 8 | **Running total:** 113/181
> Reveal/exile from hand, LTB return framework

- [ ] **Implement Behold Mechanic engine support**

### Single-dependency cards (2)

- [ ] Champion of the Clachan
- [ ] Champions of the Perfect

### Multi-dependency cards (6) — now fully unblocked

- [ ] Celestial Reunion _BEHOLD + TYPE_ — Behold mechanic, creature type choice, search library, conditional battlefield vs hand placement
- [ ] Champion of the Path _BEHOLD + COND_
- [ ] Champion of the Weird _BEHOLD + COST_
- [ ] Champions of the Shoal _BEHOLD + COST_ — Behold+exile cost, tap+stun counter on ETB/tap, LTB return exiled card
- [ ] Mudbutton Cursetosser _BEHOLD + COND_ — Behold Goblin or pay {2}, can't block, dies then destroy opponent creature power<=2
- [ ] Soulbright Seeker _BEHOLD + COND_ — Behold Elemental or pay {2}, grant trample, 3rd resolution adds RRRR

## 8. Dynamic Mana Production (MANA)

**Effort:** Medium | **Cards unlocked:** 8 | **Running total:** 121/181
> Color-dependent and conditional mana

- [ ] **Implement Dynamic Mana Production engine support**

### Single-dependency cards (4)

- [ ] Bloom Tender
- [ ] Flamebraider — Conditional mana (2 any color, only for Elemental spells/abilities)
- [ ] Great Forest Druid
- [ ] Springleaf Drum

### Multi-dependency cards (4) — now fully unblocked

- [ ] Firdoch Core _COND + MANA_ — Changeling, any-color mana, animated artifact (becomes 4/4 creature until EOT)
- [ ] Foraging Wickermaw _COND + MANA_ — Surveil 1, any-color mana + becomes that color until EOT, once per turn
- [ ] Lavaleaper _COND + MANA_ — All creatures have haste (global static), basic land mana doubling
- [ ] Lys Alana Dignitary _BEHOLD + MANA_ — Behold Elf or pay {2}, conditional mana (GG if Elf in GY)

## 9. Spell/Permanent Copy (COPY)

**Effort:** Hard | **Cards unlocked:** 7 | **Running total:** 128/181
> Stack manipulation, clone effects

- [ ] **Implement Spell/Permanent Copy engine support**

### Single-dependency cards (4)

- [ ] Kirol, Attentive First-Year — Tap 2 creatures cost, copy target triggered ability, once per turn
- [ ] Mirrorform
- [ ] Omni-Changeling — Changeling, Convoke, enter as copy of creature with changeling (clone effect)
- [ ] Spinerock Tyrant — Flying, Wither, copy instant/sorcery with single target + both gain wither

### Multi-dependency cards (3) — now fully unblocked

- [ ] Kindle the Inner Flame _BEHOLD + COPY_ — Token copy of creature with haste + end-step sacrifice, Flashback with behold 3 Elementals
- [ ] Rimefire Torque _COPY + TYPE_ — Choose creature type, charge counters on type ETB, remove 3 charges then copy next spell
- [ ] Twilight Diviner _COND + COPY_ — ETB surveil 2, creatures from GY entering then create token copy (once per turn)

## 10. Modal Spells (MODAL)

**Effort:** Hard | **Cards unlocked:** 7 | **Running total:** 135/181
> Mode selection + conditional resolution

- [ ] **Implement Modal Spells engine support**

### Single-dependency cards (6)

- [ ] Ashling's Command
- [ ] Auntie's Sentence
- [ ] Brigid's Command
- [ ] Grub's Command
- [ ] Perfect Intimidation — Choose one or both, exile 2 from opponent hand, remove all counters from creature
- [ ] Sygg's Command

### Multi-dependency cards (1) — now fully unblocked

- [ ] Unbury _MODAL + TYPE_ — Choose one: return creature from GY; or return 2 creatures sharing type from GY

## 11. Transform/DFC (TRANSFORM)

**Effort:** Hard | **Cards unlocked:** 6 | **Running total:** 141/181
> Double-faced card system

- [ ] **Implement Transform/DFC engine support**

### Single-dependency cards (4)

- [ ] Ashling, Rekindled — Transform/DFC system, discard-draw, conditional mana, BeginningOfMainPhase trigger
- [ ] Eirdu, Carrier of Dawn — Transform/DFC, creature spells have convoke, other creatures have persist
- [ ] Sygg, Wanderwine Wisdom — Transform/DFC, can't be blocked, grant combat-damage-draw, protection from colors
- [ ] Trystan, Callous Cultivator — Transform/DFC, mill + conditional gain life, exile Elf for opponents lose life

### Multi-dependency cards (2) — now fully unblocked

- [ ] Brigid, Clachan's Heart _MANA + TRANSFORM_ — Transform/DFC, create Kithkin token on ETB/transform, dynamic mana based on creature count
- [ ] Grub, Storied Matriarch _COST + TRANSFORM_ — Transform/DFC, return Goblin from GY, attacks blight then token copy tapped+attacking

## 12. Miscellaneous Effects (MISC)

**Effort:** Easy-Medium | **Cards unlocked:** 6 | **Running total:** 147/181
> Individual small effects (self-tap, land tokens, GY exile, library tuck)

- [x] **Implement Miscellaneous Effects engine support** — PutOnLibrary effect added

### Single-dependency cards (5)

- [x] Crib Swap — exile() typed, opponent token is Custom
- [x] Iron-Shield Elf — gain_keyword_eot("indestructible") typed, self-tap is Custom
- [ ] Mutable Explorer (land tokens — needs land token creation)
- [x] Rooftop Percher — gain_life(3) typed, GY exile is Custom
- [x] Temporal Cleansing — put_on_library() typed with PermanentFiltered targeting

### Multi-dependency cards (1) — now fully unblocked

- [ ] Deepway Navigator _COND + MISC_

## 13. Evoke Mechanic (EVOKE)

**Effort:** Medium | **Cards unlocked:** 5 | **Running total:** 152/181
> Alternative cost + ETB + sacrifice on resolution

- [ ] **Implement Evoke Mechanic engine support**

### Single-dependency cards (3)

- [ ] Catharsis
- [ ] Deceit
- [ ] Emptiness

### Multi-dependency cards (2) — now fully unblocked

- [ ] Vibrance _COND + EVOKE_ — Evoke, conditional ETB (if RR then 3 damage, if GG then search land + gain 2 life)
- [ ] Wistfulness _COND + EVOKE_ — Evoke, conditional ETB (if GG exile artifact/enchantment, if UU draw 2 discard 1)

## 14. Impulse Draw (IMPULSE)

**Effort:** Hard | **Cards unlocked:** 5 | **Running total:** 157/181
> Exile zone play permissions

- [ ] **Implement Impulse Draw engine support**

### Single-dependency cards (3)

- [ ] Burning Curiosity
- [ ] Kulrath Zealot
- [ ] Sizzling Changeling

### Multi-dependency cards (2) — now fully unblocked

- [ ] Sanar, Innovative First-Year _IMPULSE + VIVID_ — Vivid reveal X nonland cards, exile one per color, cast this turn
- [ ] Shadow Urchin _COST + IMPULSE_ — Attacks then blight 1, creature with counters dies then impulse draw equal to counter count

## 15. Eclipsed Cycle (ECLIPSED)

**Effort:** Medium | **Cards unlocked:** 5 | **Running total:** 162/181
> Look at top 4, reveal matching to hand

- [x] **Implement Eclipsed Cycle engine support** — `Effect::LookTopAndPick { count, filter }` + game.rs match arm

### Single-dependency cards (5)

- [x] Eclipsed Boggart — look_top_and_pick(4, "Goblin or Swamp or Mountain")
- [x] Eclipsed Elf — look_top_and_pick(4, "Elf or Swamp or Forest")
- [x] Eclipsed Flamekin — look_top_and_pick(4, "Elemental or Island or Mountain")
- [x] Eclipsed Kithkin — look_top_and_pick(4, "Kithkin or Forest or Plains")
- [x] Eclipsed Merrow — look_top_and_pick(4, "Merfolk or Plains or Island")

## 16. Shock Lands (SHOCK)

**Effort:** Medium | **Cards unlocked:** 5 | **Running total:** 167/181
> ETB replacement effect: pay 2 life or enters tapped

- [x] **Implement Shock Lands engine support** — EntersTappedUnless + life payment implemented

### Single-dependency cards (5)

- [x] Blood Crypt
- [x] Hallowed Fountain
- [x] Overgrown Tomb
- [x] Steam Vents
- [x] Temple Garden

## 17. Cast from Exile (EXILE_CAST)

**Effort:** Hard | **Cards unlocked:** 4 | **Running total:** 171/181
> Exile zone cast permissions, play-until-EOT

- [ ] **Implement Cast from Exile engine support**

### Single-dependency cards (2)

- [ ] Dawnhand Dissident
- [ ] Dream Harvest — Exile from opponent libraries until MV>=5, cast from exile without paying costs until EOT

### Multi-dependency cards (2) — now fully unblocked

- [ ] Goliath Daydreamer _COND + EXILE_CAST_ — Replacement effect (exile with dream counter instead of GY), attacks trigger cast from exile free
- [ ] Maralen, Fae Ascendant _COND + EXILE_CAST_ — ETB this/Elf/Faerie exile opponent top 2, cast from exile with MV restriction, once per turn

## 18. Equipment System (EQUIP)

**Effort:** Hard | **Cards unlocked:** 4 | **Running total:** 175/181
> Attach/detach/equip system

- [ ] **Implement Equipment System engine support**

### Single-dependency cards (4)

- [ ] Barbed Bloodletter
- [ ] Bark of Doran
- [ ] Mirrormind Crown
- [ ] Stalactite Dagger

## 19. Delayed Triggers (DELAYED)

**Effort:** Medium | **Cards unlocked:** 3 | **Running total:** 178/181
> One-shot delayed trigger registration

- [ ] **Implement Delayed Triggers engine support**

### Single-dependency cards (1)

- [ ] Scarblades Malice

### Multi-dependency cards (2) — now fully unblocked

- [ ] End-Blaze Epiphany _DELAYED + IMPULSE_ — X damage, delayed trigger on creature death, exile cards equal to power, play until next turn end
- [ ] Flitterwing Nuisance _COST + DELAYED_ — ETB with -1/-1 counter, remove counter cost, delayed trigger (combat damage draw)

## 20. Planeswalker System (PW)

**Effort:** Hard | **Cards unlocked:** 2 | **Running total:** 180/181
> Loyalty counters, loyalty abilities, emblem

- [ ] **Implement Planeswalker System engine support**

### Single-dependency cards (1)

- [ ] Ajani, Outland Chaperone

### Multi-dependency cards (1) — now fully unblocked

- [ ] Oko, Lorwyn Liege _PW + TRANSFORM_ — Transform/DFC Planeswalker, loyalty abilities, mill, token creation, emblem

## 21. Gain Control (CONTROL)

**Effort:** Medium | **Cards unlocked:** 1 | **Running total:** 181/181
> Control-change effect

- [x] **Implement Gain Control engine support** — GainControl + GainControlUntilEndOfTurn match arms + cleanup revert

### Single-dependency cards (1)

- [x] Goatnap — gain_control_eot() typed with Creature targeting

---

## Schedule Summary

| # | Engine Capability | Effort | Single | Multi | Cumulative |
|---|---|---|---|---|---|
| 1 | Conditional/Dynamic Effects | Hard | 51 | 0 | 51/181 |
| 2 | Cost System | Medium | 12 | 3 | 66/181 |
| 3 | Vivid Mechanic | Medium | 10 | 1 | 77/181 |
| 4 | Player Choice | Medium | 7 | 3 | 87/181 |
| 5 | Creature Type Choice | Medium | 5 | 4 | 96/181 |
| 6 | Aura System | Hard | 9 | 0 | 105/181 |
| 7 | Behold Mechanic | Hard | 2 | 6 | 113/181 |
| 8 | Dynamic Mana Production | Medium | 4 | 4 | 121/181 |
| 9 | Spell/Permanent Copy | Hard | 4 | 3 | 128/181 |
| 10 | Modal Spells | Hard | 6 | 1 | 135/181 |
| 11 | Transform/DFC | Hard | 4 | 2 | 141/181 |
| 12 | Miscellaneous Effects | Easy-Medium | 5 | 1 | 147/181 |
| 13 | Evoke Mechanic | Medium | 3 | 2 | 152/181 |
| 14 | Impulse Draw | Hard | 3 | 2 | 157/181 |
| 15 | Eclipsed Cycle | Medium | 5 | 0 | 162/181 |
| 16 | Shock Lands | Medium | 5 | 0 | 167/181 |
| 17 | Cast from Exile | Hard | 2 | 2 | 171/181 |
| 18 | Equipment System | Hard | 4 | 0 | 175/181 |
| 19 | Delayed Triggers | Medium | 1 | 2 | 178/181 |
| 20 | Planeswalker System | Hard | 1 | 1 | 180/181 |
| 21 | Gain Control | Medium | 1 | 0 | 181/181 |
