// Mobilize N — TDM (Tarkir: Dragonstorm) set-specific keyword.
//
// "Whenever this creature attacks, create N 1/1 red Warrior creature tokens
// that are tapped and attacking. Sacrifice them at the beginning of the
// next end step."
//
// This is an attacks-triggered ability. The tokens are temporary — they
// exist only for the combat they're created in and are sacrificed afterward.
//
// Ported from: mage.abilities.keyword.MobilizeAbility

use mtg_engine::abilities::{Ability, Effect, TargetSpec};
use mtg_engine::types::ObjectId;

/// Create a Mobilize N ability for a card.
///
/// Returns a triggered ability: "Whenever this creature attacks, create N
/// 1/1 red Warrior creature tokens tapped and attacking. Sacrifice them
/// at the beginning of the next end step."
pub fn mobilize(source_id: ObjectId, count: u32) -> Ability {
    let rules_text = if count == 1 {
        "Mobilize 1 (Whenever this creature attacks, create a 1/1 red Warrior \
         creature token tapped and attacking. Sacrifice it at the beginning of \
         the next end step.)"
            .to_string()
    } else {
        format!(
            "Mobilize {} (Whenever this creature attacks, create {} 1/1 red Warrior \
             creature tokens tapped and attacking. Sacrifice them at the beginning of \
             the next end step.)",
            count, count,
        )
    };

    Ability::attacks_triggered(
        source_id,
        &rules_text,
        vec![Effect::create_token_tapped_attacking("1/1 red Warrior", count)],
        TargetSpec::None,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use mtg_engine::constants::AbilityType;
    use mtg_engine::events::{EventType, GameEvent};

    #[test]
    fn mobilize_1_creates_triggered_ability() {
        let source = ObjectId::new();
        let ability = mobilize(source, 1);

        assert_eq!(ability.ability_type, AbilityType::TriggeredNonMana);
        assert!(ability.trigger_events.contains(&EventType::AttackerDeclared));
        assert_eq!(ability.effects.len(), 1);
        assert!(ability.rules_text.contains("Mobilize 1"));

        // Should trigger when this creature attacks
        let event = GameEvent::new(EventType::AttackerDeclared).source(source);
        assert!(ability.should_trigger(&event));
    }

    #[test]
    fn mobilize_2_creates_two_tokens() {
        let source = ObjectId::new();
        let ability = mobilize(source, 2);

        assert!(ability.rules_text.contains("Mobilize 2"));
        match &ability.effects[0] {
            Effect::CreateTokenTappedAttacking { token_name, count } => {
                assert_eq!(token_name, "1/1 red Warrior");
                assert_eq!(*count, 2);
            }
            other => panic!("Expected CreateTokenTappedAttacking, got {:?}", other),
        }
    }

    #[test]
    fn mobilize_3() {
        let source = ObjectId::new();
        let ability = mobilize(source, 3);

        assert!(ability.rules_text.contains("Mobilize 3"));
        match &ability.effects[0] {
            Effect::CreateTokenTappedAttacking { count, .. } => {
                assert_eq!(*count, 3);
            }
            other => panic!("Expected CreateTokenTappedAttacking, got {:?}", other),
        }
    }
}
