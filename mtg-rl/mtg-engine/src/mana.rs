// Mana and mana cost types.
// Ported from Mage/src/main/java/mage/Mana.java.

use crate::constants::{Color, ManaColor};
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// Represents an amount of mana (either in a mana pool or as a cost).
///
/// Fields represent counts of each mana type:
/// - `white`, `blue`, `black`, `red`, `green`: colored mana
/// - `colorless`: colorless mana (e.g. from Wastes, Eldrazi)
/// - `generic`: generic mana cost (payable by any type)
/// - `any`: mana that can be any color (from "add one mana of any color")
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Mana {
    pub white: u32,
    pub blue: u32,
    pub black: u32,
    pub red: u32,
    pub green: u32,
    pub colorless: u32,
    pub generic: u32,
    pub any: u32,
}

impl Mana {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn white(amount: u32) -> Self {
        Mana { white: amount, ..Default::default() }
    }

    pub fn blue(amount: u32) -> Self {
        Mana { blue: amount, ..Default::default() }
    }

    pub fn black(amount: u32) -> Self {
        Mana { black: amount, ..Default::default() }
    }

    pub fn red(amount: u32) -> Self {
        Mana { red: amount, ..Default::default() }
    }

    pub fn green(amount: u32) -> Self {
        Mana { green: amount, ..Default::default() }
    }

    pub fn colorless(amount: u32) -> Self {
        Mana { colorless: amount, ..Default::default() }
    }

    pub fn generic(amount: u32) -> Self {
        Mana { generic: amount, ..Default::default() }
    }

    pub fn any(amount: u32) -> Self {
        Mana { any: amount, ..Default::default() }
    }

    /// Total count of all mana types.
    pub fn count(&self) -> u32 {
        self.white + self.blue + self.black + self.red + self.green
            + self.colorless + self.generic + self.any
    }

    /// Total colored mana.
    pub fn colored_count(&self) -> u32 {
        self.white + self.blue + self.black + self.red + self.green
    }

    /// Converted mana cost (total mana value).
    pub fn mana_value(&self) -> u32 {
        self.count()
    }

    /// Get the amount of a specific mana color.
    pub fn get_color(&self, color: ManaColor) -> u32 {
        match color {
            ManaColor::White => self.white,
            ManaColor::Blue => self.blue,
            ManaColor::Black => self.black,
            ManaColor::Red => self.red,
            ManaColor::Green => self.green,
            ManaColor::Colorless => self.colorless,
        }
    }

    /// Add mana of a specific color.
    pub fn add_color(&mut self, color: ManaColor, amount: u32) {
        match color {
            ManaColor::White => self.white += amount,
            ManaColor::Blue => self.blue += amount,
            ManaColor::Black => self.black += amount,
            ManaColor::Red => self.red += amount,
            ManaColor::Green => self.green += amount,
            ManaColor::Colorless => self.colorless += amount,
        }
    }

    /// Check if this mana pool can pay the given mana cost.
    /// Uses a simplified algorithm (does not handle hybrid mana).
    pub fn can_pay(&self, cost: &Mana) -> bool {
        // First check colored requirements
        if self.white < cost.white
            || self.blue < cost.blue
            || self.black < cost.black
            || self.red < cost.red
            || self.green < cost.green
            || self.colorless < cost.colorless
        {
            return false;
        }

        // Remaining after paying colored costs
        let remaining = self.white - cost.white
            + self.blue - cost.blue
            + self.black - cost.black
            + self.red - cost.red
            + self.green - cost.green
            + self.colorless - cost.colorless
            + self.any;

        remaining >= cost.generic
    }

    /// Returns the colors present in this mana.
    pub fn colors(&self) -> Vec<Color> {
        let mut result = Vec::new();
        if self.white > 0 { result.push(Color::White); }
        if self.blue > 0 { result.push(Color::Blue); }
        if self.black > 0 { result.push(Color::Black); }
        if self.red > 0 { result.push(Color::Red); }
        if self.green > 0 { result.push(Color::Green); }
        result
    }

    /// Returns true if this represents no mana.
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }
}

impl Add for Mana {
    type Output = Mana;
    fn add(self, rhs: Mana) -> Mana {
        Mana {
            white: self.white + rhs.white,
            blue: self.blue + rhs.blue,
            black: self.black + rhs.black,
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            colorless: self.colorless + rhs.colorless,
            generic: self.generic + rhs.generic,
            any: self.any + rhs.any,
        }
    }
}

impl AddAssign for Mana {
    fn add_assign(&mut self, rhs: Mana) {
        self.white += rhs.white;
        self.blue += rhs.blue;
        self.black += rhs.black;
        self.red += rhs.red;
        self.green += rhs.green;
        self.colorless += rhs.colorless;
        self.generic += rhs.generic;
        self.any += rhs.any;
    }
}

impl Sub for Mana {
    type Output = Mana;
    fn sub(self, rhs: Mana) -> Mana {
        Mana {
            white: self.white.saturating_sub(rhs.white),
            blue: self.blue.saturating_sub(rhs.blue),
            black: self.black.saturating_sub(rhs.black),
            red: self.red.saturating_sub(rhs.red),
            green: self.green.saturating_sub(rhs.green),
            colorless: self.colorless.saturating_sub(rhs.colorless),
            generic: self.generic.saturating_sub(rhs.generic),
            any: self.any.saturating_sub(rhs.any),
        }
    }
}

impl SubAssign for Mana {
    fn sub_assign(&mut self, rhs: Mana) {
        self.white = self.white.saturating_sub(rhs.white);
        self.blue = self.blue.saturating_sub(rhs.blue);
        self.black = self.black.saturating_sub(rhs.black);
        self.red = self.red.saturating_sub(rhs.red);
        self.green = self.green.saturating_sub(rhs.green);
        self.colorless = self.colorless.saturating_sub(rhs.colorless);
        self.generic = self.generic.saturating_sub(rhs.generic);
        self.any = self.any.saturating_sub(rhs.any);
    }
}

impl std::fmt::Display for Mana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();
        if self.generic > 0 { parts.push(format!("{{{}}}", self.generic)); }
        for _ in 0..self.white { parts.push("{W}".to_string()); }
        for _ in 0..self.blue { parts.push("{U}".to_string()); }
        for _ in 0..self.black { parts.push("{B}".to_string()); }
        for _ in 0..self.red { parts.push("{R}".to_string()); }
        for _ in 0..self.green { parts.push("{G}".to_string()); }
        for _ in 0..self.colorless { parts.push("{C}".to_string()); }
        for _ in 0..self.any { parts.push("{A}".to_string()); }
        if parts.is_empty() {
            write!(f, "{{0}}")
        } else {
            write!(f, "{}", parts.join(""))
        }
    }
}

/// A mana cost component: either colored, colorless, or generic.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ManaCostItem {
    /// Colored mana symbol (W, U, B, R, G).
    Colored(ManaColor),
    /// Colorless mana ({C}).
    Colorless,
    /// Generic mana ({1}, {2}, etc.).
    Generic(u32),
    /// X cost.
    X,
    /// Hybrid mana ({W/U}, {2/W}, etc.).
    Hybrid(ManaColor, ManaColor),
    /// Phyrexian mana ({W/P}, etc.). Can be paid with 2 life.
    Phyrexian(ManaColor),
    /// Snow mana ({S}).
    Snow,
}

/// A complete mana cost, such as "{2}{B}{G}".
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManaCost {
    pub items: Vec<ManaCostItem>,
}

impl ManaCost {
    pub fn new() -> Self {
        ManaCost { items: Vec::new() }
    }

    /// Parse a mana cost string like "{2}{B}{G}" or "{X}{R}{R}".
    pub fn parse(s: &str) -> ManaCost {
        let mut items = Vec::new();
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '{' {
                let mut inner = String::new();
                while let Some(&nc) = chars.peek() {
                    chars.next();
                    if nc == '}' { break; }
                    inner.push(nc);
                }
                match inner.as_str() {
                    "W" => items.push(ManaCostItem::Colored(ManaColor::White)),
                    "U" => items.push(ManaCostItem::Colored(ManaColor::Blue)),
                    "B" => items.push(ManaCostItem::Colored(ManaColor::Black)),
                    "R" => items.push(ManaCostItem::Colored(ManaColor::Red)),
                    "G" => items.push(ManaCostItem::Colored(ManaColor::Green)),
                    "C" => items.push(ManaCostItem::Colorless),
                    "X" => items.push(ManaCostItem::X),
                    "S" => items.push(ManaCostItem::Snow),
                    num => {
                        if let Ok(n) = num.parse::<u32>() {
                            items.push(ManaCostItem::Generic(n));
                        }
                        // Hybrid/phyrexian parsing would go here for full support
                    }
                }
            }
        }
        ManaCost { items }
    }

    /// Convert to a Mana struct (ignoring X and special costs).
    pub fn to_mana(&self) -> Mana {
        let mut mana = Mana::new();
        for item in &self.items {
            match item {
                ManaCostItem::Colored(c) => mana.add_color(*c, 1),
                ManaCostItem::Colorless => mana.colorless += 1,
                ManaCostItem::Generic(n) => mana.generic += n,
                ManaCostItem::Snow => mana.generic += 1,
                _ => {}
            }
        }
        mana
    }

    /// Calculate the mana value (converted mana cost).
    pub fn mana_value(&self) -> u32 {
        self.items.iter().map(|item| match item {
            ManaCostItem::Colored(_) => 1,
            ManaCostItem::Colorless => 1,
            ManaCostItem::Generic(n) => *n,
            ManaCostItem::X => 0,
            ManaCostItem::Hybrid(_, _) => 1,
            ManaCostItem::Phyrexian(_) => 1,
            ManaCostItem::Snow => 1,
        }).sum()
    }

    /// Returns the colors in this mana cost.
    pub fn colors(&self) -> Vec<Color> {
        let mut result = Vec::new();
        for item in &self.items {
            if let ManaCostItem::Colored(mc) = item {
                if let Some(c) = match mc {
                    ManaColor::White => Some(Color::White),
                    ManaColor::Blue => Some(Color::Blue),
                    ManaColor::Black => Some(Color::Black),
                    ManaColor::Red => Some(Color::Red),
                    ManaColor::Green => Some(Color::Green),
                    _ => None,
                } {
                    if !result.contains(&c) {
                        result.push(c);
                    }
                }
            }
        }
        result
    }

    /// Whether this mana cost contains an X component.
    pub fn has_x_cost(&self) -> bool {
        self.items.iter().any(|item| matches!(item, ManaCostItem::X))
    }

    /// Count how many X symbols are in this cost (e.g. {X}{X}{B} has 2).
    pub fn x_count(&self) -> u32 {
        self.items.iter().filter(|item| matches!(item, ManaCostItem::X)).count() as u32
    }

    /// Convert to a Mana struct with X substituted as generic mana.
    pub fn to_mana_with_x(&self, x_value: u32) -> Mana {
        let mut mana = Mana::new();
        for item in &self.items {
            match item {
                ManaCostItem::Colored(c) => mana.add_color(*c, 1),
                ManaCostItem::Colorless => mana.colorless += 1,
                ManaCostItem::Generic(n) => mana.generic += n,
                ManaCostItem::Snow => mana.generic += 1,
                ManaCostItem::X => mana.generic += x_value,
                _ => {}
            }
        }
        mana
    }
}

impl Default for ManaCost {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ManaCost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in &self.items {
            match item {
                ManaCostItem::Colored(c) => write!(f, "{{{}}}", c.symbol())?,
                ManaCostItem::Colorless => write!(f, "{{C}}")?,
                ManaCostItem::Generic(n) => write!(f, "{{{}}}", n)?,
                ManaCostItem::X => write!(f, "{{X}}")?,
                ManaCostItem::Hybrid(a, b) => write!(f, "{{{}/{}}}", a.symbol(), b.symbol())?,
                ManaCostItem::Phyrexian(c) => write!(f, "{{{}/P}}", c.symbol())?,
                ManaCostItem::Snow => write!(f, "{{S}}")?,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_cost() {
        let cost = ManaCost::parse("{2}{B}{G}");
        assert_eq!(cost.mana_value(), 4);
        assert_eq!(cost.items.len(), 3);
        assert_eq!(cost.to_string(), "{2}{B}{G}");
    }

    #[test]
    fn mana_arithmetic() {
        let a = Mana::white(2) + Mana::blue(1);
        assert_eq!(a.white, 2);
        assert_eq!(a.blue, 1);
        assert_eq!(a.count(), 3);

        let b = a - Mana::white(1);
        assert_eq!(b.white, 1);
        assert_eq!(b.blue, 1);
    }

    #[test]
    fn can_pay_colored() {
        let pool = Mana { white: 1, blue: 1, black: 0, red: 0, green: 0, colorless: 0, generic: 0, any: 0 };
        let cost = Mana { white: 1, blue: 1, black: 0, red: 0, green: 0, colorless: 0, generic: 0, any: 0 };
        assert!(pool.can_pay(&cost));
    }

    #[test]
    fn can_pay_generic() {
        let pool = Mana { white: 2, blue: 0, black: 0, red: 0, green: 0, colorless: 0, generic: 0, any: 0 };
        let cost = Mana { white: 1, blue: 0, black: 0, red: 0, green: 0, colorless: 0, generic: 1, any: 0 };
        assert!(pool.can_pay(&cost));
    }

    #[test]
    fn cannot_pay_insufficient() {
        let pool = Mana { white: 1, blue: 0, black: 0, red: 0, green: 0, colorless: 0, generic: 0, any: 0 };
        let cost = Mana { white: 0, blue: 1, black: 0, red: 0, green: 0, colorless: 0, generic: 0, any: 0 };
        assert!(!pool.can_pay(&cost));
    }

    #[test]
    fn cost_colors() {
        let cost = ManaCost::parse("{1}{R}{G}");
        let colors = cost.colors();
        assert!(colors.contains(&Color::Red));
        assert!(colors.contains(&Color::Green));
        assert_eq!(colors.len(), 2);
    }

    #[test]
    fn mana_display() {
        let m = Mana { white: 0, blue: 0, black: 1, red: 0, green: 1, colorless: 0, generic: 2, any: 0 };
        assert_eq!(m.to_string(), "{2}{B}{G}");
    }
}
