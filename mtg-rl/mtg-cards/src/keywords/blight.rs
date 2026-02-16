// Blight N — ECL (Lorwyn Eclipsed) set-specific keyword.
//
// An additional cost that puts N -1/-1 counters on a creature you control.
// Cards with blight have a base effect, and the blight cost makes them
// cheaper or enables them in the first place.
//
// Examples:
// - "As an additional cost to cast this spell, put a -1/-1 counter on a
//   creature you control." (Blight 1)
// - "As an additional cost to cast this spell, put two -1/-1 counters on a
//   creature you control." (Blight 2)
//
// Ported from: mage.abilities.costs.common.BlightCost

use mtg_engine::abilities::Cost;

/// Create a Blight N cost.
///
/// Returns a `Cost::Blight(n)` that represents putting N -1/-1 counters on
/// a creature you control as an additional cost.
pub fn blight_cost(count: u32) -> Cost {
    Cost::Blight(count)
}

/// Generate the rules text for a Blight N cost.
pub fn blight_rules_text(count: u32) -> String {
    if count == 1 {
        "As an additional cost to cast this spell, put a -1/-1 counter on a creature you control."
            .to_string()
    } else {
        format!(
            "As an additional cost to cast this spell, put {count} -1/-1 counters on a creature you control."
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blight_1_cost() {
        let cost = blight_cost(1);
        match cost {
            Cost::Blight(n) => assert_eq!(n, 1),
            other => panic!("Expected Blight(1), got {other:?}"),
        }
    }

    #[test]
    fn blight_2_cost() {
        let cost = blight_cost(2);
        match cost {
            Cost::Blight(n) => assert_eq!(n, 2),
            other => panic!("Expected Blight(2), got {other:?}"),
        }
    }

    #[test]
    fn blight_rules_text_singular() {
        let text = blight_rules_text(1);
        assert!(text.contains("a -1/-1 counter"));
        assert!(!text.contains("counters"));
    }

    #[test]
    fn blight_rules_text_plural() {
        let text = blight_rules_text(2);
        assert!(text.contains("2 -1/-1 counters"));
    }
}
