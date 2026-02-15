// Tests for the game module
// Re-export all the test content that was previously inline in game.rs

#[path = "../tests/all_game_tests.rs"]
mod all_tests;

// Re-export everything from all_tests to maintain the same test structure
pub use all_tests::*;