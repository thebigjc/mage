// Tests extracted from game.rs

use crate::game::*;
use crate::abilities::{Ability, Cost, Effect, TargetSpec, StaticEffect, ModalMode};
use crate::card::CardData;
use crate::combat::CombatState;
use crate::constants::{CardType, Color, KeywordAbilities, Outcome, PhaseStep, SubType, SuperType, TurnPhase, Zone};
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

    /// Decision maker that always passes and says yes to optional triggers.
    struct TriggerTestPlayer {
        attack_all: bool,
    }

    impl TriggerTestPlayer {
        fn passive() -> Self { TriggerTestPlayer { attack_all: false } }
        fn attacker() -> Self { TriggerTestPlayer { attack_all: true } }
    }

    impl PlayerDecisionMaker for TriggerTestPlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction {
            PlayerAction::Pass
        }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { true }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(
            &mut self,
            _: &GameView<'_>,
            possible_attackers: &[ObjectId],
            possible_defenders: &[ObjectId],
        ) -> Vec<(ObjectId, ObjectId)> {
            if self.attack_all && !possible_defenders.is_empty() {
                let defender = possible_defenders[0];
                possible_attackers.iter().map(|&a| (a, defender)).collect()
            } else {
                vec![]
            }
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

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        (0..40).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    fn setup(
        p1_dm: Box<dyn PlayerDecisionMaker>,
        p2_dm: Box<dyn PlayerDecisionMaker>,
    ) -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(config, vec![(p1, p1_dm), (p2, p2_dm)]);
        (game, p1, p2)
    }

    #[test]
    fn etb_trigger_fires_and_resolves() {
        let (mut game, p1, _p2) = setup(
            Box::new(TriggerTestPlayer::passive()),
            Box::new(TriggerTestPlayer::passive()),
        );

        // Create a creature with an ETB trigger: "When this enters, gain 3 life"
        let card_id = ObjectId::new();
        let mut card = CardData::new(card_id, p1, "Soul Warden");
        card.card_types = vec![CardType::Creature];
        card.power = Some(1);
        card.toughness = Some(1);
        card.abilities.push(Ability::triggered(
            card_id,
            "When Soul Warden enters the battlefield, you gain 3 life.",
            vec![EventType::EnteredTheBattlefield],
            vec![Effect::GainLife { amount: 3 }],
            TargetSpec::None,
        ));

        // Register the card in the card store
        game.state.card_store.insert(card.clone());

        // Register abilities
        for ability in &card.abilities {
            game.state.ability_store.add(ability.clone());
        }

        // Put the permanent on the battlefield and emit ETB
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);
        game.state.set_zone(card_id, crate::constants::Zone::Battlefield, None);
        game.emit_event(GameEvent::enters_battlefield(card_id, p1));

        // Before processing triggers, life should be 20
        assert_eq!(game.state.players[&p1].life, 20);

        // Process SBAs + triggers
        game.process_sba_and_triggers();

        // The trigger should have been put on the stack
        // Since we called process_sba_and_triggers (not the full priority loop),
        // the ability is on the stack. Let's resolve it.
        assert!(!game.state.stack.is_empty());

        // Resolve the triggered ability
        game.resolve_top_of_stack();

        // Life should now be 23
        assert_eq!(game.state.players[&p1].life, 23);
    }

    #[test]
    fn attack_trigger_fires() {
        let (mut game, p1, p2) = setup(
            Box::new(TriggerTestPlayer::attacker()),
            Box::new(TriggerTestPlayer::passive()),
        );

        // Create creature with attack trigger: "Whenever this attacks, each opponent loses 1 life"
        let card_id = ObjectId::new();
        let mut card = CardData::new(card_id, p1, "Pulse Tracker");
        card.card_types = vec![CardType::Creature];
        card.power = Some(1);
        card.toughness = Some(1);
        card.abilities.push(Ability::triggered(
            card_id,
            "Whenever Pulse Tracker attacks, each opponent loses 1 life.",
            vec![EventType::AttackerDeclared],
            vec![Effect::LoseLifeOpponents { amount: 1 }],
            TargetSpec::None,
        ));

        // Register
        game.state.card_store.insert(card.clone());
        for ability in &card.abilities {
            game.state.ability_store.add(ability.clone());
        }
        let mut perm = Permanent::new(card, p1);
        perm.remove_summoning_sickness();
        game.state.battlefield.add(perm);
        game.state.set_zone(card_id, crate::constants::Zone::Battlefield, None);

        // Declare attackers
        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.declare_attackers_step(p1);

        // Process SBAs + triggers
        game.process_sba_and_triggers();

        // Attack trigger should be on the stack
        assert!(!game.state.stack.is_empty());

        // Resolve it
        game.resolve_top_of_stack();

        // Opponent should have lost 1 life
        assert_eq!(game.state.players[&p2].life, 19);
    }

    #[test]
    fn life_gain_trigger_fires() {
        let (mut game, p1, _p2) = setup(
            Box::new(TriggerTestPlayer::passive()),
            Box::new(TriggerTestPlayer::passive()),
        );

        // Create "Ajani's Pridemate" — whenever you gain life, put a +1/+1 counter
        let card_id = ObjectId::new();
        let mut card = CardData::new(card_id, p1, "Ajani's Pridemate");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        card.abilities.push(Ability::triggered(
            card_id,
            "Whenever you gain life, put a +1/+1 counter on Ajani's Pridemate.",
            vec![EventType::GainLife],
            vec![Effect::add_counters_self("+1/+1", 1)],
            TargetSpec::None,
        ));

        // Register
        game.state.card_store.insert(card.clone());
        for ability in &card.abilities {
            game.state.ability_store.add(ability.clone());
        }
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);

        // Gain life
        game.execute_effects(&[Effect::GainLife { amount: 5 }], p1, &[], None, None);

        // Process triggers
        game.process_sba_and_triggers();
        assert!(!game.state.stack.is_empty());

        // Resolve the trigger
        game.resolve_top_of_stack();

        // Should have a +1/+1 counter
        let pridemate = game.state.battlefield.get(card_id).unwrap();
        assert_eq!(pridemate.counters.get(&crate::counters::CounterType::P1P1), 1);
        assert_eq!(pridemate.power(), 3);
        assert_eq!(pridemate.toughness(), 3);
    }

    #[test]
    fn optional_trigger_not_forced() {
        let (mut game, p1, _p2) = setup(
            Box::new(TriggerTestPlayer::passive()),
            Box::new(TriggerTestPlayer::passive()),
        );

        // A "may" trigger that the player says yes to (TriggerTestPlayer says yes)
        let card_id = ObjectId::new();
        let mut card = CardData::new(card_id, p1, "Optional Creature");
        card.card_types = vec![CardType::Creature];
        card.power = Some(1);
        card.toughness = Some(1);
        let ability = Ability::triggered(
            card_id,
            "When Optional Creature enters, you may gain 2 life.",
            vec![EventType::EnteredTheBattlefield],
            vec![Effect::GainLife { amount: 2 }],
            TargetSpec::None,
        ).set_optional();
        card.abilities.push(ability);

        game.state.card_store.insert(card.clone());
        for ability in &card.abilities {
            game.state.ability_store.add(ability.clone());
        }
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);
        game.emit_event(GameEvent::enters_battlefield(card_id, p1));

        // Process triggers — TriggerTestPlayer always says yes
        game.process_sba_and_triggers();

        // Should be on the stack (player chose yes)
        assert!(!game.state.stack.is_empty());
        game.resolve_top_of_stack();
        assert_eq!(game.state.players[&p1].life, 22);
    }

    #[test]
    fn trigger_only_fires_for_own_permanent() {
        let (mut game, p1, p2) = setup(
            Box::new(TriggerTestPlayer::passive()),
            Box::new(TriggerTestPlayer::passive()),
        );

        // p1's creature has attack trigger
        let card_id = ObjectId::new();
        let mut card = CardData::new(card_id, p1, "TriggerCreature");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        card.abilities.push(Ability::triggered(
            card_id,
            "Whenever TriggerCreature attacks, gain 1 life.",
            vec![EventType::AttackerDeclared],
            vec![Effect::GainLife { amount: 1 }],
            TargetSpec::None,
        ));

        game.state.card_store.insert(card.clone());
        for ability in &card.abilities {
            game.state.ability_store.add(ability.clone());
        }
        let mut perm = Permanent::new(card, p1);
        perm.remove_summoning_sickness();
        game.state.battlefield.add(perm);

        // A DIFFERENT creature from p2 attacks — p1's trigger should NOT fire
        let other_id = ObjectId::new();
        let mut other = CardData::new(other_id, p2, "Other");
        other.card_types = vec![CardType::Creature];
        other.power = Some(1);
        other.toughness = Some(1);
        let mut perm2 = Permanent::new(other, p2);
        perm2.remove_summoning_sickness();
        game.state.battlefield.add(perm2);

        // Emit attack event for p2's creature (not p1's)
        game.emit_event(
            GameEvent::new(EventType::AttackerDeclared)
                .target(other_id)
                .player(p2),
        );

        // Process triggers
        game.process_sba_and_triggers();

        // No trigger should have fired (the attacker was a different creature)
        assert!(game.state.stack.is_empty());
    }


    // Additional tests

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

    fn make_deck2(owner: PlayerId) -> Vec<CardData> {
        (0..40).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    fn setup2() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Player1".into(), deck: make_deck2(p1) },
                PlayerConfig { name: "Player2".into(), deck: make_deck2(p2) },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(
            config,
            vec![(p1, Box::new(PassivePlayer)), (p2, Box::new(PassivePlayer))],
        );
        (game, p1, p2)
    }

    #[test]
    fn dies_trigger_fires_on_lethal_damage() {
        let (mut game, p1, _p2) = setup2();

        // Create a creature with "When this creature dies, draw a card"
        let mut card = CardData::new(ObjectId::new(), p1, "Doomed Traveler");
        card.card_types = vec![CardType::Creature];
        card.power = Some(1);
        card.toughness = Some(1);
        let id = card.id;
        card.abilities = vec![
            Ability::triggered(id, "When Doomed Traveler dies, draw a card.",
                vec![EventType::Dies],
                vec![Effect::DrawCards { count: 1 }],
                TargetSpec::None),
        ];
        for ability in &card.abilities {
            game.state.ability_store.add(ability.clone());
        }
        let mut perm = Permanent::new(card, p1);
        perm.remove_summoning_sickness();
        game.state.battlefield.add(perm);

        // Mark lethal damage on the creature
        game.state.battlefield.get_mut(id).unwrap().apply_damage(1);

        // Process SBAs + triggers
        game.process_sba_and_triggers();

        // Creature should be in graveyard
        assert!(!game.state.battlefield.contains(id));

        // Dies trigger should have put an ability on the stack
        assert!(!game.state.stack.is_empty(), "Dies trigger should be on stack");
    }

    #[test]
    fn dies_trigger_fires_on_destroy_effect() {
        let (mut game, p1, p2) = setup(
            Box::new(PassivePlayer),
            Box::new(PassivePlayer),
        );

        // Create a creature with dies trigger controlled by p2
        let mut card = CardData::new(ObjectId::new(), p2, "Blood Artist");
        card.card_types = vec![CardType::Creature];
        card.power = Some(0);
        card.toughness = Some(1);
        let id = card.id;
        card.abilities = vec![
            Ability::triggered(id, "When Blood Artist dies, opponent loses 1 life.",
                vec![EventType::Dies],
                vec![Effect::LoseLifeOpponents { amount: 1 }],
                TargetSpec::None),
        ];
        for ability in &card.abilities {
            game.state.ability_store.add(ability.clone());
        }
        let perm = Permanent::new(card, p2);
        game.state.battlefield.add(perm);

        // Destroy it with an effect
        game.execute_effects(
            &[Effect::Destroy],
            p1,
            &[id],
            None, None,
        );

        // Creature should be in graveyard
        assert!(!game.state.battlefield.contains(id));

        // Check that dies event was emitted
        assert!(!game.event_log.is_empty(), "Dies event should be in log");
    }

    #[test]
    fn dies_trigger_only_for_dying_creature() {
        let (mut game, p1, _p2) = setup(
            Box::new(PassivePlayer),
            Box::new(PassivePlayer),
        );

        // Creature A has a dies trigger
        let mut card_a = CardData::new(ObjectId::new(), p1, "Creature A");
        card_a.card_types = vec![CardType::Creature];
        card_a.power = Some(1);
        card_a.toughness = Some(1);
        let id_a = card_a.id;
        card_a.abilities = vec![
            Ability::triggered(id_a, "When this dies, draw a card.",
                vec![EventType::Dies],
                vec![Effect::DrawCards { count: 1 }],
                TargetSpec::None),
        ];
        for ability in &card_a.abilities {
            game.state.ability_store.add(ability.clone());
        }
        let perm_a = Permanent::new(card_a, p1);
        game.state.battlefield.add(perm_a);

        // Creature B has NO dies trigger
        let mut card_b = CardData::new(ObjectId::new(), p1, "Creature B");
        card_b.card_types = vec![CardType::Creature];
        card_b.power = Some(1);
        card_b.toughness = Some(1);
        let id_b = card_b.id;
        let perm_b = Permanent::new(card_b, p1);
        game.state.battlefield.add(perm_b);

        // Kill Creature B only (not A)
        game.state.battlefield.get_mut(id_b).unwrap().apply_damage(1);

        // Process SBAs
        game.process_sba_and_triggers();

        // Creature B should be dead, Creature A should be alive
        assert!(game.state.battlefield.contains(id_a));
        assert!(!game.state.battlefield.contains(id_b));

        // No trigger should fire (the dying creature had no dies trigger)
        assert!(game.state.stack.is_empty());
    }


// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Equipment tests
// ---------------------------------------------------------------------------



    struct PassivePlayer2;
    impl PlayerDecisionMaker for PassivePlayer2 {
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

    fn make_deck3(owner: PlayerId) -> Vec<CardData> {
        (0..40).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    #[test]
    fn upkeep_trigger_fires_on_upkeep_step() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "A".into(), deck: make_deck3(p1) },
                PlayerConfig { name: "B".into(), deck: make_deck3(p2) },
            ],
            starting_life: 20,
        };
        let mut game = Game::new_two_player(
            config,
            vec![(p1, Box::new(PassivePlayer)), (p2, Box::new(PassivePlayer))],
        );
        game.state.active_player = p1;
        game.state.priority_player = p1;

        // Create a creature with upkeep trigger (gain 1 life at upkeep)
        let creature_id = ObjectId::new();
        let mut card = CardData::new(creature_id, p1, "Upkeep Healer");
        card.card_types = vec![CardType::Creature];
        card.power = Some(1);
        card.toughness = Some(1);
        let perm = Permanent::new(card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card);

        let trigger = Ability::triggered(
            creature_id,
            "At the beginning of your upkeep, gain 1 life.",
            vec![EventType::UpkeepStep],
            vec![Effect::GainLife { amount: 1 }],
            TargetSpec::None,
        );
        game.state.ability_store.add(trigger);

        let life_before = game.state.player(p1).unwrap().life;

        // Emit upkeep event and process triggers
        let mut event = GameEvent::new(EventType::UpkeepStep);
        event.player_id = Some(p1);
        game.emit_event(event);
        game.process_sba_and_triggers();

        // Triggered ability should be on the stack — resolve it
        assert!(!game.state.stack.is_empty(), "Trigger should be on the stack");
        game.resolve_top_of_stack();

        let life_after = game.state.player(p1).unwrap().life;
        assert_eq!(life_after, life_before + 1, "Upkeep trigger should have gained 1 life");
    }

    #[test]
    fn end_step_trigger_fires() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "A".into(), deck: make_deck(p1) },
                PlayerConfig { name: "B".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let mut game = Game::new_two_player(
            config,
            vec![(p1, Box::new(PassivePlayer)), (p2, Box::new(PassivePlayer))],
        );
        game.state.active_player = p1;
        game.state.priority_player = p1;

        // Create a creature with end step trigger (draw a card)
        let creature_id = ObjectId::new();
        let mut card = CardData::new(creature_id, p1, "End Step Draw");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        let perm = Permanent::new(card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card);

        let trigger = Ability::triggered(
            creature_id,
            "At the beginning of your end step, draw a card.",
            vec![EventType::EndStep],
            vec![Effect::DrawCards { count: 1 }],
            TargetSpec::None,
        );
        game.state.ability_store.add(trigger);

        let hand_before = game.state.player(p1).unwrap().hand.len();

        // Emit end step event and process triggers
        let mut event = GameEvent::new(EventType::EndStep);
        event.player_id = Some(p1);
        game.emit_event(event);
        game.process_sba_and_triggers();

        // Triggered ability should be on the stack — resolve it
        assert!(!game.state.stack.is_empty(), "Trigger should be on the stack");
        game.resolve_top_of_stack();

        let hand_after = game.state.player(p1).unwrap().hand.len();
        assert_eq!(hand_after, hand_before + 1, "End step trigger should have drawn 1 card");
    }

    #[test]
    fn upkeep_trigger_only_fires_for_controller() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "A".into(), deck: make_deck(p1) },
                PlayerConfig { name: "B".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let mut game = Game::new_two_player(
            config,
            vec![(p1, Box::new(PassivePlayer)), (p2, Box::new(PassivePlayer))],
        );
        game.state.active_player = p1;
        game.state.priority_player = p1;

        // Create an upkeep trigger creature controlled by p2
        let creature_id = ObjectId::new();
        let mut card = CardData::new(creature_id, p2, "Opponent Healer");
        card.card_types = vec![CardType::Creature];
        card.power = Some(1);
        card.toughness = Some(1);
        let perm = Permanent::new(card.clone(), p2);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card);

        let trigger = Ability::triggered(
            creature_id,
            "At the beginning of your upkeep, gain 1 life.",
            vec![EventType::UpkeepStep],
            vec![Effect::GainLife { amount: 1 }],
            TargetSpec::None,
        );
        game.state.ability_store.add(trigger);

        let p2_life = game.state.player(p2).unwrap().life;

        // Emit p1's upkeep — p2's trigger should NOT fire
        let mut event = GameEvent::new(EventType::UpkeepStep);
        event.player_id = Some(p1);
        game.emit_event(event);
        game.process_sba_and_triggers();

        let p2_life_after = game.state.player(p2).unwrap().life;
        assert_eq!(p2_life_after, p2_life, "P2's upkeep trigger should not fire during p1's upkeep");
    }


    // Additional tests

    struct PassivePlayer3;
    impl PlayerDecisionMaker for PassivePlayer3 {
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

    fn setup_delayed_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            starting_life: 20,
            players: vec![
                PlayerConfig { name: "P1".into(), deck: vec![] },
                PlayerConfig { name: "P2".into(), deck: vec![] },
            ],
        };
        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(PassivePlayer)),
                (p2, Box::new(PassivePlayer)),
            ],
        );
        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.state.current_phase = TurnPhase::PrecombatMain;
        game.state.current_step = PhaseStep::PrecombatMain;
        game.state.turn_number = 1;
        (game, p1, p2)
    }

    #[test]
    fn delayed_on_death_fires_when_creature_dies() {
        let (mut game, p1, _p2) = setup_delayed_game();

        // Put a card in library so we can draw
        let draw_card_id = ObjectId::new();
        let draw_card = CardData::new(draw_card_id, p1, "Prize");
        game.state.card_store.insert(draw_card);
        game.state.players.get_mut(&p1).unwrap().library.put_on_top(draw_card_id);

        // Put a creature on the battlefield
        let creature_id = ObjectId::new();
        let mut card = CardData::new(creature_id, p1, "Doomed Creature");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        let perm = Permanent::new(card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card);

        // Create a delayed trigger: "when Doomed Creature dies this turn, draw a card"
        let effects = vec![Effect::delayed_on_death(vec![Effect::DrawCards { count: 1 }])];
        game.execute_effects(&effects, p1, &[creature_id], None, None);

        // Should have 1 delayed trigger registered
        assert_eq!(game.state.delayed_triggers.len(), 1);
        assert_eq!(game.state.delayed_triggers[0].watching, Some(creature_id));

        let hand_before = game.state.players.get(&p1).unwrap().hand.len();

        // Kill the creature (emit dies event)
        game.state.battlefield.remove(creature_id);
        game.emit_event(GameEvent::dies(creature_id, p1));
        game.check_triggered_abilities();

        let hand_after = game.state.players.get(&p1).unwrap().hand.len();
        assert_eq!(hand_after, hand_before + 1, "Should have drawn a card when creature died");

        // Delayed trigger should have been removed (trigger_only_once)
        assert_eq!(game.state.delayed_triggers.len(), 0);
    }

    #[test]
    fn delayed_on_death_does_not_fire_for_wrong_creature() {
        let (mut game, p1, p2) = setup_delayed_game();

        // Two creatures
        let watched_id = ObjectId::new();
        let mut watched_card = CardData::new(watched_id, p1, "Watched");
        watched_card.card_types = vec![CardType::Creature];
        watched_card.power = Some(2);
        watched_card.toughness = Some(2);
        game.state.battlefield.add(Permanent::new(watched_card.clone(), p1));
        game.state.card_store.insert(watched_card);

        let other_id = ObjectId::new();
        let mut other_card = CardData::new(other_id, p2, "Other");
        other_card.card_types = vec![CardType::Creature];
        other_card.power = Some(2);
        other_card.toughness = Some(2);
        game.state.battlefield.add(Permanent::new(other_card.clone(), p2));
        game.state.card_store.insert(other_card);

        // Delayed trigger watching the first creature
        game.execute_effects(
            &[Effect::delayed_on_death(vec![Effect::GainLife { amount: 5 }])],
            p1, &[watched_id], None, None,
        );

        let life_before = game.state.players.get(&p1).unwrap().life;

        // Kill the OTHER creature (should NOT trigger)
        game.state.battlefield.remove(other_id);
        game.emit_event(GameEvent::dies(other_id, p2));
        game.check_triggered_abilities();

        let life_after = game.state.players.get(&p1).unwrap().life;
        assert_eq!(life_after, life_before, "Should NOT gain life when wrong creature dies");

        // Trigger should still be registered
        assert_eq!(game.state.delayed_triggers.len(), 1);
    }

    #[test]
    fn delayed_trigger_expires_at_end_of_turn() {
        let (mut game, p1, _p2) = setup_delayed_game();

        // Create a delayed trigger with EndOfTurn duration
        game.execute_effects(
            &[Effect::delayed_on_death(vec![Effect::GainLife { amount: 5 }])],
            p1, &[], Some(ObjectId::new()), None,
        );
        assert_eq!(game.state.delayed_triggers.len(), 1);

        // Cleanup step should remove EndOfTurn delayed triggers
        game.turn_based_actions(PhaseStep::Cleanup, p1);
        assert_eq!(game.state.delayed_triggers.len(), 0,
            "EndOfTurn delayed trigger should be removed at cleanup");
    }
    
    #[test]
    fn at_next_end_step_fires_once() {
        let (mut game, p1, _p2) = setup_delayed_game();

        // Put a card in library
        let card_id = ObjectId::new();
        let card = CardData::new(card_id, p1, "Prize");
        game.state.card_store.insert(card);
        game.state.players.get_mut(&p1).unwrap().library.put_on_top(card_id);

        // Create "at the beginning of the next end step, draw a card"
        game.execute_effects(
            &[Effect::at_next_end_step(vec![Effect::DrawCards { count: 1 }])],
            p1, &[], None, None,
        );
        assert_eq!(game.state.delayed_triggers.len(), 1);

        let hand_before = game.state.players.get(&p1).unwrap().hand.len();

        // Emit end step event
        let mut event = GameEvent::new(EventType::EndStep);
        event.player_id = Some(p1);
        game.emit_event(event);
        game.check_triggered_abilities();

        let hand_after = game.state.players.get(&p1).unwrap().hand.len();
        assert_eq!(hand_after, hand_before + 1, "Should draw on end step");

        // Trigger should be removed (trigger_only_once)
        assert_eq!(game.state.delayed_triggers.len(), 0);

}
