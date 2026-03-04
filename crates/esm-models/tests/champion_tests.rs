use esm_models::champion::{Champion, ChampionClass, ChampionScaling, ChampionTag, MasteryLevel};

// ---------------------------------------------------------------------------
// ChampionClass enum
// ---------------------------------------------------------------------------

#[test]
fn champion_class_variants_exist() {
    let _ = ChampionClass::Tank;
    let _ = ChampionClass::Fighter;
    let _ = ChampionClass::Assassin;
    let _ = ChampionClass::Mage;
    let _ = ChampionClass::Marksman;
    let _ = ChampionClass::Support;
}

// ---------------------------------------------------------------------------
// ChampionScaling enum
// ---------------------------------------------------------------------------

#[test]
fn champion_scaling_variants_exist() {
    let _ = ChampionScaling::Early;
    let _ = ChampionScaling::Mid;
    let _ = ChampionScaling::Late;
}

// ---------------------------------------------------------------------------
// ChampionTag enum
// ---------------------------------------------------------------------------

#[test]
fn champion_tag_variants_exist() {
    let _ = ChampionTag::Knockup;
    let _ = ChampionTag::Engage;
    let _ = ChampionTag::Poke;
    let _ = ChampionTag::Splitpush;
    let _ = ChampionTag::Waveclear;
    let _ = ChampionTag::Peel;
    let _ = ChampionTag::Burst;
    let _ = ChampionTag::Sustain;
}

// ---------------------------------------------------------------------------
// MasteryLevel enum
// ---------------------------------------------------------------------------

#[test]
fn mastery_level_variants_exist() {
    let _ = MasteryLevel::Bronze;
    let _ = MasteryLevel::Silver;
    let _ = MasteryLevel::Gold;
    let _ = MasteryLevel::Platinum;
    let _ = MasteryLevel::Diamond;
    let _ = MasteryLevel::Master;
    let _ = MasteryLevel::Challenger;
}

#[test]
fn mastery_level_multiplier_ranges() {
    assert!(MasteryLevel::Bronze.multiplier() < MasteryLevel::Silver.multiplier());
    assert!(MasteryLevel::Silver.multiplier() < MasteryLevel::Gold.multiplier());
    assert!(MasteryLevel::Gold.multiplier() < MasteryLevel::Platinum.multiplier());
    assert!(MasteryLevel::Platinum.multiplier() < MasteryLevel::Diamond.multiplier());
    assert!(MasteryLevel::Diamond.multiplier() < MasteryLevel::Master.multiplier());
    assert!(MasteryLevel::Master.multiplier() < MasteryLevel::Challenger.multiplier());
}

#[test]
fn mastery_bronze_multiplier_is_0_5() {
    assert!((MasteryLevel::Bronze.multiplier() - 0.5).abs() < f64::EPSILON);
}

#[test]
fn mastery_challenger_multiplier_is_1_5() {
    assert!((MasteryLevel::Challenger.multiplier() - 1.5).abs() < f64::EPSILON);
}

// ---------------------------------------------------------------------------
// Champion entity
// ---------------------------------------------------------------------------

fn make_test_champion() -> Champion {
    Champion::new(
        "Orianna".to_string(),
        ChampionClass::Mage,
        ChampionScaling::Mid,
        vec![ChampionTag::Poke, ChampionTag::Waveclear, ChampionTag::Peel],
    )
}

#[test]
fn champion_creation_stores_identity() {
    let champ = make_test_champion();
    assert_eq!(champ.name(), "Orianna");
    assert_eq!(champ.class(), ChampionClass::Mage);
    assert_eq!(champ.scaling(), ChampionScaling::Mid);
}

#[test]
fn champion_has_tags() {
    let champ = make_test_champion();
    assert_eq!(champ.tags().len(), 3);
    assert!(champ.tags().contains(&ChampionTag::Poke));
    assert!(champ.tags().contains(&ChampionTag::Waveclear));
    assert!(champ.tags().contains(&ChampionTag::Peel));
}

#[test]
fn champion_without_tags() {
    let champ = Champion::new(
        "TestChamp".to_string(),
        ChampionClass::Fighter,
        ChampionScaling::Early,
        vec![],
    );
    assert!(champ.tags().is_empty());
}

#[test]
fn champion_has_tag_returns_true_for_present_tag() {
    let champ = make_test_champion();
    assert!(champ.has_tag(&ChampionTag::Poke));
}

#[test]
fn champion_has_tag_returns_false_for_absent_tag() {
    let champ = make_test_champion();
    assert!(!champ.has_tag(&ChampionTag::Knockup));
}
