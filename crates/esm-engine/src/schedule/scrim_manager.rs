use esm_models::time::TimeSlot;
use serde::{Deserialize, Serialize};

use super::scrim::{Scrim, ScrimDraftRules, ScrimStatus};
use super::validation::{ScheduleError, ScheduleValidator};
use super::{ScheduleEntry, ScrimId, TeamWeeklySchedule};

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrimScheduleError {
    SameDayScheduling,
    PastDate,
    HomeSlotOccupied,
    AwaySlotOccupied,
    MaxScrimsPerDay,
    CannotCancelCompleted,
    CannotCancelPast,
    ScrimNotFound,
}

impl From<ScheduleError> for ScrimScheduleError {
    fn from(e: ScheduleError) -> Self {
        match e {
            ScheduleError::SlotOccupied => ScrimScheduleError::HomeSlotOccupied,
            ScheduleError::MaxScrimsPerDay => ScrimScheduleError::MaxScrimsPerDay,
            ScheduleError::SameDayScheduling => ScrimScheduleError::SameDayScheduling,
            ScheduleError::PastDate => ScrimScheduleError::PastDate,
            ScheduleError::PlayerSoloQueueLimit => ScrimScheduleError::HomeSlotOccupied,
        }
    }
}

// ---------------------------------------------------------------------------
// ScrimManager
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrimManager {
    scrims: Vec<Scrim>,
    next_id: ScrimId,
}

impl ScrimManager {
    pub fn new() -> Self {
        Self {
            scrims: Vec::new(),
            next_id: 1,
        }
    }

    pub fn scrims(&self) -> &[Scrim] {
        &self.scrims
    }

    pub fn scrim_by_id(&self, id: ScrimId) -> Option<&Scrim> {
        self.scrims.iter().find(|s| s.id() == id)
    }

    pub fn scrim_by_id_mut(&mut self, id: ScrimId) -> Option<&mut Scrim> {
        self.scrims.iter_mut().find(|s| s.id() == id)
    }

    pub fn scrims_for_team(&self, team_name: &str) -> Vec<&Scrim> {
        self.scrims
            .iter()
            .filter(|s| s.involves_team(team_name))
            .collect()
    }

    /// Schedule a new scrim between two teams.
    ///
    /// Validates all constraints, creates the scrim, and fills both teams'
    /// weekly schedules. Returns the new scrim ID on success.
    #[allow(clippy::too_many_arguments)]
    pub fn schedule_scrim(
        &mut self,
        schedules: &mut [TeamWeeklySchedule],
        home_index: usize,
        away_index: usize,
        home_name: &str,
        away_name: &str,
        scheduled_day: u32,
        time_slot: TimeSlot,
        game_count: u32,
        draft_rules: ScrimDraftRules,
        current_day: u32,
    ) -> Result<ScrimId, ScrimScheduleError> {
        // Day index within the week (0-6)
        let day_index = scheduled_day as usize % 7;

        // Validate: must be in the future
        ScheduleValidator::check_not_same_day(scheduled_day, current_day)?;

        // Validate: home team slot is free
        ScheduleValidator::check_slot_free(&schedules[home_index], day_index, time_slot)
            .map_err(|_| ScrimScheduleError::HomeSlotOccupied)?;

        // Validate: away team slot is free
        ScheduleValidator::check_slot_free(&schedules[away_index], day_index, time_slot)
            .map_err(|_| ScrimScheduleError::AwaySlotOccupied)?;

        // Validate: home team scrim limit
        ScheduleValidator::check_scrim_limit(&schedules[home_index], day_index)
            .map_err(|_| ScrimScheduleError::MaxScrimsPerDay)?;

        // Create the scrim
        let id = self.next_id;
        self.next_id += 1;

        let scrim = Scrim::new(
            id,
            home_name.to_string(),
            away_name.to_string(),
            scheduled_day,
            game_count,
            time_slot,
            draft_rules,
        );
        self.scrims.push(scrim);

        // Fill both schedules
        let entry = ScheduleEntry::Scrim { scrim_id: id };
        schedules[home_index]
            .day_mut(day_index)
            .set(time_slot, entry.clone());
        schedules[away_index]
            .day_mut(day_index)
            .set(time_slot, entry);

        Ok(id)
    }

    /// Return IDs and game counts of all scrims scheduled for the given day
    /// that are still in `Scheduled` status.
    pub fn pending_scrims_for_day(&self, day: u32) -> Vec<(ScrimId, u32)> {
        self.scrims
            .iter()
            .filter(|s| s.scheduled_day() == day && s.status() == ScrimStatus::Scheduled)
            .map(|s| (s.id(), s.game_count()))
            .collect()
    }

    /// Cancel a scheduled (not yet played) scrim and free both teams' slots.
    pub fn cancel_scrim(
        &mut self,
        scrim_id: ScrimId,
        schedules: &mut [TeamWeeklySchedule],
        home_index: usize,
        away_index: usize,
        current_day: u32,
    ) -> Result<(), ScrimScheduleError> {
        let scrim = self
            .scrim_by_id(scrim_id)
            .ok_or(ScrimScheduleError::ScrimNotFound)?;

        if scrim.status() == ScrimStatus::Completed {
            return Err(ScrimScheduleError::CannotCancelCompleted);
        }

        if scrim.scheduled_day() <= current_day {
            return Err(ScrimScheduleError::CannotCancelPast);
        }

        let day_index = scrim.scheduled_day() as usize % 7;
        let time_slot = scrim.time_slot();

        // Clear both schedules
        schedules[home_index].day_mut(day_index).clear(time_slot);
        schedules[away_index].day_mut(day_index).clear(time_slot);

        // Mark scrim as cancelled
        self.scrim_by_id_mut(scrim_id).unwrap().cancel();

        Ok(())
    }
}

impl Default for ScrimManager {
    fn default() -> Self {
        Self::new()
    }
}
