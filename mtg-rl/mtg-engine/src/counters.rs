// Counter types (+1/+1, -1/-1, loyalty, etc.) and a Counters container.
// Ported from Mage/src/main/java/mage/counters/CounterType.java.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The most commonly used counter types in Magic.
/// Uses a Custom variant for the long tail of named counters.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CounterType {
    // Power/toughness modifiers
    P1P1,     // +1/+1
    M1M1,     // -1/-1

    // Planeswalker
    Loyalty,

    // Common named counters
    Charge,
    Shield,
    Stun,
    Finality,
    Level,
    Lore,      // saga counters
    Time,
    Fade,
    Age,
    Storage,
    Divinity,
    Energy,    // player counter
    Experience, // player counter
    Poison,    // player counter
    Rad,       // player counter
    Ticket,    // player counter

    // Keyword counters (from Ikoria)
    Flying,
    FirstStrike,
    DoubleStrike,
    Deathtouch,
    Hexproof,
    Indestructible,
    Lifelink,
    Menace,
    Reach,
    Trample,
    Vigilance,

    // Artifact/misc
    Bounty,
    Brick,
    Dream,
    Doom,
    Egg,
    Flood,
    Fungus,
    Growth,
    Hatchling,
    Ice,
    Luck,
    Mine,
    Page,
    Quest,
    Slime,
    Spore,
    Tower,
    Verse,
    Wish,
    Vitality,
    Valor,
    Training,

    /// Catch-all for counter types not in the above list.
    Custom(String),
}

impl CounterType {
    /// Look up a counter type by name.
    pub fn from_name(name: &str) -> CounterType {
        match name.to_lowercase().as_str() {
            "+1/+1" | "p1p1" => CounterType::P1P1,
            "-1/-1" | "m1m1" => CounterType::M1M1,
            "loyalty" => CounterType::Loyalty,
            "charge" => CounterType::Charge,
            "shield" => CounterType::Shield,
            "stun" => CounterType::Stun,
            "finality" => CounterType::Finality,
            "level" => CounterType::Level,
            "lore" => CounterType::Lore,
            "time" => CounterType::Time,
            "fade" => CounterType::Fade,
            "age" => CounterType::Age,
            "storage" => CounterType::Storage,
            "divinity" => CounterType::Divinity,
            "energy" => CounterType::Energy,
            "experience" => CounterType::Experience,
            "poison" => CounterType::Poison,
            "rad" => CounterType::Rad,
            "ticket" => CounterType::Ticket,
            "flying" => CounterType::Flying,
            "first strike" => CounterType::FirstStrike,
            "double strike" => CounterType::DoubleStrike,
            "deathtouch" => CounterType::Deathtouch,
            "hexproof" => CounterType::Hexproof,
            "indestructible" => CounterType::Indestructible,
            "lifelink" => CounterType::Lifelink,
            "menace" => CounterType::Menace,
            "reach" => CounterType::Reach,
            "trample" => CounterType::Trample,
            "vigilance" => CounterType::Vigilance,
            "bounty" => CounterType::Bounty,
            "brick" => CounterType::Brick,
            "dream" => CounterType::Dream,
            "doom" => CounterType::Doom,
            "egg" => CounterType::Egg,
            "flood" => CounterType::Flood,
            "fungus" => CounterType::Fungus,
            "growth" => CounterType::Growth,
            "hatchling" => CounterType::Hatchling,
            "ice" => CounterType::Ice,
            "luck" => CounterType::Luck,
            "mine" => CounterType::Mine,
            "page" => CounterType::Page,
            "quest" => CounterType::Quest,
            "slime" => CounterType::Slime,
            "spore" => CounterType::Spore,
            "tower" => CounterType::Tower,
            "verse" => CounterType::Verse,
            "wish" => CounterType::Wish,
            "vitality" => CounterType::Vitality,
            "valor" => CounterType::Valor,
            "training" => CounterType::Training,
            other => CounterType::Custom(other.to_string()),
        }
    }

    /// Display name of this counter type.
    pub fn name(&self) -> &str {
        match self {
            CounterType::P1P1 => "+1/+1",
            CounterType::M1M1 => "-1/-1",
            CounterType::Loyalty => "loyalty",
            CounterType::Charge => "charge",
            CounterType::Shield => "shield",
            CounterType::Stun => "stun",
            CounterType::Finality => "finality",
            CounterType::Level => "level",
            CounterType::Lore => "lore",
            CounterType::Time => "time",
            CounterType::Fade => "fade",
            CounterType::Age => "age",
            CounterType::Storage => "storage",
            CounterType::Divinity => "divinity",
            CounterType::Energy => "energy",
            CounterType::Experience => "experience",
            CounterType::Poison => "poison",
            CounterType::Rad => "rad",
            CounterType::Ticket => "ticket",
            CounterType::Flying => "flying",
            CounterType::FirstStrike => "first strike",
            CounterType::DoubleStrike => "double strike",
            CounterType::Deathtouch => "deathtouch",
            CounterType::Hexproof => "hexproof",
            CounterType::Indestructible => "indestructible",
            CounterType::Lifelink => "lifelink",
            CounterType::Menace => "menace",
            CounterType::Reach => "reach",
            CounterType::Trample => "trample",
            CounterType::Vigilance => "vigilance",
            CounterType::Bounty => "bounty",
            CounterType::Brick => "brick",
            CounterType::Dream => "dream",
            CounterType::Doom => "doom",
            CounterType::Egg => "egg",
            CounterType::Flood => "flood",
            CounterType::Fungus => "fungus",
            CounterType::Growth => "growth",
            CounterType::Hatchling => "hatchling",
            CounterType::Ice => "ice",
            CounterType::Luck => "luck",
            CounterType::Mine => "mine",
            CounterType::Page => "page",
            CounterType::Quest => "quest",
            CounterType::Slime => "slime",
            CounterType::Spore => "spore",
            CounterType::Tower => "tower",
            CounterType::Verse => "verse",
            CounterType::Wish => "wish",
            CounterType::Vitality => "vitality",
            CounterType::Valor => "valor",
            CounterType::Training => "training",
            CounterType::Custom(s) => s.as_str(),
        }
    }

    /// Returns the P/T modification this counter type provides.
    /// Returns (power_mod, toughness_mod).
    pub fn pt_modifier(&self) -> (i32, i32) {
        match self {
            CounterType::P1P1 => (1, 1),
            CounterType::M1M1 => (-1, -1),
            _ => (0, 0),
        }
    }
}

impl std::fmt::Display for CounterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Container for counters on a game object (permanent, player, card, etc.).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Counters {
    map: HashMap<CounterType, u32>,
}

impl Counters {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add `count` counters of the given type.
    pub fn add(&mut self, counter_type: CounterType, count: u32) {
        *self.map.entry(counter_type).or_insert(0) += count;
    }

    /// Remove up to `count` counters of the given type. Returns the actual number removed.
    pub fn remove(&mut self, counter_type: &CounterType, count: u32) -> u32 {
        if let Some(current) = self.map.get_mut(counter_type) {
            let removed = (*current).min(count);
            *current -= removed;
            if *current == 0 {
                self.map.remove(counter_type);
            }
            removed
        } else {
            0
        }
    }

    /// Remove all counters of the given type. Returns how many were removed.
    pub fn remove_all(&mut self, counter_type: &CounterType) -> u32 {
        self.map.remove(counter_type).unwrap_or(0)
    }

    /// Get the count of a specific counter type.
    pub fn get(&self, counter_type: &CounterType) -> u32 {
        self.map.get(counter_type).copied().unwrap_or(0)
    }

    /// Check if any counters of the given type are present.
    pub fn has(&self, counter_type: &CounterType) -> bool {
        self.get(counter_type) > 0
    }

    /// Total number of counters of all types.
    pub fn total_count(&self) -> u32 {
        self.map.values().sum()
    }

    /// Returns true if there are no counters.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Remove all counters of all types.
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Iterator over all (counter_type, count) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&CounterType, &u32)> {
        self.map.iter()
    }

    /// Calculate the total P/T modification from all counters.
    pub fn pt_modification(&self) -> (i32, i32) {
        let mut power = 0i32;
        let mut toughness = 0i32;
        for (ct, &count) in &self.map {
            let (p, t) = ct.pt_modifier();
            power += p * count as i32;
            toughness += t * count as i32;
        }
        (power, toughness)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_remove_counters() {
        let mut counters = Counters::new();
        counters.add(CounterType::P1P1, 3);
        assert_eq!(counters.get(&CounterType::P1P1), 3);

        let removed = counters.remove(&CounterType::P1P1, 2);
        assert_eq!(removed, 2);
        assert_eq!(counters.get(&CounterType::P1P1), 1);

        let removed = counters.remove(&CounterType::P1P1, 5);
        assert_eq!(removed, 1);
        assert!(!counters.has(&CounterType::P1P1));
    }

    #[test]
    fn pt_modification() {
        let mut counters = Counters::new();
        counters.add(CounterType::P1P1, 2);
        counters.add(CounterType::M1M1, 1);
        let (p, t) = counters.pt_modification();
        assert_eq!(p, 1);
        assert_eq!(t, 1);
    }

    #[test]
    fn counter_type_from_name() {
        assert_eq!(CounterType::from_name("+1/+1"), CounterType::P1P1);
        assert_eq!(CounterType::from_name("loyalty"), CounterType::Loyalty);
        assert_eq!(
            CounterType::from_name("something_custom"),
            CounterType::Custom("something_custom".to_string())
        );
    }
}
