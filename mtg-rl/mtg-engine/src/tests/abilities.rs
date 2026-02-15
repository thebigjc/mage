// Tests extracted from game.rs

use crate::game::*;
use crate::abilities::{Ability, Cost, Effect, TargetSpec, StaticEffect, ModalMode};
use crate::card::CardData;
use crate::combat::CombatState;
use crate::constants::{CardType, Color, KeywordAbilities, Outcome, PhaseStep, SubType, SuperType, Zone};
use crate::counters::CounterType;
use crate::decision::{AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction, PlayerDecisionMaker, ReplacementEffectChoice, TargetRequirement, UnpaidMana};
use crate::events::{EventType, GameEvent};
use crate::mana::{Mana, ManaCost};
use crate::permanent::Permanent;
use crate::state::StateBasedActions;
use crate::types::{AbilityId, ObjectId, PlayerId};
use crate::watchers::WatcherManager;


use super::*;

#[cfg(test)]

    struct AlwaysPassDM;
    impl PlayerDecisionMaker for AlwaysPassDM {
        fn priority(&mut self, _: &GameView, actions: &[PlayerAction]) -> PlayerAction {
            actions.iter().find(|a| matches!(a, PlayerAction::Pass)).cloned().unwrap_or(PlayerAction::Pass)
        }
        fn choose_targets(&mut self, _: &GameView, _: crate::constants::Outcome, req: &TargetRequirement) -> Vec<ObjectId> {
            if req.min_targets > 0 && !req.legal_targets.is_empty() { vec![req.legal_targets[0]] } else { vec![] }
        }
        fn choose_use(&mut self, _: &GameView, _: crate::constants::Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView, modes: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView, a: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![(a.targets[0], a.total_damage)] }
        fn choose_mulligan(&mut self, _: &GameView, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView, hand: &[ObjectId], count: usize) -> Vec<ObjectId> { hand.iter().take(count).copied().collect() }
        fn choose_amount(&mut self, _: &GameView, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView, _: &UnpaidMana, abilities: &[PlayerAction]) -> Option<PlayerAction> { abilities.first().cloned() }
        fn choose_replacement_effect(&mut self, _: &GameView, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView, _: crate::constants::Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView, _: crate::constants::Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn setup_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            starting_life: 20,
            players: vec![
                PlayerConfig { name: "P1".into(), deck: vec![] },
                PlayerConfig { name: "P2".into(), deck: vec![] },
            ],
        };
        let game = Game::new_two_player(config, vec![
            (p1, Box::new(AlwaysPassDM)),
            (p2, Box::new(AlwaysPassDM)),
        ]);
        (game, p1, p2)
    }

    #[test]
    fn tap_self_taps_source() {
        let (mut game, p1, _p2) = setup_game();

        // Create a creature with an activated ability that taps self as part of its effect
        let creature_id = ObjectId::new();
        let mut card = CardData::new(creature_id, p1, "Self Tapper");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        card.abilities = vec![Ability::activated(creature_id,
            "Pay 1: Tap this creature.",
            vec![Cost::pay_mana("{1}")],
            vec![Effect::TapSelf],
            TargetSpec::None)];

        let perm = crate::permanent::Permanent::new(card.clone(), p1);
        game.state.card_store.insert(card.clone());
        game.state.battlefield.add(perm);
        for ability in &card.abilities {
            game.state.ability_store.add(ability.clone());
        }

        // Creature should start untapped
        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(!perm.tapped, "should start untapped");

        // Execute TapSelf effect with source
        game.execute_effects(
            &[Effect::TapSelf],
            p1,
            &[],
            Some(creature_id),
            None,
        );

        // Creature should now be tapped
        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(perm.tapped, "should be tapped after TapSelf");
    }

    #[test]
    fn return_all_type_from_graveyard() {
        let (mut game, p1, _p2) = setup_game();

        // Put some Goblins and non-Goblins in the graveyard
        let goblin1_id = ObjectId::new();
        let mut goblin1 = CardData::new(goblin1_id, p1, "Goblin Warrior");
        goblin1.card_types = vec![CardType::Creature];
        goblin1.subtypes = vec![SubType::Goblin, SubType::Warrior];
        goblin1.power = Some(2);
        goblin1.toughness = Some(1);

        let goblin2_id = ObjectId::new();
        let mut goblin2 = CardData::new(goblin2_id, p1, "Goblin Shaman");
        goblin2.card_types = vec![CardType::Creature];
        goblin2.subtypes = vec![SubType::Goblin];
        goblin2.power = Some(1);
        goblin2.toughness = Some(1);

        let elf_id = ObjectId::new();
        let mut elf = CardData::new(elf_id, p1, "Llanowar Elves");
        elf.card_types = vec![CardType::Creature];
        elf.subtypes = vec![SubType::Elf];
        elf.power = Some(1);
        elf.toughness = Some(1);

        game.state.card_store.insert(goblin1.clone());
        game.state.card_store.insert(goblin2.clone());
        game.state.card_store.insert(elf.clone());

        game.state.players.get_mut(&p1).unwrap().graveyard.add(goblin1_id);
        game.state.players.get_mut(&p1).unwrap().graveyard.add(goblin2_id);
        game.state.players.get_mut(&p1).unwrap().graveyard.add(elf_id);

        // Execute ReturnAllTypeFromGraveyard for Goblins
        game.execute_effects(
            &[Effect::ReturnAllTypeFromGraveyard { creature_type: "Goblin".into() }],
            p1,
            &[],
            None,
            None,
        );

        // Both goblins should be on the battlefield
        assert!(game.state.battlefield.get(goblin1_id).is_some(), "goblin1 should be on battlefield");
        assert!(game.state.battlefield.get(goblin2_id).is_some(), "goblin2 should be on battlefield");
        // Elf should still be in graveyard
        let gy = &game.state.players.get(&p1).unwrap().graveyard;
        assert!(gy.iter().any(|&id| id == elf_id), "elf should still be in graveyard");
        assert!(!gy.iter().any(|&id| id == goblin1_id), "goblin1 should not be in graveyard");
    }

    #[test]
    fn create_token_dynamic_count() {
        let (mut game, p1, _p2) = setup_game();

        // Put 3 Elf cards in graveyard
        for i in 0..3 {
            let elf_id = ObjectId::new();
            let mut elf = CardData::new(elf_id, p1, &format!("Dead Elf {}", i));
            elf.card_types = vec![CardType::Creature];
            elf.subtypes = vec![SubType::Elf];
            elf.power = Some(1);
            elf.toughness = Some(1);
            game.state.card_store.insert(elf.clone());
            game.state.players.get_mut(&p1).unwrap().graveyard.add(elf_id);
        }

        // Create tokens equal to Elf cards in graveyard
        game.execute_effects(
            &[Effect::CreateTokenDynamic {
                token_name: "2/2 green Elf Warrior creature token".into(),
                count_filter: "Elf cards in your graveyard".into(),
            }],
            p1,
            &[],
            None,
            None,
        );

        // Should have 3 tokens on the battlefield
        let tokens: Vec<_> = game.state.battlefield.iter()
            .filter(|p| p.controller == p1 && p.card.is_token)
            .collect();
        assert_eq!(tokens.len(), 3, "should have 3 elf tokens");
    }
