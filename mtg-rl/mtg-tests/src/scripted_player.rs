// ScriptedPlayer — a decision maker that follows pre-programmed actions.
//
// This is the Rust equivalent of Java's TestPlayer. It executes queued
// actions at specific turns/steps, enabling deterministic, reproducible tests.

use mtg_engine::constants::{Outcome, PhaseStep};
use mtg_engine::decision::{
    AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
    PlayerDecisionMaker, ReplacementEffectChoice, TargetRequirement, UnpaidMana,
};
use mtg_engine::types::{ObjectId, PlayerId};
use std::collections::VecDeque;

/// A single scripted action to be taken at a specific turn and step.
#[derive(Clone, Debug)]
pub struct ScriptedAction {
    /// Turn number (1-based).
    pub turn: u32,
    /// Step within the turn.
    pub step: PhaseStep,
    /// The action to take.
    pub action: ScriptedActionKind,
}

/// The kind of scripted action.
#[derive(Clone, Debug)]
pub enum ScriptedActionKind {
    /// Cast a spell by name, optionally targeting something by name.
    CastSpell {
        card_name: String,
        target_name: Option<String>,
    },
    /// Play a land by name.
    PlayLand {
        card_name: String,
    },
    /// Activate an ability by description text.
    ActivateAbility {
        ability_text: String,
        target_name: Option<String>,
    },
    /// Attack with a named creature.
    Attack {
        creature_name: String,
    },
    /// Block a named attacker with a named blocker.
    Block {
        blocker_name: String,
        attacker_name: String,
    },
    /// Make a choice (yes/no or named choice).
    SetChoice {
        choice: String,
    },
}

/// A player that follows a script of pre-programmed actions.
///
/// Actions are queued in advance and executed when the game reaches
/// the specified turn and step. If no action is queued for a given
/// priority check, the player passes.
pub struct ScriptedPlayer {
    /// The player's ID (set when the game starts).
    pub player_id: Option<PlayerId>,
    /// Queued actions.
    actions: VecDeque<ScriptedAction>,
    /// Current turn/step tracking (updated by the game).
    current_turn: u32,
    current_step: PhaseStep,
    /// Queued choices for yes/no and named choice prompts.
    choices: VecDeque<String>,
    /// Card name → ObjectId mapping (built during game setup).
    pub card_names: std::collections::HashMap<String, Vec<ObjectId>>,
}

impl ScriptedPlayer {
    pub fn new() -> Self {
        ScriptedPlayer {
            player_id: None,
            actions: VecDeque::new(),
            current_turn: 1,
            current_step: PhaseStep::Untap,
            choices: VecDeque::new(),
            card_names: std::collections::HashMap::new(),
        }
    }

    /// Queue an action.
    pub fn add_action(&mut self, action: ScriptedAction) {
        self.actions.push_back(action);
    }

    /// Queue a choice response.
    pub fn add_choice(&mut self, choice: &str) {
        self.choices.push_back(choice.to_string());
    }

    /// Register a card name → ObjectId mapping.
    pub fn register_card(&mut self, name: &str, id: ObjectId) {
        self.card_names
            .entry(name.to_string())
            .or_default()
            .push(id);
    }

    /// Find a card ID by name.
    pub fn find_card(&self, name: &str) -> Option<ObjectId> {
        self.card_names.get(name).and_then(|ids| ids.first().copied())
    }

    /// Find the next action that matches the current game state.
    fn next_matching_action(&mut self, legal_actions: &[PlayerAction]) -> Option<PlayerAction> {
        // Find the first queued action for the current turn/step
        let idx = self.actions.iter().position(|a| {
            a.turn == self.current_turn && a.step == self.current_step
        });

        let scripted = match idx {
            Some(i) => self.actions.remove(i).unwrap(),
            None => return None,
        };

        match &scripted.action {
            ScriptedActionKind::CastSpell { card_name, target_name: _ } => {
                // Find the CastSpell action for this card
                let card_id = self.find_card(card_name)?;
                legal_actions.iter().find(|a| {
                    matches!(a, PlayerAction::CastSpell { card_id: id, .. } if *id == card_id)
                }).cloned()
            }
            ScriptedActionKind::PlayLand { card_name } => {
                let card_id = self.find_card(card_name)?;
                legal_actions.iter().find(|a| {
                    matches!(a, PlayerAction::PlayLand { card_id: id } if *id == card_id)
                }).cloned()
            }
            ScriptedActionKind::ActivateAbility { .. } => {
                // Find an activate ability action (simplified: pick the first one)
                legal_actions.iter().find(|a| {
                    matches!(a, PlayerAction::ActivateAbility { .. })
                }).cloned()
            }
            _ => None,
        }
    }
}

impl Default for ScriptedPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayerDecisionMaker for ScriptedPlayer {
    fn priority(&mut self, _game: &GameView<'_>, legal_actions: &[PlayerAction]) -> PlayerAction {
        if let Some(action) = self.next_matching_action(legal_actions) {
            return action;
        }
        PlayerAction::Pass
    }

    fn choose_targets(
        &mut self,
        _game: &GameView<'_>,
        _outcome: Outcome,
        requirement: &TargetRequirement,
    ) -> Vec<ObjectId> {
        // Return the first legal target
        if requirement.legal_targets.is_empty() {
            return vec![];
        }
        vec![requirement.legal_targets[0]]
    }

    fn choose_use(&mut self, _game: &GameView<'_>, _outcome: Outcome, _message: &str) -> bool {
        if let Some(choice) = self.choices.pop_front() {
            choice.eq_ignore_ascii_case("yes")
        } else {
            false
        }
    }

    fn choose_mode(&mut self, _game: &GameView<'_>, _modes: &[NamedChoice]) -> usize {
        if let Some(choice) = self.choices.pop_front() {
            if let Ok(idx) = choice.parse::<usize>() {
                return idx.saturating_sub(1); // 1-based to 0-based
            }
        }
        0
    }

    fn select_attackers(
        &mut self,
        _game: &GameView<'_>,
        possible_attackers: &[ObjectId],
        possible_defenders: &[ObjectId],
    ) -> Vec<(ObjectId, ObjectId)> {
        // Find attack actions queued for this turn/step
        let mut attacks = Vec::new();
        let defender = possible_defenders.first().copied().unwrap_or(ObjectId::new());

        let mut i = 0;
        while i < self.actions.len() {
            if self.actions[i].turn == self.current_turn {
                if let ScriptedActionKind::Attack { creature_name } = &self.actions[i].action {
                    if let Some(attacker_id) = self.find_card(creature_name) {
                        if possible_attackers.contains(&attacker_id) {
                            attacks.push((attacker_id, defender));
                            self.actions.remove(i);
                            continue;
                        }
                    }
                }
            }
            i += 1;
        }
        attacks
    }

    fn select_blockers(
        &mut self,
        _game: &GameView<'_>,
        _attackers: &[AttackerInfo],
    ) -> Vec<(ObjectId, ObjectId)> {
        let mut blocks = Vec::new();

        let mut i = 0;
        while i < self.actions.len() {
            if self.actions[i].turn == self.current_turn {
                if let ScriptedActionKind::Block { blocker_name, attacker_name } = &self.actions[i].action {
                    if let (Some(blocker_id), Some(attacker_id)) =
                        (self.find_card(blocker_name), self.find_card(attacker_name))
                    {
                        blocks.push((blocker_id, attacker_id));
                        self.actions.remove(i);
                        continue;
                    }
                }
            }
            i += 1;
        }
        blocks
    }

    fn assign_damage(
        &mut self,
        _game: &GameView<'_>,
        assignment: &DamageAssignment,
    ) -> Vec<(ObjectId, u32)> {
        // Default: assign all damage to the first target
        if assignment.targets.is_empty() {
            return vec![];
        }
        vec![(assignment.targets[0], assignment.total_damage)]
    }

    fn choose_mulligan(&mut self, _game: &GameView<'_>, _hand: &[ObjectId]) -> bool {
        false // Never mulligan in tests by default
    }

    fn choose_cards_to_put_back(
        &mut self,
        _game: &GameView<'_>,
        hand: &[ObjectId],
        count: usize,
    ) -> Vec<ObjectId> {
        hand.iter().rev().take(count).copied().collect()
    }

    fn choose_discard(
        &mut self,
        _game: &GameView<'_>,
        hand: &[ObjectId],
        count: usize,
    ) -> Vec<ObjectId> {
        hand.iter().rev().take(count).copied().collect()
    }

    fn choose_amount(&mut self, _game: &GameView<'_>, _message: &str, min: u32, _max: u32) -> u32 {
        min
    }

    fn choose_mana_payment(
        &mut self,
        _game: &GameView<'_>,
        _unpaid: &UnpaidMana,
        mana_abilities: &[PlayerAction],
    ) -> Option<PlayerAction> {
        mana_abilities.first().cloned()
    }

    fn choose_replacement_effect(
        &mut self,
        _game: &GameView<'_>,
        _effects: &[ReplacementEffectChoice],
    ) -> usize {
        0
    }

    fn choose_pile(
        &mut self,
        _game: &GameView<'_>,
        _outcome: Outcome,
        _message: &str,
        _pile1: &[ObjectId],
        _pile2: &[ObjectId],
    ) -> bool {
        true
    }

    fn choose_option(
        &mut self,
        _game: &GameView<'_>,
        _outcome: Outcome,
        _message: &str,
        _options: &[NamedChoice],
    ) -> usize {
        0
    }

    fn on_game_start(&mut self, _game: &GameView<'_>, player_id: PlayerId) {
        self.player_id = Some(player_id);
    }
}
