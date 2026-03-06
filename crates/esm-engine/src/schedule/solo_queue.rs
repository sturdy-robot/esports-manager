use esm_core::rng::GameRng;
use serde::{Deserialize, Serialize};

use esm_models::time::TimeSlot;

use super::SoloQueueFocus;

// ---------------------------------------------------------------------------
// Solo queue result (pseudo-game outcome)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SoloQueueResult {
    wins: u32,
    losses: u32,
    experience_gained: u32,
}

impl SoloQueueResult {
    pub fn new(wins: u32, losses: u32, experience_gained: u32) -> Self {
        Self {
            wins,
            losses,
            experience_gained,
        }
    }

    pub fn wins(&self) -> u32 {
        self.wins
    }

    pub fn losses(&self) -> u32 {
        self.losses
    }

    pub fn games_played(&self) -> u32 {
        self.wins + self.losses
    }

    pub fn experience_gained(&self) -> u32 {
        self.experience_gained
    }
}

// ---------------------------------------------------------------------------
// Solo queue session
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoloQueueSession {
    player_index: usize,
    time_slot: TimeSlot,
    focus: SoloQueueFocus,
    scheduled_day: u32,
    result: Option<SoloQueueResult>,
}

impl SoloQueueSession {
    pub fn new(
        player_index: usize,
        time_slot: TimeSlot,
        focus: SoloQueueFocus,
        scheduled_day: u32,
    ) -> Self {
        Self {
            player_index,
            time_slot,
            focus,
            scheduled_day,
            result: None,
        }
    }

    pub fn player_index(&self) -> usize {
        self.player_index
    }

    pub fn time_slot(&self) -> TimeSlot {
        self.time_slot
    }

    pub fn focus(&self) -> SoloQueueFocus {
        self.focus
    }

    pub fn scheduled_day(&self) -> u32 {
        self.scheduled_day
    }

    pub fn result(&self) -> Option<&SoloQueueResult> {
        self.result.as_ref()
    }

    pub fn complete(&mut self, result: SoloQueueResult) {
        self.result = Some(result);
    }
}

// ---------------------------------------------------------------------------
// Solo queue simulator — generates pseudo-game results
// ---------------------------------------------------------------------------

pub struct SoloQueueSimulator;

impl SoloQueueSimulator {
    /// Generate a pseudo-game result for a solo queue session.
    ///
    /// `skill` is the player's overall skill rating (0–100).
    /// Higher skill → higher win probability.
    /// Games played per session: 3–5.
    /// Experience per game: base 1 + focus bonus (wins give extra).
    pub fn generate_result(
        rng: &mut GameRng,
        skill: u32,
        focus: SoloQueueFocus,
    ) -> SoloQueueResult {
        let games = rng.range_u32(3, 6); // 3..=5

        // Win probability: base 30% + skill contribution (up to ~40%)
        let win_prob = 0.30 + (skill as f64 / 100.0) * 0.40;

        let mut wins = 0u32;
        for _ in 0..games {
            if rng.check_probability(win_prob) {
                wins += 1;
            }
        }
        let losses = games - wins;

        // Experience: base per game + bonus per win + focus multiplier
        let focus_multiplier = match focus {
            SoloQueueFocus::Champions => 2,
            SoloQueueFocus::Tactics => 2,
            SoloQueueFocus::Mechanics => 1,
            SoloQueueFocus::Mentality => 1,
        };
        let experience = games + wins * focus_multiplier + focus_multiplier;

        SoloQueueResult::new(wins, losses, experience)
    }
}
