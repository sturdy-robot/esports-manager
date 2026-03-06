use serde::{Deserialize, Serialize};

/// Playstyle affects the balance between aggression and farming.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Playstyle {
    Aggressive,
    Balanced,
    Defensive,
}

/// Focus determines the team's strategic priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Focus {
    Teamfight,
    Splitpush,
    Objective,
}

/// Tactical choices the player can set before or during a match.
/// These modify event weights in the simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchTactics {
    pub playstyle: Playstyle,
    pub focus: Focus,
}

impl Default for MatchTactics {
    fn default() -> Self {
        Self {
            playstyle: Playstyle::Balanced,
            focus: Focus::Teamfight,
        }
    }
}

impl MatchTactics {
    /// Weight multiplier for solo kill events.
    pub fn solo_kill_mult(&self) -> f64 {
        match self.playstyle {
            Playstyle::Aggressive => 1.6,
            Playstyle::Balanced => 1.0,
            Playstyle::Defensive => 0.5,
        }
    }

    /// Weight multiplier for teamfight events.
    pub fn teamfight_mult(&self) -> f64 {
        let base = match self.playstyle {
            Playstyle::Aggressive => 1.3,
            Playstyle::Balanced => 1.0,
            Playstyle::Defensive => 0.7,
        };
        let focus = match self.focus {
            Focus::Teamfight => 1.4,
            Focus::Splitpush => 0.7,
            Focus::Objective => 0.9,
        };
        base * focus
    }

    /// Weight multiplier for tower siege events (splitpush).
    pub fn tower_mult(&self) -> f64 {
        match self.focus {
            Focus::Splitpush => 1.6,
            Focus::Teamfight => 0.8,
            Focus::Objective => 1.0,
        }
    }

    /// Weight multiplier for objective (dragon/baron/herald) events.
    pub fn objective_mult(&self) -> f64 {
        match self.focus {
            Focus::Objective => 1.5,
            Focus::Teamfight => 0.9,
            Focus::Splitpush => 0.8,
        }
    }

    /// Weight multiplier for farm tick events.
    pub fn farm_mult(&self) -> f64 {
        match self.playstyle {
            Playstyle::Defensive => 1.4,
            Playstyle::Balanced => 1.0,
            Playstyle::Aggressive => 0.7,
        }
    }
}
