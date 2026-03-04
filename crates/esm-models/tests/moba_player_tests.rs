use esm_models::moba::player::{MobaPlayer, MobaPlayerAttributes, MobaRole, RoleAssignment};
use esm_models::player::{BoundedAttribute, Confidence};

// ---------------------------------------------------------------------------
// MobaRole
// ---------------------------------------------------------------------------

#[test]
fn moba_role_variants_exist() {
    let _ = MobaRole::Top;
    let _ = MobaRole::Jungle;
    let _ = MobaRole::Mid;
    let _ = MobaRole::Bot;
    let _ = MobaRole::Support;
}

// ---------------------------------------------------------------------------
// RoleAssignment — primary + secondary roles
// ---------------------------------------------------------------------------

#[test]
fn role_assignment_primary_only() {
    let ra = RoleAssignment::new(MobaRole::Mid, vec![]);
    assert_eq!(ra.primary(), MobaRole::Mid);
    assert!(ra.secondary().is_empty());
}

#[test]
fn role_assignment_with_secondary_roles() {
    let ra = RoleAssignment::new(MobaRole::Mid, vec![MobaRole::Top, MobaRole::Support]);
    assert_eq!(ra.primary(), MobaRole::Mid);
    assert_eq!(ra.secondary().len(), 2);
    assert!(ra.secondary().contains(&MobaRole::Top));
    assert!(ra.secondary().contains(&MobaRole::Support));
}

#[test]
fn role_assignment_can_play_primary() {
    let ra = RoleAssignment::new(MobaRole::Mid, vec![MobaRole::Top]);
    assert!(ra.can_play(MobaRole::Mid));
}

#[test]
fn role_assignment_can_play_secondary() {
    let ra = RoleAssignment::new(MobaRole::Mid, vec![MobaRole::Top]);
    assert!(ra.can_play(MobaRole::Top));
}

#[test]
fn role_assignment_cannot_play_unassigned() {
    let ra = RoleAssignment::new(MobaRole::Mid, vec![MobaRole::Top]);
    assert!(!ra.can_play(MobaRole::Jungle));
}

#[test]
fn role_assignment_skill_modifier_primary_is_1() {
    let ra = RoleAssignment::new(MobaRole::Mid, vec![MobaRole::Top]);
    assert!((ra.skill_modifier(MobaRole::Mid) - 1.0).abs() < f64::EPSILON);
}

#[test]
fn role_assignment_skill_modifier_secondary_is_reduced() {
    let ra = RoleAssignment::new(MobaRole::Mid, vec![MobaRole::Top]);
    let modifier = ra.skill_modifier(MobaRole::Top);
    assert!(
        modifier < 1.0,
        "Secondary role modifier should be < 1.0, got {modifier}"
    );
    assert!(
        modifier >= 0.5,
        "Secondary role modifier should be >= 0.5, got {modifier}"
    );
}

#[test]
fn role_assignment_skill_modifier_unassigned_is_heavily_penalized() {
    let ra = RoleAssignment::new(MobaRole::Mid, vec![]);
    let modifier = ra.skill_modifier(MobaRole::Jungle);
    assert!(
        modifier < 0.7,
        "Unassigned role modifier should be < 0.7, got {modifier}"
    );
}

// ---------------------------------------------------------------------------
// MobaPlayerAttributes (MOBA-specific)
// ---------------------------------------------------------------------------

#[test]
fn moba_player_attributes_constructed() {
    let attrs = MobaPlayerAttributes {
        endurance: BoundedAttribute::new(70),
        reaction_time: BoundedAttribute::new(90),
        decision_making: BoundedAttribute::new(95),
        clutch: BoundedAttribute::new(99),
        discipline: BoundedAttribute::new(85),
        tilt_resistance: BoundedAttribute::new(80),
        mechanics: BoundedAttribute::new(97),
        vision_control: BoundedAttribute::new(88),
        teamfighting: BoundedAttribute::new(92),
    };
    assert_eq!(attrs.mechanics.value(), 97);
    assert_eq!(attrs.clutch.value(), 99);
}

// ---------------------------------------------------------------------------
// MobaPlayer entity
// ---------------------------------------------------------------------------

fn make_test_player() -> MobaPlayer {
    MobaPlayer::new(
        "Faker".to_string(),
        "Lee".to_string(),
        "Sang-hyeok".to_string(),
        RoleAssignment::new(MobaRole::Mid, vec![MobaRole::Support]),
        MobaPlayerAttributes {
            endurance: BoundedAttribute::new(70),
            reaction_time: BoundedAttribute::new(90),
            decision_making: BoundedAttribute::new(95),
            clutch: BoundedAttribute::new(99),
            discipline: BoundedAttribute::new(85),
            tilt_resistance: BoundedAttribute::new(80),
            mechanics: BoundedAttribute::new(97),
            vision_control: BoundedAttribute::new(88),
            teamfighting: BoundedAttribute::new(92),
        },
    )
}

#[test]
fn moba_player_stores_identity() {
    let p = make_test_player();
    assert_eq!(p.nickname(), "Faker");
    assert_eq!(p.first_name(), "Lee");
    assert_eq!(p.last_name(), "Sang-hyeok");
}

#[test]
fn moba_player_has_role_assignment() {
    let p = make_test_player();
    assert_eq!(p.roles().primary(), MobaRole::Mid);
    assert!(p.roles().can_play(MobaRole::Support));
    assert!(!p.roles().can_play(MobaRole::Jungle));
}

#[test]
fn moba_player_has_default_state() {
    let p = make_test_player();
    assert_eq!(p.state().stamina.value(), 100);
    assert_eq!(p.state().morale.value(), 50);
    assert_eq!(p.state().confidence, Confidence::Neutral);
}

#[test]
fn moba_player_attributes_accessible() {
    let p = make_test_player();
    assert_eq!(p.attributes().mechanics.value(), 97);
    assert_eq!(p.attributes().vision_control.value(), 88);
}

#[test]
fn moba_player_state_is_mutable() {
    let mut p = make_test_player();
    p.state_mut().stamina.decrease(25);
    assert_eq!(p.state().stamina.value(), 75);
}

#[test]
fn moba_player_effective_skill_on_primary_role() {
    let p = make_test_player();
    let base = p.attributes().mechanics.value();
    let effective = p.effective_attribute(p.attributes().mechanics, MobaRole::Mid);
    assert_eq!(
        effective, base,
        "On primary role, effective should equal base"
    );
}

#[test]
fn moba_player_effective_skill_on_secondary_role_is_reduced() {
    let p = make_test_player();
    let base = p.attributes().mechanics.value();
    let effective = p.effective_attribute(p.attributes().mechanics, MobaRole::Support);
    assert!(
        effective < base,
        "On secondary role, effective ({effective}) should be < base ({base})"
    );
}

#[test]
fn moba_player_effective_skill_on_unassigned_role_heavily_reduced() {
    let p = make_test_player();
    let base = p.attributes().mechanics.value();
    let effective = p.effective_attribute(p.attributes().mechanics, MobaRole::Jungle);
    assert!(
        effective < base,
        "On unassigned role, effective ({effective}) should be < base ({base})"
    );
    let secondary_effective = p.effective_attribute(p.attributes().mechanics, MobaRole::Support);
    assert!(
        effective < secondary_effective,
        "Unassigned ({effective}) should be < secondary ({secondary_effective})"
    );
}
