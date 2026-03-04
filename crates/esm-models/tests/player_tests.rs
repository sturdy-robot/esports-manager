use esm_models::player::{
    BoundedAttribute, Confidence, MentalAttributes, PhysicalAttributes, Player, PlayerAttributes,
    PlayerState, Role, TechnicalAttributes,
};

// ---------------------------------------------------------------------------
// BoundedAttribute value object
// ---------------------------------------------------------------------------

#[test]
fn bounded_attribute_clamps_to_max_100() {
    let attr = BoundedAttribute::new(120);
    assert_eq!(attr.value(), 100);
}

#[test]
fn bounded_attribute_clamps_to_min_0() {
    let _attr = BoundedAttribute::new(-5_i16 as u8);
    // Since u8 wraps, let's test with the explicit setter instead
    let mut attr = BoundedAttribute::new(5);
    attr.decrease(10);
    assert_eq!(attr.value(), 0);
}

#[test]
fn bounded_attribute_stores_valid_value() {
    let attr = BoundedAttribute::new(75);
    assert_eq!(attr.value(), 75);
}

#[test]
fn bounded_attribute_increase_clamps_at_100() {
    let mut attr = BoundedAttribute::new(95);
    attr.increase(10);
    assert_eq!(attr.value(), 100);
}

#[test]
fn bounded_attribute_decrease_clamps_at_0() {
    let mut attr = BoundedAttribute::new(5);
    attr.decrease(10);
    assert_eq!(attr.value(), 0);
}

#[test]
fn bounded_attribute_increase_by_normal_amount() {
    let mut attr = BoundedAttribute::new(50);
    attr.increase(10);
    assert_eq!(attr.value(), 60);
}

#[test]
fn bounded_attribute_decrease_by_normal_amount() {
    let mut attr = BoundedAttribute::new(50);
    attr.decrease(10);
    assert_eq!(attr.value(), 40);
}

// ---------------------------------------------------------------------------
// Confidence enum
// ---------------------------------------------------------------------------

#[test]
fn confidence_default_is_neutral() {
    let conf = Confidence::default();
    assert_eq!(conf, Confidence::Neutral);
}

#[test]
fn confidence_variants_exist() {
    let _ = Confidence::Slumping;
    let _ = Confidence::Neutral;
    let _ = Confidence::Confident;
    let _ = Confidence::Hyped;
}

// ---------------------------------------------------------------------------
// Role enum
// ---------------------------------------------------------------------------

#[test]
fn role_has_all_moba_positions() {
    let _ = Role::Top;
    let _ = Role::Jungle;
    let _ = Role::Mid;
    let _ = Role::Bot;
    let _ = Role::Support;
}

// ---------------------------------------------------------------------------
// Attribute groups
// ---------------------------------------------------------------------------

#[test]
fn physical_attributes_constructed_with_bounded_values() {
    let phys = PhysicalAttributes {
        endurance: BoundedAttribute::new(70),
        reaction_time: BoundedAttribute::new(85),
    };
    assert_eq!(phys.endurance.value(), 70);
    assert_eq!(phys.reaction_time.value(), 85);
}

#[test]
fn mental_attributes_constructed_with_bounded_values() {
    let mental = MentalAttributes {
        decision_making: BoundedAttribute::new(60),
        clutch: BoundedAttribute::new(55),
        discipline: BoundedAttribute::new(70),
        tilt_resistance: BoundedAttribute::new(45),
    };
    assert_eq!(mental.decision_making.value(), 60);
    assert_eq!(mental.clutch.value(), 55);
    assert_eq!(mental.discipline.value(), 70);
    assert_eq!(mental.tilt_resistance.value(), 45);
}

#[test]
fn technical_attributes_constructed_with_bounded_values() {
    let tech = TechnicalAttributes {
        mechanics: BoundedAttribute::new(80),
        vision_control: BoundedAttribute::new(65),
        teamfighting: BoundedAttribute::new(75),
    };
    assert_eq!(tech.mechanics.value(), 80);
    assert_eq!(tech.vision_control.value(), 65);
    assert_eq!(tech.teamfighting.value(), 75);
}

#[test]
fn player_attributes_groups_all_categories() {
    let attrs = PlayerAttributes {
        physical: PhysicalAttributes {
            endurance: BoundedAttribute::new(50),
            reaction_time: BoundedAttribute::new(50),
        },
        mental: MentalAttributes {
            decision_making: BoundedAttribute::new(50),
            clutch: BoundedAttribute::new(50),
            discipline: BoundedAttribute::new(50),
            tilt_resistance: BoundedAttribute::new(50),
        },
        technical: TechnicalAttributes {
            mechanics: BoundedAttribute::new(50),
            vision_control: BoundedAttribute::new(50),
            teamfighting: BoundedAttribute::new(50),
        },
    };
    assert_eq!(attrs.physical.endurance.value(), 50);
    assert_eq!(attrs.mental.decision_making.value(), 50);
    assert_eq!(attrs.technical.mechanics.value(), 50);
}

// ---------------------------------------------------------------------------
// PlayerState (dynamic modifiers)
// ---------------------------------------------------------------------------

#[test]
fn player_state_defaults_are_correct() {
    let state = PlayerState::default();
    assert_eq!(state.stamina.value(), 100);
    assert_eq!(state.morale.value(), 50);
    assert_eq!(state.confidence, Confidence::Neutral);
}

#[test]
fn player_state_stamina_depletion() {
    let mut state = PlayerState::default();
    state.stamina.decrease(20);
    assert_eq!(state.stamina.value(), 80);
}

#[test]
fn player_state_morale_boost() {
    let mut state = PlayerState::default();
    state.morale.increase(15);
    assert_eq!(state.morale.value(), 65);
}

// ---------------------------------------------------------------------------
// Player entity
// ---------------------------------------------------------------------------

fn make_test_player() -> Player {
    Player::new(
        "Faker".to_string(),
        "Lee".to_string(),
        "Sang-hyeok".to_string(),
        Role::Mid,
        make_test_attributes(),
    )
}

fn make_test_attributes() -> PlayerAttributes {
    PlayerAttributes {
        physical: PhysicalAttributes {
            endurance: BoundedAttribute::new(70),
            reaction_time: BoundedAttribute::new(90),
        },
        mental: MentalAttributes {
            decision_making: BoundedAttribute::new(95),
            clutch: BoundedAttribute::new(99),
            discipline: BoundedAttribute::new(85),
            tilt_resistance: BoundedAttribute::new(80),
        },
        technical: TechnicalAttributes {
            mechanics: BoundedAttribute::new(97),
            vision_control: BoundedAttribute::new(88),
            teamfighting: BoundedAttribute::new(92),
        },
    }
}

#[test]
fn player_creation_stores_identity() {
    let player = make_test_player();
    assert_eq!(player.nickname(), "Faker");
    assert_eq!(player.first_name(), "Lee");
    assert_eq!(player.last_name(), "Sang-hyeok");
    assert_eq!(player.role(), Role::Mid);
}

#[test]
fn player_creation_has_default_state() {
    let player = make_test_player();
    assert_eq!(player.state().stamina.value(), 100);
    assert_eq!(player.state().morale.value(), 50);
    assert_eq!(player.state().confidence, Confidence::Neutral);
}

#[test]
fn player_attributes_are_accessible() {
    let player = make_test_player();
    assert_eq!(player.attributes().technical.mechanics.value(), 97);
    assert_eq!(player.attributes().mental.clutch.value(), 99);
    assert_eq!(player.attributes().physical.reaction_time.value(), 90);
}

#[test]
fn player_state_is_mutable() {
    let mut player = make_test_player();
    player.state_mut().stamina.decrease(25);
    assert_eq!(player.state().stamina.value(), 75);

    player.state_mut().morale.increase(10);
    assert_eq!(player.state().morale.value(), 60);

    player.state_mut().confidence = Confidence::Hyped;
    assert_eq!(player.state().confidence, Confidence::Hyped);
}
