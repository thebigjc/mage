// Basic game mechanics tests

use super::*;
use crate::game::{Game, GameConfig, PlayerConfig};
use crate::permanent::Permanent;
use crate::counters::CounterType;
use crate::constants::{SuperType, PhaseStep};

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
    perm.add_counters(CounterType::P1P1, 3);
    perm.add_counters(CounterType::M1M1, 2);
    game.state.battlefield.add(perm);

    // Process SBAs
    game.process_state_based_actions();

    // After annihilation: 3 P1P1 - 2 M1M1 = 1 P1P1 remaining, 0 M1M1
    let perm = game.state.battlefield.get(card_id).unwrap();
    assert_eq!(perm.counters.get(&CounterType::P1P1), 1);
    assert_eq!(perm.counters.get(&CounterType::M1M1), 0);
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
    card1.supertypes = vec![SuperType::Legendary];
    card1.power = Some(2);
    card1.toughness = Some(1);
    card1.keywords = KeywordAbilities::empty();
    let id1 = card1.id;

    let mut card2 = CardData::new(ObjectId::new(), p1, "Thalia");
    card2.card_types = vec![CardType::Creature];
    card2.supertypes = vec![SuperType::Legendary];
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

    // Set up main phase
    game.turn_manager.set_phase_step(PhaseStep::PrecombatMain);

    // Put a creature on the battlefield with an activated ability
    let creature_id = ObjectId::new();
    let mut creature = make_creature("Test Creature", p1, 2, 2);
    let activated_ability = Ability::activated(
        creature_id,
        "{T}: Deal 1 damage to any target",
        vec![Cost::tap_self()],
        vec![Effect::DealDamageAny { amount: 1 }],
        TargetSpec::AnyTarget,
    );
    let ability_id = activated_ability.id;
    creature.abilities.push(activated_ability);

    let perm = Permanent::new(creature.clone(), p1);
    game.state.battlefield.add(perm);
    game.state.card_store.insert(creature.clone());
    for ability in &creature.abilities {
        game.state.ability_store.add(ability.clone());
    }

    // Clear summoning sickness
    let perm = game.state.battlefield.get_mut(creature_id).unwrap();
    perm.entered_this_turn = false;

    // Activate the ability targeting player 2
    game.activate_ability(
        p1,
        creature_id,
        ability_id,
        vec![p2.as_target()],
    );

    // Check that the stack has the ability
    assert_eq!(game.state.stack.len(), 1);
    let stack_item = &game.state.stack[0];
    assert_eq!(stack_item.controller, p1);

    // The permanent should be tapped from the cost
    let perm = game.state.battlefield.get(creature_id).unwrap();
    assert!(perm.tapped);

    // Pass priority and let it resolve
    game.pass_priority(p1);
    game.pass_priority(p2);

    // Stack should be empty after resolution
    assert_eq!(game.state.stack.len(), 0);

    // Player 2 should have taken 1 damage
    let player2 = game.state.players.get(&p2).unwrap();
    assert_eq!(player2.life, 19);
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

    // Set up main phase
    game.turn_manager.set_phase_step(PhaseStep::PrecombatMain);

    // Create a spell that deals 3 damage to any target
    let spell_id = ObjectId::new();
    let mut spell = CardData::new(spell_id, p1, "Lightning Bolt");
    spell.card_types = vec![CardType::Instant];
    spell.mana_cost = crate::mana::ManaCost::parse("{R}");
    spell.abilities.push(Ability::spell(
        spell_id,
        vec![Effect::DealDamageAny { amount: 3 }],
        TargetSpec::AnyTarget,
    ));

    // Add red mana to player 1's pool
    let player = game.state.players.get_mut(&p1).unwrap();
    player.mana_pool.add(Mana::red(1));

    // Put the card in player 1's hand
    player.hand.push(spell_id);
    game.state.card_store.insert(spell.clone());
    for ability in &spell.abilities {
        game.state.ability_store.add(ability.clone());
    }

    // Cast the spell targeting player 2
    game.cast_spell(p1, spell_id, vec![p2.as_target()]);

    // Spell should be on the stack
    assert_eq!(game.state.stack.len(), 1);
    let stack_item = &game.state.stack[0];
    assert_eq!(stack_item.controller, p1);

    // Resolve the spell
    game.pass_priority(p1);
    game.pass_priority(p2);

    // Stack should be empty
    assert_eq!(game.state.stack.len(), 0);

    // Player 2 should have taken 3 damage
    let player2 = game.state.players.get(&p2).unwrap();
    assert_eq!(player2.life, 17);

    // Spell should be in the graveyard
    let player1 = game.state.players.get(&p1).unwrap();
    assert!(player1.graveyard.contains(&spell_id));
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

    // Add a target creature
    let bear = make_creature("Bear", p2, 2, 2);
    let bear_id = bear.id;
    game.state.battlefield.add(Permanent::new(bear, p2));

    // Create a destroy spell
    let spell_id = ObjectId::new();
    let mut spell = CardData::new(spell_id, p1, "Doom Blade");
    spell.card_types = vec![CardType::Instant];
    spell.abilities.push(Ability::spell(
        spell_id,
        vec![Effect::Destroy],
        TargetSpec::Creature,
    ));

    // Put the card in player 1's hand
    let player = game.state.players.get_mut(&p1).unwrap();
    player.hand.push(spell_id);
    game.state.card_store.insert(spell.clone());
    for ability in &spell.abilities {
        game.state.ability_store.add(ability.clone());
    }

    // Cast the spell targeting the bear
    game.cast_spell(p1, spell_id, vec![bear_id]);

    // Spell should be on the stack
    assert_eq!(game.state.stack.len(), 1);

    // Remove the bear before the spell resolves
    game.state.battlefield.remove(bear_id);

    // Try to resolve the spell - it should fizzle
    game.pass_priority(p1);
    game.pass_priority(p2);

    // Stack should be empty
    assert_eq!(game.state.stack.len(), 0);

    // Spell should still end up in graveyard (fizzled spells go to graveyard)
    let player1 = game.state.players.get(&p1).unwrap();
    assert!(player1.graveyard.contains(&spell_id));
}