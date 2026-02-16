// Tests extracted from game.rs

use crate::game::*;
use crate::abilities::{Ability, Cost, Effect, StaticEffect};
use crate::card::CardData;
use crate::constants::{CardType, KeywordAbilities, Outcome, PhaseStep, SubType, TurnPhase};
use crate::counters::CounterType;
use crate::decision::{AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction, PlayerDecisionMaker, ReplacementEffectChoice, TargetRequirement, UnpaidMana};
use crate::mana::{Mana, ManaCost};
use crate::permanent::Permanent;
use crate::types::{ObjectId, PlayerId};


#[cfg(test)]

    struct AlwaysPassPlayer;
    impl PlayerDecisionMaker for AlwaysPassPlayer {
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

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        (0..20).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    fn setup() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "A".into(), deck: make_deck(p1) },
                PlayerConfig { name: "B".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(config, vec![
            (p1, Box::new(AlwaysPassPlayer)),
            (p2, Box::new(AlwaysPassPlayer)),
        ]);
        (game, p1, p2)
    }

    fn add_colored_creature(game: &mut Game, owner: PlayerId, name: &str, mana: &str) -> ObjectId {
        let id = ObjectId::new();
        let mut card = CardData::new(id, owner, name);
        card.card_types = vec![CardType::Creature];
        card.mana_cost = ManaCost::parse(mana);
        card.power = Some(2);
        card.toughness = Some(2);
        card.keywords = KeywordAbilities::empty();
        game.state.battlefield.add(Permanent::new(card, owner));
        id
    }

    #[test]
    fn count_colors_zero_for_colorless() {
        let (game, p1, _) = setup();
        // Only lands (colorless) on battlefield
        assert_eq!(game.count_colors_among_permanents(p1), 0);
    }

    #[test]
    fn count_colors_counts_distinct() {
        let (mut game, p1, _) = setup();
        // Add a red creature
        add_colored_creature(&mut game, p1, "Goblin", "{R}");
        assert_eq!(game.count_colors_among_permanents(p1), 1);

        // Add another red creature (still 1 color)
        add_colored_creature(&mut game, p1, "Goblin 2", "{1}{R}");
        assert_eq!(game.count_colors_among_permanents(p1), 1);

        // Add a green creature (now 2 colors)
        add_colored_creature(&mut game, p1, "Elf", "{G}");
        assert_eq!(game.count_colors_among_permanents(p1), 2);

        // Add a multicolor creature (adds blue and white)
        add_colored_creature(&mut game, p1, "Angel", "{W}{U}");
        assert_eq!(game.count_colors_among_permanents(p1), 4);
    }

    #[test]
    fn vivid_gain_life() {
        let (mut game, p1, _) = setup();
        // 3 colors
        add_colored_creature(&mut game, p1, "R", "{R}");
        add_colored_creature(&mut game, p1, "G", "{G}");
        add_colored_creature(&mut game, p1, "B", "{B}");

        game.execute_effects(&[Effect::GainLifeVivid], p1, &[], None, None);
        assert_eq!(game.state.players[&p1].life, 23); // 20 + 3
    }

    #[test]
    fn vivid_deal_damage_to_creature() {
        let (mut game, p1, p2) = setup();
        // p1 has 2 colors
        add_colored_creature(&mut game, p1, "R", "{R}");
        add_colored_creature(&mut game, p1, "G", "{G}");
        // p2 has a creature to target
        let target = add_colored_creature(&mut game, p2, "Bear", "{1}{W}");

        game.execute_effects(&[Effect::DealDamageVivid], p1, &[target], None, None);
        assert_eq!(game.state.battlefield.get(target).unwrap().damage, 2);
    }

    #[test]
    fn vivid_boost_until_eot() {
        let (mut game, p1, _) = setup();
        // p1 has 3 colors
        add_colored_creature(&mut game, p1, "R", "{R}");
        add_colored_creature(&mut game, p1, "G", "{G}");
        let target = add_colored_creature(&mut game, p1, "B", "{B}");

        game.execute_effects(&[Effect::BoostUntilEotVivid], p1, &[target], None, None);
        let perm = game.state.battlefield.get(target).unwrap();
        assert_eq!(perm.power(), 5); // 2 base + 3 vivid
        assert_eq!(perm.toughness(), 5);
    }

    #[test]
    fn vivid_create_tokens() {
        let (mut game, p1, _) = setup();
        // p1 has 3 colors
        add_colored_creature(&mut game, p1, "R", "{R}");
        add_colored_creature(&mut game, p1, "G", "{G}");
        add_colored_creature(&mut game, p1, "B", "{B}");

        let before = game.state.battlefield.controlled_by(p1).count();
        game.execute_effects(&[Effect::create_token_vivid("1/1 Kithkin")], p1, &[], None, None);
        let after = game.state.battlefield.controlled_by(p1).count();
        assert_eq!(after - before, 3); // 3 tokens for 3 colors
    }


    // Additional tests

    /// Decision maker that always says "yes" to choose_use.
    struct AlwaysPayPlayer;
    impl PlayerDecisionMaker for AlwaysPayPlayer {
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

    /// Decision maker that always says "no" to choose_use.
    struct NeverPayPlayer;
    impl PlayerDecisionMaker for NeverPayPlayer {
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

    fn make_deck2(owner: PlayerId) -> Vec<CardData> {
        (0..20).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    #[test]
    fn do_if_cost_paid_pays_and_executes() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "A".into(), deck: make_deck2(p1) },
                PlayerConfig { name: "B".into(), deck: make_deck2(p2) },
            ],
            starting_life: 20,
        };
        let mut game = Game::new_two_player(config, vec![
            (p1, Box::new(AlwaysPayPlayer)),
            (p2, Box::new(NeverPayPlayer)),
        ]);

        // Add a creature for source
        let src_id = ObjectId::new();
        let mut card = CardData::new(src_id, p1, "Source");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        card.keywords = KeywordAbilities::empty();
        game.state.battlefield.add(Permanent::new(card, p1));

        // "You may blight 1. If you do, gain 3 life."
        let effect = Effect::do_if_cost_paid(
            Cost::Blight(1),
            vec![Effect::GainLife { amount: 3 }],
            vec![],
        );

        game.execute_effects(&[effect], p1, &[], Some(src_id), None);

        // AlwaysPayPlayer says yes, blight adds -1/-1, gain 3 life
        assert_eq!(game.state.battlefield.get(src_id).unwrap().counters.get(&CounterType::M1M1), 1);
        assert_eq!(game.state.players[&p1].life, 23); // 20 + 3
    }

    #[test]
    fn do_if_cost_paid_declines_runs_else() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "A".into(), deck: make_deck(p1) },
                PlayerConfig { name: "B".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let mut game = Game::new_two_player(config, vec![
            (p1, Box::new(NeverPayPlayer)),
            (p2, Box::new(NeverPayPlayer)),
        ]);

        // Add a creature for source
        let src_id = ObjectId::new();
        let mut card = CardData::new(src_id, p1, "Source");
        card.card_types = vec![CardType::Creature];
        card.power = Some(5);
        card.toughness = Some(4);
        card.keywords = KeywordAbilities::empty();
        game.state.battlefield.add(Permanent::new(card, p1));

        // "You may pay 3 life. If you don't, blight 2."
        let effect = Effect::do_if_cost_paid(
            Cost::PayLife(3),
            vec![],
            vec![Effect::add_counters_self("-1/-1", 2)],
        );

        game.execute_effects(&[effect], p1, &[], Some(src_id), None);

        // NeverPayPlayer says no, so blight 2 happens
        assert_eq!(game.state.players[&p1].life, 20); // no life paid
        assert_eq!(game.state.battlefield.get(src_id).unwrap().counters.get(&CounterType::M1M1), 2);
    }


    // Additional tests

    /// Decision maker that picks a given index for choose_option.
    struct OptionPicker(usize);

    impl PlayerDecisionMaker for OptionPicker {
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
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize {
            self.0
        }
    }

    fn make_deck3(owner: PlayerId) -> Vec<CardData> {
        (0..20).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    fn setup_game_with_picker(pick_index: usize) -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".into(), deck: make_deck3(p1) },
                PlayerConfig { name: "Bob".into(), deck: make_deck3(p2) },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(OptionPicker(pick_index))),
                (p2, Box::new(OptionPicker(0))),
            ],
        );
        (game, p1, p2)
    }

    #[test]
    fn choose_creature_type_stores_on_permanent() {
        let (mut game, p1, _p2) = setup_game_with_picker(0); // picks first = "Elemental"

        let src_id = ObjectId::new();
        let mut card = CardData::new(src_id, p1, "Chronicle of Victory");
        card.card_types = vec![CardType::Artifact];
        game.state.battlefield.add(Permanent::new(card, p1));

        let effects = vec![Effect::choose_creature_type_restricted(
            vec!["Elemental", "Elf", "Faerie"]
        )];
        game.execute_effects(&effects, p1, &[], Some(src_id), None);

        let perm = game.state.battlefield.get(src_id).unwrap();
        assert!(perm.chosen_type.is_some());
        match &perm.chosen_type {
            Some(SubType::Custom(s)) => assert_eq!(s.as_str(), "Elemental"),
            other => panic!("Expected SubType::Custom(\"Elemental\"), got {:?}", other),
        }
    }

    #[test]
    fn choose_creature_type_picks_second_option() {
        let (mut game, p1, _p2) = setup_game_with_picker(1); // picks second = "Elf"

        let src_id = ObjectId::new();
        let mut card = CardData::new(src_id, p1, "Test Permanent");
        card.card_types = vec![CardType::Artifact];
        game.state.battlefield.add(Permanent::new(card, p1));

        let effects = vec![Effect::choose_creature_type_restricted(
            vec!["Goblin", "Elf", "Merfolk"]
        )];
        game.execute_effects(&effects, p1, &[], Some(src_id), None);

        let perm = game.state.battlefield.get(src_id).unwrap();
        match &perm.chosen_type {
            Some(SubType::Custom(s)) => assert_eq!(s.as_str(), "Elf"),
            other => panic!("Expected SubType::Custom(\"Elf\"), got {:?}", other),
        }
    }

    #[test]
    fn choose_type_and_draw_per_permanent() {
        // Pick index 4 = "Goblin" from the default list
        let (mut game, p1, _p2) = setup_game_with_picker(4);

        // Place 3 Goblins and 1 Elf on battlefield
        for i in 0..3 {
            let cid = ObjectId::new();
            let mut card = CardData::new(cid, p1, &format!("Goblin #{}", i));
            card.card_types = vec![CardType::Creature];
            card.subtypes = vec![SubType::Custom("Goblin".into())];
            game.state.battlefield.add(Permanent::new(card, p1));
        }
        {
            let cid = ObjectId::new();
            let mut card = CardData::new(cid, p1, "Some Elf");
            card.card_types = vec![CardType::Creature];
            card.subtypes = vec![SubType::Elf];
            game.state.battlefield.add(Permanent::new(card, p1));
        }

        let hand_before = game.state.players[&p1].hand.len();
        let effects = vec![Effect::choose_type_and_draw_per_permanent()];
        game.execute_effects(&effects, p1, &[], None, None);

        // Should have drawn 3 cards (3 Goblins)
        let hand_after = game.state.players[&p1].hand.len();
        assert_eq!(hand_after - hand_before, 3);
    }


    // Additional tests

    struct PassivePlayer;
    impl PlayerDecisionMaker for PassivePlayer {
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

    fn make_deck4(owner: PlayerId) -> Vec<CardData> {
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
                PlayerConfig { name: "Player1".into(), deck: make_deck4(p1) },
                PlayerConfig { name: "Player2".into(), deck: make_deck4(p2) },
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
    fn enters_tapped_self_filter_taps_permanent() {
        let (mut game, p1, _p2) = setup2();

        // Create a guildgate-like land that enters tapped
        let mut card = CardData::new(ObjectId::new(), p1, "Azorius Guildgate");
        card.card_types = vec![CardType::Land];
        card.subtypes = vec![SubType::Gate];
        let id = card.id;
        card.abilities = vec![
            Ability::static_ability(id, "Azorius Guildgate enters tapped.",
                vec![StaticEffect::EntersTapped { filter: "self".into() }]),
            Ability::mana_ability(id, "{T}: Add {W}.", Mana::white(1)),
        ];
        // Register abilities first
        for ability in &card.abilities {
            game.state.ability_store.add(ability.clone());
        }
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);
        game.check_enters_tapped(id);

        // Should be tapped
        assert!(game.state.battlefield.get(id).unwrap().tapped);
    }

    #[test]
    fn regular_land_enters_untapped() {
        let (mut game, p1, _p2) = setup2();

        let mut card = CardData::new(ObjectId::new(), p1, "Forest");
        card.card_types = vec![CardType::Land];
        let id = card.id;
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);
        game.check_enters_tapped(id);

        // Should NOT be tapped
        assert!(!game.state.battlefield.get(id).unwrap().tapped);
    }

    #[test]
    fn creature_without_enters_tapped_stays_untapped() {
        let (mut game, p1, _p2) = setup();

        let mut card = CardData::new(ObjectId::new(), p1, "Grizzly Bears");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        let id = card.id;
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);
        game.check_enters_tapped(id);

        assert!(!game.state.battlefield.get(id).unwrap().tapped);
    }


    // Additional tests

    fn setup_impulse_game() -> (Game, PlayerId, PlayerId) {
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

    /// Add N cards to a player's library.
    fn add_library_cards(game: &mut Game, player: PlayerId, n: usize) -> Vec<ObjectId> {
        let mut ids = Vec::new();
        for i in 0..n {
            let id = ObjectId::new();
            let mut card = CardData::new(id, player, &format!("Library Card {}", i));
            card.card_types = vec![CardType::Creature];
            card.power = Some(2);
            card.toughness = Some(2);
            card.mana_cost = ManaCost::parse("{1}{R}");
            game.state.card_store.insert(card);
            game.state.players.get_mut(&player).unwrap().library.put_on_top(id);
            ids.push(id);
        }
        ids
    }

    #[test]
    fn exile_top_and_play_creates_impulse_entries() {
        let (mut game, p1, _p2) = setup_impulse_game();
        let _lib_ids = add_library_cards(&mut game, p1, 3);

        // Execute ExileTopAndPlay effect
        game.execute_effects(
            &[Effect::exile_top_and_play(2)],
            p1, &[], None, None,
        );

        // Should have exiled 2 cards and created 2 impulse entries
        assert_eq!(game.state.impulse_playable.len(), 2);
        assert_eq!(game.state.players.get(&p1).unwrap().library.len(), 1);

        // Exiled cards should be in exile zone
        for ip in &game.state.impulse_playable {
            assert!(game.state.exile.contains(ip.card_id));
            assert_eq!(ip.player_id, p1);
        }
    }

    #[test]
    fn impulse_cards_appear_in_legal_actions() {
        let (mut game, p1, _p2) = setup_impulse_game();
        let _lib_ids = add_library_cards(&mut game, p1, 3);

        // Give P1 mana to cast
        game.state.players.get_mut(&p1).unwrap()
            .mana_pool.add(Mana { red: 2, generic: 2, ..Mana::new() }, None, false);

        // Execute ExileTopAndPlay
        game.execute_effects(
            &[Effect::exile_top_and_play(1)],
            p1, &[], None, None,
        );

        let actions = game.compute_legal_actions(p1);
        let cast_actions: Vec<_> = actions.iter()
            .filter(|a| matches!(a, crate::decision::PlayerAction::CastSpell { .. }))
            .collect();
        assert!(!cast_actions.is_empty(), "Should be able to cast impulse-exiled card");
    }

    #[test]
    fn cast_from_exile_resolves() {
        let (mut game, p1, _p2) = setup_impulse_game();
        let _lib_ids = add_library_cards(&mut game, p1, 3);

        // Give P1 mana
        game.state.players.get_mut(&p1).unwrap()
            .mana_pool.add(Mana { red: 2, generic: 2, ..Mana::new() }, None, false);

        // Execute ExileTopAndPlay (1 card)
        game.execute_effects(
            &[Effect::exile_top_and_play(1)],
            p1, &[], None, None,
        );

        let impulse_card_id = game.state.impulse_playable[0].card_id;

        // Cast the exiled card
        game.cast_spell(p1, impulse_card_id);

        // Card should be on stack (not in exile anymore)
        assert!(!game.state.exile.contains(impulse_card_id));
        assert!(!game.state.stack.is_empty());

        // Impulse entry should be removed
        assert!(game.state.impulse_playable.is_empty());

        // Resolve the spell — it's a creature, should go to battlefield
        game.resolve_top_of_stack();
        assert!(game.state.battlefield.contains(impulse_card_id));
    }

    #[test]
    fn impulse_expires_at_end_of_turn() {
        let (mut game, p1, _p2) = setup_impulse_game();
        let _lib_ids = add_library_cards(&mut game, p1, 3);

        // Execute ExileTopAndPlay
        game.execute_effects(
            &[Effect::exile_top_and_play(2)],
            p1, &[], None, None,
        );
        assert_eq!(game.state.impulse_playable.len(), 2);

        // Simulate cleanup step
        game.turn_based_actions(PhaseStep::Cleanup, p1);

        // All EndOfTurn impulse entries should be removed
        assert_eq!(game.state.impulse_playable.len(), 0);

        // Cards should still be in exile (just no longer playable)
        // (we can't track which cards were impulse vs regular exile without
        // the impulse entries, but they're still there)
    }

    #[test]
    fn impulse_next_turn_persists_through_opponent_cleanup() {
        let (mut game, p1, p2) = setup_impulse_game();
        let _lib_ids = add_library_cards(&mut game, p1, 3);

        // P1 exiles a card with "until end of next turn"
        game.execute_effects(
            &[Effect::exile_top_and_play_next_turn(1)],
            p1, &[], None, None,
        );
        assert_eq!(game.state.impulse_playable.len(), 1);

        // Simulate P1's cleanup (creation turn)
        game.turn_based_actions(PhaseStep::Cleanup, p1);
        // Should NOT expire on the creation turn (active_player is p1, but turn_number == created_turn)
        assert_eq!(game.state.impulse_playable.len(), 1,
            "UntilEndOfNextTurn should survive creation turn cleanup");

        // Simulate opponent's turn cleanup
        game.state.active_player = p2;
        game.state.turn_number = 2;
        game.turn_based_actions(PhaseStep::Cleanup, p2);
        assert_eq!(game.state.impulse_playable.len(), 1,
            "UntilEndOfNextTurn should survive opponent's cleanup");

        // Simulate P1's next turn cleanup (this is when it should expire)
        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.state.turn_number = 3;
        game.turn_based_actions(PhaseStep::Cleanup, p1);
        assert_eq!(game.state.impulse_playable.len(), 0,
            "UntilEndOfNextTurn should expire at controller's next turn cleanup");
    }

    #[test]
    fn exile_top_and_play_free_skips_mana() {
        let (mut game, p1, _p2) = setup_impulse_game();
        let _lib_ids = add_library_cards(&mut game, p1, 3);

        // NO mana given to P1

        // Execute ExileTopAndPlay with without_mana=true
        game.execute_effects(
            &[Effect::exile_top_and_play_free(1)],
            p1, &[], None, None,
        );

        let impulse_card_id = game.state.impulse_playable[0].card_id;

        // Should appear in legal actions even without mana
        let actions = game.compute_legal_actions(p1);
        let cast_actions: Vec<_> = actions.iter()
            .filter(|a| matches!(a, crate::decision::PlayerAction::CastSpell { card_id, .. } if *card_id == impulse_card_id))
            .collect();
        assert!(!cast_actions.is_empty(), "Should be able to cast free impulse card without mana");

        // Cast the card with no mana
        game.cast_spell(p1, impulse_card_id);

        // Should be on stack
        assert!(!game.state.stack.is_empty());
        // Mana should still be 0
        assert_eq!(game.state.players.get(&p1).unwrap().mana_pool.available().count(), 0);
    }


    // Additional tests

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

    fn add_creature_to_battlefield(game: &mut Game, owner: PlayerId, name: &str, subtype: SubType) -> ObjectId {
        let id = ObjectId::new();
        let mut card = CardData::new(id, owner, name);
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![subtype];
        card.power = Some(1);
        card.toughness = Some(1);
        game.state.card_store.insert(card);
        let card_clone = game.state.card_store.get(id).unwrap().clone();
        let perm = Permanent::new(card_clone, owner);
        game.state.battlefield.add(perm);
        id
    }

    fn add_creature_to_hand(game: &mut Game, owner: PlayerId, name: &str, subtype: SubType) -> ObjectId {
        let id = ObjectId::new();
        let mut card = CardData::new(id, owner, name);
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![subtype];
        card.power = Some(1);
        card.toughness = Some(1);
        game.state.card_store.insert(card);
        game.state.players.get_mut(&owner).unwrap().hand.add(id);
        id
    }

    #[test]
    fn behold_cost_with_battlefield_creature() {
        let (mut game, p1, _p2) = setup_game();

        // Put an Elf on the battlefield
        let elf_id = add_creature_to_battlefield(&mut game, p1, "Llanowar Elves", SubType::Elf);

        // Create a source permanent for the activated ability
        let source_id = add_creature_to_battlefield(&mut game, p1, "Source", SubType::Warrior);

        // Pay behold cost — should succeed (Elf on battlefield)
        assert!(game.pay_costs(p1, source_id, &[Cost::behold("Elf")]));
        // Elf should still be on battlefield (behold doesn't remove)
        assert!(game.state.battlefield.get(elf_id).is_some());
    }

    #[test]
    fn behold_cost_with_hand_creature() {
        let (mut game, p1, _p2) = setup_game();

        // Put an Elf in hand
        let elf_id = add_creature_to_hand(&mut game, p1, "Llanowar Elves", SubType::Elf);

        let source_id = add_creature_to_battlefield(&mut game, p1, "Source", SubType::Warrior);

        // Pay behold cost — should succeed (Elf in hand)
        assert!(game.pay_costs(p1, source_id, &[Cost::behold("Elf")]));
        // Elf should still be in hand (behold just reveals)
        assert!(game.state.players.get(&p1).unwrap().hand.contains(elf_id));
    }

    #[test]
    fn behold_cost_fails_without_matching_creature() {
        let (mut game, p1, _p2) = setup_game();

        // Only have a Goblin, need to behold an Elf
        let _goblin_id = add_creature_to_battlefield(&mut game, p1, "Goblin Piker", SubType::Goblin);
        let source_id = add_creature_to_battlefield(&mut game, p1, "Source", SubType::Warrior);

        assert!(!game.pay_costs(p1, source_id, &[Cost::behold("Elf")]));
    }

    #[test]
    fn behold_and_exile_removes_from_battlefield() {
        let (mut game, p1, _p2) = setup_game();

        let goblin_id = add_creature_to_battlefield(&mut game, p1, "Goblin Piker", SubType::Goblin);
        let source_id = add_creature_to_battlefield(&mut game, p1, "Source", SubType::Warrior);

        assert!(game.pay_costs(p1, source_id, &[Cost::behold_and_exile("Goblin")]));
        // Goblin should be exiled
        assert!(game.state.battlefield.get(goblin_id).is_none());
        assert!(game.state.exile.contains(goblin_id));
    }

    #[test]
    fn behold_and_exile_removes_from_hand() {
        let (mut game, p1, _p2) = setup_game();

        let goblin_id = add_creature_to_hand(&mut game, p1, "Goblin Piker", SubType::Goblin);
        let source_id = add_creature_to_battlefield(&mut game, p1, "Source", SubType::Warrior);

        assert!(game.pay_costs(p1, source_id, &[Cost::behold_and_exile("Goblin")]));
        // Goblin should be exiled from hand
        assert!(!game.state.players.get(&p1).unwrap().hand.contains(goblin_id));
        assert!(game.state.exile.contains(goblin_id));
    }

    #[test]
    fn behold_or_pay_prefers_behold() {
        let (mut game, p1, _p2) = setup_game();

        let kithkin_id = add_creature_to_battlefield(&mut game, p1, "Goldmeadow Harrier", SubType::Kithkin);
        let source_id = add_creature_to_battlefield(&mut game, p1, "Source", SubType::Warrior);

        // Give mana too
        game.state.players.get_mut(&p1).unwrap()
            .mana_pool.add(Mana { white: 2, ..Mana::new() }, None, false);

        // Behold or pay {2} — should succeed via behold (free), mana untouched
        assert!(game.pay_costs(p1, source_id, &[Cost::behold_or_pay("Kithkin", "{2}")]));
        // Kithkin still on battlefield (behold doesn't remove)
        assert!(game.state.battlefield.get(kithkin_id).is_some());
        // Mana should still be available (behold was free)
        assert_eq!(game.state.players.get(&p1).unwrap().mana_pool.total_count(), 2);
    }

    #[test]
    fn behold_or_pay_falls_back_to_mana() {
        let (mut game, p1, _p2) = setup_game();

        // No Kithkin available — must pay mana
        let source_id = add_creature_to_battlefield(&mut game, p1, "Source", SubType::Warrior);

        game.state.players.get_mut(&p1).unwrap()
            .mana_pool.add(Mana { white: 2, ..Mana::new() }, None, false);

        assert!(game.pay_costs(p1, source_id, &[Cost::behold_or_pay("Kithkin", "{2}")]));
        // Mana should have been spent
        assert_eq!(game.state.players.get(&p1).unwrap().mana_pool.total_count(), 0);
    }

    #[test]
    fn behold_or_pay_fails_without_either() {
        let (mut game, p1, _p2) = setup_game();

        // No Kithkin, no mana
        let source_id = add_creature_to_battlefield(&mut game, p1, "Source", SubType::Warrior);

        assert!(!game.pay_costs(p1, source_id, &[Cost::behold_or_pay("Kithkin", "{2}")]));
    }

    #[test]
    fn behold_works_with_changeling() {
        let (mut game, p1, _p2) = setup_game();

        // Changeling counts as every creature type
        let changeling_id = ObjectId::new();
        let mut card = CardData::new(changeling_id, p1, "Mothdust Changeling");
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![SubType::Shapeshifter];
        card.keywords = KeywordAbilities::CHANGELING;
        card.power = Some(1);
        card.toughness = Some(1);
        game.state.card_store.insert(card);
        let card_clone = game.state.card_store.get(changeling_id).unwrap().clone();
        let perm = Permanent::new(card_clone, p1);
        game.state.battlefield.add(perm);

        let source_id = add_creature_to_battlefield(&mut game, p1, "Source", SubType::Warrior);

        // Changeling should count as an Elf for behold
        assert!(game.pay_costs(p1, source_id, &[Cost::behold("Elf")]));
    }


    // Additional tests
    use uuid::Uuid;

    struct PassPlayer;
    impl crate::decision::PlayerDecisionMaker for PassPlayer {
        fn priority(&mut self, _: &crate::decision::GameView, actions: &[crate::decision::PlayerAction]) -> crate::decision::PlayerAction { actions[0].clone() }
        fn choose_targets(&mut self, _: &crate::decision::GameView, _: crate::constants::Outcome, req: &crate::decision::TargetRequirement) -> Vec<ObjectId> {
            // Pick the first legal target
            req.legal_targets.iter().take(1).copied().collect()
        }
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

    #[test]
    fn blight_opponents_puts_counter() {
        let p1 = PlayerId(Uuid::new_v4());
        let p2 = PlayerId(Uuid::new_v4());
        let config = GameConfig { players: vec![PlayerConfig { name: "P1".into(), deck: vec![] }, PlayerConfig { name: "P2".into(), deck: vec![] }], starting_life: 20 };
        let mut game = Game::new_two_player(config, vec![
            (p1, Box::new(PassPlayer)),
            (p2, Box::new(PassPlayer)),
        ]);

        // Give opponent a creature
        let opp_creature = ObjectId(Uuid::new_v4());
        let card = CardData {
            id: opp_creature, owner: p2, name: "Bear".into(),
            card_types: vec![crate::constants::CardType::Creature],
            power: Some(3), toughness: Some(3),
            ..Default::default()
        };
        let perm = crate::permanent::Permanent::new(card.clone(), p2);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card);

        // Blight opponents 1
        game.execute_effects(&[Effect::blight_opponents(1)], p1, &[], None, None);

        // Opponent's creature should have a -1/-1 counter
        let perm = game.state.battlefield.get(opp_creature).unwrap();
        assert_eq!(perm.counters.get(&CounterType::M1M1), 1, "should have -1/-1 counter");
        assert_eq!(perm.power(), 2, "power should be reduced by -1/-1");
    }

    #[test]
    fn gain_all_creature_types() {
        let p1 = PlayerId(Uuid::new_v4());
        let p2 = PlayerId(Uuid::new_v4());
        let config = GameConfig { players: vec![PlayerConfig { name: "P1".into(), deck: vec![] }, PlayerConfig { name: "P2".into(), deck: vec![] }], starting_life: 20 };
        let mut game = Game::new_two_player(config, vec![
            (p1, Box::new(PassPlayer)),
            (p2, Box::new(PassPlayer)),
        ]);

        let creature_id = ObjectId(Uuid::new_v4());
        let card = CardData {
            id: creature_id, owner: p1, name: "Type Gainer".into(),
            card_types: vec![crate::constants::CardType::Creature],
            subtypes: vec![crate::constants::SubType::Human],
            power: Some(2), toughness: Some(2),
            ..Default::default()
        };
        let perm = crate::permanent::Permanent::new(card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card);

        // Should not have Elf type initially
        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(!perm.has_subtype(&crate::constants::SubType::Elf));

        // Grant all creature types
        game.execute_effects(&[Effect::gain_all_creature_types()], p1, &[creature_id], None, None);

        // Should now have changeling (all creature types)
        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(perm.has_keyword(crate::constants::KeywordAbilities::CHANGELING), "should have changeling");
        assert!(perm.has_subtype(&crate::constants::SubType::Elf), "should have Elf as changeling");
        assert!(perm.has_subtype(&crate::constants::SubType::Goblin), "should have Goblin as changeling");
    }


    // Additional tests

    fn make_deck5(owner: PlayerId) -> Vec<CardData> {
        (0..20).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    fn setup_game_with_picker2(pick_index: usize) -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".into(), deck: make_deck5(p1) },
                PlayerConfig { name: "Bob".into(), deck: make_deck5(p2) },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(OptionPicker(pick_index))),
                (p2, Box::new(OptionPicker(0))),
            ],
        );
        (game, p1, p2)
    }

    fn make_creature_with_type(name: &str, owner: PlayerId, subtype: &str, power: i32, toughness: i32) -> CardData {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![SubType::by_description(subtype)];
        card.power = Some(power);
        card.toughness = Some(toughness);
        card
    }

    fn put_in_graveyard(game: &mut Game, card: CardData, owner: PlayerId) -> ObjectId {
        let id = card.id;
        game.state.card_store.insert(card);
        if let Some(player) = game.state.players.get_mut(&owner) {
            player.graveyard.add(id);
        }
        game.state.set_zone(id, crate::constants::Zone::Graveyard, Some(owner));
        id
    }

    #[test]
    fn returns_all_matching_creatures_from_graveyard() {
        let (mut game, p1, _p2) = setup_game_with_picker2(0);

        let elf1 = make_creature_with_type("Elf Warrior", p1, "Elemental", 2, 2);
        let elf1_id = put_in_graveyard(&mut game, elf1, p1);
        let elf2 = make_creature_with_type("Elf Shaman", p1, "Elemental", 1, 1);
        let elf2_id = put_in_graveyard(&mut game, elf2, p1);

        let effects = vec![Effect::choose_type_and_return_from_graveyard()];
        game.execute_effects(&effects, p1, &[], None, None);

        assert!(game.state.battlefield.get(elf1_id).is_some(), "First Elemental should be on battlefield");
        assert!(game.state.battlefield.get(elf2_id).is_some(), "Second Elemental should be on battlefield");
        if let Some(player) = game.state.players.get(&p1) {
            assert_eq!(player.graveyard.len(), 0, "Graveyard should be empty");
        }
    }

    #[test]
    fn ignores_non_matching_types() {
        let (mut game, p1, _p2) = setup_game_with_picker(0);

        let elemental = make_creature_with_type("Fire Elemental", p1, "Elemental", 3, 3);
        let elemental_id = put_in_graveyard(&mut game, elemental, p1);
        let goblin = make_creature_with_type("Goblin Piker", p1, "Goblin", 2, 1);
        let goblin_id = put_in_graveyard(&mut game, goblin, p1);

        let effects = vec![Effect::choose_type_and_return_from_graveyard()];
        game.execute_effects(&effects, p1, &[], None, None);

        assert!(game.state.battlefield.get(elemental_id).is_some(), "Elemental should be on battlefield");
        assert!(game.state.battlefield.get(goblin_id).is_none(), "Goblin should NOT be on battlefield");
        if let Some(player) = game.state.players.get(&p1) {
            assert_eq!(player.graveyard.len(), 1, "Goblin should remain in graveyard");
        }
    }

    #[test]
    fn ignores_non_creature_cards() {
        let (mut game, p1, _p2) = setup_game_with_picker(0);

        let creature = make_creature_with_type("Air Elemental", p1, "Elemental", 4, 4);
        let creature_id = put_in_graveyard(&mut game, creature, p1);

        let mut artifact = CardData::new(ObjectId::new(), p1, "Elemental Artifact");
        artifact.card_types = vec![CardType::Artifact];
        artifact.subtypes = vec![SubType::by_description("Elemental")];
        let artifact_id = put_in_graveyard(&mut game, artifact, p1);

        let effects = vec![Effect::choose_type_and_return_from_graveyard()];
        game.execute_effects(&effects, p1, &[], None, None);

        assert!(game.state.battlefield.get(creature_id).is_some(), "Creature should be on battlefield");
        assert!(game.state.battlefield.get(artifact_id).is_none(), "Non-creature should NOT be on battlefield");
    }

    #[test]
    fn empty_graveyard_does_nothing() {
        let (mut game, p1, _p2) = setup_game_with_picker(0);

        let bf_before = game.state.battlefield.iter().count();
        let effects = vec![Effect::choose_type_and_return_from_graveyard()];
        game.execute_effects(&effects, p1, &[], None, None);
        let bf_after = game.state.battlefield.iter().count();

        assert_eq!(bf_before, bf_after, "No new permanents should enter battlefield");
    }

    #[test]
    fn picks_different_type_index() {
        let (mut game, p1, _p2) = setup_game_with_picker(4);

        let goblin = make_creature_with_type("Goblin Lackey", p1, "Goblin", 1, 1);
        let goblin_id = put_in_graveyard(&mut game, goblin, p1);
        let elf = make_creature_with_type("Llanowar Elves", p1, "Elf", 1, 1);
        let elf_id = put_in_graveyard(&mut game, elf, p1);

        let effects = vec![Effect::choose_type_and_return_from_graveyard()];
        game.execute_effects(&effects, p1, &[], None, None);

        assert!(game.state.battlefield.get(goblin_id).is_some(), "Goblin should be on battlefield");
        assert!(game.state.battlefield.get(elf_id).is_none(), "Elf should NOT be on battlefield");
    }

    #[test]
    fn helper_constructor_returns_correct_variant() {
        match Effect::choose_type_and_return_from_graveyard() {
            Effect::ChooseTypeAndReturnFromGraveyard => {}
            other => panic!("Expected ChooseTypeAndReturnFromGraveyard, got {:?}", other),
        }
    }

    #[test]
    fn only_returns_controllers_creatures() {
        let (mut game, p1, p2) = setup_game_with_picker(0);

        let p1_elemental = make_creature_with_type("Fire Elemental", p1, "Elemental", 3, 3);
        let p1_id = put_in_graveyard(&mut game, p1_elemental, p1);
        let p2_elemental = make_creature_with_type("Water Elemental", p2, "Elemental", 2, 4);
        let p2_id = put_in_graveyard(&mut game, p2_elemental, p2);

        let effects = vec![Effect::choose_type_and_return_from_graveyard()];
        game.execute_effects(&effects, p1, &[], None, None);

        assert!(game.state.battlefield.get(p1_id).is_some(), "P1's creature should be on battlefield");
        assert!(game.state.battlefield.get(p2_id).is_none(), "P2's creature should NOT be on battlefield");
        if let Some(player) = game.state.players.get(&p2) {
            assert_eq!(player.graveyard.len(), 1, "P2's graveyard should be unchanged");
        }
    }


    // Additional tests

    fn make_basic_land(name: &str, owner: PlayerId) -> CardData {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Land];
        card
    }

    fn make_creature(name: &str, owner: PlayerId, power: i32, toughness: i32) -> CardData {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(power);
        card.toughness = Some(toughness);
        card
    }

    fn make_artifact(name: &str, owner: PlayerId) -> CardData {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Artifact];
        card
    }

    fn make_deck6(owner: PlayerId) -> Vec<CardData> {
        let mut deck = Vec::new();
        for _ in 0..20 { deck.push(make_basic_land("Forest", owner)); }
        for _ in 0..20 { deck.push(make_creature("Grizzly Bears", owner, 2, 2)); }
        deck
    }

    fn setup_game2() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck6(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck6(p2) },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );
        (game, p1, p2)
    }

    #[test]
    fn becomes_creature_adds_creature_type() {
        let (mut game, p1, _p2) = setup_game2();

        let artifact = make_artifact("Test Artifact", p1);
        let artifact_id = artifact.id;
        game.state.battlefield.add(Permanent::new(artifact, p1));

        assert!(!game.state.battlefield.get(artifact_id).unwrap().is_creature());
        assert!(game.state.battlefield.get(artifact_id).unwrap().is_artifact());

        game.execute_effects(
            &[Effect::becomes_creature(4, 4)],
            p1, &[], Some(artifact_id), None,
        );

        let perm = game.state.battlefield.get(artifact_id).unwrap();
        assert!(perm.is_creature(), "Artifact should now be a creature");
        assert!(perm.is_artifact(), "Artifact should still be an artifact");
    }

    #[test]
    fn becomes_creature_sets_base_pt() {
        let (mut game, p1, _p2) = setup_game2();

        let artifact = make_artifact("Test Artifact", p1);
        let artifact_id = artifact.id;
        game.state.battlefield.add(Permanent::new(artifact, p1));

        assert_eq!(game.state.battlefield.get(artifact_id).unwrap().power(), 0);
        assert_eq!(game.state.battlefield.get(artifact_id).unwrap().toughness(), 0);

        game.execute_effects(
            &[Effect::becomes_creature(4, 4)],
            p1, &[], Some(artifact_id), None,
        );

        let perm = game.state.battlefield.get(artifact_id).unwrap();
        assert_eq!(perm.power(), 4);
        assert_eq!(perm.toughness(), 4);
    }

    #[test]
    fn becomes_creature_cleared_at_cleanup() {
        let (mut game, p1, _p2) = setup_game();

        let artifact = make_artifact("Test Artifact", p1);
        let artifact_id = artifact.id;
        game.state.battlefield.add(Permanent::new(artifact, p1));

        game.execute_effects(
            &[Effect::becomes_creature(4, 4)],
            p1, &[], Some(artifact_id), None,
        );

        assert!(game.state.battlefield.get(artifact_id).unwrap().is_creature());

        game.turn_based_actions(PhaseStep::Cleanup, p1);

        let perm = game.state.battlefield.get(artifact_id).unwrap();
        assert!(!perm.is_creature(), "Creature type should be removed at cleanup");
        assert!(perm.is_artifact(), "Base artifact type should remain");
        assert_eq!(perm.power(), 0, "Power should revert");
        assert_eq!(perm.toughness(), 0, "Toughness should revert");
    }

    #[test]
    fn becomes_creature_interacts_with_counters() {
        let (mut game, p1, _p2) = setup_game();

        let artifact = make_artifact("Test Artifact", p1);
        let artifact_id = artifact.id;
        game.state.battlefield.add(Permanent::new(artifact, p1));

        game.execute_effects(
            &[Effect::becomes_creature(4, 4)],
            p1, &[], Some(artifact_id), None,
        );

        if let Some(perm) = game.state.battlefield.get_mut(artifact_id) {
            perm.add_counters(crate::counters::CounterType::P1P1, 2);
        }

        let perm = game.state.battlefield.get(artifact_id).unwrap();
        assert_eq!(perm.power(), 6, "Should be 4 base + 2 from counters");
        assert_eq!(perm.toughness(), 6, "Should be 4 base + 2 from counters");
    }

    #[test]
    fn becomes_creature_no_double_type() {
        let (mut game, p1, _p2) = setup_game();

        let mut card = CardData::new(ObjectId::new(), p1, "Artifact Creature");
        card.card_types = vec![CardType::Artifact, CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(3);
        let card_id = card.id;
        game.state.battlefield.add(Permanent::new(card, p1));

        game.execute_effects(
            &[Effect::becomes_creature(4, 4)],
            p1, &[], Some(card_id), None,
        );

        let perm = game.state.battlefield.get(card_id).unwrap();
        assert!(perm.is_creature());
        assert!(perm.added_card_types.is_empty(), "Should not add Creature type if already a creature");
        assert_eq!(perm.power(), 4, "P/T should still be overridden");
        assert_eq!(perm.toughness(), 4);
    }

    #[test]
    fn becomes_creature_with_continuous_boost() {
        let (mut game, p1, _p2) = setup_game();

        let artifact = make_artifact("Test Artifact", p1);
        let artifact_id = artifact.id;
        game.state.battlefield.add(Permanent::new(artifact, p1));

        game.execute_effects(
            &[Effect::becomes_creature(3, 3)],
            p1, &[], Some(artifact_id), None,
        );

        if let Some(perm) = game.state.battlefield.get_mut(artifact_id) {
            perm.continuous_boost_power = 2;
            perm.continuous_boost_toughness = 1;
        }
        
        let perm = game.state.battlefield.get(artifact_id).unwrap();
        assert_eq!(perm.power(), 5, "Should be 3 base + 2 boost");
        assert_eq!(perm.toughness(), 4, "Should be 3 base + 1 boost");
    }

    #[test]
    fn becomes_creature_has_card_type_check() {
        let (mut game, p1, _p2) = setup_game();

        let artifact = make_artifact("Test Artifact", p1);
        let artifact_id = artifact.id;
        game.state.battlefield.add(Permanent::new(artifact, p1));

        game.execute_effects(
            &[Effect::becomes_creature(4, 4)],
            p1, &[], Some(artifact_id), None,
        );

        let perm = game.state.battlefield.get(artifact_id).unwrap();
        assert!(perm.has_card_type(CardType::Creature));
        assert!(perm.has_card_type(CardType::Artifact));
        assert!(!perm.has_card_type(CardType::Enchantment));
    }

    #[test]
    fn helper_constructor() {
        match Effect::becomes_creature(4, 4) {
            Effect::BecomesCreature { power, toughness } => {
                assert_eq!(power, 4);
                assert_eq!(toughness, 4);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn base_power_eot_takes_priority_over_override() {
        let owner = PlayerId::new();
        let mut card = CardData::new(ObjectId::new(), owner, "Test");
        card.card_types = vec![CardType::Artifact];
        let mut perm = Permanent::new(card, owner);

        perm.base_power_override = Some(2);
        perm.base_toughness_override = Some(2);
        perm.base_power_eot = Some(5);
        perm.base_toughness_eot = Some(5);

        assert_eq!(perm.power(), 5, "EOT base should take priority over continuous override");
        assert_eq!(perm.toughness(), 5, "EOT base should take priority over continuous override");

}

    #[test]
    fn enters_with_counters_adds_counters_on_etb() {
        let (mut game, p1, _p2) = setup2();
        let mut card = CardData::new(ObjectId::new(), p1, "Brambleback Brute");
        card.card_types = vec![CardType::Creature];
        card.power = Some(4);
        card.toughness = Some(5);
        let id = card.id;
        card.abilities = vec![
            Ability::static_ability(id,
                "Brambleback Brute enters with two -1/-1 counters on it.",
                vec![StaticEffect::EntersWithCounters {
                    counter_type: "-1/-1".into(), count: 2,
                }]),
        ];
        for ability in &card.abilities {
            game.state.ability_store.add(ability.clone());
        }
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);
        game.check_enters_with_counters(id);

        let perm = game.state.battlefield.get(id).unwrap();
        assert_eq!(perm.counters.get(&CounterType::M1M1), 2);
        assert_eq!(perm.power(), 2);
        assert_eq!(perm.toughness(), 3);
    }

    #[test]
    fn enters_with_counters_p1p1() {
        let (mut game, p1, _p2) = setup2();
        let mut card = CardData::new(ObjectId::new(), p1, "Workhorse");
        card.card_types = vec![CardType::Creature];
        card.power = Some(0);
        card.toughness = Some(0);
        let id = card.id;
        card.abilities = vec![
            Ability::static_ability(id,
                "Workhorse enters with four +1/+1 counters on it.",
                vec![StaticEffect::enters_with_counters("+1/+1", 4)]),
        ];
        for ability in &card.abilities {
            game.state.ability_store.add(ability.clone());
        }
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);
        game.check_enters_with_counters(id);

        let perm = game.state.battlefield.get(id).unwrap();
        assert_eq!(perm.counters.get(&CounterType::P1P1), 4);
        assert_eq!(perm.power(), 4);
        assert_eq!(perm.toughness(), 4);
    }

    #[test]
    fn enters_with_counters_no_effect_without_ability() {
        let (mut game, p1, _p2) = setup2();
        let mut card = CardData::new(ObjectId::new(), p1, "Grizzly Bears");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        let id = card.id;
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);
        game.check_enters_with_counters(id);

        let perm = game.state.battlefield.get(id).unwrap();
        assert_eq!(perm.counters.get(&CounterType::M1M1), 0);
        assert_eq!(perm.counters.get(&CounterType::P1P1), 0);
        assert_eq!(perm.power(), 2);
        assert_eq!(perm.toughness(), 2);
    }

    #[test]
    fn enters_with_counters_multiple_counter_types() {
        let (mut game, p1, _p2) = setup2();
        let mut card = CardData::new(ObjectId::new(), p1, "Multi Counter");
        card.card_types = vec![CardType::Creature];
        card.power = Some(3);
        card.toughness = Some(3);
        let id = card.id;
        card.abilities = vec![
            Ability::static_ability(id, "Enters with counters.",
                vec![
                    StaticEffect::enters_with_counters("-1/-1", 2),
                    StaticEffect::enters_with_counters("+1/+1", 1),
                ]),
        ];
        for ability in &card.abilities {
            game.state.ability_store.add(ability.clone());
        }
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);
        game.check_enters_with_counters(id);

        let perm = game.state.battlefield.get(id).unwrap();
        assert_eq!(perm.counters.get(&CounterType::M1M1), 2);
        assert_eq!(perm.counters.get(&CounterType::P1P1), 1);
    }

    #[test]
    fn enters_with_counters_helper_constructor() {
        match StaticEffect::enters_with_counters("-1/-1", 3) {
            StaticEffect::EntersWithCounters { counter_type, count } => {
                assert_eq!(counter_type, "-1/-1");
                assert_eq!(count, 3);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn enters_with_counters_six_m1m1_on_big_creature() {
        let (mut game, p1, _p2) = setup2();
        let mut card = CardData::new(ObjectId::new(), p1, "Moonshadow");
        card.card_types = vec![CardType::Creature];
        card.power = Some(7);
        card.toughness = Some(7);
        let id = card.id;
        card.abilities = vec![
            Ability::static_ability(id,
                "Moonshadow enters with six -1/-1 counters on it.",
                vec![StaticEffect::enters_with_counters("-1/-1", 6)]),
        ];
        for ability in &card.abilities {
            game.state.ability_store.add(ability.clone());
        }
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);
        game.check_enters_with_counters(id);

        let perm = game.state.battlefield.get(id).unwrap();
        assert_eq!(perm.counters.get(&CounterType::M1M1), 6);
        assert_eq!(perm.power(), 1);
        assert_eq!(perm.toughness(), 1);
    }

    #[test]
    fn enters_with_counters_stun_counters() {
        let (mut game, p1, _p2) = setup2();
        let mut card = CardData::new(ObjectId::new(), p1, "Stunned Creature");
        card.card_types = vec![CardType::Creature];
        card.power = Some(3);
        card.toughness = Some(3);
        let id = card.id;
        card.abilities = vec![
            Ability::static_ability(id,
                "Stunned Creature enters with two stun counters on it.",
                vec![StaticEffect::enters_with_counters("stun", 2)]),
        ];
        for ability in &card.abilities {
            game.state.ability_store.add(ability.clone());
        }
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);
        game.check_enters_with_counters(id);

        let perm = game.state.battlefield.get(id).unwrap();
        assert_eq!(perm.counters.get(&CounterType::Stun), 2);
    }

    fn make_dfc_creature(owner: PlayerId) -> CardData {
        let id = ObjectId::new();
        let mut front = CardData::new(id, owner, "Front Face");
        front.card_types = vec![CardType::Creature];
        front.subtypes = vec![SubType::Elf];
        front.power = Some(3);
        front.toughness = Some(3);
        front.keywords = KeywordAbilities::FLYING;

        let mut back = CardData::new(id, owner, "Back Face");
        back.card_types = vec![CardType::Creature];
        back.subtypes = vec![SubType::Goblin];
        back.power = Some(5);
        back.toughness = Some(5);
        back.keywords = KeywordAbilities::DEATHTOUCH;

        front.back_face = Some(Box::new(back));
        front
    }

    #[test]
    fn transform_self_swaps_characteristics() {
        let (mut game, p1, _p2) = setup_game2();

        let card = make_dfc_creature(p1);
        let card_id = card.id;
        game.state.battlefield.add(Permanent::new(card, p1));

        let perm = game.state.battlefield.get(card_id).unwrap();
        assert_eq!(perm.name(), "Front Face");
        assert_eq!(perm.power(), 3);
        assert_eq!(perm.toughness(), 3);
        assert!(perm.has_flying());
        assert!(!perm.has_deathtouch());
        assert!(perm.has_subtype(&SubType::Elf));
        assert!(!perm.transformed);

        game.execute_effects(
            &[Effect::transform_self()],
            p1, &[], Some(card_id), None,
        );

        let perm = game.state.battlefield.get(card_id).unwrap();
        assert_eq!(perm.name(), "Back Face");
        assert_eq!(perm.power(), 5);
        assert_eq!(perm.toughness(), 5);
        assert!(perm.has_deathtouch());
        assert!(!perm.has_flying());
        assert!(perm.has_subtype(&SubType::Goblin));
        assert!(perm.transformed);
    }

    #[test]
    fn transform_back_restores_front_face() {
        let (mut game, p1, _p2) = setup_game2();

        let card = make_dfc_creature(p1);
        let card_id = card.id;
        game.state.battlefield.add(Permanent::new(card, p1));

        game.execute_effects(&[Effect::transform_self()], p1, &[], Some(card_id), None);
        assert_eq!(game.state.battlefield.get(card_id).unwrap().name(), "Back Face");

        game.execute_effects(&[Effect::transform_self()], p1, &[], Some(card_id), None);

        let perm = game.state.battlefield.get(card_id).unwrap();
        assert_eq!(perm.name(), "Front Face");
        assert_eq!(perm.power(), 3);
        assert_eq!(perm.toughness(), 3);
        assert!(perm.has_flying());
        assert!(!perm.transformed);
    }

    #[test]
    fn transform_preserves_counters() {
        let (mut game, p1, _p2) = setup_game2();

        let card = make_dfc_creature(p1);
        let card_id = card.id;
        game.state.battlefield.add(Permanent::new(card, p1));
        game.state.battlefield.get_mut(card_id).unwrap().add_counters(CounterType::P1P1, 2);

        assert_eq!(game.state.battlefield.get(card_id).unwrap().power(), 5);

        game.execute_effects(&[Effect::transform_self()], p1, &[], Some(card_id), None);

        let perm = game.state.battlefield.get(card_id).unwrap();
        assert_eq!(perm.counters.get(&CounterType::P1P1), 2);
        assert_eq!(perm.power(), 7);
        assert_eq!(perm.toughness(), 7);
    }

    #[test]
    fn transform_preserves_tapped_state() {
        let (mut game, p1, _p2) = setup_game2();

        let card = make_dfc_creature(p1);
        let card_id = card.id;
        game.state.battlefield.add(Permanent::new(card, p1));
        game.state.battlefield.get_mut(card_id).unwrap().tap();

        game.execute_effects(&[Effect::transform_self()], p1, &[], Some(card_id), None);

        assert!(game.state.battlefield.get(card_id).unwrap().tapped);
    }

    #[test]
    fn transform_no_back_face_does_nothing() {
        let (mut game, p1, _p2) = setup_game2();

        let mut card = CardData::new(ObjectId::new(), p1, "Regular Creature");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        let card_id = card.id;
        game.state.battlefield.add(Permanent::new(card, p1));

        game.execute_effects(&[Effect::transform_self()], p1, &[], Some(card_id), None);

        let perm = game.state.battlefield.get(card_id).unwrap();
        assert_eq!(perm.name(), "Regular Creature");
        assert!(!perm.transformed);
    }

    #[test]
    fn transform_preserves_object_id() {
        let (mut game, p1, _p2) = setup_game2();

        let card = make_dfc_creature(p1);
        let card_id = card.id;
        game.state.battlefield.add(Permanent::new(card, p1));

        game.execute_effects(&[Effect::transform_self()], p1, &[], Some(card_id), None);

        assert!(game.state.battlefield.get(card_id).is_some());
        assert_eq!(game.state.battlefield.get(card_id).unwrap().id(), card_id);
    }

    #[test]
    fn transform_back_face_abilities_registered() {
        let (mut game, p1, _p2) = setup_game2();

        let id = ObjectId::new();
        let mut front = CardData::new(id, p1, "Front");
        front.card_types = vec![CardType::Creature];
        front.power = Some(2);
        front.toughness = Some(2);
        front.abilities = vec![
            Ability::static_ability(id, "Flying.",
                vec![StaticEffect::GrantKeyword { filter: "self".into(), keyword: "flying".into() }]),
        ];

        let mut back = CardData::new(id, p1, "Back");
        back.card_types = vec![CardType::Creature];
        back.power = Some(4);
        back.toughness = Some(4);
        back.abilities = vec![
            Ability::static_ability(id, "Deathtouch.",
                vec![StaticEffect::GrantKeyword { filter: "self".into(), keyword: "deathtouch".into() }]),
        ];

        front.back_face = Some(Box::new(back));
        for ability in &front.abilities {
            game.state.ability_store.add(ability.clone());
        }
        game.state.battlefield.add(Permanent::new(front, p1));

        let abilities_before = game.state.ability_store.for_source(id);
        assert_eq!(abilities_before.len(), 1);
        assert!(abilities_before[0].rules_text.contains("Flying"));

        game.execute_effects(&[Effect::transform_self()], p1, &[], Some(id), None);

        let abilities_after = game.state.ability_store.for_source(id);
        assert_eq!(abilities_after.len(), 1);
        assert!(abilities_after[0].rules_text.contains("Deathtouch"));
    }

    #[test]
    fn transform_helper_constructor() {
        let effect = Effect::transform_self();
        match effect {
            Effect::TransformSelf => {}
            _ => panic!("Expected TransformSelf"),
        }
    }

    #[test]
    fn permanent_can_transform_check() {
        let owner = PlayerId::new();
        let card = make_dfc_creature(owner);
        let perm = Permanent::new(card, owner);
        assert!(perm.can_transform());

        let mut card2 = CardData::new(ObjectId::new(), owner, "No DFC");
        card2.card_types = vec![CardType::Creature];
        card2.power = Some(1);
        card2.toughness = Some(1);
        let perm2 = Permanent::new(card2, owner);
        assert!(!perm2.can_transform());
    }
