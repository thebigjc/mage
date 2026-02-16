// Tests extracted from game.rs

use crate::game::*;
use crate::abilities::{Ability, Cost, Effect, TargetSpec, StaticEffect};
use crate::filters::Filter;
use crate::card::CardData;
use crate::constants::{CardType, KeywordAbilities, Outcome, SubType};
use crate::counters::CounterType;
use crate::decision::{AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction, PlayerAgent, PlayerDecisionMaker, ReplacementEffectChoice, TargetRequirement, UnpaidMana};
use crate::mana::{Mana, ManaCost};
use crate::permanent::Permanent;
use crate::types::{ObjectId, PlayerId};
use crate::abilities::X_VALUE;


#[cfg(test)]
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
            starting_life: Life::new(20),
        };
        let game = Game::new_two_player(
            config,
            vec![
                (p1, PlayerAgent::new(LastCardPicker)),
                (p2, PlayerAgent::new(LastCardPicker)),
            ],
        );
        (game, p1, p2)
    }

    fn add_creature(game: &mut Game, owner: PlayerId, name: &str) -> ObjectId {
        let id = ObjectId::new();
        let mut card = CardData::new(id, owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(Power::new(2));
        card.toughness = Some(Toughness::new(2));
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


    // Additional tests

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
            starting_life: Life::new(20),
            players: vec![
                PlayerConfig { name: "P1".into(), deck: vec![] },
                PlayerConfig { name: "P2".into(), deck: vec![] },
            ],
        };

        let dms: Vec<(PlayerId, PlayerAgent)> = vec![
            (p1, PlayerAgent::new(XChooserPlayer { x_choice })),
            (p2, PlayerAgent::new(XChooserPlayer { x_choice: 0 })),
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
        creature.power = Some(Power::new(5));
        creature.toughness = Some(Toughness::new(5));
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


    // Additional tests

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

    fn setup_game2() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            starting_life: Life::new(20),
            players: vec![
                PlayerConfig { name: "P1".into(), deck: vec![] },
                PlayerConfig { name: "P2".into(), deck: vec![] },
            ],
        };
        let game = Game::new_two_player(config, vec![
            (p1, PlayerAgent::new(AlwaysPassDM)),
            (p2, PlayerAgent::new(AlwaysPassDM)),
        ]);
        (game, p1, p2)
    }

    #[test]
    fn cost_reduction_reduces_generic_mana() {
        let (mut game, p1, _p2) = setup_game2();

        // Add a lord with CostReduction for Elf spells
        let lord_id = ObjectId::new();
        let mut lord = CardData::new(lord_id, p1, "Elf Cost Reducer");
        lord.card_types = vec![CardType::Creature];
        lord.subtypes = vec![SubType::Elf];
        lord.power = Some(Power::new(1));
        lord.toughness = Some(Toughness::new(1));
        lord.abilities = vec![Ability::static_ability(lord_id,
            "Elf spells you cast cost {1} less.",
            vec![StaticEffect::CostReduction { filter: Filter::parse("Elf"), amount: 1, condition: None }])];
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
        lord.power = Some(Power::new(1));
        lord.toughness = Some(Toughness::new(1));
        lord.abilities = vec![Ability::static_ability(lord_id,
            "Elf spells cost {1} less.",
            vec![StaticEffect::CostReduction { filter: Filter::parse("Elf"), amount: 1, condition: None }])];
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

    #[test]
    fn conditional_cost_reduction_toughness_greater_than_power() {
        let (mut game, p1, _p2) = setup_game2();

        let doran_id = ObjectId::new();
        let mut doran = CardData::new(doran_id, p1, "Doran");
        doran.card_types = vec![CardType::Creature];
        doran.power = Some(Power::new(0));
        doran.toughness = Some(Toughness::new(5));
        doran.abilities = vec![Ability::static_ability(doran_id,
            "Creature spells with toughness > power cost {1} less.",
            vec![StaticEffect::cost_reduction_if_toughness_greater("creature spells", 1)])];
        let perm = crate::permanent::Permanent::new(doran.clone(), p1);
        game.state.card_store.insert(doran.clone());
        game.state.battlefield.add(perm);
        for ab in &doran.abilities {
            game.state.ability_store.add(ab.clone());
        }

        let high_tough_id = ObjectId::new();
        let mut high_tough = CardData::new(high_tough_id, p1, "Wall");
        high_tough.card_types = vec![CardType::Creature];
        high_tough.power = Some(Power::new(0));
        high_tough.toughness = Some(Toughness::new(4));
        high_tough.mana_cost = ManaCost::parse("{2}{W}");

        let reduction = game.calculate_cost_reduction(p1, &high_tough);
        assert_eq!(reduction, 1, "Creature with toughness > power should get reduction");
    }

    #[test]
    fn conditional_cost_reduction_no_reduction_when_power_greater() {
        let (mut game, p1, _p2) = setup_game2();

        let doran_id = ObjectId::new();
        let mut doran = CardData::new(doran_id, p1, "Doran");
        doran.card_types = vec![CardType::Creature];
        doran.power = Some(Power::new(0));
        doran.toughness = Some(Toughness::new(5));
        doran.abilities = vec![Ability::static_ability(doran_id,
            "Creature spells with toughness > power cost {1} less.",
            vec![StaticEffect::cost_reduction_if_toughness_greater("creature spells", 1)])];
        let perm = crate::permanent::Permanent::new(doran.clone(), p1);
        game.state.card_store.insert(doran.clone());
        game.state.battlefield.add(perm);
        for ab in &doran.abilities {
            game.state.ability_store.add(ab.clone());
        }

        let aggro_id = ObjectId::new();
        let mut aggro = CardData::new(aggro_id, p1, "Aggro Creature");
        aggro.card_types = vec![CardType::Creature];
        aggro.power = Some(Power::new(4));
        aggro.toughness = Some(Toughness::new(2));
        aggro.mana_cost = ManaCost::parse("{2}{R}");

        let reduction = game.calculate_cost_reduction(p1, &aggro);
        assert_eq!(reduction, 0, "Creature with power > toughness should NOT get reduction");
    }

    #[test]
    fn conditional_cost_reduction_no_reduction_when_equal() {
        let (mut game, p1, _p2) = setup_game2();

        let doran_id = ObjectId::new();
        let mut doran = CardData::new(doran_id, p1, "Doran");
        doran.card_types = vec![CardType::Creature];
        doran.power = Some(Power::new(0));
        doran.toughness = Some(Toughness::new(5));
        doran.abilities = vec![Ability::static_ability(doran_id,
            "Creature spells with toughness > power cost {1} less.",
            vec![StaticEffect::cost_reduction_if_toughness_greater("creature spells", 1)])];
        let perm = crate::permanent::Permanent::new(doran.clone(), p1);
        game.state.card_store.insert(doran.clone());
        game.state.battlefield.add(perm);
        for ab in &doran.abilities {
            game.state.ability_store.add(ab.clone());
        }

        let equal_id = ObjectId::new();
        let mut equal = CardData::new(equal_id, p1, "Bear");
        equal.card_types = vec![CardType::Creature];
        equal.power = Some(Power::new(2));
        equal.toughness = Some(Toughness::new(2));
        equal.mana_cost = ManaCost::parse("{1}{G}");

        let reduction = game.calculate_cost_reduction(p1, &equal);
        assert_eq!(reduction, 0, "Creature with equal power/toughness should NOT get reduction");
    }

    #[test]
    fn conditional_cost_reduction_noncreature_not_affected() {
        let (mut game, p1, _p2) = setup_game2();

        let doran_id = ObjectId::new();
        let mut doran = CardData::new(doran_id, p1, "Doran");
        doran.card_types = vec![CardType::Creature];
        doran.power = Some(Power::new(0));
        doran.toughness = Some(Toughness::new(5));
        doran.abilities = vec![Ability::static_ability(doran_id,
            "Creature spells with toughness > power cost {1} less.",
            vec![StaticEffect::cost_reduction_if_toughness_greater("creature spells", 1)])];
        let perm = crate::permanent::Permanent::new(doran.clone(), p1);
        game.state.card_store.insert(doran.clone());
        game.state.battlefield.add(perm);
        for ab in &doran.abilities {
            game.state.ability_store.add(ab.clone());
        }

        let spell_id = ObjectId::new();
        let mut spell = CardData::new(spell_id, p1, "Sorcery");
        spell.card_types = vec![CardType::Sorcery];
        spell.mana_cost = ManaCost::parse("{2}{G}");

        let reduction = game.calculate_cost_reduction(p1, &spell);
        assert_eq!(reduction, 0, "Non-creature spell should NOT get reduction from creature-only filter");
    }

    #[test]
    fn conditional_cost_reduction_applied_in_legal_actions() {
        let (mut game, p1, _p2) = setup_game();

        game.state.players.get_mut(&p1).unwrap().mana_pool.add(Mana::white(2), None, false);

        let doran_id = ObjectId::new();
        let mut doran = CardData::new(doran_id, p1, "Doran");
        doran.card_types = vec![CardType::Creature];
        doran.power = Some(Power::new(0));
        doran.toughness = Some(Toughness::new(5));
        doran.abilities = vec![Ability::static_ability(doran_id,
            "Creature spells with toughness > power cost {1} less.",
            vec![StaticEffect::cost_reduction_if_toughness_greater("creature spells", 1)])];
        let perm = crate::permanent::Permanent::new(doran.clone(), p1);
        game.state.card_store.insert(doran.clone());
        game.state.battlefield.add(perm);
        for ab in &doran.abilities {
            game.state.ability_store.add(ab.clone());
        }

        let wall_id = ObjectId::new();
        let mut wall = CardData::new(wall_id, p1, "Wall of Stone");
        wall.card_types = vec![CardType::Creature];
        wall.power = Some(Power::new(0));
        wall.toughness = Some(Toughness::new(4));
        wall.mana_cost = ManaCost::parse("{2}{W}");
        game.state.card_store.insert(wall.clone());
        game.state.players.get_mut(&p1).unwrap().hand.add(wall_id);

        game.state.current_phase = crate::constants::TurnPhase::PrecombatMain;
        game.state.current_step = crate::constants::PhaseStep::PrecombatMain;
        game.state.active_player = p1;
        game.state.priority_player = p1;

        let actions = game.compute_legal_actions(p1);
        let can_cast = actions.iter().any(|a| matches!(a, PlayerAction::CastSpell { card_id, .. } if *card_id == wall_id));
        assert!(can_cast, "Should be able to cast 2W wall with 2W mana and 1 conditional reduction");
    }

    #[test]
    fn conditional_cost_reduction_no_reduction_in_legal_actions_for_power_creature() {
        let (mut game, p1, _p2) = setup_game();

        game.state.players.get_mut(&p1).unwrap().mana_pool.add(Mana::red(2), None, false);

        let doran_id = ObjectId::new();
        let mut doran = CardData::new(doran_id, p1, "Doran");
        doran.card_types = vec![CardType::Creature];
        doran.power = Some(Power::new(0));
        doran.toughness = Some(Toughness::new(5));
        doran.abilities = vec![Ability::static_ability(doran_id,
            "Creature spells with toughness > power cost {1} less.",
            vec![StaticEffect::cost_reduction_if_toughness_greater("creature spells", 1)])];
        let perm = crate::permanent::Permanent::new(doran.clone(), p1);
        game.state.card_store.insert(doran.clone());
        game.state.battlefield.add(perm);
        for ab in &doran.abilities {
            game.state.ability_store.add(ab.clone());
        }

        let aggro_id = ObjectId::new();
        let mut aggro = CardData::new(aggro_id, p1, "Aggro Creature");
        aggro.card_types = vec![CardType::Creature];
        aggro.power = Some(Power::new(3));
        aggro.toughness = Some(Toughness::new(1));
        aggro.mana_cost = ManaCost::parse("{2}{R}");
        game.state.card_store.insert(aggro.clone());
        game.state.players.get_mut(&p1).unwrap().hand.add(aggro_id);

        game.state.current_phase = crate::constants::TurnPhase::PrecombatMain;
        game.state.current_step = crate::constants::PhaseStep::PrecombatMain;
        game.state.active_player = p1;
        game.state.priority_player = p1;

        let actions = game.compute_legal_actions(p1);
        let can_cast = actions.iter().any(|a| matches!(a, PlayerAction::CastSpell { card_id, .. } if *card_id == aggro_id));
        assert!(!can_cast, "Should NOT be able to cast 2R creature (power > toughness) with only 2R mana");
    }

    #[test]
    fn conditional_cost_reduction_helper_constructor() {
        match StaticEffect::cost_reduction_if_toughness_greater("creature spells", 1) {
            StaticEffect::CostReduction { filter, amount, condition } => {
                assert_eq!(filter.message, "creature spells");
                assert_eq!(amount, 1);
                assert_eq!(condition.as_deref(), Some("toughness_greater_than_power"));
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn variable_blight_cost_puts_counters_and_sets_x() {
        let (mut game, p1, _p2) = setup_x_game(3);

        let creature_id = ObjectId::new();
        let mut creature = CardData::new(creature_id, p1, "Blight Target");
        creature.card_types = vec![CardType::Creature];
        creature.power = Some(Power::new(2));
        creature.toughness = Some(Toughness::new(5));
        game.state.battlefield.add(Permanent::new(creature.clone(), p1));
        game.state.card_store.insert(creature);

        assert!(game.pay_costs(p1, ObjectId::new(), &[Cost::VariableBlight]));

        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert_eq!(perm.counters.get(&CounterType::M1M1), 3);
        assert_eq!(game.variable_blight_amount, Some(3));
    }

    #[test]
    fn variable_blight_fails_without_creatures() {
        let (mut game, p1, _p2) = setup_game();

        let non_creatures: Vec<ObjectId> = game.state.battlefield.controlled_by(p1)
            .map(|p| p.id())
            .collect();
        for id in non_creatures {
            game.state.battlefield.remove(id);
        }

        assert!(!game.can_pay_additional_costs(p1, ObjectId::new(), &[Cost::VariableBlight]));
    }

    #[test]
    fn variable_blight_soul_immolation_pattern() {
        let (mut game, p1, p2) = setup_x_game(2);

        let my_creature_id = ObjectId::new();
        let mut my_creature = CardData::new(my_creature_id, p1, "My Creature");
        my_creature.card_types = vec![CardType::Creature];
        my_creature.power = Some(Power::new(3));
        my_creature.toughness = Some(Toughness::new(4));
        game.state.battlefield.add(Permanent::new(my_creature.clone(), p1));
        game.state.card_store.insert(my_creature);

        let opp_creature_id = ObjectId::new();
        let mut opp_creature = CardData::new(opp_creature_id, p2, "Opp Creature");
        opp_creature.card_types = vec![CardType::Creature];
        opp_creature.power = Some(Power::new(3));
        opp_creature.toughness = Some(Toughness::new(5));
        game.state.battlefield.add(Permanent::new(opp_creature.clone(), p2));
        game.state.card_store.insert(opp_creature);

        if let Some(player) = game.state.players.get_mut(&p1) {
            player.mana_pool.add(Mana { red: 2, green: 3, ..Mana::new() }, None, false);
        }

        let spell_id = ObjectId::new();
        let mut spell = CardData::new(spell_id, p1, "Soul Immolation");
        spell.card_types = vec![CardType::Sorcery];
        spell.mana_cost = ManaCost::parse("{3}{R}{R}");
        spell.additional_costs = vec![Cost::VariableBlight];
        spell.abilities = vec![Ability::spell(spell_id,
            vec![Effect::DealDamageOpponents { amount: X_VALUE },
                 Effect::DealDamageOpponentsCreatures { amount: X_VALUE }],
            TargetSpec::None)];

        if let Some(player) = game.state.players.get_mut(&p1) {
            player.hand.add(spell_id);
        }
        game.state.card_store.insert(spell);

        game.cast_spell(p1, spell_id);

        assert_eq!(game.state.stack.top().unwrap().x_value, Some(2));

        let my_perm = game.state.battlefield.get(my_creature_id).unwrap();
        assert_eq!(my_perm.counters.get(&CounterType::M1M1), 2);

        game.resolve_top_of_stack();

        assert_eq!(game.state.players.get(&p2).unwrap().life, 18);
        let opp_perm = game.state.battlefield.get(opp_creature_id).unwrap();
        assert_eq!(opp_perm.damage, 2);
    }
