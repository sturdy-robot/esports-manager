use serde::{Deserialize, Serialize};

use esm_models::time::TimeSlot;

use super::ScrimId;

// ---------------------------------------------------------------------------
// Draft rules for scrims
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrimDraftRules {
    Standard,
    Fearless,
}

// ---------------------------------------------------------------------------
// Scrim status
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrimStatus {
    Scheduled,
    Completed,
    Cancelled,
}

// ---------------------------------------------------------------------------
// Scrim entity
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scrim {
    id: ScrimId,
    home_team: String,
    away_team: String,
    scheduled_day: u32,
    game_count: u32,
    time_slot: TimeSlot,
    draft_rules: ScrimDraftRules,
    status: ScrimStatus,
    home_wins: u32,
    away_wins: u32,
}

impl Scrim {
    pub fn new(
        id: ScrimId,
        home_team: String,
        away_team: String,
        scheduled_day: u32,
        game_count: u32,
        time_slot: TimeSlot,
        draft_rules: ScrimDraftRules,
    ) -> Self {
        Self {
            id,
            home_team,
            away_team,
            scheduled_day,
            game_count,
            time_slot,
            draft_rules,
            status: ScrimStatus::Scheduled,
            home_wins: 0,
            away_wins: 0,
        }
    }

    pub fn id(&self) -> ScrimId {
        self.id
    }

    pub fn home_team(&self) -> &str {
        &self.home_team
    }

    pub fn away_team(&self) -> &str {
        &self.away_team
    }

    pub fn scheduled_day(&self) -> u32 {
        self.scheduled_day
    }

    pub fn game_count(&self) -> u32 {
        self.game_count
    }

    pub fn time_slot(&self) -> TimeSlot {
        self.time_slot
    }

    pub fn draft_rules(&self) -> ScrimDraftRules {
        self.draft_rules
    }

    pub fn status(&self) -> ScrimStatus {
        self.status
    }

    pub fn home_wins(&self) -> u32 {
        self.home_wins
    }

    pub fn away_wins(&self) -> u32 {
        self.away_wins
    }

    pub fn complete(&mut self, home_wins: u32, away_wins: u32) {
        self.status = ScrimStatus::Completed;
        self.home_wins = home_wins;
        self.away_wins = away_wins;
    }

    pub fn cancel(&mut self) {
        self.status = ScrimStatus::Cancelled;
    }

    pub fn involves_team(&self, team_name: &str) -> bool {
        self.home_team == team_name || self.away_team == team_name
    }
}
