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

    struct PassivePlayer;
    impl PlayerDecisionMaker for PassivePlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { true }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn setup() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "P1".into(), deck: vec![] },
                PlayerConfig { name: "P2".into(), deck: vec![] },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(PassivePlayer)),
                (p2, Box::new(PassivePlayer)),
            ],
        );
        (game, p1, p2)
    }

    fn make_creature(id: ObjectId, owner: PlayerId, name: &str, power: i32, toughness: i32) -> CardData {
        let mut card = CardData::new(id, owner, name);
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![SubType::Human];
        card.power = Some(power);
        card.toughness = Some(toughness);
        card
    }

    fn make_equipment(id: ObjectId, owner: PlayerId, name: &str, power_boost: i32, toughness_boost: i32) -> CardData {
        let mut card = CardData::new(id, owner, name);
        card.card_types = vec![CardType::Artifact];
        card.subtypes = vec![SubType::Equipment];
        card.mana_cost = ManaCost::parse("{1}");
        card.abilities = vec![
            Ability::static_ability(id,
                "Equipped creature gets boost.",
                vec![StaticEffect::Boost {
                    filter: "equipped creature".into(),
                    power: power_boost,
                    toughness: toughness_boost,
                }]),
            Ability::activated(id,
                "Equip {1}",
                vec![Cost::pay_mana("{1}")],
                vec![Effect::equip()],
                TargetSpec::CreatureYouControl),
        ];
        card
    }

    fn register_abilities(game: &mut Game, perm_id: ObjectId) {
        let abilities = game.state.battlefield.get(perm_id).unwrap().card.abilities.clone();
        for ability in abilities {
            game.state.ability_store.add(ability);
        }
    }

    #[test]
    fn equip_attaches_equipment_to_creature() {
        let (mut game, p1, _p2) = setup();
        let creature_id = ObjectId::new();
        let equip_id = ObjectId::new();

        game.state.battlefield.add(Permanent::new(make_creature(creature_id, p1, "Soldier", 2, 2), p1));
        game.state.battlefield.add(Permanent::new(make_equipment(equip_id, p1, "Short Sword", 1, 1), p1));
        register_abilities(&mut game, equip_id);

        game.execute_effects(&[Effect::equip()], p1, &[creature_id], Some(equip_id), None);

        let equip = game.state.battlefield.get(equip_id).unwrap();
        assert_eq!(equip.attached_to, Some(creature_id));
        let creature = game.state.battlefield.get(creature_id).unwrap();
        assert!(creature.attachments.contains(&equip_id));
    }

    #[test]
    fn equipped_creature_gets_stat_boost() {
        let (mut game, p1, _p2) = setup();
        let creature_id = ObjectId::new();
        let equip_id = ObjectId::new();

        game.state.battlefield.add(Permanent::new(make_creature(creature_id, p1, "Soldier", 2, 2), p1));
        game.state.battlefield.add(Permanent::new(make_equipment(equip_id, p1, "Short Sword", 1, 1), p1));
        register_abilities(&mut game, equip_id);

        assert_eq!(game.state.battlefield.get(creature_id).unwrap().power(), 2);
        game.execute_effects(&[Effect::equip()], p1, &[creature_id], Some(equip_id), None);
        game.apply_continuous_effects();

        assert_eq!(game.state.battlefield.get(creature_id).unwrap().power(), 3);
        assert_eq!(game.state.battlefield.get(creature_id).unwrap().toughness(), 3);
    }

    #[test]
    fn equipment_detaches_when_creature_leaves() {
        let (mut game, p1, _p2) = setup();
        let creature_id = ObjectId::new();
        let equip_id = ObjectId::new();

        game.state.battlefield.add(Permanent::new(make_creature(creature_id, p1, "Soldier", 2, 2), p1));
        game.state.battlefield.add(Permanent::new(make_equipment(equip_id, p1, "Short Sword", 1, 1), p1));
        register_abilities(&mut game, equip_id);

        game.execute_effects(&[Effect::equip()], p1, &[creature_id], Some(equip_id), None);
        assert_eq!(game.state.battlefield.get(equip_id).unwrap().attached_to, Some(creature_id));

        // Remove creature (simulating death)
        game.state.battlefield.remove(creature_id);

        let sba = game.state.check_state_based_actions();
        assert!(sba.attachments_to_detach.contains(&equip_id));

        game.apply_state_based_actions(&sba);
        let equip = game.state.battlefield.get(equip_id).unwrap();
        assert_eq!(equip.attached_to, None);
        assert!(game.state.battlefield.contains(equip_id));
    }

    #[test]
    fn re_equip_moves_to_new_creature() {
        let (mut game, p1, _p2) = setup();
        let c1 = ObjectId::new();
        let c2 = ObjectId::new();
        let equip_id = ObjectId::new();

        game.state.battlefield.add(Permanent::new(make_creature(c1, p1, "Soldier A", 2, 2), p1));
        game.state.battlefield.add(Permanent::new(make_creature(c2, p1, "Soldier B", 3, 3), p1));
        game.state.battlefield.add(Permanent::new(make_equipment(equip_id, p1, "Short Sword", 1, 1), p1));
        register_abilities(&mut game, equip_id);

        game.execute_effects(&[Effect::equip()], p1, &[c1], Some(equip_id), None);
        assert_eq!(game.state.battlefield.get(equip_id).unwrap().attached_to, Some(c1));

        game.execute_effects(&[Effect::equip()], p1, &[c2], Some(equip_id), None);
        assert_eq!(game.state.battlefield.get(equip_id).unwrap().attached_to, Some(c2));
        assert!(game.state.battlefield.get(c2).unwrap().attachments.contains(&equip_id));
        assert!(!game.state.battlefield.get(c1).unwrap().attachments.contains(&equip_id));

        game.apply_continuous_effects();
        assert_eq!(game.state.battlefield.get(c1).unwrap().power(), 2);
        assert_eq!(game.state.battlefield.get(c2).unwrap().power(), 4);
    }

    #[test]
    fn equipment_keyword_grant() {
        let (mut game, p1, _p2) = setup();
        let creature_id = ObjectId::new();
        let equip_id = ObjectId::new();

        game.state.battlefield.add(Permanent::new(make_creature(creature_id, p1, "Soldier", 2, 2), p1));

        let mut equipment = CardData::new(equip_id, p1, "Swiftfoot Boots");
        equipment.card_types = vec![CardType::Artifact];
        equipment.subtypes = vec![SubType::Equipment];
        equipment.mana_cost = ManaCost::parse("{2}");
        equipment.abilities = vec![
            Ability::static_ability(equip_id,
                "Equipped creature has hexproof and haste.",
                vec![StaticEffect::GrantKeyword {
                    filter: "equipped creature".into(),
                    keyword: "hexproof, haste".into(),
                }]),
            Ability::activated(equip_id,
                "Equip {1}",
                vec![Cost::pay_mana("{1}")],
                vec![Effect::equip()],
                TargetSpec::CreatureYouControl),
        ];
        game.state.battlefield.add(Permanent::new(equipment, p1));
        register_abilities(&mut game, equip_id);

        assert!(!game.state.battlefield.get(creature_id).unwrap().has_hexproof());
        assert!(!game.state.battlefield.get(creature_id).unwrap().has_haste());

        game.execute_effects(&[Effect::equip()], p1, &[creature_id], Some(equip_id), None);
        game.apply_continuous_effects();

        assert!(game.state.battlefield.get(creature_id).unwrap().has_hexproof());
        assert!(game.state.battlefield.get(creature_id).unwrap().has_haste());
    }

// ---------------------------------------------------------------------------
// Aura tests
// ---------------------------------------------------------------------------



    struct PassivePlayer2;
    impl PlayerDecisionMaker for PassivePlayer2 {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { true }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn setup2() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "P1".into(), deck: vec![] },
                PlayerConfig { name: "P2".into(), deck: vec![] },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(PassivePlayer)),
                (p2, Box::new(PassivePlayer)),
            ],
        );
        (game, p1, p2)
    }

    fn make_creature2(id: ObjectId, owner: PlayerId, power: i32, toughness: i32) -> CardData {
        let mut card = CardData::new(id, owner, "Creature");
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![SubType::Human];
        card.power = Some(power);
        card.toughness = Some(toughness);
        card
    }

    fn make_aura_boost(id: ObjectId, owner: PlayerId, name: &str, power: i32, toughness: i32) -> CardData {
        let mut card = CardData::new(id, owner, name);
        card.card_types = vec![CardType::Enchantment];
        card.subtypes = vec![SubType::Aura];
        card.abilities = vec![
            Ability::static_ability(id,
                &format!("Enchanted creature gets +{power}/+{toughness}."),
                vec![StaticEffect::Boost {
                    filter: "enchanted creature".into(),
                    power,
                    toughness,
                }]),
        ];
        card
    }

    fn register_abilities2(game: &mut Game, perm_id: ObjectId) {
        let abilities = game.state.battlefield.get(perm_id).unwrap().card.abilities.clone();
        for ability in abilities {
            game.state.ability_store.add(ability);
        }
    }

    #[test]
    fn aura_attached_creature_gets_boost() {
        let (mut game, p1, _p2) = setup();
        let creature_id = ObjectId::new();
        let aura_id = ObjectId::new();

        game.state.battlefield.add(Permanent::new(make_creature2(creature_id, p1, 2, 2), p1));

        let aura = make_aura_boost(aura_id, p1, "Giant Growth Aura", 3, 3);
        game.state.battlefield.add(Permanent::new(aura, p1));
        register_abilities2(&mut game, aura_id);

        // Manually attach aura to creature
        if let Some(a) = game.state.battlefield.get_mut(aura_id) {
            a.attach_to(creature_id);
        }
        if let Some(c) = game.state.battlefield.get_mut(creature_id) {
            c.add_attachment(aura_id);
        }

        game.apply_continuous_effects();

        assert_eq!(game.state.battlefield.get(creature_id).unwrap().power(), 5);
        assert_eq!(game.state.battlefield.get(creature_id).unwrap().toughness(), 5);
    }

    #[test]
    fn aura_falls_off_to_graveyard_when_creature_dies() {
        let (mut game, p1, _p2) = setup();
        let creature_id = ObjectId::new();
        let aura_id = ObjectId::new();

        game.state.battlefield.add(Permanent::new(make_creature(creature_id, p1, "Creature", 2, 2), p1));
        let aura = make_aura_boost(aura_id, p1, "Ethereal Armor", 2, 2);
        game.state.battlefield.add(Permanent::new(aura, p1));
        register_abilities2(&mut game, aura_id);

        // Attach
        if let Some(a) = game.state.battlefield.get_mut(aura_id) {
            a.attach_to(creature_id);
        }
        if let Some(c) = game.state.battlefield.get_mut(creature_id) {
            c.add_attachment(aura_id);
        }

        // Remove creature (simulating death)
        game.state.battlefield.remove(creature_id);

        // SBA should detect aura needs to go to graveyard
        let sba = game.state.check_state_based_actions();
        assert!(sba.auras_to_graveyard.contains(&aura_id));
        assert!(!sba.attachments_to_detach.contains(&aura_id));

        game.apply_state_based_actions(&sba);

        // Aura should be gone from battlefield (moved to graveyard)
        assert!(!game.state.battlefield.contains(aura_id));
    }

    #[test]
    fn pacifism_prevents_attack_and_block() {
        let (mut game, p1, p2) = setup();
        let creature_id = ObjectId::new();
        let pacifism_id = ObjectId::new();

        game.state.battlefield.add(Permanent::new(make_creature(creature_id, p2, "Creature", 3, 3), p2));

        // Make Pacifism aura
        let mut pacifism = CardData::new(pacifism_id, p1, "Pacifism");
        pacifism.card_types = vec![CardType::Enchantment];
        pacifism.subtypes = vec![SubType::Aura];
        pacifism.abilities = vec![
            Ability::static_ability(pacifism_id,
                "Enchanted creature can't attack or block.",
                vec![
                    StaticEffect::CantAttack { filter: "enchanted creature".into() },
                    StaticEffect::CantBlock { filter: "enchanted creature".into() },
                ]),
        ];
        game.state.battlefield.add(Permanent::new(pacifism, p1));
        register_abilities(&mut game, pacifism_id);

        // Attach to creature
        if let Some(a) = game.state.battlefield.get_mut(pacifism_id) {
            a.attach_to(creature_id);
        }
        if let Some(c) = game.state.battlefield.get_mut(creature_id) {
            c.add_attachment(pacifism_id);
        }

        // Apply continuous effects to enforce CantAttack/CantBlock
        game.apply_continuous_effects();

        // After Pacifism, creature should not be able to attack
        let creature = game.state.battlefield.get(creature_id).unwrap();
        assert!(!creature.can_attack(), "Pacified creature should not be able to attack");
    }

// ---------------------------------------------------------------------------
// Prowess and landwalk tests
// ---------------------------------------------------------------------------

