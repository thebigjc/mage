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
    pub name: &'static str,
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
    cards: HashMap<&'static str, CardInfo>,
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
    pub fn register(&mut self, name: &'static str, factory: CardFactory, set_code: &'static str) {
        let entry = self.cards.entry(name).or_insert_with(|| CardInfo {
            name,
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
        self.cards.keys().copied().collect()
    }

    /// Get card names for a specific set.
    pub fn cards_in_set(&self, set_code: &str) -> Vec<&str> {
        self.cards
            .iter()
            .filter(|(_, info)| info.sets.contains(&set_code))
            .map(|(name, _)| *name)
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
    use mtg_engine::types::{Power, Toughness};

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
        assert_eq!(archer.power, Some(Power::new(4)));
        assert_eq!(archer.toughness, Some(Toughness::new(2)));
        assert!(archer.keywords.contains(KeywordAbilities::FLASH));
        assert!(archer.keywords.contains(KeywordAbilities::REACH));

        // Test Crib Swap - Kindred Instant with Changeling
        let crib = registry.create("Crib Swap", id, owner).unwrap();
        assert!(crib.card_types.contains(&CardType::Kindred));
        assert!(crib.card_types.contains(&CardType::Instant));
        assert!(crib.keywords.contains(KeywordAbilities::CHANGELING));

        // Test Bloom Tender
        let bloom = registry.create("Bloom Tender", id, owner).unwrap();
        assert_eq!(bloom.power, Some(Power::new(1)));
        assert_eq!(bloom.toughness, Some(Toughness::new(1)));
    }

    #[test]
    fn fdn_tier2_cards() {
        let registry = CardRegistry::with_all_sets();
        let id = ObjectId::new();
        let owner = PlayerId::new();

        // Savannah Lions — vanilla 2/1 Cat for {W}
        let lions = registry.create("Savannah Lions", id, owner).unwrap();
        assert_eq!(lions.power, Some(Power::new(2)));
        assert_eq!(lions.toughness, Some(Toughness::new(1)));

        // Gigantosaurus — 10/10 for {G}{G}{G}{G}{G}
        let giga = registry.create("Gigantosaurus", id, owner).unwrap();
        assert_eq!(giga.power, Some(Power::new(10)));
        assert_eq!(giga.toughness, Some(Toughness::new(10)));

        // Heartfire Immolator — 2/2 with Prowess
        let hf = registry.create("Heartfire Immolator", id, owner).unwrap();
        assert_eq!(hf.power, Some(Power::new(2)));
        assert!(hf.keywords.contains(KeywordAbilities::PROWESS));

        // Gleaming Barrier — 0/4 wall with Defender
        let barrier = registry.create("Gleaming Barrier", id, owner).unwrap();
        assert_eq!(barrier.toughness, Some(Toughness::new(4)));
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
        assert_eq!(shivan.power, Some(Power::new(5)));
        assert!(shivan.keywords.contains(KeywordAbilities::FLYING));
    }

    #[test]
    fn fdn_tier3_cards() {
        let registry = CardRegistry::with_all_sets();
        let id = ObjectId::new();
        let owner = PlayerId::new();

        // Vampire Nighthawk — 2/3 flying deathtouch lifelink
        let vn = registry.create("Vampire Nighthawk", id, owner).unwrap();
        assert_eq!(vn.power, Some(Power::new(2)));
        assert_eq!(vn.toughness, Some(Toughness::new(3)));
        assert!(vn.keywords.contains(KeywordAbilities::FLYING));
        assert!(vn.keywords.contains(KeywordAbilities::DEATHTOUCH));
        assert!(vn.keywords.contains(KeywordAbilities::LIFELINK));

        // Death Baron — 2/2 Zombie Wizard lord
        let db = registry.create("Death Baron", id, owner).unwrap();
        assert_eq!(db.power, Some(Power::new(2)));
        assert!(db.subtypes.contains(&SubType::Zombie));

        // Abrade — modal instant
        let ab = registry.create("Abrade", id, owner).unwrap();
        assert!(ab.card_types.contains(&CardType::Instant));

        // Overrun — sorcery {2}{G}{G}{G}
        let ov = registry.create("Overrun", id, owner).unwrap();
        assert!(ov.card_types.contains(&CardType::Sorcery));

        // Fog Bank — 0/2 Wall with defender and flying
        let fb = registry.create("Fog Bank", id, owner).unwrap();
        assert_eq!(fb.power, Some(Power::new(0)));
        assert_eq!(fb.toughness, Some(Toughness::new(2)));
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
    fn ecl_cond_card_abilities() {
        let registry = CardRegistry::with_all_sets();
        let id = ObjectId::new();
        let owner = PlayerId::new();
        use mtg_engine::abilities::{Effect, StaticEffect, TargetSpec};

        // Impolite Entrance — Sorcery, spell: trample+haste EOT + draw, targets creature
        let ie = registry.create("Impolite Entrance", id, owner).unwrap();
        assert!(ie.card_types.contains(&CardType::Sorcery));
        assert_eq!(ie.abilities.len(), 1);
        let spell = &ie.abilities[0];
        assert_eq!(spell.effects.len(), 3);
        assert!(matches!(spell.effects[0], Effect::GainKeywordUntilEndOfTurn { ref keyword } if keyword == "trample"));
        assert!(matches!(spell.effects[1], Effect::GainKeywordUntilEndOfTurn { ref keyword } if keyword == "haste"));
        assert!(matches!(spell.effects[2], Effect::DrawCards { count: 1 }));
        assert!(matches!(spell.targets, TargetSpec::Creature));

        // Riverguard's Reflexes — Instant, +2/+2 + first strike EOT + untap, targets creature
        let rr = registry.create("Riverguard's Reflexes", id, owner).unwrap();
        assert!(rr.card_types.contains(&CardType::Instant));
        assert_eq!(rr.abilities[0].effects.len(), 3);
        assert!(matches!(rr.abilities[0].effects[0], Effect::BoostUntilEndOfTurn { power, toughness } if power == Power::new(2) && toughness == Toughness::new(2)));
        assert!(matches!(rr.abilities[0].effects[1], Effect::GainKeywordUntilEndOfTurn { ref keyword } if keyword == "first_strike"));
        assert!(matches!(rr.abilities[0].effects[2], Effect::UntapTarget));
        assert!(matches!(rr.abilities[0].targets, TargetSpec::Creature));

        // Morcant's Loyalist — lord + dies trigger
        let ml = registry.create("Morcant's Loyalist", id, owner).unwrap();
        assert_eq!(ml.power, Some(Power::new(3)));
        assert_eq!(ml.toughness, Some(Toughness::new(2)));
        assert!(ml.abilities.len() >= 2);
        // First ability: static boost to other Elves
        assert!(matches!(&ml.abilities[0].static_effects[..],
            [StaticEffect::Boost { ref filter, power, toughness }] if filter.message.contains("Elf") && *power == Power::new(1) && *toughness == Toughness::new(1)));
        // Second ability: dies trigger returns from GY
        assert!(matches!(ml.abilities[1].effects[..], [Effect::ReturnFromGraveyard]));

        // Gallant Fowlknight — ETB with boost_all + grant_keyword_all
        let gf = registry.create("Gallant Fowlknight", id, owner).unwrap();
        assert_eq!(gf.power, Some(Power::new(3)));
        assert_eq!(gf.abilities[0].effects.len(), 2);
        assert!(matches!(gf.abilities[0].effects[0], Effect::BoostAllUntilEndOfTurn { ref filter, power, toughness } if filter.message.contains("creature") && power == Power::new(1) && toughness == Toughness::new(0)));
        assert!(matches!(gf.abilities[0].effects[1], Effect::GrantKeywordAllUntilEndOfTurn { ref filter, ref keyword } if filter.message.contains("Kithkin") && keyword == "first_strike"));
    }

    #[test]
    fn ecl_cost_card_abilities() {
        let registry = CardRegistry::with_all_sets();
        let id = ObjectId::new();
        let owner = PlayerId::new();
        use mtg_engine::abilities::{Cost, Effect, StaticEffect, TargetSpec};

        // Hovel Hurler — 6/7 Giant, ETB with 2 -1/-1 counters, activated remove counter + boost+fly
        let hh = registry.create("Hovel Hurler", id, owner).unwrap();
        assert_eq!(hh.power, Some(Power::new(6)));
        assert_eq!(hh.toughness, Some(Toughness::new(7)));
        assert_eq!(hh.abilities.len(), 2);
        // ETB: enters with 2 -1/-1 counters (replacement effect via static ability)
        assert!(matches!(hh.abilities[0].static_effects[..],
            [StaticEffect::EntersWithCounters { ref counter_type, count: 2 }] if counter_type == "-1/-1"));
        // Activated: remove counter cost, +1/+0 + flying EOT
        assert!(matches!(&hh.abilities[1].costs[1], Cost::RemoveCounters(ref ct, 1) if ct == "-1/-1"));
        assert!(matches!(hh.abilities[1].effects[0], Effect::BoostUntilEndOfTurn { power, toughness } if power == Power::new(1) && toughness == Toughness::new(0)));
        assert!(matches!(hh.abilities[1].effects[1], Effect::GainKeywordUntilEndOfTurn { ref keyword } if keyword == "flying"));
        assert!(matches!(hh.abilities[1].targets, TargetSpec::CreatureYouControl));

        // Glen Elendra Guardian — 3/4 Flash Flying, ETB with 1 counter, activated counter+draw
        let ge = registry.create("Glen Elendra Guardian", id, owner).unwrap();
        assert_eq!(ge.power, Some(Power::new(3)));
        assert_eq!(ge.toughness, Some(Toughness::new(4)));
        assert!(ge.keywords.contains(KeywordAbilities::FLASH));
        assert!(ge.keywords.contains(KeywordAbilities::FLYING));
        assert_eq!(ge.abilities.len(), 2);
        assert!(matches!(ge.abilities[0].static_effects[..],
            [StaticEffect::EntersWithCounters { ref counter_type, count: 1 }] if counter_type == "-1/-1"));
        assert!(matches!(ge.abilities[1].effects[0], Effect::CounterSpell));
        assert!(matches!(ge.abilities[1].targets, TargetSpec::Spell));

        // Loch Mare — 4/5, ETB with 3 counters, 2 activated abilities
        let lm = registry.create("Loch Mare", id, owner).unwrap();
        assert_eq!(lm.power, Some(Power::new(4)));
        assert_eq!(lm.toughness, Some(Toughness::new(5)));
        assert_eq!(lm.abilities.len(), 3);
        // ETB: enters with 3 -1/-1 counters (replacement effect via static ability)
        assert!(matches!(lm.abilities[0].static_effects[..],
            [StaticEffect::EntersWithCounters { ref counter_type, count: 3 }] if counter_type == "-1/-1"));
        // Activated 1: remove 1, draw
        assert!(matches!(&lm.abilities[1].costs[1], Cost::RemoveCounters(ref ct, 1) if ct == "-1/-1"));
        assert!(matches!(lm.abilities[1].effects[..], [Effect::DrawCards { count: 1 }]));
        // Activated 2: remove 2, tap + stun
        assert!(matches!(&lm.abilities[2].costs[1], Cost::RemoveCounters(ref ct, 2) if ct == "-1/-1"));
        assert!(matches!(lm.abilities[2].effects[0], Effect::TapTarget));
        assert!(matches!(lm.abilities[2].effects[1], Effect::AddCounters { ref counter_type, count: 1 } if counter_type == "stun"));

        // Reaping Willow — 3/6 Lifelink, ETB with 2 counters, activated reanimate
        let rw = registry.create("Reaping Willow", id, owner).unwrap();
        assert_eq!(rw.power, Some(Power::new(3)));
        assert_eq!(rw.toughness, Some(Toughness::new(6)));
        assert!(rw.keywords.contains(KeywordAbilities::LIFELINK));
        assert_eq!(rw.abilities.len(), 2);
        assert!(matches!(rw.abilities[0].static_effects[..],
            [StaticEffect::EntersWithCounters { ref counter_type, count: 2 }] if counter_type == "-1/-1"));
        assert!(matches!(&rw.abilities[1].costs[1], Cost::RemoveCounters(ref ct, 2) if ct == "-1/-1"));
        assert!(matches!(rw.abilities[1].effects[..], [Effect::Reanimate]));
        assert!(matches!(rw.abilities[1].targets, TargetSpec::CardInYourGraveyard));

        // Creakwood Safewright — 5/5, ETB with 3 counters, end step remove counter
        let cs = registry.create("Creakwood Safewright", id, owner).unwrap();
        assert_eq!(cs.power, Some(Power::new(5)));
        assert_eq!(cs.toughness, Some(Toughness::new(5)));
        assert_eq!(cs.abilities.len(), 2);
        assert!(matches!(cs.abilities[0].static_effects[..],
            [StaticEffect::EntersWithCounters { ref counter_type, count: 3 }] if counter_type == "-1/-1"));
        assert!(matches!(cs.abilities[1].effects[..],
            [Effect::RemoveCounters { ref counter_type, count: 1 }] if counter_type == "-1/-1"));

        // Bogslither's Embrace — Sorcery, exile target creature
        let be = registry.create("Bogslither's Embrace", id, owner).unwrap();
        assert!(be.card_types.contains(&CardType::Sorcery));
        assert!(matches!(be.abilities[0].effects[..], [Effect::Exile]));
        assert!(matches!(be.abilities[0].targets, TargetSpec::Creature));

        // Requiting Hex — Instant, destroy target creature
        let rh = registry.create("Requiting Hex", id, owner).unwrap();
        assert!(rh.card_types.contains(&CardType::Instant));
        assert!(matches!(rh.abilities[0].effects[0], Effect::Destroy));
        assert!(matches!(rh.abilities[0].targets, TargetSpec::Creature));
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
