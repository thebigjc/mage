// Object filters for targeting and selection.
//
// Replaces Java's deep FilterPermanent / FilterCard / FilterObject / Predicate
// hierarchy with a composable, data-oriented predicate system.
//
// Filters are used in two main contexts:
//   1. Targeting: "target creature", "target nonland permanent", etc.
//   2. Selection: "creatures you control", "all artifacts", etc.
//
// Rather than Java's class hierarchy, we use an enum of predicate conditions
// that can be composed with And/Or/Not combinators.

use crate::card::CardData;
use crate::constants::{
    CardType, Color, ComparisonType, KeywordAbilities, SubType, SuperType, TargetController,
};
use crate::permanent::Permanent;
use crate::types::{ObjectId, PlayerId};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Predicate — a composable filter condition
// ---------------------------------------------------------------------------

/// A composable predicate for filtering game objects.
///
/// Predicates can match against permanents (on the battlefield) or cards
/// (in hand, graveyard, library, exile). They compose via And/Or/Not.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Predicate {
    // ── Type checks ─────────────────────────────────────────────────────
    /// Has a specific card type (Creature, Land, Artifact, etc.).
    HasCardType(CardType),
    /// Does NOT have a specific card type.
    NotCardType(CardType),
    /// Has a specific subtype (Human, Equipment, Forest, etc.).
    HasSubType(SubType),
    /// Has a specific supertype (Basic, Legendary, Snow, World).
    HasSuperType(SuperType),

    // ── Color checks ────────────────────────────────────────────────────
    /// Has a specific color.
    HasColor(Color),
    /// Is colorless (has no colors).
    IsColorless,
    /// Is multicolored (has 2+ colors).
    IsMulticolored,
    /// Is monocolored (has exactly 1 color).
    IsMonocolored,

    // ── Keyword checks ──────────────────────────────────────────────────
    /// Has a specific keyword ability.
    HasKeyword(KeywordAbilities),

    // ── P/T checks ──────────────────────────────────────────────────────
    /// Power comparison.
    PowerCompare(ComparisonType, i32),
    /// Toughness comparison.
    ToughnessCompare(ComparisonType, i32),

    // ── Mana value checks ───────────────────────────────────────────────
    /// Mana value comparison.
    ManaValueCompare(ComparisonType, i32),

    // ── Controller/owner checks ─────────────────────────────────────────
    /// Controlled by a specific relationship (You, Opponent, Any, etc.).
    Controller(TargetController),
    /// Owned by a specific player.
    OwnedBy(PlayerId),

    // ── Name checks ─────────────────────────────────────────────────────
    /// Has a specific name.
    NameIs(String),
    /// Name is different from the given string.
    NameIsNot(String),

    // ── Identity checks ─────────────────────────────────────────────────
    /// Is a specific object (by ObjectId).
    IsObject(ObjectId),
    /// Is not a specific object.
    IsNotObject(ObjectId),

    // ── Battlefield state checks ────────────────────────────────────────
    /// Is tapped.
    IsTapped,
    /// Is untapped.
    IsUntapped,
    /// Is a token.
    IsToken,
    /// Is not a token.
    IsNontoken,

    // ── Combinators ─────────────────────────────────────────────────────
    /// All child predicates must match.
    And(Vec<Predicate>),
    /// At least one child predicate must match.
    Or(Vec<Predicate>),
    /// The child predicate must NOT match.
    Not(Box<Predicate>),

    /// Always matches.
    All,
    /// Never matches.
    None,
}

impl Predicate {
    // ── Builder methods for common patterns ─────────────────────────────

    pub fn creature() -> Self {
        Predicate::HasCardType(CardType::Creature)
    }

    pub fn land() -> Self {
        Predicate::HasCardType(CardType::Land)
    }

    pub fn nonland() -> Self {
        Predicate::NotCardType(CardType::Land)
    }

    pub fn artifact() -> Self {
        Predicate::HasCardType(CardType::Artifact)
    }

    pub fn enchantment() -> Self {
        Predicate::HasCardType(CardType::Enchantment)
    }

    pub fn instant() -> Self {
        Predicate::HasCardType(CardType::Instant)
    }

    pub fn sorcery() -> Self {
        Predicate::HasCardType(CardType::Sorcery)
    }

    pub fn planeswalker() -> Self {
        Predicate::HasCardType(CardType::Planeswalker)
    }

    pub fn legendary() -> Self {
        Predicate::HasSuperType(SuperType::Legendary)
    }

    pub fn nonland_permanent() -> Self {
        Predicate::And(vec![
            Predicate::NotCardType(CardType::Land),
            Predicate::Or(vec![
                Predicate::HasCardType(CardType::Creature),
                Predicate::HasCardType(CardType::Artifact),
                Predicate::HasCardType(CardType::Enchantment),
                Predicate::HasCardType(CardType::Planeswalker),
            ]),
        ])
    }

    pub fn and(self, other: Predicate) -> Predicate {
        match self {
            Predicate::And(mut v) => {
                v.push(other);
                Predicate::And(v)
            }
            _ => Predicate::And(vec![self, other]),
        }
    }

    pub fn or(self, other: Predicate) -> Predicate {
        match self {
            Predicate::Or(mut v) => {
                v.push(other);
                Predicate::Or(v)
            }
            _ => Predicate::Or(vec![self, other]),
        }
    }

}

impl std::ops::Not for Predicate {
    type Output = Predicate;

    fn not(self) -> Predicate {
        Predicate::Not(Box::new(self))
    }
}

// ---------------------------------------------------------------------------
// Filter — wraps a predicate with a display name
// ---------------------------------------------------------------------------

/// A named filter combining a human-readable description with a predicate.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Filter {
    /// Human-readable description (e.g. "target creature", "nonland permanent").
    pub message: String,
    /// The predicate that must be satisfied.
    pub predicate: Predicate,
}

impl Filter {
    pub fn new(message: &str, predicate: Predicate) -> Self {
        Filter {
            message: message.to_string(),
            predicate,
        }
    }

    /// Match against a permanent on the battlefield.
    pub fn matches_permanent(&self, perm: &Permanent, you: PlayerId) -> bool {
        predicate_matches_permanent(&self.predicate, perm, you)
    }

    /// Match against a card (in hand, graveyard, etc.).
    pub fn matches_card(&self, card: &CardData, you: PlayerId) -> bool {
        predicate_matches_card(&self.predicate, card, you)
    }
}

// ---------------------------------------------------------------------------
// Common pre-built filters (static-like, analogous to Java's StaticFilters)
// ---------------------------------------------------------------------------

impl Filter {
    pub fn any_permanent() -> Self {
        Filter::new("permanent", Predicate::All)
    }

    pub fn any_creature() -> Self {
        Filter::new("creature", Predicate::creature())
    }

    pub fn any_nonland_permanent() -> Self {
        Filter::new("nonland permanent", Predicate::nonland_permanent())
    }

    pub fn creature_you_control() -> Self {
        Filter::new(
            "creature you control",
            Predicate::creature().and(Predicate::Controller(TargetController::You)),
        )
    }

    pub fn permanent_you_control() -> Self {
        Filter::new(
            "permanent you control",
            Predicate::Controller(TargetController::You),
        )
    }

    pub fn permanent_opponent_controls() -> Self {
        Filter::new(
            "permanent an opponent controls",
            Predicate::Controller(TargetController::Opponent),
        )
    }

    pub fn any_card() -> Self {
        Filter::new("card", Predicate::All)
    }

    pub fn creature_card() -> Self {
        Filter::new("creature card", Predicate::creature())
    }
}

// ---------------------------------------------------------------------------
// Predicate matching logic
// ---------------------------------------------------------------------------

/// Evaluate a predicate against a permanent.
pub fn predicate_matches_permanent(pred: &Predicate, perm: &Permanent, you: PlayerId) -> bool {
    match pred {
        Predicate::HasCardType(ct) => perm.has_card_type(*ct),
        Predicate::NotCardType(ct) => !perm.has_card_type(*ct),
        Predicate::HasSubType(st) => perm.has_subtype(st),
        Predicate::HasSuperType(st) => perm.has_supertype(*st),
        Predicate::HasColor(c) => perm.card.colors().contains(c),
        Predicate::IsColorless => perm.card.colors().is_empty(),
        Predicate::IsMulticolored => perm.card.colors().len() >= 2,
        Predicate::IsMonocolored => perm.card.colors().len() == 1,
        Predicate::HasKeyword(kw) => perm.has_keyword(*kw),
        Predicate::PowerCompare(cmp, val) => {
            perm.is_creature() && cmp.compare(perm.power(), *val)
        }
        Predicate::ToughnessCompare(cmp, val) => {
            perm.is_creature() && cmp.compare(perm.toughness(), *val)
        }
        Predicate::ManaValueCompare(cmp, val) => {
            cmp.compare(perm.card.mana_value() as i32, *val)
        }
        Predicate::Controller(tc) => match tc {
            TargetController::You => perm.controller == you,
            TargetController::Opponent | TargetController::NotYou => perm.controller != you,
            TargetController::Any => true,
            _ => true,
        },
        Predicate::OwnedBy(pid) => perm.owner() == *pid,
        Predicate::NameIs(name) => perm.name() == name,
        Predicate::NameIsNot(name) => perm.name() != name,
        Predicate::IsObject(id) => perm.id() == *id,
        Predicate::IsNotObject(id) => perm.id() != *id,
        Predicate::IsTapped => perm.tapped,
        Predicate::IsUntapped => !perm.tapped,
        Predicate::IsToken => false, // TODO: token tracking
        Predicate::IsNontoken => true, // TODO: token tracking
        Predicate::And(preds) => preds.iter().all(|p| predicate_matches_permanent(p, perm, you)),
        Predicate::Or(preds) => preds.iter().any(|p| predicate_matches_permanent(p, perm, you)),
        Predicate::Not(p) => !predicate_matches_permanent(p, perm, you),
        Predicate::All => true,
        Predicate::None => false,
    }
}

/// Evaluate a predicate against a card (not on the battlefield).
pub fn predicate_matches_card(pred: &Predicate, card: &CardData, _you: PlayerId) -> bool {
    match pred {
        Predicate::HasCardType(ct) => card.card_types.contains(ct),
        Predicate::NotCardType(ct) => !card.card_types.contains(ct),
        Predicate::HasSubType(st) => card.subtypes.contains(st),
        Predicate::HasSuperType(st) => card.supertypes.contains(st),
        Predicate::HasColor(c) => card.colors().contains(c),
        Predicate::IsColorless => card.colors().is_empty(),
        Predicate::IsMulticolored => card.colors().len() >= 2,
        Predicate::IsMonocolored => card.colors().len() == 1,
        Predicate::HasKeyword(kw) => card.keywords.contains(*kw),
        Predicate::PowerCompare(cmp, val) => {
            card.power.map_or(false, |p| cmp.compare(p, *val))
        }
        Predicate::ToughnessCompare(cmp, val) => {
            card.toughness.map_or(false, |t| cmp.compare(t, *val))
        }
        Predicate::ManaValueCompare(cmp, val) => {
            cmp.compare(card.mana_value() as i32, *val)
        }
        Predicate::Controller(_tc) => true, // cards don't have controllers
        Predicate::OwnedBy(pid) => card.owner == *pid,
        Predicate::NameIs(name) => card.name == *name,
        Predicate::NameIsNot(name) => card.name != *name,
        Predicate::IsObject(id) => card.id == *id,
        Predicate::IsNotObject(id) => card.id != *id,
        Predicate::IsTapped => false,
        Predicate::IsUntapped => true,
        Predicate::IsToken => false,
        Predicate::IsNontoken => true,
        Predicate::And(preds) => preds.iter().all(|p| predicate_matches_card(p, card, _you)),
        Predicate::Or(preds) => preds.iter().any(|p| predicate_matches_card(p, card, _you)),
        Predicate::Not(p) => !predicate_matches_card(p, card, _you),
        Predicate::All => true,
        Predicate::None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::CardData;
    use crate::constants::{CardType, Color, KeywordAbilities, SubType};
    use crate::mana::ManaCost;
    use crate::permanent::Permanent;
    use crate::types::{ObjectId, PlayerId};

    fn make_creature(name: &str, power: i32, toughness: i32, kw: KeywordAbilities) -> Permanent {
        let owner = PlayerId::new();
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(power);
        card.toughness = Some(toughness);
        card.keywords = kw;
        Permanent::new(card, owner)
    }

    fn make_land(name: &str, owner: PlayerId) -> Permanent {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Land];
        card.subtypes = vec![SubType::Forest];
        card.supertypes = vec![SuperType::Basic];
        Permanent::new(card, owner)
    }

    #[test]
    fn filter_creature() {
        let bear = make_creature("Bear", 2, 2, KeywordAbilities::empty());
        let you = bear.controller;
        let filter = Filter::any_creature();
        assert!(filter.matches_permanent(&bear, you));

        let land = make_land("Forest", you);
        assert!(!filter.matches_permanent(&land, you));
    }

    #[test]
    fn filter_nonland_permanent() {
        let bear = make_creature("Bear", 2, 2, KeywordAbilities::empty());
        let you = bear.controller;
        let filter = Filter::any_nonland_permanent();
        assert!(filter.matches_permanent(&bear, you));

        let land = make_land("Forest", you);
        assert!(!filter.matches_permanent(&land, you));
    }

    #[test]
    fn filter_controller() {
        let owner = PlayerId::new();
        let opponent = PlayerId::new();
        let bear = make_creature("Bear", 2, 2, KeywordAbilities::empty());
        let controller = bear.controller;

        let your_filter = Filter::creature_you_control();
        assert!(your_filter.matches_permanent(&bear, controller));
        assert!(!your_filter.matches_permanent(&bear, opponent));

        let opp_filter = Filter::permanent_opponent_controls();
        assert!(!opp_filter.matches_permanent(&bear, controller));
        assert!(opp_filter.matches_permanent(&bear, opponent));
        let _ = owner; // suppress unused warning
    }

    #[test]
    fn filter_keyword() {
        let flyer = make_creature("Bird", 1, 1, KeywordAbilities::FLYING);
        let you = flyer.controller;
        let pred = Predicate::creature().and(Predicate::HasKeyword(KeywordAbilities::FLYING));
        let filter = Filter::new("creature with flying", pred);
        assert!(filter.matches_permanent(&flyer, you));

        let bear = make_creature("Bear", 2, 2, KeywordAbilities::empty());
        assert!(!filter.matches_permanent(&bear, you));
    }

    #[test]
    fn filter_power_comparison() {
        let big = make_creature("Big", 5, 5, KeywordAbilities::empty());
        let you = big.controller;
        let pred = Predicate::PowerCompare(ComparisonType::GreaterOrEqual, 4);
        assert!(predicate_matches_permanent(&pred, &big, you));

        let small = make_creature("Small", 1, 1, KeywordAbilities::empty());
        assert!(!predicate_matches_permanent(&pred, &small, you));
    }

    #[test]
    fn filter_mana_value() {
        let owner = PlayerId::new();
        let mut card = CardData::new(ObjectId::new(), owner, "Expensive");
        card.card_types = vec![CardType::Creature];
        card.mana_cost = ManaCost::parse("{3}{R}{R}");
        card.power = Some(4);
        card.toughness = Some(4);
        let perm = Permanent::new(card, owner);

        let pred = Predicate::ManaValueCompare(ComparisonType::LessOrEqual, 3);
        assert!(!predicate_matches_permanent(&pred, &perm, owner));

        let pred2 = Predicate::ManaValueCompare(ComparisonType::Equal, 5);
        assert!(predicate_matches_permanent(&pred2, &perm, owner));
    }

    #[test]
    fn filter_card_data() {
        let owner = PlayerId::new();
        let mut card = CardData::new(ObjectId::new(), owner, "Lightning Bolt");
        card.card_types = vec![CardType::Instant];
        card.mana_cost = ManaCost::parse("{R}");

        let pred = Predicate::instant();
        assert!(predicate_matches_card(&pred, &card, owner));
        assert!(!predicate_matches_card(&Predicate::creature(), &card, owner));
    }

    #[test]
    fn filter_or_combinator() {
        let bear = make_creature("Bear", 2, 2, KeywordAbilities::empty());
        let you = bear.controller;

        let pred = Predicate::creature().or(Predicate::artifact());
        assert!(predicate_matches_permanent(&pred, &bear, you));

        let land = make_land("Forest", you);
        assert!(!predicate_matches_permanent(&pred, &land, you));
    }

    #[test]
    fn filter_not_combinator() {
        let bear = make_creature("Bear", 2, 2, KeywordAbilities::empty());
        let you = bear.controller;

        let pred = !Predicate::creature();
        assert!(!predicate_matches_permanent(&pred, &bear, you));

        let land = make_land("Forest", you);
        assert!(predicate_matches_permanent(&pred, &land, you));
    }

    #[test]
    fn filter_color() {
        let owner = PlayerId::new();
        let mut card = CardData::new(ObjectId::new(), owner, "Shock");
        card.card_types = vec![CardType::Instant];
        card.mana_cost = ManaCost::parse("{R}");

        assert!(predicate_matches_card(&Predicate::HasColor(Color::Red), &card, owner));
        assert!(!predicate_matches_card(&Predicate::HasColor(Color::Blue), &card, owner));
        assert!(!predicate_matches_card(&Predicate::IsColorless, &card, owner));
        assert!(!predicate_matches_card(&Predicate::IsMulticolored, &card, owner));
    }

    #[test]
    fn filter_tapped_untapped() {
        let mut bear = make_creature("Bear", 2, 2, KeywordAbilities::empty());
        let you = bear.controller;

        assert!(predicate_matches_permanent(&Predicate::IsUntapped, &bear, you));
        assert!(!predicate_matches_permanent(&Predicate::IsTapped, &bear, you));

        bear.tap();
        assert!(!predicate_matches_permanent(&Predicate::IsUntapped, &bear, you));
        assert!(predicate_matches_permanent(&Predicate::IsTapped, &bear, you));
    }
}
