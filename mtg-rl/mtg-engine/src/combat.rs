// Combat system — declare attackers, declare blockers, and combat damage.
//
// Implements the full MTG combat phase:
// 1. Beginning of combat (triggers)
// 2. Declare attackers (tap, validate legal attackers)
// 3. Declare blockers (validate legal blockers, blocking restrictions)
// 4. Combat damage (first strike, regular, trample, deathtouch)
// 5. End of combat (cleanup)
//
// Ported from mage.game.combat.Combat.java.

use crate::constants::KeywordAbilities;
use crate::permanent::Permanent;
use crate::types::{ObjectId, PlayerId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Combat group — one attacker and its blockers
// ---------------------------------------------------------------------------

/// A combat group: one attacker and its blockers.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CombatGroup {
    /// The attacking creature's ObjectId.
    pub attacker_id: ObjectId,
    /// What the attacker is attacking (player or planeswalker).
    pub defending_id: ObjectId,
    /// Whether the defender is a player (true) or planeswalker (false).
    pub defending_player: bool,
    /// Blockers assigned to this attacker, in damage assignment order.
    pub blockers: Vec<ObjectId>,
    /// Whether this attacker is blocked (even if all blockers are removed).
    pub blocked: bool,
    /// Damage assignment: how much damage each blocker receives.
    pub damage_to_blockers: HashMap<ObjectId, u32>,
    /// Damage that overflows to the defending player (trample).
    pub trample_damage: u32,
}

impl CombatGroup {
    pub fn new(attacker_id: ObjectId, defending_id: ObjectId, defending_player: bool) -> Self {
        CombatGroup {
            attacker_id,
            defending_id,
            defending_player,
            blockers: Vec::new(),
            blocked: false,
            damage_to_blockers: HashMap::new(),
            trample_damage: 0,
        }
    }

    /// Add a blocker to this combat group.
    pub fn add_blocker(&mut self, blocker_id: ObjectId) {
        if !self.blockers.contains(&blocker_id) {
            self.blockers.push(blocker_id);
            self.blocked = true;
        }
    }

    /// Whether this attacker has any blockers.
    pub fn is_blocked(&self) -> bool {
        self.blocked
    }

    /// Whether this attacker is unblocked.
    pub fn is_unblocked(&self) -> bool {
        !self.blocked
    }
}

// ---------------------------------------------------------------------------
// Combat state
// ---------------------------------------------------------------------------

/// The complete combat state for a combat phase.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CombatState {
    /// The attacking player.
    pub attacking_player: Option<PlayerId>,
    /// All combat groups (one per attacker).
    pub groups: Vec<CombatGroup>,
    /// Set of all attacking creature IDs (for quick lookup).
    pub attackers: Vec<ObjectId>,
    /// Map of blocker ID -> which attacker it's blocking.
    pub blocker_to_attacker: HashMap<ObjectId, ObjectId>,
    /// Whether first strike damage has been dealt this combat.
    pub first_strike_dealt: bool,
    /// Whether regular combat damage has been dealt.
    pub regular_damage_dealt: bool,
}

impl CombatState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Clear all combat state (at end of combat phase).
    pub fn clear(&mut self) {
        self.attacking_player = None;
        self.groups.clear();
        self.attackers.clear();
        self.blocker_to_attacker.clear();
        self.first_strike_dealt = false;
        self.regular_damage_dealt = false;
    }

    /// Check if there are any attackers.
    pub fn has_attackers(&self) -> bool {
        !self.attackers.is_empty()
    }

    /// Declare an attacker. Adds a combat group for this creature.
    pub fn declare_attacker(
        &mut self,
        attacker_id: ObjectId,
        defending_id: ObjectId,
        defending_player: bool,
    ) {
        self.attackers.push(attacker_id);
        self.groups.push(CombatGroup::new(attacker_id, defending_id, defending_player));
    }

    /// Declare a blocker. Assigns it to the combat group for the attacker.
    pub fn declare_blocker(&mut self, blocker_id: ObjectId, attacker_id: ObjectId) {
        self.blocker_to_attacker.insert(blocker_id, attacker_id);
        for group in &mut self.groups {
            if group.attacker_id == attacker_id {
                group.add_blocker(blocker_id);
                return;
            }
        }
    }

    /// Get the combat group for a specific attacker.
    pub fn group_for_attacker(&self, attacker_id: ObjectId) -> Option<&CombatGroup> {
        self.groups.iter().find(|g| g.attacker_id == attacker_id)
    }

    /// Get a mutable reference to the combat group for a specific attacker.
    pub fn group_for_attacker_mut(&mut self, attacker_id: ObjectId) -> Option<&mut CombatGroup> {
        self.groups.iter_mut().find(|g| g.attacker_id == attacker_id)
    }

    /// Check if a creature is currently attacking.
    pub fn is_attacking(&self, creature_id: ObjectId) -> bool {
        self.attackers.contains(&creature_id)
    }

    /// Check if a creature is currently blocking.
    pub fn is_blocking(&self, creature_id: ObjectId) -> bool {
        self.blocker_to_attacker.contains_key(&creature_id)
    }

    /// Get all defending players/planeswalkers being attacked.
    pub fn defenders(&self) -> Vec<ObjectId> {
        let mut defenders: Vec<ObjectId> = self.groups.iter().map(|g| g.defending_id).collect();
        defenders.sort();
        defenders.dedup();
        defenders
    }

    /// Whether any creature has first/double strike (determines if we need
    /// the first strike damage step).
    pub fn has_first_strikers(&self, get_perm: &dyn Fn(ObjectId) -> Option<KeywordAbilities>) -> bool {
        for &id in &self.attackers {
            if let Some(kw) = get_perm(id) {
                if kw.contains(KeywordAbilities::FIRST_STRIKE)
                    || kw.contains(KeywordAbilities::DOUBLE_STRIKE)
                {
                    return true;
                }
            }
        }
        for &id in self.blocker_to_attacker.keys() {
            if let Some(kw) = get_perm(id) {
                if kw.contains(KeywordAbilities::FIRST_STRIKE)
                    || kw.contains(KeywordAbilities::DOUBLE_STRIKE)
                {
                    return true;
                }
            }
        }
        false
    }
}

// ---------------------------------------------------------------------------
// Combat validation functions
// ---------------------------------------------------------------------------

/// Check if a creature can legally attack.
pub fn can_attack(perm: &Permanent) -> bool {
    perm.can_attack()
}

/// Check if a creature can legally block a specific attacker.
pub fn can_block(blocker: &Permanent, attacker: &Permanent) -> bool {
    if !blocker.can_block() {
        return false;
    }

    // Flying: can only be blocked by creatures with flying or reach
    if attacker.has_flying() && !blocker.has_flying() && !blocker.has_reach() {
        return false;
    }

    // TODO: Add more blocking restrictions (menace, intimidate, fear, etc.)

    true
}

/// Check if blocking assignments satisfy menace (attacker with menace must
/// be blocked by 2+ creatures).
pub fn satisfies_menace(
    attacker: &Permanent,
    blocker_count: usize,
) -> bool {
    if attacker.has_menace() && blocker_count > 0 && blocker_count < 2 {
        return false;
    }
    true
}

// ---------------------------------------------------------------------------
// Combat damage assignment
// ---------------------------------------------------------------------------

/// Compute combat damage for one combat group.
///
/// Returns a list of (target_id, damage_amount, is_player) tuples.
pub fn assign_combat_damage(
    group: &CombatGroup,
    attacker: &Permanent,
    blockers: &[(ObjectId, &Permanent)],
    is_first_strike_step: bool,
) -> Vec<(ObjectId, u32, bool)> {
    let mut results = Vec::new();

    let has_first_strike = attacker.has_first_strike() || attacker.has_double_strike();
    let has_regular = !attacker.has_first_strike() || attacker.has_double_strike();

    // Determine if this attacker deals damage in this step
    let deals_damage = if is_first_strike_step {
        has_first_strike
    } else {
        has_regular
    };

    if !deals_damage {
        return results;
    }

    let power = attacker.power().max(0) as u32;
    if power == 0 {
        return results;
    }

    if group.is_unblocked() {
        // Unblocked: all damage goes to the defender
        results.push((group.defending_id, power, group.defending_player));
    } else if blockers.is_empty() {
        // Blocked but all blockers removed: damage is "blocked" (dealt to nothing)
        // unless the attacker has trample
        if attacker.has_trample() && group.defending_player {
            results.push((group.defending_id, power, true));
        }
    } else {
        // Blocked: assign damage to blockers in order
        let has_deathtouch = attacker.has_deathtouch();
        let has_trample = attacker.has_trample();
        let mut remaining_damage = power;

        for &(blocker_id, blocker_perm) in blockers {
            if remaining_damage == 0 {
                break;
            }

            let lethal = if has_deathtouch {
                // With deathtouch, 1 damage is lethal
                1u32.saturating_sub(blocker_perm.damage)
            } else {
                let toughness = blocker_perm.toughness().max(0) as u32;
                toughness.saturating_sub(blocker_perm.damage)
            };

            let damage = remaining_damage.min(lethal);
            if damage > 0 {
                results.push((blocker_id, damage, false));
                remaining_damage -= damage;
            }
        }

        // Trample: excess damage goes to the defending player
        if remaining_damage > 0 && has_trample && group.defending_player {
            results.push((group.defending_id, remaining_damage, true));
        }
    }

    results
}

/// Compute blocker damage (damage dealt by blockers to the attacker).
pub fn assign_blocker_damage(
    blocker: &Permanent,
    _attacker_id: ObjectId,
    is_first_strike_step: bool,
) -> u32 {
    let has_first_strike = blocker.has_first_strike() || blocker.has_double_strike();
    let has_regular = !blocker.has_first_strike() || blocker.has_double_strike();

    let deals_damage = if is_first_strike_step {
        has_first_strike
    } else {
        has_regular
    };

    if !deals_damage {
        return 0;
    }

    blocker.power().max(0) as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities};

    fn make_creature(
        name: &str,
        power: i32,
        toughness: i32,
        keywords: KeywordAbilities,
    ) -> Permanent {
        let owner = PlayerId::new();
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(power);
        card.toughness = Some(toughness);
        card.keywords = keywords;
        Permanent::new(card, owner)
    }

    #[test]
    fn combat_state_basics() {
        let mut state = CombatState::new();
        let attacker = ObjectId::new();
        let defender = PlayerId::new();

        state.attacking_player = Some(PlayerId::new());
        state.declare_attacker(attacker, ObjectId(defender.0), true);

        assert!(state.has_attackers());
        assert!(state.is_attacking(attacker));
        assert_eq!(state.groups.len(), 1);
        assert!(state.groups[0].is_unblocked());
    }

    #[test]
    fn blocking() {
        let mut state = CombatState::new();
        let attacker = ObjectId::new();
        let blocker = ObjectId::new();
        let defender = ObjectId::new();

        state.declare_attacker(attacker, defender, true);
        state.declare_blocker(blocker, attacker);

        assert!(state.is_blocking(blocker));
        assert!(state.groups[0].is_blocked());
        assert_eq!(state.groups[0].blockers.len(), 1);
    }

    #[test]
    fn unblocked_damage() {
        let attacker = make_creature("Bear", 2, 2, KeywordAbilities::empty());
        let attacker_id = attacker.id();
        let defender_id = ObjectId::new();

        let group = CombatGroup::new(attacker_id, defender_id, true);
        let results = assign_combat_damage(&group, &attacker, &[], false);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, defender_id);
        assert_eq!(results[0].1, 2);
        assert!(results[0].2); // is player
    }

    #[test]
    fn blocked_damage() {
        let attacker = make_creature("Bear", 2, 2, KeywordAbilities::empty());
        let attacker_id = attacker.id();
        let blocker = make_creature("Wall", 0, 4, KeywordAbilities::DEFENDER);
        let blocker_id = blocker.id();
        let defender_id = ObjectId::new();

        let mut group = CombatGroup::new(attacker_id, defender_id, true);
        group.add_blocker(blocker_id);

        let results = assign_combat_damage(
            &group,
            &attacker,
            &[(blocker_id, &blocker)],
            false,
        );

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, blocker_id);
        assert_eq!(results[0].1, 2);
        assert!(!results[0].2); // not player
    }

    #[test]
    fn trample_overflow() {
        let attacker = make_creature("Trampler", 5, 5, KeywordAbilities::TRAMPLE);
        let attacker_id = attacker.id();
        let blocker = make_creature("Bear", 2, 2, KeywordAbilities::empty());
        let blocker_id = blocker.id();
        let defender_id = ObjectId::new();

        let mut group = CombatGroup::new(attacker_id, defender_id, true);
        group.add_blocker(blocker_id);

        let results = assign_combat_damage(
            &group,
            &attacker,
            &[(blocker_id, &blocker)],
            false,
        );

        // 2 damage to blocker (toughness 2), 3 trample to player
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, blocker_id);
        assert_eq!(results[0].1, 2);
        assert_eq!(results[1].0, defender_id);
        assert_eq!(results[1].1, 3);
    }

    #[test]
    fn deathtouch_minimizes_damage() {
        let attacker = make_creature("Deathtouch", 5, 5, KeywordAbilities::DEATHTOUCH | KeywordAbilities::TRAMPLE);
        let attacker_id = attacker.id();
        let blocker = make_creature("BigCreature", 1, 10, KeywordAbilities::empty());
        let blocker_id = blocker.id();
        let defender_id = ObjectId::new();

        let mut group = CombatGroup::new(attacker_id, defender_id, true);
        group.add_blocker(blocker_id);

        let results = assign_combat_damage(
            &group,
            &attacker,
            &[(blocker_id, &blocker)],
            false,
        );

        // With deathtouch, only 1 damage needed to be lethal; rest tramples
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, blocker_id);
        assert_eq!(results[0].1, 1); // deathtouch = 1 is lethal
        assert_eq!(results[1].0, defender_id);
        assert_eq!(results[1].1, 4); // 5 - 1 = 4 trample
    }

    #[test]
    fn first_strike_timing() {
        let attacker = make_creature("FirstStriker", 3, 3, KeywordAbilities::FIRST_STRIKE);
        let attacker_id = attacker.id();
        let defender_id = ObjectId::new();

        let group = CombatGroup::new(attacker_id, defender_id, true);

        // First strike step: deals damage
        let results = assign_combat_damage(&group, &attacker, &[], true);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].1, 3);

        // Regular damage step: does NOT deal damage (only has first strike)
        let results = assign_combat_damage(&group, &attacker, &[], false);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn double_strike_both_steps() {
        let attacker = make_creature("DoubleStriker", 4, 4, KeywordAbilities::DOUBLE_STRIKE);
        let attacker_id = attacker.id();
        let defender_id = ObjectId::new();

        let group = CombatGroup::new(attacker_id, defender_id, true);

        // First strike step: deals damage
        let results = assign_combat_damage(&group, &attacker, &[], true);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].1, 4);

        // Regular damage step: also deals damage
        let results = assign_combat_damage(&group, &attacker, &[], false);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].1, 4);
    }

    #[test]
    fn flying_blocks() {
        let flyer = make_creature("Bird", 2, 2, KeywordAbilities::FLYING);
        let ground = make_creature("Bear", 2, 2, KeywordAbilities::empty());
        let reacher = make_creature("Spider", 2, 4, KeywordAbilities::REACH);
        let other_flyer = make_creature("Angel", 4, 4, KeywordAbilities::FLYING);

        // Ground creature can't block flying
        assert!(!can_block(&ground, &flyer));
        // Reach creature can block flying
        assert!(can_block(&reacher, &flyer));
        // Flying creature can block flying
        assert!(can_block(&other_flyer, &flyer));
        // Ground creature can block ground creature
        assert!(can_block(&ground, &ground));
    }

    #[test]
    fn menace_requires_two_blockers() {
        let menace = make_creature("Menace", 3, 3, KeywordAbilities::MENACE);
        let normal = make_creature("Bear", 2, 2, KeywordAbilities::empty());

        // Menace with 1 blocker: not satisfied
        assert!(!satisfies_menace(&menace, 1));
        // Menace with 2 blockers: satisfied
        assert!(satisfies_menace(&menace, 2));
        // Menace with 0 blockers (unblocked): ok
        assert!(satisfies_menace(&menace, 0));
        // Normal creature with 1 blocker: satisfied
        assert!(satisfies_menace(&normal, 1));
    }

    #[test]
    fn combat_clear() {
        let mut state = CombatState::new();
        state.declare_attacker(ObjectId::new(), ObjectId::new(), true);
        assert!(state.has_attackers());
        state.clear();
        assert!(!state.has_attackers());
    }

    #[test]
    fn blocker_damage_timing() {
        let first_striker = make_creature("FS", 3, 3, KeywordAbilities::FIRST_STRIKE);
        let normal = make_creature("Normal", 2, 2, KeywordAbilities::empty());
        let attacker_id = ObjectId::new();

        // First strike blocker deals damage in first strike step only
        assert_eq!(assign_blocker_damage(&first_striker, attacker_id, true), 3);
        assert_eq!(assign_blocker_damage(&first_striker, attacker_id, false), 0);

        // Normal blocker deals damage in regular step only
        assert_eq!(assign_blocker_damage(&normal, attacker_id, true), 0);
        assert_eq!(assign_blocker_damage(&normal, attacker_id, false), 2);
    }
}
