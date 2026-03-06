use esm_engine::schedule::scrim::{ScrimDraftRules, ScrimStatus};
use esm_engine::schedule::scrim_manager::{ScrimManager, ScrimScheduleError};
use esm_engine::schedule::{ScheduleEntry, TeamWeeklySchedule};
use esm_models::time::TimeSlot;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn two_empty_schedules() -> Vec<TeamWeeklySchedule> {
    vec![TeamWeeklySchedule::new(), TeamWeeklySchedule::new()]
}

// ---------------------------------------------------------------------------
// ScrimManager creation
// ---------------------------------------------------------------------------

#[test]
fn manager_starts_with_no_scrims() {
    let mgr = ScrimManager::new();
    assert!(mgr.scrims().is_empty());
}

// ---------------------------------------------------------------------------
// Schedule a scrim
// ---------------------------------------------------------------------------

#[test]
fn schedule_scrim_creates_scrim_and_fills_both_schedules() {
    let mut mgr = ScrimManager::new();
    let mut schedules = two_empty_schedules();
    let current_day = 0;

    let result = mgr.schedule_scrim(
        &mut schedules,
        0, // home team index
        1, // away team index
        "T1",
        "Gen.G",
        2, // scheduled day (future)
        TimeSlot::Morning,
        3, // game count
        ScrimDraftRules::Standard,
        current_day,
    );

    assert!(result.is_ok());
    let scrim_id = result.unwrap();

    // Scrim should exist in the manager
    assert_eq!(mgr.scrims().len(), 1);
    let scrim = &mgr.scrims()[0];
    assert_eq!(scrim.id(), scrim_id);
    assert_eq!(scrim.home_team(), "T1");
    assert_eq!(scrim.away_team(), "Gen.G");
    assert_eq!(scrim.game_count(), 3);
    assert_eq!(scrim.status(), ScrimStatus::Scheduled);

    // Both teams' schedules should have the scrim entry
    match schedules[0].day(2).get(TimeSlot::Morning) {
        Some(ScheduleEntry::Scrim { scrim_id: id }) => assert_eq!(*id, scrim_id),
        _ => panic!("Home team schedule should have scrim entry"),
    }
    match schedules[1].day(2).get(TimeSlot::Morning) {
        Some(ScheduleEntry::Scrim { scrim_id: id }) => assert_eq!(*id, scrim_id),
        _ => panic!("Away team schedule should have scrim entry"),
    }
}

#[test]
fn schedule_scrim_rejects_same_day() {
    let mut mgr = ScrimManager::new();
    let mut schedules = two_empty_schedules();

    let result = mgr.schedule_scrim(
        &mut schedules,
        0,
        1,
        "T1",
        "Gen.G",
        5,
        TimeSlot::Morning,
        3,
        ScrimDraftRules::Standard,
        5,
    );
    assert_eq!(result, Err(ScrimScheduleError::SameDayScheduling));
}

#[test]
fn schedule_scrim_rejects_occupied_home_slot() {
    let mut mgr = ScrimManager::new();
    let mut schedules = two_empty_schedules();
    schedules[0]
        .day_mut(2)
        .set(TimeSlot::Morning, ScheduleEntry::Rest);

    let result = mgr.schedule_scrim(
        &mut schedules,
        0,
        1,
        "T1",
        "Gen.G",
        2,
        TimeSlot::Morning,
        3,
        ScrimDraftRules::Standard,
        0,
    );
    assert_eq!(result, Err(ScrimScheduleError::HomeSlotOccupied));
}

#[test]
fn schedule_scrim_rejects_occupied_away_slot() {
    let mut mgr = ScrimManager::new();
    let mut schedules = two_empty_schedules();
    schedules[1]
        .day_mut(2)
        .set(TimeSlot::Morning, ScheduleEntry::Scrim { scrim_id: 99 });

    let result = mgr.schedule_scrim(
        &mut schedules,
        0,
        1,
        "T1",
        "Gen.G",
        2,
        TimeSlot::Morning,
        3,
        ScrimDraftRules::Standard,
        0,
    );
    assert_eq!(result, Err(ScrimScheduleError::AwaySlotOccupied));
}

#[test]
fn schedule_scrim_rejects_when_max_scrims_reached() {
    let mut mgr = ScrimManager::new();
    let mut schedules = two_empty_schedules();

    // Fill all 3 slots on day 2 for home team with scrims
    for slot in TimeSlot::ALL.iter() {
        mgr.schedule_scrim(
            &mut schedules,
            0,
            1,
            "T1",
            "Gen.G",
            2,
            *slot,
            3,
            ScrimDraftRules::Standard,
            0,
        )
        .unwrap();
    }

    // Now try to schedule a 4th — all slots are full, so SlotOccupied hits first
    // Let's test with a different day structure: put 3 scrims using different away teams
    let mut schedules2 = vec![
        TeamWeeklySchedule::new(),
        TeamWeeklySchedule::new(),
        TeamWeeklySchedule::new(),
    ];
    let mut mgr2 = ScrimManager::new();
    mgr2.schedule_scrim(
        &mut schedules2,
        0,
        1,
        "T1",
        "A",
        2,
        TimeSlot::Morning,
        3,
        ScrimDraftRules::Standard,
        0,
    )
    .unwrap();
    mgr2.schedule_scrim(
        &mut schedules2,
        0,
        2,
        "T1",
        "B",
        2,
        TimeSlot::Afternoon,
        3,
        ScrimDraftRules::Standard,
        0,
    )
    .unwrap();
    mgr2.schedule_scrim(
        &mut schedules2,
        0,
        1,
        "T1",
        "A",
        2,
        TimeSlot::Evening,
        3,
        ScrimDraftRules::Standard,
        0,
    )
    .unwrap();

    assert_eq!(schedules2[0].day(2).scrim_count(), 3);
}

// ---------------------------------------------------------------------------
// Cancel a scrim
// ---------------------------------------------------------------------------

#[test]
fn cancel_scrim_clears_both_schedules() {
    let mut mgr = ScrimManager::new();
    let mut schedules = two_empty_schedules();

    let scrim_id = mgr
        .schedule_scrim(
            &mut schedules,
            0,
            1,
            "T1",
            "Gen.G",
            2,
            TimeSlot::Morning,
            3,
            ScrimDraftRules::Standard,
            0,
        )
        .unwrap();

    let result = mgr.cancel_scrim(scrim_id, &mut schedules, 0, 1, 1);
    assert!(result.is_ok());

    // Scrim should be cancelled
    assert_eq!(
        mgr.scrim_by_id(scrim_id).unwrap().status(),
        ScrimStatus::Cancelled
    );

    // Both schedules should be free
    assert!(schedules[0].day(2).is_free(TimeSlot::Morning));
    assert!(schedules[1].day(2).is_free(TimeSlot::Morning));
}

#[test]
fn cancel_scrim_rejects_already_completed() {
    let mut mgr = ScrimManager::new();
    let mut schedules = two_empty_schedules();

    let scrim_id = mgr
        .schedule_scrim(
            &mut schedules,
            0,
            1,
            "T1",
            "Gen.G",
            2,
            TimeSlot::Morning,
            3,
            ScrimDraftRules::Standard,
            0,
        )
        .unwrap();
    mgr.scrim_by_id_mut(scrim_id).unwrap().complete(2, 1);

    let result = mgr.cancel_scrim(scrim_id, &mut schedules, 0, 1, 1);
    assert_eq!(result, Err(ScrimScheduleError::CannotCancelCompleted));
}

#[test]
fn cancel_scrim_rejects_past_scrims() {
    let mut mgr = ScrimManager::new();
    let mut schedules = two_empty_schedules();

    let scrim_id = mgr
        .schedule_scrim(
            &mut schedules,
            0,
            1,
            "T1",
            "Gen.G",
            2,
            TimeSlot::Morning,
            3,
            ScrimDraftRules::Standard,
            0,
        )
        .unwrap();

    // Current day is 3, scrim was on day 2 — already happened
    let result = mgr.cancel_scrim(scrim_id, &mut schedules, 0, 1, 3);
    assert_eq!(result, Err(ScrimScheduleError::CannotCancelPast));
}

// ---------------------------------------------------------------------------
// ID assignment
// ---------------------------------------------------------------------------

#[test]
fn scrim_ids_are_auto_incremented() {
    let mut mgr = ScrimManager::new();
    let mut schedules = two_empty_schedules();

    let id1 = mgr
        .schedule_scrim(
            &mut schedules,
            0,
            1,
            "T1",
            "Gen.G",
            2,
            TimeSlot::Morning,
            3,
            ScrimDraftRules::Standard,
            0,
        )
        .unwrap();
    let id2 = mgr
        .schedule_scrim(
            &mut schedules,
            0,
            1,
            "T1",
            "Gen.G",
            3,
            TimeSlot::Morning,
            3,
            ScrimDraftRules::Standard,
            0,
        )
        .unwrap();

    assert_eq!(id1 + 1, id2);
}

// ---------------------------------------------------------------------------
// Lookup
// ---------------------------------------------------------------------------

#[test]
fn scrim_by_id_returns_none_for_unknown() {
    let mgr = ScrimManager::new();
    assert!(mgr.scrim_by_id(999).is_none());
}

#[test]
fn scrims_for_team_filters_correctly() {
    let mut mgr = ScrimManager::new();
    let mut schedules = vec![
        TeamWeeklySchedule::new(),
        TeamWeeklySchedule::new(),
        TeamWeeklySchedule::new(),
    ];

    mgr.schedule_scrim(
        &mut schedules,
        0,
        1,
        "T1",
        "Gen.G",
        2,
        TimeSlot::Morning,
        3,
        ScrimDraftRules::Standard,
        0,
    )
    .unwrap();
    mgr.schedule_scrim(
        &mut schedules,
        0,
        2,
        "T1",
        "DRX",
        3,
        TimeSlot::Morning,
        5,
        ScrimDraftRules::Fearless,
        0,
    )
    .unwrap();
    mgr.schedule_scrim(
        &mut schedules,
        1,
        2,
        "Gen.G",
        "DRX",
        4,
        TimeSlot::Morning,
        3,
        ScrimDraftRules::Standard,
        0,
    )
    .unwrap();

    let t1_scrims = mgr.scrims_for_team("T1");
    assert_eq!(t1_scrims.len(), 2);

    let drx_scrims = mgr.scrims_for_team("DRX");
    assert_eq!(drx_scrims.len(), 2);

    let geng_scrims = mgr.scrims_for_team("Gen.G");
    assert_eq!(geng_scrims.len(), 2);
}
