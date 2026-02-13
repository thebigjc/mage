// Test framework — declarative game test builder.
//
// Mirrors the Java CardTestPlayerBase API for setting up game scenarios,
// executing them, and asserting results.

use crate::scripted_player::{ScriptedAction, ScriptedActionKind, ScriptedPlayer};
use mtg_engine::abilities::{Ability, Effect, TargetSpec};
use mtg_engine::card::CardData;
use mtg_engine::constants::{CardType, KeywordAbilities, PhaseStep, Zone};
use mtg_engine::counters::CounterType;
use mtg_engine::game::{Game, GameConfig, PlayerConfig};
use mtg_engine::mana::{Mana, ManaCost};
use mtg_engine::permanent::Permanent;
use mtg_engine::types::{ObjectId, PlayerId};

/// Which player in a two-player test.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    A,
    B,
}

/// A card to be placed in the game during setup.
#[derive(Clone, Debug)]
struct SetupCard {
    zone: Zone,
    player: Player,
    card: CardData,
}

/// The main test builder. Create a scenario, execute it, then assert results.
pub struct GameTest {
    /// Player IDs.
    player_a_id: PlayerId,
    player_b_id: PlayerId,
    /// Cards to place during setup.
    setup_cards: Vec<SetupCard>,
    /// Scripted actions for player A.
    actions_a: Vec<ScriptedAction>,
    /// Scripted actions for player B.
    actions_b: Vec<ScriptedAction>,
    /// Choices for player A.
    choices_a: Vec<String>,
    /// Choices for player B.
    choices_b: Vec<String>,
    /// When to stop the game.
    stop_turn: u32,
    stop_step: PhaseStep,
    /// Starting life total.
    starting_life: i32,
    /// The game (created on execute).
    game: Option<Game>,
    /// Card registry: name -> factory function for known cards.
    card_factories: std::collections::HashMap<String, CardFactory>,
}

/// Factory function for creating a card by name.
type CardFactory = Box<dyn Fn(ObjectId, PlayerId) -> CardData>;

impl GameTest {
    /// Create a new test with default settings.
    pub fn new() -> Self {
        let mut test = GameTest {
            player_a_id: PlayerId::new(),
            player_b_id: PlayerId::new(),
            setup_cards: Vec::new(),
            actions_a: Vec::new(),
            actions_b: Vec::new(),
            choices_a: Vec::new(),
            choices_b: Vec::new(),
            stop_turn: 1,
            stop_step: PhaseStep::Cleanup,
            starting_life: 20,
            game: None,
            card_factories: std::collections::HashMap::new(),
        };
        test.register_basic_cards();
        test
    }

    /// Get player ID.
    pub fn player_id(&self, player: Player) -> PlayerId {
        match player {
            Player::A => self.player_a_id,
            Player::B => self.player_b_id,
        }
    }

    /// Register known basic cards.
    fn register_basic_cards(&mut self) {
        // Basic lands
        for &(name, color) in &[
            ("Plains", "W"),
            ("Island", "U"),
            ("Swamp", "B"),
            ("Mountain", "R"),
            ("Forest", "G"),
        ] {
            let color_str = color.to_string();
            self.card_factories.insert(name.to_string(), Box::new(move |id, owner| {
                let mut card = CardData::new(id, owner, name);
                card.card_types = vec![CardType::Land];
                card.supertypes = vec![mtg_engine::constants::SuperType::Basic];
                // Add mana ability
                let mana = match color_str.as_str() {
                    "W" => Mana::white(1),
                    "U" => Mana::blue(1),
                    "B" => Mana::black(1),
                    "R" => Mana::red(1),
                    "G" => Mana::green(1),
                    _ => Mana::colorless(1),
                };
                let mana_ability = Ability::mana_ability(
                    id,
                    &format!("{{T}}: Add {{{}}}", color_str),
                    mana,
                );
                card.abilities.push(mana_ability);
                card
            }));
        }

        // Grizzly Bears (the canonical test creature)
        self.card_factories.insert("Grizzly Bears".to_string(), Box::new(|id, owner| {
            let mut card = CardData::new(id, owner, "Grizzly Bears");
            card.mana_cost = ManaCost::parse("{1}{G}");
            card.card_types = vec![CardType::Creature];
            card.subtypes = vec![mtg_engine::constants::SubType::Bear];
            card.power = Some(2);
            card.toughness = Some(2);
            card.keywords = KeywordAbilities::empty();
            card
        }));

        // Lightning Bolt
        self.card_factories.insert("Lightning Bolt".to_string(), Box::new(|id, owner| {
            let mut card = CardData::new(id, owner, "Lightning Bolt");
            card.mana_cost = ManaCost::parse("{R}");
            card.card_types = vec![CardType::Instant];
            card.abilities.push(Ability::spell(
                id,
                vec![Effect::DealDamage { amount: 3 }],
                TargetSpec::CreatureOrPlayer,
            ));
            card
        }));

        // Llanowar Elves
        self.card_factories.insert("Llanowar Elves".to_string(), Box::new(|id, owner| {
            let mut card = CardData::new(id, owner, "Llanowar Elves");
            card.mana_cost = ManaCost::parse("{G}");
            card.card_types = vec![CardType::Creature];
            card.subtypes = vec![mtg_engine::constants::SubType::Elf];
            card.power = Some(1);
            card.toughness = Some(1);
            card.keywords = KeywordAbilities::empty();
            let mana_ability = Ability::mana_ability(id, "{T}: Add {G}", Mana::green(1));
            card.abilities.push(mana_ability);
            card
        }));

        // Giant Growth
        self.card_factories.insert("Giant Growth".to_string(), Box::new(|id, owner| {
            let mut card = CardData::new(id, owner, "Giant Growth");
            card.mana_cost = ManaCost::parse("{G}");
            card.card_types = vec![CardType::Instant];
            card.abilities.push(Ability::spell(
                id,
                vec![Effect::BoostUntilEndOfTurn { power: 3, toughness: 3 }],
                TargetSpec::Creature,
            ));
            card
        }));

        // Shock
        self.card_factories.insert("Shock".to_string(), Box::new(|id, owner| {
            let mut card = CardData::new(id, owner, "Shock");
            card.mana_cost = ManaCost::parse("{R}");
            card.card_types = vec![CardType::Instant];
            card.abilities.push(Ability::spell(
                id,
                vec![Effect::DealDamage { amount: 2 }],
                TargetSpec::CreatureOrPlayer,
            ));
            card
        }));

        // Divination
        self.card_factories.insert("Divination".to_string(), Box::new(|id, owner| {
            let mut card = CardData::new(id, owner, "Divination");
            card.mana_cost = ManaCost::parse("{2}{U}");
            card.card_types = vec![CardType::Sorcery];
            card.abilities.push(Ability::spell(
                id,
                vec![Effect::DrawCards { count: 2 }],
                TargetSpec::None,
            ));
            card
        }));

        // Murder
        self.card_factories.insert("Murder".to_string(), Box::new(|id, owner| {
            let mut card = CardData::new(id, owner, "Murder");
            card.mana_cost = ManaCost::parse("{1}{B}{B}");
            card.card_types = vec![CardType::Instant];
            card.abilities.push(Ability::spell(
                id,
                vec![Effect::Destroy],
                TargetSpec::Creature,
            ));
            card
        }));

        // Unsummon (bounce)
        self.card_factories.insert("Unsummon".to_string(), Box::new(|id, owner| {
            let mut card = CardData::new(id, owner, "Unsummon");
            card.mana_cost = ManaCost::parse("{U}");
            card.card_types = vec![CardType::Instant];
            card.abilities.push(Ability::spell(
                id,
                vec![Effect::Bounce],
                TargetSpec::Creature,
            ));
            card
        }));

        // Healing Salve (gain 3 life)
        self.card_factories.insert("Healing Salve".to_string(), Box::new(|id, owner| {
            let mut card = CardData::new(id, owner, "Healing Salve");
            card.mana_cost = ManaCost::parse("{W}");
            card.card_types = vec![CardType::Instant];
            card.abilities.push(Ability::spell(
                id,
                vec![Effect::GainLife { amount: 3 }],
                TargetSpec::None,
            ));
            card
        }));

        // Hill Giant (3/3 for 3R)
        self.card_factories.insert("Hill Giant".to_string(), Box::new(|id, owner| {
            let mut card = CardData::new(id, owner, "Hill Giant");
            card.mana_cost = ManaCost::parse("{3}{R}");
            card.card_types = vec![CardType::Creature];
            card.subtypes = vec![mtg_engine::constants::SubType::Giant];
            card.power = Some(3);
            card.toughness = Some(3);
            card.keywords = KeywordAbilities::empty();
            card
        }));

        // Serra Angel (3/4 flying vigilance)
        self.card_factories.insert("Serra Angel".to_string(), Box::new(|id, owner| {
            let mut card = CardData::new(id, owner, "Serra Angel");
            card.mana_cost = ManaCost::parse("{3}{W}{W}");
            card.card_types = vec![CardType::Creature];
            card.subtypes = vec![mtg_engine::constants::SubType::Angel];
            card.power = Some(4);
            card.toughness = Some(4);
            card.keywords = KeywordAbilities::FLYING | KeywordAbilities::VIGILANCE;
            card
        }));
    }

    /// Register a custom card factory.
    pub fn register_card(&mut self, name: &str, factory: impl Fn(ObjectId, PlayerId) -> CardData + 'static) {
        self.card_factories.insert(name.to_string(), Box::new(factory));
    }

    // ── Setup methods ────────────────────────────────────────────────

    /// Add a card to the game setup. Equivalent to Java's addCard().
    pub fn add_card(&mut self, zone: Zone, player: Player, name: &str) {
        self.add_card_count(zone, player, name, 1);
    }

    /// Add multiple copies of a card.
    pub fn add_card_count(&mut self, zone: Zone, player: Player, name: &str, count: u32) {
        let owner = self.player_id(player);
        for _ in 0..count {
            let id = ObjectId::new();
            let card = if let Some(factory) = self.card_factories.get(name) {
                factory(id, owner)
            } else {
                // Unknown card: create a minimal placeholder
                let mut card = CardData::new(id, owner, name);
                card.keywords = KeywordAbilities::empty();
                card
            };
            self.setup_cards.push(SetupCard {
                zone,
                player,
                card,
            });
        }
    }

    // ── Action methods ───────────────────────────────────────────────

    /// Queue a spell cast action.
    pub fn cast_spell(
        &mut self,
        turn: u32,
        step: PhaseStep,
        player: Player,
        card_name: &str,
        target_name: Option<&str>,
    ) {
        let action = ScriptedAction {
            turn,
            step,
            action: ScriptedActionKind::CastSpell {
                card_name: card_name.to_string(),
                target_name: target_name.map(String::from),
            },
        };
        match player {
            Player::A => self.actions_a.push(action),
            Player::B => self.actions_b.push(action),
        }
    }

    /// Queue a land play action.
    pub fn play_land(&mut self, turn: u32, step: PhaseStep, player: Player, card_name: &str) {
        let action = ScriptedAction {
            turn,
            step,
            action: ScriptedActionKind::PlayLand {
                card_name: card_name.to_string(),
            },
        };
        match player {
            Player::A => self.actions_a.push(action),
            Player::B => self.actions_b.push(action),
        }
    }

    /// Queue an attack action.
    pub fn attack(&mut self, turn: u32, player: Player, creature_name: &str) {
        let action = ScriptedAction {
            turn,
            step: PhaseStep::DeclareAttackers,
            action: ScriptedActionKind::Attack {
                creature_name: creature_name.to_string(),
            },
        };
        match player {
            Player::A => self.actions_a.push(action),
            Player::B => self.actions_b.push(action),
        }
    }

    /// Queue a block action.
    pub fn block(&mut self, turn: u32, player: Player, blocker_name: &str, attacker_name: &str) {
        let action = ScriptedAction {
            turn,
            step: PhaseStep::DeclareBlockers,
            action: ScriptedActionKind::Block {
                blocker_name: blocker_name.to_string(),
                attacker_name: attacker_name.to_string(),
            },
        };
        match player {
            Player::A => self.actions_a.push(action),
            Player::B => self.actions_b.push(action),
        }
    }

    /// Queue a choice for a player.
    pub fn set_choice(&mut self, player: Player, choice: &str) {
        match player {
            Player::A => self.choices_a.push(choice.to_string()),
            Player::B => self.choices_b.push(choice.to_string()),
        }
    }

    /// Set when to stop execution.
    pub fn stop_at(&mut self, turn: u32, step: PhaseStep) {
        self.stop_turn = turn;
        self.stop_step = step;
    }

    /// Set starting life total.
    pub fn set_starting_life(&mut self, life: i32) {
        self.starting_life = life;
    }

    // ── Execute ──────────────────────────────────────────────────────

    /// Execute the test scenario.
    pub fn execute(&mut self) {
        let p1 = self.player_a_id;
        let p2 = self.player_b_id;

        // Build scripted players
        let mut player_a = ScriptedPlayer::new();
        let mut player_b = ScriptedPlayer::new();

        // Add choices
        for choice in &self.choices_a {
            player_a.add_choice(choice);
        }
        for choice in &self.choices_b {
            player_b.add_choice(choice);
        }

        // Add actions
        for action in &self.actions_a {
            player_a.add_action(action.clone());
        }
        for action in &self.actions_b {
            player_b.add_action(action.clone());
        }

        // Create minimal decks (just enough to not immediately deck out)
        let mut deck_a = Vec::new();
        let mut deck_b = Vec::new();
        for _ in 0..40 {
            let id = ObjectId::new();
            let mut card = CardData::new(id, p1, "Plains");
            card.card_types = vec![CardType::Land];
            card.supertypes = vec![mtg_engine::constants::SuperType::Basic];
            card.keywords = KeywordAbilities::empty();
            deck_a.push(card);
        }
        for _ in 0..40 {
            let id = ObjectId::new();
            let mut card = CardData::new(id, p2, "Plains");
            card.card_types = vec![CardType::Land];
            card.supertypes = vec![mtg_engine::constants::SuperType::Basic];
            card.keywords = KeywordAbilities::empty();
            deck_b.push(card);
        }

        let config = GameConfig {
            players: vec![
                PlayerConfig { name: "PlayerA".to_string(), deck: deck_a },
                PlayerConfig { name: "PlayerB".to_string(), deck: deck_b },
            ],
            starting_life: self.starting_life,
        };

        let mut game = Game::new_two_player(
            config,
            vec![
                (p1, Box::new(player_a)),
                (p2, Box::new(player_b)),
            ],
        );

        // Place setup cards into the game
        for setup in &self.setup_cards {
            let player_id = self.player_id(setup.player);
            let card = setup.card.clone();
            let card_id = card.id;

            // Insert into card store
            game.state.card_store.insert(card.clone());

            match setup.zone {
                Zone::Battlefield => {
                    // Register abilities
                    for ability in &card.abilities {
                        game.state.ability_store.add(ability.clone());
                    }
                    let mut perm = Permanent::new(card, player_id);
                    // Remove summoning sickness for test setup permanents
                    perm.remove_summoning_sickness();
                    game.state.battlefield.add(perm);
                    game.state.set_zone(card_id, Zone::Battlefield, None);
                }
                Zone::Hand => {
                    if let Some(player) = game.state.players.get_mut(&player_id) {
                        player.hand.add(card_id);
                    }
                    game.state.set_zone(card_id, Zone::Hand, Some(player_id));
                }
                Zone::Graveyard => {
                    if let Some(player) = game.state.players.get_mut(&player_id) {
                        player.graveyard.add(card_id);
                    }
                    game.state.set_zone(card_id, Zone::Graveyard, Some(player_id));
                }
                Zone::Library => {
                    if let Some(player) = game.state.players.get_mut(&player_id) {
                        player.library.put_on_top(card_id);
                    }
                    game.state.set_zone(card_id, Zone::Library, Some(player_id));
                }
                Zone::Exile => {
                    game.state.exile.exile(card_id);
                    game.state.set_zone(card_id, Zone::Exile, None);
                }
                _ => {}
            }
        }

        self.game = Some(game);

        // For now, we don't run the full game loop since it requires
        // the scripted player to be synchronized with the turn structure.
        // The test framework provides direct state access for assertions.
    }

    // ── Assertion methods ────────────────────────────────────────────

    /// Get a reference to the game (panics if not executed).
    pub fn game(&self) -> &Game {
        self.game.as_ref().expect("execute() must be called first")
    }

    /// Assert a player's life total.
    pub fn assert_life(&self, player: Player, expected: i32) {
        let pid = self.player_id(player);
        let actual = self.game().state.player(pid).unwrap().life;
        assert_eq!(
            actual, expected,
            "Player {:?} life: expected {}, got {}",
            player, expected, actual
        );
    }

    /// Assert the number of permanents a player controls with a given name.
    pub fn assert_permanent_count(&self, player: Player, name: &str, expected: usize) {
        let pid = self.player_id(player);
        let count = self.game().state.battlefield
            .controlled_by(pid)
            .filter(|p| p.name() == name)
            .count();
        assert_eq!(
            count, expected,
            "Player {:?} permanent count for '{}': expected {}, got {}",
            player, name, expected, count
        );
    }

    /// Assert the number of cards in a player's graveyard with a given name.
    pub fn assert_graveyard_count(&self, player: Player, name: &str, expected: usize) {
        let pid = self.player_id(player);
        let player_data = self.game().state.player(pid).unwrap();
        let count = player_data.graveyard.iter()
            .filter(|&&card_id| {
                self.game().state.card_store.get(card_id)
                    .map(|c| c.name == name)
                    .unwrap_or(false)
            })
            .count();
        assert_eq!(
            count, expected,
            "Player {:?} graveyard count for '{}': expected {}, got {}",
            player, name, expected, count
        );
    }

    /// Assert the total number of cards in a player's hand.
    pub fn assert_hand_count(&self, player: Player, expected: usize) {
        let pid = self.player_id(player);
        let actual = self.game().state.player(pid).unwrap().hand.len();
        assert_eq!(
            actual, expected,
            "Player {:?} hand count: expected {}, got {}",
            player, expected, actual
        );
    }

    /// Assert the total number of permanents on the battlefield.
    pub fn assert_battlefield_count(&self, expected: usize) {
        let actual = self.game().state.battlefield.len();
        assert_eq!(
            actual, expected,
            "Battlefield count: expected {}, got {}",
            expected, actual
        );
    }

    /// Assert a permanent's power and toughness.
    pub fn assert_power_toughness(
        &self,
        player: Player,
        name: &str,
        expected_power: i32,
        expected_toughness: i32,
    ) {
        let pid = self.player_id(player);
        let perm = self.game().state.battlefield
            .controlled_by(pid)
            .find(|p| p.name() == name)
            .unwrap_or_else(|| panic!("No permanent named '{}' controlled by {:?}", name, player));
        assert_eq!(
            perm.power(), expected_power,
            "'{}' power: expected {}, got {}",
            name, expected_power, perm.power()
        );
        assert_eq!(
            perm.toughness(), expected_toughness,
            "'{}' toughness: expected {}, got {}",
            name, expected_toughness, perm.toughness()
        );
    }

    /// Assert that a permanent is tapped.
    pub fn assert_tapped(&self, player: Player, name: &str, expected: bool) {
        let pid = self.player_id(player);
        let perm = self.game().state.battlefield
            .controlled_by(pid)
            .find(|p| p.name() == name)
            .unwrap_or_else(|| panic!("No permanent named '{}' controlled by {:?}", name, player));
        assert_eq!(
            perm.tapped, expected,
            "'{}' tapped: expected {}, got {}",
            name, expected, perm.tapped
        );
    }

    /// Assert counter count on a permanent.
    pub fn assert_counter_count(
        &self,
        player: Player,
        name: &str,
        counter_type: CounterType,
        expected: u32,
    ) {
        let pid = self.player_id(player);
        let perm = self.game().state.battlefield
            .controlled_by(pid)
            .find(|p| p.name() == name)
            .unwrap_or_else(|| panic!("No permanent named '{}' controlled by {:?}", name, player));
        let actual = perm.counters.get(&counter_type);
        assert_eq!(
            actual, expected,
            "'{}' {} counters: expected {}, got {}",
            name, counter_type, expected, actual
        );
    }

    /// Assert the number of cards in exile.
    pub fn assert_exile_count(&self, name: &str, expected: usize) {
        let count = self.game().state.exile.iter_all()
            .filter(|&&card_id| {
                self.game().state.card_store.get(card_id)
                    .map(|c| c.name == name)
                    .unwrap_or(false)
            })
            .count();
        assert_eq!(
            count, expected,
            "Exile count for '{}': expected {}, got {}",
            name, expected, count
        );
    }

    /// Assert the stack size.
    pub fn assert_stack_size(&self, expected: usize) {
        let actual = self.game().state.stack.len();
        assert_eq!(
            actual, expected,
            "Stack size: expected {}, got {}",
            expected, actual
        );
    }

    /// Assert whether the game is over.
    pub fn assert_game_over(&self, expected: bool) {
        let actual = self.game().state.should_end();
        assert_eq!(
            actual, expected,
            "Game over: expected {}, got {}",
            expected, actual
        );
    }
}

impl Default for GameTest {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests for the framework itself
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn framework_setup_battlefield() {
        let mut test = GameTest::new();
        test.add_card(Zone::Battlefield, Player::A, "Grizzly Bears");
        test.add_card(Zone::Battlefield, Player::B, "Grizzly Bears");
        test.execute();

        test.assert_permanent_count(Player::A, "Grizzly Bears", 1);
        test.assert_permanent_count(Player::B, "Grizzly Bears", 1);
        test.assert_power_toughness(Player::A, "Grizzly Bears", 2, 2);
    }

    #[test]
    fn framework_setup_hand() {
        let mut test = GameTest::new();
        test.add_card(Zone::Hand, Player::A, "Lightning Bolt");
        test.add_card(Zone::Hand, Player::A, "Lightning Bolt");
        test.execute();

        // The starting hand from the deck plus our 2 bolts
        let hand_size = test.game().state.player(test.player_id(Player::A)).unwrap().hand.len();
        assert!(hand_size >= 2); // At least the 2 bolts we added
    }

    #[test]
    fn framework_setup_graveyard() {
        let mut test = GameTest::new();
        test.add_card(Zone::Graveyard, Player::A, "Lightning Bolt");
        test.execute();

        test.assert_graveyard_count(Player::A, "Lightning Bolt", 1);
    }

    #[test]
    fn framework_life_totals() {
        let mut test = GameTest::new();
        test.execute();

        test.assert_life(Player::A, 20);
        test.assert_life(Player::B, 20);
    }

    #[test]
    fn framework_custom_starting_life() {
        let mut test = GameTest::new();
        test.set_starting_life(40);
        test.execute();

        test.assert_life(Player::A, 40);
        test.assert_life(Player::B, 40);
    }

    #[test]
    fn framework_multiple_permanents() {
        let mut test = GameTest::new();
        test.add_card_count(Zone::Battlefield, Player::A, "Forest", 3);
        test.add_card_count(Zone::Battlefield, Player::B, "Mountain", 2);
        test.execute();

        test.assert_permanent_count(Player::A, "Forest", 3);
        test.assert_permanent_count(Player::B, "Mountain", 2);
    }

    #[test]
    fn framework_direct_effect_execution() {
        // Test the game loop by directly executing effects
        let mut test = GameTest::new();
        test.add_card(Zone::Battlefield, Player::B, "Grizzly Bears");
        test.execute();

        // Directly execute a destroy effect on the bear
        let bear_id = test.game().state.battlefield
            .controlled_by(test.player_id(Player::B))
            .find(|p| p.name() == "Grizzly Bears")
            .unwrap()
            .id();

        let game = test.game.as_mut().unwrap();
        game.execute_effects(
            &[Effect::Destroy],
            test.player_a_id,
            &[bear_id],
        );
        game.process_state_based_actions();

        // Bear should be gone
        assert_eq!(game.state.battlefield.len(), 0);
    }

    #[test]
    fn framework_bolt_kills_bear() {
        // Simulate Lightning Bolt killing Grizzly Bears via direct effect execution
        let mut test = GameTest::new();
        test.add_card(Zone::Battlefield, Player::B, "Grizzly Bears");
        test.execute();

        let bear_id = test.game().state.battlefield
            .controlled_by(test.player_id(Player::B))
            .find(|p| p.name() == "Grizzly Bears")
            .unwrap()
            .id();

        let game = test.game.as_mut().unwrap();
        game.execute_effects(
            &[Effect::DealDamage { amount: 3 }],
            test.player_a_id,
            &[bear_id],
        );
        game.process_state_based_actions();

        assert!(!game.state.battlefield.contains(bear_id));
    }

    #[test]
    fn framework_exile_and_assert() {
        let mut test = GameTest::new();
        test.add_card(Zone::Battlefield, Player::A, "Grizzly Bears");
        test.execute();

        let bear_id = test.game().state.battlefield
            .controlled_by(test.player_id(Player::A))
            .find(|p| p.name() == "Grizzly Bears")
            .unwrap()
            .id();

        let game = test.game.as_mut().unwrap();
        game.execute_effects(
            &[Effect::Exile],
            test.player_b_id,
            &[bear_id],
        );

        test.assert_permanent_count(Player::A, "Grizzly Bears", 0);
        test.assert_exile_count("Grizzly Bears", 1);
    }

    #[test]
    fn framework_gain_life_effect() {
        let mut test = GameTest::new();
        test.execute();

        let game = test.game.as_mut().unwrap();
        game.execute_effects(
            &[Effect::GainLife { amount: 5 }],
            test.player_a_id,
            &[],
        );

        test.assert_life(Player::A, 25);
        test.assert_life(Player::B, 20);
    }

    #[test]
    fn framework_draw_cards_effect() {
        let mut test = GameTest::new();
        test.execute();

        let initial = test.game().state.player(test.player_id(Player::A)).unwrap().hand.len();

        let game = test.game.as_mut().unwrap();
        game.draw_cards(test.player_a_id, 3);

        let final_count = game.state.player(test.player_a_id).unwrap().hand.len();
        assert_eq!(final_count, initial + 3);
    }
}
