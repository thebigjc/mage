// Test modules for the game engine

pub mod game_basics;
pub mod effects;
pub mod combat;
pub mod abilities;
pub mod continuous_effects;
pub mod modal;
pub mod costs;
pub mod triggers;
pub mod equipment_auras;
pub mod keywords;
pub mod tokens;
pub mod special_mechanics;

// Common test utilities
use crate::abilities::{Ability, Cost, Effect, TargetSpec};
use crate::card::CardData;
use crate::constants::{CardType, KeywordAbilities, Outcome, SubType};
use crate::decision::{
    AttackerInfo, DamageAssignment, GameView, NamedChoice, PlayerAction,
    ReplacementEffectChoice, TargetRequirement, UnpaidMana, PlayerDecisionMaker,
};
use crate::mana::Mana;
use crate::types::{PlayerId, ObjectId};

/// A minimal decision maker that always passes priority.
pub struct AlwaysPassPlayer;

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

pub fn make_basic_land(name: &str, owner: PlayerId) -> CardData {
    let mut card = CardData::new(ObjectId::new(), owner, name);
    card.card_types = vec![CardType::Land];
    card
}

pub fn make_creature(name: &str, owner: PlayerId, power: i32, toughness: i32) -> CardData {
    let mut card = CardData::new(ObjectId::new(), owner, name);
    card.card_types = vec![CardType::Creature];
    card.power = Some(power);
    card.toughness = Some(toughness);
    card.keywords = KeywordAbilities::empty();
    card
}

pub fn make_deck(owner: PlayerId) -> Vec<CardData> {
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