use serde::{Deserialize, Serialize};

use esm_models::player::BoundedAttribute;

// ---------------------------------------------------------------------------
// Board Evaluation
// ---------------------------------------------------------------------------

/// Represents the weekly board evaluation deltas.
/// Weights: tournament (primary, 3x), roster (secondary, 2x), financial (tertiary, 1x).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardEvaluation {
    pub tournament_delta: i16,
    pub roster_delta: i16,
    pub financial_delta: i16,
}

impl BoardEvaluation {
    fn weighted_total(&self) -> i16 {
        self.tournament_delta * 3 + self.roster_delta * 2 + self.financial_delta
    }
}

// ---------------------------------------------------------------------------
// Manager Satisfaction
// ---------------------------------------------------------------------------

const WARNING_THRESHOLD: u8 = 30;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagerSatisfaction {
    score: BoundedAttribute,
}

impl ManagerSatisfaction {
    pub fn new() -> Self {
        Self {
            score: BoundedAttribute::new(50),
        }
    }

    pub fn score(&self) -> BoundedAttribute {
        self.score
    }

    pub fn score_mut(&mut self) -> &mut BoundedAttribute {
        &mut self.score
    }

    /// Returns `true` if satisfaction is at or below the warning threshold (≤30).
    pub fn is_warning(&self) -> bool {
        self.score.value() <= WARNING_THRESHOLD
    }

    /// Returns `true` if satisfaction has dropped to 0 — triggers termination.
    pub fn is_terminated(&self) -> bool {
        self.score.value() == 0
    }

    /// Apply a weekly board evaluation to the satisfaction score.
    pub fn apply_evaluation(&mut self, eval: &BoardEvaluation) {
        let total = eval.weighted_total();
        if total >= 0 {
            self.score.increase(total as u8);
        } else {
            self.score.decrease(total.unsigned_abs() as u8);
        }
    }
}

impl Default for ManagerSatisfaction {
    fn default() -> Self {
        Self::new()
    }
}
