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
    use uuid::Uuid;

    struct PassPlayer;
    impl crate::decision::PlayerDecisionMaker for PassPlayer {
        fn priority(&mut self, _: &crate::decision::GameView, actions: &[crate::decision::PlayerAction]) -> crate::decision::PlayerAction { actions[0].clone() }
        fn choose_targets(&mut self, _: &crate::decision::GameView, _: crate::constants::Outcome, req: &crate::decision::TargetRequirement) -> Vec<ObjectId> { req.legal_targets.iter().take(1).copied().collect() }
        fn choose_use(&mut self, _: &crate::decision::GameView, _: crate::constants::Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &crate::decision::GameView, _: &[crate::decision::NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &crate::decision::GameView, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &crate::decision::GameView, _: &[crate::decision::AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &crate::decision::GameView, _: &crate::decision::DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &crate::decision::GameView, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &crate::decision::GameView, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &crate::decision::GameView, hand: &[ObjectId], count: usize) -> Vec<ObjectId> { hand.iter().take(count).copied().collect() }
        fn choose_amount(&mut self, _: &crate::decision::GameView, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &crate::decision::GameView, _: &crate::decision::UnpaidMana, _: &[crate::decision::PlayerAction]) -> Option<crate::decision::PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &crate::decision::GameView, _: &[crate::decision::ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &crate::decision::GameView, _: crate::constants::Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &crate::decision::GameView, _: crate::constants::Outcome, _: &str, _: &[crate::decision::NamedChoice]) -> usize { 0 }
    }

    fn make_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId(Uuid::new_v4());
        let p2 = PlayerId(Uuid::new_v4());
        let config = GameConfig { players: vec![PlayerConfig { name: "P1".into(), deck: vec![] }, PlayerConfig { name: "P2".into(), deck: vec![] }], starting_life: 20 };
        let game = Game::new_two_player(config, vec![(p1, Box::new(PassPlayer)), (p2, Box::new(PassPlayer))]);
        (game, p1, p2)
    }

    #[test]
    fn token_copy_basic() {
        let (mut game, p1, _p2) = make_game();

        // Create a creature to copy
        let src_id = ObjectId(Uuid::new_v4());
        let src_card = CardData {
            id: src_id, owner: p1, name: "Goblin Lord".into(),
            card_types: vec![crate::constants::CardType::Creature],
            subtypes: vec![crate::constants::SubType::Goblin],
            power: Some(3), toughness: Some(3),
            keywords: crate::constants::KeywordAbilities::MENACE,
            ..Default::default()
        };
        let perm = Permanent::new(src_card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(src_card);

        // Create a token copy
        game.execute_effects(&[Effect::create_token_copy(1)], p1, &[src_id], None, None);

        // Should now have 2 creatures on BF
        let creatures: Vec<_> = game.state.battlefield.iter()
            .filter(|p| p.is_creature() && p.controller == p1)
            .collect();
        assert_eq!(creatures.len(), 2, "should have original + token copy");

        // Find the token (not the original)
        let token = creatures.iter().find(|p| p.id() != src_id).unwrap();
        assert_eq!(token.card.name, "Goblin Lord");
        assert_eq!(token.power(), 3);
        assert_eq!(token.toughness(), 3);
        assert!(token.card.is_token);
        assert!(token.has_subtype(&crate::constants::SubType::Goblin));
        assert!(token.has_keyword(crate::constants::KeywordAbilities::MENACE));
    }

    #[test]
    fn token_copy_with_haste() {
        let (mut game, p1, _p2) = make_game();

        let src_id = ObjectId(Uuid::new_v4());
        let src_card = CardData {
            id: src_id, owner: p1, name: "Big Dragon".into(),
            card_types: vec![crate::constants::CardType::Creature],
            subtypes: vec![crate::constants::SubType::Custom("Dragon".into())],
            power: Some(5), toughness: Some(5),
            keywords: crate::constants::KeywordAbilities::FLYING,
            ..Default::default()
        };
        let perm = Permanent::new(src_card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(src_card);

        // Copy with haste
        game.execute_effects(&[Effect::create_token_copy_with_haste(1)], p1, &[src_id], None, None);

        let token = game.state.battlefield.iter()
            .find(|p| p.id() != src_id && p.is_creature())
            .unwrap();
        assert!(token.has_keyword(crate::constants::KeywordAbilities::FLYING), "should have flying from original");
        assert!(token.has_keyword(crate::constants::KeywordAbilities::HASTE), "should have haste from modification");
    }

    #[test]
    fn token_copy_with_changeling() {
        let (mut game, p1, _p2) = make_game();

        let src_id = ObjectId(Uuid::new_v4());
        let src_card = CardData {
            id: src_id, owner: p1, name: "Elf Warrior".into(),
            card_types: vec![crate::constants::CardType::Creature],
            subtypes: vec![crate::constants::SubType::Elf, crate::constants::SubType::Warrior],
            power: Some(2), toughness: Some(2),
            ..Default::default()
        };
        let perm = Permanent::new(src_card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(src_card);

        // Copy with changeling
        game.execute_effects(&[Effect::create_token_copy_with_changeling(1)], p1, &[src_id], None, None);

        let token = game.state.battlefield.iter()
            .find(|p| p.id() != src_id && p.is_creature())
            .unwrap();
        assert!(token.has_keyword(crate::constants::KeywordAbilities::CHANGELING));
        assert!(token.has_subtype(&crate::constants::SubType::Goblin), "changeling has all types");
        assert!(token.has_subtype(&crate::constants::SubType::Elf), "should still be an Elf too");
    }

    #[test]
    fn token_copy_emits_etb() {
        let (mut game, p1, _p2) = make_game();

        let src_id = ObjectId(Uuid::new_v4());
        let src_card = CardData {
            id: src_id, owner: p1, name: "Bear".into(),
            card_types: vec![crate::constants::CardType::Creature],
            power: Some(2), toughness: Some(2),
            ..Default::default()
        };
        let perm = Permanent::new(src_card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(src_card);

        game.event_log.clear();
        game.execute_effects(&[Effect::create_token_copy(1)], p1, &[src_id], None, None);

        let etb_count = game.event_log.iter()
            .filter(|e| e.event_type == crate::events::EventType::EnteredTheBattlefield)
            .count();
        assert_eq!(etb_count, 1, "token copy should emit ETB event");
    }

