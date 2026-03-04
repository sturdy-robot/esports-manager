use esm_db::database::Database;
use esm_db::repository::{PlayerRow, TeamRow};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_db() -> Database {
    Database::open_in_memory().unwrap()
}

fn sample_team() -> TeamRow {
    TeamRow {
        id: 1,
        name: "T1".to_string(),
        tag: "T1".to_string(),
        synergy: 0,
        reputation: 50,
        budget: 1_000_000,
    }
}

fn sample_player(team_id: Option<i64>) -> PlayerRow {
    PlayerRow {
        id: 1,
        nickname: "Faker".to_string(),
        first_name: "Lee".to_string(),
        last_name: "Sang-hyeok".to_string(),
        primary_role: "Mid".to_string(),
        endurance: 70,
        reaction_time: 90,
        decision_making: 95,
        clutch: 99,
        discipline: 85,
        tilt_resistance: 80,
        mechanics: 97,
        vision_control: 88,
        teamfighting: 92,
        stamina: 100,
        morale: 50,
        confidence: "Neutral".to_string(),
        team_id,
    }
}

// ---------------------------------------------------------------------------
// TeamRow CRUD
// ---------------------------------------------------------------------------

#[test]
fn insert_and_get_team() {
    let db = make_db();
    let team = sample_team();
    TeamRow::insert(db.conn(), &team).unwrap();

    let loaded = TeamRow::get_by_id(db.conn(), 1).unwrap().unwrap();
    assert_eq!(loaded.name, "T1");
    assert_eq!(loaded.tag, "T1");
    assert_eq!(loaded.budget, 1_000_000);
}

#[test]
fn get_team_returns_none_for_missing_id() {
    let db = make_db();
    let loaded = TeamRow::get_by_id(db.conn(), 999).unwrap();
    assert!(loaded.is_none());
}

#[test]
fn list_all_teams() {
    let db = make_db();
    TeamRow::insert(db.conn(), &sample_team()).unwrap();
    TeamRow::insert(
        db.conn(),
        &TeamRow {
            id: 2,
            name: "Gen.G".to_string(),
            tag: "GEN".to_string(),
            ..sample_team()
        },
    )
    .unwrap();

    let teams = TeamRow::list_all(db.conn()).unwrap();
    assert_eq!(teams.len(), 2);
}

#[test]
fn update_team() {
    let db = make_db();
    let mut team = sample_team();
    TeamRow::insert(db.conn(), &team).unwrap();

    team.reputation = 75;
    team.budget = 2_000_000;
    TeamRow::update(db.conn(), &team).unwrap();

    let loaded = TeamRow::get_by_id(db.conn(), 1).unwrap().unwrap();
    assert_eq!(loaded.reputation, 75);
    assert_eq!(loaded.budget, 2_000_000);
}

#[test]
fn delete_team() {
    let db = make_db();
    TeamRow::insert(db.conn(), &sample_team()).unwrap();
    TeamRow::delete(db.conn(), 1).unwrap();

    let loaded = TeamRow::get_by_id(db.conn(), 1).unwrap();
    assert!(loaded.is_none());
}

// ---------------------------------------------------------------------------
// PlayerRow CRUD
// ---------------------------------------------------------------------------

#[test]
fn insert_and_get_player() {
    let db = make_db();
    TeamRow::insert(db.conn(), &sample_team()).unwrap();

    let player = sample_player(Some(1));
    PlayerRow::insert(db.conn(), &player).unwrap();

    let loaded = PlayerRow::get_by_id(db.conn(), 1).unwrap().unwrap();
    assert_eq!(loaded.nickname, "Faker");
    assert_eq!(loaded.primary_role, "Mid");
    assert_eq!(loaded.mechanics, 97);
    assert_eq!(loaded.team_id, Some(1));
}

#[test]
fn get_player_returns_none_for_missing_id() {
    let db = make_db();
    let loaded = PlayerRow::get_by_id(db.conn(), 999).unwrap();
    assert!(loaded.is_none());
}

#[test]
fn list_players_by_team() {
    let db = make_db();
    TeamRow::insert(db.conn(), &sample_team()).unwrap();

    for i in 1..=5 {
        let p = PlayerRow {
            id: i,
            nickname: format!("Player{i}"),
            team_id: Some(1),
            ..sample_player(Some(1))
        };
        PlayerRow::insert(db.conn(), &p).unwrap();
    }

    let roster = PlayerRow::list_by_team(db.conn(), 1).unwrap();
    assert_eq!(roster.len(), 5);
}

#[test]
fn update_player() {
    let db = make_db();
    let mut player = sample_player(None);
    PlayerRow::insert(db.conn(), &player).unwrap();

    player.stamina = 60;
    player.morale = 75;
    player.confidence = "Hyped".to_string();
    PlayerRow::update(db.conn(), &player).unwrap();

    let loaded = PlayerRow::get_by_id(db.conn(), 1).unwrap().unwrap();
    assert_eq!(loaded.stamina, 60);
    assert_eq!(loaded.morale, 75);
    assert_eq!(loaded.confidence, "Hyped");
}

#[test]
fn delete_player() {
    let db = make_db();
    PlayerRow::insert(db.conn(), &sample_player(None)).unwrap();
    PlayerRow::delete(db.conn(), 1).unwrap();

    let loaded = PlayerRow::get_by_id(db.conn(), 1).unwrap();
    assert!(loaded.is_none());
}

#[test]
fn player_without_team() {
    let db = make_db();
    let player = sample_player(None);
    PlayerRow::insert(db.conn(), &player).unwrap();

    let loaded = PlayerRow::get_by_id(db.conn(), 1).unwrap().unwrap();
    assert_eq!(loaded.team_id, None);
}
