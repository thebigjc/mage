// Tests extracted from game.rs

use crate::game::*;
use crate::abilities::{Ability, Effect, TargetSpec};
use crate::card::CardData;
use crate::constants::{CardType, Outcome, PhaseStep, TurnPhase};
use crate::decision::{AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction, PlayerAgent, PlayerDecisionMaker, ReplacementEffectChoice, TargetRequirement, UnpaidMana};
use crate::events::{EventType, GameEvent};
use crate::permanent::Permanent;
use crate::types::{ObjectId, PlayerId, Power, Toughness};

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
        p1_dm: PlayerAgent,
        p2_dm: PlayerAgent,
    ) -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
            ],
            starting_life: Life::new(20),
        };
        let game = Game::new_two_player(config, vec![(p1, p1_dm), (p2, p2_dm)]);
        (game, p1, p2)
    }

    #[test]
    fn etb_trigger_fires_and_resolves() {
        let (mut game, p1, _p2) = setup(
            PlayerAgent::new(TriggerTestPlayer::passive()),
            PlayerAgent::new(TriggerTestPlayer::passive()),
        );

        // Create a creature with an ETB trigger: "When this enters, gain 3 life"
        let card_id = ObjectId::new();
        let mut card = CardData::new(card_id, p1, "Soul Warden");
        card.card_types = vec![CardType::Creature];
        card.power = Some(Power::new(1));
        card.toughness = Some(Toughness::new(1));
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
            PlayerAgent::new(TriggerTestPlayer::attacker()),
            PlayerAgent::new(TriggerTestPlayer::passive()),
        );

        // Create creature with attack trigger: "Whenever this attacks, each opponent loses 1 life"
        let card_id = ObjectId::new();
        let mut card = CardData::new(card_id, p1, "Pulse Tracker");
        card.card_types = vec![CardType::Creature];
        card.power = Some(Power::new(1));
        card.toughness = Some(Toughness::new(1));
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
            PlayerAgent::new(TriggerTestPlayer::passive()),
            PlayerAgent::new(TriggerTestPlayer::passive()),
        );

        // Create "Ajani's Pridemate" — whenever you gain life, put a +1/+1 counter
        let card_id = ObjectId::new();
        let mut card = CardData::new(card_id, p1, "Ajani's Pridemate");
        card.card_types = vec![CardType::Creature];
        card.power = Some(Power::new(2));
        card.toughness = Some(Toughness::new(2));
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
            PlayerAgent::new(TriggerTestPlayer::passive()),
            PlayerAgent::new(TriggerTestPlayer::passive()),
        );

        // A "may" trigger that the player says yes to (TriggerTestPlayer says yes)
        let card_id = ObjectId::new();
        let mut card = CardData::new(card_id, p1, "Optional Creature");
        card.card_types = vec![CardType::Creature];
        card.power = Some(Power::new(1));
        card.toughness = Some(Toughness::new(1));
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
            PlayerAgent::new(TriggerTestPlayer::passive()),
            PlayerAgent::new(TriggerTestPlayer::passive()),
        );

        // p1's creature has attack trigger
        let card_id = ObjectId::new();
        let mut card = CardData::new(card_id, p1, "TriggerCreature");
        card.card_types = vec![CardType::Creature];
        card.power = Some(Power::new(2));
        card.toughness = Some(Toughness::new(2));
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
        other.power = Some(Power::new(1));
        other.toughness = Some(Toughness::new(1));
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
            starting_life: Life::new(20),
        };
        let game = Game::new_two_player(
            config,
            vec![(p1, PlayerAgent::new(PassivePlayer)), (p2, PlayerAgent::new(PassivePlayer))],
        );
        (game, p1, p2)
    }

    #[test]
    fn dies_trigger_fires_on_lethal_damage() {
        let (mut game, p1, _p2) = setup2();

        // Create a creature with "When this creature dies, draw a card"
        let mut card = CardData::new(ObjectId::new(), p1, "Doomed Traveler");
        card.card_types = vec![CardType::Creature];
        card.power = Some(Power::new(1));
        card.toughness = Some(Toughness::new(1));
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
            PlayerAgent::new(PassivePlayer),
            PlayerAgent::new(PassivePlayer),
        );

        // Create a creature with dies trigger controlled by p2
        let mut card = CardData::new(ObjectId::new(), p2, "Blood Artist");
        card.card_types = vec![CardType::Creature];
        card.power = Some(Power::new(0));
        card.toughness = Some(Toughness::new(1));
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
            PlayerAgent::new(PassivePlayer),
            PlayerAgent::new(PassivePlayer),
        );

        // Creature A has a dies trigger
        let mut card_a = CardData::new(ObjectId::new(), p1, "Creature A");
        card_a.card_types = vec![CardType::Creature];
        card_a.power = Some(Power::new(1));
        card_a.toughness = Some(Toughness::new(1));
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
        card_b.power = Some(Power::new(1));
        card_b.toughness = Some(Toughness::new(1));
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



    #[test]
    fn upkeep_trigger_fires_on_upkeep_step() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "A".into(), deck: make_deck(p1) },
                PlayerConfig { name: "B".into(), deck: make_deck(p2) },
            ],
            starting_life: Life::new(20),
        };
        let mut game = Game::new_two_player(
            config,
            vec![(p1, PlayerAgent::new(PassivePlayer)), (p2, PlayerAgent::new(PassivePlayer))],
        );
        game.state.active_player = p1;
        game.state.priority_player = p1;

        // Create a creature with upkeep trigger (gain 1 life at upkeep)
        let creature_id = ObjectId::new();
        let mut card = CardData::new(creature_id, p1, "Upkeep Healer");
        card.card_types = vec![CardType::Creature];
        card.power = Some(Power::new(1));
        card.toughness = Some(Toughness::new(1));
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
            starting_life: Life::new(20),
        };
        let mut game = Game::new_two_player(
            config,
            vec![(p1, PlayerAgent::new(PassivePlayer)), (p2, PlayerAgent::new(PassivePlayer))],
        );
        game.state.active_player = p1;
        game.state.priority_player = p1;

        // Create a creature with end step trigger (draw a card)
        let creature_id = ObjectId::new();
        let mut card = CardData::new(creature_id, p1, "End Step Draw");
        card.card_types = vec![CardType::Creature];
        card.power = Some(Power::new(2));
        card.toughness = Some(Toughness::new(2));
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
            starting_life: Life::new(20),
        };
        let mut game = Game::new_two_player(
            config,
            vec![(p1, PlayerAgent::new(PassivePlayer)), (p2, PlayerAgent::new(PassivePlayer))],
        );
        game.state.active_player = p1;
        game.state.priority_player = p1;

        // Create an upkeep trigger creature controlled by p2
        let creature_id = ObjectId::new();
        let mut card = CardData::new(creature_id, p2, "Opponent Healer");
        card.card_types = vec![CardType::Creature];
        card.power = Some(Power::new(1));
        card.toughness = Some(Toughness::new(1));
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

    fn setup_delayed_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            starting_life: Life::new(20),
            players: vec![
                PlayerConfig { name: "P1".into(), deck: vec![] },
                PlayerConfig { name: "P2".into(), deck: vec![] },
            ],
        };
        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, PlayerAgent::new(PassivePlayer)),
                (p2, PlayerAgent::new(PassivePlayer)),
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
        card.power = Some(Power::new(2));
        card.toughness = Some(Toughness::new(2));
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
        game.emit_event(GameEvent::dies(creature_id, p1, 0));
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
        watched_card.power = Some(Power::new(2));
        watched_card.toughness = Some(Toughness::new(2));
        game.state.battlefield.add(Permanent::new(watched_card.clone(), p1));
        game.state.card_store.insert(watched_card);

        let other_id = ObjectId::new();
        let mut other_card = CardData::new(other_id, p2, "Other");
        other_card.card_types = vec![CardType::Creature];
        other_card.power = Some(Power::new(2));
        other_card.toughness = Some(Toughness::new(2));
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
        game.emit_event(GameEvent::dies(other_id, p2, 0));
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

    #[test]
    fn other_creature_etb_trigger_fires() {
        let (mut game, p1, _p2) = setup(
            PlayerAgent::new(TriggerTestPlayer::passive()),
            PlayerAgent::new(TriggerTestPlayer::passive()),
        );

        let warden_id = ObjectId::new();
        let mut warden = CardData::new(warden_id, p1, "Warden");
        warden.card_types = vec![CardType::Creature];
        warden.power = Some(Power::new(1));
        warden.toughness = Some(Toughness::new(1));
        warden.abilities.push(Ability::other_creature_etb_triggered(
            warden_id,
            "Whenever another creature enters under your control, gain 1 life.",
            vec![Effect::GainLife { amount: 1 }],
            TargetSpec::None,
        ));
        game.state.card_store.insert(warden.clone());
        for ab in &warden.abilities { game.state.ability_store.add(ab.clone()); }
        let perm = Permanent::new(warden, p1);
        game.state.battlefield.add(perm);
        game.state.set_zone(warden_id, crate::constants::Zone::Battlefield, None);

        let bear_id = ObjectId::new();
        let mut bear = CardData::new(bear_id, p1, "Bear");
        bear.card_types = vec![CardType::Creature];
        bear.power = Some(Power::new(2));
        bear.toughness = Some(Toughness::new(2));
        game.state.card_store.insert(bear.clone());
        let perm2 = Permanent::new(bear, p1);
        game.state.battlefield.add(perm2);
        game.state.set_zone(bear_id, crate::constants::Zone::Battlefield, None);
        game.emit_event(GameEvent::enters_battlefield(bear_id, p1));

        assert_eq!(game.state.players[&p1].life, 20);
        game.process_sba_and_triggers();
        assert!(!game.state.stack.is_empty());
        game.resolve_top_of_stack();
        assert_eq!(game.state.players[&p1].life, 21);
    }

    #[test]
    fn other_creature_etb_does_not_trigger_for_self() {
        let (mut game, p1, _p2) = setup(
            PlayerAgent::new(TriggerTestPlayer::passive()),
            PlayerAgent::new(TriggerTestPlayer::passive()),
        );

        let warden_id = ObjectId::new();
        let mut warden = CardData::new(warden_id, p1, "Warden");
        warden.card_types = vec![CardType::Creature];
        warden.power = Some(Power::new(1));
        warden.toughness = Some(Toughness::new(1));
        warden.abilities.push(Ability::other_creature_etb_triggered(
            warden_id,
            "Whenever another creature enters under your control, gain 1 life.",
            vec![Effect::GainLife { amount: 1 }],
            TargetSpec::None,
        ));
        game.state.card_store.insert(warden.clone());
        for ab in &warden.abilities { game.state.ability_store.add(ab.clone()); }
        let perm = Permanent::new(warden, p1);
        game.state.battlefield.add(perm);
        game.state.set_zone(warden_id, crate::constants::Zone::Battlefield, None);
        game.emit_event(GameEvent::enters_battlefield(warden_id, p1));

        game.process_sba_and_triggers();
        assert!(game.state.stack.is_empty(), "Should not trigger for self entering");
    }

    #[test]
    fn other_creature_etb_does_not_trigger_for_opponent() {
        let (mut game, p1, p2) = setup(
            PlayerAgent::new(TriggerTestPlayer::passive()),
            PlayerAgent::new(TriggerTestPlayer::passive()),
        );

        let warden_id = ObjectId::new();
        let mut warden = CardData::new(warden_id, p1, "Warden");
        warden.card_types = vec![CardType::Creature];
        warden.power = Some(Power::new(1));
        warden.toughness = Some(Toughness::new(1));
        warden.abilities.push(Ability::other_creature_etb_triggered(
            warden_id,
            "Whenever another creature enters under your control, gain 1 life.",
            vec![Effect::GainLife { amount: 1 }],
            TargetSpec::None,
        ));
        game.state.card_store.insert(warden.clone());
        for ab in &warden.abilities { game.state.ability_store.add(ab.clone()); }
        let perm = Permanent::new(warden, p1);
        game.state.battlefield.add(perm);
        game.state.set_zone(warden_id, crate::constants::Zone::Battlefield, None);

        let bear_id = ObjectId::new();
        let mut bear = CardData::new(bear_id, p2, "Bear");
        bear.card_types = vec![CardType::Creature];
        bear.power = Some(Power::new(2));
        bear.toughness = Some(Toughness::new(2));
        game.state.card_store.insert(bear.clone());
        let perm2 = Permanent::new(bear, p2);
        game.state.battlefield.add(perm2);
        game.state.set_zone(bear_id, crate::constants::Zone::Battlefield, None);
        game.emit_event(GameEvent::enters_battlefield(bear_id, p2));

        game.process_sba_and_triggers();
        assert!(game.state.stack.is_empty(), "Should not trigger for opponent's creature");
    }

    #[test]
    fn graveyard_etb_trigger_fires_from_graveyard() {
        let (mut game, p1, _p2) = setup(
            PlayerAgent::new(TriggerTestPlayer::passive()),
            PlayerAgent::new(TriggerTestPlayer::passive()),
        );

        let diviner_id = ObjectId::new();
        let mut diviner = CardData::new(diviner_id, p1, "Diviner");
        diviner.card_types = vec![CardType::Creature];
        diviner.power = Some(Power::new(3));
        diviner.toughness = Some(Toughness::new(3));
        diviner.abilities.push(
            Ability::other_creature_etb_from_graveyard_triggered(
                diviner_id,
                "GY ETB: create token copy.",
                vec![Effect::create_token_copy_of_triggering()],
                TargetSpec::None,
            ).set_once_per_turn()
        );
        game.state.card_store.insert(diviner.clone());
        for ab in &diviner.abilities { game.state.ability_store.add(ab.clone()); }
        let perm = Permanent::new(diviner, p1);
        game.state.battlefield.add(perm);
        game.state.set_zone(diviner_id, crate::constants::Zone::Battlefield, None);

        let bear_id = ObjectId::new();
        let mut bear = CardData::new(bear_id, p1, "Bear");
        bear.card_types = vec![CardType::Creature];
        bear.power = Some(Power::new(2));
        bear.toughness = Some(Toughness::new(2));
        game.state.card_store.insert(bear.clone());
        let perm2 = Permanent::new(bear, p1);
        game.state.battlefield.add(perm2);
        game.state.set_zone(bear_id, crate::constants::Zone::Battlefield, None);
        game.emit_event(GameEvent::enters_battlefield_from(bear_id, p1, crate::constants::Zone::Graveyard));

        let bf_count_before = game.state.battlefield.iter().count();
        game.process_sba_and_triggers();
        assert!(!game.state.stack.is_empty(), "Should trigger from graveyard");
        game.resolve_top_of_stack();
        let bf_count_after = game.state.battlefield.iter().count();
        assert_eq!(bf_count_after, bf_count_before + 1, "Token copy should be created");
    }

    #[test]
    fn graveyard_etb_trigger_does_not_fire_from_hand() {
        let (mut game, p1, _p2) = setup(
            PlayerAgent::new(TriggerTestPlayer::passive()),
            PlayerAgent::new(TriggerTestPlayer::passive()),
        );

        let diviner_id = ObjectId::new();
        let mut diviner = CardData::new(diviner_id, p1, "Diviner");
        diviner.card_types = vec![CardType::Creature];
        diviner.power = Some(Power::new(3));
        diviner.toughness = Some(Toughness::new(3));
        diviner.abilities.push(
            Ability::other_creature_etb_from_graveyard_triggered(
                diviner_id,
                "GY ETB: create token copy.",
                vec![Effect::create_token_copy_of_triggering()],
                TargetSpec::None,
            ).set_once_per_turn()
        );
        game.state.card_store.insert(diviner.clone());
        for ab in &diviner.abilities { game.state.ability_store.add(ab.clone()); }
        let perm = Permanent::new(diviner, p1);
        game.state.battlefield.add(perm);
        game.state.set_zone(diviner_id, crate::constants::Zone::Battlefield, None);

        let bear_id = ObjectId::new();
        let mut bear = CardData::new(bear_id, p1, "Bear");
        bear.card_types = vec![CardType::Creature];
        bear.power = Some(Power::new(2));
        bear.toughness = Some(Toughness::new(2));
        game.state.card_store.insert(bear.clone());
        let perm2 = Permanent::new(bear, p1);
        game.state.battlefield.add(perm2);
        game.state.set_zone(bear_id, crate::constants::Zone::Battlefield, None);
        game.emit_event(GameEvent::enters_battlefield(bear_id, p1));

        game.process_sba_and_triggers();
        assert!(game.state.stack.is_empty(), "Should not trigger from hand (no from_zone)");
    }

    #[test]
    fn once_per_turn_trigger_only_fires_once() {
        let (mut game, p1, _p2) = setup(
            PlayerAgent::new(TriggerTestPlayer::passive()),
            PlayerAgent::new(TriggerTestPlayer::passive()),
        );

        let diviner_id = ObjectId::new();
        let mut diviner = CardData::new(diviner_id, p1, "Diviner");
        diviner.card_types = vec![CardType::Creature];
        diviner.power = Some(Power::new(3));
        diviner.toughness = Some(Toughness::new(3));
        diviner.abilities.push(
            Ability::other_creature_etb_from_graveyard_triggered(
                diviner_id,
                "GY ETB: gain 1 life once/turn.",
                vec![Effect::GainLife { amount: 1 }],
                TargetSpec::None,
            ).set_once_per_turn()
        );
        game.state.card_store.insert(diviner.clone());
        for ab in &diviner.abilities { game.state.ability_store.add(ab.clone()); }
        let perm = Permanent::new(diviner, p1);
        game.state.battlefield.add(perm);
        game.state.set_zone(diviner_id, crate::constants::Zone::Battlefield, None);

        let bear1_id = ObjectId::new();
        let mut bear1 = CardData::new(bear1_id, p1, "Bear1");
        bear1.card_types = vec![CardType::Creature];
        bear1.power = Some(Power::new(2));
        bear1.toughness = Some(Toughness::new(2));
        game.state.card_store.insert(bear1.clone());
        let perm2 = Permanent::new(bear1, p1);
        game.state.battlefield.add(perm2);
        game.state.set_zone(bear1_id, crate::constants::Zone::Battlefield, None);
        game.emit_event(GameEvent::enters_battlefield_from(bear1_id, p1, crate::constants::Zone::Graveyard));

        game.process_sba_and_triggers();
        assert!(!game.state.stack.is_empty());
        game.resolve_top_of_stack();
        assert_eq!(game.state.players[&p1].life, 21);

        let bear2_id = ObjectId::new();
        let mut bear2 = CardData::new(bear2_id, p1, "Bear2");
        bear2.card_types = vec![CardType::Creature];
        bear2.power = Some(Power::new(2));
        bear2.toughness = Some(Toughness::new(2));
        game.state.card_store.insert(bear2.clone());
        let perm3 = Permanent::new(bear2, p1);
        game.state.battlefield.add(perm3);
        game.state.set_zone(bear2_id, crate::constants::Zone::Battlefield, None);
        game.emit_event(GameEvent::enters_battlefield_from(bear2_id, p1, crate::constants::Zone::Graveyard));

        game.process_sba_and_triggers();
        assert!(game.state.stack.is_empty(), "Second trigger should be blocked by once-per-turn");
        assert_eq!(game.state.players[&p1].life, 21, "Life should not change on second trigger");
    }

    #[test]
    fn once_per_turn_resets_on_new_turn() {
        let (mut game, p1, _p2) = setup(
            PlayerAgent::new(TriggerTestPlayer::passive()),
            PlayerAgent::new(TriggerTestPlayer::passive()),
        );

        let diviner_id = ObjectId::new();
        let mut diviner = CardData::new(diviner_id, p1, "Diviner");
        diviner.card_types = vec![CardType::Creature];
        diviner.power = Some(Power::new(3));
        diviner.toughness = Some(Toughness::new(3));
        diviner.abilities.push(
            Ability::other_creature_etb_from_graveyard_triggered(
                diviner_id,
                "GY ETB: gain 1 life once/turn.",
                vec![Effect::GainLife { amount: 1 }],
                TargetSpec::None,
            ).set_once_per_turn()
        );
        game.state.card_store.insert(diviner.clone());
        for ab in &diviner.abilities { game.state.ability_store.add(ab.clone()); }
        let perm = Permanent::new(diviner, p1);
        game.state.battlefield.add(perm);
        game.state.set_zone(diviner_id, crate::constants::Zone::Battlefield, None);

        let bear1_id = ObjectId::new();
        let mut bear1 = CardData::new(bear1_id, p1, "Bear1");
        bear1.card_types = vec![CardType::Creature];
        bear1.power = Some(Power::new(2));
        bear1.toughness = Some(Toughness::new(2));
        game.state.card_store.insert(bear1.clone());
        let perm2 = Permanent::new(bear1, p1);
        game.state.battlefield.add(perm2);
        game.state.set_zone(bear1_id, crate::constants::Zone::Battlefield, None);
        game.emit_event(GameEvent::enters_battlefield_from(bear1_id, p1, crate::constants::Zone::Graveyard));

        game.process_sba_and_triggers();
        game.resolve_top_of_stack();
        assert_eq!(game.state.players[&p1].life, 21);

        game.state.trigger_counts_this_turn.clear();

        let bear2_id = ObjectId::new();
        let mut bear2 = CardData::new(bear2_id, p1, "Bear2");
        bear2.card_types = vec![CardType::Creature];
        bear2.power = Some(Power::new(2));
        bear2.toughness = Some(Toughness::new(2));
        game.state.card_store.insert(bear2.clone());
        let perm3 = Permanent::new(bear2, p1);
        game.state.battlefield.add(perm3);
        game.state.set_zone(bear2_id, crate::constants::Zone::Battlefield, None);
        game.emit_event(GameEvent::enters_battlefield_from(bear2_id, p1, crate::constants::Zone::Graveyard));

        game.process_sba_and_triggers();
        assert!(!game.state.stack.is_empty(), "Should trigger again after turn reset");
        game.resolve_top_of_stack();
        assert_eq!(game.state.players[&p1].life, 22);
    }

    #[test]
    fn create_token_copy_of_triggering_copies_creature() {
        let (mut game, p1, _p2) = setup(
            PlayerAgent::new(TriggerTestPlayer::passive()),
            PlayerAgent::new(TriggerTestPlayer::passive()),
        );

        let diviner_id = ObjectId::new();
        let mut diviner = CardData::new(diviner_id, p1, "Diviner");
        diviner.card_types = vec![CardType::Creature];
        diviner.power = Some(Power::new(3));
        diviner.toughness = Some(Toughness::new(3));
        diviner.abilities.push(
            Ability::other_creature_etb_from_graveyard_triggered(
                diviner_id,
                "GY ETB: create token copy.",
                vec![Effect::create_token_copy_of_triggering()],
                TargetSpec::None,
            ).set_once_per_turn()
        );
        game.state.card_store.insert(diviner.clone());
        for ab in &diviner.abilities { game.state.ability_store.add(ab.clone()); }
        let perm = Permanent::new(diviner, p1);
        game.state.battlefield.add(perm);
        game.state.set_zone(diviner_id, crate::constants::Zone::Battlefield, None);

        let dragon_id = ObjectId::new();
        let mut dragon = CardData::new(dragon_id, p1, "Big Dragon");
        dragon.card_types = vec![CardType::Creature];
        dragon.power = Some(Power::new(5));
        dragon.toughness = Some(Toughness::new(5));
        dragon.keywords = crate::constants::KeywordAbilities::FLYING;
        game.state.card_store.insert(dragon.clone());
        let perm2 = Permanent::new(dragon, p1);
        game.state.battlefield.add(perm2);
        game.state.set_zone(dragon_id, crate::constants::Zone::Battlefield, None);
        game.emit_event(GameEvent::enters_battlefield_from(dragon_id, p1, crate::constants::Zone::Graveyard));

        game.process_sba_and_triggers();
        assert!(!game.state.stack.is_empty());
        game.resolve_top_of_stack();

        let tokens: Vec<_> = game.state.battlefield.iter()
            .filter(|p| p.card.name == "Big Dragon" && p.card.is_token)
            .collect();
        assert_eq!(tokens.len(), 1, "Should create exactly one token copy");
        let token = &tokens[0];
        assert_eq!(token.card.power, Some(Power::new(5)));
        assert_eq!(token.card.toughness, Some(Toughness::new(5)));
        assert!(token.card.keywords.contains(crate::constants::KeywordAbilities::FLYING));
        assert_eq!(token.controller, p1);
    }

    #[test]
    fn reanimate_emits_etb_event() {
        let (mut game, p1, _p2) = setup(
            PlayerAgent::new(TriggerTestPlayer::passive()),
            PlayerAgent::new(TriggerTestPlayer::passive()),
        );

        let warden_id = ObjectId::new();
        let mut warden = CardData::new(warden_id, p1, "Warden");
        warden.card_types = vec![CardType::Creature];
        warden.power = Some(Power::new(1));
        warden.toughness = Some(Toughness::new(1));
        warden.abilities.push(Ability::enters_battlefield_triggered(
            warden_id,
            "When enters, gain 3 life.",
            vec![Effect::GainLife { amount: 3 }],
            TargetSpec::None,
        ));
        game.state.card_store.insert(warden.clone());
        game.state.players.get_mut(&p1).unwrap().graveyard.add(warden_id);
        game.state.set_zone(warden_id, crate::constants::Zone::Graveyard, None);

        game.execute_effects(
            &[Effect::Reanimate],
            p1,
            &[warden_id],
            None,
            None,
        );

        assert!(game.state.battlefield.contains(warden_id), "Card should be on battlefield");
        assert!(!game.event_log.is_empty(), "ETB event should have been emitted");
        let etb_events: Vec<_> = game.event_log.iter()
            .filter(|e| e.event_type == EventType::EnteredTheBattlefield)
            .collect();
        assert_eq!(etb_events.len(), 1);
        assert_eq!(etb_events[0].zone, Some(crate::constants::Zone::Graveyard));
    }

    #[test]
    fn trigger_scope_helper_constructors() {
        use crate::abilities::TriggerScope;
        let src = ObjectId::new();

        let self_etb = Ability::enters_battlefield_triggered(
            src, "self etb", vec![Effect::GainLife { amount: 1 }], TargetSpec::None,
        );
        assert_eq!(self_etb.trigger_scope, TriggerScope::SelfOnly);
        assert_eq!(self_etb.triggers_per_turn, 0);
        assert_eq!(self_etb.trigger_from_zone, None);

        let other_etb = Ability::other_creature_etb_triggered(
            src, "other etb", vec![Effect::GainLife { amount: 1 }], TargetSpec::None,
        );
        assert_eq!(other_etb.trigger_scope, TriggerScope::OtherControlled);
        assert_eq!(other_etb.triggers_per_turn, 0);
        assert_eq!(other_etb.trigger_from_zone, None);

        let gy_etb = Ability::other_creature_etb_from_graveyard_triggered(
            src, "gy etb", vec![Effect::GainLife { amount: 1 }], TargetSpec::None,
        ).set_once_per_turn();
        assert_eq!(gy_etb.trigger_scope, TriggerScope::OtherControlled);
        assert_eq!(gy_etb.triggers_per_turn, 1);
        assert_eq!(gy_etb.trigger_from_zone, Some(crate::constants::Zone::Graveyard));

        let any_dies = Ability::any_creature_dies_triggered(
            src, "any dies", vec![Effect::GainLife { amount: 1 }], TargetSpec::None,
        );
        assert_eq!(any_dies.trigger_scope, TriggerScope::Any);
    }

    #[test]
    fn controlled_creature_attacks_trigger_fires_for_other_creature() {
        let (mut game, p1, _p2) = setup(
            PlayerAgent::new(TriggerTestPlayer::passive()),
            PlayerAgent::new(TriggerTestPlayer::passive()),
        );

        let doran_id = ObjectId::new();
        let mut doran = CardData::new(doran_id, p1, "Doran");
        doran.card_types = vec![CardType::Creature];
        doran.power = Some(Power::new(0));
        doran.toughness = Some(Toughness::new(5));
        doran.abilities.push(Ability::controlled_creature_attacks_or_blocks_triggered(
            doran_id,
            "Whenever a creature you control attacks or blocks, you gain 1 life.",
            vec![Effect::GainLife { amount: 1 }],
            TargetSpec::None,
        ));
        game.state.card_store.insert(doran.clone());
        for ability in &doran.abilities {
            game.state.ability_store.add(ability.clone());
        }
        let mut perm = Permanent::new(doran, p1);
        perm.remove_summoning_sickness();
        game.state.battlefield.add(perm);

        let other_id = ObjectId::new();
        let mut other = CardData::new(other_id, p1, "Other Creature");
        other.card_types = vec![CardType::Creature];
        other.power = Some(Power::new(2));
        other.toughness = Some(Toughness::new(2));
        game.state.card_store.insert(other.clone());
        let mut perm2 = Permanent::new(other, p1);
        perm2.remove_summoning_sickness();
        game.state.battlefield.add(perm2);

        game.emit_event(
            GameEvent::new(EventType::AttackerDeclared)
                .target(other_id)
                .player(p1),
        );

        game.process_sba_and_triggers();
        assert!(!game.state.stack.is_empty());
        game.resolve_top_of_stack();
        assert_eq!(game.state.players[&p1].life, 21);
    }

    #[test]
    fn controlled_creature_attacks_trigger_not_for_opponent() {
        let (mut game, p1, p2) = setup(
            PlayerAgent::new(TriggerTestPlayer::passive()),
            PlayerAgent::new(TriggerTestPlayer::passive()),
        );

        let doran_id = ObjectId::new();
        let mut doran = CardData::new(doran_id, p1, "Doran");
        doran.card_types = vec![CardType::Creature];
        doran.power = Some(Power::new(0));
        doran.toughness = Some(Toughness::new(5));
        doran.abilities.push(Ability::controlled_creature_attacks_or_blocks_triggered(
            doran_id,
            "Whenever a creature you control attacks or blocks, you gain 1 life.",
            vec![Effect::GainLife { amount: 1 }],
            TargetSpec::None,
        ));
        game.state.card_store.insert(doran.clone());
        for ability in &doran.abilities {
            game.state.ability_store.add(ability.clone());
        }
        let perm = Permanent::new(doran, p1);
        game.state.battlefield.add(perm);

        let enemy_id = ObjectId::new();
        let mut enemy = CardData::new(enemy_id, p2, "Enemy Creature");
        enemy.card_types = vec![CardType::Creature];
        enemy.power = Some(Power::new(3));
        enemy.toughness = Some(Toughness::new(3));
        game.state.card_store.insert(enemy.clone());
        let mut perm2 = Permanent::new(enemy, p2);
        perm2.remove_summoning_sickness();
        game.state.battlefield.add(perm2);

        game.emit_event(
            GameEvent::new(EventType::AttackerDeclared)
                .target(enemy_id)
                .player(p2),
        );

        game.process_sba_and_triggers();
        assert!(game.state.stack.is_empty());
    }

    #[test]
    fn blocker_declared_trigger_fires() {
        let (mut game, p1, _p2) = setup(
            PlayerAgent::new(TriggerTestPlayer::passive()),
            PlayerAgent::new(TriggerTestPlayer::passive()),
        );

        let doran_id = ObjectId::new();
        let mut doran = CardData::new(doran_id, p1, "Doran");
        doran.card_types = vec![CardType::Creature];
        doran.power = Some(Power::new(0));
        doran.toughness = Some(Toughness::new(5));
        doran.abilities.push(Ability::controlled_creature_attacks_or_blocks_triggered(
            doran_id,
            "Whenever a creature you control attacks or blocks, you gain 1 life.",
            vec![Effect::GainLife { amount: 1 }],
            TargetSpec::None,
        ));
        game.state.card_store.insert(doran.clone());
        for ability in &doran.abilities {
            game.state.ability_store.add(ability.clone());
        }
        let perm = Permanent::new(doran, p1);
        game.state.battlefield.add(perm);

        let blocker_id = ObjectId::new();
        let mut blocker = CardData::new(blocker_id, p1, "Blocker");
        blocker.card_types = vec![CardType::Creature];
        blocker.power = Some(Power::new(1));
        blocker.toughness = Some(Toughness::new(4));
        game.state.card_store.insert(blocker.clone());
        let perm2 = Permanent::new(blocker, p1);
        game.state.battlefield.add(perm2);

        game.emit_event(
            GameEvent::new(EventType::BlockerDeclared)
                .target(blocker_id)
                .player(p1),
        );

        game.process_sba_and_triggers();
        assert!(!game.state.stack.is_empty());
        game.resolve_top_of_stack();
        assert_eq!(game.state.players[&p1].life, 21);
    }

    #[test]
    fn controlled_creature_attacks_or_blocks_helper() {
        let src = ObjectId::new();
        let ab = Ability::controlled_creature_attacks_or_blocks_triggered(
            src,
            "test",
            vec![Effect::GainLife { amount: 1 }],
            TargetSpec::None,
        );
        assert!(ab.trigger_events.contains(&EventType::AttackerDeclared));
        assert!(ab.trigger_events.contains(&EventType::BlockerDeclared));
        assert_eq!(ab.trigger_scope, TriggerScope::Any);
    }

    #[test]
    fn grant_triggered_ability_eot_draws_on_combat_damage() {
        let (mut game, p1, _p2) = setup_delayed_game();

        let draw_card_id = ObjectId::new();
        let draw_card = CardData::new(draw_card_id, p1, "Prize");
        game.state.card_store.insert(draw_card);
        game.state.players.get_mut(&p1).unwrap().library.put_on_top(draw_card_id);

        let creature_id = ObjectId::new();
        let mut card = CardData::new(creature_id, p1, "Attacker");
        card.card_types = vec![CardType::Creature];
        card.power = Some(Power::new(2));
        card.toughness = Some(Toughness::new(2));
        let perm = Permanent::new(card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card);

        let effects = vec![Effect::grant_triggered_ability_eot(
            "damaged_player",
            Filter::parse("creatures you control"),
            vec![Effect::DrawCards { count: 1 }],
        )];
        game.execute_effects(&effects, p1, &[], None, None);

        assert_eq!(game.state.delayed_triggers.len(), 1);
        assert!(!game.state.delayed_triggers[0].trigger_only_once);

        let hand_before = game.state.players.get(&p1).unwrap().hand.len();

        let mut dmg_event = GameEvent::new(EventType::DamagedPlayer);
        dmg_event.target_id = Some(creature_id);
        dmg_event.player_id = Some(_p2);
        dmg_event.amount = 2;
        game.emit_event(dmg_event);
        game.check_triggered_abilities();

        let hand_after = game.state.players.get(&p1).unwrap().hand.len();
        assert_eq!(hand_after, hand_before + 1,
            "Should draw a card when creature deals combat damage");
    }

    #[test]
    fn grant_triggered_ability_eot_fires_for_each_creature() {
        let (mut game, p1, p2) = setup_delayed_game();

        for i in 0..3 {
            let cid = ObjectId::new();
            let mut card = CardData::new(cid, p1, &format!("Prize {i}"));
            card.card_types = vec![CardType::Land];
            game.state.card_store.insert(card);
            game.state.players.get_mut(&p1).unwrap().library.put_on_top(cid);
        }

        let creature1_id = ObjectId::new();
        let mut c1 = CardData::new(creature1_id, p1, "Attacker A");
        c1.card_types = vec![CardType::Creature];
        c1.power = Some(Power::new(2));
        c1.toughness = Some(Toughness::new(2));
        game.state.battlefield.add(Permanent::new(c1.clone(), p1));
        game.state.card_store.insert(c1);

        let creature2_id = ObjectId::new();
        let mut c2 = CardData::new(creature2_id, p1, "Attacker B");
        c2.card_types = vec![CardType::Creature];
        c2.power = Some(Power::new(3));
        c2.toughness = Some(Toughness::new(3));
        game.state.battlefield.add(Permanent::new(c2.clone(), p1));
        game.state.card_store.insert(c2);

        let effects = vec![Effect::grant_triggered_ability_eot(
            "damaged_player",
            Filter::parse("creatures you control"),
            vec![Effect::DrawCards { count: 1 }],
        )];
        game.execute_effects(&effects, p1, &[], None, None);

        let hand_before = game.state.players.get(&p1).unwrap().hand.len();

        let mut dmg1 = GameEvent::new(EventType::DamagedPlayer);
        dmg1.target_id = Some(creature1_id);
        dmg1.player_id = Some(p2);
        dmg1.amount = 2;
        game.emit_event(dmg1);

        let mut dmg2 = GameEvent::new(EventType::DamagedPlayer);
        dmg2.target_id = Some(creature2_id);
        dmg2.player_id = Some(p2);
        dmg2.amount = 3;
        game.emit_event(dmg2);

        game.check_triggered_abilities();

        let hand_after = game.state.players.get(&p1).unwrap().hand.len();
        assert_eq!(hand_after, hand_before + 2,
            "Should draw 2 cards (one per attacking creature)");

        assert_eq!(game.state.delayed_triggers.len(), 1,
            "Trigger should still be active (not once-only)");
    }

    #[test]
    fn grant_triggered_ability_eot_ignores_opponent_creatures() {
        let (mut game, p1, p2) = setup_delayed_game();

        let opp_creature = ObjectId::new();
        let mut oc = CardData::new(opp_creature, p2, "Enemy");
        oc.card_types = vec![CardType::Creature];
        oc.power = Some(Power::new(4));
        oc.toughness = Some(Toughness::new(4));
        game.state.battlefield.add(Permanent::new(oc.clone(), p2));
        game.state.card_store.insert(oc);

        let effects = vec![Effect::grant_triggered_ability_eot(
            "damaged_player",
            Filter::parse("creatures you control"),
            vec![Effect::DrawCards { count: 1 }],
        )];
        game.execute_effects(&effects, p1, &[], None, None);

        let hand_before = game.state.players.get(&p1).unwrap().hand.len();

        let mut dmg = GameEvent::new(EventType::DamagedPlayer);
        dmg.target_id = Some(opp_creature);
        dmg.player_id = Some(p1);
        dmg.amount = 4;
        game.emit_event(dmg);
        game.check_triggered_abilities();

        let hand_after = game.state.players.get(&p1).unwrap().hand.len();
        assert_eq!(hand_after, hand_before,
            "Should NOT draw when opponent's creature deals damage");
    }
