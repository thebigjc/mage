# ECL Dependency Graph

This diagram shows the work needed to complete all 267 ECL cards. Cards are leaves, engine capabilities are interior nodes, and "ECL Complete" is the root.

- **86 cards** are truly complete (direct edge to root)
- **181 cards** need engine capabilities or implementation work
- Cards needing multiple capabilities have edges to each one

```mermaid
graph LR
  ROOT(("ECL Complete<br/>267 cards"))

  %% ===== ENGINE CAPABILITY NODES =====
  COST["Cost System<br/>(RemoveCounters, Blight,<br/>ExileFromGraveyard)<br/>21 cards"]
  VIVID["Vivid Mechanic<br/>(color counting → X)<br/>14 cards"]
  MODAL["Modal Spells<br/>(choose one/two)<br/>7 cards"]
  EQUIP["Equipment System<br/>(attach/detach/equip)<br/>4 cards"]
  EVOKE["Evoke Mechanic<br/>(alt cost + sacrifice)<br/>5 cards"]
  ECLIPSED["Eclipsed Cycle<br/>(look at top 4,<br/>reveal to hand)<br/>5 cards"]
  IMPULSE["Impulse Draw<br/>(exile-and-play)<br/>6 cards"]
  BEHOLD["Behold Mechanic<br/>(reveal+exile,<br/>LTB return)<br/>10 cards"]
  SHOCK["Shock Lands<br/>(ETB replacement:<br/>pay 2 life or tapped)<br/>5 cards"]
  COND["Conditional/Dynamic<br/>Effects<br/>70 cards"]
  AURA["Aura System<br/>(enchant creature)<br/>9 cards"]
  CHOICE["Player Choice<br/>(runtime decisions)<br/>11 cards"]
  TYPE["Creature Type<br/>Choice<br/>12 cards"]
  MANA["Dynamic Mana<br/>Production<br/>9 cards"]
  PW["Planeswalker<br/>System<br/>2 cards"]
  CONTROL["Gain Control<br/>1 card"]
  DELAYED["Delayed<br/>Triggers<br/>3 cards"]
  COPY["Spell/Permanent<br/>Copy<br/>7 cards"]
  EXILE_CAST["Cast from<br/>Exile<br/>4 cards"]
  MISC["Miscellaneous<br/>Effects<br/>(self-tap, land tokens,<br/>GY exile, library tuck,<br/>token-for-controller)<br/>6 cards"]
  TRANSFORM["Transform/DFC<br/>(double-faced cards)<br/>7 cards"]

  %% ===== ENGINE → ROOT =====
  COST --> ROOT
  VIVID --> ROOT
  MODAL --> ROOT
  EQUIP --> ROOT
  EVOKE --> ROOT
  ECLIPSED --> ROOT
  IMPULSE --> ROOT
  BEHOLD --> ROOT
  SHOCK --> ROOT
  COND --> ROOT
  AURA --> ROOT
  CHOICE --> ROOT
  TYPE --> ROOT
  MANA --> ROOT
  PW --> ROOT
  CONTROL --> ROOT
  DELAYED --> ROOT
  COPY --> ROOT
  EXILE_CAST --> ROOT
  MISC --> ROOT
  TRANSFORM --> ROOT

  %% ===== COMPLETE CARDS → ROOT (86) =====
  adept_watershaper["Adept Watershaper"] --> ROOT
  appeal_to_eirdu["Appeal to Eirdu"] --> ROOT
  assert_perfection["Assert Perfection"] --> ROOT
  bile_vial_boggart["Bile-Vial Boggart"] --> ROOT
  bitterbloom_bearer["Bitterbloom Bearer"] --> ROOT
  blight_rot["Blight Rot"] --> ROOT
  blighted_blackthorn["Blighted Blackthorn"] --> ROOT
  blossoming_defense["Blossoming Defense"] --> ROOT
  boggart_cursecrafter["Boggart Cursecrafter"] --> ROOT
  boggart_prankster["Boggart Prankster"] --> ROOT
  boldwyr_aggressor["Boldwyr Aggressor"] --> ROOT
  brambleback_brute["Brambleback Brute"] --> ROOT
  bristlebane_battler["Bristlebane Battler"] --> ROOT
  burdened_stoneback["Burdened Stoneback"] --> ROOT
  changeling_wayfinder["Changeling Wayfinder"] --> ROOT
  chitinous_graspling["Chitinous Graspling"] --> ROOT
  chomping_changeling["Chomping Changeling"] --> ROOT
  cinder_strike["Cinder Strike"] --> ROOT
  clachan_festival["Clachan Festival"] --> ROOT
  crossroads_watcher["Crossroads Watcher"] --> ROOT
  darkness_descends["Darkness Descends"] --> ROOT
  dawns_light_archer["Dawn's Light Archer"] --> ROOT
  deepchannel_duelist["Deepchannel Duelist"] --> ROOT
  disruptor_of_currents["Disruptor of Currents"] --> ROOT
  dose_of_dawnglow["Dose of Dawnglow"] --> ROOT
  dream_seizer["Dream Seizer"] --> ROOT
  dundoolin_weaver["Dundoolin Weaver"] --> ROOT
  elder_auntie["Elder Auntie"] --> ROOT
  encumbered_reejerey["Encumbered Reejerey"] --> ROOT
  enraged_flamecaster["Enraged Flamecaster"] --> ROOT
  feed_the_flames["Feed the Flames"] --> ROOT
  flame_chain_mauler["Flame-Chain Mauler"] --> ROOT
  flamekin_gildweaver["Flamekin Gildweaver"] --> ROOT
  flaring_cinder["Flaring Cinder"] --> ROOT
  flock_impostor["Flock Impostor"] --> ROOT
  gangly_stompling["Gangly Stompling"] --> ROOT
  giantfall["Giantfall"] --> ROOT
  graveshifter["Graveshifter"] --> ROOT
  heirloom_auntie["Heirloom Auntie"] --> ROOT
  keep_out["Keep Out"] --> ROOT
  kinsbaile_aspirant["Kinsbaile Aspirant"] --> ROOT
  kulrath_mystic["Kulrath Mystic"] --> ROOT
  liminal_hold["Liminal Hold"] --> ROOT
  lys_alana_informant["Lys Alana Informant"] --> ROOT
  merrow_skyswimmer["Merrow Skyswimmer"] --> ROOT
  midnight_tilling["Midnight Tilling"] --> ROOT
  mischievous_sneakling["Mischievous Sneakling"] --> ROOT
  mistmeadow_council["Mistmeadow Council"] --> ROOT
  moonglove_extractor["Moonglove Extractor"] --> ROOT
  nameless_inversion["Nameless Inversion"] --> ROOT
  nightmare_sower["Nightmare Sower"] --> ROOT
  noggle_robber["Noggle Robber"] --> ROOT
  pestered_wellguard["Pestered Wellguard"] --> ROOT
  prideful_feastling["Prideful Feastling"] --> ROOT
  protective_response["Protective Response"] --> ROOT
  pyrrhic_strike["Pyrrhic Strike"] --> ROOT
  reckless_ransacking["Reckless Ransacking"] --> ROOT
  reluctant_dounguard["Reluctant Dounguard"] --> ROOT
  rimekin_recluse["Rimekin Recluse"] --> ROOT
  run_away_together["Run Away Together"] --> ROOT
  scarblade_scout["Scarblade Scout"] --> ROOT
  sear["Sear"] --> ROOT
  shore_lurker["Shore Lurker"] --> ROOT
  silvergill_mentor["Silvergill Mentor"] --> ROOT
  silvergill_peddler["Silvergill Peddler"] --> ROOT
  sourbread_auntie["Sourbread Auntie"] --> ROOT
  spell_snare["Spell Snare"] --> ROOT
  sting_slinger["Sting-Slinger"] --> ROOT
  stratosoarer["Stratosoarer"] --> ROOT
  summit_sentinel["Summit Sentinel"] --> ROOT
  sun_dappled_celebrant["Sun-Dappled Celebrant"] --> ROOT
  surly_farrier["Surly Farrier"] --> ROOT
  tanufel_rimespeaker["Tanufel Rimespeaker"] --> ROOT
  thoughtweft_charge["Thoughtweft Charge"] --> ROOT
  thoughtweft_lieutenant["Thoughtweft Lieutenant"] --> ROOT
  timid_shieldbearer["Timid Shieldbearer"] --> ROOT
  trystans_command["Trystan's Command"] --> ROOT
  tweeze["Tweeze"] --> ROOT
  unexpected_assistance["Unexpected Assistance"] --> ROOT
  unforgiving_aim["Unforgiving Aim"] --> ROOT
  unwelcome_sprite["Unwelcome Sprite"] --> ROOT
  virulent_emissary["Virulent Emissary"] --> ROOT
  wanderbrine_preacher["Wanderbrine Preacher"] --> ROOT
  wanderwine_distracter["Wanderwine Distracter"] --> ROOT
  warren_torchmaster["Warren Torchmaster"] --> ROOT
  wild_unraveling["Wild Unraveling"] --> ROOT

  %% ===== COST SYSTEM (21 cards total) =====
  gnarlbark_elm["Gnarlbark Elm"] --> COST
  moonlit_lamenter["Moonlit Lamenter"] --> COST
  gristle_glutton["Gristle Glutton"] --> COST
  goldmeadow_nomad["Goldmeadow Nomad"] --> COST
  stoic_grove_guide["Stoic Grove-Guide"] --> COST
  creakwood_safewright["Creakwood Safewright"] --> COST
  hovel_hurler["Hovel Hurler"] --> COST
  %% --- triaged from research ---
  bogslithers_embrace["Bogslither's Embrace"] --> COST
  glen_elendra_guardian["Glen Elendra Guardian"] --> COST
  loch_mare["Loch Mare"] --> COST
  reaping_willow["Reaping Willow"] --> COST
  requiting_hex["Requiting Hex"] --> COST

  %% ===== VIVID MECHANIC (14 cards total) =====
  explosive_prodigy["Explosive Prodigy"] --> VIVID
  glister_bairn["Glister Bairn"] --> VIVID
  luminollusk["Luminollusk"] --> VIVID
  prismabasher["Prismabasher"] --> VIVID
  shimmercreep["Shimmercreep"] --> VIVID
  shinestriker["Shinestriker"] --> VIVID
  squawkroaster["Squawkroaster"] --> VIVID
  aurora_awakener["Aurora Awakener"] --> VIVID
  %% --- triaged from research ---
  prismatic_undercurrents["Prismatic Undercurrents"] --> VIVID
  wildvine_pummeler["Wildvine Pummeler"] --> VIVID

  %% ===== MODAL SPELLS (7 cards total) =====
  grubs_command["Grub's Command"] --> MODAL
  ashlings_command["Ashling's Command"] --> MODAL
  aunties_sentence["Auntie's Sentence"] --> MODAL
  brigids_command["Brigid's Command"] --> MODAL
  syggs_command["Sygg's Command"] --> MODAL
  %% --- triaged from research ---
  perfect_intimidation["Perfect Intimidation"] --> MODAL

  %% ===== EQUIPMENT SYSTEM (4 cards total) =====
  barbed_bloodletter["Barbed Bloodletter"] --> EQUIP
  bark_of_doran["Bark of Doran"] --> EQUIP
  stalactite_dagger["Stalactite Dagger"] --> EQUIP
  mirrormind_crown["Mirrormind Crown"] --> EQUIP

  %% ===== EVOKE MECHANIC (5 cards total) =====
  catharsis["Catharsis"] --> EVOKE
  deceit["Deceit"] --> EVOKE
  emptiness["Emptiness"] --> EVOKE

  %% ===== ECLIPSED CYCLE (5 cards total) =====
  eclipsed_boggart["Eclipsed Boggart"] --> ECLIPSED
  eclipsed_elf["Eclipsed Elf"] --> ECLIPSED
  eclipsed_flamekin["Eclipsed Flamekin"] --> ECLIPSED
  eclipsed_kithkin["Eclipsed Kithkin"] --> ECLIPSED
  eclipsed_merrow["Eclipsed Merrow"] --> ECLIPSED

  %% ===== IMPULSE DRAW (6 cards total) =====
  kulrath_zealot["Kulrath Zealot"] --> IMPULSE
  sizzling_changeling["Sizzling Changeling"] --> IMPULSE
  burning_curiosity["Burning Curiosity"] --> IMPULSE

  %% ===== BEHOLD MECHANIC (10 cards total) =====
  champions_of_the_perfect["Champions of the Perfect"] --> BEHOLD
  champion_of_the_clachan["Champion of the Clachan"] --> BEHOLD

  %% ===== SHOCK LANDS (5 cards total) =====
  blood_crypt["Blood Crypt"] --> SHOCK
  hallowed_fountain["Hallowed Fountain"] --> SHOCK
  overgrown_tomb["Overgrown Tomb"] --> SHOCK
  steam_vents["Steam Vents"] --> SHOCK
  temple_garden["Temple Garden"] --> SHOCK

  %% ===== CONDITIONAL/DYNAMIC EFFECTS (70 cards total) =====
  feisty_spikeling["Feisty Spikeling"] --> COND
  gallant_fowlknight["Gallant Fowlknight"] --> COND
  thoughtweft_imbuer["Thoughtweft Imbuer"] --> COND
  safewright_cavalry["Safewright Cavalry"] --> COND
  bristlebane_outrider["Bristlebane Outrider"] --> COND
  bre_of_clan_stoutarm["Bre of Clan Stoutarm"] --> COND
  doran_besieged_by_time["Doran, Besieged by Time"] --> COND
  boneclub_berserker["Boneclub Berserker"] --> COND
  moon_vigil_adherents["Moon-Vigil Adherents"] --> COND
  lasting_tarfire["Lasting Tarfire"] --> COND
  %% --- triaged from research ---
  abigale_eloquent_first_year["Abigale, Eloquent First-Year"] --> COND
  boulder_dash["Boulder Dash"] --> COND
  curious_colossus["Curious Colossus"] --> COND
  dawnhand_eulogist["Dawnhand Eulogist"] --> COND
  figure_of_fable["Figure of Fable"] --> COND
  formidable_speaker["Formidable Speaker"] --> COND
  glamer_gifter["Glamer Gifter"] --> COND
  glen_elendras_answer["Glen Elendra's Answer"] --> COND
  gloom_ripper["Gloom Ripper"] --> COND
  gravelgill_scoundrel["Gravelgill Scoundrel"] --> COND
  hexing_squelcher["Hexing Squelcher"] --> COND
  illusion_spinners["Illusion Spinners"] --> COND
  impolite_entrance["Impolite Entrance"] --> COND
  kinbinding["Kinbinding"] --> COND
  kinscaer_sentry["Kinscaer Sentry"] --> COND
  lluwen_imperfect_naturalist["Lluwen, Imperfect Naturalist"] --> COND
  meanders_guide["Meanders Guide"] --> COND
  meek_attack["Meek Attack"] --> COND
  morcants_eyes["Morcant's Eyes"] --> COND
  morcants_loyalist["Morcant's Loyalist"] --> COND
  morningtides_light["Morningtide's Light"] --> COND
  mornsong_aria["Mornsong Aria"] --> COND
  personify["Personify"] --> COND
  pummeler_for_hire["Pummeler for Hire"] --> COND
  raiding_schemes["Raiding Schemes"] --> COND
  retched_wretch["Retched Wretch"] --> COND
  rhys_the_evermore["Rhys, the Evermore"] --> COND
  riverguards_reflexes["Riverguard's Reflexes"] --> COND
  sapling_nursery["Sapling Nursery"] --> COND
  spry_and_mighty["Spry and Mighty"] --> COND
  sunderflock["Sunderflock"] --> COND
  swat_away["Swat Away"] --> COND
  tam_mindful_first_year["Tam, Mindful First-Year"] --> COND
  taster_of_wares["Taster of Wares"] --> COND
  tend_the_sprigs["Tend the Sprigs"] --> COND
  tributary_vaulter["Tributary Vaulter"] --> COND
  twinflame_travelers["Twinflame Travelers"] --> COND
  vinebred_brawler["Vinebred Brawler"] --> COND
  wanderbrine_trapper["Wanderbrine Trapper"] --> COND
  wanderwine_farewell["Wanderwine Farewell"] --> COND
  wary_farmer["Wary Farmer"] --> COND

  %% ===== AURA SYSTEM (9 cards total) =====
  evershrikes_gift["Evershrike's Gift"] --> AURA
  gilt_leafs_embrace["Gilt-Leaf's Embrace"] --> AURA
  lofty_dreams["Lofty Dreams"] --> AURA
  pitiless_fists["Pitiless Fists"] --> AURA
  blossombind["Blossombind"] --> AURA
  shimmerwilds_growth["Shimmerwilds Growth"] --> AURA
  noggle_the_mind["Noggle the Mind"] --> AURA
  spiral_into_solitude["Spiral into Solitude"] --> AURA
  aquitects_defenses["Aquitect's Defenses"] --> AURA

  %% ===== PLAYER CHOICE (11 cards total) =====
  chaos_spewer["Chaos Spewer"] --> CHOICE
  glamermite["Glamermite"] --> CHOICE
  gutsplitter_gang["Gutsplitter Gang"] --> CHOICE
  scuzzback_scrounger["Scuzzback Scrounger"] --> CHOICE
  voracious_tome_skimmer["Voracious Tome-Skimmer"] --> CHOICE
  boggart_mischief["Boggart Mischief"] --> CHOICE
  %% --- triaged from research ---
  thirst_for_identity["Thirst for Identity"] --> CHOICE

  %% ===== CREATURE TYPE CHOICE (12 cards total) =====
  chronicle_of_victory["Chronicle of Victory"] --> TYPE
  collective_inferno["Collective Inferno"] --> TYPE
  dawn_blessed_pennant["Dawn-Blessed Pennant"] --> TYPE
  eclipsed_realms["Eclipsed Realms"] --> TYPE
  %% --- triaged from research ---
  harmonized_crescendo["Harmonized Crescendo"] --> TYPE

  %% ===== DYNAMIC MANA (9 cards total) =====
  bloom_tender["Bloom Tender"] --> MANA
  great_forest_druid["Great Forest Druid"] --> MANA
  springleaf_drum["Springleaf Drum"] --> MANA
  %% --- triaged from research ---
  flamebraider["Flamebraider"] --> MANA

  %% ===== PLANESWALKER (2 cards total) =====
  ajani_outland_chaperone["Ajani, Outland Chaperone"] --> PW

  %% ===== GAIN CONTROL (1 card total) =====
  goatnap["Goatnap"] --> CONTROL

  %% ===== DELAYED TRIGGERS (3 cards total) =====
  scarblades_malice["Scarblades Malice"] --> DELAYED

  %% ===== SPELL/PERMANENT COPY (7 cards total) =====
  mirrorform["Mirrorform"] --> COPY
  %% --- triaged from research ---
  kirol_attentive_first_year["Kirol, Attentive First-Year"] --> COPY
  omni_changeling["Omni-Changeling"] --> COPY
  spinerock_tyrant["Spinerock Tyrant"] --> COPY

  %% ===== CAST FROM EXILE (4 cards total) =====
  dawnhand_dissident["Dawnhand Dissident"] --> EXILE_CAST
  %% --- triaged from research ---
  dream_harvest["Dream Harvest"] --> EXILE_CAST

  %% ===== MISCELLANEOUS EFFECTS (6 cards total) =====
  iron_shield_elf["Iron-Shield Elf<br/>(self-tap)"] --> MISC
  mutable_explorer["Mutable Explorer<br/>(land tokens)"] --> MISC
  rooftop_percher["Rooftop Percher<br/>(GY exile)"] --> MISC
  crib_swap["Crib Swap<br/>(token for controller)"] --> MISC
  temporal_cleansing["Temporal Cleansing<br/>(library tuck)"] --> MISC

  %% ===== TRANSFORM/DFC (7 cards total) =====
  ashling_rekindled["Ashling, Rekindled"] --> TRANSFORM
  eirdu_carrier_of_dawn["Eirdu, Carrier of Dawn"] --> TRANSFORM
  sygg_wanderwine_wisdom["Sygg, Wanderwine Wisdom"] --> TRANSFORM
  trystan_callous_cultivator["Trystan, Callous Cultivator"] --> TRANSFORM

  %% ===== MULTI-DEPENDENCY CARDS =====
  %% These cards need 2+ engine capabilities
  champion_of_the_weird["Champion of the Weird"] --> BEHOLD
  champion_of_the_weird --> COST
  champion_of_the_path["Champion of the Path"] --> BEHOLD
  champion_of_the_path --> COND
  rime_chill["Rime Chill"] --> VIVID
  rime_chill --> CHOICE
  slumbering_walker["Slumbering Walker"] --> COST
  slumbering_walker --> COND
  deepway_navigator["Deepway Navigator"] --> MISC
  deepway_navigator --> COND
  soul_immolation["Soul Immolation"] --> CHOICE
  soul_immolation --> COST
  %% --- triaged from research ---
  bloodline_bidding["Bloodline Bidding"] --> TYPE
  bloodline_bidding --> COND
  brigid_clachans_heart["Brigid, Clachan's Heart"] --> TRANSFORM
  brigid_clachans_heart --> MANA
  celestial_reunion["Celestial Reunion"] --> BEHOLD
  celestial_reunion --> TYPE
  champions_of_the_shoal["Champions of the Shoal"] --> BEHOLD
  champions_of_the_shoal --> COST
  end_blaze_epiphany["End-Blaze Epiphany"] --> DELAYED
  end_blaze_epiphany --> IMPULSE
  firdoch_core["Firdoch Core"] --> COND
  firdoch_core --> MANA
  flitterwing_nuisance["Flitterwing Nuisance"] --> COST
  flitterwing_nuisance --> DELAYED
  foraging_wickermaw["Foraging Wickermaw"] --> MANA
  foraging_wickermaw --> COND
  gathering_stone["Gathering Stone"] --> TYPE
  gathering_stone --> CHOICE
  goliath_daydreamer["Goliath Daydreamer"] --> EXILE_CAST
  goliath_daydreamer --> COND
  grub_storied_matriarch["Grub, Storied Matriarch"] --> TRANSFORM
  grub_storied_matriarch --> COST
  high_perfect_morcant["High Perfect Morcant"] --> COST
  high_perfect_morcant --> COND
  kindle_the_inner_flame["Kindle the Inner Flame"] --> COPY
  kindle_the_inner_flame --> BEHOLD
  kithkeeper["Kithkeeper"] --> VIVID
  kithkeeper --> COND
  lavaleaper["Lavaleaper"] --> COND
  lavaleaper --> MANA
  lys_alana_dignitary["Lys Alana Dignitary"] --> BEHOLD
  lys_alana_dignitary --> MANA
  maralen_fae_ascendant["Maralen, Fae Ascendant"] --> EXILE_CAST
  maralen_fae_ascendant --> COND
  moonshadow["Moonshadow"] --> COST
  moonshadow --> COND
  mudbutton_cursetosser["Mudbutton Cursetosser"] --> BEHOLD
  mudbutton_cursetosser --> COND
  oko_lorwyn_liege["Oko, Lorwyn Liege"] --> TRANSFORM
  oko_lorwyn_liege --> PW
  pucas_eye["Puca's Eye"] --> VIVID
  pucas_eye --> CHOICE
  rimefire_torque["Rimefire Torque"] --> TYPE
  rimefire_torque --> COPY
  sanar_innovative_first_year["Sanar, Innovative First-Year"] --> VIVID
  sanar_innovative_first_year --> IMPULSE
  selfless_safewright["Selfless Safewright"] --> TYPE
  selfless_safewright --> COND
  shadow_urchin["Shadow Urchin"] --> COST
  shadow_urchin --> IMPULSE
  soulbright_seeker["Soulbright Seeker"] --> BEHOLD
  soulbright_seeker --> COND
  twilight_diviner["Twilight Diviner"] --> COND
  twilight_diviner --> COPY
  unbury["Unbury"] --> MODAL
  unbury --> TYPE
  vibrance["Vibrance"] --> EVOKE
  vibrance --> COND
  winnowing["Winnowing"] --> TYPE
  winnowing --> COND
  wistfulness["Wistfulness"] --> EVOKE
  wistfulness --> COND

  %% ===== STYLING =====
  classDef root fill:#2d5016,stroke:#1a3a0a,color:#fff,font-weight:bold
  classDef engine fill:#1a4a7a,stroke:#0d3560,color:#fff
  classDef complete fill:#2a7a2a,stroke:#1a5a1a,color:#fff
  classDef incomplete fill:#7a4a1a,stroke:#5a3010,color:#fff

  class ROOT root
  class COST,VIVID,MODAL,EQUIP,EVOKE,ECLIPSED,IMPULSE,BEHOLD,SHOCK,COND,AURA,CHOICE,TYPE,MANA,PW,CONTROL,DELAYED,COPY,EXILE_CAST,MISC,TRANSFORM engine
```

## Summary Table

Card counts include multi-dependency cards (37 cards appear in 2+ categories each).

| Engine Capability | Cards | Effort |
|---|---|---|
| **Complete** (direct to root) | 86 | Done |
| Conditional/Dynamic Effects | 70 | Hard: various state-dependent abilities |
| Cost System | 21 | Medium: implement `pay_costs()` match arms |
| Vivid Mechanic | 14 | Medium: color-counting helper + effect variants |
| Creature Type Choice | 12 | Medium: choose-type + conditional application |
| Player Choice | 11 | Medium: runtime choice framework |
| Behold Mechanic | 10 | Hard: reveal/exile/return framework |
| Aura System | 9 | Hard: attachment, continuous effects |
| Dynamic Mana | 9 | Medium: color-dependent mana |
| Modal Spells | 7 | Hard: mode selection + conditional resolution |
| Spell/Permanent Copy | 7 | Hard: stack/clone manipulation |
| Transform/DFC | 7 | Hard: double-faced card system |
| Impulse Draw | 6 | Hard: exile zone play permissions |
| Misc Effects | 6 | Easy-Medium: individual small effects |
| Evoke | 5 | Medium: alt cost + ETB + sacrifice |
| Eclipsed Cycle | 5 | Medium: look-at-top-N, reveal-to-hand |
| Shock Lands | 5 | Medium: ETB replacement effect |
| Equipment | 4 | Hard: attach/detach/equip system |
| Cast from Exile | 4 | Hard: exile zone play permissions |
| Delayed Triggers | 3 | Medium: one-shot trigger registration |
| Planeswalker | 2 | Hard: loyalty system |
| Gain Control | 1 | Medium: control-change effect |

**Total: 267 cards (86 complete + 181 remaining)**

37 multi-dependency cards need 2+ engine capabilities each.

## Critical Path

The most impactful engine capabilities to implement (by unique cards unblocked):

1. **Conditional/Dynamic Effects** (70 cards) — Most diverse, unlocks the most cards
2. **Cost System** (18 cards) — Blight costs, remove-counter costs, high ROI
3. **Vivid Mechanic** (14 cards) — Single engine addition unlocks cycles
4. **Creature Type Choice** (12 cards) — Convoke+type-matters cards
5. **Aura System** (9 cards) — Broadly useful beyond ECL
6. **Behold Mechanic** (10 cards) — ECL-specific mechanic
7. **Player Choice** (10 cards) — Needed for many partial cards
8. **Dynamic Mana** (9 cards) — Conditional mana production
9. **Transform/DFC** (7 cards) — New system for double-faced cards
10. **Spell/Permanent Copy** (7 cards) — Clone effects, spell copying
11. **Impulse Draw** (6 cards) — Exile zone play permissions
12. **Misc Effects** (6 cards) — Quick individual fixes
13. **Modal Spells + Evoke + Shock Lands + Eclipsed** (5-7 each) — Reusable cycles
14. **Cast from Exile** (4 cards) — Exile zone cast permissions
15. **Equipment + Planeswalker + Delayed Triggers + Gain Control** (2-4 each)
