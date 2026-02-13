// Card registry — maps card names to factory functions.
//
// The registry is the central lookup table for creating card instances.
// Each card has a factory function that produces a CardData struct.
// Sets register their cards at initialization time.

use mtg_engine::card::{CardData, CardFactory};
use mtg_engine::types::{ObjectId, PlayerId};
use std::collections::HashMap;

/// Metadata about a card in the registry.
#[derive(Clone, Debug)]
pub struct CardInfo {
    /// Canonical card name (as printed on the card).
    pub name: String,
    /// Factory function to create the card.
    pub factory: CardFactory,
    /// Which sets this card appears in (set codes like "FDN", "TLA").
    pub sets: Vec<&'static str>,
}

/// The card registry: maps card names to their factory functions.
///
/// Usage:
/// ```ignore
/// let registry = CardRegistry::new();
/// let card = registry.create("Lightning Bolt", id, owner).unwrap();
/// ```
pub struct CardRegistry {
    cards: HashMap<String, CardInfo>,
}

impl CardRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        CardRegistry {
            cards: HashMap::new(),
        }
    }

    /// Create a registry pre-populated with all known cards from all sets.
    pub fn with_all_sets() -> Self {
        let mut registry = Self::new();
        crate::sets::fdn::register(&mut registry);
        crate::sets::tla::register(&mut registry);
        crate::sets::tdm::register(&mut registry);
        crate::sets::ecl::register(&mut registry);
        registry
    }

    /// Register a card factory.
    pub fn register(&mut self, name: &str, factory: CardFactory, set_code: &'static str) {
        let entry = self.cards.entry(name.to_string()).or_insert_with(|| CardInfo {
            name: name.to_string(),
            factory,
            sets: Vec::new(),
        });
        if !entry.sets.contains(&set_code) {
            entry.sets.push(set_code);
        }
    }

    /// Create a card by name.
    pub fn create(&self, name: &str, id: ObjectId, owner: PlayerId) -> Option<CardData> {
        self.cards.get(name).map(|info| (info.factory)(id, owner))
    }

    /// Check if a card exists in the registry.
    pub fn contains(&self, name: &str) -> bool {
        self.cards.contains_key(name)
    }

    /// Get info about a card.
    pub fn get_info(&self, name: &str) -> Option<&CardInfo> {
        self.cards.get(name)
    }

    /// Get all registered card names.
    pub fn card_names(&self) -> Vec<&str> {
        self.cards.keys().map(|s| s.as_str()).collect()
    }

    /// Get card names for a specific set.
    pub fn cards_in_set(&self, set_code: &str) -> Vec<&str> {
        self.cards
            .iter()
            .filter(|(_, info)| info.sets.iter().any(|s| *s == set_code))
            .map(|(name, _)| name.as_str())
            .collect()
    }

    /// Total number of unique cards registered.
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

impl Default for CardRegistry {
    fn default() -> Self {
        Self::with_all_sets()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mtg_engine::constants::{CardType, KeywordAbilities, SubType};

    #[test]
    fn registry_create_and_lookup() {
        let registry = CardRegistry::with_all_sets();
        assert!(registry.contains("Forest"));
        assert!(registry.contains("Island"));
        assert!(registry.contains("Mountain"));
        assert!(registry.contains("Plains"));
        assert!(registry.contains("Swamp"));

        let id = ObjectId::new();
        let owner = PlayerId::new();
        let forest = registry.create("Forest", id, owner).unwrap();
        assert_eq!(forest.name, "Forest");
        assert!(forest.is_land());
    }

    #[test]
    fn registry_set_query() {
        let registry = CardRegistry::with_all_sets();
        let fdn_cards = registry.cards_in_set("FDN");
        assert!(!fdn_cards.is_empty());
    }

    #[test]
    fn registry_card_counts() {
        let registry = CardRegistry::with_all_sets();
        // 5 basic lands shared across sets + unique cards per set
        let fdn = registry.cards_in_set("FDN");
        let tla = registry.cards_in_set("TLA");
        let tdm = registry.cards_in_set("TDM");
        let ecl = registry.cards_in_set("ECL");
        // Counts may grow as teammates add cards; assert minimum thresholds
        assert!(fdn.len() >= 335, "FDN should have at least 335 cards (T1+T2+T3): {}", fdn.len());
        assert!(tla.len() >= 273, "TLA should have at least 273 cards: {}", tla.len());
        assert!(tdm.len() >= 232, "TDM should have at least 232 cards: {}", tdm.len());
        assert!(ecl.len() >= 225, "ECL should have at least 225 cards: {}", ecl.len());
    }

    #[test]
    fn ecl_specific_cards() {
        let registry = CardRegistry::with_all_sets();
        let id = ObjectId::new();
        let owner = PlayerId::new();

        // Test Dawn's Light Archer - has Flash + Reach
        let archer = registry.create("Dawn's Light Archer", id, owner).unwrap();
        assert_eq!(archer.power, Some(4));
        assert_eq!(archer.toughness, Some(2));
        assert!(archer.keywords.contains(KeywordAbilities::FLASH));
        assert!(archer.keywords.contains(KeywordAbilities::REACH));

        // Test Crib Swap - Kindred Instant with Changeling
        let crib = registry.create("Crib Swap", id, owner).unwrap();
        assert!(crib.card_types.contains(&CardType::Kindred));
        assert!(crib.card_types.contains(&CardType::Instant));
        assert!(crib.keywords.contains(KeywordAbilities::CHANGELING));

        // Test Bloom Tender
        let bloom = registry.create("Bloom Tender", id, owner).unwrap();
        assert_eq!(bloom.power, Some(1));
        assert_eq!(bloom.toughness, Some(1));
    }

    #[test]
    fn fdn_tier2_cards() {
        let registry = CardRegistry::with_all_sets();
        let id = ObjectId::new();
        let owner = PlayerId::new();

        // Savannah Lions — vanilla 2/1 Cat for {W}
        let lions = registry.create("Savannah Lions", id, owner).unwrap();
        assert_eq!(lions.power, Some(2));
        assert_eq!(lions.toughness, Some(1));

        // Gigantosaurus — 10/10 for {G}{G}{G}{G}{G}
        let giga = registry.create("Gigantosaurus", id, owner).unwrap();
        assert_eq!(giga.power, Some(10));
        assert_eq!(giga.toughness, Some(10));

        // Heartfire Immolator — 2/2 with Prowess
        let hf = registry.create("Heartfire Immolator", id, owner).unwrap();
        assert_eq!(hf.power, Some(2));
        assert!(hf.keywords.contains(KeywordAbilities::PROWESS));

        // Gleaming Barrier — 0/4 wall with Defender
        let barrier = registry.create("Gleaming Barrier", id, owner).unwrap();
        assert_eq!(barrier.toughness, Some(4));
        assert!(barrier.keywords.contains(KeywordAbilities::DEFENDER));
        assert!(barrier.card_types.contains(&CardType::Artifact));

        // Pacifism — Enchantment Aura
        let pac = registry.create("Pacifism", id, owner).unwrap();
        assert!(pac.card_types.contains(&CardType::Enchantment));

        // Day of Judgment — Sorcery
        let doj = registry.create("Day of Judgment", id, owner).unwrap();
        assert!(doj.card_types.contains(&CardType::Sorcery));

        // Shivan Dragon — 5/5 Dragon with Flying
        let shivan = registry.create("Shivan Dragon", id, owner).unwrap();
        assert_eq!(shivan.power, Some(5));
        assert!(shivan.keywords.contains(KeywordAbilities::FLYING));
    }

    #[test]
    fn fdn_tier3_cards() {
        let registry = CardRegistry::with_all_sets();
        let id = ObjectId::new();
        let owner = PlayerId::new();

        // Vampire Nighthawk — 2/3 flying deathtouch lifelink
        let vn = registry.create("Vampire Nighthawk", id, owner).unwrap();
        assert_eq!(vn.power, Some(2));
        assert_eq!(vn.toughness, Some(3));
        assert!(vn.keywords.contains(KeywordAbilities::FLYING));
        assert!(vn.keywords.contains(KeywordAbilities::DEATHTOUCH));
        assert!(vn.keywords.contains(KeywordAbilities::LIFELINK));

        // Death Baron — 2/2 Zombie Wizard lord
        let db = registry.create("Death Baron", id, owner).unwrap();
        assert_eq!(db.power, Some(2));
        assert!(db.subtypes.contains(&SubType::Zombie));

        // Abrade — modal instant
        let ab = registry.create("Abrade", id, owner).unwrap();
        assert!(ab.card_types.contains(&CardType::Instant));

        // Overrun — sorcery {2}{G}{G}{G}
        let ov = registry.create("Overrun", id, owner).unwrap();
        assert!(ov.card_types.contains(&CardType::Sorcery));

        // Fog Bank — 0/2 Wall with defender and flying
        let fb = registry.create("Fog Bank", id, owner).unwrap();
        assert_eq!(fb.power, Some(0));
        assert_eq!(fb.toughness, Some(2));
        assert!(fb.keywords.contains(KeywordAbilities::DEFENDER));
        assert!(fb.keywords.contains(KeywordAbilities::FLYING));

        // Boros Charm — instant {R}{W}
        let bc = registry.create("Boros Charm", id, owner).unwrap();
        assert!(bc.card_types.contains(&CardType::Instant));

        // Goblin Oriflamme — enchantment
        let go = registry.create("Goblin Oriflamme", id, owner).unwrap();
        assert!(go.card_types.contains(&CardType::Enchantment));
    }

    #[test]
    fn ecl_tier2_spells() {
        let registry = CardRegistry::with_all_sets();
        let id = ObjectId::new();
        let owner = PlayerId::new();

        // Nameless Inversion — Kindred Instant Shapeshifter with Changeling
        let ni = registry.create("Nameless Inversion", id, owner).unwrap();
        assert!(ni.card_types.contains(&CardType::Kindred));
        assert!(ni.card_types.contains(&CardType::Instant));
        assert!(ni.keywords.contains(KeywordAbilities::CHANGELING));

        // Blossoming Defense — Instant {G}
        let bd = registry.create("Blossoming Defense", id, owner).unwrap();
        assert!(bd.card_types.contains(&CardType::Instant));

        // Darkness Descends — Sorcery {2}{B}{B}
        let dd = registry.create("Darkness Descends", id, owner).unwrap();
        assert!(dd.card_types.contains(&CardType::Sorcery));

        // Boggart Mischief — Kindred Enchantment
        let bm = registry.create("Boggart Mischief", id, owner).unwrap();
        assert!(bm.card_types.contains(&CardType::Kindred));
        assert!(bm.card_types.contains(&CardType::Enchantment));
    }

    #[test]
    fn tdm_tier2_spells() {
        let registry = CardRegistry::with_all_sets();
        let id = ObjectId::new();
        let owner = PlayerId::new();

        // Twin Bolt — Instant {1}{R}
        let tb = registry.create("Twin Bolt", id, owner).unwrap();
        assert!(tb.card_types.contains(&CardType::Instant));

        // Channeled Dragonfire — Sorcery {R}
        let cd = registry.create("Channeled Dragonfire", id, owner).unwrap();
        assert!(cd.card_types.contains(&CardType::Sorcery));

        // Dragonback Assault — Enchantment {3}{G}{U}{R}
        let da = registry.create("Dragonback Assault", id, owner).unwrap();
        assert!(da.card_types.contains(&CardType::Enchantment));

        // Snakeskin Veil — Instant {G}
        let sv = registry.create("Snakeskin Veil", id, owner).unwrap();
        assert!(sv.card_types.contains(&CardType::Instant));
    }
}
