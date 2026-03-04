use esm_models::manager::{Manager, ManagerArchetype};

// ---------------------------------------------------------------------------
// ManagerArchetype enum
// ---------------------------------------------------------------------------

#[test]
fn archetype_variants_exist() {
    let _ = ManagerArchetype::TacticalGenius;
    let _ = ManagerArchetype::PlayerDeveloper;
    let _ = ManagerArchetype::Motivator;
    let _ = ManagerArchetype::Analyst;
    let _ = ManagerArchetype::Balanced;
}

// ---------------------------------------------------------------------------
// Manager entity
// ---------------------------------------------------------------------------

fn make_test_manager() -> Manager {
    Manager::new(
        "kkOma".to_string(),
        "Kim".to_string(),
        "Jeong-gyun".to_string(),
        "KR".to_string(),
        ManagerArchetype::TacticalGenius,
    )
}

#[test]
fn manager_creation_stores_identity() {
    let mgr = make_test_manager();
    assert_eq!(mgr.nickname(), "kkOma");
    assert_eq!(mgr.first_name(), "Kim");
    assert_eq!(mgr.last_name(), "Jeong-gyun");
    assert_eq!(mgr.nationality(), "KR");
}

#[test]
fn manager_creation_stores_archetype() {
    let mgr = make_test_manager();
    assert_eq!(mgr.archetype(), ManagerArchetype::TacticalGenius);
}

#[test]
fn manager_starts_with_default_reputation() {
    let mgr = make_test_manager();
    assert_eq!(mgr.reputation().value(), 50);
}

#[test]
fn manager_reputation_can_increase() {
    let mut mgr = make_test_manager();
    mgr.reputation_mut().increase(10);
    assert_eq!(mgr.reputation().value(), 60);
}

#[test]
fn manager_reputation_can_decrease() {
    let mut mgr = make_test_manager();
    mgr.reputation_mut().decrease(20);
    assert_eq!(mgr.reputation().value(), 30);
}

#[test]
fn manager_with_balanced_archetype() {
    let mgr = Manager::new(
        "Rookie".to_string(),
        "John".to_string(),
        "Doe".to_string(),
        "US".to_string(),
        ManagerArchetype::Balanced,
    );
    assert_eq!(mgr.archetype(), ManagerArchetype::Balanced);
}
