use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ObjectId(Uuid);

impl ObjectId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        debug_assert!(!uuid.is_nil(), "ObjectId must not be nil");
        Self(uuid)
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }

    pub fn from_player(player: PlayerId) -> Self {
        Self(player.0)
    }
}

impl Default for ObjectId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Obj({})", &self.0.to_string()[..8])
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0.to_string()[..8])
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PlayerId(Uuid);

impl PlayerId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        debug_assert!(!uuid.is_nil(), "PlayerId must not be nil");
        Self(uuid)
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }

    pub fn from_object(object: ObjectId) -> Self {
        Self(object.0)
    }
}

impl Default for PlayerId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Player({})", &self.0.to_string()[..8])
    }
}

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0.to_string()[..8])
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct AbilityId(Uuid);

impl AbilityId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        debug_assert!(!uuid.is_nil(), "AbilityId must not be nil");
        Self(uuid)
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for AbilityId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for AbilityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0.to_string()[..8])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_ids_are_unique() {
        let a = ObjectId::new();
        let b = ObjectId::new();
        assert_ne!(a, b);
    }

    #[test]
    fn from_uuid_roundtrips() {
        let id = ObjectId::new();
        let uuid = id.as_uuid();
        let id2 = ObjectId::from_uuid(uuid);
        assert_eq!(id, id2);
    }

    #[test]
    fn player_object_conversion_preserves_uuid() {
        let player = PlayerId::new();
        let object = ObjectId::from_player(player);
        let player2 = PlayerId::from_object(object);
        assert_eq!(player, player2);
    }

    #[test]
    #[should_panic(expected = "ObjectId must not be nil")]
    fn object_id_rejects_nil() {
        ObjectId::from_uuid(Uuid::nil());
    }

    #[test]
    #[should_panic(expected = "PlayerId must not be nil")]
    fn player_id_rejects_nil() {
        PlayerId::from_uuid(Uuid::nil());
    }

    #[test]
    #[should_panic(expected = "AbilityId must not be nil")]
    fn ability_id_rejects_nil() {
        AbilityId::from_uuid(Uuid::nil());
    }
}
