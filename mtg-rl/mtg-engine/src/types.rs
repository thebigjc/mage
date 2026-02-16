use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};
use uuid::Uuid;

#[must_use]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ObjectId(Uuid);

impl ObjectId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        debug_assert!(!uuid.is_nil(), "ObjectId must not be nil");
        Self(uuid)
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }

    pub fn from_player(player: PlayerId) -> Self {
        Self(player.0)
    }
}

impl Default for ObjectId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Obj({})", &self.0.to_string()[..8])
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0.to_string()[..8])
    }
}

#[must_use]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PlayerId(Uuid);

impl PlayerId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        debug_assert!(!uuid.is_nil(), "PlayerId must not be nil");
        Self(uuid)
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }

    pub fn from_object(object: ObjectId) -> Self {
        Self(object.0)
    }
}

impl Default for PlayerId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Player({})", &self.0.to_string()[..8])
    }
}

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0.to_string()[..8])
    }
}

#[must_use]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct AbilityId(Uuid);

impl AbilityId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        debug_assert!(!uuid.is_nil(), "AbilityId must not be nil");
        Self(uuid)
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for AbilityId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for AbilityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0.to_string()[..8])
    }
}

macro_rules! game_value_newtype {
    ($Name:ident, $display:expr) => {
        #[must_use]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
        #[repr(transparent)]
        pub struct $Name(i32);

        impl $Name {
            pub const ZERO: Self = Self(0);

            pub const fn new(value: i32) -> Self {
                Self(value)
            }

            pub const fn get(self) -> i32 {
                self.0
            }

            pub const fn abs(self) -> Self {
                Self(self.0.abs())
            }

            pub const fn unsigned_abs(self) -> u32 {
                self.0.unsigned_abs()
            }

            pub const fn max(self, other: Self) -> Self {
                if self.0 > other.0 { self } else { other }
            }

            pub const fn min(self, other: Self) -> Self {
                if self.0 < other.0 { self } else { other }
            }

            pub const fn as_u32_saturating(self) -> u32 {
                if self.0 > 0 { self.0 as u32 } else { 0 }
            }
        }

        impl From<i32> for $Name {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }

        impl From<$Name> for i32 {
            fn from(value: $Name) -> Self {
                value.0
            }
        }

        impl Add for $Name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                Self(self.0 + rhs.0)
            }
        }

        impl Add<i32> for $Name {
            type Output = Self;
            fn add(self, rhs: i32) -> Self {
                Self(self.0 + rhs)
            }
        }

        impl Sub for $Name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                Self(self.0 - rhs.0)
            }
        }

        impl Sub<i32> for $Name {
            type Output = Self;
            fn sub(self, rhs: i32) -> Self {
                Self(self.0 - rhs)
            }
        }

        impl AddAssign for $Name {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }

        impl AddAssign<i32> for $Name {
            fn add_assign(&mut self, rhs: i32) {
                self.0 += rhs;
            }
        }

        impl SubAssign for $Name {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }

        impl SubAssign<i32> for $Name {
            fn sub_assign(&mut self, rhs: i32) {
                self.0 -= rhs;
            }
        }

        impl Neg for $Name {
            type Output = Self;
            fn neg(self) -> Self {
                Self(-self.0)
            }
        }

        impl PartialEq<i32> for $Name {
            fn eq(&self, other: &i32) -> bool {
                self.0 == *other
            }
        }

        impl PartialOrd<i32> for $Name {
            fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
                Some(self.0.cmp(other))
            }
        }

        impl fmt::Debug for $Name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}({})", $display, self.0)
            }
        }

        impl fmt::Display for $Name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

game_value_newtype!(Power, "Power");
game_value_newtype!(Toughness, "Toughness");
game_value_newtype!(Life, "Life");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_ids_are_unique() {
        let a = ObjectId::new();
        let b = ObjectId::new();
        assert_ne!(a, b);
    }

    #[test]
    fn from_uuid_roundtrips() {
        let id = ObjectId::new();
        let uuid = id.as_uuid();
        let id2 = ObjectId::from_uuid(uuid);
        assert_eq!(id, id2);
    }

    #[test]
    fn player_object_conversion_preserves_uuid() {
        let player = PlayerId::new();
        let object = ObjectId::from_player(player);
        let player2 = PlayerId::from_object(object);
        assert_eq!(player, player2);
    }

    #[test]
    #[should_panic(expected = "ObjectId must not be nil")]
    fn object_id_rejects_nil() {
        let _ = ObjectId::from_uuid(Uuid::nil());
    }

    #[test]
    #[should_panic(expected = "PlayerId must not be nil")]
    fn player_id_rejects_nil() {
        let _ = PlayerId::from_uuid(Uuid::nil());
    }

    #[test]
    #[should_panic(expected = "AbilityId must not be nil")]
    fn ability_id_rejects_nil() {
        let _ = AbilityId::from_uuid(Uuid::nil());
    }

    #[test]
    fn power_arithmetic() {
        let p = Power::new(5);
        assert_eq!((p + Power::new(3)).get(), 8);
        assert_eq!((p + 3).get(), 8);
        assert_eq!((p - Power::new(2)).get(), 3);
        assert_eq!((p - 2).get(), 3);
        assert_eq!((-p).get(), -5);

        let mut p2 = Power::new(4);
        p2 += Power::new(2);
        assert_eq!(p2.get(), 6);
        p2 += 1;
        assert_eq!(p2.get(), 7);
        p2 -= Power::new(3);
        assert_eq!(p2.get(), 4);
        p2 -= 1;
        assert_eq!(p2.get(), 3);
    }

    #[test]
    fn power_comparisons() {
        let p = Power::new(5);
        assert!(p > 3);
        assert!(p >= 5);
        assert!(p < 7);
        assert!(p <= 5);
        assert!(p == 5);
        assert!(p != 4);
        assert!(p > Power::new(3));
        assert!(p < Power::new(7));
    }

    #[test]
    fn power_conversions() {
        let p = Power::new(5);
        assert_eq!(i32::from(p), 5);
        assert_eq!(Power::from(5), Power::new(5));
        assert_eq!(p.as_u32_saturating(), 5);

        let neg = Power::new(-3);
        assert_eq!(neg.as_u32_saturating(), 0);
        assert_eq!(neg.unsigned_abs(), 3);
        assert_eq!(neg.abs(), Power::new(3));
    }

    #[test]
    fn power_max_min() {
        let a = Power::new(3);
        let b = Power::new(7);
        assert_eq!(a.max(b), Power::new(7));
        assert_eq!(a.min(b), Power::new(3));
    }

    #[test]
    fn power_zero_and_default() {
        assert_eq!(Power::ZERO.get(), 0);
        assert_eq!(Power::default(), Power::ZERO);
    }

    #[test]
    fn toughness_works_like_power() {
        let t = Toughness::new(4);
        assert_eq!((t + 2).get(), 6);
        assert_eq!((t - Toughness::new(1)).get(), 3);
        assert!(t > 2);
        assert_eq!(t.as_u32_saturating(), 4);
    }

    #[test]
    fn life_arithmetic() {
        let l = Life::new(20);
        assert_eq!((l - 5).get(), 15);
        assert_eq!((l + 5).get(), 25);
        assert!(l > 0);

        let mut l2 = Life::new(20);
        l2 -= 7;
        assert_eq!(l2.get(), 13);
        l2 += 3;
        assert_eq!(l2.get(), 16);
    }

    #[test]
    fn life_negative() {
        let l = Life::new(-3);
        assert!(l < 0);
        assert!(l <= 0);
        assert_eq!(l.as_u32_saturating(), 0);
    }

    #[test]
    fn display_and_debug() {
        assert_eq!(format!("{}", Power::new(5)), "5");
        assert_eq!(format!("{:?}", Power::new(5)), "Power(5)");
        assert_eq!(format!("{}", Life::new(20)), "20");
        assert_eq!(format!("{:?}", Life::new(20)), "Life(20)");
    }
}
