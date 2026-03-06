use esm_engine::schedule::{
    ScheduleEntry, SoloQueueFocus, TeamDailySchedule, TeamWeeklySchedule,
};
use esm_models::time::TimeSlot;

// ---------------------------------------------------------------------------
// TeamDailySchedule
// ---------------------------------------------------------------------------

#[test]
fn daily_schedule_starts_all_free() {
    let ds = TeamDailySchedule::new();
    for slot in TimeSlot::ALL {
        assert!(ds.is_free(slot));
        assert!(ds.get(slot).is_none());
    }
}

#[test]
fn daily_schedule_set_scrim() {
    let mut ds = TeamDailySchedule::new();
    ds.set(TimeSlot::Morning, ScheduleEntry::Scrim { scrim_id: 1 });
    assert!(!ds.is_free(TimeSlot::Morning));
    assert_eq!(
        ds.get(TimeSlot::Morning),
        Some(&ScheduleEntry::Scrim { scrim_id: 1 })
    );
}

#[test]
fn daily_schedule_set_solo_queue() {
    let mut ds = TeamDailySchedule::new();
    ds.set(
        TimeSlot::Afternoon,
        ScheduleEntry::SoloQueue {
            players: vec![0, 2, 4],
            focus: SoloQueueFocus::Mechanics,
        },
    );
    assert!(!ds.is_free(TimeSlot::Afternoon));
    match ds.get(TimeSlot::Afternoon) {
        Some(ScheduleEntry::SoloQueue { players, focus }) => {
            assert_eq!(players, &[0, 2, 4]);
            assert_eq!(*focus, SoloQueueFocus::Mechanics);
        }
        _ => panic!("Expected SoloQueue entry"),
    }
}

#[test]
fn daily_schedule_set_rest() {
    let mut ds = TeamDailySchedule::new();
    ds.set(TimeSlot::Evening, ScheduleEntry::Rest);
    assert_eq!(ds.get(TimeSlot::Evening), Some(&ScheduleEntry::Rest));
}

#[test]
fn daily_schedule_clear_slot() {
    let mut ds = TeamDailySchedule::new();
    ds.set(TimeSlot::Morning, ScheduleEntry::Scrim { scrim_id: 5 });
    ds.clear(TimeSlot::Morning);
    assert!(ds.is_free(TimeSlot::Morning));
}

#[test]
fn daily_schedule_scrim_count_empty() {
    let ds = TeamDailySchedule::new();
    assert_eq!(ds.scrim_count(), 0);
}

#[test]
fn daily_schedule_scrim_count_mixed() {
    let mut ds = TeamDailySchedule::new();
    ds.set(TimeSlot::Morning, ScheduleEntry::Scrim { scrim_id: 1 });
    ds.set(
        TimeSlot::Afternoon,
        ScheduleEntry::SoloQueue {
            players: vec![0],
            focus: SoloQueueFocus::Champions,
        },
    );
    ds.set(TimeSlot::Evening, ScheduleEntry::Scrim { scrim_id: 2 });
    assert_eq!(ds.scrim_count(), 2);
}

#[test]
fn daily_schedule_max_three_scrims() {
    let mut ds = TeamDailySchedule::new();
    ds.set(TimeSlot::Morning, ScheduleEntry::Scrim { scrim_id: 1 });
    ds.set(TimeSlot::Afternoon, ScheduleEntry::Scrim { scrim_id: 2 });
    ds.set(TimeSlot::Evening, ScheduleEntry::Scrim { scrim_id: 3 });
    assert_eq!(ds.scrim_count(), 3);
}

// ---------------------------------------------------------------------------
// TeamWeeklySchedule
// ---------------------------------------------------------------------------

#[test]
fn weekly_schedule_starts_empty() {
    let ws = TeamWeeklySchedule::new();
    assert_eq!(ws.total_scrims(), 0);
    assert_eq!(ws.occupied_slots(), 0);
    assert_eq!(ws.total_slots(), 21); // 3 * 7
}

#[test]
fn weekly_schedule_set_entry_on_day() {
    let mut ws = TeamWeeklySchedule::new();
    ws.day_mut(0).set(TimeSlot::Morning, ScheduleEntry::Scrim { scrim_id: 10 });
    assert_eq!(ws.total_scrims(), 1);
    assert_eq!(ws.occupied_slots(), 1);
}

#[test]
fn weekly_schedule_total_scrims_across_days() {
    let mut ws = TeamWeeklySchedule::new();
    ws.day_mut(0).set(TimeSlot::Morning, ScheduleEntry::Scrim { scrim_id: 1 });
    ws.day_mut(2).set(TimeSlot::Afternoon, ScheduleEntry::Scrim { scrim_id: 2 });
    ws.day_mut(5).set(TimeSlot::Evening, ScheduleEntry::Scrim { scrim_id: 3 });
    assert_eq!(ws.total_scrims(), 3);
}

#[test]
fn weekly_schedule_occupied_slots_counts_all_entry_types() {
    let mut ws = TeamWeeklySchedule::new();
    ws.day_mut(0).set(TimeSlot::Morning, ScheduleEntry::Scrim { scrim_id: 1 });
    ws.day_mut(0).set(
        TimeSlot::Afternoon,
        ScheduleEntry::SoloQueue {
            players: vec![0, 1],
            focus: SoloQueueFocus::Tactics,
        },
    );
    ws.day_mut(1).set(TimeSlot::Morning, ScheduleEntry::Rest);
    assert_eq!(ws.occupied_slots(), 3);
}

#[test]
fn weekly_schedule_day_access_is_independent() {
    let mut ws = TeamWeeklySchedule::new();
    ws.day_mut(3).set(TimeSlot::Evening, ScheduleEntry::Rest);
    assert!(ws.day(0).is_free(TimeSlot::Evening));
    assert!(!ws.day(3).is_free(TimeSlot::Evening));
}

#[test]
fn weekly_schedule_seven_days() {
    let ws = TeamWeeklySchedule::new();
    assert_eq!(ws.days().len(), 7);
}

// ---------------------------------------------------------------------------
// SoloQueueFocus variants
// ---------------------------------------------------------------------------

#[test]
fn solo_queue_focus_variants_exist() {
    let _ = SoloQueueFocus::Champions;
    let _ = SoloQueueFocus::Tactics;
    let _ = SoloQueueFocus::Mechanics;
    let _ = SoloQueueFocus::Mentality;
}
