use rusqlite::{params, Connection, OptionalExtension, Result as SqlResult, Row};

// ---------------------------------------------------------------------------
// SessionRow
// ---------------------------------------------------------------------------

/// DTO for the `game_session` table. Stores the transient session state
/// that lives in memory during play and gets flushed to SQLite on save.
#[derive(Debug, Clone)]
pub struct SessionRow {
    pub id: i64,
    pub game_version: String,
    pub esport_type: String,
    pub rng_seed: i64,
    pub rng_state: i64,
    pub calendar_year: i32,
    pub calendar_month: i32,
    pub calendar_day: i32,
    pub calendar_phase: String,
    pub calendar_days_elapsed: i32,
    pub player_team_name: String,
    pub player_team_index: i32,
    pub manager_nickname: String,
    pub manager_first_name: String,
    pub manager_last_name: String,
    pub manager_nationality: String,
    pub manager_archetype: String,
    pub manager_reputation: i32,
    pub teams_json: String,
    pub tournament_json: String,
    pub moba_teams_json: String,
}

impl SessionRow {
    fn from_row(row: &Row) -> SqlResult<Self> {
        Ok(Self {
            id: row.get("id")?,
            game_version: row.get("game_version")?,
            esport_type: row.get("esport_type")?,
            rng_seed: row.get("rng_seed")?,
            rng_state: row.get("rng_state")?,
            calendar_year: row.get("calendar_year")?,
            calendar_month: row.get("calendar_month")?,
            calendar_day: row.get("calendar_day")?,
            calendar_phase: row.get("calendar_phase")?,
            calendar_days_elapsed: row.get("calendar_days_elapsed")?,
            player_team_name: row.get("player_team_name")?,
            player_team_index: row.get("player_team_index")?,
            manager_nickname: row.get("manager_nickname")?,

            manager_first_name: row.get("manager_first_name")?,
            manager_last_name: row.get("manager_last_name")?,
            manager_nationality: row.get("manager_nationality")?,
            manager_archetype: row.get("manager_archetype")?,
            manager_reputation: row.get("manager_reputation")?,
            teams_json: row.get("teams_json")?,
            tournament_json: row.get("tournament_json")?,
            moba_teams_json: row.get("moba_teams_json")?,
        })
    }

    /// Insert or replace the single session row (id = 1).
    pub fn upsert(conn: &Connection, s: &SessionRow) -> SqlResult<()> {
        conn.execute(
            "INSERT OR REPLACE INTO game_session
             (id, game_version, esport_type, rng_seed, rng_state,
              calendar_year, calendar_month, calendar_day, calendar_phase, calendar_days_elapsed,
              player_team_name, player_team_index, manager_nickname, manager_first_name, manager_last_name,
              manager_nationality, manager_archetype, manager_reputation, teams_json,
              tournament_json, moba_teams_json, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, datetime('now'))",
            params![
                s.id,
                s.game_version,
                s.esport_type,
                s.rng_seed,
                s.rng_state,
                s.calendar_year,
                s.calendar_month,
                s.calendar_day,
                s.calendar_phase,
                s.calendar_days_elapsed,
                s.player_team_name,
                s.player_team_index,
                s.manager_nickname,
                s.manager_first_name,
                s.manager_last_name,
                s.manager_nationality,
                s.manager_archetype,
                s.manager_reputation,
                s.teams_json,
                s.tournament_json,
                s.moba_teams_json,
            ],
        )?;
        Ok(())
    }

    /// Load the session row. Returns `None` if no session has been saved yet.
    pub fn get(conn: &Connection) -> SqlResult<Option<SessionRow>> {
        conn.query_row(
            "SELECT * FROM game_session ORDER BY id LIMIT 1",
            [],
            Self::from_row,
        )
        .optional()
    }
}

// ---------------------------------------------------------------------------
// InboxMessageRow
// ---------------------------------------------------------------------------

/// DTO for the `inbox_messages` table.
#[derive(Debug, Clone)]
pub struct InboxMessageRow {
    pub id: i64,
    pub subject: String,
    pub body: String,
    pub priority: String,
    pub category: String,
    pub day_received: i32,
    pub is_resolved: bool,
}

impl InboxMessageRow {
    fn from_row(row: &Row) -> SqlResult<Self> {
        let resolved_int: i32 = row.get("is_resolved")?;
        Ok(Self {
            id: row.get("id")?,
            subject: row.get("subject")?,
            body: row.get("body")?,
            priority: row.get("priority")?,
            category: row.get("category")?,
            day_received: row.get("day_received")?,
            is_resolved: resolved_int != 0,
        })
    }

    /// Insert a single inbox message.
    pub fn insert(conn: &Connection, m: &InboxMessageRow) -> SqlResult<()> {
        conn.execute(
            "INSERT INTO inbox_messages (id, subject, body, priority, category, day_received, is_resolved)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                m.id,
                m.subject,
                m.body,
                m.priority,
                m.category,
                m.day_received,
                m.is_resolved as i32,
            ],
        )?;
        Ok(())
    }

    /// List all inbox messages ordered by id.
    pub fn list_all(conn: &Connection) -> SqlResult<Vec<InboxMessageRow>> {
        let mut stmt = conn.prepare("SELECT * FROM inbox_messages ORDER BY id")?;
        let rows = stmt.query_map([], Self::from_row)?;
        rows.collect()
    }

    /// Update the resolved flag for a specific message.
    pub fn set_resolved(conn: &Connection, id: i64, resolved: bool) -> SqlResult<()> {
        conn.execute(
            "UPDATE inbox_messages SET is_resolved = ?1 WHERE id = ?2",
            params![resolved as i32, id],
        )?;
        Ok(())
    }

    /// Delete all inbox messages.
    pub fn delete_all(conn: &Connection) -> SqlResult<()> {
        conn.execute("DELETE FROM inbox_messages", [])?;
        Ok(())
    }

    /// Replace all inbox messages: delete existing, then insert new ones.
    pub fn replace_all(conn: &Connection, messages: &[InboxMessageRow]) -> SqlResult<()> {
        Self::delete_all(conn)?;
        for msg in messages {
            Self::insert(conn, msg)?;
        }
        Ok(())
    }
}
