// Permanent -- an on-battlefield game object.
// Ported from Mage/src/main/java/mage/game/permanent/PermanentImpl.java.

use crate::card::CardData;
use crate::constants::{CardType, KeywordAbilities, SubType, SuperType};
use crate::counters::{CounterType, Counters};
use crate::types::{ObjectId, PlayerId};
use serde::{Deserialize, Serialize};

/// A permanent on the battlefield. Contains the card data plus battlefield state.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Permanent {
    /// The underlying card data (types, costs, base P/T, etc.).
    pub card: CardData,
    /// Who currently controls this permanent.
    pub controller: PlayerId,
    /// Whether the permanent is tapped.
    pub tapped: bool,
    /// Whether the permanent has summoning sickness.
    /// Creatures with summoning sickness cannot attack or use {T} abilities.
    pub summoning_sick: bool,
    /// Damage currently marked on this permanent (creatures only).
    pub damage: u32,
    /// Counters on this permanent (+1/+1, loyalty, etc.).
    pub counters: Counters,
    /// If this permanent is attached to another (e.g. Aura, Equipment).
    pub attached_to: Option<ObjectId>,
    /// Objects attached to this permanent.
    pub attachments: Vec<ObjectId>,
    /// Whether the permanent is flipped (Kamigawa flip cards).
    pub flipped: bool,
    /// Whether the permanent is transformed (double-faced cards).
    pub transformed: bool,
    /// Whether the permanent is face-down (morph, manifest, etc.).
    pub face_down: bool,
    /// Zone change counter -- tracks how many times this object has changed zones.
    pub zone_change_count: u32,
    /// Additional keywords gained/lost from effects (applied on top of card.keywords).
    pub granted_keywords: KeywordAbilities,
    /// Keywords removed by effects.
    pub removed_keywords: KeywordAbilities,
    /// Original controller before a temporary control change (GainControlUntilEndOfTurn).
    /// Set when control is temporarily changed; reverted at cleanup step.
    pub original_controller: Option<PlayerId>,
    /// Creature type chosen via "As ~ enters, choose a creature type" effects.
    pub chosen_type: Option<SubType>,
    /// P/T boost from continuous effects (recalculated each time effects are applied).
    pub continuous_boost_power: i32,
    /// Toughness boost from continuous effects (recalculated each time effects are applied).
    pub continuous_boost_toughness: i32,
    /// Keywords granted by continuous effects from other permanents (static abilities).
    /// Distinct from `granted_keywords` which tracks one-shot until-end-of-turn effects.
    pub continuous_keywords: KeywordAbilities,
}

impl Permanent {
    /// Create a new permanent from card data entering the battlefield.
    pub fn new(card: CardData, controller: PlayerId) -> Self {
        Permanent {
            controller,
            tapped: false,
            summoning_sick: card.is_creature(),
            damage: 0,
            counters: Counters::new(),
            attached_to: None,
            attachments: Vec::new(),
            flipped: false,
            transformed: false,
            face_down: false,
            zone_change_count: 0,
            granted_keywords: KeywordAbilities::empty(),
            removed_keywords: KeywordAbilities::empty(),
            original_controller: None,
            chosen_type: None,
            continuous_boost_power: 0,
            continuous_boost_toughness: 0,
            continuous_keywords: KeywordAbilities::empty(),
            card,
        }
    }

    // ── Identity ───────────────────────────────────────────────────────

    /// The permanent's unique object ID.
    pub fn id(&self) -> ObjectId {
        self.card.id
    }

    /// The permanent's name.
    pub fn name(&self) -> &str {
        &self.card.name
    }

    /// The permanent's owner.
    pub fn owner(&self) -> PlayerId {
        self.card.owner
    }

    // ── Type checks ────────────────────────────────────────────────────

    pub fn is_creature(&self) -> bool {
        self.card.is_creature()
    }

    pub fn is_land(&self) -> bool {
        self.card.is_land()
    }

    pub fn is_artifact(&self) -> bool {
        self.card.card_types.contains(&CardType::Artifact)
    }

    pub fn is_enchantment(&self) -> bool {
        self.card.card_types.contains(&CardType::Enchantment)
    }

    pub fn is_planeswalker(&self) -> bool {
        self.card.card_types.contains(&CardType::Planeswalker)
    }

    pub fn has_card_type(&self, ct: CardType) -> bool {
        self.card.card_types.contains(&ct)
    }

    pub fn has_subtype(&self, st: &SubType) -> bool {
        // Changeling: has every creature type
        if self.has_keyword(KeywordAbilities::CHANGELING) && self.is_creature() {
            return true;
        }
        self.card.subtypes.contains(st)
    }

    pub fn has_supertype(&self, st: SuperType) -> bool {
        self.card.supertypes.contains(&st)
    }

    pub fn is_legendary(&self) -> bool {
        self.card.is_legendary()
    }

    // ── Keyword abilities ──────────────────────────────────────────────

    /// Current effective keyword abilities (base + granted + continuous - removed).
    pub fn keywords(&self) -> KeywordAbilities {
        (self.card.keywords | self.granted_keywords | self.continuous_keywords) & !self.removed_keywords
    }

    pub fn has_keyword(&self, kw: KeywordAbilities) -> bool {
        self.keywords().contains(kw)
    }

    pub fn has_flying(&self) -> bool {
        self.has_keyword(KeywordAbilities::FLYING)
    }

    pub fn has_haste(&self) -> bool {
        self.has_keyword(KeywordAbilities::HASTE)
    }

    pub fn has_defender(&self) -> bool {
        self.has_keyword(KeywordAbilities::DEFENDER)
    }

    pub fn has_reach(&self) -> bool {
        self.has_keyword(KeywordAbilities::REACH)
    }

    pub fn has_vigilance(&self) -> bool {
        self.has_keyword(KeywordAbilities::VIGILANCE)
    }

    pub fn has_first_strike(&self) -> bool {
        self.has_keyword(KeywordAbilities::FIRST_STRIKE)
    }

    pub fn has_double_strike(&self) -> bool {
        self.has_keyword(KeywordAbilities::DOUBLE_STRIKE)
    }

    pub fn has_trample(&self) -> bool {
        self.has_keyword(KeywordAbilities::TRAMPLE)
    }

    pub fn has_lifelink(&self) -> bool {
        self.has_keyword(KeywordAbilities::LIFELINK)
    }

    pub fn has_deathtouch(&self) -> bool {
        self.has_keyword(KeywordAbilities::DEATHTOUCH)
    }

    pub fn has_indestructible(&self) -> bool {
        self.has_keyword(KeywordAbilities::INDESTRUCTIBLE)
    }

    pub fn has_hexproof(&self) -> bool {
        self.has_keyword(KeywordAbilities::HEXPROOF)
    }

    pub fn has_flash(&self) -> bool {
        self.has_keyword(KeywordAbilities::FLASH)
    }

    pub fn has_menace(&self) -> bool {
        self.has_keyword(KeywordAbilities::MENACE)
    }

    // ── Power/Toughness ────────────────────────────────────────────────

    /// Get the current power, including counter and continuous effect modifications.
    pub fn power(&self) -> i32 {
        let base = self.card.power.unwrap_or(0);
        let (counter_p, _) = self.counters.pt_modification();
        base + counter_p + self.continuous_boost_power
    }

    /// Get the current toughness, including counter and continuous effect modifications.
    pub fn toughness(&self) -> i32 {
        let base = self.card.toughness.unwrap_or(0);
        let (_, counter_t) = self.counters.pt_modification();
        base + counter_t + self.continuous_boost_toughness
    }

    /// Remaining toughness after damage (used for SBA lethal damage check).
    pub fn remaining_toughness(&self) -> i32 {
        self.toughness() - self.damage as i32
    }

    /// Whether this creature has lethal damage marked on it.
    pub fn has_lethal_damage(&self) -> bool {
        self.is_creature() && self.remaining_toughness() <= 0
    }

    // ── Tap/Untap ──────────────────────────────────────────────────────

    /// Tap this permanent. Returns true if it was untapped (state changed).
    pub fn tap(&mut self) -> bool {
        if !self.tapped {
            self.tapped = true;
            true
        } else {
            false
        }
    }

    /// Untap this permanent. Returns true if it was tapped (state changed).
    pub fn untap(&mut self) -> bool {
        if self.tapped {
            self.tapped = false;
            true
        } else {
            false
        }
    }

    // ── Combat ─────────────────────────────────────────────────────────

    /// Whether this creature can attack (not tapped, no summoning sickness
    /// unless it has haste, and not a defender).
    pub fn can_attack(&self) -> bool {
        self.is_creature()
            && !self.tapped
            && !self.has_defender()
            && (!self.summoning_sick || self.has_haste())
    }

    /// Whether this creature can block.
    pub fn can_block(&self) -> bool {
        self.is_creature() && !self.tapped
    }

    // ── Damage ─────────────────────────────────────────────────────────

    /// Apply damage to this permanent. Returns the actual damage dealt.
    pub fn apply_damage(&mut self, amount: u32) -> u32 {
        if amount == 0 {
            return 0;
        }
        self.damage += amount;
        amount
    }

    /// Remove all damage from this permanent (happens during cleanup step).
    pub fn clear_damage(&mut self) {
        self.damage = 0;
    }

    // ── Counters ───────────────────────────────────────────────────────

    /// Add counters to this permanent.
    pub fn add_counters(&mut self, counter_type: CounterType, count: u32) {
        self.counters.add(counter_type, count);
    }

    /// Remove counters from this permanent. Returns actual removed count.
    pub fn remove_counters(&mut self, counter_type: &CounterType, count: u32) -> u32 {
        self.counters.remove(counter_type, count)
    }

    // ── Attachments ────────────────────────────────────────────────────

    /// Attach this permanent to another (e.g. equip, enchant).
    pub fn attach_to(&mut self, target: ObjectId) {
        self.attached_to = Some(target);
    }

    /// Detach this permanent from whatever it's attached to.
    pub fn detach(&mut self) {
        self.attached_to = None;
    }

    /// Add an attachment to this permanent.
    pub fn add_attachment(&mut self, attachment: ObjectId) {
        if !self.attachments.contains(&attachment) {
            self.attachments.push(attachment);
        }
    }

    /// Remove an attachment from this permanent.
    pub fn remove_attachment(&mut self, attachment: ObjectId) {
        self.attachments.retain(|&id| id != attachment);
    }

    // ── Summoning sickness ─────────────────────────────────────────────

    /// Remove summoning sickness (called at the start of the controller's turn
    /// for permanents they've controlled since the start of the turn).
    pub fn remove_summoning_sickness(&mut self) {
        self.summoning_sick = false;
    }

    // ── Equipment/Aura checks ──────────────────────────────────────────

    pub fn is_equipment(&self) -> bool {
        self.has_subtype(&SubType::Equipment)
    }

    pub fn is_aura(&self) -> bool {
        self.has_subtype(&SubType::Aura)
    }

    pub fn is_vehicle(&self) -> bool {
        self.has_subtype(&SubType::Vehicle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities};

    fn make_creature(name: &str, power: i32, toughness: i32, keywords: KeywordAbilities) -> Permanent {
        let owner = PlayerId::new();
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(power);
        card.toughness = Some(toughness);
        card.keywords = keywords;
        Permanent::new(card, owner)
    }

    #[test]
    fn creature_basics() {
        let perm = make_creature("Grizzly Bears", 2, 2, KeywordAbilities::empty());
        assert!(perm.is_creature());
        assert_eq!(perm.power(), 2);
        assert_eq!(perm.toughness(), 2);
        assert!(perm.summoning_sick);
        assert!(!perm.can_attack());
    }

    #[test]
    fn haste_ignores_summoning_sickness() {
        let perm = make_creature("Goblin Guide", 2, 2, KeywordAbilities::HASTE);
        assert!(perm.summoning_sick);
        assert!(perm.can_attack()); // haste lets it attack
    }

    #[test]
    fn tap_untap() {
        let mut perm = make_creature("Bear", 2, 2, KeywordAbilities::empty());
        assert!(!perm.tapped);
        assert!(perm.tap());
        assert!(perm.tapped);
        assert!(!perm.tap()); // already tapped
        assert!(perm.untap());
        assert!(!perm.tapped);
    }

    #[test]
    fn damage_and_lethal() {
        let mut perm = make_creature("Bear", 2, 2, KeywordAbilities::empty());
        assert_eq!(perm.remaining_toughness(), 2);
        perm.apply_damage(1);
        assert_eq!(perm.remaining_toughness(), 1);
        assert!(!perm.has_lethal_damage());
        perm.apply_damage(1);
        assert!(perm.has_lethal_damage());
        perm.clear_damage();
        assert!(!perm.has_lethal_damage());
    }

    #[test]
    fn counters_modify_pt() {
        let mut perm = make_creature("Bear", 2, 2, KeywordAbilities::empty());
        perm.add_counters(CounterType::P1P1, 2);
        assert_eq!(perm.power(), 4);
        assert_eq!(perm.toughness(), 4);
        perm.add_counters(CounterType::M1M1, 1);
        assert_eq!(perm.power(), 3);
        assert_eq!(perm.toughness(), 3);
    }

    #[test]
    fn keyword_grants() {
        let mut perm = make_creature("Bear", 2, 2, KeywordAbilities::empty());
        assert!(!perm.has_flying());
        perm.granted_keywords |= KeywordAbilities::FLYING;
        assert!(perm.has_flying());
        perm.removed_keywords |= KeywordAbilities::FLYING;
        assert!(!perm.has_flying());
    }

    #[test]
    fn defender_cannot_attack() {
        let perm = make_creature("Wall", 0, 4, KeywordAbilities::DEFENDER);
        let _ = perm.clone(); // ensure Clone works
        assert!(!perm.can_attack());
        assert!(perm.can_block());
    }

    #[test]
    fn changeling_has_all_creature_types() {
        let perm = make_creature("Shapeshifter", 2, 2, KeywordAbilities::CHANGELING);
        // Changeling has every creature type
        assert!(perm.has_subtype(&SubType::Elf));
        assert!(perm.has_subtype(&SubType::Goblin));
        assert!(perm.has_subtype(&SubType::Human));
        assert!(perm.has_subtype(&SubType::Spirit));
        assert!(perm.has_subtype(&SubType::Dragon));
        // Even custom types
        assert!(perm.has_subtype(&SubType::Custom("Weird".into())));
    }

    #[test]
    fn non_changeling_only_has_listed_subtypes() {
        let owner = PlayerId::new();
        let mut card = CardData::new(ObjectId::new(), owner, "Elf");
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![SubType::Elf];
        card.power = Some(1);
        card.toughness = Some(1);
        let perm = Permanent::new(card, owner);
        assert!(perm.has_subtype(&SubType::Elf));
        assert!(!perm.has_subtype(&SubType::Goblin));
    }
}
