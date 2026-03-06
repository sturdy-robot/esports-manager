use esm_models::activity::{Activity, ActivityScheduler, DailySchedule};

// ---------------------------------------------------------------------------
// Activity enum & effects
// ---------------------------------------------------------------------------

#[test]
fn activity_variants_exist() {
    let _ = Activity::ScrimBlock;
    let _ = Activity::SoloQueue;
    let _ = Activity::RestDay;
}

#[test]
fn scrim_block_has_high_stamina_cost() {
    let effect = Activity::ScrimBlock.effect();
    assert!(effect.stamina_cost >= 15 && effect.stamina_cost <= 20);
}

#[test]
fn solo_queue_has_low_stamina_cost() {
    let effect = Activity::SoloQueue.effect();
    assert!(effect.stamina_cost >= 5 && effect.stamina_cost <= 10);
}

#[test]
fn rest_day_has_positive_stamina_recovery() {
    let effect = Activity::RestDay.effect();
    assert_eq!(effect.stamina_cost, 0);
    assert_eq!(effect.stamina_recovery, 30);
}

#[test]
fn scrim_block_has_high_mastery_gain() {
    let scrim = Activity::ScrimBlock.effect();
    let solo = Activity::SoloQueue.effect();
    assert!(scrim.mastery_gain > solo.mastery_gain);
}

#[test]
fn rest_day_has_no_mastery_gain() {
    let effect = Activity::RestDay.effect();
    assert_eq!(effect.mastery_gain, 0);
}

#[test]
fn scrim_block_has_attribute_growth() {
    let effect = Activity::ScrimBlock.effect();
    assert!(effect.attribute_growth > 0);
}

#[test]
fn rest_day_has_no_attribute_growth() {
    let effect = Activity::RestDay.effect();
    assert_eq!(effect.attribute_growth, 0);
}

// ---------------------------------------------------------------------------
// DailySchedule
// ---------------------------------------------------------------------------

#[test]
fn daily_schedule_has_four_slots() {
    let schedule = DailySchedule::new();
    assert_eq!(schedule.slots().len(), 4);
}

#[test]
fn daily_schedule_starts_empty() {
    let schedule = DailySchedule::new();
    for slot in schedule.slots() {
        assert!(slot.is_none());
    }
}

#[test]
fn daily_schedule_set_slot() {
    let mut schedule = DailySchedule::new();
    assert!(schedule.set_slot(0, Activity::ScrimBlock));
    assert_eq!(schedule.slots()[0], Some(Activity::ScrimBlock));
}

#[test]
fn daily_schedule_set_slot_out_of_bounds_returns_false() {
    let mut schedule = DailySchedule::new();
    assert!(!schedule.set_slot(4, Activity::ScrimBlock));
}

#[test]
fn daily_schedule_clear_slot() {
    let mut schedule = DailySchedule::new();
    schedule.set_slot(0, Activity::ScrimBlock);
    schedule.clear_slot(0);
    assert!(schedule.slots()[0].is_none());
}

#[test]
fn daily_schedule_fill_all_slots() {
    let mut schedule = DailySchedule::new();
    schedule.set_slot(0, Activity::ScrimBlock);
    schedule.set_slot(1, Activity::ScrimBlock);
    schedule.set_slot(2, Activity::SoloQueue);
    schedule.set_slot(3, Activity::RestDay);

    assert_eq!(schedule.slots()[0], Some(Activity::ScrimBlock));
    assert_eq!(schedule.slots()[1], Some(Activity::ScrimBlock));
    assert_eq!(schedule.slots()[2], Some(Activity::SoloQueue));
    assert_eq!(schedule.slots()[3], Some(Activity::RestDay));
}

// ---------------------------------------------------------------------------
// ActivityScheduler: compute total effects for a day
// ---------------------------------------------------------------------------

#[test]
fn scheduler_computes_total_stamina_cost() {
    let mut schedule = DailySchedule::new();
    schedule.set_slot(0, Activity::ScrimBlock);
    schedule.set_slot(1, Activity::SoloQueue);

    let total = ActivityScheduler::compute_daily_effect(&schedule);
    let expected_min = 15 + 5; // min scrim + min solo
    assert!(total.stamina_cost >= expected_min);
}

#[test]
fn scheduler_empty_schedule_has_zero_effect() {
    let schedule = DailySchedule::new();
    let total = ActivityScheduler::compute_daily_effect(&schedule);
    assert_eq!(total.stamina_cost, 0);
    assert_eq!(total.stamina_recovery, 0);
    assert_eq!(total.mastery_gain, 0);
    assert_eq!(total.attribute_growth, 0);
}

#[test]
fn scheduler_rest_day_only_gives_recovery() {
    let mut schedule = DailySchedule::new();
    schedule.set_slot(0, Activity::RestDay);

    let total = ActivityScheduler::compute_daily_effect(&schedule);
    assert_eq!(total.stamina_cost, 0);
    assert_eq!(total.stamina_recovery, 30);
    assert_eq!(total.mastery_gain, 0);
}

#[test]
fn scheduler_mixed_schedule_accumulates() {
    let mut schedule = DailySchedule::new();
    schedule.set_slot(0, Activity::ScrimBlock);
    schedule.set_slot(1, Activity::RestDay);

    let total = ActivityScheduler::compute_daily_effect(&schedule);
    // Should have both cost from scrim and recovery from rest
    assert!(total.stamina_cost > 0);
    assert!(total.stamina_recovery > 0);
    assert!(total.mastery_gain > 0);
}
