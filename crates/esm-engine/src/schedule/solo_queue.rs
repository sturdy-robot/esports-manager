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
