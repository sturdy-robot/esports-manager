use esm_engine::staff_influence::StaffInfluence;
use esm_models::staff::{Staff, StaffRole};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_staff(role: StaffRole, skill: u8) -> Staff {
    Staff::new(
        "TestStaff".to_string(),
        "First".to_string(),
        "Last".to_string(),
        role,
        skill,
    )
}

// ---------------------------------------------------------------------------
// Assistant Coach: delegation quality multiplier
// ---------------------------------------------------------------------------

#[test]
fn assistant_coach_delegation_multiplier_scales_with_skill() {
    let low = make_staff(StaffRole::AssistantCoach, 30);
    let high = make_staff(StaffRole::AssistantCoach, 90);

    let low_mult = StaffInfluence::delegation_quality(&low);
    let high_mult = StaffInfluence::delegation_quality(&high);

    assert!(high_mult > low_mult);
    assert!(low_mult >= 0.5);
    assert!(high_mult <= 1.5);
}

#[test]
fn assistant_coach_delegation_multiplier_at_zero_skill() {
    let staff = make_staff(StaffRole::AssistantCoach, 0);
    let mult = StaffInfluence::delegation_quality(&staff);
    assert!((mult - 0.5).abs() < f64::EPSILON);
}

#[test]
fn assistant_coach_delegation_multiplier_at_max_skill() {
    let staff = make_staff(StaffRole::AssistantCoach, 100);
    let mult = StaffInfluence::delegation_quality(&staff);
    assert!((mult - 1.5).abs() < f64::EPSILON);
}

// ---------------------------------------------------------------------------
// Positional Coach: training growth multiplier
// ---------------------------------------------------------------------------

#[test]
fn positional_coach_training_multiplier_scales_with_skill() {
    let low = make_staff(StaffRole::PositionalCoach, 20);
    let high = make_staff(StaffRole::PositionalCoach, 80);

    let low_mult = StaffInfluence::training_growth_multiplier(&low);
    let high_mult = StaffInfluence::training_growth_multiplier(&high);

    assert!(high_mult > low_mult);
    assert!(low_mult >= 1.0);
}

#[test]
fn positional_coach_training_multiplier_at_zero() {
    let staff = make_staff(StaffRole::PositionalCoach, 0);
    let mult = StaffInfluence::training_growth_multiplier(&staff);
    assert!((mult - 1.0).abs() < f64::EPSILON);
}

#[test]
fn positional_coach_training_multiplier_at_max() {
    let staff = make_staff(StaffRole::PositionalCoach, 100);
    let mult = StaffInfluence::training_growth_multiplier(&staff);
    assert!((mult - 1.5).abs() < f64::EPSILON);
}

// ---------------------------------------------------------------------------
// Sports Psychologist: morale loss buffer and stamina penalty reduction
// ---------------------------------------------------------------------------

#[test]
fn psychologist_morale_buffer_scales_with_skill() {
    let low = make_staff(StaffRole::SportsPsychologist, 20);
    let high = make_staff(StaffRole::SportsPsychologist, 90);

    let low_buf = StaffInfluence::morale_loss_buffer(&low);
    let high_buf = StaffInfluence::morale_loss_buffer(&high);

    // Higher skill = more morale loss absorbed (returned as reduction %)
    assert!(high_buf > low_buf);
    assert!(low_buf >= 0.0);
    assert!(high_buf <= 0.5); // Max 50% reduction
}

#[test]
fn psychologist_stamina_penalty_reduction() {
    let low = make_staff(StaffRole::SportsPsychologist, 20);
    let high = make_staff(StaffRole::SportsPsychologist, 90);

    let low_red = StaffInfluence::stamina_penalty_reduction(&low);
    let high_red = StaffInfluence::stamina_penalty_reduction(&high);

    assert!(high_red > low_red);
    assert!(low_red >= 0.0);
    assert!(high_red <= 0.3); // Max 30% reduction
}

// ---------------------------------------------------------------------------
// Scout: scouting accuracy
// ---------------------------------------------------------------------------

#[test]
fn scout_accuracy_scales_with_skill() {
    let low = make_staff(StaffRole::Scout, 20);
    let high = make_staff(StaffRole::Scout, 95);

    let low_acc = StaffInfluence::scouting_accuracy(&low);
    let high_acc = StaffInfluence::scouting_accuracy(&high);

    assert!(high_acc > low_acc);
    assert!(low_acc >= 0.2);
    assert!(high_acc <= 1.0);
}

// ---------------------------------------------------------------------------
// Non-matching roles return neutral values
// ---------------------------------------------------------------------------

#[test]
fn delegation_quality_for_non_coach_returns_neutral() {
    let scout = make_staff(StaffRole::Scout, 90);
    let mult = StaffInfluence::delegation_quality(&scout);
    assert!((mult - 1.0).abs() < f64::EPSILON);
}

#[test]
fn training_multiplier_for_non_positional_coach_returns_neutral() {
    let analyst = make_staff(StaffRole::DraftAnalyst, 90);
    let mult = StaffInfluence::training_growth_multiplier(&analyst);
    assert!((mult - 1.0).abs() < f64::EPSILON);
}

#[test]
fn morale_buffer_for_non_psychologist_returns_zero() {
    let coach = make_staff(StaffRole::AssistantCoach, 90);
    let buf = StaffInfluence::morale_loss_buffer(&coach);
    assert!((buf - 0.0).abs() < f64::EPSILON);
}
