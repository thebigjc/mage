// Card data definitions — the static "blueprint" for a card.
//
// A CardData struct holds everything needed to create a card instance in a game:
// name, mana cost, types, subtypes, supertypes, P/T, keywords, color identity,
// and eventually abilities. Cards are created by factory functions in mtg-cards.
//
// This is the Rust equivalent of Java's CardImpl static data. The actual
// in-game card object (with zone, owner, controller, counters, etc.) will
// be in zones.rs / state.rs once the game state is implemented.

use crate::abilities::Ability;
use crate::constants::{
    CardType, Color, KeywordAbilities, Rarity, SubType, SuperType,
};
use crate::mana::ManaCost;
use crate::types::{ObjectId, PlayerId};
use serde::{Deserialize, Serialize};

/// Static card data — the "blueprint" for creating a card in a game.
///
/// Card factory functions in mtg-cards return this struct. The game engine
/// uses it to initialize the card object in the appropriate zone.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CardData {
    /// Unique object ID for this card instance.
    pub id: ObjectId,
    /// Player who owns this card (it goes to their graveyard, etc.).
    pub owner: PlayerId,
    /// Card name as printed.
    pub name: String,
    /// Mana cost (e.g. "{2}{B}{G}"). Lands have an empty mana cost.
    pub mana_cost: ManaCost,
    /// Card types (Creature, Instant, Sorcery, etc.).
    pub card_types: Vec<CardType>,
    /// Supertypes (Basic, Legendary, Snow, World).
    pub supertypes: Vec<SuperType>,
    /// Subtypes (creature types, land types, spell types, etc.).
    pub subtypes: Vec<SubType>,
    /// Base power for creatures. None for non-creatures.
    pub power: Option<i32>,
    /// Base toughness for creatures. None for non-creatures.
    pub toughness: Option<i32>,
    /// Keyword abilities (flying, trample, etc.) as bitflags.
    pub keywords: KeywordAbilities,
    /// Color identity (for Commander; also used as card colors when set explicitly).
    /// If empty, colors are derived from the mana cost.
    pub color_identity: Vec<Color>,
    /// Card rarity.
    pub rarity: Rarity,
    /// Starting loyalty for planeswalkers. None for non-planeswalkers.
    pub loyalty: Option<i32>,
    /// Rules text (oracle text). Informational only; actual behavior is in abilities.
    pub rules_text: String,
    /// Abilities on this card (activated, triggered, static, spell, mana).
    pub abilities: Vec<Ability>,
    /// Whether this card is a token (created during the game, not from a deck).
    pub is_token: bool,
    /// Flashback cost: if set, this card can be cast from the graveyard for this
    /// cost, then exiled instead of returning to the graveyard.
    pub flashback_cost: Option<ManaCost>,
}

impl CardData {
    /// Create a new CardData with default/empty fields.
    pub fn new(id: ObjectId, owner: PlayerId, name: &str) -> Self {
        CardData {
            id,
            owner,
            name: name.to_string(),
            mana_cost: ManaCost::new(),
            card_types: Vec::new(),
            supertypes: Vec::new(),
            subtypes: Vec::new(),
            power: None,
            toughness: None,
            keywords: KeywordAbilities::empty(),
            color_identity: Vec::new(),
            rarity: Rarity::Common,
            loyalty: None,
            rules_text: String::new(),
            abilities: Vec::new(),
            is_token: false,
            flashback_cost: None,
        }
    }

    /// Convenience: is this a creature?
    pub fn is_creature(&self) -> bool {
        self.card_types.contains(&CardType::Creature)
    }

    /// Convenience: is this a land?
    pub fn is_land(&self) -> bool {
        self.card_types.contains(&CardType::Land)
    }

    /// Convenience: is this an instant?
    pub fn is_instant(&self) -> bool {
        self.card_types.contains(&CardType::Instant)
    }

    /// Convenience: is this a sorcery?
    pub fn is_sorcery(&self) -> bool {
        self.card_types.contains(&CardType::Sorcery)
    }

    /// Convenience: is this a permanent card?
    pub fn is_permanent_card(&self) -> bool {
        self.card_types.iter().any(|ct| ct.is_permanent())
    }

    /// Convenience: is this legendary?
    pub fn is_legendary(&self) -> bool {
        self.supertypes.contains(&SuperType::Legendary)
    }

    /// Get the card's colors, derived from mana cost if color_identity is empty.
    pub fn colors(&self) -> Vec<Color> {
        if !self.color_identity.is_empty() {
            return self.color_identity.clone();
        }
        self.mana_cost.colors()
    }

    /// Mana value (converted mana cost).
    pub fn mana_value(&self) -> u32 {
        self.mana_cost.mana_value()
    }
}

impl Default for CardData {
    fn default() -> Self {
        CardData {
            id: ObjectId::new(),
            owner: PlayerId::new(),
            name: String::new(),
            mana_cost: ManaCost::new(),
            card_types: Vec::new(),
            supertypes: Vec::new(),
            subtypes: Vec::new(),
            power: None,
            toughness: None,
            keywords: KeywordAbilities::empty(),
            color_identity: Vec::new(),
            rarity: Rarity::Common,
            loyalty: None,
            rules_text: String::new(),
            abilities: Vec::new(),
            is_token: false,
            flashback_cost: None,
        }
    }
}

/// Type alias for card factory functions.
/// Each card implementation provides a factory that creates a CardData
/// given an object ID and owner player ID.
pub type CardFactory = fn(ObjectId, PlayerId) -> CardData;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::SubType;

    #[test]
    fn create_basic_creature() {
        let id = ObjectId::new();
        let owner = PlayerId::new();
        let mut card = CardData::new(id, owner, "Bear Cub");
        card.mana_cost = ManaCost::parse("{1}{G}");
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![SubType::Bear];
        card.power = Some(2);
        card.toughness = Some(2);

        assert!(card.is_creature());
        assert!(!card.is_land());
        assert_eq!(card.mana_value(), 2);
        assert_eq!(card.colors(), vec![Color::Green]);
    }

    #[test]
    fn create_basic_land() {
        let id = ObjectId::new();
        let owner = PlayerId::new();
        let mut card = CardData::new(id, owner, "Forest");
        card.card_types = vec![CardType::Land];
        card.supertypes = vec![SuperType::Basic];
        card.subtypes = vec![SubType::Forest];

        assert!(card.is_land());
        assert!(!card.is_creature());
        assert_eq!(card.mana_value(), 0);
        assert!(card.colors().is_empty());
    }
}
