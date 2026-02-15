// Effect execution tests

use super::*;
use crate::game::{Game, GameConfig, PlayerConfig};
use crate::permanent::Permanent;
use crate::counters::CounterType;
use crate::constants::{PhaseStep, Outcome};
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