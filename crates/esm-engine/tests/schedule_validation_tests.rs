use esm_engine::schedule::validation::{ScheduleError, ScheduleValidator};
use esm_engine::schedule::{ScheduleEntry, SoloQueueFocus, TeamWeeklySchedule};
use esm_models::time::TimeSlot;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn schedule_with_scrim(day: usize, slot: TimeSlot, scrim_id: u32) -> TeamWeeklySchedule {
    let mut ws = TeamWeeklySchedule::new();
    ws.day_mut(day).set(slot, ScheduleEntry::Scrim { scrim_id });
    ws
}

// ---------------------------------------------------------------------------
// Slot availability
// ---------------------------------------------------------------------------

#[test]
fn can_schedule_into_free_slot() {
    let ws = TeamWeeklySchedule::new();
    let result = ScheduleValidator::check_slot_free(&ws, 0, TimeSlot::Morning);
    assert!(result.is_ok());
}

#[test]
fn cannot_schedule_into_occupied_slot() {
    let ws = schedule_with_scrim(0, TimeSlot::Morning, 1);
    let result = ScheduleValidator::check_slot_free(&ws, 0, TimeSlot::Morning);
    assert_eq!(result, Err(ScheduleError::SlotOccupied));
}

// ---------------------------------------------------------------------------
// Max scrims per day (3)
// ---------------------------------------------------------------------------

#[test]
fn allows_up_to_three_scrims_per_day() {
    let mut ws = TeamWeeklySchedule::new();
    ws.day_mut(0)
        .set(TimeSlot::Morning, ScheduleEntry::Scrim { scrim_id: 1 });
    ws.day_mut(0)
        .set(TimeSlot::Afternoon, ScheduleEntry::Scrim { scrim_id: 2 });
    // Third scrim in evening — should be allowed (check before adding)
    let result = ScheduleValidator::check_scrim_limit(&ws, 0);
    assert!(result.is_ok());
}

#[test]
fn rejects_fourth_scrim_on_same_day() {
    let mut ws = TeamWeeklySchedule::new();
    ws.day_mut(0)
        .set(TimeSlot::Morning, ScheduleEntry::Scrim { scrim_id: 1 });
    ws.day_mut(0)
        .set(TimeSlot::Afternoon, ScheduleEntry::Scrim { scrim_id: 2 });
    ws.day_mut(0)
        .set(TimeSlot::Evening, ScheduleEntry::Scrim { scrim_id: 3 });
    // All 3 slots filled with scrims — no room for another
    let result = ScheduleValidator::check_scrim_limit(&ws, 0);
    assert_eq!(result, Err(ScheduleError::MaxScrimsPerDay));
}

// ---------------------------------------------------------------------------
// Cannot schedule on the current day (must be future)
// ---------------------------------------------------------------------------

#[test]
fn allows_scheduling_on_future_day() {
    let result = ScheduleValidator::check_not_same_day(5, 3);
    assert!(result.is_ok());
}

#[test]
fn rejects_scheduling_on_current_day() {
    let result = ScheduleValidator::check_not_same_day(5, 5);
    assert_eq!(result, Err(ScheduleError::SameDayScheduling));
}

#[test]
fn rejects_scheduling_in_the_past() {
    let result = ScheduleValidator::check_not_same_day(5, 7);
    assert_eq!(result, Err(ScheduleError::PastDate));
}

// ---------------------------------------------------------------------------
// Solo queue: player allocation limit (max 2 per slot)
// ---------------------------------------------------------------------------

#[test]
fn solo_queue_allows_player_with_no_existing_allocation() {
    let ws = TeamWeeklySchedule::new();
    let result = ScheduleValidator::check_solo_queue_player_limit(&ws, 0, 0);
    assert!(result.is_ok());
}

#[test]
fn solo_queue_allows_player_with_one_existing_allocation() {
    let mut ws = TeamWeeklySchedule::new();
    ws.day_mut(0).set(
        TimeSlot::Morning,
        ScheduleEntry::SoloQueue {
            players: vec![0, 1],
            focus: SoloQueueFocus::Mechanics,
        },
    );
    // Player 0 already has 1 allocation on day 0; allow a second
    let result = ScheduleValidator::check_solo_queue_player_limit(&ws, 0, 0);
    assert!(result.is_ok());
}

#[test]
fn solo_queue_rejects_player_with_two_existing_allocations() {
    let mut ws = TeamWeeklySchedule::new();
    ws.day_mut(0).set(
        TimeSlot::Morning,
        ScheduleEntry::SoloQueue {
            players: vec![0],
            focus: SoloQueueFocus::Mechanics,
        },
    );
    ws.day_mut(0).set(
        TimeSlot::Afternoon,
        ScheduleEntry::SoloQueue {
            players: vec![0, 2],
            focus: SoloQueueFocus::Champions,
        },
    );
    // Player 0 already has 2 allocations on day 0; reject a third
    let result = ScheduleValidator::check_solo_queue_player_limit(&ws, 0, 0);
    assert_eq!(result, Err(ScheduleError::PlayerSoloQueueLimit));
}

#[test]
fn solo_queue_limit_is_per_player() {
    let mut ws = TeamWeeklySchedule::new();
    ws.day_mut(0).set(
        TimeSlot::Morning,
        ScheduleEntry::SoloQueue {
            players: vec![0],
            focus: SoloQueueFocus::Mechanics,
        },
    );
    ws.day_mut(0).set(
        TimeSlot::Afternoon,
        ScheduleEntry::SoloQueue {
            players: vec![0],
            focus: SoloQueueFocus::Champions,
        },
    );
    // Player 1 has 0 allocations — should be fine
    let result = ScheduleValidator::check_solo_queue_player_limit(&ws, 0, 1);
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// Cannot schedule scrim if opponent slot is occupied
// ---------------------------------------------------------------------------

#[test]
fn allows_scrim_when_opponent_slot_is_free() {
    let opponent = TeamWeeklySchedule::new();
    let result = ScheduleValidator::check_slot_free(&opponent, 0, TimeSlot::Morning);
    assert!(result.is_ok());
}

#[test]
fn rejects_scrim_when_opponent_slot_is_occupied() {
    let opponent = schedule_with_scrim(0, TimeSlot::Morning, 99);
    let result = ScheduleValidator::check_slot_free(&opponent, 0, TimeSlot::Morning);
    assert_eq!(result, Err(ScheduleError::SlotOccupied));
}
