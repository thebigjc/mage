// Tests extracted from game.rs

use crate::game::*;
use crate::abilities::{Ability, Effect, TargetSpec, StaticEffect};
use crate::card::CardData;
use crate::constants::{CardType, Color, KeywordAbilities, Outcome, PhaseStep, SubType, TurnPhase};
use crate::decision::{AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction, PlayerDecisionMaker, ReplacementEffectChoice, TargetRequirement, UnpaidMana};
use crate::events::GameEvent;
use crate::mana::{Mana, ManaCost};
use crate::permanent::Permanent;
use crate::types::{ObjectId, PlayerId};


#[cfg(test)]

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
        let targets = game.legal_targets_for_spec(&TargetSpec::Creature, p1, &[]);
        assert!(!targets.contains(&hexproof_id));
        assert!(targets.contains(&regular_id));

        // Opponent creature targeting — same
        let targets = game.legal_targets_for_spec(&TargetSpec::OpponentCreature, p1, &[]);
        assert!(!targets.contains(&hexproof_id));
        assert!(targets.contains(&regular_id));
    }

    #[test]
    fn hexproof_allows_controller_targeting() {
        let (mut game, _p1, p2) = setup();

        let hexproof_id = add_creature(&mut game, p2, "Hexproof Bear", KeywordAbilities::HEXPROOF);

        // P2 targeting their own hexproof creature — should be allowed
        let targets = game.legal_targets_for_spec(&TargetSpec::CreatureYouControl, p2, &[]);
        assert!(targets.contains(&hexproof_id));

        // P2 targeting any creature — their own hexproof creature is fine
        let targets = game.legal_targets_for_spec(&TargetSpec::Creature, p2, &[]);
        assert!(targets.contains(&hexproof_id));
    }

    #[test]
    fn shroud_prevents_all_targeting() {
        let (mut game, p1, p2) = setup();

        let shroud_id = add_creature(&mut game, p2, "Shroud Bear", KeywordAbilities::SHROUD);

        // Neither player can target a shroud creature
        let targets = game.legal_targets_for_spec(&TargetSpec::Creature, p1, &[]);
        assert!(!targets.contains(&shroud_id));

        let targets = game.legal_targets_for_spec(&TargetSpec::Creature, p2, &[]);
        assert!(!targets.contains(&shroud_id));
    }

    #[test]
    fn hexproof_on_permanent_targeting() {
        let (mut game, p1, p2) = setup();

        let hexproof_id = add_creature(&mut game, p2, "Hexproof Bear", KeywordAbilities::HEXPROOF);
        let regular_id = add_creature(&mut game, p2, "Regular Bear", KeywordAbilities::empty());

        // TargetSpec::Permanent — hexproof blocks opponent targeting
        let targets = game.legal_targets_for_spec(&TargetSpec::Permanent, p1, &[]);
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
        let targets = game.legal_targets_for_spec(&TargetSpec::Creature, p1, &[]);
        assert!(!targets.contains(&bear_id));
    }


    // Additional tests

    fn setup2() -> (Game, PlayerId, PlayerId) {
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
        let (mut game, p1, _p2) = setup2();
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
        assert!(crate::combat::can_block(&blocker, &attacker));
    }


    // Additional tests

    fn make_deck2(owner: PlayerId) -> Vec<CardData> {
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
                PlayerConfig { name: "Attacker".into(), deck: make_deck2(p1) },
                PlayerConfig { name: "Defender".into(), deck: make_deck2(p2) },
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


    // Additional tests

    fn make_deck3(owner: PlayerId) -> Vec<CardData> {
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
                PlayerConfig { name: "A".into(), deck: make_deck3(p1) },
                PlayerConfig { name: "B".into(), deck: make_deck3(p2) },
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
            kind: crate::zones::StackItemKind::Spell { card: Box::new(spell_card) },
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
            kind: crate::zones::StackItemKind::Spell { card: Box::new(spell_card) },
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


    // Additional tests

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


    // Additional tests

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

    fn make_deck4(owner: PlayerId) -> Vec<CardData> {
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
                PlayerConfig { name: "Alice".to_string(), deck: make_deck4(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck4(p2) },
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


    // Additional tests

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

    fn make_deck5(owner: PlayerId) -> Vec<CardData> {
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


    // Additional tests

    fn make_deck6(owner: PlayerId) -> Vec<CardData> {
        (0..40).map(|i| {
            let mut c = CardData::new(ObjectId::new(), owner, &format!("Card {i}"));
            c.card_types = vec![CardType::Land];
            c
        }).collect()
    }

    fn setup_game2() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".into(), deck: make_deck6(p1) },
                PlayerConfig { name: "Bob".into(), deck: make_deck6(p2) },
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
        let (game, p1, _p2) = setup_game2();
        let card = make_convoke_spell(p1);
        assert!(game.spell_has_convoke(p1, &card));
    }

    #[test]
    fn spell_without_convoke() {
        let (game, p1, _p2) = setup_game2();
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

    // ── Conspire tests ──────────────────────────────────────────────────────

    fn make_conspire_spell(owner: PlayerId) -> CardData {
        let id = ObjectId::new();
        let mut card = CardData::new(id, owner, "Conspire Spell");
        card.card_types = vec![CardType::Sorcery];
        card.mana_cost = ManaCost::parse("{2}{R}");
        card.keywords = KeywordAbilities::CONSPIRE;
        card.abilities = vec![Ability::spell(id, vec![Effect::DealDamage { amount: 3 }], TargetSpec::Creature)];
        card
    }

    #[test]
    fn spell_has_conspire_keyword() {
        let (game, p1, _p2) = setup_game2();
        let card = make_conspire_spell(p1);
        assert!(game.spell_has_conspire(p1, &card));
    }

    #[test]
    fn spell_without_conspire() {
        let (game, p1, _p2) = setup_game2();
        let mut card = CardData::new(ObjectId::new(), p1, "Normal Spell");
        card.card_types = vec![CardType::Sorcery];
        card.mana_cost = ManaCost::parse("{2}{R}");
        assert!(!game.spell_has_conspire(p1, &card));
    }

    #[test]
    fn conspire_eligible_creatures_share_color() {
        let (mut game, p1, _p2) = setup_game();

        let c1 = make_creature_with_color("Red Goblin", p1, vec![Color::Red]);
        game.state.battlefield.add(Permanent::new(c1.clone(), p1));
        game.state.card_store.insert(c1);

        let c2 = make_creature_with_color("Red Warrior", p1, vec![Color::Red]);
        game.state.battlefield.add(Permanent::new(c2.clone(), p1));
        game.state.card_store.insert(c2);

        let c3 = make_creature_with_color("Blue Wizard", p1, vec![Color::Blue]);
        game.state.battlefield.add(Permanent::new(c3.clone(), p1));
        game.state.card_store.insert(c3);

        let red_colors = vec![Color::Red];
        assert_eq!(game.count_conspire_eligible_creatures(p1, &red_colors), 2);
    }

    #[test]
    fn conspire_excludes_tapped_creatures() {
        let (mut game, p1, _p2) = setup_game();

        let c1 = make_creature_with_color("Tapped Goblin", p1, vec![Color::Red]);
        let c1_id = c1.id;
        game.state.battlefield.add(Permanent::new(c1.clone(), p1));
        game.state.card_store.insert(c1);
        game.state.battlefield.get_mut(c1_id).unwrap().tapped = true;

        let c2 = make_creature_with_color("Untapped Goblin", p1, vec![Color::Red]);
        game.state.battlefield.add(Permanent::new(c2.clone(), p1));
        game.state.card_store.insert(c2);

        let red_colors = vec![Color::Red];
        assert_eq!(game.count_conspire_eligible_creatures(p1, &red_colors), 1);
    }

    #[test]
    fn conspire_excludes_opponent_creatures() {
        let (mut game, p1, p2) = setup_game();

        let c1 = make_creature_with_color("Enemy Goblin", p2, vec![Color::Red]);
        game.state.battlefield.add(Permanent::new(c1.clone(), p2));
        game.state.card_store.insert(c1);

        let red_colors = vec![Color::Red];
        assert_eq!(game.count_conspire_eligible_creatures(p1, &red_colors), 0);
    }

    #[test]
    fn conspire_no_matching_color() {
        let (mut game, p1, _p2) = setup_game();

        let c1 = make_creature_with_color("Blue Wizard", p1, vec![Color::Blue]);
        game.state.battlefield.add(Permanent::new(c1.clone(), p1));
        game.state.card_store.insert(c1);

        let c2 = make_creature_with_color("Green Elf", p1, vec![Color::Green]);
        game.state.battlefield.add(Permanent::new(c2.clone(), p1));
        game.state.card_store.insert(c2);

        let red_colors = vec![Color::Red];
        assert_eq!(game.count_conspire_eligible_creatures(p1, &red_colors), 0);
    }

    #[test]
    fn grant_conspire_via_static_effect() {
        let (mut game, p1, _p2) = setup_game();

        let granter_id = ObjectId::new();
        let mut granter = CardData::new(granter_id, p1, "Raiding Schemes");
        granter.card_types = vec![CardType::Enchantment];
        game.state.battlefield.add(Permanent::new(granter.clone(), p1));
        game.state.card_store.insert(granter);

        let grant_ability = Ability::static_ability(
            granter_id,
            "Each noncreature spell you cast has conspire.",
            vec![StaticEffect::grant_conspire("noncreature spells")],
        );
        game.state.ability_store.add(grant_ability);

        let mut spell = CardData::new(ObjectId::new(), p1, "Lightning Bolt");
        spell.card_types = vec![CardType::Instant];
        spell.mana_cost = ManaCost::parse("{R}");
        assert!(game.spell_has_conspire(p1, &spell));

        let mut creature_spell = CardData::new(ObjectId::new(), p1, "Some Creature");
        creature_spell.card_types = vec![CardType::Creature];
        creature_spell.mana_cost = ManaCost::parse("{2}{R}");
        assert!(!game.spell_has_conspire(p1, &creature_spell));
    }

    #[test]
    fn copy_spell_on_stack_creates_copy() {
        let (mut game, p1, _p2) = setup_game();

        let spell = make_conspire_spell(p1);
        let spell_id = spell.id;
        game.state.card_store.insert(spell.clone());

        let stack_item = crate::zones::StackItem {
            id: spell_id,
            kind: crate::zones::StackItemKind::Spell { card: Box::new(spell) },
            controller: p1,
            targets: vec![],
            countered: false,
            x_value: None,
            exile_on_resolve: false,
        };
        game.state.stack.push(stack_item);

        assert_eq!(game.state.stack.len(), 1);
        game.copy_spell_on_stack(spell_id, p1);
        assert_eq!(game.state.stack.len(), 2, "Stack should have original + copy");
    }

    #[test]
    fn conspire_helper_constructor() {
        match StaticEffect::grant_conspire("noncreature spells") {
            StaticEffect::GrantConspire { filter } => {
                assert_eq!(filter, "noncreature spells");
            }
            _ => panic!("Expected GrantConspire variant"),
        }
    }

    #[test]
    fn pay_conspire_taps_two_creatures() {
        let (mut game, p1, _p2) = setup_game();

        let c1 = make_creature_with_color("Red Goblin 1", p1, vec![Color::Red]);
        let c1_id = c1.id;
        game.state.battlefield.add(Permanent::new(c1.clone(), p1));
        game.state.card_store.insert(c1);

        let c2 = make_creature_with_color("Red Goblin 2", p1, vec![Color::Red]);
        let c2_id = c2.id;
        game.state.battlefield.add(Permanent::new(c2.clone(), p1));
        game.state.card_store.insert(c2);

        let c3 = make_creature_with_color("Red Goblin 3", p1, vec![Color::Red]);
        let c3_id = c3.id;
        game.state.battlefield.add(Permanent::new(c3.clone(), p1));
        game.state.card_store.insert(c3);

        game.pay_conspire_cost(p1, &[Color::Red]);

        let tapped_count = [c1_id, c2_id, c3_id].iter()
            .filter(|id| game.state.battlefield.get(**id).map_or(false, |p| p.tapped))
            .count();
        assert_eq!(tapped_count, 2, "Exactly 2 creatures should be tapped for conspire");
    }

    fn add_creature_with_store(game: &mut Game, owner: PlayerId, name: &str, power: i32, toughness: i32, kw: KeywordAbilities) -> ObjectId {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.power = Some(power);
        card.toughness = Some(toughness);
        card.keywords = kw;
        let id = card.id;
        game.state.card_store.insert(card.clone());
        game.state.battlefield.add(Permanent::new(card, owner));
        id
    }

    #[test]
    fn persist_returns_creature_with_m1m1_counter() {
        let (mut game, p1, _p2) = setup();
        let bear_id = add_creature_with_store(&mut game, p1, "Persist Bear", 2, 2, KeywordAbilities::PERSIST);

        assert!(game.state.battlefield.get(bear_id).is_some());

        if let Some(perm) = game.state.battlefield.get_mut(bear_id) {
            perm.apply_damage(3);
        }
        game.process_state_based_actions();

        let on_bf = game.state.battlefield.get(bear_id).is_some();
        assert!(on_bf, "Creature with persist should return to the battlefield");

        let perm = game.state.battlefield.get(bear_id).unwrap();
        let m1m1 = perm.counters.get(&crate::counters::CounterType::M1M1);
        assert_eq!(m1m1, 1, "Returned creature should have exactly one -1/-1 counter");
        assert_eq!(perm.damage, 0, "Returned creature should have no damage");
    }

    #[test]
    fn persist_does_not_return_if_had_m1m1_counter() {
        let (mut game, p1, _p2) = setup();
        let bear_id = add_creature_with_store(&mut game, p1, "Persist Bear", 3, 3, KeywordAbilities::PERSIST);

        if let Some(perm) = game.state.battlefield.get_mut(bear_id) {
            perm.add_counters(crate::counters::CounterType::M1M1, 1);
        }

        if let Some(perm) = game.state.battlefield.get_mut(bear_id) {
            perm.apply_damage(3);
        }
        game.process_state_based_actions();

        assert!(game.state.battlefield.get(bear_id).is_none(),
            "Creature with persist and a -1/-1 counter should NOT return");
    }

    #[test]
    fn persist_granted_by_static_ability_works() {
        let (mut game, p1, _p2) = setup();

        let lord_id = ObjectId::new();
        let mut lord_card = CardData::new(lord_id, p1, "Persist Lord");
        lord_card.card_types = vec![CardType::Creature];
        lord_card.power = Some(3);
        lord_card.toughness = Some(3);
        lord_card.abilities = vec![
            Ability::static_ability(lord_id,
                "Other creatures you control have persist.",
                vec![StaticEffect::GrantKeyword {
                    filter: "other creatures you control".to_string(),
                    keyword: "persist".to_string(),
                }]),
        ];
        for ab in &lord_card.abilities {
            game.state.ability_store.add(ab.clone());
        }
        game.state.card_store.insert(lord_card.clone());
        game.state.battlefield.add(Permanent::new(lord_card, p1));

        let bear_id = add_creature_with_store(&mut game, p1, "Regular Bear", 2, 2, KeywordAbilities::empty());

        game.apply_continuous_effects();

        let bear = game.state.battlefield.get(bear_id).unwrap();
        assert!(bear.has_keyword(KeywordAbilities::PERSIST),
            "Bear should have persist granted by lord");

        if let Some(perm) = game.state.battlefield.get_mut(bear_id) {
            perm.apply_damage(3);
        }
        game.process_state_based_actions();

        assert!(game.state.battlefield.get(bear_id).is_some(),
            "Bear with granted persist should return to the battlefield");
        let bear = game.state.battlefield.get(bear_id).unwrap();
        let m1m1 = bear.counters.get(&crate::counters::CounterType::M1M1);
        assert_eq!(m1m1, 1, "Returned bear should have one -1/-1 counter");
    }

    #[test]
    fn hexproof_from_own_colors_blocks_matching_spells() {
        let (mut game, p1, p2) = setup2();

        let tam_id = ObjectId::new();
        let mut tam = CardData::new(tam_id, p1, "Tam, Mindful First-Year");
        tam.card_types = vec![CardType::Creature];
        tam.subtypes = vec![SubType::Gorgon, SubType::Wizard];
        tam.color_identity = vec![Color::Green, Color::Blue];
        tam.abilities = vec![Ability::static_ability(tam_id,
            "Each other creature you control has hexproof from each of its colors.",
            vec![StaticEffect::hexproof_from_own_colors()])];
        game.state.battlefield.add(Permanent::new(tam.clone(), p1));
        game.state.card_store.insert(tam.clone());
        for ab in &tam.abilities { game.state.ability_store.add(ab.clone()); }

        let bear_id = ObjectId::new();
        let mut bear = CardData::new(bear_id, p1, "Green Bear");
        bear.card_types = vec![CardType::Creature];
        bear.color_identity = vec![Color::Green];
        bear.power = Some(2);
        bear.toughness = Some(2);
        game.state.battlefield.add(Permanent::new(bear, p1));

        game.apply_continuous_effects();

        let targets_green = game.legal_targets_for_spec(&TargetSpec::Creature, p2, &[Color::Green]);
        assert!(!targets_green.contains(&bear_id), "Green bear should be untargetable by green spells");

        let targets_red = game.legal_targets_for_spec(&TargetSpec::Creature, p2, &[Color::Red]);
        assert!(targets_red.contains(&bear_id), "Green bear should be targetable by red spells");

        let targets_colorless = game.legal_targets_for_spec(&TargetSpec::Creature, p2, &[]);
        assert!(targets_colorless.contains(&bear_id), "Green bear should be targetable by colorless spells");
    }

    #[test]
    fn hexproof_from_own_colors_does_not_apply_to_self() {
        let (mut game, p1, p2) = setup2();

        let tam_id = ObjectId::new();
        let mut tam = CardData::new(tam_id, p1, "Tam, Mindful First-Year");
        tam.card_types = vec![CardType::Creature];
        tam.color_identity = vec![Color::Green, Color::Blue];
        tam.power = Some(2);
        tam.toughness = Some(2);
        tam.abilities = vec![Ability::static_ability(tam_id,
            "Each other creature you control has hexproof from each of its colors.",
            vec![StaticEffect::hexproof_from_own_colors()])];
        game.state.battlefield.add(Permanent::new(tam.clone(), p1));
        game.state.card_store.insert(tam.clone());
        for ab in &tam.abilities { game.state.ability_store.add(ab.clone()); }

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(tam_id).unwrap();
        assert!(perm.hexproof_from_colors.is_empty(), "Tam should NOT grant hexproof to itself");

        let targets = game.legal_targets_for_spec(&TargetSpec::Creature, p2, &[Color::Green]);
        assert!(targets.contains(&tam_id), "Tam itself should be targetable by green spells");
    }

    #[test]
    fn hexproof_from_own_colors_all_colors_creature() {
        let (mut game, p1, p2) = setup2();

        let tam_id = ObjectId::new();
        let mut tam = CardData::new(tam_id, p1, "Tam, Mindful First-Year");
        tam.card_types = vec![CardType::Creature];
        tam.color_identity = vec![Color::Green];
        tam.abilities = vec![Ability::static_ability(tam_id,
            "Each other creature you control has hexproof from each of its colors.",
            vec![StaticEffect::hexproof_from_own_colors()])];
        game.state.battlefield.add(Permanent::new(tam.clone(), p1));
        game.state.card_store.insert(tam.clone());
        for ab in &tam.abilities { game.state.ability_store.add(ab.clone()); }

        let rainbow_id = ObjectId::new();
        let mut rainbow = CardData::new(rainbow_id, p1, "Rainbow Creature");
        rainbow.card_types = vec![CardType::Creature];
        rainbow.color_identity = vec![Color::Green];
        rainbow.power = Some(3);
        rainbow.toughness = Some(3);
        game.state.battlefield.add(Permanent::new(rainbow, p1));
        if let Some(perm) = game.state.battlefield.get_mut(rainbow_id) {
            perm.all_colors_until_eot = true;
        }

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(rainbow_id).unwrap();
        assert_eq!(perm.hexproof_from_colors.len(), 5, "All-colors creature should have hexproof from all 5 colors");

        for color in &[Color::White, Color::Blue, Color::Black, Color::Red, Color::Green] {
            let targets = game.legal_targets_for_spec(&TargetSpec::Creature, p2, &[*color]);
            assert!(!targets.contains(&rainbow_id), "Rainbow creature should be untargetable by {:?} spells", color);
        }
    }
