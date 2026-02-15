// Tarkir: Dragonstorm (TDM) set — released 2025-04-11.
// 276 unique cards. Tier 1-3 creatures, spells, lands, and enchantments.

use crate::cards::basic_lands;
use crate::registry::CardRegistry;
use mtg_engine::abilities::{Ability, Cost, Effect, StaticEffect, TargetSpec};
use mtg_engine::events::EventType;
use mtg_engine::card::CardData;
use mtg_engine::constants::*;
use mtg_engine::mana::{Mana, ManaCost};
use mtg_engine::types::{ObjectId, PlayerId};

pub fn register(registry: &mut CardRegistry) {
    basic_lands::register(registry, "TDM");

    // ── Creatures ────────────────────────────────────────────────────────────
    registry.register("Adorned Crocodile", adorned_crocodile, "TDM");
    registry.register("Aegis Sculptor", aegis_sculptor, "TDM");
    registry.register("Agent of Kotis", agent_of_kotis, "TDM");
    registry.register("Ainok Wayfarer", ainok_wayfarer, "TDM");
    registry.register("Alchemist's Assistant", alchemists_assistant, "TDM");
    registry.register("Armament Dragon", armament_dragon, "TDM");
    registry.register("Attuned Hunter", attuned_hunter, "TDM");
    registry.register("Avenger of the Fallen", avenger_of_the_fallen, "TDM");
    registry.register("Boulderborn Dragon", boulderborn_dragon, "TDM");
    registry.register("Cori Mountain Stalwart", cori_mountain_stalwart, "TDM");
    registry.register("Craterhoof Behemoth", craterhoof_behemoth, "TDM");
    registry.register("Dalkovan Packbeasts", dalkovan_packbeasts, "TDM");
    registry.register("Descendant of Storms", descendant_of_storms, "TDM");
    registry.register("Dragon Sniper", dragon_sniper, "TDM");
    registry.register("Dragonback Lancer", dragonback_lancer, "TDM");
    registry.register("Equilibrium Adept", equilibrium_adept, "TDM");
    registry.register("Evolving Wilds", super::fdn::evolving_wilds, "TDM");
    registry.register("Flamehold Grappler", flamehold_grappler, "TDM");
    registry.register("Fleeting Effigy", fleeting_effigy, "TDM");
    registry.register("Fortress Kin-Guard", fortress_kin_guard, "TDM");
    registry.register("Furious Forebear", furious_forebear, "TDM");
    registry.register("Gurmag Nightwatch", gurmag_nightwatch, "TDM");
    registry.register("Gurmag Rakshasa", gurmag_rakshasa, "TDM");
    registry.register("Highspire Bell-Ringer", highspire_bell_ringer, "TDM");
    registry.register("Humbling Elder", humbling_elder, "TDM");
    registry.register("Iceridge Serpent", iceridge_serpent, "TDM");
    registry.register("Inspirited Vanguard", inspirited_vanguard, "TDM");
    registry.register("Iridescent Tiger", iridescent_tiger, "TDM");
    registry.register("Jade-Cast Sentinel", jade_cast_sentinel, "TDM");
    registry.register("Jeskai Brushmaster", jeskai_brushmaster, "TDM");
    registry.register("Jeskai Shrinekeeper", jeskai_shrinekeeper, "TDM");
    registry.register("Kin-Tree Nurturer", kin_tree_nurturer, "TDM");
    registry.register("Kishla Skimmer", kishla_skimmer, "TDM");
    registry.register("Krotiq Nestguard", krotiq_nestguard, "TDM");
    registry.register("Loxodon Battle Priest", loxodon_battle_priest, "TDM");
    registry.register("Marshal of the Lost", marshal_of_the_lost, "TDM");
    registry.register("Nightblade Brigade", nightblade_brigade, "TDM");
    registry.register("Poised Practitioner", poised_practitioner, "TDM");
    registry.register("Qarsi Revenant", qarsi_revenant, "TDM");
    registry.register("Reigning Victor", reigning_victor, "TDM");
    registry.register("Reputable Merchant", reputable_merchant, "TDM");
    registry.register("Rescue Leopard", rescue_leopard, "TDM");
    registry.register("Sage of the Skies", sage_of_the_skies, "TDM");
    registry.register("Sagu Pummeler", sagu_pummeler, "TDM");
    registry.register("Sagu Wildling", sagu_wildling, "TDM");
    registry.register("Sandskitter Outrider", sandskitter_outrider, "TDM");
    registry.register("Shock Brigade", shock_brigade, "TDM");
    registry.register("Shocking Sharpshooter", shocking_sharpshooter, "TDM");
    registry.register("Sibsig Appraiser", sibsig_appraiser, "TDM");
    registry.register("Sinkhole Surveyor", sinkhole_surveyor, "TDM");
    registry.register("Skirmish Rhino", skirmish_rhino, "TDM");
    registry.register("Summit Intimidator", summit_intimidator, "TDM");
    registry.register("Temur Tawnyback", temur_tawnyback, "TDM");
    registry.register("Undergrowth Leopard", undergrowth_leopard, "TDM");
    registry.register("Unrooted Ancestor", unrooted_ancestor, "TDM");
    registry.register("Unsparing Boltcaster", unsparing_boltcaster, "TDM");
    registry.register("Veteran Ice Climber", veteran_ice_climber, "TDM");
    registry.register("Voice of Victory", voice_of_victory, "TDM");
    registry.register("Watcher of the Wayside", watcher_of_the_wayside, "TDM");
    registry.register("Wingblade Disciple", wingblade_disciple, "TDM");

    // ── Tier 3 — lands ──────────────────────────────────────────────────────
    // Gain lands (ETB tapped, gain 1 life, tap for 2 colors)
    registry.register("Bloodfell Caves", bloodfell_caves, "TDM");
    registry.register("Blossoming Sands", blossoming_sands, "TDM");
    registry.register("Dismal Backwater", dismal_backwater, "TDM");
    registry.register("Jungle Hollow", jungle_hollow, "TDM");
    registry.register("Rugged Highlands", rugged_highlands, "TDM");
    registry.register("Scoured Barrens", scoured_barrens, "TDM");
    registry.register("Swiftwater Cliffs", swiftwater_cliffs, "TDM");
    registry.register("Thornwood Falls", thornwood_falls, "TDM");
    registry.register("Tranquil Cove", tranquil_cove, "TDM");
    registry.register("Wind-Scarred Crag", wind_scarred_crag, "TDM");
    // Tri-lands (ETB tapped, tap for 3 colors)
    registry.register("Frontier Bivouac", frontier_bivouac, "TDM");
    registry.register("Mystic Monastery", mystic_monastery, "TDM");
    registry.register("Nomad Outpost", nomad_outpost, "TDM");
    registry.register("Opulent Palace", opulent_palace, "TDM");
    registry.register("Sandsteppe Citadel", sandsteppe_citadel, "TDM");

    // ── Tier 3 — instants/sorceries ──────────────────────────────────────────
    registry.register("Aggressive Negotiations", aggressive_negotiations, "TDM");
    registry.register("Alesha's Legacy", aleshas_legacy, "TDM");
    registry.register("Auroral Procession", auroral_procession, "TDM");
    registry.register("Bewildering Blizzard", bewildering_blizzard, "TDM");
    registry.register("Coordinated Maneuver", coordinated_maneuver, "TDM");
    registry.register("Defibrillating Current", defibrillating_current, "TDM");
    registry.register("Desperate Measures", desperate_measures, "TDM");
    registry.register("Duty Beyond Death", duty_beyond_death, "TDM");
    registry.register("Frontline Rush", frontline_rush, "TDM");
    registry.register("Inevitable Defeat", inevitable_defeat, "TDM");
    registry.register("Jeskai Revelation", jeskai_revelation, "TDM");
    registry.register("Kin-Tree Severance", kin_tree_severance, "TDM");
    registry.register("Lightfoot Technique", lightfoot_technique, "TDM");
    registry.register("Mammoth Bellow", mammoth_bellow, "TDM");
    registry.register("Overwhelming Surge", overwhelming_surge, "TDM");
    registry.register("Perennation", perennation, "TDM");
    registry.register("Rakshasa's Bargain", rakshasas_bargain, "TDM");
    registry.register("Riverwalk Technique", riverwalk_technique, "TDM");
    registry.register("Roamer's Routine", roamers_routine, "TDM");
    registry.register("Sarkhan's Resolve", sarkhans_resolve, "TDM");
    registry.register("Seize Opportunity", seize_opportunity, "TDM");
    registry.register("Unending Whisper", unending_whisper, "TDM");
    registry.register("Ureni's Rebuff", urenis_rebuff, "TDM");
    registry.register("Wild Ride", wild_ride, "TDM");
    registry.register("Worthy Cost", worthy_cost, "TDM");

    // ── Tier 3 — creatures with abilities ────────────────────────────────────
    registry.register("Abzan Devotee", abzan_devotee, "TDM");
    registry.register("Bearer of Glory", bearer_of_glory, "TDM");
    registry.register("Bone-Cairn Butcher", bone_cairn_butcher, "TDM");
    registry.register("Devoted Duelist", devoted_duelist, "TDM");
    registry.register("Dusyut Earthcarver", dusyut_earthcarver, "TDM");
    registry.register("Mardu Devotee", mardu_devotee, "TDM");
    registry.register("Meticulous Artisan", meticulous_artisan, "TDM");
    registry.register("Rainveil Rejuvenator", rainveil_rejuvenator, "TDM");
    registry.register("Salt Road Packbeast", salt_road_packbeast, "TDM");
    registry.register("Sultai Devotee", sultai_devotee, "TDM");
    registry.register("Temur Devotee", temur_devotee, "TDM");
    registry.register("Unburied Earthcarver", unburied_earthcarver, "TDM");
    registry.register("Zurgo's Vanguard", zurgos_vanguard, "TDM");

    // ── Tier 3 — enchantments ────────────────────────────────────────────────
    registry.register("Dracogenesis", dracogenesis, "TDM");
    registry.register("Dragonstorm Globe", dragonstorm_globe, "TDM");
    registry.register("Encroaching Dragonstorm", encroaching_dragonstorm, "TDM");
    registry.register("Roiling Dragonstorm", roiling_dragonstorm, "TDM");
    registry.register("Stormplain Detainment", stormplain_detainment, "TDM");
    registry.register("Teeming Dragonstorm", teeming_dragonstorm, "TDM");

    // ── Tier 2 — spells ─────────────────────────────────────────────────────
    registry.register("Caustic Exhale", caustic_exhale, "TDM");
    registry.register("Channeled Dragonfire", channeled_dragonfire, "TDM");
    registry.register("Cruel Truths", cruel_truths, "TDM");
    registry.register("Dispelling Exhale", dispelling_exhale, "TDM");
    registry.register("Dragonclaw Strike", dragonclaw_strike, "TDM");
    registry.register("Dragon's Prey", dragons_prey, "TDM");
    registry.register("Knockout Maneuver", knockout_maneuver, "TDM");
    registry.register("Molten Exhale", molten_exhale, "TDM");
    registry.register("Narset's Rebuke", narsets_rebuke, "TDM");
    registry.register("Piercing Exhale", piercing_exhale, "TDM");
    registry.register("Rebellious Strike", rebellious_strike, "TDM");
    registry.register("Salt Road Skirmish", salt_road_skirmish, "TDM");
    registry.register("Snakeskin Veil", snakeskin_veil, "TDM");
    registry.register("Spectral Denial", spectral_denial, "TDM");
    registry.register("Twin Bolt", twin_bolt, "TDM");

    // ── Tier 2 — non-creature permanents ────────────────────────────────────
    registry.register("Dragonback Assault", dragonback_assault, "TDM");

    // ── Tier 3 — batch 2 ────────────────────────────────────────────────────
    registry.register("Abzan Monument", abzan_monument, "TDM");
    registry.register("All-Out Assault", all_out_assault, "TDM");
    registry.register("Ambling Stormshell", ambling_stormshell, "TDM");
    registry.register("Anafenza, Unyielding Lineage", anafenza_unyielding_lineage, "TDM");
    registry.register("Barrensteppe Siege", barrensteppe_siege, "TDM");
    registry.register("Betor, Kin to All", betor_kin_to_all, "TDM");
    registry.register("Breaching Dragonstorm", breaching_dragonstorm, "TDM");
    registry.register("Call the Spirit Dragons", call_the_spirit_dragons, "TDM");
    registry.register("Cori Mountain Monastery", cori_mountain_monastery, "TDM");
    registry.register("Cori Steel-Cutter", cori_steel_cutter, "TDM");
    registry.register("Dalkovan Encampment", dalkovan_encampment, "TDM");
    registry.register("Death Begets Life", death_begets_life, "TDM");
    registry.register("Dragonfire Blade", dragonfire_blade, "TDM");
    registry.register("Dragonologist", dragonologist, "TDM");
    registry.register("Effortless Master", effortless_master, "TDM");
    registry.register("Embermouth Sentinel", embermouth_sentinel, "TDM");
    registry.register("Eshki, Dragonclaw", eshki_dragonclaw, "TDM");
    registry.register("Fangkeeper's Familiar", fangkeepers_familiar, "TDM");
    registry.register("Feral Deathgorger", feral_deathgorger, "TDM");
    registry.register("Fire-Rim Form", fire_rim_form, "TDM");
    registry.register("Formation Breaker", formation_breaker, "TDM");
    registry.register("Fresh Start", fresh_start, "TDM");
    registry.register("Frostcliff Siege", frostcliff_siege, "TDM");
    registry.register("Glacierwood Siege", glacierwood_siege, "TDM");
    registry.register("Great Arashin City", great_arashin_city, "TDM");
    registry.register("Heritage Reclamation", heritage_reclamation, "TDM");
    registry.register("Hollowmurk Siege", hollowmurk_siege, "TDM");
    registry.register("Host of the Hereafter", host_of_the_hereafter, "TDM");
    registry.register("Hundred-Battle Veteran", hundred_battle_veteran, "TDM");
    registry.register("Karakyk Guardian", karakyk_guardian, "TDM");

    // ── Tier 2+ — new batch ────────────────────────────────────────────────
    registry.register("Arashin Sunshield", arashin_sunshield, "TDM");
    registry.register("Champion of Dusan", champion_of_dusan, "TDM");
    registry.register("Constrictor Sage", constrictor_sage, "TDM");
    registry.register("Corroding Dragonstorm", corroding_dragonstorm, "TDM");
    registry.register("Delta Bloodflies", delta_bloodflies, "TDM");
    registry.register("Dragonstorm Forecaster", dragonstorm_forecaster, "TDM");
    registry.register("Focus the Mind", focus_the_mind, "TDM");
    registry.register("Hardened Tactician", hardened_tactician, "TDM");
    registry.register("Jeskai Devotee", jeskai_devotee, "TDM");
    registry.register("Monastery Messenger", monastery_messenger, "TDM");
    registry.register("Osseous Exhale", osseous_exhale, "TDM");
    registry.register("Rite of Renewal", rite_of_renewal, "TDM");
    registry.register("Snowmelt Stag", snowmelt_stag, "TDM");
    registry.register("Stormbeacon Blade", stormbeacon_blade, "TDM");
    registry.register("Sunset Strikemaster", sunset_strikemaster, "TDM");
    registry.register("Tempest Hawk", tempest_hawk, "TDM");
    registry.register("Trade Route Envoy", trade_route_envoy, "TDM");
    registry.register("Traveling Botanist", traveling_botanist, "TDM");
    registry.register("Twinmaw Stormbrood", twinmaw_stormbrood, "TDM");
    registry.register("Venerated Stormsinger", venerated_stormsinger, "TDM");
    registry.register("Wayspeaker Bodyguard", wayspeaker_bodyguard, "TDM");
    registry.register("Wingspan Stride", wingspan_stride, "TDM");
    registry.register("Yathan Tombguard", yathan_tombguard, "TDM");
    registry.register("Cori-Steel Cutter", cori_steel_cutter, "TDM");
    registry.register("Eshki Dragonclaw", eshki_dragonclaw, "TDM");
    registry.register("Kheru Goldkeeper", kheru_goldkeeper, "TDM");
    registry.register("Kishla Village", kishla_village, "TDM");
    registry.register("Kotis, the Fangkeeper", kotis_the_fangkeeper, "TDM");
    registry.register("Lie in Wait", lie_in_wait, "TDM");
    registry.register("Lotuslight Dancers", lotuslight_dancers, "TDM");
    registry.register("Maelstrom of the Spirit Dragon", maelstrom_of_the_spirit_dragon, "TDM");
    registry.register("Magmatic Hellkite", magmatic_hellkite, "TDM");
    registry.register("Mardu Monument", mardu_monument, "TDM");
    registry.register("Mardu Siegebreaker", mardu_siegebreaker, "TDM");
    registry.register("Mistrise Village", mistrise_village, "TDM");
    registry.register("Naga Fleshcrafter", naga_fleshcrafter, "TDM");
    registry.register("Narset, Jeskai Waymaster", narset_jeskai_waymaster, "TDM");
    registry.register("Neriv, Heart of the Storm", neriv_heart_of_the_storm, "TDM");
    registry.register("Purging Stormbrood", purging_stormbrood, "TDM");
    registry.register("Rally the Monastery", rally_the_monastery, "TDM");
    registry.register("Riling Dawnbreaker", riling_dawnbreaker, "TDM");
    registry.register("Ringing Strike Mastery", ringing_strike_mastery, "TDM");
    registry.register("Riverwheel Sweep", riverwheel_sweep, "TDM");
    registry.register("Rot-Curse Rakshasa", rot_curse_rakshasa, "TDM");
    registry.register("Severance Priest", severance_priest, "TDM");
    registry.register("Shiko, Paragon of the Way", shiko_paragon_of_the_way, "TDM");
    registry.register("Sidisi, Regent of the Mire", sidisi_regent_of_the_mire, "TDM");
    registry.register("Smile at Death", smile_at_death, "TDM");
    registry.register("Sonic Shrieker", sonic_shrieker, "TDM");
    registry.register("Stalwart Successor", stalwart_successor, "TDM");
    registry.register("Starry-Eyed Skyrider", starry_eyed_skyrider, "TDM");
    registry.register("Static Snare", static_snare, "TDM");
    registry.register("Stormscale Scion", stormscale_scion, "TDM");
    registry.register("Stormshriek Feral", stormshriek_feral, "TDM");
    registry.register("Sunpearl Kirin", sunpearl_kirin, "TDM");
    registry.register("Surrak, Elusive Hunter", surrak_elusive_hunter, "TDM");
    registry.register("Synchronized Charge", synchronized_charge, "TDM");
    registry.register("Temur Battlecrier", temur_battlecrier, "TDM");
    registry.register("Tersa Lightshatter", tersa_lightshatter, "TDM");
    registry.register("Teval, Arbiter of Virtue", teval_arbiter_of_virtue, "TDM");
    registry.register("Ureni, the Song Unending", ureni_the_song_unending, "TDM");
    registry.register("Wail of War", wail_of_war, "TDM");
    registry.register("War Effort", war_effort, "TDM");
    registry.register("Warden of the Grove", warden_of_the_grove, "TDM");
    registry.register("Whirlwing Stormbrood", whirlwing_stormbrood, "TDM");
    registry.register("Windcrag Siege", windcrag_siege, "TDM");
    registry.register("Zurgo, Thunder's Decree", zurgo_thunders_decree, "TDM");

    // ── New Creatures ────────────────────────────────────────────────────
    registry.register("Bloomvine Regent", bloomvine_regent, "TDM");
    registry.register("Clarion Conqueror", clarion_conqueror, "TDM");
    registry.register("Dirgur Island Dragon", dirgur_island_dragon, "TDM");
    registry.register("Disruptive Stormbrood", disruptive_stormbrood, "TDM");
    registry.register("Felothar, Dawn of the Abzan", felothar_dawn_of_the_abzan, "TDM");
    registry.register("Kishla Trawlers", kishla_trawlers, "TDM");
    registry.register("Krumar Initiate", krumar_initiate, "TDM");
    registry.register("Lasyd Prowler", lasyd_prowler, "TDM");
    registry.register("Marang River Regent", marang_river_regent, "TDM");
    registry.register("Runescale Stormbrood", runescale_stormbrood, "TDM");
    registry.register("Sage of the Fang", sage_of_the_fang, "TDM");
    registry.register("Sarkhan, Dragon Ascendant", sarkhan_dragon_ascendant, "TDM");
    registry.register("Scavenger Regent", scavenger_regent, "TDM");
    registry.register("Songcrafter Mage", songcrafter_mage, "TDM");
    registry.register("Stadium Headliner", stadium_headliner, "TDM");
    registry.register("Taigam, Master Opportunist", taigam_master_opportunist, "TDM");
    registry.register("Underfoot Underdogs", underfoot_underdogs, "TDM");
    registry.register("Yathan Roadwatcher", yathan_roadwatcher, "TDM");

    // ── New Instants and Sorceries ────────────────────────────────────────
    registry.register("Glacial Dragonhunt", glacial_dragonhunt, "TDM");
    registry.register("Nature's Rhythm", natures_rhythm, "TDM");
    registry.register("New Way Forward", new_way_forward, "TDM");
    registry.register("Strategic Betrayal", strategic_betrayal, "TDM");
    registry.register("United Battlefront", united_battlefront, "TDM");
    registry.register("Winternight Stories", winternight_stories, "TDM");

    // ── New Artifacts ─────────────────────────────────────────────────────
    registry.register("Dragonbroods' Relic", dragonbroods_relic, "TDM");
    registry.register("Essence Anchor", essence_anchor, "TDM");
    registry.register("Herd Heirloom", herd_heirloom, "TDM");
    registry.register("Jeskai Monument", jeskai_monument, "TDM");
    registry.register("Mox Jasper", mox_jasper, "TDM");
    registry.register("Sultai Monument", sultai_monument, "TDM");
    registry.register("Temur Monument", temur_monument, "TDM");

    // ── New Enchantments ──────────────────────────────────────────────────
    registry.register("Awaken the Honored Dead", awaken_the_honored_dead, "TDM");
    registry.register("Rediscover the Way", rediscover_the_way, "TDM");
    registry.register("Reverberating Summons", reverberating_summons, "TDM");
    registry.register("Revival of the Ancestors", revival_of_the_ancestors, "TDM");
    registry.register("Roar of Endless Song", roar_of_endless_song, "TDM");
    registry.register("Stillness in Motion", stillness_in_motion, "TDM");
    registry.register("The Sibsig Ceremony", the_sibsig_ceremony, "TDM");
    registry.register("Thunder of Unity", thunder_of_unity, "TDM");

    // ── Other ─────────────────────────────────────────────────────────────
    registry.register("Elspeth, Storm Slayer", elspeth_storm_slayer, "TDM");
    registry.register("Ugin, Eye of the Storms", ugin_eye_of_the_storms, "TDM");
}

// ── Creature implementations ─────────────────────────────────────────────────

fn adorned_crocodile(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Adorned Crocodile".into(), mana_cost: ManaCost::parse("{4}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Custom("Crocodile".into())],
        power: Some(5), toughness: Some(3), rarity: Rarity::Common,
        abilities: vec![
            Ability::dies_triggered(id,
                "When this creature dies, create a 2/2 black Zombie Druid creature token.",
                vec![Effect::CreateToken { token_name: "2/2 black Zombie Druid".into(), count: 1 }],
                TargetSpec::None),
            Ability::activated(id,
                "Renew -- {B}, Exile this card from your graveyard: Put a +1/+1 counter on target creature. Activate only as a sorcery.",
                vec![Cost::pay_mana("{B}"), Cost::ExileFromGraveyard(1)],
                vec![Effect::AddCounters { counter_type: "+1/+1".into(), count: 1 }],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn aegis_sculptor(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Aegis Sculptor".into(), mana_cost: ManaCost::parse("{3}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Bird, SubType::Wizard],
        power: Some(2), toughness: Some(3), keywords: KeywordAbilities::FLYING | KeywordAbilities::WARD,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id, "Ward {2}",
                vec![StaticEffect::ward("{2}")]),
            Ability::triggered(id,
                "At the beginning of your upkeep, you may exile two cards from your graveyard. If you do, put a +1/+1 counter on this creature.",
                vec![EventType::UpkeepStep],
                vec![Effect::AddCounters { counter_type: "+1/+1".into(), count: 1 }],
                TargetSpec::None).set_optional(),
        ],
        ..Default::default() }
}

fn agent_of_kotis(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Agent of Kotis".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Rogue],
        power: Some(2), toughness: Some(1), rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "Renew -- {3}{U}, Exile this card from your graveyard: Put two +1/+1 counters on target creature. Activate only as a sorcery.",
                vec![Cost::pay_mana("{3}{U}"), Cost::ExileFromGraveyard(1)],
                vec![Effect::AddCounters { counter_type: "+1/+1".into(), count: 2 }],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn ainok_wayfarer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ainok Wayfarer".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Dog, SubType::Scout],
        power: Some(1), toughness: Some(1), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, mill three cards. You may put a land card from among them into your hand. If you don't, put a +1/+1 counter on this creature.",
                vec![Effect::Mill { count: 3 }, Effect::Custom("You may put a land card from among them into your hand. If you don't, put a +1/+1 counter on this creature.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn alchemists_assistant(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Alchemist's Assistant".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Custom("Monkey".into())],
        power: Some(2), toughness: Some(1), keywords: KeywordAbilities::LIFELINK,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                "Renew -- {1}{B}, Exile this card from your graveyard: Put a lifelink counter on target creature. Activate only as a sorcery.",
                vec![Cost::pay_mana("{1}{B}"), Cost::ExileFromGraveyard(1)],
                vec![Effect::AddCounters { counter_type: "lifelink".into(), count: 1 }],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn armament_dragon(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Armament Dragon".into(), mana_cost: ManaCost::parse("{3}{W}{B}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Dragon],
        power: Some(3), toughness: Some(4), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, distribute three +1/+1 counters among one, two, or three target creatures you control.",
                vec![Effect::Custom("Distribute three +1/+1 counters among one, two, or three target creatures you control.".into())],
                TargetSpec::Custom("one, two, or three target creatures you control".into())),
        ],
        ..Default::default() }
}

fn attuned_hunter(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Attuned Hunter".into(), mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Ranger],
        power: Some(3), toughness: Some(3), keywords: KeywordAbilities::TRAMPLE,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                "Whenever one or more cards leave your graveyard during your turn, put a +1/+1 counter on this creature.",
                vec![EventType::ZoneChange],
                vec![Effect::AddCounters { counter_type: "+1/+1".into(), count: 1 }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn avenger_of_the_fallen(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Avenger of the Fallen".into(), mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Warrior],
        power: Some(2), toughness: Some(4), keywords: KeywordAbilities::DEATHTOUCH,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "Mobilize X, where X is the number of creature cards in your graveyard. (Create X 1/1 white Soldier creature tokens.)",
                vec![Effect::Custom("Create X 1/1 white Soldier creature tokens, where X is the number of creature cards in your graveyard.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn boulderborn_dragon(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Boulderborn Dragon".into(), mana_cost: ManaCost::parse("{5}"),
        card_types: vec![CardType::Artifact, CardType::Creature], subtypes: vec![SubType::Dragon],
        power: Some(3), toughness: Some(3),
        keywords: KeywordAbilities::FLYING | KeywordAbilities::VIGILANCE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::attacks_triggered(id,
                "Whenever this creature attacks, surveil 1.",
                vec![Effect::Scry { count: 1 }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn cori_mountain_stalwart(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Cori Mountain Stalwart".into(), mana_cost: ManaCost::parse("{1}{R}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Monk],
        power: Some(3), toughness: Some(3), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell_cast_triggered(id,
                "Flurry -- Whenever you cast your second spell each turn, this creature deals 2 damage to each opponent and you gain 2 life.",
                vec![Effect::DealDamageOpponents { amount: 2 }, Effect::GainLife { amount: 2 }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn craterhoof_behemoth(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Craterhoof Behemoth".into(), mana_cost: ManaCost::parse("{5}{G}{G}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Beast],
        power: Some(5), toughness: Some(5),
        keywords: KeywordAbilities::HASTE | KeywordAbilities::TRAMPLE,
        rarity: Rarity::Mythic,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Craterhoof Behemoth enters, creatures you control get +X/+X and trample until end of turn, where X is the number of creatures you control.",
                vec![Effect::Custom("Creatures you control get +X/+X and trample until end of turn, X = creatures you control.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn dalkovan_packbeasts(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Dalkovan Packbeasts".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Ox],
        power: Some(0), toughness: Some(4), keywords: KeywordAbilities::VIGILANCE,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "Mobilize 3 (When this creature enters, create three 1/1 white Soldier creature tokens.)",
                vec![Effect::CreateToken { token_name: "1/1 white Soldier".into(), count: 3 }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn descendant_of_storms(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Descendant of Storms".into(), mana_cost: ManaCost::parse("{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Soldier],
        power: Some(2), toughness: Some(1), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::attacks_triggered(id,
                "Whenever this creature attacks, you may pay {1}{W}. If you do, it endures 1.",
                vec![Effect::Custom("Endure 1 (put a +1/+1 counter on it; if it would die, exile it with its counters instead).".into())],
                TargetSpec::None).set_optional(),
        ],
        ..Default::default() }
}

fn dragon_sniper(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Dragon Sniper".into(), mana_cost: ManaCost::parse("{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Archer],
        power: Some(1), toughness: Some(1),
        keywords: KeywordAbilities::VIGILANCE | KeywordAbilities::REACH | KeywordAbilities::DEATHTOUCH,
        rarity: Rarity::Common, ..Default::default() }
}

fn dragonback_lancer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Dragonback Lancer".into(), mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Soldier],
        power: Some(3), toughness: Some(3), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "Mobilize 1 (When this creature enters, create a 1/1 white Soldier creature token.)",
                vec![Effect::CreateToken { token_name: "1/1 white Soldier".into(), count: 1 }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn equilibrium_adept(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Equilibrium Adept".into(), mana_cost: ManaCost::parse("{3}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Dog, SubType::Monk],
        power: Some(2), toughness: Some(4),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, exile the top card of your library. Until the end of your next turn, you may play that card.",
                vec![Effect::Custom("Exile the top card of your library. Until the end of your next turn, you may play that card.".into())],
                TargetSpec::None),
            Ability::spell_cast_triggered(id,
                "Flurry -- Whenever you cast your second spell each turn, this creature gains double strike until end of turn.",
                vec![Effect::GainKeywordUntilEndOfTurn { keyword: "double strike".into() }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn flamehold_grappler(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Flamehold Grappler".into(), mana_cost: ManaCost::parse("{U}{R}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Monk],
        power: Some(3), toughness: Some(3), keywords: KeywordAbilities::FIRST_STRIKE,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, copy the next spell you cast this turn when you cast it. You may choose new targets for the copy.",
                vec![Effect::Custom("Copy the next spell you cast this turn when you cast it. You may choose new targets for the copy.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn fleeting_effigy(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Fleeting Effigy".into(), mana_cost: ManaCost::parse("{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elemental],
        power: Some(2), toughness: Some(2), keywords: KeywordAbilities::HASTE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "At the beginning of your end step, return this creature to its owner's hand.",
                vec![EventType::EndStep],
                vec![Effect::Bounce],
                TargetSpec::None),
            Ability::activated(id,
                "{2}{R}: This creature gets +2/+0 until end of turn.",
                vec![Cost::pay_mana("{2}{R}")],
                vec![Effect::boost_until_eot(2, 0)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn fortress_kin_guard(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Fortress Kin-Guard".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Dog, SubType::Soldier],
        power: Some(1), toughness: Some(2), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, it endures 1.",
                vec![Effect::AddCounters { counter_type: "+1/+1".into(), count: 1 }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn furious_forebear(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Furious Forebear".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Spirit, SubType::Warrior],
        power: Some(3), toughness: Some(1), rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever a creature you control dies while this card is in your graveyard, you may pay {1}{W}. If you do, return this card from your graveyard to your hand.",
                vec![EventType::Dies],
                vec![Effect::ReturnFromGraveyard],
                TargetSpec::None).set_optional(),
        ],
        ..Default::default() }
}

fn gurmag_nightwatch(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Gurmag Nightwatch".into(), mana_cost: ManaCost::parse("{2/B}{2/G}{2/U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Ranger],
        power: Some(3), toughness: Some(3), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, look at the top three cards of your library. You may put one of those cards back on top of your library. Put the rest into your graveyard.",
                vec![Effect::Custom("Look at top 3 cards. Put one on top of library. Put rest into graveyard.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn gurmag_rakshasa(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Gurmag Rakshasa".into(), mana_cost: ManaCost::parse("{4}{B}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Demon],
        power: Some(5), toughness: Some(5), keywords: KeywordAbilities::MENACE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, target creature an opponent controls gets -2/-2 until end of turn and target creature you control gets +2/+2 until end of turn.",
                vec![Effect::boost_until_eot(-2, -2), Effect::boost_until_eot(2, 2)],
                TargetSpec::Custom("creature an opponent controls and creature you control".into())),
        ],
        ..Default::default() }
}

fn highspire_bell_ringer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Highspire Bell-Ringer".into(), mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Djinn, SubType::Monk],
        power: Some(1), toughness: Some(4), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "The second spell you cast each turn costs {1} less to cast.",
                vec![StaticEffect::Custom("The second spell you cast each turn costs {1} less to cast.".into())]),
        ],
        ..Default::default() }
}

fn humbling_elder(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Humbling Elder".into(), mana_cost: ManaCost::parse("{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Monk],
        power: Some(1), toughness: Some(2), keywords: KeywordAbilities::FLASH,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, target creature an opponent controls gets -2/-0 until end of turn.",
                vec![Effect::BoostUntilEndOfTurn { power: -2, toughness: 0 }],
                TargetSpec::PermanentFiltered("creature an opponent controls".into())),
        ],
        ..Default::default() }
}

fn iceridge_serpent(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Iceridge Serpent".into(), mana_cost: ManaCost::parse("{4}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Serpent],
        power: Some(3), toughness: Some(3), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, return target creature an opponent controls to its owner's hand.",
                vec![Effect::Bounce],
                TargetSpec::PermanentFiltered("creature an opponent controls".into())),
        ],
        ..Default::default() }
}

fn inspirited_vanguard(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Inspirited Vanguard".into(), mana_cost: ManaCost::parse("{4}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Soldier],
        power: Some(3), toughness: Some(2), rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever this creature enters or attacks, it endures 2.",
                vec![EventType::EnteredTheBattlefield, EventType::AttackerDeclared],
                vec![Effect::AddCounters { counter_type: "+1/+1".into(), count: 2 }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn iridescent_tiger(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Iridescent Tiger".into(), mana_cost: ManaCost::parse("{4}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Cat],
        power: Some(3), toughness: Some(4), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, if you cast it, add {W}{U}{B}{R}{G}.",
                vec![Effect::AddMana { mana: Mana { white: 1, blue: 1, black: 1, red: 1, green: 1, ..Default::default() } }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn jade_cast_sentinel(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Jade-Cast Sentinel".into(), mana_cost: ManaCost::parse("{4}"),
        card_types: vec![CardType::Artifact, CardType::Creature],
        subtypes: vec![SubType::Ape, SubType::Snake],
        power: Some(1), toughness: Some(5), keywords: KeywordAbilities::REACH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{2}, {T}: Put target card from a graveyard on the bottom of its owner's library.",
                vec![Cost::pay_mana("{2}"), Cost::TapSelf],
                vec![Effect::Custom("Put target card on the bottom of its owner's library.".into())],
                TargetSpec::CardInGraveyard),
        ],
        ..Default::default() }
}

fn jeskai_brushmaster(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Jeskai Brushmaster".into(), mana_cost: ManaCost::parse("{1}{U}{R}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Orc, SubType::Monk],
        power: Some(2), toughness: Some(4), keywords: KeywordAbilities::DOUBLE_STRIKE | KeywordAbilities::PROWESS,
        rarity: Rarity::Uncommon, ..Default::default() }
}

fn jeskai_shrinekeeper(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Jeskai Shrinekeeper".into(), mana_cost: ManaCost::parse("{2}{U}{R}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Dragon],
        power: Some(3), toughness: Some(3),
        keywords: KeywordAbilities::FLYING | KeywordAbilities::HASTE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::combat_damage_to_player_triggered(id,
                "Whenever this creature deals combat damage to a player, you gain 1 life and draw a card.",
                vec![Effect::gain_life(1), Effect::draw_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn kin_tree_nurturer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Kin-Tree Nurturer".into(), mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Druid],
        power: Some(2), toughness: Some(1), keywords: KeywordAbilities::LIFELINK,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, it endures 1.",
                vec![Effect::AddCounters { counter_type: "+1/+1".into(), count: 1 }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn kishla_skimmer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Kishla Skimmer".into(), mana_cost: ManaCost::parse("{G}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Bird, SubType::Scout],
        power: Some(2), toughness: Some(2), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever a card leaves your graveyard during your turn, draw a card. This ability triggers only once each turn.",
                vec![EventType::ZoneChange],
                vec![Effect::draw_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn krotiq_nestguard(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Krotiq Nestguard".into(), mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Insect],
        power: Some(4), toughness: Some(4), keywords: KeywordAbilities::DEFENDER,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{2}{G}: This creature can attack this turn as though it didn't have defender.",
                vec![Cost::pay_mana("{2}{G}")],
                vec![Effect::Custom("This creature can attack this turn as though it didn't have defender.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn loxodon_battle_priest(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Loxodon Battle Priest".into(), mana_cost: ManaCost::parse("{4}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elephant, SubType::Cleric],
        power: Some(3), toughness: Some(5), rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "At the beginning of combat on your turn, put a +1/+1 counter on another target creature you control.",
                vec![EventType::BeginCombat],
                vec![Effect::AddCounters { counter_type: "+1/+1".into(), count: 1 }],
                TargetSpec::PermanentFiltered("another creature you control".into())),
        ],
        ..Default::default() }
}

fn marshal_of_the_lost(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Marshal of the Lost".into(), mana_cost: ManaCost::parse("{2}{W}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Orc, SubType::Warrior],
        power: Some(3), toughness: Some(3), keywords: KeywordAbilities::DEATHTOUCH,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                "Whenever you attack, target creature gets +X/+X until end of turn, where X is the number of attacking creatures.",
                vec![EventType::DeclareAttackers],
                vec![Effect::Custom("Target creature gets +X/+X until end of turn, where X is the number of attacking creatures.".into())],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn nightblade_brigade(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Nightblade Brigade".into(), mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Soldier],
        power: Some(1), toughness: Some(3), keywords: KeywordAbilities::DEATHTOUCH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "Mobilize 1 (When this creature enters, create a 1/1 white Soldier creature token.)",
                vec![Effect::CreateToken { token_name: "1/1 white Soldier".into(), count: 1 }],
                TargetSpec::None),
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, surveil 1.",
                vec![Effect::Scry { count: 1 }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn poised_practitioner(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Poised Practitioner".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Monk],
        power: Some(2), toughness: Some(3), rarity: Rarity::Common,
        abilities: vec![
            Ability::spell_cast_triggered(id,
                "Flurry -- Whenever you cast your second spell each turn, put a +1/+1 counter on this creature. Scry 1.",
                vec![Effect::AddCounters { counter_type: "+1/+1".into(), count: 1 }, Effect::Scry { count: 1 }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn qarsi_revenant(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Qarsi Revenant".into(), mana_cost: ManaCost::parse("{1}{B}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Vampire],
        power: Some(3), toughness: Some(3),
        keywords: KeywordAbilities::FLYING | KeywordAbilities::DEATHTOUCH | KeywordAbilities::LIFELINK,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                "Renew -- {2}{B}, Exile this card from your graveyard: Put a flying counter, a deathtouch counter, and a lifelink counter on target creature. Activate only as a sorcery.",
                vec![Cost::pay_mana("{2}{B}"), Cost::ExileFromGraveyard(1)],
                vec![Effect::AddCounters { counter_type: "flying".into(), count: 1 },
                     Effect::AddCounters { counter_type: "deathtouch".into(), count: 1 },
                     Effect::AddCounters { counter_type: "lifelink".into(), count: 1 }],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn reigning_victor(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Reigning Victor".into(), mana_cost: ManaCost::parse("{2/R}{2/W}{2/B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Orc, SubType::Warrior],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "Mobilize 1 (When this creature enters, create a 1/1 white Soldier creature token.)",
                vec![Effect::CreateToken { token_name: "1/1 white Soldier".into(), count: 1 }],
                TargetSpec::None),
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, target creature gets +1/+0 and gains indestructible until end of turn.",
                vec![Effect::boost_until_eot(1, 0), Effect::GainKeywordUntilEndOfTurn { keyword: "indestructible".into() }],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn reputable_merchant(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Reputable Merchant".into(), mana_cost: ManaCost::parse("{2/W}{2/B}{2/G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Custom("Citizen".into())],
        power: Some(2), toughness: Some(2), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters or dies, put a +1/+1 counter on target creature you control.",
                vec![Effect::add_p1p1_counters(1)],
                TargetSpec::PermanentFiltered("creature you control".into())),
            Ability::dies_triggered(id,
                "When this creature enters or dies, put a +1/+1 counter on target creature you control.",
                vec![Effect::add_p1p1_counters(1)],
                TargetSpec::PermanentFiltered("creature you control".into())),
        ],
        ..Default::default() }
}

fn rescue_leopard(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Rescue Leopard".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Cat],
        power: Some(4), toughness: Some(2), rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever this creature becomes tapped, you may discard a card. If you do, draw a card.",
                vec![EventType::Tapped],
                vec![Effect::Custom("You may discard a card. If you do, draw a card.".into())],
                TargetSpec::None).set_optional(),
        ],
        ..Default::default() }
}

fn sage_of_the_skies(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sage of the Skies".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Monk],
        power: Some(2), toughness: Some(3),
        keywords: KeywordAbilities::FLYING | KeywordAbilities::LIFELINK,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When you cast this spell, if you've cast another spell this turn, copy this spell.",
                vec![EventType::SpellCast],
                vec![Effect::Custom("Copy this spell. (The copy becomes a token.)".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn sagu_pummeler(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sagu Pummeler".into(), mana_cost: ManaCost::parse("{3}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Beast],
        power: Some(4), toughness: Some(4), keywords: KeywordAbilities::REACH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "Renew -- {4}{G}, Exile this card from your graveyard: Put two +1/+1 counters and a reach counter on target creature. Activate only as a sorcery.",
                vec![Cost::pay_mana("{4}{G}"), Cost::ExileFromGraveyard(1)],
                vec![Effect::AddCounters { counter_type: "+1/+1".into(), count: 2 },
                     Effect::AddCounters { counter_type: "reach".into(), count: 1 }],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn sagu_wildling(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sagu Wildling".into(), mana_cost: ManaCost::parse("{4}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Dragon],
        power: Some(3), toughness: Some(3), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, you gain 3 life.",
                vec![Effect::gain_life(3)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn sandskitter_outrider(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sandskitter Outrider".into(), mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Soldier],
        power: Some(2), toughness: Some(1), keywords: KeywordAbilities::MENACE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, it endures 2.",
                vec![Effect::AddCounters { counter_type: "+1/+1".into(), count: 2 }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn shock_brigade(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Shock Brigade".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Soldier],
        power: Some(1), toughness: Some(3), keywords: KeywordAbilities::MENACE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "Mobilize 1 (When this creature enters, create a 1/1 white Soldier creature token.)",
                vec![Effect::CreateToken { token_name: "1/1 white Soldier".into(), count: 1 }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn shocking_sharpshooter(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Shocking Sharpshooter".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Archer],
        power: Some(1), toughness: Some(3), keywords: KeywordAbilities::REACH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever another creature you control enters, this creature deals 1 damage to target opponent.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::DealDamage { amount: 1 }],
                TargetSpec::Player),
        ],
        ..Default::default() }
}

fn sibsig_appraiser(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sibsig Appraiser".into(), mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Zombie, SubType::Advisor],
        power: Some(2), toughness: Some(1), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, look at the top two cards of your library. Put one of them into your hand and the other into your graveyard.",
                vec![Effect::Custom("Look at the top two cards of your library. Put one into your hand and the other into your graveyard.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn sinkhole_surveyor(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sinkhole Surveyor".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Bird, SubType::Scout],
        power: Some(1), toughness: Some(3), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::attacks_triggered(id,
                "Whenever this creature attacks, you lose 1 life and this creature endures 1.",
                vec![Effect::lose_life(1), Effect::AddCounters { counter_type: "+1/+1".into(), count: 1 }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn skirmish_rhino(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Skirmish Rhino".into(), mana_cost: ManaCost::parse("{W}{B}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Rhino],
        power: Some(3), toughness: Some(4), keywords: KeywordAbilities::TRAMPLE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, each opponent loses 2 life and you gain 2 life.",
                vec![Effect::lose_life_opponents(2), Effect::gain_life(2)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn summit_intimidator(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Summit Intimidator".into(), mana_cost: ManaCost::parse("{3}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Custom("Yeti".into())],
        power: Some(4), toughness: Some(3), keywords: KeywordAbilities::REACH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, target creature can't block this turn.",
                vec![Effect::CantBlock],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn temur_tawnyback(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Temur Tawnyback".into(), mana_cost: ManaCost::parse("{2/G}{2/U}{2/R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Beast],
        power: Some(4), toughness: Some(3), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, draw a card, then discard a card.",
                vec![Effect::draw_cards(1), Effect::discard_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn undergrowth_leopard(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Undergrowth Leopard".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Cat],
        power: Some(2), toughness: Some(2), keywords: KeywordAbilities::VIGILANCE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{1}, Sacrifice this creature: Destroy target artifact or enchantment.",
                vec![Cost::pay_mana("{1}"), Cost::SacrificeSelf],
                vec![Effect::Destroy],
                TargetSpec::PermanentFiltered("artifact or enchantment".into())),
        ],
        ..Default::default() }
}

fn unrooted_ancestor(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Unrooted Ancestor".into(), mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Spirit, SubType::Cleric],
        power: Some(3), toughness: Some(2),
        keywords: KeywordAbilities::FLASH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{1}, Sacrifice another creature: This creature gains indestructible until end of turn. Tap it.",
                vec![Cost::pay_mana("{1}"), Cost::SacrificeOther("another creature".into())],
                vec![Effect::GainKeywordUntilEndOfTurn { keyword: "indestructible".into() }, Effect::TapTarget],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn unsparing_boltcaster(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Unsparing Boltcaster".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Ogre, SubType::Wizard],
        power: Some(3), toughness: Some(3), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, it deals 5 damage to target creature an opponent controls that was dealt damage this turn.",
                vec![Effect::DealDamage { amount: 5 }],
                TargetSpec::PermanentFiltered("creature an opponent controls that was dealt damage this turn".into())),
        ],
        ..Default::default() }
}

fn veteran_ice_climber(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Veteran Ice Climber".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Scout],
        power: Some(1), toughness: Some(3), keywords: KeywordAbilities::VIGILANCE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "This creature can't be blocked.",
                vec![StaticEffect::Custom("This creature can't be blocked.".into())]),
            Ability::attacks_triggered(id,
                "Whenever this creature attacks, up to one target player mills cards equal to this creature's power.",
                vec![Effect::Mill { count: 0 }],
                TargetSpec::Player),
        ],
        ..Default::default() }
}

fn voice_of_victory(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Voice of Victory".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Custom("Bard".into())],
        power: Some(1), toughness: Some(3), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "Mobilize 2 (When this creature enters, create two 1/1 white Soldier creature tokens.)",
                vec![Effect::CreateToken { token_name: "1/1 white Soldier".into(), count: 2 }],
                TargetSpec::None),
            Ability::static_ability(id,
                "Your opponents can't cast spells during your turn.",
                vec![StaticEffect::Custom("Your opponents can't cast spells during your turn.".into())]),
        ],
        ..Default::default() }
}

fn watcher_of_the_wayside(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Watcher of the Wayside".into(), mana_cost: ManaCost::parse("{3}"),
        card_types: vec![CardType::Artifact, CardType::Creature], subtypes: vec![SubType::Golem],
        power: Some(3), toughness: Some(2), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, target player mills two cards. You gain 2 life.",
                vec![Effect::Mill { count: 2 }, Effect::gain_life(2)],
                TargetSpec::Player),
        ],
        ..Default::default() }
}

fn wingblade_disciple(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Wingblade Disciple".into(), mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Monk],
        power: Some(2), toughness: Some(2), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell_cast_triggered(id,
                "Flurry -- Whenever you cast your second spell each turn, create a 1/1 white Bird creature token with flying.",
                vec![Effect::CreateToken { token_name: "1/1 white Bird with flying".into(), count: 1 }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ── Tier 2 spells ────────────────────────────────────────────────────────────

fn caustic_exhale(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {B}. Additional cost: behold Dragon or pay {1}. Target creature -3/-3.
    CardData { id, owner, name: "Caustic Exhale".into(), mana_cost: ManaCost::parse("{B}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::boost_until_eot(-3, -3)], TargetSpec::Creature)],
        ..Default::default() }
}

fn channeled_dragonfire(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {R}. 2 damage to any target. Harmonize {5}{R}{R}.
    CardData { id, owner, name: "Channeled Dragonfire".into(), mana_cost: ManaCost::parse("{R}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::deal_damage(2)], TargetSpec::CreatureOrPlayer)],
        ..Default::default() }
}

fn cruel_truths(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {3}{B}. Surveil 2, draw 2, lose 2 life.
    CardData { id, owner, name: "Cruel Truths".into(), mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Uncommon,
        abilities: vec![Ability::spell(id, vec![Effect::scry(2), Effect::draw_cards(2), Effect::lose_life(2)], TargetSpec::None)],
        ..Default::default() }
}

fn dispelling_exhale(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{U}. Behold Dragon. Counter unless pays {2} (or {4} if beheld).
    CardData { id, owner, name: "Dispelling Exhale".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::counter_spell()], TargetSpec::Spell)],
        ..Default::default() }
}

fn dragonclaw_strike(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {2/G}{2/U}{2/R}. Double P/T of your creature, then fight.
    CardData { id, owner, name: "Dragonclaw Strike".into(), mana_cost: ManaCost::parse("{2/G}{2/U}{2/R}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Uncommon,
        abilities: vec![Ability::spell(id, vec![Effect::Custom("Double target creature's P/T, then it fights target creature you don't control.".into())], TargetSpec::Creature)],
        ..Default::default() }
}

fn dragons_prey(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {2}{B}. Destroy target creature. Costs {2} more if targeting Dragon.
    CardData { id, owner, name: "Dragon's Prey".into(), mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::destroy()], TargetSpec::Creature)],
        ..Default::default() }
}

fn knockout_maneuver(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {2}{G}. +1/+1 counter on your creature, then it fights opponent's creature.
    CardData { id, owner, name: "Knockout Maneuver".into(), mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::add_p1p1_counters(1), Effect::bite()], TargetSpec::fight_targets())],
        ..Default::default() }
}

fn molten_exhale(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {1}{R}. 4 damage to creature/planeswalker. Flash if behold Dragon.
    CardData { id, owner, name: "Molten Exhale".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::deal_damage(4)], TargetSpec::Creature)],
        ..Default::default() }
}

fn narsets_rebuke(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {4}{R}. 5 damage to creature. Add {U}{R}{W}. Exile if dies.
    CardData { id, owner, name: "Narset's Rebuke".into(), mana_cost: ManaCost::parse("{4}{R}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Uncommon,
        abilities: vec![Ability::spell(id, vec![Effect::deal_damage(5)], TargetSpec::Creature)],
        ..Default::default() }
}

fn piercing_exhale(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{G}. Behold Dragon. Your creature fights target. If beheld, surveil 2.
    CardData { id, owner, name: "Piercing Exhale".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::bite()], TargetSpec::fight_targets())],
        ..Default::default() }
}

fn rebellious_strike(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{W}. Target creature +3/+0. Draw a card.
    CardData { id, owner, name: "Rebellious Strike".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::boost_until_eot(3, 0), Effect::draw_cards(1)], TargetSpec::Creature)],
        ..Default::default() }
}

fn salt_road_skirmish(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {3}{B}. Destroy creature. Create two 1/1 Warrior tokens with haste.
    CardData { id, owner, name: "Salt Road Skirmish".into(), mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::destroy(), Effect::create_token("1/1 Warrior with haste", 2)], TargetSpec::Creature)],
        ..Default::default() }
}

fn snakeskin_veil(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {G}. +1/+1 counter on your creature. It gains hexproof until EOT.
    CardData { id, owner, name: "Snakeskin Veil".into(), mana_cost: ManaCost::parse("{G}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::add_p1p1_counters(1), Effect::hexproof()], TargetSpec::Creature)],
        ..Default::default() }
}

fn spectral_denial(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {X}{U}. Costs {1} less per power-4+ creature. Counter unless pays {X}.
    CardData { id, owner, name: "Spectral Denial".into(), mana_cost: ManaCost::parse("{X}{U}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Uncommon,
        abilities: vec![Ability::spell(id, vec![Effect::counter_spell()], TargetSpec::Spell)],
        ..Default::default() }
}

fn twin_bolt(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{R}. 2 damage divided among one or two targets.
    CardData { id, owner, name: "Twin Bolt".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::deal_damage(2)], TargetSpec::Multiple { spec: Box::new(TargetSpec::CreatureOrPlayer), count: 2 })],
        ..Default::default() }
}

// ── Tier 2 non-creature permanents ───────────────────────────────────────────

fn dragonback_assault(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment {3}{G}{U}{R}. ETB: 3 damage to all creatures/PWs. Landfall: create Dragon.
    CardData { id, owner, name: "Dragonback Assault".into(), mana_cost: ManaCost::parse("{3}{G}{U}{R}"),
        card_types: vec![CardType::Enchantment], rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Dragonback Assault enters, it deals 3 damage to each creature and planeswalker.",
                vec![Effect::DealDamageAll { amount: 3, filter: "creatures and planeswalkers".into() }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ── Tier 3 — gain lands ─────────────────────────────────────────────────────
// All gain lands: ETB tapped, gain 1 life on ETB, tap for 2 colors.

fn gain_land(id: ObjectId, owner: PlayerId, name: &str, mana1: Mana, mana2: Mana) -> CardData {
    CardData { id, owner, name: name.into(),
        card_types: vec![CardType::Land], rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id, "This land enters the battlefield tapped.",
                vec![StaticEffect::EntersTapped { filter: "self".into() }]),
            Ability::enters_battlefield_triggered(id, "When this land enters, you gain 1 life.",
                vec![Effect::gain_life(1)], TargetSpec::None),
            Ability::mana_ability(id, "{T}: Add mana.", mana1),
            Ability::mana_ability(id, "{T}: Add mana.", mana2),
        ],
        ..Default::default() }
}

fn bloodfell_caves(id: ObjectId, owner: PlayerId) -> CardData {
    gain_land(id, owner, "Bloodfell Caves", Mana::black(1), Mana::red(1))
}

fn blossoming_sands(id: ObjectId, owner: PlayerId) -> CardData {
    gain_land(id, owner, "Blossoming Sands", Mana::green(1), Mana::white(1))
}

fn dismal_backwater(id: ObjectId, owner: PlayerId) -> CardData {
    gain_land(id, owner, "Dismal Backwater", Mana::blue(1), Mana::black(1))
}

fn jungle_hollow(id: ObjectId, owner: PlayerId) -> CardData {
    gain_land(id, owner, "Jungle Hollow", Mana::black(1), Mana::green(1))
}

fn rugged_highlands(id: ObjectId, owner: PlayerId) -> CardData {
    gain_land(id, owner, "Rugged Highlands", Mana::red(1), Mana::green(1))
}

fn scoured_barrens(id: ObjectId, owner: PlayerId) -> CardData {
    gain_land(id, owner, "Scoured Barrens", Mana::white(1), Mana::black(1))
}

fn swiftwater_cliffs(id: ObjectId, owner: PlayerId) -> CardData {
    gain_land(id, owner, "Swiftwater Cliffs", Mana::blue(1), Mana::red(1))
}

fn thornwood_falls(id: ObjectId, owner: PlayerId) -> CardData {
    gain_land(id, owner, "Thornwood Falls", Mana::green(1), Mana::blue(1))
}

fn tranquil_cove(id: ObjectId, owner: PlayerId) -> CardData {
    gain_land(id, owner, "Tranquil Cove", Mana::white(1), Mana::blue(1))
}

fn wind_scarred_crag(id: ObjectId, owner: PlayerId) -> CardData {
    gain_land(id, owner, "Wind-Scarred Crag", Mana::red(1), Mana::white(1))
}

// ── Tier 3 — tri-lands ──────────────────────────────────────────────────────

fn tri_land(id: ObjectId, owner: PlayerId, name: &str, m1: Mana, m2: Mana, m3: Mana) -> CardData {
    CardData { id, owner, name: name.into(),
        card_types: vec![CardType::Land], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id, "This land enters the battlefield tapped.",
                vec![StaticEffect::EntersTapped { filter: "self".into() }]),
            Ability::mana_ability(id, "{T}: Add mana.", m1),
            Ability::mana_ability(id, "{T}: Add mana.", m2),
            Ability::mana_ability(id, "{T}: Add mana.", m3),
        ],
        ..Default::default() }
}

fn frontier_bivouac(id: ObjectId, owner: PlayerId) -> CardData {
    tri_land(id, owner, "Frontier Bivouac", Mana::green(1), Mana::blue(1), Mana::red(1))
}

fn mystic_monastery(id: ObjectId, owner: PlayerId) -> CardData {
    tri_land(id, owner, "Mystic Monastery", Mana::blue(1), Mana::red(1), Mana::white(1))
}

fn nomad_outpost(id: ObjectId, owner: PlayerId) -> CardData {
    tri_land(id, owner, "Nomad Outpost", Mana::red(1), Mana::white(1), Mana::black(1))
}

fn opulent_palace(id: ObjectId, owner: PlayerId) -> CardData {
    tri_land(id, owner, "Opulent Palace", Mana::black(1), Mana::green(1), Mana::blue(1))
}

fn sandsteppe_citadel(id: ObjectId, owner: PlayerId) -> CardData {
    tri_land(id, owner, "Sandsteppe Citadel", Mana::white(1), Mana::black(1), Mana::green(1))
}

// ── Tier 3 — instants/sorceries ─────────────────────────────────────────────

fn aggressive_negotiations(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {1}{B}. Each player sacrifices a creature.
    CardData { id, owner, name: "Aggressive Negotiations".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Uncommon,
        abilities: vec![Ability::spell(id,
            vec![Effect::Sacrifice { filter: "creature".into() }],
            TargetSpec::None)],
        ..Default::default() }
}

fn aleshas_legacy(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{B}. Target creature gains deathtouch and indestructible until EOT.
    CardData { id, owner, name: "Alesha's Legacy".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id,
            vec![Effect::gain_keyword_eot("deathtouch"), Effect::gain_keyword_eot("indestructible")],
            TargetSpec::Creature)],
        ..Default::default() }
}

fn auroral_procession(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {G}{U}. Return target card from your graveyard to your hand.
    CardData { id, owner, name: "Auroral Procession".into(), mana_cost: ManaCost::parse("{G}{U}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id,
            vec![Effect::return_from_graveyard()],
            TargetSpec::CardInYourGraveyard)],
        ..Default::default() }
}

fn bewildering_blizzard(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {4}{U}{U}. Draw 3. Opponents' creatures get -3/-0 until EOT.
    CardData { id, owner, name: "Bewildering Blizzard".into(), mana_cost: ManaCost::parse("{4}{U}{U}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Uncommon,
        abilities: vec![Ability::spell(id,
            vec![Effect::draw_cards(3),
                 Effect::Custom("Creatures your opponents control get -3/-0 until end of turn.".into())],
            TargetSpec::None)],
        ..Default::default() }
}

fn coordinated_maneuver(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{W}. Choose one -- Deal X damage (X = creatures you control) to creature/PW; or destroy target enchantment.
    CardData { id, owner, name: "Coordinated Maneuver".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Uncommon,
        abilities: vec![Ability::spell(id,
            vec![Effect::Custom("Choose one: Deal damage equal to creatures you control to target creature or planeswalker; or destroy target enchantment.".into())],
            TargetSpec::Permanent)],
        ..Default::default() }
}

fn defibrillating_current(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {2/R}{2/W}{2/B}. 4 damage to creature/PW, gain 2 life.
    CardData { id, owner, name: "Defibrillating Current".into(), mana_cost: ManaCost::parse("{2/R}{2/W}{2/B}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id,
            vec![Effect::deal_damage(4), Effect::gain_life(2)],
            TargetSpec::Creature)],
        ..Default::default() }
}

fn desperate_measures(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {B}. Target creature gets +1/-1. When it dies this turn, draw 2.
    CardData { id, owner, name: "Desperate Measures".into(), mana_cost: ManaCost::parse("{B}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id,
            vec![Effect::boost_until_eot(1, -1),
                 Effect::Custom("When it dies under your control this turn, draw two cards.".into())],
            TargetSpec::Creature)],
        ..Default::default() }
}

fn duty_beyond_death(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {1}{W}. Return creature from graveyard to hand. Create 1/1 Spirit token with flying.
    CardData { id, owner, name: "Duty Beyond Death".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id,
            vec![Effect::return_from_graveyard(), Effect::create_token("1/1 white Spirit with flying", 1)],
            TargetSpec::CardInYourGraveyard)],
        ..Default::default() }
}

fn frontline_rush(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {R}{W}. Choose one: create 2 goblin tokens; or creature gets +X/+X (X = creatures you control).
    CardData { id, owner, name: "Frontline Rush".into(), mana_cost: ManaCost::parse("{R}{W}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Uncommon,
        abilities: vec![Ability::spell(id,
            vec![Effect::Custom("Choose one: Create two 1/1 red Goblin creature tokens; or target creature gets +X/+X until end of turn, X = creatures you control.".into())],
            TargetSpec::None)],
        ..Default::default() }
}

fn inevitable_defeat(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{R}{W}{B}. Can't be countered. Exile target nonland permanent. Controller loses 3 life, you gain 3.
    CardData { id, owner, name: "Inevitable Defeat".into(), mana_cost: ManaCost::parse("{1}{R}{W}{B}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Rare,
        abilities: vec![Ability::spell(id,
            vec![Effect::exile(), Effect::lose_life(3), Effect::gain_life(3)],
            TargetSpec::PermanentFiltered("nonland permanent".into()))],
        ..Default::default() }
}

fn jeskai_revelation(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {4}{U}{R}{W}. Bounce spell/permanent, 4 damage any target, 2 Monk tokens, draw 2, gain 4 life.
    CardData { id, owner, name: "Jeskai Revelation".into(), mana_cost: ManaCost::parse("{4}{U}{R}{W}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Mythic,
        abilities: vec![Ability::spell(id,
            vec![Effect::bounce(), Effect::deal_damage(4),
                 Effect::create_token("1/1 white Monk with prowess", 2),
                 Effect::draw_cards(2), Effect::gain_life(4)],
            TargetSpec::Multiple { spec: Box::new(TargetSpec::CreatureOrPlayer), count: 2 })],
        ..Default::default() }
}

fn kin_tree_severance(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {2/W}{2/B}{2/G}. Exile target permanent with MV 3+.
    CardData { id, owner, name: "Kin-Tree Severance".into(), mana_cost: ManaCost::parse("{2/W}{2/B}{2/G}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Uncommon,
        abilities: vec![Ability::spell(id,
            vec![Effect::exile()],
            TargetSpec::PermanentFiltered("permanent with mana value 3 or greater".into()))],
        ..Default::default() }
}

fn lightfoot_technique(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{W}. +1/+1 counter on target creature. Gains flying and indestructible until EOT.
    CardData { id, owner, name: "Lightfoot Technique".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id,
            vec![Effect::add_p1p1_counters(1), Effect::gain_keyword_eot("flying"), Effect::gain_keyword_eot("indestructible")],
            TargetSpec::Creature)],
        ..Default::default() }
}

fn mammoth_bellow(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {2}{G}{U}{R}. Create a 5/5 green Elephant creature token.
    CardData { id, owner, name: "Mammoth Bellow".into(), mana_cost: ManaCost::parse("{2}{G}{U}{R}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Uncommon,
        abilities: vec![Ability::spell(id,
            vec![Effect::create_token("5/5 green Elephant", 1)],
            TargetSpec::None)],
        ..Default::default() }
}

fn overwhelming_surge(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {2}{R}. Choose one or both: 3 damage to creature; destroy noncreature artifact.
    CardData { id, owner, name: "Overwhelming Surge".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Uncommon,
        abilities: vec![Ability::spell(id,
            vec![Effect::Custom("Choose one or both: 3 damage to target creature; destroy target noncreature artifact.".into())],
            TargetSpec::Permanent)],
        ..Default::default() }
}

fn perennation(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {3}{W}{B}{G}. Return target permanent from graveyard to battlefield with hexproof and indestructible counters.
    CardData { id, owner, name: "Perennation".into(), mana_cost: ManaCost::parse("{3}{W}{B}{G}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Rare,
        abilities: vec![Ability::spell(id,
            vec![Effect::reanimate()],
            TargetSpec::CardInYourGraveyard)],
        ..Default::default() }
}

fn rakshasas_bargain(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {2/B}{2/G}{2/U}. Look at top 4, put 2 in hand, rest in graveyard.
    CardData { id, owner, name: "Rakshasa's Bargain".into(), mana_cost: ManaCost::parse("{2/B}{2/G}{2/U}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id,
            vec![Effect::Custom("Look at the top four cards of your library. Put two into your hand and the rest into your graveyard.".into())],
            TargetSpec::None)],
        ..Default::default() }
}

fn riverwalk_technique(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {3}{U}. Choose one: put nonland permanent on top/bottom of library; or counter noncreature spell.
    CardData { id, owner, name: "Riverwalk Technique".into(), mana_cost: ManaCost::parse("{3}{U}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id,
            vec![Effect::Custom("Choose one: Owner of target nonland permanent puts it on top or bottom of library; or counter target noncreature spell.".into())],
            TargetSpec::Permanent)],
        ..Default::default() }
}

fn roamers_routine(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {2}{G}. Search library for basic land, put onto battlefield tapped.
    CardData { id, owner, name: "Roamer's Routine".into(), mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id,
            vec![Effect::search_library("basic land card")],
            TargetSpec::None)],
        ..Default::default() }
}

fn sarkhans_resolve(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{G}. Choose one: +3/+3 target creature; or destroy target creature with flying.
    CardData { id, owner, name: "Sarkhan's Resolve".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id,
            vec![Effect::Custom("Choose one: Target creature gets +3/+3 until end of turn; or destroy target creature with flying.".into())],
            TargetSpec::Creature)],
        ..Default::default() }
}

fn seize_opportunity(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {2}{R}. Choose one: exile top 2, play until end of next turn; or up to 2 creatures get +2/+1.
    CardData { id, owner, name: "Seize Opportunity".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Uncommon,
        abilities: vec![Ability::spell(id,
            vec![Effect::Custom("Choose one: Exile top 2, play until end of next turn; or up to 2 creatures get +2/+1 until end of turn.".into())],
            TargetSpec::None)],
        ..Default::default() }
}

fn unending_whisper(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {U}. Draw a card.
    CardData { id, owner, name: "Unending Whisper".into(), mana_cost: ManaCost::parse("{U}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id,
            vec![Effect::draw_cards(1)],
            TargetSpec::None)],
        ..Default::default() }
}

fn urenis_rebuff(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {1}{U}. Return target creature to its owner's hand.
    CardData { id, owner, name: "Ureni's Rebuff".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id,
            vec![Effect::bounce()],
            TargetSpec::Creature)],
        ..Default::default() }
}

fn wild_ride(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {R}. Target creature gets +3/+0 and gains haste until EOT.
    CardData { id, owner, name: "Wild Ride".into(), mana_cost: ManaCost::parse("{R}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id,
            vec![Effect::boost_until_eot(3, 0), Effect::gain_keyword_eot("haste")],
            TargetSpec::Creature)],
        ..Default::default() }
}

fn worthy_cost(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {B}. Additional cost: sacrifice a creature. Exile target creature/PW.
    CardData { id, owner, name: "Worthy Cost".into(), mana_cost: ManaCost::parse("{B}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Uncommon,
        abilities: vec![Ability::spell(id,
            vec![Effect::exile()],
            TargetSpec::Creature)
            .with_rules_text("As an additional cost, sacrifice a creature. Exile target creature or planeswalker.")],
        ..Default::default() }
}

// ── Tier 3 — creatures with abilities ────────────────────────────────────────

fn abzan_devotee(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/3 Human Soldier for {1}{W}{B}{G}. ETB: create 1/1 Spirit token with flying.
    CardData { id, owner, name: "Abzan Devotee".into(), mana_cost: ManaCost::parse("{1}{W}{B}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Soldier],
        power: Some(3), toughness: Some(3), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Abzan Devotee enters, create a 1/1 white Spirit creature token with flying.",
                vec![Effect::create_token("1/1 white Spirit with flying", 1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn bearer_of_glory(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/3 Human Cleric for {1}{W}. ETB: gain 3 life.
    CardData { id, owner, name: "Bearer of Glory".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Cleric],
        power: Some(2), toughness: Some(3), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Bearer of Glory enters, you gain 3 life.",
                vec![Effect::gain_life(3)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn bone_cairn_butcher(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/4 Orc Warrior for {3}{R}{W}{B}. Menace, haste. ETB: each opponent sacrifices a creature.
    CardData { id, owner, name: "Bone-Cairn Butcher".into(), mana_cost: ManaCost::parse("{3}{R}{W}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Orc, SubType::Warrior],
        power: Some(4), toughness: Some(4),
        keywords: KeywordAbilities::MENACE | KeywordAbilities::HASTE,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Bone-Cairn Butcher enters, each opponent sacrifices a creature.",
                vec![Effect::Sacrifice { filter: "opponent creature".into() }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn devoted_duelist(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/1 Goblin Monk for {1}{R}. Haste. Flurry: 1 damage to each opponent.
    CardData { id, owner, name: "Devoted Duelist".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Monk],
        power: Some(2), toughness: Some(1), keywords: KeywordAbilities::HASTE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell_cast_triggered(id,
                "Flurry -- Whenever you cast your second spell each turn, this creature deals 1 damage to each opponent.",
                vec![Effect::damage_opponents(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn dusyut_earthcarver(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/4 Elephant Druid for {5}{G}. Reach. ETB: endure 3 (put +1/+1 counters).
    CardData { id, owner, name: "Dusyut Earthcarver".into(), mana_cost: ManaCost::parse("{5}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elephant, SubType::Druid],
        power: Some(4), toughness: Some(4), keywords: KeywordAbilities::REACH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, it endures 3. (Put three +1/+1 counters on it, then remove that many.)",
                vec![Effect::add_p1p1_counters(3)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn mardu_devotee(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/3 Human Warrior for {1}{R}{W}{B}. ETB: deal 2 damage to target creature/PW.
    CardData { id, owner, name: "Mardu Devotee".into(), mana_cost: ManaCost::parse("{1}{R}{W}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Warrior],
        power: Some(3), toughness: Some(3), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Mardu Devotee enters, it deals 2 damage to target creature or planeswalker.",
                vec![Effect::deal_damage(2)],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn meticulous_artisan(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/3 Djinn Artificer for {3}{R}. Prowess. ETB: create Treasure token.
    CardData { id, owner, name: "Meticulous Artisan".into(), mana_cost: ManaCost::parse("{3}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Djinn, SubType::Custom("Artificer".into())],
        power: Some(3), toughness: Some(3), keywords: KeywordAbilities::PROWESS,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, create a Treasure token.",
                vec![Effect::create_token("Treasure", 1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn rainveil_rejuvenator(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/3 Naga Druid for {2}{G}{U}. ETB: draw a card and gain 3 life.
    CardData { id, owner, name: "Rainveil Rejuvenator".into(), mana_cost: ManaCost::parse("{2}{G}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Custom("Naga".into()), SubType::Druid],
        power: Some(3), toughness: Some(3), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Rainveil Rejuvenator enters, draw a card and gain 3 life.",
                vec![Effect::draw_cards(1), Effect::gain_life(3)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn salt_road_packbeast(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/3 Beast for {3}{W}{B}{G}. Vigilance. ETB: return target permanent card from graveyard to hand.
    CardData { id, owner, name: "Salt Road Packbeast".into(), mana_cost: ManaCost::parse("{3}{W}{B}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Beast],
        power: Some(3), toughness: Some(3), keywords: KeywordAbilities::VIGILANCE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Salt Road Packbeast enters, return target permanent card from your graveyard to your hand.",
                vec![Effect::return_from_graveyard()],
                TargetSpec::CardInYourGraveyard),
        ],
        ..Default::default() }
}

fn sultai_devotee(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/3 Naga Wizard for {1}{B}{G}{U}. ETB: mill 3, then return a creature card from graveyard to hand.
    CardData { id, owner, name: "Sultai Devotee".into(), mana_cost: ManaCost::parse("{1}{B}{G}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Custom("Naga".into()), SubType::Wizard],
        power: Some(3), toughness: Some(3), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Sultai Devotee enters, mill three cards, then you may return a creature card from your graveyard to your hand.",
                vec![Effect::mill(3), Effect::return_from_graveyard()],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn temur_devotee(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/4 Human Shaman for {2}{G}{U}{R}. ETB: draw 2 cards.
    CardData { id, owner, name: "Temur Devotee".into(), mana_cost: ManaCost::parse("{2}{G}{U}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Shaman],
        power: Some(4), toughness: Some(4), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Temur Devotee enters, draw two cards.",
                vec![Effect::draw_cards(2)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn unburied_earthcarver(id: ObjectId, owner: PlayerId) -> CardData {
    // 5/5 Zombie Druid for {5}{B}. ETB: put two -1/-1 counters on target creature an opponent controls.
    CardData { id, owner, name: "Unburied Earthcarver".into(), mana_cost: ManaCost::parse("{5}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Zombie, SubType::Druid],
        power: Some(5), toughness: Some(5), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Unburied Earthcarver enters, put two -1/-1 counters on target creature an opponent controls.",
                vec![Effect::add_counters("-1/-1", 2)],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn zurgos_vanguard(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/3 Orc Warrior for {R}{W}. Haste. Dies: create 1/1 Warrior token with haste.
    CardData { id, owner, name: "Zurgo's Vanguard".into(), mana_cost: ManaCost::parse("{R}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Orc, SubType::Warrior],
        power: Some(2), toughness: Some(3), keywords: KeywordAbilities::HASTE,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::dies_triggered(id,
                "When Zurgo's Vanguard dies, create a 1/1 red Warrior creature token with haste.",
                vec![Effect::create_token("1/1 red Warrior with haste", 1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ── Tier 3 — enchantments/artifacts with abilities ───────────────────────────

fn dracogenesis(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment {6}{R}{R}. You may cast Dragon spells without paying their mana costs.
    CardData { id, owner, name: "Dracogenesis".into(), mana_cost: ManaCost::parse("{6}{R}{R}"),
        card_types: vec![CardType::Enchantment], rarity: Rarity::Mythic,
        abilities: vec![
            Ability::static_ability(id, "You may cast Dragon spells without paying their mana costs.",
                vec![StaticEffect::Custom("Cast Dragon spells for free".into())]),
        ],
        ..Default::default() }
}

fn dragonstorm_globe(id: ObjectId, owner: PlayerId) -> CardData {
    // Artifact {3}. Each Dragon enters with an extra +1/+1 counter. {T}: add any color.
    CardData { id, owner, name: "Dragonstorm Globe".into(), mana_cost: ManaCost::parse("{3}"),
        card_types: vec![CardType::Artifact], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id, "Each Dragon you control enters with an additional +1/+1 counter on it.",
                vec![StaticEffect::Custom("Dragons enter with extra +1/+1 counter".into())]),
            Ability::mana_ability(id, "{T}: Add one mana of any color.", Mana::generic(1)),
        ],
        ..Default::default() }
}

fn encroaching_dragonstorm(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment {3}{G}. ETB: search for up to 2 basic lands, put tapped. Dragon ETB: return to hand.
    CardData { id, owner, name: "Encroaching Dragonstorm".into(), mana_cost: ManaCost::parse("{3}{G}"),
        card_types: vec![CardType::Enchantment], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this enchantment enters, search your library for up to two basic land cards, put them onto the battlefield tapped, then shuffle.",
                vec![Effect::search_library("up to two basic land cards")],
                TargetSpec::None),
            Ability::other_creature_etb_triggered(id,
                "When a Dragon you control enters, return this enchantment to its owner's hand.",
                vec![Effect::bounce()],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn roiling_dragonstorm(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment {1}{U}. ETB: draw 2, discard 1. Dragon ETB: return to hand.
    CardData { id, owner, name: "Roiling Dragonstorm".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Enchantment], rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this enchantment enters, draw two cards, then discard a card.",
                vec![Effect::draw_cards(2), Effect::discard_cards(1)],
                TargetSpec::None),
            Ability::other_creature_etb_triggered(id,
                "When a Dragon you control enters, return this enchantment to its owner's hand.",
                vec![Effect::bounce()],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn stormplain_detainment(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment {2}{W}. ETB: exile target nonland permanent opponent controls until this leaves.
    CardData { id, owner, name: "Stormplain Detainment".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Enchantment], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this enchantment enters, exile target nonland permanent an opponent controls until this enchantment leaves the battlefield.",
                vec![Effect::exile()],
                TargetSpec::PermanentFiltered("nonland permanent an opponent controls".into())),
        ],
        ..Default::default() }
}

fn teeming_dragonstorm(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment {3}{W}. ETB: create two 2/2 Soldier tokens. Dragon ETB: return to hand.
    CardData { id, owner, name: "Teeming Dragonstorm".into(), mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Enchantment], rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this enchantment enters, create two 2/2 white Soldier creature tokens.",
                vec![Effect::create_token("2/2 white Soldier", 2)],
                TargetSpec::None),
            Ability::other_creature_etb_triggered(id,
                "When a Dragon you control enters, return this enchantment to its owner's hand.",
                vec![Effect::bounce()],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ── Tier 3 batch 2 — complex card implementations ──────────────────────────

fn abzan_monument(id: ObjectId, owner: PlayerId) -> CardData {
    // Artifact for {2}. ETB: search for basic Plains/Swamp/Forest.
    // {3}{W}{B}{G}, {T}, Sacrifice: create X/X Spirit token (X = greatest power among creatures you control).
    CardData { id, owner, name: "Abzan Monument".into(), mana_cost: ManaCost::parse("{2}"),
        card_types: vec![CardType::Artifact], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this artifact enters, search your library for a basic Plains, Swamp, or Forest card, reveal it, put it into your hand, then shuffle.",
                vec![Effect::search_library("basic Plains, Swamp, or Forest card")],
                TargetSpec::None),
            Ability::activated(id,
                "{3}{W}{B}{G}, {T}, Sacrifice this artifact: Create an X/X white Spirit creature token, where X is the greatest power among creatures you control.",
                vec![Cost::pay_mana("{3}{W}{B}{G}"), Cost::tap_self(), Cost::sacrifice_self()],
                vec![Effect::create_token("X/X Spirit", 1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn all_out_assault(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment for {2}{R}{W}{B}. Creatures you control get +1/+1 and have deathtouch.
    // ETB (main phase): additional combat + main phase; when you next attack, untap all creatures.
    CardData { id, owner, name: "All-Out Assault".into(), mana_cost: ManaCost::parse("{2}{R}{W}{B}"),
        card_types: vec![CardType::Enchantment], rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "Creatures you control get +1/+1 and have deathtouch.",
                vec![StaticEffect::boost_controlled("creatures you control", 1, 1),
                     StaticEffect::grant_keyword_controlled("creatures you control", "deathtouch")]),
            Ability::enters_battlefield_triggered(id,
                "When this enchantment enters, if it's your main phase, there is an additional combat phase after this phase followed by an additional main phase. When you next attack this turn, untap each creature you control.",
                vec![Effect::Custom("Additional combat phase + untap all on attack.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn ambling_stormshell(id: ObjectId, owner: PlayerId) -> CardData {
    // 5/9 Turtle for {3}{U}{U}. Ward {2}.
    // Attacks: put 3 stun counters on it, draw 3. Whenever you cast a Turtle spell, untap this.
    CardData { id, owner, name: "Ambling Stormshell".into(), mana_cost: ManaCost::parse("{3}{U}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Turtle],
        power: Some(5), toughness: Some(9), keywords: KeywordAbilities::WARD, rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id, "Ward {2}",
                vec![StaticEffect::ward("{2}")]),
            Ability::triggered(id,
                "Whenever this creature attacks, put three stun counters on it and draw three cards.",
                vec![EventType::AttackerDeclared],
                vec![Effect::AddCounters { counter_type: "stun".into(), count: 3 }, Effect::draw_cards(3)],
                TargetSpec::None),
            Ability::spell_cast_triggered(id,
                "Whenever you cast a Turtle spell, untap this creature.",
                vec![Effect::UntapTarget],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn anafenza_unyielding_lineage(id: ObjectId, owner: PlayerId) -> CardData {
    // Legendary 2/2 Spirit Soldier for {2}{W}. Flash, first strike.
    // Whenever another nontoken creature you control dies, Anafenza endures 2.
    CardData { id, owner, name: "Anafenza, Unyielding Lineage".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Spirit, SubType::Soldier],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(2),
        keywords: KeywordAbilities::FLASH | KeywordAbilities::FIRST_STRIKE,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::triggered(id,
                "Whenever another nontoken creature you control dies, Anafenza endures 2.",
                vec![EventType::Dies],
                vec![Effect::Custom("Endure 2 (put two +1/+1 counters; if it would die, exile with counters instead).".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn barrensteppe_siege(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment for {2}{W}{B}. As ETB: choose Abzan or Mardu.
    // Abzan: end step — +1/+1 counter on each creature you control.
    // Mardu: end step — if creature died this turn, opponents sacrifice a creature.
    CardData { id, owner, name: "Barrensteppe Siege".into(), mana_cost: ManaCost::parse("{2}{W}{B}"),
        card_types: vec![CardType::Enchantment], rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "As this enchantment enters, choose Abzan or Mardu.",
                vec![Effect::Custom("Choose Abzan or Mardu.".into())],
                TargetSpec::None),
            Ability::triggered(id,
                "Abzan — At the beginning of your end step, put a +1/+1 counter on each creature you control.",
                vec![EventType::EndStep],
                vec![Effect::add_counters_all("+1/+1", 1, "creatures you control")],
                TargetSpec::None),
            Ability::triggered(id,
                "Mardu — At the beginning of your end step, if a creature died under your control this turn, each opponent sacrifices a creature.",
                vec![EventType::EndStep],
                vec![Effect::Custom("Each opponent sacrifices a creature.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn betor_kin_to_all(id: ObjectId, owner: PlayerId) -> CardData {
    // Legendary 5/7 Spirit Dragon for {2}{W}{B}{G}. Flying.
    // End step: if total toughness >= 10, draw; >= 20, untap all; >= 40, opponents lose half life.
    CardData { id, owner, name: "Betor, Kin to All".into(), mana_cost: ManaCost::parse("{2}{W}{B}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Spirit, SubType::Dragon],
        supertypes: vec![SuperType::Legendary],
        power: Some(5), toughness: Some(7), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Mythic,
        abilities: vec![
            Ability::triggered(id,
                "At the beginning of your end step, if creatures you control have total toughness 10 or greater, draw a card. Then if 20 or greater, untap each creature you control. Then if 40 or greater, each opponent loses half their life, rounded up.",
                vec![EventType::EndStep],
                vec![Effect::Custom("Toughness threshold: 10→draw, 20→untap, 40→opponents lose half life.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn breaching_dragonstorm(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment for {4}{R}. ETB: exile cards until nonland, may cast if MV <= 8, else put in hand.
    // When a Dragon ETBs, return this to hand.
    CardData { id, owner, name: "Breaching Dragonstorm".into(), mana_cost: ManaCost::parse("{4}{R}"),
        card_types: vec![CardType::Enchantment], rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this enchantment enters, exile cards from the top of your library until you exile a nonland card. You may cast it without paying its mana cost if its mana value is 8 or less. If you don't, put that card into your hand.",
                vec![Effect::Custom("Cascade-like: exile until nonland, free cast if MV <= 8.".into())],
                TargetSpec::None),
            Ability::other_creature_etb_triggered(id,
                "When a Dragon you control enters, return this enchantment to its owner's hand.",
                vec![Effect::bounce()],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn call_the_spirit_dragons(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment for {W}{U}{B}{R}{G}. Dragons you control have indestructible.
    // Upkeep: for each color, +1/+1 counter on a Dragon of that color; if 5 Dragons countered, you win.
    CardData { id, owner, name: "Call the Spirit Dragons".into(), mana_cost: ManaCost::parse("{W}{U}{B}{R}{G}"),
        card_types: vec![CardType::Enchantment], rarity: Rarity::Mythic,
        abilities: vec![
            Ability::static_ability(id,
                "Dragons you control have indestructible.",
                vec![StaticEffect::grant_keyword_controlled("Dragons you control", "indestructible")]),
            Ability::triggered(id,
                "At the beginning of your upkeep, for each color, put a +1/+1 counter on a Dragon you control of that color. If you put +1/+1 counters on five Dragons this way, you win the game.",
                vec![EventType::UpkeepStep],
                vec![Effect::Custom("Put counters on Dragons by color. 5 Dragons = win the game.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn cori_mountain_monastery(id: ObjectId, owner: PlayerId) -> CardData {
    // Land. Enters tapped unless you control a Plains or Island. {T}: Add {R}.
    // {3}{R}, {T}: Exile top card, play until end of next turn.
    CardData { id, owner, name: "Cori Mountain Monastery".into(),
        card_types: vec![CardType::Land], rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "This land enters tapped unless you control a Plains or an Island.",
                vec![StaticEffect::enters_tapped_unless("you control a Plains or an Island")]),
            Ability::mana_ability(id, "{T}: Add {R}.", Mana::red(1)),
            Ability::activated(id,
                "{3}{R}, {T}: Exile the top card of your library. Until the end of your next turn, you may play that card.",
                vec![Cost::pay_mana("{3}{R}"), Cost::tap_self()],
                vec![Effect::Custom("Exile top card, play until end of next turn.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn cori_steel_cutter(id: ObjectId, owner: PlayerId) -> CardData {
    // Artifact — Equipment for {1}{R}. Equipped creature gets +2/+0 and has trample and haste.
    // Flurry: create 1/1 Monk token attached to this. Equip {3}.
    CardData { id, owner, name: "Cori Steel-Cutter".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Artifact], subtypes: vec![SubType::Equipment],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                "Equipped creature gets +2/+0 and has trample and haste.",
                vec![StaticEffect::boost_controlled("equipped creature", 2, 0),
                     StaticEffect::grant_keyword_controlled("equipped creature", "trample"),
                     StaticEffect::grant_keyword_controlled("equipped creature", "haste")]),
            Ability::spell_cast_triggered(id,
                "Flurry — Whenever you cast your second spell each turn, create a 1/1 white Monk creature token, then attach this Equipment to it.",
                vec![Effect::create_token("1/1 Monk", 1)],
                TargetSpec::None),
            Ability::activated(id, "Equip {3}",
                vec![Cost::pay_mana("{3}")],
                vec![Effect::equip()],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn dalkovan_encampment(id: ObjectId, owner: PlayerId) -> CardData {
    // Land. Enters tapped unless you control Swamp or Mountain. {T}: Add {W}.
    // {3}{W}, {T}: Create 1/1 Soldier token.
    CardData { id, owner, name: "Dalkovan Encampment".into(),
        card_types: vec![CardType::Land], rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "This land enters tapped unless you control a Swamp or a Mountain.",
                vec![StaticEffect::enters_tapped_unless("you control a Swamp or a Mountain")]),
            Ability::mana_ability(id, "{T}: Add {W}.", Mana::white(1)),
            Ability::activated(id,
                "{3}{W}, {T}: Create a 1/1 white Soldier creature token.",
                vec![Cost::pay_mana("{3}{W}"), Cost::tap_self()],
                vec![Effect::create_token("1/1 Soldier", 1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn death_begets_life(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery for {2}{W}{B}. Destroy all creatures. Create X 1/1 Spirit tokens (X = creatures destroyed).
    CardData { id, owner, name: "Death Begets Life".into(), mana_cost: ManaCost::parse("{2}{W}{B}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Rare,
        abilities: vec![Ability::spell(id,
            vec![Effect::DestroyAll { filter: "creature".into() },
                 Effect::Custom("Create X 1/1 white Spirit creature tokens with flying, where X is the number of creatures destroyed this way.".into())],
            TargetSpec::None)],
        ..Default::default() }
}

fn dragonfire_blade(id: ObjectId, owner: PlayerId) -> CardData {
    // Artifact — Equipment for {2}. Equipped creature gets +1/+0. Equip {1}.
    // When equipped creature deals combat damage, deal that much to target creature opponent controls.
    CardData { id, owner, name: "Dragonfire Blade".into(), mana_cost: ManaCost::parse("{2}"),
        card_types: vec![CardType::Artifact], subtypes: vec![SubType::Equipment],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id, "Equipped creature gets +1/+0.",
                vec![StaticEffect::boost_controlled("equipped creature", 1, 0)]),
            Ability::triggered(id,
                "Whenever equipped creature deals combat damage to a player, Dragonfire Blade deals that much damage to target creature that player controls.",
                vec![EventType::DamagedPlayer],
                vec![Effect::Custom("Deal combat damage amount to target creature opponent controls.".into())],
                TargetSpec::Creature),
            Ability::activated(id, "Equip {1}",
                vec![Cost::pay_mana("{1}")],
                vec![Effect::equip()],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn dragonologist(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/3 Human Wizard for {U}. When ETB or whenever you cast a Dragon spell, scry 1.
    CardData { id, owner, name: "Dragonologist".into(), mana_cost: ManaCost::parse("{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Wizard],
        power: Some(1), toughness: Some(3), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Dragonologist enters, scry 1.",
                vec![Effect::scry(1)],
                TargetSpec::None),
            Ability::spell_cast_triggered(id,
                "Whenever you cast a Dragon spell, scry 1.",
                vec![Effect::scry(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn effortless_master(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/4 Human Monk for {3}{U}. Flash.
    // Flurry: target creature gets -4/-0 until your next turn.
    CardData { id, owner, name: "Effortless Master".into(), mana_cost: ManaCost::parse("{3}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Monk],
        power: Some(3), toughness: Some(4), keywords: KeywordAbilities::FLASH,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell_cast_triggered(id,
                "Flurry — Whenever you cast your second spell each turn, target creature gets -4/-0 until your next turn.",
                vec![Effect::boost_until_eot(-4, 0)],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn embermouth_sentinel(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/4 Human Warrior for {3}{R}. Haste. Attacks: deal 1 damage to each opponent.
    CardData { id, owner, name: "Embermouth Sentinel".into(), mana_cost: ManaCost::parse("{3}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Warrior],
        power: Some(4), toughness: Some(4), keywords: KeywordAbilities::HASTE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever Embermouth Sentinel attacks, it deals 1 damage to each opponent.",
                vec![EventType::AttackerDeclared],
                vec![Effect::damage_opponents(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn eshki_dragonclaw(id: ObjectId, owner: PlayerId) -> CardData {
    // Legendary 4/4 Human Warrior for {1}{R}{G}. Trample.
    // Whenever attacks: +1/+1 counter. If power >= 7, creatures you control get +2/+0 and trample until EOT.
    CardData { id, owner, name: "Eshki, Dragonclaw".into(), mana_cost: ManaCost::parse("{1}{R}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Warrior],
        supertypes: vec![SuperType::Legendary],
        power: Some(4), toughness: Some(4), keywords: KeywordAbilities::TRAMPLE,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::triggered(id,
                "Whenever Eshki attacks, put a +1/+1 counter on it. If its power is 7 or greater, creatures you control get +2/+0 and gain trample until end of turn.",
                vec![EventType::AttackerDeclared],
                vec![Effect::add_p1p1_counters(1), Effect::Custom("If power >= 7: creatures +2/+0 and trample.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn fangkeepers_familiar(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/1 Snake for {1}{G}. Deathtouch. Dies: search for basic land, put on battlefield tapped.
    CardData { id, owner, name: "Fangkeeper's Familiar".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Snake],
        power: Some(2), toughness: Some(1), keywords: KeywordAbilities::DEATHTOUCH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::dies_triggered(id,
                "When Fangkeeper's Familiar dies, search your library for a basic land card, put it onto the battlefield tapped, then shuffle.",
                vec![Effect::search_library("basic land card")],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn feral_deathgorger(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/3 Beast for {1}{B}{G}. Deathtouch. Whenever a creature an opponent controls dies, put +1/+1 counter on this.
    CardData { id, owner, name: "Feral Deathgorger".into(), mana_cost: ManaCost::parse("{1}{B}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Beast],
        power: Some(3), toughness: Some(3), keywords: KeywordAbilities::DEATHTOUCH,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                "Whenever a creature an opponent controls dies, put a +1/+1 counter on Feral Deathgorger.",
                vec![EventType::Dies],
                vec![Effect::add_p1p1_counters(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn fire_rim_form(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment — Aura for {U}{R}. Enchanted creature gets +2/+2 and has flying.
    // When enchanted creature dies, return this to hand.
    CardData { id, owner, name: "Fire-Rim Form".into(), mana_cost: ManaCost::parse("{U}{R}"),
        card_types: vec![CardType::Enchantment], subtypes: vec![SubType::Aura],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                "Enchanted creature gets +2/+2 and has flying.",
                vec![StaticEffect::boost_controlled("enchanted creature", 2, 2),
                     StaticEffect::grant_keyword_controlled("enchanted creature", "flying")]),
            Ability::triggered(id,
                "When enchanted creature dies, return Fire-Rim Form to its owner's hand.",
                vec![EventType::Dies],
                vec![Effect::return_from_graveyard()],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn formation_breaker(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/3 Human Warrior for {2}{R}. Haste. When attacks alone, gets +2/+0 and gains menace until EOT.
    CardData { id, owner, name: "Formation Breaker".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Warrior],
        power: Some(4), toughness: Some(3), keywords: KeywordAbilities::HASTE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever Formation Breaker attacks alone, it gets +2/+0 and gains menace until end of turn.",
                vec![EventType::AttackerDeclared],
                vec![Effect::boost_until_eot(2, 0), Effect::gain_keyword_eot("menace")],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn fresh_start(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery for {3}{W}. Each player shuffles hand and graveyard into library, then draws 7.
    CardData { id, owner, name: "Fresh Start".into(), mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Rare,
        abilities: vec![Ability::spell(id,
            vec![Effect::Custom("Each player shuffles their hand and graveyard into their library, then draws seven cards.".into())],
            TargetSpec::None)],
        ..Default::default() }
}

fn frostcliff_siege(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment for {2}{U}{R}. Choose Jeskai or Temur.
    // Jeskai: whenever you cast noncreature spell, draw a card then discard.
    // Temur: whenever a creature ETBs under your control, it gets +2/+0 until EOT.
    CardData { id, owner, name: "Frostcliff Siege".into(), mana_cost: ManaCost::parse("{2}{U}{R}"),
        card_types: vec![CardType::Enchantment], rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "As this enchantment enters, choose Jeskai or Temur.",
                vec![Effect::Custom("Choose Jeskai or Temur.".into())],
                TargetSpec::None),
            Ability::spell_cast_triggered(id,
                "Jeskai — Whenever you cast a noncreature spell, draw a card, then discard a card.",
                vec![Effect::draw_cards(1), Effect::discard_cards(1)],
                TargetSpec::None),
            Ability::triggered(id,
                "Temur — Whenever a creature enters under your control, it gets +2/+0 until end of turn.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::boost_until_eot(2, 0)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn glacierwood_siege(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment for {2}{B}{G}. Choose Sultai or Abzan.
    // Sultai: whenever a creature you control dies, put +1/+1 counter on target creature you control.
    // Abzan: at beginning of end step, if no creatures died this turn, draw a card.
    CardData { id, owner, name: "Glacierwood Siege".into(), mana_cost: ManaCost::parse("{2}{B}{G}"),
        card_types: vec![CardType::Enchantment], rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "As this enchantment enters, choose Sultai or Abzan.",
                vec![Effect::Custom("Choose Sultai or Abzan.".into())],
                TargetSpec::None),
            Ability::triggered(id,
                "Sultai — Whenever a creature you control dies, put a +1/+1 counter on target creature you control.",
                vec![EventType::Dies],
                vec![Effect::add_p1p1_counters(1)],
                TargetSpec::Creature),
            Ability::triggered(id,
                "Abzan — At the beginning of your end step, if no creatures died this turn, draw a card.",
                vec![EventType::EndStep],
                vec![Effect::draw_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn great_arashin_city(id: ObjectId, owner: PlayerId) -> CardData {
    // Land. Enters tapped. {T}: Add one mana of any color. {5}, {T}: Gain 5 life.
    CardData { id, owner, name: "Great Arashin City".into(),
        card_types: vec![CardType::Land], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id, "Great Arashin City enters tapped.",
                vec![StaticEffect::EntersTapped { filter: "self".into() }]),
            Ability::mana_ability(id, "{T}: Add one mana of any color.", Mana::colorless(1)),
            Ability::activated(id, "{5}, {T}: You gain 5 life.",
                vec![Cost::pay_mana("{5}"), Cost::tap_self()],
                vec![Effect::gain_life(5)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn heritage_reclamation(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant for {3}{G}. Return target permanent card from graveyard to hand. If creature, may put on battlefield instead.
    CardData { id, owner, name: "Heritage Reclamation".into(), mana_cost: ManaCost::parse("{3}{G}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Uncommon,
        abilities: vec![Ability::spell(id,
            vec![Effect::return_from_graveyard()],
            TargetSpec::CardInYourGraveyard)],
        ..Default::default() }
}

fn hollowmurk_siege(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment for {2}{R}{G}. Choose Temur or Mardu.
    // Temur: creatures you control get +1/+0 and have trample.
    // Mardu: whenever a creature you control attacks alone, create a Treasure token.
    CardData { id, owner, name: "Hollowmurk Siege".into(), mana_cost: ManaCost::parse("{2}{R}{G}"),
        card_types: vec![CardType::Enchantment], rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "As this enchantment enters, choose Temur or Mardu.",
                vec![Effect::Custom("Choose Temur or Mardu.".into())],
                TargetSpec::None),
            Ability::static_ability(id,
                "Temur — Creatures you control get +1/+0 and have trample.",
                vec![StaticEffect::boost_controlled("creatures you control", 1, 0),
                     StaticEffect::grant_keyword_controlled("creatures you control", "trample")]),
            Ability::triggered(id,
                "Mardu — Whenever a creature you control attacks alone, create a Treasure token.",
                vec![EventType::AttackerDeclared],
                vec![Effect::create_token("Treasure", 1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn host_of_the_hereafter(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/5 Spirit Dragon for {3}{W}{B}. Flying. Dies: create X 1/1 Spirit tokens (X = power).
    CardData { id, owner, name: "Host of the Hereafter".into(), mana_cost: ManaCost::parse("{3}{W}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Spirit, SubType::Dragon],
        power: Some(4), toughness: Some(5), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::dies_triggered(id,
                "When Host of the Hereafter dies, create X 1/1 white Spirit creature tokens with flying, where X is its power.",
                vec![Effect::create_token("1/1 Spirit with flying", 4)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn hundred_battle_veteran(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/3 Human Warrior for {1}{R}{W}. First strike. Whenever attacks: put +1/+1 counter on another target attacking creature.
    CardData { id, owner, name: "Hundred-Battle Veteran".into(), mana_cost: ManaCost::parse("{1}{R}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Warrior],
        power: Some(3), toughness: Some(3), keywords: KeywordAbilities::FIRST_STRIKE,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                "Whenever Hundred-Battle Veteran attacks, put a +1/+1 counter on another target attacking creature you control.",
                vec![EventType::AttackerDeclared],
                vec![Effect::add_p1p1_counters(1)],
                TargetSpec::PermanentFiltered("another attacking creature you control".into())),
        ],
        ..Default::default() }
}

fn karakyk_guardian(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/5 Spirit Dragon for {2}{W}. Flying, defender. {3}{W}: can attack this turn as though it didn't have defender.
    CardData { id, owner, name: "Karakyk Guardian".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Spirit, SubType::Dragon],
        power: Some(2), toughness: Some(5),
        keywords: KeywordAbilities::FLYING | KeywordAbilities::DEFENDER,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{3}{W}: Karakyk Guardian can attack this turn as though it didn't have defender.",
                vec![Cost::pay_mana("{3}{W}")],
                vec![Effect::Custom("Can attack this turn as though it didn't have defender.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ── Tier 2+ new batch — card implementations ────────────────────────────────

fn arashin_sunshield(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/4 Human Warrior for {3}{W}. ETB: exile up to 2 cards from a graveyard.
    // {W}, {T}: Tap target creature.
    CardData { id, owner, name: "Arashin Sunshield".into(), mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Warrior],
        power: Some(3), toughness: Some(4), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, exile up to two target cards from a single graveyard.",
                vec![Effect::Exile],
                TargetSpec::Multiple { spec: Box::new(TargetSpec::CardInGraveyard), count: 2 }),
            Ability::activated(id,
                "{W}, {T}: Tap target creature.",
                vec![Cost::Mana(Mana::white(1)), Cost::TapSelf],
                vec![Effect::tap_target()],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn champion_of_dusan(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/2 Human Warrior for {2}{G}. Trample. Renew {1}{G}.
    CardData { id, owner, name: "Champion of Dusan".into(), mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Warrior],
        power: Some(4), toughness: Some(2),
        keywords: KeywordAbilities::TRAMPLE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "Renew -- {1}{G}, Exile this card from your graveyard: Put a +1/+1 counter and a trample counter on target creature. Activate only as a sorcery.",
                vec![Cost::pay_mana("{1}{G}"), Cost::ExileFromGraveyard(1)],
                vec![Effect::add_p1p1_counters(1), Effect::add_counters("trample", 1)],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn constrictor_sage(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/4 Snake Wizard for {4}{U}. ETB: tap + stun counter on opponent's creature.
    // Renew {2}{U}: same from graveyard.
    CardData { id, owner, name: "Constrictor Sage".into(), mana_cost: ManaCost::parse("{4}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Snake, SubType::Wizard],
        power: Some(4), toughness: Some(4), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, tap target creature an opponent controls and put a stun counter on it.",
                vec![Effect::tap_target(), Effect::add_counters("stun", 1)],
                TargetSpec::PermanentFiltered("creature an opponent controls".into())),
            Ability::activated(id,
                "Renew -- {2}{U}, Exile this card from your graveyard: Tap target creature an opponent controls and put a stun counter on it. Activate only as a sorcery.",
                vec![Cost::pay_mana("{2}{U}"), Cost::ExileFromGraveyard(1)],
                vec![Effect::tap_target(), Effect::add_counters("stun", 1)],
                TargetSpec::PermanentFiltered("creature an opponent controls".into())),
        ],
        ..Default::default() }
}

fn corroding_dragonstorm(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment {1}{B}. ETB: opponents lose 2, you gain 2, surveil 2.
    // Dragon ETB: return to hand.
    CardData { id, owner, name: "Corroding Dragonstorm".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Enchantment], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this enchantment enters, each opponent loses 2 life and you gain 2 life. Surveil 2.",
                vec![Effect::damage_opponents(2), Effect::gain_life(2), Effect::scry(2)],
                TargetSpec::None),
            Ability::triggered(id,
                "When a Dragon you control enters, return this enchantment to its owner's hand.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::bounce()],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn delta_bloodflies(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/2 Insect for {1}{B}. Flying. Attacks: if you control creature with counter, opponents lose 1.
    CardData { id, owner, name: "Delta Bloodflies".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Insect],
        power: Some(1), toughness: Some(2),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::attacks_triggered(id,
                "Whenever this creature attacks, if you control a creature with a counter on it, each opponent loses 1 life.",
                vec![Effect::damage_opponents(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn dragonstorm_forecaster(id: ObjectId, owner: PlayerId) -> CardData {
    // 0/3 Human Scout for {U}. {2}, {T}: search for Dragonstorm Globe or Boulderborn Dragon.
    CardData { id, owner, name: "Dragonstorm Forecaster".into(), mana_cost: ManaCost::parse("{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Scout],
        power: Some(0), toughness: Some(3), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                "{2}, {T}: Search your library for a card named Dragonstorm Globe or Boulderborn Dragon, reveal it, put it into your hand, then shuffle.",
                vec![Cost::Mana(Mana { generic: 2, ..Default::default() }), Cost::TapSelf],
                vec![Effect::search_library("Dragonstorm Globe or Boulderborn Dragon")],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn focus_the_mind(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {4}{U}. Costs {2} less if you cast another spell this turn. Draw 3, discard 1.
    CardData { id, owner, name: "Focus the Mind".into(), mana_cost: ManaCost::parse("{4}{U}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "This spell costs {2} less to cast if you've cast another spell this turn.",
                vec![StaticEffect::CostReduction { filter: "self".into(), amount: 2 }]),
            Ability::spell(id,
                vec![Effect::draw_cards(3), Effect::discard_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn hardened_tactician(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/4 Human Warrior for {1}{W}{B}. {1}, Sacrifice a token: Draw a card.
    CardData { id, owner, name: "Hardened Tactician".into(), mana_cost: ManaCost::parse("{1}{W}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Warrior],
        power: Some(2), toughness: Some(4), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                "{1}, Sacrifice a token: Draw a card.",
                vec![Cost::Mana(Mana { generic: 1, ..Default::default() }), Cost::SacrificeOther("a token".into())],
                vec![Effect::draw_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn jeskai_devotee(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Human Monk for {1}{R}. Flurry: +1/+0 until EOT. Mana ability once per turn.
    CardData { id, owner, name: "Jeskai Devotee".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Monk],
        power: Some(2), toughness: Some(2), rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Flurry -- Whenever you cast your second spell each turn, this creature gets +1/+0 until end of turn.",
                vec![EventType::SpellCast],
                vec![Effect::boost_until_eot(1, 0)],
                TargetSpec::None),
            Ability::activated(id,
                "{1}: Add one mana of any color. Activate only once each turn.",
                vec![Cost::Mana(Mana { generic: 1, ..Default::default() })],
                vec![Effect::add_mana(Mana { any: 1, ..Default::default() })],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn monastery_messenger(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/3 Bird Scout for {2/U}{2/R}{2/W}. Flying, vigilance.
    // ETB: put up to one noncreature nonland card from graveyard on top of library.
    CardData { id, owner, name: "Monastery Messenger".into(), mana_cost: ManaCost::parse("{2/U}{2/R}{2/W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Bird, SubType::Scout],
        power: Some(2), toughness: Some(3),
        keywords: KeywordAbilities::FLYING | KeywordAbilities::VIGILANCE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, put up to one target noncreature, nonland card from your graveyard on top of your library.",
                vec![Effect::Custom("Put target noncreature, nonland card from your graveyard on top of your library.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn osseous_exhale(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{W}. Behold Dragon. 5 damage to attacking/blocking creature. If beheld, gain 2 life.
    CardData { id, owner, name: "Osseous Exhale".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::deal_damage(5), Effect::Custom("If a Dragon was beheld, you gain 2 life.".into())],
                TargetSpec::PermanentFiltered("attacking or blocking creature".into())),
        ],
        ..Default::default() }
}

fn rite_of_renewal(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {3}{G}. Return up to 2 permanent cards from graveyard to hand.
    // Target player shuffles up to 4 cards from graveyard into library. Exile this.
    CardData { id, owner, name: "Rite of Renewal".into(), mana_cost: ManaCost::parse("{3}{G}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::return_from_graveyard(),
                     Effect::Custom("Target player shuffles up to four target cards from their graveyard into their library. Exile Rite of Renewal.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn snowmelt_stag(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/5 Elemental Elk for {3}{U}. Vigilance. Your turn: base P/T is 5/2.
    // {5}{U}{U}: Can't be blocked this turn.
    CardData { id, owner, name: "Snowmelt Stag".into(), mana_cost: ManaCost::parse("{3}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elemental, SubType::Elk],
        power: Some(2), toughness: Some(5),
        keywords: KeywordAbilities::VIGILANCE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "During your turn, this creature has base power and toughness 5/2.",
                vec![StaticEffect::Custom("During your turn, base P/T is 5/2.".into())]),
            Ability::activated(id,
                "{5}{U}{U}: This creature can't be blocked this turn.",
                vec![Cost::pay_mana("{5}{U}{U}")],
                vec![Effect::Custom("This creature can't be blocked this turn.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn stormbeacon_blade(id: ObjectId, owner: PlayerId) -> CardData {
    // Artifact — Equipment {1}{W}. Equipped creature gets +3/+0.
    // Attacks: draw a card if 3+ attacking creatures. Equip {2}.
    CardData { id, owner, name: "Stormbeacon Blade".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Artifact], subtypes: vec![SubType::Equipment],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                "Equipped creature gets +3/+0.",
                vec![StaticEffect::Boost { filter: "equipped creature".into(), power: 3, toughness: 0 }]),
            Ability::triggered(id,
                "Whenever equipped creature attacks, draw a card if you control three or more attacking creatures.",
                vec![EventType::DeclareAttackers],
                vec![Effect::draw_cards(1)],
                TargetSpec::None),
            Ability::activated(id,
                "Equip {2}",
                vec![Cost::pay_mana("{2}")],
                vec![Effect::equip()],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn sunset_strikemaster(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/1 Human Monk for {1}{R}. {T}: Add {R}.
    // {2}{R}, {T}, Sacrifice: 6 damage to creature with flying.
    CardData { id, owner, name: "Sunset Strikemaster".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Monk],
        power: Some(3), toughness: Some(1), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::mana_ability(id, "{T}: Add {R}.", Mana::red(1)),
            Ability::activated(id,
                "{2}{R}, {T}, Sacrifice this creature: It deals 6 damage to target creature with flying.",
                vec![Cost::pay_mana("{2}{R}"), Cost::TapSelf, Cost::SacrificeSelf],
                vec![Effect::deal_damage(6)],
                TargetSpec::PermanentFiltered("creature with flying".into())),
        ],
        ..Default::default() }
}

fn tempest_hawk(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Bird for {2}{W}. Flying. Combat damage: search for Tempest Hawk.
    // A deck can have any number.
    CardData { id, owner, name: "Tempest Hawk".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Bird],
        power: Some(2), toughness: Some(2),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever this creature deals combat damage to a player, you may search your library for a card named Tempest Hawk, reveal it, put it into your hand, then shuffle.",
                vec![EventType::DamagedPlayer],
                vec![Effect::search_library("Tempest Hawk")],
                TargetSpec::None),
            Ability::static_ability(id,
                "A deck can have any number of cards named Tempest Hawk.",
                vec![StaticEffect::Custom("A deck can have any number of cards named Tempest Hawk.".into())]),
        ],
        ..Default::default() }
}

fn trade_route_envoy(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/3 Dog Soldier for {3}{G}. ETB: draw if you control creature with counter, else +1/+1 counter.
    CardData { id, owner, name: "Trade Route Envoy".into(), mana_cost: ManaCost::parse("{3}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Dog, SubType::Soldier],
        power: Some(4), toughness: Some(3), rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, draw a card if you control a creature with a counter on it. If you don't, put a +1/+1 counter on this creature.",
                vec![Effect::Custom("Draw a card if you control a creature with a counter on it. If you don't, put a +1/+1 counter on this creature.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn traveling_botanist(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/3 Dog Scout for {1}{G}. Becomes tapped: look at top, land to hand, or to graveyard.
    CardData { id, owner, name: "Traveling Botanist".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Dog, SubType::Scout],
        power: Some(2), toughness: Some(3), rarity: Rarity::Rare,
        abilities: vec![
            Ability::triggered(id,
                "Whenever this creature becomes tapped, look at the top card of your library. If it's a land card, you may reveal it and put it into your hand. If you don't put the card into your hand, you may put it into your graveyard.",
                vec![EventType::Tapped],
                vec![Effect::Custom("Look at top card; land to hand or card to graveyard.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn twinmaw_stormbrood(id: ObjectId, owner: PlayerId) -> CardData {
    // 5/4 Dragon for {5}{W}. Flying. ETB: gain 5 life.
    CardData { id, owner, name: "Twinmaw Stormbrood".into(), mana_cost: ManaCost::parse("{5}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Dragon],
        power: Some(5), toughness: Some(4),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, you gain 5 life.",
                vec![Effect::gain_life(5)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn venerated_stormsinger(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/3 Orc Cleric for {3}{B}. Mobilize 1.
    // This or another creature you control dies: opponents lose 1, gain 1.
    CardData { id, owner, name: "Venerated Stormsinger".into(), mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Orc, SubType::Cleric],
        power: Some(3), toughness: Some(3), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                "Mobilize 1 -- When this creature enters, create a 1/1 white Soldier creature token.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::create_token("1/1 Soldier", 1)],
                TargetSpec::None),
            Ability::triggered(id,
                "Whenever this creature or another creature you control dies, each opponent loses 1 life and you gain 1 life.",
                vec![EventType::Dies],
                vec![Effect::damage_opponents(1), Effect::gain_life(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn wayspeaker_bodyguard(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/4 Orc Monk for {3}{W}. ETB: return nonland permanent card MV<=2 from graveyard to hand.
    // Flurry: tap creature opponent controls.
    CardData { id, owner, name: "Wayspeaker Bodyguard".into(), mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Orc, SubType::Monk],
        power: Some(3), toughness: Some(4), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, return target nonland permanent card with mana value 2 or less from your graveyard to your hand.",
                vec![Effect::return_from_graveyard()],
                TargetSpec::None),
            Ability::triggered(id,
                "Flurry -- Whenever you cast your second spell each turn, tap target creature an opponent controls.",
                vec![EventType::SpellCast],
                vec![Effect::tap_target()],
                TargetSpec::PermanentFiltered("creature an opponent controls".into())),
        ],
        ..Default::default() }
}

fn wingspan_stride(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment — Aura {U}. Enchanted creature +1/+1 and flying. {2}{U}: Return to hand.
    CardData { id, owner, name: "Wingspan Stride".into(), mana_cost: ManaCost::parse("{U}"),
        card_types: vec![CardType::Enchantment], subtypes: vec![SubType::Aura],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "Enchanted creature gets +1/+1 and has flying.",
                vec![StaticEffect::Boost { filter: "enchanted creature".into(), power: 1, toughness: 1 },
                     StaticEffect::GrantKeyword { filter: "enchanted creature".into(), keyword: "flying".into() }]),
            Ability::activated(id,
                "{2}{U}: Return this Aura to its owner's hand.",
                vec![Cost::pay_mana("{2}{U}")],
                vec![Effect::bounce()],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn yathan_tombguard(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/3 Human Warrior for {2}{B}. Menace.
    // Creature you control with counter deals combat damage: draw, lose 1.
    CardData { id, owner, name: "Yathan Tombguard".into(), mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Warrior],
        power: Some(2), toughness: Some(3),
        keywords: KeywordAbilities::MENACE,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                "Whenever a creature you control with a counter on it deals combat damage to a player, you draw a card and you lose 1 life.",
                vec![EventType::DamagedPlayer],
                vec![Effect::draw_cards(1), Effect::lose_life(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}
// cori_steel_cutter and eshki_dragonclaw defined earlier with full abilities

fn kheru_goldkeeper(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Kheru Goldkeeper".into(),
        mana_cost: ManaCost::parse("{1}{B}{G}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::triggered(id,
                "Whenever one or more cards leave your graveyard during your turn, create a Treasure token.",
                vec![EventType::ZoneChange],
                vec![Effect::CreateToken { token_name: "Treasure".into(), count: 1 }],
                TargetSpec::None),
            Ability::activated(id,
                "Renew -- {2}{B}{G}{U}, Exile this card from your graveyard: Put two +1/+1 counters and a flying counter on target creature. Activate only as a sorcery.",
                vec![Cost::pay_mana("{2}{B}{G}{U}"), Cost::ExileFromGraveyard(1)],
                vec![Effect::AddCounters { counter_type: "+1/+1".into(), count: 2 },
                     Effect::AddCounters { counter_type: "flying".into(), count: 1 }],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn kishla_village(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Kishla Village".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::mana_ability(id, "{T}: Add {C}.", Mana::colorless(1)),
            Ability::activated(id,
                    "{3}{G}, {T}: Create a 4/4 green Beast creature token. Activate only if you control five or more lands.",
                    vec![Cost::pay_mana("{3}{G}"), Cost::TapSelf],
                    vec![Effect::CreateToken { token_name: "4/4 green Beast".into(), count: 1 }],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn kotis_the_fangkeeper(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Kotis, the Fangkeeper".into(),
        mana_cost: ManaCost::parse("{1}{B}{G}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Zombie, SubType::Warrior],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(1),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::INDESTRUCTIBLE,
        abilities: vec![
            Ability::combat_damage_to_player_triggered(id,
                "Whenever Kotis deals combat damage to a player, exile the top X cards of their library, where X is the amount of damage dealt. You may cast spells with mana value X or less from among them without paying their mana costs.",
                vec![Effect::Custom("Exile top X cards of damaged player's library. Cast spells with MV <= X for free.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn lie_in_wait(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Lie in Wait".into(),
        mana_cost: ManaCost::parse("{B}{G}{U}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::Mill { count: 4 },
                         Effect::Custom("You may put a creature card from among the milled cards onto the battlefield.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn lotuslight_dancers(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Lotuslight Dancers".into(),
        mana_cost: ManaCost::parse("{2}{B}{G}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Zombie, SubType::Custom("Bard".into())],
        power: Some(3), toughness: Some(6),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::LIFELINK,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this creature enters, each opponent mills four cards. You may put a creature card from among all cards milled this way onto the battlefield under your control.",
                    vec![Effect::Custom("Each opponent mills four cards. You may put a creature card from among milled cards onto the battlefield under your control.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn maelstrom_of_the_spirit_dragon(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Maelstrom of the Spirit Dragon".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::mana_ability(id, "{T}: Add {C}.", Mana::colorless(1)),
            Ability::activated(id,
                    "{4}, {T}: Target Dragon you control deals damage equal to its power to any target.",
                    vec![Cost::pay_mana("{4}"), Cost::TapSelf],
                    vec![Effect::Custom("Target Dragon you control deals damage equal to its power to any target.".into())],
                    TargetSpec::Custom("target Dragon you control, any target".into())),
        ],
        ..Default::default() }
}

fn magmatic_hellkite(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Magmatic Hellkite".into(),
        mana_cost: ManaCost::parse("{2}{R}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon],
        power: Some(4), toughness: Some(5),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this creature enters, destroy target nonbasic land an opponent controls. Its controller searches their library for a basic land card, puts it onto the battlefield tapped with a stun counter on it, then shuffles.",
                    vec![Effect::Destroy, Effect::Custom("Controller searches for a basic land, puts it onto the battlefield tapped with a stun counter, then shuffles.".into())],
                    TargetSpec::PermanentFiltered("nonbasic land an opponent controls".into())),
        ],
        ..Default::default() }
}

fn mardu_monument(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Mardu Monument".into(),
        mana_cost: ManaCost::parse("{2}"),
        card_types: vec![CardType::Artifact],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this artifact enters, search your library for a basic Mountain, Plains, or Swamp card, reveal it, put it into your hand, then shuffle.",
                    vec![Effect::SearchLibrary { filter: "basic Mountain, Plains, or Swamp card".into() }],
                    TargetSpec::None),
            Ability::activated(id,
                    "{2}{R}{W}{B}, {T}, Sacrifice this artifact: Create three 1/1 red Warrior creature tokens. They gain menace and haste until end of turn. Activate only as a sorcery.",
                    vec![Cost::pay_mana("{2}{R}{W}{B}"), Cost::TapSelf, Cost::SacrificeSelf],
                    vec![Effect::CreateToken { token_name: "1/1 red Warrior".into(), count: 3 },
                         Effect::GainKeywordUntilEndOfTurn { keyword: "menace".into() },
                         Effect::GainKeywordUntilEndOfTurn { keyword: "haste".into() }],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn mardu_siegebreaker(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Mardu Siegebreaker".into(),
        mana_cost: ManaCost::parse("{1}{R}{W}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::DEATHTOUCH | KeywordAbilities::HASTE,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this creature enters, destroy target permanent an opponent controls with mana value 2 or less.",
                    vec![Effect::Destroy],
                    TargetSpec::PermanentFiltered("permanent an opponent controls with mana value 2 or less".into())),
            Ability::attacks_triggered(id,
                    "Whenever this creature attacks, create a tapped and attacking token that's a copy of it, except it's 1/1. Sacrifice the token at end of combat.",
                    vec![Effect::Custom("Create a tapped and attacking token copy of this creature (1/1). Sacrifice at end of combat.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn mistrise_village(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Mistrise Village".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::mana_ability(id, "{T}: Add {C}.", Mana::colorless(1)),
            Ability::activated(id,
                    "{U}, {T}: Scry 1.",
                    vec![Cost::pay_mana("{U}"), Cost::TapSelf],
                    vec![Effect::Scry { count: 1 }],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn naga_fleshcrafter(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Naga Fleshcrafter".into(),
        mana_cost: ManaCost::parse("{3}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Snake, SubType::Shapeshifter],
        supertypes: vec![SuperType::Legendary],
        power: Some(0), toughness: Some(0),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "You may have this creature enter as a copy of any creature on the battlefield.",
                vec![Effect::Custom("Enter as a copy of any creature on the battlefield.".into())],
                TargetSpec::None),
            Ability::activated(id,
                "Renew -- {2}{U}, Exile this card from your graveyard: Put a +1/+1 counter on target nonlegendary creature you control. Each other creature you control becomes a copy of that creature until end of turn. Activate only as a sorcery.",
                vec![Cost::pay_mana("{2}{U}"), Cost::ExileFromGraveyard(1)],
                vec![Effect::add_p1p1_counters(1), Effect::Custom("Each other creature you control becomes a copy of target creature until end of turn.".into())],
                TargetSpec::PermanentFiltered("nonlegendary creature you control".into())),
        ],
        ..Default::default() }
}

fn narset_jeskai_waymaster(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Narset, Jeskai Waymaster".into(),
        mana_cost: ManaCost::parse("{U}{R}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Monk],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(4),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::triggered(id,
                "At the beginning of your end step, you may discard your hand. If you do, draw cards equal to the number of spells you've cast this turn.",
                vec![EventType::EndStep],
                vec![Effect::Custom("You may discard your hand. If you do, draw cards equal to the number of spells you've cast this turn.".into())],
                TargetSpec::None).set_optional(),
        ],
        ..Default::default() }
}

fn neriv_heart_of_the_storm(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Neriv, Heart of the Storm".into(),
        mana_cost: ManaCost::parse("{1}{R}{W}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Spirit, SubType::Dragon],
        supertypes: vec![SuperType::Legendary],
        power: Some(4), toughness: Some(5),
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::static_ability(id,
                    "Creature tokens you control get +1/+1 and have haste.",
                    vec![StaticEffect::Boost { filter: "creature token you control".into(), power: 1, toughness: 1 },
                         StaticEffect::GrantKeyword { filter: "creature token you control".into(), keyword: "haste".into() }]),
            Ability::attacks_triggered(id,
                    "Whenever Neriv attacks, create a tapped and attacking token that's a copy of another target creature you control, except it's 1/1.",
                    vec![Effect::Custom("Create a tapped and attacking token copy of another target creature you control (1/1).".into())],
                    TargetSpec::PermanentFiltered("another creature you control".into())),
        ],
        ..Default::default() }
}

fn purging_stormbrood(id: ObjectId, owner: PlayerId) -> CardData {
    // Omen card: creature front + "Absorb Essence" instant back
    CardData { id, owner, name: "Purging Stormbrood".into(),
        mana_cost: ManaCost::parse("{4}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this creature enters, remove all counters from up to one target creature.",
                    vec![Effect::RemoveCounters { counter_type: "all".into(), count: 0 }],
                    TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn rally_the_monastery(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Rally the Monastery".into(),
        mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                    "This spell costs {2} less to cast if you've cast another spell this turn.",
                    vec![StaticEffect::CostReduction { filter: "self".into(), amount: 2 }]),
            Ability::spell(id,
                    vec![Effect::Custom("Choose one: Create two 1/1 white Monk creature tokens with prowess; or up to two target creatures you control each get +2/+2 until end of turn; or destroy target creature with power 4 or greater.".into())],
                    TargetSpec::Custom("mode-dependent".into())),
        ],
        ..Default::default() }
}

fn riling_dawnbreaker(id: ObjectId, owner: PlayerId) -> CardData {
    // Omen card: creature front + "Signaling Roar" sorcery back
    CardData { id, owner, name: "Riling Dawnbreaker".into(),
        mana_cost: ManaCost::parse("{4}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon],
        power: Some(3), toughness: Some(4),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::VIGILANCE,
        abilities: vec![
            Ability::triggered(id,
                    "At the beginning of combat on your turn, another target creature you control gets +1/+0 until end of turn.",
                    vec![EventType::BeginCombat],
                    vec![Effect::BoostUntilEndOfTurn { power: 1, toughness: 0 }],
                    TargetSpec::PermanentFiltered("another creature you control".into())),
        ],
        ..Default::default() }
}

fn ringing_strike_mastery(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ringing Strike Mastery".into(),
        mana_cost: ManaCost::parse("{U}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Aura],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                    "Enchanted creature gets +1/+1 and has ward {1}.",
                    vec![StaticEffect::Boost { filter: "enchanted creature".into(), power: 1, toughness: 1 }]),
            Ability::activated(id,
                    "{5}: Return this Aura to its owner's hand.",
                    vec![Cost::pay_mana("{5}")],
                    vec![Effect::Bounce],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn riverwheel_sweep(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Riverwheel Sweep".into(),
        mana_cost: ManaCost::parse("{2/U}{2/R}{2/W}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::DealDamage { amount: 4 }],
                    TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn rot_curse_rakshasa(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Rot-Curse Rakshasa".into(),
        mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Demon],
        power: Some(5), toughness: Some(5),
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::TRAMPLE,
        abilities: vec![
            Ability::static_ability(id,
                "Decayed (This creature can't block. When it attacks, sacrifice it at end of combat.)",
                vec![StaticEffect::Custom("Decayed".into())]),
            Ability::activated(id,
                "Renew -- {X}{B}{B}, Exile this card from your graveyard: Put a decayed counter on each of X target creatures. Activate only as a sorcery.",
                vec![Cost::pay_mana("{X}{B}{B}"), Cost::ExileFromGraveyard(1)],
                vec![Effect::AddCounters { counter_type: "decayed".into(), count: 1 }],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn severance_priest(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Severance Priest".into(),
        mana_cost: ManaCost::parse("{W}{B}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Djinn, SubType::Cleric],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::DEATHTOUCH,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this creature enters, target opponent reveals their hand. You may choose a nonland card from it and exile that card.",
                    vec![Effect::Custom("Target opponent reveals their hand. You may choose a nonland card from it and exile it.".into())],
                    TargetSpec::Player),
            Ability::triggered(id,
                    "When this creature leaves the battlefield, the exiled card's owner creates an X/X white Spirit creature token, where X is the mana value of the exiled card.",
                    vec![EventType::ZoneChange],
                    vec![Effect::Custom("Exiled card's owner creates an X/X white Spirit token (X = exiled card's mana value).".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn shiko_paragon_of_the_way(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Shiko, Paragon of the Way".into(),
        mana_cost: ManaCost::parse("{2}{U}{R}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Spirit, SubType::Dragon],
        supertypes: vec![SuperType::Legendary],
        power: Some(4), toughness: Some(5),
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::VIGILANCE,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this creature enters, draw cards equal to the number of noncreature, nonland permanents you control.",
                    vec![Effect::Custom("Draw cards equal to the number of noncreature, nonland permanents you control.".into())],
                    TargetSpec::None),
            Ability::static_ability(id,
                    "Noncreature spells you cast cost {1} less to cast.",
                    vec![StaticEffect::CostReduction { filter: "noncreature spell you cast".into(), amount: 1 }]),
        ],
        ..Default::default() }
}

fn sidisi_regent_of_the_mire(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sidisi, Regent of the Mire".into(),
        mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Zombie, SubType::Snake, SubType::Warlock],
        supertypes: vec![SuperType::Legendary],
        power: Some(1), toughness: Some(3),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::activated(id,
                "{T}, Sacrifice a creature you control with mana value X other than Sidisi: Return target creature card with mana value X plus 1 from your graveyard to the battlefield. Activate only as a sorcery.",
                vec![Cost::TapSelf, Cost::Custom("Sacrifice a creature with mana value X other than Sidisi".into())],
                vec![Effect::Reanimate],
                TargetSpec::CardInYourGraveyard),
        ],
        ..Default::default() }
}

fn smile_at_death(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Smile at Death".into(),
        mana_cost: ManaCost::parse("{3}{W}{W}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Mythic,
        abilities: vec![
            Ability::beginning_of_upkeep_triggered(id,
                "At the beginning of your upkeep, return up to two target creature cards with power 2 or less from your graveyard to the battlefield. Put a +1/+1 counter on each of those creatures.",
                vec![Effect::Reanimate, Effect::AddCounters { counter_type: "+1/+1".into(), count: 1 }],
                TargetSpec::Multiple { spec: Box::new(TargetSpec::CardInYourGraveyard), count: 2 }),
        ],
        ..Default::default() }
}

fn sonic_shrieker(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sonic Shrieker".into(),
        mana_cost: ManaCost::parse("{2}{R}{W}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this creature enters, it deals 2 damage to any target and you gain 2 life. If a player is dealt damage this way, they discard a card.",
                    vec![Effect::DealDamage { amount: 2 }, Effect::GainLife { amount: 2 },
                         Effect::Custom("If a player is dealt damage this way, they discard a card.".into())],
                    TargetSpec::CreatureOrPlayer),
        ],
        ..Default::default() }
}

fn stalwart_successor(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Stalwart Successor".into(),
        mana_cost: ManaCost::parse("{1}{B}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior],
        power: Some(3), toughness: Some(2),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::MENACE,
        abilities: vec![
            Ability::triggered(id,
                "Whenever one or more counters are put on a creature you control, if it's the first time counters have been put on that creature this turn, put a +1/+1 counter on that creature.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::add_p1p1_counters(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn starry_eyed_skyrider(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Starry-Eyed Skyrider".into(),
        mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Scout],
        power: Some(1), toughness: Some(3),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::attacks_triggered(id,
                    "Whenever this creature attacks, another target creature you control gains flying until end of turn.",
                    vec![Effect::GainKeywordUntilEndOfTurn { keyword: "flying".into() }],
                    TargetSpec::PermanentFiltered("another creature you control".into())),
            Ability::static_ability(id,
                    "Attacking tokens you control have flying.",
                    vec![StaticEffect::GrantKeyword { filter: "attacking token you control".into(), keyword: "flying".into() }]),
        ],
        ..Default::default() }
}

fn static_snare(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Static Snare".into(),
        mana_cost: ManaCost::parse("{4}{W}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLASH,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enchantment enters, exile target artifact or creature an opponent controls until this enchantment leaves the battlefield.",
                    vec![Effect::Exile],
                    TargetSpec::PermanentFiltered("artifact or creature an opponent controls".into())),
            Ability::static_ability(id,
                    "This spell costs {1} less to cast for each attacking creature.",
                    vec![StaticEffect::CostReduction { filter: "self".into(), amount: 1 }]),
        ],
        ..Default::default() }
}

fn stormscale_scion(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Stormscale Scion".into(),
        mana_cost: ManaCost::parse("{4}{R}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::STORM,
        abilities: vec![
            Ability::static_ability(id,
                    "Other Dragons you control get +1/+1.",
                    vec![StaticEffect::Boost { filter: "other Dragon you control".into(), power: 1, toughness: 1 }]),
        ],
        ..Default::default() }
}

fn stormshriek_feral(id: ObjectId, owner: PlayerId) -> CardData {
    // Omen card: creature front + "Flush Out" sorcery back
    CardData { id, owner, name: "Stormshriek Feral".into(),
        mana_cost: ManaCost::parse("{4}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::HASTE,
        abilities: vec![
            Ability::activated(id,
                    "{1}{R}: This creature gets +1/+0 until end of turn.",
                    vec![Cost::pay_mana("{1}{R}")],
                    vec![Effect::BoostUntilEndOfTurn { power: 1, toughness: 0 }],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn sunpearl_kirin(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sunpearl Kirin".into(),
        mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Custom("Kirin".into())],
        power: Some(2), toughness: Some(1),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLASH | KeywordAbilities::FLYING,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this creature enters, return up to one other target nonland permanent you control to its owner's hand. If it was a token, draw a card.",
                    vec![Effect::Bounce, Effect::Custom("If it was a token, draw a card.".into())],
                    TargetSpec::PermanentFiltered("other nonland permanent you control".into())),
        ],
        ..Default::default() }
}

fn surrak_elusive_hunter(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Surrak, Elusive Hunter".into(),
        mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior],
        supertypes: vec![SuperType::Legendary],
        power: Some(4), toughness: Some(3),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::TRAMPLE,
        abilities: vec![
            Ability::static_ability(id,
                "This spell can't be countered.",
                vec![StaticEffect::Custom("This spell can't be countered.".into())]),
            Ability::triggered(id,
                "Whenever a creature you control or a creature spell you control becomes the target of a spell or ability an opponent controls, draw a card.",
                vec![EventType::SpellCast],
                vec![Effect::draw_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn synchronized_charge(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Synchronized Charge".into(),
        mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::Custom("Distribute two +1/+1 counters among one or two target creatures you control.".into()),
                         Effect::Custom("Creatures you control with counters on them gain vigilance and trample until end of turn.".into())],
                    TargetSpec::Custom("one or two target creatures you control".into())),
        ],
        ..Default::default() }
}

fn temur_battlecrier(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Temur Battlecrier".into(),
        mana_cost: ManaCost::parse("{G}{U}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Custom("Orc".into()), SubType::Ranger],
        power: Some(4), toughness: Some(3),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::TRAMPLE,
        abilities: vec![
            Ability::static_ability(id,
                    "Other creatures you control have trample.",
                    vec![StaticEffect::GrantKeyword { filter: "other creature you control".into(), keyword: "trample".into() }]),
            Ability::attacks_triggered(id,
                    "Whenever this creature attacks, you may draw a card. If you do, discard a card.",
                    vec![Effect::DrawCards { count: 1 }, Effect::DiscardCards { count: 1 }],
                    TargetSpec::None).set_optional(),
        ],
        ..Default::default() }
}

fn tersa_lightshatter(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Tersa Lightshatter".into(),
        mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Custom("Orc".into()), SubType::Wizard],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::HASTE,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Tersa Lightshatter enters, discard up to two cards, then draw that many cards.",
                    vec![Effect::Custom("Discard up to two cards, then draw that many cards.".into())],
                    TargetSpec::None),
            Ability::attacks_triggered(id,
                    "Whenever Tersa attacks, if there are seven or more cards in your graveyard, exile a card at random from your graveyard. You may play that card this turn.",
                    vec![Effect::Custom("Threshold -- Exile a random card from your graveyard. You may play it this turn.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn teval_arbiter_of_virtue(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Teval, Arbiter of Virtue".into(),
        mana_cost: ManaCost::parse("{2}{B}{G}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Spirit, SubType::Dragon],
        supertypes: vec![SuperType::Legendary],
        power: Some(6), toughness: Some(6),
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::LIFELINK,
        abilities: vec![
            Ability::static_ability(id,
                    "Spells you cast have delve.",
                    vec![StaticEffect::Custom("Spells you cast have delve.".into())]),
            Ability::spell_cast_triggered(id,
                    "Whenever you cast a spell, you lose life equal to its mana value.",
                    vec![Effect::Custom("You lose life equal to the cast spell's mana value.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn ureni_the_song_unending(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ureni, the Song Unending".into(),
        mana_cost: ManaCost::parse("{5}{G}{U}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Spirit, SubType::Dragon],
        supertypes: vec![SuperType::Legendary],
        power: Some(10), toughness: Some(10),
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::PROTECTION,
        abilities: vec![
            Ability::static_ability(id,
                    "Protection from white and from black.",
                    vec![StaticEffect::Custom("Protection from white and from black.".into())]),
            Ability::enters_battlefield_triggered(id,
                    "When this creature enters, it deals X damage divided as you choose among any number of target creatures and/or planeswalkers your opponents control, where X is the number of lands you control.",
                    vec![Effect::Custom("Deal damage equal to lands you control, divided among target creatures and/or planeswalkers opponents control.".into())],
                    TargetSpec::Custom("any number of target creatures and/or planeswalkers your opponents control".into())),
        ],
        ..Default::default() }
}

fn wail_of_war(id: ObjectId, owner: PlayerId) -> CardData {
    // Modal: -1/-1 to opponent's creatures OR return up to two creatures from your graveyard
    CardData { id, owner, name: "Wail of War".into(),
        mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::Custom("Choose one: Creatures target opponent controls get -1/-1 until end of turn; or return up to two creature cards from your graveyard to your hand.".into())],
                    TargetSpec::Custom("target opponent or up to two creature cards in your graveyard".into())),
        ],
        ..Default::default() }
}

fn war_effort(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "War Effort".into(),
        mana_cost: ManaCost::parse("{3}{R}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                    "Creatures you control get +1/+0.",
                    vec![StaticEffect::Boost { filter: "creature you control".into(), power: 1, toughness: 0 }]),
            Ability::triggered(id,
                    "Whenever you attack, create a 1/1 red Warrior creature token that's tapped and attacking. Sacrifice it at the beginning of the next end step.",
                    vec![EventType::DeclareAttackers],
                    vec![Effect::CreateTokenTappedAttacking { token_name: "1/1 red Warrior".into(), count: 1 }],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn warden_of_the_grove(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Warden of the Grove".into(),
        mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Hydra],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::triggered(id,
                "At the beginning of your end step, put a +1/+1 counter on this creature.",
                vec![EventType::EndStep],
                vec![Effect::add_p1p1_counters(1)],
                TargetSpec::None),
            Ability::triggered(id,
                "Whenever another nontoken creature you control enters, it endures X, where X is the number of counters on this creature.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("Target creature endures X, where X is the number of counters on this creature.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn whirlwing_stormbrood(id: ObjectId, owner: PlayerId) -> CardData {
    // Omen card: creature front + "Dynamic Soar" sorcery back
    CardData { id, owner, name: "Whirlwing Stormbrood".into(),
        mana_cost: ManaCost::parse("{4}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon],
        power: Some(4), toughness: Some(3),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLASH | KeywordAbilities::FLYING,
        abilities: vec![
            Ability::static_ability(id,
                    "You may cast sorcery spells and Dragon spells as though they had flash.",
                    vec![StaticEffect::Custom("You may cast sorcery spells and Dragon spells as though they had flash.".into())]),
        ],
        ..Default::default() }
}

fn windcrag_siege(id: ObjectId, owner: PlayerId) -> CardData {
    // Choose Jeskai or Mardu as it enters
    CardData { id, owner, name: "Windcrag Siege".into(),
        mana_cost: ManaCost::parse("{1}{R}{W}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                    "As this enchantment enters, choose Jeskai or Mardu. Jeskai: Creature tokens you control have lifelink and haste. Mardu: Whenever you attack, create a 1/1 red Warrior creature token that's tapped and attacking.",
                    vec![StaticEffect::Custom("Choose Jeskai or Mardu. Jeskai: Creature tokens you control have lifelink and haste. Mardu: Whenever you attack, create a 1/1 red Warrior creature token tapped and attacking.".into())]),
        ],
        ..Default::default() }
}

fn zurgo_thunders_decree(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Zurgo, Thunder's Decree".into(),
        mana_cost: ManaCost::parse("{R}{W}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Custom("Orc".into()), SubType::Warrior],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(4),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::HASTE,
        abilities: vec![
            Ability::attacks_triggered(id,
                    "Whenever Zurgo attacks, it deals 1 damage to each creature defending player controls.",
                    vec![Effect::DealDamageAll { amount: 1, filter: "creature defending player controls".into() }],
                    TargetSpec::None),
            Ability::static_ability(id,
                    "Whenever a creature dealt damage by Zurgo this turn dies, you gain 1 life and Zurgo gets +1/+0 until end of turn.",
                    vec![StaticEffect::Custom("Whenever a creature dealt damage by Zurgo this turn dies, you gain 1 life and Zurgo gets +1/+0 until end of turn.".into())]),
        ],
        ..Default::default() }
}



// ── New TDM card factory functions ─────────────────────────────────────

fn awaken_the_honored_dead(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Awaken the Honored Dead".into(), mana_cost: ManaCost::parse("{B}{G}{U}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Saga],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("(As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn bloomvine_regent(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Bloomvine Regent".into(),
        mana_cost: ManaCost::parse("{3}{G}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon],
        power: Some(4), toughness: Some(5),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever this creature or another Dragon you control enters, you gain 3 life.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::gain_life(3)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn clarion_conqueror(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Clarion Conqueror".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon],
        power: Some(3), toughness: Some(3),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "Activated abilities of artifacts, creatures, and planeswalkers can't be activated.",
                vec![StaticEffect::Custom("Activated abilities of artifacts, creatures, and planeswalkers can't be activated.".into())]),
        ],
        ..Default::default() }
}

fn dirgur_island_dragon(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Dirgur Island Dragon".into(),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon],
        power: Some(4), toughness: Some(4),
        keywords: KeywordAbilities::FLYING | KeywordAbilities::WARD,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id, "Ward {2}",
                vec![StaticEffect::ward("{2}")]),
        ],
        ..Default::default() }
}

fn disruptive_stormbrood(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Disruptive Stormbrood".into(),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon],
        power: Some(3), toughness: Some(3),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When this creature enters, destroy up to one target artifact or enchantment.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Destroy],
                TargetSpec::PermanentFiltered("artifact or enchantment".into())),
        ],
        ..Default::default() }
}

fn dragonbroods_relic(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Dragonbroods' Relic".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Artifact],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{T}, Tap an untapped creature you control: Add one mana of any color.",
                vec![Cost::TapSelf, Cost::Custom("Tap an untapped creature you control".into())],
                vec![Effect::AddMana { mana: Mana { any: 1, ..Default::default() } }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn elspeth_storm_slayer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Elspeth, Storm Slayer".into(), mana_cost: ManaCost::parse("{3}{W}{W}"),
        card_types: vec![CardType::Planeswalker],
        subtypes: vec![SubType::Custom("Elspeth".into())],
        supertypes: vec![SuperType::Legendary],
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                "If one or more tokens would be created under your control, twice that many of those tokens are created instead.",
                vec![StaticEffect::Custom("If one or more tokens would be created under your control, twice that many of those tokens are created instead.".into())]),
        ],
        ..Default::default() }
}

fn essence_anchor(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Essence Anchor".into(), mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Artifact],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "At the beginning of your upkeep, surveil 1.",
                vec![EventType::UpkeepStep],
                vec![Effect::Scry { count: 1 }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn felothar_dawn_of_the_abzan(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Felothar, Dawn of the Abzan".into(), mana_cost: ManaCost::parse("{W}{B}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(3),
        keywords: KeywordAbilities::TRAMPLE,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::triggered(id,
                "Whenever Felothar enters or attacks, you may sacrifice a nonland permanent. When you do, put a +1/+1 counter on each creature you control.",
                vec![EventType::EnteredTheBattlefield, EventType::AttackerDeclared],
                vec![Effect::Custom("You may sacrifice a nonland permanent. When you do, put a +1/+1 counter on each creature you control.".into())],
                TargetSpec::None).set_optional(),
        ],
        ..Default::default() }
}

fn glacial_dragonhunt(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Glacial Dragonhunt".into(), mana_cost: ManaCost::parse("{U}{R}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::DrawCards { count: 1 }, Effect::Custom("You may discard a card. When you discard a nonland card this way, Glacial Dragonhunt deals 3 damage to target creature.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn herd_heirloom(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Herd Heirloom".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Artifact],
        keywords: KeywordAbilities::TRAMPLE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{T}: Until end of turn, target creature you control with power 4 or greater gains trample and \"Whenever this creature deals combat damage to a player, draw a card.\"",
                vec![Cost::TapSelf],
                vec![Effect::GainKeywordUntilEndOfTurn { keyword: "trample".into() }],
                TargetSpec::PermanentFiltered("creature you control with power 4 or greater".into())),
            Ability::mana_ability(id,
                "{T}: Add one mana of any color. Spend this mana only to cast a creature spell.",
                Mana { any: 1, ..Default::default() }),
        ],
        ..Default::default() }
}

fn jeskai_monument(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Jeskai Monument".into(), mana_cost: ManaCost::parse("{2}"),
        card_types: vec![CardType::Artifact],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When this artifact enters, search your library for a basic Island, Mountain, or Plains card, reveal it, put it into your hand, then shuffle.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::SearchLibrary { filter: "basic Island, Mountain, or Plains card".into() }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn kishla_trawlers(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Kishla Trawlers".into(), mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Custom("Citizen".into())],
        power: Some(3), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When this creature enters, you may exile a creature card from your graveyard. When you do, return target instant or sorcery card from your graveyard to your hand.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When this creature enters, you may exile a creature card from your graveyard. When you do, return target instant or sorcery card from your graveyard to your hand.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn krumar_initiate(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Krumar Initiate".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Cleric],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{X}{B}, {T}, Pay X life: This creature endures X. Activate only as a sorcery.",
                vec![Cost::pay_mana("{X}{B}"), Cost::TapSelf, Cost::PayLife(0)],
                vec![Effect::Custom("This creature endures X.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn lasyd_prowler(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Lasyd Prowler".into(), mana_cost: ManaCost::parse("{2}{G}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Snake, SubType::Ranger],
        power: Some(5), toughness: Some(5),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When this creature enters, you may mill cards equal to the number of lands you control.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When this creature enters, you may mill cards equal to the number of lands you control.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn marang_river_regent(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Marang River Regent".into(),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon],
        power: Some(6), toughness: Some(7),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When this creature enters, return up to two other target nonland permanents to their owners' hands.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Bounce],
                TargetSpec::Multiple { spec: Box::new(TargetSpec::PermanentFiltered("other nonland permanent".into())), count: 2 }),
        ],
        ..Default::default() }
}

fn mox_jasper(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Mox Jasper".into(), mana_cost: ManaCost::parse("{0}"),
        card_types: vec![CardType::Artifact],
        supertypes: vec![SuperType::Legendary],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                "{T}: Add one mana of any color. Activate only if you control a Dragon.",
                vec![Cost::TapSelf],
                vec![Effect::AddMana { mana: Mana { any: 1, ..Default::default() } }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn natures_rhythm(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Nature's Rhythm".into(), mana_cost: ManaCost::parse("{X}{G}{G}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Search your library for a creature card with mana value X or less, put it onto the battlefield, then shuffle.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn new_way_forward(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "New Way Forward".into(), mana_cost: ManaCost::parse("{2}{U}{R}{W}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("The next time a source of your choice would deal damage to you this turn, prevent that damage. When damage is prevented this way, New Way Forward deals that much damage to that source's controller and".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn rediscover_the_way(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Rediscover the Way".into(), mana_cost: ManaCost::parse("{U}{R}{W}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Saga],
        keywords: KeywordAbilities::DOUBLE_STRIKE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("(As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn reverberating_summons(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Reverberating Summons".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Enchantment],
        keywords: KeywordAbilities::HASTE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{1}{R}, Discard your hand, Sacrifice this enchantment: Draw two cards.",
                vec![Cost::pay_mana("{1}{R}"), Cost::Custom("Discard your hand".into()), Cost::SacrificeSelf],
                vec![Effect::DrawCards { count: 2 }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn revival_of_the_ancestors(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Revival of the Ancestors".into(), mana_cost: ManaCost::parse("{1}{W}{B}{G}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Saga],
        keywords: KeywordAbilities::TRAMPLE | KeywordAbilities::LIFELINK,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("(As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn roar_of_endless_song(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Roar of Endless Song".into(), mana_cost: ManaCost::parse("{2}{G}{U}{R}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Saga],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("(As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn runescale_stormbrood(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Runescale Stormbrood".into(),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon],
        power: Some(2), toughness: Some(4),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever you cast a noncreature spell or a Dragon spell, this creature gets +2/+0 until end of turn.",
                vec![EventType::SpellCast],
                vec![Effect::BoostUntilEndOfTurn { power: 2, toughness: 0 }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn sage_of_the_fang(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sage of the Fang".into(), mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Druid],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When this creature enters, put a +1/+1 counter on target creature.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::AddCounters { counter_type: "+1/+1".into(), count: 1 }],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn sarkhan_dragon_ascendant(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sarkhan, Dragon Ascendant".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Druid],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(2),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::triggered(id,
                "When Sarkhan enters, you may behold a Dragon. If you do, create a Treasure token.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("Behold a Dragon. If you do, create a Treasure token.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn scavenger_regent(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Scavenger Regent".into(),
        mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon],
        power: Some(4), toughness: Some(4),
        keywords: KeywordAbilities::FLYING | KeywordAbilities::WARD,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "Ward -- Discard a card.",
                vec![StaticEffect::ward("Discard a card.")]),
        ],
        ..Default::default() }
}

fn songcrafter_mage(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Songcrafter Mage".into(), mana_cost: ManaCost::parse("{G}{U}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Custom("Bard".into())],
        power: Some(3), toughness: Some(2),
        keywords: KeywordAbilities::FLASH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When this creature enters, target instant or sorcery card in your graveyard gains harmonize until end of turn. Its harmonize cost is equal to its mana cost.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When this creature enters, target instant or sorcery card in your graveyard gains harmonize until end of turn. Its harmonize cost is equal to its mana cost.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn stadium_headliner(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Stadium Headliner".into(), mana_cost: ManaCost::parse("{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Goblin, SubType::Warrior],
        power: Some(1), toughness: Some(1),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{1}{R}, Sacrifice this creature: It deals damage equal to the number of creatures you control to target creature.",
                vec![Cost::pay_mana("{1}{R}"), Cost::SacrificeSelf],
                vec![Effect::Custom("Deal damage equal to the number of creatures you control to target creature.".into())],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn stillness_in_motion(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Stillness in Motion".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "At the beginning of your upkeep, mill three cards. Then if you have no cards in your library, exile this enchantment and put five cards from your graveyard on top of your library in any order.",
                vec![EventType::UpkeepStep],
                vec![Effect::Mill { count: 3 }, Effect::Custom("If you have no cards in your library, exile this enchantment and put five cards from your graveyard on top of your library in any order.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn strategic_betrayal(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Strategic Betrayal".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Target opponent exiles a creature they control and their graveyard.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn sultai_monument(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sultai Monument".into(), mana_cost: ManaCost::parse("{2}"),
        card_types: vec![CardType::Artifact],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When this artifact enters, search your library for a basic Swamp, Forest, or Island card, reveal it, put it into your hand, then shuffle.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::SearchLibrary { filter: "basic Swamp, Forest, or Island card".into() }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn taigam_master_opportunist(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Taigam, Master Opportunist".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Monk],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::spell_cast_triggered(id,
                "Flurry -- Whenever you cast your second spell each turn, copy it, then exile the spell you cast with four time counters on it. If it doesn't have suspend, it gains suspend.",
                vec![Effect::Custom("Copy the spell, then exile it with four time counters. If it doesn't have suspend, it gains suspend.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn temur_monument(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Temur Monument".into(), mana_cost: ManaCost::parse("{2}"),
        card_types: vec![CardType::Artifact],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When this artifact enters, search your library for a basic Forest, Island, or Mountain card, reveal it, put it into your hand, then shuffle.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::SearchLibrary { filter: "basic Forest, Island, or Mountain card".into() }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn the_sibsig_ceremony(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "The Sibsig Ceremony".into(), mana_cost: ManaCost::parse("{B}{B}{B}"),
        card_types: vec![CardType::Enchantment],
        supertypes: vec![SuperType::Legendary],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                "Creature spells you cast cost {2} less to cast.",
                vec![StaticEffect::CostReduction { filter: "creature spell you cast".into(), amount: 2 }]),
        ],
        ..Default::default() }
}

fn thunder_of_unity(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Thunder of Unity".into(), mana_cost: ManaCost::parse("{R}{W}{B}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Saga],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("(As this Saga enters step, add a lore counter. Sacrifice after III.)".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn ugin_eye_of_the_storms(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ugin, Eye of the Storms".into(), mana_cost: ManaCost::parse("{7}"),
        card_types: vec![CardType::Planeswalker],
        subtypes: vec![SubType::Custom("Ugin".into())],
        supertypes: vec![SuperType::Legendary],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                "When you cast this spell, exile up to one target permanent that's one or more colors.",
                vec![EventType::SpellCast],
                vec![Effect::Custom("When you cast this spell, exile up to one target permanent that's one or more colors.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn underfoot_underdogs(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Underfoot Underdogs".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Goblin, SubType::Warrior],
        power: Some(1), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When this creature enters, create a 1/1 red Goblin creature token.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::CreateToken { token_name: "1/1 red Goblin".into(), count: 1 }],
                TargetSpec::None),
            Ability::activated(id,
                "{1}, {T}: Target creature you control with power 2 or less can't be blocked this turn.",
                vec![Cost::pay_mana("{1}"), Cost::TapSelf],
                vec![Effect::CantBlock],
                TargetSpec::PermanentFiltered("creature you control with power 2 or less".into())),
        ],
        ..Default::default() }
}

fn united_battlefront(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "United Battlefront".into(), mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Look at the top seven cards of your library. Put up to two noncreature, nonland permanent cards with mana value 3 or less from among them onto the battlefield. Put the rest on the bottom of your libra".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn winternight_stories(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Winternight Stories".into(), mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::DrawCards { count: 3 }, Effect::Custom("Discard two cards unless you discard a creature card.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn yathan_roadwatcher(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Yathan Roadwatcher".into(), mana_cost: ManaCost::parse("{1}{W}{B}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Scout],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When this creature enters, if you cast it, mill four cards. When you do, return target creature card with mana value 3 or less from your graveyard to the battlefield.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Mill { count: 4 }, Effect::Custom("Return target creature card with mana value 3 or less from your graveyard to the battlefield.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}
