// Mana pool for players.
// Ported from mage.players.ManaPool.
//
// The mana pool tracks available mana for a player. Mana is added by
// activating mana abilities (tapping lands, etc.) and spent to pay costs.
// The pool empties at the end of each step/phase (unless effects say otherwise).

use crate::constants::ManaColor;
use crate::mana::Mana;
use crate::types::ObjectId;
use serde::{Deserialize, Serialize};

/// A single item of mana in the pool, tracking its source and type.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ManaPoolItem {
    /// How much mana of each color this item provides.
    pub mana: Mana,
    /// The permanent that produced this mana (for tracking restrictions).
    pub source_id: Option<ObjectId>,
    /// Whether this mana was produced by a snow permanent.
    pub snow: bool,
    /// If set, this mana can only be spent to cast spells of these types.
    /// Empty means unrestricted.
    pub restriction: ManaRestriction,
}

/// Restrictions on how pooled mana can be spent.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum ManaRestriction {
    /// No restriction — can be spent on anything.
    #[default]
    None,
    /// Can only be spent to cast creature spells (e.g. Cavern of Souls).
    CreatureSpellsOnly,
    /// Can only be spent to activate abilities.
    AbilitiesOnly,
    /// Can only be spent on a specific card type or purpose (description).
    Custom(String),
}

/// A player's mana pool.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ManaPool {
    /// Individual mana items (each from a different source activation).
    items: Vec<ManaPoolItem>,
    /// Whether auto-payment is enabled (for UI; always true for AI).
    pub auto_payment: bool,
}

impl ManaPool {
    pub fn new() -> Self {
        ManaPool {
            items: Vec::new(),
            auto_payment: true,
        }
    }

    /// Add mana to the pool from a source.
    pub fn add(&mut self, mana: Mana, source: Option<ObjectId>, snow: bool) {
        self.items.push(ManaPoolItem {
            mana,
            source_id: source,
            snow,
            restriction: ManaRestriction::None,
        });
    }

    /// Add mana with a spending restriction.
    pub fn add_restricted(
        &mut self,
        mana: Mana,
        source: Option<ObjectId>,
        snow: bool,
        restriction: ManaRestriction,
    ) {
        self.items.push(ManaPoolItem {
            mana,
            source_id: source,
            snow,
            restriction,
        });
    }

    /// Get total available mana across all items (ignoring restrictions).
    pub fn available(&self) -> Mana {
        let mut total = Mana::new();
        for item in &self.items {
            total += item.mana;
        }
        total
    }

    /// Get the amount of a specific color available.
    pub fn get_color(&self, color: ManaColor) -> u32 {
        let total = self.available();
        match color {
            ManaColor::White => total.white,
            ManaColor::Blue => total.blue,
            ManaColor::Black => total.black,
            ManaColor::Red => total.red,
            ManaColor::Green => total.green,
            ManaColor::Colorless => total.colorless,
        }
    }

    /// Get the total mana count across all colors.
    pub fn total_count(&self) -> u32 {
        let m = self.available();
        m.white + m.blue + m.black + m.red + m.green + m.colorless + m.any
    }

    /// Spend one mana of the specified color. Returns true if successful.
    pub fn spend(&mut self, color: ManaColor) -> bool {
        for item in &mut self.items {
            let field = match color {
                ManaColor::White => &mut item.mana.white,
                ManaColor::Blue => &mut item.mana.blue,
                ManaColor::Black => &mut item.mana.black,
                ManaColor::Red => &mut item.mana.red,
                ManaColor::Green => &mut item.mana.green,
                ManaColor::Colorless => &mut item.mana.colorless,
            };
            if *field > 0 {
                *field -= 1;
                return true;
            }
        }
        // Try "any" mana if the color request can be satisfied by it
        if color.is_colored() {
            for item in &mut self.items {
                if item.mana.any > 0 {
                    item.mana.any -= 1;
                    return true;
                }
            }
        }
        false
    }

    /// Spend one generic mana (can use any color or colorless).
    /// Returns true if successful.
    pub fn spend_generic(&mut self) -> bool {
        // Prefer colorless first, then any, then colors (to preserve colored mana)
        for item in &mut self.items {
            if item.mana.colorless > 0 {
                item.mana.colorless -= 1;
                return true;
            }
        }
        for item in &mut self.items {
            if item.mana.any > 0 {
                item.mana.any -= 1;
                return true;
            }
        }
        // Then use any colored mana
        for item in &mut self.items {
            for field in [
                &mut item.mana.white,
                &mut item.mana.blue,
                &mut item.mana.black,
                &mut item.mana.red,
                &mut item.mana.green,
            ] {
                if *field > 0 {
                    *field -= 1;
                    return true;
                }
            }
        }
        false
    }

    /// Try to pay a full Mana cost. Returns true if the cost was fully paid.
    /// On failure, the pool is not modified (atomic operation).
    pub fn try_pay(&mut self, cost: &Mana) -> bool {
        // Check if we can pay first
        let avail = self.available();
        if !avail.can_pay(cost) {
            return false;
        }

        // Clone and try to pay
        let mut pool_clone = self.clone();

        // Pay colored costs first
        for _ in 0..cost.white {
            if !pool_clone.spend(ManaColor::White) {
                return false;
            }
        }
        for _ in 0..cost.blue {
            if !pool_clone.spend(ManaColor::Blue) {
                return false;
            }
        }
        for _ in 0..cost.black {
            if !pool_clone.spend(ManaColor::Black) {
                return false;
            }
        }
        for _ in 0..cost.red {
            if !pool_clone.spend(ManaColor::Red) {
                return false;
            }
        }
        for _ in 0..cost.green {
            if !pool_clone.spend(ManaColor::Green) {
                return false;
            }
        }
        for _ in 0..cost.colorless {
            if !pool_clone.spend(ManaColor::Colorless) {
                return false;
            }
        }
        // Pay generic costs
        for _ in 0..cost.generic {
            if !pool_clone.spend_generic() {
                return false;
            }
        }

        // Success — apply the changes
        *self = pool_clone;
        true
    }

    /// Empty the mana pool (happens at end of each step/phase).
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Remove all items that have been fully spent (zero mana remaining).
    pub fn cleanup_empty(&mut self) {
        self.items.retain(|item| {
            let m = &item.mana;
            m.white + m.blue + m.black + m.red + m.green + m.colorless + m.any > 0
        });
    }

    /// Whether the pool is empty.
    pub fn is_empty(&self) -> bool {
        self.total_count() == 0
    }

    /// Get the mana pool as an array [W, U, B, R, G, C] for observation encoding.
    pub fn as_color_array(&self) -> [u32; 6] {
        let m = self.available();
        [m.white, m.blue, m.black, m.red, m.green, m.colorless]
    }
}

impl Default for ManaPool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_spend_colored() {
        let mut pool = ManaPool::new();
        pool.add(Mana::red(3), None, false);
        assert_eq!(pool.get_color(ManaColor::Red), 3);

        assert!(pool.spend(ManaColor::Red));
        assert_eq!(pool.get_color(ManaColor::Red), 2);
    }

    #[test]
    fn spend_generic_prefers_colorless() {
        let mut pool = ManaPool::new();
        pool.add(Mana { colorless: 2, red: 1, ..Default::default() }, None, false);

        assert!(pool.spend_generic());
        // Should have spent colorless first
        let avail = pool.available();
        assert_eq!(avail.colorless, 1);
        assert_eq!(avail.red, 1);
    }

    #[test]
    fn try_pay_atomic() {
        let mut pool = ManaPool::new();
        pool.add(Mana { white: 1, blue: 1, generic: 0, ..Default::default() }, None, false);

        // Can't pay 2W
        let cost = Mana { white: 2, ..Default::default() };
        assert!(!pool.try_pay(&cost));
        // Pool unchanged
        assert_eq!(pool.get_color(ManaColor::White), 1);

        // Can pay 1W (1 generic + 1 white)
        let cost = Mana { white: 1, generic: 1, ..Default::default() };
        assert!(pool.try_pay(&cost));
        assert_eq!(pool.total_count(), 0);
    }

    #[test]
    fn clear_empties_pool() {
        let mut pool = ManaPool::new();
        pool.add(Mana::green(5), None, false);
        assert!(!pool.is_empty());
        pool.clear();
        assert!(pool.is_empty());
    }

    #[test]
    fn any_mana_pays_colored() {
        let mut pool = ManaPool::new();
        pool.add(Mana::any(2), None, false);
        assert!(pool.spend(ManaColor::Blue));
        assert!(pool.spend(ManaColor::Red));
        assert!(!pool.spend(ManaColor::Green));
    }

    #[test]
    fn color_array() {
        let mut pool = ManaPool::new();
        pool.add(Mana { white: 1, blue: 2, black: 0, red: 3, green: 0, colorless: 1, ..Default::default() }, None, false);
        assert_eq!(pool.as_color_array(), [1, 2, 0, 3, 0, 1]);
    }
}
