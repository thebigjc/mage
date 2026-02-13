// Zone containers — library, hand, graveyard, exile, battlefield, stack, command.
//
// Each zone is a typed container holding cards (by ObjectId) or permanents.
// The Battlefield is special: it holds Permanent structs, not just card IDs.
// The Stack holds StackItem entries (spells and abilities being resolved).
//
// Ported from various Java classes:
//   - mage.cards.CardsImpl (hand, graveyard)
//   - mage.players.Library
//   - mage.game.ExileZone / Exile
//   - mage.game.permanent.Battlefield
//   - mage.game.stack.SpellStack

use crate::card::CardData;
use crate::permanent::Permanent;
use crate::types::{AbilityId, ObjectId, PlayerId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Library (deck zone, hidden, ordered)
// ---------------------------------------------------------------------------

/// The library (deck) zone. Cards are ordered; index 0 is the top.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Library {
    cards: Vec<ObjectId>,
}

impl Library {
    pub fn new() -> Self {
        Library { cards: Vec::new() }
    }

    /// Create a library from a list of card IDs (top of library = index 0).
    pub fn from_cards(cards: Vec<ObjectId>) -> Self {
        Library { cards }
    }

    /// Draw the top card. Returns `None` if the library is empty.
    pub fn draw(&mut self) -> Option<ObjectId> {
        if self.cards.is_empty() {
            None
        } else {
            Some(self.cards.remove(0))
        }
    }

    /// Look at the top N cards without removing them.
    pub fn peek(&self, n: usize) -> &[ObjectId] {
        let end = n.min(self.cards.len());
        &self.cards[..end]
    }

    /// Put a card on top of the library.
    pub fn put_on_top(&mut self, card_id: ObjectId) {
        self.cards.insert(0, card_id);
    }

    /// Put a card on the bottom of the library.
    pub fn put_on_bottom(&mut self, card_id: ObjectId) {
        self.cards.push(card_id);
    }

    /// Put a card at a specific position (0 = top).
    pub fn put_at(&mut self, card_id: ObjectId, position: usize) {
        let pos = position.min(self.cards.len());
        self.cards.insert(pos, card_id);
    }

    /// Shuffle the library using the provided RNG.
    pub fn shuffle<R: rand::Rng>(&mut self, rng: &mut R) {
        use rand::seq::SliceRandom;
        self.cards.shuffle(rng);
    }

    /// Remove a specific card from the library (e.g. tutoring).
    pub fn remove(&mut self, card_id: ObjectId) -> bool {
        if let Some(pos) = self.cards.iter().position(|&id| id == card_id) {
            self.cards.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Iterate over card IDs (top to bottom).
    pub fn iter(&self) -> impl Iterator<Item = &ObjectId> {
        self.cards.iter()
    }

    pub fn contains(&self, card_id: ObjectId) -> bool {
        self.cards.contains(&card_id)
    }
}

impl Default for Library {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Hand (hidden zone, unordered for game purposes)
// ---------------------------------------------------------------------------

/// A player's hand. Unordered set of card IDs.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hand {
    cards: Vec<ObjectId>,
}

impl Hand {
    pub fn new() -> Self {
        Hand { cards: Vec::new() }
    }

    pub fn add(&mut self, card_id: ObjectId) {
        self.cards.push(card_id);
    }

    pub fn remove(&mut self, card_id: ObjectId) -> bool {
        if let Some(pos) = self.cards.iter().position(|&id| id == card_id) {
            self.cards.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn contains(&self, card_id: ObjectId) -> bool {
        self.cards.contains(&card_id)
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &ObjectId> {
        self.cards.iter()
    }

    pub fn as_slice(&self) -> &[ObjectId] {
        &self.cards
    }

    pub fn clear(&mut self) {
        self.cards.clear();
    }
}

impl Default for Hand {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Graveyard (public zone, ordered — most recent on top)
// ---------------------------------------------------------------------------

/// A player's graveyard. Ordered: index 0 is the top (most recently added).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Graveyard {
    cards: Vec<ObjectId>,
}

impl Graveyard {
    pub fn new() -> Self {
        Graveyard { cards: Vec::new() }
    }

    /// Add a card to the top of the graveyard (most recent).
    pub fn add(&mut self, card_id: ObjectId) {
        self.cards.insert(0, card_id);
    }

    pub fn remove(&mut self, card_id: ObjectId) -> bool {
        if let Some(pos) = self.cards.iter().position(|&id| id == card_id) {
            self.cards.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn contains(&self, card_id: ObjectId) -> bool {
        self.cards.contains(&card_id)
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Iterate from top to bottom (most recent first).
    pub fn iter(&self) -> impl Iterator<Item = &ObjectId> {
        self.cards.iter()
    }

    pub fn as_slice(&self) -> &[ObjectId] {
        &self.cards
    }

    pub fn clear(&mut self) {
        self.cards.clear();
    }
}

impl Default for Graveyard {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Exile (public zone, may have named sub-zones)
// ---------------------------------------------------------------------------

/// An exile sub-zone (e.g. "exiled with Oblivion Ring").
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExileZone {
    /// The source that created this exile zone.
    pub source_id: ObjectId,
    /// Human-readable name.
    pub name: String,
    /// Cards in this exile zone.
    pub cards: Vec<ObjectId>,
}

/// The exile zone. Contains a default zone and named sub-zones.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Exile {
    /// Cards exiled without a specific zone (the "main" exile).
    main: Vec<ObjectId>,
    /// Named exile zones (keyed by source object ID).
    zones: HashMap<ObjectId, ExileZone>,
}

impl Exile {
    pub fn new() -> Self {
        Exile {
            main: Vec::new(),
            zones: HashMap::new(),
        }
    }

    /// Exile a card to the main exile zone.
    pub fn exile(&mut self, card_id: ObjectId) {
        self.main.push(card_id);
    }

    /// Exile a card to a named zone (e.g. for "exile until this leaves the battlefield").
    pub fn exile_to_zone(&mut self, card_id: ObjectId, source_id: ObjectId, zone_name: &str) {
        let zone = self.zones.entry(source_id).or_insert_with(|| ExileZone {
            source_id,
            name: zone_name.to_string(),
            cards: Vec::new(),
        });
        zone.cards.push(card_id);
    }

    /// Remove a card from exile (from any zone). Returns true if found.
    pub fn remove(&mut self, card_id: ObjectId) -> bool {
        if let Some(pos) = self.main.iter().position(|&id| id == card_id) {
            self.main.remove(pos);
            return true;
        }
        for zone in self.zones.values_mut() {
            if let Some(pos) = zone.cards.iter().position(|&id| id == card_id) {
                zone.cards.remove(pos);
                return true;
            }
        }
        false
    }

    /// Get all cards in a named exile zone.
    pub fn get_zone(&self, source_id: ObjectId) -> Option<&ExileZone> {
        self.zones.get(&source_id)
    }

    /// Check if a card is in exile (any zone).
    pub fn contains(&self, card_id: ObjectId) -> bool {
        self.main.contains(&card_id)
            || self.zones.values().any(|z| z.cards.contains(&card_id))
    }

    /// Total number of exiled cards.
    pub fn len(&self) -> usize {
        self.main.len() + self.zones.values().map(|z| z.cards.len()).sum::<usize>()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterate over all exiled card IDs (main zone + all named zones).
    pub fn iter_all(&self) -> impl Iterator<Item = &ObjectId> {
        self.main.iter().chain(self.zones.values().flat_map(|z| z.cards.iter()))
    }
}

impl Default for Exile {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Battlefield (public zone, holds Permanent structs)
// ---------------------------------------------------------------------------

/// The battlefield zone. Holds all permanents currently in play.
///
/// Permanents are stored by ObjectId for O(1) lookup. The insertion order
/// is tracked for timestamp ordering (relevant for continuous effects).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Battlefield {
    permanents: HashMap<ObjectId, Permanent>,
    /// Insertion order for timestamp tracking.
    order: Vec<ObjectId>,
}

impl Battlefield {
    pub fn new() -> Self {
        Battlefield {
            permanents: HashMap::new(),
            order: Vec::new(),
        }
    }

    /// Add a permanent to the battlefield.
    pub fn add(&mut self, permanent: Permanent) {
        let id = permanent.id();
        self.order.push(id);
        self.permanents.insert(id, permanent);
    }

    /// Remove a permanent from the battlefield. Returns it if found.
    pub fn remove(&mut self, id: ObjectId) -> Option<Permanent> {
        self.order.retain(|&oid| oid != id);
        self.permanents.remove(&id)
    }

    /// Get a reference to a permanent by ID.
    pub fn get(&self, id: ObjectId) -> Option<&Permanent> {
        self.permanents.get(&id)
    }

    /// Get a mutable reference to a permanent by ID.
    pub fn get_mut(&mut self, id: ObjectId) -> Option<&mut Permanent> {
        self.permanents.get_mut(&id)
    }

    /// Check if a permanent is on the battlefield.
    pub fn contains(&self, id: ObjectId) -> bool {
        self.permanents.contains_key(&id)
    }

    /// Number of permanents on the battlefield.
    pub fn len(&self) -> usize {
        self.permanents.len()
    }

    pub fn is_empty(&self) -> bool {
        self.permanents.is_empty()
    }

    /// Iterate over all permanents in timestamp order.
    pub fn iter(&self) -> impl Iterator<Item = &Permanent> {
        self.order.iter().filter_map(|id| self.permanents.get(id))
    }

    /// Iterate over all permanents controlled by a specific player.
    pub fn controlled_by(&self, player_id: PlayerId) -> impl Iterator<Item = &Permanent> {
        self.permanents
            .values()
            .filter(move |p| p.controller == player_id)
    }

    /// Get all permanent IDs.
    pub fn ids(&self) -> impl Iterator<Item = &ObjectId> {
        self.order.iter()
    }

    /// Mutable iterator over all permanents.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Permanent> {
        self.permanents.values_mut()
    }

    /// Count permanents matching a predicate.
    pub fn count_matching<F: Fn(&Permanent) -> bool>(&self, predicate: F) -> usize {
        self.permanents.values().filter(|p| predicate(p)).count()
    }

    /// Clear the battlefield.
    pub fn clear(&mut self) {
        self.permanents.clear();
        self.order.clear();
    }
}

impl Default for Battlefield {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Stack (public zone, LIFO)
// ---------------------------------------------------------------------------

/// An item on the stack (a spell or ability waiting to resolve).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StackItem {
    /// Unique ID for this stack object.
    pub id: ObjectId,
    /// The kind of stack object (spell or ability).
    pub kind: StackItemKind,
    /// Who controls this stack object.
    pub controller: PlayerId,
    /// Targets chosen for this spell/ability.
    pub targets: Vec<ObjectId>,
    /// Whether this item has been countered.
    pub countered: bool,
}

/// What kind of object is on the stack.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StackItemKind {
    /// A spell (card being cast).
    Spell {
        /// The card data for the spell.
        card: CardData,
    },
    /// An activated or triggered ability.
    Ability {
        /// The source permanent or card.
        source_id: ObjectId,
        /// The ability's unique ID.
        ability_id: AbilityId,
        /// Human-readable description.
        description: String,
    },
}

/// The stack zone. LIFO — the last item added resolves first.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stack {
    items: Vec<StackItem>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    /// Push a new item onto the stack (on top).
    pub fn push(&mut self, item: StackItem) {
        self.items.push(item);
    }

    /// Pop the top item off the stack (for resolution).
    pub fn pop(&mut self) -> Option<StackItem> {
        self.items.pop()
    }

    /// Peek at the top item.
    pub fn top(&self) -> Option<&StackItem> {
        self.items.last()
    }

    /// Get a specific stack item by ID.
    pub fn get(&self, id: ObjectId) -> Option<&StackItem> {
        self.items.iter().find(|item| item.id == id)
    }

    /// Remove a specific item from the stack (e.g. when countered).
    pub fn remove(&mut self, id: ObjectId) -> Option<StackItem> {
        if let Some(pos) = self.items.iter().position(|item| item.id == id) {
            Some(self.items.remove(pos))
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Iterate from top to bottom (resolution order).
    pub fn iter(&self) -> impl Iterator<Item = &StackItem> {
        self.items.iter().rev()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Command zone
// ---------------------------------------------------------------------------

/// The command zone. Holds commanders, emblems, and other command zone objects.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandZone {
    /// Cards in the command zone (commanders, companions, etc.).
    pub cards: Vec<ObjectId>,
}

impl CommandZone {
    pub fn new() -> Self {
        CommandZone { cards: Vec::new() }
    }

    pub fn add(&mut self, card_id: ObjectId) {
        self.cards.push(card_id);
    }

    pub fn remove(&mut self, card_id: ObjectId) -> bool {
        if let Some(pos) = self.cards.iter().position(|&id| id == card_id) {
            self.cards.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn contains(&self, card_id: ObjectId) -> bool {
        self.cards.contains(&card_id)
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

impl Default for CommandZone {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// CardStore — stores actual card data by ObjectId
// ---------------------------------------------------------------------------

/// Central storage for all card data in the game.
///
/// Cards are stored here regardless of which zone they're in. Zones only
/// track ObjectIds. This makes zone transitions simple (just move the ID)
/// and allows looking up card data from any zone.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CardStore {
    cards: HashMap<ObjectId, CardData>,
}

impl CardStore {
    pub fn new() -> Self {
        CardStore {
            cards: HashMap::new(),
        }
    }

    pub fn insert(&mut self, card: CardData) {
        self.cards.insert(card.id, card);
    }

    pub fn get(&self, id: ObjectId) -> Option<&CardData> {
        self.cards.get(&id)
    }

    pub fn get_mut(&mut self, id: ObjectId) -> Option<&mut CardData> {
        self.cards.get_mut(&id)
    }

    pub fn remove(&mut self, id: ObjectId) -> Option<CardData> {
        self.cards.remove(&id)
    }

    pub fn contains(&self, id: ObjectId) -> bool {
        self.cards.contains_key(&id)
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ObjectId, &CardData)> {
        self.cards.iter()
    }
}

impl Default for CardStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities};

    fn make_id() -> ObjectId {
        ObjectId::new()
    }

    fn make_player() -> PlayerId {
        PlayerId::new()
    }

    // -- Library tests --

    #[test]
    fn library_draw() {
        let ids: Vec<ObjectId> = (0..5).map(|_| make_id()).collect();
        let mut lib = Library::from_cards(ids.clone());
        assert_eq!(lib.len(), 5);
        assert_eq!(lib.draw(), Some(ids[0]));
        assert_eq!(lib.len(), 4);
        assert_eq!(lib.draw(), Some(ids[1]));
    }

    #[test]
    fn library_put_on_top_and_bottom() {
        let a = make_id();
        let b = make_id();
        let mut lib = Library::new();
        lib.put_on_bottom(a);
        lib.put_on_top(b);
        assert_eq!(lib.draw(), Some(b));
        assert_eq!(lib.draw(), Some(a));
    }

    #[test]
    fn library_peek() {
        let ids: Vec<ObjectId> = (0..3).map(|_| make_id()).collect();
        let lib = Library::from_cards(ids.clone());
        assert_eq!(lib.peek(2), &ids[..2]);
        assert_eq!(lib.peek(10), &ids[..]); // peek more than available
    }

    // -- Hand tests --

    #[test]
    fn hand_add_remove() {
        let mut hand = Hand::new();
        let a = make_id();
        let b = make_id();
        hand.add(a);
        hand.add(b);
        assert_eq!(hand.len(), 2);
        assert!(hand.remove(a));
        assert_eq!(hand.len(), 1);
        assert!(!hand.remove(a)); // already removed
    }

    // -- Graveyard tests --

    #[test]
    fn graveyard_ordering() {
        let mut gy = Graveyard::new();
        let a = make_id();
        let b = make_id();
        gy.add(a);
        gy.add(b);
        // Most recent (b) should be first
        let cards: Vec<&ObjectId> = gy.iter().collect();
        assert_eq!(*cards[0], b);
        assert_eq!(*cards[1], a);
    }

    // -- Exile tests --

    #[test]
    fn exile_zones() {
        let mut exile = Exile::new();
        let card_a = make_id();
        let card_b = make_id();
        let source = make_id();

        exile.exile(card_a);
        exile.exile_to_zone(card_b, source, "Exiled with Oblivion Ring");

        assert!(exile.contains(card_a));
        assert!(exile.contains(card_b));
        assert_eq!(exile.len(), 2);

        let zone = exile.get_zone(source).unwrap();
        assert_eq!(zone.cards.len(), 1);
        assert_eq!(zone.cards[0], card_b);

        assert!(exile.remove(card_b));
        assert!(!exile.contains(card_b));
    }

    // -- Battlefield tests --

    #[test]
    fn battlefield_add_remove() {
        let mut bf = Battlefield::new();
        let owner = make_player();
        let mut card = CardData::new(make_id(), owner, "Bear");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        card.keywords = KeywordAbilities::empty();

        let perm = Permanent::new(card.clone(), owner);
        let id = perm.id();
        bf.add(perm);

        assert!(bf.contains(id));
        assert_eq!(bf.len(), 1);

        let perm_ref = bf.get(id).unwrap();
        assert_eq!(perm_ref.power(), 2);

        let removed = bf.remove(id).unwrap();
        assert_eq!(removed.name(), "Bear");
        assert!(bf.is_empty());
    }

    #[test]
    fn battlefield_controlled_by() {
        let mut bf = Battlefield::new();
        let p1 = make_player();
        let p2 = make_player();

        let mut card1 = CardData::new(make_id(), p1, "Bear");
        card1.card_types = vec![CardType::Creature];
        card1.power = Some(2);
        card1.toughness = Some(2);
        let mut card2 = CardData::new(make_id(), p2, "Bird");
        card2.card_types = vec![CardType::Creature];
        card2.power = Some(1);
        card2.toughness = Some(1);

        bf.add(Permanent::new(card1, p1));
        bf.add(Permanent::new(card2, p2));

        assert_eq!(bf.controlled_by(p1).count(), 1);
        assert_eq!(bf.controlled_by(p2).count(), 1);
    }

    // -- Stack tests --

    #[test]
    fn stack_lifo() {
        let mut stack = Stack::new();
        let id1 = make_id();
        let id2 = make_id();
        let p = make_player();

        let card1 = CardData::new(id1, p, "Lightning Bolt");
        stack.push(StackItem {
            id: id1,
            kind: StackItemKind::Spell { card: card1 },
            controller: p,
            targets: vec![],
            countered: false,
        });

        let card2 = CardData::new(id2, p, "Counterspell");
        stack.push(StackItem {
            id: id2,
            kind: StackItemKind::Spell { card: card2 },
            controller: p,
            targets: vec![],
            countered: false,
        });

        assert_eq!(stack.len(), 2);
        let top = stack.pop().unwrap();
        assert_eq!(top.id, id2); // Counterspell on top
        let next = stack.pop().unwrap();
        assert_eq!(next.id, id1); // Lightning Bolt underneath
    }
}
