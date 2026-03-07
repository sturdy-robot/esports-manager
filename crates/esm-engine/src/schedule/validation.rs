use esm_models::time::TimeSlot;

use super::{ScheduleEntry, TeamWeeklySchedule};

// ---------------------------------------------------------------------------
// Scheduling errors
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleError {
    SlotOccupied,
    MaxScrimsPerDay,
    SameDayScheduling,
    PastDate,
    PlayerSoloQueueLimit,
}

// ---------------------------------------------------------------------------
// Validator — pure functions, no side effects
// ---------------------------------------------------------------------------

const MAX_SCRIMS_PER_DAY: usize = 3;
const MAX_SOLO_QUEUE_PER_PLAYER_PER_DAY: usize = 2;

pub struct ScheduleValidator;

impl ScheduleValidator {
    /// Check that the given slot on the given day is free.
    pub fn check_slot_free(
        schedule: &TeamWeeklySchedule,
        day_index: usize,
        slot: TimeSlot,
    ) -> Result<(), ScheduleError> {
        if schedule.day(day_index).is_free(slot) {
            Ok(())
        } else {
            Err(ScheduleError::SlotOccupied)
        }
    }

    /// Check that the day has not already reached the max scrim limit.
    /// Call this *before* adding a new scrim to the day.
    pub fn check_scrim_limit(
        schedule: &TeamWeeklySchedule,
        day_index: usize,
    ) -> Result<(), ScheduleError> {
        if schedule.day(day_index).scrim_count() < MAX_SCRIMS_PER_DAY {
            Ok(())
        } else {
            Err(ScheduleError::MaxScrimsPerDay)
        }
    }

    /// Ensure the scheduled day is strictly in the future relative to the
    /// current day. `scheduled_day` and `current_day` are both `days_elapsed`.
    pub fn check_not_same_day(scheduled_day: u32, current_day: u32) -> Result<(), ScheduleError> {
        if scheduled_day > current_day {
            Ok(())
        } else if scheduled_day < current_day {
            Err(ScheduleError::PastDate)
        } else {
            Err(ScheduleError::SameDayScheduling)
        }
    }

    /// Check that a player has not exceeded the solo-queue-per-day limit.
    /// `player_index` is the roster index to check.
    pub fn check_solo_queue_player_limit(
        schedule: &TeamWeeklySchedule,
        day_index: usize,
        player_index: usize,
    ) -> Result<(), ScheduleError> {
        let count = TimeSlot::ALL
            .iter()
            .filter(|&&slot| {
                matches!(
                    schedule.day(day_index).get(slot),
                    Some(ScheduleEntry::SoloQueue { players, .. }) if players.contains(&player_index)
                )
            })
            .count();

        if count < MAX_SOLO_QUEUE_PER_PLAYER_PER_DAY {
            Ok(())
        } else {
            Err(ScheduleError::PlayerSoloQueueLimit)
        }
    }
}
