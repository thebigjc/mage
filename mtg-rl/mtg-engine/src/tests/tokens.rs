// Tests extracted from game.rs

use crate::game::*;
use crate::abilities::Effect;
use crate::card::CardData;
use crate::permanent::Permanent;
use crate::types::{ObjectId, PlayerId};

#[cfg(test)]

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
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig { players: vec![PlayerConfig { name: "P1".into(), deck: vec![] }, PlayerConfig { name: "P2".into(), deck: vec![] }], starting_life: 20 };
        let game = Game::new_two_player(config, vec![(p1, Box::new(PassPlayer)), (p2, Box::new(PassPlayer))]);
        (game, p1, p2)
    }

    #[test]
    fn token_copy_basic() {
        let (mut game, p1, _p2) = make_game();

        // Create a creature to copy
        let src_id = ObjectId::new();
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

        let src_id = ObjectId::new();
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

        let src_id = ObjectId::new();
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

        let src_id = ObjectId::new();
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

    #[test]
    fn replace_token_creation_with_equipped_creature_copy() {
        let (mut game, p1, _p2) = make_game();

        let creature_id = ObjectId::new();
        let equip_id = ObjectId::new();

        let creature_card = CardData {
            id: creature_id, owner: p1, name: "Siege Rhino".into(),
            card_types: vec![crate::constants::CardType::Creature],
            subtypes: vec![crate::constants::SubType::Rhino],
            power: Some(4), toughness: Some(5),
            keywords: crate::constants::KeywordAbilities::TRAMPLE,
            ..Default::default()
        };
        game.state.battlefield.add(Permanent::new(creature_card.clone(), p1));
        game.state.card_store.insert(creature_card);

        let mut equip_card = CardData::new(equip_id, p1, "Mirrormind Crown");
        equip_card.card_types = vec![crate::constants::CardType::Artifact];
        equip_card.subtypes = vec![crate::constants::SubType::Equipment];
        equip_card.abilities = vec![
            crate::abilities::Ability::static_ability(equip_id,
                "Replace token creation.",
                vec![crate::abilities::StaticEffect::replace_token_creation()]),
        ];
        game.state.battlefield.add(Permanent::new(equip_card.clone(), p1));
        game.state.card_store.insert(equip_card);
        let abilities = game.state.battlefield.get(equip_id).unwrap().card.abilities.clone();
        for ab in abilities { game.state.ability_store.add(ab); }

        game.execute_effects(&[Effect::Equip], p1, &[creature_id], Some(equip_id), None);
        game.apply_continuous_effects();

        assert!(!game.state.token_replacement_effects.is_empty());

        game.execute_effects(&[Effect::create_token("1/1 Soldier", 2)], p1, &[], None, None);

        let tokens: Vec<_> = game.state.battlefield.iter()
            .filter(|p| p.card.is_token && p.controller == p1)
            .collect();
        assert_eq!(tokens.len(), 2, "should have 2 token copies");
        for t in &tokens {
            assert_eq!(t.card.name, "Siege Rhino", "tokens should be copies of equipped creature");
            assert_eq!(t.card.power, Some(4));
            assert_eq!(t.card.toughness, Some(5));
            assert!(t.has_keyword(crate::constants::KeywordAbilities::TRAMPLE));
        }
    }

    #[test]
    fn replace_token_creation_only_first_per_turn() {
        let (mut game, p1, _p2) = make_game();

        let creature_id = ObjectId::new();
        let equip_id = ObjectId::new();

        let creature_card = CardData {
            id: creature_id, owner: p1, name: "Angel".into(),
            card_types: vec![crate::constants::CardType::Creature],
            power: Some(4), toughness: Some(4),
            keywords: crate::constants::KeywordAbilities::FLYING,
            ..Default::default()
        };
        game.state.battlefield.add(Permanent::new(creature_card.clone(), p1));
        game.state.card_store.insert(creature_card);

        let mut equip_card = CardData::new(equip_id, p1, "Mirrormind Crown");
        equip_card.card_types = vec![crate::constants::CardType::Artifact];
        equip_card.subtypes = vec![crate::constants::SubType::Equipment];
        equip_card.abilities = vec![
            crate::abilities::Ability::static_ability(equip_id,
                "Replace token creation.",
                vec![crate::abilities::StaticEffect::replace_token_creation()]),
        ];
        game.state.battlefield.add(Permanent::new(equip_card.clone(), p1));
        game.state.card_store.insert(equip_card);
        let abilities = game.state.battlefield.get(equip_id).unwrap().card.abilities.clone();
        for ab in abilities { game.state.ability_store.add(ab); }

        game.execute_effects(&[Effect::Equip], p1, &[creature_id], Some(equip_id), None);
        game.apply_continuous_effects();

        game.execute_effects(&[Effect::create_token("1/1 Goblin", 1)], p1, &[], None, None);
        let first_tokens: Vec<_> = game.state.battlefield.iter()
            .filter(|p| p.card.is_token && p.controller == p1)
            .collect();
        assert_eq!(first_tokens.len(), 1);
        assert_eq!(first_tokens[0].card.name, "Angel", "first token should be an Angel copy");

        game.execute_effects(&[Effect::create_token("1/1 Goblin", 1)], p1, &[], None, None);
        let goblins: Vec<_> = game.state.battlefield.iter()
            .filter(|p| p.card.is_token && p.controller == p1 && p.card.name == "1/1 Goblin")
            .collect();
        assert_eq!(goblins.len(), 1, "second token creation should NOT be replaced");
    }

    #[test]
    fn replace_token_creation_requires_equipped_creature() {
        let (mut game, p1, _p2) = make_game();

        let equip_id = ObjectId::new();

        let mut equip_card = CardData::new(equip_id, p1, "Mirrormind Crown");
        equip_card.card_types = vec![crate::constants::CardType::Artifact];
        equip_card.subtypes = vec![crate::constants::SubType::Equipment];
        equip_card.abilities = vec![
            crate::abilities::Ability::static_ability(equip_id,
                "Replace token creation.",
                vec![crate::abilities::StaticEffect::replace_token_creation()]),
        ];
        game.state.battlefield.add(Permanent::new(equip_card.clone(), p1));
        game.state.card_store.insert(equip_card);
        let abilities = game.state.battlefield.get(equip_id).unwrap().card.abilities.clone();
        for ab in abilities { game.state.ability_store.add(ab); }

        game.apply_continuous_effects();

        assert!(game.state.token_replacement_effects.is_empty(), "no replacement without equipped creature");

        game.execute_effects(&[Effect::create_token("1/1 Soldier", 1)], p1, &[], None, None);
        let tokens: Vec<_> = game.state.battlefield.iter()
            .filter(|p| p.card.is_token && p.controller == p1)
            .collect();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].card.name, "1/1 Soldier", "token should be normal soldier, not a copy");
    }

