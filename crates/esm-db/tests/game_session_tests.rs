use esm_core::game_state::GameState;
use esm_core::inbox::{Message, MessageCategory, MessagePriority};
use esm_db::database::Database;
use esm_db::game_session::GameSession;
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
        "kkOma".to_string(),
        "Kim".to_string(),
        "Jeong-gyun".to_string(),
        "KR".to_string(),
        ManagerArchetype::TacticalGenius,
    )
}

fn make_game_state() -> GameState {
    let teams = vec![make_team("Alpha"), make_team("Bravo"), make_team("Charlie")];
    GameState::new(2025, 42, EsportType::Moba, make_manager(), 0, teams)
}

// ---------------------------------------------------------------------------
// Save and load roundtrip
// ---------------------------------------------------------------------------

#[test]
fn save_and_load_roundtrip_preserves_calendar() {
    let db = Database::open_in_memory().unwrap();
    let gs = make_game_state();

    GameSession::save(db.conn(), &gs).unwrap();
    let loaded = GameSession::load(db.conn())
        .unwrap()
        .expect("should have a saved session");

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
fn save_and_load_roundtrip_preserves_rng() {
    let db = Database::open_in_memory().unwrap();
    let gs = make_game_state();

    GameSession::save(db.conn(), &gs).unwrap();
    let loaded = GameSession::load(db.conn()).unwrap().unwrap();

    assert_eq!(loaded.rng().seed(), gs.rng().seed());
    assert_eq!(loaded.rng().state(), gs.rng().state());
}

#[test]
fn save_and_load_roundtrip_preserves_esport_type() {
    let db = Database::open_in_memory().unwrap();
    let gs = make_game_state();

    GameSession::save(db.conn(), &gs).unwrap();
    let loaded = GameSession::load(db.conn()).unwrap().unwrap();

    assert_eq!(loaded.esport_type(), gs.esport_type());
}

#[test]
fn save_and_load_roundtrip_preserves_manager() {
    let db = Database::open_in_memory().unwrap();
    let gs = make_game_state();

    GameSession::save(db.conn(), &gs).unwrap();
    let loaded = GameSession::load(db.conn()).unwrap().unwrap();

    assert_eq!(loaded.manager().nickname(), gs.manager().nickname());
    assert_eq!(loaded.manager().first_name(), gs.manager().first_name());
    assert_eq!(loaded.manager().last_name(), gs.manager().last_name());
    assert_eq!(loaded.manager().nationality(), gs.manager().nationality());
    assert_eq!(loaded.manager().archetype(), gs.manager().archetype());
    assert_eq!(
        loaded.manager().reputation().value(),
        gs.manager().reputation().value()
    );
}

#[test]
fn save_and_load_roundtrip_preserves_teams() {
    let db = Database::open_in_memory().unwrap();
    let gs = make_game_state();

    GameSession::save(db.conn(), &gs).unwrap();
    let loaded = GameSession::load(db.conn()).unwrap().unwrap();

    assert_eq!(loaded.teams().len(), gs.teams().len());
    assert_eq!(loaded.player_team_index(), gs.player_team_index());
    for i in 0..loaded.teams().len() {
        assert_eq!(loaded.teams()[i].name(), gs.teams()[i].name());
    }
}

#[test]
fn save_and_load_roundtrip_preserves_inbox() {
    let db = Database::open_in_memory().unwrap();
    let mut gs = make_game_state();
    gs.inbox_mut().push(Message::new(
        "Welcome".to_string(),
        "Good luck!".to_string(),
        MessagePriority::ReadOptional,
        MessageCategory::Board,
        0,
    ));
    gs.inbox_mut().push(Message::new(
        "Transfer".to_string(),
        "Offer received.".to_string(),
        MessagePriority::RequiresResponse,
        MessageCategory::Transfer,
        1,
    ));

    GameSession::save(db.conn(), &gs).unwrap();
    let loaded = GameSession::load(db.conn()).unwrap().unwrap();

    assert_eq!(loaded.inbox().len(), 2);
    assert_eq!(loaded.inbox().messages()[0].subject(), "Welcome");
    assert_eq!(loaded.inbox().messages()[1].subject(), "Transfer");
    assert_eq!(
        loaded.inbox().messages()[0].priority(),
        MessagePriority::ReadOptional
    );
    assert_eq!(
        loaded.inbox().messages()[1].category(),
        MessageCategory::Transfer
    );
}

#[test]
fn load_returns_none_when_no_session_saved() {
    let db = Database::open_in_memory().unwrap();
    let loaded = GameSession::load(db.conn()).unwrap();
    assert!(loaded.is_none());
}

#[test]
fn save_overwrites_previous_session() {
    let db = Database::open_in_memory().unwrap();
    let gs = make_game_state();

    GameSession::save(db.conn(), &gs).unwrap();

    // Advance state and save again
    let mut gs2 = gs;
    gs2.advance_day();
    GameSession::save(db.conn(), &gs2).unwrap();

    let loaded = GameSession::load(db.conn()).unwrap().unwrap();
    assert_eq!(loaded.calendar().day(), 2);
    assert_eq!(loaded.calendar().days_elapsed(), 1);
}
