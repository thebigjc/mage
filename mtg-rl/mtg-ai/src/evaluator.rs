// Board state evaluator for heuristic AI.
//
// Port of GameStateEvaluator2.java and ArtificialScoringSystem.java from XMage.
// Evaluates a game state from a specific player's perspective, producing a
// numeric score where positive = good for that player, negative = bad.
//
// This evaluator is designed for two-player games.

use mtg_engine::types::PlayerId;

// ---------------------------------------------------------------------------
// Constants (ported from ArtificialScoringSystem.java)
// ---------------------------------------------------------------------------

pub const WIN_GAME_SCORE: i32 = 100_000_000;
pub const LOSE_GAME_SCORE: i32 = -WIN_GAME_SCORE;

/// Score per card in hand.
pub const HAND_CARD_SCORE: i32 = 5;

/// Base score for having a permanent on the battlefield.
pub const PERMANENT_BASE_SCORE: i32 = 300;

/// Life multiplier for life above the lookup table range.
const LIFE_ABOVE_MULTIPLIER: i32 = 100;

/// Non-linear life score table. Index = life total (0..=20).
/// Higher life totals have diminishing returns.
const LIFE_SCORES: [i32; 21] = [
    0, 1000, 2000, 3000, 4000, 4500, 5000, 5500, 6000, 6500, 7000, 7400,
    7800, 8200, 8600, 9000, 9200, 9400, 9600, 9800, 10000,
];

// ---------------------------------------------------------------------------
// Keyword ability scores (ported from MagicAbility.java)
// ---------------------------------------------------------------------------

/// Scores for common keyword abilities. Used when evaluating creature permanents.
/// Map from keyword name to score value.
pub fn keyword_ability_score(keyword: &str) -> i32 {
    match keyword {
        "deathtouch" => 60,
        "defender" => -100,
        "double strike" => 100,
        "exalted" => 10,
        "first strike" => 50,
        "flash" => 20,
        "flying" => 50,
        "forestwalk" => 10,
        "haste" => 20,
        "hexproof" => 60,
        "indestructible" => 150,
        "infect" => 60,
        "intimidate" => 50,
        "islandwalk" => 10,
        "lifelink" => 40,
        "menace" => 30,
        "mountainwalk" => 10,
        "plainswalk" => 10,
        "reach" => 20,
        "shroud" => 60,
        "swampwalk" => 10,
        "trample" => 30,
        "unblockable" => 100,
        "vigilance" => 20,
        "wither" => 30,
        "evolve" => 50,
        "extort" => 30,
        _ => 2, // unknown ability gets a small bonus
    }
}

// ---------------------------------------------------------------------------
// Life scoring
// ---------------------------------------------------------------------------

/// Convert a life total to a score. Uses a non-linear table for 0-20,
/// then linear growth above 20.
pub fn life_score(life: i32) -> i32 {
    if life < 0 {
        0
    } else if life as usize > LIFE_SCORES.len() - 1 {
        let max_idx = LIFE_SCORES.len() - 1;
        LIFE_SCORES[max_idx] + (life - max_idx as i32) * LIFE_ABOVE_MULTIPLIER
    } else {
        LIFE_SCORES[life as usize]
    }
}

// ---------------------------------------------------------------------------
// Permanent scoring
// ---------------------------------------------------------------------------

/// Information about a permanent needed for evaluation.
/// This is a simplified view — the full Permanent type will be defined by
/// the engine later.
#[derive(Clone, Debug)]
pub struct PermanentInfo {
    pub controller: PlayerId,
    pub is_creature: bool,
    pub is_land: bool,
    pub is_enchantment: bool,
    pub is_artifact: bool,
    pub is_equipment: bool,
    pub power: i32,
    pub toughness: i32,
    pub damage_marked: i32,
    pub is_tapped: bool,
    pub has_summoning_sickness: bool,
    pub has_haste: bool,
    pub can_attack: bool,
    pub can_block: bool,
    /// Keywords this permanent has (lowercase).
    pub keywords: Vec<String>,
    /// Charge/level counter count.
    pub charge_counters: i32,
    pub level_counters: i32,
    /// Total outcome score of attached enchantments/equipment.
    pub attachment_enchantment_score: i32,
    pub attachment_equipment_score: i32,
    /// Whether attached auras are detrimental (controlled by same player).
    pub detrimental_attachment_count: i32,
    /// Mana value (converted mana cost) of the card.
    pub mana_value: i32,
    /// Number of mana abilities on a land.
    pub mana_production_count: i32,
}

/// Evaluate a single permanent's contribution to board score.
/// Port of ArtificialScoringSystem.getFixedPermanentScore + getDynamicPermanentScore
/// + getCombatPermanentScore.
pub fn evaluate_permanent(perm: &PermanentInfo, use_combat_score: bool) -> i32 {
    let mut score = 0;

    // -- Fixed score (card definition + permanent bonus) --
    // Simplified from getCardDefinitionScore: base value of 3 (rating placeholder)
    let base_value = 3;
    if perm.is_land {
        score += (base_value as f32 / 2.0 * 50.0) as i32 + perm.mana_production_count * 50;
    } else if perm.is_creature {
        score += base_value * 100 - perm.mana_value * 20
            + (perm.power + perm.toughness) * 10;
    } else {
        // Non-creature, non-land
        score += base_value * 100 - perm.mana_value * 20;
    }
    score += PERMANENT_BASE_SCORE;

    if !perm.is_creature && perm.is_equipment {
        score += 100;
    }

    // -- Dynamic score --
    score += perm.charge_counters * 30;
    score += perm.level_counters * 30;
    score -= perm.damage_marked * 2;

    if perm.is_creature {
        let power = perm.power;
        let toughness = perm.toughness.max(0);
        let ability_score: i32 = perm
            .keywords
            .iter()
            .map(|k| keyword_ability_score(k))
            .sum();
        score += power * 300 + toughness * 200 + ability_score * (power.max(0) + 1) / 2;
        score += perm.attachment_enchantment_score;
        score += perm.attachment_equipment_score;
    }

    // Detrimental attachments penalty
    score -= perm.detrimental_attachment_count * 1000;

    // -- Combat score --
    if use_combat_score {
        let can_tap = !perm.is_tapped
            && (!perm.has_summoning_sickness || !perm.is_creature || perm.has_haste);
        if !can_tap {
            // Tapped penalty
            if perm.is_creature {
                score -= 100;
            } else if perm.is_land {
                score -= 20;
            } else {
                score -= 2;
            }
        }
        if perm.is_creature {
            if !perm.can_attack {
                score -= 100;
            }
            if !perm.can_block {
                score -= 30;
            }
        }
    }

    score
}

// ---------------------------------------------------------------------------
// Full game state evaluation
// ---------------------------------------------------------------------------

/// Information about a player needed for evaluation.
#[derive(Clone, Debug)]
pub struct PlayerInfo {
    pub player_id: PlayerId,
    pub life: i32,
    pub hand_size: usize,
    pub has_lost: bool,
    pub has_won: bool,
}

/// Result of evaluating a game state from one player's perspective.
#[derive(Clone, Debug)]
pub struct EvaluationScore {
    pub player_id: PlayerId,
    pub player_life_score: i32,
    pub player_hand_score: i32,
    pub player_permanents_score: i32,
    pub opponent_life_score: i32,
    pub opponent_hand_score: i32,
    pub opponent_permanents_score: i32,
    /// Non-zero only for terminal game states (win/loss).
    pub special_score: i32,
}

impl EvaluationScore {
    pub fn player_score(&self) -> i32 {
        self.player_life_score + self.player_hand_score + self.player_permanents_score
    }

    pub fn opponent_score(&self) -> i32 {
        self.opponent_life_score + self.opponent_hand_score + self.opponent_permanents_score
    }

    pub fn total_score(&self) -> i32 {
        if self.special_score != 0 {
            self.special_score
        } else {
            self.player_score() - self.opponent_score()
        }
    }
}

/// Evaluate a game state for a two-player game.
///
/// This is the main entry point. Takes player info and permanents, returns
/// a scored evaluation. Positive scores are good for `player`, negative
/// scores are good for `opponent`.
///
/// When the full GameState type is available, this will take a `&GameState`
/// and `PlayerId` instead.
pub fn evaluate(
    player: &PlayerInfo,
    opponent: &PlayerInfo,
    player_permanents: &[PermanentInfo],
    opponent_permanents: &[PermanentInfo],
    use_combat_score: bool,
) -> EvaluationScore {
    // Terminal states
    if opponent.has_lost || player.has_won {
        return EvaluationScore {
            player_id: player.player_id,
            player_life_score: 0,
            player_hand_score: 0,
            player_permanents_score: 0,
            opponent_life_score: 0,
            opponent_hand_score: 0,
            opponent_permanents_score: 0,
            special_score: WIN_GAME_SCORE,
        };
    }
    if player.has_lost || opponent.has_won {
        return EvaluationScore {
            player_id: player.player_id,
            player_life_score: 0,
            player_hand_score: 0,
            player_permanents_score: 0,
            opponent_life_score: 0,
            opponent_hand_score: 0,
            opponent_permanents_score: 0,
            special_score: LOSE_GAME_SCORE,
        };
    }

    // Life scores (with special handling for 0 or below)
    let (player_life_score, opponent_life_score) = if player.life <= 0 {
        (LOSE_GAME_SCORE, 0)
    } else if opponent.life <= 0 {
        (WIN_GAME_SCORE, 0)
    } else {
        (life_score(player.life), life_score(opponent.life))
    };

    // Permanent scores
    let player_permanents_score: i32 = player_permanents
        .iter()
        .map(|p| evaluate_permanent(p, use_combat_score))
        .sum();
    let opponent_permanents_score: i32 = opponent_permanents
        .iter()
        .map(|p| evaluate_permanent(p, use_combat_score))
        .sum();

    // Hand scores
    let player_hand_score = player.hand_size as i32 * HAND_CARD_SCORE;
    let opponent_hand_score = opponent.hand_size as i32 * HAND_CARD_SCORE;

    EvaluationScore {
        player_id: player.player_id,
        player_life_score,
        player_hand_score,
        player_permanents_score,
        opponent_life_score,
        opponent_hand_score,
        opponent_permanents_score,
        special_score: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn life_score_lookup_table() {
        assert_eq!(life_score(0), 0);
        assert_eq!(life_score(1), 1000);
        assert_eq!(life_score(20), 10000);
        assert_eq!(life_score(-5), 0);
    }

    #[test]
    fn life_score_above_20() {
        // 20 -> 10000, each additional life adds 100
        assert_eq!(life_score(21), 10100);
        assert_eq!(life_score(25), 10500);
    }

    #[test]
    fn win_state_gives_max_score() {
        let player = PlayerInfo {
            player_id: PlayerId::new(),
            life: 20,
            hand_size: 7,
            has_lost: false,
            has_won: true,
        };
        let opponent = PlayerInfo {
            player_id: PlayerId::new(),
            life: 20,
            hand_size: 7,
            has_lost: false,
            has_won: false,
        };
        let result = evaluate(&player, &opponent, &[], &[], true);
        assert_eq!(result.total_score(), WIN_GAME_SCORE);
    }

    #[test]
    fn loss_state_gives_min_score() {
        let player = PlayerInfo {
            player_id: PlayerId::new(),
            life: 20,
            hand_size: 7,
            has_lost: true,
            has_won: false,
        };
        let opponent = PlayerInfo {
            player_id: PlayerId::new(),
            life: 20,
            hand_size: 7,
            has_lost: false,
            has_won: false,
        };
        let result = evaluate(&player, &opponent, &[], &[], true);
        assert_eq!(result.total_score(), LOSE_GAME_SCORE);
    }

    #[test]
    fn equal_board_state_scores_zero() {
        let player = PlayerInfo {
            player_id: PlayerId::new(),
            life: 20,
            hand_size: 7,
            has_lost: false,
            has_won: false,
        };
        let opponent = PlayerInfo {
            player_id: PlayerId::new(),
            life: 20,
            hand_size: 7,
            has_lost: false,
            has_won: false,
        };
        let result = evaluate(&player, &opponent, &[], &[], true);
        // Same life, same hand, no permanents -> score should be 0
        assert_eq!(result.total_score(), 0);
    }

    #[test]
    fn more_life_is_better() {
        let player = PlayerInfo {
            player_id: PlayerId::new(),
            life: 20,
            hand_size: 7,
            has_lost: false,
            has_won: false,
        };
        let opponent = PlayerInfo {
            player_id: PlayerId::new(),
            life: 10,
            hand_size: 7,
            has_lost: false,
            has_won: false,
        };
        let result = evaluate(&player, &opponent, &[], &[], true);
        assert!(result.total_score() > 0);
    }

    #[test]
    fn creature_permanent_scores_positively() {
        let grizzly = PermanentInfo {
            controller: PlayerId::new(),
            is_creature: true,
            is_land: false,
            is_enchantment: false,
            is_artifact: false,
            is_equipment: false,
            power: 2,
            toughness: 2,
            damage_marked: 0,
            is_tapped: false,
            has_summoning_sickness: false,
            has_haste: false,
            can_attack: true,
            can_block: true,
            keywords: Vec::new(),
            charge_counters: 0,
            level_counters: 0,
            attachment_enchantment_score: 0,
            attachment_equipment_score: 0,
            detrimental_attachment_count: 0,
            mana_value: 2,
            mana_production_count: 0,
        };
        let score = evaluate_permanent(&grizzly, true);
        assert!(score > 0, "A 2/2 creature should score positively, got {score}");
    }

    #[test]
    fn flying_creature_scores_higher() {
        let vanilla = PermanentInfo {
            controller: PlayerId::new(),
            is_creature: true,
            is_land: false,
            is_enchantment: false,
            is_artifact: false,
            is_equipment: false,
            power: 2,
            toughness: 2,
            damage_marked: 0,
            is_tapped: false,
            has_summoning_sickness: false,
            has_haste: false,
            can_attack: true,
            can_block: true,
            keywords: Vec::new(),
            charge_counters: 0,
            level_counters: 0,
            attachment_enchantment_score: 0,
            attachment_equipment_score: 0,
            detrimental_attachment_count: 0,
            mana_value: 2,
            mana_production_count: 0,
        };
        let flyer = PermanentInfo {
            keywords: vec!["flying".to_string()],
            ..vanilla.clone()
        };
        let vanilla_score = evaluate_permanent(&vanilla, true);
        let flyer_score = evaluate_permanent(&flyer, true);
        assert!(
            flyer_score > vanilla_score,
            "Flying creature should score higher: {flyer_score} vs {vanilla_score}"
        );
    }

    #[test]
    fn keyword_scores_are_reasonable() {
        assert_eq!(keyword_ability_score("flying"), 50);
        assert_eq!(keyword_ability_score("deathtouch"), 60);
        assert_eq!(keyword_ability_score("indestructible"), 150);
        assert_eq!(keyword_ability_score("defender"), -100);
        assert_eq!(keyword_ability_score("unknown_ability"), 2);
    }
}
