use esm_db::database::Database;
use rusqlite::params;

// ---------------------------------------------------------------------------
// Database open + auto-migration
// ---------------------------------------------------------------------------

#[test]
fn database_open_in_memory_succeeds() {
    let db = Database::open_in_memory();
    assert!(db.is_ok());
}

#[test]
fn database_applies_migrations_on_open() {
    let db = Database::open_in_memory().unwrap();
    let version = db.schema_version().unwrap();
    assert!(version >= 1);
}

// ---------------------------------------------------------------------------
// Schema: tables created by V1 migration
// ---------------------------------------------------------------------------

#[test]
fn schema_has_moba_players_table() {
    let db = Database::open_in_memory().unwrap();
    assert!(db.table_exists("moba_players").unwrap());
}

#[test]
fn schema_has_moba_teams_table() {
    let db = Database::open_in_memory().unwrap();
    assert!(db.table_exists("moba_teams").unwrap());
}

#[test]
fn schema_has_moba_champions_table() {
    let db = Database::open_in_memory().unwrap();
    assert!(db.table_exists("moba_champions").unwrap());
}

#[test]
fn schema_has_moba_champion_tags_table() {
    let db = Database::open_in_memory().unwrap();
    assert!(db.table_exists("moba_champion_tags").unwrap());
}

#[test]
fn schema_has_staff_table() {
    let db = Database::open_in_memory().unwrap();
    assert!(db.table_exists("staff").unwrap());
}

#[test]
fn schema_has_contracts_table() {
    let db = Database::open_in_memory().unwrap();
    assert!(db.table_exists("contracts").unwrap());
}

#[test]
fn schema_has_managers_table() {
    let db = Database::open_in_memory().unwrap();
    assert!(db.table_exists("managers").unwrap());
}

// ---------------------------------------------------------------------------
// Schema: insert and query players
// ---------------------------------------------------------------------------

#[test]
fn schema_moba_players_insert_and_query() {
    let db = Database::open_in_memory().unwrap();
    db.conn()
        .execute(
            "INSERT INTO moba_players (id, nickname, first_name, last_name, primary_role,
             endurance, reaction_time, decision_making, clutch, discipline,
             tilt_resistance, mechanics, vision_control, teamfighting)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            params![
                1,
                "Faker",
                "Lee",
                "Sang-hyeok",
                "Mid",
                70,
                90,
                95,
                99,
                85,
                80,
                97,
                88,
                92
            ],
        )
        .unwrap();

    let nickname: String = db
        .conn()
        .query_row(
            "SELECT nickname FROM moba_players WHERE id = 1",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(nickname, "Faker");
}

// ---------------------------------------------------------------------------
// Schema: CHECK constraints enforce valid data
// ---------------------------------------------------------------------------

#[test]
fn schema_moba_players_rejects_invalid_role() {
    let db = Database::open_in_memory().unwrap();
    let result = db.conn().execute(
        "INSERT INTO moba_players (id, nickname, first_name, last_name, primary_role,
         endurance, reaction_time, decision_making, clutch, discipline,
         tilt_resistance, mechanics, vision_control, teamfighting)
         VALUES (1, 'Test', 'A', 'B', 'InvalidRole', 50, 50, 50, 50, 50, 50, 50, 50, 50)",
        [],
    );
    assert!(result.is_err());
}

#[test]
fn schema_moba_players_rejects_attribute_out_of_range() {
    let db = Database::open_in_memory().unwrap();
    let result = db.conn().execute(
        "INSERT INTO moba_players (id, nickname, first_name, last_name, primary_role,
         endurance, reaction_time, decision_making, clutch, discipline,
         tilt_resistance, mechanics, vision_control, teamfighting)
         VALUES (1, 'Test', 'A', 'B', 'Mid', 150, 50, 50, 50, 50, 50, 50, 50, 50)",
        [],
    );
    assert!(result.is_err());
}

#[test]
fn schema_moba_teams_insert_and_query() {
    let db = Database::open_in_memory().unwrap();
    db.conn()
        .execute(
            "INSERT INTO moba_teams (id, name, tag) VALUES (?1, ?2, ?3)",
            params![1, "T1", "T1"],
        )
        .unwrap();

    let name: String = db
        .conn()
        .query_row("SELECT name FROM moba_teams WHERE id = 1", [], |row| {
            row.get(0)
        })
        .unwrap();
    assert_eq!(name, "T1");
}

#[test]
fn schema_moba_champions_rejects_invalid_class() {
    let db = Database::open_in_memory().unwrap();
    let result = db.conn().execute(
        "INSERT INTO moba_champions (id, name, class, scaling) VALUES (1, 'Test', 'InvalidClass', 'Early')",
        [],
    );
    assert!(result.is_err());
}

#[test]
fn schema_contracts_references_moba_player_and_team() {
    let db = Database::open_in_memory().unwrap();
    db.conn()
        .execute(
            "INSERT INTO moba_teams (id, name, tag) VALUES (1, 'T1', 'T1')",
            [],
        )
        .unwrap();
    db.conn()
        .execute(
            "INSERT INTO moba_players (id, nickname, first_name, last_name, primary_role,
             endurance, reaction_time, decision_making, clutch, discipline,
             tilt_resistance, mechanics, vision_control, teamfighting, team_id)
             VALUES (1, 'Faker', 'Lee', 'SH', 'Mid', 70, 90, 95, 99, 85, 80, 97, 88, 92, 1)",
            [],
        )
        .unwrap();
    db.conn()
        .execute(
            "INSERT INTO contracts (id, player_id, team_id, salary, length_days, remaining_days)
             VALUES (1, 1, 1, 50000, 365, 365)",
            [],
        )
        .unwrap();

    let salary: i64 = db
        .conn()
        .query_row("SELECT salary FROM contracts WHERE id = 1", [], |row| {
            row.get(0)
        })
        .unwrap();
    assert_eq!(salary, 50000);
}
