// Tests for the game module
// Import and re-export test modules that were extracted from game.rs

#[path = "../tests/abilities.rs"]
mod abilities;
#[path = "../tests/combat.rs"]
mod combat;
#[path = "../tests/continuous_effects.rs"]
mod continuous_effects;
#[path = "../tests/costs.rs"]
mod costs;
#[path = "../tests/effects.rs"]
mod effects;
#[path = "../tests/equipment_auras.rs"]
mod equipment_auras;
#[path = "../tests/game_basics.rs"]
mod game_basics;
#[path = "../tests/keywords.rs"]
mod keywords;
#[path = "../tests/modal.rs"]
mod modal;
#[path = "../tests/special_mechanics.rs"]
mod special_mechanics;
#[path = "../tests/tokens.rs"]
mod tokens;
#[path = "../tests/triggers.rs"]
mod triggers;

// Re-export all test modules
pub use abilities::*;
pub use combat::*;
pub use continuous_effects::*;
pub use costs::*;
pub use effects::*;
pub use equipment_auras::*;
pub use game_basics::*;
pub use keywords::*;
pub use modal::*;
pub use special_mechanics::*;
pub use tokens::*;
pub use triggers::*;