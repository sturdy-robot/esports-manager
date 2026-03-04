use esm_core::game_state::GameState;
use esm_core::inbox::{Message, MessageCategory, MessagePriority};
use esm_core::turn::{TurnError, TurnProcessor};
use esm_models::esport_type::EsportType;
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
    let teams = vec![make_team("Alpha"), make_team("Bravo")];
    GameState::new(2025, 42, EsportType::Moba, make_manager(), 0, teams)
}

// ---------------------------------------------------------------------------
// TurnProcessor: end_day
// ---------------------------------------------------------------------------

#[test]
fn end_day_fails_when_blocking_messages_exist() {
    let mut gs = make_game_state();
    gs.inbox_mut().push(Message::new(
        "Block".to_string(),
        "Must resolve.".to_string(),
        MessagePriority::HardBlock,
        MessageCategory::Board,
        0,
    ));
    let result = TurnProcessor::end_day(&mut gs);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), TurnError::BlockingMessages);
}

#[test]
fn end_day_advances_calendar_by_one_day() {
    let mut gs = make_game_state();
    let _ = TurnProcessor::end_day(&mut gs);
    assert_eq!(gs.calendar().day(), 2);
    assert_eq!(gs.calendar().days_elapsed(), 1);
}

#[test]
fn end_day_returns_turn_result_with_day_info() {
    let mut gs = make_game_state();
    let result = TurnProcessor::end_day(&mut gs).unwrap();
    assert_eq!(result.day_elapsed, 1);
    assert!(!result.is_weekly_tick);
}

#[test]
fn end_day_reports_weekly_tick_on_day_7() {
    let mut gs = make_game_state();
    for _ in 0..6 {
        let _ = TurnProcessor::end_day(&mut gs);
    }
    let result = TurnProcessor::end_day(&mut gs).unwrap();
    assert_eq!(result.day_elapsed, 7);
    assert!(result.is_weekly_tick);
}

#[test]
fn end_day_applies_stamina_recovery_to_all_players() {
    let mut gs = make_game_state();

    // Deplete stamina on a player first
    gs.teams_mut()[0]
        .player_by_role_mut(Role::Mid)
        .unwrap()
        .state_mut()
        .stamina
        .decrease(40);
    assert_eq!(
        gs.teams()[0]
            .player_by_role(Role::Mid)
            .unwrap()
            .state()
            .stamina
            .value(),
        60
    );

    let _ = TurnProcessor::end_day(&mut gs);

    // After end_day, stamina should have recovered by the base recovery amount
    let stamina_after = gs.teams()[0]
        .player_by_role(Role::Mid)
        .unwrap()
        .state()
        .stamina
        .value();
    assert!(
        stamina_after > 60,
        "Stamina should recover after end_day, got {stamina_after}"
    );
}

#[test]
fn end_day_does_not_exceed_stamina_cap() {
    let mut gs = make_game_state();
    // Player starts at 100 stamina
    let _ = TurnProcessor::end_day(&mut gs);
    let stamina = gs.teams()[0]
        .player_by_role(Role::Mid)
        .unwrap()
        .state()
        .stamina
        .value();
    assert_eq!(stamina, 100, "Stamina should not exceed 100");
}

#[test]
fn end_day_is_deterministic() {
    let mut gs1 = make_game_state();
    let mut gs2 = make_game_state();

    // Deplete same player same amount
    gs1.teams_mut()[0]
        .player_by_role_mut(Role::Mid)
        .unwrap()
        .state_mut()
        .stamina
        .decrease(30);
    gs2.teams_mut()[0]
        .player_by_role_mut(Role::Mid)
        .unwrap()
        .state_mut()
        .stamina
        .decrease(30);

    let r1 = TurnProcessor::end_day(&mut gs1).unwrap();
    let r2 = TurnProcessor::end_day(&mut gs2).unwrap();

    assert_eq!(r1.day_elapsed, r2.day_elapsed);
    assert_eq!(r1.is_weekly_tick, r2.is_weekly_tick);

    let s1 = gs1.teams()[0]
        .player_by_role(Role::Mid)
        .unwrap()
        .state()
        .stamina
        .value();
    let s2 = gs2.teams()[0]
        .player_by_role(Role::Mid)
        .unwrap()
        .state()
        .stamina
        .value();
    assert_eq!(
        s1, s2,
        "Identical game states should produce identical stamina"
    );
}

#[test]
fn end_day_resolves_unresponded_actionable_messages_as_default() {
    let mut gs = make_game_state();
    gs.inbox_mut().push(Message::new(
        "Transfer Request".to_string(),
        "Player wants out.".to_string(),
        MessagePriority::RequiresResponse,
        MessageCategory::Transfer,
        0,
    ));
    assert!(!gs.inbox().messages()[0].is_resolved());

    let _ = TurnProcessor::end_day(&mut gs);

    // Unresolved actionable messages should be auto-resolved with default outcome
    assert!(
        gs.inbox().messages()[0].is_resolved(),
        "Unresolved RequiresResponse messages should auto-resolve on end_day"
    );
}
