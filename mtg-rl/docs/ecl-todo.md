# ECL Implementation Todo

Ordered by topological sort of the dependency graph. Engine capabilities are
scheduled greedily: each step picks the capability that unlocks the most new
cards. Multi-dependency cards appear under their last-needed capability.

**212 cards complete** | **56 cards remaining** across **21 engine capabilities**

## 1. Conditional/Dynamic Effects (COND)

**Effort:** Hard | **Cards unlocked:** 51 | **Running total:** 51/181
> Various state-dependent abilities, watchers, replacement effects

- [x] **Implement Conditional/Dynamic Effects engine support** — All COND cards now have typed effects where possible; remaining complex behaviors described as Custom

### Single-dependency cards (51)

- [x] Abigale, Eloquent First-Year — Keywords typed, ETB add_counters for keyword counters typed, LoseAllAbilities as Custom
- [x] Boneclub Berserker — StaticEffect Custom for dynamic +2/+0 per Goblin (no dynamic P/T system yet)
- [x] Boulder Dash — DealDamage split (2+1) as Custom (multi-target damage not supported)
- [x] Bre of Clan Stoutarm — activated flying+lifelink EOT typed, triggered reanimate typed (life-gain condition not enforced)
- [x] Bristlebane Outrider — Daunt + conditional +2/+0 as Custom statics (watcher not implemented)
- [x] Curious Colossus — ETB mass lose-abilities + set P/T as Custom
- [x] Dawnhand Eulogist — mill(3) + lose_life_opponents(2) + gain_life(2) (conditional Elf check not enforced)
- [x] Doran, Besieged by Time — EventType triggers typed, cost reduction + dynamic boost as Custom
- [x] Feisty Spikeling — Changeling keyword typed, conditional first strike as Custom (turn check not enforced)
- [x] Figure of Fable — Level 1 SetPowerToughness typed, levels 2-3 conditional as Custom
- [x] Formidable Speaker — activated untap typed with Permanent targeting, ETB discard-to-search is Custom
- [x] Gallant Fowlknight
- [x] Glamer Gifter — SetPowerToughness typed, Flash+Flying keywords, all-creature-types is Custom
- [x] Glen Elendra's Answer — Counter all opponent spells + conditional tokens as Custom
- [x] Gloom Ripper — ETB trigger typed, dynamic X boost as Custom
- [x] Gravelgill Scoundrel — Attacks trigger DoIfCostPaid(tap creature, unblockable) typed, Vigilance keyword
- [x] Hexing Squelcher — Ward typed, GrantKeyword for other creatures typed, cant-be-countered descriptions
- [x] Illusion Spinners — Flying+Hexproof keywords set, conditional flash/hexproof described as Custom
- [x] Impolite Entrance — Target creature gains trample+haste until EOT, draw a card
- [x] Kinbinding — BeginCombat create_token typed, dynamic +X/+X static as Custom (watcher not enforced)
- [x] Kinscaer Sentry — First strike + lifelink keywords, attacks trigger typed (cheat from hand is Custom)
- [x] Lasting Tarfire — end step damage_opponents(2) (conditional counter check not enforced)
- [x] Lluwen, Imperfect Naturalist — Mill + top-library + tokens as Custom (very complex)
- [x] Meanders Guide — AttackerDeclared trigger typed (tap Merfolk + reanimate as Custom)
- [x] Meek Attack — Activated ability with pay_mana cost typed (put creature from hand is Custom)
- [x] Moon-Vigil Adherents — Trample keyword typed, dynamic +1/+1 per creature as Custom
- [x] Morcant's Eyes — upkeep scry(1) typed, activated sac+mana with X tokens is Custom
- [x] Morcant's Loyalist — Other Elves +1/+1 lord, dies then return another Elf card from GY to hand
- [x] Morningtide's Light — Mass flicker + prevent damage as Custom
- [x] Mornsong Aria — fully typed: CantGainLife + CantDrawExtraCards statics, LoseLife + search_library triggered
- [x] Personify — flicker Custom + create_token typed, targets CreatureYouControl
- [x] Pummeler for Hire — Ward {2} typed, ETB gain life dynamic is Custom
- [x] Raiding Schemes — Conspire static as Custom (very complex stack manipulation)
- [x] Retched Wretch — Dies trigger with reanimate() typed, lose-abilities + counter condition as Custom
- [x] Rhys, the Evermore — Flash keyword, ETB gain_keyword_eot("persist") typed, activated remove-counters with pay_mana+TapSelf costs
- [x] Riverguard's Reflexes — +2/+2 + first strike until EOT + untap target creature
- [x] Safewright Cavalry — CantBlock annotation, activated boost_until_eot(2,2) typed with Elf targeting
- [x] Sapling Nursery — CostReduction + landfall create_token typed, activated exile-self as Custom
- [x] Spry and Mighty — Choose 2 creatures + dynamic effects as Custom
- [x] Sunderflock — CostReduction + conditional bounce as Custom
- [x] Swat Away — CostReduction + library tuck as Custom
- [x] Tam, Mindful First-Year — Hexproof from colors + all-colors as Custom
- [x] Taster of Wares — ETB reveal + exile + cast as Custom
- [x] Tend the Sprigs — search_library("basic land") typed, conditional Treefolk token as Custom
- [x] Thoughtweft Imbuer — AttackerDeclared trigger typed, dynamic +X/+X as Custom
- [x] Tributary Vaulter — Becomes tapped trigger then another Merfolk gets +2/+0 until EOT
- [x] Twinflame Travelers — Triggered ability doubling as Custom (replacement effect)
- [x] Vinebred Brawler — Must be blocked if able, attacks then another Elf gets +2/+1
- [x] Wanderbrine Trapper — Activated: {1}, T, tap another creature then tap opponent creature
- [x] Wanderwine Farewell — Convoke keyword, bounce() typed, conditional Merfolk tokens as Custom
- [x] Wary Farmer — end step scry(1) (conditional creature-entry check not enforced)

## 2. Cost System (COST)

**Effort:** Medium | **Cards unlocked:** 15 | **Running total:** 66/181
> RemoveCounters, Blight, ExileFromGraveyard pay_costs() match arms

- [x] **Implement Cost System engine support** — RemoveCounters, Blight, ExileFromGraveyard, ExileFromHand, SacrificeOther, UntapSelf match arms in pay_costs()

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

- [x] High Perfect Morcant _COND + COST_ — ETB trigger + tap-3 activated typed, blight opponents + proliferate as Custom
- [x] Moonshadow _COND + COST_ — Menace keyword, ETB add_counters 6 -1/-1, Dies trigger RemoveCounters typed
- [x] Slumbering Walker _COND + COST_ — ETB 2 -1/-1 counters, end step DoIfCostPaid(RemoveCounters, reanimate)

## 3. Vivid Mechanic (VIVID)

**Effort:** Medium | **Cards unlocked:** 11 | **Running total:** 77/181
> Color-counting helper → X, effect variants

- [x] **Implement Vivid Mechanic engine support** — count_colors_among_permanents() helper + DealDamageVivid, GainLifeVivid, BoostUntilEotVivid, LoseLifeOpponentsVivid, DrawCardsVivid, BoostAllUntilEotVivid effect variants

### Single-dependency cards (10)

- [x] Aurora Awakener — Vivid ETB described (complex reveal-and-put effect is Custom)
- [x] Explosive Prodigy — deal_damage_vivid() typed ETB
- [x] Glister Bairn — boost_until_eot_vivid() typed begin-combat trigger
- [x] Luminollusk — gain_life_vivid() typed ETB
- [x] Prismabasher — boost_all_until_eot_vivid() typed ETB
- [x] Prismatic Undercurrents — Vivid ETB search described, additional land static described
- [x] Shimmercreep — lose_life_opponents_vivid() + gain_life_vivid() typed ETB
- [x] Shinestriker — draw_cards_vivid() typed ETB
- [x] Squawkroaster — Vivid power described as static (dynamic set not enforced)
- [x] Wildvine Pummeler — Vivid CostReduction static, Reach + Trample keywords

### Multi-dependency cards (1) — now fully unblocked

- [x] Kithkeeper _COND + VIVID_ — Vivid create_token_vivid("1/1 Kithkin") ETB typed, tap-3 activated +3/+0+flying typed

## 4. Player Choice (CHOICE)

**Effort:** Medium | **Cards unlocked:** 10 | **Running total:** 87/181
> Runtime choice/decision framework

- [x] **Implement Player Choice engine support** — Added `Effect::DoIfCostPaid { cost, if_paid, if_not_paid }` variant + match arm. Uses `choose_use()` for yes/no decision, then `pay_costs()` if accepted. 2 tests added.

### Single-dependency cards (7)

- [x] Boggart Mischief — ETB DoIfCostPaid(Blight 1, create 2 Goblin tokens). Dies trigger already typed.
- [x] Chaos Spewer — ETB DoIfCostPaid(pay {2}, else blight 2 self)
- [x] Glamermite — Modal (tap/untap target creature), Flash+Flying keywords
- [x] Gutsplitter Gang — Precombat main DoIfCostPaid(Blight 2, else lose 3 life)
- [x] Scuzzback Scrounger — Precombat main DoIfCostPaid(Blight 1, create Treasure)
- [x] Thirst for Identity — draw_cards(3) + DoIfCostPaid(Discard 1, else discard 2). Note: creature-type filter not enforced.
- [x] Voracious Tome-Skimmer — SpellCast trigger DoIfCostPaid(PayLife 1, draw 1). Note: opponent-turn condition not enforced.

### Multi-dependency cards (3) — now fully unblocked

- [x] Puca's Eye _CHOICE + VIVID_ — ETB draw + color choice Custom, activated {3},{T}: draw (5-color condition not enforced)
- [x] Rime Chill _CHOICE + VIVID_ — TapTarget + AddCounters(stun) + draw. Vivid cost reduction not enforced.
- [x] Soul Immolation _CHOICE + COST_ — Variable blight cost described, deal X damage as Custom (X-cost not enforced)

## 5. Creature Type Choice (TYPE)

**Effort:** Medium | **Cards unlocked:** 9 | **Running total:** 96/181
> Choose-type + conditional application, Convoke

- [x] **Implement Creature Type Choice engine support** — Added `Effect::ChooseCreatureType { restricted }` + `ChooseTypeAndDrawPerPermanent` variants with match arms. Stores chosen type on permanent's `chosen_type` field via `choose_option()`. 3 tests added.

### Single-dependency cards (5)

- [x] Chronicle of Victory — choose_creature_type() typed, statics as annotation, SpellCast draw typed
- [x] Collective Inferno — choose_creature_type() typed, Convoke keyword, damage doubling is Custom
- [x] Dawn-Blessed Pennant — choose_creature_type_restricted() (8 ECL types), ETB gain_life typed, activated return_from_graveyard typed
- [x] Eclipsed Realms — choose_creature_type_restricted() ETB, mana abilities typed (conditional spending not enforced)
- [x] Harmonized Crescendo — choose_type_and_draw_per_permanent() fully typed, Convoke keyword

### Multi-dependency cards (4) — now fully unblocked

- [x] Bloodline Bidding _COND + TYPE_ — Convoke keyword, choose_creature_type() typed, mass reanimate as Custom
- [x] Gathering Stone _CHOICE + TYPE_ — choose_creature_type() typed, CostReduction static, look-at-top is Custom
- [x] Selfless Safewright _COND + TYPE_ — Flash+Convoke keywords, choose_creature_type() typed, grant hexproof+indestructible as Custom
- [x] Winnowing _COND + TYPE_ — Convoke keyword typed, choose-and-sacrifice logic as Custom

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

- [x] **Implement Modal Spells engine support** — `Effect::Modal { modes, min_modes, max_modes }` + `ModalMode` struct + `execute_effects()` match arm using `choose_mode()`

### Single-dependency cards (6)

- [x] Ashling's Command
- [x] Auntie's Sentence
- [x] Brigid's Command
- [x] Grub's Command
- [x] Perfect Intimidation — Choose one or both, exile 2 from opponent hand, remove all counters from creature
- [x] Sygg's Command

### Multi-dependency cards (1) — now fully unblocked

- [x] Unbury _MODAL + TYPE_ — Choose one: return creature from GY; or return 2 creatures sharing type from GY

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
- [x] Mutable Explorer — create_token("Mutavault") typed ETB, Changeling keyword (land token type not enforced)
- [x] Rooftop Percher — gain_life(3) typed, GY exile is Custom
- [x] Temporal Cleansing — put_on_library() typed with PermanentFiltered targeting

### Multi-dependency cards (1) — now fully unblocked

- [x] Deepway Navigator _COND + MISC_ — Flash keyword, ETB untap Custom, conditional Merfolk +1/+0 as boost_controlled (watcher condition not enforced)

## 13. Evoke Mechanic (EVOKE)

**Effort:** Medium | **Cards unlocked:** 5 | **Running total:** 152/181
> Alternative cost + ETB + sacrifice on resolution

- [x] **Implement Evoke Mechanic engine support** — Added StaticEffect::Evoke { cost } structured variant + evoke() builder. ETB effects already typed; mana color conditions not enforced.

### Single-dependency cards (3)

- [x] Catharsis — ETB create_token + boost_all_eot + grant_keyword_all_eot typed, Evoke typed
- [x] Deceit — ETB bounce + discard typed, Evoke typed
- [x] Emptiness — ETB reanimate + add_counters typed, Evoke typed

### Multi-dependency cards (2) — now fully unblocked

- [x] Vibrance _COND + EVOKE_ — ETB deal_damage + search_library + gain_life typed, Evoke typed. Color conditions not enforced.
- [x] Wistfulness _COND + EVOKE_ — ETB exile + draw + discard typed, Evoke typed. Color conditions not enforced.

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
