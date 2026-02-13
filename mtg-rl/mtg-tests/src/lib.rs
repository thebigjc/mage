// Integration test harness for MTG engine.
//
// Provides a declarative test API similar to Java's CardTestPlayerBase:
//
//   let mut test = GameTest::new();
//   test.add_card(Zone::Hand, Player::A, "Lightning Bolt");
//   test.add_card(Zone::Battlefield, Player::A, "Mountain");
//   test.add_card(Zone::Battlefield, Player::B, "Grizzly Bears");
//   test.cast_spell(1, PhaseStep::PrecombatMain, Player::A, "Lightning Bolt", Some("Grizzly Bears"));
//   test.stop_at(1, PhaseStep::PostcombatMain);
//   test.execute();
//   test.assert_graveyard_count(Player::B, "Grizzly Bears", 1);

pub mod concurrency;
pub mod framework;
pub mod scripted_player;

pub use framework::*;
pub use scripted_player::*;
