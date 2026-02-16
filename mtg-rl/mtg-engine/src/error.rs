use crate::types::{AbilityId, ObjectId, PlayerId};
use thiserror::Error;

#[must_use]
#[derive(Debug, Error, Clone)]
pub enum GameError {
    #[error("invalid player: {0}")]
    InvalidPlayer(PlayerId),

    #[error("invalid object: {0}")]
    InvalidObject(ObjectId),

    #[error("object not found in expected zone: {0}")]
    InvalidZone(ObjectId),

    #[error("invalid target: {0}")]
    InvalidTarget(String),

    #[error("invalid action: {0}")]
    InvalidAction(String),

    #[error("ability resolution error: {0}")]
    AbilityResolutionError(AbilityId),

    #[error("game state corruption: {0}")]
    GameStateCorruption(String),
}

pub type EngineResult<T> = Result<T, GameError>;
