use esm_models::staff::{Staff, StaffRole};

// ---------------------------------------------------------------------------
// StaffRole enum
// ---------------------------------------------------------------------------

#[test]
fn staff_role_variants_exist() {
    let _ = StaffRole::AssistantCoach;
    let _ = StaffRole::DraftAnalyst;
    let _ = StaffRole::PositionalCoach;
    let _ = StaffRole::SportsPsychologist;
    let _ = StaffRole::Scout;
    let _ = StaffRole::FinancialOfficer;
}

// ---------------------------------------------------------------------------
// Staff entity
// ---------------------------------------------------------------------------

fn make_test_staff() -> Staff {
    Staff::new(
        "Edgar".to_string(),
        "Lee".to_string(),
        "Edgar".to_string(),
        StaffRole::AssistantCoach,
        75,
    )
}

#[test]
fn staff_creation_stores_identity() {
    let staff = make_test_staff();
    assert_eq!(staff.nickname(), "Edgar");
    assert_eq!(staff.first_name(), "Lee");
    assert_eq!(staff.last_name(), "Edgar");
}

#[test]
fn staff_creation_stores_role() {
    let staff = make_test_staff();
    assert_eq!(staff.role(), StaffRole::AssistantCoach);
}

#[test]
fn staff_has_skill_attribute() {
    let staff = make_test_staff();
    assert_eq!(staff.skill().value(), 75);
}

#[test]
fn staff_skill_can_increase() {
    let mut staff = make_test_staff();
    staff.skill_mut().increase(10);
    assert_eq!(staff.skill().value(), 85);
}

#[test]
fn staff_skill_clamps_at_100() {
    let mut staff = make_test_staff();
    staff.skill_mut().increase(50);
    assert_eq!(staff.skill().value(), 100);
}

#[test]
fn positional_coach_creation() {
    let staff = Staff::new(
        "MidCoach".to_string(),
        "Park".to_string(),
        "Jin".to_string(),
        StaffRole::PositionalCoach,
        60,
    );
    assert_eq!(staff.role(), StaffRole::PositionalCoach);
    assert_eq!(staff.skill().value(), 60);
}

#[test]
fn scout_creation() {
    let staff = Staff::new(
        "ScoutMaster".to_string(),
        "Kim".to_string(),
        "Soo".to_string(),
        StaffRole::Scout,
        90,
    );
    assert_eq!(staff.role(), StaffRole::Scout);
    assert_eq!(staff.skill().value(), 90);
}
