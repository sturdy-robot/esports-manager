use esm_data::datapack::DataPack;
use esm_db::database::Database;
use esm_db::import::DataPackImporter;
use esm_db::repository::{PlayerRow, TeamRow};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn sample_datapack_json() -> String {
    r#"{
        "teams": [
            {"name": "T1", "tag": "T1", "budget": 1000000},
            {"name": "Gen.G", "tag": "GEN", "budget": 900000}
        ],
        "players": [
            {
                "nickname": "Faker", "first_name": "Lee", "last_name": "SH",
                "role": "Mid", "team": "T1",
                "endurance": 70, "reaction_time": 90, "decision_making": 95,
                "clutch": 99, "discipline": 85, "tilt_resistance": 80,
                "mechanics": 97, "vision_control": 88, "teamfighting": 92
            },
            {
                "nickname": "Zeus", "first_name": "Choi", "last_name": "WJ",
                "role": "Top", "team": "T1",
                "endurance": 75, "reaction_time": 85, "decision_making": 80,
                "clutch": 78, "discipline": 70, "tilt_resistance": 65,
                "mechanics": 90, "vision_control": 72, "teamfighting": 85
            },
            {
                "nickname": "Chovy", "first_name": "Jeong", "last_name": "JH",
                "role": "Mid", "team": "Gen.G",
                "endurance": 65, "reaction_time": 88, "decision_making": 90,
                "clutch": 85, "discipline": 80, "tilt_resistance": 75,
                "mechanics": 96, "vision_control": 82, "teamfighting": 88
            }
        ],
        "champions": [
            {"name": "Orianna", "class": "Mage", "scaling": "Mid", "tags": ["Poke", "Waveclear"]},
            {"name": "Malphite", "class": "Tank", "scaling": "Late", "tags": ["Knockup", "Engage"]}
        ]
    }"#
    .to_string()
}

fn import_sample() -> Database {
    let db = Database::open_in_memory().unwrap();
    let pack = DataPack::from_json(&sample_datapack_json()).unwrap();
    pack.validate().unwrap();
    DataPackImporter::import(db.conn(), &pack).unwrap();
    db
}

// ---------------------------------------------------------------------------
// Import: teams
// ---------------------------------------------------------------------------

#[test]
fn import_creates_teams() {
    let db = import_sample();
    let teams = TeamRow::list_all(db.conn()).unwrap();
    assert_eq!(teams.len(), 2);
}

#[test]
fn import_team_fields_are_correct() {
    let db = import_sample();
    let t1 = TeamRow::get_by_id(db.conn(), 1).unwrap().unwrap();
    assert_eq!(t1.name, "T1");
    assert_eq!(t1.tag, "T1");
    assert_eq!(t1.budget, 1_000_000);
}

// ---------------------------------------------------------------------------
// Import: players
// ---------------------------------------------------------------------------

#[test]
fn import_creates_players() {
    let db = import_sample();
    let t1_roster = PlayerRow::list_by_team(db.conn(), 1).unwrap();
    assert_eq!(t1_roster.len(), 2); // Faker + Zeus

    let gen_roster = PlayerRow::list_by_team(db.conn(), 2).unwrap();
    assert_eq!(gen_roster.len(), 1); // Chovy
}

#[test]
fn import_player_fields_are_correct() {
    let db = import_sample();
    let faker = PlayerRow::get_by_id(db.conn(), 1).unwrap().unwrap();
    assert_eq!(faker.nickname, "Faker");
    assert_eq!(faker.primary_role, "Mid");
    assert_eq!(faker.mechanics, 97);
    assert_eq!(faker.team_id, Some(1));
}

#[test]
fn import_player_defaults_are_applied() {
    let db = import_sample();
    let faker = PlayerRow::get_by_id(db.conn(), 1).unwrap().unwrap();
    assert_eq!(faker.stamina, 100);
    assert_eq!(faker.morale, 50);
    assert_eq!(faker.confidence, "Neutral");
}

// ---------------------------------------------------------------------------
// Import: champions
// ---------------------------------------------------------------------------

#[test]
fn import_creates_champions() {
    let db = import_sample();
    let count: i64 = db
        .conn()
        .query_row("SELECT COUNT(*) FROM moba_champions", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 2);
}

#[test]
fn import_champion_tags() {
    let db = import_sample();
    let tag_count: i64 = db
        .conn()
        .query_row(
            "SELECT COUNT(*) FROM moba_champion_tags WHERE champion_id = 1",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(tag_count, 2); // Poke + Waveclear for Orianna
}

// ---------------------------------------------------------------------------
// Import: validation before import
// ---------------------------------------------------------------------------

#[test]
fn import_rejects_invalid_datapack() {
    let db = Database::open_in_memory().unwrap();
    let json = r#"{
        "teams": [],
        "players": [{
            "nickname": "Bad", "first_name": "A", "last_name": "B",
            "role": "Mid", "team": "NonExistent",
            "endurance": 50, "reaction_time": 50, "decision_making": 50,
            "clutch": 50, "discipline": 50, "tilt_resistance": 50,
            "mechanics": 50, "vision_control": 50, "teamfighting": 50
        }],
        "champions": []
    }"#;
    let pack = DataPack::from_json(json).unwrap();
    let result = DataPackImporter::import(db.conn(), &pack);
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Import: idempotency / clean slate
// ---------------------------------------------------------------------------

#[test]
fn import_into_fresh_db_succeeds() {
    let db = Database::open_in_memory().unwrap();
    let pack = DataPack::from_json(&sample_datapack_json()).unwrap();
    let result = DataPackImporter::import(db.conn(), &pack);
    assert!(result.is_ok());
}
