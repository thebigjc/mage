// Lorwyn Eclipsed (ECL) set — released 2025-06-13.
// 273 unique cards. Tier 1 creatures, spells, and non-creature permanents.

use crate::cards::basic_lands;
use crate::registry::CardRegistry;
use mtg_engine::abilities::{Ability, Cost, Effect, ModalMode, StaticEffect, TargetSpec, TriggerScope, X_VALUE};
use mtg_engine::card::CardData;
use mtg_engine::constants::*;
use mtg_engine::events::EventType;
use mtg_engine::mana::{Mana, ManaCost};
use mtg_engine::types::{ObjectId, PlayerId};

pub fn register(registry: &mut CardRegistry) {
    basic_lands::register(registry, "ECL");

    // ── Creatures ────────────────────────────────────────────────────────────
    registry.register("Adept Watershaper", adept_watershaper, "ECL");
    registry.register("Bile-Vial Boggart", bile_vial_boggart, "ECL");
    registry.register("Bitterbloom Bearer", bitterbloom_bearer, "ECL");
    registry.register("Blighted Blackthorn", blighted_blackthorn, "ECL");
    registry.register("Bloom Tender", bloom_tender, "ECL");
    registry.register("Boggart Cursecrafter", boggart_cursecrafter, "ECL");
    registry.register("Boggart Prankster", boggart_prankster, "ECL");
    registry.register("Boldwyr Aggressor", boldwyr_aggressor, "ECL");
    registry.register("Brambleback Brute", brambleback_brute, "ECL");
    registry.register("Burdened Stoneback", burdened_stoneback, "ECL");
    registry.register("Champion of the Weird", champion_of_the_weird, "ECL");
    registry.register("Champions of the Perfect", champions_of_the_perfect, "ECL");
    registry.register("Changeling Wayfinder", changeling_wayfinder, "ECL");
    registry.register("Chaos Spewer", chaos_spewer, "ECL");
    registry.register("Chitinous Graspling", chitinous_graspling, "ECL");
    registry.register("Chomping Changeling", chomping_changeling, "ECL");
    registry.register("Crossroads Watcher", crossroads_watcher, "ECL");
    registry.register("Dawn's Light Archer", dawns_light_archer, "ECL");
    registry.register("Deepchannel Duelist", deepchannel_duelist, "ECL");
    registry.register("Dream Seizer", dream_seizer, "ECL");
    registry.register("Dundoolin Weaver", dundoolin_weaver, "ECL");
    registry.register("Eclipsed Boggart", eclipsed_boggart, "ECL");
    registry.register("Eclipsed Elf", eclipsed_elf, "ECL");
    registry.register("Eclipsed Flamekin", eclipsed_flamekin, "ECL");
    registry.register("Eclipsed Kithkin", eclipsed_kithkin, "ECL");
    registry.register("Eclipsed Merrow", eclipsed_merrow, "ECL");
    registry.register("Elder Auntie", elder_auntie, "ECL");
    registry.register("Encumbered Reejerey", encumbered_reejerey, "ECL");
    registry.register("Enraged Flamecaster", enraged_flamecaster, "ECL");
    registry.register("Explosive Prodigy", explosive_prodigy, "ECL");
    registry.register("Feisty Spikeling", feisty_spikeling, "ECL");
    registry.register("Flame-Chain Mauler", flame_chain_mauler, "ECL");
    registry.register("Flamekin Gildweaver", flamekin_gildweaver, "ECL");
    registry.register("Flaring Cinder", flaring_cinder, "ECL");
    registry.register("Flock Impostor", flock_impostor, "ECL");
    registry.register("Gallant Fowlknight", gallant_fowlknight, "ECL");
    registry.register("Gangly Stompling", gangly_stompling, "ECL");
    registry.register("Glamermite", glamermite, "ECL");
    registry.register("Glister Bairn", glister_bairn, "ECL");
    registry.register("Gnarlbark Elm", gnarlbark_elm, "ECL");
    registry.register("Goldmeadow Nomad", goldmeadow_nomad, "ECL");
    registry.register("Graveshifter", graveshifter, "ECL");
    registry.register("Great Forest Druid", great_forest_druid, "ECL");
    registry.register("Gristle Glutton", gristle_glutton, "ECL");
    registry.register("Gutsplitter Gang", gutsplitter_gang, "ECL");
    registry.register("Heirloom Auntie", heirloom_auntie, "ECL");
    registry.register("Iron-Shield Elf", iron_shield_elf, "ECL");
    registry.register("Kinsbaile Aspirant", kinsbaile_aspirant, "ECL");
    registry.register("Kulrath Mystic", kulrath_mystic, "ECL");
    registry.register("Kulrath Zealot", kulrath_zealot, "ECL");
    registry.register("Luminollusk", luminollusk, "ECL");
    registry.register("Lys Alana Informant", lys_alana_informant, "ECL");
    registry.register("Merrow Skyswimmer", merrow_skyswimmer, "ECL");
    registry.register("Mischievous Sneakling", mischievous_sneakling, "ECL");
    registry.register("Moonglove Extractor", moonglove_extractor, "ECL");
    registry.register("Moonlit Lamenter", moonlit_lamenter, "ECL");
    registry.register("Mutable Explorer", mutable_explorer, "ECL");
    registry.register("Nightmare Sower", nightmare_sower, "ECL");
    registry.register("Noggle Robber", noggle_robber, "ECL");
    registry.register("Pestered Wellguard", pestered_wellguard, "ECL");
    registry.register("Prideful Feastling", prideful_feastling, "ECL");
    registry.register("Prismabasher", prismabasher, "ECL");
    registry.register("Reluctant Dounguard", reluctant_dounguard, "ECL");
    registry.register("Rimekin Recluse", rimekin_recluse, "ECL");
    registry.register("Rooftop Percher", rooftop_percher, "ECL");
    registry.register("Safewright Cavalry", safewright_cavalry, "ECL");
    registry.register("Scarblade Scout", scarblade_scout, "ECL");
    registry.register("Scuzzback Scrounger", scuzzback_scrounger, "ECL");
    registry.register("Shimmercreep", shimmercreep, "ECL");
    registry.register("Shinestriker", shinestriker, "ECL");
    registry.register("Shore Lurker", shore_lurker, "ECL");
    registry.register("Silvergill Mentor", silvergill_mentor, "ECL");
    registry.register("Silvergill Peddler", silvergill_peddler, "ECL");
    registry.register("Sizzling Changeling", sizzling_changeling, "ECL");
    registry.register("Sourbread Auntie", sourbread_auntie, "ECL");
    registry.register("Squawkroaster", squawkroaster, "ECL");
    registry.register("Sting-Slinger", sting_slinger, "ECL");
    registry.register("Stoic Grove-Guide", stoic_grove_guide, "ECL");
    registry.register("Stratosoarer", stratosoarer, "ECL");
    registry.register("Summit Sentinel", summit_sentinel, "ECL");
    registry.register("Sun-Dappled Celebrant", sun_dappled_celebrant, "ECL");
    registry.register("Surly Farrier", surly_farrier, "ECL");
    registry.register("Tanufel Rimespeaker", tanufel_rimespeaker, "ECL");
    registry.register("Thoughtweft Imbuer", thoughtweft_imbuer, "ECL");
    registry.register("Thoughtweft Lieutenant", thoughtweft_lieutenant, "ECL");
    registry.register("Timid Shieldbearer", timid_shieldbearer, "ECL");
    registry.register("Unwelcome Sprite", unwelcome_sprite, "ECL");
    registry.register("Virulent Emissary", virulent_emissary, "ECL");
    registry.register("Voracious Tome-Skimmer", voracious_tome_skimmer, "ECL");
    registry.register("Wanderbrine Preacher", wanderbrine_preacher, "ECL");
    registry.register("Wanderwine Distracter", wanderwine_distracter, "ECL");
    registry.register("Warren Torchmaster", warren_torchmaster, "ECL");

    // ── Spells and non-creature permanents ────────────────────────────────
    registry.register("Appeal to Eirdu", appeal_to_eirdu, "ECL");
    registry.register("Burning Curiosity", burning_curiosity, "ECL");
    registry.register("Cinder Strike", cinder_strike, "ECL");
    registry.register("Crib Swap", crib_swap, "ECL");
    registry.register("Evolving Wilds", super::fdn::evolving_wilds, "ECL");
    registry.register("Liminal Hold", liminal_hold, "ECL");
    registry.register("Protective Response", protective_response, "ECL");
    registry.register("Springleaf Drum", springleaf_drum, "ECL");
    registry.register("Temporal Cleansing", temporal_cleansing, "ECL");
    registry.register("Unexpected Assistance", unexpected_assistance, "ECL");

    // ── Tier 2 — additional spells ──────────────────────────────────────────
    registry.register("Assert Perfection", assert_perfection, "ECL");
    registry.register("Blight Rot", blight_rot, "ECL");
    registry.register("Blossoming Defense", blossoming_defense, "ECL");
    registry.register("Boggart Mischief", boggart_mischief, "ECL");
    registry.register("Darkness Descends", darkness_descends, "ECL");
    registry.register("Dose of Dawnglow", dose_of_dawnglow, "ECL");
    registry.register("Feed the Flames", feed_the_flames, "ECL");
    registry.register("Meek Attack", meek_attack, "ECL");
    registry.register("Midnight Tilling", midnight_tilling, "ECL");
    registry.register("Nameless Inversion", nameless_inversion, "ECL");
    registry.register("Reckless Ransacking", reckless_ransacking, "ECL");
    registry.register("Rime Chill", rime_chill, "ECL");
    registry.register("Run Away Together", run_away_together, "ECL");
    registry.register("Scarblade's Malice", scarblades_malice, "ECL");
    registry.register("Sear", sear, "ECL");
    registry.register("Soul Immolation", soul_immolation, "ECL");
    registry.register("Spell Snare", spell_snare, "ECL");
    registry.register("Thoughtweft Charge", thoughtweft_charge, "ECL");
    registry.register("Tweeze", tweeze, "ECL");
    registry.register("Wild Unraveling", wild_unraveling, "ECL");

    // ── Tier 3 — complex cards ─────────────────────────────────────────────
    registry.register("Abigale, Eloquent First-Year", abigale_eloquent_first_year, "ECL");
    registry.register("Aquitect's Defenses", aquitects_defenses, "ECL");
    registry.register("Ashling's Command", ashlings_command, "ECL");
    registry.register("Auntie's Sentence", aunties_sentence, "ECL");
    registry.register("Aurora Awakener", aurora_awakener, "ECL");
    registry.register("Barbed Bloodletter", barbed_bloodletter, "ECL");
    registry.register("Bark of Doran", bark_of_doran, "ECL");
    registry.register("Blood Crypt", blood_crypt, "ECL");
    registry.register("Bloodline Bidding", bloodline_bidding, "ECL");
    registry.register("Blossombind", blossombind, "ECL");
    registry.register("Bre of Clan Stoutarm", bre_of_clan_stoutarm, "ECL");
    registry.register("Brigid's Command", brigids_command, "ECL");
    registry.register("Bristlebane Battler", bristlebane_battler, "ECL");
    registry.register("Bristlebane Outrider", bristlebane_outrider, "ECL");
    registry.register("Catharsis", catharsis, "ECL");
    registry.register("Celestial Reunion", celestial_reunion, "ECL");
    registry.register("Champion of the Clachan", champion_of_the_clachan, "ECL");
    registry.register("Champion of the Path", champion_of_the_path, "ECL");
    registry.register("Chronicle of Victory", chronicle_of_victory, "ECL");
    registry.register("Collective Inferno", collective_inferno, "ECL");
    registry.register("Curious Colossus", curious_colossus, "ECL");
    registry.register("Dawn-Blessed Pennant", dawn_blessed_pennant, "ECL");
    registry.register("Dawnhand Dissident", dawnhand_dissident, "ECL");
    registry.register("Deceit", deceit, "ECL");
    registry.register("Deepway Navigator", deepway_navigator, "ECL");
    registry.register("Disruptor of Currents", disruptor_of_currents, "ECL");
    registry.register("Doran, Besieged by Time", doran_besieged_by_time, "ECL");
    registry.register("Eclipsed Realms", eclipsed_realms, "ECL");
    registry.register("Eirdu, Carrier of Dawn", eirdu_carrier_of_dawn, "ECL");
    registry.register("Emptiness", emptiness, "ECL");
    registry.register("Ashling, Rekindled", ashling_rekindled, "ECL");
    registry.register("Brigid, Clachan's Heart", brigid_clachans_heart, "ECL");
    registry.register("Evershrike's Gift", evershrikes_gift, "ECL");
    registry.register("Figure of Fable", figure_of_fable, "ECL");
    registry.register("Firdoch Core", firdoch_core, "ECL");
    registry.register("Flitterwing Nuisance", flitterwing_nuisance, "ECL");
    registry.register("Foraging Wickermaw", foraging_wickermaw, "ECL");
    registry.register("Gathering Stone", gathering_stone, "ECL");
    registry.register("Giantfall", giantfall, "ECL");
    registry.register("Gilt-Leaf's Embrace", gilt_leafs_embrace, "ECL");
    registry.register("Glamer Gifter", glamer_gifter, "ECL");
    registry.register("Glen Elendra Guardian", glen_elendra_guardian, "ECL");
    registry.register("Goliath Daydreamer", goliath_daydreamer, "ECL");
    registry.register("Grub, Storied Matriarch", grub_storied_matriarch, "ECL");
    registry.register("Grub's Command", grubs_command, "ECL");
    registry.register("Hallowed Fountain", hallowed_fountain, "ECL");
    registry.register("Harmonized Crescendo", harmonized_crescendo, "ECL");
    registry.register("Hexing Squelcher", hexing_squelcher, "ECL");
    registry.register("High Perfect Morcant", high_perfect_morcant, "ECL");
    registry.register("Illusion Spinners", illusion_spinners, "ECL");
    registry.register("Keep Out", keep_out, "ECL");
    registry.register("Kinbinding", kinbinding, "ECL");
    registry.register("Kinscaer Sentry", kinscaer_sentry, "ECL");
    registry.register("Lasting Tarfire", lasting_tarfire, "ECL");
    registry.register("Lluwen, Imperfect Naturalist", lluwen_imperfect_naturalist, "ECL");
    registry.register("Loch Mare", loch_mare, "ECL");
    registry.register("Lofty Dreams", lofty_dreams, "ECL");
    registry.register("Maralen, Fae Ascendant", maralen_fae_ascendant, "ECL");
    registry.register("Mirrormind Crown", mirrormind_crown, "ECL");
    registry.register("Moonshadow", moonshadow, "ECL");
    registry.register("Morcant's Loyalist", morcants_loyalist, "ECL");
    registry.register("Mornsong Aria", mornsong_aria, "ECL");
    registry.register("Noggle the Mind", noggle_the_mind, "ECL");
    registry.register("Omni-Changeling", omni_changeling, "ECL");
    registry.register("Overgrown Tomb", overgrown_tomb, "ECL");
    registry.register("Perfect Intimidation", perfect_intimidation, "ECL");
    registry.register("Pitiless Fists", pitiless_fists, "ECL");
    registry.register("Prismatic Undercurrents", prismatic_undercurrents, "ECL");
    registry.register("Puca's Eye", pucas_eye, "ECL");
    registry.register("Pummeler for Hire", pummeler_for_hire, "ECL");
    registry.register("Pyrrhic Strike", pyrrhic_strike, "ECL");
    registry.register("Reaping Willow", reaping_willow, "ECL");
    registry.register("Retched Wretch", retched_wretch, "ECL");
    registry.register("Rhys, the Evermore", rhys_the_evermore, "ECL");
    registry.register("Rimefire Torque", rimefire_torque, "ECL");
    registry.register("Sanar, Innovative First-Year", sanar_innovative_first_year, "ECL");
    registry.register("Sapling Nursery", sapling_nursery, "ECL");
    registry.register("Selfless Safewright", selfless_safewright, "ECL");
    registry.register("Shadow Urchin", shadow_urchin, "ECL");
    registry.register("Shimmerwilds Growth", shimmerwilds_growth, "ECL");
    registry.register("Spinerock Tyrant", spinerock_tyrant, "ECL");
    registry.register("Spiral into Solitude", spiral_into_solitude, "ECL");
    registry.register("Spry and Mighty", spry_and_mighty, "ECL");
    registry.register("Stalactite Dagger", stalactite_dagger, "ECL");
    registry.register("Steam Vents", steam_vents, "ECL");
    registry.register("Sunderflock", sunderflock, "ECL");
    registry.register("Sygg, Wanderwine Wisdom", sygg_wanderwine_wisdom, "ECL");
    registry.register("Sygg's Command", syggs_command, "ECL");
    registry.register("Tam, Mindful First-Year", tam_mindful_first_year, "ECL");
    registry.register("Taster of Wares", taster_of_wares, "ECL");
    registry.register("Temple Garden", temple_garden, "ECL");
    registry.register("Trystan, Callous Cultivator", trystan_callous_cultivator, "ECL");
    registry.register("Trystan's Command", trystans_command, "ECL");
    registry.register("Twilight Diviner", twilight_diviner, "ECL");
    registry.register("Twinflame Travelers", twinflame_travelers, "ECL");
    registry.register("Unbury", unbury, "ECL");
    registry.register("Unforgiving Aim", unforgiving_aim, "ECL");
    registry.register("Vibrance", vibrance, "ECL");
    registry.register("Wanderwine Farewell", wanderwine_farewell, "ECL");
    registry.register("Wary Farmer", wary_farmer, "ECL");
    registry.register("Wildvine Pummeler", wildvine_pummeler, "ECL");
    registry.register("Winnowing", winnowing, "ECL");
    registry.register("Wistfulness", wistfulness, "ECL");

    // ── New Creatures ────────────────────────────────────────────────────
    registry.register("Boneclub Berserker", boneclub_berserker, "ECL");
    registry.register("Champions of the Shoal", champions_of_the_shoal, "ECL");
    registry.register("Creakwood Safewright", creakwood_safewright, "ECL");
    registry.register("Dawnhand Eulogist", dawnhand_eulogist, "ECL");
    registry.register("Flamebraider", flamebraider, "ECL");
    registry.register("Formidable Speaker", formidable_speaker, "ECL");
    registry.register("Gloom Ripper", gloom_ripper, "ECL");
    registry.register("Gravelgill Scoundrel", gravelgill_scoundrel, "ECL");
    registry.register("Hovel Hurler", hovel_hurler, "ECL");
    registry.register("Kirol, Attentive First-Year", kirol_attentive_first_year, "ECL");
    registry.register("Kithkeeper", kithkeeper, "ECL");
    registry.register("Lavaleaper", lavaleaper, "ECL");
    registry.register("Lys Alana Dignitary", lys_alana_dignitary, "ECL");
    registry.register("Meanders Guide", meanders_guide, "ECL");
    registry.register("Mistmeadow Council", mistmeadow_council, "ECL");
    registry.register("Moon-Vigil Adherents", moon_vigil_adherents, "ECL");
    registry.register("Mudbutton Cursetosser", mudbutton_cursetosser, "ECL");
    registry.register("Slumbering Walker", slumbering_walker, "ECL");
    registry.register("Soulbright Seeker", soulbright_seeker, "ECL");
    registry.register("Tributary Vaulter", tributary_vaulter, "ECL");
    registry.register("Vinebred Brawler", vinebred_brawler, "ECL");
    registry.register("Wanderbrine Trapper", wanderbrine_trapper, "ECL");

    // ── New Instants and Sorceries ────────────────────────────────────────
    registry.register("Bogslither's Embrace", bogslithers_embrace, "ECL");
    registry.register("Boulder Dash", boulder_dash, "ECL");
    registry.register("Dream Harvest", dream_harvest, "ECL");
    registry.register("End-Blaze Epiphany", end_blaze_epiphany, "ECL");
    registry.register("Glen Elendra's Answer", glen_elendras_answer, "ECL");
    registry.register("Goatnap", goatnap, "ECL");
    registry.register("Impolite Entrance", impolite_entrance, "ECL");
    registry.register("Kindle the Inner Flame", kindle_the_inner_flame, "ECL");
    registry.register("Mirrorform", mirrorform, "ECL");
    registry.register("Morningtide's Light", morningtides_light, "ECL");
    registry.register("Personify", personify, "ECL");
    registry.register("Requiting Hex", requiting_hex, "ECL");
    registry.register("Riverguard's Reflexes", riverguards_reflexes, "ECL");
    registry.register("Swat Away", swat_away, "ECL");
    registry.register("Tend the Sprigs", tend_the_sprigs, "ECL");
    registry.register("Thirst for Identity", thirst_for_identity, "ECL");

    // ── New Enchantments ──────────────────────────────────────────────────
    registry.register("Clachan Festival", clachan_festival, "ECL");
    registry.register("Morcant's Eyes", morcants_eyes, "ECL");
    registry.register("Raiding Schemes", raiding_schemes, "ECL");

    // ── Other ─────────────────────────────────────────────────────────────
    registry.register("Ajani, Outland Chaperone", ajani_outland_chaperone, "ECL");
    registry.register("Oko, Lorwyn Liege", oko_lorwyn_liege, "ECL");
}

// ── Creature implementations ─────────────────────────────────────────────────

fn adept_watershaper(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/4 Merfolk Cleric for {2}{W}. (Other tapped creatures you control have indestructible)
    CardData { id, owner, name: "Adept Watershaper".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Merfolk, SubType::Cleric],
        power: Some(3), toughness: Some(4), rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "Other tapped creatures you control have indestructible.",
                vec![StaticEffect::grant_keyword_controlled("other tapped creatures you control", "indestructible")]),
        ],
        ..Default::default() }
}

fn bile_vial_boggart(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/1 Goblin Assassin for {B}. (Dies: put -1/-1 counter on creature)
    CardData { id, owner, name: "Bile-Vial Boggart".into(), mana_cost: ManaCost::parse("{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Assassin],
        power: Some(1), toughness: Some(1), rarity: Rarity::Common,
        abilities: vec![
            Ability::dies_triggered(id,
                "When Bile-Vial Boggart dies, put a -1/-1 counter on target creature.",
                vec![Effect::add_counters("-1/-1", 1)],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn bitterbloom_bearer(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/1 Faerie Rogue for {B}{B}. Flash, flying. (Upkeep: lose 1 life, create 1/1 Faerie)
    CardData { id, owner, name: "Bitterbloom Bearer".into(), mana_cost: ManaCost::parse("{B}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Faerie, SubType::Rogue],
        power: Some(1), toughness: Some(1),
        keywords: KeywordAbilities::FLASH | KeywordAbilities::FLYING,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::beginning_of_upkeep_triggered(id,
                "At the beginning of your upkeep, you lose 1 life and create a 1/1 black Faerie Rogue creature token with flying.",
                vec![Effect::lose_life(1), Effect::create_token("1/1 Faerie Rogue with flying", 1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn blighted_blackthorn(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/7 Treefolk Warlock for {4}{B}. (ETB/attacks: blight 2 => draw + lose 1 life)
    CardData { id, owner, name: "Blighted Blackthorn".into(), mana_cost: ManaCost::parse("{4}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Treefolk, SubType::Warlock],
        power: Some(3), toughness: Some(7), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Blighted Blackthorn enters or attacks, put two -1/-1 counters on it, then draw a card and lose 1 life.",
                vec![Effect::add_counters("-1/-1", 2), Effect::draw_cards(1), Effect::lose_life(1)],
                TargetSpec::None),
            Ability::attacks_triggered(id,
                "Whenever Blighted Blackthorn attacks, put two -1/-1 counters on it, then draw a card and lose 1 life.",
                vec![Effect::add_counters("-1/-1", 2), Effect::draw_cards(1), Effect::lose_life(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn bloom_tender(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/1 Elf Druid for {1}{G}. (T: add mana for each color among your permanents)
    CardData { id, owner, name: "Bloom Tender".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elf, SubType::Druid],
        power: Some(1), toughness: Some(1), rarity: Rarity::Rare,
        abilities: vec![
            Ability::mana_ability(id, "{T}: For each color among permanents you control, add one mana of that color.", Mana::green(1)),
        ],
        ..Default::default() }
}

fn boggart_cursecrafter(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/3 Goblin Warlock for {B}{R}. Deathtouch. (Another Goblin dies: 1 damage to opponents)
    CardData { id, owner, name: "Boggart Cursecrafter".into(), mana_cost: ManaCost::parse("{B}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Warlock],
        power: Some(2), toughness: Some(3), keywords: KeywordAbilities::DEATHTOUCH,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::any_creature_dies_triggered(id,
                "Whenever another Goblin you control dies, Boggart Cursecrafter deals 1 damage to each opponent.",
                vec![Effect::damage_opponents(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn boggart_prankster(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/3 Goblin Warrior for {1}{B}. (Attack trigger: target Goblin +1/+0)
    CardData { id, owner, name: "Boggart Prankster".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Warrior],
        power: Some(1), toughness: Some(3), rarity: Rarity::Common,
        abilities: vec![
            Ability::attacks_triggered(id,
                "Whenever Boggart Prankster attacks, target Goblin you control gets +1/+0 until end of turn.",
                vec![Effect::boost_until_eot(1, 0)],
                TargetSpec::PermanentFiltered("Goblin you control".into())),
        ],
        ..Default::default() }
}

fn boldwyr_aggressor(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/5 Giant Warrior for {3}{R}{R}. Double strike. (Other Giants have double strike)
    CardData { id, owner, name: "Boldwyr Aggressor".into(), mana_cost: ManaCost::parse("{3}{R}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Giant, SubType::Warrior],
        power: Some(2), toughness: Some(5), keywords: KeywordAbilities::DOUBLE_STRIKE,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "Other Giant creatures you control have double strike.",
                vec![StaticEffect::grant_keyword_controlled("other Giants you control", "double strike")]),
        ],
        ..Default::default() }
}

fn brambleback_brute(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/5 Giant Warrior for {2}{R}. (ETB with two -1/-1 counters; activated: can't block)
    CardData { id, owner, name: "Brambleback Brute".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Giant, SubType::Warrior],
        power: Some(4), toughness: Some(5), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                "Brambleback Brute enters with two -1/-1 counters on it.",
                vec![StaticEffect::enters_with_counters("-1/-1", 2)]),
            Ability::activated(id,
                "{1}{R}: Target creature can't block this turn.",
                vec![Cost::pay_mana("{1}{R}")],
                vec![Effect::CantBlock],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn burdened_stoneback(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/4 Giant Warrior for {1}{W}. (ETB with two -1/-1 counters; activated: indestructible)
    CardData { id, owner, name: "Burdened Stoneback".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Giant, SubType::Warrior],
        power: Some(4), toughness: Some(4), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                "Burdened Stoneback enters with two -1/-1 counters on it.",
                vec![StaticEffect::enters_with_counters("-1/-1", 2)]),
            Ability::activated(id,
                "{2}{W}: Burdened Stoneback gains indestructible until end of turn.",
                vec![Cost::pay_mana("{2}{W}")],
                vec![Effect::gain_keyword_eot("indestructible")],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn champion_of_the_weird(id: ObjectId, owner: PlayerId) -> CardData {
    // 5/5 Goblin Berserker for {3}{B}. (Behold+exile Goblin; blight activated; LTB: return exiled)
    CardData { id, owner, name: "Champion of the Weird".into(), mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Berserker],
        power: Some(5), toughness: Some(5), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                "{1}{B}, Put a -1/-1 counter on this creature: Each opponent loses 2 life.",
                vec![Cost::pay_mana("{1}{B}"), Cost::Blight(1)],
                vec![Effect::lose_life_opponents(2)],
                TargetSpec::None),
            Ability::triggered(id,
                "When this creature leaves the battlefield, return the exiled card to its owner's hand.",
                vec![EventType::ZoneChanged],
                vec![Effect::return_exiled_to_hand()],
                TargetSpec::None),
        ],
        additional_costs: vec![Cost::behold_and_exile("Goblin")],
        ..Default::default() }
}

fn champions_of_the_perfect(id: ObjectId, owner: PlayerId) -> CardData {
    // 6/6 Elf Warrior for {3}{G}. (Behold+exile Elf; creature spell cast: draw; LTB: return exiled)
    CardData { id, owner, name: "Champions of the Perfect".into(), mana_cost: ManaCost::parse("{3}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elf, SubType::Warrior],
        power: Some(6), toughness: Some(6), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                "Whenever you cast a creature spell, draw a card.",
                vec![EventType::SpellCast],
                vec![Effect::draw_cards(1)],
                TargetSpec::None),
            Ability::triggered(id,
                "When this creature leaves the battlefield, return the exiled card to its owner's hand.",
                vec![EventType::ZoneChanged],
                vec![Effect::return_exiled_to_hand()],
                TargetSpec::None),
        ],
        additional_costs: vec![Cost::behold_and_exile("Elf")],
        ..Default::default() }
}

fn changeling_wayfinder(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/2 Shapeshifter for {3}. Changeling. (ETB: search for basic land)
    CardData { id, owner, name: "Changeling Wayfinder".into(), mana_cost: ManaCost::parse("{3}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Shapeshifter],
        power: Some(1), toughness: Some(2), keywords: KeywordAbilities::CHANGELING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Changeling Wayfinder enters, you may search your library for a basic land card, reveal it, put it into your hand, then shuffle.",
                vec![Effect::search_library("basic land")],
                TargetSpec::None).set_optional(),
        ],
        ..Default::default() }
}

fn chaos_spewer(id: ObjectId, owner: PlayerId) -> CardData {
    // 5/4 Goblin Warlock for {2}{B/R}. (ETB: pay 2 or blight 2)
    CardData { id, owner, name: "Chaos Spewer".into(), mana_cost: ManaCost::parse("{2}{B/R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Warlock],
        power: Some(5), toughness: Some(4), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Chaos Spewer enters, you may pay {2}. If you don't, put two -1/-1 counters on it.",
                vec![Effect::do_if_cost_paid(Cost::pay_mana("{2}"), vec![], vec![Effect::add_counters_self("-1/-1", 2)])],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn chitinous_graspling(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/4 Shapeshifter for {3}{G/U}. Changeling, reach.
    CardData { id, owner, name: "Chitinous Graspling".into(), mana_cost: ManaCost::parse("{3}{G/U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Shapeshifter],
        power: Some(3), toughness: Some(4),
        keywords: KeywordAbilities::CHANGELING | KeywordAbilities::REACH,
        rarity: Rarity::Common, ..Default::default() }
}

fn chomping_changeling(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/2 Shapeshifter for {2}{G}. Changeling. (ETB: destroy artifact or enchantment)
    CardData { id, owner, name: "Chomping Changeling".into(), mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Shapeshifter],
        power: Some(1), toughness: Some(2), keywords: KeywordAbilities::CHANGELING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Chomping Changeling enters, you may destroy target artifact or enchantment.",
                vec![Effect::destroy()],
                TargetSpec::PermanentFiltered("artifact or enchantment".into())).set_optional(),
        ],
        ..Default::default() }
}

fn crossroads_watcher(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/3 Kithkin Ranger for {2}{G}. Trample. (Creature ETB: +1/+0 until end of turn)
    CardData { id, owner, name: "Crossroads Watcher".into(), mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Kithkin, SubType::Ranger],
        power: Some(3), toughness: Some(3), keywords: KeywordAbilities::TRAMPLE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::other_creature_etb_triggered(id,
                "Whenever another creature enters under your control, Crossroads Watcher gets +1/+0 until end of turn.",
                vec![Effect::boost_until_eot(1, 0)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn dawns_light_archer(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/2 Elf Archer for {2}{G}. Flash, reach.
    CardData { id, owner, name: "Dawn's Light Archer".into(), mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elf, SubType::Archer],
        power: Some(4), toughness: Some(2),
        keywords: KeywordAbilities::FLASH | KeywordAbilities::REACH,
        rarity: Rarity::Common, ..Default::default() }
}

fn deepchannel_duelist(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Merfolk Soldier for {W}{U}. (End step: untap Merfolk; other Merfolk +1/+1)
    CardData { id, owner, name: "Deepchannel Duelist".into(), mana_cost: ManaCost::parse("{W}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Merfolk, SubType::Soldier],
        power: Some(2), toughness: Some(2), rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "Other Merfolk you control get +1/+1.",
                vec![StaticEffect::boost_controlled("other Merfolk you control", 1, 1)]),
        ],
        ..Default::default() }
}

fn dream_seizer(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/2 Faerie Rogue for {3}{B}. Flying. (ETB: blight 1 => opponents discard)
    CardData { id, owner, name: "Dream Seizer".into(), mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Faerie, SubType::Rogue],
        power: Some(3), toughness: Some(2), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Dream Seizer enters, put a -1/-1 counter on it. When you do, each opponent discards a card.",
                vec![Effect::add_counters("-1/-1", 1), Effect::discard_opponents(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn dundoolin_weaver(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/1 Kithkin Druid for {1}{G}. (ETB if 3+ creatures: return permanent from graveyard)
    CardData { id, owner, name: "Dundoolin Weaver".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Kithkin, SubType::Druid],
        power: Some(2), toughness: Some(1), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Dundoolin Weaver enters, if you control three or more creatures, return target permanent card from your graveyard to your hand.",
                vec![Effect::return_from_graveyard()],
                TargetSpec::CardInYourGraveyard).set_optional(),
        ],
        ..Default::default() }
}

fn eclipsed_boggart(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/3 Goblin Scout for {B/R}{B/R}{B/R}. (ETB: look top 4 for Goblin/Swamp/Mountain)
    CardData { id, owner, name: "Eclipsed Boggart".into(), mana_cost: ManaCost::parse("{B/R}{B/R}{B/R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Scout],
        power: Some(2), toughness: Some(3), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Eclipsed Boggart enters, look at the top four cards of your library. You may reveal a Goblin, Swamp, or Mountain card from among them and put it into your hand. Put the rest on the bottom in any order.",
                vec![Effect::look_top_and_pick(4, "Goblin or Swamp or Mountain")],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn eclipsed_elf(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/2 Elf Scout for {B/G}{B/G}{B/G}. (ETB: look top 4 for Elf/Swamp/Forest)
    CardData { id, owner, name: "Eclipsed Elf".into(), mana_cost: ManaCost::parse("{B/G}{B/G}{B/G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elf, SubType::Scout],
        power: Some(3), toughness: Some(2), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Eclipsed Elf enters, look at the top four cards of your library. You may reveal an Elf, Swamp, or Forest card from among them and put it into your hand. Put the rest on the bottom in any order.",
                vec![Effect::look_top_and_pick(4, "Elf or Swamp or Forest")],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn eclipsed_flamekin(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/4 Elemental Scout for {1}{U/R}{U/R}. (ETB: look top 4 for Elemental/Island/Mountain)
    CardData { id, owner, name: "Eclipsed Flamekin".into(), mana_cost: ManaCost::parse("{1}{U/R}{U/R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elemental, SubType::Scout],
        power: Some(1), toughness: Some(4), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Eclipsed Flamekin enters, look at the top four cards of your library. You may reveal an Elemental, Island, or Mountain card from among them and put it into your hand. Put the rest on the bottom in any order.",
                vec![Effect::look_top_and_pick(4, "Elemental or Island or Mountain")],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn eclipsed_kithkin(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/1 Kithkin Scout for {G/W}{G/W}. (ETB: look top 4 for Kithkin/Forest/Plains)
    CardData { id, owner, name: "Eclipsed Kithkin".into(), mana_cost: ManaCost::parse("{G/W}{G/W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Kithkin, SubType::Scout],
        power: Some(2), toughness: Some(1), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Eclipsed Kithkin enters, look at the top four cards of your library. You may reveal a Kithkin, Forest, or Plains card from among them and put it into your hand. Put the rest on the bottom in any order.",
                vec![Effect::look_top_and_pick(4, "Kithkin or Forest or Plains")],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn eclipsed_merrow(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/3 Merfolk Scout for {W/U}{W/U}{W/U}. (ETB: look top 4 for Merfolk/Plains/Island)
    CardData { id, owner, name: "Eclipsed Merrow".into(), mana_cost: ManaCost::parse("{W/U}{W/U}{W/U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Merfolk, SubType::Scout],
        power: Some(2), toughness: Some(3), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Eclipsed Merrow enters, look at the top four cards of your library. You may reveal a Merfolk, Plains, or Island card from among them and put it into your hand. Put the rest on the bottom in any order.",
                vec![Effect::look_top_and_pick(4, "Merfolk or Plains or Island")],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn elder_auntie(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Goblin Warlock for {2}{R}. (ETB: create 1/1 B/R Goblin token)
    CardData { id, owner, name: "Elder Auntie".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Warlock],
        power: Some(2), toughness: Some(2), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Elder Auntie enters, create a 1/1 black and red Goblin creature token.",
                vec![Effect::create_token("1/1 Goblin", 1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn encumbered_reejerey(id: ObjectId, owner: PlayerId) -> CardData {
    // 5/4 Merfolk Soldier for {1}{W}. (ETB with 3 -1/-1 counters; tapped: remove a counter)
    CardData { id, owner, name: "Encumbered Reejerey".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Merfolk, SubType::Soldier],
        power: Some(5), toughness: Some(4), rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "Encumbered Reejerey enters with three -1/-1 counters on it.",
                vec![StaticEffect::enters_with_counters("-1/-1", 3)]),
            Ability::triggered(id,
                "Whenever Encumbered Reejerey becomes tapped, remove a -1/-1 counter from it.",
                vec![EventType::Tapped],
                vec![Effect::RemoveCounters { counter_type: "-1/-1".into(), count: 1 }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn enraged_flamecaster(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/2 Elemental Sorcerer for {2}{R}. Reach. (Spell MV>=4: 2 damage to opponents)
    CardData { id, owner, name: "Enraged Flamecaster".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elemental, SubType::Sorcerer],
        power: Some(3), toughness: Some(2), keywords: KeywordAbilities::REACH,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                "Whenever you cast a spell with mana value 4 or greater, Enraged Flamecaster deals 2 damage to each opponent.",
                vec![EventType::SpellCast],
                vec![Effect::damage_opponents(2)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn explosive_prodigy(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/1 Elemental Sorcerer for {1}{R}. (Vivid: ETB deals X damage to creature)
    CardData { id, owner, name: "Explosive Prodigy".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elemental, SubType::Sorcerer],
        power: Some(1), toughness: Some(1), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "Vivid — When this creature enters, it deals X damage to target creature, where X is the number of colors among permanents you control.",
                vec![Effect::deal_damage_vivid()],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn feisty_spikeling(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/1 Shapeshifter for {1}{R/W}. Changeling. (Your turn: first strike)
    CardData { id, owner, name: "Feisty Spikeling".into(), mana_cost: ManaCost::parse("{1}{R/W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Shapeshifter],
        power: Some(2), toughness: Some(1), keywords: KeywordAbilities::CHANGELING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "As long as it's your turn, Feisty Spikeling has first strike.",
                vec![StaticEffect::ConditionalKeyword { keyword: "first strike".into(), condition: "your turn".into() }]),
        ],
        ..Default::default() }
}

fn flame_chain_mauler(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Elemental Warrior for {1}{R}. ({1}{R}: +1/+0 and menace until end of turn)
    CardData { id, owner, name: "Flame-Chain Mauler".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elemental, SubType::Warrior],
        power: Some(2), toughness: Some(2), rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{1}{R}: Flame-Chain Mauler gets +1/+0 and gains menace until end of turn.",
                vec![Cost::pay_mana("{1}{R}")],
                vec![Effect::boost_until_eot(1, 0), Effect::gain_keyword_eot("menace")],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn flamekin_gildweaver(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/3 Elemental Sorcerer for {3}{R}. Trample. (ETB: create Treasure)
    CardData { id, owner, name: "Flamekin Gildweaver".into(), mana_cost: ManaCost::parse("{3}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elemental, SubType::Sorcerer],
        power: Some(4), toughness: Some(3), keywords: KeywordAbilities::TRAMPLE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Flamekin Gildweaver enters, create a Treasure token.",
                vec![Effect::create_token("Treasure", 1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn flaring_cinder(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/2 Elemental Sorcerer for {1}{U/R}{U/R}. (ETB/spell MV>=4: loot)
    CardData { id, owner, name: "Flaring Cinder".into(), mana_cost: ManaCost::parse("{1}{U/R}{U/R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elemental, SubType::Sorcerer],
        power: Some(3), toughness: Some(2), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Flaring Cinder enters, you may draw a card, then discard a card.",
                vec![Effect::draw_cards(1), Effect::discard_cards(1)],
                TargetSpec::None),
            Ability::triggered(id,
                "Whenever you cast a spell with mana value 4 or greater, you may draw a card, then discard a card.",
                vec![EventType::SpellCast],
                vec![Effect::draw_cards(1), Effect::discard_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn flock_impostor(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Shapeshifter for {2}{W}. Changeling, flash, flying. (ETB: bounce own creature)
    CardData { id, owner, name: "Flock Impostor".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Shapeshifter],
        power: Some(2), toughness: Some(2),
        keywords: KeywordAbilities::CHANGELING | KeywordAbilities::FLASH | KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Flock Impostor enters, you may return another creature you control to its owner's hand.",
                vec![Effect::bounce()],
                TargetSpec::PermanentFiltered("another creature you control".into())).set_optional(),
        ],
        ..Default::default() }
}

fn gallant_fowlknight(id: ObjectId, owner: PlayerId) -> CardData {
    // DONE - 3/4 Kithkin Knight for {3}{W}. (ETB: creatures +1/+0, Kithkin gain first strike)
    CardData { id, owner, name: "Gallant Fowlknight".into(), mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Kithkin, SubType::Knight],
        power: Some(3), toughness: Some(4), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Gallant Fowlknight enters, creatures you control get +1/+0 until end of turn. Kithkin you control also gain first strike until end of turn.",
                vec![Effect::boost_all_eot("creature you control", 1, 0), Effect::grant_keyword_all_eot("Kithkin you control", "first_strike")],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn gangly_stompling(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/2 Shapeshifter for {2}{R/G}. Changeling, trample.
    CardData { id, owner, name: "Gangly Stompling".into(), mana_cost: ManaCost::parse("{2}{R/G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Shapeshifter],
        power: Some(4), toughness: Some(2),
        keywords: KeywordAbilities::CHANGELING | KeywordAbilities::TRAMPLE,
        rarity: Rarity::Common, ..Default::default() }
}

fn glamermite(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Faerie Rogue for {2}{U}. Flash, flying. (ETB: tap or untap creature)
    CardData { id, owner, name: "Glamermite".into(), mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Faerie, SubType::Rogue],
        power: Some(2), toughness: Some(2),
        keywords: KeywordAbilities::FLASH | KeywordAbilities::FLYING,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Glamermite enters, choose one — • Tap target creature. • Untap target creature.",
                vec![Effect::modal(vec![ModalMode::new("Tap target creature", vec![Effect::TapTarget]), ModalMode::new("Untap target creature", vec![Effect::UntapTarget])], 1, 1)],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn glister_bairn(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/4 Ouphe for {2}{G/U}{G/U}{G/U}. (Vivid: begin combat, creature gets +X/+X)
    CardData { id, owner, name: "Glister Bairn".into(), mana_cost: ManaCost::parse("{2}{G/U}{G/U}{G/U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Ouphe],
        power: Some(1), toughness: Some(4), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                "Vivid — At the beginning of combat on your turn, target creature you control gets +X/+X until end of turn, where X is the number of colors among permanents you control.",
                vec![EventType::BeginCombat],
                vec![Effect::boost_until_eot_vivid()],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn gnarlbark_elm(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/4 Treefolk Warlock for {2}{B}. (ETB with 2 -1/-1 counters; remove 2: target -2/-2)
    CardData { id, owner, name: "Gnarlbark Elm".into(), mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Treefolk, SubType::Warlock],
        power: Some(3), toughness: Some(4), rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "Gnarlbark Elm enters with two -1/-1 counters on it.",
                vec![StaticEffect::enters_with_counters("-1/-1", 2)]),
            Ability::activated(id,
                "Remove two -1/-1 counters from Gnarlbark Elm: Target creature gets -2/-2 until end of turn.",
                vec![Cost::RemoveCounters("-1/-1".into(), 2)],
                vec![Effect::boost_until_eot(-2, -2)],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn goldmeadow_nomad(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/2 Kithkin Scout for {W}. (From graveyard: create 1/1 Kithkin token)
    CardData { id, owner, name: "Goldmeadow Nomad".into(), mana_cost: ManaCost::parse("{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Kithkin, SubType::Scout],
        power: Some(1), toughness: Some(2), rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{3}{W}, Exile Goldmeadow Nomad from your graveyard: Create a 1/1 green and white Kithkin creature token.",
                vec![Cost::pay_mana("{3}{W}"), Cost::ExileFromGraveyard(1)],
                vec![Effect::create_token("1/1 Kithkin", 1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn graveshifter(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Shapeshifter for {3}{B}. Changeling. (ETB: return creature from graveyard)
    CardData { id, owner, name: "Graveshifter".into(), mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Shapeshifter],
        power: Some(2), toughness: Some(2), keywords: KeywordAbilities::CHANGELING,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Graveshifter enters, you may return target creature card from your graveyard to your hand.",
                vec![Effect::return_from_graveyard()],
                TargetSpec::CardInYourGraveyard).set_optional(),
        ],
        ..Default::default() }
}

fn great_forest_druid(id: ObjectId, owner: PlayerId) -> CardData {
    // 0/4 Treefolk Druid for {1}{G}. (T: add any color)
    CardData { id, owner, name: "Great Forest Druid".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Treefolk, SubType::Druid],
        power: Some(0), toughness: Some(4), rarity: Rarity::Common,
        abilities: vec![
            Ability::mana_ability(id, "{T}: Add one mana of any color.", Mana::green(1)),
        ],
        ..Default::default() }
}

fn gristle_glutton(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/3 Goblin Scout for {1}{R}. (T, blight 1: loot)
    CardData { id, owner, name: "Gristle Glutton".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Scout],
        power: Some(1), toughness: Some(3), rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{T}, Blight 1: Draw a card, then discard a card.",
                vec![Cost::tap_self(), Cost::Blight(1)],
                vec![Effect::draw_cards(1), Effect::discard_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn gutsplitter_gang(id: ObjectId, owner: PlayerId) -> CardData {
    // 6/6 Goblin Berserker for {3}{B}. (Main phase: blight 2 or lose 3 life)
    CardData { id, owner, name: "Gutsplitter Gang".into(), mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Berserker],
        power: Some(6), toughness: Some(6), rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "At the beginning of your precombat main phase, put two -1/-1 counters on Gutsplitter Gang unless you pay 3 life.",
                vec![EventType::PrecombatMainPre],
                vec![Effect::do_if_cost_paid(Cost::Blight(2), vec![], vec![Effect::LoseLife { amount: 3 }])],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn heirloom_auntie(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/4 Goblin Warlock for {2}{B}. (ETB with 2 -1/-1 counters; creature dies: surveil 1 + remove counter)
    CardData { id, owner, name: "Heirloom Auntie".into(), mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Warlock],
        power: Some(4), toughness: Some(4), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                "Heirloom Auntie enters with two -1/-1 counters on it.",
                vec![StaticEffect::enters_with_counters("-1/-1", 2)]),
            Ability::any_creature_dies_triggered(id,
                "Whenever another creature you control dies, surveil 1 and remove a -1/-1 counter from Heirloom Auntie.",
                vec![Effect::scry(1), Effect::RemoveCounters { counter_type: "-1/-1".into(), count: 1 }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn iron_shield_elf(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/1 Elf Warrior for {1}{B}. (Discard: indestructible + tap)
    CardData { id, owner, name: "Iron-Shield Elf".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elf, SubType::Warrior],
        power: Some(3), toughness: Some(1), rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "Discard a card: Iron-Shield Elf gains indestructible until end of turn. Tap it.",
                vec![Cost::Discard(1)],
                vec![Effect::gain_keyword_eot("indestructible"), Effect::tap_self()],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn kinsbaile_aspirant(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/1 Kithkin Citizen for {W}. (Behold Kithkin or pay 2; creature ETB: +1/+1)
    CardData { id, owner, name: "Kinsbaile Aspirant".into(), mana_cost: ManaCost::parse("{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Kithkin, SubType::Citizen],
        power: Some(2), toughness: Some(1), rarity: Rarity::Common,
        abilities: vec![
            Ability::other_creature_etb_triggered(id,
                "Whenever another creature enters under your control, put a +1/+1 counter on Kinsbaile Aspirant.",
                vec![Effect::add_counters("+1/+1", 1)],
                TargetSpec::None),
        ],
        additional_costs: vec![Cost::behold_or_pay("Kithkin", "{2}")],
        ..Default::default() }
}

fn kulrath_mystic(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/4 Elemental Wizard for {2}{U}. (Cast MV>=4: +2/+0 and vigilance)
    CardData { id, owner, name: "Kulrath Mystic".into(), mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elemental, SubType::Wizard],
        power: Some(2), toughness: Some(4), rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever you cast a spell with mana value 4 or greater, Kulrath Mystic gets +2/+0 and gains vigilance until end of turn.",
                vec![EventType::SpellCast],
                vec![Effect::boost_until_eot(2, 0), Effect::gain_keyword_eot("vigilance")],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn kulrath_zealot(id: ObjectId, owner: PlayerId) -> CardData {
    // 6/5 Elemental Warrior for {5}{R}. (ETB: exile top card, play until next end; landcycling)
    CardData { id, owner, name: "Kulrath Zealot".into(), mana_cost: ManaCost::parse("{5}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elemental, SubType::Warrior],
        power: Some(6), toughness: Some(5), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Kulrath Zealot enters, exile the top card of your library. You may play it until the end of your next turn.",
                vec![Effect::exile_top_and_play_next_turn(1)],
                TargetSpec::None),
            Ability::activated(id,
                "Basic landcycling {2}",
                vec![Cost::pay_mana("{2}"), Cost::Discard(1)],
                vec![Effect::search_library("basic land card")],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn luminollusk(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/4 Elemental for {3}{G}. Deathtouch. (Vivid: ETB gain life = colors among permanents)
    CardData { id, owner, name: "Luminollusk".into(), mana_cost: ManaCost::parse("{3}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elemental],
        power: Some(2), toughness: Some(4), keywords: KeywordAbilities::DEATHTOUCH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "Vivid — When Luminollusk enters, you gain X life, where X is the number of colors among permanents you control.",
                vec![Effect::gain_life_vivid()],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn lys_alana_informant(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/1 Elf Scout for {1}{G}. (ETB/dies: surveil 1)
    CardData { id, owner, name: "Lys Alana Informant".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elf, SubType::Scout],
        power: Some(3), toughness: Some(1), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Lys Alana Informant enters, surveil 1.",
                vec![Effect::scry(1)],
                TargetSpec::None),
            Ability::dies_triggered(id,
                "When Lys Alana Informant dies, surveil 1.",
                vec![Effect::scry(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn merrow_skyswimmer(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Merfolk Soldier for {3}{W/U}{W/U}. Convoke, flying, vigilance. (ETB: create Merfolk token)
    CardData { id, owner, name: "Merrow Skyswimmer".into(), mana_cost: ManaCost::parse("{3}{W/U}{W/U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Merfolk, SubType::Soldier],
        power: Some(2), toughness: Some(2),
        keywords: KeywordAbilities::CONVOKE | KeywordAbilities::FLYING | KeywordAbilities::VIGILANCE,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Merrow Skyswimmer enters, create a 1/1 white and blue Merfolk creature token.",
                vec![Effect::create_token("1/1 Merfolk", 1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn mischievous_sneakling(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Shapeshifter for {1}{U/B}. Changeling, flash.
    CardData { id, owner, name: "Mischievous Sneakling".into(), mana_cost: ManaCost::parse("{1}{U/B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Shapeshifter],
        power: Some(2), toughness: Some(2),
        keywords: KeywordAbilities::CHANGELING | KeywordAbilities::FLASH,
        rarity: Rarity::Common, ..Default::default() }
}

fn moonglove_extractor(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/1 Elf Warlock for {2}{B}. (Attacks: draw a card, lose 1 life)
    CardData { id, owner, name: "Moonglove Extractor".into(), mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elf, SubType::Warlock],
        power: Some(2), toughness: Some(1), rarity: Rarity::Common,
        abilities: vec![
            Ability::attacks_triggered(id,
                "Whenever Moonglove Extractor attacks, you draw a card and lose 1 life.",
                vec![Effect::draw_cards(1), Effect::lose_life(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn moonlit_lamenter(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/5 Treefolk Cleric for {2}{W}. (ETB with -1/-1 counter; remove counter: draw, sorcery)
    CardData { id, owner, name: "Moonlit Lamenter".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Treefolk, SubType::Cleric],
        power: Some(2), toughness: Some(5), rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "Moonlit Lamenter enters with a -1/-1 counter on it.",
                vec![StaticEffect::enters_with_counters("-1/-1", 1)]),
            Ability::activated(id,
                "Remove a -1/-1 counter from Moonlit Lamenter: Draw a card. Activate only as a sorcery.",
                vec![Cost::RemoveCounters("-1/-1".into(), 1)],
                vec![Effect::draw_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn mutable_explorer(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/1 Shapeshifter for {2}{G}. Changeling. ETB: create a tapped Mutavault land token.
    CardData { id, owner, name: "Mutable Explorer".into(), mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Shapeshifter],
        power: Some(1), toughness: Some(1), keywords: KeywordAbilities::CHANGELING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Mutable Explorer enters, create a tapped colorless land token named Mutavault.",
                vec![Effect::create_token("Mutavault", 1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn nightmare_sower(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/3 Faerie Assassin for {3}{B}. Flying, lifelink. (Opponent's turn spell: -1/-1 counter)
    CardData { id, owner, name: "Nightmare Sower".into(), mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Faerie, SubType::Assassin],
        power: Some(2), toughness: Some(3),
        keywords: KeywordAbilities::FLYING | KeywordAbilities::LIFELINK,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                "Whenever an opponent casts a spell during your turn, put a -1/-1 counter on target creature that player controls.",
                vec![EventType::SpellCast],
                vec![Effect::add_counters("-1/-1", 1)],
                TargetSpec::PermanentFiltered("creature an opponent controls".into())),
        ],
        ..Default::default() }
}

fn noggle_robber(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/3 Noggle Rogue for {1}{R/G}{R/G}. (ETB/dies: create Treasure)
    CardData { id, owner, name: "Noggle Robber".into(), mana_cost: ManaCost::parse("{1}{R/G}{R/G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Noggle, SubType::Rogue],
        power: Some(3), toughness: Some(3), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Noggle Robber enters, create a Treasure token.",
                vec![Effect::create_token("Treasure", 1)],
                TargetSpec::None),
            Ability::dies_triggered(id,
                "When Noggle Robber dies, create a Treasure token.",
                vec![Effect::create_token("Treasure", 1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn pestered_wellguard(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/2 Merfolk Soldier for {3}{U}. (Becomes tapped: create 1/1 Faerie token)
    CardData { id, owner, name: "Pestered Wellguard".into(), mana_cost: ManaCost::parse("{3}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Merfolk, SubType::Soldier],
        power: Some(3), toughness: Some(2), rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever Pestered Wellguard becomes tapped, create a 1/1 blue Faerie creature token with flying.",
                vec![EventType::Tapped],
                vec![Effect::create_token("1/1 Faerie with flying", 1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn prideful_feastling(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/3 Shapeshifter for {2}{W/B}. Changeling, lifelink.
    CardData { id, owner, name: "Prideful Feastling".into(), mana_cost: ManaCost::parse("{2}{W/B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Shapeshifter],
        power: Some(2), toughness: Some(3),
        keywords: KeywordAbilities::CHANGELING | KeywordAbilities::LIFELINK,
        rarity: Rarity::Common, ..Default::default() }
}

fn prismabasher(id: ObjectId, owner: PlayerId) -> CardData {
    // 6/6 Elemental for {4}{G}{G}. Trample. (Vivid: ETB creatures get +X/+X)
    CardData { id, owner, name: "Prismabasher".into(), mana_cost: ManaCost::parse("{4}{G}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elemental],
        power: Some(6), toughness: Some(6), keywords: KeywordAbilities::TRAMPLE,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "Vivid — When Prismabasher enters, other creatures you control get +X/+X until end of turn, where X is the number of colors among permanents you control.",
                vec![Effect::boost_all_until_eot_vivid()],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn reluctant_dounguard(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/4 Kithkin Soldier for {2}{W}. (ETB with 2 -1/-1 counters; creature ETB: remove counter)
    CardData { id, owner, name: "Reluctant Dounguard".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Kithkin, SubType::Soldier],
        power: Some(4), toughness: Some(4), rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "Reluctant Dounguard enters with two -1/-1 counters on it.",
                vec![StaticEffect::enters_with_counters("-1/-1", 2)]),
            Ability::other_creature_etb_triggered(id,
                "Whenever another creature enters under your control, remove a -1/-1 counter from Reluctant Dounguard.",
                vec![Effect::RemoveCounters { counter_type: "-1/-1".into(), count: 1 }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn rimekin_recluse(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/2 Elemental Wizard for {2}{U}. (ETB: bounce another creature)
    CardData { id, owner, name: "Rimekin Recluse".into(), mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elemental, SubType::Wizard],
        power: Some(3), toughness: Some(2), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Rimekin Recluse enters, return up to one other creature to its owner's hand.",
                vec![Effect::bounce()],
                TargetSpec::Creature).set_optional(),
        ],
        ..Default::default() }
}

fn rooftop_percher(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/3 Shapeshifter for {5}. Changeling, flying. (ETB: exile from graveyards, gain 3 life)
    CardData { id, owner, name: "Rooftop Percher".into(), mana_cost: ManaCost::parse("{5}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Shapeshifter],
        power: Some(3), toughness: Some(3),
        keywords: KeywordAbilities::CHANGELING | KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Rooftop Percher enters, exile up to two target cards from graveyards. You gain 3 life.",
                vec![Effect::exile_from_graveyards(2), Effect::gain_life(3)],
                TargetSpec::CardInGraveyard),
        ],
        ..Default::default() }
}

fn safewright_cavalry(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/4 Elf Warrior for {3}{G}. (Can't be blocked by more than one; {5}: Elf +2/+2)
    CardData { id, owner, name: "Safewright Cavalry".into(), mana_cost: ManaCost::parse("{3}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elf, SubType::Warrior],
        power: Some(4), toughness: Some(4), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                "Safewright Cavalry can't be blocked by more than one creature.",
                vec![StaticEffect::CantBeBlockedByMoreThan { count: 1 }]),
            Ability::activated(id,
                "{5}: Target Elf you control gets +2/+2 until end of turn.",
                vec![Cost::pay_mana("{5}")],
                vec![Effect::boost_until_eot(2, 2)],
                TargetSpec::PermanentFiltered("Elf you control".into())),
        ],
        ..Default::default() }
}

fn scarblade_scout(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Elf Scout for {1}{B}. Lifelink. (ETB: mill 2)
    CardData { id, owner, name: "Scarblade Scout".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elf, SubType::Scout],
        power: Some(2), toughness: Some(2), keywords: KeywordAbilities::LIFELINK,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Scarblade Scout enters, mill two cards.",
                vec![Effect::mill(2)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn scuzzback_scrounger(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/2 Goblin Warrior for {1}{R}. (Main phase: blight 1 => create Treasure)
    CardData { id, owner, name: "Scuzzback Scrounger".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Warrior],
        power: Some(3), toughness: Some(2), rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "At the beginning of your precombat main phase, you may put a -1/-1 counter on Scuzzback Scrounger. If you do, create a Treasure token.",
                vec![EventType::PrecombatMainPre],
                vec![Effect::do_if_cost_paid(Cost::Blight(1), vec![Effect::create_token("Treasure token", 1)], vec![])],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn shimmercreep(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/5 Elemental for {4}{B}. Menace. (Vivid: ETB opponents lose X, you gain X)
    CardData { id, owner, name: "Shimmercreep".into(), mana_cost: ManaCost::parse("{4}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elemental],
        power: Some(3), toughness: Some(5), keywords: KeywordAbilities::MENACE,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "Vivid — When Shimmercreep enters, each opponent loses X life and you gain X life, where X is the number of colors among permanents you control.",
                vec![Effect::lose_life_opponents_vivid(), Effect::gain_life_vivid()],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn shinestriker(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/3 Elemental for {4}{U}{U}. Flying. (Vivid: ETB draw X cards)
    CardData { id, owner, name: "Shinestriker".into(), mana_cost: ManaCost::parse("{4}{U}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elemental],
        power: Some(3), toughness: Some(3), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "Vivid — When Shinestriker enters, draw X cards, where X is the number of colors among permanents you control.",
                vec![Effect::draw_cards_vivid()],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn shore_lurker(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/3 Merfolk Scout for {3}{W}. Flying. (ETB: surveil 1)
    CardData { id, owner, name: "Shore Lurker".into(), mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Merfolk, SubType::Scout],
        power: Some(3), toughness: Some(3), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Shore Lurker enters, surveil 1.",
                vec![Effect::scry(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn silvergill_mentor(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/1 Merfolk Wizard for {1}{U}. (Behold Merfolk or pay 2; ETB: create Merfolk token)
    CardData { id, owner, name: "Silvergill Mentor".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Merfolk, SubType::Wizard],
        power: Some(2), toughness: Some(1), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Silvergill Mentor enters, create a 1/1 white and blue Merfolk creature token.",
                vec![Effect::create_token("1/1 Merfolk", 1)],
                TargetSpec::None),
        ],
        additional_costs: vec![Cost::behold_or_pay("Merfolk", "{2}")],
        ..Default::default() }
}

fn silvergill_peddler(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/3 Merfolk Citizen for {2}{U}. (Becomes tapped: draw, discard)
    CardData { id, owner, name: "Silvergill Peddler".into(), mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Merfolk, SubType::Citizen],
        power: Some(2), toughness: Some(3), rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever Silvergill Peddler becomes tapped, draw a card, then discard a card.",
                vec![EventType::Tapped],
                vec![Effect::draw_cards(1), Effect::discard_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn sizzling_changeling(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/2 Shapeshifter for {2}{R}. Changeling. (Dies: exile top, play until next end step)
    CardData { id, owner, name: "Sizzling Changeling".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Shapeshifter],
        power: Some(3), toughness: Some(2), keywords: KeywordAbilities::CHANGELING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::dies_triggered(id,
                "When Sizzling Changeling dies, exile the top card of your library. You may play it until the end of your next turn.",
                vec![Effect::exile_top_and_play_next_turn(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn sourbread_auntie(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/3 Goblin Warrior for {2}{R}{R}. (ETB: blight 2 => create two Goblin tokens)
    CardData { id, owner, name: "Sourbread Auntie".into(), mana_cost: ManaCost::parse("{2}{R}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Warrior],
        power: Some(4), toughness: Some(3), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Sourbread Auntie enters, put two -1/-1 counters on it and create two 1/1 black and red Goblin creature tokens.",
                vec![Effect::add_counters("-1/-1", 2), Effect::create_token("1/1 Goblin", 2)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn squawkroaster(id: ObjectId, owner: PlayerId) -> CardData {
    // */4 Elemental for {3}{R}. Double strike. (Vivid: power = colors among permanents)
    CardData { id, owner, name: "Squawkroaster".into(), mana_cost: ManaCost::parse("{3}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elemental],
        power: Some(0), toughness: Some(4), keywords: KeywordAbilities::DOUBLE_STRIKE,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "Vivid — Squawkroaster's power is equal to the number of colors among permanents you control.",
                vec![StaticEffect::set_power_to_color_count()]),
        ],
        ..Default::default() }
}

fn sting_slinger(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/3 Goblin Warrior for {2}{R}. ({1}{R}, T, blight 1: 2 damage to opponents)
    CardData { id, owner, name: "Sting-Slinger".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Warrior],
        power: Some(3), toughness: Some(3), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                "{1}{R}, {T}, Put a -1/-1 counter on Sting-Slinger: It deals 2 damage to each opponent.",
                vec![Cost::pay_mana("{1}{R}"), Cost::tap_self()],
                vec![Effect::add_counters("-1/-1", 1), Effect::damage_opponents(2)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn stoic_grove_guide(id: ObjectId, owner: PlayerId) -> CardData {
    // 5/4 Elf Druid for {4}{B/G}. (From graveyard: {1}{B/G}, exile self: create 2/2 Elf token, sorcery)
    CardData { id, owner, name: "Stoic Grove-Guide".into(), mana_cost: ManaCost::parse("{4}{B/G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elf, SubType::Druid],
        power: Some(5), toughness: Some(4), rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{1}{B/G}, Exile Stoic Grove-Guide from your graveyard: Create a 2/2 black and green Elf creature token. Activate only as a sorcery.",
                vec![Cost::pay_mana("{1}{B/G}"), Cost::ExileFromGraveyard(1)],
                vec![Effect::create_token("2/2 Elf Warrior", 1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn stratosoarer(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/5 Elemental for {4}{U}. Flying. (ETB: give flying to creature; landcycling {2})
    CardData { id, owner, name: "Stratosoarer".into(), mana_cost: ManaCost::parse("{4}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elemental],
        power: Some(3), toughness: Some(5), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Stratosoarer enters, target creature gains flying until end of turn.",
                vec![Effect::gain_keyword_eot("flying")],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn summit_sentinel(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/3 Elemental Soldier for {1}{U}. (Dies: draw a card)
    CardData { id, owner, name: "Summit Sentinel".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elemental, SubType::Soldier],
        power: Some(1), toughness: Some(3), rarity: Rarity::Common,
        abilities: vec![
            Ability::dies_triggered(id,
                "When Summit Sentinel dies, draw a card.",
                vec![Effect::draw_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn sun_dappled_celebrant(id: ObjectId, owner: PlayerId) -> CardData {
    // 5/6 Treefolk Cleric for {4}{W}{W}. Convoke, vigilance.
    CardData { id, owner, name: "Sun-Dappled Celebrant".into(), mana_cost: ManaCost::parse("{4}{W}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Treefolk, SubType::Cleric],
        power: Some(5), toughness: Some(6),
        keywords: KeywordAbilities::CONVOKE | KeywordAbilities::VIGILANCE,
        rarity: Rarity::Common, ..Default::default() }
}

fn surly_farrier(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Kithkin Citizen for {1}{G}. (T: target creature +1/+1 and vigilance, sorcery only)
    CardData { id, owner, name: "Surly Farrier".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Kithkin, SubType::Citizen],
        power: Some(2), toughness: Some(2), rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{T}: Target creature gets +1/+1 and gains vigilance until end of turn. Activate only as a sorcery.",
                vec![Cost::tap_self()],
                vec![Effect::boost_until_eot(1, 1), Effect::gain_keyword_eot("vigilance")],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn tanufel_rimespeaker(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/4 Elemental Wizard for {3}{U}. (Cast MV>=4: draw a card)
    CardData { id, owner, name: "Tanufel Rimespeaker".into(), mana_cost: ManaCost::parse("{3}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elemental, SubType::Wizard],
        power: Some(2), toughness: Some(4), rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever you cast a spell with mana value 4 or greater, draw a card.",
                vec![EventType::SpellCast],
                vec![Effect::draw_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn thoughtweft_imbuer(id: ObjectId, owner: PlayerId) -> CardData {
    // 0/5 Kithkin Advisor for {3}{W}. (Creature attacks alone: +X/+X where X = Kithkin count)
    CardData { id, owner, name: "Thoughtweft Imbuer".into(), mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Kithkin, SubType::Advisor],
        power: Some(0), toughness: Some(5), rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever a creature you control attacks alone, it gets +X/+X until end of turn, where X is the number of Kithkin you control.",
                vec![EventType::AttackerDeclared],
                vec![Effect::boost_target_dynamic("Kithkin you control")],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn thoughtweft_lieutenant(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Kithkin Soldier for {G}{W}. (Kithkin ETB: target creature +1/+1 + trample)
    CardData { id, owner, name: "Thoughtweft Lieutenant".into(), mana_cost: ManaCost::parse("{G}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Kithkin, SubType::Soldier],
        power: Some(2), toughness: Some(2), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                "Whenever another Kithkin enters under your control, target creature you control gets +1/+1 and gains trample until end of turn.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::boost_until_eot(1, 1), Effect::gain_keyword_eot("trample")],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn timid_shieldbearer(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Kithkin Soldier for {1}{W}. ({4}{W}: creatures +1/+1 until end of turn)
    CardData { id, owner, name: "Timid Shieldbearer".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Kithkin, SubType::Soldier],
        power: Some(2), toughness: Some(2), rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{4}{W}: Creatures you control get +1/+1 until end of turn.",
                vec![Cost::pay_mana("{4}{W}")],
                vec![Effect::boost_all_eot("creatures you control", 1, 1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn unwelcome_sprite(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/1 Faerie Rogue for {1}{U}. Flying. (Opponent's turn spell: surveil 2)
    CardData { id, owner, name: "Unwelcome Sprite".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Faerie, SubType::Rogue],
        power: Some(2), toughness: Some(1), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever an opponent casts a spell during your turn, surveil 2.",
                vec![EventType::SpellCast],
                vec![Effect::scry(2)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn virulent_emissary(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/1 Elf Assassin for {G}. Deathtouch. (Creature ETB: gain 1 life)
    CardData { id, owner, name: "Virulent Emissary".into(), mana_cost: ManaCost::parse("{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elf, SubType::Assassin],
        power: Some(1), toughness: Some(1), keywords: KeywordAbilities::DEATHTOUCH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::other_creature_etb_triggered(id,
                "Whenever another creature enters under your control, you gain 1 life.",
                vec![Effect::gain_life(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn voracious_tome_skimmer(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/3 Faerie Rogue for {U/B}{U/B}{U/B}. Flying. (Opponent's turn spell: pay 1 life => draw)
    CardData { id, owner, name: "Voracious Tome-Skimmer".into(), mana_cost: ManaCost::parse("{U/B}{U/B}{U/B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Faerie, SubType::Rogue],
        power: Some(2), toughness: Some(3), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                "Whenever an opponent casts a spell during your turn, you may pay 1 life. If you do, draw a card.",
                vec![EventType::SpellCast],
                vec![Effect::do_if_cost_paid(Cost::PayLife(1), vec![Effect::draw_cards(1)], vec![])],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn wanderbrine_preacher(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Merfolk Cleric for {1}{W}. (Becomes tapped: gain 2 life)
    CardData { id, owner, name: "Wanderbrine Preacher".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Merfolk, SubType::Cleric],
        power: Some(2), toughness: Some(2), rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever Wanderbrine Preacher becomes tapped, you gain 2 life.",
                vec![EventType::Tapped],
                vec![Effect::gain_life(2)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn wanderwine_distracter(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/3 Merfolk Wizard for {3}{U}. (Becomes tapped: creature gets -3/-0)
    CardData { id, owner, name: "Wanderwine Distracter".into(), mana_cost: ManaCost::parse("{3}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Merfolk, SubType::Wizard],
        power: Some(4), toughness: Some(3), rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever Wanderwine Distracter becomes tapped, target creature an opponent controls gets -3/-0 until end of turn.",
                vec![EventType::Tapped],
                vec![Effect::boost_until_eot(-3, 0)],
                TargetSpec::PermanentFiltered("creature an opponent controls".into())),
        ],
        ..Default::default() }
}

fn warren_torchmaster(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Goblin Warrior for {1}{R}. (Begin combat: blight 1 => creature gains haste)
    CardData { id, owner, name: "Warren Torchmaster".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Warrior],
        power: Some(2), toughness: Some(2), rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "At the beginning of combat on your turn, you may put a -1/-1 counter on Warren Torchmaster. If you do, target creature gains haste until end of turn.",
                vec![EventType::BeginCombat],
                vec![Effect::add_counters_self("-1/-1", 1), Effect::gain_keyword_eot("haste")],
                TargetSpec::Creature).set_optional(),
        ],
        ..Default::default() }
}

// ── Spells and non-creature permanents ───────────────────────────────────────

fn appeal_to_eirdu(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {3}{W}. Convoke. +2/+1 to up to two creatures.
    CardData { id, owner, name: "Appeal to Eirdu".into(), mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Instant], keywords: KeywordAbilities::CONVOKE,
        rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::boost_until_eot(2, 1)], TargetSpec::Multiple { spec: Box::new(TargetSpec::Creature), count: 2 })],
        ..Default::default() }
}

fn burning_curiosity(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {2}{R}. Exile top cards, play until next end step.
    CardData { id, owner, name: "Burning Curiosity".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![Ability::spell(id,
            vec![Effect::exile_top_and_play_next_turn(3)],
            TargetSpec::None)],
        ..Default::default() }
}

fn cinder_strike(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {R}. 2 damage to creature (4 if blighted).
    CardData { id, owner, name: "Cinder Strike".into(), mana_cost: ManaCost::parse("{R}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::deal_damage(2)], TargetSpec::Creature)],
        ..Default::default() }
}

fn crib_swap(id: ObjectId, owner: PlayerId) -> CardData {
    // Kindred Instant {2}{W} — Shapeshifter. Changeling. Exile creature, give 1/1 token.
    CardData { id, owner, name: "Crib Swap".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Kindred, CardType::Instant],
        subtypes: vec![SubType::Shapeshifter],
        keywords: KeywordAbilities::CHANGELING,
        rarity: Rarity::Uncommon,
        abilities: vec![Ability::spell(id, vec![Effect::exile(), Effect::target_controller_creates_token("1/1 colorless Shapeshifter creature token with changeling")], TargetSpec::Creature)],
        ..Default::default() }
}

fn liminal_hold(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment {3}{W}. ETB: exile nonland permanent, gain 2 life.
    CardData { id, owner, name: "Liminal Hold".into(), mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Liminal Hold enters, exile target nonland permanent an opponent controls and you gain 2 life.",
                vec![Effect::exile(), Effect::gain_life(2)],
                TargetSpec::PermanentFiltered("nonland permanent an opponent controls".into())),
        ],
        ..Default::default() }
}

fn protective_response(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {2}{W}. Convoke. Destroy attacking or blocking creature.
    CardData { id, owner, name: "Protective Response".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Instant], keywords: KeywordAbilities::CONVOKE,
        rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::destroy()], TargetSpec::PermanentFiltered("attacking or blocking creature".into()))],
        ..Default::default() }
}

fn springleaf_drum(id: ObjectId, owner: PlayerId) -> CardData {
    // Artifact {1}. T, tap creature: add any color.
    CardData { id, owner, name: "Springleaf Drum".into(), mana_cost: ManaCost::parse("{1}"),
        card_types: vec![CardType::Artifact],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::mana_ability(id,
                "{T}, Tap an untapped creature you control: Add one mana of any color.",
                Mana::green(1)),
        ],
        ..Default::default() }
}

fn temporal_cleansing(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {3}{U}. Convoke. Put nonland permanent on top of library (simplified from 2nd-from-top or bottom choice).
    CardData { id, owner, name: "Temporal Cleansing".into(), mana_cost: ManaCost::parse("{3}{U}"),
        card_types: vec![CardType::Sorcery], keywords: KeywordAbilities::CONVOKE,
        rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::put_on_library()], TargetSpec::PermanentFiltered("nonland permanent".into()))],
        ..Default::default() }
}

fn unexpected_assistance(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {3}{U}{U}. Convoke. Draw 3, discard 1.
    CardData { id, owner, name: "Unexpected Assistance".into(), mana_cost: ManaCost::parse("{3}{U}{U}"),
        card_types: vec![CardType::Instant], keywords: KeywordAbilities::CONVOKE,
        rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::draw_cards(3), Effect::discard_cards(1)], TargetSpec::None)],
        ..Default::default() }
}

// ── Tier 2 additional spells ──────────────────────────────────────────────────

fn assert_perfection(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {1}{G}. Target creature +1/+0, then fights opponent's creature.
    CardData { id, owner, name: "Assert Perfection".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::boost_until_eot(1, 0), Effect::bite()], TargetSpec::fight_targets())],
        ..Default::default() }
}

fn blight_rot(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {2}{B}. Put four -1/-1 counters on target creature.
    CardData { id, owner, name: "Blight Rot".into(), mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::add_counters("-1/-1", 4)], TargetSpec::Creature)],
        ..Default::default() }
}

fn blossoming_defense(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {G}. Target creature you control +2/+2 and hexproof until EOT.
    CardData { id, owner, name: "Blossoming Defense".into(), mana_cost: ManaCost::parse("{G}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Uncommon,
        abilities: vec![Ability::spell(id, vec![Effect::boost_until_eot(2, 2), Effect::hexproof()], TargetSpec::Creature)],
        ..Default::default() }
}

fn boggart_mischief(id: ObjectId, owner: PlayerId) -> CardData {
    // Kindred Enchantment {2}{B} — Goblin. ETB: blight 1 => create Goblin tokens.
    // Goblin dies: opponents lose 1 life, you gain 1 life.
    CardData { id, owner, name: "Boggart Mischief".into(), mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Kindred, CardType::Enchantment],
        subtypes: vec![SubType::Goblin],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Boggart Mischief enters, you may put a -1/-1 counter on a creature you control. If you do, create two 1/1 black Goblin Rogue creature tokens.",
                vec![Effect::do_if_cost_paid(Cost::Blight(1), vec![Effect::create_token("1/1 black Goblin Rogue creature token", 2)], vec![])],
                TargetSpec::None),
            Ability::any_creature_dies_triggered(id,
                "Whenever a Goblin you control dies, each opponent loses 1 life and you gain 1 life.",
                vec![Effect::lose_life_opponents(1), Effect::gain_life(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn darkness_descends(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {2}{B}{B}. Put two -1/-1 counters on each creature.
    CardData { id, owner, name: "Darkness Descends".into(), mana_cost: ManaCost::parse("{2}{B}{B}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Rare,
        abilities: vec![Ability::spell(id, vec![Effect::add_counters_all("-1/-1", 2, "creatures")], TargetSpec::None)],
        ..Default::default() }
}

fn dose_of_dawnglow(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {4}{B}. Return target creature from your graveyard to battlefield.
    CardData { id, owner, name: "Dose of Dawnglow".into(), mana_cost: ManaCost::parse("{4}{B}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::reanimate()], TargetSpec::CardInYourGraveyard)],
        ..Default::default() }
}

fn feed_the_flames(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {3}{R}. 5 damage to target creature. Exile if dies.
    CardData { id, owner, name: "Feed the Flames".into(), mana_cost: ManaCost::parse("{3}{R}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::deal_damage(5)], TargetSpec::Creature)],
        ..Default::default() }
}

// ENGINE DEPS: [COND] Activated ability put creature from hand (P+T<=5), haste, end-step sacrifice
fn meek_attack(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment {2}{R}. Activated: cheat small creature from hand with haste.
    CardData { id, owner, name: "Meek Attack".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Enchantment], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                "{1}{R}: You may put a creature card with mana value 2 or less from your hand onto the battlefield. It gains haste. Sacrifice it at the beginning of the next end step.",
                vec![Cost::pay_mana("{1}{R}")],
                vec![Effect::put_from_hand_with_haste_sacrifice(2)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn midnight_tilling(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{G}. Mill 4, return a permanent card to hand.
    CardData { id, owner, name: "Midnight Tilling".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::mill(4), Effect::return_from_graveyard()], TargetSpec::None)],
        ..Default::default() }
}

fn nameless_inversion(id: ObjectId, owner: PlayerId) -> CardData {
    // Kindred Instant {1}{B} — Shapeshifter. Changeling. Target creature +3/-3.
    CardData { id, owner, name: "Nameless Inversion".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Kindred, CardType::Instant],
        subtypes: vec![SubType::Shapeshifter],
        keywords: KeywordAbilities::CHANGELING,
        rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::boost_until_eot(3, -3)], TargetSpec::Creature)],
        ..Default::default() }
}

fn reckless_ransacking(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{R}. Target creature +3/+2. Create a Treasure token.
    CardData { id, owner, name: "Reckless Ransacking".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::boost_until_eot(3, 2), Effect::create_token("Treasure", 1)], TargetSpec::Creature)],
        ..Default::default() }
}

fn rime_chill(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {6}{U}. Vivid (cost reduction). Tap up to 2 creatures, stun counters, draw.
    CardData { id, owner, name: "Rime Chill".into(), mana_cost: ManaCost::parse("{6}{U}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id,
            vec![Effect::TapTarget, Effect::add_counters("stun", 1), Effect::draw_cards(1)],
            TargetSpec::Multiple { spec: Box::new(TargetSpec::Creature), count: 2 })],
        ..Default::default() }
}

fn run_away_together(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{U}. Bounce 2 creatures controlled by different players.
    CardData { id, owner, name: "Run Away Together".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::bounce(), Effect::bounce()], TargetSpec::Multiple { spec: Box::new(TargetSpec::Creature), count: 2 })],
        ..Default::default() }
}

fn scarblades_malice(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {B}. Creature gains deathtouch + lifelink. If it dies this turn, create 2/2 Elf.
    CardData { id, owner, name: "Scarblade's Malice".into(), mana_cost: ManaCost::parse("{B}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::gain_keyword_eot("deathtouch"), Effect::gain_keyword_eot("lifelink")], TargetSpec::Creature)],
        ..Default::default() }
}

fn sear(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{R}. 4 damage to target creature or planeswalker.
    CardData { id, owner, name: "Sear".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::deal_damage(4)], TargetSpec::Creature)],
        ..Default::default() }
}

fn soul_immolation(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Soul Immolation".into(), mana_cost: ManaCost::parse("{3}{R}{R}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Rare,
        additional_costs: vec![Cost::variable_blight()],
        abilities: vec![Ability::spell(id,
            vec![Effect::damage_opponents(X_VALUE),
                 Effect::damage_opponents_creatures(X_VALUE)],
            TargetSpec::None)],
        ..Default::default() }
}

fn spell_snare(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {U}. Counter target spell with mana value 2.
    CardData { id, owner, name: "Spell Snare".into(), mana_cost: ManaCost::parse("{U}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Uncommon,
        abilities: vec![Ability::spell(id, vec![Effect::counter_spell()], TargetSpec::Spell)],
        ..Default::default() }
}

fn thoughtweft_charge(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{G}. Target creature +3/+3. Conditional draw.
    CardData { id, owner, name: "Thoughtweft Charge".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::boost_until_eot(3, 3)], TargetSpec::Creature)],
        ..Default::default() }
}

fn tweeze(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {2}{R}. 3 damage to any target. May loot.
    CardData { id, owner, name: "Tweeze".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id, vec![Effect::deal_damage(3)], TargetSpec::CreatureOrPlayer)],
        ..Default::default() }
}

fn wild_unraveling(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {U}{U}. Additional cost: blight 2 or pay {1}. Counter target spell.
    CardData { id, owner, name: "Wild Unraveling".into(), mana_cost: ManaCost::parse("{U}{U}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Uncommon,
        abilities: vec![Ability::spell(id, vec![Effect::counter_spell()], TargetSpec::Spell)],
        ..Default::default() }
}

// ── Tier 3 — complex card implementations ────────────────────────────────────

// ENGINE DEPS: [COND] LoseAllAbilities effect, keyword counters (flying/first_strike/lifelink)
fn abigale_eloquent_first_year(id: ObjectId, owner: PlayerId) -> CardData {
    // Legendary 1/1 Bird Bard for {W/B}{W/B}. Flying, first strike, lifelink.
    // ETB: up to one other target creature loses all abilities, gets flying/first strike/lifelink counters.
    CardData { id, owner, name: "Abigale, Eloquent First-Year".into(), mana_cost: ManaCost::parse("{W/B}{W/B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Bird, SubType::Bard],
        supertypes: vec![SuperType::Legendary],
        power: Some(1), toughness: Some(1),
        keywords: KeywordAbilities::FLYING | KeywordAbilities::FIRST_STRIKE | KeywordAbilities::LIFELINK,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, up to one other target creature loses all abilities. Put a flying counter, a first strike counter, and a lifelink counter on that creature.",
                vec![Effect::lose_all_abilities(), Effect::add_counters("flying", 1), Effect::add_counters("first strike", 1), Effect::add_counters("lifelink", 1)],
                TargetSpec::PermanentFiltered("another creature".into())),
        ],
        ..Default::default() }
}

fn aquitects_defenses(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment — Aura for {1}{U}. Flash. Enchant creature you control.
    // ETB: enchanted creature gains hexproof until EOT. Static: +1/+2.
    CardData { id, owner, name: "Aquitect's Defenses".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Enchantment], subtypes: vec![SubType::Aura],
        keywords: KeywordAbilities::FLASH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this Aura enters, enchanted creature gains hexproof until end of turn.",
                vec![Effect::gain_keyword_eot("hexproof")],
                TargetSpec::None),
            Ability::static_ability(id,
                "Enchanted creature gets +1/+2.",
                vec![StaticEffect::boost_controlled("enchanted creature", 1, 2)]),
        ],
        ..Default::default() }
}

fn ashlings_command(id: ObjectId, owner: PlayerId) -> CardData {
    // {3}{U}{R} Kindred Instant — Elemental. Choose two of 4 modes.
    CardData { id, owner, name: "Ashling's Command".into(), mana_cost: ManaCost::parse("{3}{U}{R}"),
        card_types: vec![CardType::Kindred, CardType::Instant], subtypes: vec![SubType::Elemental],
        rarity: Rarity::Rare,
        abilities: vec![Ability::spell(id,
            vec![Effect::modal(vec![
                ModalMode::new("Create a token that's a copy of target Elemental you control.",
                    vec![Effect::create_token_copy(1)]),
                ModalMode::new("Target player draws two cards.",
                    vec![Effect::draw_cards(2)]),
                ModalMode::new("Deal 2 damage to each creature target player controls.",
                    vec![Effect::DealDamageAll { amount: 2, filter: "creature target player controls".into() }]),
                ModalMode::new("Target player creates two Treasure tokens.",
                    vec![Effect::create_token("Treasure", 2)]),
            ], 2, 2)],
            TargetSpec::Custom("various".into()))],
        ..Default::default() }
}

fn aunties_sentence(id: ObjectId, owner: PlayerId) -> CardData {
    // {1}{B} Sorcery. Choose one: opponent reveals hand, discard nonland permanent; or creature -2/-2 until EOT.
    CardData { id, owner, name: "Auntie's Sentence".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Common,
        abilities: vec![Ability::spell(id,
            vec![Effect::modal(vec![
                ModalMode::new("Target opponent reveals their hand. You choose a nonland permanent card from it. That player discards that card.",
                    vec![Effect::discard_opponents(1)]),
                ModalMode::new("Target creature gets -2/-2 until end of turn.",
                    vec![Effect::boost_until_eot(-2, -2)]),
            ], 1, 1)],
            TargetSpec::Custom("opponent or creature".into()))],
        ..Default::default() }
}

fn aurora_awakener(id: ObjectId, owner: PlayerId) -> CardData {
    // 7/7 Giant Druid for {6}{G}. Trample.
    // Vivid: ETB — reveal cards until X permanents (X = colors among your permanents), put some onto battlefield.
    CardData { id, owner, name: "Aurora Awakener".into(), mana_cost: ManaCost::parse("{6}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Giant, SubType::Druid],
        power: Some(7), toughness: Some(7), keywords: KeywordAbilities::TRAMPLE,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "Vivid — When this creature enters, reveal cards from the top of your library until you reveal X permanent cards, where X is the number of colors among permanents you control. Put any number of those permanent cards onto the battlefield, then put the rest on the bottom in a random order.",
                vec![Effect::reveal_from_library_vivid()],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn barbed_bloodletter(id: ObjectId, owner: PlayerId) -> CardData {
    // Artifact — Equipment for {1}{B}. Flash. ETB: attach to target creature, it gains wither until EOT.
    // Equipped creature gets +1/+2. Equip {2}.
    CardData { id, owner, name: "Barbed Bloodletter".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Artifact], subtypes: vec![SubType::Equipment],
        keywords: KeywordAbilities::FLASH,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this Equipment enters, attach it to target creature you control. That creature gains wither until end of turn.",
                vec![Effect::equip(), Effect::gain_keyword_eot("wither")],
                TargetSpec::Creature),
            Ability::static_ability(id,
                "Equipped creature gets +1/+2.",
                vec![StaticEffect::boost_controlled("equipped creature", 1, 2)]),
            Ability::activated(id, "Equip {2}",
                vec![Cost::pay_mana("{2}")],
                vec![Effect::equip()],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn bark_of_doran(id: ObjectId, owner: PlayerId) -> CardData {
    // Artifact — Equipment for {1}{W}. Equipped creature gets +0/+1.
    // If toughness > power, assigns combat damage equal to toughness. Equip {1}.
    CardData { id, owner, name: "Bark of Doran".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Artifact], subtypes: vec![SubType::Equipment],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                "Equipped creature gets +0/+1.",
                vec![StaticEffect::boost_controlled("equipped creature", 0, 1)]),
            Ability::static_ability(id,
                "As long as equipped creature's toughness is greater than its power, it assigns combat damage equal to its toughness rather than its power.",
                vec![StaticEffect::assign_damage_with_toughness_if_greater("equipped creature")]),
            Ability::activated(id, "Equip {1}",
                vec![Cost::pay_mana("{1}")],
                vec![Effect::equip()],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn blood_crypt(id: ObjectId, owner: PlayerId) -> CardData {
    // DONE - Land — Swamp Mountain. Pay 2 life or enters tapped. Taps for {B} or {R}.
    CardData { id, owner, name: "Blood Crypt".into(),
        card_types: vec![CardType::Land], subtypes: vec![SubType::Swamp, SubType::Mountain],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "As Blood Crypt enters, you may pay 2 life. If you don't, it enters tapped.",
                vec![StaticEffect::enters_tapped_unless("pay 2 life")]),
            Ability::mana_ability(id, "{T}: Add {B}.", Mana::black(1)),
            Ability::mana_ability(id, "{T}: Add {R}.", Mana::red(1)),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [TYPE+COND] Convoke, choose creature type, return all creatures of type from GY to battlefield
fn bloodline_bidding(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery for {6}{B}{B}. Convoke. Choose a creature type, return all of type from GY to battlefield.
    CardData { id, owner, name: "Bloodline Bidding".into(), mana_cost: ManaCost::parse("{6}{B}{B}"),
        card_types: vec![CardType::Sorcery], keywords: KeywordAbilities::CONVOKE,
        rarity: Rarity::Rare,
        abilities: vec![Ability::spell(id,
            vec![Effect::choose_type_and_return_from_graveyard()],
            TargetSpec::None)],
        ..Default::default() }
}

fn blossombind(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment — Aura for {1}{U}. Enchant creature. ETB: tap enchanted creature. It can't untap or have counters put on it.
    CardData { id, owner, name: "Blossombind".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Enchantment], subtypes: vec![SubType::Aura],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this Aura enters, tap enchanted creature.",
                vec![Effect::tap_attached()],
                TargetSpec::None),
            Ability::static_ability(id,
                "Enchanted creature can't become untapped and can't have counters put on it.",
                vec![StaticEffect::cant_untap("enchanted creature")]),
        ],
        ..Default::default() }
}

fn bre_of_clan_stoutarm(id: ObjectId, owner: PlayerId) -> CardData {
    // Legendary 4/4 Giant Warrior for {2}{R}{W}.
    // {1}{W}, {T}: Another target creature gains flying and lifelink until EOT.
    // End step (if you gained life): return creature card with MV <= life gained from graveyard to battlefield.
    CardData { id, owner, name: "Bre of Clan Stoutarm".into(), mana_cost: ManaCost::parse("{2}{R}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Giant, SubType::Warrior],
        supertypes: vec![SuperType::Legendary],
        power: Some(4), toughness: Some(4), rarity: Rarity::Mythic,
        abilities: vec![
            Ability::activated(id,
                "{1}{W}, {T}: Another target creature you control gains flying and lifelink until end of turn.",
                vec![Cost::pay_mana("{1}{W}"), Cost::tap_self()],
                vec![Effect::gain_keyword_eot("flying"), Effect::gain_keyword_eot("lifelink")],
                TargetSpec::PermanentFiltered("another creature you control".into())),
            Ability::triggered(id,
                "At the beginning of each end step, if you gained life this turn, return target creature card with mana value X or less from your graveyard to the battlefield, where X is the amount of life you gained this turn.",
                vec![EventType::EndStep],
                vec![Effect::reanimate()],
                TargetSpec::CardInYourGraveyard),
        ],
        ..Default::default() }
}

fn brigids_command(id: ObjectId, owner: PlayerId) -> CardData {
    // {1}{G}{W} Kindred Sorcery — Kithkin. Choose two of 4 modes.
    CardData { id, owner, name: "Brigid's Command".into(), mana_cost: ManaCost::parse("{1}{G}{W}"),
        card_types: vec![CardType::Kindred, CardType::Sorcery],
        subtypes: vec![SubType::Kithkin],
        rarity: Rarity::Rare,
        abilities: vec![Ability::spell(id,
            vec![Effect::modal(vec![
                ModalMode::new("Create a token that's a copy of target Kithkin you control.",
                    vec![Effect::create_token_copy(1)]),
                ModalMode::new("Target player creates a 1/1 green and white Kithkin creature token.",
                    vec![Effect::create_token("1/1 Kithkin", 1)]),
                ModalMode::new("Target creature you control gets +3/+3 until end of turn.",
                    vec![Effect::boost_until_eot(3, 3)]),
                ModalMode::new("Target creature you control fights target creature an opponent controls.",
                    vec![Effect::Fight]),
            ], 2, 2)],
            TargetSpec::Custom("various".into()))],
        ..Default::default() }
}

fn bristlebane_battler(id: ObjectId, owner: PlayerId) -> CardData {
    // 6/6 Kithkin Soldier for {1}{G}. Trample. Ward {2}.
    // Enters with five -1/-1 counters. Whenever another creature you control enters while this has a -1/-1 counter, remove one.
    CardData { id, owner, name: "Bristlebane Battler".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Kithkin, SubType::Soldier],
        power: Some(6), toughness: Some(6),
        keywords: KeywordAbilities::TRAMPLE | KeywordAbilities::WARD,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id, "Ward {2}",
                vec![StaticEffect::ward("{2}")]),
            Ability::static_ability(id,
                "This creature enters with five -1/-1 counters on it.",
                vec![StaticEffect::enters_with_counters("-1/-1", 5)]),
            Ability::triggered(id,
                "Whenever another creature you control enters while this creature has a -1/-1 counter on it, remove a -1/-1 counter from this creature.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::RemoveCounters { counter_type: "-1/-1".into(), count: 1 }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn bristlebane_outrider(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/5 Kithkin Knight for {3}{G}. Can't be blocked by power 2 or less (Daunt).
    // If another creature entered this turn, gets +2/+0.
    CardData { id, owner, name: "Bristlebane Outrider".into(), mana_cost: ManaCost::parse("{3}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Kithkin, SubType::Knight],
        power: Some(3), toughness: Some(5), rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "This creature can't be blocked by creatures with power 2 or less.",
                vec![StaticEffect::CantBeBlockedByPowerLessOrEqual { power: 2 }]),
            Ability::static_ability(id,
                "As long as another creature entered the battlefield under your control this turn, this creature gets +2/+0.",
                vec![StaticEffect::ConditionalBoostSelf { power: 2, toughness: 0, condition: "creature entered this turn".into() }]),
        ],
        ..Default::default() }
}

fn catharsis(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/4 Elemental Incarnation for {4}{R/W}{R/W}.
    // ETB if {W}{W} spent: create two 1/1 Kithkin tokens.
    // ETB if {R}{R} spent: creatures +1/+1 and haste until EOT.
    // Evoke {R/W}{R/W}.
    CardData { id, owner, name: "Catharsis".into(), mana_cost: ManaCost::parse("{4}{R/W}{R/W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elemental, SubType::Incarnation],
        power: Some(3), toughness: Some(4), rarity: Rarity::Mythic,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, if {W}{W} was spent to cast it, create two 1/1 green and white Kithkin creature tokens.",
                vec![Effect::create_token("1/1 Kithkin", 2)],
                TargetSpec::None),
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, if {R}{R} was spent to cast it, creatures you control get +1/+1 and gain haste until end of turn.",
                vec![Effect::boost_all_eot("creatures you control", 1, 1), Effect::grant_keyword_all_eot("creatures you control", "haste")],
                TargetSpec::None),
            Ability::static_ability(id, "Evoke {R/W}{R/W}",
                vec![StaticEffect::evoke("{R/W}{R/W}")]),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [BEHOLD+TYPE] Behold mechanic, creature type choice, search library, conditional battlefield vs hand placement
fn celestial_reunion(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery for {X}{G}. Optional additional cost: choose creature type and behold two.
    // Search library for creature card MV X or less.
    CardData { id, owner, name: "Celestial Reunion".into(), mana_cost: ManaCost::parse("{X}{G}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Rare,
        abilities: vec![Ability::spell(id,
            vec![Effect::search_library("creature card with mana value X or less")],
            TargetSpec::None)],
        ..Default::default() }
}

fn champion_of_the_clachan(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/5 Kithkin Knight for {3}{W}. Flash. Additional cost: behold a Kithkin and exile it.
    // Other Kithkin you control get +1/+1. Leaves: return exiled card to hand.
    CardData { id, owner, name: "Champion of the Clachan".into(), mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Kithkin, SubType::Knight],
        power: Some(4), toughness: Some(5),
        keywords: KeywordAbilities::FLASH,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "Other Kithkin you control get +1/+1.",
                vec![StaticEffect::boost_controlled("other Kithkin you control", 1, 1)]),
            Ability::triggered(id,
                "When this creature leaves the battlefield, return the exiled card to its owner's hand.",
                vec![EventType::ZoneChanged],
                vec![Effect::return_exiled_to_hand()],
                TargetSpec::None),
        ],
        additional_costs: vec![Cost::behold_and_exile("Kithkin")],
        ..Default::default() }
}

fn champion_of_the_path(id: ObjectId, owner: PlayerId) -> CardData {
    // 7/3 Elemental Sorcerer for {3}{R}. Additional cost: behold an Elemental and exile it.
    // Whenever another Elemental you control enters, it deals damage equal to its power to each opponent.
    // Leaves: return exiled card to hand.
    CardData { id, owner, name: "Champion of the Path".into(), mana_cost: ManaCost::parse("{3}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elemental, SubType::Sorcerer],
        power: Some(7), toughness: Some(3), rarity: Rarity::Rare,
        abilities: vec![
            Ability::triggered(id,
                "Whenever another Elemental you control enters, it deals damage equal to its power to each opponent.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::damage_opponents(0)],  // damage = entering creature's power (dynamic)
                TargetSpec::None),
            Ability::triggered(id,
                "When this creature leaves the battlefield, return the exiled card to its owner's hand.",
                vec![EventType::ZoneChanged],
                vec![Effect::return_exiled_to_hand()],
                TargetSpec::None),
        ],
        additional_costs: vec![Cost::behold_and_exile("Elemental")],
        ..Default::default() }
}

fn chronicle_of_victory(id: ObjectId, owner: PlayerId) -> CardData {
    // Legendary Artifact for {6}. As ETB: choose creature type.
    // Chosen type creatures you control get +2/+2 and have first strike and trample.
    // Whenever you cast spell of chosen type, draw a card.
    CardData { id, owner, name: "Chronicle of Victory".into(), mana_cost: ManaCost::parse("{6}"),
        card_types: vec![CardType::Artifact], supertypes: vec![SuperType::Legendary],
        rarity: Rarity::Mythic,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "As Chronicle of Victory enters, choose a creature type.",
                vec![Effect::choose_creature_type()],
                TargetSpec::None),
            Ability::static_ability(id,
                "Creatures you control of the chosen type get +2/+2 and have first strike and trample.",
                vec![StaticEffect::boost_controlled("creatures of chosen type", 2, 2),
                     StaticEffect::grant_keyword_controlled("creatures of chosen type", "first strike"),
                     StaticEffect::grant_keyword_controlled("creatures of chosen type", "trample")]),
            Ability::triggered(id,
                "Whenever you cast a spell of the chosen type, draw a card.",
                vec![EventType::SpellCast],
                vec![Effect::draw_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn collective_inferno(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment for {3}{R}{R}. Convoke. As ETB: choose creature type.
    // Double all damage that sources you control of the chosen type would deal.
    CardData { id, owner, name: "Collective Inferno".into(), mana_cost: ManaCost::parse("{3}{R}{R}"),
        card_types: vec![CardType::Enchantment], keywords: KeywordAbilities::CONVOKE,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "As this enchantment enters, choose a creature type.",
                vec![Effect::choose_creature_type()],
                TargetSpec::None),
            Ability::static_ability(id,
                "Double all damage that sources you control of the chosen type would deal.",
                vec![StaticEffect::damage_doubling_from_type()]),
        ],
        ..Default::default() }
}

fn curious_colossus(id: ObjectId, owner: PlayerId) -> CardData {
    // 7/7 Giant Warrior for {5}{W}{W}.
    // ETB: each creature target opponent controls loses all abilities, becomes Coward, base P/T 1/1.
    CardData { id, owner, name: "Curious Colossus".into(), mana_cost: ManaCost::parse("{5}{W}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Giant, SubType::Warrior],
        power: Some(7), toughness: Some(7), rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, each creature target opponent controls loses all abilities, becomes a Coward in addition to its other types, and has base power and toughness 1/1.",
                vec![Effect::lose_all_abilities_all("creatures opponents control"),
                     Effect::set_base_pt_all(1, 1, "creatures opponents control"),
                     Effect::add_subtype_all("Coward", "creatures opponents control")],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn dawn_blessed_pennant(id: ObjectId, owner: PlayerId) -> CardData {
    // Artifact for {1}. As ETB: choose creature type (ECL types).
    // Whenever chosen type permanent ETBs, gain 1 life.
    // {2}, {T}, Sacrifice: return card of chosen type from graveyard to hand.
    CardData { id, owner, name: "Dawn-Blessed Pennant".into(), mana_cost: ManaCost::parse("{1}"),
        card_types: vec![CardType::Artifact], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "As this artifact enters, choose Elemental, Elf, Faerie, Giant, Goblin, Kithkin, Merfolk, or Treefolk.",
                vec![Effect::choose_creature_type_restricted(vec!["Elemental", "Elf", "Faerie", "Giant", "Goblin", "Kithkin", "Merfolk", "Treefolk"])],
                TargetSpec::None),
            Ability::triggered(id,
                "Whenever a permanent you control of the chosen type enters, you gain 1 life.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::gain_life(1)],
                TargetSpec::None),
            Ability::activated(id,
                "{2}, {T}, Sacrifice this artifact: Return target card of the chosen type from your graveyard to your hand.",
                vec![Cost::pay_mana("{2}"), Cost::tap_self(), Cost::sacrifice_self()],
                vec![Effect::return_from_graveyard()],
                TargetSpec::CardInYourGraveyard),
        ],
        ..Default::default() }
}

fn dawnhand_dissident(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/2 Elf Warlock for {B}. {T}, Blight 1: Surveil 1. {T}, Blight 2: Exile card from graveyard.
    // You may cast creature spells from among exiled cards by removing three counters from your creatures.
    CardData { id, owner, name: "Dawnhand Dissident".into(), mana_cost: ManaCost::parse("{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elf, SubType::Warlock],
        power: Some(1), toughness: Some(2), rarity: Rarity::Rare,
        abilities: vec![
            Ability::activated(id,
                "{T}, Blight 1: Surveil 1.",
                vec![Cost::tap_self(), Cost::Blight(1)],
                vec![Effect::scry(1)],
                TargetSpec::None),
            Ability::activated(id,
                "{T}, Blight 2: Exile target card from a graveyard.",
                vec![Cost::tap_self(), Cost::Blight(2)],
                vec![Effect::exile_target_to_source_zone()],
                TargetSpec::CardInGraveyard),
            Ability::static_ability(id,
                "During your turn, you may cast creature spells from among cards you own exiled with this creature by removing three counters from among creatures you control in addition to paying their other costs.",
                vec![StaticEffect::cast_from_exile_with_counter_cost(3)]),
        ],
        ..Default::default() }
}

fn deceit(id: ObjectId, owner: PlayerId) -> CardData {
    // 5/5 Elemental Incarnation for {4}{U/B}{U/B}.
    // ETB if {U}{U}: bounce up to one other nonland permanent.
    // ETB if {B}{B}: opponent reveals hand, discard nonland card.
    // Evoke {U/B}{U/B}.
    CardData { id, owner, name: "Deceit".into(), mana_cost: ManaCost::parse("{4}{U/B}{U/B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elemental, SubType::Incarnation],
        power: Some(5), toughness: Some(5), rarity: Rarity::Mythic,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, if {U}{U} was spent to cast it, return up to one other target nonland permanent to its owner's hand.",
                vec![Effect::bounce()],
                TargetSpec::PermanentFiltered("other nonland permanent".into())),
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, if {B}{B} was spent to cast it, target opponent reveals their hand. You choose a nonland card from it. That player discards that card.",
                vec![Effect::discard_cards(1)],
                TargetSpec::Player),
            Ability::static_ability(id, "Evoke {U/B}{U/B}",
                vec![StaticEffect::evoke("{U/B}{U/B}")]),
        ],
        ..Default::default() }
}

fn deepway_navigator(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Merfolk Wizard for {W}{U}. Flash.
    // ETB: untap each other Merfolk you control.
    // As long as you attacked with 3+ Merfolk this turn, Merfolk you control get +1/+0.
    CardData { id, owner, name: "Deepway Navigator".into(), mana_cost: ManaCost::parse("{W}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Merfolk, SubType::Wizard],
        power: Some(2), toughness: Some(2),
        keywords: KeywordAbilities::FLASH,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, untap each other Merfolk you control.",
                vec![Effect::untap_all("other Merfolk you control")],
                TargetSpec::None),
            Ability::static_ability(id,
                "As long as you attacked with three or more Merfolk this turn, Merfolk you control get +1/+0.",
                vec![StaticEffect::boost_controlled("Merfolk you control", 1, 0)]),
        ],
        ..Default::default() }
}

fn disruptor_of_currents(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/3 Merfolk Wizard for {3}{U}{U}. Flash. Convoke.
    // ETB: return up to one other target nonland permanent to its owner's hand.
    CardData { id, owner, name: "Disruptor of Currents".into(), mana_cost: ManaCost::parse("{3}{U}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Merfolk, SubType::Wizard],
        power: Some(3), toughness: Some(3),
        keywords: KeywordAbilities::FLASH | KeywordAbilities::CONVOKE,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, return up to one other target nonland permanent to its owner's hand.",
                vec![Effect::bounce()],
                TargetSpec::PermanentFiltered("other nonland permanent".into())),
        ],
        ..Default::default() }
}

fn doran_besieged_by_time(id: ObjectId, owner: PlayerId) -> CardData {
    // Legendary 0/5 Treefolk Druid for {1}{W}{B}{G}.
    // Creature spells with toughness > power cost {1} less.
    // Whenever a creature you control attacks or blocks, it gets +X/+X where X = toughness - power.
    CardData { id, owner, name: "Doran, Besieged by Time".into(), mana_cost: ManaCost::parse("{1}{W}{B}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Treefolk, SubType::Druid],
        supertypes: vec![SuperType::Legendary],
        power: Some(0), toughness: Some(5), rarity: Rarity::Mythic,
        abilities: vec![
            Ability::static_ability(id,
                "Each creature spell you cast with toughness greater than its power costs {1} less to cast.",
                vec![StaticEffect::cost_reduction_if_toughness_greater("creature spells", 1)]),
            Ability::controlled_creature_attacks_or_blocks_triggered(id,
                "Whenever a creature you control attacks or blocks, it gets +X/+X until end of turn, where X is the difference between its toughness and power.",
                vec![Effect::boost_by_toughness_minus_power()],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn eclipsed_realms(id: ObjectId, owner: PlayerId) -> CardData {
    // Land. As ETB: choose creature type (ECL types). {T}: Add {C}. {T}: Add one mana of any color (spend only for chosen type).
    CardData { id, owner, name: "Eclipsed Realms".into(),
        card_types: vec![CardType::Land], rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "As this land enters, choose Elemental, Elf, Faerie, Giant, Goblin, Kithkin, Merfolk, or Treefolk.",
                vec![Effect::choose_creature_type_restricted(vec!["Elemental", "Elf", "Faerie", "Giant", "Goblin", "Kithkin", "Merfolk", "Treefolk"])],
                TargetSpec::None),
            Ability::mana_ability(id, "{T}: Add {C}.", Mana::colorless(1)),
            Ability::mana_ability(id, "{T}: Add one mana of any color. Spend this mana only to cast a spell of the chosen type.", Mana::colorless(1)),
        ],
        ..Default::default() }
}

fn eirdu_carrier_of_dawn(id: ObjectId, owner: PlayerId) -> CardData {
    let mut back = CardData { id, owner, name: "Isilu, Carrier of Twilight".into(),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elemental, SubType::God],
        supertypes: vec![SuperType::Legendary],
        power: Some(5), toughness: Some(5),
        keywords: KeywordAbilities::FLYING | KeywordAbilities::LIFELINK,
        color_identity: vec![Color::Black],
        rarity: Rarity::Mythic,
        abilities: vec![
            Ability::static_ability(id,
                "Each other nontoken creature you control has persist.",
                vec![StaticEffect::grant_keyword_controlled("other nontoken creatures you control", "persist")]),
            Ability::triggered(id,
                "At the beginning of your first main phase, you may pay {W}. If you do, transform Isilu.",
                vec![EventType::PrecombatMainPre],
                vec![Effect::do_if_cost_paid(Cost::pay_mana("{W}"), vec![Effect::transform_self()], vec![])],
                TargetSpec::None),
        ],
        ..Default::default() };
    back.back_face = None;
    CardData { id, owner, name: "Eirdu, Carrier of Dawn".into(), mana_cost: ManaCost::parse("{3}{W}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elemental, SubType::God],
        supertypes: vec![SuperType::Legendary],
        power: Some(5), toughness: Some(5),
        keywords: KeywordAbilities::FLYING | KeywordAbilities::LIFELINK,
        rarity: Rarity::Mythic,
        abilities: vec![
            Ability::static_ability(id,
                "Creature spells you cast have convoke.",
                vec![StaticEffect::grant_convoke("creature spells")]),
            Ability::triggered(id,
                "At the beginning of your first main phase, you may pay {B}. If you do, transform Eirdu.",
                vec![EventType::PrecombatMainPre],
                vec![Effect::do_if_cost_paid(Cost::pay_mana("{B}"), vec![Effect::transform_self()], vec![])],
                TargetSpec::None),
        ],
        back_face: Some(Box::new(back)),
        ..Default::default() }
}

fn emptiness(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/5 Elemental Incarnation for {4}{W/B}{W/B}.
    // ETB if {W}{W}: reanimate creature MV 3 or less from your graveyard.
    // ETB if {B}{B}: put three -1/-1 counters on up to one target creature.
    // Evoke {W/B}{W/B}.
    CardData { id, owner, name: "Emptiness".into(), mana_cost: ManaCost::parse("{4}{W/B}{W/B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elemental, SubType::Incarnation],
        power: Some(3), toughness: Some(5), rarity: Rarity::Mythic,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, if {W}{W} was spent to cast it, return target creature card with mana value 3 or less from your graveyard to the battlefield.",
                vec![Effect::reanimate()],
                TargetSpec::CardInYourGraveyard),
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, if {B}{B} was spent to cast it, put three -1/-1 counters on up to one target creature.",
                vec![Effect::add_counters("-1/-1", 3)],
                TargetSpec::Creature),
            Ability::static_ability(id, "Evoke {W/B}{W/B}",
                vec![StaticEffect::evoke("{W/B}{W/B}")]),
        ],
        ..Default::default() }
}
// ENGINE DEPS: [TRANSFORM] Transform/DFC system, discard-draw, conditional mana, BeginningOfMainPhase trigger
fn ashling_rekindled(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ashling, Rekindled".into(),
        card_types: vec![CardType::Creature],
        supertypes: vec![SuperType::Legendary],
        rarity: Rarity::Rare,
        ..Default::default() }
}

// ENGINE DEPS: [TRANSFORM+MANA] Transform/DFC, create Kithkin token on ETB/transform, dynamic mana based on creature count
fn brigid_clachans_heart(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Brigid, Clachan's Heart".into(),
        card_types: vec![CardType::Creature],
        supertypes: vec![SuperType::Legendary],
        rarity: Rarity::Rare,
        ..Default::default() }
}

fn evershrikes_gift(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment — Aura for {W}. Enchant creature. +1/+0 and flying.
    // {1}{W}, Blight 2: Return from GY to hand. Sorcery speed.
    CardData { id, owner, name: "Evershrike's Gift".into(),
        mana_cost: ManaCost::parse("{W}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Aura],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                "Enchanted creature gets +1/+0 and has flying.",
                vec![StaticEffect::Boost { filter: "enchanted creature".into(), power: 1, toughness: 0 },
                     StaticEffect::GrantKeyword { filter: "enchanted creature".into(), keyword: "flying".into() }]),
            Ability::activated(id,
                "{1}{W}, Blight 2: Return this card from your graveyard to your hand. Activate only as a sorcery.",
                vec![Cost::pay_mana("{1}{W}"), Cost::Blight(2)],
                vec![Effect::return_from_graveyard()],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn figure_of_fable(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Figure of Fable".into(),
        mana_cost: ManaCost::parse("{G/W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Kithkin],
        power: Some(1), toughness: Some(1),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::activated(id,
                "{G/W}: This creature becomes a Kithkin Scout with base power and toughness 2/3.",
                vec![Cost::pay_mana("{G/W}")],
                vec![Effect::set_subtypes_self(vec!["Kithkin", "Scout"]),
                     Effect::set_pt(2, 3)],
                TargetSpec::None),
            Ability::activated(id,
                "{1}{G/W}{G/W}: If this creature is a Scout, it becomes a Kithkin Soldier with base power and toughness 4/5.",
                vec![Cost::pay_mana("{1}{G/W}{G/W}")],
                vec![Effect::conditional("source is a Scout",
                    vec![Effect::set_subtypes_self(vec!["Kithkin", "Soldier"]),
                         Effect::set_pt(4, 5)],
                    vec![])],
                TargetSpec::None),
            Ability::activated(id,
                "{3}{G/W}{G/W}{G/W}: If this creature is a Soldier, it becomes a Kithkin Avatar 7/8 with protection from each opponent.",
                vec![Cost::pay_mana("{3}{G/W}{G/W}{G/W}")],
                vec![Effect::conditional("source is a Soldier",
                    vec![Effect::set_subtypes_self(vec!["Kithkin", "Avatar"]),
                         Effect::set_pt(7, 8),
                         Effect::GainKeyword { keyword: "protection".into() }],
                    vec![])],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND+MANA] Changeling, any-color mana, animated artifact (becomes 4/4 creature until EOT)
fn firdoch_core(id: ObjectId, owner: PlayerId) -> CardData {
    // Kindred Artifact Shapeshifter {3}. Changeling. T: any color mana. {4}: becomes 4/4 creature until EOT.
    CardData { id, owner, name: "Firdoch Core".into(),
        mana_cost: ManaCost::parse("{3}"),
        card_types: vec![CardType::Kindred, CardType::Artifact],
        subtypes: vec![SubType::Shapeshifter],
        keywords: KeywordAbilities::CHANGELING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::mana_ability(id, "{T}: Add one mana of any color.", Mana::green(1)),
            Ability::activated(id,
                "{4}: This artifact becomes a 4/4 artifact creature until end of turn.",
                vec![Cost::pay_mana("{4}")],
                vec![Effect::becomes_creature(4, 4)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn flitterwing_nuisance(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Flitterwing Nuisance".into(),
        mana_cost: ManaCost::parse("{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Faerie, SubType::Rogue],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::static_ability(id,
                "This creature enters with a -1/-1 counter on it.",
                vec![StaticEffect::enters_with_counters("-1/-1", 1)]),
            Ability::activated(id,
                "{2}{U}, Remove a counter from this creature: Whenever a creature you control deals combat damage to a player or planeswalker this turn, draw a card.",
                vec![Cost::pay_mana("{2}{U}"), Cost::remove_counters("any", 1)],
                vec![Effect::grant_triggered_ability_eot(
                    "damaged_player",
                    "creatures you control",
                    vec![Effect::draw_cards(1)])],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND+MANA] Surveil 1 ETB, any-color mana + color change once per turn
fn foraging_wickermaw(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/3 Artifact Creature Scarecrow {2}. ETB: surveil 1. {1}: any color mana (once per turn).
    CardData { id, owner, name: "Foraging Wickermaw".into(),
        mana_cost: ManaCost::parse("{2}"),
        card_types: vec![CardType::Artifact, CardType::Creature],
        subtypes: vec![SubType::Scarecrow],
        power: Some(1), toughness: Some(3),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, surveil 1.",
                vec![Effect::scry(1)],
                TargetSpec::None),
            Ability::mana_ability(id,
                "{1}: Add one mana of any color. This creature becomes that color until end of turn. Activate only once each turn.",
                Mana::green(1)),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [TYPE+CHOICE] Choose creature type, cost reduction for chosen type, look at top card + conditional reveal
fn gathering_stone(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Gathering Stone".into(),
        mana_cost: ManaCost::parse("{4}"),
        card_types: vec![CardType::Artifact],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "As Gathering Stone enters, choose a creature type.",
                vec![Effect::choose_creature_type()],
                TargetSpec::None),
            Ability::static_ability(id,
                "Spells you cast of the chosen type cost {1} less to cast.",
                vec![StaticEffect::CostReduction { filter: "spells of chosen type".into(), amount: 1, condition: None }]),
            Ability::triggered(id,
                "When this artifact enters and at the beginning of your upkeep, look at the top card of your library. If it's a card of the chosen type, you may reveal it and put it into your hand.",
                vec![EventType::EnteredTheBattlefield, EventType::UpkeepStep],
                vec![Effect::Custom("Look at top card, reveal if chosen type, may put to hand or graveyard.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn giantfall(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Giantfall".into(),
        mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::destroy()],
                    TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn gilt_leafs_embrace(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment — Aura for {2}{G}. Flash. Enchant creature.
    // ETB: enchanted creature gains trample and indestructible until EOT. Static: +2/+0.
    CardData { id, owner, name: "Gilt-Leaf's Embrace".into(),
        mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Aura],
        keywords: KeywordAbilities::FLASH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this Aura enters, enchanted creature gains trample and indestructible until end of turn.",
                vec![Effect::gain_keyword_eot("trample"), Effect::gain_keyword_eot("indestructible")],
                TargetSpec::None),
            Ability::static_ability(id,
                "Enchanted creature gets +2/+0.",
                vec![StaticEffect::Boost { filter: "enchanted creature".into(), power: 2, toughness: 0 }]),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] Set base P/T 4/4 + gain all creature types on target until EOT
fn glamer_gifter(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/2 Faerie Wizard {1}{U}. Flash, Flying. ETB: target creature becomes 4/4 + gains all types until EOT.
    CardData { id, owner, name: "Glamer Gifter".into(),
        mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Faerie, SubType::Wizard],
        power: Some(1), toughness: Some(2),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLASH | KeywordAbilities::FLYING,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this enters, up to one other target creature has base power and toughness 4/4 and gains all creature types until end of turn.",
                vec![Effect::SetPowerToughness { power: 4, toughness: 4 }, Effect::gain_all_creature_types()],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COST] ETB with -1/-1 counter, remove counter cost, counter noncreature spell, target's controller draws
fn glen_elendra_guardian(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/4 Faerie Wizard for {2}{U}. Flash, Flying. (ETB with -1/-1 counter; {1}{U}, remove 1: counter noncreature spell, controller draws)
    CardData { id, owner, name: "Glen Elendra Guardian".into(),
        mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Faerie, SubType::Wizard],
        power: Some(3), toughness: Some(4),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLASH | KeywordAbilities::FLYING,
        abilities: vec![
            Ability::static_ability(id,
                "Glen Elendra Guardian enters with a -1/-1 counter on it.",
                vec![StaticEffect::enters_with_counters("-1/-1", 1)]),
            Ability::activated(id,
                "{1}{U}, Remove a -1/-1 counter from Glen Elendra Guardian: Counter target noncreature spell. Its controller draws a card.",
                vec![Cost::pay_mana("{1}{U}"), Cost::remove_counters("-1/-1", 1)],
                vec![Effect::counter_spell(), Effect::target_controller_draws(1)],
                TargetSpec::Spell),
        ],
        ..Default::default() }
}

fn goliath_daydreamer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Goliath Daydreamer".into(),
        mana_cost: ManaCost::parse("{2}{R}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Giant, SubType::Wizard],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::spell_cast_triggered(id,
                    "Whenever you cast an instant or sorcery spell from your hand, exile that card with a dream counter on it instead of putting it into your graveyard as it resolves.",
                    vec![Effect::exile_with_dream_counter()],
                    TargetSpec::None),
            Ability::triggered(id,
                    "Whenever Goliath Daydreamer attacks, you may cast a spell from among cards you own in exile with dream counters on them without paying its mana cost.",
                    vec![EventType::AttackerDeclared],
                    vec![Effect::cast_from_exile_with_dream_counters()],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [TRANSFORM+COST] Transform/DFC, return Goblin from GY, attacks blight then token copy tapped+attacking
fn grub_storied_matriarch(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Grub, Storied Matriarch".into(),
        mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Goblin, SubType::Warlock],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(1),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::MENACE,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever Grub transforms into this, return up to one target Goblin creature card from your graveyard to your hand.",
                    vec![EventType::EnteredTheBattlefield],
                    vec![Effect::return_from_graveyard()],
                    TargetSpec::CardInYourGraveyard),
            Ability::triggered(id,
                    "Whenever Grub attacks, you may blight 1. If you do, create a tapped and attacking token copy of it. Sacrifice that token at end of combat.",
                    vec![EventType::AttackerDeclared],
                    vec![Effect::Custom("Blight 1, create tapped+attacking token copy, sacrifice at end of combat.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn grubs_command(id: ObjectId, owner: PlayerId) -> CardData {
    // {3}{B}{R} Kindred Sorcery — Goblin. Choose two of 4 modes.
    CardData { id, owner, name: "Grub's Command".into(),
        mana_cost: ManaCost::parse("{3}{B}{R}"),
        card_types: vec![CardType::Kindred, CardType::Sorcery],
        subtypes: vec![SubType::Goblin],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::modal(vec![
                    ModalMode::new("Create a token that's a copy of target Goblin you control.",
                        vec![Effect::create_token_copy(1)]),
                    ModalMode::new("Creatures target player controls get +1/+1 and gain haste until end of turn.",
                        vec![Effect::boost_all_eot("creatures target player controls", 1, 1),
                             Effect::grant_keyword_all_eot("creatures target player controls", "haste")]),
                    ModalMode::new("Destroy target artifact or creature.",
                        vec![Effect::destroy()]),
                    ModalMode::new("Target player mills five cards, then puts each Goblin card milled this way into their hand.",
                        vec![Effect::mill_and_return_all(5, "Goblin")]),
                ], 2, 2)],
                TargetSpec::Custom("various".into())),
        ],
        ..Default::default() }
}

fn hallowed_fountain(id: ObjectId, owner: PlayerId) -> CardData {
    // DONE - Land — Plains Island. Pay 2 life or enters tapped.
    CardData { id, owner, name: "Hallowed Fountain".into(),
        card_types: vec![CardType::Land],
        subtypes: vec![SubType::Plains, SubType::Island],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "As Hallowed Fountain enters, you may pay 2 life. If you don't, it enters tapped.",
                vec![StaticEffect::enters_tapped_unless("pay 2 life")]),
            Ability::mana_ability(id, "{T}: Add {W}.", Mana::white(1)),
            Ability::mana_ability(id, "{T}: Add {U}.", Mana::blue(1)),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [TYPE] Convoke, choose creature type, draw cards equal to permanents of that type
fn harmonized_crescendo(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Harmonized Crescendo".into(),
        mana_cost: ManaCost::parse("{4}{U}{U}"),
        card_types: vec![CardType::Instant],
        keywords: KeywordAbilities::CONVOKE,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::choose_type_and_draw_per_permanent()],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] Can't be countered, Ward-pay 2 life, spells can't be countered static, grant ward to others
fn hexing_squelcher(id: ObjectId, owner: PlayerId) -> CardData {
    // {1}{R} 2/2 Goblin Sorcerer. Can't be countered. Ward-pay 2 life. Spells can't be countered. Others have ward.
    CardData { id, owner, name: "Hexing Squelcher".into(),
        mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Goblin, SubType::Sorcerer],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::WARD,
        abilities: vec![
            Ability::static_ability(id,
                "This spell can't be countered.",
                vec![StaticEffect::CantBeCountered]),
            Ability::static_ability(id,
                "Ward--Pay 2 life.",
                vec![StaticEffect::Ward { cost: "Pay 2 life".into() }]),
            Ability::static_ability(id,
                "Spells you control can't be countered.",
                vec![StaticEffect::SpellsCantBeCountered]),
            Ability::static_ability(id,
                "Other creatures you control have ward--pay 2 life.",
                vec![StaticEffect::GrantKeyword { filter: "other creature you control".into(), keyword: "ward".into() }]),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COST+COND] ETB this/another Elf then opponents blight 1, tap 3 Elves then proliferate (sorcery speed)
fn high_perfect_morcant(id: ObjectId, owner: PlayerId) -> CardData {
    // Legendary 4/4 Elf Noble for {2}{B}{G}.
    // Whenever this or another Elf ETBs, each opponent blights 1.
    // Tap 3 untapped Elves: proliferate. Activate only as sorcery.
    CardData { id, owner, name: "High Perfect Morcant".into(),
        mana_cost: ManaCost::parse("{2}{B}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elf, SubType::Noble],
        supertypes: vec![SuperType::Legendary],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::triggered(id,
                "Whenever High Perfect Morcant or another Elf enters the battlefield under your control, each opponent blights 1.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::blight_opponents(1)],
                TargetSpec::None),
            Ability::activated(id,
                "Tap three untapped Elves you control: Proliferate. Activate only as a sorcery.",
                vec![Cost::tap_creatures("Elf", 3)],
                vec![Effect::proliferate()],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] Conditional flash (if you control Faerie), hexproof while untapped
fn illusion_spinners(id: ObjectId, owner: PlayerId) -> CardData {
    // {4}{U} 4/3 Faerie Wizard. Flash if you control Faerie. Flying. Hexproof while untapped.
    CardData { id, owner, name: "Illusion Spinners".into(),
        mana_cost: ManaCost::parse("{4}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Faerie, SubType::Wizard],
        power: Some(4), toughness: Some(3),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::HEXPROOF,
        abilities: vec![
            Ability::static_ability(id,
                "You may cast this spell as though it had flash if you control a Faerie.",
                vec![StaticEffect::ConditionalKeyword { keyword: "flash".into(), condition: "you control a Faerie".into() }]),
            Ability::static_ability(id,
                "This creature has hexproof as long as it's untapped.",
                vec![StaticEffect::ConditionalKeyword { keyword: "hexproof".into(), condition: "untapped".into() }]),
        ],
        ..Default::default() }
}

fn keep_out(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Keep Out".into(),
        mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::destroy()],
                    TargetSpec::Creature),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] Dynamic +X/+X where X=creatures entered this turn (watcher), begin-of-combat token creation
fn kinbinding(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment {3}{W}{W}. Creatures +X/+X (X=creatures ETBd this turn). Combat: create 1/1 Kithkin.
    CardData { id, owner, name: "Kinbinding".into(),
        mana_cost: ManaCost::parse("{3}{W}{W}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "Creatures you control get +X/+X, where X is the number of creatures that entered the battlefield under your control this turn.",
                vec![StaticEffect::boost_per_turn_event("creatures you control", "creatures_entered", 1, 1)]),
            Ability::triggered(id,
                "At the beginning of combat on your turn, create a 1/1 green and white Kithkin creature token.",
                vec![EventType::BeginCombat],
                vec![Effect::create_token("1/1 Kithkin", 1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] Attacks then put creature from hand onto battlefield tapped+attacking if MV <= attacking count
fn kinscaer_sentry(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Kithkin Soldier {1}{W}. First strike, lifelink. Attacks: put creature MV<=attackers from hand tapped+attacking.
    CardData { id, owner, name: "Kinscaer Sentry".into(),
        mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Kithkin, SubType::Soldier],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FIRST_STRIKE | KeywordAbilities::LIFELINK,
        abilities: vec![
            Ability::attacks_triggered(id,
                "Whenever this creature attacks, you may put a creature card with mana value X or less from your hand onto the battlefield tapped and attacking, where X is the number of attacking creatures you control.",
                vec![Effect::put_from_hand_tapped_attacking_dynamic("attacking creatures you control")],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// TRIAGED: Lasting Tarfire is an Enchantment {1}{R}
// "At the beginning of each end step, if you put a counter on a creature this turn,
//  this enchantment deals 2 damage to each opponent."
// Needs: conditional end-step trigger + watcher tracking CountersAdded on creatures by player.
// Engine has: beginning_of_end_step_triggered, DealDamageOpponents, CountersAdded event type,
// CustomWatcher infra. Missing: conditional trigger check (intervening-if clause on watcher state).
// Category: COND (Conditional/Dynamic Effects)
// ENGINE DEPS: [COND] PARTIAL — damage typed, conditional counter-placement check not enforced
fn lasting_tarfire(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Lasting Tarfire".into(),
        mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::beginning_of_end_step_triggered(id,
                "At the beginning of each end step, if you put a counter on a creature this turn, Lasting Tarfire deals 2 damage to each opponent.",
                vec![Effect::damage_opponents(2)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] Mill 4 + top-of-library manipulation, discard land cost, tokens = lands in GY
fn lluwen_imperfect_naturalist(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Lluwen, Imperfect Naturalist".into(),
        mana_cost: ManaCost::parse("{B/G}{B/G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elf, SubType::Druid],
        supertypes: vec![SuperType::Legendary],
        power: Some(1), toughness: Some(3),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Lluwen enters, mill four cards. You may put a creature or land card from among them on top of your library.",
                    vec![Effect::mill_and_select(4, "creature or land", "top")],
                    TargetSpec::None),
            Ability::activated(id,
                    "{2}{B/G}{B/G}{B/G}, {T}, Discard a land card: Create X 1/1 black and green Worm creature tokens, where X is the number of land cards in your graveyard.",
                    vec![Cost::pay_mana("{2}{B/G}{B/G}{B/G}"), Cost::tap_self()],
                    vec![Effect::CreateTokenDynamic { token_name: "1/1 Worm".into(), count_filter: "land cards in your graveyard".into() }],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COST] ETB with 3 -1/-1 counters, remove counter costs, draw card, tap+stun counter
fn loch_mare(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/5 Horse Serpent for {1}{U}. (ETB with 3 -1/-1 counters; {1}{U}, remove 1: draw; {2}{U}, remove 2: tap+stun)
    CardData { id, owner, name: "Loch Mare".into(),
        mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Horse, SubType::Serpent],
        power: Some(4), toughness: Some(5),
        rarity: Rarity::Mythic,
        abilities: vec![
            Ability::static_ability(id,
                "Loch Mare enters with three -1/-1 counters on it.",
                vec![StaticEffect::enters_with_counters("-1/-1", 3)]),
            Ability::activated(id,
                "{1}{U}, Remove a -1/-1 counter from Loch Mare: Draw a card.",
                vec![Cost::pay_mana("{1}{U}"), Cost::remove_counters("-1/-1", 1)],
                vec![Effect::draw_cards(1)],
                TargetSpec::None),
            Ability::activated(id,
                "{2}{U}, Remove two -1/-1 counters from Loch Mare: Tap target creature. Put a stun counter on it.",
                vec![Cost::pay_mana("{2}{U}"), Cost::remove_counters("-1/-1", 2)],
                vec![Effect::tap_target(), Effect::add_counters("stun", 1)],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}
// ENGINE DEPS: [AURA] Convoke, enchant creature, ETB draw, +2/+2 and flying
fn lofty_dreams(id: ObjectId, owner: PlayerId) -> CardData {
    // Aura {3}{U}{U}. Convoke. ETB: draw a card. Enchanted creature gets +2/+2 and has flying.
    CardData { id, owner, name: "Lofty Dreams".into(),
        mana_cost: ManaCost::parse("{3}{U}{U}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Aura],
        keywords: KeywordAbilities::CONVOKE,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this Aura enters, draw a card.",
                vec![Effect::draw_cards(1)],
                TargetSpec::None),
            Ability::static_ability(id,
                "Enchanted creature gets +2/+2 and has flying.",
                vec![StaticEffect::Boost { filter: "enchanted creature".into(), power: 2, toughness: 2 },
                     StaticEffect::GrantKeyword { filter: "enchanted creature".into(), keyword: "flying".into() }]),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [EXILE_CAST+COND] ETB this/Elf/Faerie exile opponent top 2, cast from exile with MV restriction, once per turn
fn maralen_fae_ascendant(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Maralen, Fae Ascendant".into(),
        mana_cost: ManaCost::parse("{2}{B}{G}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elf, SubType::Faerie, SubType::Noble],
        supertypes: vec![SuperType::Legendary],
        power: Some(4), toughness: Some(5),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::other_creature_etb_triggered(id,
                    "Whenever Maralen or another Elf or Faerie you control enters, exile the top two cards of target opponent's library.",
                    vec![Effect::exile_from_opponent_library(2)],
                    TargetSpec::None),
            Ability::static_ability(id,
                    "Once each turn, you may cast a spell with mana value less than or equal to the number of Elves and Faeries you control from among cards exiled with Maralen without paying its mana cost.",
                    vec![StaticEffect::Custom("Once per turn, cast exiled spell with MV <= Elves+Faeries you control for free.".into())]),
        ],
        ..Default::default() }
}

fn mirrormind_crown(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Mirrormind Crown".into(),
        mana_cost: ManaCost::parse("{4}"),
        card_types: vec![CardType::Artifact],
        subtypes: vec![SubType::Equipment],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                    "The first time you would create one or more tokens each turn while equipped creature is on the battlefield, you may instead create that many token copies of equipped creature.",
                    vec![StaticEffect::Custom("First token creation each turn may instead create copies of equipped creature.".into())]),
            Ability::activated(id, "Equip {2}",
                    vec![Cost::pay_mana("{2}")],
                    vec![Effect::Equip],
                    TargetSpec::Creature),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COST+COND] ETB with 6 -1/-1 counters, trigger on permanent cards to GY then remove -1/-1 counter
fn moonshadow(id: ObjectId, owner: PlayerId) -> CardData {
    // 7/7 Elemental for {B}. Menace. ETB with 6 -1/-1 counters.
    // Whenever permanents go to your GY while this has -1/-1 counter, remove a counter.
    CardData { id, owner, name: "Moonshadow".into(),
        mana_cost: ManaCost::parse("{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elemental],
        power: Some(7), toughness: Some(7),
        keywords: KeywordAbilities::MENACE,
        rarity: Rarity::Mythic,
        abilities: vec![
            Ability::static_ability(id,
                "Moonshadow enters with six -1/-1 counters on it.",
                vec![StaticEffect::enters_with_counters("-1/-1", 6)]),
            Ability::triggered(id,
                "Whenever one or more permanent cards are put into your graveyard from anywhere while this creature has a -1/-1 counter on it, remove a -1/-1 counter from this creature.",
                vec![EventType::Dies],
                vec![Effect::RemoveCounters { counter_type: "-1/-1".into(), count: 1 }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] DONE - Other Elves +1/+1 lord, dies then return another Elf card from GY to hand
fn morcants_loyalist(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Morcant's Loyalist".into(),
        mana_cost: ManaCost::parse("{1}{B}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elf, SubType::Warrior],
        power: Some(3), toughness: Some(2),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                    "Other Elves you control get +1/+1.",
                    vec![StaticEffect::Boost { filter: "other Elf you control".into(), power: 1, toughness: 1 }]),
            Ability::dies_triggered(id,
                    "When Morcant's Loyalist dies, return target Elf card from your graveyard to your hand.",
                    vec![Effect::return_from_graveyard()],
                    TargetSpec::CardInYourGraveyard),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] Players cant draw or gain life (static), each draw step: lose 3 life + search library
fn mornsong_aria(id: ObjectId, owner: PlayerId) -> CardData {
    // Legendary Enchantment {1}{B}{B}. No draw/life gain. Draw step: lose 3 life + search library.
    CardData { id, owner, name: "Mornsong Aria".into(),
        mana_cost: ManaCost::parse("{1}{B}{B}"),
        card_types: vec![CardType::Enchantment],
        supertypes: vec![SuperType::Legendary],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "Players can't draw cards or gain life.",
                vec![StaticEffect::CantGainLife, StaticEffect::CantDrawExtraCards]),
            Ability::triggered(id,
                "At the beginning of each player's draw step, that player loses 3 life, searches their library for a card, puts it into their hand, then shuffles.",
                vec![EventType::DrawStep],
                vec![Effect::LoseLife { amount: 3 }, Effect::search_library("card")],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn noggle_the_mind(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment — Aura for {1}{U}. Flash. Enchant creature.
    // Enchanted creature loses all abilities and is a colorless 1/1 Noggle.
    CardData { id, owner, name: "Noggle the Mind".into(),
        mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Aura],
        keywords: KeywordAbilities::FLASH,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                "Enchanted creature loses all abilities and is a colorless Noggle creature with base power and toughness 1/1.",
                vec![StaticEffect::lose_all_abilities("enchanted creature"),
                     StaticEffect::set_base_pt("enchanted creature", 1, 1),
                     StaticEffect::Custom("Enchanted creature is a colorless Noggle creature.".into())]),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COPY] Changeling, Convoke, enter as copy of creature with changeling (clone effect)
fn omni_changeling(id: ObjectId, owner: PlayerId) -> CardData {
    // {3}{U}{U} 0/0 Shapeshifter. Changeling. Convoke. Enter as copy of creature with changeling.
    CardData { id, owner, name: "Omni-Changeling".into(),
        mana_cost: ManaCost::parse("{3}{U}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Shapeshifter],
        power: Some(0), toughness: Some(0),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::CHANGELING | KeywordAbilities::CONVOKE,
        abilities: vec![
            Ability::static_ability(id,
                "You may have this creature enter as a copy of any creature on the battlefield, except it has changeling.",
                vec![StaticEffect::enter_as_a_copy("creature", &["changeling"])]),
        ],
        ..Default::default() }
}

fn overgrown_tomb(id: ObjectId, owner: PlayerId) -> CardData {
    // DONE - Land — Swamp Forest. Pay 2 life or enters tapped.
    CardData { id, owner, name: "Overgrown Tomb".into(),
        card_types: vec![CardType::Land],
        subtypes: vec![SubType::Swamp, SubType::Forest],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "As Overgrown Tomb enters, you may pay 2 life. If you don't, it enters tapped.",
                vec![StaticEffect::enters_tapped_unless("pay 2 life")]),
            Ability::mana_ability(id, "{T}: Add {B}.", Mana::black(1)),
            Ability::mana_ability(id, "{T}: Add {G}.", Mana::green(1)),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [MODAL] Choose one or both, exile 2 from opponent hand, remove all counters from creature
fn perfect_intimidation(id: ObjectId, owner: PlayerId) -> CardData {
    // {3}{B} Sorcery. Choose one or both: opponent exiles 2 from hand; or remove all counters from creature.
    CardData { id, owner, name: "Perfect Intimidation".into(),
        mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::modal(vec![
                    ModalMode::new("Target opponent exiles two cards from their hand.",
                        vec![Effect::opponent_exiles_from_hand(2)]),
                    ModalMode::new("Remove all counters from target creature.",
                        vec![Effect::remove_all_counters()]),
                ], 1, 2)],
                TargetSpec::Custom("opponent and/or creature".into())),
        ],
        ..Default::default() }
}

fn pitiless_fists(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment — Aura for {3}{G}. Enchant creature you control.
    // ETB: enchanted creature fights up to one target opponent creature. Static: +2/+2.
    CardData { id, owner, name: "Pitiless Fists".into(),
        mana_cost: ManaCost::parse("{3}{G}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Aura],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this Aura enters, enchanted creature fights up to one target creature an opponent controls.",
                vec![Effect::Fight],
                TargetSpec::OpponentCreature),
            Ability::static_ability(id,
                "Enchanted creature gets +2/+2.",
                vec![StaticEffect::Boost { filter: "enchanted creature".into(), power: 2, toughness: 2 }]),
        ],
        ..Default::default() }
}

fn prismatic_undercurrents(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Prismatic Undercurrents".into(),
        mana_cost: ManaCost::parse("{3}{G}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "Vivid — When this enters, search your library for up to X basic land cards, where X is the number of colors among permanents you control. Reveal them, put them into your hand, then shuffle.",
                    vec![Effect::search_library_vivid()],
                    TargetSpec::None),
            Ability::static_ability(id,
                    "You may play an additional land on each of your turns.",
                    vec![StaticEffect::AdditionalLandPlays { count: 1 }]),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [VIVID+CHOICE] ETB draw + choose color + become that color, activated draw if 5 colors
fn pucas_eye(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Puca's Eye".into(),
        mana_cost: ManaCost::parse("{2}"),
        card_types: vec![CardType::Artifact],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Puca's Eye enters, draw a card, then choose a color. This artifact becomes the chosen color.",
                vec![Effect::draw_cards(1), Effect::choose_color()],
                TargetSpec::None),
            Ability::activated(id,
                "{3}, {T}: Draw a card. Activate only if there are five colors among permanents you control.",
                vec![Cost::pay_mana("{3}"), Cost::TapSelf],
                vec![Effect::draw_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] Ward {2}, ETB gain X life where X=greatest power among Giants you control
// ENGINE DEPS: [COND] PARTIAL — Ward+keywords typed, ETB gain life dynamic (greatest Giant power) is Custom
fn pummeler_for_hire(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Pummeler for Hire".into(),
        mana_cost: ManaCost::parse("{4}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Giant, SubType::Mercenary],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::VIGILANCE | KeywordAbilities::REACH | KeywordAbilities::WARD,
        abilities: vec![
            Ability::static_ability(id,
                "Ward {2}",
                vec![StaticEffect::ward("{2}")]),
            Ability::enters_battlefield_triggered(id,
                "When Pummeler for Hire enters, you gain X life, where X is the greatest power among Giants you control.",
                vec![Effect::gain_life_dynamic("greatest power among Giants you control")],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn pyrrhic_strike(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Pyrrhic Strike".into(),
        mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::destroy()],
                    TargetSpec::Creature),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COST] ETB with 2 -1/-1 counters, remove 2 counters cost, return creature MV<=3 from GY
fn reaping_willow(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/6 Treefolk Cleric for {1}{W/B}{W/B}{W/B}. Lifelink. (ETB with 2 -1/-1 counters; {1}{W/B}, remove 2: reanimate creature MV<=3, sorcery)
    CardData { id, owner, name: "Reaping Willow".into(),
        mana_cost: ManaCost::parse("{1}{W/B}{W/B}{W/B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Treefolk, SubType::Cleric],
        power: Some(3), toughness: Some(6),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::LIFELINK,
        abilities: vec![
            Ability::static_ability(id,
                "Reaping Willow enters with two -1/-1 counters on it.",
                vec![StaticEffect::enters_with_counters("-1/-1", 2)]),
            Ability::activated(id,
                "{1}{W/B}, Remove two -1/-1 counters from Reaping Willow: Return target creature card with mana value 3 or less from your graveyard to the battlefield. Activate only as a sorcery.",
                vec![Cost::pay_mana("{1}{W/B}"), Cost::remove_counters("-1/-1", 2)],
                vec![Effect::reanimate()],
                TargetSpec::CardInYourGraveyard),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] Conditional dies trigger (if had -1/-1 counter), return to battlefield + lose all abilities
fn retched_wretch(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/2 Goblin {2}{B}. Dies: if had -1/-1 counter, return to BF losing all abilities.
    CardData { id, owner, name: "Retched Wretch".into(),
        mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Goblin],
        power: Some(4), toughness: Some(2),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::dies_triggered(id,
                "When this dies, if it had a -1/-1 counter on it, return it to the battlefield under its owner's control and it loses all abilities.",
                vec![Effect::reanimate(), Effect::lose_all_abilities()],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] Flash, grant persist until EOT, activated remove any number of counters (sorcery speed)
fn rhys_the_evermore(id: ObjectId, owner: PlayerId) -> CardData {
    // Legendary 2/2 Elf Warrior for {1}{W}. Flash.
    // ETB: another target creature gains persist until EOT.
    // {W}, {T}: Remove any number of counters from target creature you control. Sorcery speed.
    CardData { id, owner, name: "Rhys, the Evermore".into(),
        mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elf, SubType::Warrior],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLASH,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Rhys enters, another target creature you control gains persist until end of turn.",
                vec![Effect::gain_keyword_eot("persist")],
                TargetSpec::CreatureYouControl),
            Ability::activated(id,
                "{W}, {T}: Remove any number of counters from target creature you control. Activate only as a sorcery.",
                vec![Cost::pay_mana("{W}"), Cost::TapSelf],
                vec![Effect::remove_all_counters()],
                TargetSpec::CreatureYouControl),
        ],
        ..Default::default() }
}

fn rimefire_torque(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Rimefire Torque".into(),
        mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Artifact],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "As Rimefire Torque enters, choose a creature type.",
                    vec![Effect::ChooseCreatureType { restricted: vec![] }],
                    TargetSpec::None),
            Ability::other_creature_etb_triggered(id,
                    "Whenever a creature of the chosen type enters under your control, put a charge counter on Rimefire Torque.",
                    vec![Effect::AddCountersSelf { counter_type: "charge".into(), count: 1 }],
                    TargetSpec::None),
            Ability::activated(id,
                    "{T}, Remove three charge counters from Rimefire Torque: When you next cast an instant or sorcery spell this turn, copy that spell. You may choose new targets for the copy.",
                    vec![Cost::tap_self(), Cost::remove_counters("charge", 3)],
                    vec![Effect::copy_next_spell()],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [VIVID+IMPULSE] Vivid reveal X nonland cards, exile one per color, cast this turn
fn sanar_innovative_first_year(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sanar, Innovative First-Year".into(),
        mana_cost: ManaCost::parse("{2}{U/R}{U/R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Goblin, SubType::Sorcerer],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(4),
        rarity: Rarity::Rare,
        ..Default::default() }
}

// ENGINE DEPS: [COND] Affinity for Forests, landfall then Treefolk token, exile self then indestructible until EOT
fn sapling_nursery(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment {6}{G}{G}. Affinity for Forests. Landfall: Treefolk token. Exile: indestructible until EOT.
    CardData { id, owner, name: "Sapling Nursery".into(),
        mana_cost: ManaCost::parse("{6}{G}{G}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "Affinity for Forests.",
                vec![StaticEffect::CostReduction { filter: "Forest".into(), amount: 1, condition: None }]),
            Ability::triggered(id,
                "Landfall — Whenever a land you control enters, create a 3/4 green Treefolk creature token with reach.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::create_token("3/4 Treefolk with reach", 1)],
                TargetSpec::None),
            Ability::activated(id,
                "{1}{G}, Exile this enchantment: Treefolk and Forests you control gain indestructible until end of turn.",
                vec![Cost::pay_mana("{1}{G}"), Cost::ExileSelf],
                vec![Effect::GrantKeywordAllUntilEndOfTurn { filter: "Treefolk you control".into(), keyword: "indestructible".into() }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn selfless_safewright(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/2 Elf Warrior for {3}{G}{G}. Flash, Convoke.
    // ETB: choose a creature type. Other permanents of that type gain hexproof+indestructible until EOT.
    CardData { id, owner, name: "Selfless Safewright".into(),
        mana_cost: ManaCost::parse("{3}{G}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elf, SubType::Warrior],
        power: Some(4), toughness: Some(2),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLASH | KeywordAbilities::CONVOKE,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, choose a creature type. Other permanents you control of the chosen type gain hexproof and indestructible until end of turn.",
                vec![Effect::choose_type_and_grant_keywords(vec!["hexproof", "indestructible"], true)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn shadow_urchin(id: ObjectId, owner: PlayerId) -> CardData {
    let mut dies_trigger = Ability::triggered(id,
        "Whenever a creature you control with one or more counters on it dies, exile that many cards from the top of your library. Until your next end step, you may play those cards.",
        vec![EventType::Dies],
        vec![Effect::ExileTopAndPlay { count: X_VALUE, duration: "until_end_of_next_turn".into(), without_mana: false }],
        TargetSpec::None);
    dies_trigger.trigger_scope = TriggerScope::OtherControlled;
    CardData { id, owner, name: "Shadow Urchin".into(),
        mana_cost: ManaCost::parse("{2}{B/R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Ouphe],
        power: Some(3), toughness: Some(4),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever Shadow Urchin attacks, blight 1.",
                    vec![EventType::AttackerDeclared],
                    vec![Effect::BlightOpponents { count: 1 }],
                    TargetSpec::None),
            dies_trigger,
        ],
        ..Default::default() }
}

fn shimmerwilds_growth(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment — Aura for {1}{G}. Enchant land.
    // As enters, choose a color. Enchanted land tapped for mana produces additional mana of chosen color.
    CardData { id, owner, name: "Shimmerwilds Growth".into(),
        mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Aura],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "As this Aura enters, choose a color.",
                vec![Effect::choose_color()],
                TargetSpec::None),
            Ability::static_ability(id,
                "Whenever enchanted land is tapped for mana, its controller adds one additional mana of the chosen color.",
                vec![StaticEffect::enhanced_mana_production()]),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COPY] Flying, Wither, copy instant/sorcery with single target + both gain wither
fn spinerock_tyrant(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Spinerock Tyrant".into(),
        mana_cost: ManaCost::parse("{3}{R}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon],
        power: Some(6), toughness: Some(6),
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::WITHER,
        abilities: vec![
            Ability::spell_cast_triggered(id,
                    "Whenever you cast an instant or sorcery spell that targets only a single target, you may copy that spell. You may choose new targets for the copy. Both spells gain wither.",
                    vec![Effect::copy_triggering_spell(vec!["wither"], true)],
                    TargetSpec::None).set_optional(),
        ],
        ..Default::default() }
}

fn spiral_into_solitude(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment — Aura for {1}{W}. Enchant creature. Can't attack or block.
    // {1}{W}, Blight 1, Sacrifice: Exile enchanted creature.
    CardData { id, owner, name: "Spiral into Solitude".into(),
        mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Aura],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "Enchanted creature can't attack or block.",
                vec![StaticEffect::CantAttack { filter: "enchanted creature".into() },
                     StaticEffect::CantBlock { filter: "enchanted creature".into() }]),
            Ability::activated(id,
                "{1}{W}, Blight 1, Sacrifice this Aura: Exile enchanted creature.",
                vec![Cost::pay_mana("{1}{W}"), Cost::Blight(1), Cost::sacrifice_self()],
                vec![Effect::exile()],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn spry_and_mighty(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Spry and Mighty".into(),
        mana_cost: ManaCost::parse("{4}{G}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::compare_and_boost()],
                    TargetSpec::PermanentFiltered("two creatures you control".into())),
        ],
        ..Default::default() }
}
// ENGINE DEPS: [EQUIP] ETB: Shapeshifter token with changeling. Equip {2}, equipped gets +1/+1 + all types.
fn stalactite_dagger(id: ObjectId, owner: PlayerId) -> CardData {
    // Artifact Equipment {2}. ETB: create 1/1 Shapeshifter with changeling. Equip: +1/+1 + all types.
    CardData { id, owner, name: "Stalactite Dagger".into(),
        mana_cost: ManaCost::parse("{2}"),
        card_types: vec![CardType::Artifact],
        subtypes: vec![SubType::Equipment],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this Equipment enters, create a 1/1 colorless Shapeshifter creature token with changeling.",
                vec![Effect::create_token("1/1 Shapeshifter with changeling", 1)],
                TargetSpec::None),
            Ability::static_ability(id,
                "Equipped creature gets +1/+1 and is all creature types.",
                vec![StaticEffect::Boost { filter: "equipped creature".into(), power: 1, toughness: 1 }]),
            Ability::activated(id,
                "Equip {2}",
                vec![Cost::pay_mana("{2}")],
                vec![Effect::equip()],
                TargetSpec::CreatureYouControl),
        ],
        ..Default::default() }
}

fn steam_vents(id: ObjectId, owner: PlayerId) -> CardData {
    // DONE - Land — Island Mountain. Pay 2 life or enters tapped.
    CardData { id, owner, name: "Steam Vents".into(),
        card_types: vec![CardType::Land],
        subtypes: vec![SubType::Island, SubType::Mountain],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "As Steam Vents enters, you may pay 2 life. If you don't, it enters tapped.",
                vec![StaticEffect::enters_tapped_unless("pay 2 life")]),
            Ability::mana_ability(id, "{T}: Add {U}.", Mana::blue(1)),
            Ability::mana_ability(id, "{T}: Add {R}.", Mana::red(1)),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] Cost reduction by greatest MV among Elementals, if cast then bounce all non-Elemental creatures
fn sunderflock(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sunderflock".into(),
        mana_cost: ManaCost::parse("{7}{U}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elemental],
        power: Some(5), toughness: Some(5),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::static_ability(id,
                    "This spell costs {X} less to cast, where X is the greatest mana value among Elementals you control.",
                    vec![StaticEffect::cost_reduction_dynamic("creature spells", "greatest mana value among Elementals you control")]),
            Ability::enters_battlefield_triggered(id,
                    "When Sunderflock enters, if you cast it, return each non-Elemental creature to its owner's hand.",
                    vec![Effect::bounce_all("non-Elemental creatures")],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [TRANSFORM] Transform/DFC, can't be blocked, grant combat-damage-draw, protection from colors
fn sygg_wanderwine_wisdom(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sygg, Wanderwine Wisdom".into(),
        card_types: vec![CardType::Creature],
        supertypes: vec![SuperType::Legendary],
        rarity: Rarity::Rare,
        ..Default::default() }
}

fn syggs_command(id: ObjectId, owner: PlayerId) -> CardData {
    // {1}{W}{U} Kindred Sorcery — Merfolk. Choose two of 4 modes.
    CardData { id, owner, name: "Sygg's Command".into(),
        mana_cost: ManaCost::parse("{1}{W}{U}"),
        card_types: vec![CardType::Kindred, CardType::Sorcery],
        subtypes: vec![SubType::Merfolk],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::modal(vec![
                    ModalMode::new("Create a token that's a copy of target Merfolk you control.",
                        vec![Effect::create_token_copy(1)]),
                    ModalMode::new("Creatures target player controls gain lifelink until end of turn.",
                        vec![Effect::grant_keyword_all_eot("creatures target player controls", "lifelink")]),
                    ModalMode::new("Target player draws a card.",
                        vec![Effect::draw_cards(1)]),
                    ModalMode::new("Tap target creature. Put a stun counter on it.",
                        vec![Effect::TapTarget, Effect::add_counters("stun", 1)]),
                ], 2, 2)],
                TargetSpec::Custom("various".into())),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] Other creatures hexproof from each of their colors, make creature all colors
fn tam_mindful_first_year(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Tam, Mindful First-Year".into(),
        mana_cost: ManaCost::parse("{1}{G/U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Gorgon, SubType::Wizard],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                    "Each other creature you control has hexproof from each of its colors.",
                    vec![StaticEffect::Custom("Each other creature you control has hexproof from each of its colors.".into())]),
            Ability::activated(id,
                    "{T}: Target creature becomes all colors until end of turn.",
                    vec![Cost::tap_self()],
                    vec![Effect::become_all_colors()],
                    TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn taster_of_wares(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Taster of Wares".into(),
        mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Goblin, SubType::Warlock],
        power: Some(3), toughness: Some(2),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this creature enters, target opponent reveals X cards from their hand, where X is the number of Goblins you control. You choose one of those cards. That player exiles it. If an instant or sorcery card is exiled this way, you may cast it for as long as you control this creature, and mana of any type can be spent to cast that spell.",
                    vec![Effect::opponent_reveals_from_hand_exile_cast("Goblins you control", true)],
                    TargetSpec::Player),
        ],
        ..Default::default() }
}

fn temple_garden(id: ObjectId, owner: PlayerId) -> CardData {
    // DONE - Land — Forest Plains. Pay 2 life or enters tapped.
    CardData { id, owner, name: "Temple Garden".into(),
        card_types: vec![CardType::Land],
        subtypes: vec![SubType::Forest, SubType::Plains],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "As Temple Garden enters, you may pay 2 life. If you don't, it enters tapped.",
                vec![StaticEffect::enters_tapped_unless("pay 2 life")]),
            Ability::mana_ability(id, "{T}: Add {G}.", Mana::green(1)),
            Ability::mana_ability(id, "{T}: Add {W}.", Mana::white(1)),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [TRANSFORM] Transform/DFC, mill + conditional gain life, exile Elf for opponents lose life
fn trystan_callous_cultivator(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Trystan, Callous Cultivator".into(),
        card_types: vec![CardType::Creature],
        supertypes: vec![SuperType::Legendary],
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::DEATHTOUCH | KeywordAbilities::DEATHTOUCH,
        ..Default::default() }
}

fn trystans_command(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Trystan's Command".into(),
        mana_cost: ManaCost::parse("{4}{B}{G}"),
        card_types: vec![CardType::Kindred, CardType::Sorcery],
        subtypes: vec![SubType::Elf],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::destroy()],
                    TargetSpec::Creature),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND+COPY] ETB surveil 2, creatures from GY entering then create token copy (once per turn)
// ENGINE DEPS: [COND+COPY] ETB surveil 2 (approx scry), GY creature trigger creates token copy (once/turn)
fn twilight_diviner(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Twilight Diviner".into(),
        mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elf, SubType::Cleric],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, surveil 2.",
                vec![Effect::scry(2)],
                TargetSpec::None),
            Ability::other_creature_etb_from_graveyard_triggered(id,
                "Whenever one or more other creatures you control enter, if they entered from a graveyard, create a token that's a copy of one of them. This ability triggers only once each turn.",
                vec![Effect::create_token_copy_of_triggering()],
                TargetSpec::None).set_once_per_turn(),
        ],
        ..Default::default() }
}

fn twinflame_travelers(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Twinflame Travelers".into(),
        mana_cost: ManaCost::parse("{2}{U}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elemental, SubType::Sorcerer],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::static_ability(id,
                "Whenever a triggered ability of another Elemental you control triggers, it triggers an additional time.",
                vec![StaticEffect::trigger_doubling("other Elementals you control")]),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [MODAL+TYPE] Choose one: return creature from GY; or return 2 creatures sharing type from GY
fn unbury(id: ObjectId, owner: PlayerId) -> CardData {
    // {1}{B} Instant. Choose one: return creature from GY to hand; or return 2 sharing type from GY to hand.
    CardData { id, owner, name: "Unbury".into(),
        mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::modal(vec![
                    ModalMode::new("Return target creature card from your graveyard to your hand.",
                        vec![Effect::ReturnFromGraveyard]),
                    ModalMode::new("Return two target creature cards that share a creature type from your graveyard to your hand.",
                        vec![Effect::ReturnFromGraveyard, Effect::ReturnFromGraveyard]),
                ], 1, 1)],
                TargetSpec::CardInYourGraveyard),
        ],
        ..Default::default() }
}

fn unforgiving_aim(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Unforgiving Aim".into(),
        mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::destroy()],
                    TargetSpec::Creature),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [EVOKE+COND] Evoke, conditional ETB (if RR then 3 damage, if GG then search land + gain 2 life)
fn vibrance(id: ObjectId, owner: PlayerId) -> CardData {
    // {3}{R/G}{R/G} 4/4 Elemental Incarnation. Conditional ETBs + Evoke {R/G}{R/G}
    CardData { id, owner, name: "Vibrance".into(),
        mana_cost: ManaCost::parse("{3}{R/G}{R/G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elemental, SubType::Incarnation],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Mythic,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this enters, if {R}{R} was spent to cast it, deal 3 damage to any target.",
                vec![Effect::deal_damage(3)],
                TargetSpec::CreatureOrPlayer),
            Ability::enters_battlefield_triggered(id,
                "When this enters, if {G}{G} was spent, search your library for a land card, put it in hand. You gain 2 life.",
                vec![Effect::search_library("land"), Effect::gain_life(2)],
                TargetSpec::None),
            Ability::static_ability(id, "Evoke {R/G}{R/G}",
                vec![StaticEffect::evoke("{R/G}{R/G}")]),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] Convoke, bounce 1-2 nonland permanents, conditional Merfolk tokens
fn wanderwine_farewell(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Wanderwine Farewell".into(),
        mana_cost: ManaCost::parse("{5}{U}{U}"),
        card_types: vec![CardType::Kindred, CardType::Sorcery],
        subtypes: vec![SubType::Merfolk],
        keywords: KeywordAbilities::CONVOKE,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::bounce(),
                     Effect::conditional("you control a Merfolk",
                         vec![Effect::create_token("1/1 white and blue Merfolk creature token", 1)], vec![])],
                TargetSpec::PermanentFiltered("nonland permanent".into())),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] End step trigger if another creature entered this turn then surveil 1
// ENGINE DEPS: [COND] PARTIAL — surveil as scry, conditional creature-entry check not enforced
fn wary_farmer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Wary Farmer".into(),
        mana_cost: ManaCost::parse("{1}{G/W}{G/W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Kithkin, SubType::Citizen],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::beginning_of_end_step_triggered(id,
                "At the beginning of your end step, if another creature entered the battlefield under your control this turn, surveil 1.",
                vec![Effect::scry(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [VIVID] Vivid cost reduction, Reach, Trample
fn wildvine_pummeler(id: ObjectId, owner: PlayerId) -> CardData {
    // {6}{G} 6/5 Giant Berserker. Vivid cost reduction. Reach. Trample.
    CardData { id, owner, name: "Wildvine Pummeler".into(),
        mana_cost: ManaCost::parse("{6}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Giant, SubType::Berserker],
        power: Some(6), toughness: Some(5),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::REACH | KeywordAbilities::TRAMPLE,
        abilities: vec![
            Ability::static_ability(id,
                "Vivid -- This spell costs {1} less for each color among permanents you control.",
                vec![StaticEffect::CostReduction { filter: "self".into(), amount: 1, condition: None }]),
        ],
        ..Default::default() }
}

fn winnowing(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Winnowing".into(),
        mana_cost: ManaCost::parse("{4}{W}{W}"),
        card_types: vec![CardType::Sorcery],
        keywords: KeywordAbilities::CONVOKE,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::winnowing()],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [EVOKE+COND] Evoke, conditional ETB (if GG exile artifact/enchantment, if UU draw 2 discard 1)
fn wistfulness(id: ObjectId, owner: PlayerId) -> CardData {
    // {3}{G/U}{G/U} 6/5 Elemental Incarnation. Conditional ETBs + Evoke {G/U}{G/U}
    CardData { id, owner, name: "Wistfulness".into(),
        mana_cost: ManaCost::parse("{3}{G/U}{G/U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elemental, SubType::Incarnation],
        power: Some(6), toughness: Some(5),
        rarity: Rarity::Mythic,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this enters, if {G}{G} was spent, exile target artifact or enchantment an opponent controls.",
                vec![Effect::exile()],
                TargetSpec::Permanent),
            Ability::enters_battlefield_triggered(id,
                "When this enters, if {U}{U} was spent, draw two cards, then discard a card.",
                vec![Effect::draw_cards(2), Effect::discard_cards(1)],
                TargetSpec::None),
            Ability::static_ability(id, "Evoke {G/U}{G/U}",
                vec![StaticEffect::evoke("{G/U}{G/U}")]),
        ],
        ..Default::default() }
}



// ── New ECL card factory functions ─────────────────────────────────────

fn ajani_outland_chaperone(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ajani, Outland Chaperone".into(), mana_cost: ManaCost::parse("{1}{W}{W}"),
        card_types: vec![CardType::Planeswalker],
        subtypes: vec![SubType::PwAjani],
        supertypes: vec![SuperType::Legendary],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::create_token("1/1 Kithkin", 1)],
                TargetSpec::None),
            Ability::spell(id,
                vec![Effect::deal_damage(4)],
                TargetSpec::PermanentFiltered("tapped creature".into())),
            Ability::spell(id,
                vec![Effect::Custom("−8: Look at top X cards where X is your life total. Put any number of nonland permanents MV<=3 onto the battlefield.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COST] Blight cost (OrCost: blight 1 or pay {3}), exile target creature
fn bogslithers_embrace(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery for {1}{B}. (Additional cost: blight 1 or pay {3}; exile target creature)
    CardData { id, owner, name: "Bogslither's Embrace".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::exile()],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn boneclub_berserker(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Boneclub Berserker".into(), mana_cost: ManaCost::parse("{3}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Goblin, SubType::Berserker],
        power: Some(2), toughness: Some(4),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id, "This creature gets +2/+0 for each other Goblin you control.",
                vec![StaticEffect::BoostPerCount { count_filter: "other Goblin you control".into(), power_per: 2, toughness_per: 0 }]),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] Multi-target damage split (2 to one target, 1 to another)
fn boulder_dash(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Boulder Dash".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::deal_damage(2), Effect::deal_damage(1)],
                TargetSpec::Pair {
                    first: Box::new(TargetSpec::CreatureOrPlayer),
                    second: Box::new(TargetSpec::CreatureOrPlayer),
                }),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [BEHOLD+COST] Behold+exile cost, tap+stun counter on ETB/tap, LTB return exiled card
fn champions_of_the_shoal(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Champions of the Shoal".into(), mana_cost: ManaCost::parse("{3}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Merfolk, SubType::Soldier],
        power: Some(4), toughness: Some(6),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever this creature enters or becomes tapped, tap up to one target creature and put a stun counter on it.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::tap_target(), Effect::add_counters("stun", 1)],
                TargetSpec::Creature),
        ],
        additional_costs: vec![Cost::behold_and_exile("Merfolk")],
        ..Default::default() }
}

fn clachan_festival(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Clachan Festival".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Kindred, CardType::Enchantment],
        subtypes: vec![SubType::Kithkin],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When this enchantment enters, create two 1/1 green and white Kithkin creature tokens.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::create_token("1/1 Kithkin", 2)],
                TargetSpec::None),
            Ability::activated(id,
                "{4}{W}: Create a 1/1 green and white Kithkin creature token.",
                vec![Cost::pay_mana("{4}{W}")],
                vec![Effect::create_token("1/1 Kithkin", 1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn creakwood_safewright(id: ObjectId, owner: PlayerId) -> CardData {
    // 5/5 Elf Warrior for {1}{B}. (ETB with 3 -1/-1 counters; end step: if Elf in GY and has counter, remove one)
    CardData { id, owner, name: "Creakwood Safewright".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elf, SubType::Warrior],
        power: Some(5), toughness: Some(5),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "Creakwood Safewright enters with three -1/-1 counters on it.",
                vec![StaticEffect::enters_with_counters("-1/-1", 3)]),
            Ability::beginning_of_end_step_triggered(id,
                "At the beginning of your end step, if there is an Elf card in your graveyard and this creature has a -1/-1 counter on it, remove a -1/-1 counter from this creature.",
                vec![Effect::RemoveCounters { counter_type: "-1/-1".into(), count: 1 }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] PARTIAL — mill+drain typed, conditional Elf check not enforced
fn dawnhand_eulogist(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Dawnhand Eulogist".into(), mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elf, SubType::Warlock],
        power: Some(3), toughness: Some(3),
        keywords: KeywordAbilities::MENACE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Dawnhand Eulogist enters, mill three cards. Then if there is an Elf card in your graveyard, each opponent loses 2 life and you gain 2 life.",
                vec![Effect::mill(3), Effect::lose_life_opponents(2), Effect::gain_life(2)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [EXILE_CAST] Exile from opponent libraries until MV>=5, cast from exile without paying costs until EOT
fn dream_harvest(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Dream Harvest".into(), mana_cost: ManaCost::parse("{5}{U/B}{U/B}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::opponents_exile_until_mv_and_cast(5)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn end_blaze_epiphany(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "End-Blaze Epiphany".into(), mana_cost: ManaCost::parse("{X}{R}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::deal_damage_with_delayed_exile()],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [MANA] Conditional mana (2 any color, only for Elemental spells/abilities)
fn flamebraider(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Elemental Bard {1}{R}. T: Add two mana in any combination of colors (only for Elementals).
    CardData { id, owner, name: "Flamebraider".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elemental, SubType::Bard],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::mana_ability(id,
                "{T}: Add two mana in any combination of colors. Spend this mana only to cast Elemental spells or activate abilities of Elementals.",
                Mana::red(2)),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] ETB may discard to search for creature, activated untap another permanent
fn formidable_speaker(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/4 Elf Druid for {2}{G}. ETB: may discard to search creature. {1}, T: untap another permanent.
    CardData { id, owner, name: "Formidable Speaker".into(), mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elf, SubType::Druid],
        power: Some(2), toughness: Some(4),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this enters, you may discard a card. If you do, search your library for a creature card, reveal it, put it into your hand, then shuffle.",
                vec![Effect::do_if_cost_paid(
                    Cost::Discard(1),
                    vec![Effect::search_library("creature")],
                    vec![],
                )],
                TargetSpec::None),
            Ability::activated(id,
                "{1}, {T}: Untap another target permanent.",
                vec![Cost::pay_mana("{1}"), Cost::tap_self()],
                vec![Effect::untap_target()],
                TargetSpec::Permanent),
        ],
        ..Default::default() }
}

fn glen_elendras_answer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Glen Elendra's Answer".into(), mana_cost: ManaCost::parse("{2}{U}{U}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Mythic,
        abilities: vec![
            Ability::static_ability(id,
                "This spell can't be countered.",
                vec![StaticEffect::CantBeCountered]),
            Ability::spell(id,
                vec![Effect::counter_all_opponent_spells_and_abilities("1/1 Faerie with flying")],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn gloom_ripper(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Gloom Ripper".into(), mana_cost: ManaCost::parse("{3}{B}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elf, SubType::Assassin],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, target creature you control gets +X/+0 until end of turn and up to one target creature an opponent controls gets -0/-X until end of turn, where X is the number of Elves you control plus the number of Elf cards in your graveyard.",
                vec![Effect::boost_dual_target_dynamic("Elves you control + Elf cards in your graveyard")],
                TargetSpec::Pair {
                    first: Box::new(TargetSpec::CreatureYouControl),
                    second: Box::new(TargetSpec::OpponentCreature),
                }),
        ],
        ..Default::default() }
}

fn goatnap(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Goatnap".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::gain_control_eot(), Effect::untap_target(), Effect::gain_keyword_eot("haste"),
                     Effect::conditional("target is a Goat", vec![Effect::boost_until_eot(3, 0)], vec![])],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] Attacks trigger may tap another creature for unblockable this turn
fn gravelgill_scoundrel(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/3 Merfolk Rogue for {1}{U}. Vigilance.
    // Whenever this creature attacks, you may tap another untapped creature you control.
    // If you do, this creature cannot be blocked this turn.
    CardData { id, owner, name: "Gravelgill Scoundrel".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Merfolk, SubType::Rogue],
        power: Some(1), toughness: Some(3),
        keywords: KeywordAbilities::VIGILANCE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever this creature attacks, you may tap another untapped creature you control. If you do, this creature can\x27t be blocked this turn.",
                vec![EventType::AttackerDeclared],
                vec![Effect::do_if_cost_paid(
                    Cost::tap_creatures("creature", 1),
                    vec![Effect::cant_be_blocked_eot()],
                    vec![])],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// TRIAGED: Hovel Hurler is a 6/7 Giant Warrior for {3}{R/W}{R/W}
// "This creature enters with two -1/-1 counters on it." (replacement effect, not trigger)
// "{R/W}{R/W}, Remove a counter from this creature: Another target creature you control
//  gets +1/+0 and gains flying until end of turn. Activate only as a sorcery."
// NOTE: Current stub is WRONG — card does NOT have innate flying! The flying keyword was incorrect.
// Needs: ETB-with-counters replacement, RemoveCounters cost (any counter, not typed),
//   sorcery-speed activated ability restriction, boost+grant-flying-eot on target.
// Engine has: Cost::RemoveCounters, AddCountersSelf, gain_keyword_eot, boost effects.
// Missing: ETB-with-counters as replacement (vs trigger), sorcery-speed ability restriction.
// Category: COST (Cost System — RemoveCounters)
fn hovel_hurler(id: ObjectId, owner: PlayerId) -> CardData {
    // 6/7 Giant Warrior for {3}{R/W}{R/W}. (ETB with 2 -1/-1 counters; {R/W}{R/W}, remove 1: another creature +1/+0 + flying EOT, sorcery)
    CardData { id, owner, name: "Hovel Hurler".into(), mana_cost: ManaCost::parse("{3}{R/W}{R/W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Giant, SubType::Warrior],
        power: Some(6), toughness: Some(7),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "Hovel Hurler enters with two -1/-1 counters on it.",
                vec![StaticEffect::enters_with_counters("-1/-1", 2)]),
            Ability::activated(id,
                "{R/W}{R/W}, Remove a -1/-1 counter from Hovel Hurler: Target other creature you control gets +1/+0 and gains flying until end of turn. Activate only as a sorcery.",
                vec![Cost::pay_mana("{R/W}{R/W}"), Cost::remove_counters("-1/-1", 1)],
                vec![Effect::boost_until_eot(1, 0), Effect::gain_keyword_eot("flying")],
                TargetSpec::CreatureYouControl),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] DONE - Target creature gains trample+haste until EOT, draw a card
fn impolite_entrance(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Impolite Entrance".into(), mana_cost: ManaCost::parse("{R}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::gain_keyword_eot("trample"), Effect::gain_keyword_eot("haste"), Effect::draw_cards(1)],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}














// ENGINE DEPS: [COPY+BEHOLD] Token copy of creature with haste + end-step sacrifice, Flashback with behold 3 Elementals
fn kindle_the_inner_flame(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Kindle the Inner Flame".into(), mana_cost: ManaCost::parse("{3}{R}"),
        card_types: vec![CardType::Kindred, CardType::Sorcery],
        subtypes: vec![SubType::Elemental],
        keywords: KeywordAbilities::HASTE,
        flashback_cost: Some(ManaCost::parse("{1}{R}")),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Create a token that's a copy of target creature you control, except it has haste and \"At the beginning of the end step, sacrifice this token.\"",
                vec![EventType::EndStep],
                vec![Effect::create_token_copy_haste_sacrifice(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COPY] Tap 2 creatures cost, copy target triggered ability, once per turn
fn kirol_attentive_first_year(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Kirol, Attentive First-Year".into(), mana_cost: ManaCost::parse("{1}{R/W}{R/W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Vampire, SubType::Cleric],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Rare,
        ..Default::default() }
}

// ENGINE DEPS: [VIVID+COND] Vivid (X = colors among permanents), create X Kithkin tokens, tap 3 creatures then +3/+0 + flying
fn kithkeeper(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/3 Elemental for {6}{W}. Vivid ETB: create X 1/1 Kithkin tokens.
    // Tap 3 creatures: +3/+0 and flying until EOT.
    CardData { id, owner, name: "Kithkeeper".into(), mana_cost: ManaCost::parse("{6}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elemental],
        power: Some(3), toughness: Some(3),
        keywords: KeywordAbilities::empty(),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Vivid — When this creature enters, create X 1/1 green and white Kithkin creature tokens, where X is the number of colors among permanents you control.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::create_token_vivid("1/1 Kithkin")],
                TargetSpec::None),
            Ability::activated(id,
                "Tap three untapped creatures you control: This creature gets +3/+0 and gains flying until end of turn.",
                vec![Cost::tap_creatures("creature", 3)],
                vec![Effect::boost_until_eot(3, 0), Effect::gain_keyword_eot("flying")],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND+MANA] All creatures have haste (global static), basic land mana doubling
fn lavaleaper(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/4 Elemental {3}{R}. Haste. All creatures have haste. Basic lands you control tap for extra mana.
    CardData { id, owner, name: "Lavaleaper".into(), mana_cost: ManaCost::parse("{3}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elemental],
        power: Some(4), toughness: Some(4),
        keywords: KeywordAbilities::HASTE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "All creatures have haste.",
                vec![StaticEffect::GrantKeyword { filter: "creature".into(), keyword: "haste".into() }]),
            Ability::static_ability(id,
                "Whenever a player taps a basic land for mana, that player adds one mana of any type that land produced.",
                vec![StaticEffect::mana_doubling_basic_lands()]),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [BEHOLD+MANA] Behold Elf or pay {2}, conditional mana (GG if Elf in GY)
fn lys_alana_dignitary(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Lys Alana Dignitary".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elf, SubType::Advisor],
        power: Some(2), toughness: Some(3),
        rarity: Rarity::Common,
        additional_costs: vec![Cost::behold_or_pay("Elf", "{2}")],
        ..Default::default() }
}

// ENGINE DEPS: [COND] Attacks then may tap Merfolk then return creature MV<=3 from GY to battlefield
fn meanders_guide(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Meanders Guide".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Merfolk, SubType::Scout],
        power: Some(3), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::attacks_triggered(id,
                "Whenever this creature attacks, you may tap another untapped Merfolk you control. When you do, return target creature card with mana value 3 or less from your graveyard to the battlefield.",
                vec![Effect::tap_target()],
                TargetSpec::PermanentFiltered("another untapped Merfolk you control".into()))
                .set_optional(),
        ],
        ..Default::default() }
}

fn mirrorform(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Mirrorform".into(), mana_cost: ManaCost::parse("{4}{U}{U}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::mass_become_copy()],
                TargetSpec::PermanentFiltered("non-Aura permanent".into())),
        ],
        ..Default::default() }
}

fn mistmeadow_council(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Mistmeadow Council".into(), mana_cost: ManaCost::parse("{4}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Kithkin, SubType::Advisor],
        power: Some(4), toughness: Some(3),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When this creature enters, draw a card.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::draw_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn moon_vigil_adherents(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Moon-Vigil Adherents".into(), mana_cost: ManaCost::parse("{2}{G}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elf, SubType::Druid],
        power: Some(0), toughness: Some(0),
        keywords: KeywordAbilities::TRAMPLE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id, "This creature gets +1/+1 for each creature you control and each creature card in your graveyard.",
                vec![StaticEffect::BoostPerCount { count_filter: "creature you control and creature card in your graveyard".into(), power_per: 1, toughness_per: 1 }]),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] PARTIAL — surveil as scry, activated X=Elves-in-GY is Custom
fn morcants_eyes(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Morcant's Eyes".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Kindred, CardType::Enchantment],
        subtypes: vec![SubType::Elf],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::beginning_of_upkeep_triggered(id,
                "At the beginning of your upkeep, surveil 1.",
                vec![Effect::scry(1)],
                TargetSpec::None),
            Ability::activated(id,
                "{4}{G}{G}, Sacrifice Morcant's Eyes: Create X 2/2 black and green Elf creature tokens, where X is the number of Elf cards in your graveyard. Activate only as a sorcery.",
                vec![Cost::pay_mana("{4}{G}{G}"), Cost::SacrificeSelf],
                vec![Effect::create_token_dynamic("2/2 green Elf Warrior creature token", "Elf cards in your graveyard")],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] Mass flicker, prevent damage until next turn, exile self
fn morningtides_light(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Morningtide's Light".into(), mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::flicker_end_step()],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [BEHOLD+COND] Behold Goblin or pay {2}, can't block, dies then destroy opponent creature power<=2
fn mudbutton_cursetosser(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/1 Goblin Warlock for {B}. Behold Goblin or pay {2}. Can't block.
    // Dies: destroy target opponent creature power<=2.
    CardData { id, owner, name: "Mudbutton Cursetosser".into(), mana_cost: ManaCost::parse("{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Goblin, SubType::Warlock],
        power: Some(2), toughness: Some(1),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "This creature can't block.",
                vec![StaticEffect::CantBlock { filter: "self".into() }]),
            Ability::dies_triggered(id,
                "When this creature dies, destroy target creature an opponent controls with power 2 or less.",
                vec![Effect::destroy()],
                TargetSpec::PermanentFiltered("creature an opponent controls with power 2 or less".into())),
        ],
        additional_costs: vec![Cost::behold_or_pay("Goblin", "{2}")],
        ..Default::default() }
}

// ENGINE DEPS: [TRANSFORM+PW] Transform/DFC Planeswalker, loyalty abilities, mill, token creation, emblem
fn oko_lorwyn_liege(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Oko, Lorwyn Liege".into(),
        card_types: vec![CardType::Creature],
        supertypes: vec![SuperType::Legendary],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                "At the beginning of your first main phase, you may pay {G}. If you do, transform Oko.",
                vec![EventType::PrecombatMainPre],
                vec![Effect::do_if_cost_paid(
                    Cost::pay_mana("{G}"),
                    vec![Effect::transform_self()],
                    vec![],
                )],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] Flicker (exile+return own creature), create 1/1 changeling token
// ENGINE DEPS: [COND] PARTIAL — token typed, flicker is Custom
fn personify(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Personify".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::flicker(),
                     Effect::create_token("1/1 Shapeshifter with changeling", 1)],
                TargetSpec::CreatureYouControl),
        ],
        ..Default::default() }
}

fn raiding_schemes(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Raiding Schemes".into(), mana_cost: ManaCost::parse("{3}{R}{G}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "Each noncreature spell you cast has conspire.",
                vec![StaticEffect::grant_conspire("noncreature spells")]),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COST] Optional blight 1, destroy creature MV<=2, conditional gain 2 life if blighted
fn requiting_hex(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Requiting Hex".into(), mana_cost: ManaCost::parse("{B}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::destroy(), Effect::do_if_cost_paid(Cost::Blight(1), vec![Effect::gain_life(2)], vec![])],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] DONE - +2/+2 + first strike until EOT + untap target creature
fn riverguards_reflexes(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Riverguard's Reflexes".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::boost_until_eot(2, 2), Effect::gain_keyword_eot("first_strike"), Effect::untap_target()],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn slumbering_walker(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/7 Giant Warrior for {3}{W}{W}. ETB with 2 -1/-1 counters.
    // End step: remove counter, then reanimate creature with power<=2.
    CardData { id, owner, name: "Slumbering Walker".into(), mana_cost: ManaCost::parse("{3}{W}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Giant, SubType::Warrior],
        power: Some(4), toughness: Some(7),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "Slumbering Walker enters with two -1/-1 counters on it.",
                vec![StaticEffect::enters_with_counters("-1/-1", 2)]),
            Ability::triggered(id,
                "At the beginning of your end step, you may remove a counter from this creature. When you do, return target creature card with power 2 or less from your graveyard to the battlefield.",
                vec![EventType::EndStep],
                vec![Effect::do_if_cost_paid(Cost::RemoveCounters("-1/-1".into(), 1), vec![Effect::reanimate()], vec![])],
                TargetSpec::CardInYourGraveyard),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [BEHOLD+COND] Behold Elemental or pay {2}, grant trample, 3rd resolution adds RRRR
fn soulbright_seeker(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/1 Elemental Sorcerer for {R}. Trample. Behold Elemental or pay {2}.
    // {R}: Target creature gains trample until EOT. 3rd time adds {R}{R}{R}{R}.
    CardData { id, owner, name: "Soulbright Seeker".into(), mana_cost: ManaCost::parse("{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elemental, SubType::Sorcerer],
        power: Some(2), toughness: Some(1),
        keywords: KeywordAbilities::TRAMPLE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{R}: Target creature you control gains trample until end of turn. If this is the third time this ability has resolved this turn, add {R}{R}{R}{R}.",
                vec![Cost::pay_mana("{R}")],
                vec![Effect::gain_keyword_eot("trample"), Effect::if_resolved_n_times(3, vec![Effect::add_mana(Mana::red(4))])],
                TargetSpec::CreatureYouControl),
        ],
        additional_costs: vec![Cost::behold_or_pay("Elemental", "{2}")],
        ..Default::default() }
}

// ENGINE DEPS: [COND] Cost reduction if creature attacking you, put spell/creature on top/bottom of library
fn swat_away(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Swat Away".into(), mana_cost: ManaCost::parse("{2}{U}{U}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "This spell costs {2} less to cast if a creature is attacking you.",
                vec![StaticEffect::CostReduction { filter: "self if creature attacking you".into(), amount: 2, condition: None }]),
            Ability::spell(id,
                vec![Effect::PutOnLibrary],
                TargetSpec::PermanentFiltered("spell or creature".into())),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] Search basic land to battlefield tapped, conditional create Treefolk token if 7+ lands/Treefolk
// ENGINE DEPS: [COND] Search basic land + conditional Treefolk token
fn tend_the_sprigs(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Tend the Sprigs".into(), mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::search_library("basic land"),
                     Effect::conditional("you control 7 or more lands and/or Treefolk",
                         vec![Effect::create_token("3/4 green Treefolk creature token with reach", 1)], vec![])],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [CHOICE] Draw 3, then discard 2 unless you discard a creature card
fn thirst_for_identity(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Thirst for Identity".into(), mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::draw_cards(3), Effect::do_if_cost_paid(Cost::Discard(1), vec![], vec![Effect::discard_cards(2)])],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] DONE - Becomes tapped trigger then another Merfolk gets +2/+0 until EOT
fn tributary_vaulter(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Tributary Vaulter".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Merfolk, SubType::Warrior],
        power: Some(1), toughness: Some(3),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever Tributary Vaulter becomes tapped, another target Merfolk you control gets +2/+0 until end of turn.",
                vec![EventType::Tapped],
                vec![Effect::boost_until_eot(2, 0)],
                TargetSpec::CreatureYouControl),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] DONE - Must be blocked if able, attacks then another Elf gets +2/+1
fn vinebred_brawler(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Vinebred Brawler".into(), mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elf, SubType::Berserker],
        power: Some(4), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "Vinebred Brawler must be blocked if able.",
                vec![StaticEffect::MustBeBlocked]),
            Ability::attacks_triggered(id,
                "Whenever Vinebred Brawler attacks, another target Elf you control gets +2/+1 until end of turn.",
                vec![Effect::boost_until_eot(2, 1)],
                TargetSpec::CreatureYouControl),
        ],
        ..Default::default() }
}

// ENGINE DEPS: [COND] DONE - Activated: {1}, T, tap another creature then tap opponent creature
fn wanderbrine_trapper(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Wanderbrine Trapper".into(), mana_cost: ManaCost::parse("{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Merfolk, SubType::Scout],
        power: Some(2), toughness: Some(1),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{1}, {T}, Tap another untapped creature you control: Tap target creature an opponent controls.",
                vec![Cost::pay_mana("{1}"), Cost::tap_self(), Cost::tap_creatures("creature", 1)],
                vec![Effect::tap_target()],
                TargetSpec::OpponentCreature),
        ],
        ..Default::default() }
}
