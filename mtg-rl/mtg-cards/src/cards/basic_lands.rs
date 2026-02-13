// Basic land card implementations.
// These are shared across all sets that include basic lands.

use mtg_engine::card::CardData;
use mtg_engine::constants::{CardType, Rarity, SubType, SuperType};
use mtg_engine::mana::ManaCost;
use mtg_engine::types::{ObjectId, PlayerId};

pub fn plains(id: ObjectId, owner: PlayerId) -> CardData {
    CardData {
        id,
        owner,
        name: "Plains".to_string(),
        mana_cost: ManaCost::new(),
        card_types: vec![CardType::Land],
        supertypes: vec![SuperType::Basic],
        subtypes: vec![SubType::Plains],
        rarity: Rarity::Land,
        ..Default::default()
    }
}

pub fn island(id: ObjectId, owner: PlayerId) -> CardData {
    CardData {
        id,
        owner,
        name: "Island".to_string(),
        mana_cost: ManaCost::new(),
        card_types: vec![CardType::Land],
        supertypes: vec![SuperType::Basic],
        subtypes: vec![SubType::Island],
        rarity: Rarity::Land,
        ..Default::default()
    }
}

pub fn swamp(id: ObjectId, owner: PlayerId) -> CardData {
    CardData {
        id,
        owner,
        name: "Swamp".to_string(),
        mana_cost: ManaCost::new(),
        card_types: vec![CardType::Land],
        supertypes: vec![SuperType::Basic],
        subtypes: vec![SubType::Swamp],
        rarity: Rarity::Land,
        ..Default::default()
    }
}

pub fn mountain(id: ObjectId, owner: PlayerId) -> CardData {
    CardData {
        id,
        owner,
        name: "Mountain".to_string(),
        mana_cost: ManaCost::new(),
        card_types: vec![CardType::Land],
        supertypes: vec![SuperType::Basic],
        subtypes: vec![SubType::Mountain],
        rarity: Rarity::Land,
        ..Default::default()
    }
}

/// Register all five basic lands into a CardRegistry under the given set code.
pub fn register(registry: &mut crate::registry::CardRegistry, set_code: &'static str) {
    registry.register("Plains", plains, set_code);
    registry.register("Island", island, set_code);
    registry.register("Swamp", swamp, set_code);
    registry.register("Mountain", mountain, set_code);
    registry.register("Forest", forest, set_code);
}

pub fn forest(id: ObjectId, owner: PlayerId) -> CardData {
    CardData {
        id,
        owner,
        name: "Forest".to_string(),
        mana_cost: ManaCost::new(),
        card_types: vec![CardType::Land],
        supertypes: vec![SuperType::Basic],
        subtypes: vec![SubType::Forest],
        rarity: Rarity::Land,
        ..Default::default()
    }
}
