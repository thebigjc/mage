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

fn make_deck(owner: PlayerId) -> Vec<CardData> {
    (0..20).map(|i| {
        let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
        c.card_types = vec![CardType::Land];
        c
    }).collect()
}

struct AlwaysPassPlayer;

impl PlayerDecisionMaker for AlwaysPassPlayer {
    fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction {
        PlayerAction::Pass
    }
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

    /// Decision maker that always picks mode 0 (first available).
    struct PickFirstModePlayer;

    impl PlayerDecisionMaker for PickFirstModePlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction {
            PlayerAction::Pass
        }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _modes: &[NamedChoice]) -> usize { 0 }
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

    /// Decision maker that always picks mode 1 (second available).
    struct PickSecondModePlayer;

    impl PlayerDecisionMaker for PickSecondModePlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction {
            PlayerAction::Pass
        }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, modes: &[NamedChoice]) -> usize {
            if modes.len() > 1 { 1 } else { 0 }
        }
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

    #[test]
    fn modal_choose_one_of_two() {
        // Test "Choose one" modal with 2 modes.
        // Mode 1: Gain 5 life. Mode 2: Deal 3 damage.
        // PickFirstModePlayer always picks mode 0 (gain life).
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".into(), deck: vec![] },
                PlayerConfig { name: "Bob".into(), deck: vec![] },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(PickFirstModePlayer)),
                (p2, Box::new(PickFirstModePlayer)),
            ],
        );

        let modal = Effect::modal(
            vec![
                ModalMode::new("Gain 5 life", vec![Effect::gain_life(5)]),
                ModalMode::new("Deal 3 damage to target", vec![Effect::deal_damage(3)]),
            ],
            1, // min modes
            1, // max modes (choose one)
        );

        // Execute with p1 as controller
        game.execute_effects(&[modal], p1, &[], None, None);

        // PickFirstModePlayer always picks mode 0 (gain life)
        assert_eq!(game.state.players[&p1].life, 25, "p1 should have gained 5 life");
        assert_eq!(game.state.players[&p2].life, 20, "p2 should be unchanged");
    }

    #[test]
    fn modal_choose_two_of_three() {
        // Test "Choose two" modal with 3 modes.
        // Mode 0: Gain 3 life. Mode 1: Draw 2 cards. Mode 2: Deal 2 damage to opponents.
        // PickFirstModePlayer always picks index 0 from available modes.
        // First call: available = [0,1,2], picks 0 (gain life)
        // Second call: available = [1,2], picks index 0 -> mode 1 (draw)
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let _config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".into(), deck: vec![] },
                PlayerConfig { name: "Bob".into(), deck: vec![] },
            ],
            starting_life: 20,
        };

        // Give p1 some cards in library to draw from
        let mut deck = Vec::new();
        for _ in 0..5 {
            let mut card = CardData::new(ObjectId::new(), p1, "Forest");
            card.card_types = vec![CardType::Land];
            deck.push(card);
        }

        let mut game = Game::new_two_player(
            GameConfig {
                players: vec![
                    PlayerConfig { name: "Alice".into(), deck: vec![] },
                    PlayerConfig { name: "Bob".into(), deck: vec![] },
                ],
                starting_life: 20,
            },
            vec![
                (p1, Box::new(PickFirstModePlayer)),
                (p2, Box::new(PickFirstModePlayer)),
            ],
        );

        // Manually add cards to p1's library
        for card in deck {
            game.state.card_store.insert(card.clone());
            game.state.players.get_mut(&p1).unwrap().library.put_on_top(card.id);
        }

        let modal = Effect::modal(
            vec![
                ModalMode::new("Gain 3 life", vec![Effect::gain_life(3)]),
                ModalMode::new("Draw 2 cards", vec![Effect::draw_cards(2)]),
                ModalMode::new("Each opponent loses 2 life", vec![Effect::LoseLifeOpponents { amount: 2 }]),
            ],
            2, // min modes
            2, // max modes (choose two)
        );

        let hand_before = game.state.players[&p1].hand.len();
        game.execute_effects(&[modal], p1, &[], None, None);

        // Picks mode 0 (gain life) and mode 1 (draw)
        assert_eq!(game.state.players[&p1].life, 23, "p1 should have gained 3 life");
        assert_eq!(game.state.players[&p1].hand.len(), hand_before + 2, "p1 should have drawn 2");
        assert_eq!(game.state.players[&p2].life, 20, "p2 should be unchanged (mode 2 not chosen)");
    }

    #[test]
    fn modal_second_mode_chosen() {
        // PickSecondModePlayer picks mode 1 when 2+ available.
        // With "choose one" of 2 modes, should execute mode 1 only.
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let mut game = Game::new_two_player(
            GameConfig {
                players: vec![
                    PlayerConfig { name: "Alice".into(), deck: vec![] },
                    PlayerConfig { name: "Bob".into(), deck: vec![] },
                ],
                starting_life: 20,
            },
            vec![
                (p1, Box::new(PickSecondModePlayer)),
                (p2, Box::new(PickSecondModePlayer)),
            ],
        );

        let modal = Effect::modal(
            vec![
                ModalMode::new("Gain 5 life", vec![Effect::gain_life(5)]),
                ModalMode::new("Each opponent loses 3 life", vec![Effect::LoseLifeOpponents { amount: 3 }]),
            ],
            1,
            1, // choose one
        );

        game.execute_effects(&[modal], p1, &[], None, None);

        // PickSecondModePlayer picks mode 1 (opponents lose life)
        assert_eq!(game.state.players[&p1].life, 20, "p1 should be unchanged");
        assert_eq!(game.state.players[&p2].life, 17, "p2 should have lost 3 life");
    }

