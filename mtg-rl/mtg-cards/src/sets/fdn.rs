// Foundations (FDN) set — released 2024-11-15.
// 517 unique cards. Basic lands + Tier 1 vanilla/keyword creatures.

use crate::cards::basic_lands;
use crate::registry::CardRegistry;
use mtg_engine::abilities::{Ability, Cost, Effect, StaticEffect, TargetSpec};
use mtg_engine::filters::Filter;
use mtg_engine::card::CardData;
use mtg_engine::constants::*;
use mtg_engine::events::EventType;
use mtg_engine::mana::{Mana, ManaCost};
use mtg_engine::types::{ObjectId, PlayerId};

/// Register all implemented FDN cards into the registry.
pub fn register(registry: &mut CardRegistry) {
    basic_lands::register(registry, "FDN");

    registry.register("Aegis Turtle", aegis_turtle, "FDN");
    registry.register("Bear Cub", bear_cub, "FDN");
    registry.register("Bigfin Bouncer", bigfin_bouncer, "FDN");
    registry.register("Bishop's Soldier", bishops_soldier, "FDN");
    registry.register("Brazen Scourge", brazen_scourge, "FDN");
    registry.register("Burglar Rat", burglar_rat, "FDN");
    registry.register("Campus Guide", campus_guide, "FDN");
    registry.register("Diregraf Ghoul", diregraf_ghoul, "FDN");
    registry.register("Dragon Trainer", dragon_trainer, "FDN");
    registry.register("Druid of the Cowl", druid_of_the_cowl, "FDN");
    registry.register("Erudite Wizard", erudite_wizard, "FDN");
    registry.register("Evolving Wilds", evolving_wilds, "FDN");
    registry.register("Felidar Cub", felidar_cub, "FDN");
    registry.register("Firebrand Archer", firebrand_archer, "FDN");
    registry.register("Grappling Kraken", grappling_kraken, "FDN");
    registry.register("Healer's Hawk", healers_hawk, "FDN");
    registry.register("Hinterland Sanctifier", hinterland_sanctifier, "FDN");
    registry.register("Homunculus Horde", homunculus_horde, "FDN");
    registry.register("Infestation Sage", infestation_sage, "FDN");
    registry.register("Kitesail Corsair", kitesail_corsair, "FDN");
    registry.register("Leonin Skyhunter", leonin_skyhunter, "FDN");
    registry.register("Llanowar Elves", llanowar_elves, "FDN");
    registry.register("Maalfeld Twins", maalfeld_twins, "FDN");
    registry.register("Magnigoth Sentry", magnigoth_sentry, "FDN");
    registry.register("Marauding Blight-Priest", marauding_blight_priest, "FDN");
    registry.register("Pulse Tracker", pulse_tracker, "FDN");
    registry.register("Raging Redcap", raging_redcap, "FDN");
    registry.register("Ravenous Giant", ravenous_giant, "FDN");
    registry.register("Sanguine Syphoner", sanguine_syphoner, "FDN");
    registry.register("Serra Angel", serra_angel, "FDN");
    registry.register("Skeleton Archer", skeleton_archer, "FDN");
    registry.register("Skyraker Giant", skyraker_giant, "FDN");
    registry.register("Storm Fleet Spy", storm_fleet_spy, "FDN");
    registry.register("Surrak, the Hunt Caller", surrak_the_hunt_caller, "FDN");
    registry.register("Tajuru Pathwarden", tajuru_pathwarden, "FDN");
    registry.register("Thornweald Archer", thornweald_archer, "FDN");
    registry.register("Vampire Spawn", vampire_spawn, "FDN");
    registry.register("Viashino Pyromancer", viashino_pyromancer, "FDN");
    registry.register("Wary Thespian", wary_thespian, "FDN");
    registry.register("Wildheart Invoker", wildheart_invoker, "FDN");

    // Remaining Tier 1 — creatures with abilities
    registry.register("Ajani's Pridemate", ajanis_pridemate, "FDN");
    registry.register("Axgard Cavalry", axgard_cavalry, "FDN");
    registry.register("Battle-Rattle Shaman", battle_rattle_shaman, "FDN");
    registry.register("Crackling Cyclops", crackling_cyclops, "FDN");
    registry.register("Crusader of Odric", crusader_of_odric, "FDN");
    registry.register("Dryad Militant", dryad_militant, "FDN");
    registry.register("Good-Fortune Unicorn", good_fortune_unicorn, "FDN");
    registry.register("Helpful Hunter", helpful_hunter, "FDN");
    registry.register("Mild-Mannered Librarian", mild_mannered_librarian, "FDN");
    registry.register("Mystic Archaeologist", mystic_archaeologist, "FDN");
    registry.register("Reassembling Skeleton", reassembling_skeleton, "FDN");
    registry.register("Spitfire Lagac", spitfire_lagac, "FDN");
    registry.register("Tatyova, Benthic Druid", tatyova_benthic_druid, "FDN");

    // Remaining Tier 1 — spells
    registry.register("Banishing Light", banishing_light, "FDN");
    registry.register("Burst Lightning", burst_lightning, "FDN");
    registry.register("Divine Resilience", divine_resilience, "FDN");
    registry.register("Electroduplicate", electroduplicate, "FDN");
    registry.register("Grow from the Ashes", grow_from_the_ashes, "FDN");
    registry.register("Into the Roil", into_the_roil, "FDN");
    registry.register("Rite of Replication", rite_of_replication, "FDN");
    registry.register("Rite of the Dragoncaller", rite_of_the_dragoncaller, "FDN");
    registry.register("Self-Reflection", self_reflection, "FDN");
    registry.register("Think Twice", think_twice, "FDN");

    // Remaining Tier 1 — artifacts and enchantments
    registry.register("Basilisk Collar", basilisk_collar, "FDN");
    registry.register("Feldon's Cane", feldons_cane, "FDN");
    registry.register("Gilded Lotus", gilded_lotus, "FDN");
    registry.register("Impact Tremors", impact_tremors, "FDN");
    registry.register("Omniscience", omniscience, "FDN");
    registry.register("Phyrexian Arena", phyrexian_arena, "FDN");
    registry.register("Swiftfoot Boots", swiftfoot_boots, "FDN");

    // ── Tier 2 — vanilla/keyword creatures ──────────────────────────────────
    registry.register("Savannah Lions", savannah_lions, "FDN");
    registry.register("Fire Elemental", fire_elemental, "FDN");
    registry.register("Gigantosaurus", gigantosaurus, "FDN");
    registry.register("Highborn Vampire", highborn_vampire, "FDN");
    registry.register("Swab Goblin", swab_goblin, "FDN");
    registry.register("Vampire Interloper", vampire_interloper, "FDN");
    registry.register("Juggernaut", juggernaut, "FDN");
    registry.register("Shivan Dragon", shivan_dragon, "FDN");
    registry.register("Thrashing Brontodon", thrashing_brontodon, "FDN");
    registry.register("Gnarlback Rhino", gnarlback_rhino, "FDN");
    registry.register("Eager Trufflesnout", eager_trufflesnout, "FDN");
    registry.register("Tempest Djinn", tempest_djinn, "FDN");
    registry.register("Tolarian Terror", tolarian_terror, "FDN");
    registry.register("Solemn Simulacrum", solemn_simulacrum, "FDN");

    // ── Tier 2 — creatures with abilities ───────────────────────────────────
    registry.register("Dazzling Angel", dazzling_angel, "FDN");
    registry.register("Inspiring Overseer", inspiring_overseer, "FDN");
    registry.register("Empyrean Eagle", empyrean_eagle, "FDN");
    registry.register("Brineborn Cutthroat", brineborn_cutthroat, "FDN");
    registry.register("Exclusion Mage", exclusion_mage, "FDN");
    registry.register("Crow of Dark Tidings", crow_of_dark_tidings, "FDN");
    registry.register("Vampire Neonate", vampire_neonate, "FDN");
    registry.register("Hungry Ghoul", hungry_ghoul, "FDN");
    registry.register("Driver of the Dead", driver_of_the_dead, "FDN");
    registry.register("Fanatical Firebrand", fanatical_firebrand, "FDN");
    registry.register("Goblin Smuggler", goblin_smuggler, "FDN");
    registry.register("Frenzied Goblin", frenzied_goblin, "FDN");
    registry.register("Ghitu Lavarunner", ghitu_lavarunner, "FDN");
    registry.register("Courageous Goblin", courageous_goblin, "FDN");
    registry.register("Heartfire Immolator", heartfire_immolator, "FDN");
    registry.register("Dragonlord's Servant", dragonlords_servant, "FDN");
    registry.register("Rapacious Dragon", rapacious_dragon, "FDN");
    registry.register("Dragon Mage", dragon_mage, "FDN");
    registry.register("Reclamation Sage", reclamation_sage, "FDN");
    registry.register("Fierce Empath", fierce_empath, "FDN");
    registry.register("Beast-Kin Ranger", beast_kin_ranger, "FDN");
    registry.register("Elvish Regrower", elvish_regrower, "FDN");
    registry.register("Springbloom Druid", springbloom_druid, "FDN");
    registry.register("Mold Adder", mold_adder, "FDN");
    registry.register("Leonin Vanguard", leonin_vanguard, "FDN");
    registry.register("Skyship Buccaneer", skyship_buccaneer, "FDN");
    registry.register("Felidar Savior", felidar_savior, "FDN");
    registry.register("Guttersnipe", guttersnipe, "FDN");
    registry.register("Stromkirk Noble", stromkirk_noble, "FDN");
    registry.register("Gleaming Barrier", gleaming_barrier, "FDN");
    registry.register("Rune-Sealed Wall", rune_sealed_wall, "FDN");
    registry.register("Gateway Sneak", gateway_sneak, "FDN");

    // ── Tier 2 — instants ───────────────────────────────────────────────────
    registry.register("Giant Growth", giant_growth, "FDN");
    registry.register("Unsummon", unsummon, "FDN");
    registry.register("Cancel", cancel, "FDN");
    registry.register("Negate", negate, "FDN");
    registry.register("Essence Scatter", essence_scatter, "FDN");
    registry.register("Opt", opt, "FDN");
    registry.register("Sure Strike", sure_strike, "FDN");
    registry.register("Dive Down", dive_down, "FDN");
    registry.register("Disenchant", disenchant, "FDN");
    registry.register("Kindled Fury", kindled_fury, "FDN");
    registry.register("Stab", stab, "FDN");
    registry.register("Adamant Will", adamant_will, "FDN");
    registry.register("Broken Wings", broken_wings, "FDN");
    registry.register("Moment of Triumph", moment_of_triumph, "FDN");
    registry.register("Moment of Craving", moment_of_craving, "FDN");
    registry.register("Quick Study", quick_study, "FDN");
    registry.register("Aetherize", aetherize, "FDN");
    registry.register("Scorching Dragonfire", scorching_dragonfire, "FDN");
    registry.register("Bite Down", bite_down, "FDN");
    registry.register("Snakeskin Veil", snakeskin_veil, "FDN");
    registry.register("Fleeting Flight", fleeting_flight, "FDN");
    registry.register("Undying Malice", undying_malice, "FDN");
    registry.register("Fake Your Own Death", fake_your_own_death, "FDN");

    // ── Tier 2 — sorceries ──────────────────────────────────────────────────
    registry.register("Duress", duress, "FDN");
    registry.register("Deathmark", deathmark, "FDN");
    registry.register("Crash Through", crash_through, "FDN");
    registry.register("Dragon Fodder", dragon_fodder, "FDN");
    registry.register("Day of Judgment", day_of_judgment, "FDN");
    registry.register("Angelic Edict", angelic_edict, "FDN");
    registry.register("Chart a Course", chart_a_course, "FDN");
    registry.register("Macabre Waltz", macabre_waltz, "FDN");
    registry.register("Bulk Up", bulk_up, "FDN");

    // ── Tier 2 — enchantments ───────────────────────────────────────────────
    registry.register("Pacifism", pacifism, "FDN");

    // ── Tier 3 — modal spells ─────────────────────────────────────────────
    registry.register("Abrade", abrade, "FDN");
    registry.register("Boros Charm", boros_charm, "FDN");
    registry.register("Slagstorm", slagstorm, "FDN");
    registry.register("Valorous Stance", valorous_stance, "FDN");

    // ── Tier 3 — pump/anthem spells ───────────────────────────────────────
    registry.register("Heroic Reinforcements", heroic_reinforcements, "FDN");
    registry.register("Make a Stand", make_a_stand, "FDN");
    registry.register("Overrun", overrun, "FDN");

    // ── Tier 3 — lords and tribal creatures ───────────────────────────────
    registry.register("Death Baron", death_baron, "FDN");
    registry.register("Elvish Archdruid", elvish_archdruid, "FDN");
    registry.register("Vampire Nighthawk", vampire_nighthawk, "FDN");

    // ── Tier 3 — enchantments ─────────────────────────────────────────────
    registry.register("Goblin Oriflamme", goblin_oriflamme, "FDN");

    // ── Tier 3 — creatures with complex abilities ─────────────────────────
    registry.register("Fog Bank", fog_bank, "FDN");

    // ── Tier 3 — batch 2 ────────────────────────────────────────────────────
    registry.register("Abyssal Harvester", abyssal_harvester, "FDN");
    registry.register("Adaptive Automaton", adaptive_automaton, "FDN");
    registry.register("Aggressive Mammoth", aggressive_mammoth, "FDN");
    registry.register("Alesha, Who Laughs at Fate", alesha_who_laughs_at_fate, "FDN");
    registry.register("Angelic Destiny", angelic_destiny, "FDN");
    registry.register("Angel of Vitality", angel_of_vitality, "FDN");
    registry.register("Anthem of Champions", anthem_of_champions, "FDN");
    registry.register("Apothecary Stomper", apothecary_stomper, "FDN");
    registry.register("Arahbo, the First Fang", arahbo_the_first_fang, "FDN");
    registry.register("Aurelia, the Warleader", aurelia_the_warleader, "FDN");
    registry.register("Ayli, Eternal Pilgrim", ayli_eternal_pilgrim, "FDN");
    registry.register("Azorius Guildgate", azorius_guildgate, "FDN");
    registry.register("Ball Lightning", ball_lightning, "FDN");
    registry.register("Balmor, Battlemage Captain", balmor_battlemage_captain, "FDN");
    registry.register("Banner of Kinship", banner_of_kinship, "FDN");
    registry.register("Billowing Shriekmass", billowing_shriekmass, "FDN");
    registry.register("Bloodfell Caves", fdn_bloodfell_caves, "FDN");
    registry.register("Bloodthirsty Conqueror", bloodthirsty_conqueror, "FDN");
    registry.register("Blossoming Sands", fdn_blossoming_sands, "FDN");
    registry.register("Boros Guildgate", boros_guildgate, "FDN");
    registry.register("Bushwhack", bushwhack, "FDN");
    registry.register("Carnelian Orb of Dragonkind", carnelian_orb_of_dragonkind, "FDN");
    registry.register("Celestial Armor", celestial_armor, "FDN");
    registry.register("Charming Prince", charming_prince, "FDN");
    registry.register("Claws Out", claws_out, "FDN");
    registry.register("Consuming Aberration", consuming_aberration, "FDN");
    registry.register("Corsair Captain", corsair_captain, "FDN");
    registry.register("Crossway Troublemakers", crossway_troublemakers, "FDN");
    registry.register("Crystal Barricade", crystal_barricade, "FDN");
    registry.register("Curator of Destinies", curator_of_destinies, "FDN");
    registry.register("Darksteel Colossus", darksteel_colossus, "FDN");
    registry.register("Dauntless Veteran", dauntless_veteran, "FDN");
    registry.register("Dawnwing Marshal", dawnwing_marshal, "FDN");
    registry.register("Deadly Brew", deadly_brew, "FDN");
    registry.register("Deadly Plot", deadly_plot, "FDN");
    registry.register("Demonic Pact", demonic_pact, "FDN");
    registry.register("Desecration Demon", desecration_demon, "FDN");
    registry.register("Diamond Mare", diamond_mare, "FDN");
    registry.register("Dimir Guildgate", dimir_guildgate, "FDN");
    registry.register("Dismal Backwater", dismal_backwater, "FDN");
    registry.register("Doubling Season", doubling_season, "FDN");
    registry.register("Drake Hatcher", drake_hatcher, "FDN");
    registry.register("Dread Summons", dread_summons, "FDN");
    registry.register("Dreadwing Scavenger", dreadwing_scavenger, "FDN");
    registry.register("Drogskol Reaver", drogskol_reaver, "FDN");
    registry.register("Dropkick Bomber", dropkick_bomber, "FDN");
    registry.register("Dwynen, Gilt-Leaf Daen", dwynen_gilt_leaf_daen, "FDN");
    registry.register("Eaten by Piranhas", eaten_by_piranhas, "FDN");
    registry.register("Elenda, Saint of Dusk", elenda_saint_of_dusk, "FDN");
    registry.register("Etali, Primal Storm", etali_primal_storm, "FDN");
    registry.register("Exemplar of Light", exemplar_of_light, "FDN");
    registry.register("Feed the Swarm", feed_the_swarm, "FDN");
    registry.register("Felidar Retreat", felidar_retreat, "FDN");
    registry.register("Fiendish Panda", fiendish_panda, "FDN");
    registry.register("Finale of Revelation", finale_of_revelation, "FDN");
    registry.register("Fishing Pole", fishing_pole, "FDN");
    registry.register("Flamewake Phoenix", flamewake_phoenix, "FDN");
    registry.register("Garruk's Uprising", garruks_uprising, "FDN");
    registry.register("Gate Colossus", gate_colossus, "FDN");
    registry.register("Genesis Wave", genesis_wave, "FDN");
    registry.register("Ghalta, Primal Hunger", ghalta_primal_hunger, "FDN");
    registry.register("Giada, Font of Hope", giada_font_of_hope, "FDN");
    registry.register("Gnarlid Colony", gnarlid_colony, "FDN");
    registry.register("Goblin Surprise", goblin_surprise, "FDN");
    registry.register("Goldvein Pick", goldvein_pick, "FDN");
    registry.register("Golgari Guildgate", golgari_guildgate, "FDN");
    registry.register("Gratuitous Violence", gratuitous_violence, "FDN");
    registry.register("Gruul Guildgate", gruul_guildgate, "FDN");
    registry.register("Halana and Alena, Partners", halana_and_alena_partners, "FDN");
    registry.register("Herald of Eternal Dawn", herald_of_eternal_dawn, "FDN");
    registry.register("Heraldic Banner", heraldic_banner, "FDN");
    registry.register("High Fae Trickster", high_fae_trickster, "FDN");
    registry.register("High-Society Hunter", high_society_hunter, "FDN");
    registry.register("Hoarding Dragon", hoarding_dragon, "FDN");
    registry.register("Immersturm Predator", immersturm_predator, "FDN");
    registry.register("Imperious Perfect", imperious_perfect, "FDN");
    registry.register("Infernal Vessel", infernal_vessel, "FDN");
    registry.register("Ingenious Leonin", ingenious_leonin, "FDN");
    registry.register("Inspiration from Beyond", inspiration_from_beyond, "FDN");
    registry.register("Inspiring Call", inspiring_call, "FDN");
    registry.register("Inspiring Paladin", inspiring_paladin, "FDN");
    registry.register("Izzet Guildgate", izzet_guildgate, "FDN");
    registry.register("Jazal Goldmane", jazal_goldmane, "FDN");
    registry.register("Joraga Invocation", joraga_invocation, "FDN");
    registry.register("Jungle Hollow", jungle_hollow, "FDN");
    registry.register("Kalastria Highborn", kalastria_highborn, "FDN");
    registry.register("Kellan, Planar Trailblazer", kellan_planar_trailblazer, "FDN");
    registry.register("Knight of Grace", knight_of_grace, "FDN");
    registry.register("Knight of Malice", knight_of_malice, "FDN");
    registry.register("Koma, World-Eater", koma_world_eater, "FDN");
    registry.register("Kykar, Zephyr Awakener", kykar_zephyr_awakener, "FDN");
    registry.register("Lathliss, Dragon Queen", lathliss_dragon_queen, "FDN");
    registry.register("Lathril, Blade of the Elves", lathril_blade_of_the_elves, "FDN");
    registry.register("Leyline Axe", leyline_axe, "FDN");
    registry.register("Lunar Insight", lunar_insight, "FDN");
    registry.register("Lyra Dawnbringer", lyra_dawnbringer, "FDN");
    registry.register("Massacre Wurm", massacre_wurm, "FDN");
    registry.register("Mazemind Tome", mazemind_tome, "FDN");
    registry.register("Maze's End", mazes_end, "FDN");
    registry.register("Mindsparker", mindsparker, "FDN");
    registry.register("Mossborn Hydra", mossborn_hydra, "FDN");
    registry.register("Muldrotha, the Gravetide", muldrotha_the_gravetide, "FDN");
    registry.register("Myojin of Night's Reach", myojin_of_nights_reach, "FDN");
    registry.register("New Horizons", new_horizons, "FDN");
    registry.register("Nine-Lives Familiar", nine_lives_familiar, "FDN");
    registry.register("Niv-Mizzet, Visionary", niv_mizzet_visionary, "FDN");
    registry.register("Nullpriest of Oblivion", nullpriest_of_oblivion, "FDN");
    registry.register("Ordeal of Nylea", ordeal_of_nylea, "FDN");
    registry.register("Orzhov Guildgate", orzhov_guildgate, "FDN");
    registry.register("Ovika, Enigma Goliath", ovika_enigma_goliath, "FDN");
    registry.register("Pelakka Wurm", pelakka_wurm, "FDN");
    registry.register("Perforating Artist", perforating_artist, "FDN");
    registry.register("Pirate's Cutlass", pirates_cutlass, "FDN");
    registry.register("Predator Ooze", predator_ooze, "FDN");
    registry.register("Preposterous Proportions", preposterous_proportions, "FDN");
    registry.register("Primeval Bounty", primeval_bounty, "FDN");
    registry.register("Pyromancer's Goggles", pyromancers_goggles, "FDN");
    registry.register("Quilled Greatwurm", quilled_greatwurm, "FDN");
    registry.register("Rakdos Guildgate", rakdos_guildgate, "FDN");
    registry.register("Ramos, Dragon Engine", ramos_dragon_engine, "FDN");
    registry.register("Redcap Gutter-Dweller", redcap_gutter_dweller, "FDN");
    registry.register("Regal Caracal", regal_caracal, "FDN");
    registry.register("Ruby, Daring Tracker", ruby_daring_tracker, "FDN");
    registry.register("Rugged Highlands", rugged_highlands, "FDN");
    registry.register("Scavenging Ooze", scavenging_ooze, "FDN");
    registry.register("Scoured Barrens", scoured_barrens, "FDN");
    registry.register("Secluded Courtyard", secluded_courtyard, "FDN");
    registry.register("Seeker's Folly", seekers_folly, "FDN");
    registry.register("Selesnya Guildgate", selesnya_guildgate, "FDN");
    registry.register("Simic Guildgate", simic_guildgate, "FDN");
    registry.register("Sire of Seven Deaths", sire_of_seven_deaths, "FDN");
    registry.register("Soul-Guide Lantern", soul_guide_lantern, "FDN");
    registry.register("Soul-Shackled Zombie", soul_shackled_zombie, "FDN");
    registry.register("Spectral Sailor", spectral_sailor, "FDN");
    registry.register("Sphinx of Forgotten Lore", sphinx_of_forgotten_lore, "FDN");
    registry.register("Sphinx of the Final Word", sphinx_of_the_final_word, "FDN");
    registry.register("Starlight Snare", starlight_snare, "FDN");
    registry.register("Steel Hellkite", steel_hellkite, "FDN");
    registry.register("Strix Lookout", strix_lookout, "FDN");
    registry.register("Sun-Blessed Healer", sun_blessed_healer, "FDN");
    registry.register("Swiftblade Vindicator", swiftblade_vindicator, "FDN");
    registry.register("Swiftwater Cliffs", swiftwater_cliffs, "FDN");
    registry.register("Sylvan Scavenging", sylvan_scavenging, "FDN");
    registry.register("Syr Alin, the Lion's Claw", syr_alin_the_lions_claw, "FDN");
    registry.register("Temple of Abandon", temple_of_abandon, "FDN");
    registry.register("Temple of Deceit", temple_of_deceit, "FDN");
    registry.register("Temple of Enlightenment", temple_of_enlightenment, "FDN");
    registry.register("Temple of Epiphany", temple_of_epiphany, "FDN");
    registry.register("Temple of Malady", temple_of_malady, "FDN");
    registry.register("Temple of Malice", temple_of_malice, "FDN");
    registry.register("Temple of Mystery", temple_of_mystery, "FDN");
    registry.register("Temple of Plenty", temple_of_plenty, "FDN");
    registry.register("Temple of Silence", temple_of_silence, "FDN");
    registry.register("Temple of Triumph", temple_of_triumph, "FDN");
    registry.register("Terror of Mount Velus", terror_of_mount_velus, "FDN");
    registry.register("Thornwood Falls", thornwood_falls, "FDN");
    registry.register("Thousand-Year Storm", thousand_year_storm, "FDN");
    registry.register("Tinybones, Bauble Burglar", tinybones_bauble_burglar, "FDN");
    registry.register("Tranquil Cove", tranquil_cove, "FDN");
    registry.register("Treetop Snarespinner", treetop_snarespinner, "FDN");
    registry.register("Twinblade Blessing", twinblade_blessing, "FDN");
    registry.register("Twinflame Tyrant", twinflame_tyrant, "FDN");
    registry.register("Valkyrie's Call", valkyries_call, "FDN");
    registry.register("Vampire Soulcaller", vampire_soulcaller, "FDN");
    registry.register("Vizier of the Menagerie", vizier_of_the_menagerie, "FDN");
    registry.register("Wardens of the Cycle", wardens_of_the_cycle, "FDN");
    registry.register("Wildborn Preserver", wildborn_preserver, "FDN");
    registry.register("Wildwood Scourge", wildwood_scourge, "FDN");
    registry.register("Wilt-Leaf Liege", wilt_leaf_liege, "FDN");
    registry.register("Wind-Scarred Crag", wind_scarred_crag, "FDN");
    registry.register("Wishclaw Talisman", wishclaw_talisman, "FDN");
    registry.register("Witness Protection", witness_protection, "FDN");
    registry.register("Zetalpa, Primal Dawn", zetalpa_primal_dawn, "FDN");
    registry.register("Zimone, Paradox Sculptor", zimone_paradox_sculptor, "FDN");

    // ── New Creatures ────────────────────────────────────────────────────
    registry.register("Affectionate Indrik", affectionate_indrik, "FDN");
    registry.register("Ambush Wolf", ambush_wolf, "FDN");
    registry.register("Ancestor Dragon", ancestor_dragon, "FDN");
    registry.register("Angel of Finality", angel_of_finality, "FDN");
    registry.register("Arbiter of Woe", arbiter_of_woe, "FDN");
    registry.register("Arcanis the Omnipotent", arcanis_the_omnipotent, "FDN");
    registry.register("Archmage of Runes", archmage_of_runes, "FDN");
    registry.register("Archway Angel", archway_angel, "FDN");
    registry.register("Armasaur Guide", armasaur_guide, "FDN");
    registry.register("Ashroot Animist", ashroot_animist, "FDN");
    registry.register("Ballyrush Banneret", ballyrush_banneret, "FDN");
    registry.register("Battlesong Berserker", battlesong_berserker, "FDN");
    registry.register("Bloodtithe Collector", bloodtithe_collector, "FDN");
    registry.register("Burnished Hart", burnished_hart, "FDN");
    registry.register("Burrog Befuddler", burrog_befuddler, "FDN");
    registry.register("Cackling Prowler", cackling_prowler, "FDN");
    registry.register("Cat Collector", cat_collector, "FDN");
    registry.register("Cathar Commando", cathar_commando, "FDN");
    registry.register("Cephalid Inkmage", cephalid_inkmage, "FDN");
    registry.register("Clinquant Skymage", clinquant_skymage, "FDN");
    registry.register("Cloudblazer", cloudblazer, "FDN");
    registry.register("Crypt Feaster", crypt_feaster, "FDN");
    registry.register("Cultivator's Caravan", cultivators_caravan, "FDN");
    registry.register("Dragonmaster Outcast", dragonmaster_outcast, "FDN");
    registry.register("Drakuseth, Maw of Flames", drakuseth_maw_of_flames, "FDN");
    registry.register("Dwynen's Elite", dwynens_elite, "FDN");
    registry.register("Elementalist Adept", elementalist_adept, "FDN");
    registry.register("Elfsworn Giant", elfsworn_giant, "FDN");
    registry.register("Enigma Drake", enigma_drake, "FDN");
    registry.register("Firespitter Whelp", firespitter_whelp, "FDN");
    registry.register("Fynn, the Fangbearer", fynn_the_fangbearer, "FDN");
    registry.register("Garna, Bloodfist of Keld", garna_bloodfist_of_keld, "FDN");
    registry.register("Gatekeeper of Malakir", gatekeeper_of_malakir, "FDN");
    registry.register("Giant Cindermaw", giant_cindermaw, "FDN");
    registry.register("Goblin Boarders", goblin_boarders, "FDN");
    registry.register("Gorehorn Raider", gorehorn_raider, "FDN");
    registry.register("Guarded Heir", guarded_heir, "FDN");
    registry.register("Gutless Plunderer", gutless_plunderer, "FDN");
    registry.register("Harbinger of the Tides", harbinger_of_the_tides, "FDN");
    registry.register("Hare Apparent", hare_apparent, "FDN");
    registry.register("Herald of Faith", herald_of_faith, "FDN");
    registry.register("Heroes' Bane", heroes_bane, "FDN");
    registry.register("Icewind Elemental", icewind_elemental, "FDN");
    registry.register("Kargan Dragonrider", kargan_dragonrider, "FDN");
    registry.register("Kiora, the Rising Tide", kiora_the_rising_tide, "FDN");
    registry.register("Krenko, Mob Boss", krenko_mob_boss, "FDN");
    registry.register("Lightshell Duo", lightshell_duo, "FDN");
    registry.register("Linden, the Steadfast Queen", linden_the_steadfast_queen, "FDN");
    registry.register("Loot, Exuberant Explorer", loot_exuberant_explorer, "FDN");
    registry.register("Mentor of the Meek", mentor_of_the_meek, "FDN");
    registry.register("Meteor Golem", meteor_golem, "FDN");
    registry.register("Micromancer", micromancer, "FDN");
    registry.register("Midnight Reaper", midnight_reaper, "FDN");
    registry.register("Mischievous Mystic", mischievous_mystic, "FDN");
    registry.register("Mischievous Pup", mischievous_pup, "FDN");
    registry.register("Mocking Sprite", mocking_sprite, "FDN");
    registry.register("Needletooth Pack", needletooth_pack, "FDN");
    registry.register("Nessian Hornbeetle", nessian_hornbeetle, "FDN");
    registry.register("Prideful Parent", prideful_parent, "FDN");
    registry.register("Prime Speaker Zegana", prime_speaker_zegana, "FDN");
    registry.register("Progenitus", progenitus, "FDN");
    registry.register("Quakestrider Ceratops", quakestrider_ceratops, "FDN");
    registry.register("Rampaging Baloths", rampaging_baloths, "FDN");
    registry.register("Resolute Reinforcements", resolute_reinforcements, "FDN");
    registry.register("Savage Ventmaw", savage_ventmaw, "FDN");
    registry.register("Scrawling Crawler", scrawling_crawler, "FDN");
    registry.register("Searslicer Goblin", searslicer_goblin, "FDN");
    registry.register("Shipwreck Dowser", shipwreck_dowser, "FDN");
    registry.register("Skyknight Squire", skyknight_squire, "FDN");
    registry.register("Slumbering Cerberus", slumbering_cerberus, "FDN");
    registry.register("Sower of Chaos", sower_of_chaos, "FDN");
    registry.register("Spinner of Souls", spinner_of_souls, "FDN");
    registry.register("Squad Rallier", squad_rallier, "FDN");
    registry.register("Stromkirk Bloodthief", stromkirk_bloodthief, "FDN");
    registry.register("Strongbox Raider", strongbox_raider, "FDN");
    registry.register("Suspicious Shambler", suspicious_shambler, "FDN");
    registry.register("Taurean Mauler", taurean_mauler, "FDN");
    registry.register("Three Tree Mascot", three_tree_mascot, "FDN");
    registry.register("Tragic Banshee", tragic_banshee, "FDN");
    registry.register("Trygon Predator", trygon_predator, "FDN");
    registry.register("Twinblade Paladin", twinblade_paladin, "FDN");
    registry.register("Vampire Gourmand", vampire_gourmand, "FDN");
    registry.register("Vanguard Seraph", vanguard_seraph, "FDN");
    registry.register("Vengeful Bloodwitch", vengeful_bloodwitch, "FDN");
    registry.register("Venom Connoisseur", venom_connoisseur, "FDN");
    registry.register("Vile Entomber", vile_entomber, "FDN");
    registry.register("Volley Veteran", volley_veteran, "FDN");
    registry.register("Voracious Greatshark", voracious_greatshark, "FDN");
    registry.register("Youthful Valkyrie", youthful_valkyrie, "FDN");
    registry.register("Zul Ashur, Lich Lord", zul_ashur_lich_lord, "FDN");

    // ── New Instants and Sorceries ────────────────────────────────────────
    registry.register("An Offer You Can't Refuse", an_offer_you_cant_refuse, "FDN");
    registry.register("Arcane Epiphany", arcane_epiphany, "FDN");
    registry.register("Bake into a Pie", bake_into_a_pie, "FDN");
    registry.register("Biogenic Upgrade", biogenic_upgrade, "FDN");
    registry.register("Blasphemous Edict", blasphemous_edict, "FDN");
    registry.register("Bolt Bend", bolt_bend, "FDN");
    registry.register("Boltwave", boltwave, "FDN");
    registry.register("Brass's Bounty", brasss_bounty, "FDN");
    registry.register("Cemetery Recruitment", cemetery_recruitment, "FDN");
    registry.register("Circuitous Route", circuitous_route, "FDN");
    registry.register("Deadly Riposte", deadly_riposte, "FDN");
    registry.register("Devout Decree", devout_decree, "FDN");
    registry.register("Eaten Alive", eaten_alive, "FDN");
    registry.register("Elspeth's Smite", elspeths_smite, "FDN");
    registry.register("Exsanguinate", exsanguinate, "FDN");
    registry.register("Faebloom Trick", faebloom_trick, "FDN");
    registry.register("Felling Blow", felling_blow, "FDN");
    registry.register("Fiery Annihilation", fiery_annihilation, "FDN");
    registry.register("Flashfreeze", flashfreeze, "FDN");
    registry.register("Fleeting Distraction", fleeting_distraction, "FDN");
    registry.register("Fumigate", fumigate, "FDN");
    registry.register("Goblin Negotiation", goblin_negotiation, "FDN");
    registry.register("Harmless Offering", harmless_offering, "FDN");
    registry.register("Hero's Downfall", heros_downfall, "FDN");
    registry.register("Hidetsugu's Second Rite", hidetsugus_second_rite, "FDN");
    registry.register("Incinerating Blast", incinerating_blast, "FDN");
    registry.register("Involuntary Employment", involuntary_employment, "FDN");
    registry.register("Joust Through", joust_through, "FDN");
    registry.register("Luminous Rebuke", luminous_rebuke, "FDN");
    registry.register("Maelstrom Pulse", maelstrom_pulse, "FDN");
    registry.register("Make Your Move", make_your_move, "FDN");
    registry.register("Mortify", mortify, "FDN");
    registry.register("Mystical Teachings", mystical_teachings, "FDN");
    registry.register("Obliterating Bolt", obliterating_bolt, "FDN");
    registry.register("Offer Immortality", offer_immortality, "FDN");
    registry.register("Pilfer", pilfer, "FDN");
    registry.register("Primal Might", primal_might, "FDN");
    registry.register("Raise the Past", raise_the_past, "FDN");
    registry.register("Refute", refute, "FDN");
    registry.register("Release the Dogs", release_the_dogs, "FDN");
    registry.register("Revenge of the Rats", revenge_of_the_rats, "FDN");
    registry.register("Rise of the Dark Realms", rise_of_the_dark_realms, "FDN");
    registry.register("River's Rebuke", rivers_rebuke, "FDN");
    registry.register("Run Away Together", run_away_together, "FDN");
    registry.register("Sanguine Indulgence", sanguine_indulgence, "FDN");
    registry.register("Seismic Rupture", seismic_rupture, "FDN");
    registry.register("Seize the Spoils", seize_the_spoils, "FDN");
    registry.register("Stroke of Midnight", stroke_of_midnight, "FDN");
    registry.register("Teach by Example", teach_by_example, "FDN");
    registry.register("Thrill of Possibility", thrill_of_possibility, "FDN");
    registry.register("Time Stop", time_stop, "FDN");
    registry.register("Tribute to Hunger", tribute_to_hunger, "FDN");
    registry.register("Uncharted Voyage", uncharted_voyage, "FDN");
    registry.register("Zombify", zombify, "FDN");

    // ── New Lands ─────────────────────────────────────────────────────────
    registry.register("Crawling Barrens", crawling_barrens, "FDN");
    registry.register("Cryptic Caves", cryptic_caves, "FDN");
    registry.register("Demolition Field", demolition_field, "FDN");
    registry.register("Rogue's Passage", rogues_passage, "FDN");
    registry.register("Soulstone Sanctuary", soulstone_sanctuary, "FDN");
    registry.register("Uncharted Haven", uncharted_haven, "FDN");

    // ── New Artifacts ─────────────────────────────────────────────────────
    registry.register("Adventuring Gear", adventuring_gear, "FDN");
    registry.register("Expedition Map", expedition_map, "FDN");
    registry.register("Fireshrieker", fireshrieker, "FDN");
    registry.register("Goblin Firebomb", goblin_firebomb, "FDN");
    registry.register("Hedron Archive", hedron_archive, "FDN");
    registry.register("Quick-Draw Katana", quick_draw_katana, "FDN");
    registry.register("Ravenous Amulet", ravenous_amulet, "FDN");
    registry.register("Sorcerous Spyglass", sorcerous_spyglass, "FDN");

    // ── New Enchantments ──────────────────────────────────────────────────
    registry.register("Authority of the Consuls", authority_of_the_consuls, "FDN");
    registry.register("Blanchwood Armor", blanchwood_armor, "FDN");
    registry.register("Confiscate", confiscate, "FDN");
    registry.register("Dictate of Kruphix", dictate_of_kruphix, "FDN");
    registry.register("Extravagant Replication", extravagant_replication, "FDN");
    registry.register("Imprisoned in the Moon", imprisoned_in_the_moon, "FDN");
    registry.register("Midnight Snack", midnight_snack, "FDN");
    registry.register("Painful Quandary", painful_quandary, "FDN");
    registry.register("Prayer of Binding", prayer_of_binding, "FDN");
    registry.register("Stasis Snare", stasis_snare, "FDN");
    registry.register("Unflinching Courage", unflinching_courage, "FDN");
    registry.register("Untamed Hunger", untamed_hunger, "FDN");
    registry.register("Vampiric Rites", vampiric_rites, "FDN");

    // ── Other ─────────────────────────────────────────────────────────────
    registry.register("Ajani, Caller of the Pride", ajani_caller_of_the_pride, "FDN");
    registry.register("Chandra, Flameshaper", chandra_flameshaper, "FDN");
    registry.register("Kaito, Cunning Infiltrator", kaito_cunning_infiltrator, "FDN");
    registry.register("Liliana, Dreadhorde General", liliana_dreadhorde_general, "FDN");
    registry.register("Rune-Scarred Demon", rune_scarred_demon, "FDN");
    registry.register("Vivien Reid", vivien_reid, "FDN");
}

fn aegis_turtle(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Aegis Turtle".into(), mana_cost: ManaCost::parse("{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Turtle],
        power: Some(0), toughness: Some(5), rarity: Rarity::Common, ..Default::default() }
}

fn bear_cub(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Bear Cub".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Bear],
        power: Some(2), toughness: Some(2), rarity: Rarity::Common, ..Default::default() }
}

fn bigfin_bouncer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Bigfin Bouncer".into(), mana_cost: ManaCost::parse("{3}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Pirate, SubType::Shark],
        power: Some(3), toughness: Some(2), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Bigfin Bouncer enters, return up to one target nonland permanent an opponent controls to its owner's hand.",
                vec![Effect::bounce()],
                TargetSpec::PermanentFiltered(Filter::parse("nonland permanent an opponent controls"))),
        ],
        ..Default::default() }
}

fn bishops_soldier(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Bishop's Soldier".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Vampire, SubType::Soldier],
        power: Some(2), toughness: Some(2), keywords: KeywordAbilities::LIFELINK,
        rarity: Rarity::Common, ..Default::default() }
}

fn brazen_scourge(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Brazen Scourge".into(), mana_cost: ManaCost::parse("{1}{R}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Gremlin],
        power: Some(3), toughness: Some(3), keywords: KeywordAbilities::HASTE,
        rarity: Rarity::Uncommon, ..Default::default() }
}

fn burglar_rat(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Burglar Rat".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Rat],
        power: Some(1), toughness: Some(1), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Burglar Rat enters, each opponent discards a card.",
                vec![Effect::Custom("Each opponent discards a card.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn campus_guide(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Campus Guide".into(), mana_cost: ManaCost::parse("{2}"),
        card_types: vec![CardType::Artifact, CardType::Creature], subtypes: vec![SubType::Golem],
        power: Some(2), toughness: Some(1), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Campus Guide enters, you may search your library for a basic land card, reveal it, put it into your hand, then shuffle.",
                vec![Effect::search_library("basic land")],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn diregraf_ghoul(id: ObjectId, owner: PlayerId) -> CardData {
    // Diregraf Ghoul enters the battlefield tapped.
    CardData { id, owner, name: "Diregraf Ghoul".into(), mana_cost: ManaCost::parse("{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Zombie],
        power: Some(2), toughness: Some(2), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                "Diregraf Ghoul enters the battlefield tapped.",
                vec![StaticEffect::Custom("Enters tapped.".into())]),
        ],
        ..Default::default() }
}

fn dragon_trainer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Dragon Trainer".into(), mana_cost: ManaCost::parse("{3}{R}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human],
        power: Some(1), toughness: Some(1), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Dragon Trainer enters, create a 4/4 red Dragon creature token with flying.",
                vec![Effect::create_token("4/4 Dragon with flying", 1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn druid_of_the_cowl(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Druid of the Cowl".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elf, SubType::Druid],
        power: Some(1), toughness: Some(3), rarity: Rarity::Common,
        abilities: vec![
            Ability::mana_ability(id, "{T}: Add {G}.", Mana::green(1)),
        ],
        ..Default::default() }
}

fn erudite_wizard(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Erudite Wizard".into(), mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Wizard],
        power: Some(2), toughness: Some(3), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Erudite Wizard enters, scry 1.",
                vec![Effect::scry(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

pub fn evolving_wilds(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Evolving Wilds".into(), mana_cost: ManaCost::new(),
        card_types: vec![CardType::Land], rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{T}, Sacrifice Evolving Wilds: Search your library for a basic land card, put it onto the battlefield tapped, then shuffle.",
                vec![Cost::tap_self(), Cost::sacrifice_self()],
                vec![Effect::search_library("basic land")],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn felidar_cub(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Felidar Cub".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Cat, SubType::Beast],
        power: Some(2), toughness: Some(2), rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "Sacrifice Felidar Cub: Destroy target enchantment.",
                vec![Cost::sacrifice_self()],
                vec![Effect::destroy()],
                TargetSpec::PermanentFiltered(Filter::parse("enchantment"))),
        ],
        ..Default::default() }
}

fn firebrand_archer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Firebrand Archer".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Archer],
        power: Some(2), toughness: Some(1), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                "Whenever you cast a noncreature spell, Firebrand Archer deals 1 damage to each opponent.",
                vec![EventType::SpellCast],
                vec![Effect::damage_opponents(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn grappling_kraken(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Grappling Kraken".into(), mana_cost: ManaCost::parse("{4}{U}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Kraken],
        power: Some(5), toughness: Some(6), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                "Whenever Grappling Kraken attacks, tap target creature defending player controls. That creature doesn't untap during its controller's next untap step.",
                vec![EventType::AttackerDeclared],
                vec![Effect::Custom("Tap target creature defending player controls. It doesn't untap during its controller's next untap step.".into())],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn healers_hawk(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Healer's Hawk".into(), mana_cost: ManaCost::parse("{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Bird],
        power: Some(1), toughness: Some(1), keywords: KeywordAbilities::FLYING | KeywordAbilities::LIFELINK,
        rarity: Rarity::Common, ..Default::default() }
}

fn hinterland_sanctifier(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Hinterland Sanctifier".into(), mana_cost: ManaCost::parse("{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Cleric, SubType::Rabbit],
        power: Some(1), toughness: Some(2), rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "Protection from multicolored.",
                vec![StaticEffect::Custom("Protection from multicolored.".into())]),
        ],
        ..Default::default() }
}

fn homunculus_horde(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Homunculus Horde".into(), mana_cost: ManaCost::parse("{3}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Homunculus],
        power: Some(2), toughness: Some(2), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Homunculus Horde enters, create a number of 2/2 blue Homunculus creature tokens equal to the number of instant and sorcery cards in your graveyard.",
                vec![Effect::Custom("Create 2/2 Homunculus tokens equal to instants/sorceries in your graveyard.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn infestation_sage(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Infestation Sage".into(), mana_cost: ManaCost::parse("{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elf, SubType::Warlock],
        power: Some(1), toughness: Some(1), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Infestation Sage enters, each player mills two cards.",
                vec![Effect::mill(2)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn kitesail_corsair(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Kitesail Corsair".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Pirate],
        power: Some(2), toughness: Some(1), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common, ..Default::default() }
}

fn leonin_skyhunter(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Leonin Skyhunter".into(), mana_cost: ManaCost::parse("{W}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Cat, SubType::Knight],
        power: Some(2), toughness: Some(2), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common, ..Default::default() }
}

fn llanowar_elves(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Llanowar Elves".into(), mana_cost: ManaCost::parse("{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elf, SubType::Druid],
        power: Some(1), toughness: Some(1), rarity: Rarity::Common,
        abilities: vec![
            Ability::mana_ability(id, "{T}: Add {G}.", Mana::green(1)),
        ],
        ..Default::default() }
}

fn maalfeld_twins(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Maalfeld Twins".into(), mana_cost: ManaCost::parse("{5}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Zombie],
        power: Some(4), toughness: Some(4), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::dies_triggered(id,
                "When Maalfeld Twins dies, create two 2/2 black Zombie creature tokens.",
                vec![Effect::create_token("2/2 Zombie", 2)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn magnigoth_sentry(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Magnigoth Sentry".into(), mana_cost: ManaCost::parse("{3}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Treefolk],
        power: Some(4), toughness: Some(4), keywords: KeywordAbilities::REACH,
        rarity: Rarity::Common, ..Default::default() }
}

fn marauding_blight_priest(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Marauding Blight-Priest".into(), mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Vampire, SubType::Cleric],
        power: Some(3), toughness: Some(2), rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever you gain life, each opponent loses 1 life.",
                vec![EventType::GainLife],
                vec![Effect::Custom("Each opponent loses 1 life.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn pulse_tracker(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Pulse Tracker".into(), mana_cost: ManaCost::parse("{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Vampire, SubType::Rogue],
        power: Some(1), toughness: Some(1), rarity: Rarity::Common,
        abilities: vec![
            Ability::attacks_triggered(id,
                "Whenever Pulse Tracker attacks, each opponent loses 1 life.",
                vec![Effect::Custom("Each opponent loses 1 life.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn raging_redcap(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Raging Redcap".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Knight],
        power: Some(1), toughness: Some(2), keywords: KeywordAbilities::DOUBLE_STRIKE,
        rarity: Rarity::Uncommon, ..Default::default() }
}

fn ravenous_giant(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ravenous Giant".into(), mana_cost: ManaCost::parse("{2}{R}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Giant],
        power: Some(5), toughness: Some(5), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                "At the beginning of your upkeep, Ravenous Giant deals 1 damage to you.",
                vec![EventType::UpkeepStep],
                vec![Effect::deal_damage(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn sanguine_syphoner(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sanguine Syphoner".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Vampire, SubType::Warlock],
        power: Some(1), toughness: Some(3), rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{3}{B}, {T}: Each opponent loses 1 life and you gain 1 life.",
                vec![Cost::Mana(Mana { black: 1, generic: 3, ..Default::default() }), Cost::TapSelf],
                vec![Effect::damage_opponents(1), Effect::gain_life(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn serra_angel(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Serra Angel".into(), mana_cost: ManaCost::parse("{3}{W}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Angel],
        power: Some(4), toughness: Some(4), keywords: KeywordAbilities::FLYING | KeywordAbilities::VIGILANCE,
        rarity: Rarity::Uncommon, ..Default::default() }
}

fn skeleton_archer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Skeleton Archer".into(), mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Skeleton, SubType::Archer],
        power: Some(3), toughness: Some(3), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Skeleton Archer enters, it deals 1 damage to any target.",
                vec![Effect::deal_damage(1)],
                TargetSpec::CreatureOrPlayer),
        ],
        ..Default::default() }
}

fn skyraker_giant(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Skyraker Giant".into(), mana_cost: ManaCost::parse("{2}{R}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Giant],
        power: Some(4), toughness: Some(3), keywords: KeywordAbilities::REACH,
        rarity: Rarity::Uncommon, ..Default::default() }
}

fn storm_fleet_spy(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Storm Fleet Spy".into(), mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Pirate],
        power: Some(2), toughness: Some(2), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Storm Fleet Spy enters, draw a card.",
                vec![Effect::draw_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn surrak_the_hunt_caller(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Surrak, the Hunt Caller".into(), mana_cost: ManaCost::parse("{2}{G}{G}"),
        card_types: vec![CardType::Creature], supertypes: vec![SuperType::Legendary],
        subtypes: vec![SubType::Human, SubType::Warrior],
        power: Some(5), toughness: Some(4), keywords: KeywordAbilities::HASTE,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::triggered(id,
                "Formidable — At the beginning of combat on your turn, if creatures you control have total power 8 or greater, target creature you control gains haste until end of turn.",
                vec![EventType::BeginCombat],
                vec![Effect::gain_keyword_eot("haste")],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn tajuru_pathwarden(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Tajuru Pathwarden".into(), mana_cost: ManaCost::parse("{4}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elf, SubType::Warrior, SubType::Ally],
        power: Some(5), toughness: Some(4), keywords: KeywordAbilities::TRAMPLE | KeywordAbilities::VIGILANCE,
        rarity: Rarity::Common, ..Default::default() }
}

fn thornweald_archer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Thornweald Archer".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elf, SubType::Archer],
        power: Some(2), toughness: Some(1), keywords: KeywordAbilities::DEATHTOUCH | KeywordAbilities::REACH,
        rarity: Rarity::Common, ..Default::default() }
}

fn vampire_spawn(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Vampire Spawn".into(), mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Vampire],
        power: Some(2), toughness: Some(3), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Vampire Spawn enters, each opponent loses 2 life and you gain 2 life.",
                vec![Effect::Custom("Each opponent loses 2 life, you gain 2 life.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn viashino_pyromancer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Viashino Pyromancer".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Lizard, SubType::Wizard],
        power: Some(2), toughness: Some(1), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Viashino Pyromancer enters, it deals 2 damage to target player or planeswalker.",
                vec![Effect::deal_damage(2)],
                TargetSpec::Player),
        ],
        ..Default::default() }
}

fn wary_thespian(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Wary Thespian".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Cat, SubType::Druid],
        power: Some(3), toughness: Some(1), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Wary Thespian enters, mill two cards.",
                vec![Effect::mill(2)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn wildheart_invoker(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Wildheart Invoker".into(), mana_cost: ManaCost::parse("{2}{G}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elf, SubType::Shaman],
        power: Some(4), toughness: Some(3), keywords: KeywordAbilities::TRAMPLE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{8}: Target creature gets +5/+5 and gains trample until end of turn.",
                vec![Cost::pay_mana("{8}")],
                vec![Effect::boost_until_eot(5, 5), Effect::gain_keyword_eot("trample")],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

// ── Remaining Tier 1 creatures ───────────────────────────────────────────────

fn ajanis_pridemate(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Cat Soldier for {1}{W}. (Gain life: +1/+1 counter)
    CardData { id, owner, name: "Ajani's Pridemate".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Cat, SubType::Soldier],
        power: Some(2), toughness: Some(2), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                "Whenever you gain life, put a +1/+1 counter on Ajani's Pridemate.",
                vec![EventType::GainLife],
                vec![Effect::add_p1p1_counters(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn axgard_cavalry(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Dwarf Berserker for {1}{R}. (T: target creature gains haste)
    CardData { id, owner, name: "Axgard Cavalry".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Dwarf, SubType::Berserker],
        power: Some(2), toughness: Some(2), rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{T}: Target creature gains haste until end of turn.",
                vec![Cost::tap_self()],
                vec![Effect::gain_keyword_eot("haste")],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn battle_rattle_shaman(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Goblin Shaman for {3}{R}. (Begin combat: target creature +2/+0)
    CardData { id, owner, name: "Battle-Rattle Shaman".into(), mana_cost: ManaCost::parse("{3}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Shaman],
        power: Some(2), toughness: Some(2), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                "At the beginning of combat on your turn, target creature you control gets +2/+0 until end of turn.",
                vec![EventType::BeginCombat],
                vec![Effect::boost_until_eot(2, 0)],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn crackling_cyclops(id: ObjectId, owner: PlayerId) -> CardData {
    // 0/4 Cyclops Wizard for {2}{R}. (Noncreature spell: +3/+0 until EOT)
    CardData { id, owner, name: "Crackling Cyclops".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Cyclops, SubType::Wizard],
        power: Some(0), toughness: Some(4), rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever you cast a noncreature spell, Crackling Cyclops gets +3/+0 until end of turn.",
                vec![EventType::SpellCast],
                vec![Effect::boost_until_eot(3, 0)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn crusader_of_odric(id: ObjectId, owner: PlayerId) -> CardData {
    // */*, P/T = creatures you control. Human Soldier for {2}{W}.
    CardData { id, owner, name: "Crusader of Odric".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Soldier],
        power: Some(0), toughness: Some(0), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                "Crusader of Odric's power and toughness are each equal to the number of creatures you control.",
                vec![StaticEffect::Custom("P/T = number of creatures you control.".into())]),
        ],
        ..Default::default() }
}

fn dryad_militant(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/1 Dryad Soldier for {G/W}. (Instants/sorceries exiled instead of graveyard)
    CardData { id, owner, name: "Dryad Militant".into(), mana_cost: ManaCost::parse("{G/W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dryad, SubType::Soldier],
        power: Some(2), toughness: Some(1), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                "If an instant or sorcery card would be put into a graveyard from anywhere, exile it instead.",
                vec![StaticEffect::Custom("Instant/sorcery cards are exiled instead of going to graveyard.".into())]),
        ],
        ..Default::default() }
}

fn good_fortune_unicorn(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Unicorn for {1}{G}{W}. (Other creature ETB: +1/+1 counter on it)
    CardData { id, owner, name: "Good-Fortune Unicorn".into(), mana_cost: ManaCost::parse("{1}{G}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Unicorn],
        power: Some(2), toughness: Some(2), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::other_creature_etb_triggered(id,
                "Whenever another creature you control enters, put a +1/+1 counter on that creature.",
                vec![Effect::add_p1p1_counters(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn helpful_hunter(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/1 Cat for {1}{W}. (ETB: draw a card)
    CardData { id, owner, name: "Helpful Hunter".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Cat],
        power: Some(1), toughness: Some(1), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Helpful Hunter enters, draw a card.",
                vec![Effect::draw_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn mild_mannered_librarian(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/1 Human for {G}. ({3}{G}: become Werewolf, +1/+1 counters, draw)
    CardData { id, owner, name: "Mild-Mannered Librarian".into(), mana_cost: ManaCost::parse("{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human],
        power: Some(1), toughness: Some(1), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                "{3}{G}: Put three +1/+1 counters on Mild-Mannered Librarian. It becomes a Werewolf and draws a card. Activate only once.",
                vec![Cost::pay_mana("{3}{G}")],
                vec![Effect::add_p1p1_counters(3), Effect::draw_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn mystic_archaeologist(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/1 Human Wizard for {1}{U}. ({3}{U}{U}: draw 2)
    CardData { id, owner, name: "Mystic Archaeologist".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Wizard],
        power: Some(2), toughness: Some(1), rarity: Rarity::Rare,
        abilities: vec![
            Ability::activated(id,
                "{3}{U}{U}: Draw two cards.",
                vec![Cost::pay_mana("{3}{U}{U}")],
                vec![Effect::draw_cards(2)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn reassembling_skeleton(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/1 Skeleton Warrior for {1}{B}. ({1}{B}: return from graveyard tapped)
    CardData { id, owner, name: "Reassembling Skeleton".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Skeleton, SubType::Warrior],
        power: Some(1), toughness: Some(1), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                "{1}{B}: Return Reassembling Skeleton from your graveyard to the battlefield tapped.",
                vec![Cost::pay_mana("{1}{B}")],
                vec![Effect::return_from_graveyard()],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn spitfire_lagac(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/4 Lizard for {3}{R}. (Landfall: 1 damage to each opponent)
    CardData { id, owner, name: "Spitfire Lagac".into(), mana_cost: ManaCost::parse("{3}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Lizard],
        power: Some(3), toughness: Some(4), rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Landfall — Whenever a land you control enters, Spitfire Lagac deals 1 damage to each opponent.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::damage_opponents(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn tatyova_benthic_druid(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/3 Legendary Merfolk Druid for {3}{G}{U}. (Landfall: gain 1 life, draw a card)
    CardData { id, owner, name: "Tatyova, Benthic Druid".into(), mana_cost: ManaCost::parse("{3}{G}{U}"),
        card_types: vec![CardType::Creature], supertypes: vec![SuperType::Legendary],
        subtypes: vec![SubType::Merfolk, SubType::Druid],
        power: Some(3), toughness: Some(3), rarity: Rarity::Rare,
        abilities: vec![
            Ability::triggered(id,
                "Landfall — Whenever a land you control enters, you gain 1 life and draw a card.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::gain_life(1), Effect::draw_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ── Remaining Tier 1 spells ──────────────────────────────────────────────────

fn banishing_light(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment {2}{W}. ETB: exile nonland permanent until leaves.
    CardData { id, owner, name: "Banishing Light".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Enchantment], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Banishing Light enters, exile target nonland permanent an opponent controls until Banishing Light leaves the battlefield.",
                vec![Effect::exile()],
                TargetSpec::PermanentFiltered(Filter::parse("nonland permanent an opponent controls"))),
        ],
        ..Default::default() }
}

fn burst_lightning(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {R}. Kicker {4}. 2 damage (4 if kicked).
    CardData { id, owner, name: "Burst Lightning".into(), mana_cost: ManaCost::parse("{R}"),
        card_types: vec![CardType::Instant],        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::deal_damage(2)],
                TargetSpec::CreatureOrPlayer),
        ],
        ..Default::default() }
}

fn divine_resilience(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {W}. Kicker {2}{W}. Indestructible until EOT (all creatures if kicked).
    CardData { id, owner, name: "Divine Resilience".into(), mana_cost: ManaCost::parse("{W}"),
        card_types: vec![CardType::Instant],        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::gain_keyword_eot("indestructible")],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn electroduplicate(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {2}{R}. Copy creature with haste + end-step sacrifice. Flashback {2}{R}{R}.
    CardData { id, owner, name: "Electroduplicate".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Create a token that's a copy of target creature you control, except it has haste. Sacrifice it at the beginning of the next end step.".into())],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn grow_from_the_ashes(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {2}{G}. Kicker {2}. Search for basic land (two if kicked).
    CardData { id, owner, name: "Grow from the Ashes".into(), mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Sorcery],        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::search_library("basic land")],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn into_the_roil(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{U}. Kicker {1}{U}. Bounce nonland permanent (draw if kicked).
    CardData { id, owner, name: "Into the Roil".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Instant],        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::bounce()],
                TargetSpec::PermanentFiltered(Filter::parse("nonland permanent"))),
        ],
        ..Default::default() }
}

fn rite_of_replication(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {2}{U}{U}. Kicker {5}. Copy creature (five copies if kicked).
    CardData { id, owner, name: "Rite of Replication".into(), mana_cost: ManaCost::parse("{2}{U}{U}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Rare,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Create a token that's a copy of target creature. If this spell was kicked, create five of those tokens instead.".into())],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn rite_of_the_dragoncaller(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment {4}{R}{R}. (Cast instant/sorcery: create 5/5 Dragon token with flying)
    CardData { id, owner, name: "Rite of the Dragoncaller".into(), mana_cost: ManaCost::parse("{4}{R}{R}"),
        card_types: vec![CardType::Enchantment], rarity: Rarity::Rare,
        abilities: vec![
            Ability::spell_cast_triggered(id,
                "Whenever you cast an instant or sorcery spell, create a 5/5 red Dragon creature token with flying.",
                vec![Effect::create_token("5/5 Dragon with flying", 1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn self_reflection(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {4}{U}{U}. Copy creature you control. Flashback {3}{U}.
    CardData { id, owner, name: "Self-Reflection".into(), mana_cost: ManaCost::parse("{4}{U}{U}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Rare,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Create a token that's a copy of target creature you control.".into())],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn think_twice(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{U}. Draw a card. Flashback {2}{U}.
    CardData { id, owner, name: "Think Twice".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Instant],        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::draw_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ── Remaining Tier 1 artifacts and enchantments ──────────────────────────────

fn basilisk_collar(id: ObjectId, owner: PlayerId) -> CardData {
    // Artifact — Equipment {1}. Equip {2}. Deathtouch + Lifelink.
    CardData { id, owner, name: "Basilisk Collar".into(), mana_cost: ManaCost::parse("{1}"),
        card_types: vec![CardType::Artifact], subtypes: vec![SubType::Equipment],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "Equipped creature has deathtouch and lifelink.",
                vec![StaticEffect::GrantKeyword { filter: "equipped creature".into(), keyword: "deathtouch, lifelink".into() }]),
            Ability::activated(id,
                "Equip {2}",
                vec![Cost::pay_mana("{2}")],
                vec![Effect::equip()],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn feldons_cane(id: ObjectId, owner: PlayerId) -> CardData {
    // Artifact {1}. T, exile: shuffle graveyard into library.
    CardData { id, owner, name: "Feldon's Cane".into(), mana_cost: ManaCost::parse("{1}"),
        card_types: vec![CardType::Artifact], rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{T}, Exile Feldon's Cane: Shuffle your graveyard into your library.",
                vec![Cost::tap_self(), Cost::Custom("Exile Feldon's Cane".into())],
                vec![Effect::Custom("Shuffle your graveyard into your library.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn gilded_lotus(id: ObjectId, owner: PlayerId) -> CardData {
    // Artifact {5}. T: add 3 mana of any one color.
    CardData { id, owner, name: "Gilded Lotus".into(), mana_cost: ManaCost::parse("{5}"),
        card_types: vec![CardType::Artifact], rarity: Rarity::Rare,
        abilities: vec![
            Ability::mana_ability(id, "{T}: Add three mana of any one color.", Mana::colorless(3)),
        ],
        ..Default::default() }
}

fn impact_tremors(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment {1}{R}. (Creature ETB: 1 damage to each opponent)
    CardData { id, owner, name: "Impact Tremors".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Enchantment], rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever a creature you control enters, Impact Tremors deals 1 damage to each opponent.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::damage_opponents(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn omniscience(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment {7}{U}{U}{U}. Cast nonland cards without paying mana costs.
    CardData { id, owner, name: "Omniscience".into(), mana_cost: ManaCost::parse("{7}{U}{U}{U}"),
        card_types: vec![CardType::Enchantment], rarity: Rarity::Mythic,
        abilities: vec![
            Ability::static_ability(id,
                "You may cast spells from your hand without paying their mana costs.",
                vec![StaticEffect::CostReduction { filter: Filter::parse("spells you cast from hand"), amount: 99, condition: None }]),
        ],
        ..Default::default() }
}

fn phyrexian_arena(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment {1}{B}{B}. (Upkeep: draw a card, lose 1 life)
    CardData { id, owner, name: "Phyrexian Arena".into(), mana_cost: ManaCost::parse("{1}{B}{B}"),
        card_types: vec![CardType::Enchantment], rarity: Rarity::Rare,
        abilities: vec![
            Ability::triggered(id,
                "At the beginning of your upkeep, you draw a card and you lose 1 life.",
                vec![EventType::UpkeepStepPre],
                vec![Effect::draw_cards(1), Effect::Custom("You lose 1 life.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn swiftfoot_boots(id: ObjectId, owner: PlayerId) -> CardData {
    // Artifact — Equipment {2}. Equip {1}. Hexproof + Haste.
    CardData { id, owner, name: "Swiftfoot Boots".into(), mana_cost: ManaCost::parse("{2}"),
        card_types: vec![CardType::Artifact], subtypes: vec![SubType::Equipment],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                "Equipped creature has hexproof and haste.",
                vec![StaticEffect::GrantKeyword { filter: "equipped creature".into(), keyword: "hexproof, haste".into() }]),
            Ability::activated(id,
                "Equip {1}",
                vec![Cost::pay_mana("{1}")],
                vec![Effect::equip()],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

// ── Tier 2 vanilla/keyword creatures ─────────────────────────────────────────

fn savannah_lions(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/1 Cat for {W}.
    CardData { id, owner, name: "Savannah Lions".into(), mana_cost: ManaCost::parse("{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Cat],
        power: Some(2), toughness: Some(1), rarity: Rarity::Uncommon, ..Default::default() }
}

fn fire_elemental(id: ObjectId, owner: PlayerId) -> CardData {
    // 5/4 Elemental for {3}{R}{R}.
    CardData { id, owner, name: "Fire Elemental".into(), mana_cost: ManaCost::parse("{3}{R}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elemental],
        power: Some(5), toughness: Some(4), rarity: Rarity::Common, ..Default::default() }
}

fn gigantosaurus(id: ObjectId, owner: PlayerId) -> CardData {
    // 10/10 Dinosaur for {G}{G}{G}{G}{G}.
    CardData { id, owner, name: "Gigantosaurus".into(), mana_cost: ManaCost::parse("{G}{G}{G}{G}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Dinosaur],
        power: Some(10), toughness: Some(10), rarity: Rarity::Rare, ..Default::default() }
}

fn highborn_vampire(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/3 Vampire Warrior for {3}{B}.
    CardData { id, owner, name: "Highborn Vampire".into(), mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Vampire, SubType::Warrior],
        power: Some(4), toughness: Some(3), rarity: Rarity::Common, ..Default::default() }
}

fn swab_goblin(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Goblin Pirate for {1}{R}.
    CardData { id, owner, name: "Swab Goblin".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Pirate],
        power: Some(2), toughness: Some(2), rarity: Rarity::Common, ..Default::default() }
}

fn vampire_interloper(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/1 Vampire Scout for {1}{B}. Flying. Can't block.
    CardData { id, owner, name: "Vampire Interloper".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Vampire, SubType::Scout],
        power: Some(2), toughness: Some(1), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "Vampire Interloper can't block.",
                vec![StaticEffect::CantBlock { filter: "self".into() }]),
        ],
        ..Default::default() }
}

fn juggernaut(id: ObjectId, owner: PlayerId) -> CardData {
    // 5/3 Artifact Creature — Juggernaut for {4}. Attacks each turn if able.
    CardData { id, owner, name: "Juggernaut".into(), mana_cost: ManaCost::parse("{4}"),
        card_types: vec![CardType::Artifact, CardType::Creature],
        subtypes: vec![SubType::Juggernaut],
        power: Some(5), toughness: Some(3), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                "Juggernaut attacks each combat if able.",
                vec![StaticEffect::Custom("Attacks each combat if able.".into())]),
        ],
        ..Default::default() }
}

fn shivan_dragon(id: ObjectId, owner: PlayerId) -> CardData {
    // 5/5 Dragon for {4}{R}{R}. Flying. {R}: +1/+0.
    CardData { id, owner, name: "Shivan Dragon".into(), mana_cost: ManaCost::parse("{4}{R}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Dragon],
        power: Some(5), toughness: Some(5), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::activated(id,
                "{R}: Shivan Dragon gets +1/+0 until end of turn.",
                vec![Cost::pay_mana("{R}")],
                vec![Effect::boost_until_eot(1, 0)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn thrashing_brontodon(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/4 Dinosaur for {1}{G}{G}. {1}, Sacrifice: destroy artifact or enchantment.
    CardData { id, owner, name: "Thrashing Brontodon".into(), mana_cost: ManaCost::parse("{1}{G}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Dinosaur],
        power: Some(3), toughness: Some(4), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                "{1}, Sacrifice Thrashing Brontodon: Destroy target artifact or enchantment.",
                vec![Cost::pay_mana("{1}"), Cost::sacrifice_self()],
                vec![Effect::destroy()],
                TargetSpec::PermanentFiltered(Filter::parse("artifact or enchantment"))),
        ],
        ..Default::default() }
}

fn gnarlback_rhino(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/4 Rhino for {2}{G}{G}. Trample. Heroic: draw a card.
    CardData { id, owner, name: "Gnarlback Rhino".into(), mana_cost: ManaCost::parse("{2}{G}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Rhino],
        power: Some(4), toughness: Some(4), keywords: KeywordAbilities::TRAMPLE,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                "Whenever you cast a spell that targets Gnarlback Rhino, draw a card.",
                vec![EventType::SpellCast],
                vec![Effect::draw_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn eager_trufflesnout(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/2 Boar for {2}{G}. Trample. Combat damage to player: create Food.
    CardData { id, owner, name: "Eager Trufflesnout".into(), mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Boar],
        power: Some(4), toughness: Some(2), keywords: KeywordAbilities::TRAMPLE,
        rarity: Rarity::Uncommon, ..Default::default() }
}

fn tempest_djinn(id: ObjectId, owner: PlayerId) -> CardData {
    // 0/4 Djinn for {U}{U}{U}. Flying. Gets +1/+0 for each basic Island you control.
    CardData { id, owner, name: "Tempest Djinn".into(), mana_cost: ManaCost::parse("{U}{U}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Djinn],
        power: Some(0), toughness: Some(4), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "Tempest Djinn gets +1/+0 for each basic Island you control.",
                vec![StaticEffect::Custom("+1/+0 for each basic Island you control.".into())]),
        ],
        ..Default::default() }
}

fn tolarian_terror(id: ObjectId, owner: PlayerId) -> CardData {
    // 5/5 Serpent for {6}{U}. Costs {1} less per instant/sorcery in graveyard. Ward {2}.
    CardData { id, owner, name: "Tolarian Terror".into(), mana_cost: ManaCost::parse("{6}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Serpent],
        power: Some(5), toughness: Some(5), keywords: KeywordAbilities::WARD,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "This spell costs {1} less to cast for each instant and sorcery card in your graveyard.",
                vec![StaticEffect::CostReduction { filter: Filter::parse("instants/sorceries in graveyard"), amount: 1, condition: None }]),
        ],
        ..Default::default() }
}

fn solemn_simulacrum(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Artifact Creature — Golem for {4}. ETB: search for basic land. Dies: draw.
    CardData { id, owner, name: "Solemn Simulacrum".into(), mana_cost: ManaCost::parse("{4}"),
        card_types: vec![CardType::Artifact, CardType::Creature], subtypes: vec![SubType::Golem],
        power: Some(2), toughness: Some(2), rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Solemn Simulacrum enters, you may search your library for a basic land card, put that card onto the battlefield tapped, then shuffle.",
                vec![Effect::search_library("basic land")],
                TargetSpec::None),
            Ability::dies_triggered(id,
                "When Solemn Simulacrum dies, you may draw a card.",
                vec![Effect::draw_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ── Tier 2 creatures with abilities ──────────────────────────────────────────

fn dazzling_angel(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/3 Angel for {2}{W}. Flying. Other creature ETB: gain 1 life.
    CardData { id, owner, name: "Dazzling Angel".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Angel],
        power: Some(2), toughness: Some(3), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::other_creature_etb_triggered(id,
                "Whenever another creature you control enters, you gain 1 life.",
                vec![Effect::gain_life(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn inspiring_overseer(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/1 Angel Cleric for {2}{W}. Flying. ETB: gain 1 life, draw a card.
    CardData { id, owner, name: "Inspiring Overseer".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Angel, SubType::Cleric],
        power: Some(2), toughness: Some(1), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Inspiring Overseer enters, you gain 1 life and draw a card.",
                vec![Effect::gain_life(1), Effect::draw_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn empyrean_eagle(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/3 Bird Spirit for {1}{W}{U}. Flying. Creatures you control with flying get +1/+1.
    CardData { id, owner, name: "Empyrean Eagle".into(), mana_cost: ManaCost::parse("{1}{W}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Bird, SubType::Spirit],
        power: Some(2), toughness: Some(3), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                "Other creatures you control with flying get +1/+1.",
                vec![StaticEffect::boost_controlled("creatures with flying", 1, 1)]),
        ],
        ..Default::default() }
}

fn brineborn_cutthroat(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/1 Merfolk Pirate for {1}{U}. Flash. Spell on opponent's turn: +1/+1 counter.
    CardData { id, owner, name: "Brineborn Cutthroat".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Merfolk, SubType::Pirate],
        power: Some(2), toughness: Some(1), keywords: KeywordAbilities::FLASH,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell_cast_triggered(id,
                "Whenever you cast a spell during an opponent's turn, put a +1/+1 counter on Brineborn Cutthroat.",
                vec![Effect::add_p1p1_counters(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn exclusion_mage(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Human Wizard for {2}{U}. ETB: bounce opponent's creature.
    CardData { id, owner, name: "Exclusion Mage".into(), mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Wizard],
        power: Some(2), toughness: Some(2), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Exclusion Mage enters, return target creature an opponent controls to its owner's hand.",
                vec![Effect::bounce()],
                TargetSpec::PermanentFiltered(Filter::parse("creature an opponent controls"))),
        ],
        ..Default::default() }
}

fn crow_of_dark_tidings(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/1 Zombie Bird for {2}{B}. Flying. ETB or dies: mill 2.
    CardData { id, owner, name: "Crow of Dark Tidings".into(), mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Zombie, SubType::Bird],
        power: Some(2), toughness: Some(1), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Crow of Dark Tidings enters or dies, mill two cards.",
                vec![Effect::mill(2)],
                TargetSpec::None),
            Ability::dies_triggered(id,
                "When Crow of Dark Tidings dies, mill two cards.",
                vec![Effect::mill(2)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn vampire_neonate(id: ObjectId, owner: PlayerId) -> CardData {
    // 0/3 Vampire for {B}. {2}, T: opponents lose 1 life, you gain 1 life.
    CardData { id, owner, name: "Vampire Neonate".into(), mana_cost: ManaCost::parse("{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Vampire],
        power: Some(0), toughness: Some(3), rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{2}, {T}: Each opponent loses 1 life and you gain 1 life.",
                vec![Cost::pay_mana("{2}"), Cost::tap_self()],
                vec![Effect::Custom("Each opponent loses 1 life, you gain 1 life.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn hungry_ghoul(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Zombie for {1}{B}. {1}, sacrifice creature: +1/+1 counter.
    CardData { id, owner, name: "Hungry Ghoul".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Zombie],
        power: Some(2), toughness: Some(2), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                "{1}, Sacrifice another creature: Put a +1/+1 counter on Hungry Ghoul.",
                vec![Cost::pay_mana("{1}"), Cost::SacrificeOther("another creature".into())],
                vec![Effect::add_p1p1_counters(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn driver_of_the_dead(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/2 Vampire for {3}{B}. Dies: return creature with MV 2 or less from graveyard.
    CardData { id, owner, name: "Driver of the Dead".into(), mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Vampire],
        power: Some(3), toughness: Some(2), rarity: Rarity::Common,
        abilities: vec![
            Ability::dies_triggered(id,
                "When Driver of the Dead dies, return target creature card with mana value 2 or less from your graveyard to the battlefield.",
                vec![Effect::reanimate()],
                TargetSpec::CardInYourGraveyard),
        ],
        ..Default::default() }
}

fn fanatical_firebrand(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/1 Goblin Pirate for {R}. Haste. T, Sac: 1 damage to any target.
    CardData { id, owner, name: "Fanatical Firebrand".into(), mana_cost: ManaCost::parse("{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Pirate],
        power: Some(1), toughness: Some(1), keywords: KeywordAbilities::HASTE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{T}, Sacrifice Fanatical Firebrand: It deals 1 damage to any target.",
                vec![Cost::tap_self(), Cost::sacrifice_self()],
                vec![Effect::deal_damage(1)],
                TargetSpec::CreatureOrPlayer),
        ],
        ..Default::default() }
}

fn goblin_smuggler(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Goblin Rogue for {2}{R}. Haste. T: creature with power <=2 can't be blocked.
    CardData { id, owner, name: "Goblin Smuggler".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Rogue],
        power: Some(2), toughness: Some(2), keywords: KeywordAbilities::HASTE,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                "{T}: Another target creature you control with power 2 or less can't be blocked this turn.",
                vec![Cost::tap_self()],
                vec![Effect::Custom("Target creature with power 2 or less can't be blocked this turn.".into())],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn frenzied_goblin(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/1 Goblin Berserker for {R}. Attacks: pay {R}, creature can't block.
    CardData { id, owner, name: "Frenzied Goblin".into(), mana_cost: ManaCost::parse("{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Berserker],
        power: Some(1), toughness: Some(1), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::attacks_triggered(id,
                "Whenever Frenzied Goblin attacks, you may pay {R}. If you do, target creature can't block this turn.",
                vec![Effect::Custom("Target creature can't block this turn.".into())],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn ghitu_lavarunner(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/2 Human Wizard for {R}. +1/+0 and haste if 2+ instants/sorceries in graveyard.
    CardData { id, owner, name: "Ghitu Lavarunner".into(), mana_cost: ManaCost::parse("{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Wizard],
        power: Some(1), toughness: Some(2), rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "As long as there are two or more instant and/or sorcery cards in your graveyard, Ghitu Lavarunner gets +1/+0 and has haste.",
                vec![StaticEffect::Custom("+1/+0 and haste if 2+ instants/sorceries in graveyard.".into())]),
        ],
        ..Default::default() }
}

fn courageous_goblin(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Goblin for {1}{R}. Attacks with ferocious: +1/+0, menace until EOT.
    CardData { id, owner, name: "Courageous Goblin".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin],
        power: Some(2), toughness: Some(2), rarity: Rarity::Common, ..Default::default() }
}

fn heartfire_immolator(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Human Wizard for {1}{R}. Prowess. {R}, Sac: damage equal to power.
    CardData { id, owner, name: "Heartfire Immolator".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Wizard],
        power: Some(2), toughness: Some(2), keywords: KeywordAbilities::PROWESS,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::activated(id,
                "{R}, Sacrifice Heartfire Immolator: It deals damage equal to its power to target creature or planeswalker.",
                vec![Cost::pay_mana("{R}"), Cost::sacrifice_self()],
                vec![Effect::Custom("Deal damage equal to this creature's power.".into())],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn dragonlords_servant(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/3 Goblin Shaman for {1}{R}. Dragon spells cost {1} less.
    CardData { id, owner, name: "Dragonlord's Servant".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Shaman],
        power: Some(1), toughness: Some(3), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                "Dragon spells you cast cost {1} less to cast.",
                vec![StaticEffect::CostReduction { filter: Filter::parse("Dragon spells"), amount: 1, condition: None }]),
        ],
        ..Default::default() }
}

fn rapacious_dragon(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/3 Dragon for {4}{R}. Flying. ETB: create two Treasure tokens.
    CardData { id, owner, name: "Rapacious Dragon".into(), mana_cost: ManaCost::parse("{4}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Dragon],
        power: Some(3), toughness: Some(3), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Rapacious Dragon enters, create two Treasure tokens.",
                vec![Effect::create_token("Treasure", 2)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn dragon_mage(id: ObjectId, owner: PlayerId) -> CardData {
    // 5/5 Dragon Wizard for {5}{R}{R}. Flying. Combat damage: all discard + draw 7.
    CardData { id, owner, name: "Dragon Mage".into(), mana_cost: ManaCost::parse("{5}{R}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Dragon, SubType::Wizard],
        power: Some(5), toughness: Some(5), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                "Whenever Dragon Mage deals combat damage to a player, each player discards their hand, then draws seven cards.",
                vec![EventType::AttackerDeclared],
                vec![Effect::Custom("Each player discards their hand, then draws seven cards.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn reclamation_sage(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/1 Elf Shaman for {2}{G}. ETB: may destroy artifact or enchantment.
    CardData { id, owner, name: "Reclamation Sage".into(), mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elf, SubType::Shaman],
        power: Some(2), toughness: Some(1), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Reclamation Sage enters, you may destroy target artifact or enchantment.",
                vec![Effect::destroy()],
                TargetSpec::PermanentFiltered(Filter::parse("artifact or enchantment"))),
        ],
        ..Default::default() }
}

fn fierce_empath(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/1 Elf for {2}{G}. ETB: search library for creature with MV 6+.
    CardData { id, owner, name: "Fierce Empath".into(), mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elf],
        power: Some(1), toughness: Some(1), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Fierce Empath enters, you may search your library for a creature card with mana value 6 or greater, reveal it, put it into your hand, then shuffle.",
                vec![Effect::search_library("creature with mana value 6 or greater")],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn beast_kin_ranger(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/3 Elf Ranger for {2}{G}. Trample. Other creature ETB: +1/+0 until EOT.
    CardData { id, owner, name: "Beast-Kin Ranger".into(), mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elf, SubType::Ranger],
        power: Some(3), toughness: Some(3), keywords: KeywordAbilities::TRAMPLE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::other_creature_etb_triggered(id,
                "Whenever another creature you control enters, Beast-Kin Ranger gets +1/+0 until end of turn.",
                vec![Effect::boost_until_eot(1, 0)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn elvish_regrower(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/3 Elf Druid for {2}{G}{G}. ETB: return permanent card from graveyard to hand.
    CardData { id, owner, name: "Elvish Regrower".into(), mana_cost: ManaCost::parse("{2}{G}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elf, SubType::Druid],
        power: Some(4), toughness: Some(3), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Elvish Regrower enters, return target permanent card from your graveyard to your hand.",
                vec![Effect::return_from_graveyard()],
                TargetSpec::CardInYourGraveyard),
        ],
        ..Default::default() }
}

fn springbloom_druid(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/1 Elf Druid for {2}{G}. ETB: sacrifice land, search for 2 basic lands tapped.
    CardData { id, owner, name: "Springbloom Druid".into(), mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elf, SubType::Druid],
        power: Some(1), toughness: Some(1), rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Springbloom Druid enters, you may sacrifice a land. If you do, search your library for up to two basic land cards, put them onto the battlefield tapped, then shuffle.",
                vec![Effect::Custom("Sacrifice a land, search for 2 basic lands tapped.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn mold_adder(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/1 Fungus Snake for {G}. Opponent casts blue/black spell: +1/+1 counter.
    CardData { id, owner, name: "Mold Adder".into(), mana_cost: ManaCost::parse("{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Fungus, SubType::Snake],
        power: Some(1), toughness: Some(1), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                "Whenever an opponent casts a blue or black spell, put a +1/+1 counter on Mold Adder.",
                vec![EventType::SpellCast],
                vec![Effect::add_p1p1_counters(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn leonin_vanguard(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/1 Cat Soldier for {W}. Begin combat, 3+ creatures: +1/+1 and gain 1 life.
    CardData { id, owner, name: "Leonin Vanguard".into(), mana_cost: ManaCost::parse("{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Cat, SubType::Soldier],
        power: Some(1), toughness: Some(1), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                "At the beginning of combat on your turn, if you control three or more creatures, Leonin Vanguard gets +1/+1 until end of turn and you gain 1 life.",
                vec![EventType::BeginCombat],
                vec![Effect::boost_until_eot(1, 1), Effect::gain_life(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn skyship_buccaneer(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/3 Human Pirate for {3}{U}{U}. Flying. Raid: ETB draw a card.
    CardData { id, owner, name: "Skyship Buccaneer".into(), mana_cost: ManaCost::parse("{3}{U}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Pirate],
        power: Some(4), toughness: Some(3), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Skyship Buccaneer enters, if you attacked this turn, draw a card.",
                vec![Effect::draw_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn felidar_savior(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/3 Cat Beast for {3}{W}. Lifelink. ETB: +1/+1 counter on up to 2 other creatures.
    CardData { id, owner, name: "Felidar Savior".into(), mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Cat, SubType::Beast],
        power: Some(2), toughness: Some(3), keywords: KeywordAbilities::LIFELINK,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Felidar Savior enters, put a +1/+1 counter on each of up to two other target creatures you control.",
                vec![Effect::add_p1p1_counters(1)],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn guttersnipe(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Goblin Shaman for {2}{R}. Cast instant/sorcery: 2 damage to each opponent.
    CardData { id, owner, name: "Guttersnipe".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Goblin, SubType::Shaman],
        power: Some(2), toughness: Some(2), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell_cast_triggered(id,
                "Whenever you cast an instant or sorcery spell, Guttersnipe deals 2 damage to each opponent.",
                vec![Effect::damage_opponents(2)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn stromkirk_noble(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/1 Vampire Noble for {R}. Can't be blocked by Humans. Combat damage: +1/+1 counter.
    CardData { id, owner, name: "Stromkirk Noble".into(), mana_cost: ManaCost::parse("{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Vampire, SubType::Noble],
        power: Some(1), toughness: Some(1), rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "Stromkirk Noble can't be blocked by Humans.",
                vec![StaticEffect::Custom("Can't be blocked by Humans.".into())]),
            Ability::triggered(id,
                "Whenever Stromkirk Noble deals combat damage to a player, put a +1/+1 counter on it.",
                vec![EventType::DamagedPlayer],
                vec![Effect::add_p1p1_counters(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn gleaming_barrier(id: ObjectId, owner: PlayerId) -> CardData {
    // 0/4 Artifact Creature — Wall for {2}. Defender. Dies: create Treasure token.
    CardData { id, owner, name: "Gleaming Barrier".into(), mana_cost: ManaCost::parse("{2}"),
        card_types: vec![CardType::Artifact, CardType::Creature], subtypes: vec![SubType::Wall],
        power: Some(0), toughness: Some(4), keywords: KeywordAbilities::DEFENDER,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::dies_triggered(id,
                "When Gleaming Barrier dies, create a Treasure token.",
                vec![Effect::create_token("Treasure", 1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn rune_sealed_wall(id: ObjectId, owner: PlayerId) -> CardData {
    // 0/6 Artifact Creature — Wall for {2}{U}. Defender. T: Surveil 1.
    CardData { id, owner, name: "Rune-Sealed Wall".into(), mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Artifact, CardType::Creature], subtypes: vec![SubType::Wall],
        power: Some(0), toughness: Some(6), keywords: KeywordAbilities::DEFENDER,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{T}: Surveil 1.",
                vec![Cost::TapSelf],
                vec![Effect::scry(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn gateway_sneak(id: ObjectId, owner: PlayerId) -> CardData {
    // 1/3 Vedalken Rogue for {2}{U}. Gate ETB: can't be blocked. Combat damage: draw.
    CardData { id, owner, name: "Gateway Sneak".into(), mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Vedalken, SubType::Rogue],
        power: Some(1), toughness: Some(3), rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                "Whenever a Gate enters under your control, Gateway Sneak can't be blocked this turn.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("Gateway Sneak can't be blocked this turn.".into())],
                TargetSpec::None),
            Ability::triggered(id,
                "Whenever Gateway Sneak deals combat damage to a player, draw a card.",
                vec![EventType::DamagedPlayer],
                vec![Effect::draw_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ── Tier 2 instants ──────────────────────────────────────────────────────────

fn giant_growth(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {G}. Target creature gets +3/+3 until end of turn.
    CardData { id, owner, name: "Giant Growth".into(), mana_cost: ManaCost::parse("{G}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::boost_until_eot(3, 3)],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn unsummon(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {U}. Return target creature to its owner's hand.
    CardData { id, owner, name: "Unsummon".into(), mana_cost: ManaCost::parse("{U}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::bounce()],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn cancel(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{U}{U}. Counter target spell.
    CardData { id, owner, name: "Cancel".into(), mana_cost: ManaCost::parse("{1}{U}{U}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::counter_spell()],
                TargetSpec::Spell),
        ],
        ..Default::default() }
}

fn negate(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{U}. Counter target noncreature spell.
    CardData { id, owner, name: "Negate".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::counter_spell()],
                TargetSpec::Spell),
        ],
        ..Default::default() }
}

fn essence_scatter(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{U}. Counter target creature spell.
    CardData { id, owner, name: "Essence Scatter".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::counter_spell()],
                TargetSpec::Spell),
        ],
        ..Default::default() }
}

fn opt(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {U}. Scry 1. Draw a card.
    CardData { id, owner, name: "Opt".into(), mana_cost: ManaCost::parse("{U}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::scry(1), Effect::draw_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn sure_strike(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{R}. Target creature gets +3/+0 and first strike until EOT.
    CardData { id, owner, name: "Sure Strike".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::boost_until_eot(3, 0), Effect::gain_keyword_eot("first strike")],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn dive_down(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {U}. Target creature you control gets +0/+3 and hexproof until EOT.
    CardData { id, owner, name: "Dive Down".into(), mana_cost: ManaCost::parse("{U}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::boost_until_eot(0, 3), Effect::gain_keyword_eot("hexproof")],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn disenchant(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{W}. Destroy target artifact or enchantment.
    CardData { id, owner, name: "Disenchant".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::destroy()],
                TargetSpec::PermanentFiltered(Filter::parse("artifact or enchantment"))),
        ],
        ..Default::default() }
}

fn kindled_fury(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {R}. Target creature gets +1/+0 and first strike until EOT.
    CardData { id, owner, name: "Kindled Fury".into(), mana_cost: ManaCost::parse("{R}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::boost_until_eot(1, 0), Effect::gain_keyword_eot("first strike")],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn stab(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {B}. Target creature gets -2/-2 until end of turn.
    CardData { id, owner, name: "Stab".into(), mana_cost: ManaCost::parse("{B}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::boost_until_eot(-2, -2)],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn adamant_will(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{W}. Target creature gets +2/+2 and indestructible until EOT.
    CardData { id, owner, name: "Adamant Will".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::boost_until_eot(2, 2), Effect::gain_keyword_eot("indestructible")],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn broken_wings(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {2}{G}. Destroy target artifact, enchantment, or creature with flying.
    CardData { id, owner, name: "Broken Wings".into(), mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::destroy()],
                TargetSpec::PermanentFiltered(Filter::parse("artifact, enchantment, or creature with flying"))),
        ],
        ..Default::default() }
}

fn moment_of_triumph(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {W}. Target creature gets +2/+2 until EOT. You gain 2 life.
    CardData { id, owner, name: "Moment of Triumph".into(), mana_cost: ManaCost::parse("{W}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::boost_until_eot(2, 2), Effect::gain_life(2)],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn moment_of_craving(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{B}. Target creature gets -2/-2 until EOT. You gain 2 life.
    CardData { id, owner, name: "Moment of Craving".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::boost_until_eot(-2, -2), Effect::gain_life(2)],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn quick_study(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {2}{U}. Draw two cards.
    CardData { id, owner, name: "Quick Study".into(), mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::draw_cards(2)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn aetherize(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {3}{U}. Return all attacking creatures to their owners' hands.
    CardData { id, owner, name: "Aetherize".into(), mana_cost: ManaCost::parse("{3}{U}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Return all attacking creatures to their owners' hands.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn scorching_dragonfire(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{R}. 3 damage to creature/planeswalker. If it would die, exile instead.
    CardData { id, owner, name: "Scorching Dragonfire".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::deal_damage(3)],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn bite_down(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{G}. Your creature deals damage equal to its power to creature/planeswalker.
    CardData { id, owner, name: "Bite Down".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::bite()],
                TargetSpec::fight_targets()),
        ],
        ..Default::default() }
}

fn snakeskin_veil(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {G}. +1/+1 counter on creature you control. It gains hexproof until EOT.
    CardData { id, owner, name: "Snakeskin Veil".into(), mana_cost: ManaCost::parse("{G}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::add_p1p1_counters(1), Effect::gain_keyword_eot("hexproof")],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn fleeting_flight(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {W}. +1/+1 counter, flying until EOT, prevent combat damage to it.
    CardData { id, owner, name: "Fleeting Flight".into(), mana_cost: ManaCost::parse("{W}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::add_p1p1_counters(1), Effect::gain_keyword_eot("flying")],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn undying_malice(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {B}. Creature gains "dies: return tapped with +1/+1 counter."
    CardData { id, owner, name: "Undying Malice".into(), mana_cost: ManaCost::parse("{B}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Until end of turn, target creature gains 'When this creature dies, return it to the battlefield tapped under its owner's control with a +1/+1 counter on it.'".into())],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn fake_your_own_death(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{B}. Creature gets +2/+0 and "dies: return tapped + Treasure."
    CardData { id, owner, name: "Fake Your Own Death".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::boost_until_eot(2, 0),
                     Effect::Custom("Until end of turn, target creature gains 'When this creature dies, return it to the battlefield tapped under its owner's control and create a Treasure token.'".into())],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

// ── Tier 2 sorceries ─────────────────────────────────────────────────────────

fn duress(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {B}. Target opponent reveals hand; you choose a noncreature, nonland card to discard.
    CardData { id, owner, name: "Duress".into(), mana_cost: ManaCost::parse("{B}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::discard_cards(1)],
                TargetSpec::Player),
        ],
        ..Default::default() }
}

fn deathmark(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {B}. Destroy target green or white creature.
    CardData { id, owner, name: "Deathmark".into(), mana_cost: ManaCost::parse("{B}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::destroy()],
                TargetSpec::PermanentFiltered(Filter::parse("green or white creature"))),
        ],
        ..Default::default() }
}

fn crash_through(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {R}. Creatures you control gain trample until EOT. Draw a card.
    CardData { id, owner, name: "Crash Through".into(), mana_cost: ManaCost::parse("{R}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Creatures you control gain trample until end of turn.".into()),
                     Effect::draw_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn dragon_fodder(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {1}{R}. Create two 1/1 red Goblin creature tokens.
    CardData { id, owner, name: "Dragon Fodder".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::create_token("1/1 Goblin", 2)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn day_of_judgment(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {2}{W}{W}. Destroy all creatures.
    CardData { id, owner, name: "Day of Judgment".into(), mana_cost: ManaCost::parse("{2}{W}{W}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Rare,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Destroy all creatures.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn angelic_edict(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {4}{W}. Exile target creature or enchantment.
    CardData { id, owner, name: "Angelic Edict".into(), mana_cost: ManaCost::parse("{4}{W}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::exile()],
                TargetSpec::PermanentFiltered(Filter::parse("creature or enchantment"))),
        ],
        ..Default::default() }
}

fn chart_a_course(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {1}{U}. Draw 2. Discard 1 unless you attacked this turn.
    CardData { id, owner, name: "Chart a Course".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::draw_cards(2), Effect::discard_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn macabre_waltz(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {1}{B}. Return up to 2 creature cards from graveyard to hand, discard 1.
    CardData { id, owner, name: "Macabre Waltz".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::return_from_graveyard(), Effect::discard_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn bulk_up(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{R}. Double target creature's power until EOT. Flashback {4}{R}{R}.
    CardData { id, owner, name: "Bulk Up".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Double target creature's power until end of turn.".into())],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

// ── Tier 2 enchantments ──────────────────────────────────────────────────────

fn pacifism(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment — Aura {1}{W}. Enchanted creature can't attack or block.
    CardData { id, owner, name: "Pacifism".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Enchantment], subtypes: vec![SubType::Aura],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                "Enchanted creature can't attack or block.",
                vec![StaticEffect::CantAttack { filter: "enchanted creature".into() },
                     StaticEffect::CantBlock { filter: "enchanted creature".into() }]),
        ],
        ..Default::default() }
}

// ── Tier 3 — modal spells ────────────────────────────────────────────────────

fn abrade(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{R}. Choose one: 3 damage to creature, or destroy artifact.
    CardData { id, owner, name: "Abrade".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Choose one: Deal 3 damage to target creature, or destroy target artifact.".into())],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn boros_charm(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {R}{W}. Choose one: 4 damage to player/PW, permanents indestructible, or double strike.
    CardData { id, owner, name: "Boros Charm".into(), mana_cost: ManaCost::parse("{R}{W}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Choose one: 4 damage to target player/planeswalker; or permanents you control gain indestructible until EOT; or target creature gains double strike until EOT.".into())],
                TargetSpec::CreatureOrPlayer),
        ],
        ..Default::default() }
}

fn slagstorm(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {1}{R}{R}. Choose one: 3 damage to each creature, or 3 damage to each player.
    CardData { id, owner, name: "Slagstorm".into(), mana_cost: ManaCost::parse("{1}{R}{R}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Rare,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Choose one: Slagstorm deals 3 damage to each creature; or Slagstorm deals 3 damage to each player.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn valorous_stance(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {1}{W}. Choose one: creature gains indestructible, or destroy creature with toughness 4+.
    CardData { id, owner, name: "Valorous Stance".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Choose one: Target creature gains indestructible until end of turn; or Destroy target creature with toughness 4 or greater.".into())],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

// ── Tier 3 — pump/anthem spells ──────────────────────────────────────────────

fn heroic_reinforcements(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {2}{R}{W}. Create two 1/1 Soldier tokens. Creatures +1/+1 and haste until EOT.
    CardData { id, owner, name: "Heroic Reinforcements".into(), mana_cost: ManaCost::parse("{2}{R}{W}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::create_token("1/1 Soldier", 2),
                     Effect::Custom("Creatures you control get +1/+1 until end of turn.".into()),
                     Effect::Custom("Creatures you control gain haste until end of turn.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn make_a_stand(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant {2}{W}. Creatures you control +1/+0 and indestructible until EOT.
    CardData { id, owner, name: "Make a Stand".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Creatures you control get +1/+0 until end of turn.".into()),
                     Effect::Custom("Creatures you control gain indestructible until end of turn.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn overrun(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery {2}{G}{G}{G}. Creatures you control +3/+3 and trample until EOT.
    CardData { id, owner, name: "Overrun".into(), mana_cost: ManaCost::parse("{2}{G}{G}{G}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Creatures you control get +3/+3 until end of turn.".into()),
                     Effect::Custom("Creatures you control gain trample until end of turn.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

// ── Tier 3 — lords and tribal creatures ──────────────────────────────────────

fn death_baron(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Zombie Wizard for {1}{B}{B}. Skeletons and other Zombies +1/+1 and deathtouch.
    CardData { id, owner, name: "Death Baron".into(), mana_cost: ManaCost::parse("{1}{B}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Zombie, SubType::Wizard],
        power: Some(2), toughness: Some(2), rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "Skeletons you control and other Zombies you control get +1/+1 and have deathtouch.",
                vec![StaticEffect::Boost { filter: "Skeleton you control".into(), power: 1, toughness: 1 },
                     StaticEffect::GrantKeyword { filter: "Skeleton you control".into(), keyword: "deathtouch".into() },
                     StaticEffect::Boost { filter: "other Zombie you control".into(), power: 1, toughness: 1 },
                     StaticEffect::GrantKeyword { filter: "other Zombie you control".into(), keyword: "deathtouch".into() }]),
        ],
        ..Default::default() }
}

fn elvish_archdruid(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Elf Druid for {1}{G}{G}. Other Elves +1/+1. T: add {G} for each Elf.
    CardData { id, owner, name: "Elvish Archdruid".into(), mana_cost: ManaCost::parse("{1}{G}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elf, SubType::Druid],
        power: Some(2), toughness: Some(2), rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "Other Elf creatures you control get +1/+1.",
                vec![StaticEffect::Boost { filter: "other Elf you control".into(), power: 1, toughness: 1 }]),
            Ability::mana_ability(id,
                "{T}: Add {G} for each Elf you control.",
                Mana::green(1)),
        ],
        ..Default::default() }
}

fn vampire_nighthawk(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/3 Vampire Shaman for {1}{B}{B}. Flying, deathtouch, lifelink.
    CardData { id, owner, name: "Vampire Nighthawk".into(), mana_cost: ManaCost::parse("{1}{B}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Vampire, SubType::Shaman],
        power: Some(2), toughness: Some(3),
        keywords: KeywordAbilities::FLYING | KeywordAbilities::DEATHTOUCH | KeywordAbilities::LIFELINK,
        rarity: Rarity::Uncommon, ..Default::default() }
}

// ── Tier 3 — enchantments ────────────────────────────────────────────────────

fn goblin_oriflamme(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment {1}{R}. Attacking creatures you control get +1/+0.
    CardData { id, owner, name: "Goblin Oriflamme".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Enchantment], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                "Attacking creatures you control get +1/+0.",
                vec![StaticEffect::Boost { filter: "attacking creature you control".into(), power: 1, toughness: 0 }]),
        ],
        ..Default::default() }
}

// ── Tier 3 — creatures with complex abilities ────────────────────────────────

fn fog_bank(id: ObjectId, owner: PlayerId) -> CardData {
    // 0/2 Wall for {1}{U}. Defender, flying. Prevents all combat damage to/from it.
    CardData { id, owner, name: "Fog Bank".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Wall],
        power: Some(0), toughness: Some(2),
        keywords: KeywordAbilities::DEFENDER | KeywordAbilities::FLYING,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                "Prevent all combat damage that would be dealt to and dealt by Fog Bank.",
                vec![StaticEffect::Custom("Prevent all combat damage to and from Fog Bank.".into())]),
        ],
        ..Default::default() }
}

// ── Tier 3 batch 2 — complex card implementations ──────────────────────────

fn abyssal_harvester(id: ObjectId, owner: PlayerId) -> CardData {
    // 3/2 Demon Warlock for {1}{B}{B}. {T}: create token copy of creature card put into graveyard this turn, then sacrifice non-Demon creature tokens.
    CardData { id, owner, name: "Abyssal Harvester".into(), mana_cost: ManaCost::parse("{1}{B}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Demon, SubType::Warlock],
        power: Some(3), toughness: Some(2), rarity: Rarity::Rare,
        abilities: vec![
            Ability::activated(id,
                "{T}: Create a token that's a copy of target creature card in a graveyard that was put there this turn. Then sacrifice each creature token you control that isn't a Demon.",
                vec![Cost::tap_self()],
                vec![Effect::create_token("copy of creature in graveyard", 1)],
                TargetSpec::CardInGraveyard),
        ],
        ..Default::default() }
}

fn adaptive_automaton(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Construct for {3}. Artifact Creature. As ETB: choose creature type; is that type. Other creatures of chosen type get +1/+1.
    CardData { id, owner, name: "Adaptive Automaton".into(), mana_cost: ManaCost::parse("{3}"),
        card_types: vec![CardType::Artifact, CardType::Creature],
        subtypes: vec![SubType::Construct],
        power: Some(2), toughness: Some(2), rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "As Adaptive Automaton enters, choose a creature type. Adaptive Automaton is the chosen type in addition to its other types.",
                vec![Effect::Custom("Choose a creature type. This creature becomes that type.".into())],
                TargetSpec::None),
            Ability::static_ability(id,
                "Other creatures you control of the chosen type get +1/+1.",
                vec![StaticEffect::boost_controlled("other creatures of chosen type", 1, 1)]),
        ],
        ..Default::default() }
}

fn aggressive_mammoth(id: ObjectId, owner: PlayerId) -> CardData {
    // 8/8 Elephant for {3}{G}{G}{G}. Trample. Other creatures you control have trample.
    CardData { id, owner, name: "Aggressive Mammoth".into(), mana_cost: ManaCost::parse("{3}{G}{G}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elephant],
        power: Some(8), toughness: Some(8), keywords: KeywordAbilities::TRAMPLE,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "Other creatures you control have trample.",
                vec![StaticEffect::grant_keyword_controlled("other creatures you control", "trample")]),
        ],
        ..Default::default() }
}

fn alesha_who_laughs_at_fate(id: ObjectId, owner: PlayerId) -> CardData {
    // Legendary 1/4 Human Warrior for {1}{B}{R}. First strike.
    // Whenever attacks: +1/+1 counter. End step (raid): reanimate creature with MV <= its power.
    CardData { id, owner, name: "Alesha, Who Laughs at Fate".into(), mana_cost: ManaCost::parse("{1}{B}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Warrior],
        supertypes: vec![SuperType::Legendary],
        power: Some(1), toughness: Some(4), keywords: KeywordAbilities::FIRST_STRIKE,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::triggered(id,
                "Whenever Alesha attacks, put a +1/+1 counter on it.",
                vec![EventType::AttackerDeclared],
                vec![Effect::add_p1p1_counters(1)],
                TargetSpec::None),
            Ability::triggered(id,
                "Raid — At the beginning of your end step, if you attacked this turn, return target creature card with mana value less than or equal to Alesha's power from your graveyard to the battlefield.",
                vec![EventType::EndStep],
                vec![Effect::reanimate()],
                TargetSpec::CardInYourGraveyard),
        ],
        ..Default::default() }
}

fn angelic_destiny(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment — Aura for {2}{W}{W}. Enchanted creature gets +4/+4, has flying and first strike, is an Angel.
    // When enchanted creature dies, return this to owner's hand.
    CardData { id, owner, name: "Angelic Destiny".into(), mana_cost: ManaCost::parse("{2}{W}{W}"),
        card_types: vec![CardType::Enchantment], subtypes: vec![SubType::Aura],
        rarity: Rarity::Mythic,
        abilities: vec![
            Ability::static_ability(id,
                "Enchanted creature gets +4/+4, has flying and first strike, and is an Angel in addition to its other types.",
                vec![StaticEffect::boost_controlled("enchanted creature", 4, 4),
                     StaticEffect::grant_keyword_controlled("enchanted creature", "flying"),
                     StaticEffect::grant_keyword_controlled("enchanted creature", "first strike")]),
            Ability::triggered(id,
                "When enchanted creature dies, return Angelic Destiny to its owner's hand.",
                vec![EventType::Dies],
                vec![Effect::return_from_graveyard()],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn angel_of_vitality(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Angel for {2}{W}. Flying. If you would gain life, gain that much +1 instead. Gets +2/+2 if you have 25+ life.
    CardData { id, owner, name: "Angel of Vitality".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Angel],
        power: Some(2), toughness: Some(2), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                "If you would gain life, you gain that much life plus 1 instead.",
                vec![StaticEffect::Custom("Life gain replacement: gain +1.".into())]),
            Ability::static_ability(id,
                "Angel of Vitality gets +2/+2 as long as you have 25 or more life.",
                vec![StaticEffect::Custom("Conditional +2/+2 if life >= 25.".into())]),
        ],
        ..Default::default() }
}

fn anthem_of_champions(id: ObjectId, owner: PlayerId) -> CardData {
    // Enchantment for {G}{W}. Creatures you control get +1/+1.
    CardData { id, owner, name: "Anthem of Champions".into(), mana_cost: ManaCost::parse("{G}{W}"),
        card_types: vec![CardType::Enchantment], rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "Creatures you control get +1/+1.",
                vec![StaticEffect::boost_controlled("creatures you control", 1, 1)]),
        ],
        ..Default::default() }
}

fn apothecary_stomper(id: ObjectId, owner: PlayerId) -> CardData {
    // 4/4 Elephant for {4}{G}{G}. Vigilance. ETB: choose one — two +1/+1 counters on target creature; or gain 4 life.
    CardData { id, owner, name: "Apothecary Stomper".into(), mana_cost: ManaCost::parse("{4}{G}{G}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elephant],
        power: Some(4), toughness: Some(4), keywords: KeywordAbilities::VIGILANCE,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, choose one — Put two +1/+1 counters on target creature you control; or You gain 4 life.",
                vec![Effect::Custom("Choose one: two +1/+1 counters on creature or gain 4 life.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn arahbo_the_first_fang(id: ObjectId, owner: PlayerId) -> CardData {
    // Legendary 2/2 Cat Avatar for {2}{W}. Other Cats get +1/+1. ETB or nontoken Cat ETB: create 1/1 Cat token.
    CardData { id, owner, name: "Arahbo, the First Fang".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Cat, SubType::Avatar],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(2), rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "Other Cats you control get +1/+1.",
                vec![StaticEffect::boost_controlled("other Cats you control", 1, 1)]),
            Ability::triggered(id,
                "Whenever Arahbo or another nontoken Cat you control enters, create a 1/1 white Cat creature token.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::create_token("1/1 Cat", 1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn aurelia_the_warleader(id: ObjectId, owner: PlayerId) -> CardData {
    // Legendary 3/4 Angel for {2}{R}{R}{W}{W}. Flying, vigilance, haste.
    // First attack each turn: untap all creatures, additional combat phase.
    CardData { id, owner, name: "Aurelia, the Warleader".into(), mana_cost: ManaCost::parse("{2}{R}{R}{W}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Angel],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(4),
        keywords: KeywordAbilities::FLYING | KeywordAbilities::VIGILANCE | KeywordAbilities::HASTE,
        rarity: Rarity::Mythic,
        abilities: vec![
            Ability::triggered(id,
                "Whenever Aurelia attacks for the first time each turn, untap all creatures you control. After this phase, there is an additional combat phase.",
                vec![EventType::AttackerDeclared],
                vec![Effect::Custom("Untap all creatures you control. Additional combat phase.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn ayli_eternal_pilgrim(id: ObjectId, owner: PlayerId) -> CardData {
    // Legendary 2/3 Kor Cleric for {W}{B}. Deathtouch.
    // {1}, Sacrifice creature: gain life = sacrificed creature's toughness.
    // {1}{W}{B}, Sacrifice creature: exile nonland permanent (if life >= starting+10).
    CardData { id, owner, name: "Ayli, Eternal Pilgrim".into(), mana_cost: ManaCost::parse("{W}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Kor, SubType::Cleric],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(3), keywords: KeywordAbilities::DEATHTOUCH,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::activated(id,
                "{1}, Sacrifice another creature: You gain life equal to the sacrificed creature's toughness.",
                vec![Cost::pay_mana("{1}")],
                vec![Effect::Custom("Sacrifice creature, gain life equal to its toughness.".into())],
                TargetSpec::None),
            Ability::activated(id,
                "{1}{W}{B}, Sacrifice another creature: Exile target nonland permanent. Activate only if you have at least 10 life more than your starting life total.",
                vec![Cost::pay_mana("{1}{W}{B}")],
                vec![Effect::exile()],
                TargetSpec::PermanentFiltered(Filter::parse("nonland permanent"))),
        ],
        ..Default::default() }
}

fn azorius_guildgate(id: ObjectId, owner: PlayerId) -> CardData {
    // Land — Gate. Enters tapped. {T}: Add {W} or {U}.
    CardData { id, owner, name: "Azorius Guildgate".into(),
        card_types: vec![CardType::Land], subtypes: vec![SubType::Gate],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id, "Azorius Guildgate enters tapped.",
                vec![StaticEffect::EntersTapped { filter: "self".into() }]),
            Ability::mana_ability(id, "{T}: Add {W}.", Mana::white(1)),
            Ability::mana_ability(id, "{T}: Add {U}.", Mana::blue(1)),
        ],
        ..Default::default() }
}

fn ball_lightning(id: ObjectId, owner: PlayerId) -> CardData {
    // 6/1 Elemental for {R}{R}{R}. Trample, haste. At beginning of end step, sacrifice it.
    CardData { id, owner, name: "Ball Lightning".into(), mana_cost: ManaCost::parse("{R}{R}{R}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Elemental],
        power: Some(6), toughness: Some(1),
        keywords: KeywordAbilities::TRAMPLE | KeywordAbilities::HASTE,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::triggered(id,
                "At the beginning of the end step, sacrifice Ball Lightning.",
                vec![EventType::EndStep],
                vec![Effect::Sacrifice { filter: Filter::parse("self") }],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn balmor_battlemage_captain(id: ObjectId, owner: PlayerId) -> CardData {
    // Legendary 1/3 Bird Wizard for {U}{R}. Flying.
    // Whenever you cast instant/sorcery, creatures you control get +1/+0 and gain trample until EOT.
    CardData { id, owner, name: "Balmor, Battlemage Captain".into(), mana_cost: ManaCost::parse("{U}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Bird, SubType::Wizard],
        supertypes: vec![SuperType::Legendary],
        power: Some(1), toughness: Some(3), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell_cast_triggered(id,
                "Whenever you cast an instant or sorcery spell, creatures you control get +1/+0 and gain trample until end of turn.",
                vec![Effect::Custom("Creatures you control get +1/+0 and gain trample until end of turn.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn banner_of_kinship(id: ObjectId, owner: PlayerId) -> CardData {
    // Artifact for {5}. As ETB: choose creature type, enter with fellowship counters.
    // Creatures of chosen type get +1/+1 per fellowship counter.
    CardData { id, owner, name: "Banner of Kinship".into(), mana_cost: ManaCost::parse("{5}"),
        card_types: vec![CardType::Artifact], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "As this artifact enters, choose a creature type. This artifact enters with a fellowship counter for each creature you control of the chosen type.",
                vec![Effect::Custom("Choose type, enter with fellowship counters.".into())],
                TargetSpec::None),
            Ability::static_ability(id,
                "Creatures you control of the chosen type get +1/+1 for each fellowship counter on this artifact.",
                vec![StaticEffect::Custom("Chosen type creatures get +1/+1 per fellowship counter.".into())]),
        ],
        ..Default::default() }
}

fn billowing_shriekmass(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/3 Spirit for {3}{B}. Flying. ETB: mill 3. Threshold: +2/+1 if 7+ cards in graveyard.
    CardData { id, owner, name: "Billowing Shriekmass".into(), mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Spirit],
        power: Some(2), toughness: Some(3), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, mill three cards.",
                vec![Effect::mill(3)],
                TargetSpec::None),
            Ability::static_ability(id,
                "Threshold — This creature gets +2/+1 as long as there are seven or more cards in your graveyard.",
                vec![StaticEffect::Custom("Threshold: +2/+1 if graveyard >= 7.".into())]),
        ],
        ..Default::default() }
}

fn fdn_bloodfell_caves(id: ObjectId, owner: PlayerId) -> CardData {
    // Land. Enters tapped. ETB: gain 1 life. {T}: Add {B} or {R}.
    CardData { id, owner, name: "Bloodfell Caves".into(),
        card_types: vec![CardType::Land], rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id, "Bloodfell Caves enters tapped.",
                vec![StaticEffect::EntersTapped { filter: "self".into() }]),
            Ability::enters_battlefield_triggered(id,
                "When Bloodfell Caves enters, you gain 1 life.",
                vec![Effect::gain_life(1)],
                TargetSpec::None),
            Ability::mana_ability(id, "{T}: Add {B}.", Mana::black(1)),
            Ability::mana_ability(id, "{T}: Add {R}.", Mana::red(1)),
        ],
        ..Default::default() }
}

fn bloodthirsty_conqueror(id: ObjectId, owner: PlayerId) -> CardData {
    // 5/5 Vampire Knight for {3}{B}{B}. Flying, deathtouch. Whenever opponent loses life, you gain that much.
    CardData { id, owner, name: "Bloodthirsty Conqueror".into(), mana_cost: ManaCost::parse("{3}{B}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Vampire, SubType::Knight],
        power: Some(5), toughness: Some(5),
        keywords: KeywordAbilities::FLYING | KeywordAbilities::DEATHTOUCH,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::triggered(id,
                "Whenever an opponent loses life, you gain that much life.",
                vec![EventType::LostLife],
                vec![Effect::gain_life(0)],  // dynamic: amount opponent lost
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn fdn_blossoming_sands(id: ObjectId, owner: PlayerId) -> CardData {
    // Land. Enters tapped. ETB: gain 1 life. {T}: Add {G} or {W}.
    CardData { id, owner, name: "Blossoming Sands".into(),
        card_types: vec![CardType::Land], rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id, "Blossoming Sands enters tapped.",
                vec![StaticEffect::EntersTapped { filter: "self".into() }]),
            Ability::enters_battlefield_triggered(id,
                "When Blossoming Sands enters, you gain 1 life.",
                vec![Effect::gain_life(1)],
                TargetSpec::None),
            Ability::mana_ability(id, "{T}: Add {G}.", Mana::green(1)),
            Ability::mana_ability(id, "{T}: Add {W}.", Mana::white(1)),
        ],
        ..Default::default() }
}

fn boros_guildgate(id: ObjectId, owner: PlayerId) -> CardData {
    // Land — Gate. Enters tapped. {T}: Add {R} or {W}.
    CardData { id, owner, name: "Boros Guildgate".into(),
        card_types: vec![CardType::Land], subtypes: vec![SubType::Gate],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id, "Boros Guildgate enters tapped.",
                vec![StaticEffect::EntersTapped { filter: "self".into() }]),
            Ability::mana_ability(id, "{T}: Add {R}.", Mana::red(1)),
            Ability::mana_ability(id, "{T}: Add {W}.", Mana::white(1)),
        ],
        ..Default::default() }
}

fn bushwhack(id: ObjectId, owner: PlayerId) -> CardData {
    // Sorcery for {G}. Choose one: search for basic land; or creature you control fights creature you don't.
    CardData { id, owner, name: "Bushwhack".into(), mana_cost: ManaCost::parse("{G}"),
        card_types: vec![CardType::Sorcery], rarity: Rarity::Uncommon,
        abilities: vec![Ability::spell(id,
            vec![Effect::Custom("Choose one: Search for basic land; or target creature you control fights target creature you don't control.".into())],
            TargetSpec::Custom("various".into()))],
        ..Default::default() }
}

fn carnelian_orb_of_dragonkind(id: ObjectId, owner: PlayerId) -> CardData {
    // Artifact for {2}{R}. {T}: Add {R}. If spent on a Dragon, it gains haste until EOT.
    CardData { id, owner, name: "Carnelian Orb of Dragonkind".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Artifact], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::mana_ability(id, "{T}: Add {R}. If that mana is spent on a Dragon creature spell, it gains haste until end of turn.", Mana::red(1)),
        ],
        ..Default::default() }
}

fn celestial_armor(id: ObjectId, owner: PlayerId) -> CardData {
    // Artifact — Equipment for {2}{W}. Flash. ETB: attach + hexproof/indestructible until EOT.
    // Equipped creature gets +2/+0 and has flying. Equip {3}{W}.
    CardData { id, owner, name: "Celestial Armor".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Artifact], subtypes: vec![SubType::Equipment],
        keywords: KeywordAbilities::FLASH,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When this Equipment enters, attach it to target creature you control. That creature gains hexproof and indestructible until end of turn.",
                vec![Effect::Custom("Attach and grant hexproof + indestructible until EOT.".into())],
                TargetSpec::Creature),
            Ability::static_ability(id,
                "Equipped creature gets +2/+0 and has flying.",
                vec![StaticEffect::boost_controlled("equipped creature", 2, 0),
                     StaticEffect::grant_keyword_controlled("equipped creature", "flying")]),
            Ability::activated(id, "Equip {3}{W}",
                vec![Cost::pay_mana("{3}{W}")],
                vec![Effect::equip()],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn charming_prince(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Human Noble for {1}{W}. ETB: choose one — Scry 2; gain 3 life; exile another creature you own, return at next end step.
    CardData { id, owner, name: "Charming Prince".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Noble],
        power: Some(2), toughness: Some(2), rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Charming Prince enters, choose one — Scry 2; You gain 3 life; Exile another target creature you own, return it at the beginning of the next end step.",
                vec![Effect::Custom("Choose one: Scry 2; gain 3 life; flicker another creature.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn claws_out(id: ObjectId, owner: PlayerId) -> CardData {
    // Instant for {3}{W}{W}. Costs {1} less per Cat you control. Creatures you control get +2/+2 until EOT.
    CardData { id, owner, name: "Claws Out".into(), mana_cost: ManaCost::parse("{3}{W}{W}"),
        card_types: vec![CardType::Instant], rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                "This spell costs {1} less to cast for each Cat you control.",
                vec![StaticEffect::Custom("Cost reduction: {1} less per Cat.".into())]),
            Ability::spell(id,
                vec![Effect::Custom("Creatures you control get +2/+2 until end of turn.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn consuming_aberration(id: ObjectId, owner: PlayerId) -> CardData {
    // 0/0 Horror for {3}{U}{B}. P/T = cards in opponents' graveyards. Whenever you cast a spell, each opponent reveals/mills until a land.
    CardData { id, owner, name: "Consuming Aberration".into(), mana_cost: ManaCost::parse("{3}{U}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Horror],
        power: Some(0), toughness: Some(0), rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "Consuming Aberration's power and toughness are each equal to the number of cards in your opponents' graveyards.",
                vec![StaticEffect::Custom("P/T = cards in opponents' graveyards.".into())]),
            Ability::spell_cast_triggered(id,
                "Whenever you cast a spell, each opponent reveals cards from the top of their library until they reveal a land card, then puts those cards into their graveyard.",
                vec![Effect::Custom("Each opponent mills until land revealed.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn corsair_captain(id: ObjectId, owner: PlayerId) -> CardData {
    // 2/2 Human Pirate for {2}{U}. ETB: create Treasure token. Other Pirates get +1/+1.
    CardData { id, owner, name: "Corsair Captain".into(), mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Human, SubType::Pirate],
        power: Some(2), toughness: Some(2), rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                "When Corsair Captain enters, create a Treasure token.",
                vec![Effect::create_token("Treasure", 1)],
                TargetSpec::None),
            Ability::static_ability(id,
                "Other Pirates you control get +1/+1.",
                vec![StaticEffect::boost_controlled("other Pirates you control", 1, 1)]),
        ],
        ..Default::default() }
}

fn crossway_troublemakers(id: ObjectId, owner: PlayerId) -> CardData {
    // 5/5 Vampire for {5}{B}. Attacking Vampires have deathtouch and lifelink.
    // Whenever a Vampire you control dies, you may pay 2 life; if so, draw a card.
    CardData { id, owner, name: "Crossway Troublemakers".into(), mana_cost: ManaCost::parse("{5}{B}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Vampire],
        power: Some(5), toughness: Some(5), rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                "Attacking Vampires you control have deathtouch and lifelink.",
                vec![StaticEffect::grant_keyword_controlled("attacking Vampires you control", "deathtouch"),
                     StaticEffect::grant_keyword_controlled("attacking Vampires you control", "lifelink")]),
            Ability::triggered(id,
                "Whenever a Vampire you control dies, you may pay 2 life. If you do, draw a card.",
                vec![EventType::Dies],
                vec![Effect::draw_cards(1)],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn crystal_barricade(id: ObjectId, owner: PlayerId) -> CardData {
    // 0/4 Artifact Creature — Wall for {1}{W}. Defender. You have hexproof. Prevent all noncombat damage to other creatures you control.
    CardData { id, owner, name: "Crystal Barricade".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Artifact, CardType::Creature], subtypes: vec![SubType::Wall],
        power: Some(0), toughness: Some(4), keywords: KeywordAbilities::DEFENDER,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id, "You have hexproof.",
                vec![StaticEffect::Custom("You have hexproof.".into())]),
            Ability::static_ability(id,
                "Prevent all noncombat damage that would be dealt to other creatures you control.",
                vec![StaticEffect::Custom("Prevent noncombat damage to other creatures you control.".into())]),
        ],
        ..Default::default() }
}

fn curator_of_destinies(id: ObjectId, owner: PlayerId) -> CardData {
    // 5/5 Sphinx for {4}{U}{U}. Can't be countered. Flying. ETB: fact or fiction (top 5 cards).
    CardData { id, owner, name: "Curator of Destinies".into(), mana_cost: ManaCost::parse("{4}{U}{U}"),
        card_types: vec![CardType::Creature], subtypes: vec![SubType::Sphinx],
        power: Some(5), toughness: Some(5), keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id, "This spell can't be countered.",
                vec![StaticEffect::CantBeCountered]),
            Ability::enters_battlefield_triggered(id,
                "When this creature enters, look at the top five cards of your library and separate them into a face-down pile and a face-up pile. An opponent chooses one. Put that pile into your hand and the other into your graveyard.",
                vec![Effect::Custom("Fact or Fiction: separate top 5, opponent chooses pile for hand or graveyard.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}
fn darksteel_colossus(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Darksteel Colossus".into(),
        mana_cost: ManaCost::parse("{11}"),
        card_types: vec![CardType::Artifact, CardType::Creature],
        subtypes: vec![SubType::Golem],
        power: Some(11), toughness: Some(11),
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::TRAMPLE | KeywordAbilities::INDESTRUCTIBLE,
        ..Default::default() }
}

fn dauntless_veteran(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Dauntless Veteran".into(),
        mana_cost: ManaCost::parse("{1}{W}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Soldier],
        power: Some(2), toughness: Some(2),
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

fn dawnwing_marshal(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Dawnwing Marshal".into(),
        mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Cat, SubType::Soldier],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{4}{W}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn deadly_brew(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Deadly Brew".into(),
        mana_cost: ManaCost::parse("{B}{G}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::Custom("Spell effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn deadly_plot(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Deadly Plot".into(),
        mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::destroy()],
                    TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn demonic_pact(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Demonic Pact".into(),
        mana_cost: ManaCost::parse("{2}{B}{B}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Mythic,
        ..Default::default() }
}

fn desecration_demon(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Desecration Demon".into(),
        mana_cost: ManaCost::parse("{2}{B}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Demon],
        power: Some(6), toughness: Some(6),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLYING,
        ..Default::default() }
}

fn diamond_mare(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Diamond Mare".into(),
        mana_cost: ManaCost::parse("{2}"),
        card_types: vec![CardType::Artifact, CardType::Creature],
        subtypes: vec![SubType::Horse],
        power: Some(1), toughness: Some(3),
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn dimir_guildgate(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Dimir Guildgate".into(),
        card_types: vec![CardType::Land],
        subtypes: vec![SubType::Gate],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn dismal_backwater(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Dismal Backwater".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn doubling_season(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Doubling Season".into(),
        mana_cost: ManaCost::parse("{4}{G}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Mythic,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn drake_hatcher(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Drake Hatcher".into(),
        mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Wizard],
        power: Some(1), toughness: Some(3),
        rarity: Rarity::Rare,
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

fn dread_summons(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Dread Summons".into(),
        mana_cost: ManaCost::parse("{X}{B}{B}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::Custom("Spell effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn dreadwing_scavenger(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Dreadwing Scavenger".into(),
        mana_cost: ManaCost::parse("{1}{U}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Nightmare, SubType::Bird],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::DEATHTOUCH,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn drogskol_reaver(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Drogskol Reaver".into(),
        mana_cost: ManaCost::parse("{5}{W}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Spirit],
        power: Some(3), toughness: Some(5),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::DOUBLE_STRIKE | KeywordAbilities::LIFELINK,
        ..Default::default() }
}

fn dropkick_bomber(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Dropkick Bomber".into(),
        mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Goblin, SubType::Warrior],
        power: Some(2), toughness: Some(3),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{R}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn dwynen_gilt_leaf_daen(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Dwynen, Gilt-Leaf Daen".into(),
        mana_cost: ManaCost::parse("{2}{G}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elf, SubType::Warrior],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(4),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::REACH,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever this attacks, trigger effect.",
                    vec![EventType::AttackerDeclared],
                    vec![Effect::Custom("Attack trigger.".into())],
                    TargetSpec::None),
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn eaten_by_piranhas(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Eaten by Piranhas".into(),
        mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Aura],
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLASH,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn elenda_saint_of_dusk(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Elenda, Saint of Dusk".into(),
        mana_cost: ManaCost::parse("{2}{W}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Vampire, SubType::Knight],
        supertypes: vec![SuperType::Legendary],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::LIFELINK,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn etali_primal_storm(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Etali, Primal Storm".into(),
        mana_cost: ManaCost::parse("{4}{R}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elder, SubType::Dinosaur],
        supertypes: vec![SuperType::Legendary],
        power: Some(6), toughness: Some(6),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever this attacks, trigger effect.",
                    vec![EventType::AttackerDeclared],
                    vec![Effect::Custom("Attack trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn exemplar_of_light(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Exemplar of Light".into(),
        mana_cost: ManaCost::parse("{2}{W}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Angel],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLYING,
        ..Default::default() }
}

fn feed_the_swarm(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Feed the Swarm".into(),
        mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::Custom("Spell effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn felidar_retreat(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Felidar Retreat".into(),
        mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::VIGILANCE,
        ..Default::default() }
}

fn fiendish_panda(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Fiendish Panda".into(),
        mana_cost: ManaCost::parse("{2}{W}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Bear, SubType::Demon],
        power: Some(3), toughness: Some(2),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::dies_triggered(id,
                    "When this dies, trigger effect.",
                    vec![Effect::Custom("Dies effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn finale_of_revelation(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Finale of Revelation".into(),
        mana_cost: ManaCost::parse("{X}{U}{U}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Mythic,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::Custom("Spell effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn fishing_pole(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Fishing Pole".into(),
        mana_cost: ManaCost::parse("{1}"),
        card_types: vec![CardType::Artifact],
        subtypes: vec![SubType::Equipment],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{2}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn flamewake_phoenix(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Flamewake Phoenix".into(),
        mana_cost: ManaCost::parse("{1}{R}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Phoenix],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::HASTE,
        ..Default::default() }
}

fn garruks_uprising(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Garruk's Uprising".into(),
        mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::TRAMPLE,
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

fn gate_colossus(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Gate Colossus".into(),
        mana_cost: ManaCost::parse("{8}"),
        card_types: vec![CardType::Artifact, CardType::Creature],
        subtypes: vec![SubType::Construct],
        power: Some(8), toughness: Some(8),
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn genesis_wave(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Genesis Wave".into(),
        mana_cost: ManaCost::parse("{X}{G}{G}{G}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::Custom("Spell effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn ghalta_primal_hunger(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ghalta, Primal Hunger".into(),
        mana_cost: ManaCost::parse("{10}{G}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elder, SubType::Dinosaur],
        supertypes: vec![SuperType::Legendary],
        power: Some(12), toughness: Some(12),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::TRAMPLE,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn giada_font_of_hope(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Giada, Font of Hope".into(),
        mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Angel],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::VIGILANCE,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn gnarlid_colony(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Gnarlid Colony".into(),
        mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Beast],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::TRAMPLE,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn goblin_surprise(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Goblin Surprise".into(),
        mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::Custom("Spell effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn goldvein_pick(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Goldvein Pick".into(),
        mana_cost: ManaCost::parse("{2}"),
        card_types: vec![CardType::Artifact],
        subtypes: vec![SubType::Equipment],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn golgari_guildgate(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Golgari Guildgate".into(),
        card_types: vec![CardType::Land],
        subtypes: vec![SubType::Gate],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn gratuitous_violence(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Gratuitous Violence".into(),
        mana_cost: ManaCost::parse("{2}{R}{R}{R}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn gruul_guildgate(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Gruul Guildgate".into(),
        card_types: vec![CardType::Land],
        subtypes: vec![SubType::Gate],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn halana_and_alena_partners(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Halana and Alena, Partners".into(),
        mana_cost: ManaCost::parse("{2}{R}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Ranger],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(3),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FIRST_STRIKE | KeywordAbilities::REACH | KeywordAbilities::HASTE,
        ..Default::default() }
}

fn herald_of_eternal_dawn(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Herald of Eternal Dawn".into(),
        mana_cost: ManaCost::parse("{4}{W}{W}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Angel],
        power: Some(6), toughness: Some(6),
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::FLASH | KeywordAbilities::FLYING,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn heraldic_banner(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Heraldic Banner".into(),
        mana_cost: ManaCost::parse("{3}"),
        card_types: vec![CardType::Artifact],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn high_fae_trickster(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "High Fae Trickster".into(),
        mana_cost: ManaCost::parse("{3}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Faerie, SubType::Wizard],
        power: Some(4), toughness: Some(2),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLASH | KeywordAbilities::FLYING,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn high_society_hunter(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "High-Society Hunter".into(),
        mana_cost: ManaCost::parse("{3}{B}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Vampire, SubType::Noble],
        power: Some(5), toughness: Some(3),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever this attacks, trigger effect.",
                    vec![EventType::AttackerDeclared],
                    vec![Effect::Custom("Attack trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn hoarding_dragon(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Hoarding Dragon".into(),
        mana_cost: ManaCost::parse("{3}{R}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
            Ability::dies_triggered(id,
                    "When this dies, trigger effect.",
                    vec![Effect::Custom("Dies effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn immersturm_predator(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Immersturm Predator".into(),
        mana_cost: ManaCost::parse("{2}{B}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Vampire, SubType::Dragon],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::INDESTRUCTIBLE,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever this becomes tapped, trigger effect.",
                    vec![EventType::Tapped],
                    vec![Effect::Custom("Tapped trigger.".into())],
                    TargetSpec::None),
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::tap_self()],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn imperious_perfect(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Imperious Perfect".into(),
        mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elf, SubType::Warrior],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Uncommon,
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

fn infernal_vessel(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Infernal Vessel".into(),
        mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Cleric],
        power: Some(2), toughness: Some(1),
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::dies_triggered(id,
                    "When this dies, trigger effect.",
                    vec![Effect::Custom("Dies effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn ingenious_leonin(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ingenious Leonin".into(),
        mana_cost: ManaCost::parse("{4}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Cat, SubType::Soldier],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FIRST_STRIKE,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{3}{W}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn inspiration_from_beyond(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Inspiration from Beyond".into(),
        mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::Custom("Spell effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn inspiring_call(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Inspiring Call".into(),
        mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::INDESTRUCTIBLE,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::draw_cards(1)],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn inspiring_paladin(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Inspiring Paladin".into(),
        mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Knight],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Common,

        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn izzet_guildgate(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Izzet Guildgate".into(),
        card_types: vec![CardType::Land],
        subtypes: vec![SubType::Gate],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn jazal_goldmane(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Jazal Goldmane".into(),
        mana_cost: ManaCost::parse("{2}{W}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Cat, SubType::Warrior],
        supertypes: vec![SuperType::Legendary],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FIRST_STRIKE,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{3}{W}{W}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn joraga_invocation(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Joraga Invocation".into(),
        mana_cost: ManaCost::parse("{4}{G}{G}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::Custom("Spell effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn jungle_hollow(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Jungle Hollow".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn kalastria_highborn(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Kalastria Highborn".into(),
        mana_cost: ManaCost::parse("{B}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Vampire, SubType::Shaman],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Rare,
        ..Default::default() }
}

fn kellan_planar_trailblazer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Kellan, Planar Trailblazer".into(),
        mana_cost: ManaCost::parse("{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Faerie, SubType::Scout],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(1),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::DOUBLE_STRIKE,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{1}{R}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn knight_of_grace(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Knight of Grace".into(),
        mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature],
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FIRST_STRIKE,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn knight_of_malice(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Knight of Malice".into(),
        mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature],
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FIRST_STRIKE,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn koma_world_eater(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Koma, World-Eater".into(),
        mana_cost: ManaCost::parse("{3}{G}{G}{U}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Serpent],
        supertypes: vec![SuperType::Legendary],
        power: Some(8), toughness: Some(12),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::TRAMPLE,
        ..Default::default() }
}

fn kykar_zephyr_awakener(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Kykar, Zephyr Awakener".into(),
        mana_cost: ManaCost::parse("{2}{W}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Bird, SubType::Wizard],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(4),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::spell_cast_triggered(id,
                    "Whenever you cast a spell, trigger effect.",
                    vec![Effect::Custom("Spell cast trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn lathliss_dragon_queen(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Lathliss, Dragon Queen".into(),
        mana_cost: ManaCost::parse("{4}{R}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon],
        supertypes: vec![SuperType::Legendary],
        power: Some(6), toughness: Some(6),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{1}{R}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn lathril_blade_of_the_elves(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Lathril, Blade of the Elves".into(),
        mana_cost: ManaCost::parse("{2}{B}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elf, SubType::Noble],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(3),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::tap_self()],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn leyline_axe(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Leyline Axe".into(),
        mana_cost: ManaCost::parse("{4}"),
        card_types: vec![CardType::Artifact],
        subtypes: vec![SubType::Equipment],
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::DOUBLE_STRIKE | KeywordAbilities::TRAMPLE,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn lunar_insight(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Lunar Insight".into(),
        mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::draw_cards(1)],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn lyra_dawnbringer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Lyra Dawnbringer".into(),
        card_types: vec![CardType::Creature],
        supertypes: vec![SuperType::Legendary],
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::FIRST_STRIKE | KeywordAbilities::LIFELINK | KeywordAbilities::LIFELINK,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn massacre_wurm(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Massacre Wurm".into(),
        mana_cost: ManaCost::parse("{3}{B}{B}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Phyrexian, SubType::Wurm],
        power: Some(6), toughness: Some(5),
        rarity: Rarity::Mythic,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn mazemind_tome(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Mazemind Tome".into(),
        mana_cost: ManaCost::parse("{2}"),
        card_types: vec![CardType::Artifact],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{2}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn mazes_end(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Maze's End".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Mythic,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{3}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn mindsparker(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Mindsparker".into(),
        mana_cost: ManaCost::parse("{1}{R}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elemental],
        power: Some(3), toughness: Some(2),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FIRST_STRIKE,
        ..Default::default() }
}

fn mossborn_hydra(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Mossborn Hydra".into(),
        mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elemental, SubType::Hydra],
        power: Some(0), toughness: Some(0),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::TRAMPLE,
        ..Default::default() }
}

fn muldrotha_the_gravetide(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Muldrotha, the Gravetide".into(),
        mana_cost: ManaCost::parse("{3}{B}{G}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elemental, SubType::Avatar],
        supertypes: vec![SuperType::Legendary],
        power: Some(6), toughness: Some(6),
        rarity: Rarity::Mythic,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn myojin_of_nights_reach(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Myojin of Night's Reach".into(),
        mana_cost: ManaCost::parse("{5}{B}{B}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Spirit],
        supertypes: vec![SuperType::Legendary],
        power: Some(5), toughness: Some(2),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::INDESTRUCTIBLE,
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

fn new_horizons(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "New Horizons".into(),
        mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Aura],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::Permanent),
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn nine_lives_familiar(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Nine-Lives Familiar".into(),
        mana_cost: ManaCost::parse("{1}{B}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Cat],
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

fn niv_mizzet_visionary(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Niv-Mizzet, Visionary".into(),
        mana_cost: ManaCost::parse("{4}{U}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon, SubType::Wizard],
        supertypes: vec![SuperType::Legendary],
        power: Some(5), toughness: Some(5),
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn nullpriest_of_oblivion(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Nullpriest of Oblivion".into(),
        mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Vampire, SubType::Cleric],
        power: Some(2), toughness: Some(1),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::LIFELINK,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn ordeal_of_nylea(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ordeal of Nylea".into(),
        mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Aura],
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn orzhov_guildgate(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Orzhov Guildgate".into(),
        card_types: vec![CardType::Land],
        subtypes: vec![SubType::Gate],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn ovika_enigma_goliath(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ovika, Enigma Goliath".into(),
        mana_cost: ManaCost::parse("{5}{U}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Phyrexian, SubType::Nightmare],
        supertypes: vec![SuperType::Legendary],
        power: Some(6), toughness: Some(6),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::HASTE,
        abilities: vec![
            Ability::spell_cast_triggered(id,
                    "Whenever you cast a spell, trigger effect.",
                    vec![Effect::Custom("Spell cast trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn pelakka_wurm(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Pelakka Wurm".into(),
        mana_cost: ManaCost::parse("{4}{G}{G}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Wurm],
        power: Some(7), toughness: Some(7),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::TRAMPLE,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
            Ability::dies_triggered(id,
                    "When this dies, trigger effect.",
                    vec![Effect::Custom("Dies effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn perforating_artist(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Perforating Artist".into(),
        mana_cost: ManaCost::parse("{1}{B}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Devil],
        power: Some(3), toughness: Some(2),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::DEATHTOUCH,
        ..Default::default() }
}

fn pirates_cutlass(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Pirate's Cutlass".into(),
        mana_cost: ManaCost::parse("{3}"),
        card_types: vec![CardType::Artifact],
        subtypes: vec![SubType::Equipment],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn predator_ooze(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Predator Ooze".into(),
        mana_cost: ManaCost::parse("{G}{G}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Ooze],
        power: Some(1), toughness: Some(1),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::INDESTRUCTIBLE,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever this attacks, trigger effect.",
                    vec![EventType::AttackerDeclared],
                    vec![Effect::Custom("Attack trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn preposterous_proportions(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Preposterous Proportions".into(),
        mana_cost: ManaCost::parse("{5}{G}{G}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::VIGILANCE,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::Custom("Spell effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn primeval_bounty(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Primeval Bounty".into(),
        mana_cost: ManaCost::parse("{5}{G}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Mythic,
        abilities: vec![
            Ability::spell_cast_triggered(id,
                    "Whenever you cast a spell, trigger effect.",
                    vec![Effect::Custom("Spell cast trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn pyromancers_goggles(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Pyromancer's Goggles".into(),
        mana_cost: ManaCost::parse("{5}"),
        card_types: vec![CardType::Artifact],
        supertypes: vec![SuperType::Legendary],
        rarity: Rarity::Mythic,
        ..Default::default() }
}

fn quilled_greatwurm(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Quilled Greatwurm".into(),
        mana_cost: ManaCost::parse("{4}{G}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Wurm],
        power: Some(7), toughness: Some(7),
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::TRAMPLE,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn rakdos_guildgate(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Rakdos Guildgate".into(),
        card_types: vec![CardType::Land],
        subtypes: vec![SubType::Gate],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn ramos_dragon_engine(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ramos, Dragon Engine".into(),
        mana_cost: ManaCost::parse("{6}"),
        card_types: vec![CardType::Artifact, CardType::Creature],
        subtypes: vec![SubType::Dragon],
        supertypes: vec![SuperType::Legendary],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::spell_cast_triggered(id,
                    "Whenever you cast a spell, trigger effect.",
                    vec![Effect::Custom("Spell cast trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn redcap_gutter_dweller(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Redcap Gutter-Dweller".into(),
        mana_cost: ManaCost::parse("{2}{R}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Goblin, SubType::Warrior],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn regal_caracal(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Regal Caracal".into(),
        mana_cost: ManaCost::parse("{3}{W}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Cat],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::LIFELINK,
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

fn ruby_daring_tracker(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ruby, Daring Tracker".into(),
        mana_cost: ManaCost::parse("{R}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Scout],
        supertypes: vec![SuperType::Legendary],
        power: Some(1), toughness: Some(2),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::HASTE,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever this attacks, trigger effect.",
                    vec![EventType::AttackerDeclared],
                    vec![Effect::Custom("Attack trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn rugged_highlands(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Rugged Highlands".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn scavenging_ooze(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Scavenging Ooze".into(),
        mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Ooze],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{G}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn scoured_barrens(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Scoured Barrens".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn secluded_courtyard(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Secluded Courtyard".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn seekers_folly(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Seeker's Folly".into(),
        mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                    vec![Effect::Custom("Spell effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn selesnya_guildgate(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Selesnya Guildgate".into(),
        card_types: vec![CardType::Land],
        subtypes: vec![SubType::Gate],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn simic_guildgate(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Simic Guildgate".into(),
        card_types: vec![CardType::Land],
        subtypes: vec![SubType::Gate],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn sire_of_seven_deaths(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sire of Seven Deaths".into(),
        mana_cost: ManaCost::parse("{7}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Eldrazi],
        power: Some(7), toughness: Some(7),
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::FIRST_STRIKE | KeywordAbilities::VIGILANCE | KeywordAbilities::TRAMPLE | KeywordAbilities::REACH | KeywordAbilities::LIFELINK,
        ..Default::default() }
}

fn soul_guide_lantern(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Soul-Guide Lantern".into(),
        mana_cost: ManaCost::parse("{1}"),
        card_types: vec![CardType::Artifact],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{1}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn soul_shackled_zombie(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Soul-Shackled Zombie".into(),
        mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Zombie],
        power: Some(4), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn spectral_sailor(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Spectral Sailor".into(),
        mana_cost: ManaCost::parse("{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Spirit, SubType::Pirate],
        power: Some(1), toughness: Some(1),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLASH | KeywordAbilities::FLYING,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{3}{U}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn sphinx_of_forgotten_lore(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sphinx of Forgotten Lore".into(),
        mana_cost: ManaCost::parse("{2}{U}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Sphinx],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::FLASH | KeywordAbilities::FLYING,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever this attacks, trigger effect.",
                    vec![EventType::AttackerDeclared],
                    vec![Effect::Custom("Attack trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn sphinx_of_the_final_word(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sphinx of the Final Word".into(),
        mana_cost: ManaCost::parse("{5}{U}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Sphinx],
        power: Some(5), toughness: Some(5),
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::HEXPROOF,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn starlight_snare(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Starlight Snare".into(),
        mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Aura],
        rarity: Rarity::Common,
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

fn steel_hellkite(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Steel Hellkite".into(),
        mana_cost: ManaCost::parse("{6}"),
        card_types: vec![CardType::Artifact, CardType::Creature],
        subtypes: vec![SubType::Dragon],
        power: Some(5), toughness: Some(5),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{X}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn strix_lookout(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Strix Lookout".into(),
        mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Bird],
        power: Some(1), toughness: Some(2),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::VIGILANCE,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{1}{U}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn sun_blessed_healer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sun-Blessed Healer".into(),
        mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Cleric],
        power: Some(3), toughness: Some(1),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::LIFELINK,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn swiftblade_vindicator(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Swiftblade Vindicator".into(),
        mana_cost: ManaCost::parse("{R}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Soldier],
        power: Some(1), toughness: Some(1),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::DOUBLE_STRIKE | KeywordAbilities::VIGILANCE | KeywordAbilities::TRAMPLE,
        ..Default::default() }
}

fn swiftwater_cliffs(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Swiftwater Cliffs".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn sylvan_scavenging(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sylvan Scavenging".into(),
        mana_cost: ManaCost::parse("{1}{G}{G}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Rare,
        ..Default::default() }
}

fn syr_alin_the_lions_claw(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Syr Alin, the Lion's Claw".into(),
        mana_cost: ManaCost::parse("{3}{W}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Knight],
        supertypes: vec![SuperType::Legendary],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FIRST_STRIKE,
        abilities: vec![
            Ability::triggered(id,
                    "Whenever this attacks, trigger effect.",
                    vec![EventType::AttackerDeclared],
                    vec![Effect::Custom("Attack trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn temple_of_abandon(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Temple of Abandon".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn temple_of_deceit(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Temple of Deceit".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn temple_of_enlightenment(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Temple of Enlightenment".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn temple_of_epiphany(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Temple of Epiphany".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn temple_of_malady(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Temple of Malady".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn temple_of_malice(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Temple of Malice".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn temple_of_mystery(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Temple of Mystery".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn temple_of_plenty(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Temple of Plenty".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn temple_of_silence(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Temple of Silence".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn temple_of_triumph(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Temple of Triumph".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn terror_of_mount_velus(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Terror of Mount Velus".into(),
        mana_cost: ManaCost::parse("{5}{R}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon],
        power: Some(5), toughness: Some(5),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::DOUBLE_STRIKE | KeywordAbilities::DOUBLE_STRIKE,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn thornwood_falls(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Thornwood Falls".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn thousand_year_storm(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Thousand-Year Storm".into(),
        mana_cost: ManaCost::parse("{4}{U}{R}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::spell_cast_triggered(id,
                    "Whenever you cast a spell, trigger effect.",
                    vec![Effect::Custom("Spell cast trigger.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn tinybones_bauble_burglar(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Tinybones, Bauble Burglar".into(),
        mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Skeleton, SubType::Rogue],
        supertypes: vec![SuperType::Legendary],
        power: Some(1), toughness: Some(3),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn tranquil_cove(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Tranquil Cove".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn treetop_snarespinner(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Treetop Snarespinner".into(),
        mana_cost: ManaCost::parse("{3}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Spider],
        power: Some(1), toughness: Some(4),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::REACH | KeywordAbilities::DEATHTOUCH,
        ..Default::default() }
}

fn twinblade_blessing(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Twinblade Blessing".into(),
        mana_cost: ManaCost::parse("{1}{W}{W}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Aura],
        rarity: Rarity::Uncommon,
        keywords: KeywordAbilities::FLASH | KeywordAbilities::DOUBLE_STRIKE,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn twinflame_tyrant(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Twinflame Tyrant".into(),
        mana_cost: ManaCost::parse("{3}{R}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon],
        power: Some(3), toughness: Some(5),
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn valkyries_call(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Valkyrie's Call".into(),
        mana_cost: ManaCost::parse("{3}{W}{W}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Mythic,
        keywords: KeywordAbilities::FLYING,
        ..Default::default() }
}

fn vampire_soulcaller(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Vampire Soulcaller".into(),
        mana_cost: ManaCost::parse("{4}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Vampire, SubType::Warlock],
        power: Some(3), toughness: Some(2),
        rarity: Rarity::Common,
        keywords: KeywordAbilities::FLYING,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn vizier_of_the_menagerie(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Vizier of the Menagerie".into(),
        mana_cost: ManaCost::parse("{3}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Snake, SubType::Cleric],
        power: Some(3), toughness: Some(4),
        rarity: Rarity::Mythic,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn wardens_of_the_cycle(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Wardens of the Cycle".into(),
        mana_cost: ManaCost::parse("{1}{B}{G}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elf, SubType::Warlock],
        power: Some(3), toughness: Some(4),
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn wildborn_preserver(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Wildborn Preserver".into(),
        mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elf, SubType::Archer],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLASH | KeywordAbilities::REACH,
        ..Default::default() }
}

fn wildwood_scourge(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Wildwood Scourge".into(),
        mana_cost: ManaCost::parse("{X}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Hydra],
        power: Some(0), toughness: Some(0),
        rarity: Rarity::Uncommon,
        ..Default::default() }
}

fn wilt_leaf_liege(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Wilt-Leaf Liege".into(),
        mana_cost: ManaCost::parse("{1}{G/W}{G/W}{G/W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elf, SubType::Knight],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::static_ability(id,
                    "Static effect.",
                    vec![StaticEffect::Custom("Static effect.".into())]),
        ],
        ..Default::default() }
}

fn wind_scarred_crag(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Wind-Scarred Crag".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::enters_battlefield_triggered(id,
                    "When this enters, trigger effect.",
                    vec![Effect::Custom("ETB effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}

fn wishclaw_talisman(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Wishclaw Talisman".into(),
        mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Artifact],
        rarity: Rarity::Rare,
        ..Default::default() }
}

fn witness_protection(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Witness Protection".into(),
        mana_cost: ManaCost::parse("{U}"),
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

fn zetalpa_primal_dawn(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Zetalpa, Primal Dawn".into(),
        mana_cost: ManaCost::parse("{6}{W}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elder, SubType::Dinosaur],
        supertypes: vec![SuperType::Legendary],
        power: Some(4), toughness: Some(8),
        rarity: Rarity::Rare,
        keywords: KeywordAbilities::FLYING | KeywordAbilities::DOUBLE_STRIKE | KeywordAbilities::VIGILANCE | KeywordAbilities::TRAMPLE | KeywordAbilities::INDESTRUCTIBLE,
        ..Default::default() }
}

fn zimone_paradox_sculptor(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Zimone, Paradox Sculptor".into(),
        mana_cost: ManaCost::parse("{2}{G}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Wizard],
        supertypes: vec![SuperType::Legendary],
        power: Some(1), toughness: Some(4),
        rarity: Rarity::Mythic,
        abilities: vec![
            Ability::activated(id,
                    "Activated ability.",
                    vec![Cost::pay_mana("{G}{U}")],
                    vec![Effect::Custom("Activated effect.".into())],
                    TargetSpec::None),
        ],
        ..Default::default() }
}



// ── New FDN card factory functions ─────────────────────────────────────

fn adventuring_gear(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Adventuring Gear".into(), mana_cost: ManaCost::parse("{1}"),
        card_types: vec![CardType::Artifact],
        subtypes: vec![SubType::Equipment],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "Equip {1}",
                vec![Cost::pay_mana("{1}")],
                vec![Effect::equip()],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn affectionate_indrik(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Affectionate Indrik".into(), mana_cost: ManaCost::parse("{5}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Beast],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When Affectionate Indrik enters the battlefield, you may have it fight target creature you don't control.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::fight()],
                TargetSpec::OpponentCreature),
        ],
        ..Default::default() }
}

fn ajani_caller_of_the_pride(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ajani, Caller of the Pride".into(), mana_cost: ManaCost::parse("{1}{W}{W}"),
        card_types: vec![CardType::Planeswalker],
        subtypes: vec![SubType::PwAjani],
        supertypes: vec![SuperType::Legendary],
        keywords: KeywordAbilities::FLYING | KeywordAbilities::DOUBLE_STRIKE,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("+1: Put a +1/+1 counter on up to one target creature.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn ambush_wolf(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ambush Wolf".into(), mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Wolf],
        power: Some(4), toughness: Some(2),
        keywords: KeywordAbilities::FLASH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When this creature enters, exile up to one target card from a graveyard.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When this creature enters, exile up to one target card from a graveyard.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn an_offer_you_cant_refuse(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "An Offer You Can't Refuse".into(), mana_cost: ManaCost::parse("{U}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Counter target noncreature spell. Its controller creates two Treasure tokens.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn ancestor_dragon(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ancestor Dragon".into(), mana_cost: ManaCost::parse("{4}{W}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon],
        power: Some(5), toughness: Some(6),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        ..Default::default() }
}

fn angel_of_finality(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Angel of Finality".into(), mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Angel],
        power: Some(3), toughness: Some(4),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When Angel of Finality enters the battlefield, exile all cards from target player's graveyard.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When Angel of Finality enters the battlefield, exile all cards from target player's graveyard.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn arbiter_of_woe(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Arbiter of Woe".into(), mana_cost: ManaCost::parse("{4}{B}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Demon],
        power: Some(5), toughness: Some(4),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When this creature enters, each opponent discards a card and loses 2 life. You draw a card and gain 2 life.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When this creature enters, each opponent discards a card and loses 2 life. You draw a card and gain 2 life.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn arcane_epiphany(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Arcane Epiphany".into(), mana_cost: ManaCost::parse("{3}{U}{U}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("This spell costs {1} less to cast if you control a Wizard.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn arcanis_the_omnipotent(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Arcanis the Omnipotent".into(), mana_cost: ManaCost::parse("{3}{U}{U}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Wizard],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(4),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::activated(id,
                "{2}{U}{U}: Return Arcanis the Omnipotent to its owner's hand.",
                vec![Cost::Custom("{2}{U}{U}: Return Arcanis the Omnipotent to its owner's hand.".into())],
                vec![Effect::Custom("{2}{U}{U}: Return Arcanis the Omnipotent to its owner's hand.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn archmage_of_runes(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Archmage of Runes".into(), mana_cost: ManaCost::parse("{3}{U}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Giant, SubType::Wizard],
        power: Some(3), toughness: Some(6),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Instant and sorcery spells you cast cost {1} less to cast.",
                vec![EventType::SpellCast],
                vec![Effect::Custom("Instant and sorcery spells you cast cost {1} less to cast.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn archway_angel(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Archway Angel".into(), mana_cost: ManaCost::parse("{5}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Angel],
        power: Some(3), toughness: Some(4),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When Archway Angel enters the battlefield, you gain 2 life for each Gate you control.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When Archway Angel enters the battlefield, you gain 2 life for each Gate you control.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn armasaur_guide(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Armasaur Guide".into(), mana_cost: ManaCost::parse("{4}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dinosaur],
        power: Some(4), toughness: Some(4),
        keywords: KeywordAbilities::VIGILANCE,
        rarity: Rarity::Common,
        ..Default::default() }
}

fn ashroot_animist(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ashroot Animist".into(), mana_cost: ManaCost::parse("{2}{R}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Lizard, SubType::Druid],
        power: Some(4), toughness: Some(4),
        keywords: KeywordAbilities::TRAMPLE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When this creature attacks, another target creature you control gains trample and gets +X/+X until end of turn, where X is this creature's power.",
                vec![EventType::AttackerDeclared],
                vec![Effect::Custom("When this creature attacks, another target creature you control gains trample and gets +X/+X until end of turn, where X is this creature's power.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn authority_of_the_consuls(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Authority of the Consuls".into(), mana_cost: ManaCost::parse("{W}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Creatures your opponents control enter the battlefield tapped.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn bake_into_a_pie(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Bake into a Pie".into(), mana_cost: ManaCost::parse("{2}{B}{B}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Destroy target creature. Create a Food token.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn ballyrush_banneret(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ballyrush Banneret".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature],
        power: Some(2), toughness: Some(1),
        rarity: Rarity::Common,
        ..Default::default() }
}

fn battlesong_berserker(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Battlesong Berserker".into(), mana_cost: ManaCost::parse("{3}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Berserker],
        power: Some(3), toughness: Some(4),
        keywords: KeywordAbilities::MENACE,
        rarity: Rarity::Common,
        ..Default::default() }
}

fn biogenic_upgrade(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Biogenic Upgrade".into(), mana_cost: ManaCost::parse("{4}{G}{G}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Distribute three +1/+1 counters among one, two, or three target creatures, then double the number of +1/+1 counters on each of those creatures.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn blanchwood_armor(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Blanchwood Armor".into(), mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Aura],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn blasphemous_edict(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Blasphemous Edict".into(), mana_cost: ManaCost::parse("{3}{B}{B}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("You may pay {B} rather than pay this spell's mana cost if there are thirteen or more creatures on the battlefield.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn bloodtithe_collector(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Bloodtithe Collector".into(), mana_cost: ManaCost::parse("{4}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Vampire, SubType::Noble],
        power: Some(3), toughness: Some(4),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When this creature enters, if an opponent lost life this turn, each opponent discards a card.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When this creature enters, if an opponent lost life this turn, each opponent discards a card.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn bolt_bend(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Bolt Bend".into(), mana_cost: ManaCost::parse("{3}{R}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("This spell costs {3} less to cast if you control a creature with power 4 or greater.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn boltwave(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Boltwave".into(), mana_cost: ManaCost::parse("{R}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Boltwave deals 3 damage to each opponent.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn brasss_bounty(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Brass's Bounty".into(), mana_cost: ManaCost::parse("{6}{R}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("For each land you control, create a colorless Treasure artifact token with \"{T}, Sacrifice this artifact: Add one mana of any color.\"".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn burnished_hart(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Burnished Hart".into(), mana_cost: ManaCost::parse("{3}"),
        card_types: vec![CardType::Artifact, CardType::Creature],
        subtypes: vec![SubType::Elk],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{3}, Sacrifice Burnished Hart: Search your library for up to two basic land cards, put them onto the battlefield tapped, then shuffle your library.",
                vec![Cost::Custom("{3}, Sacrifice Burnished Hart: Search your library for up to two basic land cards, put them onto the battlefield tapped, then shuffle your library.".into())],
                vec![Effect::Custom("{3}, Sacrifice Burnished Hart: Search your library for up to two basic land cards, put them onto the battlefield tapped, then shuffle your library.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn burrog_befuddler(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Burrog Befuddler".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Frog, SubType::Wizard],
        power: Some(2), toughness: Some(1),
        keywords: KeywordAbilities::FLASH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When Burrog Befuddler enters the battlefield, target creature an opponent controls gets -1/-0 until end of turn.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When Burrog Befuddler enters the battlefield, target creature an opponent controls gets -1/-0 until end of turn.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn cackling_prowler(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Cackling Prowler".into(), mana_cost: ManaCost::parse("{3}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Hyena, SubType::Rogue],
        power: Some(4), toughness: Some(3),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id, "Ward {2}",
                vec![StaticEffect::Custom("Ward {2}".into())]),
            Ability::triggered(id,
                "Morbid -- At the beginning of your end step, if a creature died this turn put a +1/+1 counter on this creature.",
                vec![EventType::EndStep],
                vec![Effect::Custom("Morbid -- At the beginning of your end step, if a creature died this turn put a +1/+1 counter on this creature.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn cat_collector(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Cat Collector".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Citizen],
        power: Some(3), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When this creature enters, create a Food token.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When this creature enters, create a Food token.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn cathar_commando(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Cathar Commando".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Soldier],
        power: Some(3), toughness: Some(1),
        keywords: KeywordAbilities::FLASH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{1}, Sacrifice Cathar Commando: Destroy target artifact or enchantment.",
                vec![Cost::Custom("{1}, Sacrifice Cathar Commando: Destroy target artifact or enchantment.".into())],
                vec![Effect::Custom("{1}, Sacrifice Cathar Commando: Destroy target artifact or enchantment.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn cemetery_recruitment(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Cemetery Recruitment".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Return target creature card from your graveyard to your hand. If it's a Zombie card, draw a card.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn cephalid_inkmage(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Cephalid Inkmage".into(), mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Octopus, SubType::Wizard],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When this creature enters, surveil 3.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When this creature enters, surveil 3.".into())],
                TargetSpec::None),
            Ability::static_ability(id, "Threshold -- This creature can't be blocked as long as there are seven or more cards in your graveyard.",
                vec![StaticEffect::Custom("Threshold -- This creature can't be blocked as long as there are seven or more cards in your graveyard.".into())]),
        ],
        ..Default::default() }
}

fn chandra_flameshaper(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Chandra, Flameshaper".into(), mana_cost: ManaCost::parse("{5}{R}{R}"),
        card_types: vec![CardType::Planeswalker],
        subtypes: vec![SubType::PwChandra],
        supertypes: vec![SuperType::Legendary],
        keywords: KeywordAbilities::HASTE,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::triggered(id,
                "+1: Create a token that's a copy of target creature you control, except it has haste and \"At the beginning of the end step, sacrifice this token.\"",
                vec![EventType::EndStep],
                vec![Effect::Custom("+1: Create a token that's a copy of target creature you control, except it has haste and \"At the beginning of the end step, sacrifice this token.\"".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn circuitous_route(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Circuitous Route".into(), mana_cost: ManaCost::parse("{3}{G}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Search your library for up to two basic lands and/or Gates and put them onto the battlefield tapped.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn clinquant_skymage(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Clinquant Skymage".into(), mana_cost: ManaCost::parse("{3}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Bird, SubType::Wizard],
        power: Some(1), toughness: Some(1),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        ..Default::default() }
}

fn cloudblazer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Cloudblazer".into(), mana_cost: ManaCost::parse("{3}{W}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Scout],
        power: Some(2), toughness: Some(2),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When Cloudblazer enters the battlefield, you gain 2 life and draw two cards.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When Cloudblazer enters the battlefield, you gain 2 life and draw two cards.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn confiscate(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Confiscate".into(), mana_cost: ManaCost::parse("{4}{U}{U}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Aura],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Enchant permanent".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn crawling_barrens(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Crawling Barrens".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::mana_ability(id, "{T}: Add {C}.", Mana::colorless(1)),
            Ability::activated(id,
                "{4}: Put two +1/+1 counters on Crawling Barrens. Then you may have it become a 0/0 Elemental creature until end of turn. It's still a land.",
                vec![Cost::Custom("{4}: Put two +1/+1 counters on Crawling Barrens. Then you may have it become a 0/0 Elemental creature until end of turn. It's still a land.".into())],
                vec![Effect::Custom("{4}: Put two +1/+1 counters on Crawling Barrens. Then you may have it become a 0/0 Elemental creature until end of turn. It's still a land.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn crypt_feaster(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Crypt Feaster".into(), mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Zombie],
        power: Some(3), toughness: Some(4),
        keywords: KeywordAbilities::MENACE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Threshold -- Whenever this creature attacks, if there are seven or more cards in your graveyard, this creature gets +2/+0 until end of turn.",
                vec![EventType::AttackerDeclared],
                vec![Effect::Custom("Threshold -- Whenever this creature attacks, if there are seven or more cards in your graveyard, this creature gets +2/+0 until end of turn.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn cryptic_caves(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Cryptic Caves".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::mana_ability(id, "{T}: Add {C}.", Mana::colorless(1)),
        ],
        ..Default::default() }
}

fn cultivators_caravan(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Cultivator's Caravan".into(), mana_cost: ManaCost::parse("{3}"),
        card_types: vec![CardType::Artifact],
        subtypes: vec![SubType::Vehicle],
        power: Some(5), toughness: Some(5),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "Crew 3",
                vec![Cost::Custom("Tap creatures with total power 3+".into())],
                vec![Effect::Custom("Becomes artifact creature until end of turn.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn deadly_riposte(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Deadly Riposte".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Deadly Riposte deals 3 damage to target tapped creature and you gain 2 life.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn demolition_field(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Demolition Field".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::mana_ability(id, "{T}: Add {C}.", Mana::colorless(1)),
            Ability::activated(id,
                "{2}, {T}, Sacrifice Demolition Field: Destroy target nonbasic land an opponent controls.",
                vec![Cost::Custom("{2}, {T}, Sacrifice Demolition Field: Destroy target nonbasic land an opponent controls.".into())],
                vec![Effect::Custom("{2}, {T}, Sacrifice Demolition Field: Destroy target nonbasic land an opponent controls.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn devout_decree(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Devout Decree".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Exile target creature or planeswalker that's black or red. Scry 1.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn dictate_of_kruphix(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Dictate of Kruphix".into(), mana_cost: ManaCost::parse("{1}{U}{U}"),
        card_types: vec![CardType::Enchantment],
        keywords: KeywordAbilities::FLASH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("At the beginning of each player's draw step, that player draws an additional card.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn dragonmaster_outcast(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Dragonmaster Outcast".into(), mana_cost: ManaCost::parse("{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Shaman],
        power: Some(1), toughness: Some(1),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "At the beginning of your upkeep, if you control six or more lands, create a 5/5 red Dragon creature token with flying.",
                vec![EventType::UpkeepStep],
                vec![Effect::Custom("At the beginning of your upkeep, if you control six or more lands, create a 5/5 red Dragon creature token with flying.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn drakuseth_maw_of_flames(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Drakuseth, Maw of Flames".into(), mana_cost: ManaCost::parse("{4}{R}{R}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon],
        supertypes: vec![SuperType::Legendary],
        power: Some(7), toughness: Some(7),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::triggered(id,
                "Whenever Drakuseth, Maw of Flames attacks, it deals 4 damage to any target and 3 damage to each of up to two other targets.",
                vec![EventType::AttackerDeclared],
                vec![Effect::Custom("Whenever Drakuseth, Maw of Flames attacks, it deals 4 damage to any target and 3 damage to each of up to two other targets.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn dwynens_elite(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Dwynen's Elite".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elf, SubType::Warrior],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When this creature enters, if you control another Elf, create a 1/1 green Elf Warrior creature token.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When this creature enters, if you control another Elf, create a 1/1 green Elf Warrior creature token.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn eaten_alive(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Eaten Alive".into(), mana_cost: ManaCost::parse("{B}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("As an additional cost to cast this spell, sacrifice a creature or pay {3}{B}.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn elementalist_adept(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Elementalist Adept".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Wizard],
        power: Some(2), toughness: Some(1),
        keywords: KeywordAbilities::FLASH | KeywordAbilities::PROWESS,
        rarity: Rarity::Common,
        ..Default::default() }
}

fn elfsworn_giant(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Elfsworn Giant".into(), mana_cost: ManaCost::parse("{3}{G}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Giant],
        power: Some(5), toughness: Some(3),
        keywords: KeywordAbilities::REACH,
        rarity: Rarity::Common,
        ..Default::default() }
}

fn elspeths_smite(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Elspeth's Smite".into(), mana_cost: ManaCost::parse("{W}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Elspeth's Smite deals 3 damage to target attacking or blocking creature. If that creature would die this turn, exile it instead.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn enigma_drake(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Enigma Drake".into(), mana_cost: ManaCost::parse("{1}{U}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Drake],
        power: Some(0), toughness: Some(4),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        ..Default::default() }
}

fn expedition_map(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Expedition Map".into(), mana_cost: ManaCost::parse("{1}"),
        card_types: vec![CardType::Artifact],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{2}, {tap}, Sacrifice Expedition Map: Search your library for a land card, reveal it, and put it into your hand. Then shuffle your library.",
                vec![Cost::Custom("{2}, {tap}, Sacrifice Expedition Map: Search your library for a land card, reveal it, and put it into your hand. Then shuffle your library.".into())],
                vec![Effect::Custom("{2}, {tap}, Sacrifice Expedition Map: Search your library for a land card, reveal it, and put it into your hand. Then shuffle your library.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn exsanguinate(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Exsanguinate".into(), mana_cost: ManaCost::parse("{X}{B}{B}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Each opponent loses X life. You gain life equal to the life lost this way.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn extravagant_replication(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Extravagant Replication".into(), mana_cost: ManaCost::parse("{4}{U}{U}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "At the beginning of your upkeep, create a token that's a copy of another target nonland permanent you control.",
                vec![EventType::UpkeepStep],
                vec![Effect::Custom("At the beginning of your upkeep, create a token that's a copy of another target nonland permanent you control.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn faebloom_trick(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Faebloom Trick".into(), mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Create two 1/1 blue Faerie creature tokens with flying. When you do, tap target creature an opponent controls.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn felling_blow(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Felling Blow".into(), mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::add_p1p1_counters(1), Effect::bite()],
                TargetSpec::fight_targets()),
        ],
        ..Default::default() }
}

fn fiery_annihilation(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Fiery Annihilation".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Fiery Annihilation deals 5 damage to target creature. Exile up to one target Equipment attached to that creature. If that creature would die this turn, exile it instead.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn fireshrieker(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Fireshrieker".into(), mana_cost: ManaCost::parse("{3}"),
        card_types: vec![CardType::Artifact],
        subtypes: vec![SubType::Equipment],
        keywords: KeywordAbilities::DOUBLE_STRIKE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "Equip {2}",
                vec![Cost::pay_mana("{2}")],
                vec![Effect::equip()],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn firespitter_whelp(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Firespitter Whelp".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon],
        power: Some(2), toughness: Some(2),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever you cast a noncreature or Dragon spell, Firespitter Whelp deals 1 damage to each opponent.",
                vec![EventType::SpellCast],
                vec![Effect::Custom("Whenever you cast a noncreature or Dragon spell, Firespitter Whelp deals 1 damage to each opponent.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn flashfreeze(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Flashfreeze".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn fleeting_distraction(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Fleeting Distraction".into(), mana_cost: ManaCost::parse("{U}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn fumigate(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Fumigate".into(), mana_cost: ManaCost::parse("{3}{W}{W}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Destroy all creatures. You gain 1 life for each creature destroyed this way.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn fynn_the_fangbearer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Fynn, the Fangbearer".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior],
        supertypes: vec![SuperType::Legendary],
        power: Some(1), toughness: Some(3),
        keywords: KeywordAbilities::DEATHTOUCH,
        rarity: Rarity::Rare,
        ..Default::default() }
}

fn garna_bloodfist_of_keld(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Garna, Bloodfist of Keld".into(), mana_cost: ManaCost::parse("{1}{B}{R}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Berserker],
        supertypes: vec![SuperType::Legendary],
        power: Some(4), toughness: Some(3),
        rarity: Rarity::Rare,
        ..Default::default() }
}

fn gatekeeper_of_malakir(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Gatekeeper of Malakir".into(), mana_cost: ManaCost::parse("{B}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Vampire, SubType::Warrior],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::static_ability(id, "Kicker {B}",
                vec![StaticEffect::Custom("Kicker {B}".into())]),
            Ability::triggered(id,
                "When this creature enters, if it was kicked, target player sacrifices a creature.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When this creature enters, if it was kicked, target player sacrifices a creature.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn giant_cindermaw(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Giant Cindermaw".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dinosaur, SubType::Beast],
        power: Some(4), toughness: Some(3),
        keywords: KeywordAbilities::TRAMPLE,
        rarity: Rarity::Common,
        ..Default::default() }
}

fn goblin_boarders(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Goblin Boarders".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Goblin, SubType::Pirate],
        power: Some(3), toughness: Some(2),
        rarity: Rarity::Common,
        ..Default::default() }
}

fn goblin_firebomb(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Goblin Firebomb".into(), mana_cost: ManaCost::parse("{1}"),
        card_types: vec![CardType::Artifact],
        keywords: KeywordAbilities::FLASH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{7}, {T}, Sacrifice Goblin Firebomb: Destroy target permanent.",
                vec![Cost::Custom("{7}, {T}, Sacrifice Goblin Firebomb: Destroy target permanent.".into())],
                vec![Effect::Custom("{7}, {T}, Sacrifice Goblin Firebomb: Destroy target permanent.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn goblin_negotiation(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Goblin Negotiation".into(), mana_cost: ManaCost::parse("{X}{R}{R}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Goblin Negotiation deals X damage to target creature. Create a number of 1/1 red Goblin creature tokens equal to the amount of excess damage dealt to that creature this way.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn gorehorn_raider(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Gorehorn Raider".into(), mana_cost: ManaCost::parse("{4}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Minotaur, SubType::Pirate],
        power: Some(4), toughness: Some(4),
        rarity: Rarity::Common,
        ..Default::default() }
}

fn guarded_heir(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Guarded Heir".into(), mana_cost: ManaCost::parse("{5}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Noble],
        power: Some(1), toughness: Some(1),
        keywords: KeywordAbilities::LIFELINK,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When this creature enters, create two 3/3 white Knight creature tokens.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When this creature enters, create two 3/3 white Knight creature tokens.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn gutless_plunderer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Gutless Plunderer".into(), mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Skeleton, SubType::Pirate],
        power: Some(2), toughness: Some(2),
        keywords: KeywordAbilities::DEATHTOUCH,
        rarity: Rarity::Common,
        ..Default::default() }
}

fn harbinger_of_the_tides(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Harbinger of the Tides".into(), mana_cost: ManaCost::parse("{U}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Merfolk, SubType::Wizard],
        power: Some(2), toughness: Some(2),
        keywords: KeywordAbilities::FLASH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When Harbinger of the Tides enters the battlefield, you may return target tapped creature an opponent controls to its owner's hand.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When Harbinger of the Tides enters the battlefield, you may return target tapped creature an opponent controls to its owner's hand.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn hare_apparent(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Hare Apparent".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Rabbit, SubType::Noble],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When this creature enters, create a number of 1/1 white Rabbit creature tokens equal to the number of other creatures you control named Hare Apparent.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When this creature enters, create a number of 1/1 white Rabbit creature tokens equal to the number of other creatures you control named Hare Apparent.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn harmless_offering(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Harmless Offering".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Target opponent gains control of target permanent you control.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn hedron_archive(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Hedron Archive".into(), mana_cost: ManaCost::parse("{4}"),
        card_types: vec![CardType::Artifact],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{T}: Add {C}{C}.",
                vec![Cost::Custom("{T}: Add {C}{C}.".into())],
                vec![Effect::Custom("{T}: Add {C}{C}.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn herald_of_faith(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Herald of Faith".into(), mana_cost: ManaCost::parse("{3}{W}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Angel],
        power: Some(4), toughness: Some(3),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When Herald of Faith attacks, you gain 2 life.",
                vec![EventType::AttackerDeclared],
                vec![Effect::Custom("When Herald of Faith attacks, you gain 2 life.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn heros_downfall(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Hero's Downfall".into(), mana_cost: ManaCost::parse("{1}{B}{B}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Destroy target creature or planeswalker.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn heroes_bane(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Heroes' Bane".into(), mana_cost: ManaCost::parse("{3}{G}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Hydra],
        power: Some(0), toughness: Some(0),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{2}{G}{G}: Put X +1/+1 counters on Heroe's Bane, where X is its power.",
                vec![Cost::Custom("{2}{G}{G}: Put X +1/+1 counters on Heroe's Bane, where X is its power.".into())],
                vec![Effect::Custom("{2}{G}{G}: Put X +1/+1 counters on Heroe's Bane, where X is its power.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn hidetsugus_second_rite(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Hidetsugu's Second Rite".into(), mana_cost: ManaCost::parse("{3}{R}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("If target player has exactly 10 life, Hidetsugu's Second Rite deals 10 damage to that player.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn icewind_elemental(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Icewind Elemental".into(), mana_cost: ManaCost::parse("{4}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Elemental],
        power: Some(3), toughness: Some(4),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When this creature enters, draw a card, then discard a card.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When this creature enters, draw a card, then discard a card.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn imprisoned_in_the_moon(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Imprisoned in the Moon".into(), mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Aura],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Enchant creature, land, or planeswalker".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn incinerating_blast(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Incinerating Blast".into(), mana_cost: ManaCost::parse("{4}{R}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Incinerating Blast deals 6 damage to target creature.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn involuntary_employment(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Involuntary Employment".into(), mana_cost: ManaCost::parse("{3}{R}"),
        card_types: vec![CardType::Sorcery],
        keywords: KeywordAbilities::HASTE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Gain control of target creature until end of turn. Untap that creature. It gains haste until end of turn. Create a Treasure token.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn joust_through(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Joust Through".into(), mana_cost: ManaCost::parse("{W}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Joust Through deals 3 damage to target attacking or blocking creature. You gain 1 life.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn kaito_cunning_infiltrator(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Kaito, Cunning Infiltrator".into(), mana_cost: ManaCost::parse("{1}{U}{U}"),
        card_types: vec![CardType::Planeswalker],
        subtypes: vec![SubType::PwKaito],
        supertypes: vec![SuperType::Legendary],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Whenever a creature you control deals combat damage to a player, put a loyalty counter on Kaito.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn kargan_dragonrider(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Kargan Dragonrider".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Warrior],
        power: Some(2), toughness: Some(2),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        ..Default::default() }
}

fn kiora_the_rising_tide(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Kiora, the Rising Tide".into(), mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Merfolk, SubType::Noble],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(2),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::triggered(id,
                "When Kiora enters, draw two cards, then discard two cards.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When Kiora enters, draw two cards, then discard two cards.".into())],
                TargetSpec::None),
            Ability::triggered(id,
                "Threshold -- Whenever Kiora attacks, if there are seven or more cards in your graveyard, you may create Scion of the Deep, a legendary 8/8 blue Octopus creature token.",
                vec![EventType::AttackerDeclared],
                vec![Effect::Custom("Threshold -- Whenever Kiora attacks, if there are seven or more cards in your graveyard, you may create Scion of the Deep, a legendary 8/8 blue Octopus creature token.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn krenko_mob_boss(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Krenko, Mob Boss".into(), mana_cost: ManaCost::parse("{2}{R}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Goblin, SubType::Warrior],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::activated(id,
                "{T}: Create X 1/1 red Goblin creature tokens, where X is the number of Goblins you control.",
                vec![Cost::Custom("{T}: Create X 1/1 red Goblin creature tokens, where X is the number of Goblins you control.".into())],
                vec![Effect::Custom("{T}: Create X 1/1 red Goblin creature tokens, where X is the number of Goblins you control.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn lightshell_duo(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Lightshell Duo".into(), mana_cost: ManaCost::parse("{3}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Rat, SubType::Otter],
        power: Some(3), toughness: Some(4),
        keywords: KeywordAbilities::PROWESS,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When Lightshell Duo enters, surveil 2.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When Lightshell Duo enters, surveil 2.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn liliana_dreadhorde_general(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Liliana, Dreadhorde General".into(), mana_cost: ManaCost::parse("{4}{B}{B}"),
        card_types: vec![CardType::Planeswalker],
        subtypes: vec![SubType::PwLiliana],
        supertypes: vec![SuperType::Legendary],
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Whenever a creature you control dies, draw a card.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn linden_the_steadfast_queen(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Linden, the Steadfast Queen".into(), mana_cost: ManaCost::parse("{W}{W}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Noble],
        supertypes: vec![SuperType::Legendary],
        power: Some(3), toughness: Some(3),
        keywords: KeywordAbilities::VIGILANCE,
        rarity: Rarity::Rare,
        ..Default::default() }
}

fn loot_exuberant_explorer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Loot, Exuberant Explorer".into(), mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Beast, SubType::Noble],
        supertypes: vec![SuperType::Legendary],
        power: Some(1), toughness: Some(4),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::activated(id,
                "{4}{G}{G}, {T}: Look at the top six cards of your library. You may reveal a creature card with mana value less than or equal to the number of lands you control from among them and put it onto the batt",
                vec![Cost::Custom("{4}{G}{G}, {T}: Look at the top six cards of your library. You may reveal a creature card with mana value less than or equal to the number of lands you control from among them and put it onto the batt".into())],
                vec![Effect::Custom("{4}{G}{G}, {T}: Look at the top six cards of your library. You may reveal a creature card with mana value less than or equal to the number of lands you control from among them and put it onto the batt".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn luminous_rebuke(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Luminous Rebuke".into(), mana_cost: ManaCost::parse("{4}{W}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("This spell costs {3} less to cast if it targets a tapped creature.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn maelstrom_pulse(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Maelstrom Pulse".into(), mana_cost: ManaCost::parse("{1}{B}{G}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Destroy target nonland permanent and all other permanents with the same name as that permanent.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn make_your_move(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Make Your Move".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Destroy target artifact, enchantment, or creature with power 4 or greater.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn mentor_of_the_meek(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Mentor of the Meek".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Soldier],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Common,
        ..Default::default() }
}

fn meteor_golem(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Meteor Golem".into(), mana_cost: ManaCost::parse("{7}"),
        card_types: vec![CardType::Artifact, CardType::Creature],
        subtypes: vec![SubType::Golem],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When Meteor Golem enters the battlefield, destroy target nonland permanent an opponent controls.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When Meteor Golem enters the battlefield, destroy target nonland permanent an opponent controls.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn micromancer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Micromancer".into(), mana_cost: ManaCost::parse("{3}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Wizard],
        power: Some(3), toughness: Some(3),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When Micromancer enters the battlefield, you may search your library for an instant or sorcery card with mana value 1, reveal it, put it into your hand, then shuffle.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When Micromancer enters the battlefield, you may search your library for an instant or sorcery card with mana value 1, reveal it, put it into your hand, then shuffle.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn midnight_reaper(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Midnight Reaper".into(), mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Zombie, SubType::Knight],
        power: Some(3), toughness: Some(2),
        rarity: Rarity::Common,
        ..Default::default() }
}

fn midnight_snack(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Midnight Snack".into(), mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Raid -- At the beginning of your end step, if you attacked this turn, create a Food token.",
                vec![EventType::EndStep],
                vec![Effect::Custom("Raid -- At the beginning of your end step, if you attacked this turn, create a Food token.".into())],
                TargetSpec::None),
            Ability::activated(id,
                "{2}{B}, Sacrifice this enchantment: Target opponent loses X life, where X is the amount of life you gained this turn.",
                vec![Cost::Custom("{2}{B}, Sacrifice this enchantment: Target opponent loses X life, where X is the amount of life you gained this turn.".into())],
                vec![Effect::Custom("{2}{B}, Sacrifice this enchantment: Target opponent loses X life, where X is the amount of life you gained this turn.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn mischievous_mystic(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Mischievous Mystic".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Wizard],
        power: Some(2), toughness: Some(1),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        ..Default::default() }
}

fn mischievous_pup(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Mischievous Pup".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dog],
        power: Some(3), toughness: Some(1),
        keywords: KeywordAbilities::FLASH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When Mischievous Pup enters the battlefield, return up to one other target permanent you control to its owner's hand.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When Mischievous Pup enters the battlefield, return up to one other target permanent you control to its owner's hand.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn mocking_sprite(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Mocking Sprite".into(), mana_cost: ManaCost::parse("{2}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Faerie, SubType::Rogue],
        power: Some(2), toughness: Some(1),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        ..Default::default() }
}

fn mortify(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Mortify".into(), mana_cost: ManaCost::parse("{1}{W}{B}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn mystical_teachings(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Mystical Teachings".into(), mana_cost: ManaCost::parse("{3}{U}"),
        card_types: vec![CardType::Instant],
        keywords: KeywordAbilities::FLASH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "Flashback {5}{B}",
                vec![Cost::pay_mana("{5}{B}")],
                vec![Effect::Custom("Cast from graveyard, then exile.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn needletooth_pack(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Needletooth Pack".into(), mana_cost: ManaCost::parse("{3}{G}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dinosaur],
        power: Some(4), toughness: Some(5),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Morbid -- At the beginning of your end step, if a creature died this turn, put two +1/+1 counters on target creature you control.",
                vec![EventType::EndStep],
                vec![Effect::Custom("Morbid -- At the beginning of your end step, if a creature died this turn, put two +1/+1 counters on target creature you control.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn nessian_hornbeetle(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Nessian Hornbeetle".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Insect],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Common,
        ..Default::default() }
}

fn obliterating_bolt(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Obliterating Bolt".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Obliterating Bolt deals 4 damage to target creature or planeswalker. If that creature or planeswalker would die this turn, exile it instead.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn offer_immortality(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Offer Immortality".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Instant],
        keywords: KeywordAbilities::DEATHTOUCH | KeywordAbilities::INDESTRUCTIBLE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Target creature gains deathtouch and indestructible until end of turn.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn painful_quandary(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Painful Quandary".into(), mana_cost: ManaCost::parse("{3}{B}{B}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Whenever an opponent casts a spell, that player loses 5 life unless they discard a card.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn pilfer(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Pilfer".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Target opponent reveals their hand. You choose a nonland card from it. That player discards that card.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn prayer_of_binding(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Prayer of Binding".into(), mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Enchantment],
        keywords: KeywordAbilities::FLASH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When Prayer of Binding enters the battlefield, exile up to one target nonland permanent an opponent controls until Prayer of Binding leaves the battlefield. You gain 2 life.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When Prayer of Binding enters the battlefield, exile up to one target nonland permanent an opponent controls until Prayer of Binding leaves the battlefield. You gain 2 life.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn prideful_parent(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Prideful Parent".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Cat],
        power: Some(2), toughness: Some(2),
        keywords: KeywordAbilities::VIGILANCE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When this creature enters, create a 1/1 white Cat creature token.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When this creature enters, create a 1/1 white Cat creature token.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn primal_might(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Primal Might".into(), mana_cost: ManaCost::parse("{X}{G}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Target creature you control gets +X/+X until end of turn. Then it fights up to one target creature you don’t control.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn prime_speaker_zegana(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Prime Speaker Zegana".into(), mana_cost: ManaCost::parse("{2}{G}{G}{U}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Merfolk, SubType::Wizard],
        supertypes: vec![SuperType::Legendary],
        power: Some(1), toughness: Some(1),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::triggered(id,
                "When Prime Speaker Zegana enters the battlefield, draw cards equal to its power.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When Prime Speaker Zegana enters the battlefield, draw cards equal to its power.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn progenitus(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Progenitus".into(), mana_cost: ManaCost::parse("{W}{W}{U}{U}{B}{B}{R}{R}{G}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Hydra, SubType::Avatar],
        supertypes: vec![SuperType::Legendary],
        power: Some(10), toughness: Some(10),
        rarity: Rarity::Rare,
        ..Default::default() }
}

fn quakestrider_ceratops(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Quakestrider Ceratops".into(), mana_cost: ManaCost::parse("{3}{G}{G}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dinosaur],
        power: Some(12), toughness: Some(8),
        rarity: Rarity::Common,
        ..Default::default() }
}

fn quick_draw_katana(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Quick-Draw Katana".into(), mana_cost: ManaCost::parse("{2}"),
        card_types: vec![CardType::Artifact],
        subtypes: vec![SubType::Equipment],
        keywords: KeywordAbilities::FIRST_STRIKE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "Equip {2}",
                vec![Cost::pay_mana("{2}")],
                vec![Effect::equip()],
                TargetSpec::Creature),
        ],
        ..Default::default() }
}

fn raise_the_past(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Raise the Past".into(), mana_cost: ManaCost::parse("{2}{W}{W}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Return all creature cards with mana value 2 or less from your graveyard to the battlefield.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn rampaging_baloths(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Rampaging Baloths".into(), mana_cost: ManaCost::parse("{4}{G}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Beast],
        power: Some(6), toughness: Some(6),
        keywords: KeywordAbilities::TRAMPLE,
        rarity: Rarity::Common,
        ..Default::default() }
}

fn ravenous_amulet(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Ravenous Amulet".into(), mana_cost: ManaCost::parse("{2}"),
        card_types: vec![CardType::Artifact],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{1}, {T}, Sacrifice a creature: Draw a card and put a soul counter on this artifact. Activate only as a sorcery.",
                vec![Cost::Custom("{1}, {T}, Sacrifice a creature: Draw a card and put a soul counter on this artifact. Activate only as a sorcery.".into())],
                vec![Effect::Custom("{1}, {T}, Sacrifice a creature: Draw a card and put a soul counter on this artifact. Activate only as a sorcery.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn refute(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Refute".into(), mana_cost: ManaCost::parse("{1}{U}{U}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Counter target spell. Draw a card, then discard a card.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn release_the_dogs(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Release the Dogs".into(), mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Create four 1/1 white Dog creature tokens.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn resolute_reinforcements(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Resolute Reinforcements".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Soldier],
        power: Some(1), toughness: Some(1),
        keywords: KeywordAbilities::FLASH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When Resolute Reinforcements enters the battlefield, create a 1/1 white Soldier creature token.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When Resolute Reinforcements enters the battlefield, create a 1/1 white Soldier creature token.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn revenge_of_the_rats(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Revenge of the Rats".into(), mana_cost: ManaCost::parse("{2}{B}{B}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "Flashback {2}{B}{B}",
                vec![Cost::pay_mana("{2}{B}{B}")],
                vec![Effect::Custom("Cast from graveyard, then exile.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn rise_of_the_dark_realms(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Rise of the Dark Realms".into(), mana_cost: ManaCost::parse("{7}{B}{B}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Put all creature cards from all graveyards onto the battlefield under your control.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn rivers_rebuke(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "River's Rebuke".into(), mana_cost: ManaCost::parse("{4}{U}{U}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Return all nonland permanents target player controls to their owner's hand.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn rogues_passage(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Rogue's Passage".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{T}: Add {C}.",
                vec![Cost::Custom("{T}: Add {C}.".into())],
                vec![Effect::Custom("{T}: Add {C}.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn run_away_together(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Run Away Together".into(), mana_cost: ManaCost::parse("{1}{U}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Choose two target creatures controlled by different players. Return those creatures to their owners' hands.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn rune_scarred_demon(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Rune-Scarred Demon".into(), mana_cost: ManaCost::parse("{5}{B}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Demon],
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        ..Default::default() }
}

fn sanguine_indulgence(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sanguine Indulgence".into(), mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("This spell costs {3} less to cast if you've gained 3 or more life this turn.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn savage_ventmaw(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Savage Ventmaw".into(), mana_cost: ManaCost::parse("{4}{R}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dragon],
        power: Some(4), toughness: Some(4),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever Savage Ventmaw attacks, add {R}{R}{R}{G}{G}{G}. Until end of turn, you don't lose this mana as steps and phases end.",
                vec![EventType::AttackerDeclared],
                vec![Effect::Custom("Whenever Savage Ventmaw attacks, add {R}{R}{R}{G}{G}{G}. Until end of turn, you don't lose this mana as steps and phases end.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn scrawling_crawler(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Scrawling Crawler".into(), mana_cost: ManaCost::parse("{3}"),
        card_types: vec![CardType::Artifact, CardType::Creature],
        subtypes: vec![SubType::Phyrexian, SubType::Construct],
        power: Some(3), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "At the beginning of your upkeep, each player draws a card.",
                vec![EventType::UpkeepStep],
                vec![Effect::Custom("At the beginning of your upkeep, each player draws a card.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn searslicer_goblin(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Searslicer Goblin".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Goblin, SubType::Warrior],
        power: Some(2), toughness: Some(1),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Raid -- At the beginning of your end step, if you attacked this turn, create a 1/1 red Goblin creature token.",
                vec![EventType::EndStep],
                vec![Effect::Custom("Raid -- At the beginning of your end step, if you attacked this turn, create a 1/1 red Goblin creature token.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn seismic_rupture(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Seismic Rupture".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Sorcery],
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Seismic Rupture deals 2 damage to each creature without flying.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn seize_the_spoils(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Seize the Spoils".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("As an additional cost to cast this spell, discard a card.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn shipwreck_dowser(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Shipwreck Dowser".into(), mana_cost: ManaCost::parse("{3}{U}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Merfolk, SubType::Wizard],
        power: Some(3), toughness: Some(3),
        keywords: KeywordAbilities::PROWESS,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When Shipwreck Dowser enters the battlefield, return target instant or sorcery card from your graveyard to your hand.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When Shipwreck Dowser enters the battlefield, return target instant or sorcery card from your graveyard to your hand.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn skyknight_squire(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Skyknight Squire".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Cat, SubType::Scout],
        power: Some(1), toughness: Some(1),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        ..Default::default() }
}

fn slumbering_cerberus(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Slumbering Cerberus".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Dog],
        power: Some(4), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Morbid -- At the beginning of each end step, if a creature died this turn, untap this creature.",
                vec![EventType::EndStep],
                vec![Effect::Custom("Morbid -- At the beginning of each end step, if a creature died this turn, untap this creature.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn sorcerous_spyglass(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sorcerous Spyglass".into(), mana_cost: ManaCost::parse("{2}"),
        card_types: vec![CardType::Artifact],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("As Sorcerous Spyglass enters the battlefield, look at an opponent's hand, then choose any card name.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn soulstone_sanctuary(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Soulstone Sanctuary".into(),
        card_types: vec![CardType::Land],
        keywords: KeywordAbilities::VIGILANCE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::mana_ability(id, "{T}: Add {C}.", Mana::colorless(1)),
            Ability::activated(id,
                "{4}: This land becomes a 3/3 creature with vigilance and all creature types. It's still a land.",
                vec![Cost::Custom("{4}: This land becomes a 3/3 creature with vigilance and all creature types. It's still a land.".into())],
                vec![Effect::Custom("{4}: This land becomes a 3/3 creature with vigilance and all creature types. It's still a land.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn sower_of_chaos(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Sower of Chaos".into(), mana_cost: ManaCost::parse("{3}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Devil],
        power: Some(4), toughness: Some(3),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{2}{R}: Target creature can't block this turn.",
                vec![Cost::Custom("{2}{R}: Target creature can't block this turn.".into())],
                vec![Effect::Custom("{2}{R}: Target creature can't block this turn.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn spinner_of_souls(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Spinner of Souls".into(), mana_cost: ManaCost::parse("{2}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Spider, SubType::Spirit],
        power: Some(4), toughness: Some(3),
        keywords: KeywordAbilities::REACH,
        rarity: Rarity::Common,
        ..Default::default() }
}

fn squad_rallier(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Squad Rallier".into(), mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Scout],
        power: Some(3), toughness: Some(4),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{2}{W}: Look at the top four cards of your library. You may reveal a creature card with power 2 or less from among them and put it into your hand. Put the rest on the bottom of your library in a rando",
                vec![Cost::Custom("{2}{W}: Look at the top four cards of your library. You may reveal a creature card with power 2 or less from among them and put it into your hand. Put the rest on the bottom of your library in a rando".into())],
                vec![Effect::Custom("{2}{W}: Look at the top four cards of your library. You may reveal a creature card with power 2 or less from among them and put it into your hand. Put the rest on the bottom of your library in a rando".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn stasis_snare(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Stasis Snare".into(), mana_cost: ManaCost::parse("{1}{W}{W}"),
        card_types: vec![CardType::Enchantment],
        keywords: KeywordAbilities::FLASH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When Stasis Snare enters the battlefield, exile target creature an opponent controls until Stasis Snare leaves the battlefield.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When Stasis Snare enters the battlefield, exile target creature an opponent controls until Stasis Snare leaves the battlefield.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn stroke_of_midnight(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Stroke of Midnight".into(), mana_cost: ManaCost::parse("{2}{W}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Destroy target nonland permanent. Its controller creates a 1/1 white Human creature token.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn stromkirk_bloodthief(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Stromkirk Bloodthief".into(), mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Vampire, SubType::Rogue],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "At the beginning of your end step, if an opponent lost life this turn, put a +1/+1 counter on target Vampire you control.",
                vec![EventType::EndStep],
                vec![Effect::Custom("At the beginning of your end step, if an opponent lost life this turn, put a +1/+1 counter on target Vampire you control.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn strongbox_raider(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Strongbox Raider".into(), mana_cost: ManaCost::parse("{2}{R}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Orc, SubType::Pirate],
        power: Some(5), toughness: Some(2),
        rarity: Rarity::Common,
        ..Default::default() }
}

fn suspicious_shambler(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Suspicious Shambler".into(), mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Zombie],
        power: Some(4), toughness: Some(2),
        rarity: Rarity::Common,
        ..Default::default() }
}

fn taurean_mauler(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Taurean Mauler".into(), mana_cost: ManaCost::parse("{2}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Shapeshifter],
        power: Some(2), toughness: Some(2),
        keywords: KeywordAbilities::CHANGELING,
        rarity: Rarity::Common,
        ..Default::default() }
}

fn teach_by_example(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Teach by Example".into(), mana_cost: ManaCost::parse("{U/R}{U/R}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("When you cast your next instant or sorcery spell this turn, copy that spell. You may choose new targets for the copy.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn three_tree_mascot(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Three Tree Mascot".into(), mana_cost: ManaCost::parse("{2}"),
        card_types: vec![CardType::Artifact, CardType::Creature],
        subtypes: vec![SubType::Shapeshifter],
        power: Some(2), toughness: Some(1),
        keywords: KeywordAbilities::CHANGELING,
        rarity: Rarity::Common,
        ..Default::default() }
}

fn thrill_of_possibility(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Thrill of Possibility".into(), mana_cost: ManaCost::parse("{1}{R}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("As an additional cost to cast this spell, discard a card.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn time_stop(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Time Stop".into(), mana_cost: ManaCost::parse("{4}{U}{U}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("End the turn.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn tragic_banshee(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Tragic Banshee".into(), mana_cost: ManaCost::parse("{4}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Spirit],
        power: Some(5), toughness: Some(3),
        rarity: Rarity::Common,
        ..Default::default() }
}

fn tribute_to_hunger(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Tribute to Hunger".into(), mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Target opponent sacrifices a creature. You gain life equal to that creature's toughness.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn trygon_predator(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Trygon Predator".into(), mana_cost: ManaCost::parse("{1}{G}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Beast],
        power: Some(2), toughness: Some(3),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever Trygon Predator deals combat damage to a player, you may destroy target artifact or enchantment that player controls.",
                vec![EventType::DamagedPlayer],
                vec![Effect::Custom("Whenever Trygon Predator deals combat damage to a player, you may destroy target artifact or enchantment that player controls.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn twinblade_paladin(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Twinblade Paladin".into(), mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Knight],
        power: Some(3), toughness: Some(3),
        keywords: KeywordAbilities::DOUBLE_STRIKE,
        rarity: Rarity::Common,
        ..Default::default() }
}

fn uncharted_haven(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Uncharted Haven".into(),
        card_types: vec![CardType::Land],
        rarity: Rarity::Common,
        ..Default::default() }
}

fn uncharted_voyage(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Uncharted Voyage".into(), mana_cost: ManaCost::parse("{3}{U}"),
        card_types: vec![CardType::Instant],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Target creature's owner puts it on their choice of the top or bottom of their library.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn unflinching_courage(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Unflinching Courage".into(), mana_cost: ManaCost::parse("{1}{G}{W}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Aura],
        keywords: KeywordAbilities::TRAMPLE | KeywordAbilities::LIFELINK,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Enchant creature".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn untamed_hunger(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Untamed Hunger".into(), mana_cost: ManaCost::parse("{2}{B}"),
        card_types: vec![CardType::Enchantment],
        subtypes: vec![SubType::Aura],
        keywords: KeywordAbilities::MENACE,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Enchant creature".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn vampire_gourmand(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Vampire Gourmand".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Vampire],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "Whenever this creature attacks, you may sacrifice another creature. If you do, draw a card and this creature can't be blocked this turn.",
                vec![EventType::AttackerDeclared],
                vec![Effect::Custom("Whenever this creature attacks, you may sacrifice another creature. If you do, draw a card and this creature can't be blocked this turn.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn vampiric_rites(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Vampiric Rites".into(), mana_cost: ManaCost::parse("{B}"),
        card_types: vec![CardType::Enchantment],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::activated(id,
                "{1}{B}, Sacrifice a creature: You gain 1 life and draw a card.",
                vec![Cost::Custom("{1}{B}, Sacrifice a creature: You gain 1 life and draw a card.".into())],
                vec![Effect::Custom("{1}{B}, Sacrifice a creature: You gain 1 life and draw a card.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn vanguard_seraph(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Vanguard Seraph".into(), mana_cost: ManaCost::parse("{3}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Angel, SubType::Warrior],
        power: Some(3), toughness: Some(3),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        ..Default::default() }
}

fn vengeful_bloodwitch(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Vengeful Bloodwitch".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Vampire, SubType::Warlock],
        power: Some(1), toughness: Some(1),
        rarity: Rarity::Common,
        ..Default::default() }
}

fn venom_connoisseur(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Venom Connoisseur".into(), mana_cost: ManaCost::parse("{1}{G}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Human, SubType::Druid],
        power: Some(2), toughness: Some(2),
        keywords: KeywordAbilities::DEATHTOUCH,
        rarity: Rarity::Common,
        ..Default::default() }
}

fn vile_entomber(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Vile Entomber".into(), mana_cost: ManaCost::parse("{2}{B}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Zombie, SubType::Warlock],
        power: Some(2), toughness: Some(2),
        keywords: KeywordAbilities::DEATHTOUCH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When Vile Entomber enters the battlefield, search your library for a card, put that card into your graveyard, then shuffle.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When Vile Entomber enters the battlefield, search your library for a card, put that card into your graveyard, then shuffle.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn vivien_reid(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Vivien Reid".into(), mana_cost: ManaCost::parse("{3}{G}{G}"),
        card_types: vec![CardType::Planeswalker],
        subtypes: vec![SubType::PwVivien],
        supertypes: vec![SuperType::Legendary],
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Uncommon,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("+1: Look at the top four cards of your library. You may reveal a creature or land card from among them and put it into your hand.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn volley_veteran(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Volley Veteran".into(), mana_cost: ManaCost::parse("{3}{R}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Goblin, SubType::Warrior],
        power: Some(4), toughness: Some(2),
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When Volley Veteran enters the battlefield, it deals damage to target creature an opponent controls equal to the number of Goblins you control.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When Volley Veteran enters the battlefield, it deals damage to target creature an opponent controls equal to the number of Goblins you control.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn voracious_greatshark(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Voracious Greatshark".into(), mana_cost: ManaCost::parse("{3}{U}{U}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Shark],
        power: Some(5), toughness: Some(4),
        keywords: KeywordAbilities::FLASH,
        rarity: Rarity::Common,
        abilities: vec![
            Ability::triggered(id,
                "When Voracious Greatshark enters the battlefield, counter target artifact or creature spell.",
                vec![EventType::EnteredTheBattlefield],
                vec![Effect::Custom("When Voracious Greatshark enters the battlefield, counter target artifact or creature spell.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn youthful_valkyrie(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Youthful Valkyrie".into(), mana_cost: ManaCost::parse("{1}{W}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Angel],
        power: Some(1), toughness: Some(3),
        keywords: KeywordAbilities::FLYING,
        rarity: Rarity::Common,
        ..Default::default() }
}

fn zombify(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Zombify".into(), mana_cost: ManaCost::parse("{3}{B}"),
        card_types: vec![CardType::Sorcery],
        rarity: Rarity::Common,
        abilities: vec![
            Ability::spell(id,
                vec![Effect::Custom("Return target creature card from your graveyard to the battlefield.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}

fn zul_ashur_lich_lord(id: ObjectId, owner: PlayerId) -> CardData {
    CardData { id, owner, name: "Zul Ashur, Lich Lord".into(), mana_cost: ManaCost::parse("{1}{B}"),
        card_types: vec![CardType::Creature],
        subtypes: vec![SubType::Zombie, SubType::Warlock],
        supertypes: vec![SuperType::Legendary],
        power: Some(2), toughness: Some(2),
        rarity: Rarity::Rare,
        abilities: vec![
            Ability::activated(id,
                "{T}: You may cast target Zombie creature card from your graveyard this turn.",
                vec![Cost::Custom("{T}: You may cast target Zombie creature card from your graveyard this turn.".into())],
                vec![Effect::Custom("{T}: You may cast target Zombie creature card from your graveyard this turn.".into())],
                TargetSpec::None),
        ],
        ..Default::default() }
}
