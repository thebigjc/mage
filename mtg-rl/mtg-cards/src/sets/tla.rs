// Avatar: The Last Airbender (TLA) set — released 2025-07-25.
// 281 unique cards. Tier 1-2 creatures, spells, lands, and enchantments.

use crate::cards::basic_lands;
use crate::registry::CardRegistry;
use mtg_engine::abilities::{Ability, Cost, Effect, StaticEffect, TargetSpec};
use mtg_engine::card::CardData;
use mtg_engine::constants::*;
use mtg_engine::events::EventType;
use mtg_engine::mana::ManaCost;
use mtg_engine::types::{ObjectId, PlayerId};

pub fn register(registry: &mut CardRegistry) {
    basic_lands::register(registry, "TLA");

    registry.register("Aang's Journey", aangs_journey, "TLA");
    registry.register("Abandon Attachments", abandon_attachments, "TLA");
    registry.register("Accumulate Wisdom", accumulate_wisdom, "TLA");
    registry.register("Airbending Lesson", airbending_lesson, "TLA");
    registry.register("Allies at Last", allies_at_last, "TLA");
    registry.register("Avatar Enthusiasts", avatar_enthusiasts, "TLA");
    registry.register("Azula, On the Hunt", azula_on_the_hunt, "TLA");
    registry.register("Badgermole", badgermole, "TLA");
    registry.register("Badgermole Cub", badgermole_cub, "TLA");
    registry.register("Barrels of Blasting Jelly", barrels_of_blasting_jelly, "TLA");
    registry.register("Beetle-Headed Merchants", beetle_headed_merchants, "TLA");
    registry.register("Bender's Waterskin", benders_waterskin, "TLA");
    registry.register("Bitter Work", bitter_work, "TLA");
    registry.register("Boar-q-pine", boar_q_pine, "TLA");
    registry.register("Boomerang Basics", boomerang_basics, "TLA");
    registry.register("Callous Inspector", callous_inspector, "TLA");
    registry.register("Cat-Gator", cat_gator, "TLA");
    registry.register("Cat-Owl", cat_owl, "TLA");
    registry.register("Combustion Technique", combustion_technique, "TLA");
    registry.register("Compassionate Healer", compassionate_healer, "TLA");
    registry.register("Corrupt Court Official", corrupt_court_official, "TLA");
    registry.register("Crescent Island Temple", crescent_island_temple, "TLA");
    registry.register("Cruel Administrator", cruel_administrator, "TLA");
    registry.register("Cunning Maneuver", cunning_maneuver, "TLA");
    registry.register("Curious Farm Animals", curious_farm_animals, "TLA");
    registry.register("Cycle of Renewal", cycle_of_renewal, "TLA");
    registry.register("Dai Li Agents", dai_li_agents, "TLA");
    registry.register("Day of Black Sun", day_of_black_sun, "TLA");
    registry.register("Deadly Precision", deadly_precision, "TLA");
    registry.register("Deserter's Disciple", deserters_disciple, "TLA");
    registry.register("Earth Kingdom General", earth_kingdom_general, "TLA");
    registry.register("Earth Kingdom Jailer", earth_kingdom_jailer, "TLA");
    registry.register("Earth Kingdom Protectors", earth_kingdom_protectors, "TLA");
    registry.register("Earth Kingdom Soldier", earth_kingdom_soldier, "TLA");
    registry.register("Earth Rumble", earth_rumble, "TLA");
    registry.register("Earth Rumble Wrestlers", earth_rumble_wrestlers, "TLA");
    registry.register("Earth Village Ruffians", earth_village_ruffians, "TLA");
    registry.register("Earthbending Lesson", earthbending_lesson, "TLA");
    registry.register("Earthen Ally", earthen_ally, "TLA");
    registry.register("Elemental Teachings", elemental_teachings, "TLA");
    registry.register("Energybending", energybending, "TLA");
    registry.register("Enter the Avatar State", enter_the_avatar_state, "TLA");
    registry.register("Epic Downfall", epic_downfall, "TLA");
    registry.register("Fancy Footwork", fancy_footwork, "TLA");
    registry.register("Fatal Fissure", fatal_fissure, "TLA");
    registry.register("Fire Lord Azula", fire_lord_azula, "TLA");
    registry.register("Fire Nation Attacks", fire_nation_attacks, "TLA");
    registry.register("Fire Nation Cadets", fire_nation_cadets, "TLA");
    registry.register("Fire Nation Engineer", fire_nation_engineer, "TLA");
    registry.register("Fire Nation Raider", fire_nation_raider, "TLA");
    registry.register("Fire Sages", fire_sages, "TLA");
    registry.register("Firebending Lesson", firebending_lesson, "TLA");
    registry.register("Firebending Student", firebending_student, "TLA");
    registry.register("First-Time Flyer", first_time_flyer, "TLA");
    registry.register("Flexible Waterbender", flexible_waterbender, "TLA");
    registry.register("Flopsie, Bumi's Buddy", flopsie_bumis_buddy, "TLA");
    registry.register("Foggy Swamp Hunters", foggy_swamp_hunters, "TLA");
    registry.register("Foggy Swamp Spirit Keeper", foggy_swamp_spirit_keeper, "TLA");
    registry.register("Foggy Swamp Vinebender", foggy_swamp_vinebender, "TLA");
    registry.register("Forecasting Fortune Teller", forecasting_fortune_teller, "TLA");
    registry.register("Gather the White Lotus", gather_the_white_lotus, "TLA");
    registry.register("Geyser Leaper", geyser_leaper, "TLA");
    registry.register("Giant Koi", giant_koi, "TLA");
    registry.register("Glider Kids", glider_kids, "TLA");
    registry.register("Gran-Gran", gran_gran, "TLA");
    registry.register("Guru Pathik", guru_pathik, "TLA");
    registry.register("Haru, Hidden Talent", haru_hidden_talent, "TLA");
    registry.register("Hei Bai, Spirit of Balance", hei_bai_spirit_of_balance, "TLA");
    registry.register("Hermitic Herbalist", hermitic_herbalist, "TLA");
    registry.register("Hog-Monkey", hog_monkey, "TLA");
    registry.register("Invasion Reinforcements", invasion_reinforcements, "TLA");
    registry.register("Invasion Submersible", invasion_submersible, "TLA");
    registry.register("It'll Quench Ya!", itll_quench_ya, "TLA");
    registry.register("Jeong Jeong, the Deserter", jeong_jeong_the_deserter, "TLA");
    registry.register("Jeong Jeong's Deserters", jeong_jeongs_deserters, "TLA");
    registry.register("Jet, Freedom Fighter", jet_freedom_fighter, "TLA");
    registry.register("Jet's Brainwashing", jets_brainwashing, "TLA");
    registry.register("Joo Dee, One of Many", joo_dee_one_of_many, "TLA");
    registry.register("June, Bounty Hunter", june_bounty_hunter, "TLA");
    registry.register("Katara, Bending Prodigy", katara_bending_prodigy, "TLA");
    registry.register("Katara, the Fearless", katara_the_fearless, "TLA");
    registry.register("Kyoshi Island Plaza", kyoshi_island_plaza, "TLA");
    registry.register("Kyoshi Warriors", kyoshi_warriors, "TLA");
    registry.register("Lightning Strike", lightning_strike, "TLA");
    registry.register("Long Feng, Grand Secretariat", long_feng_grand_secretariat, "TLA");
    registry.register("Lost Days", lost_days, "TLA");
    registry.register("Mai, Jaded Edge", mai_jaded_edge, "TLA");
    registry.register("Mai, Scornful Striker", mai_scornful_striker, "TLA");
    registry.register("Master Pakku", master_pakku, "TLA");
    registry.register("Master Piandao", master_piandao, "TLA");
    registry.register("Merchant of Many Hats", merchant_of_many_hats, "TLA");
    registry.register("North Pole Patrol", north_pole_patrol, "TLA");
    registry.register("Northern Air Temple", northern_air_temple, "TLA");
    registry.register("Obsessive Pursuit", obsessive_pursuit, "TLA");
    registry.register("Octopus Form", octopus_form, "TLA");
    registry.register("Ostrich-Horse", ostrich_horse, "TLA");
    registry.register("Otter-Penguin", otter_penguin, "TLA");
    registry.register("Ozai's Cruelty", ozais_cruelty, "TLA");
    registry.register("Pillar Launch", pillar_launch, "TLA");
    registry.register("Pirate Peddlers", pirate_peddlers, "TLA");
    registry.register("Pretending Poxbearers", pretending_poxbearers, "TLA");
    registry.register("Price of Freedom", price_of_freedom, "TLA");
    registry.register("Professor Zei, Anthropologist", professor_zei_anthropologist, "TLA");
    registry.register("Rabaroo Troop", rabaroo_troop, "TLA");
    registry.register("Raucous Audience", raucous_audience, "TLA");
    registry.register("Razor Rings", razor_rings, "TLA");
    registry.register("Rebellious Captives", rebellious_captives, "TLA");
    registry.register("Redirect Lightning", redirect_lightning, "TLA");
    registry.register("Rockalanche", rockalanche, "TLA");
    registry.register("Rocky Rebuke", rocky_rebuke, "TLA");
    registry.register("Rough Rhino Cavalry", rough_rhino_cavalry, "TLA");
    registry.register("Rowdy Snowballers", rowdy_snowballers, "TLA");
    registry.register("Saber-Tooth Moose-Lion", saber_tooth_moose_lion, "TLA");
    registry.register("Seismic Sense", seismic_sense, "TLA");
    registry.register("Serpent of the Pass", serpent_of_the_pass, "TLA");
    registry.register("Shared Roots", shared_roots, "TLA");
    registry.register("Sokka, Bold Boomeranger", sokka_bold_boomeranger, "TLA");
    registry.register("Sokka, Lateral Strategist", sokka_lateral_strategist, "TLA");
    registry.register("Sokka's Haiku", sokkas_haiku, "TLA");
    registry.register("Sold Out", sold_out, "TLA");
    registry.register("South Pole Voyager", south_pole_voyager, "TLA");
    registry.register("Southern Air Temple", southern_air_temple, "TLA");
    registry.register("Spirit Water Revival", spirit_water_revival, "TLA");
    registry.register("Suki, Kyoshi Warrior", suki_kyoshi_warrior, "TLA");
    registry.register("Sun Warriors", sun_warriors, "TLA");
    registry.register("Team Avatar", team_avatar, "TLA");
    registry.register("The Boulder, Ready to Rumble", the_boulder_ready_to_rumble, "TLA");
    registry.register("The Mechanist, Aerial Artisan", the_mechanist_aerial_artisan, "TLA");
    registry.register("The Spirit Oasis", the_spirit_oasis, "TLA");
    registry.register("Tiger-Dillo", tiger_dillo, "TLA");
    registry.register("Tolls of War", tolls_of_war, "TLA");
    registry.register("Treetop Freedom Fighters", treetop_freedom_fighters, "TLA");
    registry.register("True Ancestry", true_ancestry, "TLA");
    registry.register("Turtle-Duck", turtle_duck, "TLA");
    registry.register("Ty Lee, Artful Acrobat", ty_lee_artful_acrobat, "TLA");
    registry.register("Uncle Iroh", uncle_iroh, "TLA");
    registry.register("United Front", united_front, "TLA");
    registry.register("Vengeful Villagers", vengeful_villagers, "TLA");
    registry.register("Wartime Protestors", wartime_protestors, "TLA");
    registry.register("Water Tribe Rallier", water_tribe_rallier, "TLA");
    registry.register("Waterbender Ascension", waterbender_ascension, "TLA");
    registry.register("Waterbending Lesson", waterbending_lesson, "TLA");
    registry.register("Waterbending Scroll", waterbending_scroll, "TLA");
    registry.register("White Lotus Tile", white_lotus_tile, "TLA");
    registry.register("Wolfbat", wolfbat, "TLA");
    registry.register("Yip Yip!", yip_yip, "TLA");
    registry.register("Yuyan Archers", yuyan_archers, "TLA");
    registry.register("Zuko, Exiled Prince", zuko_exiled_prince, "TLA");
    registry.register("Zuko's Conviction", zukos_conviction, "TLA");
    registry.register("Zuko's Exile", zukos_exile, "TLA");
    registry.register("Aang, at the Crossroads", aang_at_the_crossroads, "TLA");
    registry.register("Aang, Swift Savior", aang_swift_savior, "TLA");
    registry.register("Aang, the Last Airbender", aang_the_last_airbender, "TLA");
    registry.register("Aang's Iceberg", aangs_iceberg, "TLA");
    registry.register("Abandoned Air Temple", abandoned_air_temple, "TLA");
    registry.register("Agna Qel'a", agna_qela, "TLA");
    registry.register("Air Nomad Legacy", air_nomad_legacy, "TLA");
    registry.register("Airbender Ascension", airbender_ascension, "TLA");
    registry.register("Airbender's Reversal", airbenders_reversal, "TLA");
    registry.register("Airship Engine Room", airship_engine_room, "TLA");
    registry.register("Appa, Loyal Sky Bison", appa_loyal_sky_bison, "TLA");
    registry.register("Appa, Steadfast Guardian", appa_steadfast_guardian, "TLA");
    registry.register("Avatar Aang", avatar_aang, "TLA");
    registry.register("Avatar Destiny", avatar_destiny, "TLA");
    registry.register("Avatar's Wrath", avatars_wrath, "TLA");
    registry.register("Azula Always Lies", azula_always_lies, "TLA");
    registry.register("Azula, Cunning Usurper", azula_cunning_usurper, "TLA");
    registry.register("Ba Sing Se", ba_sing_se, "TLA");
    registry.register("Beifong's Bounty Hunters", beifongs_bounty_hunters, "TLA");
    registry.register("Benevolent River Spirit", benevolent_river_spirit, "TLA");
    registry.register("Boiling Rock Prison", boiling_rock_prison, "TLA");
    registry.register("Boiling Rock Rioter", boiling_rock_rioter, "TLA");
    registry.register("Bumi Bash", bumi_bash, "TLA");
    registry.register("Bumi, King of Three Trials", bumi_king_of_three_trials, "TLA");
    registry.register("Bumi, Unleashed", bumi_unleashed, "TLA");
    registry.register("Buzzard-Wasp Colony", buzzard_wasp_colony, "TLA");
    registry.register("Canyon Crawler", canyon_crawler, "TLA");
    registry.register("Combustion Man", combustion_man, "TLA");
    registry.register("Crashing Wave", crashing_wave, "TLA");
    registry.register("Dai Li Indoctrination", dai_li_indoctrination, "TLA");
    registry.register("Destined Confrontation", destined_confrontation, "TLA");
    registry.register("Diligent Zookeeper", diligent_zookeeper, "TLA");
    registry.register("Dragonfly Swarm", dragonfly_swarm, "TLA");
    registry.register("Earth King's Lieutenant", earth_kings_lieutenant, "TLA");
    registry.register("Earthbender Ascension", earthbender_ascension, "TLA");
    registry.register("Ember Island Production", ember_island_production, "TLA");
    registry.register("Fated Firepower", fated_firepower, "TLA");
    registry.register("Fire Lord Zuko", fire_lord_zuko, "TLA");
    registry.register("Fire Nation Palace", fire_nation_palace, "TLA");
    registry.register("Fire Nation Warship", fire_nation_warship, "TLA");
    registry.register("Fire Navy Trebuchet", fire_navy_trebuchet, "TLA");
    registry.register("Firebender Ascension", firebender_ascension, "TLA");
    registry.register("Foggy Bottom Swamp", foggy_bottom_swamp, "TLA");
    registry.register("Foggy Swamp Visions", foggy_swamp_visions, "TLA");
    registry.register("Glider Staff", glider_staff, "TLA");
    registry.register("Great Divide Guide", great_divide_guide, "TLA");
    registry.register("Hakoda, Selfless Commander", hakoda_selfless_commander, "TLA");
    registry.register("Hama, the Bloodbender", hama_the_bloodbender, "TLA");
    registry.register("Heartless Act", heartless_act, "TLA");
    registry.register("Honest Work", honest_work, "TLA");
    registry.register("How to Start a Riot", how_to_start_a_riot, "TLA");
    registry.register("Iguana Parrot", iguana_parrot, "TLA");
    registry.register("Invasion Tactics", invasion_tactics, "TLA");
    registry.register("Iroh, Grand Lotus", iroh_grand_lotus, "TLA");
    registry.register("Iroh, Tea Master", iroh_tea_master, "TLA");
    registry.register("Iroh's Demonstration", irohs_demonstration, "TLA");
    registry.register("Jasmine Dragon Tea Shop", jasmine_dragon_tea_shop, "TLA");
    registry.register("Katara, Water Tribe's Hope", katara_water_tribes_hope, "TLA");
    registry.register("Knowledge Seeker", knowledge_seeker, "TLA");
    registry.register("Koh, the Face Stealer", koh_the_face_stealer, "TLA");
    registry.register("Kyoshi Battle Fan", kyoshi_battle_fan, "TLA");
    registry.register("Kyoshi Village", kyoshi_village, "TLA");
    registry.register("Lo and Li, Twin Tutors", lo_and_li_twin_tutors, "TLA");
    registry.register("Meditation Pools", meditation_pools, "TLA");
    registry.register("Messenger Hawk", messenger_hawk, "TLA");
    registry.register("Meteor Sword", meteor_sword, "TLA");
    registry.register("Misty Palms Oasis", misty_palms_oasis, "TLA");
    registry.register("Momo, Friendly Flier", momo_friendly_flier, "TLA");
    registry.register("Momo, Playful Pet", momo_playful_pet, "TLA");
    registry.register("Mongoose Lizard", mongoose_lizard, "TLA");
    registry.register("North Pole Gates", north_pole_gates, "TLA");
    registry.register("Omashu City", omashu_city, "TLA");
    registry.register("Origin of Metalbending", origin_of_metalbending, "TLA");
    registry.register("Ozai, the Phoenix King", ozai_the_phoenix_king, "TLA");
    registry.register("Path to Redemption", path_to_redemption, "TLA");
    registry.register("Phoenix Fleet Airship", phoenix_fleet_airship, "TLA");
    registry.register("Planetarium of Wan Shi Tong", planetarium_of_wan_shi_tong, "TLA");
    registry.register("Platypus-Bear", platypus_bear, "TLA");
    registry.register("Ran and Shaw", ran_and_shaw, "TLA");
    registry.register("Raven Eagle", raven_eagle, "TLA");
    registry.register("Realm of Koh", realm_of_koh, "TLA");
    registry.register("Ruinous Waterbending", ruinous_waterbending, "TLA");
    registry.register("Rumble Arena", rumble_arena, "TLA");
    registry.register("Sandbender Scavengers", sandbender_scavengers, "TLA");
    registry.register("Sandbenders' Storm", sandbenders_storm, "TLA");
    registry.register("Secret Tunnel", secret_tunnel, "TLA");
    registry.register("Serpent's Pass", serpents_pass, "TLA");
    registry.register("Sokka, Tenacious Tactician", sokka_tenacious_tactician, "TLA");
    registry.register("Solstice Revelations", solstice_revelations, "TLA");
    registry.register("Sozin's Comet", sozins_comet, "TLA");
    registry.register("Sparring Dummy", sparring_dummy, "TLA");
    registry.register("Suki, Courageous Rescuer", suki_courageous_rescuer, "TLA");
    registry.register("Sun-Blessed Peak", sun_blessed_peak, "TLA");
    registry.register("Swampsnare Trap", swampsnare_trap, "TLA");
    registry.register("Teo, Spirited Glider", teo_spirited_glider, "TLA");
    registry.register("The Earth King", the_earth_king, "TLA");
    registry.register("The Fire Nation Drill", the_fire_nation_drill, "TLA");
    registry.register("The Last Agni Kai", the_last_agni_kai, "TLA");
    registry.register("The Lion-Turtle", the_lion_turtle, "TLA");
    registry.register("The Unagi of Kyoshi Island", the_unagi_of_kyoshi_island, "TLA");
    registry.register("The Walls of Ba Sing Se", the_walls_of_ba_sing_se, "TLA");
    registry.register("Tiger-Seal", tiger_seal, "TLA");
    registry.register("Toph, Hardheaded Teacher", toph_hardheaded_teacher, "TLA");
    registry.register("Toph, the Blind Bandit", toph_the_blind_bandit, "TLA");
    registry.register("Toph, the First Metalbender", toph_the_first_metalbender, "TLA");
    registry.register("Trusty Boomerang", trusty_boomerang, "TLA");
    registry.register("Tundra Tank", tundra_tank, "TLA");
    registry.register("Twin Blades", twin_blades, "TLA");
    registry.register("Ty Lee, Chi Blocker", ty_lee_chi_blocker, "TLA");
    registry.register("Unlucky Cabbage Merchant", unlucky_cabbage_merchant, "TLA");
    registry.register("Vindictive Warden", vindictive_warden, "TLA");
    registry.register("Walltop Sentries", walltop_sentries, "TLA");
    registry.register("Wan Shi Tong, Librarian", wan_shi_tong_librarian, "TLA");
    registry.register("Wandering Musicians", wandering_musicians, "TLA");
    registry.register("War Balloon", war_balloon, "TLA");
    registry.register("Water Tribe Captain", water_tribe_captain, "TLA");
    registry.register("Watery Grasp", watery_grasp, "TLA");
    registry.register("White Lotus Hideout", white_lotus_hideout, "TLA");
    registry.register("White Lotus Reinforcements", white_lotus_reinforcements, "TLA");
    registry.register("Yue, the Moon Spirit", yue_the_moon_spirit, "TLA");
    registry.register("Zhao, Ruthless Admiral", zhao_ruthless_admiral, "TLA");
    registry.register("Zhao, the Moon Slayer", zhao_the_moon_slayer, "TLA");
    registry.register("Zuko, Conflicted", zuko_conflicted, "TLA");
}

fn aangs_journey(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Aang's Journey".into(),
        mana_cost: ManaCost::parse("{2}"),
        card_types: vec![CardType::Sorcery],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn abandon_attachments(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Abandon Attachments".into(),
        mana_cost: ManaCost::parse("{1}{U/R}"),
        card_types: vec![CardType::Instant],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn accumulate_wisdom(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Accumulate Wisdom".into(),
        mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Instant],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn airbending_lesson(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Airbending Lesson".into(),
        mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Instant],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn allies_at_last(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Allies at Last".into(),
        mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                    "Affinity for allies",
                    vec![StaticEffect::Custom("Affinity for allies.".into())]),
        ],
        ..Default::default() }
}

fn avatar_enthusiasts(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Avatar Enthusiasts".into(),
        mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Peasant, SubType::Ally],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Common,
        ..Default::default() }
}

fn azula_on_the_hunt(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Azula, On the Hunt".into(),
        mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Noble],
        supertypes: vec![SuperType::Legendary],
        power: Some(4), toughness: Some(3),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever this creature attacks, trigger effect.",
                    vec![EventType::AttackerDeclared],
                    vec![Effect::create_token("token", 1)],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn badgermole(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Badgermole".into(),
        mana_cost: ManaCost::parse("{4}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Custom("Badger".into()), SubType::Custom("Mole".into())],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::TRAMPLE,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Badgermole enters, trigger effect.",
                    vec![Effect::Custom("Earthbend 2.".into())],
                    TargetSpec::None),
            Ability::static_ability(id,
                    "Static ability.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn badgermole_cub(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Badgermole Cub".into(),
        mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Custom("Badger".into()), SubType::Custom("Mole".into())],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Mythic,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Badgermole Cub enters, trigger effect.",
                    vec![Effect::Custom("Earthbend 1.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn barrels_of_blasting_jelly(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Barrels of Blasting Jelly".into(),
        mana_cost: ManaCost::parse("{1}"),
        card_types: vec![CardType::Artifact],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                    "Deal 1 damage to any target.",
                    vec![Cost::pay_mana("{1}")],
                    vec![Effect::deal_damage(1)],
                    TargetSpec::CreatureOrPlayer),
        ],
        ..Default::default() }
}

fn beetle_headed_merchants(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Beetle-Headed Merchants".into(),
        mana_cost: ManaCost::parse("{4}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Custom("Citizen".into())],
        power: Some(5), toughness: Some(4),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever this creature attacks, trigger effect.",
                    vec![EventType::AttackerDeclared],
                    vec![Effect::add_p1p1_counters(1)],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn benders_waterskin(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Bender's Waterskin".into(),
        mana_cost: ManaCost::parse("{3}"),
        card_types: vec![CardType::Artifact],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                    "Static ability.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn bitter_work(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Bitter Work".into(),
        mana_cost: ManaCost::parse("{1}{R}{G}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn boar_q_pine(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Boar-q-pine".into(),
        mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Custom("Boar".into()), SubType::Custom("Porcupine".into())],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell_cast_triggered(id,
                    "Whenever you cast a noncreature spell, put a +1/+1 counter on this creature.",
                    vec![Effect::add_p1p1_counters(1)],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn boomerang_basics(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Boomerang Basics".into(),
        mana_cost: ManaCost::parse("{U}"),
        card_types: vec![CardType::Sorcery],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn callous_inspector(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Callous Inspector".into(),
        mana_cost: ManaCost::parse("{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Soldier],
        power: Some(1), toughness: Some(1),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::dies_triggered(id,
                    "When Callous Inspector dies, trigger effect.",
                    vec![Effect::create_token("Clue Artifact", 1)],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn cat_gator(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Cat-Gator".into(),
        mana_cost: ManaCost::parse("{6}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Fish, SubType::Crocodile],
        power: Some(3), toughness: Some(2),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::LIFELINK,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Cat-Gator enters, trigger effect.",
                    vec![Effect::deal_damage(1)],
                    TargetSpec::CreatureOrPlayer),
        ],
        ..Default::default() }
}

fn cat_owl(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Cat-Owl".into(),
        mana_cost: ManaCost::parse("{3}{W/U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Cat, SubType::Bird],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever this creature attacks, trigger effect.",
                    vec![EventType::AttackerDeclared],
                    vec![Effect::Custom("Attack trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn combustion_technique(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Combustion Technique".into(),
        mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Instant],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn compassionate_healer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Compassionate Healer".into(),
        mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Cleric, SubType::Ally],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever this creature becomes tapped, you gain 1 life and scry 1.",
                    vec![EventType::Tapped],
                    vec![Effect::gain_life(1), Effect::scry(1)],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn corrupt_court_official(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Corrupt Court Official".into(),
        mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Advisor],
        power: Some(1), toughness: Some(1),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Corrupt Court Official enters, trigger effect.",
                    vec![Effect::discard_cards(1)],
                    TargetSpec::Player),
        ],
        ..Default::default() }
}

fn crescent_island_temple(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Crescent Island Temple".into(),
        mana_cost: ManaCost::parse("{3}{R}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Shrine],
        supertypes: vec![SuperType::Legendary],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Crescent Island Temple enters, trigger effect.",
                    vec![Effect::create_token("Monk Red", 1)],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn cruel_administrator(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Cruel Administrator".into(),
        mana_cost: ManaCost::parse("{3}{B}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Soldier],
        power: Some(5), toughness: Some(4),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever this creature attacks, trigger effect.",
                    vec![EventType::AttackerDeclared],
                    vec![Effect::add_p1p1_counters(1)],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn cunning_maneuver(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Cunning Maneuver".into(),
        mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn curious_farm_animals(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Curious Farm Animals".into(),
        mana_cost: ManaCost::parse("{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Custom("Boar".into()), SubType::Elk, SubType::Bird, SubType::Ox],
        power: Some(1), toughness: Some(1),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::dies_triggered(id,
                    "When Curious Farm Animals dies, trigger effect.",
                    vec![Effect::gain_life(3)],
                    TargetSpec::None),
            Ability::activated(id,
                    "Destroy target permanent.",
                    vec![Cost::pay_mana("{2}")],
                    vec![Effect::destroy()],
                    TargetSpec::Permanent),
        ],
        ..Default::default() }
}

fn cycle_of_renewal(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Cycle of Renewal".into(),
        mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Instant],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn dai_li_agents(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Dai Li Agents".into(),
        mana_cost: ManaCost::parse("{3}{B}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Soldier],
        power: Some(3), toughness: Some(4),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Dai Li Agents enters, trigger effect.",
                    vec![Effect::gain_life(1), Effect::Custom("Earthbend 1.".into())],
                    TargetSpec::None),
            Ability::triggered(id,
                    "Whenever this creature attacks, trigger effect.",
                    vec![EventType::AttackerDeclared],
                    vec![Effect::Custom("Attack trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn day_of_black_sun(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Day of Black Sun".into(),
        mana_cost: ManaCost::parse("{X}{B}{B}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Rare,
        ..Default::default() }
}

fn deadly_precision(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Deadly Precision".into(),
        mana_cost: ManaCost::parse("{B}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn deserters_disciple(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Deserter's Disciple".into(),
        mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Rebel, SubType::Ally],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::tap_self()],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn earth_kingdom_general(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Earth Kingdom General".into(),
        mana_cost: ManaCost::parse("{3}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Soldier, SubType::Ally],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Earth Kingdom General enters, trigger effect.",
                    vec![Effect::gain_life(1), Effect::Custom("Earthbend 2.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn earth_kingdom_jailer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Earth Kingdom Jailer".into(),
        mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Soldier, SubType::Ally],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Earth Kingdom Jailer enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn earth_kingdom_protectors(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Earth Kingdom Protectors".into(),
        mana_cost: ManaCost::parse("{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Soldier, SubType::Ally],
        power: Some(1), toughness: Some(1),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::VIGILANCE | KeywordAbilities::INDESTRUCTIBLE,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::tap_self()],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn earth_kingdom_soldier(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Earth Kingdom Soldier".into(),
        mana_cost: ManaCost::parse("{4}{G/W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Soldier],
        power: Some(3), toughness: Some(4),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::VIGILANCE,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Earth Kingdom Soldier enters, trigger effect.",
                    vec![Effect::add_p1p1_counters(1)],
                    TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn earth_rumble(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Earth Rumble".into(),
        mana_cost: ManaCost::parse("{3}{G}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn earth_rumble_wrestlers(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Earth Rumble Wrestlers".into(),
        mana_cost: ManaCost::parse("{3}{R/G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior, SubType::Performer],
        power: Some(3), toughness: Some(4),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::REACH | KeywordAbilities::TRAMPLE,
        abilities: vec![
            Ability::static_ability(id,
                    "Conditional static effect.",
                    vec![StaticEffect::Custom("Conditional continuous effect.".into())]),
        ],
        ..Default::default() }
}

fn earth_village_ruffians(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Earth Village Ruffians".into(),
        mana_cost: ManaCost::parse("{2}{B/G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Soldier, SubType::Rogue],
        power: Some(3), toughness: Some(1),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::dies_triggered(id,
                    "When Earth Village Ruffians dies, trigger effect.",
                    vec![Effect::Custom("Earthbend 2.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn earthbending_lesson(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Earthbending Lesson".into(),
        mana_cost: ManaCost::parse("{3}{G}"),
        card_types: vec![CardType::Sorcery],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn earthen_ally(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Earthen Ally".into(),
        mana_cost: ManaCost::parse("{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Soldier, SubType::Ally],
        power: Some(0), toughness: Some(2),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                    "Static ability.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{2}{W}{U}{B}{R}{G}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn elemental_teachings(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Elemental Teachings".into(),
        mana_cost: ManaCost::parse("{4}{G}"),
        card_types: vec![CardType::Instant],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Rare,
        ..Default::default() }
}

fn energybending(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Energybending".into(),
        mana_cost: ManaCost::parse("{2}"),
        card_types: vec![CardType::Instant],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn enter_the_avatar_state(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Enter the Avatar State".into(),
        mana_cost: ManaCost::parse("{W}"),
        card_types: vec![CardType::Instant],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::FIRST_STRIKE | KeywordAbilities::LIFELINK | KeywordAbilities::HEXPROOF,
        ..Default::default() }
}

fn epic_downfall(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Epic Downfall".into(),
        mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn fancy_footwork(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Fancy Footwork".into(),
        mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Instant],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn fatal_fissure(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Fatal Fissure".into(),
        mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn fire_lord_azula(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Fire Lord Azula".into(),
        mana_cost: ManaCost::parse("{1}{U}{B}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Noble],
        supertypes: vec![SuperType::Legendary],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::spell_cast_triggered(id,
                    "Whenever you cast a spell, trigger effect.",
                    vec![Effect::Custom("Spell cast trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn fire_nation_attacks(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Fire Nation Attacks".into(),
        mana_cost: ManaCost::parse("{4}{R}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                    "Flashback {8}{R}",
                    vec![Cost::pay_mana("{8}{R}")],
                    vec![Effect::Custom("Cast from graveyard.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn fire_nation_cadets(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Fire Nation Cadets".into(),
        mana_cost: ManaCost::parse("{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Soldier],
        power: Some(1), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                    "Conditional static effect.",
                    vec![StaticEffect::Custom("Conditional continuous effect.".into())]),
            Ability::activated(id,
                    "Gain ability until end of turn.",
                    vec![Cost::pay_mana("{2}")],
                    vec![Effect::Custom("Gain ability until end of turn.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn fire_nation_engineer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Fire Nation Engineer".into(),
        mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Custom("Artificer".into())],
        power: Some(2), toughness: Some(3),
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn fire_nation_raider(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Fire Nation Raider".into(),
        mana_cost: ManaCost::parse("{3}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Soldier],
        power: Some(4), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Fire Nation Raider enters, trigger effect.",
                    vec![Effect::create_token("Clue Artifact", 1)],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn fire_sages(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Fire Sages".into(),
        mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Cleric],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{1}{R}{R}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn firebending_lesson(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Firebending Lesson".into(),
        mana_cost: ManaCost::parse("{R}"),
        card_types: vec![CardType::Instant],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn firebending_student(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Firebending Student".into(),
        mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Monk],
        power: Some(1), toughness: Some(2),
        rarity: Rarity::Rare,
        ..Default::default() }
}

fn first_time_flyer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "First-Time Flyer".into(),
        mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Pilot, SubType::Ally],
        power: Some(1), toughness: Some(2),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::static_ability(id,
                    "Conditional static effect.",
                    vec![StaticEffect::Custom("Conditional continuous effect.".into())]),
        ],
        ..Default::default() }
}

fn flexible_waterbender(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Flexible Waterbender".into(),
        mana_cost: ManaCost::parse("{3}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior, SubType::Ally],
        power: Some(2), toughness: Some(5),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::VIGILANCE,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::tap_self()],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn flopsie_bumis_buddy(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Flopsie, Bumi's Buddy".into(),
        mana_cost: ManaCost::parse("{4}{G}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Ape, SubType::Goat],
        supertypes: vec![SuperType::Legendary],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Flopsie, Bumi's Buddy enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
            Ability::static_ability(id,
                    "Static ability.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn foggy_swamp_hunters(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Foggy Swamp Hunters".into(),
        mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Ranger, SubType::Ally],
        power: Some(3), toughness: Some(4),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::LIFELINK,
        abilities: vec![
            Ability::static_ability(id,
                    "Conditional static effect.",
                    vec![StaticEffect::Custom("Conditional continuous effect.".into())]),
        ],
        ..Default::default() }
}

fn foggy_swamp_spirit_keeper(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Foggy Swamp Spirit Keeper".into(),
        mana_cost: ManaCost::parse("{1}{U}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Druid, SubType::Ally],
        power: Some(2), toughness: Some(4),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::LIFELINK,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever you draw your second card each turn, this creature gets +1/+2 until end of turn and can't be blocked this turn.",
                    vec![EventType::DrewCard],
                    vec![Effect::Custom("Draw second card trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn foggy_swamp_vinebender(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Foggy Swamp Vinebender".into(),
        mana_cost: ManaCost::parse("{3}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Plant, SubType::Ally],
        power: Some(4), toughness: Some(3),
        rarity: Rarity::Common,
        ..Default::default() }
}

fn forecasting_fortune_teller(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Forecasting Fortune Teller".into(),
        mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Advisor, SubType::Ally],
        power: Some(1), toughness: Some(3),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Forecasting Fortune Teller enters, trigger effect.",
                    vec![Effect::create_token("Clue Artifact", 1)],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn gather_the_white_lotus(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Gather the White Lotus".into(),
        mana_cost: ManaCost::parse("{4}{W}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn geyser_leaper(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Geyser Leaper".into(),
        mana_cost: ManaCost::parse("{4}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior, SubType::Ally],
        power: Some(4), toughness: Some(3),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::tap_self()],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn giant_koi(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Giant Koi".into(),
        mana_cost: ManaCost::parse("{4}{U}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Fish],
        power: Some(5), toughness: Some(7),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{2}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn glider_kids(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Glider Kids".into(),
        mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Pilot, SubType::Ally],
        power: Some(2), toughness: Some(3),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Glider Kids enters, trigger effect.",
                    vec![Effect::scry(1)],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn gran_gran(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Gran-Gran".into(),
        mana_cost: ManaCost::parse("{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Peasant, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(1), toughness: Some(2),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever this creature becomes tapped, trigger effect.",
                    vec![EventType::Tapped],
                    vec![Effect::Custom("Tapped trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn guru_pathik(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Guru Pathik".into(),
        mana_cost: ManaCost::parse("{2}{G/U}{G/U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Monk, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(4),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Guru Pathik enters, trigger effect.",
                    vec![Effect::add_p1p1_counters(1)],
                    TargetSpec::Creature),
            Ability::spell_cast_triggered(id,
                    "Whenever you cast a spell, trigger effect.",
                    vec![Effect::Custom("Spell cast trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn haru_hidden_talent(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Haru, Hidden Talent".into(),
        mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Peasant, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(1), toughness: Some(1),
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn hei_bai_spirit_of_balance(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Hei Bai, Spirit of Balance".into(),
        mana_cost: ManaCost::parse("{2}{W/B}{W/B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Bear, SubType::Spirit],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn hermitic_herbalist(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Hermitic Herbalist".into(),
        mana_cost: ManaCost::parse("{G}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Druid, SubType::Ally],
        power: Some(2), toughness: Some(3),
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn hog_monkey(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Hog-Monkey".into(),
        mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Custom("Boar".into()), SubType::Custom("Monkey".into())],
        power: Some(3), toughness: Some(2),
        rarity: Rarity::Common,
        ..Default::default() }
}

fn invasion_reinforcements(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Invasion Reinforcements".into(),
        mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior, SubType::Ally],
        power: Some(1), toughness: Some(1),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLASH,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Invasion Reinforcements enters, trigger effect.",
                    vec![Effect::create_token("Ally", 1)],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn invasion_submersible(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Invasion Submersible".into(),
        mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Artifact],
        subtypes: vec![SubType::Vehicle],
        power: Some(0), toughness: Some(0),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Invasion Submersible enters, trigger effect.",
                    vec![Effect::add_p1p1_counters(1), Effect::bounce()],
                    TargetSpec::Permanent),
        ],
        ..Default::default() }
}

fn itll_quench_ya(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "It'll Quench Ya!".into(),
        mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Instant],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn jeong_jeong_the_deserter(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Jeong Jeong, the Deserter".into(),
        mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Rebel, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(3),
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn jeong_jeongs_deserters(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Jeong Jeong's Deserters".into(),
        mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Rebel, SubType::Ally],
        power: Some(1), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Jeong Jeong's Deserters enters, trigger effect.",
                    vec![Effect::add_p1p1_counters(1)],
                    TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn jet_freedom_fighter(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Jet, Freedom Fighter".into(),
        mana_cost: ManaCost::parse("{2}{R/W}{R/W}{R/W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Rebel, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(1),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Jet, Freedom Fighter enters, trigger effect.",
                    vec![Effect::add_p1p1_counters(1), Effect::deal_damage(1)],
                    TargetSpec::CreatureOrPlayer),
            Ability::dies_triggered(id,
                    "When Jet, Freedom Fighter dies, trigger effect.",
                    vec![Effect::Custom("Dies effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn jets_brainwashing(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Jet's Brainwashing".into(),
        mana_cost: ManaCost::parse("{R}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::HASTE,
        ..Default::default() }
}

fn joo_dee_one_of_many(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Joo Dee, One of Many".into(),
        mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Advisor],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn june_bounty_hunter(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "June, Bounty Hunter".into(),
        mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Mercenary],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                    "Static ability.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn katara_bending_prodigy(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Katara, Bending Prodigy".into(),
        mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(3),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                    "Draw a card.",
                    vec![Cost::tap_self()],
                    vec![Effect::draw_cards(1)],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn katara_the_fearless(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Katara, the Fearless".into(),
        mana_cost: ManaCost::parse("{G}{W}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                    "Static ability.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn kyoshi_island_plaza(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Kyoshi Island Plaza".into(),
        mana_cost: ManaCost::parse("{3}{G}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Shrine],
        supertypes: vec![SuperType::Legendary],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Kyoshi Island Plaza enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn kyoshi_warriors(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Kyoshi Warriors".into(),
        mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior, SubType::Ally],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Kyoshi Warriors enters, trigger effect.",
                    vec![Effect::create_token("Ally", 1)],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn lightning_strike(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Lightning Strike".into(),
        mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn long_feng_grand_secretariat(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Long Feng, Grand Secretariat".into(),
        mana_cost: ManaCost::parse("{1}{B/G}{B/G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Advisor],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(3),
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn lost_days(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Lost Days".into(),
        mana_cost: ManaCost::parse("{4}{U}"),
        card_types: vec![CardType::Instant],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn mai_jaded_edge(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Mai, Jaded Edge".into(),
        mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Noble],
        supertypes: vec![SuperType::Legendary],
        power: Some(1), toughness: Some(3),
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn mai_scornful_striker(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Mai, Scornful Striker".into(),
        mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Noble, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FIRST_STRIKE,
        ..Default::default() }
}

fn master_pakku(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Master Pakku".into(),
        mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Advisor, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(1), toughness: Some(3),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever this creature becomes tapped, trigger effect.",
                    vec![EventType::Tapped],
                    vec![Effect::Custom("Tapped trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn master_piandao(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Master Piandao".into(),
        mana_cost: ManaCost::parse("{4}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FIRST_STRIKE,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever this creature attacks, trigger effect.",
                    vec![EventType::AttackerDeclared],
                    vec![Effect::Custom("Attack trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn merchant_of_many_hats(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Merchant of Many Hats".into(),
        mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Peasant, SubType::Ally],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                    "Return this card from your graveyard to your hand.",
                    vec![Cost::pay_mana("{2}{B}")],
                    vec![Effect::return_from_graveyard()],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn north_pole_patrol(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "North Pole Patrol".into(),
        mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Soldier, SubType::Ally],
        power: Some(2), toughness: Some(3),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::tap_self()],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn northern_air_temple(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Northern Air Temple".into(),
        mana_cost: ManaCost::parse("{B}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Shrine],
        supertypes: vec![SuperType::Legendary],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Northern Air Temple enters, trigger effect.",
                    vec![Effect::gain_life(1)],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn obsessive_pursuit(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Obsessive Pursuit".into(),
        mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::LIFELINK,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Obsessive Pursuit enters, trigger effect.",
                    vec![Effect::create_token("Clue Artifact", 1), Effect::add_p1p1_counters(1)],
                    TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn octopus_form(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Octopus Form".into(),
        mana_cost: ManaCost::parse("{U}"),
        card_types: vec![CardType::Instant],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Common,
        keywords: KeywordAbilities::HEXPROOF,
        ..Default::default() }
}

fn ostrich_horse(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ostrich-Horse".into(),
        mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Bird, SubType::Horse],
        power: Some(3), toughness: Some(1),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Ostrich-Horse enters, trigger effect.",
                    vec![Effect::add_p1p1_counters(1), Effect::mill(3), Effect::Custom("Put a land card from among them into your hand or +1/+1 counter.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn otter_penguin(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Otter-Penguin".into(),
        mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Otter, SubType::Bird],
        power: Some(2), toughness: Some(1),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever you draw your second card each turn, this creature gets +1/+2 until end of turn and can't be blocked this turn.",
                    vec![EventType::DrewCard],
                    vec![Effect::boost_until_eot(1, 2), Effect::Custom("Can't be blocked this turn.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn ozais_cruelty(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ozai's Cruelty".into(),
        mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Sorcery],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn pillar_launch(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Pillar Launch".into(),
        mana_cost: ManaCost::parse("{G}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        keywords: KeywordAbilities::REACH,
        ..Default::default() }
}

fn pirate_peddlers(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Pirate Peddlers".into(),
        mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Pirate],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::DEATHTOUCH,
        ..Default::default() }
}

fn pretending_poxbearers(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Pretending Poxbearers".into(),
        mana_cost: ManaCost::parse("{1}{W/B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Custom("Citizen".into()), SubType::Ally],
        power: Some(2), toughness: Some(1),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::dies_triggered(id,
                    "When Pretending Poxbearers dies, trigger effect.",
                    vec![Effect::create_token("Ally", 1)],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn price_of_freedom(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Price of Freedom".into(),
        mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Sorcery],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn professor_zei_anthropologist(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Professor Zei, Anthropologist".into(),
        mana_cost: ManaCost::parse("{U/R}{U/R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Advisor, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(0), toughness: Some(3),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                    "Draw a card.",
                    vec![Cost::pay_mana("{1}")],
                    vec![Effect::draw_cards(1)],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn rabaroo_troop(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Rabaroo Troop".into(),
        mana_cost: ManaCost::parse("{3}{W}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Rabbit, SubType::Custom("Kangaroo".into())],
        power: Some(3), toughness: Some(5),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::FLYING,
        ..Default::default() }
}

fn raucous_audience(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Raucous Audience".into(),
        mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Custom("Citizen".into())],
        power: Some(2), toughness: Some(1),
        rarity: Rarity::Common,
        ..Default::default() }
}

fn razor_rings(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Razor Rings".into(),
        mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn rebellious_captives(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Rebellious Captives".into(),
        mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Peasant, SubType::Ally],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Common,
        ..Default::default() }
}

fn redirect_lightning(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Redirect Lightning".into(),
        mana_cost: ManaCost::parse("{R}"),
        card_types: vec![CardType::Instant],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Rare,
        ..Default::default() }
}

fn rockalanche(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Rockalanche".into(),
        mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Sorcery],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                    "Flashback {5}{G}",
                    vec![Cost::pay_mana("{5}{G}")],
                    vec![Effect::Custom("Cast from graveyard.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn rocky_rebuke(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Rocky Rebuke".into(),
        mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn rough_rhino_cavalry(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Rough Rhino Cavalry".into(),
        mana_cost: ManaCost::parse("{4}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Mercenary],
        power: Some(5), toughness: Some(5),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::TRAMPLE,
        ..Default::default() }
}

fn rowdy_snowballers(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Rowdy Snowballers".into(),
        mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Peasant, SubType::Ally],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Rowdy Snowballers enters, trigger effect.",
                    vec![Effect::add_p1p1_counters(1)],
                    TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn saber_tooth_moose_lion(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Saber-Tooth Moose-Lion".into(),
        mana_cost: ManaCost::parse("{4}{G}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elk, SubType::Cat],
        power: Some(7), toughness: Some(7),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::REACH,
        ..Default::default() }
}

fn seismic_sense(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Seismic Sense".into(),
        mana_cost: ManaCost::parse("{G}"),
        card_types: vec![CardType::Sorcery],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn serpent_of_the_pass(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Serpent of the Pass".into(),
        mana_cost: ManaCost::parse("{5}{U}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Serpent],
        power: Some(6), toughness: Some(5),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                    "Static ability.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn shared_roots(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Shared Roots".into(),
        mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Sorcery],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn sokka_bold_boomeranger(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sokka, Bold Boomeranger".into(),
        mana_cost: ManaCost::parse("{U}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(1), toughness: Some(1),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Sokka, Bold Boomeranger enters, trigger effect.",
                    vec![Effect::add_p1p1_counters(1)],
                    TargetSpec::None),
            Ability::spell_cast_triggered(id,
                    "Whenever you cast a noncreature spell, put a +1/+1 counter on this creature.",
                    vec![Effect::add_p1p1_counters(1)],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn sokka_lateral_strategist(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sokka, Lateral Strategist".into(),
        mana_cost: ManaCost::parse("{1}{W/U}{W/U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(4),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::VIGILANCE,
        ..Default::default() }
}

fn sokkas_haiku(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sokka's Haiku".into(),
        mana_cost: ManaCost::parse("{3}{U}{U}"),
        card_types: vec![CardType::Instant],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn sold_out(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sold Out".into(),
        mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn south_pole_voyager(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "South Pole Voyager".into(),
        mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Scout, SubType::Ally],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Rare,
        ..Default::default() }
}

fn southern_air_temple(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Southern Air Temple".into(),
        mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Shrine],
        supertypes: vec![SuperType::Legendary],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Southern Air Temple enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn spirit_water_revival(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Spirit Water Revival".into(),
        mana_cost: ManaCost::parse("{1}{U}{U}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Rare,
        ..Default::default() }
}

fn suki_kyoshi_warrior(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Suki, Kyoshi Warrior".into(),
        mana_cost: ManaCost::parse("{2}{G/W}{G/W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(0), toughness: Some(4),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever this creature attacks, trigger effect.",
                    vec![EventType::AttackerDeclared],
                    vec![Effect::create_token("token", 1)],
                    TargetSpec::None),
            Ability::static_ability(id,
                    "Static ability.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn sun_warriors(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sun Warriors".into(),
        mana_cost: ManaCost::parse("{2}{R}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior, SubType::Ally],
        power: Some(3), toughness: Some(5),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{5}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn team_avatar(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Team Avatar".into(),
        mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                    "Deal 1 damage to any target.",
                    vec![Cost::pay_mana("{2}{W}")],
                    vec![Effect::deal_damage(1)],
                    TargetSpec::CreatureOrPlayer),
        ],
        ..Default::default() }
}

fn the_boulder_ready_to_rumble(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "The Boulder, Ready to Rumble".into(),
        mana_cost: ManaCost::parse("{3}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior, SubType::Performer],
        supertypes: vec![SuperType::Legendary],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever this creature attacks, trigger effect.",
                    vec![EventType::AttackerDeclared],
                    vec![Effect::Custom("Attack trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn the_mechanist_aerial_artisan(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "The Mechanist, Aerial Artisan".into(),
        mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Custom("Artificer".into()), SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(1), toughness: Some(3),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::spell_cast_triggered(id,
                    "Whenever you cast a spell, trigger effect.",
                    vec![Effect::Custom("Spell cast trigger.".into())],
                    TargetSpec::None),
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::tap_self()],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn the_spirit_oasis(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "The Spirit Oasis".into(),
        mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Shrine],
        supertypes: vec![SuperType::Legendary],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When The Spirit Oasis enters, trigger effect.",
                    vec![Effect::draw_cards(1)],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn tiger_dillo(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Tiger-Dillo".into(),
        mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Cat, SubType::Custom("Armadillo".into())],
        power: Some(4), toughness: Some(3),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                    "Static ability.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn tolls_of_war(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Tolls of War".into(),
        mana_cost: ManaCost::parse("{W}{B}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Tolls of War enters, trigger effect.",
                    vec![Effect::create_token("Clue Artifact", 1)],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn treetop_freedom_fighters(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Treetop Freedom Fighters".into(),
        mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Rebel, SubType::Ally],
        power: Some(2), toughness: Some(1),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::HASTE,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Treetop Freedom Fighters enters, trigger effect.",
                    vec![Effect::create_token("Ally", 1)],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn true_ancestry(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "True Ancestry".into(),
        mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Sorcery],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn turtle_duck(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Turtle-Duck".into(),
        mana_cost: ManaCost::parse("{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Turtle, SubType::Bird],
        power: Some(0), toughness: Some(4),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::TRAMPLE,
        abilities: vec![
            Ability::activated(id,
                    "Until end of turn, this creature has base power 4.",
                    vec![Cost::pay_mana("{3}")],
                    vec![Effect::Custom("Set base power to 4 until end of turn.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn ty_lee_artful_acrobat(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ty Lee, Artful Acrobat".into(),
        mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Performer],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(2),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever this creature attacks, trigger effect.",
                    vec![EventType::AttackerDeclared],
                    vec![Effect::Custom("Attack trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn uncle_iroh(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Uncle Iroh".into(),
        mana_cost: ManaCost::parse("{1}{R/G}{R/G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Noble, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(4), toughness: Some(2),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                    "Static ability.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn united_front(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "United Front".into(),
        mana_cost: ManaCost::parse("{X}{W}{W}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Mythic,
        ..Default::default() }
}

fn vengeful_villagers(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Vengeful Villagers".into(),
        mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Custom("Citizen".into())],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever this creature attacks, trigger effect.",
                    vec![EventType::AttackerDeclared],
                    vec![Effect::Custom("Attack trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn wartime_protestors(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Wartime Protestors".into(),
        mana_cost: ManaCost::parse("{3}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Rebel, SubType::Ally],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::HASTE | KeywordAbilities::HASTE,
        ..Default::default() }
}

fn water_tribe_rallier(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Water Tribe Rallier".into(),
        mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Soldier, SubType::Ally],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::tap_self()],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn waterbender_ascension(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Waterbender Ascension".into(),
        mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::activated(id,
                    "Draw a card.",
                    vec![Cost::tap_self()],
                    vec![Effect::draw_cards(1)],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn waterbending_lesson(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Waterbending Lesson".into(),
        mana_cost: ManaCost::parse("{3}{U}"),
        card_types: vec![CardType::Sorcery],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn waterbending_scroll(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Waterbending Scroll".into(),
        mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Artifact],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                    "Draw a card.",
                    vec![Cost::pay_mana("{6}")],
                    vec![Effect::draw_cards(1)],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn white_lotus_tile(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "White Lotus Tile".into(),
        mana_cost: ManaCost::parse("{4}"),
        card_types: vec![CardType::Artifact],
        rarity: Rarity::Mythic,
        ..Default::default() }
}

fn wolfbat(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Wolfbat".into(),
        mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Wolf, SubType::Custom("Bat".into())],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever you draw your second card each turn, this creature gets +1/+2 until end of turn and can't be blocked this turn.",
                    vec![EventType::DrewCard],
                    vec![Effect::Custom("Draw second card trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn yip_yip(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Yip Yip!".into(),
        mana_cost: ManaCost::parse("{W}"),
        card_types: vec![CardType::Instant],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Common,
        keywords: KeywordAbilities::FLYING,
        ..Default::default() }
}

fn yuyan_archers(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Yuyan Archers".into(),
        mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Archer],
        power: Some(3), toughness: Some(1),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::REACH,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When Yuyan Archers enters, trigger effect.",
                    vec![Effect::draw_cards(1)],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn zuko_exiled_prince(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Zuko, Exiled Prince".into(),
        mana_cost: ManaCost::parse("{3}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Noble],
        supertypes: vec![SuperType::Legendary],
        power: Some(4), toughness: Some(3),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{3}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn zukos_conviction(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Zuko's Conviction".into(),
        mana_cost: ManaCost::parse("{B}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn zukos_exile(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Zuko's Exile".into(),
        mana_cost: ManaCost::parse("{5}"),
        card_types: vec![CardType::Instant],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn aang_at_the_crossroads(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Aang, at the Crossroads".into(),
        card_types: vec![CardType::Creature],
        supertypes: vec![SuperType::Legendary],
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::FLYING | KeywordAbilities::VIGILANCE,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn aang_swift_savior(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Aang, Swift Savior".into(),
        card_types: vec![CardType::Creature],
        supertypes: vec![SuperType::Legendary],
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLASH | KeywordAbilities::FLYING | KeywordAbilities::REACH | KeywordAbilities::TRAMPLE,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
            Ability::triggered(id,
                    "Whenever this attacks, trigger effect.",
                    vec![EventType::AttackerDeclared],
                    vec![Effect::Custom("Attack trigger.".into())],
                    TargetSpec::None),
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::tap_self()],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn aang_the_last_airbender(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Aang, the Last Airbender".into(),
        mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Avatar, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(2),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::LIFELINK,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::Creature),
            Ability::spell_cast_triggered(id,
                    "Whenever you cast a spell, trigger effect.",
                    vec![Effect::Custom("Spell cast trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn aangs_iceberg(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Aang's Iceberg".into(),
        mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLASH,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::Creature),
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::tap_self()],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn abandoned_air_temple(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Abandoned Air Temple".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{3}{W}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn agna_qela(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Agna Qel'a".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{2}{U}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn air_nomad_legacy(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Air Nomad Legacy".into(),
        mana_cost: ManaCost::parse("{W}{U}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn airbender_ascension(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Airbender Ascension".into(),
        mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn airbenders_reversal(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Airbender's Reversal".into(),
        mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Instant],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::destroy()],
                    TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn airship_engine_room(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Airship Engine Room".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{4}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn appa_loyal_sky_bison(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Appa, Loyal Sky Bison".into(),
        mana_cost: ManaCost::parse("{4}{W}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Custom("Bison".into()), SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::FLYING,
        ..Default::default() }
}

fn appa_steadfast_guardian(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Appa, Steadfast Guardian".into(),
        mana_cost: ManaCost::parse("{2}{W}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Custom("Bison".into()), SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(4),
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::FLASH | KeywordAbilities::FLYING,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::Creature),
            Ability::spell_cast_triggered(id,
                    "Whenever you cast a spell, trigger effect.",
                    vec![Effect::Custom("Spell cast trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn avatar_aang(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Avatar Aang".into(),
        card_types: vec![CardType::Creature],
        supertypes: vec![SuperType::Legendary],
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::FLYING,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn avatar_destiny(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Avatar Destiny".into(),
        mana_cost: ManaCost::parse("{2}{G}{G}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Aura],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn avatars_wrath(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Avatar's Wrath".into(),
        mana_cost: ManaCost::parse("{2}{W}{W}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::Custom("Spell effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn azula_always_lies(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Azula Always Lies".into(),
        mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Instant],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::Custom("Spell effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn azula_cunning_usurper(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Azula, Cunning Usurper".into(),
        mana_cost: ManaCost::parse("{2}{U}{B}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Noble, SubType::Rogue],
        supertypes: vec![SuperType::Legendary],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::Creature),
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn ba_sing_se(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ba Sing Se".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Rare,
        ..Default::default() }
}

fn beifongs_bounty_hunters(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Beifong's Bounty Hunters".into(),
        mana_cost: ManaCost::parse("{2}{B}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Mercenary],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Rare,
        ..Default::default() }
}

fn benevolent_river_spirit(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Benevolent River Spirit".into(),
        mana_cost: ManaCost::parse("{U}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Spirit],
        power: Some(4), toughness: Some(5),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn boiling_rock_prison(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Boiling Rock Prison".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{4}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn boiling_rock_rioter(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Boiling Rock Rioter".into(),
        mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Rogue, SubType::Ally],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever this attacks, trigger effect.",
                    vec![EventType::AttackerDeclared],
                    vec![Effect::Custom("Attack trigger.".into())],
                    TargetSpec::None),
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::tap_self()],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn bumi_bash(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Bumi Bash".into(),
        mana_cost: ManaCost::parse("{3}{R}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::destroy()],
                    TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn bumi_king_of_three_trials(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Bumi, King of Three Trials".into(),
        mana_cost: ManaCost::parse("{5}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Noble, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn bumi_unleashed(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Bumi, Unleashed".into(),
        mana_cost: ManaCost::parse("{3}{R}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Noble, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(5), toughness: Some(4),
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::TRAMPLE,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn buzzard_wasp_colony(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Buzzard-Wasp Colony".into(),
        mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Bird, SubType::Insect],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn canyon_crawler(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Canyon Crawler".into(),
        mana_cost: ManaCost::parse("{4}{B}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Spider, SubType::Beast],
        power: Some(6), toughness: Some(6),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::DEATHTOUCH,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn combustion_man(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Combustion Man".into(),
        mana_cost: ManaCost::parse("{3}{R}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Assassin],
        supertypes: vec![SuperType::Legendary],
        power: Some(4), toughness: Some(6),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever this attacks, trigger effect.",
                    vec![EventType::AttackerDeclared],
                    vec![Effect::Custom("Attack trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn crashing_wave(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Crashing Wave".into(),
        mana_cost: ManaCost::parse("{U}{U}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn dai_li_indoctrination(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Dai Li Indoctrination".into(),
        mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Sorcery],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::Custom("Spell effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn destined_confrontation(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Destined Confrontation".into(),
        mana_cost: ManaCost::parse("{2}{W}{W}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::Custom("Spell effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn diligent_zookeeper(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Diligent Zookeeper".into(),
        mana_cost: ManaCost::parse("{3}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Custom("Citizen".into()), SubType::Ally],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn dragonfly_swarm(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Dragonfly Swarm".into(),
        mana_cost: ManaCost::parse("{1}{U}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon, SubType::Insect],
        power: Some(0), toughness: Some(3),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::dies_triggered(id,
                    "When this dies, trigger effect.",
                    vec![Effect::Custom("Dies effect.".into())],
                    TargetSpec::None),
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn earth_kings_lieutenant(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Earth King's Lieutenant".into(),
        mana_cost: ManaCost::parse("{G}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Soldier, SubType::Ally],
        power: Some(1), toughness: Some(1),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::TRAMPLE,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn earthbender_ascension(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Earthbender Ascension".into(),
        mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::TRAMPLE,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn ember_island_production(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ember Island Production".into(),
        mana_cost: ManaCost::parse("{3}{U}{U}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::Custom("Spell effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn fated_firepower(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Fated Firepower".into(),
        mana_cost: ManaCost::parse("{X}{R}{R}{R}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::FLASH,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn fire_lord_zuko(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Fire Lord Zuko".into(),
        mana_cost: ManaCost::parse("{R}{W}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Noble, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(4),
        rarity: Rarity::Rare,
        ..Default::default() }
}

fn fire_nation_palace(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Fire Nation Palace".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{1}{R}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn fire_nation_warship(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Fire Nation Warship".into(),
        mana_cost: ManaCost::parse("{3}"),
        card_types: vec![CardType::Artifact],
        subtypes: vec![SubType::Vehicle],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::REACH,
        abilities: vec![
            Ability::dies_triggered(id,
                    "When this dies, trigger effect.",
                    vec![Effect::Custom("Dies effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn fire_navy_trebuchet(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Fire Navy Trebuchet".into(),
        mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Artifact, CardType::Creature],
        subtypes: vec![SubType::Wall],
        power: Some(0), toughness: Some(4),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::DEFENDER | KeywordAbilities::REACH,
        ..Default::default() }
}

fn firebender_ascension(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Firebender Ascension".into(),
        mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn foggy_bottom_swamp(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Foggy Bottom Swamp".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{4}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn foggy_swamp_visions(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Foggy Swamp Visions".into(),
        mana_cost: ManaCost::parse("{1}{B}{B}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn glider_staff(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Glider Staff".into(),
        mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Artifact],
        subtypes: vec![SubType::Equipment],
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::Creature),
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn great_divide_guide(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Great Divide Guide".into(),
        mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Scout, SubType::Ally],
        power: Some(2), toughness: Some(3),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn hakoda_selfless_commander(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Hakoda, Selfless Commander".into(),
        mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(5),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::VIGILANCE | KeywordAbilities::INDESTRUCTIBLE,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::tap_self()],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn hama_the_bloodbender(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Hama, the Bloodbender".into(),
        mana_cost: ManaCost::parse("{2}{U/B}{U/B}{U/B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Custom("Warlock".into())],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn heartless_act(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Heartless Act".into(),
        mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::destroy()],
                    TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn honest_work(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Honest Work".into(),
        mana_cost: ManaCost::parse("{U}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Aura],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::Creature),
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn how_to_start_a_riot(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "How to Start a Riot".into(),
        mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Instant],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::Custom("Spell effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn iguana_parrot(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Iguana Parrot".into(),
        mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Custom("Lizard".into()), SubType::Bird, SubType::Pirate],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::VIGILANCE,
        ..Default::default() }
}

fn invasion_tactics(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Invasion Tactics".into(),
        mana_cost: ManaCost::parse("{4}{G}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn iroh_grand_lotus(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Iroh, Grand Lotus".into(),
        mana_cost: ManaCost::parse("{3}{G}{U}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Noble, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(5), toughness: Some(5),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn iroh_tea_master(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Iroh, Tea Master".into(),
        mana_cost: ManaCost::parse("{1}{R}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Custom("Citizen".into()), SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn irohs_demonstration(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Iroh's Demonstration".into(),
        mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Sorcery],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::deal_damage(4)],
                    TargetSpec::CreatureOrPlayer),
        ],
        ..Default::default() }
}

fn jasmine_dragon_tea_shop(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Jasmine Dragon Tea Shop".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{5}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn katara_water_tribes_hope(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Katara, Water Tribe's Hope".into(),
        mana_cost: ManaCost::parse("{2}{W}{U}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::VIGILANCE,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn knowledge_seeker(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Knowledge Seeker".into(),
        mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Fox, SubType::Spirit],
        power: Some(2), toughness: Some(1),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::VIGILANCE,
        abilities: vec![
            Ability::dies_triggered(id,
                    "When this dies, trigger effect.",
                    vec![Effect::Custom("Dies effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn koh_the_face_stealer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Koh, the Face Stealer".into(),
        mana_cost: ManaCost::parse("{4}{B}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Shapeshifter, SubType::Spirit],
        supertypes: vec![SuperType::Legendary],
        power: Some(6), toughness: Some(6),
        rarity: Rarity::Mythic,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::Creature),
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::tap_self()],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn kyoshi_battle_fan(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Kyoshi Battle Fan".into(),
        mana_cost: ManaCost::parse("{2}"),
        card_types: vec![CardType::Artifact],
        subtypes: vec![SubType::Equipment],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn kyoshi_village(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Kyoshi Village".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{4}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn lo_and_li_twin_tutors(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Lo and Li, Twin Tutors".into(),
        mana_cost: ManaCost::parse("{4}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Advisor],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::LIFELINK | KeywordAbilities::LIFELINK,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn meditation_pools(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Meditation Pools".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{4}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn messenger_hawk(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Messenger Hawk".into(),
        mana_cost: ManaCost::parse("{2}{U/B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Bird, SubType::Scout],
        power: Some(1), toughness: Some(2),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn meteor_sword(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Meteor Sword".into(),
        mana_cost: ManaCost::parse("{7}"),
        card_types: vec![CardType::Artifact],
        subtypes: vec![SubType::Equipment],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::Creature),
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn misty_palms_oasis(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Misty Palms Oasis".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{4}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn momo_friendly_flier(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Momo, Friendly Flier".into(),
        mana_cost: ManaCost::parse("{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Custom("Lemur".into()), SubType::Custom("Bat".into()), SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(1), toughness: Some(1),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn momo_playful_pet(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Momo, Playful Pet".into(),
        mana_cost: ManaCost::parse("{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Custom("Lemur".into()), SubType::Custom("Bat".into()), SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(1), toughness: Some(1),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::VIGILANCE,
        ..Default::default() }
}

fn mongoose_lizard(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Mongoose Lizard".into(),
        mana_cost: ManaCost::parse("{4}{R}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Custom("Mongoose".into()), SubType::Custom("Lizard".into())],
        power: Some(5), toughness: Some(6),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn north_pole_gates(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "North Pole Gates".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{4}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn omashu_city(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Omashu City".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{4}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn origin_of_metalbending(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Origin of Metalbending".into(),
        mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Instant],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Common,
        keywords: KeywordAbilities::INDESTRUCTIBLE,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::destroy()],
                    TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn ozai_the_phoenix_king(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ozai, the Phoenix King".into(),
        mana_cost: ManaCost::parse("{2}{B}{B}{R}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Noble],
        supertypes: vec![SuperType::Legendary],
        power: Some(7), toughness: Some(7),
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::TRAMPLE | KeywordAbilities::HASTE | KeywordAbilities::FLYING | KeywordAbilities::INDESTRUCTIBLE,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn path_to_redemption(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Path to Redemption".into(),
        mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Aura],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn phoenix_fleet_airship(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Phoenix Fleet Airship".into(),
        mana_cost: ManaCost::parse("{2}{B}{B}"),
        card_types: vec![CardType::Artifact],
        subtypes: vec![SubType::Vehicle],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn planetarium_of_wan_shi_tong(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Planetarium of Wan Shi Tong".into(),
        mana_cost: ManaCost::parse("{6}"),
        card_types: vec![CardType::Artifact],
        supertypes: vec![SuperType::Legendary],
        rarity: Rarity::Mythic,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{1}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn platypus_bear(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Platypus-Bear".into(),
        mana_cost: ManaCost::parse("{1}{G/U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Custom("Platypus".into()), SubType::Bear],
        power: Some(2), toughness: Some(3),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::DEFENDER,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn ran_and_shaw(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ran and Shaw".into(),
        mana_cost: ManaCost::parse("{3}{R}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon],
        supertypes: vec![SuperType::Legendary],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{3}{R}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn raven_eagle(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Raven Eagle".into(),
        mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Bird, SubType::Assassin],
        power: Some(2), toughness: Some(3),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLYING,
        ..Default::default() }
}

fn realm_of_koh(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Realm of Koh".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{3}{B}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn ruinous_waterbending(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ruinous Waterbending".into(),
        mana_cost: ManaCost::parse("{1}{B}{B}"),
        card_types: vec![CardType::Sorcery],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::Custom("Spell effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn rumble_arena(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Rumble Arena".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Common,
        keywords: KeywordAbilities::VIGILANCE,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn sandbender_scavengers(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sandbender Scavengers".into(),
        mana_cost: ManaCost::parse("{W}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Rogue],
        power: Some(1), toughness: Some(1),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::dies_triggered(id,
                    "When this dies, trigger effect.",
                    vec![Effect::Custom("Dies effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn sandbenders_storm(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sandbenders' Storm".into(),
        mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::destroy()],
                    TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn secret_tunnel(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Secret Tunnel".into(),
        card_types: vec![CardType::Land],
        subtypes: vec![SubType::Cave],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{4}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn serpents_pass(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Serpent's Pass".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{4}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn sokka_tenacious_tactician(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sokka, Tenacious Tactician".into(),
        mana_cost: ManaCost::parse("{1}{U}{R}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::spell_cast_triggered(id,
                    "Whenever you cast a spell, trigger effect.",
                    vec![Effect::Custom("Spell cast trigger.".into())],
                    TargetSpec::None),
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn solstice_revelations(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Solstice Revelations".into(),
        mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Instant],
        subtypes: vec![SubType::Lesson],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                    "Flashback {6}{R}",
                    vec![Cost::pay_mana("{6}{R}")],
                    vec![Effect::Custom("Cast from graveyard.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn sozins_comet(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sozin's Comet".into(),
        mana_cost: ManaCost::parse("{3}{R}{R}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Mythic,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::Custom("Spell effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn sparring_dummy(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sparring Dummy".into(),
        mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Artifact, CardType::Creature],
        subtypes: vec![SubType::Custom("Scarecrow".into())],
        power: Some(1), toughness: Some(3),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::DEFENDER,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::tap_self()],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn suki_courageous_rescuer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Suki, Courageous Rescuer".into(),
        mana_cost: ManaCost::parse("{1}{W}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(4),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn sun_blessed_peak(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sun-Blessed Peak".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{4}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn swampsnare_trap(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Swampsnare Trap".into(),
        mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Aura],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn teo_spirited_glider(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Teo, Spirited Glider".into(),
        mana_cost: ManaCost::parse("{3}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Pilot, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(1), toughness: Some(4),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLYING,
        ..Default::default() }
}

fn the_earth_king(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "The Earth King".into(),
        mana_cost: ManaCost::parse("{3}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Noble, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn the_fire_nation_drill(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "The Fire Nation Drill".into(),
        mana_cost: ManaCost::parse("{2}{B}{B}"),
        card_types: vec![CardType::Artifact],
        subtypes: vec![SubType::Vehicle],
        supertypes: vec![SuperType::Legendary],
        power: Some(6), toughness: Some(3),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::TRAMPLE | KeywordAbilities::HEXPROOF | KeywordAbilities::INDESTRUCTIBLE,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::Creature),
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{1}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn the_last_agni_kai(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "The Last Agni Kai".into(),
        mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::Custom("Spell effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn the_lion_turtle(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "The Lion-Turtle".into(),
        mana_cost: ManaCost::parse("{1}{G}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elder, SubType::Cat, SubType::Turtle],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(6),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::VIGILANCE | KeywordAbilities::REACH,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn the_unagi_of_kyoshi_island(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "The Unagi of Kyoshi Island".into(),
        mana_cost: ManaCost::parse("{3}{U}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Serpent],
        supertypes: vec![SuperType::Legendary],
        power: Some(5), toughness: Some(5),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLASH,
        ..Default::default() }
}

fn the_walls_of_ba_sing_se(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "The Walls of Ba Sing Se".into(),
        mana_cost: ManaCost::parse("{8}"),
        card_types: vec![CardType::Artifact, CardType::Creature],
        subtypes: vec![SubType::Wall],
        supertypes: vec![SuperType::Legendary],
        power: Some(0), toughness: Some(30),
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::DEFENDER | KeywordAbilities::INDESTRUCTIBLE,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn tiger_seal(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Tiger-Seal".into(),
        mana_cost: ManaCost::parse("{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Cat, SubType::Custom("Seal".into())],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::VIGILANCE,
        ..Default::default() }
}

fn toph_hardheaded_teacher(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Toph, Hardheaded Teacher".into(),
        mana_cost: ManaCost::parse("{2}{R}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(4),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
            Ability::spell_cast_triggered(id,
                    "Whenever you cast a spell, trigger effect.",
                    vec![Effect::Custom("Spell cast trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn toph_the_blind_bandit(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Toph, the Blind Bandit".into(),
        mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(0), toughness: Some(3),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn toph_the_first_metalbender(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Toph, the First Metalbender".into(),
        mana_cost: ManaCost::parse("{1}{R}{G}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn trusty_boomerang(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Trusty Boomerang".into(),
        mana_cost: ManaCost::parse("{1}"),
        card_types: vec![CardType::Artifact],
        subtypes: vec![SubType::Equipment],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn tundra_tank(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Tundra Tank".into(),
        mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Artifact],
        subtypes: vec![SubType::Vehicle],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::INDESTRUCTIBLE,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn twin_blades(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Twin Blades".into(),
        mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Artifact],
        subtypes: vec![SubType::Equipment],
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLASH | KeywordAbilities::DOUBLE_STRIKE,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn ty_lee_chi_blocker(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ty Lee, Chi Blocker".into(),
        mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Performer, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(1),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLASH,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn unlucky_cabbage_merchant(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Unlucky Cabbage Merchant".into(),
        mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Custom("Citizen".into())],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn vindictive_warden(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Vindictive Warden".into(),
        mana_cost: ManaCost::parse("{2}{B/R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Soldier],
        power: Some(2), toughness: Some(3),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{3}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn walltop_sentries(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Walltop Sentries".into(),
        mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Soldier, SubType::Ally],
        power: Some(2), toughness: Some(3),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::REACH | KeywordAbilities::DEATHTOUCH,
        abilities: vec![
            Ability::dies_triggered(id,
                    "When this dies, trigger effect.",
                    vec![Effect::Custom("Dies effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn wan_shi_tong_librarian(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Wan Shi Tong, Librarian".into(),
        mana_cost: ManaCost::parse("{X}{U}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Bird, SubType::Spirit],
        supertypes: vec![SuperType::Legendary],
        power: Some(1), toughness: Some(1),
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::FLASH | KeywordAbilities::FLYING | KeywordAbilities::VIGILANCE,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn wandering_musicians(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Wandering Musicians".into(),
        mana_cost: ManaCost::parse("{3}{R/W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Custom("Bard".into()), SubType::Ally],
        power: Some(2), toughness: Some(5),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever this attacks, trigger effect.",
                    vec![EventType::AttackerDeclared],
                    vec![Effect::Custom("Attack trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn war_balloon(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "War Balloon".into(),
        mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Artifact],
        subtypes: vec![SubType::Vehicle],
        power: Some(4), toughness: Some(3),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{1}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn water_tribe_captain(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Water Tribe Captain".into(),
        mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Soldier, SubType::Ally],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{5}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn watery_grasp(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Watery Grasp".into(),
        mana_cost: ManaCost::parse("{U}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Aura],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::tap_self()],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn white_lotus_hideout(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "White Lotus Hideout".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn white_lotus_reinforcements(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "White Lotus Reinforcements".into(),
        mana_cost: ManaCost::parse("{1}{G}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Soldier, SubType::Ally],
        power: Some(2), toughness: Some(3),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::VIGILANCE,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn yue_the_moon_spirit(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Yue, the Moon Spirit".into(),
        mana_cost: ManaCost::parse("{3}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Spirit, SubType::Ally],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::VIGILANCE,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::tap_self()],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn zhao_ruthless_admiral(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Zhao, Ruthless Admiral".into(),
        mana_cost: ManaCost::parse("{2}{B/R}{B/R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Soldier],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(4),
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn zhao_the_moon_slayer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Zhao, the Moon Slayer".into(),
        mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Soldier],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{7}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn zuko_conflicted(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Zuko, Conflicted".into(),
        mana_cost: ManaCost::parse("{B}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Rogue],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(3),
        rarity: Rarity::Rare,
        ..Default::default() }
}

