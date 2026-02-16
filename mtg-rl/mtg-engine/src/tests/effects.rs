// Effect execution tests

use crate::game::{Game, GameConfig, PlayerConfig};
use crate::permanent::Permanent;
use crate::counters::CounterType;
use crate::constants::Outcome;
use crate::decision::{AttackerInfo, DamageAssignment};
use crate::types::PlayerId;
use crate::abilities::{Effect, Cost};
use crate::constants::SubType;
use crate::card::CardData;
use crate::constants::{CardType, KeywordAbilities};
use crate::decision::{GameView, NamedChoice, PlayerAction, PlayerDecisionMaker, ReplacementEffectChoice, TargetRequirement, UnpaidMana};
use crate::types::ObjectId;

fn make_deck(owner: PlayerId) -> Vec<CardData> {
    (0..20).map(|i| {
        let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
        c.card_types = vec![CardType::Land];
        c
    }).collect()
}

fn make_creature(name: &str, owner: PlayerId, power: i32, toughness: i32) -> CardData {
    let mut card = CardData::new(ObjectId::new(), owner, name);
    card.card_types = vec![CardType::Creature];
    card.power = Some(power);
    card.toughness = Some(toughness);
    card.keywords = KeywordAbilities::empty();
    card
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

#[cfg(test)]
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
    assert!(game.pay_costs(p1, source_id, &[Cost::tap_self()]));
    let perm = game.state.battlefield.get(source_id).unwrap();
    assert!(perm.tapped);

    // Can't pay tap again (already tapped)
    assert!(!game.pay_costs(p1, source_id, &[Cost::tap_self()]));

    // Pay sacrifice self cost
    assert!(game.pay_costs(p1, source_id, &[Cost::sacrifice_self()]));
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
        None,
    );

    let perm = game.state.battlefield.get(source_id).unwrap();
    assert_eq!(perm.counters.get(&CounterType::M1M1), 2);

    // Execute RemoveCounters with no targets but with source — should remove from self
    game.execute_effects(
        &[Effect::RemoveCounters { counter_type: "-1/-1".into(), count: 1 }],
        p1,
        &[],
        Some(source_id),
        None,
    );

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
        None,
    );

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
        None,
    );

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
        None,
    );

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

    // My creature should have +1/+1 counter
    let my = game.state.battlefield.get(my_id).unwrap();
    assert_eq!(my.counters.get(&CounterType::P1P1), 1);
    assert_eq!(my.power(), 4); // 3 + 1 from counter
    assert_eq!(my.remaining_toughness(), 4); // 3 base + 1 from counter, no damage taken

    // Opponent creature took damage but no counter
    let opp = game.state.battlefield.get(opp_id).unwrap();
    assert_eq!(opp.counters.get(&CounterType::P1P1), 0);
    assert_eq!(opp.remaining_toughness(), 0); // 4 - 4 damage = 0
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

    // Create three creatures for p1, one for p2
    for i in 0..3 {
        let mut creature = make_creature(&format!("Elf {}", i), p1, 2, 2);
        creature.subtypes = vec![SubType::Elf];
        game.state.battlefield.add(Permanent::new(creature, p1));
    }

    let mut opp_creature = make_creature("Goblin", p2, 3, 3);
    opp_creature.subtypes = vec![SubType::Goblin];
    let opp_id = opp_creature.id;
    game.state.battlefield.add(Permanent::new(opp_creature, p2));

    // Add +1/+1 counter to each Elf you control
    game.execute_effects(
        &[Effect::add_counters_all("+1/+1", 1, "each Elf you control")],
        p1, &[], None, None,
    );

    // Count Elves with counters
    let elves_with_counters = game.state.battlefield.iter()
        .filter(|p| p.controller == p1 && p.has_subtype(&SubType::Elf))
        .filter(|p| p.counters.get(&CounterType::P1P1) == 1)
        .count();

    assert_eq!(elves_with_counters, 3);
    // Opponent's Goblin should have no counters
    assert_eq!(game.state.battlefield.get(opp_id).unwrap().counters.get(&CounterType::P1P1), 0);
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

    // Mark top 3 cards to track them
    let lib_before = game.state.players.get(&p1).unwrap().library.len();
    let top_cards: Vec<ObjectId> = game.state.players.get(&p1).unwrap().library.peek(3).to_vec();

    // Look at top 3, pick lands to hand, rest to graveyard
    game.execute_effects(
        &[Effect::look_top_and_pick(3, "land card")],
        p1, &[], None, None,
    );

    let lib_after = game.state.players.get(&p1).unwrap().library.len();
    assert_eq!(lib_after, lib_before - 1, "One card should move from library to hand");

    // One card picked to hand, rest put on bottom of library
    let hand_ids = game.state.players.get(&p1).unwrap().hand.as_slice().to_vec();
    let picked_count = top_cards.iter().filter(|id| hand_ids.contains(id)).count();
    assert_eq!(picked_count, 1, "Exactly one card should be picked to hand");
}

#[test]
fn gain_control_until_end_of_turn() {
    // Test that GainControlUntilEndOfTurn changes controller, untaps, grants haste.
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

    // P2 has a tapped creature
    let creature = make_creature("Stolen Bear", p2, 3, 3);
    let creature_id = creature.id;
    let mut perm = Permanent::new(creature, p2);
    perm.tapped = true; // Start tapped
    game.state.battlefield.add(perm);

    // P1 gains control
    game.execute_effects(
        &[Effect::gain_control_eot()],
        p1,
        &[creature_id],
        None, None,
    );

    let perm = game.state.battlefield.get(creature_id).unwrap();
    // Controller changed to P1
    assert_eq!(perm.controller, p1);
    // Creature untapped
    assert!(!perm.tapped);
    // Creature has haste
    assert!(perm.has_keyword(KeywordAbilities::HASTE));
}

#[cfg(test)]
#[test]
fn boost_by_toughness_minus_power_applies_diff() {
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
        vec![
            (p1, Box::new(AlwaysPassPlayer) as Box<dyn crate::decision::PlayerDecisionMaker>),
            (p2, Box::new(AlwaysPassPlayer) as Box<dyn crate::decision::PlayerDecisionMaker>),
        ],
    );

    let creature_id = ObjectId::new();
    let card = make_creature("Wall", p1, 1, 5);
    let mut card_clone = card.clone();
    card_clone.id = creature_id;
    game.state.card_store.insert(card_clone.clone());
    let perm = Permanent::new(card_clone, p1);
    game.state.battlefield.add(perm);

    assert_eq!(game.state.battlefield.get(creature_id).unwrap().power(), 1);
    assert_eq!(game.state.battlefield.get(creature_id).unwrap().toughness(), 5);

    game.execute_effects(
        &[Effect::boost_by_toughness_minus_power()],
        p1,
        &[creature_id],
        None, None,
    );

    let perm = game.state.battlefield.get(creature_id).unwrap();
    assert_eq!(perm.power(), 5);
    assert_eq!(perm.toughness(), 9);
}

#[cfg(test)]
#[test]
fn boost_by_toughness_minus_power_zero_when_equal() {
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
        vec![
            (p1, Box::new(AlwaysPassPlayer) as Box<dyn crate::decision::PlayerDecisionMaker>),
            (p2, Box::new(AlwaysPassPlayer) as Box<dyn crate::decision::PlayerDecisionMaker>),
        ],
    );

    let creature_id = ObjectId::new();
    let card = make_creature("Bear", p1, 3, 3);
    let mut card_clone = card.clone();
    card_clone.id = creature_id;
    game.state.card_store.insert(card_clone.clone());
    let perm = Permanent::new(card_clone, p1);
    game.state.battlefield.add(perm);

    game.execute_effects(
        &[Effect::boost_by_toughness_minus_power()],
        p1,
        &[creature_id],
        None, None,
    );

    let perm = game.state.battlefield.get(creature_id).unwrap();
    assert_eq!(perm.power(), 3);
    assert_eq!(perm.toughness(), 3);
}

#[cfg(test)]
#[test]
fn boost_by_toughness_minus_power_no_boost_when_power_greater() {
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
        vec![
            (p1, Box::new(AlwaysPassPlayer) as Box<dyn crate::decision::PlayerDecisionMaker>),
            (p2, Box::new(AlwaysPassPlayer) as Box<dyn crate::decision::PlayerDecisionMaker>),
        ],
    );

    let creature_id = ObjectId::new();
    let card = make_creature("Giant", p1, 5, 2);
    let mut card_clone = card.clone();
    card_clone.id = creature_id;
    game.state.card_store.insert(card_clone.clone());
    let perm = Permanent::new(card_clone, p1);
    game.state.battlefield.add(perm);

    game.execute_effects(
        &[Effect::boost_by_toughness_minus_power()],
        p1,
        &[creature_id],
        None, None,
    );

    let perm = game.state.battlefield.get(creature_id).unwrap();
    assert_eq!(perm.power(), 5);
    assert_eq!(perm.toughness(), 2);
}

#[cfg(test)]
#[test]
fn boost_by_toughness_minus_power_helper_constructor() {
    let effect = Effect::boost_by_toughness_minus_power();
    assert!(matches!(effect, Effect::BoostByToughnessMinusPower));
}

#[cfg(test)]
#[test]
fn bounce_all_returns_matching_creatures_to_hand() {
    let p1 = PlayerId::new();
    let p2 = PlayerId::new();
    let config = GameConfig {
        players: vec![
            PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
            PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
        ],
        starting_life: 20,
    };
    let mut game = Game::new_two_player(config, vec![
        (p1, Box::new(AlwaysPassPlayer)),
        (p2, Box::new(AlwaysPassPlayer)),
    ]);
    let mut elf = make_creature("Elf", p1, 1, 1);
    elf.subtypes = vec![SubType::Elf];
    let elf_id = elf.id;
    let mut goblin = make_creature("Goblin", p1, 2, 2);
    goblin.subtypes = vec![SubType::Goblin];
    let goblin_id = goblin.id;
    game.state.battlefield.add(Permanent::new(elf, p1));
    game.state.battlefield.add(Permanent::new(goblin, p1));
    assert_eq!(game.state.battlefield.len(), 2);
    game.execute_effects(&[Effect::bounce_all("elf")], p1, &[], None, None);
    assert_eq!(game.state.battlefield.len(), 1);
    assert!(game.state.battlefield.get(goblin_id).is_some());
    assert!(game.state.battlefield.get(elf_id).is_none());
}

#[cfg(test)]
#[test]
fn bounce_all_non_type_filter() {
    let p1 = PlayerId::new();
    let p2 = PlayerId::new();
    let config = GameConfig {
        players: vec![
            PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
            PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
        ],
        starting_life: 20,
    };
    let mut game = Game::new_two_player(config, vec![
        (p1, Box::new(AlwaysPassPlayer)),
        (p2, Box::new(AlwaysPassPlayer)),
    ]);
    let mut elemental = make_creature("Fire Elemental", p1, 5, 5);
    elemental.subtypes = vec![SubType::Elemental];
    let elemental_id = elemental.id;
    let mut human = make_creature("Human", p1, 2, 2);
    human.subtypes = vec![SubType::Human];
    let human_id = human.id;
    let mut elf = make_creature("Elf", p2, 1, 1);
    elf.subtypes = vec![SubType::Elf];
    let elf_id = elf.id;
    game.state.battlefield.add(Permanent::new(elemental, p1));
    game.state.battlefield.add(Permanent::new(human, p1));
    game.state.battlefield.add(Permanent::new(elf, p2));
    game.execute_effects(&[Effect::bounce_all("non-Elemental creatures")], p1, &[], None, None);
    assert!(game.state.battlefield.get(elemental_id).is_some());
    assert!(game.state.battlefield.get(human_id).is_none());
    assert!(game.state.battlefield.get(elf_id).is_none());
}

#[cfg(test)]
#[test]
fn exile_from_opponent_library_exiles_cards() {
    let p1 = PlayerId::new();
    let p2 = PlayerId::new();
    let config = GameConfig {
        players: vec![
            PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
            PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
        ],
        starting_life: 20,
    };
    let mut game = Game::new_two_player(config, vec![
        (p1, Box::new(AlwaysPassPlayer)),
        (p2, Box::new(AlwaysPassPlayer)),
    ]);
    let initial_lib = game.state.players.get(&p2).unwrap().library.len();
    game.execute_effects(&[Effect::exile_from_opponent_library(2)], p1, &[], None, None);
    let final_lib = game.state.players.get(&p2).unwrap().library.len();
    assert_eq!(final_lib, initial_lib - 2);
    assert_eq!(game.state.exile.len(), 2);
}

#[cfg(test)]
#[test]
fn exile_from_opponent_library_does_not_exile_own() {
    let p1 = PlayerId::new();
    let p2 = PlayerId::new();
    let config = GameConfig {
        players: vec![
            PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
            PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
        ],
        starting_life: 20,
    };
    let mut game = Game::new_two_player(config, vec![
        (p1, Box::new(AlwaysPassPlayer)),
        (p2, Box::new(AlwaysPassPlayer)),
    ]);
    let initial_lib_p1 = game.state.players.get(&p1).unwrap().library.len();
    game.execute_effects(&[Effect::exile_from_opponent_library(3)], p1, &[], None, None);
    let final_lib_p1 = game.state.players.get(&p1).unwrap().library.len();
    assert_eq!(final_lib_p1, initial_lib_p1);
}

#[cfg(test)]
#[test]
fn become_all_colors_sets_flag() {
    let p1 = PlayerId::new();
    let p2 = PlayerId::new();
    let config = GameConfig {
        players: vec![
            PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
            PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
        ],
        starting_life: 20,
    };
    let mut game = Game::new_two_player(config, vec![
        (p1, Box::new(AlwaysPassPlayer)),
        (p2, Box::new(AlwaysPassPlayer)),
    ]);
    let creature = make_creature("Test", p1, 2, 2);
    let creature_id = creature.id;
    game.state.battlefield.add(Permanent::new(creature, p1));
    assert!(!game.state.battlefield.get(creature_id).unwrap().all_colors_until_eot);
    game.execute_effects(&[Effect::become_all_colors()], p1, &[creature_id], None, None);
    assert!(game.state.battlefield.get(creature_id).unwrap().all_colors_until_eot);
}

#[cfg(test)]
#[test]
fn become_all_colors_makes_5_colors_counted() {
    let p1 = PlayerId::new();
    let p2 = PlayerId::new();
    let config = GameConfig {
        players: vec![
            PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
            PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
        ],
        starting_life: 20,
    };
    let mut game = Game::new_two_player(config, vec![
        (p1, Box::new(AlwaysPassPlayer)),
        (p2, Box::new(AlwaysPassPlayer)),
    ]);
    let creature = make_creature("Test", p1, 2, 2);
    let creature_id = creature.id;
    game.state.battlefield.add(Permanent::new(creature, p1));
    game.execute_effects(&[Effect::become_all_colors()], p1, &[creature_id], None, None);
    game.execute_effects(&[Effect::GainLifeVivid], p1, &[], None, None);
    assert_eq!(game.state.players.get(&p1).unwrap().life, 25);
}

#[cfg(test)]
#[test]
fn cost_reduction_dynamic_greatest_mv() {
    use crate::abilities::{Ability, StaticEffect};
    use crate::mana::ManaCost;
    let p1 = PlayerId::new();
    let p2 = PlayerId::new();
    let config = GameConfig {
        players: vec![
            PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
            PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
        ],
        starting_life: 20,
    };
    let mut game = Game::new_two_player(config, vec![
        (p1, Box::new(AlwaysPassPlayer)),
        (p2, Box::new(AlwaysPassPlayer)),
    ]);
    let mut elemental = make_creature("Fire Elemental", p1, 5, 5);
    elemental.subtypes = vec![SubType::Elemental];
    elemental.mana_cost = ManaCost::parse("{3}{R}{R}");
    game.state.battlefield.add(Permanent::new(elemental, p1));
    let mut spell = make_creature("Big Creature", p1, 7, 7);
    spell.mana_cost = ManaCost::parse("{7}{U}{U}");
    spell.abilities = vec![
        Ability::static_ability(spell.id,
            "This spell costs {X} less.",
            vec![StaticEffect::cost_reduction_dynamic("creature spells", "greatest mana value among Elementals you control")]),
    ];
    let reduction = game.calculate_cost_reduction(p1, &spell);
    assert_eq!(reduction, 5);
}

#[cfg(test)]
#[test]
fn cost_reduction_dynamic_no_matching_creatures() {
    use crate::abilities::{Ability, StaticEffect};
    let p1 = PlayerId::new();
    let p2 = PlayerId::new();
    let config = GameConfig {
        players: vec![
            PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
            PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
        ],
        starting_life: 20,
    };
    let game = Game::new_two_player(config, vec![
        (p1, Box::new(AlwaysPassPlayer)),
        (p2, Box::new(AlwaysPassPlayer)),
    ]);
    let mut spell = make_creature("Big Creature", p1, 7, 7);
    spell.abilities = vec![
        Ability::static_ability(spell.id,
            "This spell costs {X} less.",
            vec![StaticEffect::cost_reduction_dynamic("creature spells", "greatest mana value among Elementals you control")]),
    ];
    let reduction = game.calculate_cost_reduction(p1, &spell);
    assert_eq!(reduction, 0);
}

#[cfg(test)]
#[test]
fn bounce_all_helper_constructor() {
    let effect = Effect::bounce_all("elf");
    assert!(matches!(effect, Effect::BounceAll { .. }));
}

#[cfg(test)]
#[test]
fn exile_from_opponent_library_helper_constructor() {
    let effect = Effect::exile_from_opponent_library(3);
    assert!(matches!(effect, Effect::ExileFromOpponentLibrary { count: 3 }));
}

#[cfg(test)]
#[test]
fn become_all_colors_helper_constructor() {
    let effect = Effect::become_all_colors();
    assert!(matches!(effect, Effect::BecomeAllColors));
}

#[cfg(test)]
#[test]
fn conditional_target_is_subtype_true() {
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

    let goat_id = ObjectId::new();
    let mut goat = CardData::new(goat_id, p2, "Mountain Goat");
    goat.card_types = vec![CardType::Creature];
    goat.subtypes = vec![SubType::Goat];
    goat.power = Some(1);
    goat.toughness = Some(1);
    goat.keywords = KeywordAbilities::empty();
    game.state.battlefield.add(Permanent::new(goat, p2));

    game.execute_effects(
        &[Effect::conditional("target is a Goat", vec![Effect::boost_until_eot(3, 0)], vec![])],
        p1, &[goat_id], None, None,
    );

    let perm = game.state.battlefield.get(goat_id).unwrap();
    assert_eq!(perm.power(), 4);
    assert_eq!(perm.toughness(), 4);
}

#[cfg(test)]
#[test]
fn conditional_target_is_subtype_false() {
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
    let mut bear = CardData::new(bear_id, p2, "Grizzly Bears");
    bear.card_types = vec![CardType::Creature];
    bear.subtypes = vec![SubType::Bear];
    bear.power = Some(2);
    bear.toughness = Some(2);
    bear.keywords = KeywordAbilities::empty();
    game.state.battlefield.add(Permanent::new(bear, p2));

    game.execute_effects(
        &[Effect::conditional("target is a Goat", vec![Effect::boost_until_eot(3, 0)], vec![])],
        p1, &[bear_id], None, None,
    );

    let perm = game.state.battlefield.get(bear_id).unwrap();
    assert_eq!(perm.power(), 2);
    assert_eq!(perm.toughness(), 2);
}

#[cfg(test)]
#[test]
fn conditional_count_lands_and_or_treefolk_true() {
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

    for i in 0..6 {
        let land_id = ObjectId::new();
        let mut land = CardData::new(land_id, p1, &format!("Forest {i}"));
        land.card_types = vec![CardType::Land];
        game.state.battlefield.add(Permanent::new(land, p1));
    }
    let tree_id = ObjectId::new();
    let mut tree = CardData::new(tree_id, p1, "Treefolk Warrior");
    tree.card_types = vec![CardType::Creature];
    tree.subtypes = vec![SubType::Treefolk];
    tree.power = Some(3);
    tree.toughness = Some(4);
    tree.keywords = KeywordAbilities::empty();
    game.state.battlefield.add(Permanent::new(tree, p1));

    game.execute_effects(
        &[Effect::conditional("you control 7 or more lands and/or Treefolk",
            vec![Effect::gain_life(5)], vec![])],
        p1, &[], None, None,
    );

    assert_eq!(game.state.players.get(&p1).unwrap().life, 25);
}

#[cfg(test)]
#[test]
fn conditional_count_lands_and_or_treefolk_false() {
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

    for i in 0..3 {
        let land_id = ObjectId::new();
        let mut land = CardData::new(land_id, p1, &format!("Forest {i}"));
        land.card_types = vec![CardType::Land];
        game.state.battlefield.add(Permanent::new(land, p1));
    }

    game.execute_effects(
        &[Effect::conditional("you control 7 or more lands and/or Treefolk",
            vec![Effect::gain_life(5)], vec![Effect::lose_life(1)])],
        p1, &[], None, None,
    );

    assert_eq!(game.state.players.get(&p1).unwrap().life, 19);
}

#[cfg(test)]
#[test]
fn conditional_you_control_a_merfolk() {
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

    let merfolk_id = ObjectId::new();
    let mut merfolk = CardData::new(merfolk_id, p1, "Silvergill Adept");
    merfolk.card_types = vec![CardType::Creature];
    merfolk.subtypes = vec![SubType::Merfolk];
    merfolk.power = Some(2);
    merfolk.toughness = Some(1);
    merfolk.keywords = KeywordAbilities::empty();
    game.state.battlefield.add(Permanent::new(merfolk, p1));

    let hand_before = game.state.players.get(&p1).unwrap().hand.len();
    let source_id = ObjectId::new();
    game.execute_effects(
        &[Effect::conditional("you control a Merfolk", vec![Effect::draw_cards(1)], vec![])],
        p1, &[], Some(source_id), None,
    );

    assert_eq!(game.state.players.get(&p1).unwrap().hand.len(), hand_before + 1);
}

#[cfg(test)]
#[test]
fn conditional_helper_constructor() {
    let effect = Effect::conditional("target is a Goat", vec![Effect::gain_life(2)], vec![]);
    assert!(matches!(effect, Effect::Conditional { .. }));
}

#[test]
fn mill_and_select_puts_creature_on_top() {
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

    let creature = CardData::new(ObjectId::new(), p1, "Hidden Bear");
    let creature_id = creature.id;
    let mut creature = creature;
    creature.card_types = vec![CardType::Creature];
    creature.power = Some(2);
    creature.toughness = Some(2);
    if let Some(player) = game.state.players.get_mut(&p1) {
        player.library.put_on_top(creature_id);
        game.state.card_store.insert(creature);
    }

    let lib_before = game.state.players.get(&p1).unwrap().library.len();
    let gy_before = game.state.players.get(&p1).unwrap().graveyard.len();

    game.execute_effects(
        &[Effect::mill_and_select(4, "creature or land", "top")],
        p1, &[], None, None,
    );

    let lib_after = game.state.players.get(&p1).unwrap().library.len();
    let gy_after = game.state.players.get(&p1).unwrap().graveyard.len();

    assert_eq!(lib_after, lib_before - 4 + 1, "4 milled, 1 put back on top");
    assert_eq!(gy_after, gy_before + 3, "3 cards remain in graveyard");

    let top = game.state.players.get(&p1).unwrap().library.peek(1);
    assert_eq!(top[0], creature_id, "Creature should be on top of library");
}

#[test]
fn mill_and_select_to_hand() {
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

    let creature = CardData::new(ObjectId::new(), p1, "Hidden Bear");
    let creature_id = creature.id;
    let mut creature = creature;
    creature.card_types = vec![CardType::Creature];
    creature.power = Some(2);
    creature.toughness = Some(2);
    if let Some(player) = game.state.players.get_mut(&p1) {
        player.library.put_on_top(creature_id);
        game.state.card_store.insert(creature);
    }

    let hand_before = game.state.players.get(&p1).unwrap().hand.len();

    game.execute_effects(
        &[Effect::mill_and_select(4, "creature", "hand")],
        p1, &[], None, None,
    );

    let hand_after = game.state.players.get(&p1).unwrap().hand.len();
    assert_eq!(hand_after, hand_before + 1, "Creature should be in hand");
    let hand_ids = game.state.players.get(&p1).unwrap().hand.as_slice().to_vec();
    assert!(hand_ids.contains(&creature_id), "The creature should be in hand");
}

#[test]
fn mill_and_return_all_returns_matching_cards() {
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

    let goblin1 = {
        let mut c = CardData::new(ObjectId::new(), p1, "Goblin A");
        c.card_types = vec![CardType::Creature];
        c.subtypes = vec![SubType::Goblin];
        c.power = Some(1);
        c.toughness = Some(1);
        c
    };
    let goblin2 = {
        let mut c = CardData::new(ObjectId::new(), p1, "Goblin B");
        c.card_types = vec![CardType::Creature];
        c.subtypes = vec![SubType::Goblin];
        c.power = Some(2);
        c.toughness = Some(1);
        c
    };
    let non_goblin = {
        let mut c = CardData::new(ObjectId::new(), p1, "Elf Scout");
        c.card_types = vec![CardType::Creature];
        c.subtypes = vec![SubType::Elf];
        c.power = Some(1);
        c.toughness = Some(1);
        c
    };
    let g1_id = goblin1.id;
    let g2_id = goblin2.id;
    let ng_id = non_goblin.id;

    if let Some(player) = game.state.players.get_mut(&p1) {
        player.library.put_on_top(g1_id);
        game.state.card_store.insert(goblin1);
        player.library.put_on_top(ng_id);
        game.state.card_store.insert(non_goblin);
        player.library.put_on_top(g2_id);
        game.state.card_store.insert(goblin2);
    }

    let hand_before = game.state.players.get(&p1).unwrap().hand.len();
    let lib_before = game.state.players.get(&p1).unwrap().library.len();

    game.execute_effects(
        &[Effect::mill_and_return_all(5, "Goblin")],
        p1, &[], None, None,
    );

    let hand_after = game.state.players.get(&p1).unwrap().hand.len();
    let lib_after = game.state.players.get(&p1).unwrap().library.len();
    let gy_after = game.state.players.get(&p1).unwrap().graveyard.len();

    assert_eq!(hand_after, hand_before + 2, "Both goblins should be in hand");
    assert_eq!(lib_after, lib_before - 5, "5 cards milled from library");
    assert_eq!(gy_after, 3, "3 non-goblin cards in graveyard");

    let hand_ids = game.state.players.get(&p1).unwrap().hand.as_slice().to_vec();
    assert!(hand_ids.contains(&g1_id), "Goblin A should be in hand");
    assert!(hand_ids.contains(&g2_id), "Goblin B should be in hand");
    assert!(!hand_ids.contains(&ng_id), "Elf should NOT be in hand");
}

#[test]
fn mill_and_return_all_no_matches() {
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

    let hand_before = game.state.players.get(&p1).unwrap().hand.len();

    game.execute_effects(
        &[Effect::mill_and_return_all(3, "Goblin")],
        p1, &[], None, None,
    );

    let hand_after = game.state.players.get(&p1).unwrap().hand.len();
    let gy_after = game.state.players.get(&p1).unwrap().graveyard.len();

    assert_eq!(hand_after, hand_before, "No goblins milled, hand unchanged");
    assert_eq!(gy_after, 3, "All 3 milled cards stay in graveyard");
}

#[test]
fn boost_dual_target_dynamic_boosts_both_targets() {
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

    let mut elf1 = make_creature("Elf Warrior", p1, 2, 2);
    elf1.subtypes = vec![SubType::Elf];
    let _elf1_id = elf1.id;

    let mut elf2 = make_creature("Elf Scout", p1, 1, 1);
    elf2.subtypes = vec![SubType::Elf];
    let _elf2_id = elf2.id;

    let my_creature = make_creature("My Fighter", p1, 3, 3);
    let my_id = my_creature.id;

    let opp_creature = make_creature("Enemy Beast", p2, 5, 5);
    let opp_id = opp_creature.id;

    game.state.battlefield.add(Permanent::new(elf1, p1));
    game.state.battlefield.add(Permanent::new(elf2, p1));
    game.state.battlefield.add(Permanent::new(my_creature, p1));
    game.state.battlefield.add(Permanent::new(opp_creature, p2));

    let mut elf_in_gy = CardData::new(ObjectId::new(), p1, "Dead Elf");
    elf_in_gy.subtypes = vec![SubType::Elf];
    let gy_elf_id = elf_in_gy.id;
    game.state.card_store.insert(elf_in_gy);
    if let Some(player) = game.state.players.get_mut(&p1) {
        player.graveyard.add(gy_elf_id);
    }

    game.execute_effects(
        &[Effect::boost_dual_target_dynamic("Elves you control + Elf cards in your graveyard")],
        p1, &[my_id, opp_id], None, None,
    );

    let my_perm = game.state.battlefield.get(my_id).unwrap();
    assert_eq!(my_perm.continuous_boost_power, 3, "+3/+0: 2 Elves on battlefield + 1 Elf in graveyard");
    assert_eq!(my_perm.continuous_boost_toughness, 0, "No toughness boost on first target");

    let opp_perm = game.state.battlefield.get(opp_id).unwrap();
    assert_eq!(opp_perm.continuous_boost_power, 0, "No power reduction on second target");
    assert_eq!(opp_perm.continuous_boost_toughness, -3, "-0/-3: same X applied negatively");
}

#[test]
fn boost_dual_target_dynamic_single_target_only() {
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

    let mut elf1 = make_creature("Elf Warrior", p1, 2, 2);
    elf1.subtypes = vec![SubType::Elf];
    game.state.battlefield.add(Permanent::new(elf1, p1));

    let my_creature = make_creature("My Fighter", p1, 3, 3);
    let my_id = my_creature.id;
    game.state.battlefield.add(Permanent::new(my_creature, p1));

    game.execute_effects(
        &[Effect::boost_dual_target_dynamic("Elves you control")],
        p1, &[my_id], None, None,
    );

    let my_perm = game.state.battlefield.get(my_id).unwrap();
    assert_eq!(my_perm.continuous_boost_power, 1, "+1/+0: 1 Elf on battlefield");
    assert_eq!(my_perm.continuous_boost_toughness, 0, "No toughness change on first target");
}

#[test]
fn compare_and_boost_draws_and_boosts() {
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

    let creature_a = make_creature("Big Guy", p1, 5, 5);
    let id_a = creature_a.id;
    let creature_b = make_creature("Little Guy", p1, 2, 2);
    let id_b = creature_b.id;
    game.state.battlefield.add(Permanent::new(creature_a, p1));
    game.state.battlefield.add(Permanent::new(creature_b, p1));

    let hand_before = game.state.players.get(&p1).unwrap().hand.len();

    game.execute_effects(
        &[Effect::compare_and_boost()],
        p1, &[id_a, id_b], None, None,
    );

    let hand_after = game.state.players.get(&p1).unwrap().hand.len();
    assert_eq!(hand_after - hand_before, 3, "drew 3 cards (|5-2|=3)");

    let perm_a = game.state.battlefield.get(id_a).unwrap();
    assert_eq!(perm_a.power(), 8, "5 + 3 P1P1 counters = 8");
    assert_eq!(perm_a.toughness(), 8, "5 + 3 P1P1 counters = 8");
    assert!(perm_a.has_keyword(KeywordAbilities::TRAMPLE));

    let perm_b = game.state.battlefield.get(id_b).unwrap();
    assert_eq!(perm_b.power(), 5, "2 + 3 P1P1 counters = 5");
    assert_eq!(perm_b.toughness(), 5, "2 + 3 P1P1 counters = 5");
    assert!(perm_b.has_keyword(KeywordAbilities::TRAMPLE));
}

#[test]
fn compare_and_boost_equal_power_only_trample() {
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

    let creature_a = make_creature("Bear A", p1, 3, 3);
    let id_a = creature_a.id;
    let creature_b = make_creature("Bear B", p1, 3, 4);
    let id_b = creature_b.id;
    game.state.battlefield.add(Permanent::new(creature_a, p1));
    game.state.battlefield.add(Permanent::new(creature_b, p1));

    let hand_before = game.state.players.get(&p1).unwrap().hand.len();

    game.execute_effects(
        &[Effect::compare_and_boost()],
        p1, &[id_a, id_b], None, None,
    );

    let hand_after = game.state.players.get(&p1).unwrap().hand.len();
    assert_eq!(hand_after, hand_before, "no cards drawn when power equal");

    let perm_a = game.state.battlefield.get(id_a).unwrap();
    assert_eq!(perm_a.power(), 3, "no boost when X=0");
    assert!(perm_a.has_keyword(KeywordAbilities::TRAMPLE), "still gets trample");

    let perm_b = game.state.battlefield.get(id_b).unwrap();
    assert_eq!(perm_b.power(), 3, "no boost when X=0");
    assert!(perm_b.has_keyword(KeywordAbilities::TRAMPLE), "still gets trample");
}

#[test]
fn opponent_reveals_from_hand_exile_cast_instant_sorcery() {
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

    let source_id = ObjectId::new();
    let mut source_card = make_creature("Taster of Wares", p1, 3, 2);
    source_card.subtypes = vec![SubType::Goblin];
    game.state.card_store.insert(source_card.clone());
    game.state.battlefield.add(Permanent::new(source_card, p1));

    let goblin2_id = ObjectId::new();
    let mut goblin2 = make_creature("Goblin Ally", p1, 2, 1);
    goblin2.id = goblin2_id;
    goblin2.subtypes = vec![SubType::Goblin];
    game.state.card_store.insert(goblin2.clone());
    game.state.battlefield.add(Permanent::new(goblin2, p1));

    let instant_id = ObjectId::new();
    let mut instant_card = CardData::new(instant_id, p2, "Lightning Bolt");
    instant_card.card_types = vec![CardType::Instant];
    game.state.card_store.insert(instant_card.clone());
    game.state.players.get_mut(&p2).unwrap().hand.add(instant_id);

    let creature_id = ObjectId::new();
    let mut creature_card = make_creature("Bear", p2, 2, 2);
    creature_card.id = creature_id;
    game.state.card_store.insert(creature_card.clone());
    game.state.players.get_mut(&p2).unwrap().hand.add(creature_id);

    let p2_hand_before = game.state.players.get(&p2).unwrap().hand.len();

    game.execute_effects(
        &[Effect::opponent_reveals_from_hand_exile_cast("Goblins you control", true)],
        p1, &[], Some(source_id), None,
    );

    let p2_hand_after = game.state.players.get(&p2).unwrap().hand.len();
    assert_eq!(p2_hand_after, p2_hand_before - 1, "one card exiled from opponent hand");

    assert_eq!(game.state.exile.len(), 1, "one card in exile");

    assert!(game.state.impulse_playable.len() <= 1, "at most 1 impulse playable");
}

#[test]
fn opponent_reveals_creature_not_impulse_playable() {
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

    let source_id = ObjectId::new();
    let mut source_card = make_creature("Taster of Wares", p1, 3, 2);
    source_card.subtypes = vec![SubType::Goblin];
    game.state.card_store.insert(source_card.clone());
    game.state.battlefield.add(Permanent::new(source_card, p1));

    let creature_id = ObjectId::new();
    let mut creature_card = make_creature("Bear", p2, 2, 2);
    creature_card.id = creature_id;
    game.state.card_store.insert(creature_card.clone());
    game.state.players.get_mut(&p2).unwrap().hand.add(creature_id);

    game.execute_effects(
        &[Effect::opponent_reveals_from_hand_exile_cast("Goblins you control", true)],
        p1, &[], Some(source_id), None,
    );

    assert_eq!(game.state.exile.len(), 1, "creature card exiled");
    assert_eq!(game.state.impulse_playable.len(), 0,
        "creature card should NOT be impulse-playable when instant_sorcery_only is true");
}

#[test]
fn while_source_controlled_impulse_expires_when_source_leaves() {
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

    let source_id = ObjectId::new();
    let mut source_card = make_creature("Taster of Wares", p1, 3, 2);
    source_card.id = source_id;
    source_card.subtypes = vec![SubType::Goblin];
    game.state.card_store.insert(source_card.clone());
    game.state.battlefield.add(Permanent::new(source_card, p1));

    let instant_id = ObjectId::new();
    let mut instant_card = CardData::new(instant_id, p2, "Lightning Bolt");
    instant_card.card_types = vec![CardType::Instant];
    game.state.card_store.insert(instant_card.clone());
    game.state.players.get_mut(&p2).unwrap().hand.add(instant_id);

    game.execute_effects(
        &[Effect::opponent_reveals_from_hand_exile_cast("Goblins you control", true)],
        p1, &[], Some(source_id), None,
    );

    assert_eq!(game.state.impulse_playable.len(), 1, "instant should be impulse-playable");

    game.state.battlefield.remove(source_id);

    let actions = game.compute_legal_actions(p1);
    let has_cast_bolt = actions.iter().any(|a| match a {
        PlayerAction::CastSpell { card_id, .. } => card_id == &instant_id,
        _ => false,
    });
    assert!(!has_cast_bolt, "should not be able to cast exiled card after source leaves battlefield");
}

#[test]
fn set_subtypes_self_replaces_subtypes() {
    let p1 = PlayerId::new();
    let p2 = PlayerId::new();
    let config = GameConfig {
        players: vec![
            PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
            PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
        ],
        starting_life: 20,
    };
    let mut game = Game::new_two_player(config, vec![
        (p1, Box::new(AlwaysPassPlayer)),
        (p2, Box::new(AlwaysPassPlayer)),
    ]);

    let mut creature = make_creature("Test Kithkin", p1, 1, 1);
    creature.subtypes = vec![SubType::Kithkin];
    let cid = creature.id;
    game.state.battlefield.add(Permanent::new(creature, p1));

    game.execute_effects(
        &[Effect::set_subtypes_self(vec!["Kithkin", "Scout"])],
        p1, &[], Some(cid), None,
    );

    let perm = game.state.battlefield.get(cid).unwrap();
    assert_eq!(perm.card.subtypes.len(), 2);
    assert!(perm.has_subtype(&SubType::Scout));
    assert!(perm.has_subtype(&SubType::Kithkin));
}

#[test]
fn set_power_toughness_falls_back_to_source() {
    let p1 = PlayerId::new();
    let p2 = PlayerId::new();
    let config = GameConfig {
        players: vec![
            PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
            PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
        ],
        starting_life: 20,
    };
    let mut game = Game::new_two_player(config, vec![
        (p1, Box::new(AlwaysPassPlayer)),
        (p2, Box::new(AlwaysPassPlayer)),
    ]);

    let creature = make_creature("Test Creature", p1, 1, 1);
    let cid = creature.id;
    game.state.battlefield.add(Permanent::new(creature, p1));

    game.execute_effects(
        &[Effect::set_pt(4, 5)],
        p1, &[], Some(cid), None,
    );

    let perm = game.state.battlefield.get(cid).unwrap();
    assert_eq!(perm.power(), 4);
    assert_eq!(perm.toughness(), 5);
}

#[test]
fn conditional_source_is_a_type_level_up() {
    let p1 = PlayerId::new();
    let p2 = PlayerId::new();
    let config = GameConfig {
        players: vec![
            PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
            PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
        ],
        starting_life: 20,
    };
    let mut game = Game::new_two_player(config, vec![
        (p1, Box::new(AlwaysPassPlayer)),
        (p2, Box::new(AlwaysPassPlayer)),
    ]);

    let mut creature = make_creature("Figure", p1, 1, 1);
    creature.subtypes = vec![SubType::Kithkin];
    let cid = creature.id;
    game.state.battlefield.add(Permanent::new(creature, p1));

    game.execute_effects(
        &[Effect::conditional("source is a Scout",
            vec![Effect::set_subtypes_self(vec!["Kithkin", "Soldier"]),
                 Effect::set_pt(4, 5)],
            vec![])],
        p1, &[], Some(cid), None,
    );
    let perm = game.state.battlefield.get(cid).unwrap();
    assert_eq!(perm.power(), 1, "should not transform: not a Scout yet");

    game.execute_effects(
        &[Effect::set_subtypes_self(vec!["Kithkin", "Scout"]),
          Effect::set_pt(2, 3)],
        p1, &[], Some(cid), None,
    );
    let perm = game.state.battlefield.get(cid).unwrap();
    assert_eq!(perm.power(), 2);
    assert!(perm.has_subtype(&SubType::Scout));

    game.execute_effects(
        &[Effect::conditional("source is a Scout",
            vec![Effect::set_subtypes_self(vec!["Kithkin", "Soldier"]),
                 Effect::set_pt(4, 5)],
            vec![])],
        p1, &[], Some(cid), None,
    );
    let perm = game.state.battlefield.get(cid).unwrap();
    assert_eq!(perm.power(), 4);
    assert_eq!(perm.toughness(), 5);
    assert!(perm.has_subtype(&SubType::Soldier));
    assert!(!perm.has_subtype(&SubType::Scout), "Scout subtype should be replaced");

    game.execute_effects(
        &[Effect::conditional("source is a Soldier",
            vec![Effect::set_subtypes_self(vec!["Kithkin", "Avatar"]),
                 Effect::set_pt(7, 8),
                 Effect::GainKeyword { keyword: "protection".into() }],
            vec![])],
        p1, &[], Some(cid), None,
    );
    let perm = game.state.battlefield.get(cid).unwrap();
    assert_eq!(perm.power(), 7);
    assert_eq!(perm.toughness(), 8);
    assert!(perm.has_subtype(&SubType::Avatar));
    assert!(perm.keywords().contains(KeywordAbilities::PROTECTION));
}

#[test]
fn winnowing_sacrifices_non_sharing_creatures() {
    let p1 = PlayerId::new();
    let p2 = PlayerId::new();
    let config = GameConfig {
        players: vec![
            PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
            PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
        ],
        starting_life: 20,
    };
    let mut game = Game::new_two_player(config, vec![
        (p1, Box::new(AlwaysPassPlayer)),
        (p2, Box::new(AlwaysPassPlayer)),
    ]);

    let mut elf1 = make_creature("Llanowar Elves", p1, 1, 1);
    elf1.subtypes = vec![SubType::Elf, SubType::Druid];
    let elf1_id = elf1.id;
    game.state.card_store.insert(elf1.clone());
    game.state.battlefield.add(Permanent::new(elf1, p1));

    let mut elf2 = make_creature("Elvish Mystic", p1, 1, 1);
    elf2.subtypes = vec![SubType::Elf];
    let elf2_id = elf2.id;
    game.state.card_store.insert(elf2.clone());
    game.state.battlefield.add(Permanent::new(elf2, p1));

    let mut goblin = make_creature("Goblin Piker", p1, 2, 1);
    goblin.subtypes = vec![SubType::Goblin, SubType::Warrior];
    let goblin_id = goblin.id;
    game.state.card_store.insert(goblin.clone());
    game.state.battlefield.add(Permanent::new(goblin, p1));

    let mut human = make_creature("Grizzly Bears", p2, 2, 2);
    human.subtypes = vec![SubType::Human];
    let human_id = human.id;
    game.state.card_store.insert(human.clone());
    game.state.battlefield.add(Permanent::new(human, p2));

    let mut merfolk = make_creature("Merfolk Looter", p2, 1, 2);
    merfolk.subtypes = vec![SubType::Merfolk];
    let merfolk_id = merfolk.id;
    game.state.card_store.insert(merfolk.clone());
    game.state.battlefield.add(Permanent::new(merfolk, p2));

    game.execute_effects(&[Effect::winnowing()], p1, &[], None, None);

    assert!(game.state.battlefield.get(elf1_id).is_some(), "Chosen elf should survive");
    assert!(game.state.battlefield.get(elf2_id).is_some(), "Elf sharing type should survive");
    assert!(game.state.battlefield.get(goblin_id).is_none(), "Goblin should be sacrificed");
    assert!(game.state.battlefield.get(human_id).is_some(), "Chosen human should survive");
    assert!(game.state.battlefield.get(merfolk_id).is_none(), "Merfolk should be sacrificed");
}

#[test]
fn winnowing_changeling_survives() {
    let p1 = PlayerId::new();
    let p2 = PlayerId::new();
    let config = GameConfig {
        players: vec![
            PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
            PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
        ],
        starting_life: 20,
    };
    let mut game = Game::new_two_player(config, vec![
        (p1, Box::new(AlwaysPassPlayer)),
        (p2, Box::new(AlwaysPassPlayer)),
    ]);

    let mut elf = make_creature("Llanowar Elves", p1, 1, 1);
    elf.subtypes = vec![SubType::Elf];
    let elf_id = elf.id;
    game.state.card_store.insert(elf.clone());
    game.state.battlefield.add(Permanent::new(elf, p1));

    let mut changeling = make_creature("Changeling Outcast", p1, 1, 1);
    changeling.subtypes = vec![SubType::Shapeshifter];
    changeling.keywords = KeywordAbilities::CHANGELING;
    let changeling_id = changeling.id;
    game.state.card_store.insert(changeling.clone());
    game.state.battlefield.add(Permanent::new(changeling, p1));

    let mut goblin = make_creature("Goblin Piker", p1, 2, 1);
    goblin.subtypes = vec![SubType::Goblin];
    let goblin_id = goblin.id;
    game.state.card_store.insert(goblin.clone());
    game.state.battlefield.add(Permanent::new(goblin, p1));

    game.execute_effects(&[Effect::winnowing()], p1, &[], None, None);

    assert!(game.state.battlefield.get(elf_id).is_some(), "Chosen elf should survive");
    assert!(game.state.battlefield.get(changeling_id).is_some(), "Changeling shares all types, should survive");
    assert!(game.state.battlefield.get(goblin_id).is_none(), "Goblin does not share Elf type, should be sacrificed");
}

#[test]
fn winnowing_no_creatures_is_noop() {
    let p1 = PlayerId::new();
    let p2 = PlayerId::new();
    let config = GameConfig {
        players: vec![
            PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
            PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
        ],
        starting_life: 20,
    };
    let mut game = Game::new_two_player(config, vec![
        (p1, Box::new(AlwaysPassPlayer)),
        (p2, Box::new(AlwaysPassPlayer)),
    ]);

    let initial_bf = game.state.battlefield.iter().count();
    game.execute_effects(&[Effect::winnowing()], p1, &[], None, None);
    assert_eq!(game.state.battlefield.iter().count(), initial_bf);
}

#[test]
fn counter_all_opponent_spells_and_abilities_counters_and_creates_tokens() {
    use crate::zones::{StackItem, StackItemKind};
    use crate::mana::ManaCost;

    let p1 = PlayerId::new();
    let p2 = PlayerId::new();
    let config = GameConfig {
        players: vec![
            PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
            PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
        ],
        starting_life: 20,
    };
    let mut game = Game::new_two_player(config, vec![
        (p1, Box::new(AlwaysPassPlayer)),
        (p2, Box::new(AlwaysPassPlayer)),
    ]);

    let mut spell1 = CardData::new(ObjectId::new(), p2, "Lightning Bolt");
    spell1.card_types = vec![CardType::Instant];
    spell1.mana_cost = ManaCost::parse("{R}");
    let spell1_id = spell1.id;
    game.state.stack.push(StackItem {
        id: spell1_id,
        kind: StackItemKind::Spell { card: Box::new(spell1) },
        controller: p2,
        targets: vec![],
        countered: false,
        x_value: None,
        exile_on_resolve: false,
    });

    let mut spell2 = CardData::new(ObjectId::new(), p2, "Giant Growth");
    spell2.card_types = vec![CardType::Instant];
    spell2.mana_cost = ManaCost::parse("{G}");
    let spell2_id = spell2.id;
    game.state.stack.push(StackItem {
        id: spell2_id,
        kind: StackItemKind::Spell { card: Box::new(spell2) },
        controller: p2,
        targets: vec![],
        countered: false,
        x_value: None,
        exile_on_resolve: false,
    });

    let ability_id = ObjectId::new();
    game.state.stack.push(StackItem {
        id: ability_id,
        kind: StackItemKind::Ability {
            source_id: ObjectId::new(),
            ability_id: crate::types::AbilityId::new(),
            description: "Some triggered ability".into(),
        },
        controller: p2,
        targets: vec![],
        countered: false,
        x_value: None,
        exile_on_resolve: false,
    });

    let initial_tokens = game.state.battlefield.iter()
        .filter(|p| p.card.is_token).count();
    assert_eq!(initial_tokens, 0);
    assert_eq!(game.state.stack.len(), 3);

    game.execute_effects(
        &[Effect::counter_all_opponent_spells_and_abilities("1/1 Faerie with flying")],
        p1, &[], None, None,
    );

    assert_eq!(game.state.stack.len(), 0);
    let tokens: Vec<_> = game.state.battlefield.iter()
        .filter(|p| p.card.is_token).collect();
    assert_eq!(tokens.len(), 3);
    for t in &tokens {
        assert_eq!(t.card.power, Some(1));
        assert_eq!(t.card.toughness, Some(1));
        assert!(t.card.keywords.contains(KeywordAbilities::FLYING));
        assert_eq!(t.controller, p1);
    }
}

#[test]
fn counter_all_opponent_respects_cant_be_countered() {
    use crate::zones::{StackItem, StackItemKind};
    use crate::mana::ManaCost;
    use crate::abilities::{Ability, StaticEffect};

    let p1 = PlayerId::new();
    let p2 = PlayerId::new();
    let config = GameConfig {
        players: vec![
            PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
            PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
        ],
        starting_life: 20,
    };
    let mut game = Game::new_two_player(config, vec![
        (p1, Box::new(AlwaysPassPlayer)),
        (p2, Box::new(AlwaysPassPlayer)),
    ]);

    let mut uncounterable = CardData::new(ObjectId::new(), p2, "Uncounterable Spell");
    uncounterable.card_types = vec![CardType::Instant];
    uncounterable.mana_cost = ManaCost::parse("{U}");
    uncounterable.abilities = vec![
        Ability::static_ability(uncounterable.id,
            "This spell can't be countered.",
            vec![StaticEffect::CantBeCountered]),
    ];
    let unc_id = uncounterable.id;
    game.state.stack.push(StackItem {
        id: unc_id,
        kind: StackItemKind::Spell { card: Box::new(uncounterable) },
        controller: p2,
        targets: vec![],
        countered: false,
        x_value: None,
        exile_on_resolve: false,
    });

    let mut counterable = CardData::new(ObjectId::new(), p2, "Counterable Spell");
    counterable.card_types = vec![CardType::Instant];
    counterable.mana_cost = ManaCost::parse("{R}");
    let cnt_id = counterable.id;
    game.state.stack.push(StackItem {
        id: cnt_id,
        kind: StackItemKind::Spell { card: Box::new(counterable) },
        controller: p2,
        targets: vec![],
        countered: false,
        x_value: None,
        exile_on_resolve: false,
    });

    game.execute_effects(
        &[Effect::counter_all_opponent_spells_and_abilities("1/1 Faerie with flying")],
        p1, &[], None, None,
    );

    assert!(game.state.stack.get(unc_id).is_some(), "Uncounterable spell should remain on stack");
    assert!(game.state.stack.get(cnt_id).is_none(), "Counterable spell should be countered");
    let tokens: Vec<_> = game.state.battlefield.iter()
        .filter(|p| p.card.is_token).collect();
    assert_eq!(tokens.len(), 1, "Only 1 token for the 1 spell actually countered");
}

#[test]
fn counter_all_opponent_ignores_own_spells() {
    use crate::zones::{StackItem, StackItemKind};
    use crate::mana::ManaCost;

    let p1 = PlayerId::new();
    let p2 = PlayerId::new();
    let config = GameConfig {
        players: vec![
            PlayerConfig { name: "Alice".into(), deck: make_deck(p1) },
            PlayerConfig { name: "Bob".into(), deck: make_deck(p2) },
        ],
        starting_life: 20,
    };
    let mut game = Game::new_two_player(config, vec![
        (p1, Box::new(AlwaysPassPlayer)),
        (p2, Box::new(AlwaysPassPlayer)),
    ]);

    let mut own_spell = CardData::new(ObjectId::new(), p1, "Own Spell");
    own_spell.card_types = vec![CardType::Instant];
    own_spell.mana_cost = ManaCost::parse("{U}");
    let own_id = own_spell.id;
    game.state.stack.push(StackItem {
        id: own_id,
        kind: StackItemKind::Spell { card: Box::new(own_spell) },
        controller: p1,
        targets: vec![],
        countered: false,
        x_value: None,
        exile_on_resolve: false,
    });

    let mut opp_spell = CardData::new(ObjectId::new(), p2, "Opponent Spell");
    opp_spell.card_types = vec![CardType::Instant];
    opp_spell.mana_cost = ManaCost::parse("{R}");
    let opp_id = opp_spell.id;
    game.state.stack.push(StackItem {
        id: opp_id,
        kind: StackItemKind::Spell { card: Box::new(opp_spell) },
        controller: p2,
        targets: vec![],
        countered: false,
        x_value: None,
        exile_on_resolve: false,
    });

    game.execute_effects(
        &[Effect::counter_all_opponent_spells_and_abilities("1/1 Faerie with flying")],
        p1, &[], None, None,
    );

    assert!(game.state.stack.get(own_id).is_some(), "Own spell should remain on stack");
    assert!(game.state.stack.get(opp_id).is_none(), "Opponent spell should be countered");
    let tokens: Vec<_> = game.state.battlefield.iter()
        .filter(|p| p.card.is_token).collect();
    assert_eq!(tokens.len(), 1, "1 token for the 1 opponent spell countered");
}

#[cfg(test)]
#[test]
fn mass_become_copy_basic() {
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

    let mut dragon = make_creature("Shivan Dragon", p2, 5, 5);
    dragon.keywords = KeywordAbilities::FLYING;
    let dragon_id = dragon.id;
    game.state.battlefield.add(Permanent::new(dragon, p2));

    let bear1 = make_creature("Grizzly Bears", p1, 2, 2);
    let bear1_id = bear1.id;
    game.state.battlefield.add(Permanent::new(bear1, p1));

    let bear2 = make_creature("Runeclaw Bear", p1, 2, 2);
    let bear2_id = bear2.id;
    game.state.battlefield.add(Permanent::new(bear2, p1));

    game.execute_effects(
        &[Effect::mass_become_copy()],
        p1, &[dragon_id], None, None,
    );

    let b1 = game.state.battlefield.get(bear1_id).unwrap();
    assert_eq!(b1.name(), "Shivan Dragon");
    assert_eq!(b1.card.power, Some(5));
    assert_eq!(b1.card.toughness, Some(5));
    assert!(b1.has_keyword(KeywordAbilities::FLYING));

    let b2 = game.state.battlefield.get(bear2_id).unwrap();
    assert_eq!(b2.name(), "Shivan Dragon");
    assert_eq!(b2.card.power, Some(5));
}

#[cfg(test)]
#[test]
fn mass_become_copy_skips_lands() {
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

    let dragon = make_creature("Shivan Dragon", p1, 5, 5);
    let dragon_id = dragon.id;
    game.state.battlefield.add(Permanent::new(dragon, p1));

    let mut land = CardData::new(ObjectId::new(), p1, "Forest");
    land.card_types = vec![CardType::Land];
    let land_id = land.id;
    game.state.battlefield.add(Permanent::new(land, p1));

    let bear = make_creature("Grizzly Bears", p1, 2, 2);
    let bear_id = bear.id;
    game.state.battlefield.add(Permanent::new(bear, p1));

    game.execute_effects(
        &[Effect::mass_become_copy()],
        p1, &[dragon_id], None, None,
    );

    let l = game.state.battlefield.get(land_id).unwrap();
    assert_eq!(l.name(), "Forest", "Land should not become a copy");

    let b = game.state.battlefield.get(bear_id).unwrap();
    assert_eq!(b.name(), "Shivan Dragon", "Creature should become a copy");
}

#[cfg(test)]
#[test]
fn mass_become_copy_preserves_identity() {
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

    let dragon = make_creature("Shivan Dragon", p2, 5, 5);
    let dragon_id = dragon.id;
    game.state.battlefield.add(Permanent::new(dragon, p2));

    let mut token = make_creature("Soldier", p1, 1, 1);
    token.is_token = true;
    let token_id = token.id;
    game.state.battlefield.add(Permanent::new(token, p1));

    game.execute_effects(
        &[Effect::mass_become_copy()],
        p1, &[dragon_id], None, None,
    );

    let t = game.state.battlefield.get(token_id).unwrap();
    assert_eq!(t.name(), "Shivan Dragon");
    assert_eq!(t.card.id, token_id, "Object ID should be preserved");
    assert_eq!(t.card.owner, p1, "Owner should be preserved");
    assert!(t.card.is_token, "Token status should be preserved");
}