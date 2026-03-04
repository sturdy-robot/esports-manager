use esm_db::database::Database;

// ---------------------------------------------------------------------------
// V2 migration: moba_ prefixed tables exist
// ---------------------------------------------------------------------------

#[test]
fn v2_moba_players_table_exists() {
    let db = Database::open_in_memory().unwrap();
    assert!(db.table_exists("moba_players").unwrap());
}

#[test]
fn v2_moba_teams_table_exists() {
    let db = Database::open_in_memory().unwrap();
    assert!(db.table_exists("moba_teams").unwrap());
}

#[test]
fn v2_moba_champions_table_exists() {
    let db = Database::open_in_memory().unwrap();
    assert!(db.table_exists("moba_champions").unwrap());
}

#[test]
fn v2_moba_champion_tags_table_exists() {
    let db = Database::open_in_memory().unwrap();
    assert!(db.table_exists("moba_champion_tags").unwrap());
}

#[test]
fn v2_moba_tournaments_table_exists() {
    let db = Database::open_in_memory().unwrap();
    assert!(db.table_exists("moba_tournaments").unwrap());
}

#[test]
fn v2_moba_matches_table_exists() {
    let db = Database::open_in_memory().unwrap();
    assert!(db.table_exists("moba_matches").unwrap());
}

#[test]
fn v2_shared_managers_table_still_exists() {
    let db = Database::open_in_memory().unwrap();
    assert!(db.table_exists("managers").unwrap());
}

#[test]
fn v2_shared_staff_table_still_exists() {
    let db = Database::open_in_memory().unwrap();
    assert!(db.table_exists("staff").unwrap());
}

#[test]
fn v2_shared_contracts_table_still_exists() {
    let db = Database::open_in_memory().unwrap();
    assert!(db.table_exists("contracts").unwrap());
}

// ---------------------------------------------------------------------------
// V2: old table names no longer exist
// ---------------------------------------------------------------------------

#[test]
fn v2_old_players_table_gone() {
    let db = Database::open_in_memory().unwrap();
    assert!(!db.table_exists("players").unwrap());
}

#[test]
fn v2_old_teams_table_gone() {
    let db = Database::open_in_memory().unwrap();
    assert!(!db.table_exists("teams").unwrap());
}

// ---------------------------------------------------------------------------
// V2: multi-role columns on moba_players
// ---------------------------------------------------------------------------

#[test]
fn v2_moba_players_has_primary_role_and_secondary_roles() {
    let db = Database::open_in_memory().unwrap();

    // Insert a team first
    db.conn()
        .execute(
            "INSERT INTO moba_teams (id, name, tag) VALUES (1, 'T1', 'T1')",
            [],
        )
        .unwrap();

    // Insert a player with multi-role data
    db.conn()
        .execute(
            "INSERT INTO moba_players (id, nickname, first_name, last_name, primary_role,
             secondary_roles, endurance, reaction_time, decision_making, clutch, discipline,
             tilt_resistance, mechanics, vision_control, teamfighting, team_id)
             VALUES (1, 'Faker', 'Lee', 'SH', 'Mid', 'Support,Top', 70, 90, 95, 99, 85, 80, 97, 88, 92, 1)",
            [],
        )
        .unwrap();

    let (primary, secondary): (String, String) = db
        .conn()
        .query_row(
            "SELECT primary_role, secondary_roles FROM moba_players WHERE id = 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .unwrap();

    assert_eq!(primary, "Mid");
    assert_eq!(secondary, "Support,Top");
}

// ---------------------------------------------------------------------------
// V2: moba_tournaments and moba_matches
// ---------------------------------------------------------------------------

#[test]
fn v2_insert_tournament_and_match() {
    let db = Database::open_in_memory().unwrap();

    // Insert teams
    db.conn()
        .execute(
            "INSERT INTO moba_teams (id, name, tag) VALUES (1, 'T1', 'T1')",
            [],
        )
        .unwrap();
    db.conn()
        .execute(
            "INSERT INTO moba_teams (id, name, tag) VALUES (2, 'GenG', 'GEN')",
            [],
        )
        .unwrap();

    // Insert tournament
    db.conn()
        .execute(
            "INSERT INTO moba_tournaments (id, name, format, bracket_kind, start_day)
             VALUES (1, 'LCK Spring', 'DoubleRoundRobin', 'Bo3', 3)",
            [],
        )
        .unwrap();

    // Insert match
    db.conn()
        .execute(
            "INSERT INTO moba_matches (id, tournament_id, blue_team_id, red_team_id, scheduled_day)
             VALUES (1, 1, 1, 2, 3)",
            [],
        )
        .unwrap();

    let name: String = db
        .conn()
        .query_row(
            "SELECT name FROM moba_tournaments WHERE id = 1",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(name, "LCK Spring");

    let scheduled_day: i32 = db
        .conn()
        .query_row(
            "SELECT scheduled_day FROM moba_matches WHERE id = 1",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(scheduled_day, 3);
}

#[test]
fn v2_schema_version_is_2() {
    let db = Database::open_in_memory().unwrap();
    let version = db.schema_version().unwrap();
    assert!(version >= 2, "expected schema version >= 2, got {version}");
}
