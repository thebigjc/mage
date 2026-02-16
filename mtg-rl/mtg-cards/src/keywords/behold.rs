// Behold <subtype> — ECL (Lorwyn Eclipsed) set-specific keyword.
//
// An optional additional cost: "As an additional cost to cast this spell,
// you may choose a <subtype> you control or reveal a <subtype> card from
// your hand."
//
// If the behold cost was paid, the spell gets an enhanced effect (e.g.,
// larger bonus, additional effect). The base spell still works without it,
// but the beheld version is strictly better.
//
// Ported from: mage.abilities.keyword.BeholdAbility

use mtg_engine::abilities::Cost;

/// Create a Behold cost for a given creature subtype.
///
/// Returns a `Cost::RevealFromHand(subtype)` representing the behold
/// mechanic's reveal-or-choose option. The game engine handles the
/// "choose a <subtype> you control OR reveal from hand" as a single
/// composite cost.
///
/// This is an *optional* additional cost — the spell can be cast without
/// it, but if paid, provides an enhanced effect.
pub fn behold_cost(subtype: &str) -> Cost {
    Cost::RevealFromHand(format!("Behold {subtype}"))
}

/// Generate the rules text for Behold.
pub fn behold_rules_text(subtype: &str) -> String {
    format!(
        "Behold a {subtype} (As an additional cost to cast this spell, you may choose a {subtype} you control \
         or reveal a {subtype} card from your hand.)"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn behold_elf_cost() {
        let cost = behold_cost("Elf");
        match cost {
            Cost::RevealFromHand(desc) => {
                assert_eq!(desc, "Behold Elf");
            }
            other => panic!("Expected RevealFromHand, got {:?}", other),
        }
    }

    #[test]
    fn behold_goblin_cost() {
        let cost = behold_cost("Goblin");
        match cost {
            Cost::RevealFromHand(desc) => {
                assert_eq!(desc, "Behold Goblin");
            }
            other => panic!("Expected RevealFromHand, got {:?}", other),
        }
    }

    #[test]
    fn behold_rules_text_format() {
        let text = behold_rules_text("Merfolk");
        assert!(text.contains("Behold a Merfolk"));
        assert!(text.contains("choose a Merfolk you control"));
        assert!(text.contains("reveal a Merfolk card from your hand"));
    }
}
