use std::path::PathBuf;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::State;

use esm_core::calendar::DayPhase;
use esm_core::game_state::GameState;
use esm_core::turn::TurnProcessor;
use esm_db::save_manager::{SaveEntry, SaveManager};
use esm_engine::moba_match::engine::{MobaMatchConfig, MobaMatchEngine};
use esm_engine::moba_match::state::TeamSide;
use esm_engine::tournament::{BracketKind, Tournament, TournamentFormat};
use esm_models::esport_type::EsportType;
use esm_models::manager::{Manager, ManagerArchetype};
use esm_models::moba::team::MobaTeam;

// ---------------------------------------------------------------------------
// Application state
// ---------------------------------------------------------------------------

pub struct AppState {
    saves_dir: PathBuf,
    game_state: Mutex<Option<GameState>>,
    tournament: Mutex<Option<Tournament>>,
    moba_teams: Mutex<Option<Vec<MobaTeam>>>,
}

impl AppState {
    pub fn new(saves_dir: PathBuf) -> Self {
        Self {
            saves_dir,
            game_state: Mutex::new(None),
            tournament: Mutex::new(None),
            moba_teams: Mutex::new(None),
        }
    }
}

// ---------------------------------------------------------------------------
// DTOs for frontend ↔ backend
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct TeamInfo {
    pub name: String,
    pub tag: String,
    pub player_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct SaveInfo {
    pub name: String,
    pub checksum: String,
}

impl From<SaveEntry> for SaveInfo {
    fn from(e: SaveEntry) -> Self {
        Self {
            name: e.name,
            checksum: e.checksum,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GameInfo {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub phase: String,
    pub manager_nickname: String,
    pub team_name: String,
    pub teams_count: usize,
    pub is_match_day: bool,
    pub match_results: Vec<MatchResultInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MatchResultInfo {
    pub blue_team: String,
    pub red_team: String,
    pub winner: String,
    pub duration_minutes: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct StandingInfo {
    pub rank: usize,
    pub team_name: String,
    pub wins: u32,
    pub losses: u32,
    pub win_pct: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScheduleMatchInfo {
    pub id: u32,
    pub blue_team: String,
    pub red_team: String,
    pub scheduled_day: u32,
    pub status: String,
    pub winner: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerInfo {
    pub nickname: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub stamina: u8,
    pub morale: u8,
    pub mechanics: u8,
    pub vision: u8,
    pub teamfighting: u8,
}

#[derive(Debug, Clone, Serialize)]
pub struct InboxMessageInfo {
    pub id: String,
    pub subject: String,
    pub category: String,
    pub priority: String,
    pub day: u32,
    pub read: bool,
}

#[derive(Debug, Deserialize)]
pub struct NewGameParams {
    pub first_name: String,
    pub last_name: String,
    pub nickname: String,
    pub nationality: String,
    pub esport_type: String,
    pub datapack_path: String,
    pub team_index: usize,
    pub save_name: String,
}

// ---------------------------------------------------------------------------
// Tauri commands
// ---------------------------------------------------------------------------

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to eSports Manager.", name)
}

#[tauri::command]
fn load_datapack(path: String) -> Result<Vec<TeamInfo>, String> {
    let resolved = resolve_data_path(&path);
    let json = std::fs::read_to_string(&resolved)
        .map_err(|e| format!("Failed to read file '{}': {e}", resolved.display()))?;
    let pack =
        esm_data::datapack::DataPack::from_json(&json).map_err(|e| format!("Parse error: {e}"))?;
    pack.validate()
        .map_err(|e| format!("Validation error: {e}"))?;

    let teams: Vec<TeamInfo> = pack
        .teams
        .iter()
        .map(|t| {
            let player_count = pack.players.iter().filter(|p| p.team == t.name).count();
            TeamInfo {
                name: t.name.clone(),
                tag: t.tag.clone(),
                player_count,
            }
        })
        .collect();

    Ok(teams)
}

#[tauri::command]
fn list_saves(state: State<'_, AppState>) -> Result<Vec<SaveInfo>, String> {
    let saves = SaveManager::list_saves(&state.saves_dir)
        .map_err(|e| format!("Failed to list saves: {e}"))?;
    Ok(saves.into_iter().map(SaveInfo::from).collect())
}

#[tauri::command]
fn new_game(params: NewGameParams, state: State<'_, AppState>) -> Result<GameInfo, String> {
    // Parse esport type
    let esport_type: EsportType = params
        .esport_type
        .parse()
        .map_err(|e| format!("Invalid esport type: {e}"))?;

    // Load data pack to get teams
    let resolved_path = resolve_data_path(&params.datapack_path);
    let json = std::fs::read_to_string(&resolved_path)
        .map_err(|e| format!("Failed to read datapack '{}': {e}", resolved_path.display()))?;
    let pack =
        esm_data::datapack::DataPack::from_json(&json).map_err(|e| format!("Parse error: {e}"))?;
    pack.validate()
        .map_err(|e| format!("Validation error: {e}"))?;

    let teams = esm_db::import::build_teams_from_datapack(&pack);
    let moba_teams = esm_db::import::build_moba_teams_from_datapack(&pack);

    if params.team_index >= teams.len() {
        return Err(format!(
            "Team index {} out of range (0..{})",
            params.team_index,
            teams.len()
        ));
    }

    let manager = Manager::new(
        params.nickname.clone(),
        params.first_name,
        params.last_name,
        params.nationality,
        ManagerArchetype::TacticalGenius,
    );

    // Create game state with a random-ish seed based on current time
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    let gs = GameState::new(2025, seed, esport_type, manager, params.team_index, teams);

    // Create tournament (Double Round Robin, Bo1, starting day 3)
    let team_names: Vec<String> = moba_teams.iter().map(|t| t.name().to_string()).collect();
    let tournament = Tournament::new(
        "Season 2025".to_string(),
        team_names,
        TournamentFormat::DoubleRoundRobin,
        BracketKind::Bo1,
        3,
    );

    // Serialize tournament + moba teams for persistence
    let tournament_json =
        serde_json::to_string(&tournament).map_err(|e| format!("Serialize tournament: {e}"))?;
    let moba_teams_json =
        serde_json::to_string(&moba_teams).map_err(|e| format!("Serialize moba teams: {e}"))?;

    // Save to disk
    SaveManager::create_save_full(
        &state.saves_dir,
        &params.save_name,
        &gs,
        &tournament_json,
        &moba_teams_json,
    )
    .map_err(|e| format!("Failed to save game: {e}"))?;

    let info = game_info_from_state(&gs, &tournament);

    // Store in memory
    *state.game_state.lock().unwrap() = Some(gs);
    *state.tournament.lock().unwrap() = Some(tournament);
    *state.moba_teams.lock().unwrap() = Some(moba_teams);

    Ok(info)
}

#[tauri::command]
fn load_save(name: String, state: State<'_, AppState>) -> Result<GameInfo, String> {
    let (gs, tournament_json, moba_teams_json) =
        SaveManager::load_save_full(&state.saves_dir, &name)
            .map_err(|e| format!("Failed to load save: {e}"))?;

    let tournament: Option<Tournament> = if tournament_json.is_empty() {
        None
    } else {
        serde_json::from_str(&tournament_json).ok()
    };

    let moba_teams: Option<Vec<MobaTeam>> = if moba_teams_json == "[]" || moba_teams_json.is_empty()
    {
        None
    } else {
        serde_json::from_str(&moba_teams_json).ok()
    };

    let info = game_info_from_state(&gs, tournament.as_ref().unwrap_or(&empty_tournament()));
    *state.game_state.lock().unwrap() = Some(gs);
    *state.tournament.lock().unwrap() = tournament;
    *state.moba_teams.lock().unwrap() = moba_teams;

    Ok(info)
}

#[tauri::command]
fn delete_save(name: String, state: State<'_, AppState>) -> Result<(), String> {
    SaveManager::delete_save(&state.saves_dir, &name)
        .map_err(|e| format!("Failed to delete save: {e}"))?;
    Ok(())
}

#[tauri::command]
fn save_game(name: String, state: State<'_, AppState>) -> Result<(), String> {
    let lock = state.game_state.lock().unwrap();
    let gs = lock.as_ref().ok_or("No active game session")?;

    let t_lock = state.tournament.lock().unwrap();
    let tournament_json = t_lock
        .as_ref()
        .map(|t| serde_json::to_string(t).unwrap_or_default())
        .unwrap_or_default();

    let m_lock = state.moba_teams.lock().unwrap();
    let moba_teams_json = m_lock
        .as_ref()
        .map(|m| serde_json::to_string(m).unwrap_or_else(|_| "[]".to_string()))
        .unwrap_or_else(|| "[]".to_string());

    SaveManager::create_save_full(
        &state.saves_dir,
        &name,
        gs,
        &tournament_json,
        &moba_teams_json,
    )
    .map_err(|e| format!("Failed to save game: {e}"))?;
    Ok(())
}

#[tauri::command]
fn get_roster(state: State<'_, AppState>) -> Result<Vec<PlayerInfo>, String> {
    let lock = state.game_state.lock().unwrap();
    let gs = lock.as_ref().ok_or("No active game session")?;
    let team = gs
        .teams()
        .get(gs.player_team_index())
        .ok_or("Player team not found")?;
    let players = team
        .roster()
        .iter()
        .map(|p| PlayerInfo {
            nickname: p.nickname().to_string(),
            first_name: p.first_name().to_string(),
            last_name: p.last_name().to_string(),
            role: format!("{:?}", p.role()),
            stamina: p.state().stamina.value(),
            morale: p.state().morale.value(),
            mechanics: p.attributes().technical.mechanics.value(),
            vision: p.attributes().technical.vision_control.value(),
            teamfighting: p.attributes().technical.teamfighting.value(),
        })
        .collect();
    Ok(players)
}

#[tauri::command]
fn get_inbox(state: State<'_, AppState>) -> Result<Vec<InboxMessageInfo>, String> {
    let lock = state.game_state.lock().unwrap();
    let gs = lock.as_ref().ok_or("No active game session")?;
    let messages = gs
        .inbox()
        .messages()
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let priority = match m.priority() {
                esm_core::inbox::MessagePriority::HardBlock => "Urgent",
                esm_core::inbox::MessagePriority::RequiresResponse => "Action",
                esm_core::inbox::MessagePriority::ReadOptional => "Info",
            };
            InboxMessageInfo {
                id: format!("msg_{i}"),
                subject: m.subject().to_string(),
                category: format!("{:?}", m.category()),
                priority: priority.to_string(),
                day: m.day_received(),
                read: m.is_resolved(),
            }
        })
        .collect();
    Ok(messages)
}

#[tauri::command]
fn advance_turn(state: State<'_, AppState>) -> Result<GameInfo, String> {
    let mut lock = state.game_state.lock().unwrap();
    let gs = lock.as_mut().ok_or("No active game session")?;
    let mut t_lock = state.tournament.lock().unwrap();
    let m_lock = state.moba_teams.lock().unwrap();

    let mut match_results: Vec<MatchResultInfo> = Vec::new();

    // If currently Evening, advancing will trigger end-of-day via TurnProcessor
    if gs.calendar().phase() == DayPhase::Evening {
        TurnProcessor::end_day(gs).map_err(|e| match e {
            esm_core::turn::TurnError::BlockingMessages => {
                "Cannot advance: there are unresolved urgent messages in your inbox.".to_string()
            }
        })?;

        // Simulate today's matches after advancing the day
        if let (Some(tournament), Some(moba_teams)) = (t_lock.as_mut(), m_lock.as_ref()) {
            let day = gs.calendar().days_elapsed();
            let todays: Vec<_> = tournament
                .matches_today(day)
                .iter()
                .map(|m| (m.id(), m.blue_team_idx(), m.red_team_idx()))
                .collect();

            let config = MobaMatchConfig::default();
            for (match_id, blue_idx, red_idx) in todays {
                if blue_idx < moba_teams.len() && red_idx < moba_teams.len() {
                    let blue_attrs = extract_team_attrs(&moba_teams[blue_idx]);
                    let red_attrs = extract_team_attrs(&moba_teams[red_idx]);
                    let result =
                        MobaMatchEngine::simulate(gs.rng_mut(), &blue_attrs, &red_attrs, &config);

                    let (bw, rw) = match result.winner {
                        TeamSide::Blue => (1u32, 0u32),
                        TeamSide::Red => (0u32, 1u32),
                    };
                    tournament.record_result(match_id, bw, rw);

                    let winner_name = match result.winner {
                        TeamSide::Blue => moba_teams[blue_idx].name().to_string(),
                        TeamSide::Red => moba_teams[red_idx].name().to_string(),
                    };
                    match_results.push(MatchResultInfo {
                        blue_team: moba_teams[blue_idx].name().to_string(),
                        red_team: moba_teams[red_idx].name().to_string(),
                        winner: winner_name,
                        duration_minutes: result.duration_minutes,
                    });
                }
            }
        }
    } else {
        gs.advance_phase();
    }

    let tournament_ref = t_lock.as_ref();
    let mut info = game_info_from_state(gs, tournament_ref.unwrap_or(&empty_tournament()));
    info.match_results = match_results;
    Ok(info)
}

#[tauri::command]
fn get_game_info(state: State<'_, AppState>) -> Result<GameInfo, String> {
    let lock = state.game_state.lock().unwrap();
    let gs = lock.as_ref().ok_or("No active game session")?;
    let t_lock = state.tournament.lock().unwrap();
    Ok(game_info_from_state(
        gs,
        t_lock.as_ref().unwrap_or(&empty_tournament()),
    ))
}

#[tauri::command]
fn get_standings(state: State<'_, AppState>) -> Result<Vec<StandingInfo>, String> {
    let t_lock = state.tournament.lock().unwrap();
    let tournament = t_lock.as_ref().ok_or("No tournament active")?;

    let standings: Vec<StandingInfo> = tournament
        .standings()
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let total = s.wins + s.losses;
            let win_pct = if total > 0 {
                s.wins as f64 / total as f64 * 100.0
            } else {
                0.0
            };
            StandingInfo {
                rank: i + 1,
                team_name: s.team_name.clone(),
                wins: s.wins,
                losses: s.losses,
                win_pct,
            }
        })
        .collect();
    Ok(standings)
}

#[tauri::command]
fn get_schedule(state: State<'_, AppState>) -> Result<Vec<ScheduleMatchInfo>, String> {
    let t_lock = state.tournament.lock().unwrap();
    let tournament = t_lock.as_ref().ok_or("No tournament active")?;

    let matches: Vec<ScheduleMatchInfo> = tournament
        .schedule()
        .matches()
        .iter()
        .map(|m| {
            let blue_name = tournament
                .team_name(m.blue_team_idx())
                .unwrap_or("")
                .to_string();
            let red_name = tournament
                .team_name(m.red_team_idx())
                .unwrap_or("")
                .to_string();
            let winner = m
                .winner_team_idx()
                .and_then(|idx| tournament.team_name(idx))
                .map(|s| s.to_string());
            ScheduleMatchInfo {
                id: m.id(),
                blue_team: blue_name,
                red_team: red_name,
                scheduled_day: m.scheduled_day(),
                status: format!("{:?}", m.status()),
                winner,
            }
        })
        .collect();
    Ok(matches)
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn game_info_from_state(gs: &GameState, tournament: &Tournament) -> GameInfo {
    let team_name = gs
        .teams()
        .get(gs.player_team_index())
        .map(|t| t.name().to_string())
        .unwrap_or_default();

    let next_day = gs.calendar().days_elapsed() + 1;
    let is_match_day = !tournament.matches_today(next_day).is_empty();

    GameInfo {
        year: gs.calendar().year(),
        month: gs.calendar().month(),
        day: gs.calendar().day(),
        phase: gs.calendar().phase().as_str().to_string(),
        manager_nickname: gs.manager().nickname().to_string(),
        team_name,
        teams_count: gs.teams().len(),
        is_match_day,
        match_results: Vec::new(),
    }
}

fn extract_team_attrs(moba_team: &MobaTeam) -> Vec<[u8; 9]> {
    moba_team
        .roster()
        .iter()
        .map(|p| {
            let a = p.attributes();
            [
                a.endurance.value(),
                a.reaction_time.value(),
                a.decision_making.value(),
                a.clutch.value(),
                a.discipline.value(),
                a.tilt_resistance.value(),
                a.mechanics.value(),
                a.vision_control.value(),
                a.teamfighting.value(),
            ]
        })
        .collect()
}

fn empty_tournament() -> Tournament {
    Tournament::new(
        String::new(),
        Vec::new(),
        TournamentFormat::RoundRobin,
        BracketKind::Bo1,
        1,
    )
}

/// Resolve a relative data path against the workspace root.
/// In dev mode, `CARGO_MANIFEST_DIR` is `crates/esm-ui/src-tauri`,
/// so the workspace root is three levels up.
fn resolve_data_path(path: &str) -> PathBuf {
    let p = PathBuf::from(path);
    if p.is_absolute() && p.exists() {
        return p;
    }
    // Try as-is first (relative to cwd)
    if p.exists() {
        return p;
    }
    // Dev mode: resolve relative to workspace root
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if let Some(workspace_root) = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
    {
        let resolved = workspace_root.join(path);
        if resolved.exists() {
            return resolved;
        }
    }
    // Fallback to the original path (will produce a clear "file not found" error)
    p
}

fn default_saves_dir() -> PathBuf {
    dirs_next_or_fallback()
}

fn dirs_next_or_fallback() -> PathBuf {
    // Use a simple fallback: ~/.esm/saves
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".esm").join("saves")
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let saves_dir = default_saves_dir();

    tauri::Builder::default()
        .manage(AppState::new(saves_dir))
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            load_datapack,
            list_saves,
            new_game,
            load_save,
            delete_save,
            save_game,
            advance_turn,
            get_roster,
            get_inbox,
            get_game_info,
            get_standings,
            get_schedule,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
