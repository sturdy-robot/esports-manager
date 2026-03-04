use std::str::FromStr;

use esm_core::calendar::{Calendar, DayPhase};

#[test]
fn new_calendar_starts_at_given_date() {
    let cal = Calendar::new(2025, 1, 1);
    assert_eq!(cal.year(), 2025);
    assert_eq!(cal.month(), 1);
    assert_eq!(cal.day(), 1);
    assert_eq!(cal.days_elapsed(), 0);
}

#[test]
fn new_calendar_starts_in_morning_phase() {
    let cal = Calendar::new(2025, 1, 1);
    assert_eq!(cal.phase(), DayPhase::Morning);
}

#[test]
fn advance_phase_progresses_morning_to_afternoon() {
    let mut cal = Calendar::new(2025, 1, 1);
    cal.advance_phase();
    assert_eq!(cal.phase(), DayPhase::Afternoon);
    assert_eq!(cal.day(), 1);
    assert_eq!(cal.days_elapsed(), 0);
}

#[test]
fn advance_phase_progresses_afternoon_to_evening() {
    let mut cal = Calendar::new(2025, 1, 1);
    cal.advance_phase(); // Morning -> Afternoon
    cal.advance_phase(); // Afternoon -> Evening
    assert_eq!(cal.phase(), DayPhase::Evening);
    assert_eq!(cal.day(), 1);
    assert_eq!(cal.days_elapsed(), 0);
}

#[test]
fn advance_phase_from_evening_starts_new_day() {
    let mut cal = Calendar::new(2025, 1, 1);
    cal.advance_phase(); // Morning -> Afternoon
    cal.advance_phase(); // Afternoon -> Evening
    cal.advance_phase(); // Evening -> Morning (next day)
    assert_eq!(cal.phase(), DayPhase::Morning);
    assert_eq!(cal.day(), 2);
    assert_eq!(cal.days_elapsed(), 1);
}

#[test]
fn advance_day_skips_all_phases_and_moves_to_next_day() {
    let mut cal = Calendar::new(2025, 1, 1);
    cal.advance_day();
    assert_eq!(cal.day(), 2);
    assert_eq!(cal.phase(), DayPhase::Morning);
    assert_eq!(cal.days_elapsed(), 1);
}

#[test]
fn advance_day_from_mid_phase_resets_to_morning() {
    let mut cal = Calendar::new(2025, 1, 1);
    cal.advance_phase(); // Morning -> Afternoon
    cal.advance_day();
    assert_eq!(cal.day(), 2);
    assert_eq!(cal.phase(), DayPhase::Morning);
    assert_eq!(cal.days_elapsed(), 1);
}

#[test]
fn advance_day_rolls_over_month() {
    let mut cal = Calendar::new(2025, 1, 31);
    cal.advance_day();
    assert_eq!(cal.month(), 2);
    assert_eq!(cal.day(), 1);
}

#[test]
fn advance_day_rolls_over_year() {
    let mut cal = Calendar::new(2025, 12, 31);
    cal.advance_day();
    assert_eq!(cal.year(), 2026);
    assert_eq!(cal.month(), 1);
    assert_eq!(cal.day(), 1);
}

#[test]
fn february_has_28_days_in_non_leap_year() {
    let mut cal = Calendar::new(2025, 2, 28);
    cal.advance_day();
    assert_eq!(cal.month(), 3);
    assert_eq!(cal.day(), 1);
}

#[test]
fn february_has_29_days_in_leap_year() {
    let mut cal = Calendar::new(2024, 2, 28);
    cal.advance_day();
    assert_eq!(cal.month(), 2);
    assert_eq!(cal.day(), 29);

    cal.advance_day();
    assert_eq!(cal.month(), 3);
    assert_eq!(cal.day(), 1);
}

#[test]
fn is_weekly_tick_fires_every_7_days() {
    let mut cal = Calendar::new(2025, 1, 1);
    assert!(!cal.is_weekly_tick(), "Day 0 should not be a weekly tick");

    for _ in 0..7 {
        cal.advance_day();
    }
    assert!(cal.is_weekly_tick(), "Day 7 should be a weekly tick");
    assert_eq!(cal.days_elapsed(), 7);

    for _ in 0..7 {
        cal.advance_day();
    }
    assert!(cal.is_weekly_tick(), "Day 14 should be a weekly tick");
}

#[test]
fn is_weekly_tick_does_not_fire_on_non_7th_day() {
    let mut cal = Calendar::new(2025, 1, 1);
    for _ in 0..5 {
        cal.advance_day();
    }
    assert!(!cal.is_weekly_tick(), "Day 5 should not be a weekly tick");
}

#[test]
fn multiple_days_accumulate_correctly() {
    let mut cal = Calendar::new(2025, 1, 1);
    for _ in 0..30 {
        cal.advance_day();
    }
    assert_eq!(cal.days_elapsed(), 30);
    assert_eq!(cal.month(), 1);
    assert_eq!(cal.day(), 31);
}

#[test]
fn day_phase_ordering() {
    assert!(DayPhase::Morning < DayPhase::Afternoon);
    assert!(DayPhase::Afternoon < DayPhase::Evening);
}

// ---------------------------------------------------------------------------
// DayPhase: string conversions
// ---------------------------------------------------------------------------

#[test]
fn day_phase_as_str() {
    assert_eq!(DayPhase::Morning.as_str(), "Morning");
    assert_eq!(DayPhase::Afternoon.as_str(), "Afternoon");
    assert_eq!(DayPhase::Evening.as_str(), "Evening");
}

#[test]
fn day_phase_display() {
    assert_eq!(format!("{}", DayPhase::Morning), "Morning");
    assert_eq!(format!("{}", DayPhase::Evening), "Evening");
}

#[test]
fn day_phase_from_str_valid() {
    assert_eq!(DayPhase::from_str("Morning").unwrap(), DayPhase::Morning);
    assert_eq!(
        DayPhase::from_str("Afternoon").unwrap(),
        DayPhase::Afternoon
    );
    assert_eq!(DayPhase::from_str("Evening").unwrap(), DayPhase::Evening);
}

#[test]
fn day_phase_from_str_invalid() {
    assert!(DayPhase::from_str("Midnight").is_err());
    assert!(DayPhase::from_str("").is_err());
}
