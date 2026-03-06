use esm_engine::schedule::processor::ScheduleProcessor;
use esm_engine::schedule::{ScheduleEntry, SoloQueueFocus, TeamDailySchedule};
use esm_models::player::{
    BoundedAttribute, MentalAttributes, PhysicalAttributes, Player, PlayerAttributes, Role,
    TechnicalAttributes,
};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_player(nickname: &str, role: Role) -> Player {
    Player::new(
        nickname.to_string(),
        "First".to_string(),
        "Last".to_string(),
        role,
        PlayerAttributes {
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
        },
    )
}

fn make_roster() -> Vec<Player> {
    vec![
        make_player("top", Role::Top),
        make_player("jg", Role::Jungle),
        make_player("mid", Role::Mid),
        make_player("bot", Role::Bot),
        make_player("sup", Role::Support),
    ]
}

// ---------------------------------------------------------------------------
// Scrim slot effects
// ---------------------------------------------------------------------------

#[test]
fn scrim_slot_depletes_stamina_for_all_players() {
    let mut roster = make_roster();
    let mut day = TeamDailySchedule::new();
    day.set(
        esm_models::time::TimeSlot::Morning,
        ScheduleEntry::Scrim { scrim_id: 1 },
    );

    ScheduleProcessor::apply_daily_effects(&day, &mut roster);

    for player in &roster {
        assert!(
            player.state().stamina.value() < 100,
            "Scrim should deplete stamina for {}",
            player.nickname()
        );
    }
}

#[test]
fn scrim_slot_costs_expected_stamina() {
    let mut roster = make_roster();
    let mut day = TeamDailySchedule::new();
    day.set(
        esm_models::time::TimeSlot::Morning,
        ScheduleEntry::Scrim { scrim_id: 1 },
    );

    ScheduleProcessor::apply_daily_effects(&day, &mut roster);

    // Scrim cost is 18 per the Activity::ScrimBlock effect
    let expected = 100 - 18;
    assert_eq!(roster[0].state().stamina.value(), expected);
}

// ---------------------------------------------------------------------------
// Solo queue slot effects
// ---------------------------------------------------------------------------

#[test]
fn solo_queue_depletes_stamina_only_for_assigned_players() {
    let mut roster = make_roster();
    let mut day = TeamDailySchedule::new();
    day.set(
        esm_models::time::TimeSlot::Afternoon,
        ScheduleEntry::SoloQueue {
            players: vec![0, 2], // top and mid
            focus: SoloQueueFocus::Mechanics,
        },
    );

    ScheduleProcessor::apply_daily_effects(&day, &mut roster);

    // Assigned players lose stamina
    assert!(roster[0].state().stamina.value() < 100);
    assert!(roster[2].state().stamina.value() < 100);

    // Unassigned players are unaffected
    assert_eq!(roster[1].state().stamina.value(), 100);
    assert_eq!(roster[3].state().stamina.value(), 100);
    assert_eq!(roster[4].state().stamina.value(), 100);
}

#[test]
fn solo_queue_costs_expected_stamina() {
    let mut roster = make_roster();
    let mut day = TeamDailySchedule::new();
    day.set(
        esm_models::time::TimeSlot::Morning,
        ScheduleEntry::SoloQueue {
            players: vec![0],
            focus: SoloQueueFocus::Champions,
        },
    );

    ScheduleProcessor::apply_daily_effects(&day, &mut roster);

    // SoloQueue cost is 8 per the Activity::SoloQueue effect
    let expected = 100 - 8;
    assert_eq!(roster[0].state().stamina.value(), expected);
}

// ---------------------------------------------------------------------------
// Rest slot effects
// ---------------------------------------------------------------------------

#[test]
fn rest_slot_recovers_stamina_for_all_players() {
    let mut roster = make_roster();
    // Deplete stamina first
    for player in &mut roster {
        player.state_mut().stamina.decrease(50);
    }

    let mut day = TeamDailySchedule::new();
    day.set(
        esm_models::time::TimeSlot::Morning,
        ScheduleEntry::Rest,
    );

    ScheduleProcessor::apply_daily_effects(&day, &mut roster);

    // Rest day gives +30 recovery
    for player in &roster {
        assert_eq!(
            player.state().stamina.value(),
            80, // 50 + 30
            "Rest should recover stamina for {}",
            player.nickname()
        );
    }
}

// ---------------------------------------------------------------------------
// Free slot (None) has no effect
// ---------------------------------------------------------------------------

#[test]
fn free_slot_has_no_effect() {
    let mut roster = make_roster();
    let day = TeamDailySchedule::new(); // all free

    ScheduleProcessor::apply_daily_effects(&day, &mut roster);

    for player in &roster {
        assert_eq!(
            player.state().stamina.value(),
            100,
            "Free slots should not affect stamina for {}",
            player.nickname()
        );
    }
}

// ---------------------------------------------------------------------------
// Multiple slots accumulate
// ---------------------------------------------------------------------------

#[test]
fn multiple_slots_accumulate_effects() {
    let mut roster = make_roster();
    let mut day = TeamDailySchedule::new();
    day.set(
        esm_models::time::TimeSlot::Morning,
        ScheduleEntry::Scrim { scrim_id: 1 },
    );
    day.set(
        esm_models::time::TimeSlot::Afternoon,
        ScheduleEntry::Scrim { scrim_id: 2 },
    );

    ScheduleProcessor::apply_daily_effects(&day, &mut roster);

    // Two scrims: 2 * 18 = 36 stamina cost
    let expected = 100 - 36;
    assert_eq!(roster[0].state().stamina.value(), expected);
}

#[test]
fn scrim_and_rest_partial_recovery() {
    let mut roster = make_roster();
    let mut day = TeamDailySchedule::new();
    day.set(
        esm_models::time::TimeSlot::Morning,
        ScheduleEntry::Scrim { scrim_id: 1 },
    );
    day.set(
        esm_models::time::TimeSlot::Evening,
        ScheduleEntry::Rest,
    );

    ScheduleProcessor::apply_daily_effects(&day, &mut roster);

    // Scrim costs 18, rest recovers 30: net +12 but capped at 100
    assert_eq!(roster[0].state().stamina.value(), 100);
}

// ---------------------------------------------------------------------------
// Stamina depletion penalty
// ---------------------------------------------------------------------------

#[test]
fn stamina_depletion_penalizes_morale() {
    let mut roster = make_roster();
    // Set very low stamina
    for player in &mut roster {
        player.state_mut().stamina = BoundedAttribute::new(10);
    }

    let mut day = TeamDailySchedule::new();
    // Three scrims = 54 stamina cost, player starts at 10
    day.set(esm_models::time::TimeSlot::Morning, ScheduleEntry::Scrim { scrim_id: 1 });
    day.set(esm_models::time::TimeSlot::Afternoon, ScheduleEntry::Scrim { scrim_id: 2 });
    day.set(esm_models::time::TimeSlot::Evening, ScheduleEntry::Scrim { scrim_id: 3 });

    ScheduleProcessor::apply_daily_effects(&day, &mut roster);

    // Stamina should be at 0
    assert_eq!(roster[0].state().stamina.value(), 0);
    // Morale should have been penalized (default morale is 70)
    assert!(roster[0].state().morale.value() < 70);
}
