// PlayerMap — a fixed-size array-backed map keyed by PlayerId.
//
// Replaces HashMap<PlayerId, V> for the common 2-player case. Lookups are
// a simple branch instead of hashing, giving better cache locality and
// eliminating hash overhead on every player access (~213 sites in game.rs).

use crate::types::PlayerId;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

/// A fixed-capacity map from `PlayerId` to `V`, backed by a 2-element array.
///
/// This replaces `HashMap<PlayerId, V>` for the two-player game case.
/// Lookups are O(1) with a simple equality check instead of hashing.
#[derive(Clone, Debug)]
pub struct PlayerMap<V> {
    keys: [PlayerId; 2],
    values: [V; 2],
}

impl<V> PlayerMap<V> {
    /// Create a new PlayerMap from two (PlayerId, V) pairs.
    pub fn new(a: (PlayerId, V), b: (PlayerId, V)) -> Self {
        debug_assert_ne!(a.0, b.0, "PlayerMap requires distinct player IDs");
        PlayerMap {
            keys: [a.0, b.0],
            values: [a.1, b.1],
        }
    }

    #[inline]
    fn index_of(&self, id: &PlayerId) -> Option<usize> {
        if *id == self.keys[0] {
            Some(0)
        } else if *id == self.keys[1] {
            Some(1)
        } else {
            None
        }
    }

    /// Get a reference to the value for the given player.
    #[inline]
    pub fn get(&self, id: &PlayerId) -> Option<&V> {
        self.index_of(id).map(|i| &self.values[i])
    }

    /// Get a mutable reference to the value for the given player.
    #[inline]
    pub fn get_mut(&mut self, id: &PlayerId) -> Option<&mut V> {
        self.index_of(id).map(|i| &mut self.values[i])
    }

    /// Returns the number of entries (always 2).
    #[inline]
    pub fn len(&self) -> usize {
        2
    }

    /// Returns false (a PlayerMap always contains exactly 2 entries).
    #[inline]
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Iterate over all values.
    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.values.iter()
    }

    /// Iterate over all values mutably.
    #[inline]
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.values.iter_mut()
    }

    /// Iterate over (PlayerId, &V) pairs.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&PlayerId, &V)> {
        self.keys.iter().zip(self.values.iter())
    }

    /// Iterate over (PlayerId, &mut V) pairs.
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&PlayerId, &mut V)> {
        self.keys.iter().zip(self.values.iter_mut())
    }

    /// Insert a value for a player. The player must already be one of the two keys.
    /// Returns the old value, or None if the player was not found.
    pub fn insert(&mut self, id: PlayerId, value: V) -> Option<V> {
        self.index_of(&id).map(|i| std::mem::replace(&mut self.values[i], value))
    }

    /// Check if a player ID is contained in this map.
    #[inline]
    pub fn contains_key(&self, id: &PlayerId) -> bool {
        self.index_of(id).is_some()
    }

    /// Get the two player IDs in order.
    #[inline]
    pub fn player_ids(&self) -> [PlayerId; 2] {
        self.keys
    }
}

// Index by &PlayerId — panics if the player ID is not found (matches HashMap behavior).
impl<V> Index<&PlayerId> for PlayerMap<V> {
    type Output = V;
    #[inline]
    fn index(&self, id: &PlayerId) -> &V {
        self.get(id).expect("PlayerMap: player ID not found")
    }
}

impl<V> IndexMut<&PlayerId> for PlayerMap<V> {
    #[inline]
    fn index_mut(&mut self, id: &PlayerId) -> &mut V {
        self.get_mut(id).expect("PlayerMap: player ID not found")
    }
}

// IntoIterator for &PlayerMap — yields (&PlayerId, &V)
impl<'a, V> IntoIterator for &'a PlayerMap<V> {
    type Item = (&'a PlayerId, &'a V);
    type IntoIter = std::iter::Zip<std::slice::Iter<'a, PlayerId>, std::slice::Iter<'a, V>>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.keys.iter().zip(self.values.iter())
    }
}

// IntoIterator for &mut PlayerMap — yields (&PlayerId, &mut V)
impl<'a, V> IntoIterator for &'a mut PlayerMap<V> {
    type Item = (&'a PlayerId, &'a mut V);
    type IntoIter = std::iter::Zip<std::slice::Iter<'a, PlayerId>, std::slice::IterMut<'a, V>>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.keys.iter().zip(self.values.iter_mut())
    }
}

// Serde: serialize as HashMap for compatibility
impl<V: Serialize> Serialize for PlayerMap<V> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let map: HashMap<&PlayerId, &V> = self.iter().collect();
        map.serialize(serializer)
    }
}

impl<'de, V: Deserialize<'de>> Deserialize<'de> for PlayerMap<V> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let map: HashMap<PlayerId, V> = HashMap::deserialize(deserializer)?;
        if map.len() != 2 {
            return Err(serde::de::Error::custom(format!(
                "PlayerMap requires exactly 2 entries, got {}",
                map.len()
            )));
        }
        let mut entries: Vec<(PlayerId, V)> = map.into_iter().collect();
        let b = entries.pop().unwrap();
        let a = entries.pop().unwrap();
        Ok(PlayerMap::new(a, b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_get() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let map = PlayerMap::new((p1, 10), (p2, 20));
        assert_eq!(map.get(&p1), Some(&10));
        assert_eq!(map.get(&p2), Some(&20));
        assert_eq!(map.get(&PlayerId::new()), None);
    }

    #[test]
    fn get_mut_and_insert() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let mut map = PlayerMap::new((p1, 10), (p2, 20));

        *map.get_mut(&p1).unwrap() = 100;
        assert_eq!(map.get(&p1), Some(&100));

        let old = map.insert(p2, 200);
        assert_eq!(old, Some(20));
        assert_eq!(map.get(&p2), Some(&200));
    }

    #[test]
    fn iteration() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let map = PlayerMap::new((p1, 10), (p2, 20));

        let vals: Vec<_> = map.values().collect();
        assert_eq!(vals.len(), 2);

        let pairs: Vec<_> = map.iter().collect();
        assert_eq!(pairs.len(), 2);
    }

    #[test]
    fn len_and_contains() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let map = PlayerMap::new((p1, "a"), (p2, "b"));
        assert_eq!(map.len(), 2);
        assert!(map.contains_key(&p1));
        assert!(map.contains_key(&p2));
        assert!(!map.contains_key(&PlayerId::new()));
    }
}
