use esm_db::migration::{Migration, MigrationRunner};
use rusqlite::Connection;

// ---------------------------------------------------------------------------
// Migration struct
// ---------------------------------------------------------------------------

#[test]
fn migration_stores_version_and_sql() {
    let m = Migration::new(1, "Create table", "CREATE TABLE test (id INTEGER PRIMARY KEY);");
    assert_eq!(m.version(), 1);
    assert_eq!(m.description(), "Create table");
    assert_eq!(m.sql(), "CREATE TABLE test (id INTEGER PRIMARY KEY);");
}

// ---------------------------------------------------------------------------
// MigrationRunner — initialization
// ---------------------------------------------------------------------------

#[test]
fn runner_creates_migrations_table_on_init() {
    let conn = Connection::open_in_memory().unwrap();
    MigrationRunner::initialize(&conn).unwrap();

    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM sqlite_master WHERE type='table' AND name='_migrations'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert!(exists);
}

#[test]
fn runner_init_is_idempotent() {
    let conn = Connection::open_in_memory().unwrap();
    MigrationRunner::initialize(&conn).unwrap();
    MigrationRunner::initialize(&conn).unwrap(); // Should not error
}

// ---------------------------------------------------------------------------
// MigrationRunner — applying migrations
// ---------------------------------------------------------------------------

#[test]
fn runner_applies_single_migration() {
    let conn = Connection::open_in_memory().unwrap();
    MigrationRunner::initialize(&conn).unwrap();

    let migrations = vec![Migration::new(
        1,
        "Create players",
        "CREATE TABLE players (id INTEGER PRIMARY KEY, nickname TEXT NOT NULL);",
    )];

    let applied = MigrationRunner::run(&conn, &migrations).unwrap();
    assert_eq!(applied, 1);

    // Verify the table exists
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM sqlite_master WHERE type='table' AND name='players'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert!(exists);
}

#[test]
fn runner_skips_already_applied_migrations() {
    let conn = Connection::open_in_memory().unwrap();
    MigrationRunner::initialize(&conn).unwrap();

    let migrations = vec![Migration::new(
        1,
        "Create players",
        "CREATE TABLE players (id INTEGER PRIMARY KEY, nickname TEXT NOT NULL);",
    )];

    let first_run = MigrationRunner::run(&conn, &migrations).unwrap();
    assert_eq!(first_run, 1);

    let second_run = MigrationRunner::run(&conn, &migrations).unwrap();
    assert_eq!(second_run, 0);
}

#[test]
fn runner_applies_multiple_migrations_in_order() {
    let conn = Connection::open_in_memory().unwrap();
    MigrationRunner::initialize(&conn).unwrap();

    let migrations = vec![
        Migration::new(
            1,
            "Create players",
            "CREATE TABLE players (id INTEGER PRIMARY KEY, nickname TEXT NOT NULL);",
        ),
        Migration::new(
            2,
            "Create teams",
            "CREATE TABLE teams (id INTEGER PRIMARY KEY, name TEXT NOT NULL);",
        ),
        Migration::new(
            3,
            "Create champions",
            "CREATE TABLE champions (id INTEGER PRIMARY KEY, name TEXT NOT NULL);",
        ),
    ];

    let applied = MigrationRunner::run(&conn, &migrations).unwrap();
    assert_eq!(applied, 3);
}

#[test]
fn runner_applies_only_new_migrations() {
    let conn = Connection::open_in_memory().unwrap();
    MigrationRunner::initialize(&conn).unwrap();

    let first_batch = vec![Migration::new(
        1,
        "Create players",
        "CREATE TABLE players (id INTEGER PRIMARY KEY);",
    )];
    MigrationRunner::run(&conn, &first_batch).unwrap();

    let full_batch = vec![
        Migration::new(1, "Create players", "CREATE TABLE players (id INTEGER PRIMARY KEY);"),
        Migration::new(2, "Create teams", "CREATE TABLE teams (id INTEGER PRIMARY KEY);"),
    ];

    let applied = MigrationRunner::run(&conn, &full_batch).unwrap();
    assert_eq!(applied, 1); // Only migration 2 is new
}

#[test]
fn runner_current_version_starts_at_zero() {
    let conn = Connection::open_in_memory().unwrap();
    MigrationRunner::initialize(&conn).unwrap();

    let version = MigrationRunner::current_version(&conn).unwrap();
    assert_eq!(version, 0);
}

#[test]
fn runner_current_version_tracks_latest() {
    let conn = Connection::open_in_memory().unwrap();
    MigrationRunner::initialize(&conn).unwrap();

    let migrations = vec![
        Migration::new(1, "V1", "CREATE TABLE t1 (id INTEGER PRIMARY KEY);"),
        Migration::new(2, "V2", "CREATE TABLE t2 (id INTEGER PRIMARY KEY);"),
    ];
    MigrationRunner::run(&conn, &migrations).unwrap();

    let version = MigrationRunner::current_version(&conn).unwrap();
    assert_eq!(version, 2);
}

#[test]
fn runner_rejects_invalid_sql() {
    let conn = Connection::open_in_memory().unwrap();
    MigrationRunner::initialize(&conn).unwrap();

    let migrations = vec![Migration::new(1, "Bad SQL", "THIS IS NOT VALID SQL;")];
    let result = MigrationRunner::run(&conn, &migrations);
    assert!(result.is_err());
}
