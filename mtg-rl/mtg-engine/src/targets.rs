// Target definitions and validation.
//
// Targets are what spells and abilities select when cast/activated.
// This module defines target requirements, selection, and validation.
//
// Replaces Java's Target/TargetPermanent/TargetCreaturePermanent/etc.
// hierarchy with a data-oriented approach.

use crate::constants::{Outcome, Zone};
use crate::filters::{Filter, Predicate};
use crate::types::{ObjectId, PlayerId};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Target requirement — what kind of target an ability needs
// ---------------------------------------------------------------------------

/// Defines what an ability or spell targets.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TargetDefinition {
    /// What zone the target must be in.
    pub zone: Zone,
    /// Filter for valid targets.
    pub filter: Filter,
    /// Minimum number of targets that must be selected.
    pub min_targets: u32,
    /// Maximum number of targets that can be selected.
    pub max_targets: u32,
    /// Whether this is a "target" (affected by hexproof/shroud/protection)
    /// or a "choose" (not affected by those).
    pub is_targeted: bool,
    /// What outcome selecting this target represents (for AI evaluation).
    pub outcome: Outcome,
    /// Description for UI/display.
    pub description: String,
}

impl TargetDefinition {
    /// Create a new target definition.
    pub fn new(zone: Zone, filter: Filter, is_targeted: bool, outcome: Outcome) -> Self {
        TargetDefinition {
            description: filter.message.clone(),
            zone,
            filter,
            min_targets: 1,
            max_targets: 1,
            is_targeted,
            outcome,
        }
    }

    /// Create a "target creature" definition.
    pub fn target_creature() -> Self {
        Self::new(
            Zone::Battlefield,
            Filter::any_creature(),
            true,
            Outcome::Damage,
        )
    }

    /// Create a "target permanent" definition.
    pub fn target_permanent() -> Self {
        Self::new(
            Zone::Battlefield,
            Filter::any_permanent(),
            true,
            Outcome::Removal,
        )
    }

    /// Create a "target nonland permanent" definition.
    pub fn target_nonland_permanent() -> Self {
        Self::new(
            Zone::Battlefield,
            Filter::any_nonland_permanent(),
            true,
            Outcome::Removal,
        )
    }

    /// Create a "target player" definition.
    pub fn target_player() -> Self {
        Self::new(
            Zone::Outside, // players are not in a zone
            Filter::new("player", Predicate::All),
            true,
            Outcome::Damage,
        )
    }

    /// Create a "target creature or player" definition.
    pub fn target_creature_or_player() -> Self {
        let mut def = Self::new(
            Zone::Battlefield,
            Filter::new("creature or player", Predicate::creature().or(Predicate::All)),
            true,
            Outcome::Damage,
        );
        def.description = "target creature or player".to_string();
        def
    }

    /// Create a "choose a card in your graveyard" definition (not targeted).
    pub fn choose_card_in_graveyard(filter: Filter) -> Self {
        Self::new(Zone::Graveyard, filter, false, Outcome::ReturnToHand)
    }

    /// Set the number of targets.
    #[must_use]
    pub fn with_count(mut self, min: u32, max: u32) -> Self {
        self.min_targets = min;
        self.max_targets = max;
        self
    }

    /// "Up to N" targets (min = 0).
    #[must_use]
    pub fn up_to(mut self, max: u32) -> Self {
        self.min_targets = 0;
        self.max_targets = max;
        self
    }

    /// Set the description.
    #[must_use]
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    /// Set the outcome.
    #[must_use]
    pub fn with_outcome(mut self, outcome: Outcome) -> Self {
        self.outcome = outcome;
        self
    }

    /// Whether "up to" (min_targets == 0 so you can choose fewer).
    pub fn is_up_to(&self) -> bool {
        self.min_targets == 0
    }
}

// ---------------------------------------------------------------------------
// Selected targets — the actual chosen targets for a spell/ability
// ---------------------------------------------------------------------------

/// The selected targets for a spell or ability on the stack.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SelectedTargets {
    /// Ordered list of targets. Each entry has (target_id, is_player, amount).
    /// `amount` is used for divided effects (e.g. "deal 3 damage divided").
    entries: Vec<TargetEntry>,
}

/// A single selected target.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TargetEntry {
    /// The ID of the target (ObjectId for permanents/cards, reinterpreted for players).
    pub id: ObjectId,
    /// Whether this target is a player.
    pub is_player: bool,
    /// Player ID if this is a player target.
    pub player_id: Option<PlayerId>,
    /// Amount for divided effects.
    pub amount: u32,
    /// Whether this target is still legal (checked on resolution).
    pub legal: bool,
}

impl SelectedTargets {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a permanent/card target.
    pub fn add_object(&mut self, id: ObjectId) {
        self.entries.push(TargetEntry {
            id,
            is_player: false,
            player_id: None,
            amount: 0,
            legal: true,
        });
    }

    /// Add a permanent/card target with an amount (for divided effects).
    pub fn add_object_with_amount(&mut self, id: ObjectId, amount: u32) {
        self.entries.push(TargetEntry {
            id,
            is_player: false,
            player_id: None,
            amount,
            legal: true,
        });
    }

    /// Add a player target.
    pub fn add_player(&mut self, player_id: PlayerId) {
        self.entries.push(TargetEntry {
            id: ObjectId::new(), // placeholder; player_id is the real identifier
            is_player: true,
            player_id: Some(player_id),
            amount: 0,
            legal: true,
        });
    }

    /// Get the first target object ID.
    pub fn first_object(&self) -> Option<ObjectId> {
        self.entries
            .iter()
            .find(|e| !e.is_player && e.legal)
            .map(|e| e.id)
    }

    /// Get the first target player ID.
    pub fn first_player(&self) -> Option<PlayerId> {
        self.entries
            .iter()
            .find(|e| e.is_player && e.legal)
            .and_then(|e| e.player_id)
    }

    /// Get all legal object targets.
    pub fn objects(&self) -> Vec<ObjectId> {
        self.entries
            .iter()
            .filter(|e| !e.is_player && e.legal)
            .map(|e| e.id)
            .collect()
    }

    /// Get all legal player targets.
    pub fn players(&self) -> Vec<PlayerId> {
        self.entries
            .iter()
            .filter(|e| e.is_player && e.legal)
            .filter_map(|e| e.player_id)
            .collect()
    }

    /// Get all entries.
    pub fn entries(&self) -> &[TargetEntry] {
        &self.entries
    }

    /// Get mutable entries (for marking illegal targets).
    pub fn entries_mut(&mut self) -> &mut [TargetEntry] {
        &mut self.entries
    }

    /// Number of selected targets.
    pub fn count(&self) -> usize {
        self.entries.len()
    }

    /// Number of still-legal targets.
    pub fn legal_count(&self) -> usize {
        self.entries.iter().filter(|e| e.legal).count()
    }

    /// Whether any targets were selected.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Whether all targets are still legal.
    pub fn all_legal(&self) -> bool {
        self.entries.iter().all(|e| e.legal)
    }

    /// Whether any targets are still legal.
    pub fn any_legal(&self) -> bool {
        self.entries.iter().any(|e| e.legal)
    }

    /// Mark a target as illegal.
    pub fn mark_illegal(&mut self, id: ObjectId) {
        for entry in &mut self.entries {
            if entry.id == id {
                entry.legal = false;
            }
        }
    }

    /// Mark a player target as illegal.
    pub fn mark_player_illegal(&mut self, player_id: PlayerId) {
        for entry in &mut self.entries {
            if entry.player_id == Some(player_id) {
                entry.legal = false;
            }
        }
    }

    /// Check if a specific object is targeted.
    pub fn contains_object(&self, id: ObjectId) -> bool {
        self.entries.iter().any(|e| !e.is_player && e.id == id && e.legal)
    }

    /// Check if a specific player is targeted.
    pub fn contains_player(&self, player_id: PlayerId) -> bool {
        self.entries
            .iter()
            .any(|e| e.is_player && e.player_id == Some(player_id) && e.legal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_definition_basics() {
        let def = TargetDefinition::target_creature();
        assert_eq!(def.zone, Zone::Battlefield);
        assert!(def.is_targeted);
        assert_eq!(def.min_targets, 1);
        assert_eq!(def.max_targets, 1);
    }

    #[test]
    fn target_definition_up_to() {
        let def = TargetDefinition::target_creature().up_to(3);
        assert_eq!(def.min_targets, 0);
        assert_eq!(def.max_targets, 3);
        assert!(def.is_up_to());
    }

    #[test]
    fn selected_targets_objects() {
        let mut targets = SelectedTargets::new();
        let id1 = ObjectId::new();
        let id2 = ObjectId::new();

        targets.add_object(id1);
        targets.add_object(id2);

        assert_eq!(targets.count(), 2);
        assert_eq!(targets.legal_count(), 2);
        assert_eq!(targets.first_object(), Some(id1));
        assert!(targets.contains_object(id1));
        assert!(targets.contains_object(id2));

        let objs = targets.objects();
        assert_eq!(objs.len(), 2);
    }

    #[test]
    fn selected_targets_players() {
        let mut targets = SelectedTargets::new();
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        targets.add_player(p1);
        targets.add_player(p2);

        assert_eq!(targets.count(), 2);
        assert_eq!(targets.first_player(), Some(p1));
        assert!(targets.contains_player(p1));
        assert!(targets.contains_player(p2));
    }

    #[test]
    fn mark_illegal() {
        let mut targets = SelectedTargets::new();
        let id1 = ObjectId::new();
        let id2 = ObjectId::new();

        targets.add_object(id1);
        targets.add_object(id2);

        assert!(targets.all_legal());
        targets.mark_illegal(id1);
        assert!(!targets.all_legal());
        assert!(targets.any_legal());
        assert!(!targets.contains_object(id1));
        assert!(targets.contains_object(id2));
        assert_eq!(targets.legal_count(), 1);
    }

    #[test]
    fn mark_player_illegal() {
        let mut targets = SelectedTargets::new();
        let p1 = PlayerId::new();

        targets.add_player(p1);
        assert!(targets.contains_player(p1));

        targets.mark_player_illegal(p1);
        assert!(!targets.contains_player(p1));
        assert_eq!(targets.legal_count(), 0);
    }

    #[test]
    fn divided_amounts() {
        let mut targets = SelectedTargets::new();
        let id1 = ObjectId::new();
        let id2 = ObjectId::new();

        targets.add_object_with_amount(id1, 2);
        targets.add_object_with_amount(id2, 1);

        let entries = targets.entries();
        assert_eq!(entries[0].amount, 2);
        assert_eq!(entries[1].amount, 1);
    }

    #[test]
    fn empty_targets() {
        let targets = SelectedTargets::new();
        assert!(targets.is_empty());
        assert_eq!(targets.count(), 0);
        assert_eq!(targets.legal_count(), 0);
        assert_eq!(targets.first_object(), None);
        assert_eq!(targets.first_player(), None);
    }
}
