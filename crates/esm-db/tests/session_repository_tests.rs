use esm_db::database::Database;
use esm_db::session_repository::{InboxMessageRow, SessionRow};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_session_row() -> SessionRow {
    SessionRow {
        id: 1,
        game_version: "0.1.0".to_string(),
        esport_type: "Moba".to_string(),
        rng_seed: 42,
        rng_state: 42,
        calendar_year: 2025,
        calendar_month: 1,
        calendar_day: 1,
        calendar_phase: "Morning".to_string(),
        calendar_days_elapsed: 0,
        player_team_name: "T1".to_string(),
        player_team_index: 0,
        manager_nickname: "kkOma".to_string(),
        manager_first_name: "Kim".to_string(),
        manager_last_name: "Jeong-gyun".to_string(),
        manager_nationality: "KR".to_string(),
        manager_archetype: "TacticalGenius".to_string(),
        manager_reputation: 50,
        teams_json: "[]".to_string(),
        tournament_json: "".to_string(),
        moba_teams_json: "[]".to_string(),
        schedules_json: "[]".to_string(),
        scrims_json: "{\"scrims\":[],\"next_id\":1}".to_string(),
    }
}

fn make_inbox_row(id: i64, subject: &str, priority: &str, category: &str) -> InboxMessageRow {
    InboxMessageRow {
        id,
        subject: subject.to_string(),
        body: format!("{subject} body"),
        priority: priority.to_string(),
        category: category.to_string(),
        day_received: 0,
        is_resolved: false,
    }
}

// ---------------------------------------------------------------------------
// SessionRow: insert + get
// ---------------------------------------------------------------------------

#[test]
fn session_row_insert_and_get() {
    let db = Database::open_in_memory().unwrap();
    let row = make_session_row();
    SessionRow::upsert(db.conn(), &row).unwrap();

    let loaded = SessionRow::get(db.conn()).unwrap();
    assert!(loaded.is_some());
    let loaded = loaded.unwrap();
    assert_eq!(loaded.game_version, "0.1.0");
    assert_eq!(loaded.esport_type, "Moba");
    assert_eq!(loaded.rng_seed, 42);
    assert_eq!(loaded.calendar_year, 2025);
    assert_eq!(loaded.player_team_name, "T1");
    assert_eq!(loaded.manager_nickname, "kkOma");
    assert_eq!(loaded.manager_archetype, "TacticalGenius");
}

#[test]
fn session_row_upsert_updates_existing() {
    let db = Database::open_in_memory().unwrap();
    let mut row = make_session_row();
    SessionRow::upsert(db.conn(), &row).unwrap();

    row.calendar_day = 15;
    row.calendar_days_elapsed = 14;
    row.rng_state = 99999;
    SessionRow::upsert(db.conn(), &row).unwrap();

    let loaded = SessionRow::get(db.conn()).unwrap().unwrap();
    assert_eq!(loaded.calendar_day, 15);
    assert_eq!(loaded.calendar_days_elapsed, 14);
    assert_eq!(loaded.rng_state, 99999);
}

#[test]
fn session_row_get_returns_none_when_empty() {
    let db = Database::open_in_memory().unwrap();
    let loaded = SessionRow::get(db.conn()).unwrap();
    assert!(loaded.is_none());
}

// ---------------------------------------------------------------------------
// InboxMessageRow: insert + list + update
// ---------------------------------------------------------------------------

#[test]
fn inbox_message_insert_and_list() {
    let db = Database::open_in_memory().unwrap();
    let msg1 = make_inbox_row(1, "Welcome", "ReadOptional", "Board");
    let msg2 = make_inbox_row(2, "Transfer Offer", "RequiresResponse", "Transfer");

    InboxMessageRow::insert(db.conn(), &msg1).unwrap();
    InboxMessageRow::insert(db.conn(), &msg2).unwrap();

    let all = InboxMessageRow::list_all(db.conn()).unwrap();
    assert_eq!(all.len(), 2);
    assert_eq!(all[0].subject, "Welcome");
    assert_eq!(all[1].subject, "Transfer Offer");
}

#[test]
fn inbox_message_resolve() {
    let db = Database::open_in_memory().unwrap();
    let msg = make_inbox_row(1, "Urgent", "HardBlock", "Board");
    InboxMessageRow::insert(db.conn(), &msg).unwrap();

    InboxMessageRow::set_resolved(db.conn(), 1, true).unwrap();

    let all = InboxMessageRow::list_all(db.conn()).unwrap();
    assert!(all[0].is_resolved);
}

#[test]
fn inbox_message_delete_all() {
    let db = Database::open_in_memory().unwrap();
    InboxMessageRow::insert(db.conn(), &make_inbox_row(1, "A", "ReadOptional", "News")).unwrap();
    InboxMessageRow::insert(db.conn(), &make_inbox_row(2, "B", "ReadOptional", "News")).unwrap();

    InboxMessageRow::delete_all(db.conn()).unwrap();

    let all = InboxMessageRow::list_all(db.conn()).unwrap();
    assert!(all.is_empty());
}

#[test]
fn inbox_message_replace_all_clears_and_reinserts() {
    let db = Database::open_in_memory().unwrap();
    InboxMessageRow::insert(db.conn(), &make_inbox_row(1, "Old", "ReadOptional", "News")).unwrap();

    let new_msgs = vec![
        make_inbox_row(1, "New1", "HardBlock", "Board"),
        make_inbox_row(2, "New2", "RequiresResponse", "Transfer"),
    ];
    InboxMessageRow::replace_all(db.conn(), &new_msgs).unwrap();

    let all = InboxMessageRow::list_all(db.conn()).unwrap();
    assert_eq!(all.len(), 2);
    assert_eq!(all[0].subject, "New1");
    assert_eq!(all[1].subject, "New2");
}
