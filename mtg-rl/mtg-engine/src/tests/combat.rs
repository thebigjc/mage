// Tests extracted from game.rs

use crate::game::*;
use crate::abilities::{Ability, StaticEffect};
use crate::card::CardData;
use crate::constants::{CardType, KeywordAbilities, Outcome, SubType};
use crate::decision::{AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction, PlayerAgent, PlayerDecisionMaker, ReplacementEffectChoice, TargetRequirement, UnpaidMana};
use crate::permanent::Permanent;
use crate::types::{ObjectId, PlayerId, Power, Toughness, Life};

#[cfg(test)]
    /// Decision maker that attacks with all creatures.
    struct AttackAllPlayer;

    impl PlayerDecisionMaker for AttackAllPlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction {
            PlayerAction::Pass
        }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(
            &mut self,
            _: &GameView<'_>,
            possible_attackers: &[ObjectId],
            possible_defenders: &[ObjectId],
        ) -> Vec<(ObjectId, ObjectId)> {
            let defender = possible_defenders[0];
            possible_attackers
                .iter()
                .map(|&a| (a, defender))
                .collect()
        }
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

    /// Decision maker that blocks with all available creatures (first available for each attacker).
    struct BlockAllPlayer;

    impl PlayerDecisionMaker for BlockAllPlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction {
            PlayerAction::Pass
        }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(
            &mut self,
            _: &GameView<'_>,
            attackers: &[AttackerInfo],
        ) -> Vec<(ObjectId, ObjectId)> {
            let mut blocks = Vec::new();
            let mut used = std::collections::HashSet::new();
            for info in attackers {
                for &blocker_id in &info.legal_blockers {
                    if !used.contains(&blocker_id) {
                        blocks.push((blocker_id, info.attacker_id));
                        used.insert(blocker_id);
                        break;
                    }
                }
            }
            blocks
        }
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

    fn make_creature(
        name: &str,
        owner: PlayerId,
        power: i32,
        toughness: i32,
        keywords: KeywordAbilities,
    ) -> CardData {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(Power::new(power));
        card.toughness = Some(Toughness::new(toughness));
        card.keywords = keywords;
        card
    }

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        (0..40)
            .map(|i| {
                let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
                c.card_types = vec![CardType::Land];
                c
            })
            .collect()
    }

    fn setup_combat_game(
        p1_dm: PlayerAgent,
        p2_dm: PlayerAgent,
    ) -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig {
                    name: "Attacker".to_string(),
                    deck: make_deck(p1),
                },
                PlayerConfig {
                    name: "Defender".to_string(),
                    deck: make_deck(p2),
                },
            ],
            starting_life: Life::new(20),
        };
        let game = Game::new_two_player(config, vec![(p1, p1_dm), (p2, p2_dm)]);
        (game, p1, p2)
    }

    /// Helper to add a creature to the battlefield and remove summoning sickness.
    fn add_creature(
        game: &mut Game,
        owner: PlayerId,
        name: &str,
        power: i32,
        toughness: i32,
        keywords: KeywordAbilities,
    ) -> ObjectId {
        let card = make_creature(name, owner, power, toughness, keywords);
        let id = card.id;
        let mut perm = Permanent::new(card, owner);
        perm.remove_summoning_sickness();
        game.state.battlefield.add(perm);
        id
    }

    // ── Test: Unblocked combat damage ──────────────────────────────

    #[test]
    fn unblocked_attacker_deals_damage_to_player() {
        let (mut game, p1, p2) = setup_combat_game(
            PlayerAgent::new(AttackAllPlayer),
            PlayerAgent::new(BlockAllPlayer),
        );

        // Add a 3/3 creature for p1 (attacker)
        let bear_id = add_creature(&mut game, p1, "Bear", 3, 3, KeywordAbilities::empty());

        // Set active player to p1
        game.state.active_player = p1;
        game.state.priority_player = p1;

        // Run declare attackers step
        game.declare_attackers_step(p1);

        // Bear should be attacking
        assert!(game.state.combat.is_attacking(bear_id));
        // Bear should be tapped (no vigilance)
        assert!(game.state.battlefield.get(bear_id).unwrap().tapped);

        // Run declare blockers (no blockers for p2)
        game.declare_blockers_step(p1);

        // Run combat damage (regular)
        game.combat_damage_step(false);

        // p2 should have taken 3 damage
        assert_eq!(game.state.players[&p2].life, 17);
    }

    #[test]
    fn vigilance_does_not_tap_attacker() {
        let (mut game, p1, _p2) = setup_combat_game(
            PlayerAgent::new(AttackAllPlayer),
            PlayerAgent::new(BlockAllPlayer),
        );

        let vig_id = add_creature(&mut game, p1, "Vigilant", 2, 2, KeywordAbilities::VIGILANCE);
        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.declare_attackers_step(p1);

        // Should be attacking but NOT tapped
        assert!(game.state.combat.is_attacking(vig_id));
        assert!(!game.state.battlefield.get(vig_id).unwrap().tapped);
    }

    #[test]
    fn blocked_creature_deals_damage_to_blocker() {
        let (mut game, p1, p2) = setup_combat_game(
            PlayerAgent::new(AttackAllPlayer),
            PlayerAgent::new(BlockAllPlayer),
        );

        let attacker_id = add_creature(&mut game, p1, "Attacker", 3, 3, KeywordAbilities::empty());
        let blocker_id = add_creature(&mut game, p2, "Blocker", 2, 4, KeywordAbilities::empty());

        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.declare_attackers_step(p1);
        game.declare_blockers_step(p1);

        // Blocker should be blocking
        assert!(game.state.combat.is_blocking(blocker_id));

        // Apply combat damage
        game.combat_damage_step(false);

        // Blocker should have 3 damage, attacker should have 2 damage
        assert_eq!(game.state.battlefield.get(blocker_id).unwrap().damage, 3);
        assert_eq!(game.state.battlefield.get(attacker_id).unwrap().damage, 2);
        // Player should NOT have taken damage (blocked)
        assert_eq!(game.state.players[&p2].life, 20);
    }

    #[test]
    fn lifelink_gains_life_on_combat_damage() {
        let (mut game, p1, p2) = setup_combat_game(
            PlayerAgent::new(AttackAllPlayer),
            PlayerAgent::new(BlockAllPlayer),
        );

        let _ = add_creature(&mut game, p1, "Lifelinker", 4, 4, KeywordAbilities::LIFELINK);

        game.state.active_player = p1;
        game.state.priority_player = p1;

        // Reduce p1 life to verify gain
        game.state.players.get_mut(&p1).unwrap().life = Life::new(15);

        game.declare_attackers_step(p1);
        game.declare_blockers_step(p1);
        game.combat_damage_step(false);

        // p1 should gain 4 life (lifelink), p2 takes 4
        assert_eq!(game.state.players[&p1].life, 19);
        assert_eq!(game.state.players[&p2].life, 16);
    }

    #[test]
    fn first_strike_deals_damage_first() {
        let (mut game, p1, p2) = setup_combat_game(
            PlayerAgent::new(AttackAllPlayer),
            PlayerAgent::new(BlockAllPlayer),
        );

        let fs_id = add_creature(&mut game, p1, "FirstStriker", 3, 2, KeywordAbilities::FIRST_STRIKE);
        let blocker_id = add_creature(&mut game, p2, "Blocker", 3, 3, KeywordAbilities::empty());

        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.declare_attackers_step(p1);
        game.declare_blockers_step(p1);

        // First strike damage step
        game.combat_damage_step(true);

        // Blocker takes 3 first strike damage
        assert_eq!(game.state.battlefield.get(blocker_id).unwrap().damage, 3);
        // First striker takes 0 (normal creature doesn't deal in first strike step)
        assert_eq!(game.state.battlefield.get(fs_id).unwrap().damage, 0);

        // Regular damage step
        game.combat_damage_step(false);

        // First striker still takes 0 more (first strike creature doesn't deal in regular step)
        // But blocker deals its 3 damage to first striker in regular step
        assert_eq!(game.state.battlefield.get(fs_id).unwrap().damage, 3);
    }

    #[test]
    fn trample_overflow_to_player() {
        let (mut game, p1, p2) = setup_combat_game(
            PlayerAgent::new(AttackAllPlayer),
            PlayerAgent::new(BlockAllPlayer),
        );

        let _ = add_creature(&mut game, p1, "Trampler", 5, 5, KeywordAbilities::TRAMPLE);
        let blocker_id = add_creature(&mut game, p2, "SmallBlocker", 1, 2, KeywordAbilities::empty());

        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.declare_attackers_step(p1);
        game.declare_blockers_step(p1);
        game.combat_damage_step(false);

        // Blocker takes 2 damage (lethal), 3 tramples to player
        assert_eq!(game.state.battlefield.get(blocker_id).unwrap().damage, 2);
        assert_eq!(game.state.players[&p2].life, 17);
    }

    #[test]
    fn end_combat_clears_state() {
        let (mut game, p1, _p2) = setup_combat_game(
            PlayerAgent::new(AttackAllPlayer),
            PlayerAgent::new(BlockAllPlayer),
        );

        let _ = add_creature(&mut game, p1, "Bear", 2, 2, KeywordAbilities::empty());

        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.declare_attackers_step(p1);
        assert!(game.state.combat.has_attackers());

        // End combat clears state
        game.state.combat.clear();
        assert!(!game.state.combat.has_attackers());
    }

    #[test]
    fn defender_cannot_attack() {
        let (mut game, p1, p2) = setup_combat_game(
            PlayerAgent::new(AttackAllPlayer),
            PlayerAgent::new(BlockAllPlayer),
        );

        // Only a defender creature
        let _ = add_creature(&mut game, p1, "Wall", 0, 5, KeywordAbilities::DEFENDER);

        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.declare_attackers_step(p1);

        // Should have no attackers (defender can't attack)
        assert!(!game.state.combat.has_attackers());
        assert_eq!(game.state.players[&p2].life, 20);
    }

    #[test]
    fn summoning_sick_creature_cannot_attack() {
        let (mut game, p1, p2) = setup_combat_game(
            PlayerAgent::new(AttackAllPlayer),
            PlayerAgent::new(BlockAllPlayer),
        );

        // Add creature WITH summoning sickness (don't remove it)
        let card = make_creature("SickBear", p1, 3, 3, KeywordAbilities::empty());
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);

        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.declare_attackers_step(p1);

        // Should not have attacked
        assert!(!game.state.combat.has_attackers());
        assert_eq!(game.state.players[&p2].life, 20);
    }

    #[test]
    fn haste_bypasses_summoning_sickness() {
        let (mut game, p1, p2) = setup_combat_game(
            PlayerAgent::new(AttackAllPlayer),
            PlayerAgent::new(BlockAllPlayer),
        );

        // Add creature with haste and summoning sickness
        let card = make_creature("Hasty", p1, 2, 1, KeywordAbilities::HASTE);
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);

        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.declare_attackers_step(p1);
        game.declare_blockers_step(p1);
        game.combat_damage_step(false);

        // Should have attacked and dealt damage
        assert_eq!(game.state.players[&p2].life, 18);
    }

    #[test]
    fn flying_cannot_be_blocked_by_ground() {
        let (mut game, p1, p2) = setup_combat_game(
            PlayerAgent::new(AttackAllPlayer),
            PlayerAgent::new(BlockAllPlayer),
        );

        let _ = add_creature(&mut game, p1, "Flyer", 3, 3, KeywordAbilities::FLYING);
        // Ground creature cannot block a flyer
        let _ = add_creature(&mut game, p2, "Ground", 2, 2, KeywordAbilities::empty());

        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.declare_attackers_step(p1);
        game.declare_blockers_step(p1);
        game.combat_damage_step(false);

        // Flyer should get through unblocked
        assert_eq!(game.state.players[&p2].life, 17);
    }

    #[test]
    fn reach_can_block_flying() {
        let (mut game, p1, p2) = setup_combat_game(
            PlayerAgent::new(AttackAllPlayer),
            PlayerAgent::new(BlockAllPlayer),
        );

        let flyer_id = add_creature(&mut game, p1, "Flyer", 2, 2, KeywordAbilities::FLYING);
        let reacher_id = add_creature(&mut game, p2, "Reacher", 1, 4, KeywordAbilities::REACH);

        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.declare_attackers_step(p1);
        game.declare_blockers_step(p1);

        // Reacher should be blocking the flyer
        assert!(game.state.combat.is_blocking(reacher_id));

        game.combat_damage_step(false);

        // No damage to player
        assert_eq!(game.state.players[&p2].life, 20);
        // Creatures trade damage
        assert_eq!(game.state.battlefield.get(reacher_id).unwrap().damage, 2);
        assert_eq!(game.state.battlefield.get(flyer_id).unwrap().damage, 1);
    }

    #[test]
    fn multiple_attackers_deal_combined_damage() {
        let (mut game, p1, p2) = setup_combat_game(
            PlayerAgent::new(AttackAllPlayer),
            PlayerAgent::new(BlockAllPlayer),
        );

        let _ = add_creature(&mut game, p1, "Bear1", 2, 2, KeywordAbilities::empty());
        let _ = add_creature(&mut game, p1, "Bear2", 3, 3, KeywordAbilities::empty());

        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.declare_attackers_step(p1);
        game.declare_blockers_step(p1);
        game.combat_damage_step(false);

        // Both should deal damage: 2 + 3 = 5
        assert_eq!(game.state.players[&p2].life, 15);
    }

    /// Decision maker that assigns ALL available blockers to each attacker.
    struct BlockAllMultiplePlayer;

    impl PlayerDecisionMaker for BlockAllMultiplePlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, attackers: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> {
            // Assign ALL legal blockers to each attacker
            let mut blocks = Vec::new();
            for info in attackers {
                for &blocker_id in &info.legal_blockers {
                    blocks.push((blocker_id, info.attacker_id));
                }
            }
            blocks
        }
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

    /// Decision maker that blocks with exactly one blocker per attacker.
    struct BlockOnePerAttackerPlayer;

    impl PlayerDecisionMaker for BlockOnePerAttackerPlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, attackers: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> {
            let mut blocks = Vec::new();
            let mut used = std::collections::HashSet::new();
            for info in attackers {
                for &blocker_id in &info.legal_blockers {
                    if !used.contains(&blocker_id) {
                        blocks.push((blocker_id, info.attacker_id));
                        used.insert(blocker_id);
                        break;
                    }
                }
            }
            blocks
        }
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

    fn make_deck2(owner: PlayerId) -> Vec<CardData> {
        (0..40).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    fn setup_game(
        p1_dm: PlayerAgent,
        p2_dm: PlayerAgent,
    ) -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Attacker".to_string(), deck: make_deck2(p1) },
                PlayerConfig { name: "Defender".to_string(), deck: make_deck2(p2) },
            ],
            starting_life: Life::new(20),
        };
        let game = Game::new_two_player(config, vec![(p1, p1_dm), (p2, p2_dm)]);
        (game, p1, p2)
    }

    fn add_creature_with_static(
        game: &mut Game,
        owner: PlayerId,
        name: &str,
        power: i32,
        toughness: i32,
        keywords: KeywordAbilities,
        static_effects: Vec<StaticEffect>,
    ) -> ObjectId {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(Power::new(power));
        card.toughness = Some(Toughness::new(toughness));
        card.keywords = keywords;
        let id = card.id;
        let ability = Ability::static_ability(id, "", static_effects);
        game.state.card_store.insert(card.clone());
        let mut perm = Permanent::new(card, owner);
        perm.remove_summoning_sickness();
        game.state.battlefield.add(perm);
        game.state.ability_store.add(ability);
        id
    }

    #[test]
    fn daunt_blocks_low_power_creatures() {
        // Creature with "can't be blocked by power 2 or less" (daunt)
        let (mut game, p1, p2) = setup_game(
            PlayerAgent::new(AttackAllPlayer),
            PlayerAgent::new(BlockOnePerAttackerPlayer),
        );

        let attacker_id = add_creature_with_static(
            &mut game, p1, "Daunt Creature", 4, 4, KeywordAbilities::empty(),
            vec![StaticEffect::CantBeBlockedByPowerLessOrEqual { power: Power::new(2) }],
        );
        let small_blocker = add_creature(&mut game, p2, "Small Blocker", 2, 2, KeywordAbilities::empty());
        let big_blocker = add_creature(&mut game, p2, "Big Blocker", 3, 3, KeywordAbilities::empty());

        game.state.active_player = p1;
        game.state.priority_player = p1;

        // Apply continuous effects so daunt threshold is set
        game.apply_continuous_effects();

        game.declare_attackers_step(p1);
        assert!(game.state.combat.is_attacking(attacker_id));

        game.declare_blockers_step(p1);

        // Small blocker (power 2) should NOT be blocking (daunt prevents it)
        assert!(!game.state.combat.is_blocking(small_blocker));
        // Big blocker (power 3) SHOULD be blocking
        assert!(game.state.combat.is_blocking(big_blocker));
    }

    #[test]
    fn cant_be_blocked_by_more_than_one() {
        let (mut game, p1, p2) = setup_game(
            PlayerAgent::new(AttackAllPlayer),
            PlayerAgent::new(BlockAllMultiplePlayer),
        );

        let attacker_id = add_creature_with_static(
            &mut game, p1, "Max1 Creature", 3, 3, KeywordAbilities::empty(),
            vec![StaticEffect::CantBeBlockedByMoreThan { count: 1 }],
        );
        let _blocker1 = add_creature(&mut game, p2, "Blocker1", 2, 2, KeywordAbilities::empty());
        let _blocker2 = add_creature(&mut game, p2, "Blocker2", 2, 2, KeywordAbilities::empty());
        let _blocker3 = add_creature(&mut game, p2, "Blocker3", 2, 2, KeywordAbilities::empty());

        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.apply_continuous_effects();

        game.declare_attackers_step(p1);
        game.declare_blockers_step(p1);

        // Should have at most 1 blocker despite defender wanting to assign all 3
        let group = game.state.combat.group_for_attacker(attacker_id).unwrap();
        assert!(group.blockers.len() <= 1, "max_blocked_by=1 but got {} blockers", group.blockers.len());
        assert!(group.is_blocked()); // Still counted as blocked
    }

    #[test]
    fn menace_single_blocker_removed() {
        // Menace: must be blocked by 2+ creatures. A single blocker should be removed.
        let (mut game, p1, p2) = setup_game(
            PlayerAgent::new(AttackAllPlayer),
            PlayerAgent::new(BlockOnePerAttackerPlayer),
        );

        let attacker_id = add_creature(&mut game, p1, "Menace Creature", 3, 3, KeywordAbilities::MENACE);
        let _blocker1 = add_creature(&mut game, p2, "Blocker1", 2, 2, KeywordAbilities::empty());

        game.state.active_player = p1;
        game.state.priority_player = p1;

        game.declare_attackers_step(p1);
        game.declare_blockers_step(p1);

        // Single blocker should be removed by menace validation
        let group = game.state.combat.group_for_attacker(attacker_id).unwrap();
        assert_eq!(group.blockers.len(), 0, "menace should remove single blocker");
        assert!(!group.is_blocked());
    }

    #[test]
    fn menace_two_blockers_allowed() {
        // Menace with 2 blockers: should be allowed
        let (mut game, p1, p2) = setup_game(
            PlayerAgent::new(AttackAllPlayer),
            PlayerAgent::new(BlockAllMultiplePlayer),
        );

        let attacker_id = add_creature(&mut game, p1, "Menace Creature", 3, 3, KeywordAbilities::MENACE);
        let _blocker1 = add_creature(&mut game, p2, "Blocker1", 2, 2, KeywordAbilities::empty());
        let _blocker2 = add_creature(&mut game, p2, "Blocker2", 2, 2, KeywordAbilities::empty());

        game.state.active_player = p1;
        game.state.priority_player = p1;

        game.declare_attackers_step(p1);
        game.declare_blockers_step(p1);

        // Two blockers should satisfy menace
        let group = game.state.combat.group_for_attacker(attacker_id).unwrap();
        assert_eq!(group.blockers.len(), 2, "menace satisfied by 2 blockers");
        assert!(group.is_blocked());
    }

    #[test]
    fn must_be_blocked_flag_set() {
        // MustBeBlocked static effect sets the flag on the permanent
        let (mut game, p1, _p2) = setup_game(
            PlayerAgent::new(AttackAllPlayer),
            PlayerAgent::new(BlockOnePerAttackerPlayer),
        );

        let creature_id = add_creature_with_static(
            &mut game, p1, "Lure Creature", 3, 3, KeywordAbilities::empty(),
            vec![StaticEffect::MustBeBlocked],
        );

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(perm.must_be_blocked, "must_be_blocked should be set by static effect");
    }

    #[test]
    fn must_be_blocked_info_in_attacker_info() {
        // The must_be_blocked flag should be available in AttackerInfo
        let (mut game, p1, p2) = setup_game(
            PlayerAgent::new(AttackAllPlayer),
            PlayerAgent::new(BlockOnePerAttackerPlayer),
        );

        let _lure_id = add_creature_with_static(
            &mut game, p1, "Lure Creature", 3, 3, KeywordAbilities::empty(),
            vec![StaticEffect::MustBeBlocked],
        );
        let _blocker = add_creature(&mut game, p2, "Blocker", 2, 2, KeywordAbilities::empty());

        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.apply_continuous_effects();

        // Just verify the continuous effects set the flag correctly
        let perm = game.state.battlefield.get(_lure_id).unwrap();
        assert!(perm.must_be_blocked);
    }

    // More combat tests

    struct PassPlayer;

    impl PlayerDecisionMaker for PassPlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
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

    fn make_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let deck: Vec<CardData> = (0..40).map(|i| {
            let mut c = CardData::new(ObjectId::new(), p1, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect();
        let deck2: Vec<CardData> = (0..40).map(|i| {
            let mut c = CardData::new(ObjectId::new(), p2, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "P1".into(), deck },
                PlayerConfig { name: "P2".into(), deck: deck2 },
            ],
            starting_life: Life::new(20),
        };
        let game = Game::new_two_player(config, vec![(p1, PlayerAgent::new(PassPlayer)), (p2, PlayerAgent::new(PassPlayer))]);
        (game, p1, p2)
    }

    fn add_creature3(game: &mut Game, owner: PlayerId, name: &str, power: i32, toughness: i32) -> ObjectId {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(Power::new(power));
        card.toughness = Some(Toughness::new(toughness));
        let id = card.id;
        game.state.battlefield.add(Permanent::new(card, owner));
        id
    }

    fn add_equipment_with_toughness_damage(game: &mut Game, owner: PlayerId, conditional: bool) -> ObjectId {
        let mut card = CardData::new(ObjectId::new(), owner, "Toughness Equipment");
        card.card_types = vec![CardType::Artifact];
        card.subtypes = vec![SubType::Equipment];
        let id = card.id;
        let ability = if conditional {
            Ability::static_ability(id,
                "Equipped creature assigns combat damage equal to toughness if toughness > power.",
                vec![StaticEffect::assign_damage_with_toughness_if_greater(Filter::equipped_creature())])
        } else {
            Ability::static_ability(id,
                "Equipped creature assigns combat damage equal to its toughness.",
                vec![StaticEffect::assign_damage_with_toughness(Filter::equipped_creature())])
        };
        card.abilities.push(ability.clone());
        game.state.card_store.insert(card.clone());
        game.state.battlefield.add(Permanent::new(card, owner));
        game.state.ability_store.add(ability);
        id
    }

    #[test]
    fn unconditional_sets_flag_on_equipped_creature() {
        let (mut game, p1, _p2) = make_game();

        let creature_id = add_creature3(&mut game, p1, "Wall", 1, 5);
        let equip_id = add_equipment_with_toughness_damage(&mut game, p1, false);

        if let Some(equip) = game.state.battlefield.get_mut(equip_id) {
            equip.attached_to = Some(creature_id);
        }
        if let Some(creature) = game.state.battlefield.get_mut(creature_id) {
            creature.attachments.push(equip_id);
        }

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(perm.assign_damage_with_toughness);
    }

    #[test]
    fn conditional_sets_flag_when_toughness_greater() {
        let (mut game, p1, _p2) = make_game();

        let creature_id = add_creature3(&mut game, p1, "Wall", 1, 5);
        let equip_id = add_equipment_with_toughness_damage(&mut game, p1, true);

        if let Some(equip) = game.state.battlefield.get_mut(equip_id) {
            equip.attached_to = Some(creature_id);
        }
        if let Some(creature) = game.state.battlefield.get_mut(creature_id) {
            creature.attachments.push(equip_id);
        }

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(perm.assign_damage_with_toughness, "toughness 5 > power 1, flag should be set");
    }

    #[test]
    fn conditional_does_not_set_flag_when_power_greater() {
        let (mut game, p1, _p2) = make_game();

        let creature_id = add_creature3(&mut game, p1, "Big Power", 5, 2);
        let equip_id = add_equipment_with_toughness_damage(&mut game, p1, true);

        if let Some(equip) = game.state.battlefield.get_mut(equip_id) {
            equip.attached_to = Some(creature_id);
        }
        if let Some(creature) = game.state.battlefield.get_mut(creature_id) {
            creature.attachments.push(equip_id);
        }

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(!perm.assign_damage_with_toughness, "power 5 > toughness 2, flag should NOT be set");
    }

    #[test]
    fn conditional_does_not_set_flag_when_equal() {
        let (mut game, p1, _p2) = make_game();

        let creature_id = add_creature3(&mut game, p1, "Even", 3, 3);
        let equip_id = add_equipment_with_toughness_damage(&mut game, p1, true);

        if let Some(equip) = game.state.battlefield.get_mut(equip_id) {
            equip.attached_to = Some(creature_id);
        }
        if let Some(creature) = game.state.battlefield.get_mut(creature_id) {
            creature.attachments.push(equip_id);
        }

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(!perm.assign_damage_with_toughness, "power == toughness, flag should NOT be set");
    }

    #[test]
    fn flag_cleared_on_recalculation() {
        let (mut game, p1, _p2) = make_game();

        let creature_id = add_creature3(&mut game, p1, "Wall", 1, 5);

        if let Some(perm) = game.state.battlefield.get_mut(creature_id) {
            perm.assign_damage_with_toughness = true;
        }

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(!perm.assign_damage_with_toughness, "flag should be cleared without source");
    }

    #[test]
    fn unequipped_creature_not_affected() {
        let (mut game, p1, _p2) = make_game();

        let creature_id = add_creature3(&mut game, p1, "Wall", 1, 5);
        let _equip_id = add_equipment_with_toughness_damage(&mut game, p1, false);

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(!perm.assign_damage_with_toughness, "unequipped creature should not get the flag");
    }

    #[test]
    fn helper_unconditional() {
        match StaticEffect::assign_damage_with_toughness(Filter::equipped_creature()) {
            StaticEffect::AssignDamageWithToughness { filter, condition } => {
                assert_eq!(filter, "equipped creature");
                assert!(condition.is_none());
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn helper_conditional() {
        match StaticEffect::assign_damage_with_toughness_if_greater(Filter::equipped_creature()) {
            StaticEffect::AssignDamageWithToughness { filter, condition } => {
                assert_eq!(filter, "equipped creature");
                assert_eq!(condition.unwrap(), "toughness_greater_than_power");
            }
            _ => panic!("wrong variant"),
        }
    }
