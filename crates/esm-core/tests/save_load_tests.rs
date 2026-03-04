use esm_core::game_state::GameState;
use esm_core::inbox::{Message, MessageCategory, MessagePriority};
use esm_core::turn::TurnProcessor;
use esm_models::esport_type::EsportType;
use esm_models::manager::{Manager, ManagerArchetype};
use esm_models::player::{
    BoundedAttribute, Confidence, MentalAttributes, PhysicalAttributes, Player, PlayerAttributes,
    Role, TechnicalAttributes,
};
use esm_models::team::Team;

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
                endurance: BoundedAttribute::new(60),
                reaction_time: BoundedAttribute::new(70),
            },
            mental: MentalAttributes {
                decision_making: BoundedAttribute::new(55),
                clutch: BoundedAttribute::new(65),
                discipline: BoundedAttribute::new(50),
                tilt_resistance: BoundedAttribute::new(45),
            },
            technical: TechnicalAttributes {
                mechanics: BoundedAttribute::new(80),
                vision_control: BoundedAttribute::new(60),
                teamfighting: BoundedAttribute::new(75),
            },
        },
    )
}

fn make_team(name: &str) -> Team {
    Team::new(
        name.to_string(),
        name[..2].to_uppercase(),
        vec![
            make_player(&format!("{name}_top"), Role::Top),
            make_player(&format!("{name}_jg"), Role::Jungle),
            make_player(&format!("{name}_mid"), Role::Mid),
            make_player(&format!("{name}_bot"), Role::Bot),
            make_player(&format!("{name}_sup"), Role::Support),
        ],
    )
}

fn make_game_state() -> GameState {
    let teams = vec![make_team("Alpha"), make_team("Bravo")];
    GameState::new(
        2025,
        12345,
        EsportType::Moba,
        Manager::new(
            "SaveTest".to_string(),
            "Jane".to_string(),
            "Doe".to_string(),
            "KR".to_string(),
            ManagerArchetype::Analyst,
        ),
        0,
        teams,
    )
}

// ---------------------------------------------------------------------------
// Basic roundtrip: serialize → deserialize → fields match
// ---------------------------------------------------------------------------

#[test]
fn game_state_json_roundtrip_preserves_calendar() {
    let gs = make_game_state();
    let json = serde_json::to_string(&gs).unwrap();
    let loaded: GameState = serde_json::from_str(&json).unwrap();

    assert_eq!(loaded.calendar().year(), gs.calendar().year());
    assert_eq!(loaded.calendar().month(), gs.calendar().month());
    assert_eq!(loaded.calendar().day(), gs.calendar().day());
    assert_eq!(loaded.calendar().phase(), gs.calendar().phase());
    assert_eq!(
        loaded.calendar().days_elapsed(),
        gs.calendar().days_elapsed()
    );
}

#[test]
fn game_state_json_roundtrip_preserves_rng_seed() {
    let gs = make_game_state();
    let json = serde_json::to_string(&gs).unwrap();
    let loaded: GameState = serde_json::from_str(&json).unwrap();

    assert_eq!(loaded.rng().seed(), gs.rng().seed());
}

#[test]
fn game_state_json_roundtrip_preserves_manager() {
    let gs = make_game_state();
    let json = serde_json::to_string(&gs).unwrap();
    let loaded: GameState = serde_json::from_str(&json).unwrap();

    assert_eq!(loaded.manager().nickname(), "SaveTest");
    assert_eq!(loaded.manager().archetype(), ManagerArchetype::Analyst);
    assert_eq!(
        loaded.manager().reputation().value(),
        gs.manager().reputation().value()
    );
}

#[test]
fn game_state_json_roundtrip_preserves_teams() {
    let gs = make_game_state();
    let json = serde_json::to_string(&gs).unwrap();
    let loaded: GameState = serde_json::from_str(&json).unwrap();

    assert_eq!(loaded.teams().len(), 2);
    assert_eq!(loaded.teams()[0].name(), "Alpha");
    assert_eq!(loaded.teams()[1].name(), "Bravo");
    assert_eq!(loaded.player_team_index(), 0);
}

#[test]
fn game_state_json_roundtrip_preserves_player_attributes() {
    let gs = make_game_state();
    let json = serde_json::to_string(&gs).unwrap();
    let loaded: GameState = serde_json::from_str(&json).unwrap();

    let original_mid = gs.teams()[0].player_by_role(Role::Mid).unwrap();
    let loaded_mid = loaded.teams()[0].player_by_role(Role::Mid).unwrap();

    assert_eq!(
        loaded_mid.attributes().technical.mechanics.value(),
        original_mid.attributes().technical.mechanics.value()
    );
    assert_eq!(
        loaded_mid.state().stamina.value(),
        original_mid.state().stamina.value()
    );
}

// ---------------------------------------------------------------------------
// Roundtrip after state mutations
// ---------------------------------------------------------------------------

#[test]
fn game_state_roundtrip_after_advancing_days() {
    let mut gs = make_game_state();
    for _ in 0..10 {
        let _ = TurnProcessor::end_day(&mut gs);
    }

    let json = serde_json::to_string(&gs).unwrap();
    let loaded: GameState = serde_json::from_str(&json).unwrap();

    assert_eq!(loaded.calendar().days_elapsed(), 10);
    assert_eq!(loaded.calendar().day(), gs.calendar().day());
}

#[test]
fn game_state_roundtrip_after_rng_consumption() {
    let mut gs = make_game_state();
    // Consume some RNG values
    for _ in 0..50 {
        gs.rng_mut().next_u32();
    }

    let json = serde_json::to_string(&gs).unwrap();
    let mut loaded: GameState = serde_json::from_str(&json).unwrap();

    // Next RNG value should be identical after load
    let original_next = gs.rng_mut().next_u32();
    let loaded_next = loaded.rng_mut().next_u32();
    assert_eq!(
        original_next, loaded_next,
        "RNG state must survive serialization"
    );
}

#[test]
fn game_state_roundtrip_after_player_state_changes() {
    let mut gs = make_game_state();
    // Modify a player's state
    gs.teams_mut()[0]
        .player_by_role_mut(Role::Mid)
        .unwrap()
        .state_mut()
        .stamina
        .decrease(35);
    gs.teams_mut()[0]
        .player_by_role_mut(Role::Mid)
        .unwrap()
        .state_mut()
        .confidence = Confidence::Hyped;

    let json = serde_json::to_string(&gs).unwrap();
    let loaded: GameState = serde_json::from_str(&json).unwrap();

    let loaded_mid = loaded.teams()[0].player_by_role(Role::Mid).unwrap();
    assert_eq!(loaded_mid.state().stamina.value(), 65);
    assert_eq!(loaded_mid.state().confidence, Confidence::Hyped);
}

#[test]
fn game_state_roundtrip_preserves_inbox() {
    let mut gs = make_game_state();
    gs.inbox_mut().push(Message::new(
        "Test".to_string(),
        "Body".to_string(),
        MessagePriority::RequiresResponse,
        MessageCategory::Transfer,
        5,
    ));

    let json = serde_json::to_string(&gs).unwrap();
    let loaded: GameState = serde_json::from_str(&json).unwrap();

    assert_eq!(loaded.inbox().len(), 1);
    assert_eq!(loaded.inbox().messages()[0].subject(), "Test");
    assert_eq!(
        loaded.inbox().messages()[0].priority(),
        MessagePriority::RequiresResponse
    );
}

// ---------------------------------------------------------------------------
// Pretty JSON is valid and parseable
// ---------------------------------------------------------------------------

#[test]
fn game_state_pretty_json_roundtrip() {
    let gs = make_game_state();
    let pretty = serde_json::to_string_pretty(&gs).unwrap();
    let loaded: GameState = serde_json::from_str(&pretty).unwrap();

    assert_eq!(loaded.calendar().year(), 2025);
    assert_eq!(loaded.rng().seed(), 12345);
}
