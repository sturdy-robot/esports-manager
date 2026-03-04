use esm_models::moba::champion::{
    Champion, ChampionClass, ChampionScaling, ChampionTag, MasteryLevel,
};

// ---------------------------------------------------------------------------
// ChampionClass
// ---------------------------------------------------------------------------

#[test]
fn champion_class_variants() {
    let _ = ChampionClass::Tank;
    let _ = ChampionClass::Fighter;
    let _ = ChampionClass::Assassin;
    let _ = ChampionClass::Mage;
    let _ = ChampionClass::Marksman;
    let _ = ChampionClass::Support;
}

// ---------------------------------------------------------------------------
// ChampionScaling
// ---------------------------------------------------------------------------

#[test]
fn champion_scaling_variants() {
    let _ = ChampionScaling::Early;
    let _ = ChampionScaling::Mid;
    let _ = ChampionScaling::Late;
}

// ---------------------------------------------------------------------------
// ChampionTag
// ---------------------------------------------------------------------------

#[test]
fn champion_tag_variants() {
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
// MasteryLevel
// ---------------------------------------------------------------------------

#[test]
fn mastery_level_multiplier_ordering() {
    assert!(MasteryLevel::Bronze.multiplier() < MasteryLevel::Silver.multiplier());
    assert!(MasteryLevel::Silver.multiplier() < MasteryLevel::Gold.multiplier());
    assert!(MasteryLevel::Gold.multiplier() < MasteryLevel::Platinum.multiplier());
    assert!(MasteryLevel::Platinum.multiplier() < MasteryLevel::Diamond.multiplier());
    assert!(MasteryLevel::Diamond.multiplier() < MasteryLevel::Master.multiplier());
    assert!(MasteryLevel::Master.multiplier() < MasteryLevel::Challenger.multiplier());
}

#[test]
fn mastery_extreme_values() {
    assert!((MasteryLevel::Bronze.multiplier() - 0.5).abs() < f64::EPSILON);
    assert!((MasteryLevel::Challenger.multiplier() - 1.5).abs() < f64::EPSILON);
}

// ---------------------------------------------------------------------------
// Champion entity
// ---------------------------------------------------------------------------

#[test]
fn champion_creation() {
    let champ = Champion::new(
        "Orianna".to_string(),
        ChampionClass::Mage,
        ChampionScaling::Mid,
        vec![ChampionTag::Poke, ChampionTag::Waveclear, ChampionTag::Peel],
    );
    assert_eq!(champ.name(), "Orianna");
    assert_eq!(champ.class(), ChampionClass::Mage);
    assert_eq!(champ.scaling(), ChampionScaling::Mid);
    assert_eq!(champ.tags().len(), 3);
}

#[test]
fn champion_has_tag() {
    let champ = Champion::new(
        "Orianna".to_string(),
        ChampionClass::Mage,
        ChampionScaling::Mid,
        vec![ChampionTag::Poke],
    );
    assert!(champ.has_tag(&ChampionTag::Poke));
    assert!(!champ.has_tag(&ChampionTag::Knockup));
}
