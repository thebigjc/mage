// Tests extracted from game.rs

use crate::game::*;
use crate::abilities::{Ability, Effect, TargetSpec, StaticEffect};
use crate::card::CardData;
use crate::constants::{CardType, Color, KeywordAbilities, Outcome, SubType, SuperType};
use crate::decision::{AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction, PlayerAgent, PlayerDecisionMaker, ReplacementEffectChoice, TargetRequirement, UnpaidMana};
use crate::filters::Filter;
use crate::mana::Mana;
use crate::permanent::Permanent;
use crate::types::{ObjectId, PlayerId, Power, Toughness};


#[cfg(test)]
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
            starting_life: Life::new(20),
        };
        let game = Game::new_two_player(
            config,
            vec![(p1, PlayerAgent::new(PassivePlayer)), (p2, PlayerAgent::new(PassivePlayer))],
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
        card.power = Some(Power::new(power));
        card.toughness = Some(Toughness::new(toughness));
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
        card.power = Some(Power::new(power));
        card.toughness = Some(Toughness::new(toughness));
        let id = card.id;
        let perm = Permanent::new(card, owner);
        game.state.battlefield.add(perm);
        id
    }

    #[allow(clippy::too_many_arguments)]
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
        card.power = Some(Power::new(power));
        card.toughness = Some(Toughness::new(toughness));
        let id = card.id;
        card.abilities = vec![
            Ability::static_ability(id, &format!("Other creatures get +{boost_p}/+{boost_t}"),
                vec![StaticEffect::Boost { filter: Filter::parse(filter), power: Power::new(boost_p), toughness: Toughness::new(boost_t) }]),
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
        card.power = Some(Power::new(power));
        card.toughness = Some(Toughness::new(toughness));
        let id = card.id;
        card.abilities = vec![
            Ability::static_ability(id, &format!("Creatures have {keyword}"),
                vec![StaticEffect::GrantKeyword { filter: Filter::parse(filter), keyword: keyword.into() }]),
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
            SubType::Spirit, "creature you control", 1, 1);

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
        let _ = add_keyword_lord(&mut game, p1, "Archetype of Imagination", 3, 2,
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
        let _ = add_keyword_lord(&mut game, p1, "Basilisk Collar", 0, 0,
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
        let _ = add_lord_with_boost(&mut game, p1, "Lord 1", 2, 2,
            SubType::Elf, "other Elf you control", 1, 1);
        let _ = add_lord_with_boost(&mut game, p1, "Lord 2", 2, 2,
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
        card.power = Some(Power::new(1));
        card.toughness = Some(Toughness::new(1));
        let id = card.id;
        card.abilities = vec![
            Ability::static_ability(id, "+2/+2 to self",
                vec![StaticEffect::Boost { filter: Filter::parse("self"), power: Power::new(2), toughness: Toughness::new(2) }]),
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
        let _ = add_lord_with_boost(&mut game, p1, "Token Lord", 2, 2,
            SubType::Human, "creature token you control", 1, 1);

        // Regular creature
        let regular_id = add_creature(&mut game, p1, "Regular Bear", 2, 2, KeywordAbilities::empty());

        // Token creature
        let token_id = ObjectId::new();
        let mut token_card = CardData::new(token_id, p1, "Bear Token");
        token_card.card_types = vec![CardType::Creature];
        token_card.power = Some(Power::new(2));
        token_card.toughness = Some(Toughness::new(2));
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
        let _ = add_lord_with_boost(&mut game, p2, "Enemy Lord", 2, 2,
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
        card.power = Some(Power::new(2));
        card.toughness = Some(Toughness::new(2));
        let id = card.id;
        card.abilities = vec![
            Ability::static_ability(id, "Other Spirit creatures you control get +1/+1 and have hexproof",
                vec![
                    StaticEffect::Boost { filter: Filter::parse("other Spirit you control"), power: Power::new(1), toughness: Toughness::new(1) },
                    StaticEffect::GrantKeyword { filter: Filter::parse("other Spirit you control"), keyword: "hexproof".into() },
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


    // Additional tests

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
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig { players: vec![PlayerConfig { name: "P1".to_string(), deck: vec![] }, PlayerConfig { name: "P2".to_string(), deck: vec![] }], starting_life: Life::new(20) };
        let game = Game::new_two_player(config, vec![
            (p1, PlayerAgent::new(PassPlayer)),
            (p2, PlayerAgent::new(PassPlayer)),
        ]);
        (game, p1, p2)
    }

    #[test]
    fn conditional_keyword_your_turn() {
        let (mut game, p1, _p2) = make_test_game();

        // Create creature with "first strike on your turn"
        let card_id = ObjectId::new();
        let card = CardData {
            id: card_id, owner: p1, name: "First Strike Guy".into(),
            card_types: vec![crate::constants::CardType::Creature],
            power: Some(Power::new(2)), toughness: Some(Toughness::new(1)),
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

        let card_id = ObjectId::new();
        let card = CardData {
            id: card_id, owner: p1, name: "Hexproof Untapped".into(),
            card_types: vec![crate::constants::CardType::Creature],
            power: Some(Power::new(3)), toughness: Some(Toughness::new(3)),
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
        let card_id = ObjectId::new();
        let card = CardData {
            id: card_id, owner: p1, name: "Faerie Pal".into(),
            card_types: vec![crate::constants::CardType::Creature],
            power: Some(Power::new(2)), toughness: Some(Toughness::new(2)),
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
        let faerie_id = ObjectId::new();
        let faerie = CardData {
            id: faerie_id, owner: p1, name: "Faerie Token".into(),
            card_types: vec![crate::constants::CardType::Creature],
            subtypes: vec![crate::constants::SubType::Faerie],
            power: Some(Power::new(1)), toughness: Some(Toughness::new(1)),
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

        let card_id = ObjectId::new();
        let card = CardData {
            id: card_id, owner: p1, name: "Boost on ETB".into(),
            card_types: vec![crate::constants::CardType::Creature],
            power: Some(Power::new(3)), toughness: Some(Toughness::new(3)),
            abilities: vec![Ability::static_ability(card_id, "+2/+0 if creature entered this turn.",
                vec![StaticEffect::ConditionalBoostSelf { power: Power::new(2), toughness: Toughness::new(0), condition: "creature entered this turn".into() }])],
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
        game.emit_event(crate::events::GameEvent::enters_battlefield(ObjectId::new(), p1));
        game.apply_continuous_effects();
        let perm = game.state.battlefield.get(card_id).unwrap();
        assert_eq!(perm.power(), 5, "should be 3+2 with ETB event this turn");
    }


    // Additional tests

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
        card.power = Some(Power::new(power));
        card.toughness = Some(Toughness::new(toughness));
        card.keywords = KeywordAbilities::empty();
        card
    }

    fn make_deck2(owner: PlayerId) -> Vec<CardData> {
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
                PlayerConfig { name: "Alice".to_string(), deck: make_deck2(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck2(p2) },
            ],
            starting_life: Life::new(20),
        };
        let game = Game::new_two_player(
            config,
            vec![
                (p1, PlayerAgent::new(AlwaysPassPlayer)),
                (p2, PlayerAgent::new(AlwaysPassPlayer)),
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
        card.power = Some(Power::new(5));
        card.toughness = Some(Toughness::new(5));
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
        card.power = Some(Power::new(1));
        card.toughness = Some(Toughness::new(1));
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
        card.power = Some(Power::new(4));
        card.toughness = Some(Toughness::new(4));
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
        card.power = Some(Power::new(2));
        card.toughness = Some(Toughness::new(2));
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
        creature.power = Some(Power::new(5));
        creature.toughness = Some(Toughness::new(5));
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
                vec![StaticEffect::LoseAllAbilities { filter: Filter::parse("enchanted creature") }]),
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


    fn setup_game2() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "Alice".to_string(), deck: make_deck2(p1) },
                PlayerConfig { name: "Bob".to_string(), deck: make_deck2(p2) },
            ],
            starting_life: Life::new(20),
        };
        let game = Game::new_two_player(
            config,
            vec![
                (p1, PlayerAgent::new(AlwaysPassPlayer)),
                (p2, PlayerAgent::new(AlwaysPassPlayer)),
            ],
        );
        (game, p1, p2)
    }

    #[test]
    fn set_base_pt_all_opponents_creatures() {
        let (mut game, p1, p2) = setup_game2();

        let bear_id = ObjectId::new();
        let mut bear = CardData::new(bear_id, p2, "Big Bear");
        bear.card_types = vec![CardType::Creature];
        bear.power = Some(Power::new(5));
        bear.toughness = Some(Toughness::new(5));
        bear.keywords = KeywordAbilities::TRAMPLE;
        game.state.battlefield.add(crate::permanent::Permanent::new(bear, p2));

        let angel_id = ObjectId::new();
        let mut angel = CardData::new(angel_id, p2, "Angel");
        angel.card_types = vec![CardType::Creature];
        angel.power = Some(Power::new(4));
        angel.toughness = Some(Toughness::new(4));
        angel.keywords = KeywordAbilities::FLYING;
        game.state.battlefield.add(crate::permanent::Permanent::new(angel, p2));

        let own_id = ObjectId::new();
        let mut own = CardData::new(own_id, p1, "Own Bear");
        own.card_types = vec![CardType::Creature];
        own.power = Some(Power::new(3));
        own.toughness = Some(Toughness::new(3));
        game.state.battlefield.add(crate::permanent::Permanent::new(own, p1));

        game.execute_effects(
            &[Effect::SetBasePowerToughnessAll { power: Power::new(1), toughness: Toughness::new(1), filter: Filter::parse("creatures opponents control") }],
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
        bear.power = Some(Power::new(3));
        bear.toughness = Some(Toughness::new(3));
        bear.keywords = KeywordAbilities::FLYING | KeywordAbilities::TRAMPLE;
        game.state.battlefield.add(crate::permanent::Permanent::new(bear, p2));

        let own_id = ObjectId::new();
        let mut own = CardData::new(own_id, p1, "Own Flyer");
        own.card_types = vec![CardType::Creature];
        own.power = Some(Power::new(2));
        own.toughness = Some(Toughness::new(2));
        own.keywords = KeywordAbilities::FLYING;
        game.state.battlefield.add(crate::permanent::Permanent::new(own, p1));

        game.execute_effects(
            &[Effect::LoseAllAbilitiesAll { filter: Filter::parse("creatures opponents control") }],
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
        creature.power = Some(Power::new(5));
        creature.toughness = Some(Toughness::new(5));
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
                    filter: Filter::parse("enchanted creature"),
                    power: Power::new(1),
                    toughness: Toughness::new(1),
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
        creature.power = Some(Power::new(5));
        creature.toughness = Some(Toughness::new(5));
        game.state.battlefield.add(crate::permanent::Permanent::new(creature, p1));

        let lord_id = ObjectId::new();
        let mut lord = CardData::new(lord_id, p1, "Lord");
        lord.card_types = vec![CardType::Creature];
        lord.power = Some(Power::new(2));
        lord.toughness = Some(Toughness::new(2));
        lord.abilities = vec![
            Ability::static_ability(lord_id,
                "Other creatures you control get +1/+1.",
                vec![StaticEffect::Boost {
                    filter: Filter::parse("other creatures you control"),
                    power: Power::new(1),
                    toughness: Toughness::new(1),
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
                    filter: Filter::parse("enchanted creature"),
                    power: Power::new(1),
                    toughness: Toughness::new(1),
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
        creature.power = Some(Power::new(5));
        creature.toughness = Some(Toughness::new(5));
        game.state.battlefield.add(crate::permanent::Permanent::new(creature, p1));

        let aura_id = ObjectId::new();
        let mut aura = CardData::new(aura_id, p1, "Shrinking Aura");
        aura.card_types = vec![CardType::Enchantment];
        aura.subtypes = vec![SubType::Aura];
        aura.abilities = vec![
            Ability::static_ability(aura_id,
                "Enchanted creature has base power and toughness 1/1.",
                vec![StaticEffect::SetBasePowerToughness {
                    filter: Filter::parse("enchanted creature"),
                    power: Power::new(1),
                    toughness: Toughness::new(1),
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
        bear.power = Some(Power::new(3));
        bear.toughness = Some(Toughness::new(3));
        game.state.battlefield.add(crate::permanent::Permanent::new(bear, p2));

        game.execute_effects(
            &[Effect::SetBasePowerToughnessAll { power: Power::new(1), toughness: Toughness::new(1), filter: Filter::parse("creatures opponents control") }],
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
            starting_life: Life::new(20),
        };
        let game = Game::new_two_player(config, vec![(p1, PlayerAgent::new(PassPlayer)), (p2, PlayerAgent::new(PassPlayer))]);
        (game, p1, p2)
    }

    fn add_colored_creature(game: &mut Game, owner: PlayerId, name: &str, colors: Vec<Color>) -> ObjectId {
        let mut card = CardData::new(ObjectId::new(), owner, name);
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![SubType::Elemental];
        card.power = Some(Power::new(1));
        card.toughness = Some(Toughness::new(1));
        card.color_identity = colors;
        let id = card.id;
        game.state.battlefield.add(Permanent::new(card, owner));
        id
    }

    fn add_vivid_creature(game: &mut Game, owner: PlayerId) -> ObjectId {
        let mut card = CardData::new(ObjectId::new(), owner, "Squawkroaster");
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![SubType::Elemental];
        card.power = Some(Power::new(0));
        card.toughness = Some(Toughness::new(4));
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
        let _land_id = land.id;
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
        let _ = add_colored_creature(&mut game, p1, "Green Elf", vec![Color::Green]);
        let _ = add_colored_creature(&mut game, p1, "Blue Wizard", vec![Color::Blue]);

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(vivid_id).unwrap();
        assert_eq!(perm.power(), 3, "red + green + blue = power 3");
    }

    #[test]
    fn power_equals_five_with_all_colors() {
        let (mut game, p1, _p2) = make_game();

        let vivid_id = add_vivid_creature(&mut game, p1);
        let _ = add_colored_creature(&mut game, p1, "White Knight", vec![Color::White]);
        let _ = add_colored_creature(&mut game, p1, "Blue Mage", vec![Color::Blue]);
        let _ = add_colored_creature(&mut game, p1, "Black Rogue", vec![Color::Black]);
        let _ = add_colored_creature(&mut game, p1, "Green Beast", vec![Color::Green]);

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(vivid_id).unwrap();
        assert_eq!(perm.power(), 5, "all five colors = power 5");
        assert_eq!(perm.toughness(), 4, "toughness unchanged");
    }

    #[test]
    fn multicolored_permanent_counts_multiple_colors() {
        let (mut game, p1, _p2) = make_game();

        let vivid_id = add_vivid_creature(&mut game, p1);
        let _ = add_colored_creature(&mut game, p1, "Niv-Mizzet", vec![Color::Blue, Color::Red]);
        let _ = add_colored_creature(&mut game, p1, "Siege Rhino", vec![Color::White, Color::Black, Color::Green]);

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(vivid_id).unwrap();
        assert_eq!(perm.power(), 5, "W+U+B+R+G from multicolor = power 5");
    }

    #[test]
    fn duplicate_colors_not_double_counted() {
        let (mut game, p1, _p2) = make_game();

        let vivid_id = add_vivid_creature(&mut game, p1);
        let _ = add_colored_creature(&mut game, p1, "Red Goblin 1", vec![Color::Red]);
        let _ = add_colored_creature(&mut game, p1, "Red Goblin 2", vec![Color::Red]);

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(vivid_id).unwrap();
        assert_eq!(perm.power(), 1, "multiple red permanents still = 1 color");
    }

    #[test]
    fn opponent_permanents_dont_count() {
        let (mut game, p1, p2) = make_game();

        let vivid_id = add_vivid_creature(&mut game, p1);
        let _ = add_colored_creature(&mut game, p2, "Opponent Blue", vec![Color::Blue]);
        let _ = add_colored_creature(&mut game, p2, "Opponent Green", vec![Color::Green]);

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

// ── DamageDoublingFromType ─────────────────────────────────────────────────

mod damage_doubling_tests {
    use super::*;

    fn setup_with_enchantment(chosen: SubType) -> (Game, PlayerId, PlayerId, ObjectId) {
        let (mut game, p1, p2) = setup();

        let mut ench_card = CardData::new(ObjectId::new(), p1, "Damage Doubler");
        ench_card.card_types = vec![CardType::Enchantment];
        let ench_id = ench_card.id;
        let ability = Ability::static_ability(ench_id,
            "Double all damage from chosen type.",
            vec![StaticEffect::damage_doubling_from_type()]);
        let mut ench_perm = Permanent::new(ench_card, p1);
        ench_perm.chosen_type = Some(chosen);
        game.state.battlefield.add(ench_perm);
        game.state.ability_store.add(ability);

        (game, p1, p2, ench_id)
    }

    #[test]
    fn doubles_combat_damage_from_matching_type() {
        let (mut game, p1, p2, _ench_id) = setup_with_enchantment(SubType::Goblin);

        let mut goblin = CardData::new(ObjectId::new(), p1, "Goblin Raider");
        goblin.card_types = vec![CardType::Creature];
        goblin.subtypes = vec![SubType::Goblin];
        goblin.power = Some(Power::new(3));
        goblin.toughness = Some(Toughness::new(2));
        goblin.keywords = KeywordAbilities::empty();
        let goblin_id = goblin.id;
        game.state.battlefield.add(Permanent::new(goblin, p1));

        let mut target = CardData::new(ObjectId::new(), p2, "Big Wall");
        target.card_types = vec![CardType::Creature];
        target.power = Some(Power::new(0));
        target.toughness = Some(Toughness::new(10));
        target.keywords = KeywordAbilities::empty();
        let target_id = target.id;
        game.state.battlefield.add(Permanent::new(target, p2));

        game.apply_continuous_effects();

        assert_eq!(game.state.damage_doublings.len(), 1);
        assert_eq!(game.get_damage_multiplier(goblin_id), 2);

        game.execute_effects(
            &[Effect::DealDamage { amount: 3 }],
            p1,
            &[target_id],
            Some(goblin_id),
            None,
        );

        let wall = game.state.battlefield.get(target_id).unwrap();
        assert_eq!(wall.damage, 6, "3 damage doubled to 6");
    }

    #[test]
    fn no_doubling_for_non_matching_type() {
        let (mut game, p1, _p2, _ench_id) = setup_with_enchantment(SubType::Goblin);

        let mut elf = CardData::new(ObjectId::new(), p1, "Elf Archer");
        elf.card_types = vec![CardType::Creature];
        elf.subtypes = vec![SubType::Elf];
        elf.power = Some(Power::new(2));
        elf.toughness = Some(Toughness::new(2));
        elf.keywords = KeywordAbilities::empty();
        let elf_id = elf.id;
        game.state.battlefield.add(Permanent::new(elf, p1));

        game.apply_continuous_effects();

        assert_eq!(game.get_damage_multiplier(elf_id), 1);
    }

    #[test]
    fn no_doubling_for_opponent_creatures() {
        let (mut game, _p1, p2, _ench_id) = setup_with_enchantment(SubType::Goblin);

        let mut opp_goblin = CardData::new(ObjectId::new(), p2, "Enemy Goblin");
        opp_goblin.card_types = vec![CardType::Creature];
        opp_goblin.subtypes = vec![SubType::Goblin];
        opp_goblin.power = Some(Power::new(2));
        opp_goblin.toughness = Some(Toughness::new(2));
        opp_goblin.keywords = KeywordAbilities::empty();
        let opp_id = opp_goblin.id;
        game.state.battlefield.add(Permanent::new(opp_goblin, p2));

        game.apply_continuous_effects();

        assert_eq!(game.get_damage_multiplier(opp_id), 1);
    }

    #[test]
    fn changeling_matches_any_chosen_type() {
        let (mut game, p1, _p2, _ench_id) = setup_with_enchantment(SubType::Dragon);

        let mut changeling = CardData::new(ObjectId::new(), p1, "Changeling Outcast");
        changeling.card_types = vec![CardType::Creature];
        changeling.power = Some(Power::new(1));
        changeling.toughness = Some(Toughness::new(1));
        changeling.keywords = KeywordAbilities::CHANGELING;
        let ch_id = changeling.id;
        game.state.battlefield.add(Permanent::new(changeling, p1));

        game.apply_continuous_effects();

        assert_eq!(game.get_damage_multiplier(ch_id), 2, "changeling is every creature type");
    }

    #[test]
    fn doubling_removed_when_enchantment_leaves() {
        let (mut game, p1, _p2, ench_id) = setup_with_enchantment(SubType::Goblin);

        let mut goblin = CardData::new(ObjectId::new(), p1, "Goblin");
        goblin.card_types = vec![CardType::Creature];
        goblin.subtypes = vec![SubType::Goblin];
        goblin.power = Some(Power::new(2));
        goblin.toughness = Some(Toughness::new(2));
        goblin.keywords = KeywordAbilities::empty();
        let goblin_id = goblin.id;
        game.state.battlefield.add(Permanent::new(goblin, p1));

        game.apply_continuous_effects();
        assert_eq!(game.get_damage_multiplier(goblin_id), 2);

        game.state.battlefield.remove(ench_id);
        game.state.ability_store.remove_source(ench_id);
        game.apply_continuous_effects();
        assert_eq!(game.get_damage_multiplier(goblin_id), 1, "no doubling after enchantment removed");
    }

    #[test]
    fn multiple_doublings_stack_multiplicatively() {
        let (mut game, p1, _p2, _) = setup_with_enchantment(SubType::Goblin);

        let mut ench2 = CardData::new(ObjectId::new(), p1, "Damage Doubler 2");
        ench2.card_types = vec![CardType::Enchantment];
        let ench2_id = ench2.id;
        let ability2 = Ability::static_ability(ench2_id,
            "Double all damage from chosen type.",
            vec![StaticEffect::damage_doubling_from_type()]);
        let mut ench2_perm = Permanent::new(ench2, p1);
        ench2_perm.chosen_type = Some(SubType::Goblin);
        game.state.battlefield.add(ench2_perm);
        game.state.ability_store.add(ability2);

        let mut goblin = CardData::new(ObjectId::new(), p1, "Goblin");
        goblin.card_types = vec![CardType::Creature];
        goblin.subtypes = vec![SubType::Goblin];
        goblin.power = Some(Power::new(1));
        goblin.toughness = Some(Toughness::new(1));
        goblin.keywords = KeywordAbilities::empty();
        let goblin_id = goblin.id;
        game.state.battlefield.add(Permanent::new(goblin, p1));

        game.apply_continuous_effects();

        assert_eq!(game.get_damage_multiplier(goblin_id), 4, "2 * 2 = 4x damage");
    }

    #[test]
    fn no_chosen_type_means_no_doubling() {
        let (mut game, p1, _p2) = setup();

        let mut ench = CardData::new(ObjectId::new(), p1, "Empty Doubler");
        ench.card_types = vec![CardType::Enchantment];
        let ench_id = ench.id;
        let ability = Ability::static_ability(ench_id,
            "Double all damage from chosen type.",
            vec![StaticEffect::damage_doubling_from_type()]);
        let ench_perm = Permanent::new(ench, p1);
        game.state.battlefield.add(ench_perm);
        game.state.ability_store.add(ability);

        let mut goblin = CardData::new(ObjectId::new(), p1, "Goblin");
        goblin.card_types = vec![CardType::Creature];
        goblin.subtypes = vec![SubType::Goblin];
        goblin.power = Some(Power::new(2));
        goblin.toughness = Some(Toughness::new(2));
        goblin.keywords = KeywordAbilities::empty();
        let goblin_id = goblin.id;
        game.state.battlefield.add(Permanent::new(goblin, p1));

        game.apply_continuous_effects();

        assert!(game.state.damage_doublings.is_empty(), "no chosen type, no doubling entry");
        assert_eq!(game.get_damage_multiplier(goblin_id), 1);
    }

    #[test]
    fn doubles_deal_damage_to_player() {
        let (mut game, p1, p2, _ench_id) = setup_with_enchantment(SubType::Goblin);

        let mut goblin = CardData::new(ObjectId::new(), p1, "Goblin Shaman");
        goblin.card_types = vec![CardType::Creature];
        goblin.subtypes = vec![SubType::Goblin];
        goblin.power = Some(Power::new(1));
        goblin.toughness = Some(Toughness::new(1));
        goblin.keywords = KeywordAbilities::empty();
        let goblin_id = goblin.id;
        game.state.battlefield.add(Permanent::new(goblin, p1));

        game.apply_continuous_effects();

        let starting_life = game.state.players.get(&p2).unwrap().life;

        game.execute_effects(
            &[Effect::DealDamage { amount: 2 }],
            p1,
            &[],
            Some(goblin_id),
            None,
        );

        let final_life = game.state.players.get(&p2).unwrap().life;
        assert_eq!(starting_life - final_life, 4, "2 damage doubled to 4 to opponent");
    }

    #[test]
    fn helper_constructor_damage_doubling() {
        match StaticEffect::damage_doubling_from_type() {
            StaticEffect::DamageDoublingFromType => {}
            _ => panic!("wrong variant"),
        }
    }
}

mod mana_doubling_basic_lands_tests {
    use super::*;

    fn add_basic_land(game: &mut Game, owner: PlayerId, name: &str, mana: Mana) -> (ObjectId, crate::types::AbilityId) {
        let id = ObjectId::new();
        let mut card = CardData::new(id, owner, name);
        card.card_types = vec![CardType::Land];
        card.supertypes = vec![SuperType::Basic];
        let mana_ability = Ability::mana_ability(id, &format!("{{T}}: Add {mana}."), mana);
        let ability_id = mana_ability.id;
        card.abilities.push(mana_ability.clone());
        let perm = Permanent::new(card, owner);
        game.state.battlefield.add(perm);
        game.state.ability_store.add(mana_ability);
        (id, ability_id)
    }

    fn add_nonbasic_land(game: &mut Game, owner: PlayerId, name: &str, mana: Mana) -> (ObjectId, crate::types::AbilityId) {
        let id = ObjectId::new();
        let mut card = CardData::new(id, owner, name);
        card.card_types = vec![CardType::Land];
        let mana_ability = Ability::mana_ability(id, &format!("{{T}}: Add {mana}."), mana);
        let ability_id = mana_ability.id;
        card.abilities.push(mana_ability.clone());
        let perm = Permanent::new(card, owner);
        game.state.battlefield.add(perm);
        game.state.ability_store.add(mana_ability);
        (id, ability_id)
    }

    fn add_doubler(game: &mut Game, owner: PlayerId) -> ObjectId {
        let mut card = CardData::new(ObjectId::new(), owner, "Mana Doubler");
        card.card_types = vec![CardType::Creature];
        card.power = Some(Power::new(4));
        card.toughness = Some(Toughness::new(4));
        card.keywords = KeywordAbilities::empty();
        let ench_id = card.id;
        let ability = Ability::static_ability(ench_id,
            "Basic lands produce double mana.",
            vec![StaticEffect::mana_doubling_basic_lands()]);
        card.abilities.push(ability.clone());
        game.state.battlefield.add(Permanent::new(card, owner));
        game.state.ability_store.add(ability);
        ench_id
    }

    #[test]
    fn basic_land_produces_double_mana() {
        let (mut game, p1, _p2) = setup();
        let _ = add_doubler(&mut game, p1);
        let (forest_id, ability_id) = add_basic_land(&mut game, p1, "Forest", Mana::green(1));
        game.apply_continuous_effects();

        assert_eq!(game.state.mana_doubling_basic_lands, 1);

        game.activate_mana_ability(p1, forest_id, ability_id);
        let available = game.state.players.get(&p1).unwrap().mana_pool.available();
        assert_eq!(available.green, 2, "basic Forest should produce 2G with doubler");
    }

    #[test]
    fn nonbasic_land_not_doubled() {
        let (mut game, p1, _p2) = setup();
        let _ = add_doubler(&mut game, p1);
        let (land_id, ability_id) = add_nonbasic_land(&mut game, p1, "Mystic Gate", Mana::white(1));
        game.apply_continuous_effects();

        game.activate_mana_ability(p1, land_id, ability_id);
        let available = game.state.players.get(&p1).unwrap().mana_pool.available();
        assert_eq!(available.white, 1, "nonbasic land should produce only 1W");
    }

    #[test]
    fn applies_to_opponent_basic_lands() {
        let (mut game, p1, p2) = setup();
        let _ = add_doubler(&mut game, p1);
        let (opp_forest_id, opp_ability_id) = add_basic_land(&mut game, p2, "Forest", Mana::green(1));
        game.apply_continuous_effects();

        game.activate_mana_ability(p2, opp_forest_id, opp_ability_id);
        let available = game.state.players.get(&p2).unwrap().mana_pool.available();
        assert_eq!(available.green, 2, "opponent's basic Forest should also be doubled");
    }

    #[test]
    fn multiple_doublers_stack() {
        let (mut game, p1, _p2) = setup();
        let _ = add_doubler(&mut game, p1);
        let _ = add_doubler(&mut game, p1);
        let (forest_id, ability_id) = add_basic_land(&mut game, p1, "Forest", Mana::green(1));
        game.apply_continuous_effects();

        assert_eq!(game.state.mana_doubling_basic_lands, 2);

        game.activate_mana_ability(p1, forest_id, ability_id);
        let available = game.state.players.get(&p1).unwrap().mana_pool.available();
        assert_eq!(available.green, 3, "two doublers: base 1 + 1 + 1 = 3G");
    }

    #[test]
    fn removed_when_source_leaves_battlefield() {
        let (mut game, p1, _p2) = setup();
        let doubler_id = add_doubler(&mut game, p1);
        game.apply_continuous_effects();
        assert_eq!(game.state.mana_doubling_basic_lands, 1);

        game.state.battlefield.remove(doubler_id);
        game.state.ability_store.remove_source(doubler_id);
        game.apply_continuous_effects();
        assert_eq!(game.state.mana_doubling_basic_lands, 0);
    }

    #[test]
    fn no_doubling_without_effect() {
        let (mut game, p1, _p2) = setup();
        let (forest_id, ability_id) = add_basic_land(&mut game, p1, "Forest", Mana::green(1));
        game.apply_continuous_effects();

        assert_eq!(game.state.mana_doubling_basic_lands, 0);

        game.activate_mana_ability(p1, forest_id, ability_id);
        let available = game.state.players.get(&p1).unwrap().mana_pool.available();
        assert_eq!(available.green, 1, "no doubler, basic Forest produces only 1G");
    }

    #[test]
    fn colored_mana_doubled_correctly() {
        let (mut game, p1, _p2) = setup();
        let _ = add_doubler(&mut game, p1);
        let (mountain_id, ability_id) = add_basic_land(&mut game, p1, "Mountain", Mana::red(1));
        game.apply_continuous_effects();

        game.activate_mana_ability(p1, mountain_id, ability_id);
        let available = game.state.players.get(&p1).unwrap().mana_pool.available();
        assert_eq!(available.red, 2, "Mountain produces 2R with doubler");
        assert_eq!(available.green, 0);
    }

    #[test]
    fn helper_constructor_mana_doubling() {
        match StaticEffect::mana_doubling_basic_lands() {
            StaticEffect::ManaDoublingBasicLands => {}
            _ => panic!("wrong variant"),
        }
    }
}

mod enhanced_mana_production_tests {
    use super::*;
    use crate::constants::ManaColor;

    fn add_land(game: &mut Game, owner: PlayerId, name: &str, mana: Mana, basic: bool) -> (ObjectId, crate::types::AbilityId) {
        let id = ObjectId::new();
        let mut card = CardData::new(id, owner, name);
        card.card_types = vec![CardType::Land];
        if basic {
            card.supertypes = vec![SuperType::Basic];
        }
        let mana_ability = Ability::mana_ability(id, &format!("{{T}}: Add {mana}."), mana);
        let ability_id = mana_ability.id;
        card.abilities.push(mana_ability.clone());
        let perm = Permanent::new(card, owner);
        game.state.battlefield.add(perm);
        game.state.ability_store.add(mana_ability);
        (id, ability_id)
    }

    fn add_aura_enhancer(game: &mut Game, owner: PlayerId, land_id: ObjectId, color: ManaColor) -> ObjectId {
        let id = ObjectId::new();
        let mut card = CardData::new(id, owner, "Shimmerwilds Growth");
        card.card_types = vec![CardType::Enchantment];
        card.subtypes = vec![SubType::Aura];
        let ability = Ability::static_ability(id,
            "Enchanted land produces additional mana of chosen color.",
            vec![StaticEffect::enhanced_mana_production()]);
        card.abilities.push(ability.clone());
        let mut perm = Permanent::new(card, owner);
        perm.attached_to = Some(land_id);
        perm.chosen_color = Some(color);
        game.state.battlefield.add(perm);
        game.state.ability_store.add(ability);
        if let Some(land) = game.state.battlefield.get_mut(land_id) {
            land.add_attachment(id);
        }
        id
    }

    #[test]
    fn enchanted_land_produces_additional_mana() {
        let (mut game, p1, _p2) = setup();
        let (forest_id, ability_id) = add_land(&mut game, p1, "Forest", Mana::green(1), true);
        let _ = add_aura_enhancer(&mut game, p1, forest_id, ManaColor::Red);
        game.apply_continuous_effects();

        assert_eq!(game.state.enhanced_mana_productions.len(), 1);

        game.activate_mana_ability(p1, forest_id, ability_id);
        let available = game.state.players.get(&p1).unwrap().mana_pool.available();
        assert_eq!(available.green, 1, "Forest still produces 1G");
        assert_eq!(available.red, 1, "additionally produces 1R from aura");
    }

    #[test]
    fn non_enchanted_land_not_affected() {
        let (mut game, p1, _p2) = setup();
        let (forest_id, ability_id) = add_land(&mut game, p1, "Forest", Mana::green(1), true);
        let (other_forest_id, _) = add_land(&mut game, p1, "Forest2", Mana::green(1), true);
        let _ = add_aura_enhancer(&mut game, p1, other_forest_id, ManaColor::Blue);
        game.apply_continuous_effects();

        game.activate_mana_ability(p1, forest_id, ability_id);
        let available = game.state.players.get(&p1).unwrap().mana_pool.available();
        assert_eq!(available.green, 1, "non-enchanted Forest produces only 1G");
        assert_eq!(available.blue, 0, "no bonus blue from non-enchanted land");
    }

    #[test]
    fn works_on_nonbasic_lands() {
        let (mut game, p1, _p2) = setup();
        let (land_id, ability_id) = add_land(&mut game, p1, "Mystic Gate", Mana::white(1), false);
        let _ = add_aura_enhancer(&mut game, p1, land_id, ManaColor::Green);
        game.apply_continuous_effects();

        game.activate_mana_ability(p1, land_id, ability_id);
        let available = game.state.players.get(&p1).unwrap().mana_pool.available();
        assert_eq!(available.white, 1, "nonbasic still produces 1W");
        assert_eq!(available.green, 1, "additionally produces 1G from aura");
    }

    #[test]
    fn removal_reverts_effect() {
        let (mut game, p1, _p2) = setup();
        let (forest_id, _ability_id) = add_land(&mut game, p1, "Forest", Mana::green(1), true);
        let aura_id = add_aura_enhancer(&mut game, p1, forest_id, ManaColor::Blue);
        game.apply_continuous_effects();
        assert_eq!(game.state.enhanced_mana_productions.len(), 1);

        game.state.battlefield.remove(aura_id);
        game.state.ability_store.remove_source(aura_id);
        game.apply_continuous_effects();
        assert_eq!(game.state.enhanced_mana_productions.len(), 0);
    }

    #[test]
    fn multiple_auras_on_same_land() {
        let (mut game, p1, _p2) = setup();
        let (forest_id, ability_id) = add_land(&mut game, p1, "Forest", Mana::green(1), true);
        let _ = add_aura_enhancer(&mut game, p1, forest_id, ManaColor::Red);
        let _ = add_aura_enhancer(&mut game, p1, forest_id, ManaColor::Blue);
        game.apply_continuous_effects();

        assert_eq!(game.state.enhanced_mana_productions.len(), 2);

        game.activate_mana_ability(p1, forest_id, ability_id);
        let available = game.state.players.get(&p1).unwrap().mana_pool.available();
        assert_eq!(available.green, 1, "base 1G");
        assert_eq!(available.red, 1, "bonus 1R from first aura");
        assert_eq!(available.blue, 1, "bonus 1U from second aura");
    }

    #[test]
    fn same_color_as_land_stacks() {
        let (mut game, p1, _p2) = setup();
        let (forest_id, ability_id) = add_land(&mut game, p1, "Forest", Mana::green(1), true);
        let _ = add_aura_enhancer(&mut game, p1, forest_id, ManaColor::Green);
        game.apply_continuous_effects();

        game.activate_mana_ability(p1, forest_id, ability_id);
        let available = game.state.players.get(&p1).unwrap().mana_pool.available();
        assert_eq!(available.green, 2, "1G base + 1G from aura = 2G total");
    }

    #[test]
    fn no_effect_without_aura() {
        let (mut game, p1, _p2) = setup();
        let (forest_id, ability_id) = add_land(&mut game, p1, "Forest", Mana::green(1), true);
        game.apply_continuous_effects();

        assert_eq!(game.state.enhanced_mana_productions.len(), 0);

        game.activate_mana_ability(p1, forest_id, ability_id);
        let available = game.state.players.get(&p1).unwrap().mana_pool.available();
        assert_eq!(available.green, 1, "no aura, only 1G");
    }

    #[test]
    fn helper_constructor() {
        match StaticEffect::enhanced_mana_production() {
            StaticEffect::EnhancedManaProduction => {}
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn choose_color_helper() {
        match Effect::choose_color() {
            Effect::ChooseColor => {}
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn mana_of_color_helper() {
        let m = Mana::of_color(ManaColor::Red, 1);
        assert_eq!(m.red, 1);
        assert_eq!(m.green, 0);
        let m2 = Mana::of_color(ManaColor::Blue, 3);
        assert_eq!(m2.blue, 3);
    }
}

mod trigger_doubling_tests {
    use super::*;
    use crate::events::GameEvent;

    fn add_elemental_with_etb(game: &mut Game, owner: PlayerId, name: &str, life_gain: u32) -> ObjectId {
        let id = ObjectId::new();
        let mut card = CardData::new(id, owner, name);
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![SubType::Elemental];
        card.power = Some(Power::new(2));
        card.toughness = Some(Toughness::new(2));
        let ability = Ability::enters_battlefield_triggered(
            id,
            &format!("When {name} enters, you gain {life_gain} life."),
            vec![Effect::GainLife { amount: life_gain }],
            TargetSpec::None,
        );
        card.abilities.push(ability.clone());
        game.state.card_store.insert(card.clone());
        game.state.ability_store.add(ability);
        let perm = Permanent::new(card, owner);
        game.state.battlefield.add(perm);
        game.state.set_zone(id, crate::constants::Zone::Battlefield, None);
        id
    }

    fn add_trigger_doubler(game: &mut Game, owner: PlayerId, filter: &str) -> ObjectId {
        let id = ObjectId::new();
        let mut card = CardData::new(id, owner, "Twinflame Travelers");
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![SubType::Elemental, SubType::Sorcerer];
        card.power = Some(Power::new(3));
        card.toughness = Some(Toughness::new(3));
        let ability = Ability::static_ability(
            id,
            "Triggered abilities of matching permanents trigger an additional time.",
            vec![StaticEffect::trigger_doubling(filter)],
        );
        card.abilities.push(ability.clone());
        game.state.card_store.insert(card.clone());
        game.state.ability_store.add(ability);
        let perm = Permanent::new(card, owner);
        game.state.battlefield.add(perm);
        id
    }

    #[test]
    fn matching_elemental_etb_triggers_twice() {
        let (mut game, p1, _p2) = setup();
        let _ = add_trigger_doubler(&mut game, p1, "other Elementals you control");
        game.apply_continuous_effects();
        assert_eq!(game.state.trigger_doublings.len(), 1);

        let elem_id = add_elemental_with_etb(&mut game, p1, "Fire Elemental", 3);
        game.emit_event(GameEvent::enters_battlefield(elem_id, p1));
        game.process_sba_and_triggers();

        assert_eq!(game.state.stack.len(), 2, "ETB should be doubled");

        game.resolve_top_of_stack();
        game.resolve_top_of_stack();
        assert_eq!(game.state.players[&p1].life, 26, "3 life x2 = 6 gained");
    }

    #[test]
    fn non_matching_type_not_doubled() {
        let (mut game, p1, _p2) = setup();
        let _ = add_trigger_doubler(&mut game, p1, "other Elementals you control");
        game.apply_continuous_effects();

        let id = ObjectId::new();
        let mut card = CardData::new(id, owner_placeholder(p1), "Goblin Raider");
        card.card_types = vec![CardType::Creature];
        card.subtypes = vec![SubType::Goblin];
        card.power = Some(Power::new(2));
        card.toughness = Some(Toughness::new(2));
        let ability = Ability::enters_battlefield_triggered(
            id, "When this enters, gain 3 life.",
            vec![Effect::GainLife { amount: 3 }],
            TargetSpec::None,
        );
        card.abilities.push(ability.clone());
        game.state.card_store.insert(card.clone());
        game.state.ability_store.add(ability);
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);
        game.state.set_zone(id, crate::constants::Zone::Battlefield, None);
        game.emit_event(GameEvent::enters_battlefield(id, p1));
        game.process_sba_and_triggers();

        assert_eq!(game.state.stack.len(), 1, "non-Elemental not doubled");
    }

    #[test]
    fn doubler_self_excluded_by_other_filter() {
        let (mut game, p1, _p2) = setup();
        let doubler_id = add_trigger_doubler(&mut game, p1, "other Elementals you control");
        game.apply_continuous_effects();

        let ability = Ability::enters_battlefield_triggered(
            doubler_id, "When this enters, gain 2 life.",
            vec![Effect::GainLife { amount: 2 }],
            TargetSpec::None,
        );
        game.state.ability_store.add(ability);
        game.emit_event(GameEvent::enters_battlefield(doubler_id, p1));
        game.process_sba_and_triggers();

        assert_eq!(game.state.stack.len(), 1, "doubler's own trigger not doubled");
    }

    #[test]
    fn opponent_elemental_not_doubled() {
        let (mut game, p1, p2) = setup();
        let _ = add_trigger_doubler(&mut game, p1, "other Elementals you control");
        game.apply_continuous_effects();

        let elem_id = add_elemental_with_etb(&mut game, p2, "Opp Elemental", 3);
        game.emit_event(GameEvent::enters_battlefield(elem_id, p2));
        game.process_sba_and_triggers();

        assert_eq!(game.state.stack.len(), 1, "opponent's Elemental not doubled");
    }

    #[test]
    fn removal_reverts() {
        let (mut game, p1, _p2) = setup();
        let doubler_id = add_trigger_doubler(&mut game, p1, "other Elementals you control");
        game.apply_continuous_effects();
        assert_eq!(game.state.trigger_doublings.len(), 1);

        game.state.battlefield.remove(doubler_id);
        game.state.ability_store.remove_source(doubler_id);
        game.apply_continuous_effects();
        assert_eq!(game.state.trigger_doublings.len(), 0);

        let elem_id = add_elemental_with_etb(&mut game, p1, "Late Elemental", 3);
        game.emit_event(GameEvent::enters_battlefield(elem_id, p1));
        game.process_sba_and_triggers();

        assert_eq!(game.state.stack.len(), 1, "no doubling after removal");
    }

    #[test]
    fn multiple_doublers_stack() {
        let (mut game, p1, _p2) = setup();
        let _ = add_trigger_doubler(&mut game, p1, "other Elementals you control");
        let _ = add_trigger_doubler(&mut game, p1, "other Elementals you control");
        game.apply_continuous_effects();
        assert_eq!(game.state.trigger_doublings.len(), 2);

        let elem_id = add_elemental_with_etb(&mut game, p1, "Double Fire", 3);
        game.emit_event(GameEvent::enters_battlefield(elem_id, p1));
        game.process_sba_and_triggers();

        assert_eq!(game.state.stack.len(), 3, "1 original + 2 extra = 3 triggers");
    }

    #[test]
    fn changeling_matches_filter() {
        let (mut game, p1, _p2) = setup();
        let _ = add_trigger_doubler(&mut game, p1, "other Elementals you control");
        game.apply_continuous_effects();

        let id = ObjectId::new();
        let mut card = CardData::new(id, p1, "Changeling Sentinel");
        card.card_types = vec![CardType::Creature];
        card.power = Some(Power::new(1));
        card.toughness = Some(Toughness::new(1));
        card.keywords = KeywordAbilities::CHANGELING;
        let ability = Ability::enters_battlefield_triggered(
            id, "When this enters, gain 2 life.",
            vec![Effect::GainLife { amount: 2 }],
            TargetSpec::None,
        );
        card.abilities.push(ability.clone());
        game.state.card_store.insert(card.clone());
        game.state.ability_store.add(ability);
        let perm = Permanent::new(card, p1);
        game.state.battlefield.add(perm);
        game.state.set_zone(id, crate::constants::Zone::Battlefield, None);
        game.emit_event(GameEvent::enters_battlefield(id, p1));
        game.process_sba_and_triggers();

        assert_eq!(game.state.stack.len(), 2, "changeling matches Elemental filter");
    }

    #[test]
    fn no_doubling_without_doubler() {
        let (mut game, p1, _p2) = setup();
        game.apply_continuous_effects();
        assert_eq!(game.state.trigger_doublings.len(), 0);

        let elem_id = add_elemental_with_etb(&mut game, p1, "Solo Elemental", 3);
        game.emit_event(GameEvent::enters_battlefield(elem_id, p1));
        game.process_sba_and_triggers();

        assert_eq!(game.state.stack.len(), 1, "no doubling without doubler");
    }

    #[test]
    fn helper_constructor() {
        match StaticEffect::trigger_doubling("other Elementals you control") {
            StaticEffect::TriggerDoubling { filter } => {
                assert_eq!(filter, "other Elementals you control");
            }
            _ => panic!("wrong variant"),
        }
    }

    fn owner_placeholder(p: PlayerId) -> PlayerId { p }

    #[test]
    fn add_subtype_all_opponents_creatures() {
        let (mut game, p1, p2) = setup_game2();

        let bear1 = ObjectId::new();
        let mut bear_card1 = CardData::new(bear1, p2, "Grizzly Bears");
        bear_card1.card_types = vec![CardType::Creature];
        bear_card1.subtypes = vec![SubType::Bear];
        bear_card1.power = Some(Power::new(2));
        bear_card1.toughness = Some(Toughness::new(2));
        game.state.battlefield.add(Permanent::new(bear_card1, p2));

        let bear2 = ObjectId::new();
        let mut bear_card2 = CardData::new(bear2, p2, "Another Bear");
        bear_card2.card_types = vec![CardType::Creature];
        bear_card2.subtypes = vec![SubType::Bear];
        bear_card2.power = Some(Power::new(3));
        bear_card2.toughness = Some(Toughness::new(3));
        game.state.battlefield.add(Permanent::new(bear_card2, p2));

        let own_id = ObjectId::new();
        let mut own = CardData::new(own_id, p1, "Own Creature");
        own.card_types = vec![CardType::Creature];
        own.subtypes = vec![SubType::Human];
        own.power = Some(Power::new(1));
        own.toughness = Some(Toughness::new(1));
        game.state.battlefield.add(Permanent::new(own, p1));

        game.execute_effects(
            &[Effect::add_subtype_all("Coward", "creatures opponents control")],
            p1, &[], None, None,
        );

        let b1 = game.state.battlefield.get(bear1).unwrap();
        assert!(b1.has_subtype(&SubType::Coward));
        assert!(b1.has_subtype(&SubType::Bear));

        let b2 = game.state.battlefield.get(bear2).unwrap();
        assert!(b2.has_subtype(&SubType::Coward));
        assert!(b2.has_subtype(&SubType::Bear));

        let mine = game.state.battlefield.get(own_id).unwrap();
        assert!(!mine.has_subtype(&SubType::Coward));
    }
}

mod boost_per_turn_event_tests {
    use super::*;
    use crate::events::GameEvent;

    struct PassPlayer2;
    impl crate::decision::PlayerDecisionMaker for PassPlayer2 {
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

    fn make_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig { players: vec![
            PlayerConfig { name: "P1".into(), deck: vec![] },
            PlayerConfig { name: "P2".into(), deck: vec![] },
        ], starting_life: Life::new(20) };
        let game = Game::new_two_player(config, vec![
            (p1, PlayerAgent::new(PassPlayer2)),
            (p2, PlayerAgent::new(PassPlayer2)),
        ]);
        (game, p1, p2)
    }

    #[test]
    fn boost_per_creatures_entered_no_events() {
        let (mut game, p1, _p2) = make_game();

        let ench_id = ObjectId::new();
        let ench = CardData {
            id: ench_id, owner: p1, name: "Kinbinding".into(),
            card_types: vec![CardType::Enchantment],
            abilities: vec![Ability::static_ability(ench_id,
                "Creatures you control get +X/+X where X = creatures entered.",
                vec![StaticEffect::boost_per_turn_event("creatures you control", "creatures_entered", 1, 1)])],
            ..Default::default()
        };
        game.state.battlefield.add(Permanent::new(ench.clone(), p1));
        game.state.card_store.insert(ench.clone());
        for ab in &ench.abilities { game.state.ability_store.add(ab.clone()); }

        let bear_id = ObjectId::new();
        let mut bear = CardData::new(bear_id, p1, "Bear");
        bear.card_types = vec![CardType::Creature];
        bear.power = Some(Power::new(2));
        bear.toughness = Some(Toughness::new(2));
        game.state.battlefield.add(Permanent::new(bear, p1));

        game.apply_continuous_effects();
        let perm = game.state.battlefield.get(bear_id).unwrap();
        assert_eq!(perm.power(), 2);
        assert_eq!(perm.toughness(), 2);
    }

    #[test]
    fn boost_per_creatures_entered_scales_with_count() {
        let (mut game, p1, _p2) = make_game();

        let ench_id = ObjectId::new();
        let ench = CardData {
            id: ench_id, owner: p1, name: "Kinbinding".into(),
            card_types: vec![CardType::Enchantment],
            abilities: vec![Ability::static_ability(ench_id,
                "Creatures you control get +X/+X where X = creatures entered.",
                vec![StaticEffect::boost_per_turn_event("creatures you control", "creatures_entered", 1, 1)])],
            ..Default::default()
        };
        game.state.battlefield.add(Permanent::new(ench.clone(), p1));
        game.state.card_store.insert(ench.clone());
        for ab in &ench.abilities { game.state.ability_store.add(ab.clone()); }

        let bear_id = ObjectId::new();
        let mut bear = CardData::new(bear_id, p1, "Bear");
        bear.card_types = vec![CardType::Creature];
        bear.power = Some(Power::new(2));
        bear.toughness = Some(Toughness::new(2));
        game.state.battlefield.add(Permanent::new(bear, p1));

        game.emit_event(GameEvent::enters_battlefield(ObjectId::new(), p1));
        game.emit_event(GameEvent::enters_battlefield(ObjectId::new(), p1));
        game.emit_event(GameEvent::enters_battlefield(ObjectId::new(), p1));

        game.apply_continuous_effects();
        let perm = game.state.battlefield.get(bear_id).unwrap();
        assert_eq!(perm.power(), 5, "2 base + 3 creatures entered");
        assert_eq!(perm.toughness(), 5);
    }

    #[test]
    fn boost_per_creatures_entered_only_your_creatures() {
        let (mut game, p1, p2) = make_game();

        let ench_id = ObjectId::new();
        let ench = CardData {
            id: ench_id, owner: p1, name: "Kinbinding".into(),
            card_types: vec![CardType::Enchantment],
            abilities: vec![Ability::static_ability(ench_id,
                "Creatures you control get +X/+X where X = creatures entered.",
                vec![StaticEffect::boost_per_turn_event("creatures you control", "creatures_entered", 1, 1)])],
            ..Default::default()
        };
        game.state.battlefield.add(Permanent::new(ench.clone(), p1));
        game.state.card_store.insert(ench.clone());
        for ab in &ench.abilities { game.state.ability_store.add(ab.clone()); }

        let bear_id = ObjectId::new();
        let mut bear = CardData::new(bear_id, p1, "Bear");
        bear.card_types = vec![CardType::Creature];
        bear.power = Some(Power::new(2));
        bear.toughness = Some(Toughness::new(2));
        game.state.battlefield.add(Permanent::new(bear, p1));

        let opp_bear_id = ObjectId::new();
        let mut opp_bear = CardData::new(opp_bear_id, p2, "Opp Bear");
        opp_bear.card_types = vec![CardType::Creature];
        opp_bear.power = Some(Power::new(3));
        opp_bear.toughness = Some(Toughness::new(3));
        game.state.battlefield.add(Permanent::new(opp_bear, p2));

        game.emit_event(GameEvent::enters_battlefield(ObjectId::new(), p1));
        game.emit_event(GameEvent::enters_battlefield(ObjectId::new(), p2));

        game.apply_continuous_effects();
        let own = game.state.battlefield.get(bear_id).unwrap();
        assert_eq!(own.power(), 3, "2 base + 1 own creature entered");
        assert_eq!(own.toughness(), 3);

        let opp = game.state.battlefield.get(opp_bear_id).unwrap();
        assert_eq!(opp.power(), 3, "opponent creature unaffected by your enchantment");
        assert_eq!(opp.toughness(), 3);
    }
}

mod becomes_creature_attached_tests {
    use super::*;

    struct PassPlayer3;
    impl crate::decision::PlayerDecisionMaker for PassPlayer3 {
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

    fn make_game() -> (Game, PlayerId, PlayerId) {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();
        let config = GameConfig { players: vec![
            PlayerConfig { name: "P1".into(), deck: vec![] },
            PlayerConfig { name: "P2".into(), deck: vec![] },
        ], starting_life: Life::new(20) };
        let game = Game::new_two_player(config, vec![(p1, PlayerAgent::new(PassPlayer3)), (p2, PlayerAgent::new(PassPlayer3))]);
        (game, p1, p2)
    }

    #[test]
    fn becomes_creature_attached_overrides_subtypes_and_color() {
        let (mut game, p1, _p2) = make_game();

        let creature_id = ObjectId::new();
        let mut creature = CardData::new(creature_id, p1, "Tarmogoyf");
        creature.card_types = vec![CardType::Creature];
        creature.subtypes = vec![SubType::Elemental];
        creature.power = Some(Power::new(4));
        creature.toughness = Some(Toughness::new(5));
        creature.color_identity = vec![Color::Green];
        game.state.battlefield.add(Permanent::new(creature, p1));

        let aura_id = ObjectId::new();
        let aura = CardData {
            id: aura_id, owner: p1, name: "Noggle the Mind".into(),
            card_types: vec![CardType::Enchantment],
            subtypes: vec![SubType::Aura],
            abilities: vec![Ability::static_ability(aura_id,
                "Enchanted creature loses all abilities and is a colorless Noggle with base P/T 1/1.",
                vec![StaticEffect::lose_all_abilities("enchanted creature"),
                     StaticEffect::set_base_pt("enchanted creature", 1, 1),
                     StaticEffect::becomes_creature_attached(&["Noggle"], true)])],
            ..Default::default()
        };
        let mut aura_perm = Permanent::new(aura.clone(), p1);
        aura_perm.attach_to(creature_id);
        game.state.battlefield.add(aura_perm);
        game.state.card_store.insert(aura.clone());
        for ab in &aura.abilities { game.state.ability_store.add(ab.clone()); }
        if let Some(target) = game.state.battlefield.get_mut(creature_id) {
            target.add_attachment(aura_id);
        }

        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert_eq!(perm.power(), 1, "base P/T should be 1/1");
        assert_eq!(perm.toughness(), 1);
        assert!(perm.has_subtype(&SubType::Noggle), "should be a Noggle");
        assert!(!perm.has_subtype(&SubType::Elemental), "should not be a Lhurgoyf anymore");
        assert!(perm.colorless_override, "should be colorless");
        assert!(perm.abilities_lost, "should have lost all abilities");
    }

    #[test]
    fn becomes_creature_attached_removed_when_aura_leaves() {
        let (mut game, p1, _p2) = make_game();

        let creature_id = ObjectId::new();
        let mut creature = CardData::new(creature_id, p1, "Grizzly Bears");
        creature.card_types = vec![CardType::Creature];
        creature.subtypes = vec![SubType::Bear];
        creature.power = Some(Power::new(2));
        creature.toughness = Some(Toughness::new(2));
        creature.color_identity = vec![Color::Green];
        game.state.battlefield.add(Permanent::new(creature, p1));

        let aura_id = ObjectId::new();
        let aura = CardData {
            id: aura_id, owner: p1, name: "Noggle the Mind".into(),
            card_types: vec![CardType::Enchantment],
            subtypes: vec![SubType::Aura],
            abilities: vec![Ability::static_ability(aura_id,
                "Enchanted creature is a colorless Noggle 1/1.",
                vec![StaticEffect::lose_all_abilities("enchanted creature"),
                     StaticEffect::set_base_pt("enchanted creature", 1, 1),
                     StaticEffect::becomes_creature_attached(&["Noggle"], true)])],
            ..Default::default()
        };
        let mut aura_perm = Permanent::new(aura.clone(), p1);
        aura_perm.attach_to(creature_id);
        game.state.battlefield.add(aura_perm);
        game.state.card_store.insert(aura.clone());
        for ab in &aura.abilities { game.state.ability_store.add(ab.clone()); }
        if let Some(target) = game.state.battlefield.get_mut(creature_id) {
            target.add_attachment(aura_id);
        }

        game.apply_continuous_effects();
        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(perm.has_subtype(&SubType::Noggle));
        assert!(!perm.has_subtype(&SubType::Bear));

        game.state.battlefield.remove(aura_id);
        if let Some(target) = game.state.battlefield.get_mut(creature_id) {
            target.remove_attachment(aura_id);
        }
        game.apply_continuous_effects();

        let perm = game.state.battlefield.get(creature_id).unwrap();
        assert!(!perm.has_subtype(&SubType::Noggle), "should no longer be Noggle after aura removed");
        assert!(perm.has_subtype(&SubType::Bear), "should be Bear again");
        assert!(!perm.colorless_override, "should not be colorless anymore");
        assert_eq!(perm.power(), 2, "should be back to 2/2");
        assert_eq!(perm.toughness(), 2);
    }

    #[test]
    fn colorless_override_affects_color_count() {
        let (mut game, p1, _p2) = make_game();

        let creature_id = ObjectId::new();
        let mut creature = CardData::new(creature_id, p1, "Blue Creature");
        creature.card_types = vec![CardType::Creature];
        creature.power = Some(Power::new(3));
        creature.toughness = Some(Toughness::new(3));
        creature.color_identity = vec![Color::Blue];
        game.state.battlefield.add(Permanent::new(creature, p1));

        assert_eq!(game.count_colors_among_permanents(p1), 1, "one blue permanent");

        let aura_id = ObjectId::new();
        let aura = CardData {
            id: aura_id, owner: p1, name: "Colorless Aura".into(),
            card_types: vec![CardType::Enchantment],
            subtypes: vec![SubType::Aura],
            abilities: vec![Ability::static_ability(aura_id,
                "Enchanted creature is colorless.",
                vec![StaticEffect::becomes_creature_attached(&[], true)])],
            ..Default::default()
        };
        let mut aura_perm = Permanent::new(aura.clone(), p1);
        aura_perm.attach_to(creature_id);
        game.state.battlefield.add(aura_perm);
        game.state.card_store.insert(aura.clone());
        for ab in &aura.abilities { game.state.ability_store.add(ab.clone()); }
        if let Some(target) = game.state.battlefield.get_mut(creature_id) {
            target.add_attachment(aura_id);
        }

        game.apply_continuous_effects();
        assert_eq!(game.count_colors_among_permanents(p1), 0, "creature is now colorless");
    }
}

