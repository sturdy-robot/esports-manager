use esm_engine::patch::{MetaTier, Patch, PatchCycle, PatchModifier};

// ---------------------------------------------------------------------------
// MetaTier
// ---------------------------------------------------------------------------

#[test]
fn meta_tier_variants_exist() {
    let _ = MetaTier::S;
    let _ = MetaTier::A;
    let _ = MetaTier::B;
    let _ = MetaTier::C;
    let _ = MetaTier::D;
}

#[test]
fn meta_tier_multiplier_values() {
    assert!((MetaTier::S.multiplier() - 1.5).abs() < f64::EPSILON);
    assert!((MetaTier::A.multiplier() - 1.2).abs() < f64::EPSILON);
    assert!((MetaTier::B.multiplier() - 1.0).abs() < f64::EPSILON);
    assert!((MetaTier::C.multiplier() - 0.8).abs() < f64::EPSILON);
    assert!((MetaTier::D.multiplier() - 0.6).abs() < f64::EPSILON);
}

#[test]
fn meta_tier_ordering() {
    assert!(MetaTier::S.multiplier() > MetaTier::A.multiplier());
    assert!(MetaTier::A.multiplier() > MetaTier::B.multiplier());
    assert!(MetaTier::B.multiplier() > MetaTier::C.multiplier());
    assert!(MetaTier::C.multiplier() > MetaTier::D.multiplier());
}

// ---------------------------------------------------------------------------
// PatchModifier
// ---------------------------------------------------------------------------

#[test]
fn patch_modifier_stores_champion_tier() {
    let m = PatchModifier::new("Orianna".to_string(), MetaTier::S);
    assert_eq!(m.champion_name(), "Orianna");
    assert_eq!(m.tier(), MetaTier::S);
}

// ---------------------------------------------------------------------------
// Patch
// ---------------------------------------------------------------------------

#[test]
fn patch_stores_version_and_modifiers() {
    let patch = Patch::new(
        "14.5".to_string(),
        vec![
            PatchModifier::new("Orianna".to_string(), MetaTier::S),
            PatchModifier::new("Malphite".to_string(), MetaTier::C),
        ],
    );
    assert_eq!(patch.version(), "14.5");
    assert_eq!(patch.modifiers().len(), 2);
}

#[test]
fn patch_get_tier_for_champion() {
    let patch = Patch::new(
        "14.5".to_string(),
        vec![
            PatchModifier::new("Orianna".to_string(), MetaTier::S),
            PatchModifier::new("Malphite".to_string(), MetaTier::C),
        ],
    );
    assert_eq!(patch.tier_for("Orianna"), Some(MetaTier::S));
    assert_eq!(patch.tier_for("Malphite"), Some(MetaTier::C));
    assert_eq!(patch.tier_for("Unknown"), None);
}

#[test]
fn patch_default_tier_for_unlisted_champion() {
    let patch = Patch::new("14.5".to_string(), vec![]);
    assert_eq!(patch.tier_for_or_default("AnyChamp"), MetaTier::B);
}

#[test]
fn patch_tier_for_or_default_returns_listed_tier() {
    let patch = Patch::new(
        "14.5".to_string(),
        vec![PatchModifier::new("Orianna".to_string(), MetaTier::S)],
    );
    assert_eq!(patch.tier_for_or_default("Orianna"), MetaTier::S);
}

// ---------------------------------------------------------------------------
// PatchCycle
// ---------------------------------------------------------------------------

#[test]
fn patch_cycle_creation() {
    let patches = vec![
        Patch::new("14.1".to_string(), vec![]),
        Patch::new("14.2".to_string(), vec![]),
    ];
    let cycle = PatchCycle::new(patches, 14); // new patch every 14 days
    assert_eq!(cycle.current_patch().version(), "14.1");
    assert_eq!(cycle.interval_days(), 14);
}

#[test]
fn patch_cycle_does_not_advance_before_interval() {
    let patches = vec![
        Patch::new("14.1".to_string(), vec![]),
        Patch::new("14.2".to_string(), vec![]),
    ];
    let mut cycle = PatchCycle::new(patches, 14);
    let changed = cycle.check_advance(13);
    assert!(!changed);
    assert_eq!(cycle.current_patch().version(), "14.1");
}

#[test]
fn patch_cycle_advances_at_interval() {
    let patches = vec![
        Patch::new("14.1".to_string(), vec![]),
        Patch::new("14.2".to_string(), vec![]),
    ];
    let mut cycle = PatchCycle::new(patches, 14);
    let changed = cycle.check_advance(14);
    assert!(changed);
    assert_eq!(cycle.current_patch().version(), "14.2");
}

#[test]
fn patch_cycle_does_not_advance_past_last_patch() {
    let patches = vec![
        Patch::new("14.1".to_string(), vec![]),
        Patch::new("14.2".to_string(), vec![]),
    ];
    let mut cycle = PatchCycle::new(patches, 14);
    cycle.check_advance(14); // → 14.2
    let changed = cycle.check_advance(28);
    assert!(!changed); // No more patches
    assert_eq!(cycle.current_patch().version(), "14.2");
}

#[test]
fn patch_cycle_advances_through_multiple_patches() {
    let patches = vec![
        Patch::new("1".to_string(), vec![]),
        Patch::new("2".to_string(), vec![]),
        Patch::new("3".to_string(), vec![]),
    ];
    let mut cycle = PatchCycle::new(patches, 7);
    cycle.check_advance(7);  // → 2
    cycle.check_advance(14); // → 3
    assert_eq!(cycle.current_patch().version(), "3");
}

#[test]
fn patch_cycle_current_index() {
    let patches = vec![
        Patch::new("1".to_string(), vec![]),
        Patch::new("2".to_string(), vec![]),
    ];
    let mut cycle = PatchCycle::new(patches, 7);
    assert_eq!(cycle.current_index(), 0);
    cycle.check_advance(7);
    assert_eq!(cycle.current_index(), 1);
}
