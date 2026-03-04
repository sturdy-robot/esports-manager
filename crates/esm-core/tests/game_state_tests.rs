use esm_core::calendar::DayPhase;
use esm_core::game_state::GameState;
use esm_core::inbox::{Message, MessageCategory, MessagePriority};
use esm_models::manager::{Manager, ManagerArchetype};
use esm_models::player::{
    BoundedAttribute, MentalAttributes, PhysicalAttributes, Player, PlayerAttributes, Role,
    TechnicalAttributes,
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

fn make_manager() -> Manager {
    Manager::new(
        "TestMgr".to_string(),
        "John".to_string(),
        "Doe".to_string(),
        "US".to_string(),
        ManagerArchetype::Balanced,
    )
}

fn make_game_state() -> GameState {
    let teams = vec![make_team("Alpha"), make_team("Bravo"), make_team("Charlie")];
    GameState::new(2025, 42, make_manager(), 0, teams)
}

// ---------------------------------------------------------------------------
// Construction
// ---------------------------------------------------------------------------

#[test]
fn game_state_starts_at_jan_1_of_given_year() {
    let gs = make_game_state();
    assert_eq!(gs.calendar().year(), 2025);
    assert_eq!(gs.calendar().month(), 1);
    assert_eq!(gs.calendar().day(), 1);
}

#[test]
fn game_state_starts_in_morning_phase() {
    let gs = make_game_state();
    assert_eq!(gs.calendar().phase(), DayPhase::Morning);
}

#[test]
fn game_state_stores_seed() {
    let gs = make_game_state();
    assert_eq!(gs.rng().seed(), 42);
}

#[test]
fn game_state_holds_manager() {
    let gs = make_game_state();
    assert_eq!(gs.manager().nickname(), "TestMgr");
}

#[test]
fn game_state_holds_teams() {
    let gs = make_game_state();
    assert_eq!(gs.teams().len(), 3);
}

#[test]
fn game_state_player_team_index_is_stored() {
    let gs = make_game_state();
    assert_eq!(gs.player_team_index(), 0);
    assert_eq!(gs.player_team().name(), "Alpha");
}

// ---------------------------------------------------------------------------
// Phase advancement
// ---------------------------------------------------------------------------

#[test]
fn advance_phase_moves_through_day() {
    let mut gs = make_game_state();
    gs.advance_phase();
    assert_eq!(gs.calendar().phase(), DayPhase::Afternoon);

    gs.advance_phase();
    assert_eq!(gs.calendar().phase(), DayPhase::Evening);

    gs.advance_phase();
    assert_eq!(gs.calendar().phase(), DayPhase::Morning);
    assert_eq!(gs.calendar().day(), 2);
}

// ---------------------------------------------------------------------------
// Daily tick
// ---------------------------------------------------------------------------

#[test]
fn advance_day_increments_calendar() {
    let mut gs = make_game_state();
    gs.advance_day();
    assert_eq!(gs.calendar().day(), 2);
    assert_eq!(gs.calendar().days_elapsed(), 1);
    assert_eq!(gs.calendar().phase(), DayPhase::Morning);
}

#[test]
fn advance_day_seven_times_triggers_weekly_tick() {
    let mut gs = make_game_state();
    for _ in 0..7 {
        gs.advance_day();
    }
    assert!(gs.calendar().is_weekly_tick());
}

// ---------------------------------------------------------------------------
// Team lookup
// ---------------------------------------------------------------------------

#[test]
fn team_by_name_returns_matching_team() {
    let gs = make_game_state();
    let team = gs.team_by_name("Bravo");
    assert!(team.is_some());
    assert_eq!(team.unwrap().name(), "Bravo");
}

#[test]
fn team_by_name_returns_none_for_unknown() {
    let gs = make_game_state();
    assert!(gs.team_by_name("Unknown").is_none());
}

// ---------------------------------------------------------------------------
// Inbox integration
// ---------------------------------------------------------------------------

#[test]
fn game_state_starts_with_empty_inbox() {
    let gs = make_game_state();
    assert!(gs.inbox().is_empty());
}

#[test]
fn game_state_can_continue_with_empty_inbox() {
    let gs = make_game_state();
    assert!(gs.can_continue());
}

#[test]
fn game_state_cannot_continue_with_blocking_message() {
    let mut gs = make_game_state();
    gs.inbox_mut().push(Message::new(
        "Roster Gap".to_string(),
        "Fill mid lane.".to_string(),
        MessagePriority::HardBlock,
        MessageCategory::Board,
        0,
    ));
    assert!(!gs.can_continue());
}

#[test]
fn game_state_can_continue_after_resolving_blocking_message() {
    let mut gs = make_game_state();
    gs.inbox_mut().push(Message::new(
        "Roster Gap".to_string(),
        "Fill mid lane.".to_string(),
        MessagePriority::HardBlock,
        MessageCategory::Board,
        0,
    ));
    assert!(!gs.can_continue());

    gs.inbox_mut().resolve_at(0);
    assert!(gs.can_continue());
}

#[test]
fn game_state_can_continue_with_non_blocking_messages() {
    let mut gs = make_game_state();
    gs.inbox_mut().push(Message::new(
        "News".to_string(),
        "Patch notes.".to_string(),
        MessagePriority::ReadOptional,
        MessageCategory::News,
        0,
    ));
    gs.inbox_mut().push(Message::new(
        "Transfer".to_string(),
        "Player wants out.".to_string(),
        MessagePriority::RequiresResponse,
        MessageCategory::Transfer,
        0,
    ));
    assert!(gs.can_continue());
}

// ---------------------------------------------------------------------------
// Determinism: two identical game states advance identically
// ---------------------------------------------------------------------------

#[test]
fn deterministic_rng_across_identical_game_states() {
    let mut gs1 = make_game_state();
    let mut gs2 = make_game_state();

    let vals1: Vec<u32> = (0..20).map(|_| gs1.rng_mut().next_u32()).collect();
    let vals2: Vec<u32> = (0..20).map(|_| gs2.rng_mut().next_u32()).collect();
    assert_eq!(vals1, vals2);
}
