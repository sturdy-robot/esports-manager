use esm_engine::schedule::scrim::{Scrim, ScrimDraftRules, ScrimStatus};
use esm_models::time::TimeSlot;

// ---------------------------------------------------------------------------
// Scrim creation & defaults
// ---------------------------------------------------------------------------

#[test]
fn scrim_creation_stores_fields() {
    let scrim = Scrim::new(
        1,
        "T1".to_string(),
        "Gen.G".to_string(),
        5,
        3,
        TimeSlot::Morning,
        ScrimDraftRules::Standard,
    );
    assert_eq!(scrim.id(), 1);
    assert_eq!(scrim.home_team(), "T1");
    assert_eq!(scrim.away_team(), "Gen.G");
    assert_eq!(scrim.scheduled_day(), 5);
    assert_eq!(scrim.game_count(), 3);
    assert_eq!(scrim.time_slot(), TimeSlot::Morning);
    assert_eq!(scrim.draft_rules(), ScrimDraftRules::Standard);
    assert_eq!(scrim.status(), ScrimStatus::Scheduled);
}

#[test]
fn scrim_default_status_is_scheduled() {
    let scrim = Scrim::new(
        2,
        "DRX".to_string(),
        "KT".to_string(),
        10,
        5,
        TimeSlot::Afternoon,
        ScrimDraftRules::Fearless,
    );
    assert_eq!(scrim.status(), ScrimStatus::Scheduled);
}

// ---------------------------------------------------------------------------
// ScrimDraftRules variants
// ---------------------------------------------------------------------------

#[test]
fn scrim_draft_rules_standard_and_fearless() {
    let _ = ScrimDraftRules::Standard;
    let _ = ScrimDraftRules::Fearless;
}

// ---------------------------------------------------------------------------
// ScrimStatus transitions
// ---------------------------------------------------------------------------

#[test]
fn scrim_can_be_completed() {
    let mut scrim = Scrim::new(
        3,
        "T1".to_string(),
        "Gen.G".to_string(),
        5,
        3,
        TimeSlot::Evening,
        ScrimDraftRules::Standard,
    );
    scrim.complete(2, 1);
    assert_eq!(scrim.status(), ScrimStatus::Completed);
    assert_eq!(scrim.home_wins(), 2);
    assert_eq!(scrim.away_wins(), 1);
}

#[test]
fn scrim_can_be_cancelled() {
    let mut scrim = Scrim::new(
        4,
        "DRX".to_string(),
        "KT".to_string(),
        8,
        5,
        TimeSlot::Morning,
        ScrimDraftRules::Fearless,
    );
    scrim.cancel();
    assert_eq!(scrim.status(), ScrimStatus::Cancelled);
}

#[test]
fn scrim_completed_wins_sum_to_game_count() {
    let mut scrim = Scrim::new(
        5,
        "T1".to_string(),
        "Gen.G".to_string(),
        5,
        5,
        TimeSlot::Afternoon,
        ScrimDraftRules::Standard,
    );
    scrim.complete(3, 2);
    assert_eq!(scrim.home_wins() + scrim.away_wins(), scrim.game_count());
}

#[test]
fn scrim_game_count_typical_values() {
    let s3 = Scrim::new(1, "A".into(), "B".into(), 1, 3, TimeSlot::Morning, ScrimDraftRules::Standard);
    let s5 = Scrim::new(2, "A".into(), "B".into(), 1, 5, TimeSlot::Morning, ScrimDraftRules::Standard);
    assert_eq!(s3.game_count(), 3);
    assert_eq!(s5.game_count(), 5);
}

// ---------------------------------------------------------------------------
// Scrim involves_team helper
// ---------------------------------------------------------------------------

#[test]
fn scrim_involves_team_matches_home() {
    let scrim = Scrim::new(1, "T1".into(), "Gen.G".into(), 1, 3, TimeSlot::Morning, ScrimDraftRules::Standard);
    assert!(scrim.involves_team("T1"));
}

#[test]
fn scrim_involves_team_matches_away() {
    let scrim = Scrim::new(1, "T1".into(), "Gen.G".into(), 1, 3, TimeSlot::Morning, ScrimDraftRules::Standard);
    assert!(scrim.involves_team("Gen.G"));
}

#[test]
fn scrim_involves_team_returns_false_for_unrelated() {
    let scrim = Scrim::new(1, "T1".into(), "Gen.G".into(), 1, 3, TimeSlot::Morning, ScrimDraftRules::Standard);
    assert!(!scrim.involves_team("DRX"));
}
