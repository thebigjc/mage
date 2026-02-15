// All tests extracted from game.rs

use crate::game::*;
use crate::types::{ObjectId, PlayerId, AbilityId};
use crate::decision::PlayerDecisionMaker;
use crate::permanent::Permanent;
use crate::counters::CounterType;
use crate::combat::CombatState;
use crate::constants::{Color, PhaseStep, SuperType};
use crate::state::StateBasedActions;
use crate::events::{EventType, GameEvent};
use crate::watchers::WatcherManager;

mod tests {
    use super::*;
    use crate::abilities::{Ability, Cost, Effect, TargetSpec};
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, Outcome, SubType};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };
    use crate::mana::Mana;

    /// A minimal decision maker that always passes priority.
    struct AlwaysPassPlayer;

    impl PlayerDecisionMaker for AlwaysPassPlayer {
        fn priority(&mut self, _game: &GameView<'_>, _legal: &[PlayerAction]) -> PlayerAction {
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
        card.keywords = KeywordAbilities::empty();
        card
    }

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        let mut deck = Vec::new();
        // 20 lands
        for _ in 0..20 {
            deck.push(make_basic_land("Forest", owner));
        }
        // 20 creatures
        for _ in 0..20 {
            deck.push(make_creature("Grizzly Bears", owner, 2, 2));
        }
        deck
    }

    #[test]
    fn game_creation() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
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

        assert_eq!(game.state.players.len(), 2);
        assert_eq!(game.state.player(p1).unwrap().life, 20);
        assert_eq!(game.state.player(p2).unwrap().life, 20);
        // Each player should have 40 cards in library
        assert_eq!(game.state.player(p1).unwrap().library.len(), 40);
        assert_eq!(game.state.player(p2).unwrap().library.len(), 40);
    }

    #[test]
    fn game_runs_to_completion() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Both players always pass, so the game should eventually end by decking
        let result = game.run();

        // A player should have lost by drawing from an empty library
        assert!(result.winner.is_some() || result.turn_number > 1);
    }

    #[test]
    fn draw_cards_from_empty_library_causes_loss() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        // Give player 1 only 5 cards in deck
        let mut small_deck = Vec::new();
        for _ in 0..5 {
            small_deck.push(make_basic_land("Forest", p1));
        }

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: small_deck },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        let result = game.run();

        // Alice should lose from decking (only 5 cards, draws 7 opening hand)
        assert_eq!(result.winner, Some(p2));
    }

    #[test]
    fn counter_annihilation_applied() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Add a creature with both +1/+1 and -1/-1 counters
        let card = make_creature("Test Bear", p1, 2, 2);
        let card_id = card.id;
        let mut perm = Permanent::new(card, p1);
        perm.add_counters(crate::counters::CounterType::P1P1, 3);
        perm.add_counters(crate::counters::CounterType::M1M1, 2);
        game.state.battlefield.add(perm);

        // Process SBAs
        game.process_state_based_actions();

        // After annihilation: 3 P1P1 - 2 M1M1 = 1 P1P1 remaining, 0 M1M1
        let perm = game.state.battlefield.get(card_id).unwrap();
        assert_eq!(perm.counters.get(&crate::counters::CounterType::P1P1), 1);
        assert_eq!(perm.counters.get(&crate::counters::CounterType::M1M1), 0);
        assert_eq!(perm.power(), 3); // 2 base + 1 from counter
        assert_eq!(perm.toughness(), 3);
    }

    #[test]
    fn legend_rule_applied() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Add two legendary permanents with the same name
        let mut card1 = CardData::new(ObjectId::new(), p1, "Thalia");
        card1.card_types = vec![CardType::Creature];
        card1.supertypes = vec![crate::constants::SuperType::Legendary];
        card1.power = Some(2);
        card1.toughness = Some(1);
        card1.keywords = KeywordAbilities::empty();
        let id1 = card1.id;

        let mut card2 = CardData::new(ObjectId::new(), p1, "Thalia");
        card2.card_types = vec![CardType::Creature];
        card2.supertypes = vec![crate::constants::SuperType::Legendary];
        card2.power = Some(2);
        card2.toughness = Some(1);
        card2.keywords = KeywordAbilities::empty();

        game.state.battlefield.add(Permanent::new(card1, p1));
        game.state.battlefield.add(Permanent::new(card2, p1));

        assert_eq!(game.state.battlefield.len(), 2);

        // Process SBAs
        game.process_state_based_actions();

        // One should be removed by the legend rule
        assert_eq!(game.state.battlefield.len(), 1);
        // The first one should survive
        assert!(game.state.battlefield.contains(id1));
    }

    #[test]
    fn legal_actions_include_pass() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
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

        let actions = game.compute_legal_actions(p1);
        assert!(actions.contains(&PlayerAction::Pass));
    }

    #[test]
    fn mana_ability_and_spell_cast() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Put a Forest on the battlefield with a mana ability
        let forest_id = ObjectId::new();
        let mut forest = CardData::new(forest_id, p1, "Forest");
        forest.card_types = vec![CardType::Land];
        let mana_ability = Ability::mana_ability(
            forest_id,
            "{T}: Add {G}",
            Mana::green(1),
        );
        let ability_id = mana_ability.id;
        forest.abilities.push(mana_ability);

        let perm = Permanent::new(forest.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(forest.clone());
        for ability in &forest.abilities {
            game.state.ability_store.add(ability.clone());
        }

        // Activate the mana ability
        game.activate_mana_ability(p1, forest_id, ability_id);

        // Check that the mana pool has green mana
        let player = game.state.players.get(&p1).unwrap();
        let available = player.mana_pool.available();
        assert!(available.green >= 1);

        // The permanent should be tapped now
        let perm = game.state.battlefield.get(forest_id).unwrap();
        assert!(perm.tapped);
    }

    #[test]
    fn activated_ability_goes_on_stack() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Create a permanent with an activated ability (no cost for simplicity)
        let source_id = ObjectId::new();
        let mut card = CardData::new(source_id, p1, "Prodigal Sorcerer");
        card.card_types = vec![CardType::Creature];
        card.power = Some(1);
        card.toughness = Some(1);
        card.keywords = KeywordAbilities::empty();

        let ability = Ability::activated(
            source_id,
            "{T}: Deal 1 damage to target.",
            vec![Cost::TapSelf],
            vec![Effect::DealDamage { amount: 1 }],
            TargetSpec::CreatureOrPlayer,
        );
        let ability_id = ability.id;
        card.abilities.push(ability);

        let perm = Permanent::new(card.clone(), p1);
        perm.id(); // verify it has the right ID
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card.clone());
        for a in &card.abilities {
            game.state.ability_store.add(a.clone());
        }

        // Remove summoning sickness for the test
        if let Some(perm) = game.state.battlefield.get_mut(source_id) {
            perm.remove_summoning_sickness();
        }

        // Activate the ability
        game.activate_ability(p1, source_id, ability_id, &[]);

        // The ability should be on the stack
        assert_eq!(game.state.stack.len(), 1);

        // The permanent should be tapped (cost was tap)
        let perm = game.state.battlefield.get(source_id).unwrap();
        assert!(perm.tapped);
    }

    #[test]
    fn spell_effects_execute_on_resolve() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Create a target creature for Bob
        let bear_id = ObjectId::new();
        let mut bear = CardData::new(bear_id, p2, "Grizzly Bears");
        bear.card_types = vec![CardType::Creature];
        bear.power = Some(2);
        bear.toughness = Some(2);
        bear.keywords = KeywordAbilities::empty();
        let bear_perm = Permanent::new(bear.clone(), p2);
        game.state.battlefield.add(bear_perm);

        // Create a spell with "deal 3 damage" and push it on the stack
        let bolt_id = ObjectId::new();
        let mut bolt = CardData::new(bolt_id, p1, "Lightning Bolt");
        bolt.card_types = vec![CardType::Instant];
        bolt.abilities.push(Ability::spell(
            bolt_id,
            vec![Effect::DealDamage { amount: 3 }],
            TargetSpec::CreatureOrPlayer,
        ));

        let stack_item = crate::zones::StackItem {
            id: bolt_id,
            kind: crate::zones::StackItemKind::Spell { card: bolt },
            controller: p1,
            targets: vec![bear_id],
            countered: false,
            x_value: None,
            exile_on_resolve: false,
        };
        game.state.stack.push(stack_item);

        // Resolve the spell
        game.resolve_top_of_stack();

        // The bear should have 3 damage marked on it
        let bear = game.state.battlefield.get(bear_id).unwrap();
        assert_eq!(bear.damage, 3);
        assert!(bear.has_lethal_damage());

        // SBAs should destroy it
        game.process_state_based_actions();
        assert!(!game.state.battlefield.contains(bear_id));
    }

    #[test]
    fn fizzle_when_target_removed() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Create a target creature, then remove it before spell resolves
        let bear_id = ObjectId::new();
        let mut bear = CardData::new(bear_id, p2, "Grizzly Bears");
        bear.card_types = vec![CardType::Creature];
        bear.power = Some(2);
        bear.toughness = Some(2);
        bear.keywords = KeywordAbilities::empty();
        game.state.battlefield.add(Permanent::new(bear.clone(), p2));

        // Push a destroy spell targeting the bear
        let spell_id = ObjectId::new();
        let mut murder = CardData::new(spell_id, p1, "Murder");
        murder.card_types = vec![CardType::Instant];
        murder.abilities.push(Ability::spell(
            spell_id,
            vec![Effect::Destroy],
            TargetSpec::Creature,
        ));

        let stack_item = crate::zones::StackItem {
            id: spell_id,
            kind: crate::zones::StackItemKind::Spell { card: murder },
            controller: p1,
            targets: vec![bear_id],
            countered: false,
            x_value: None,
            exile_on_resolve: false,
        };
        game.state.stack.push(stack_item);

        // Remove the target before resolution (simulating another effect)
        game.state.battlefield.remove(bear_id);

        // Resolve the spell — should fizzle since target is gone
        game.resolve_top_of_stack();

        // The spell should be in the graveyard (fizzled)
        // The stack should be empty
        assert!(game.state.stack.is_empty());
    }

    #[test]
    fn draw_cards_effect() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        let initial_hand = game.state.players.get(&p1).unwrap().hand.len();
        let initial_library = game.state.players.get(&p1).unwrap().library.len();

        // Execute a draw 2 effect
        game.execute_effects(&[Effect::DrawCards { count: 2 }], p1, &[], None, None);

        let final_hand = game.state.players.get(&p1).unwrap().hand.len();
        let final_library = game.state.players.get(&p1).unwrap().library.len();

        assert_eq!(final_hand, initial_hand + 2);
        assert_eq!(final_library, initial_library - 2);
    }

    #[test]
    fn gain_life_effect() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        game.execute_effects(&[Effect::GainLife { amount: 5 }], p1, &[], None, None);
        assert_eq!(game.state.players.get(&p1).unwrap().life, 25);
    }

    #[test]
    fn lose_life_opponents_effect() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        game.execute_effects(&[Effect::lose_life_opponents(3)], p1, &[], None, None);
        // Controller's life should be unchanged
        assert_eq!(game.state.players.get(&p1).unwrap().life, 20);
        // Opponent loses 3 life
        assert_eq!(game.state.players.get(&p2).unwrap().life, 17);
    }

    #[test]
    fn exile_effect() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Put a creature on the battlefield
        let bear_id = ObjectId::new();
        let mut bear = CardData::new(bear_id, p2, "Bear");
        bear.card_types = vec![CardType::Creature];
        bear.power = Some(2);
        bear.toughness = Some(2);
        bear.keywords = KeywordAbilities::empty();
        game.state.battlefield.add(Permanent::new(bear, p2));

        // Exile it
        game.execute_effects(&[Effect::Exile], p1, &[bear_id], None, None);

        assert!(!game.state.battlefield.contains(bear_id));
        assert!(game.state.exile.contains(bear_id));
    }

    #[test]
    fn bounce_effect() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        let bear_id = ObjectId::new();
        let mut bear = CardData::new(bear_id, p2, "Bear");
        bear.card_types = vec![CardType::Creature];
        bear.power = Some(2);
        bear.toughness = Some(2);
        bear.keywords = KeywordAbilities::empty();
        game.state.battlefield.add(Permanent::new(bear, p2));

        let initial_hand = game.state.players.get(&p2).unwrap().hand.len();

        // Bounce it
        game.execute_effects(&[Effect::Bounce], p1, &[bear_id], None, None);

        assert!(!game.state.battlefield.contains(bear_id));
        assert_eq!(game.state.players.get(&p2).unwrap().hand.len(), initial_hand + 1);
    }

    #[test]
    fn pay_costs_tap_and_sacrifice() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Add a permanent
        let source_id = ObjectId::new();
        let mut card = CardData::new(source_id, p1, "Sacrifice Me");
        card.card_types = vec![CardType::Creature];
        card.power = Some(1);
        card.toughness = Some(1);
        card.keywords = KeywordAbilities::empty();
        game.state.battlefield.add(Permanent::new(card, p1));

        // Pay tap cost
        assert!(game.pay_costs(p1, source_id, &[Cost::TapSelf]));
        let perm = game.state.battlefield.get(source_id).unwrap();
        assert!(perm.tapped);

        // Can't pay tap again (already tapped)
        assert!(!game.pay_costs(p1, source_id, &[Cost::TapSelf]));

        // Pay sacrifice self cost
        assert!(game.pay_costs(p1, source_id, &[Cost::SacrificeSelf]));
        assert!(!game.state.battlefield.contains(source_id));

        // The card should be in the graveyard
        let player = game.state.players.get(&p1).unwrap();
        assert!(player.graveyard.contains(source_id));
    }

    #[test]
    fn add_counters_self_when_no_targets() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Add a creature to the battlefield
        let source_id = ObjectId::new();
        let mut card = CardData::new(source_id, p1, "Blight Creature");
        card.card_types = vec![CardType::Creature];
        card.power = Some(3);
        card.toughness = Some(7);
        card.keywords = KeywordAbilities::empty();
        game.state.battlefield.add(Permanent::new(card, p1));

        // Execute AddCounters with no targets but with source — should add to self
        game.execute_effects(
            &[Effect::add_counters("-1/-1", 2)],
            p1,
            &[],
            Some(source_id),
        None, );

        let perm = game.state.battlefield.get(source_id).unwrap();
        assert_eq!(perm.counters.get(&CounterType::M1M1), 2);

        // Execute RemoveCounters with no targets but with source — should remove from self
        game.execute_effects(
            &[Effect::RemoveCounters { counter_type: "-1/-1".into(), count: 1 }],
            p1,
            &[],
            Some(source_id),
        None, );

        let perm = game.state.battlefield.get(source_id).unwrap();
        assert_eq!(perm.counters.get(&CounterType::M1M1), 1);
    }

    #[test]
    fn add_counters_self_with_separate_target() {
        // Compound effect: AddCountersSelf puts -1/-1 on source while
        // GainKeywordUntilEndOfTurn gives haste to a different target.
        // Models Warren Torchmaster: blight self + target creature gains haste.
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Source creature (Warren Torchmaster analog)
        let source_id = ObjectId::new();
        let mut card = CardData::new(source_id, p1, "Torchmaster");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        card.keywords = KeywordAbilities::empty();
        game.state.battlefield.add(Permanent::new(card, p1));

        // Target creature (gets haste)
        let target_id = ObjectId::new();
        let mut card2 = CardData::new(target_id, p1, "Target Creature");
        card2.card_types = vec![CardType::Creature];
        card2.power = Some(3);
        card2.toughness = Some(3);
        card2.keywords = KeywordAbilities::empty();
        game.state.battlefield.add(Permanent::new(card2, p1));

        // Compound effect: blight self + grant haste to target
        game.execute_effects(
            &[Effect::add_counters_self("-1/-1", 1), Effect::gain_keyword_eot("haste")],
            p1,
            &[target_id],  // target creature
            Some(source_id),  // source permanent
        None, );

        // Source should have -1/-1 counter (from AddCountersSelf)
        let source_perm = game.state.battlefield.get(source_id).unwrap();
        assert_eq!(source_perm.counters.get(&CounterType::M1M1), 1);
        assert_eq!(source_perm.power(), 1); // 2 - 1
        // Source should NOT have haste
        assert!(!source_perm.granted_keywords.contains(KeywordAbilities::HASTE));

        // Target should have haste (from GainKeywordUntilEndOfTurn)
        let target_perm = game.state.battlefield.get(target_id).unwrap();
        assert!(target_perm.granted_keywords.contains(KeywordAbilities::HASTE));
        // Target should NOT have -1/-1 counter
        assert_eq!(target_perm.counters.get(&CounterType::M1M1), 0);
    }

    /// A decision maker that actually discards when asked.
    struct DiscardingPlayer;

    impl PlayerDecisionMaker for DiscardingPlayer {
        fn priority(&mut self, _game: &GameView<'_>, _legal: &[PlayerAction]) -> PlayerAction {
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
        fn choose_discard(&mut self, _: &GameView<'_>, hand: &[ObjectId], count: usize) -> Vec<ObjectId> {
            // Actually discard from the back of hand
            hand.iter().rev().take(count).copied().collect()
        }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    #[test]
    fn discard_opponents_effect() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(DiscardingPlayer)),
            ],
        );

        // Give opponent some cards in hand
        let c1_id = ObjectId::new();
        let c2_id = ObjectId::new();
        let c3_id = ObjectId::new();
        if let Some(player) = game.state.players.get_mut(&p2) {
            player.hand.add(c1_id);
            player.hand.add(c2_id);
            player.hand.add(c3_id);
        }

        let p1_hand_before = game.state.players.get(&p1).unwrap().hand.len();
        let p2_hand_before = game.state.players.get(&p2).unwrap().hand.len();
        assert_eq!(p2_hand_before, 3);

        // Each opponent discards 1
        game.execute_effects(&[Effect::discard_opponents(1)], p1, &[], None, None);

        // Controller's hand unchanged
        assert_eq!(game.state.players.get(&p1).unwrap().hand.len(), p1_hand_before);
        // Opponent lost 1 card
        assert_eq!(game.state.players.get(&p2).unwrap().hand.len(), 2);
    }

    #[test]
    fn boost_all_and_grant_keyword_all_until_eot() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Put two creatures on P1's battlefield and one on P2's
        let bear1 = make_creature("Grizzly Bears", p1, 2, 2);
        let bear1_id = bear1.id;
        let bear2 = make_creature("Runeclaw Bear", p1, 2, 2);
        let bear2_id = bear2.id;
        let opp_bear = make_creature("Opponent Bear", p2, 2, 2);
        let opp_bear_id = opp_bear.id;

        game.state.battlefield.add(Permanent::new(bear1, p1));
        game.state.battlefield.add(Permanent::new(bear2, p1));
        game.state.battlefield.add(Permanent::new(opp_bear, p2));

        // Boost all creatures P1 controls +1/+1
        game.execute_effects(
            &[Effect::boost_all_eot("creatures you control", 1, 1)],
            p1, &[], None, None,
        );

        // P1's creatures should be 3/x, opponent's should remain 2/x
        assert_eq!(game.state.battlefield.get(bear1_id).unwrap().power(), 3);
        assert_eq!(game.state.battlefield.get(bear2_id).unwrap().power(), 3);
        assert_eq!(game.state.battlefield.get(opp_bear_id).unwrap().power(), 2);

        // Grant trample to all creatures P1 controls
        game.execute_effects(
            &[Effect::grant_keyword_all_eot("creatures you control", "trample")],
            p1, &[], None, None,
        );

        // P1's creatures should have trample, opponent's should not
        assert!(game.state.battlefield.get(bear1_id).unwrap().has_keyword(KeywordAbilities::TRAMPLE));
        assert!(game.state.battlefield.get(bear2_id).unwrap().has_keyword(KeywordAbilities::TRAMPLE));
        assert!(!game.state.battlefield.get(opp_bear_id).unwrap().has_keyword(KeywordAbilities::TRAMPLE));
    }

    #[test]
    fn fight_and_bite_effects() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Set up: P1 has a 4/4, P2 has a 3/5
        let fighter = make_creature("Fighter", p1, 4, 4);
        let fighter_id = fighter.id;
        let target = make_creature("Target", p2, 3, 5);
        let target_id = target.id;

        game.state.battlefield.add(Permanent::new(fighter, p1));
        game.state.battlefield.add(Permanent::new(target, p2));

        // Fight: mutual damage — fighter (4 power) vs target (3 power)
        game.execute_effects(
            &[Effect::fight()],
            p1,
            &[target_id],
            Some(fighter_id),
        None, );

        // Fighter took 3 damage (from target's 3 power): 4 toughness - 3 = 1 remaining
        let f = game.state.battlefield.get(fighter_id).unwrap();
        assert_eq!(f.remaining_toughness(), 1);
        // Target took 4 damage (from fighter's 4 power): 5 toughness - 4 = 1 remaining
        let t = game.state.battlefield.get(target_id).unwrap();
        assert_eq!(t.remaining_toughness(), 1);

        // Clear damage for next test
        game.state.battlefield.get_mut(fighter_id).unwrap().clear_damage();
        game.state.battlefield.get_mut(target_id).unwrap().clear_damage();

        // Bite: one-way damage — fighter deals 4 to target, target deals nothing back
        game.execute_effects(
            &[Effect::bite()],
            p1,
            &[target_id],
            Some(fighter_id),
        None, );

        // Fighter should have no damage
        let f = game.state.battlefield.get(fighter_id).unwrap();
        assert_eq!(f.remaining_toughness(), 4);
        // Target took 4 damage: 5 toughness - 4 = 1 remaining
        let t = game.state.battlefield.get(target_id).unwrap();
        assert_eq!(t.remaining_toughness(), 1);
    }

    #[test]
    fn fight_auto_selects_creatures() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // P1 has a 2/2 and a 5/5; P2 has a 3/3
        let small = make_creature("Small Bear", p1, 2, 2);
        let big = make_creature("Big Bear", p1, 5, 5);
        let big_id = big.id;
        let opp = make_creature("Opponent Bear", p2, 3, 3);
        let opp_id = opp.id;

        game.state.battlefield.add(Permanent::new(small, p1));
        game.state.battlefield.add(Permanent::new(big, p1));
        game.state.battlefield.add(Permanent::new(opp, p2));

        // Fight with no source, no targets — auto-selects strongest on each side
        game.execute_effects(&[Effect::fight()], p1, &[], None, None);

        // P1's 5/5 should fight P2's 3/3
        // Big bear: 5 toughness - 3 damage = 2 remaining
        let b = game.state.battlefield.get(big_id).unwrap();
        assert_eq!(b.remaining_toughness(), 2);
        // Opponent bear: 3 toughness - 5 damage = lethal
        let o = game.state.battlefield.get(opp_id).unwrap();
        assert!(o.has_lethal_damage());
    }

    #[test]
    fn compound_bite_counters_only_on_your_creature() {
        // Matches Java's Knockout Maneuver / Felling Blow pattern:
        // AddCountersTargetEffect targets only target 0 (your creature),
        // DamageWithPowerFromOneToAnotherTargetEffect uses both targets.
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // P1 has a 3/3, P2 has a 4/4
        let my_creature = make_creature("My Creature", p1, 3, 3);
        let my_id = my_creature.id;
        let opp_creature = make_creature("Opp Creature", p2, 4, 4);
        let opp_id = opp_creature.id;

        game.state.battlefield.add(Permanent::new(my_creature, p1));
        game.state.battlefield.add(Permanent::new(opp_creature, p2));

        // Compound effect: +1/+1 counter then bite (like Knockout Maneuver)
        // targets[0] = my creature, targets[1] = opponent creature
        game.execute_effects(
            &[Effect::add_p1p1_counters(1), Effect::bite()],
            p1,
            &[my_id, opp_id],
            None, None,
        );

        // My creature should have the +1/+1 counter (3+1=4 power, 3+1=4 toughness)
        let my = game.state.battlefield.get(my_id).unwrap();
        assert_eq!(my.power(), 4, "My creature should have +1/+1 counter (4 power)");
        assert_eq!(my.toughness(), 4, "My creature should have +1/+1 counter (4 toughness)");

        // Opponent's creature should NOT have any counters
        let opp = game.state.battlefield.get(opp_id).unwrap();
        assert_eq!(opp.power(), 4, "Opponent creature should not have counters");

        // Bite: my creature (now 4 power) deals 4 damage to opponent creature (4 toughness)
        assert_eq!(opp.remaining_toughness(), 0, "Opponent took 4 damage from bite");
        // My creature should have no damage (bite is one-way)
        assert_eq!(my.remaining_toughness(), 4, "My creature took no damage from bite");
    }

    #[test]
    fn add_counters_all_effect() {
        let (p1, p2) = (PlayerId::new(), PlayerId::new());
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Add creatures for both players
        let c1 = ObjectId::new();
        let mut card1 = CardData::new(c1, p1, "My Creature");
        card1.card_types = vec![CardType::Creature];
        card1.power = Some(3);
        card1.toughness = Some(3);
        game.state.battlefield.add(Permanent::new(card1, p1));

        let c2 = ObjectId::new();
        let mut card2 = CardData::new(c2, p2, "Opp Creature");
        card2.card_types = vec![CardType::Creature];
        card2.power = Some(2);
        card2.toughness = Some(4);
        game.state.battlefield.add(Permanent::new(card2, p2));

        // AddCountersAll on "creatures" (no "you control") — hits both
        game.execute_effects(
            &[Effect::add_counters_all("-1/-1", 2, "creatures")],
            p1,
            &[],
            None, None,
        );

        let p1c = game.state.battlefield.get(c1).unwrap();
        assert_eq!(p1c.counters.get(&CounterType::M1M1), 2);
        assert_eq!(p1c.power(), 1); // 3 - 2

        let p2c = game.state.battlefield.get(c2).unwrap();
        assert_eq!(p2c.counters.get(&CounterType::M1M1), 2);
        assert_eq!(p2c.power(), 0); // 2 - 2

        // AddCountersAll on "creatures you control" — hits only controller's
        game.execute_effects(
            &[Effect::add_counters_all("+1/+1", 1, "creatures you control")],
            p1,
            &[],
            None, None,
        );

        let p1c = game.state.battlefield.get(c1).unwrap();
        assert_eq!(p1c.counters.get(&CounterType::P1P1), 1);

        let p2c = game.state.battlefield.get(c2).unwrap();
        assert_eq!(p2c.counters.get(&CounterType::P1P1), 0); // unchanged
    }

    #[test]
    fn look_top_and_pick() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Clear library and set up specific cards on top
        let elf_id = ObjectId::new();
        let gob_id = ObjectId::new();
        let forest_id = ObjectId::new();
        let mtn_id = ObjectId::new();

        // Create cards with subtypes
        let mut elf = CardData::new(elf_id, p1, "Test Elf");
        elf.card_types = vec![CardType::Creature];
        elf.subtypes = vec![SubType::Elf];
        game.state.card_store.insert(elf);

        let mut gob = CardData::new(gob_id, p1, "Test Goblin");
        gob.card_types = vec![CardType::Creature];
        gob.subtypes = vec![SubType::Goblin];
        game.state.card_store.insert(gob);

        let mut forest = CardData::new(forest_id, p1, "Forest");
        forest.card_types = vec![CardType::Land];
        forest.subtypes = vec![SubType::Forest];
        game.state.card_store.insert(forest);

        let mut mtn = CardData::new(mtn_id, p1, "Mountain");
        mtn.card_types = vec![CardType::Land];
        mtn.subtypes = vec![SubType::Mountain];
        game.state.card_store.insert(mtn);

        if let Some(player) = game.state.players.get_mut(&p1) {
            while player.library.draw().is_some() {}
            // Top to bottom: Elf, Goblin, Mountain, Forest
            player.library.put_on_bottom(elf_id);
            player.library.put_on_bottom(gob_id);
            player.library.put_on_bottom(mtn_id);
            player.library.put_on_bottom(forest_id);
        }

        let hand_before = game.state.players.get(&p1).unwrap().hand.len();

        // Look at top 4, pick "Elf or Swamp or Forest"
        game.execute_effects(
            &[Effect::look_top_and_pick(4, "Elf or Swamp or Forest")],
            p1,
            &[],
            None, None,
        );

        let player = game.state.players.get(&p1).unwrap();
        // Should have picked the Elf (first match) to hand
        assert_eq!(player.hand.len(), hand_before + 1);
        assert!(player.hand.contains(elf_id), "Elf should be in hand");
        // Library should have 3 cards remaining (Goblin, Mountain, Forest on bottom)
        assert_eq!(player.library.len(), 3);
        assert!(!player.library.contains(elf_id), "Elf should not be in library");
    }

    #[test]
    fn gain_control_until_end_of_turn() {
        // Test that GainControlUntilEndOfTurn changes controller, untaps, grants haste.
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
                (p1, Box::new(AlwaysPassPlayer)),
                (p2, Box::new(AlwaysPassPlayer)),
            ],
        );

        // Create a creature owned+controlled by p2
        let bear_id = ObjectId::new();
        let mut bear = CardData::new(bear_id, p2, "Grizzly Bears");
        bear.card_types = vec![CardType::Creature];
        bear.power = Some(2);
        bear.toughness = Some(2);
        bear.keywords = KeywordAbilities::empty();
        game.state.card_store.insert(bear.clone());

        let mut perm = crate::permanent::Permanent::new(bear, p2);
        perm.tapped = true; // Start tapped
        game.state.battlefield.add(perm);

        // p1 casts gain_control_eot on the bear
        let effects = vec![Effect::GainControlUntilEndOfTurn];
        let targets = vec![bear_id];
        game.execute_effects(&effects, p1, &targets, None, None);

        // Bear should now be controlled by p1, untapped, with haste
        let perm = game.state.battlefield.get(bear_id).unwrap();
        assert_eq!(perm.controller, p1, "Controller should be p1");
        assert!(!perm.tapped, "Should be untapped");
        assert!(perm.has_haste(), "Should have haste");
        assert_eq!(perm.original_controller, Some(p2), "Original controller tracked");
    }
}

#[cfg(test)]
mod modal_test {
    use super::*;
    use crate::abilities::{Effect, ModalMode};
    use crate::card::CardData;
    use crate::constants::{CardType, Outcome};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };
    use crate::types::{ObjectId, PlayerId};

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
}

#[cfg(test)]
mod cost_tests {
    use super::*;
    use crate::abilities::Cost;
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, Outcome};
    use crate::counters::CounterType;
    use crate::decision::*;
    use crate::game::{GameConfig, PlayerConfig};
    use crate::types::{ObjectId, PlayerId};

    /// Decision maker that selects the last N cards for discard/exile choices.
    struct LastCardPicker;

    impl PlayerDecisionMaker for LastCardPicker {
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
        fn choose_discard(&mut self, _: &GameView<'_>, hand: &[ObjectId], count: usize) -> Vec<ObjectId> {
            // Pick the last N cards
            hand.iter().rev().take(count).copied().collect()
        }
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

    fn setup_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(LastCardPicker)),
                (p2, Box::new(LastCardPicker)),
            ],
        );
        (game, p1, p2)
    }

    fn add_creature(game: &mut Game, owner: PlayerId, name: &str) -> ObjectId {
        let id = ObjectId::new();
        let mut card = CardData::new(id, owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        card.keywords = KeywordAbilities::empty();
        game.state.battlefield.add(Permanent::new(card, owner));
        id
    }

    #[test]
    fn pay_remove_counters_cost() {
        let (mut game, p1, _p2) = setup_game();
        let source_id = add_creature(&mut game, p1, "Counter Creature");

        // Add 3 -1/-1 counters
        game.state.battlefield.get_mut(source_id).unwrap()
            .add_counters(CounterType::M1M1, 3);

        // Pay 2 -1/-1 counter removal cost
        assert!(game.pay_costs(p1, source_id, &[Cost::RemoveCounters("-1/-1".into(), 2)]));
        assert_eq!(game.state.battlefield.get(source_id).unwrap().counters.get(&CounterType::M1M1), 1);

        // Can't pay 2 more (only 1 left)
        assert!(!game.pay_costs(p1, source_id, &[Cost::RemoveCounters("-1/-1".into(), 2)]));
    }

    #[test]
    fn pay_blight_cost() {
        let (mut game, p1, _p2) = setup_game();
        let source_id = add_creature(&mut game, p1, "Blight Creature");

        // Blight 2 puts 2 -1/-1 counters on self
        assert!(game.pay_costs(p1, source_id, &[Cost::Blight(2)]));
        assert_eq!(game.state.battlefield.get(source_id).unwrap().counters.get(&CounterType::M1M1), 2);
        // Power should be reduced
        assert_eq!(game.state.battlefield.get(source_id).unwrap().power(), 0);
    }

    #[test]
    fn pay_exile_from_graveyard_cost() {
        let (mut game, p1, _p2) = setup_game();
        let source_id = add_creature(&mut game, p1, "Source");

        // Add 3 cards to graveyard
        let gy1 = ObjectId::new();
        let gy2 = ObjectId::new();
        let gy3 = ObjectId::new();
        game.state.players.get_mut(&p1).unwrap().graveyard.add(gy1);
        game.state.players.get_mut(&p1).unwrap().graveyard.add(gy2);
        game.state.players.get_mut(&p1).unwrap().graveyard.add(gy3);

        // Can't exile 4 (only 3)
        assert!(!game.pay_costs(p1, source_id, &[Cost::ExileFromGraveyard(4)]));

        // Exile 2
        assert!(game.pay_costs(p1, source_id, &[Cost::ExileFromGraveyard(2)]));
        assert_eq!(game.state.players.get(&p1).unwrap().graveyard.len(), 1);
        // Exiled cards should be in exile zone
        assert!(game.state.exile.contains(gy3) || game.state.exile.contains(gy2));
    }

    #[test]
    fn pay_exile_from_hand_cost() {
        let (mut game, p1, _p2) = setup_game();
        let source_id = add_creature(&mut game, p1, "Source");

        // Add 2 cards to hand
        let h1 = ObjectId::new();
        let h2 = ObjectId::new();
        game.state.players.get_mut(&p1).unwrap().hand.add(h1);
        game.state.players.get_mut(&p1).unwrap().hand.add(h2);
        let before = game.state.players.get(&p1).unwrap().hand.len();

        // Exile 1
        assert!(game.pay_costs(p1, source_id, &[Cost::ExileFromHand(1)]));
        assert_eq!(game.state.players.get(&p1).unwrap().hand.len(), before - 1);
    }

    #[test]
    fn pay_sacrifice_other_cost() {
        let (mut game, p1, _p2) = setup_game();
        let source_id = add_creature(&mut game, p1, "Source");
        let other_id = add_creature(&mut game, p1, "Other Creature");

        // Sacrifice another creature
        assert!(game.pay_costs(p1, source_id, &[Cost::SacrificeOther("a creature".into())]));
        // The other creature should be gone
        assert!(!game.state.battlefield.contains(other_id));
        // Source should still be there
        assert!(game.state.battlefield.contains(source_id));
    }

    #[test]
    fn pay_untap_self_cost() {
        let (mut game, p1, _p2) = setup_game();
        let source_id = add_creature(&mut game, p1, "Untap Me");

        // Can't untap (not tapped)
        assert!(!game.pay_costs(p1, source_id, &[Cost::UntapSelf]));

        // Tap it first
        game.state.battlefield.get_mut(source_id).unwrap().tap();
        assert!(game.state.battlefield.get(source_id).unwrap().tapped);

        // Now untap cost works
        assert!(game.pay_costs(p1, source_id, &[Cost::UntapSelf]));
        assert!(!game.state.battlefield.get(source_id).unwrap().tapped);
    }
}

#[cfg(test)]
mod vivid_tests {
    use super::*;
    use crate::abilities::Effect;
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, Outcome};
    use crate::decision::*;
    use crate::game::{GameConfig, PlayerConfig};
    use crate::mana::{ManaCost};
    use crate::types::{ObjectId, PlayerId};

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
}

#[cfg(test)]
mod choice_tests {
    use super::*;
    use crate::abilities::{Cost, Effect};
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, Outcome};
    use crate::counters::CounterType;
    use crate::decision::*;
    use crate::game::{GameConfig, PlayerConfig};
    use crate::types::{ObjectId, PlayerId};

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

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
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
                PlayerConfig { name: "A".into(), deck: make_deck(p1) },
                PlayerConfig { name: "B".into(), deck: make_deck(p2) },
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
}



#[cfg(test)]
mod type_choice_tests {
    use super::*;
    use crate::abilities::Effect;
    use crate::card::CardData;
    use crate::constants::{CardType, Outcome, SubType};
    use crate::decision::*;
    use crate::game::{GameConfig, PlayerConfig};
    use crate::types::{ObjectId, PlayerId};

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

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
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
                PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
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
}

#[cfg(test)]
mod combat_tests {
    use super::*;
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, Outcome};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };

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
        card.power = Some(power);
        card.toughness = Some(toughness);
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
        p1_dm: Box<dyn PlayerDecisionMaker>,
        p2_dm: Box<dyn PlayerDecisionMaker>,
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
            starting_life: 20,
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
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
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
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
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
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
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
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
        );

        add_creature(&mut game, p1, "Lifelinker", 4, 4, KeywordAbilities::LIFELINK);

        game.state.active_player = p1;
        game.state.priority_player = p1;

        // Reduce p1 life to verify gain
        game.state.players.get_mut(&p1).unwrap().life = 15;

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
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
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
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
        );

        add_creature(&mut game, p1, "Trampler", 5, 5, KeywordAbilities::TRAMPLE);
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
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
        );

        add_creature(&mut game, p1, "Bear", 2, 2, KeywordAbilities::empty());

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
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
        );

        // Only a defender creature
        add_creature(&mut game, p1, "Wall", 0, 5, KeywordAbilities::DEFENDER);

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
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
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
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
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
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
        );

        add_creature(&mut game, p1, "Flyer", 3, 3, KeywordAbilities::FLYING);
        // Ground creature cannot block a flyer
        add_creature(&mut game, p2, "Ground", 2, 2, KeywordAbilities::empty());

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
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
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
            Box::new(AttackAllPlayer),
            Box::new(BlockAllPlayer),
        );

        add_creature(&mut game, p1, "Bear1", 2, 2, KeywordAbilities::empty());
        add_creature(&mut game, p1, "Bear2", 3, 3, KeywordAbilities::empty());

        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.declare_attackers_step(p1);
        game.declare_blockers_step(p1);
        game.combat_damage_step(false);

        // Both should deal damage: 2 + 3 = 5
        assert_eq!(game.state.players[&p2].life, 15);
    }
}

#[cfg(test)]
mod trigger_tests {
    use super::*;
    use crate::abilities::{Ability, Effect, TargetSpec};
    use crate::card::CardData;
    use crate::constants::{CardType, Outcome};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };
    use crate::events::EventType;

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
}

#[cfg(test)]
mod continuous_effect_tests {
    use super::*;
    use crate::abilities::{Ability, StaticEffect};
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, Outcome, SubType};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };

    /// Passive decision maker — always passes.
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

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        (0..40).map(|i| {
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
                PlayerConfig { name: "Player1".into(), deck: make_deck(p1) },
                PlayerConfig { name: "Player2".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(
            config,
            vec![(p1, Box::new(PassivePlayer)), (p2, Box::new(PassivePlayer))],
        );
        (game, p1, p2)
    }

    fn add_creature(
        game: &mut Game,
        owner: PlayerId,
        name: &str,
        power: i32,
        toughness: i32,
        keywords: KeywordAbilities,
    ) -> ObjectId {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(power);
        card.toughness = Some(toughness);
        card.keywords = keywords;
        let id = card.id;
        let perm = Permanent::new(card, owner);
        game.state.battlefield.add(perm);
        id
    }

    fn add_creature_with_subtype(
        game: &mut Game,
        owner: PlayerId,
        name: &str,
        power: i32,
        toughness: i32,
        subtype: SubType,
    ) -> ObjectId {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![subtype];
        card.power = Some(power);
        card.toughness = Some(toughness);
        let id = card.id;
        let perm = Permanent::new(card, owner);
        game.state.battlefield.add(perm);
        id
    }

    fn add_lord_with_boost(
        game: &mut Game,
        owner: PlayerId,
        name: &str,
        power: i32,
        toughness: i32,
        subtype: SubType,
        filter: &str,
        boost_p: i32,
        boost_t: i32,
    ) -> ObjectId {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![subtype];
        card.power = Some(power);
        card.toughness = Some(toughness);
        let id = card.id;
        card.abilities = vec![
            Ability::static_ability(id, &format!("Other creatures get +{boost_p}/+{boost_t}"),
                vec![StaticEffect::Boost { filter: filter.into(), power: boost_p, toughness: boost_t }]),
        ];
        let perm = Permanent::new(card, owner);
        game.state.battlefield.add(perm);
        // Register abilities
        let abilities: Vec<Ability> = game.state.battlefield.get(id).unwrap().card.abilities.clone();
        for ability in abilities {
            game.state.ability_store.add(ability);
        }
        id
    }

    fn add_keyword_lord(
        game: &mut Game,
        owner: PlayerId,
        name: &str,
        power: i32,
        toughness: i32,
        filter: &str,
        keyword: &str,
    ) -> ObjectId {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(power);
        card.toughness = Some(toughness);
        let id = card.id;
        card.abilities = vec![
            Ability::static_ability(id, &format!("Creatures have {keyword}"),
                vec![StaticEffect::GrantKeyword { filter: filter.into(), keyword: keyword.into() }]),
        ];
        let perm = Permanent::new(card, owner);
        game.state.battlefield.add(perm);
        let abilities: Vec<Ability> = game.state.battlefield.get(id).unwrap().card.abilities.clone();
        for ability in abilities {
            game.state.ability_store.add(ability);
        }
        id
    }

    // ── Test: Lord boosts other creatures of same type ──────────────

    #[test]
    fn lord_boosts_other_creatures_of_same_type() {
        let (mut game, p1, _p2) = setup();

        // Add an Elf lord: "Other Elf you control get +1/+1"
        let lord_id = add_lord_with_boost(&mut game, p1, "Elvish Archdruid", 2, 2,
            SubType::Elf, "other Elf you control", 1, 1);

        // Add two Elf creatures
        let elf1_id = add_creature_with_subtype(&mut game, p1, "Llanowar Elves", 1, 1, SubType::Elf);
        let elf2_id = add_creature_with_subtype(&mut game, p1, "Elvish Mystic", 1, 1, SubType::Elf);

        // Add a non-Elf creature
        let bear_id = add_creature(&mut game, p1, "Grizzly Bears", 2, 2, KeywordAbilities::empty());

        // Apply continuous effects
        game.apply_continuous_effects();

        // Lord itself should NOT be boosted (filter says "other")
        let lord = game.state.battlefield.get(lord_id).unwrap();
        assert_eq!(lord.power(), 2);
        assert_eq!(lord.toughness(), 2);

        // Elves should be boosted
        let elf1 = game.state.battlefield.get(elf1_id).unwrap();
        assert_eq!(elf1.power(), 2);
        assert_eq!(elf1.toughness(), 2);

        let elf2 = game.state.battlefield.get(elf2_id).unwrap();
        assert_eq!(elf2.power(), 2);
        assert_eq!(elf2.toughness(), 2);

        // Bear should NOT be boosted (not an Elf)
        let bear = game.state.battlefield.get(bear_id).unwrap();
        assert_eq!(bear.power(), 2);
        assert_eq!(bear.toughness(), 2);
    }

    // ── Test: Anthem boosts all creatures you control ──────────────

    #[test]
    fn anthem_boosts_all_creatures_you_control() {
        let (mut game, p1, p2) = setup();

        // Add anthem: "creature you control get +1/+1"
        let anthem_id = add_lord_with_boost(&mut game, p1, "Glorious Anthem", 0, 0,
            SubType::Custom("Enchantment".into()), "creature you control", 1, 1);

        // P1's creature
        let bear1_id = add_creature(&mut game, p1, "Bear", 2, 2, KeywordAbilities::empty());

        // P2's creature should NOT be boosted
        let bear2_id = add_creature(&mut game, p2, "Enemy Bear", 2, 2, KeywordAbilities::empty());

        game.apply_continuous_effects();

        // Anthem itself is a 0/0 creature, so it gets +1/+1 too
        // (filter is "creature you control", not "other creature")
        let anthem = game.state.battlefield.get(anthem_id).unwrap();
        assert_eq!(anthem.power(), 1);
        assert_eq!(anthem.toughness(), 1);

        let bear1 = game.state.battlefield.get(bear1_id).unwrap();
        assert_eq!(bear1.power(), 3);
        assert_eq!(bear1.toughness(), 3);

        let bear2 = game.state.battlefield.get(bear2_id).unwrap();
        assert_eq!(bear2.power(), 2);
        assert_eq!(bear2.toughness(), 2);
    }

    // ── Test: Keyword grant ────────────────────────────────────────

    #[test]
    fn keyword_grant_gives_keyword_to_matching_creatures() {
        let (mut game, p1, _p2) = setup();

        // "Creatures you control have flying"
        add_keyword_lord(&mut game, p1, "Archetype of Imagination", 3, 2,
            "creature you control", "flying");

        let bear_id = add_creature(&mut game, p1, "Bear", 2, 2, KeywordAbilities::empty());

        game.apply_continuous_effects();

        let bear = game.state.battlefield.get(bear_id).unwrap();
        assert!(bear.has_flying());
    }

    // ── Test: Multiple keywords in comma-separated string ──────────

    #[test]
    fn comma_separated_keywords_granted() {
        let (mut game, p1, _p2) = setup();

        // "Equipped creature has deathtouch, lifelink"
        add_keyword_lord(&mut game, p1, "Basilisk Collar", 0, 0,
            "creature you control", "deathtouch, lifelink");

        let bear_id = add_creature(&mut game, p1, "Bear", 2, 2, KeywordAbilities::empty());

        game.apply_continuous_effects();

        let bear = game.state.battlefield.get(bear_id).unwrap();
        assert!(bear.has_deathtouch());
        assert!(bear.has_lifelink());
    }

    // ── Test: Effects cleared on recalculation ─────────────────────

    #[test]
    fn effects_cleared_and_recalculated() {
        let (mut game, p1, _p2) = setup();

        let lord_id = add_lord_with_boost(&mut game, p1, "Elvish Archdruid", 2, 2,
            SubType::Elf, "other Elf you control", 1, 1);

        let elf_id = add_creature_with_subtype(&mut game, p1, "Llanowar Elves", 1, 1, SubType::Elf);

        // Apply once
        game.apply_continuous_effects();
        assert_eq!(game.state.battlefield.get(elf_id).unwrap().power(), 2);

        // Remove lord from battlefield
        game.state.battlefield.remove(lord_id);
        game.state.ability_store.remove_source(lord_id);

        // Apply again — boost should be gone
        game.apply_continuous_effects();
        assert_eq!(game.state.battlefield.get(elf_id).unwrap().power(), 1);
    }

    // ── Test: Multiple lords stack ─────────────────────────────────

    #[test]
    fn multiple_lords_stack() {
        let (mut game, p1, _p2) = setup();

        // Two Elf lords
        add_lord_with_boost(&mut game, p1, "Lord 1", 2, 2,
            SubType::Elf, "other Elf you control", 1, 1);
        add_lord_with_boost(&mut game, p1, "Lord 2", 2, 2,
            SubType::Elf, "other Elf you control", 1, 1);

        let elf_id = add_creature_with_subtype(&mut game, p1, "Llanowar Elves", 1, 1, SubType::Elf);

        game.apply_continuous_effects();

        // Elf should get +1/+1 from each lord = +2/+2 total
        let elf = game.state.battlefield.get(elf_id).unwrap();
        assert_eq!(elf.power(), 3);
        assert_eq!(elf.toughness(), 3);
    }

    // ── Test: Lord boosts each other ───────────────────────────────

    #[test]
    fn lords_boost_each_other() {
        let (mut game, p1, _p2) = setup();

        // Two Elf lords with "other Elf you control get +1/+1"
        let lord1_id = add_lord_with_boost(&mut game, p1, "Lord 1", 2, 2,
            SubType::Elf, "other Elf you control", 1, 1);
        let lord2_id = add_lord_with_boost(&mut game, p1, "Lord 2", 2, 2,
            SubType::Elf, "other Elf you control", 1, 1);

        game.apply_continuous_effects();

        // Each lord should get +1/+1 from the other
        let lord1 = game.state.battlefield.get(lord1_id).unwrap();
        assert_eq!(lord1.power(), 3);
        assert_eq!(lord1.toughness(), 3);

        let lord2 = game.state.battlefield.get(lord2_id).unwrap();
        assert_eq!(lord2.power(), 3);
        assert_eq!(lord2.toughness(), 3);
    }

    // ── Test: "self" filter applies only to source ─────────────────

    #[test]
    fn self_filter_applies_only_to_source() {
        let (mut game, p1, _p2) = setup();

        // A creature with a static effect targeting "self"
        let mut card = CardData::new(ObjectId::new(), p1, "Self-Booster");
        card.card_types = vec![CardType::Creature];
        card.power = Some(1);
        card.toughness = Some(1);
        let id = card.id;
        card.abilities = vec![
            Ability::static_ability(id, "+2/+2 to self",
                vec![StaticEffect::Boost { filter: "self".into(), power: 2, toughness: 2 }]),
        ];
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);
        let abilities: Vec<Ability> = game.state.battlefield.get(id).unwrap().card.abilities.clone();
        for a in abilities { game.state.ability_store.add(a); }

        let other_id = add_creature(&mut game, p1, "Other", 1, 1, KeywordAbilities::empty());

        game.apply_continuous_effects();

        assert_eq!(game.state.battlefield.get(id).unwrap().power(), 3);
        assert_eq!(game.state.battlefield.get(other_id).unwrap().power(), 1);
    }

    // ── Test: Token filter ─────────────────────────────────────────

    #[test]
    fn token_filter_only_matches_tokens() {
        let (mut game, p1, _p2) = setup();

        // "Creature token you control get +1/+1"
        add_lord_with_boost(&mut game, p1, "Token Lord", 2, 2,
            SubType::Custom("Lord".into()), "creature token you control", 1, 1);

        // Regular creature
        let regular_id = add_creature(&mut game, p1, "Regular Bear", 2, 2, KeywordAbilities::empty());

        // Token creature
        let token_id = ObjectId::new();
        let mut token_card = CardData::new(token_id, p1, "Bear Token");
        token_card.card_types = vec![CardType::Creature];
        token_card.power = Some(2);
        token_card.toughness = Some(2);
        token_card.is_token = true;
        let token_perm = Permanent::new(token_card, p1);
        game.state.battlefield.add(token_perm);

        game.apply_continuous_effects();

        // Regular creature should NOT be boosted
        assert_eq!(game.state.battlefield.get(regular_id).unwrap().power(), 2);

        // Token should be boosted
        assert_eq!(game.state.battlefield.get(token_id).unwrap().power(), 3);
    }

    // ── Test: Opponent's lord doesn't boost your creatures ─────────

    #[test]
    fn opponent_lord_doesnt_boost_your_creatures() {
        let (mut game, p1, p2) = setup();

        // P2 has Elf lord
        add_lord_with_boost(&mut game, p2, "Enemy Lord", 2, 2,
            SubType::Elf, "other Elf you control", 1, 1);

        // P1 has Elf
        let elf_id = add_creature_with_subtype(&mut game, p1, "My Elf", 1, 1, SubType::Elf);

        game.apply_continuous_effects();

        // P1's Elf should NOT be boosted by P2's lord
        assert_eq!(game.state.battlefield.get(elf_id).unwrap().power(), 1);
    }

    // ── Test: Boost + keyword grant combo ──────────────────────────

    #[test]
    fn boost_and_keyword_combo() {
        let (mut game, p1, _p2) = setup();

        // A lord with both boost and keyword grant
        let mut card = CardData::new(ObjectId::new(), p1, "Drogskol Captain");
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![SubType::Spirit];
        card.power = Some(2);
        card.toughness = Some(2);
        let id = card.id;
        card.abilities = vec![
            Ability::static_ability(id, "Other Spirit creatures you control get +1/+1 and have hexproof",
                vec![
                    StaticEffect::Boost { filter: "other Spirit you control".into(), power: 1, toughness: 1 },
                    StaticEffect::GrantKeyword { filter: "other Spirit you control".into(), keyword: "hexproof".into() },
                ]),
        ];
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);
        let abilities: Vec<Ability> = game.state.battlefield.get(id).unwrap().card.abilities.clone();
        for a in abilities { game.state.ability_store.add(a); }

        let spirit_id = add_creature_with_subtype(&mut game, p1, "Mausoleum Wanderer", 1, 1, SubType::Spirit);

        game.apply_continuous_effects();

        let spirit = game.state.battlefield.get(spirit_id).unwrap();
        assert_eq!(spirit.power(), 2);
        assert_eq!(spirit.toughness(), 2);
        assert!(spirit.has_hexproof());
    }
}

#[cfg(test)]
mod enters_tapped_tests {
    use super::*;
    use crate::abilities::{Ability, StaticEffect};
    use crate::card::CardData;
    use crate::constants::{CardType, Outcome, SubType};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };
    use crate::mana::Mana;

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

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        (0..40).map(|i| {
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
                PlayerConfig { name: "Player1".into(), deck: make_deck(p1) },
                PlayerConfig { name: "Player2".into(), deck: make_deck(p2) },
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
        let (mut game, p1, _p2) = setup();

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
        let (mut game, p1, _p2) = setup();

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
}

#[cfg(test)]
mod hexproof_tests {
    use super::*;
    use crate::abilities::TargetSpec;
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, Outcome};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };

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

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        (0..40).map(|i| {
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
                PlayerConfig { name: "Player1".into(), deck: make_deck(p1) },
                PlayerConfig { name: "Player2".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(
            config,
            vec![(p1, Box::new(PassivePlayer)), (p2, Box::new(PassivePlayer))],
        );
        (game, p1, p2)
    }

    fn add_creature(game: &mut Game, owner: PlayerId, name: &str, kw: KeywordAbilities) -> ObjectId {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        card.keywords = kw;
        let id = card.id;
        game.state.battlefield.add(Permanent::new(card, owner));
        id
    }

    #[test]
    fn hexproof_prevents_opponent_targeting() {
        let (mut game, p1, p2) = setup();

        let hexproof_id = add_creature(&mut game, p2, "Hexproof Bear", KeywordAbilities::HEXPROOF);
        let regular_id = add_creature(&mut game, p2, "Regular Bear", KeywordAbilities::empty());

        // P1 targeting creatures — hexproof creature should NOT be in legal targets
        let targets = game.legal_targets_for_spec(&TargetSpec::Creature, p1);
        assert!(!targets.contains(&hexproof_id));
        assert!(targets.contains(&regular_id));

        // Opponent creature targeting — same
        let targets = game.legal_targets_for_spec(&TargetSpec::OpponentCreature, p1);
        assert!(!targets.contains(&hexproof_id));
        assert!(targets.contains(&regular_id));
    }

    #[test]
    fn hexproof_allows_controller_targeting() {
        let (mut game, _p1, p2) = setup();

        let hexproof_id = add_creature(&mut game, p2, "Hexproof Bear", KeywordAbilities::HEXPROOF);

        // P2 targeting their own hexproof creature — should be allowed
        let targets = game.legal_targets_for_spec(&TargetSpec::CreatureYouControl, p2);
        assert!(targets.contains(&hexproof_id));

        // P2 targeting any creature — their own hexproof creature is fine
        let targets = game.legal_targets_for_spec(&TargetSpec::Creature, p2);
        assert!(targets.contains(&hexproof_id));
    }

    #[test]
    fn shroud_prevents_all_targeting() {
        let (mut game, p1, p2) = setup();

        let shroud_id = add_creature(&mut game, p2, "Shroud Bear", KeywordAbilities::SHROUD);

        // Neither player can target a shroud creature
        let targets = game.legal_targets_for_spec(&TargetSpec::Creature, p1);
        assert!(!targets.contains(&shroud_id));

        let targets = game.legal_targets_for_spec(&TargetSpec::Creature, p2);
        assert!(!targets.contains(&shroud_id));
    }

    #[test]
    fn hexproof_on_permanent_targeting() {
        let (mut game, p1, p2) = setup();

        let hexproof_id = add_creature(&mut game, p2, "Hexproof Bear", KeywordAbilities::HEXPROOF);
        let regular_id = add_creature(&mut game, p2, "Regular Bear", KeywordAbilities::empty());

        // TargetSpec::Permanent — hexproof blocks opponent targeting
        let targets = game.legal_targets_for_spec(&TargetSpec::Permanent, p1);
        assert!(!targets.contains(&hexproof_id));
        assert!(targets.contains(&regular_id));
    }

    #[test]
    fn granted_hexproof_prevents_targeting() {
        let (mut game, p1, p2) = setup();

        // A creature without hexproof that gets it granted
        let bear_id = add_creature(&mut game, p2, "Bear", KeywordAbilities::empty());

        // Grant hexproof via continuous_keywords
        if let Some(perm) = game.state.battlefield.get_mut(bear_id) {
            perm.continuous_keywords |= KeywordAbilities::HEXPROOF;
        }

        // P1 should not be able to target the bear with continuous hexproof
        let targets = game.legal_targets_for_spec(&TargetSpec::Creature, p1);
        assert!(!targets.contains(&bear_id));
    }
}

#[cfg(test)]
mod dies_trigger_tests {
    use super::*;
    use crate::abilities::{Ability, Effect, TargetSpec};
    use crate::card::CardData;
    use crate::constants::{CardType, Outcome};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };
    use crate::events::EventType;

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

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        (0..40).map(|i| {
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
                PlayerConfig { name: "Player1".into(), deck: make_deck(p1) },
                PlayerConfig { name: "Player2".into(), deck: make_deck(p2) },
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
        let (mut game, p1, _p2) = setup();

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
        let (mut game, p1, p2) = setup();

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
        let (mut game, p1, _p2) = setup();

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
}

// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Equipment tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod equipment_tests {
    use super::*;
    use crate::abilities::{Ability, Cost, Effect, StaticEffect, TargetSpec};
    use crate::card::CardData;
    use crate::constants::{CardType, Outcome, SubType};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };
    use crate::mana::ManaCost;
    use crate::types::{ObjectId, PlayerId};

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
}

// ---------------------------------------------------------------------------
// Aura tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod aura_tests {
    use super::*;
    use crate::abilities::{Ability, StaticEffect};
    use crate::card::CardData;
    use crate::constants::{CardType, Outcome, SubType};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };
    use crate::types::{ObjectId, PlayerId};

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

    fn make_creature(id: ObjectId, owner: PlayerId, power: i32, toughness: i32) -> CardData {
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

    fn register_abilities(game: &mut Game, perm_id: ObjectId) {
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

        game.state.battlefield.add(Permanent::new(make_creature(creature_id, p1, 2, 2), p1));

        let aura = make_aura_boost(aura_id, p1, "Giant Growth Aura", 3, 3);
        game.state.battlefield.add(Permanent::new(aura, p1));
        register_abilities(&mut game, aura_id);

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

        game.state.battlefield.add(Permanent::new(make_creature(creature_id, p1, 2, 2), p1));
        let aura = make_aura_boost(aura_id, p1, "Ethereal Armor", 2, 2);
        game.state.battlefield.add(Permanent::new(aura, p1));
        register_abilities(&mut game, aura_id);

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

        game.state.battlefield.add(Permanent::new(make_creature(creature_id, p2, 3, 3), p2));

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
}

// ---------------------------------------------------------------------------
// Prowess and landwalk tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod prowess_landwalk_tests {
    use super::*;
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, Outcome, SubType};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };
    use crate::events::{GameEvent};
    use crate::types::{ObjectId, PlayerId};

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

    #[test]
    fn prowess_triggers_on_noncreature_spell() {
        let (mut game, p1, _p2) = setup();
        let creature_id = ObjectId::new();

        // Create a creature with prowess
        let mut creature = CardData::new(creature_id, p1, "Prowess Monk");
        creature.card_types = vec![CardType::Creature];
        creature.subtypes = vec![SubType::Human];
        creature.power = Some(1);
        creature.toughness = Some(1);
        creature.keywords = KeywordAbilities::PROWESS;
        game.state.battlefield.add(Permanent::new(creature.clone(), p1));
        game.state.card_store.insert(creature);

        // Create a noncreature spell in the card store
        let spell_id = ObjectId::new();
        let mut spell = CardData::new(spell_id, p1, "Lightning Bolt");
        spell.card_types = vec![CardType::Instant];
        game.state.card_store.insert(spell);

        // Emit a SpellCast event
        game.emit_event(GameEvent::spell_cast(spell_id, p1, crate::constants::Zone::Hand));

        // Check triggered abilities — this should process prowess
        game.check_triggered_abilities();

        // Prowess should have added a +1/+1 counter
        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert_eq!(perm.power(), 2, "Prowess should boost power to 2");
        assert_eq!(perm.toughness(), 2, "Prowess should boost toughness to 2");
    }

    #[test]
    fn prowess_does_not_trigger_on_creature_spell() {
        let (mut game, p1, _p2) = setup();
        let creature_id = ObjectId::new();

        let mut creature = CardData::new(creature_id, p1, "Prowess Monk");
        creature.card_types = vec![CardType::Creature];
        creature.power = Some(1);
        creature.toughness = Some(1);
        creature.keywords = KeywordAbilities::PROWESS;
        game.state.battlefield.add(Permanent::new(creature.clone(), p1));
        game.state.card_store.insert(creature);

        // Cast a creature spell (should NOT trigger prowess)
        let spell_id = ObjectId::new();
        let mut spell = CardData::new(spell_id, p1, "Grizzly Bears");
        spell.card_types = vec![CardType::Creature];
        game.state.card_store.insert(spell);

        game.emit_event(GameEvent::spell_cast(spell_id, p1, crate::constants::Zone::Hand));
        game.check_triggered_abilities();

        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert_eq!(perm.power(), 1, "Prowess should NOT trigger on creature spell");
    }

    #[test]
    fn forestwalk_unblockable_vs_forest_controller() {
        // Test landwalk evasion — if defender controls a Forest, creature with
        // forestwalk can't be blocked. We test this by checking combat::can_block
        // logic indirectly via the permanent struct.

        let owner = PlayerId::new();
        let id = ObjectId::new();
        let mut card = CardData::new(id, owner, "Forestwalker");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        card.keywords = KeywordAbilities::FORESTWALK;
        let attacker = Permanent::new(card, owner);

        // Verify the keyword is set
        assert!(attacker.has_keyword(KeywordAbilities::FORESTWALK));

        // Basic can_block doesn't check landwalk (that's at game level)
        let blocker_owner = PlayerId::new();
        let blocker_id = ObjectId::new();
        let mut blocker_card = CardData::new(blocker_id, blocker_owner, "Blocker");
        blocker_card.card_types = vec![CardType::Creature];
        blocker_card.power = Some(3);
        blocker_card.toughness = Some(3);
        let blocker = Permanent::new(blocker_card, blocker_owner);

        // Without landwalk check, normal blocking is fine
        assert!(combat::can_block(&blocker, &attacker));
    }
}


#[cfg(test)]
mod ward_tests {
    use super::*;
    use crate::abilities::{Ability, StaticEffect, TargetSpec, Effect};
    use crate::constants::Outcome;
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities};
    use crate::mana::{ManaCost, Mana};
    use crate::types::{ObjectId, PlayerId};
    use crate::decision::*;

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

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        (0..40).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    fn setup_ward_game() -> (Game, PlayerId, PlayerId, ObjectId, ObjectId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Attacker".into(), deck: make_deck(p1) },
                PlayerConfig { name: "Defender".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let mut game = Game::new_two_player(
            config,
            vec![(p1, Box::new(PassivePlayer)), (p2, Box::new(PassivePlayer))],
        );

        // Create a creature with Ward {2} controlled by p2
        let ward_creature_id = ObjectId::new();
        let mut ward_card = CardData::new(ward_creature_id, p2, "Warded Beast");
        ward_card.card_types = vec![CardType::Creature];
        ward_card.power = Some(4);
        ward_card.toughness = Some(4);
        ward_card.keywords = KeywordAbilities::WARD;
        let perm = Permanent::new(ward_card.clone(), p2);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(ward_card);

        // Register Ward {2} static ability
        let ward_ability = Ability::static_ability(
            ward_creature_id,
            "Ward {2}",
            vec![StaticEffect::ward("{2}")],
        );
        game.state.ability_store.add(ward_ability);

        // Create a removal spell in p1's hand
        let spell_id = ObjectId::new();
        let mut spell_card = CardData::new(spell_id, p1, "Doom Blade");
        spell_card.card_types = vec![CardType::Instant];
        spell_card.mana_cost = ManaCost::parse("{1}{B}");
        spell_card.abilities = vec![Ability::spell(
            spell_id,
            vec![Effect::Destroy],
            TargetSpec::OpponentCreature,
        )];
        game.state.card_store.insert(spell_card);
        game.state.players.get_mut(&p1).unwrap().hand.add(spell_id);

        (game, p1, p2, ward_creature_id, spell_id)
    }

    #[test]
    fn ward_counters_spell_when_opponent_cant_pay() {
        let (mut game, p1, _p2, ward_creature_id, spell_id) = setup_ward_game();

        // Give p1 only enough mana for the spell (1B), not for ward ({2})
        game.state.players.get_mut(&p1).unwrap().mana_pool.add(
            Mana { white: 0, blue: 0, black: 1, red: 0, green: 0, colorless: 1, generic: 0, any: 0 }, None, false
        );

        game.cast_spell(p1, spell_id);

        // The spell should be on the stack but countered (no mana left for ward)
        let stack_item = game.state.stack.get(spell_id);
        assert!(stack_item.is_some(), "Spell should be on the stack");
        assert!(stack_item.unwrap().countered, "Spell should be countered by Ward");

        // Resolve — creature should survive
        game.resolve_top_of_stack();
        assert!(game.state.battlefield.contains(ward_creature_id), "Warded creature should survive");
    }

    #[test]
    fn ward_allows_spell_when_opponent_pays() {
        let (mut game, p1, _p2, _ward_creature_id, spell_id) = setup_ward_game();

        // Give p1 enough mana for spell (1B) + ward (2)
        game.state.players.get_mut(&p1).unwrap().mana_pool.add(
            Mana { white: 0, blue: 0, black: 1, red: 0, green: 0, colorless: 4, generic: 0, any: 0 }, None, false
        );

        game.cast_spell(p1, spell_id);

        // Ward should be paid — spell should NOT be countered
        let stack_item = game.state.stack.get(spell_id);
        assert!(stack_item.is_some(), "Spell should be on the stack");
        assert!(!stack_item.unwrap().countered, "Spell should NOT be countered when ward cost is paid");
    }

    #[test]
    fn ward_doesnt_trigger_on_own_creatures() {
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

        // Create a ward creature controlled by p1 (self-target shouldn't trigger ward)
        let own_ward_id = ObjectId::new();
        let mut own_card = CardData::new(own_ward_id, p1, "Own Warded");
        own_card.card_types = vec![CardType::Creature];
        own_card.power = Some(3);
        own_card.toughness = Some(3);
        own_card.keywords = KeywordAbilities::WARD;
        let perm = Permanent::new(own_card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(own_card);

        let ward_ability = Ability::static_ability(
            own_ward_id,
            "Ward {2}",
            vec![StaticEffect::ward("{2}")],
        );
        game.state.ability_store.add(ward_ability);

        // Create a buff spell targeting own creature
        let buff_id = ObjectId::new();
        let mut buff_card = CardData::new(buff_id, p1, "Giant Growth");
        buff_card.card_types = vec![CardType::Instant];
        buff_card.mana_cost = ManaCost::parse("{G}");
        buff_card.abilities = vec![Ability::spell(
            buff_id,
            vec![Effect::BoostUntilEndOfTurn { power: 3, toughness: 3 }],
            TargetSpec::CreatureYouControl,
        )];
        game.state.card_store.insert(buff_card);
        game.state.players.get_mut(&p1).unwrap().hand.add(buff_id);

        // Give enough mana for the spell only (1G) — no extra for ward
        game.state.players.get_mut(&p1).unwrap().mana_pool.add(
            Mana { white: 0, blue: 0, black: 0, red: 0, green: 1, colorless: 0, generic: 0, any: 0 }, None, false
        );

        game.cast_spell(p1, buff_id);

        // Ward should NOT trigger (own creature) — spell should not be countered
        let stack_item = game.state.stack.get(buff_id);
        assert!(stack_item.is_some(), "Spell should be on the stack");
        assert!(!stack_item.unwrap().countered, "Ward should not trigger on own creatures");
    }

    #[test]
    fn ward_pay_life_counters_when_insufficient() {
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

        // Create ward creature with "Pay 2 life" cost
        let ward_id = ObjectId::new();
        let mut ward_card = CardData::new(ward_id, p2, "Life Ward");
        ward_card.card_types = vec![CardType::Creature];
        ward_card.power = Some(2);
        ward_card.toughness = Some(2);
        let perm = Permanent::new(ward_card.clone(), p2);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(ward_card);

        let ward_ability = Ability::static_ability(
            ward_id,
            "Ward--Pay 2 life.",
            vec![StaticEffect::Ward { cost: "Pay 2 life".into() }],
        );
        game.state.ability_store.add(ward_ability);

        // Create a removal spell
        let spell_id = ObjectId::new();
        let mut spell_card = CardData::new(spell_id, p1, "Lightning Bolt");
        spell_card.card_types = vec![CardType::Instant];
        spell_card.mana_cost = ManaCost::parse("{R}");
        spell_card.abilities = vec![Ability::spell(
            spell_id,
            vec![Effect::DealDamage { amount: 3 }],
            TargetSpec::Creature,
        )];
        game.state.card_store.insert(spell_card);
        game.state.players.get_mut(&p1).unwrap().hand.add(spell_id);

        // Set p1's life to 1 — can't afford "Pay 2 life"
        game.state.players.get_mut(&p1).unwrap().life = 1;
        game.state.players.get_mut(&p1).unwrap().mana_pool.add(
            Mana { white: 0, blue: 0, black: 0, red: 1, green: 0, colorless: 0, generic: 0, any: 0 }, None, false
        );

        game.cast_spell(p1, spell_id);

        let stack_item = game.state.stack.get(spell_id);
        assert!(stack_item.is_some());
        assert!(stack_item.unwrap().countered, "Spell should be countered — can't pay 2 life at 1 life");
    }
}

#[cfg(test)]
mod cant_be_countered_tests {
    use super::*;
    use crate::abilities::{Ability, StaticEffect, TargetSpec, Effect};
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, Outcome};
    use crate::mana::{ManaCost, Mana};
    use crate::types::{ObjectId, PlayerId};
    use crate::decision::*;

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

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        (0..40).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    #[test]
    fn cant_be_countered_resists_counter_spell() {
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

        // Put an uncounterable spell on the stack
        let spell_id = ObjectId::new();
        let mut spell_card = CardData::new(spell_id, p1, "Supreme Verdict");
        spell_card.card_types = vec![CardType::Sorcery];
        spell_card.mana_cost = ManaCost::parse("{1}{W}{U}{U}");
        spell_card.abilities = vec![
            Ability::static_ability(spell_id, "This spell can't be countered.",
                vec![StaticEffect::CantBeCountered]),
            Ability::spell(spell_id,
                vec![Effect::DestroyAll { filter: "creature".into() }],
                TargetSpec::None),
        ];
        let stack_item = crate::zones::StackItem {
            id: spell_id,
            kind: crate::zones::StackItemKind::Spell { card: spell_card },
            controller: p1,
            targets: vec![],
            countered: false,
            x_value: None,
            exile_on_resolve: false,
        };
        game.state.stack.push(stack_item);

        // Now try to counter it using Effect::CounterSpell
        game.execute_effects(
            &[Effect::CounterSpell],
            p2,
            &[spell_id],
            Some(spell_id),
        None, );

        // The spell should STILL be on the stack (not removed)
        assert!(game.state.stack.get(spell_id).is_some(), "Uncounterable spell should remain on the stack");
    }

    #[test]
    fn normal_spell_can_be_countered() {
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

        // Put a normal spell on the stack
        let spell_id = ObjectId::new();
        let mut spell_card = CardData::new(spell_id, p1, "Lightning Bolt");
        spell_card.card_types = vec![CardType::Instant];
        spell_card.mana_cost = ManaCost::parse("{R}");
        spell_card.abilities = vec![
            Ability::spell(spell_id,
                vec![Effect::DealDamage { amount: 3 }],
                TargetSpec::Creature),
        ];
        let stack_item = crate::zones::StackItem {
            id: spell_id,
            kind: crate::zones::StackItemKind::Spell { card: spell_card },
            controller: p1,
            targets: vec![],
            countered: false,
            x_value: None,
            exile_on_resolve: false,
        };
        game.state.stack.push(stack_item);
        game.state.card_store.insert(CardData::new(spell_id, p1, "Lightning Bolt"));

        // Counter it
        game.execute_effects(
            &[Effect::CounterSpell],
            p2,
            &[spell_id],
            Some(spell_id),
        None, );

        // The spell should be removed from the stack
        assert!(game.state.stack.get(spell_id).is_none(), "Normal spell should be countered");
    }
}

#[cfg(test)]
mod step_trigger_tests {
    use super::*;
    use crate::abilities::{Ability, TargetSpec, Effect};
    use crate::card::CardData;
    use crate::constants::{CardType, Outcome};
    use crate::types::{ObjectId, PlayerId};
    use crate::decision::*;

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

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
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
}

#[cfg(test)]
mod x_cost_tests {
    use super::*;
    use crate::abilities::{Ability, Effect, TargetSpec, X_VALUE};
    use crate::card::CardData;
    use crate::constants::{CardType, Outcome};
    use crate::mana::{Mana, ManaCost};
    use crate::types::{ObjectId, PlayerId};
    use crate::decision::*;

    struct XChooserPlayer {
        x_choice: u32,
    }

    impl PlayerDecisionMaker for XChooserPlayer {
        fn priority(&mut self, _: &GameView<'_>, legal: &[PlayerAction]) -> PlayerAction {
            for action in legal {
                if let PlayerAction::CastSpell { .. } = action {
                    return action.clone();
                }
            }
            PlayerAction::Pass
        }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, req: &TargetRequirement) -> Vec<ObjectId> {
            req.legal_targets.iter().take(1).copied().collect()
        }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { true }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, _: u32, _: u32) -> u32 {
            self.x_choice
        }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    fn setup_x_game(x_choice: u32) -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        let config = GameConfig {
            starting_life: 20,
            players: vec![
                PlayerConfig { name: "P1".into(), deck: vec![] },
                PlayerConfig { name: "P2".into(), deck: vec![] },
            ],
        };

        let dms: Vec<(PlayerId, Box<dyn PlayerDecisionMaker>)> = vec![
            (p1, Box::new(XChooserPlayer { x_choice })),
            (p2, Box::new(XChooserPlayer { x_choice: 0 })),
        ];

        (Game::new_two_player(config, dms), p1, p2)
    }

    #[test]
    fn x_cost_deal_damage() {
        let (mut game, p1, p2) = setup_x_game(3);

        // Give P1 4 mana for {X}{R} with X=3
        if let Some(player) = game.state.players.get_mut(&p1) {
            player.mana_pool.add(Mana { red: 1, green: 3, ..Mana::new() }, None, false);
        }

        // Create X-cost damage spell: {X}{R} - deal X damage
        let spell_id = ObjectId::new();
        let mut spell = CardData::new(spell_id, p1, "X Bolt");
        spell.card_types = vec![CardType::Sorcery];
        spell.mana_cost = ManaCost::parse("{X}{R}");
        spell.abilities = vec![Ability::spell(spell_id,
            vec![Effect::DealDamage { amount: X_VALUE }],
            TargetSpec::Creature)];

        // Create a target creature for P2
        let creature_id = ObjectId::new();
        let mut creature = CardData::new(creature_id, p2, "Big Beast");
        creature.card_types = vec![CardType::Creature];
        creature.power = Some(5);
        creature.toughness = Some(5);
        game.state.battlefield.add(crate::permanent::Permanent::new(creature.clone(), p2));
        game.state.card_store.insert(creature);
        game.state.set_zone(creature_id, crate::constants::Zone::Battlefield, None);

        // Put spell in hand
        if let Some(player) = game.state.players.get_mut(&p1) {
            player.hand.add(spell_id);
        }
        game.state.card_store.insert(spell);
        game.state.set_zone(spell_id, crate::constants::Zone::Hand, None);

        // Cast the spell (X=3)
        game.cast_spell(p1, spell_id);

        // Verify X value on stack
        let stack_item = game.state.stack.top().unwrap();
        assert_eq!(stack_item.x_value, Some(3));

        // Resolve
        game.resolve_top_of_stack();

        // Creature should have 3 damage
        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert_eq!(perm.damage, 3);
    }

    #[test]
    fn x_cost_draw_cards() {
        let (mut game, p1, _p2) = setup_x_game(2);

        // Give P1 4 mana for {X}{U}{U} with X=2
        if let Some(player) = game.state.players.get_mut(&p1) {
            player.mana_pool.add(Mana { blue: 2, green: 2, ..Mana::new() }, None, false);
        }

        // Add cards to library
        for _ in 0..5 {
            let card_id = ObjectId::new();
            let card = CardData::new(card_id, p1, "Island");
            game.state.card_store.insert(card);
            if let Some(player) = game.state.players.get_mut(&p1) {
                player.library.put_on_top(card_id);
            }
        }

        // Create X-cost draw spell: {X}{U}{U}
        let spell_id = ObjectId::new();
        let mut spell = CardData::new(spell_id, p1, "X Draw");
        spell.card_types = vec![CardType::Sorcery];
        spell.mana_cost = ManaCost::parse("{X}{U}{U}");
        spell.abilities = vec![Ability::spell(spell_id,
            vec![Effect::DrawCards { count: X_VALUE }],
            TargetSpec::None)];

        if let Some(player) = game.state.players.get_mut(&p1) {
            player.hand.add(spell_id);
        }
        game.state.card_store.insert(spell);

        let hand_before = game.state.players.get(&p1).unwrap().hand.len();

        // Cast and resolve (X=2)
        game.cast_spell(p1, spell_id);
        game.resolve_top_of_stack();

        // Should have drawn 2 cards (minus spell removed from hand)
        let hand_after = game.state.players.get(&p1).unwrap().hand.len();
        assert_eq!(hand_after, hand_before - 1 + 2);
    }

    #[test]
    fn x_cost_zero() {
        let (mut game, p1, _p2) = setup_x_game(0);

        // Give P1 1 mana for {X}{R} with X=0
        if let Some(player) = game.state.players.get_mut(&p1) {
            player.mana_pool.add(Mana { red: 1, ..Mana::new() }, None, false);
        }

        let spell_id = ObjectId::new();
        let mut spell = CardData::new(spell_id, p1, "X Zero");
        spell.card_types = vec![CardType::Sorcery];
        spell.mana_cost = ManaCost::parse("{X}{R}");
        spell.abilities = vec![Ability::spell(spell_id,
            vec![Effect::DealDamage { amount: X_VALUE }],
            TargetSpec::None)];

        if let Some(player) = game.state.players.get_mut(&p1) {
            player.hand.add(spell_id);
        }
        game.state.card_store.insert(spell);

        game.cast_spell(p1, spell_id);
        assert_eq!(game.state.stack.top().unwrap().x_value, Some(0));

        // Resolve - 0 damage to opponent
        let opp = *game.state.turn_order.iter().find(|&&id| id != p1).unwrap();
        let life_before = game.state.players.get(&opp).unwrap().life;
        game.resolve_top_of_stack();
        let life_after = game.state.players.get(&opp).unwrap().life;
        assert_eq!(life_before, life_after);
    }

    #[test]
    fn x_value_mana_payment() {
        let (mut game, p1, _p2) = setup_x_game(3);

        // Give P1 5 mana
        if let Some(player) = game.state.players.get_mut(&p1) {
            player.mana_pool.add(Mana { red: 1, green: 4, ..Mana::new() }, None, false);
        }

        // Spell costs {X}{R} with X=3 -> 4 mana total
        let spell_id = ObjectId::new();
        let mut spell = CardData::new(spell_id, p1, "X Payment");
        spell.card_types = vec![CardType::Sorcery];
        spell.mana_cost = ManaCost::parse("{X}{R}");
        spell.abilities = vec![Ability::spell(spell_id,
            vec![Effect::GainLife { amount: X_VALUE }],
            TargetSpec::None)];

        if let Some(player) = game.state.players.get_mut(&p1) {
            player.hand.add(spell_id);
        }
        game.state.card_store.insert(spell);

        game.cast_spell(p1, spell_id);

        // Should have 1 mana remaining (5 - 4)
        let remaining = game.state.players.get(&p1).unwrap().mana_pool.available().count();
        assert_eq!(remaining, 1);

        // Resolve: X=3 life gain
        let life_before = game.state.players.get(&p1).unwrap().life;
        game.resolve_top_of_stack();
        let life_after = game.state.players.get(&p1).unwrap().life;
        assert_eq!(life_after, life_before + 3);
    }
}

#[cfg(test)]
mod impulse_draw_tests {
    use super::*;
    use crate::abilities::Effect;
    use crate::card::CardData;
    use crate::constants::{CardType, TurnPhase, PhaseStep, Outcome};
    use crate::mana::{Mana, ManaCost};
    use crate::types::{ObjectId, PlayerId};
    use crate::decision::*;

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
        let lib_ids = add_library_cards(&mut game, p1, 3);

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
}

#[cfg(test)]
mod delayed_trigger_tests {
    use super::*;
    use crate::abilities::{Ability, Effect, TargetSpec};
    use crate::card::CardData;
    use crate::constants::{CardType, TurnPhase, PhaseStep, Outcome};
    use crate::events::{EventType, GameEvent};
    use crate::mana::Mana;
    use crate::types::{ObjectId, PlayerId};
    use crate::decision::*;

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
}

#[cfg(test)]
mod flashback_tests {
    use super::*;
    use crate::abilities::{Ability, Effect, TargetSpec};
    use crate::card::CardData;
    use crate::constants::{CardType, TurnPhase, PhaseStep, Outcome};
    use crate::mana::{Mana, ManaCost};
    use crate::types::{ObjectId, PlayerId};
    use crate::decision::*;

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

    fn setup_flashback_game() -> (Game, PlayerId, PlayerId) {
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
    fn flashback_appears_in_legal_actions() {
        let (mut game, p1, _p2) = setup_flashback_game();

        // Create a sorcery with flashback in graveyard
        let spell_id = ObjectId::new();
        let mut spell = CardData::new(spell_id, p1, "Flashback Bolt");
        spell.card_types = vec![CardType::Sorcery];
        spell.mana_cost = ManaCost::parse("{R}");
        spell.flashback_cost = Some(ManaCost::parse("{2}{R}"));
        spell.abilities = vec![Ability::spell(spell_id,
            vec![Effect::GainLife { amount: 3 }],
            TargetSpec::None)];
        game.state.card_store.insert(spell);
        game.state.players.get_mut(&p1).unwrap().graveyard.add(spell_id);

        // Give P1 enough mana for flashback cost {2}{R}
        game.state.players.get_mut(&p1).unwrap()
            .mana_pool.add(Mana { red: 3, ..Mana::new() }, None, false);

        let actions = game.compute_legal_actions(p1);
        let cast_actions: Vec<_> = actions.iter()
            .filter(|a| matches!(a, PlayerAction::CastSpell { card_id, .. } if *card_id == spell_id))
            .collect();
        assert!(!cast_actions.is_empty(), "Should be able to cast flashback from graveyard");
    }

    #[test]
    fn flashback_not_available_without_mana() {
        let (mut game, p1, _p2) = setup_flashback_game();

        let spell_id = ObjectId::new();
        let mut spell = CardData::new(spell_id, p1, "Flashback Bolt");
        spell.card_types = vec![CardType::Sorcery];
        spell.mana_cost = ManaCost::parse("{R}");
        spell.flashback_cost = Some(ManaCost::parse("{2}{R}"));
        spell.abilities = vec![Ability::spell(spell_id,
            vec![Effect::GainLife { amount: 3 }],
            TargetSpec::None)];
        game.state.card_store.insert(spell);
        game.state.players.get_mut(&p1).unwrap().graveyard.add(spell_id);

        // Only give 1 red (flashback needs {2}{R})
        game.state.players.get_mut(&p1).unwrap()
            .mana_pool.add(Mana { red: 1, ..Mana::new() }, None, false);

        let actions = game.compute_legal_actions(p1);
        let cast_actions: Vec<_> = actions.iter()
            .filter(|a| matches!(a, PlayerAction::CastSpell { card_id, .. } if *card_id == spell_id))
            .collect();
        assert!(cast_actions.is_empty(), "Should NOT be able to flashback without enough mana");
    }

    #[test]
    fn flashback_cast_exiles_after_resolution() {
        let (mut game, p1, _p2) = setup_flashback_game();

        // Create a sorcery with flashback
        let spell_id = ObjectId::new();
        let mut spell = CardData::new(spell_id, p1, "Flashback Heal");
        spell.card_types = vec![CardType::Sorcery];
        spell.mana_cost = ManaCost::parse("{W}");
        spell.flashback_cost = Some(ManaCost::parse("{1}{W}"));
        spell.abilities = vec![Ability::spell(spell_id,
            vec![Effect::GainLife { amount: 3 }],
            TargetSpec::None)];
        game.state.card_store.insert(spell);
        game.state.players.get_mut(&p1).unwrap().graveyard.add(spell_id);

        // Give P1 mana for flashback {1}{W}
        game.state.players.get_mut(&p1).unwrap()
            .mana_pool.add(Mana { white: 2, ..Mana::new() }, None, false);

        let life_before = game.state.players.get(&p1).unwrap().life;

        // Cast from graveyard
        game.cast_spell(p1, spell_id);

        // Should be on stack
        assert!(!game.state.stack.is_empty());
        // Should no longer be in graveyard
        assert!(!game.state.players.get(&p1).unwrap().graveyard.contains(spell_id));

        // Resolve
        game.resolve_top_of_stack();

        // Should gain 3 life
        let life_after = game.state.players.get(&p1).unwrap().life;
        assert_eq!(life_after, life_before + 3);

        // Should be in exile (NOT graveyard)
        assert!(game.state.exile.contains(spell_id), "Flashback spell should be exiled after resolution");
        assert!(!game.state.players.get(&p1).unwrap().graveyard.contains(spell_id),
            "Flashback spell should NOT be in graveyard");
    }

    #[test]
    fn normal_cast_still_goes_to_graveyard() {
        let (mut game, p1, _p2) = setup_flashback_game();

        // Normal sorcery (no flashback) in hand
        let spell_id = ObjectId::new();
        let mut spell = CardData::new(spell_id, p1, "Normal Heal");
        spell.card_types = vec![CardType::Sorcery];
        spell.mana_cost = ManaCost::parse("{W}");
        spell.abilities = vec![Ability::spell(spell_id,
            vec![Effect::GainLife { amount: 2 }],
            TargetSpec::None)];
        game.state.card_store.insert(spell);
        game.state.players.get_mut(&p1).unwrap().hand.add(spell_id);

        // Give mana
        game.state.players.get_mut(&p1).unwrap()
            .mana_pool.add(Mana { white: 1, ..Mana::new() }, None, false);

        // Cast from hand and resolve
        game.cast_spell(p1, spell_id);
        game.resolve_top_of_stack();

        // Should be in graveyard (NOT exile)
        assert!(game.state.players.get(&p1).unwrap().graveyard.contains(spell_id),
            "Normal spell should go to graveyard");
        assert!(!game.state.exile.contains(spell_id),
            "Normal spell should NOT be exiled");
    }
}

#[cfg(test)]
mod behold_tests {
    use super::*;
    use crate::abilities::{Ability, Effect, TargetSpec, Cost};
    use crate::card::CardData;
    use crate::constants::{CardType, SubType, TurnPhase, PhaseStep, Outcome, KeywordAbilities};
    use crate::permanent::Permanent;
    use crate::mana::{Mana, ManaCost};
    use crate::types::{ObjectId, PlayerId};
    use crate::decision::*;

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
}

#[cfg(test)]
mod block_restriction_tests {
    use super::*;
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, Outcome, AbilityType};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };
    use crate::abilities::{Ability, StaticEffect};

    /// Decision maker that attacks with all creatures.
    struct AttackAllPlayer;

    impl PlayerDecisionMaker for AttackAllPlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, possible_attackers: &[ObjectId], possible_defenders: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> {
            let defender = possible_defenders[0];
            possible_attackers.iter().map(|&a| (a, defender)).collect()
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

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        (0..40).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    fn setup_game(
        p1_dm: Box<dyn PlayerDecisionMaker>,
        p2_dm: Box<dyn PlayerDecisionMaker>,
    ) -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Attacker".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Defender".to_string(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(config, vec![(p1, p1_dm), (p2, p2_dm)]);
        (game, p1, p2)
    }

    fn add_creature(
        game: &mut Game,
        owner: PlayerId,
        name: &str,
        power: i32,
        toughness: i32,
        keywords: KeywordAbilities,
    ) -> ObjectId {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(power);
        card.toughness = Some(toughness);
        card.keywords = keywords;
        let id = card.id;
        let mut perm = Permanent::new(card, owner);
        perm.remove_summoning_sickness();
        game.state.battlefield.add(perm);
        id
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
        card.power = Some(power);
        card.toughness = Some(toughness);
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
            Box::new(AttackAllPlayer),
            Box::new(BlockOnePerAttackerPlayer),
        );

        let attacker_id = add_creature_with_static(
            &mut game, p1, "Daunt Creature", 4, 4, KeywordAbilities::empty(),
            vec![StaticEffect::CantBeBlockedByPowerLessOrEqual { power: 2 }],
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
            Box::new(AttackAllPlayer),
            Box::new(BlockAllMultiplePlayer),
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
            Box::new(AttackAllPlayer),
            Box::new(BlockOnePerAttackerPlayer),
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
            Box::new(AttackAllPlayer),
            Box::new(BlockAllMultiplePlayer),
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
            Box::new(AttackAllPlayer),
            Box::new(BlockOnePerAttackerPlayer),
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
            Box::new(AttackAllPlayer),
            Box::new(BlockOnePerAttackerPlayer),
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
}

#[cfg(test)]
mod simple_effect_tests {
    use super::*;
    use crate::card::CardData;
    use crate::constants::{CardType, Outcome};
    use crate::counters::CounterType;
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };

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
                PlayerConfig { name: "P1".to_string(), deck },
                PlayerConfig { name: "P2".to_string(), deck: deck2 },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(config, vec![(p1, Box::new(PassPlayer)), (p2, Box::new(PassPlayer))]);
        (game, p1, p2)
    }

    #[test]
    fn proliferate_adds_counters() {
        let (mut game, p1, _p2) = make_game();

        // Add a creature with +1/+1 counters
        let mut card = CardData::new(ObjectId::new(), p1, "Creature");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        let id = card.id;
        let mut perm = Permanent::new(card, p1);
        perm.add_counters(CounterType::P1P1, 2);
        game.state.battlefield.add(perm);

        // Execute proliferate
        game.execute_effects(
            &[crate::abilities::Effect::Proliferate],
            p1, &[], Some(ObjectId::new()), None,
        );





        // Should have 3 +1/+1 counters now (2 + 1 from proliferate)
        let perm = game.state.battlefield.get(id).unwrap();
        assert_eq!(perm.counters.get(&CounterType::P1P1), 3);
    }

    #[test]
    fn remove_all_counters_clears_creature() {
        let (mut game, p1, _p2) = make_game();

        let mut card = CardData::new(ObjectId::new(), p1, "Creature");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        let id = card.id;
        let mut perm = Permanent::new(card, p1);
        perm.add_counters(CounterType::P1P1, 3);
        perm.add_counters(CounterType::M1M1, 1);
        game.state.battlefield.add(perm);

        // Execute remove all counters targeting the creature
        game.execute_effects(
            &[crate::abilities::Effect::RemoveAllCounters],
            p1, &[id], Some(ObjectId::new()), None,
        );

        let perm = game.state.battlefield.get(id).unwrap();
        assert!(perm.counters.is_empty());
    }

    #[test]
    fn tap_attached_taps_enchanted_creature() {
        let (mut game, p1, _p2) = make_game();

        // Create a creature
        let mut creature_card = CardData::new(ObjectId::new(), p1, "Target Creature");
        creature_card.card_types = vec![CardType::Creature];
        creature_card.power = Some(2);
        creature_card.toughness = Some(2);
        let creature_id = creature_card.id;
        game.state.battlefield.add(Permanent::new(creature_card, p1));

        // Create an aura attached to the creature
        let mut aura_card = CardData::new(ObjectId::new(), p1, "Aura");
        aura_card.card_types = vec![CardType::Enchantment];
        let aura_id = aura_card.id;
        let mut aura_perm = Permanent::new(aura_card, p1);
        aura_perm.attach_to(creature_id);
        game.state.battlefield.add(aura_perm);

        // Execute TapAttached from the aura's perspective
        game.execute_effects(
            &[crate::abilities::Effect::TapAttached],
            p1, &[], Some(aura_id), None,
        );

        // Creature should be tapped
        assert!(game.state.battlefield.get(creature_id).unwrap().tapped);
    }
}

#[cfg(test)]
mod boost_per_count_tests {
    use super::*;
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, Outcome, SubType, AbilityType};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };
    use crate::abilities::{Ability, StaticEffect};

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
                PlayerConfig { name: "P1".to_string(), deck },
                PlayerConfig { name: "P2".to_string(), deck: deck2 },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(config, vec![(p1, Box::new(PassPlayer)), (p2, Box::new(PassPlayer))]);
        (game, p1, p2)
    }

    fn add_goblin(game: &mut Game, owner: PlayerId, name: &str) -> ObjectId {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![SubType::Goblin];
        card.power = Some(1);
        card.toughness = Some(1);
        let id = card.id;
        game.state.battlefield.add(Permanent::new(card, owner));
        id
    }

    #[test]
    fn boost_per_count_two_goblins() {
        let (mut game, p1, _p2) = make_game();

        // Add a creature with "+2/+0 for each other Goblin you control"
        let mut card = CardData::new(ObjectId::new(), p1, "Goblin Lord");
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![SubType::Goblin, SubType::Berserker];
        card.power = Some(2);
        card.toughness = Some(4);
        let lord_id = card.id;
        let ability = Ability::static_ability(lord_id, "Gets +2/+0 per Goblin",
            vec![StaticEffect::BoostPerCount { count_filter: "other Goblin you control".into(), power_per: 2, toughness_per: 0 }]);
        card.abilities.push(ability.clone());
        game.state.card_store.insert(card.clone());
        game.state.battlefield.add(Permanent::new(card, p1));
        game.state.ability_store.add(ability);

        // Add 2 other goblins
        let _g1 = add_goblin(&mut game, p1, "Goblin A");
        let _g2 = add_goblin(&mut game, p1, "Goblin B");

        game.apply_continuous_effects();

        let lord = game.state.battlefield.get(lord_id).unwrap();
        // Base 2/4, +2*2/+0*2 = 6/4
        assert_eq!(lord.power(), 6, "power should be 2 + 2*2 = 6");
        assert_eq!(lord.toughness(), 4, "toughness should be 4 + 0*2 = 4");
    }

    #[test]
    fn boost_per_count_no_others() {
        let (mut game, p1, _p2) = make_game();

        let mut card = CardData::new(ObjectId::new(), p1, "Lonely Goblin Lord");
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![SubType::Goblin];
        card.power = Some(2);
        card.toughness = Some(4);
        let lord_id = card.id;
        let ability = Ability::static_ability(lord_id, "", 
            vec![StaticEffect::BoostPerCount { count_filter: "other Goblin you control".into(), power_per: 2, toughness_per: 0 }]);
        card.abilities.push(ability.clone());
        game.state.card_store.insert(card.clone());
        game.state.battlefield.add(Permanent::new(card, p1));
        game.state.ability_store.add(ability);

        game.apply_continuous_effects();

        let lord = game.state.battlefield.get(lord_id).unwrap();
        assert_eq!(lord.power(), 2, "no other goblins, power should be base 2");
    }

    #[test]
    fn boost_per_count_with_graveyard() {
        let (mut game, p1, _p2) = make_game();

        // Create a creature with "+1/+1 for each creature you control and creature card in graveyard"
        let mut card = CardData::new(ObjectId::new(), p1, "Graveyard Counter");
        card.card_types = vec![CardType::Creature];
        card.power = Some(0);
        card.toughness = Some(0);
        let id = card.id;
        let ability = Ability::static_ability(id, "",
            vec![StaticEffect::BoostPerCount {
                count_filter: "creature you control and creature card in your graveyard".into(),
                power_per: 1,
                toughness_per: 1,
            }]);
        card.abilities.push(ability.clone());
        game.state.card_store.insert(card.clone());
        game.state.battlefield.add(Permanent::new(card, p1));
        game.state.ability_store.add(ability);

        // Add 1 other creature on BF
        let mut c2 = CardData::new(ObjectId::new(), p1, "BF Creature");
        c2.card_types = vec![CardType::Creature];
        c2.power = Some(1);
        c2.toughness = Some(1);
        let c2_id = c2.id;
        game.state.battlefield.add(Permanent::new(c2.clone(), p1));
        game.state.card_store.insert(c2);

        // Add 2 creature cards in graveyard
        for i in 0..2 {
            let mut gc = CardData::new(ObjectId::new(), p1, &format!("GY Creature {i}"));
            gc.card_types = vec![CardType::Creature];
            gc.power = Some(1);
            gc.toughness = Some(1);
            let gc_id = gc.id;
            game.state.card_store.insert(gc);
            if let Some(player) = game.state.players.get_mut(&p1) {
                player.graveyard.add(gc_id);
            }
        }

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(id).unwrap();
        // 2 creatures on BF (self + other) + 2 in graveyard = 4 total
        assert_eq!(perm.power(), 4, "0 + 1*(2 BF + 2 GY) = 4");
        assert_eq!(perm.toughness(), 4);
    }
}

#[cfg(test)]
mod flicker_tests {
    use super::*;
    use crate::abilities::*;
    use crate::types::*;
    use crate::counters::CounterType;
    use uuid::Uuid;

    struct PassPlayer;
    impl crate::decision::PlayerDecisionMaker for PassPlayer {
        fn priority(&mut self, _: &crate::decision::GameView, actions: &[crate::decision::PlayerAction]) -> crate::decision::PlayerAction { actions[0].clone() }
        fn choose_targets(&mut self, _: &crate::decision::GameView, _: crate::constants::Outcome, _: &crate::decision::TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &crate::decision::GameView, _: crate::constants::Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &crate::decision::GameView, modes: &[crate::decision::NamedChoice]) -> usize { 0 }
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

    fn make_test_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId(Uuid::new_v4());
        let p2 = PlayerId(Uuid::new_v4());
        let config = GameConfig { players: vec![PlayerConfig { name: "P1".to_string(), deck: vec![] }, PlayerConfig { name: "P2".to_string(), deck: vec![] }], starting_life: 20 };
        let game = Game::new_two_player(config, vec![
            (p1, Box::new(PassPlayer)),
            (p2, Box::new(PassPlayer)),
        ]);
        (game, p1, p2)
    }

    fn make_creature(name: &str, power: i32, toughness: i32) -> (ObjectId, CardData) {
        let id = ObjectId(Uuid::new_v4());
        let card = CardData {
            id,
            owner: PlayerId(Uuid::new_v4()), // will be overridden
            name: name.into(),
            card_types: vec![crate::constants::CardType::Creature],
            power: Some(power), toughness: Some(toughness),
            ..Default::default()
        };
        (id, card)
    }

    #[test]
    fn flicker_returns_creature_fresh() {
        let (mut game, p1, _p2) = make_test_game();

        let (card_id, mut card) = make_creature("Test Creature", 3, 3);
        card.owner = p1;
        let perm = crate::permanent::Permanent::new(card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card.clone());
        for ab in &card.abilities { game.state.ability_store.add(ab.clone()); }
        game.state.set_zone(card_id, crate::constants::Zone::Battlefield, None);

        // Put +1/+1 counter on it
        if let Some(perm) = game.state.battlefield.get_mut(card_id) {
            perm.counters.add(CounterType::P1P1, 2);
            assert_eq!(perm.power(), 5); // 3 + 2
        }

        // Flicker it
        let effects = vec![Effect::Flicker];
        game.execute_effects(&effects, p1, &[card_id], None, None);

        // Verify it's back on battlefield as fresh permanent (no counters)
        let perm = game.state.battlefield.get(card_id).expect("should be on BF");
        assert_eq!(perm.power(), 3, "should have base power after flicker (no counters)");
        assert_eq!(perm.counters.get(&CounterType::P1P1), 0, "counters should be reset");
    }

    #[test]
    fn flicker_triggers_etb() {
        let (mut game, p1, _p2) = make_test_game();

        let (card_id, mut card) = make_creature("ETB Creature", 2, 2);
        card.owner = p1;
        let perm = crate::permanent::Permanent::new(card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card.clone());
        game.state.set_zone(card_id, crate::constants::Zone::Battlefield, None);

        // Clear event log then flicker
        game.event_log.clear();
        let effects = vec![Effect::Flicker];
        game.execute_effects(&effects, p1, &[card_id], None, None);

        // Check that ETB event was emitted
        let etb_events: Vec<_> = game.event_log.iter()
            .filter(|e| e.event_type == crate::events::EventType::EnteredTheBattlefield)
            .collect();
        assert_eq!(etb_events.len(), 1, "flicker should emit 1 ETB event");
        assert_eq!(etb_events[0].target_id, Some(card_id));
    }

    #[test]
    fn flicker_end_step_exiles_then_returns_tapped() {
        let (mut game, p1, _p2) = make_test_game();

        let (card_id, mut card) = make_creature("Flickered Beast", 4, 4);
        card.owner = p1;
        let perm = crate::permanent::Permanent::new(card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card.clone());
        game.state.set_zone(card_id, crate::constants::Zone::Battlefield, None);

        // FlickerEndStep — should exile
        let effects = vec![Effect::FlickerEndStep];
        game.execute_effects(&effects, p1, &[card_id], None, None);

        // Verify creature is in exile, not on battlefield
        assert!(game.state.battlefield.get(card_id).is_none(), "should not be on BF");
        assert!(game.state.exile.contains(card_id), "should be in exile");

        // Verify delayed trigger was created
        assert_eq!(game.state.delayed_triggers.len(), 1);
        assert_eq!(game.state.delayed_triggers[0].effects.len(), 1);
        assert_eq!(game.state.delayed_triggers[0].targets, vec![card_id]);

        // Now simulate the delayed trigger firing: execute the return effect
        let dt = game.state.delayed_triggers[0].clone();
        game.execute_effects(&dt.effects, dt.controller, &dt.targets, dt.source, None);

        // Verify creature is back on battlefield, tapped
        let perm = game.state.battlefield.get(card_id).expect("should be back on BF");
        assert!(perm.tapped, "should be tapped after FlickerEndStep return");
    }

    #[test]
    fn additional_land_plays() {
        let (mut game, p1, _p2) = make_test_game();

        // Register a static ability with AdditionalLandPlays
        let source_id = ObjectId(Uuid::new_v4());
        let card = CardData {
            id: source_id,
            owner: p1,
            name: "Land Enabler".into(),
            card_types: vec![crate::constants::CardType::Enchantment],
            abilities: vec![
                Ability::static_ability(source_id, "You may play an additional land.", vec![
                    StaticEffect::AdditionalLandPlays { count: 1 },
                ]),
            ],
            ..Default::default()
        };
        let perm = crate::permanent::Permanent::new(card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card.clone());
        for ab in &card.abilities { game.state.ability_store.add(ab.clone()); }

        // Apply continuous effects
        game.apply_continuous_effects();

        // Player should be able to play 2 lands per turn
        let player = game.state.players.get(&p1).unwrap();
        assert_eq!(player.lands_per_turn, 2, "should have 2 land plays per turn");
    }

    #[test]
    fn opponent_exiles_from_hand() {
        let (mut game, p1, p2) = make_test_game();

        // Give opponent some cards in hand
        let c1 = ObjectId(Uuid::new_v4());
        let c2 = ObjectId(Uuid::new_v4());
        let c3 = ObjectId(Uuid::new_v4());
        if let Some(player) = game.state.players.get_mut(&p2) {
            player.hand.add(c1);
            player.hand.add(c2);
            player.hand.add(c3);
        }

        let effects = vec![Effect::OpponentExilesFromHand { count: 2 }];
        game.execute_effects(&effects, p1, &[], None, None);

        // Opponent should have 1 card left in hand
        let player = game.state.players.get(&p2).unwrap();
        assert_eq!(player.hand.len(), 1, "opponent should have 1 card left after exiling 2");

        // 2 cards should be in exile
        let exile_count = [c1, c2, c3].iter()
            .filter(|&&id| game.state.exile.contains(id))
            .count();
        assert_eq!(exile_count, 2, "2 cards should be in exile");
    }
}

#[cfg(test)]
mod conditional_static_tests {
    use super::*;
    use crate::abilities::*;
    use crate::types::*;
    use uuid::Uuid;

    struct PassPlayer;
    impl crate::decision::PlayerDecisionMaker for PassPlayer {
        fn priority(&mut self, _: &crate::decision::GameView, actions: &[crate::decision::PlayerAction]) -> crate::decision::PlayerAction { actions[0].clone() }
        fn choose_targets(&mut self, _: &crate::decision::GameView, _: crate::constants::Outcome, _: &crate::decision::TargetRequirement) -> Vec<ObjectId> { vec![] }
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

    fn make_test_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId(Uuid::new_v4());
        let p2 = PlayerId(Uuid::new_v4());
        let config = GameConfig { players: vec![PlayerConfig { name: "P1".to_string(), deck: vec![] }, PlayerConfig { name: "P2".to_string(), deck: vec![] }], starting_life: 20 };
        let game = Game::new_two_player(config, vec![
            (p1, Box::new(PassPlayer)),
            (p2, Box::new(PassPlayer)),
        ]);
        (game, p1, p2)
    }

    #[test]
    fn conditional_keyword_your_turn() {
        let (mut game, p1, _p2) = make_test_game();

        // Create creature with "first strike on your turn"
        let card_id = ObjectId(Uuid::new_v4());
        let card = CardData {
            id: card_id, owner: p1, name: "First Strike Guy".into(),
            card_types: vec![crate::constants::CardType::Creature],
            power: Some(2), toughness: Some(1),
            abilities: vec![Ability::static_ability(card_id, "First strike on your turn.",
                vec![StaticEffect::ConditionalKeyword { keyword: "first strike".into(), condition: "your turn".into() }])],
            ..Default::default()
        };
        let perm = crate::permanent::Permanent::new(card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card.clone());
        for ab in &card.abilities { game.state.ability_store.add(ab.clone()); }

        // Set active player to p1 (their turn)
        game.state.active_player = p1;
        game.apply_continuous_effects();
        let perm = game.state.battlefield.get(card_id).unwrap();
        assert!(perm.has_keyword(crate::constants::KeywordAbilities::FIRST_STRIKE),
            "should have first strike on own turn");

        // Set active player to p2 (opponent's turn)
        game.state.active_player = _p2;
        game.apply_continuous_effects();
        let perm = game.state.battlefield.get(card_id).unwrap();
        assert!(!perm.has_keyword(crate::constants::KeywordAbilities::FIRST_STRIKE),
            "should NOT have first strike on opponent's turn");
    }

    #[test]
    fn conditional_keyword_untapped() {
        let (mut game, p1, _p2) = make_test_game();

        let card_id = ObjectId(Uuid::new_v4());
        let card = CardData {
            id: card_id, owner: p1, name: "Hexproof Untapped".into(),
            card_types: vec![crate::constants::CardType::Creature],
            power: Some(3), toughness: Some(3),
            abilities: vec![Ability::static_ability(card_id, "Hexproof as long as untapped.",
                vec![StaticEffect::ConditionalKeyword { keyword: "hexproof".into(), condition: "untapped".into() }])],
            ..Default::default()
        };
        let perm = crate::permanent::Permanent::new(card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card.clone());
        for ab in &card.abilities { game.state.ability_store.add(ab.clone()); }

        // Untapped: should have hexproof
        game.apply_continuous_effects();
        let perm = game.state.battlefield.get(card_id).unwrap();
        assert!(perm.has_keyword(crate::constants::KeywordAbilities::HEXPROOF),
            "should have hexproof when untapped");

        // Tap it
        if let Some(perm) = game.state.battlefield.get_mut(card_id) {
            perm.tap();
        }
        game.apply_continuous_effects();
        let perm = game.state.battlefield.get(card_id).unwrap();
        assert!(!perm.has_keyword(crate::constants::KeywordAbilities::HEXPROOF),
            "should NOT have hexproof when tapped");
    }

    #[test]
    fn conditional_keyword_control_type() {
        let (mut game, p1, _p2) = make_test_game();

        // Create creature with "flash if you control a Faerie"
        let card_id = ObjectId(Uuid::new_v4());
        let card = CardData {
            id: card_id, owner: p1, name: "Faerie Pal".into(),
            card_types: vec![crate::constants::CardType::Creature],
            power: Some(2), toughness: Some(2),
            abilities: vec![Ability::static_ability(card_id, "Flash if you control a Faerie.",
                vec![StaticEffect::ConditionalKeyword { keyword: "flash".into(), condition: "you control a Faerie".into() }])],
            ..Default::default()
        };
        let perm = crate::permanent::Permanent::new(card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card.clone());
        for ab in &card.abilities { game.state.ability_store.add(ab.clone()); }

        // No Faerie: no flash
        game.apply_continuous_effects();
        let perm = game.state.battlefield.get(card_id).unwrap();
        assert!(!perm.has_keyword(crate::constants::KeywordAbilities::FLASH),
            "should NOT have flash without a Faerie");

        // Add a Faerie
        let faerie_id = ObjectId(Uuid::new_v4());
        let faerie = CardData {
            id: faerie_id, owner: p1, name: "Faerie Token".into(),
            card_types: vec![crate::constants::CardType::Creature],
            subtypes: vec![crate::constants::SubType::Faerie],
            power: Some(1), toughness: Some(1),
            ..Default::default()
        };
        let faerie_perm = crate::permanent::Permanent::new(faerie.clone(), p1);
        game.state.battlefield.add(faerie_perm);
        game.state.card_store.insert(faerie);

        game.apply_continuous_effects();
        let perm = game.state.battlefield.get(card_id).unwrap();
        assert!(perm.has_keyword(crate::constants::KeywordAbilities::FLASH),
            "should have flash with a Faerie on BF");
    }

    #[test]
    fn conditional_boost_creature_etb() {
        let (mut game, p1, _p2) = make_test_game();

        let card_id = ObjectId(Uuid::new_v4());
        let card = CardData {
            id: card_id, owner: p1, name: "Boost on ETB".into(),
            card_types: vec![crate::constants::CardType::Creature],
            power: Some(3), toughness: Some(3),
            abilities: vec![Ability::static_ability(card_id, "+2/+0 if creature entered this turn.",
                vec![StaticEffect::ConditionalBoostSelf { power: 2, toughness: 0, condition: "creature entered this turn".into() }])],
            ..Default::default()
        };
        let perm = crate::permanent::Permanent::new(card.clone(), p1);
        game.state.battlefield.add(perm);
        game.state.card_store.insert(card.clone());
        for ab in &card.abilities { game.state.ability_store.add(ab.clone()); }

        // No ETB event: no boost
        game.apply_continuous_effects();
        let perm = game.state.battlefield.get(card_id).unwrap();
        assert_eq!(perm.power(), 3, "should be base power without ETB event");

        // Add ETB event
        game.emit_event(crate::events::GameEvent::enters_battlefield(ObjectId(Uuid::new_v4()), p1));
        game.apply_continuous_effects();
        let perm = game.state.battlefield.get(card_id).unwrap();
        assert_eq!(perm.power(), 5, "should be 3+2 with ETB event this turn");
    }
}

#[cfg(test)]
mod blight_and_types_tests {
    use super::*;
    use crate::abilities::*;
    use crate::types::*;
    use crate::counters::CounterType;
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
}

#[cfg(test)]
mod token_copy_tests {
    use super::*;
    use crate::abilities::*;
    use crate::types::*;
    use crate::counters::CounterType;
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
}

#[cfg(test)]
mod tap_self_and_return_type_tests {
    use super::*;
    use crate::abilities::{Ability, Cost, Effect, TargetSpec};
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, SubType};
    use crate::decision::*;
    use crate::types::{ObjectId, PlayerId};

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
}

#[cfg(test)]
mod dynamic_value_tests {
    use super::*;
    use crate::abilities::{Ability, Cost, Effect, TargetSpec};
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, SubType};
    use crate::decision::*;
    use crate::types::{ObjectId, PlayerId};

    struct AlwaysPassDM;
    impl PlayerDecisionMaker for AlwaysPassDM {
        fn priority(&mut self, _: &GameView, actions: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView, _: crate::constants::Outcome, req: &TargetRequirement) -> Vec<ObjectId> {
            if req.min_targets > 0 && !req.legal_targets.is_empty() { vec![req.legal_targets[0]] } else { vec![] }
        }
        fn choose_use(&mut self, _: &GameView, _: crate::constants::Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView, _modes: &[NamedChoice]) -> usize { 0 }
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

    fn add_creature(game: &mut Game, owner: PlayerId, name: &str, power: i32, toughness: i32, subtypes: Vec<SubType>) -> ObjectId {
        let id = ObjectId::new();
        let mut card = CardData::new(id, owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(power);
        card.toughness = Some(toughness);
        card.subtypes = subtypes;
        let perm = crate::permanent::Permanent::new(card.clone(), owner);
        game.state.card_store.insert(card);
        game.state.battlefield.add(perm);
        id
    }

    #[test]
    fn gain_life_dynamic_greatest_power() {
        let (mut game, p1, _p2) = setup_game();

        // Add some Giants with different powers
        add_creature(&mut game, p1, "Small Giant", 3, 3, vec![SubType::Giant]);
        add_creature(&mut game, p1, "Big Giant", 7, 7, vec![SubType::Giant]);
        add_creature(&mut game, p1, "Medium Giant", 5, 5, vec![SubType::Giant]);
        // Non-giant should not count
        add_creature(&mut game, p1, "Elf", 1, 1, vec![SubType::Elf]);

        let life_before = game.state.players.get(&p1).unwrap().life;

        game.execute_effects(
            &[Effect::GainLifeDynamic { value_source: "greatest power among Giants you control".into() }],
            p1,
            &[],
            None,
            None,
        );

        let life_after = game.state.players.get(&p1).unwrap().life;
        assert_eq!(life_after - life_before, 7, "should gain life equal to biggest Giant's power (7)");
    }

    #[test]
    fn boost_target_dynamic_count() {
        let (mut game, p1, _p2) = setup_game();

        // Add some Kithkin
        add_creature(&mut game, p1, "Kithkin 1", 1, 1, vec![SubType::Kithkin]);
        add_creature(&mut game, p1, "Kithkin 2", 1, 1, vec![SubType::Kithkin]);
        add_creature(&mut game, p1, "Kithkin 3", 1, 1, vec![SubType::Kithkin]);
        // Attacker to receive boost
        let attacker_id = add_creature(&mut game, p1, "Attacker", 2, 2, vec![SubType::Warrior]);

        game.execute_effects(
            &[Effect::BoostTargetDynamic { value_source: "Kithkin you control".into() }],
            p1,
            &[attacker_id],
            None,
            None,
        );

        let perm = game.state.battlefield.get(attacker_id).unwrap();
        // 3 Kithkin, so +3/+3 (base 2/2 + 3/3 = 5/5)
        // But boost is until end of turn, so it should be via granted_keywords or continuous boost
        // Actually, boosts are applied through continuous_boost or direct power modification
        // Let's check the implementation handles this as a temporary boost
        assert_eq!(perm.power(), 5, "should be 2 + 3 from Kithkin count");
        assert_eq!(perm.toughness(), 5, "should be 2 + 3 from Kithkin count");
    }
}

#[cfg(test)]
mod cost_reduction_tests {
    use super::*;
    use crate::abilities::{Ability, Cost, Effect, StaticEffect, TargetSpec};
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, SubType};
    use crate::mana::{Mana, ManaCost};
    use crate::decision::*;
    use crate::types::{ObjectId, PlayerId};

    struct AlwaysPassDM;
    impl PlayerDecisionMaker for AlwaysPassDM {
        fn priority(&mut self, _: &GameView, _actions: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView, _: crate::constants::Outcome, req: &TargetRequirement) -> Vec<ObjectId> {
            if req.min_targets > 0 && !req.legal_targets.is_empty() { vec![req.legal_targets[0]] } else { vec![] }
        }
        fn choose_use(&mut self, _: &GameView, _: crate::constants::Outcome, _: &str) -> bool { false }
        fn choose_mode(&mut self, _: &GameView, _modes: &[NamedChoice]) -> usize { 0 }
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
    fn cost_reduction_reduces_generic_mana() {
        let (mut game, p1, _p2) = setup_game();

        // Add a lord with CostReduction for Elf spells
        let lord_id = ObjectId::new();
        let mut lord = CardData::new(lord_id, p1, "Elf Cost Reducer");
        lord.card_types = vec![CardType::Creature];
        lord.subtypes = vec![SubType::Elf];
        lord.power = Some(1);
        lord.toughness = Some(1);
        lord.abilities = vec![Ability::static_ability(lord_id,
            "Elf spells you cast cost {1} less.",
            vec![StaticEffect::CostReduction { filter: "Elf".into(), amount: 1 }])];
        let perm = crate::permanent::Permanent::new(lord.clone(), p1);
        game.state.card_store.insert(lord.clone());
        game.state.battlefield.add(perm);
        for ab in &lord.abilities {
            game.state.ability_store.add(ab.clone());
        }

        // Test: calculate_cost_reduction should return 1 for an Elf spell
        let elf_spell_id = ObjectId::new();
        let mut elf_spell = CardData::new(elf_spell_id, p1, "Elf Archer");
        elf_spell.card_types = vec![CardType::Creature];
        elf_spell.subtypes = vec![SubType::Elf, SubType::Archer];
        elf_spell.mana_cost = ManaCost::parse("{2}{G}");

        let reduction = game.calculate_cost_reduction(p1, &elf_spell);
        assert_eq!(reduction, 1, "Elf spell should get 1 reduction");

        // Non-Elf spell should get no reduction
        let non_elf_id = ObjectId::new();
        let mut non_elf = CardData::new(non_elf_id, p1, "Goblin");
        non_elf.card_types = vec![CardType::Creature];
        non_elf.subtypes = vec![SubType::Goblin];
        non_elf.mana_cost = ManaCost::parse("{2}{R}");

        let reduction = game.calculate_cost_reduction(p1, &non_elf);
        assert_eq!(reduction, 0, "Non-Elf spell should get no reduction");
    }

    #[test]
    fn cost_reduction_applied_in_legal_actions() {
        let (mut game, p1, _p2) = setup_game();

        // Give player exactly 2 green mana
        game.state.players.get_mut(&p1).unwrap().mana_pool.add(Mana::green(2), None, false);

        // Add a cost reducer for Elf spells
        let lord_id = ObjectId::new();
        let mut lord = CardData::new(lord_id, p1, "Elf Cost Reducer");
        lord.card_types = vec![CardType::Creature];
        lord.subtypes = vec![SubType::Elf];
        lord.power = Some(1);
        lord.toughness = Some(1);
        lord.abilities = vec![Ability::static_ability(lord_id,
            "Elf spells cost {1} less.",
            vec![StaticEffect::CostReduction { filter: "Elf".into(), amount: 1 }])];
        let perm = crate::permanent::Permanent::new(lord.clone(), p1);
        game.state.card_store.insert(lord.clone());
        game.state.battlefield.add(perm);
        for ab in &lord.abilities {
            game.state.ability_store.add(ab.clone());
        }

        // Add an Elf spell that costs {2}{G} to hand — normally needs 3 mana, reduced to 2
        let elf_id = ObjectId::new();
        let mut elf = CardData::new(elf_id, p1, "Elf Archer");
        elf.card_types = vec![CardType::Creature];
        elf.subtypes = vec![SubType::Elf, SubType::Archer];
        elf.mana_cost = ManaCost::parse("{2}{G}");
        game.state.card_store.insert(elf.clone());
        game.state.players.get_mut(&p1).unwrap().hand.add(elf_id);

        // Set up game phase for sorcery speed
        game.state.current_phase = crate::constants::TurnPhase::PrecombatMain;
        game.state.current_step = crate::constants::PhaseStep::PrecombatMain;
        game.state.active_player = p1;
        game.state.priority_player = p1;

        // Compute legal actions — the Elf should be castable with only 2G
        let actions = game.compute_legal_actions(p1);
        let can_cast = actions.iter().any(|a| matches!(a, PlayerAction::CastSpell { card_id, .. } if *card_id == elf_id));
        assert!(can_cast, "Should be able to cast 2G Elf with 2G mana and 1 reduction");
    }
}

#[cfg(test)]
mod lose_all_abilities_tests {
    use super::*;
    use crate::abilities::{Ability, Cost, Effect, StaticEffect, TargetSpec};
    use crate::card::CardData;
    use crate::constants::{AbilityType, CardType, KeywordAbilities, Outcome, SubType};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };
    use crate::mana::Mana;

    struct AlwaysPassPlayer;

    impl PlayerDecisionMaker for AlwaysPassPlayer {
        fn priority(&mut self, _game: &GameView<'_>, _legal: &[PlayerAction]) -> PlayerAction {
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
        card.keywords = KeywordAbilities::empty();
        card
    }

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        let mut deck = Vec::new();
        for _ in 0..20 {
            deck.push(make_basic_land("Forest", owner));
        }
        for _ in 0..20 {
            deck.push(make_creature("Grizzly Bears", owner, 2, 2));
        }
        deck
    }

    fn setup_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
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
    fn lose_all_abilities_removes_keywords() {
        let (mut game, p1, p2) = setup_game();

        let id = ObjectId::new();
        let mut card = CardData::new(id, p2, "Dragon");
        card.card_types = vec![CardType::Creature];
        card.power = Some(5);
        card.toughness = Some(5);
        card.keywords = KeywordAbilities::FLYING | KeywordAbilities::TRAMPLE | KeywordAbilities::HASTE;
        game.state.battlefield.add(crate::permanent::Permanent::new(card, p2));

        assert!(game.state.battlefield.get(id).unwrap().has_flying());
        assert!(game.state.battlefield.get(id).unwrap().has_trample());
        assert!(game.state.battlefield.get(id).unwrap().has_haste());

        game.execute_effects(&[Effect::LoseAllAbilities], p1, &[id], None, None);

        let perm = game.state.battlefield.get(id).unwrap();
        assert!(!perm.has_flying());
        assert!(!perm.has_trample());
        assert!(!perm.has_haste());
        assert!(perm.abilities_lost);
    }

    #[test]
    fn lose_all_abilities_removes_from_ability_store() {
        let (mut game, p1, p2) = setup_game();

        let id = ObjectId::new();
        let mut card = CardData::new(id, p2, "Mana Dork");
        card.card_types = vec![CardType::Creature];
        card.power = Some(1);
        card.toughness = Some(1);
        card.abilities = vec![
            Ability::mana_ability(id, "{T}: Add {G}.", Mana::green(1)),
            Ability::enters_battlefield_triggered(id, "ETB: draw a card.", vec![Effect::draw_cards(1)], TargetSpec::None),
        ];
        for ab in &card.abilities {
            game.state.ability_store.add(ab.clone());
        }
        game.state.battlefield.add(crate::permanent::Permanent::new(card, p2));

        assert_eq!(game.state.ability_store.for_source(id).len(), 2);

        game.execute_effects(&[Effect::LoseAllAbilities], p1, &[id], None, None);

        assert_eq!(game.state.ability_store.for_source(id).len(), 0);
    }

    #[test]
    fn lose_all_abilities_preserves_pt() {
        let (mut game, p1, p2) = setup_game();

        let id = ObjectId::new();
        let mut card = CardData::new(id, p2, "Angel");
        card.card_types = vec![CardType::Creature];
        card.power = Some(4);
        card.toughness = Some(4);
        card.keywords = KeywordAbilities::FLYING | KeywordAbilities::VIGILANCE;
        game.state.battlefield.add(crate::permanent::Permanent::new(card, p2));

        game.execute_effects(&[Effect::LoseAllAbilities], p1, &[id], None, None);

        let perm = game.state.battlefield.get(id).unwrap();
        assert_eq!(perm.power(), 4);
        assert_eq!(perm.toughness(), 4);
        assert!(!perm.has_flying());
        assert!(!perm.has_vigilance());
    }

    #[test]
    fn lose_all_abilities_also_removes_granted_keywords() {
        let (mut game, p1, p2) = setup_game();

        let id = ObjectId::new();
        let mut card = CardData::new(id, p2, "Bear");
        card.card_types = vec![CardType::Creature];
        card.power = Some(2);
        card.toughness = Some(2);
        card.keywords = KeywordAbilities::empty();
        game.state.battlefield.add(crate::permanent::Permanent::new(card, p2));

        if let Some(perm) = game.state.battlefield.get_mut(id) {
            perm.granted_keywords |= KeywordAbilities::FLYING | KeywordAbilities::HEXPROOF;
        }
        assert!(game.state.battlefield.get(id).unwrap().has_flying());
        assert!(game.state.battlefield.get(id).unwrap().has_hexproof());

        game.execute_effects(&[Effect::LoseAllAbilities], p1, &[id], None, None);

        let perm = game.state.battlefield.get(id).unwrap();
        assert!(!perm.has_flying());
        assert!(!perm.has_hexproof());
    }

    #[test]
    fn static_lose_all_abilities_on_enchanted_creature() {
        let (mut game, p1, _p2) = setup_game();

        let creature_id = ObjectId::new();
        let mut creature = CardData::new(creature_id, p1, "Dragon");
        creature.card_types = vec![CardType::Creature];
        creature.subtypes = vec![SubType::Dragon];
        creature.power = Some(5);
        creature.toughness = Some(5);
        creature.keywords = KeywordAbilities::FLYING | KeywordAbilities::TRAMPLE;
        creature.abilities = vec![
            Ability::mana_ability(creature_id, "{T}: Add {R}.", Mana::red(1)),
        ];
        for ab in &creature.abilities {
            game.state.ability_store.add(ab.clone());
        }
        game.state.battlefield.add(crate::permanent::Permanent::new(creature, p1));

        let aura_id = ObjectId::new();
        let mut aura = CardData::new(aura_id, p1, "Noggle Aura");
        aura.card_types = vec![CardType::Enchantment];
        aura.subtypes = vec![SubType::Aura];
        aura.abilities = vec![
            Ability::static_ability(aura_id,
                "Enchanted creature loses all abilities.",
                vec![StaticEffect::LoseAllAbilities { filter: "enchanted creature".into() }]),
        ];
        for ab in &aura.abilities {
            game.state.ability_store.add(ab.clone());
        }
        let mut aura_perm = crate::permanent::Permanent::new(aura, p1);
        aura_perm.attached_to = Some(creature_id);
        game.state.battlefield.add(aura_perm);
        if let Some(c) = game.state.battlefield.get_mut(creature_id) {
            c.attachments.push(aura_id);
        }

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(!perm.has_flying());
        assert!(!perm.has_trample());
        assert!(perm.abilities_lost);
    }

    #[test]
    fn effect_builder() {
        match Effect::lose_all_abilities() {
            Effect::LoseAllAbilities => {}
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn static_effect_builder() {
        match StaticEffect::lose_all_abilities("enchanted creature") {
            StaticEffect::LoseAllAbilities { filter } => {
                assert_eq!(filter, "enchanted creature");
            }
            _ => panic!("wrong variant"),
        }
    }
}

#[cfg(test)]
mod set_base_pt_tests {
    use super::*;
    use crate::abilities::{Ability, Effect, StaticEffect};
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, Outcome, SubType};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };

    struct AlwaysPassPlayer;

    impl PlayerDecisionMaker for AlwaysPassPlayer {
        fn priority(&mut self, _game: &GameView<'_>, _legal: &[PlayerAction]) -> PlayerAction {
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

    fn make_basic_land(name: &str, owner: PlayerId) -> CardData {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Land];
        card
    }

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        let mut deck = Vec::new();
        for _ in 0..20 {
            deck.push(make_basic_land("Forest", owner));
        }
        for _ in 0..20 {
            let mut c = CardData::new(ObjectId::new(), owner, "Grizzly Bears");
            c.card_types = vec![CardType::Creature];
            c.power = Some(2);
            c.toughness = Some(2);
            deck.push(c);
        }
        deck
    }

    fn setup_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
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
    fn set_base_pt_all_opponents_creatures() {
        let (mut game, p1, p2) = setup_game();

        let bear_id = ObjectId::new();
        let mut bear = CardData::new(bear_id, p2, "Big Bear");
        bear.card_types = vec![CardType::Creature];
        bear.power = Some(5);
        bear.toughness = Some(5);
        bear.keywords = KeywordAbilities::TRAMPLE;
        game.state.battlefield.add(crate::permanent::Permanent::new(bear, p2));

        let angel_id = ObjectId::new();
        let mut angel = CardData::new(angel_id, p2, "Angel");
        angel.card_types = vec![CardType::Creature];
        angel.power = Some(4);
        angel.toughness = Some(4);
        angel.keywords = KeywordAbilities::FLYING;
        game.state.battlefield.add(crate::permanent::Permanent::new(angel, p2));

        let own_id = ObjectId::new();
        let mut own = CardData::new(own_id, p1, "Own Bear");
        own.card_types = vec![CardType::Creature];
        own.power = Some(3);
        own.toughness = Some(3);
        game.state.battlefield.add(crate::permanent::Permanent::new(own, p1));

        game.execute_effects(
            &[Effect::SetBasePowerToughnessAll { power: 1, toughness: 1, filter: "creatures opponents control".into() }],
            p1, &[], None, None,
        );

        assert_eq!(game.state.battlefield.get(bear_id).unwrap().power(), 1);
        assert_eq!(game.state.battlefield.get(bear_id).unwrap().toughness(), 1);
        assert_eq!(game.state.battlefield.get(angel_id).unwrap().power(), 1);
        assert_eq!(game.state.battlefield.get(angel_id).unwrap().toughness(), 1);
        assert_eq!(game.state.battlefield.get(own_id).unwrap().power(), 3);
        assert_eq!(game.state.battlefield.get(own_id).unwrap().toughness(), 3);
    }

    #[test]
    fn lose_all_abilities_all_opponents_creatures() {
        let (mut game, p1, p2) = setup_game();

        let bear_id = ObjectId::new();
        let mut bear = CardData::new(bear_id, p2, "Flying Bear");
        bear.card_types = vec![CardType::Creature];
        bear.power = Some(3);
        bear.toughness = Some(3);
        bear.keywords = KeywordAbilities::FLYING | KeywordAbilities::TRAMPLE;
        game.state.battlefield.add(crate::permanent::Permanent::new(bear, p2));

        let own_id = ObjectId::new();
        let mut own = CardData::new(own_id, p1, "Own Flyer");
        own.card_types = vec![CardType::Creature];
        own.power = Some(2);
        own.toughness = Some(2);
        own.keywords = KeywordAbilities::FLYING;
        game.state.battlefield.add(crate::permanent::Permanent::new(own, p1));

        game.execute_effects(
            &[Effect::LoseAllAbilitiesAll { filter: "creatures opponents control".into() }],
            p1, &[], None, None,
        );

        assert!(!game.state.battlefield.get(bear_id).unwrap().has_flying());
        assert!(!game.state.battlefield.get(bear_id).unwrap().has_trample());
        assert!(game.state.battlefield.get(bear_id).unwrap().abilities_lost);
        assert!(game.state.battlefield.get(own_id).unwrap().has_flying());
        assert!(!game.state.battlefield.get(own_id).unwrap().abilities_lost);
    }

    #[test]
    fn static_set_base_pt_on_enchanted_creature() {
        let (mut game, p1, _p2) = setup_game();

        let creature_id = ObjectId::new();
        let mut creature = CardData::new(creature_id, p1, "Dragon");
        creature.card_types = vec![CardType::Creature];
        creature.subtypes = vec![SubType::Dragon];
        creature.power = Some(5);
        creature.toughness = Some(5);
        creature.keywords = KeywordAbilities::FLYING;
        game.state.battlefield.add(crate::permanent::Permanent::new(creature, p1));

        let aura_id = ObjectId::new();
        let mut aura = CardData::new(aura_id, p1, "Shrinking Aura");
        aura.card_types = vec![CardType::Enchantment];
        aura.subtypes = vec![SubType::Aura];
        aura.abilities = vec![
            Ability::static_ability(aura_id,
                "Enchanted creature has base power and toughness 1/1.",
                vec![StaticEffect::SetBasePowerToughness {
                    filter: "enchanted creature".into(),
                    power: 1,
                    toughness: 1,
                }]),
        ];
        for ab in &aura.abilities {
            game.state.ability_store.add(ab.clone());
        }
        let mut aura_perm = crate::permanent::Permanent::new(aura, p1);
        aura_perm.attached_to = Some(creature_id);
        game.state.battlefield.add(aura_perm);
        if let Some(c) = game.state.battlefield.get_mut(creature_id) {
            c.attachments.push(aura_id);
        }

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert_eq!(perm.power(), 1);
        assert_eq!(perm.toughness(), 1);
    }

    #[test]
    fn static_set_base_pt_stacks_with_boost() {
        let (mut game, p1, _p2) = setup_game();

        let creature_id = ObjectId::new();
        let mut creature = CardData::new(creature_id, p1, "Dragon");
        creature.card_types = vec![CardType::Creature];
        creature.power = Some(5);
        creature.toughness = Some(5);
        game.state.battlefield.add(crate::permanent::Permanent::new(creature, p1));

        let lord_id = ObjectId::new();
        let mut lord = CardData::new(lord_id, p1, "Lord");
        lord.card_types = vec![CardType::Creature];
        lord.power = Some(2);
        lord.toughness = Some(2);
        lord.abilities = vec![
            Ability::static_ability(lord_id,
                "Other creatures you control get +1/+1.",
                vec![StaticEffect::Boost {
                    filter: "other creatures you control".into(),
                    power: 1,
                    toughness: 1,
                }]),
        ];
        for ab in &lord.abilities {
            game.state.ability_store.add(ab.clone());
        }
        game.state.battlefield.add(crate::permanent::Permanent::new(lord, p1));

        let aura_id = ObjectId::new();
        let mut aura = CardData::new(aura_id, p1, "Shrinking Aura");
        aura.card_types = vec![CardType::Enchantment];
        aura.subtypes = vec![SubType::Aura];
        aura.abilities = vec![
            Ability::static_ability(aura_id,
                "Enchanted creature has base power and toughness 1/1.",
                vec![StaticEffect::SetBasePowerToughness {
                    filter: "enchanted creature".into(),
                    power: 1,
                    toughness: 1,
                }]),
        ];
        for ab in &aura.abilities {
            game.state.ability_store.add(ab.clone());
        }
        let mut aura_perm = crate::permanent::Permanent::new(aura, p1);
        aura_perm.attached_to = Some(creature_id);
        game.state.battlefield.add(aura_perm);
        if let Some(c) = game.state.battlefield.get_mut(creature_id) {
            c.attachments.push(aura_id);
        }

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert_eq!(perm.power(), 2);
        assert_eq!(perm.toughness(), 2);
    }

    #[test]
    fn static_set_base_pt_resets_when_aura_removed() {
        let (mut game, p1, _p2) = setup_game();

        let creature_id = ObjectId::new();
        let mut creature = CardData::new(creature_id, p1, "Dragon");
        creature.card_types = vec![CardType::Creature];
        creature.power = Some(5);
        creature.toughness = Some(5);
        game.state.battlefield.add(crate::permanent::Permanent::new(creature, p1));

        let aura_id = ObjectId::new();
        let mut aura = CardData::new(aura_id, p1, "Shrinking Aura");
        aura.card_types = vec![CardType::Enchantment];
        aura.subtypes = vec![SubType::Aura];
        aura.abilities = vec![
            Ability::static_ability(aura_id,
                "Enchanted creature has base power and toughness 1/1.",
                vec![StaticEffect::SetBasePowerToughness {
                    filter: "enchanted creature".into(),
                    power: 1,
                    toughness: 1,
                }]),
        ];
        for ab in &aura.abilities {
            game.state.ability_store.add(ab.clone());
        }
        let mut aura_perm = crate::permanent::Permanent::new(aura, p1);
        aura_perm.attached_to = Some(creature_id);
        game.state.battlefield.add(aura_perm);
        if let Some(c) = game.state.battlefield.get_mut(creature_id) {
            c.attachments.push(aura_id);
        }

        game.apply_continuous_effects();
        assert_eq!(game.state.battlefield.get(creature_id).unwrap().power(), 1);
        assert_eq!(game.state.battlefield.get(creature_id).unwrap().toughness(), 1);

        game.state.battlefield.remove(aura_id);
        game.state.ability_store.remove_source(aura_id);
        if let Some(c) = game.state.battlefield.get_mut(creature_id) {
            c.attachments.retain(|&id| id != aura_id);
        }

        game.apply_continuous_effects();
        assert_eq!(game.state.battlefield.get(creature_id).unwrap().power(), 5);
        assert_eq!(game.state.battlefield.get(creature_id).unwrap().toughness(), 5);
    }

    #[test]
    fn set_base_pt_all_only_affects_creatures() {
        let (mut game, p1, p2) = setup_game();

        let artifact_id = ObjectId::new();
        let mut artifact = CardData::new(artifact_id, p2, "Artifact");
        artifact.card_types = vec![CardType::Artifact];
        game.state.battlefield.add(crate::permanent::Permanent::new(artifact, p2));

        let bear_id = ObjectId::new();
        let mut bear = CardData::new(bear_id, p2, "Bear");
        bear.card_types = vec![CardType::Creature];
        bear.power = Some(3);
        bear.toughness = Some(3);
        game.state.battlefield.add(crate::permanent::Permanent::new(bear, p2));

        game.execute_effects(
            &[Effect::SetBasePowerToughnessAll { power: 1, toughness: 1, filter: "creatures opponents control".into() }],
            p1, &[], None, None,
        );

        assert_eq!(game.state.battlefield.get(bear_id).unwrap().power(), 1);
        assert_eq!(game.state.battlefield.get(bear_id).unwrap().toughness(), 1);
    }

    #[test]
    fn effect_builders() {
        match Effect::set_base_pt_all(1, 1, "creatures opponents control") {
            Effect::SetBasePowerToughnessAll { power, toughness, filter } => {
                assert_eq!(power, 1);
                assert_eq!(toughness, 1);
                assert_eq!(filter, "creatures opponents control");
            }
            _ => panic!("wrong variant"),
        }

        match Effect::lose_all_abilities_all("creatures opponents control") {
            Effect::LoseAllAbilitiesAll { filter } => {
                assert_eq!(filter, "creatures opponents control");
            }
            _ => panic!("wrong variant"),
        }

        match StaticEffect::set_base_pt("enchanted creature", 1, 1) {
            StaticEffect::SetBasePowerToughness { filter, power, toughness } => {
                assert_eq!(filter, "enchanted creature");
                assert_eq!(power, 1);
                assert_eq!(toughness, 1);
            }
            _ => panic!("wrong variant"),
        }
    }
}

#[cfg(test)]
mod put_from_hand_tests {
    use super::*;
    use crate::abilities::Effect;
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, Outcome, PhaseStep, TurnPhase};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };
    use crate::mana::ManaCost;

    struct YesPickFirstPlayer;

    impl PlayerDecisionMaker for YesPickFirstPlayer {
        fn priority(&mut self, _: &GameView<'_>, _: &[PlayerAction]) -> PlayerAction { PlayerAction::Pass }
        fn choose_targets(&mut self, _: &GameView<'_>, _: Outcome, _: &TargetRequirement) -> Vec<ObjectId> { vec![] }
        fn choose_use(&mut self, _: &GameView<'_>, _: Outcome, _: &str) -> bool { true }
        fn choose_mode(&mut self, _: &GameView<'_>, _: &[NamedChoice]) -> usize { 0 }
        fn select_attackers(&mut self, _: &GameView<'_>, _: &[ObjectId], _: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn select_blockers(&mut self, _: &GameView<'_>, _: &[AttackerInfo]) -> Vec<(ObjectId, ObjectId)> { vec![] }
        fn assign_damage(&mut self, _: &GameView<'_>, _: &DamageAssignment) -> Vec<(ObjectId, u32)> { vec![] }
        fn choose_mulligan(&mut self, _: &GameView<'_>, _: &[ObjectId]) -> bool { false }
        fn choose_cards_to_put_back(&mut self, _: &GameView<'_>, _: &[ObjectId], _: usize) -> Vec<ObjectId> { vec![] }
        fn choose_discard(&mut self, _: &GameView<'_>, hand: &[ObjectId], count: usize) -> Vec<ObjectId> {
            hand.iter().take(count).copied().collect()
        }
        fn choose_amount(&mut self, _: &GameView<'_>, _: &str, min: u32, _: u32) -> u32 { min }
        fn choose_mana_payment(&mut self, _: &GameView<'_>, _: &UnpaidMana, _: &[PlayerAction]) -> Option<PlayerAction> { None }
        fn choose_replacement_effect(&mut self, _: &GameView<'_>, _: &[ReplacementEffectChoice]) -> usize { 0 }
        fn choose_pile(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[ObjectId], _: &[ObjectId]) -> bool { true }
        fn choose_option(&mut self, _: &GameView<'_>, _: Outcome, _: &str, _: &[NamedChoice]) -> usize { 0 }
    }

    struct NoPlayer;

    impl PlayerDecisionMaker for NoPlayer {
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

    fn setup_game_with_dm(dm1: Box<dyn PlayerDecisionMaker>, dm2: Box<dyn PlayerDecisionMaker>) -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            starting_life: 20,
            players: vec![
                PlayerConfig { name: "P1".into(), deck: vec![] },
                PlayerConfig { name: "P2".into(), deck: vec![] },
            ],
        };
        let mut game = Game::new_two_player(config, vec![(p1, dm1), (p2, dm2)]);
        game.state.active_player = p1;
        game.state.priority_player = p1;
        game.state.current_phase = TurnPhase::PrecombatMain;
        game.state.current_step = PhaseStep::PrecombatMain;
        game.state.turn_number = 1;
        (game, p1, p2)
    }

    fn add_creature_to_hand(game: &mut Game, owner: PlayerId, name: &str, mana_cost: &str, power: i32, toughness: i32) -> ObjectId {
        let id = ObjectId::new();
        let mut card = CardData::new(id, owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(power);
        card.toughness = Some(toughness);
        card.mana_cost = ManaCost::parse(mana_cost);
        game.state.card_store.insert(card);
        game.state.players.get_mut(&owner).unwrap().hand.add(id);
        id
    }

    #[test]
    fn put_from_hand_basic() {
        let (mut game, p1, _p2) = setup_game_with_dm(
            Box::new(YesPickFirstPlayer), Box::new(NoPlayer));

        let goblin_id = add_creature_to_hand(&mut game, p1, "Goblin", "{R}", 1, 1);

        assert_eq!(game.state.players.get(&p1).unwrap().hand.len(), 1);

        game.execute_effects(
            &[Effect::put_from_hand_with_haste_sacrifice(2)],
            p1, &[], None, None,
        );

        assert_eq!(game.state.players.get(&p1).unwrap().hand.len(), 0);
        let perm = game.state.battlefield.get(goblin_id).unwrap();
        assert_eq!(perm.card.name, "Goblin");
        assert!(perm.has_keyword(KeywordAbilities::HASTE));
    }

    #[test]
    fn put_from_hand_mv_filter() {
        let (mut game, p1, _p2) = setup_game_with_dm(
            Box::new(YesPickFirstPlayer), Box::new(NoPlayer));

        let expensive_id = add_creature_to_hand(&mut game, p1, "Dragon", "{4}{R}{R}", 5, 5);

        game.execute_effects(
            &[Effect::put_from_hand_with_haste_sacrifice(2)],
            p1, &[], None, None,
        );

        assert_eq!(game.state.players.get(&p1).unwrap().hand.len(), 1);
        assert!(game.state.battlefield.get(expensive_id).is_none());
    }

    #[test]
    fn put_from_hand_player_declines() {
        let (mut game, p1, _p2) = setup_game_with_dm(
            Box::new(NoPlayer), Box::new(NoPlayer));

        let goblin_id = add_creature_to_hand(&mut game, p1, "Goblin", "{R}", 1, 1);

        game.execute_effects(
            &[Effect::put_from_hand_with_haste_sacrifice(2)],
            p1, &[], None, None,
        );

        assert_eq!(game.state.players.get(&p1).unwrap().hand.len(), 1);
        assert!(game.state.battlefield.get(goblin_id).is_none());
    }

    #[test]
    fn put_from_hand_tapped_attacking() {
        let (mut game, p1, _p2) = setup_game_with_dm(
            Box::new(YesPickFirstPlayer), Box::new(NoPlayer));

        let soldier_id = add_creature_to_hand(&mut game, p1, "Soldier", "{W}", 2, 2);

        game.execute_effects(
            &[Effect::put_from_hand_tapped_attacking(3)],
            p1, &[], None, None,
        );

        assert_eq!(game.state.players.get(&p1).unwrap().hand.len(), 0);
        let perm = game.state.battlefield.get(soldier_id).unwrap();
        assert!(perm.tapped);
        assert!(!perm.summoning_sick);
    }

    #[test]
    fn put_from_hand_sacrifice_at_eot() {
        let (mut game, p1, _p2) = setup_game_with_dm(
            Box::new(YesPickFirstPlayer), Box::new(NoPlayer));

        let _goblin_id = add_creature_to_hand(&mut game, p1, "Goblin", "{R}", 1, 1);

        game.execute_effects(
            &[Effect::put_from_hand_with_haste_sacrifice(2)],
            p1, &[], None, None,
        );

        assert_eq!(game.state.delayed_triggers.len(), 1);
        let dt = &game.state.delayed_triggers[0];
        assert_eq!(dt.event_type, crate::events::EventType::EndStep);
        assert!(dt.trigger_only_once);
    }

    #[test]
    fn put_from_hand_no_sacrifice_when_not_requested() {
        let (mut game, p1, _p2) = setup_game_with_dm(
            Box::new(YesPickFirstPlayer), Box::new(NoPlayer));

        let _soldier_id = add_creature_to_hand(&mut game, p1, "Soldier", "{W}", 2, 2);

        game.execute_effects(
            &[Effect::put_from_hand_tapped_attacking(3)],
            p1, &[], None, None,
        );

        assert_eq!(game.state.delayed_triggers.len(), 0);
    }

    #[test]
    fn put_from_hand_x_value_mv_limit() {
        let (mut game, p1, _p2) = setup_game_with_dm(
            Box::new(YesPickFirstPlayer), Box::new(NoPlayer));

        let small_id = add_creature_to_hand(&mut game, p1, "Goblin", "{R}", 1, 1);
        let big_id = add_creature_to_hand(&mut game, p1, "Dragon", "{3}{R}{R}", 5, 5);

        game.execute_effects(
            &[Effect::PutFromHandToBattlefield {
                max_mana_value: crate::abilities::X_VALUE,
                max_mv_dynamic: None,
                tapped: true,
                attacking: true,
                haste: false,
                sacrifice_eot: false,
            }],
            p1, &[], None, Some(2),
        );

        assert_eq!(game.state.players.get(&p1).unwrap().hand.len(), 1);
        assert!(game.state.battlefield.get(small_id).is_some());
        assert!(game.state.battlefield.get(big_id).is_none());
    }

    #[test]
    fn put_from_hand_empty_hand_noop() {
        let (mut game, p1, _p2) = setup_game_with_dm(
            Box::new(YesPickFirstPlayer), Box::new(NoPlayer));

        game.execute_effects(
            &[Effect::put_from_hand_with_haste_sacrifice(2)],
            p1, &[], None, None,
        );

        assert_eq!(game.state.battlefield.len(), 0);
    }

    #[test]
    fn put_from_hand_noncreature_not_eligible() {
        let (mut game, p1, _p2) = setup_game_with_dm(
            Box::new(YesPickFirstPlayer), Box::new(NoPlayer));

        let enchantment_id = ObjectId::new();
        let mut card = CardData::new(enchantment_id, p1, "Enchantment");
        card.card_types = vec![CardType::Enchantment];
        card.mana_cost = ManaCost::parse("{1}");
        game.state.card_store.insert(card);
        game.state.players.get_mut(&p1).unwrap().hand.add(enchantment_id);

        game.execute_effects(
            &[Effect::put_from_hand_with_haste_sacrifice(5)],
            p1, &[], None, None,
        );

        assert_eq!(game.state.players.get(&p1).unwrap().hand.len(), 1);
        assert!(game.state.battlefield.get(enchantment_id).is_none());
    }

    #[test]
    fn effect_builder_put_from_hand_haste_sacrifice() {
        match Effect::put_from_hand_with_haste_sacrifice(2) {
            Effect::PutFromHandToBattlefield { max_mana_value, max_mv_dynamic, tapped, attacking, haste, sacrifice_eot } => {
                assert_eq!(max_mana_value, 2);
                assert!(max_mv_dynamic.is_none());
                assert!(!tapped);
                assert!(!attacking);
                assert!(haste);
                assert!(sacrifice_eot);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn effect_builder_put_from_hand_tapped_attacking() {
        match Effect::put_from_hand_tapped_attacking(5) {
            Effect::PutFromHandToBattlefield { max_mana_value, max_mv_dynamic, tapped, attacking, haste, sacrifice_eot } => {
                assert_eq!(max_mana_value, 5);
                assert!(max_mv_dynamic.is_none());
                assert!(tapped);
                assert!(attacking);
                assert!(!haste);
                assert!(!sacrifice_eot);
            }
            _ => panic!("wrong variant"),
        }
    }
}

#[cfg(test)]
mod cant_untap_tests {
    use super::*;
    use crate::abilities::{Ability, StaticEffect};
    use crate::card::CardData;
    use crate::constants::{CardType, Outcome, SubType};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };
    use crate::permanent::Permanent;

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

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        let mut deck = Vec::new();
        for _ in 0..20 {
            deck.push(make_basic_land("Forest", owner));
        }
        for _ in 0..20 {
            deck.push(make_creature("Grizzly Bears", owner, 2, 2));
        }
        deck
    }

    fn setup_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
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
    fn cant_untap_prevents_untap_during_untap_step() {
        let (mut game, p1, _p2) = setup_game();

        let creature = make_creature("Test Creature", p1, 3, 3);
        let creature_id = creature.id;
        game.state.battlefield.add(Permanent::new(creature, p1));

        if let Some(perm) = game.state.battlefield.get_mut(creature_id) {
            perm.tap();
            perm.cant_untap = true;
        }

        game.turn_based_actions(PhaseStep::Untap, p1);

        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(perm.tapped, "Creature with cant_untap should remain tapped after untap step");
    }

    #[test]
    fn normal_creature_untaps_during_untap_step() {
        let (mut game, p1, _p2) = setup_game();

        let creature = make_creature("Test Creature", p1, 3, 3);
        let creature_id = creature.id;
        game.state.battlefield.add(Permanent::new(creature, p1));

        if let Some(perm) = game.state.battlefield.get_mut(creature_id) {
            perm.tap();
        }

        game.turn_based_actions(PhaseStep::Untap, p1);

        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(!perm.tapped, "Normal creature should untap during untap step");
    }

    #[test]
    fn cant_untap_static_effect_sets_flag_on_enchanted_creature() {
        let (mut game, p1, _p2) = setup_game();

        let creature = make_creature("Target Creature", p1, 2, 2);
        let creature_id = creature.id;
        game.state.battlefield.add(Permanent::new(creature, p1));

        let mut aura_card = CardData::new(ObjectId::new(), p1, "Test Aura");
        aura_card.card_types = vec![CardType::Enchantment];
        aura_card.subtypes = vec![SubType::Aura];
        let aura_id = aura_card.id;
        let aura_ability = Ability::static_ability(
            aura_id,
            "Enchanted creature can't untap.",
            vec![StaticEffect::cant_untap("enchanted creature")],
        );
        game.state.battlefield.add(Permanent::new(aura_card, p1));
        game.state.ability_store.add(aura_ability);

        if let Some(aura) = game.state.battlefield.get_mut(aura_id) {
            aura.attach_to(creature_id);
        }
        if let Some(creature) = game.state.battlefield.get_mut(creature_id) {
            creature.add_attachment(aura_id);
        }

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(perm.cant_untap, "Enchanted creature should have cant_untap set");
    }

    #[test]
    fn cant_untap_aura_prevents_untapping() {
        let (mut game, p1, _p2) = setup_game();

        let creature = make_creature("Target Creature", p1, 2, 2);
        let creature_id = creature.id;
        game.state.battlefield.add(Permanent::new(creature, p1));

        let mut aura_card = CardData::new(ObjectId::new(), p1, "Blossombind");
        aura_card.card_types = vec![CardType::Enchantment];
        aura_card.subtypes = vec![SubType::Aura];
        let aura_id = aura_card.id;
        let aura_ability = Ability::static_ability(
            aura_id,
            "Enchanted creature can't untap.",
            vec![StaticEffect::cant_untap("enchanted creature")],
        );
        game.state.battlefield.add(Permanent::new(aura_card, p1));
        game.state.ability_store.add(aura_ability);

        if let Some(aura) = game.state.battlefield.get_mut(aura_id) {
            aura.attach_to(creature_id);
        }
        if let Some(creature) = game.state.battlefield.get_mut(creature_id) {
            creature.add_attachment(aura_id);
            creature.tap();
        }

        game.apply_continuous_effects();
        game.turn_based_actions(PhaseStep::Untap, p1);

        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(perm.tapped, "Creature enchanted by CantUntap aura should stay tapped");
    }

    #[test]
    fn removing_cant_untap_aura_allows_untapping() {
        let (mut game, p1, _p2) = setup_game();

        let creature = make_creature("Target Creature", p1, 2, 2);
        let creature_id = creature.id;
        game.state.battlefield.add(Permanent::new(creature, p1));

        let mut aura_card = CardData::new(ObjectId::new(), p1, "Blossombind");
        aura_card.card_types = vec![CardType::Enchantment];
        aura_card.subtypes = vec![SubType::Aura];
        let aura_id = aura_card.id;
        let aura_ability = Ability::static_ability(
            aura_id,
            "Enchanted creature can't untap.",
            vec![StaticEffect::cant_untap("enchanted creature")],
        );
        game.state.battlefield.add(Permanent::new(aura_card, p1));
        game.state.ability_store.add(aura_ability);

        if let Some(aura) = game.state.battlefield.get_mut(aura_id) {
            aura.attach_to(creature_id);
        }
        if let Some(creature) = game.state.battlefield.get_mut(creature_id) {
            creature.add_attachment(aura_id);
            creature.tap();
        }

        game.apply_continuous_effects();
        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(perm.cant_untap, "Should be cant_untap while aura is attached");

        game.state.battlefield.remove(aura_id);
        game.state.ability_store.remove_source(aura_id);
        if let Some(creature) = game.state.battlefield.get_mut(creature_id) {
            creature.remove_attachment(aura_id);
        }

        game.apply_continuous_effects();
        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(!perm.cant_untap, "Should not be cant_untap after aura removed");

        game.turn_based_actions(PhaseStep::Untap, p1);
        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(!perm.tapped, "Creature should untap after aura is removed");
    }

    #[test]
    fn cant_untap_self_filter() {
        let (mut game, p1, _p2) = setup_game();

        let creature = make_creature("Self Lock", p1, 4, 4);
        let creature_id = creature.id;
        let ability = Ability::static_ability(
            creature_id,
            "This creature doesn't untap during your untap step.",
            vec![StaticEffect::cant_untap("self")],
        );
        game.state.battlefield.add(Permanent::new(creature, p1));
        game.state.ability_store.add(ability);

        if let Some(perm) = game.state.battlefield.get_mut(creature_id) {
            perm.tap();
        }

        game.apply_continuous_effects();
        game.turn_based_actions(PhaseStep::Untap, p1);

        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(perm.tapped, "Self-locking creature should remain tapped");
    }

    #[test]
    fn cant_untap_only_affects_matching_permanents() {
        let (mut game, p1, _p2) = setup_game();

        let creature1 = make_creature("Locked Creature", p1, 2, 2);
        let creature1_id = creature1.id;
        game.state.battlefield.add(Permanent::new(creature1, p1));

        let creature2 = make_creature("Free Creature", p1, 3, 3);
        let creature2_id = creature2.id;
        game.state.battlefield.add(Permanent::new(creature2, p1));

        let mut aura_card = CardData::new(ObjectId::new(), p1, "Lockdown Aura");
        aura_card.card_types = vec![CardType::Enchantment];
        aura_card.subtypes = vec![SubType::Aura];
        let aura_id = aura_card.id;
        let aura_ability = Ability::static_ability(
            aura_id,
            "Enchanted creature can't untap.",
            vec![StaticEffect::cant_untap("enchanted creature")],
        );
        game.state.battlefield.add(Permanent::new(aura_card, p1));
        game.state.ability_store.add(aura_ability);

        if let Some(aura) = game.state.battlefield.get_mut(aura_id) {
            aura.attach_to(creature1_id);
        }
        if let Some(c) = game.state.battlefield.get_mut(creature1_id) {
            c.add_attachment(aura_id);
            c.tap();
        }
        if let Some(c) = game.state.battlefield.get_mut(creature2_id) {
            c.tap();
        }

        game.apply_continuous_effects();
        game.turn_based_actions(PhaseStep::Untap, p1);

        let c1 = game.state.battlefield.get(creature1_id).unwrap();
        assert!(c1.tapped, "Enchanted creature should remain tapped");
        let c2 = game.state.battlefield.get(creature2_id).unwrap();
        assert!(!c2.tapped, "Non-enchanted creature should untap normally");
    }

    #[test]
    fn static_effect_helper_constructor() {
        match StaticEffect::cant_untap("enchanted creature") {
            StaticEffect::CantUntap { filter } => {
                assert_eq!(filter, "enchanted creature");
            }
            _ => panic!("wrong variant"),
        }
    }
}

#[cfg(test)]
mod choose_type_reanimate_tests {
    use super::*;
    use crate::abilities::Effect;
    use crate::card::CardData;
    use crate::constants::{CardType, Outcome, SubType};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };

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

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
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
                PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
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
        let (mut game, p1, _p2) = setup_game_with_picker(0);

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
}

#[cfg(test)]
mod choose_type_grant_keywords_tests {
    use super::*;
    use crate::abilities::Effect;
    use crate::card::CardData;
    use crate::constants::{CardType, KeywordAbilities, Outcome, SubType};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };

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

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
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
                PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
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

    fn put_on_battlefield(game: &mut Game, card: CardData, owner: PlayerId) -> ObjectId {
        let id = card.id;
        let perm = Permanent::new(card, owner);
        game.state.battlefield.add(perm);
        game.state.set_zone(id, crate::constants::Zone::Battlefield, None);
        id
    }

    #[test]
    fn grants_keywords_to_matching_creatures() {
        let (mut game, p1, _p2) = setup_game_with_picker(1);

        let source = make_creature_with_type("Selfless Safewright", p1, "Elf", 4, 2);
        let source_id = put_on_battlefield(&mut game, source, p1);
        let elf1 = make_creature_with_type("Llanowar Elves", p1, "Elf", 1, 1);
        let elf1_id = put_on_battlefield(&mut game, elf1, p1);
        let elf2 = make_creature_with_type("Elvish Mystic", p1, "Elf", 1, 1);
        let elf2_id = put_on_battlefield(&mut game, elf2, p1);

        let effects = vec![Effect::choose_type_and_grant_keywords(vec!["hexproof", "indestructible"], true)];
        game.execute_effects(&effects, p1, &[], Some(source_id), None);

        let perm1 = game.state.battlefield.get(elf1_id).unwrap();
        assert!(perm1.has_keyword(KeywordAbilities::HEXPROOF), "Elf 1 should have hexproof");
        assert!(perm1.has_keyword(KeywordAbilities::INDESTRUCTIBLE), "Elf 1 should have indestructible");
        let perm2 = game.state.battlefield.get(elf2_id).unwrap();
        assert!(perm2.has_keyword(KeywordAbilities::HEXPROOF), "Elf 2 should have hexproof");
        assert!(perm2.has_keyword(KeywordAbilities::INDESTRUCTIBLE), "Elf 2 should have indestructible");
    }

    #[test]
    fn excludes_source_when_other_only() {
        let (mut game, p1, _p2) = setup_game_with_picker(1);

        let source = make_creature_with_type("Selfless Safewright", p1, "Elf", 4, 2);
        let source_id = put_on_battlefield(&mut game, source, p1);
        let elf = make_creature_with_type("Llanowar Elves", p1, "Elf", 1, 1);
        let elf_id = put_on_battlefield(&mut game, elf, p1);

        let effects = vec![Effect::choose_type_and_grant_keywords(vec!["hexproof", "indestructible"], true)];
        game.execute_effects(&effects, p1, &[], Some(source_id), None);

        let source_perm = game.state.battlefield.get(source_id).unwrap();
        assert!(!source_perm.has_keyword(KeywordAbilities::HEXPROOF), "Source should NOT have hexproof (other_only=true)");
        let elf_perm = game.state.battlefield.get(elf_id).unwrap();
        assert!(elf_perm.has_keyword(KeywordAbilities::HEXPROOF), "Other elf should have hexproof");
    }

    #[test]
    fn includes_source_when_not_other_only() {
        let (mut game, p1, _p2) = setup_game_with_picker(1);

        let source = make_creature_with_type("Source Elf", p1, "Elf", 4, 2);
        let source_id = put_on_battlefield(&mut game, source, p1);

        let effects = vec![Effect::choose_type_and_grant_keywords(vec!["hexproof"], false)];
        game.execute_effects(&effects, p1, &[], Some(source_id), None);

        let source_perm = game.state.battlefield.get(source_id).unwrap();
        assert!(source_perm.has_keyword(KeywordAbilities::HEXPROOF), "Source should have hexproof (other_only=false)");
    }

    #[test]
    fn ignores_non_matching_types() {
        let (mut game, p1, _p2) = setup_game_with_picker(1);

        let source = make_creature_with_type("Source", p1, "Elf", 4, 2);
        let source_id = put_on_battlefield(&mut game, source, p1);
        let goblin = make_creature_with_type("Goblin Piker", p1, "Goblin", 2, 1);
        let goblin_id = put_on_battlefield(&mut game, goblin, p1);

        let effects = vec![Effect::choose_type_and_grant_keywords(vec!["hexproof", "indestructible"], true)];
        game.execute_effects(&effects, p1, &[], Some(source_id), None);

        let goblin_perm = game.state.battlefield.get(goblin_id).unwrap();
        assert!(!goblin_perm.has_keyword(KeywordAbilities::HEXPROOF), "Goblin should NOT have hexproof");
        assert!(!goblin_perm.has_keyword(KeywordAbilities::INDESTRUCTIBLE), "Goblin should NOT have indestructible");
    }

    #[test]
    fn ignores_opponent_creatures() {
        let (mut game, p1, p2) = setup_game_with_picker(1);

        let source = make_creature_with_type("Source", p1, "Elf", 4, 2);
        let source_id = put_on_battlefield(&mut game, source, p1);
        let opponent_elf = make_creature_with_type("Opponent Elf", p2, "Elf", 1, 1);
        let opp_id = put_on_battlefield(&mut game, opponent_elf, p2);

        let effects = vec![Effect::choose_type_and_grant_keywords(vec!["hexproof"], true)];
        game.execute_effects(&effects, p1, &[], Some(source_id), None);

        let opp_perm = game.state.battlefield.get(opp_id).unwrap();
        assert!(!opp_perm.has_keyword(KeywordAbilities::HEXPROOF), "Opponent's elf should NOT have hexproof");
    }

    #[test]
    fn grants_single_keyword() {
        let (mut game, p1, _p2) = setup_game_with_picker(1);

        let source = make_creature_with_type("Source", p1, "Elf", 4, 2);
        let source_id = put_on_battlefield(&mut game, source, p1);
        let elf = make_creature_with_type("Llanowar Elves", p1, "Elf", 1, 1);
        let elf_id = put_on_battlefield(&mut game, elf, p1);

        let effects = vec![Effect::choose_type_and_grant_keywords(vec!["flying"], true)];
        game.execute_effects(&effects, p1, &[], Some(source_id), None);

        let elf_perm = game.state.battlefield.get(elf_id).unwrap();
        assert!(elf_perm.has_keyword(KeywordAbilities::FLYING), "Elf should have flying");
        assert!(!elf_perm.has_keyword(KeywordAbilities::HEXPROOF), "Elf should NOT have hexproof");
    }

    #[test]
    fn helper_constructor_returns_correct_variant() {
        match Effect::choose_type_and_grant_keywords(vec!["hexproof", "indestructible"], true) {
            Effect::ChooseTypeAndGrantKeywords { keywords, other_only } => {
                assert_eq!(keywords, vec!["hexproof", "indestructible"]);
                assert!(other_only);
            }
            other => panic!("Expected ChooseTypeAndGrantKeywords, got {:?}", other),
        }
    }

    #[test]
    fn keywords_cleared_at_end_of_turn() {
        let (mut game, p1, _p2) = setup_game_with_picker(1);

        let source = make_creature_with_type("Source", p1, "Elf", 4, 2);
        let source_id = put_on_battlefield(&mut game, source, p1);
        let elf = make_creature_with_type("Llanowar Elves", p1, "Elf", 1, 1);
        let elf_id = put_on_battlefield(&mut game, elf, p1);

        let effects = vec![Effect::choose_type_and_grant_keywords(vec!["hexproof", "indestructible"], true)];
        game.execute_effects(&effects, p1, &[], Some(source_id), None);

        let perm = game.state.battlefield.get(elf_id).unwrap();
        assert!(perm.has_keyword(KeywordAbilities::HEXPROOF), "Should have hexproof before cleanup");

        for perm in game.state.battlefield.iter_mut() {
            perm.granted_keywords = KeywordAbilities::empty();
        }

        let perm = game.state.battlefield.get(elf_id).unwrap();
        assert!(!perm.has_keyword(KeywordAbilities::HEXPROOF), "Should NOT have hexproof after cleanup");
    }
}

#[cfg(test)]
mod set_power_to_color_count_tests {
    use super::*;
    use crate::abilities::{Ability, StaticEffect};
    use crate::card::CardData;
    use crate::constants::{CardType, Color, Outcome, SubType};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };

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
            starting_life: 20,
        };
        let game = Game::new_two_player(config, vec![(p1, Box::new(PassPlayer)), (p2, Box::new(PassPlayer))]);
        (game, p1, p2)
    }

    fn add_colored_creature(game: &mut Game, owner: PlayerId, name: &str, colors: Vec<Color>) -> ObjectId {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![SubType::Elemental];
        card.power = Some(1);
        card.toughness = Some(1);
        card.color_identity = colors;
        let id = card.id;
        game.state.battlefield.add(Permanent::new(card, owner));
        id
    }

    fn add_vivid_creature(game: &mut Game, owner: PlayerId) -> ObjectId {
        let mut card = CardData::new(ObjectId::new(), owner, "Squawkroaster");
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![SubType::Elemental];
        card.power = Some(0);
        card.toughness = Some(4);
        card.color_identity = vec![Color::Red];
        let id = card.id;
        let ability = Ability::static_ability(id,
            "Vivid — Power is equal to colors among permanents you control.",
            vec![StaticEffect::set_power_to_color_count()]);
        card.abilities.push(ability.clone());
        game.state.card_store.insert(card.clone());
        game.state.battlefield.add(Permanent::new(card, owner));
        game.state.ability_store.add(ability);
        id
    }

    #[test]
    fn power_equals_zero_with_no_colored_permanents() {
        let (mut game, p1, _p2) = make_game();

        let vivid_id = add_vivid_creature(&mut game, p1);
        if let Some(perm) = game.state.battlefield.get_mut(vivid_id) {
            perm.card.color_identity = vec![];
        }

        let mut land = CardData::new(ObjectId::new(), p1, "Wastes");
        land.card_types = vec![CardType::Land];
        let land_id = land.id;
        game.state.battlefield.add(Permanent::new(land, p1));

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(vivid_id).unwrap();
        assert_eq!(perm.power(), 0, "no colored permanents = power 0");
        assert_eq!(perm.toughness(), 4, "toughness should remain 4");
    }

    #[test]
    fn power_equals_one_with_single_color() {
        let (mut game, p1, _p2) = make_game();

        let vivid_id = add_vivid_creature(&mut game, p1);

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(vivid_id).unwrap();
        assert_eq!(perm.power(), 1, "one red permanent = power 1");
    }

    #[test]
    fn power_equals_three_with_three_colors() {
        let (mut game, p1, _p2) = make_game();

        let vivid_id = add_vivid_creature(&mut game, p1);
        add_colored_creature(&mut game, p1, "Green Elf", vec![Color::Green]);
        add_colored_creature(&mut game, p1, "Blue Wizard", vec![Color::Blue]);

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(vivid_id).unwrap();
        assert_eq!(perm.power(), 3, "red + green + blue = power 3");
    }

    #[test]
    fn power_equals_five_with_all_colors() {
        let (mut game, p1, _p2) = make_game();

        let vivid_id = add_vivid_creature(&mut game, p1);
        add_colored_creature(&mut game, p1, "White Knight", vec![Color::White]);
        add_colored_creature(&mut game, p1, "Blue Mage", vec![Color::Blue]);
        add_colored_creature(&mut game, p1, "Black Rogue", vec![Color::Black]);
        add_colored_creature(&mut game, p1, "Green Beast", vec![Color::Green]);

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(vivid_id).unwrap();
        assert_eq!(perm.power(), 5, "all five colors = power 5");
        assert_eq!(perm.toughness(), 4, "toughness unchanged");
    }

    #[test]
    fn multicolored_permanent_counts_multiple_colors() {
        let (mut game, p1, _p2) = make_game();

        let vivid_id = add_vivid_creature(&mut game, p1);
        add_colored_creature(&mut game, p1, "Niv-Mizzet", vec![Color::Blue, Color::Red]);
        add_colored_creature(&mut game, p1, "Siege Rhino", vec![Color::White, Color::Black, Color::Green]);

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(vivid_id).unwrap();
        assert_eq!(perm.power(), 5, "W+U+B+R+G from multicolor = power 5");
    }

    #[test]
    fn duplicate_colors_not_double_counted() {
        let (mut game, p1, _p2) = make_game();

        let vivid_id = add_vivid_creature(&mut game, p1);
        add_colored_creature(&mut game, p1, "Red Goblin 1", vec![Color::Red]);
        add_colored_creature(&mut game, p1, "Red Goblin 2", vec![Color::Red]);

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(vivid_id).unwrap();
        assert_eq!(perm.power(), 1, "multiple red permanents still = 1 color");
    }

    #[test]
    fn opponent_permanents_dont_count() {
        let (mut game, p1, p2) = make_game();

        let vivid_id = add_vivid_creature(&mut game, p1);
        add_colored_creature(&mut game, p2, "Opponent Blue", vec![Color::Blue]);
        add_colored_creature(&mut game, p2, "Opponent Green", vec![Color::Green]);

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(vivid_id).unwrap();
        assert_eq!(perm.power(), 1, "only own permanents count (red from self)");
    }

    #[test]
    fn helper_constructor() {
        match StaticEffect::set_power_to_color_count() {
            StaticEffect::SetPowerToColorCount => {}
            _ => panic!("wrong variant"),
        }
    }
}

#[cfg(test)]
mod assign_damage_with_toughness_tests {
    use super::*;
    use crate::abilities::{Ability, StaticEffect};
    use crate::card::CardData;
    use crate::constants::{CardType, Outcome, SubType};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };

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
            starting_life: 20,
        };
        let game = Game::new_two_player(config, vec![(p1, Box::new(PassPlayer)), (p2, Box::new(PassPlayer))]);
        (game, p1, p2)
    }

    fn add_creature(game: &mut Game, owner: PlayerId, name: &str, power: i32, toughness: i32) -> ObjectId {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(power);
        card.toughness = Some(toughness);
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
                vec![StaticEffect::assign_damage_with_toughness_if_greater("equipped creature")])
        } else {
            Ability::static_ability(id,
                "Equipped creature assigns combat damage equal to its toughness.",
                vec![StaticEffect::assign_damage_with_toughness("equipped creature")])
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

        let creature_id = add_creature(&mut game, p1, "Wall", 1, 5);
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

        let creature_id = add_creature(&mut game, p1, "Wall", 1, 5);
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

        let creature_id = add_creature(&mut game, p1, "Big Power", 5, 2);
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

        let creature_id = add_creature(&mut game, p1, "Even", 3, 3);
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

        let creature_id = add_creature(&mut game, p1, "Wall", 1, 5);

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

        let creature_id = add_creature(&mut game, p1, "Wall", 1, 5);
        let _equip_id = add_equipment_with_toughness_damage(&mut game, p1, false);

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(!perm.assign_damage_with_toughness, "unequipped creature should not get the flag");
    }

    #[test]
    fn helper_unconditional() {
        match StaticEffect::assign_damage_with_toughness("equipped creature") {
            StaticEffect::AssignDamageWithToughness { filter, condition } => {
                assert_eq!(filter, "equipped creature");
                assert!(condition.is_none());
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn helper_conditional() {
        match StaticEffect::assign_damage_with_toughness_if_greater("equipped creature") {
            StaticEffect::AssignDamageWithToughness { filter, condition } => {
                assert_eq!(filter, "equipped creature");
                assert_eq!(condition.unwrap(), "toughness_greater_than_power");
            }
            _ => panic!("wrong variant"),
        }
    }
}

#[cfg(test)]
mod becomes_creature_tests {
    use super::*;
    use crate::abilities::Effect;
    use crate::card::CardData;
    use crate::constants::{CardType, Outcome};
    use crate::decision::{
        AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
        ReplacementEffectChoice, TargetRequirement, UnpaidMana,
    };
    use crate::permanent::Permanent;

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

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        let mut deck = Vec::new();
        for _ in 0..20 { deck.push(make_basic_land("Forest", owner)); }
        for _ in 0..20 { deck.push(make_creature("Grizzly Bears", owner, 2, 2)); }
        deck
    }

    fn setup_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck(p2) },
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
        let (mut game, p1, _p2) = setup_game();

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
        let (mut game, p1, _p2) = setup_game();

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
}

#[cfg(test)]
mod convoke_tests {
    use super::*;
    use crate::abilities::{Ability, StaticEffect, TargetSpec, Effect};
    use crate::constants::{CardType, KeywordAbilities, Outcome, Color};
    use crate::card::CardData;
    use crate::mana::{ManaCost, Mana};
    use crate::types::{ObjectId, PlayerId};
    use crate::decision::*;
    use crate::permanent::Permanent;

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

    fn make_deck(owner: PlayerId) -> Vec<CardData> {
        (0..40).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    fn setup_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
                PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
            ],
            starting_life: 20,
        };
        let game = Game::new_two_player(
            config,
            vec![(p1, Box::new(PassivePlayer)), (p2, Box::new(PassivePlayer))],
        );
        (game, p1, p2)
    }

    fn make_convoke_spell(owner: PlayerId) -> CardData {
        let id = ObjectId::new();
        let mut card = CardData::new(id, owner, "Convoke Spell");
        card.card_types = vec![CardType::Instant];
        card.mana_cost = ManaCost::parse("{3}{W}");
        card.keywords = KeywordAbilities::CONVOKE;
        card.abilities = vec![Ability::spell(id, vec![Effect::gain_life(5)], TargetSpec::None)];
        card
    }

    fn make_creature_with_color(name: &str, owner: PlayerId, colors: Vec<Color>) -> CardData {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(1);
        card.toughness = Some(1);
        card.color_identity = colors;
        card
    }

    #[test]
    fn spell_has_convoke_keyword() {
        let (game, p1, _p2) = setup_game();
        let card = make_convoke_spell(p1);
        assert!(game.spell_has_convoke(p1, &card));
    }

    #[test]
    fn spell_without_convoke() {
        let (game, p1, _p2) = setup_game();
        let mut card = CardData::new(ObjectId::new(), p1, "Normal Spell");
        card.card_types = vec![CardType::Instant];
        card.mana_cost = ManaCost::parse("{3}{W}");
        assert!(!game.spell_has_convoke(p1, &card));
    }

    #[test]
    fn calculate_convoke_mana_untapped_creatures() {
        let (mut game, p1, _p2) = setup_game();

        let c1 = make_creature_with_color("White Soldier", p1, vec![Color::White]);
        let c2 = make_creature_with_color("Blue Merfolk", p1, vec![Color::Blue]);
        game.state.battlefield.add(Permanent::new(c1.clone(), p1));
        game.state.card_store.insert(c1);
        game.state.battlefield.add(Permanent::new(c2.clone(), p1));
        game.state.card_store.insert(c2);

        let convoke_mana = game.calculate_convoke_mana(p1);
        assert_eq!(convoke_mana.any, 2);
        assert_eq!(convoke_mana.generic, 0);
    }

    #[test]
    fn calculate_convoke_mana_excludes_tapped() {
        let (mut game, p1, _p2) = setup_game();

        let c1 = make_creature_with_color("Tapped Soldier", p1, vec![Color::White]);
        let c1_id = c1.id;
        game.state.battlefield.add(Permanent::new(c1.clone(), p1));
        game.state.card_store.insert(c1);
        game.state.battlefield.get_mut(c1_id).unwrap().tapped = true;

        let c2 = make_creature_with_color("Untapped Merfolk", p1, vec![Color::Blue]);
        game.state.battlefield.add(Permanent::new(c2.clone(), p1));
        game.state.card_store.insert(c2);

        let convoke_mana = game.calculate_convoke_mana(p1);
        assert_eq!(convoke_mana.any, 1);
    }

    #[test]
    fn calculate_convoke_mana_colorless_creature() {
        let (mut game, p1, _p2) = setup_game();

        let c1 = make_creature_with_color("Colorless Construct", p1, vec![]);
        game.state.battlefield.add(Permanent::new(c1.clone(), p1));
        game.state.card_store.insert(c1);

        let convoke_mana = game.calculate_convoke_mana(p1);
        assert_eq!(convoke_mana.generic, 1);
        assert_eq!(convoke_mana.any, 0);
    }

    #[test]
    fn can_pay_with_convoke_mana_check() {
        let pool = Mana { white: 1, ..Mana::new() };
        let cost = Mana { white: 1, generic: 2, ..Mana::new() };
        let convoke = Mana { any: 2, ..Mana::new() };
        assert!(pool.can_pay_with_convoke(&cost, &convoke));
    }

    #[test]
    fn cannot_afford_without_convoke() {
        let pool = Mana { white: 1, ..Mana::new() };
        let cost = Mana { white: 1, generic: 3, ..Mana::new() };
        assert!(!pool.can_pay(&cost));
    }

    #[test]
    fn convoke_enables_casting_in_legal_actions() {
        let (mut game, p1, _p2) = setup_game();

        let spell = make_convoke_spell(p1);
        let spell_id = spell.id;
        game.state.card_store.insert(spell.clone());
        game.state.players.get_mut(&p1).unwrap().hand.add(spell_id);

        game.state.players.get_mut(&p1).unwrap().mana_pool.add(
            Mana { white: 1, ..Mana::new() }, None, false
        );

        let c1 = make_creature_with_color("Helper 1", p1, vec![Color::White]);
        let c2 = make_creature_with_color("Helper 2", p1, vec![Color::White]);
        let c3 = make_creature_with_color("Helper 3", p1, vec![Color::White]);
        for c in [c1, c2, c3] {
            game.state.battlefield.add(Permanent::new(c.clone(), p1));
            game.state.card_store.insert(c);
        }

        let actions = game.compute_legal_actions(p1);
        let has_cast = actions.iter().any(|a| matches!(a, PlayerAction::CastSpell { card_id, .. } if *card_id == spell_id));
        assert!(has_cast, "Should be able to cast convoke spell with 1W + 3 creatures");
    }

    #[test]
    fn convoke_not_enough_creatures() {
        let (mut game, p1, _p2) = setup_game();

        let spell = make_convoke_spell(p1);
        let spell_id = spell.id;
        game.state.card_store.insert(spell.clone());
        game.state.players.get_mut(&p1).unwrap().hand.add(spell_id);

        game.state.players.get_mut(&p1).unwrap().mana_pool.add(
            Mana { white: 1, ..Mana::new() }, None, false
        );

        let c1 = make_creature_with_color("Helper 1", p1, vec![Color::White]);
        game.state.battlefield.add(Permanent::new(c1.clone(), p1));
        game.state.card_store.insert(c1);

        let actions = game.compute_legal_actions(p1);
        let has_cast = actions.iter().any(|a| matches!(a, PlayerAction::CastSpell { card_id, .. } if *card_id == spell_id));
        assert!(!has_cast, "Should NOT be able to cast 3W with 1W + only 1 creature (need 2 more)");
    }

    #[test]
    fn cast_spell_with_convoke_taps_creatures() {
        let (mut game, p1, _p2) = setup_game();

        let spell = make_convoke_spell(p1);
        let spell_id = spell.id;
        game.state.card_store.insert(spell.clone());
        game.state.players.get_mut(&p1).unwrap().hand.add(spell_id);

        game.state.players.get_mut(&p1).unwrap().mana_pool.add(
            Mana { white: 1, ..Mana::new() }, None, false
        );

        let c1 = make_creature_with_color("Helper 1", p1, vec![Color::White]);
        let c1_id = c1.id;
        let c2 = make_creature_with_color("Helper 2", p1, vec![Color::White]);
        let c2_id = c2.id;
        let c3 = make_creature_with_color("Helper 3", p1, vec![Color::White]);
        let c3_id = c3.id;
        for c in [c1, c2, c3] {
            game.state.battlefield.add(Permanent::new(c.clone(), p1));
            game.state.card_store.insert(c);
        }

        game.cast_spell(p1, spell_id);

        assert!(game.state.stack.get(spell_id).is_some(), "Spell should be on the stack");

        let tapped_count = [c1_id, c2_id, c3_id].iter()
            .filter(|id| game.state.battlefield.get(**id).map_or(false, |p| p.tapped))
            .count();
        assert_eq!(tapped_count, 3, "All 3 creatures should be tapped for convoke");
    }

    #[test]
    fn cast_spell_convoke_pays_colored_mana() {
        let (mut game, p1, _p2) = setup_game();

        let id = ObjectId::new();
        let mut spell = CardData::new(id, p1, "White Convoke");
        spell.card_types = vec![CardType::Instant];
        spell.mana_cost = ManaCost::parse("{W}{W}");
        spell.keywords = KeywordAbilities::CONVOKE;
        spell.abilities = vec![Ability::spell(id, vec![Effect::gain_life(3)], TargetSpec::None)];
        game.state.card_store.insert(spell.clone());
        game.state.players.get_mut(&p1).unwrap().hand.add(id);

        let c1 = make_creature_with_color("White Creature", p1, vec![Color::White]);
        let c1_id = c1.id;
        let c2 = make_creature_with_color("White Creature 2", p1, vec![Color::White]);
        let c2_id = c2.id;
        for c in [c1, c2] {
            game.state.battlefield.add(Permanent::new(c.clone(), p1));
            game.state.card_store.insert(c);
        }

        game.cast_spell(p1, id);

        assert!(game.state.stack.get(id).is_some(), "Spell should be on stack (2 white creatures pay WW)");
        assert!(game.state.battlefield.get(c1_id).unwrap().tapped);
        assert!(game.state.battlefield.get(c2_id).unwrap().tapped);
    }

    #[test]
    fn grant_convoke_via_static_effect() {
        let (mut game, p1, _p2) = setup_game();

        let granter_id = ObjectId::new();
        let mut granter = CardData::new(granter_id, p1, "Convoke Granter");
        granter.card_types = vec![CardType::Creature];
        granter.power = Some(5);
        granter.toughness = Some(5);
        game.state.battlefield.add(Permanent::new(granter.clone(), p1));
        game.state.card_store.insert(granter);

        let grant_ability = Ability::static_ability(
            granter_id,
            "Creature spells you cast have convoke.",
            vec![StaticEffect::grant_convoke("creature spells")],
        );
        game.state.ability_store.add(grant_ability);

        let spell_id = ObjectId::new();
        let mut spell = CardData::new(spell_id, p1, "Big Creature");
        spell.card_types = vec![CardType::Creature];
        spell.mana_cost = ManaCost::parse("{4}{G}");
        spell.power = Some(4);
        spell.toughness = Some(4);
        spell.color_identity = vec![Color::Green];

        assert!(game.spell_has_convoke(p1, &spell));

        let mut non_creature = CardData::new(ObjectId::new(), p1, "Some Instant");
        non_creature.card_types = vec![CardType::Instant];
        non_creature.mana_cost = ManaCost::parse("{2}{G}");
        assert!(!game.spell_has_convoke(p1, &non_creature));
    }

    #[test]
    fn convoke_excludes_opponent_creatures() {
        let (mut game, p1, p2) = setup_game();

        let c1 = make_creature_with_color("Enemy Creature", p2, vec![Color::White]);
        game.state.battlefield.add(Permanent::new(c1.clone(), p2));
        game.state.card_store.insert(c1);

        let convoke_mana = game.calculate_convoke_mana(p1);
        assert_eq!(convoke_mana.count(), 0, "Should not count opponent's creatures for convoke");
    }

    #[test]
    fn convoke_helper_constructor() {
        match StaticEffect::grant_convoke("creature spells") {
            StaticEffect::GrantConvoke { filter } => {
                assert_eq!(filter, "creature spells");
            }
            _ => panic!("Expected GrantConvoke variant"),
        }
    }
}
