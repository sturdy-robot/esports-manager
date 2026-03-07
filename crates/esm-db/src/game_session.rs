use std::str::FromStr;

use rusqlite::{Connection, Result as SqlResult};

use esm_core::calendar::DayPhase;
use esm_core::game_state::GameState;
use esm_core::inbox::{Message, MessageCategory, MessagePriority};
use esm_core::rng::GameRng;
use esm_models::esport_type::EsportType;
use esm_models::manager::{Manager, ManagerArchetype};
use esm_models::team::Team;

use crate::session_repository::{InboxMessageRow, SessionRow};

const GAME_VERSION: &str = env!("CARGO_PKG_VERSION");

/// High-level bridge between in-memory `GameState` and SQLite persistence.
///
/// `save` decomposes a `GameState` into `SessionRow` + `InboxMessageRow`s and
/// writes them.  `load` reads those rows and reassembles a `GameState`.
pub struct GameSession;

impl GameSession {
    /// Persist the current `GameState` into the database.
    pub fn save(conn: &Connection, state: &GameState) -> SqlResult<()> {
        Self::save_full(conn, state, "", "[]")
    }

    /// Persist `GameState` plus tournament, moba teams, schedules, and scrims JSON blobs.
    pub fn save_full(
        conn: &Connection,
        state: &GameState,
        tournament_json: &str,
        moba_teams_json: &str,
    ) -> SqlResult<()> {
        Self::save_all(
            conn,
            state,
            tournament_json,
            moba_teams_json,
            "[]",
            "{\"scrims\":[],\"next_id\":1}",
        )
    }

    /// Persist `GameState` plus all ancillary JSON blobs.
    pub fn save_all(
        conn: &Connection,
        state: &GameState,
        tournament_json: &str,
        moba_teams_json: &str,
        schedules_json: &str,
        scrims_json: &str,
    ) -> SqlResult<()> {
        let teams_json =
            serde_json::to_string(state.teams()).expect("teams serialization cannot fail");

        let row = SessionRow {
            id: 1,
            game_version: GAME_VERSION.to_string(),
            esport_type: state.esport_type().as_str().to_string(),
            rng_seed: state.rng().seed() as i64,
            rng_state: state.rng().state() as i64,
            calendar_year: state.calendar().year() as i32,
            calendar_month: state.calendar().month() as i32,
            calendar_day: state.calendar().day() as i32,
            calendar_phase: state.calendar().phase().as_str().to_string(),
            calendar_days_elapsed: state.calendar().days_elapsed() as i32,
            player_team_name: state.player_team().name().to_string(),
            player_team_index: state.player_team_index() as i32,
            manager_nickname: state.manager().nickname().to_string(),
            manager_first_name: state.manager().first_name().to_string(),
            manager_last_name: state.manager().last_name().to_string(),
            manager_nationality: state.manager().nationality().to_string(),
            manager_archetype: state.manager().archetype().as_str().to_string(),
            manager_reputation: state.manager().reputation().value() as i32,
            teams_json,
            tournament_json: tournament_json.to_string(),
            moba_teams_json: moba_teams_json.to_string(),
            schedules_json: schedules_json.to_string(),
            scrims_json: scrims_json.to_string(),
        };

        SessionRow::upsert(conn, &row)?;

        // Persist inbox messages
        let inbox_rows: Vec<InboxMessageRow> = state
            .inbox()
            .messages()
            .iter()
            .enumerate()
            .map(|(i, msg)| InboxMessageRow {
                id: (i + 1) as i64,
                subject: msg.subject().to_string(),
                body: msg.body().to_string(),
                priority: msg.priority().as_str().to_string(),
                category: msg.category().as_str().to_string(),
                day_received: msg.day_received() as i32,
                is_resolved: msg.is_resolved(),
            })
            .collect();

        InboxMessageRow::replace_all(conn, &inbox_rows)?;

        Ok(())
    }

    /// Load a `GameState` from the database. Returns `None` if no session has
    /// been saved yet.
    pub fn load(conn: &Connection) -> SqlResult<Option<GameState>> {
        Ok(Self::load_full(conn)?.map(|(gs, _, _)| gs))
    }

    /// Load `GameState` plus tournament and moba teams JSON blobs.
    /// Returns `(GameState, tournament_json, moba_teams_json)`.
    pub fn load_full(conn: &Connection) -> SqlResult<Option<(GameState, String, String)>> {
        Ok(Self::load_all(conn)?.map(|(gs, t, m, _, _)| (gs, t, m)))
    }

    /// Load `GameState` plus all ancillary JSON blobs.
    /// Returns `(GameState, tournament_json, moba_teams_json, schedules_json, scrims_json)`.
    pub fn load_all(
        conn: &Connection,
    ) -> SqlResult<Option<(GameState, String, String, String, String)>> {
        let session = match SessionRow::get(conn)? {
            Some(s) => s,
            None => return Ok(None),
        };

        let tournament_json = session.tournament_json.clone();
        let moba_teams_json = session.moba_teams_json.clone();
        let schedules_json = session.schedules_json.clone();
        let scrims_json = session.scrims_json.clone();

        // Reconstruct teams from JSON
        let teams: Vec<Team> =
            serde_json::from_str(&session.teams_json).expect("teams deserialization cannot fail");

        // Reconstruct manager
        let archetype =
            ManagerArchetype::from_str(&session.manager_archetype).expect("valid archetype in DB");
        let mut manager = Manager::new(
            session.manager_nickname,
            session.manager_first_name,
            session.manager_last_name,
            session.manager_nationality,
            archetype,
        );
        // Adjust reputation from default (50) to saved value
        let rep_delta = session.manager_reputation - manager.reputation().value() as i32;
        if rep_delta > 0 {
            manager.reputation_mut().increase(rep_delta as u8);
        } else if rep_delta < 0 {
            manager.reputation_mut().decrease((-rep_delta) as u8);
        }

        // Reconstruct esport type
        let esport_type =
            EsportType::from_str(&session.esport_type).expect("valid esport_type in DB");

        // Create GameState via constructor then patch calendar/RNG
        let mut state = GameState::new(
            session.calendar_year as u32,
            session.rng_seed as u64,
            esport_type,
            manager,
            session.player_team_index as usize,
            teams,
        );

        // Restore RNG state (the constructor sets state = seed, but we need the saved state)
        *state.rng_mut() = GameRng::from_state(session.rng_seed as u64, session.rng_state as u64);

        // Restore calendar by advancing to saved position
        let target_phase = DayPhase::from_str(&session.calendar_phase).expect("valid phase in DB");
        for _ in 0..session.calendar_days_elapsed {
            state.advance_day();
        }
        while state.calendar().phase() != target_phase {
            state.advance_phase();
        }

        // Reconstruct inbox
        let inbox_rows = InboxMessageRow::list_all(conn)?;
        for row in &inbox_rows {
            let priority = MessagePriority::from_str(&row.priority).expect("valid priority in DB");
            let category = MessageCategory::from_str(&row.category).expect("valid category in DB");
            let mut msg = Message::new(
                row.subject.clone(),
                row.body.clone(),
                priority,
                category,
                row.day_received as u32,
            );
            if row.is_resolved {
                msg.resolve();
            }
            state.inbox_mut().push(msg);
        }

        Ok(Some((
            state,
            tournament_json,
            moba_teams_json,
            schedules_json,
            scrims_json,
        )))
    }
}
