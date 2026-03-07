use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::State;

use esm_core::calendar::DayPhase;
use esm_core::game_state::GameState;
use esm_core::turn::TurnProcessor;
use esm_db::save_manager::{SaveEntry, SaveManager};
use esm_engine::draft::DraftFormat;
use esm_engine::draft_session::{
    build_champion_evals, ChampionDraftInfo, DraftPlayerInfo, DraftSession, DraftSessionState,
};
use esm_engine::match_sim::TeamSide as DraftTeamSide;
use esm_engine::moba_match::engine::{MobaMatchConfig, MobaMatchEngine, MobaMatchResult};
use esm_engine::moba_match::event::MatchEventKind;
use esm_engine::moba_match::game_state::MatchPlayerSimulationData;
use esm_engine::moba_match::state::TeamSide;
use esm_engine::moba_match::tactics::{Focus, MatchTactics, Playstyle};
use esm_engine::patch::Patch;
use esm_engine::schedule::processor::ScheduleProcessor;
use esm_engine::schedule::scrim::ScrimDraftRules;
use esm_engine::schedule::scrim_manager::ScrimManager;
use esm_engine::schedule::{ScheduleEntry, SoloQueueFocus, TeamWeeklySchedule};
use esm_engine::tournament::{BracketKind, Tournament, TournamentFormat};
use esm_models::esport_type::EsportType;
use esm_models::manager::{Manager, ManagerArchetype};
use esm_models::moba::team::MobaTeam;
use esm_models::player::PlayerTalk;
use esm_models::time::TimeSlot;

// ---------------------------------------------------------------------------
// Application state
// ---------------------------------------------------------------------------

pub struct AppState {
    saves_dir: PathBuf,
    game_state: Mutex<Option<GameState>>,
    tournament: Mutex<Option<Tournament>>,
    moba_teams: Mutex<Option<Vec<MobaTeam>>>,
    champion_names: Mutex<Vec<String>>,
    champion_detail_map: Mutex<HashMap<String, ChampionDraftInfo>>,
    draft_session: Mutex<Option<DraftSession>>,
    match_tactics: Mutex<MatchTactics>,
    team_schedules: Mutex<Vec<TeamWeeklySchedule>>,
    scrim_manager: Mutex<ScrimManager>,
    current_patch: Mutex<Patch>,
}

impl AppState {
    pub fn new(saves_dir: PathBuf) -> Self {
        Self {
            saves_dir,
            game_state: Mutex::new(None),
            tournament: Mutex::new(None),
            moba_teams: Mutex::new(None),
            champion_names: Mutex::new(Vec::new()),
            champion_detail_map: Mutex::new(HashMap::new()),
            draft_session: Mutex::new(None),
            match_tactics: Mutex::new(MatchTactics::default()),
            team_schedules: Mutex::new(Vec::new()),
            scrim_manager: Mutex::new(ScrimManager::new()),
            current_patch: Mutex::new(Patch::new("1.0".to_string(), Vec::new())),
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
pub struct PlayerSnapshotInfo {
    pub kills: u32,
    pub deaths: u32,
    pub assists: u32,
    pub cs: u32,
    pub gold: u32,
    pub is_dead: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct GameSnapshotInfo {
    pub blue_players: Vec<PlayerSnapshotInfo>,
    pub red_players: Vec<PlayerSnapshotInfo>,
    pub blue_team_gold: u32,
    pub red_team_gold: u32,
    pub dragons_blue: u32,
    pub dragons_red: u32,
    pub baron_alive: bool,
    pub baron_timer: u32,
    pub dragon_timer: u32,
    pub herald_available: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct MatchEventInfo {
    pub minute: u32,
    pub phase: String,
    pub kind: String,
    pub commentary: Option<String>,
    pub snapshot: Option<GameSnapshotInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SimulateMatchResultInfo {
    pub winner: String,
    pub duration_minutes: u32,
    pub blue_team: String,
    pub red_team: String,
    pub blue_gold: u32,
    pub red_gold: u32,
    pub events: Vec<MatchEventInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SeriesInfo {
    pub match_id: u32,
    pub blue_team: String,
    pub red_team: String,
    pub blue_wins: u32,
    pub red_wins: u32,
    pub wins_needed: u32,
    pub is_complete: bool,
    pub game_number: u32,
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

fn init_team_schedules(team_count: usize) -> Vec<TeamWeeklySchedule> {
    (0..team_count).map(|_| TeamWeeklySchedule::new()).collect()
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

    // Create tournament (Double Round Robin, Bo3, starting day 3)
    let team_names: Vec<String> = moba_teams.iter().map(|t| t.name().to_string()).collect();
    let tournament = Tournament::new(
        "Season 2025".to_string(),
        team_names,
        TournamentFormat::DoubleRoundRobin,
        BracketKind::Bo3,
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

    // Store champion names and details from data pack
    let champ_names: Vec<String> = pack.champions.iter().map(|c| c.name.clone()).collect();
    *state.champion_names.lock().unwrap() = champ_names;

    let detail_map: HashMap<String, ChampionDraftInfo> = pack
        .champions
        .iter()
        .map(|c| {
            (
                c.name.clone(),
                ChampionDraftInfo {
                    name: c.name.clone(),
                    class: c.class.clone(),
                    scaling: c.scaling.clone(),
                    tags: c.tags.clone(),
                    meta_tier: "B".to_string(),
                },
            )
        })
        .collect();
    *state.champion_detail_map.lock().unwrap() = detail_map;

    // Initialize team schedules (one per team)
    let team_count = gs.teams().len();
    *state.team_schedules.lock().unwrap() = init_team_schedules(team_count);
    *state.scrim_manager.lock().unwrap() = ScrimManager::new();

    // Store in memory
    *state.game_state.lock().unwrap() = Some(gs);
    *state.tournament.lock().unwrap() = Some(tournament);
    *state.moba_teams.lock().unwrap() = Some(moba_teams);

    Ok(info)
}

#[tauri::command]
fn load_save(name: String, state: State<'_, AppState>) -> Result<GameInfo, String> {
    let (gs, tournament_json, moba_teams_json, schedules_json, scrims_json) =
        SaveManager::load_save_all(&state.saves_dir, &name)
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

    // Restore team schedules from save, or initialize fresh if missing/corrupt
    let team_count = gs.teams().len();
    let schedules: Vec<TeamWeeklySchedule> = serde_json::from_str(&schedules_json)
        .ok()
        .filter(|v: &Vec<TeamWeeklySchedule>| v.len() == team_count)
        .unwrap_or_else(|| init_team_schedules(team_count));
    *state.team_schedules.lock().unwrap() = schedules;

    let scrim_mgr: ScrimManager =
        serde_json::from_str(&scrims_json).unwrap_or_else(|_| ScrimManager::new());
    *state.scrim_manager.lock().unwrap() = scrim_mgr;

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

    let s_lock = state.team_schedules.lock().unwrap();
    let schedules_json = serde_json::to_string(&*s_lock).unwrap_or_else(|_| "[]".to_string());

    let sm_lock = state.scrim_manager.lock().unwrap();
    let scrims_json = serde_json::to_string(&*sm_lock)
        .unwrap_or_else(|_| "{\"scrims\":[],\"next_id\":1}".to_string());

    SaveManager::create_save_all(
        &state.saves_dir,
        &name,
        gs,
        &tournament_json,
        &moba_teams_json,
        &schedules_json,
        &scrims_json,
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
        // Apply team schedule effects (scrims/solo-queue/rest) to player rosters
        {
            let schedules = state.team_schedules.lock().unwrap();
            let day_in_week = (gs.calendar().days_elapsed() % 7) as usize;
            for (team_idx, team) in gs.teams_mut().iter_mut().enumerate() {
                if let Some(sched) = schedules.get(team_idx) {
                    let today = sched.day(day_in_week);
                    ScheduleProcessor::apply_daily_effects(today, team.roster_mut());
                }
            }
        }

        // Resolve scrims scheduled for today
        if let Some(moba_teams) = m_lock.as_ref() {
            let today = gs.calendar().days_elapsed();
            let mut scrim_mgr = state.scrim_manager.lock().unwrap();
            let pending = scrim_mgr.pending_scrims_for_day(today);
            let config = MobaMatchConfig::default();

            for (scrim_id, game_count) in pending {
                // Look up team indices from names
                let scrim = scrim_mgr.scrim_by_id(scrim_id).unwrap();
                let home_name = scrim.home_team().to_string();
                let away_name = scrim.away_team().to_string();

                let home_idx = moba_teams.iter().position(|t| t.name() == home_name);
                let away_idx = moba_teams.iter().position(|t| t.name() == away_name);

                if let (Some(hi), Some(ai)) = (home_idx, away_idx) {
                    let home_attrs = extract_team_attrs(&moba_teams[hi]);
                    let away_attrs = extract_team_attrs(&moba_teams[ai]);

                    let mut hw = 0u32;
                    let mut aw = 0u32;
                    for _ in 0..game_count {
                        let result = MobaMatchEngine::simulate(
                            gs.rng_mut(),
                            &home_attrs,
                            &away_attrs,
                            &config,
                        );
                        match result.winner {
                            TeamSide::Blue => hw += 1,
                            TeamSide::Red => aw += 1,
                        }
                    }

                    scrim_mgr
                        .scrim_by_id_mut(scrim_id)
                        .unwrap()
                        .complete(hw, aw);
                }
            }
        }

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
                .map(|m| (m.id(), m.blue_team_idx(), m.red_team_idx(), m.bracket()))
                .collect();

            let config = MobaMatchConfig::default();
            let player_idx = gs.player_team_index();
            for (match_id, blue_idx, red_idx, bracket) in &todays {
                let blue_idx = *blue_idx;
                let red_idx = *red_idx;

                if blue_idx == player_idx || red_idx == player_idx {
                    // Stop! Don't simulate this one, it's the player's match!
                    continue;
                }

                if blue_idx < moba_teams.len() && red_idx < moba_teams.len() {
                    let blue_attrs = extract_team_attrs(&moba_teams[blue_idx]);
                    let red_attrs = extract_team_attrs(&moba_teams[red_idx]);

                    // Simulate full series (Bo1/Bo3/Bo5)
                    let wins_needed = bracket.wins_needed();
                    let mut bw = 0u32;
                    let mut rw = 0u32;
                    let mut last_duration = 0u32;
                    while bw < wins_needed && rw < wins_needed {
                        let result = MobaMatchEngine::simulate(
                            gs.rng_mut(),
                            &blue_attrs,
                            &red_attrs,
                            &config,
                        );
                        match result.winner {
                            TeamSide::Blue => bw += 1,
                            TeamSide::Red => rw += 1,
                        }
                        last_duration = result.duration_minutes;
                    }
                    tournament.record_result(*match_id, bw, rw);

                    let winner_name = if bw > rw {
                        moba_teams[blue_idx].name().to_string()
                    } else {
                        moba_teams[red_idx].name().to_string()
                    };
                    match_results.push(MatchResultInfo {
                        blue_team: moba_teams[blue_idx].name().to_string(),
                        red_team: moba_teams[red_idx].name().to_string(),
                        winner: winner_name,
                        duration_minutes: last_duration,
                    });
                }
            }

            // Add pre-match staff message if player has a match today
            if let Some(player_match) = todays
                .iter()
                .find(|(_, b, r, _)| *b == player_idx || *r == player_idx)
            {
                let opponent_idx = if player_match.1 == player_idx {
                    player_match.2
                } else {
                    player_match.1
                };
                let opponent_name = moba_teams
                    .get(opponent_idx)
                    .map(|t| t.name().to_string())
                    .unwrap_or_default();
                let msg = esm_core::inbox::Message::new(
                    format!("Pre-match Report: vs {}", opponent_name),
                    format!("Coach:\nWe're playing against {} today. Make sure you select the best activity schedule beforehand to manage player stamina.", opponent_name),
                    esm_core::inbox::MessagePriority::ReadOptional,
                    esm_core::inbox::MessageCategory::Staff,
                    day,
                );
                gs.inbox_mut().push(msg);
            }

            if !todays.is_empty() && tournament.is_complete() {
                let msg = esm_core::inbox::Message::new(
                    "Tournament Concluded".to_string(),
                    format!(
                        "The {} has concluded! Check the final standings.",
                        tournament.name()
                    ),
                    esm_core::inbox::MessagePriority::HardBlock,
                    esm_core::inbox::MessageCategory::News,
                    gs.calendar().days_elapsed(),
                );
                gs.inbox_mut().push(msg);
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
fn play_match_delegate(state: State<'_, AppState>) -> Result<GameInfo, String> {
    let mut lock = state.game_state.lock().unwrap();
    let gs = lock.as_mut().ok_or("No active game session")?;
    let mut t_lock = state.tournament.lock().unwrap();
    let m_lock = state.moba_teams.lock().unwrap();

    let mut match_results: Vec<MatchResultInfo> = Vec::new();

    if let (Some(tournament), Some(moba_teams)) = (t_lock.as_mut(), m_lock.as_ref()) {
        let player_idx = gs.player_team_index();
        let day = gs.calendar().days_elapsed();

        let player_match = tournament.matches_today(day).into_iter().find(|m| {
            (m.blue_team_idx() == player_idx || m.red_team_idx() == player_idx)
                && m.winner_team_idx().is_none()
        });

        if let Some(m) = player_match {
            let match_id = m.id();
            let blue_idx = m.blue_team_idx();
            let red_idx = m.red_team_idx();

            let config = MobaMatchConfig::default();
            let blue_attrs = extract_team_attrs(&moba_teams[blue_idx]);
            let red_attrs = extract_team_attrs(&moba_teams[red_idx]);

            let result = MobaMatchEngine::simulate(gs.rng_mut(), &blue_attrs, &red_attrs, &config);

            let (bw, rw) = match result.winner {
                TeamSide::Blue => (1u32, 0u32),
                TeamSide::Red => (0u32, 1u32),
            };
            tournament.record_result(match_id, bw, rw);

            let is_blue = blue_idx == player_idx;
            let player_won = (is_blue && bw > 0) || (!is_blue && rw > 0);
            gs.teams_mut()[player_idx].apply_match_result(player_won);

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

            // Same logic to conclude tournament if this was the last match
            if tournament.is_complete() {
                let msg = esm_core::inbox::Message::new(
                    "Tournament Concluded".to_string(),
                    format!(
                        "The {} has concluded! Check the final standings.",
                        tournament.name()
                    ),
                    esm_core::inbox::MessagePriority::HardBlock,
                    esm_core::inbox::MessageCategory::News,
                    gs.calendar().days_elapsed(),
                );
                gs.inbox_mut().push(msg);
            }
        } else {
            return Err("No pending match today".to_string());
        }
    }

    let tournament_ref = t_lock.as_ref();
    let mut info = game_info_from_state(gs, tournament_ref.unwrap_or(&empty_tournament()));
    info.match_results = match_results;
    Ok(info)
}

// ---------------------------------------------------------------------------
// Draft commands
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct StartDraftParams {
    pub player_side: String, // "blue" or "red"
    pub format: String,      // "three_ban", "five_ban", "fearless"
    #[serde(default)]
    pub fearless_bans: Vec<String>, // champions picked in previous series games (Fearless mode)
    #[serde(default)]
    pub blue_team: String,
    #[serde(default)]
    pub red_team: String,
}

#[tauri::command]
fn start_draft(
    params: StartDraftParams,
    state: State<'_, AppState>,
) -> Result<DraftSessionState, String> {
    let champion_pool = state.champion_names.lock().unwrap().clone();
    if champion_pool.is_empty() {
        return Err("No champions loaded".to_string());
    }

    let player_side = match params.player_side.as_str() {
        "blue" => DraftTeamSide::Blue,
        "red" => DraftTeamSide::Red,
        _ => return Err(format!("Invalid side: {}", params.player_side)),
    };

    let format = match params.format.as_str() {
        "three_ban" => DraftFormat::ThreeBan,
        "five_ban" => DraftFormat::FiveBan,
        "fearless" => DraftFormat::Fearless,
        _ => return Err(format!("Invalid format: {}", params.format)),
    };

    // For Fearless mode, remove previously-picked champions from the pool
    let pool: Vec<String> = if !params.fearless_bans.is_empty() {
        champion_pool
            .into_iter()
            .filter(|c| !params.fearless_bans.contains(c))
            .collect()
    } else {
        champion_pool
    };

    // Build AI champion evaluations from current patch meta tiers.
    // Mastery map is empty for now (defaults to Bronze); will be populated
    // once champion pool mastery is wired into the game flow.
    let patch = state.current_patch.lock().unwrap();
    let mastery_map: HashMap<String, esm_models::champion::MasteryLevel> = HashMap::new();
    let evals = build_champion_evals(&pool, &patch, &mastery_map);
    drop(patch);

    // Use the game's RNG for AI draft randomization
    let gs_lock = state.game_state.lock().unwrap();
    let rng_seed = gs_lock.as_ref().map(|gs| gs.rng().state()).unwrap_or(42);
    drop(gs_lock);
    let rng = esm_core::rng::GameRng::from_seed(rng_seed);

    let mut session = DraftSession::with_evals(format, player_side, pool, evals, rng);

    // Enrich session with champion details
    let detail_map = state.champion_detail_map.lock().unwrap().clone();
    session.set_champion_details(detail_map);

    // Enrich session with team/player info from moba_teams
    let m_lock = state.moba_teams.lock().unwrap();
    if let Some(moba_teams) = m_lock.as_ref() {
        let find_team_players = |team_name: &str| -> Vec<DraftPlayerInfo> {
            moba_teams
                .iter()
                .find(|t| t.name() == team_name)
                .map(|t| {
                    t.roster()
                        .iter()
                        .map(|p| DraftPlayerInfo {
                            nickname: p.nickname().to_string(),
                            role: format!("{:?}", p.roles().primary()),
                        })
                        .collect()
                })
                .unwrap_or_default()
        };
        let blue_players = find_team_players(&params.blue_team);
        let red_players = find_team_players(&params.red_team);
        session.set_team_info(
            params.blue_team.clone(),
            params.red_team.clone(),
            blue_players,
            red_players,
        );
    }
    drop(m_lock);

    session.run_ai_turns();
    let draft_state = session.state();

    *state.draft_session.lock().unwrap() = Some(session);
    Ok(draft_state)
}

#[tauri::command]
fn draft_hover(champion: String, state: State<'_, AppState>) -> Result<DraftSessionState, String> {
    let mut lock = state.draft_session.lock().unwrap();
    let session = lock.as_mut().ok_or("No active draft session")?;

    session
        .hover(champion)
        .map_err(|e| format!("Draft error: {:?}", e))?;
    Ok(session.state())
}

#[tauri::command]
fn draft_lock(state: State<'_, AppState>) -> Result<DraftSessionState, String> {
    let mut lock = state.draft_session.lock().unwrap();
    let session = lock.as_mut().ok_or("No active draft session")?;

    session
        .lock()
        .map_err(|e| format!("Draft error: {:?}", e))?;
    // After player locks, run AI turns
    session.run_ai_turns();
    Ok(session.state())
}

#[tauri::command]
fn get_draft_state(state: State<'_, AppState>) -> Result<DraftSessionState, String> {
    let lock = state.draft_session.lock().unwrap();
    let session = lock.as_ref().ok_or("No active draft session")?;
    Ok(session.state())
}

#[tauri::command]
fn auto_draft_complete(state: State<'_, AppState>) -> Result<DraftSessionState, String> {
    let mut ds = state.draft_session.lock().unwrap();
    let session = ds.as_mut().ok_or("No active draft session")?;
    while !session.is_complete() {
        session.force_ai_act();
    }
    Ok(session.state())
}

#[tauri::command]
fn draft_swap_picks(
    a: usize,
    b: usize,
    state: State<'_, AppState>,
) -> Result<DraftSessionState, String> {
    let mut lock = state.draft_session.lock().unwrap();
    let session = lock.as_mut().ok_or("No active draft session")?;
    session
        .swap_picks(a, b)
        .map_err(|e| format!("Swap error: {:?}", e))?;
    Ok(session.state())
}

// ---------------------------------------------------------------------------
// Player Talk commands (between-match motivational system)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct ApplyTalkParams {
    pub player_index: usize,
    pub talk: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerStateInfo {
    pub nickname: String,
    pub role: String,
    pub stamina: u8,
    pub morale: u8,
    pub confidence: String,
    pub satisfaction: u8,
}

fn confidence_str(c: esm_models::player::Confidence) -> &'static str {
    match c {
        esm_models::player::Confidence::Slumping => "slumping",
        esm_models::player::Confidence::Neutral => "neutral",
        esm_models::player::Confidence::Confident => "confident",
        esm_models::player::Confidence::Hyped => "hyped",
    }
}

fn role_str(r: esm_models::moba::player::MobaRole) -> &'static str {
    match r {
        esm_models::moba::player::MobaRole::Top => "Top",
        esm_models::moba::player::MobaRole::Jungle => "Jungle",
        esm_models::moba::player::MobaRole::Mid => "Mid",
        esm_models::moba::player::MobaRole::Bot => "Bot",
        esm_models::moba::player::MobaRole::Support => "Support",
    }
}

fn parse_talk(s: &str) -> Result<PlayerTalk, String> {
    match s {
        "motivate" => Ok(PlayerTalk::Motivate),
        "calm" => Ok(PlayerTalk::Calm),
        "strategize" => Ok(PlayerTalk::Strategize),
        "rest" => Ok(PlayerTalk::Rest),
        _ => Err(format!("Unknown talk type: {s}")),
    }
}

fn player_state_info(player: &esm_models::moba::player::MobaPlayer) -> PlayerStateInfo {
    let st = player.state();
    PlayerStateInfo {
        nickname: player.nickname().to_string(),
        role: role_str(player.roles().primary()).to_string(),
        stamina: st.stamina.value(),
        morale: st.morale.value(),
        confidence: confidence_str(st.confidence).to_string(),
        satisfaction: st.satisfaction.value(),
    }
}

#[tauri::command]
fn get_roster_state(state: State<'_, AppState>) -> Result<Vec<PlayerStateInfo>, String> {
    let gs = state.game_state.lock().unwrap();
    let gs = gs.as_ref().ok_or("No active game session")?;
    let m_lock = state.moba_teams.lock().unwrap();
    let moba_teams = m_lock.as_ref().ok_or("No teams loaded")?;
    let idx = gs.player_team_index();
    let team = &moba_teams[idx];
    Ok(team.roster().iter().map(player_state_info).collect())
}

#[tauri::command]
fn apply_player_talk(
    state: State<'_, AppState>,
    params: ApplyTalkParams,
) -> Result<Vec<PlayerStateInfo>, String> {
    let gs = state.game_state.lock().unwrap();
    let gs = gs.as_ref().ok_or("No active game session")?;
    let mut m_lock = state.moba_teams.lock().unwrap();
    let moba_teams = m_lock.as_mut().ok_or("No teams loaded")?;
    let idx = gs.player_team_index();
    let team = &mut moba_teams[idx];

    let talk = parse_talk(&params.talk)?;
    let roster = team.roster_mut();
    if params.player_index >= roster.len() {
        return Err(format!(
            "Player index {} out of range (roster has {})",
            params.player_index,
            roster.len()
        ));
    }
    roster[params.player_index].state_mut().apply_talk(talk);

    Ok(team.roster().iter().map(player_state_info).collect())
}

// ---------------------------------------------------------------------------
// Tactics commands
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct SetTacticsParams {
    pub playstyle: String,
    pub focus: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TacticsInfo {
    pub playstyle: String,
    pub focus: String,
}

fn parse_playstyle(s: &str) -> Result<Playstyle, String> {
    match s {
        "aggressive" => Ok(Playstyle::Aggressive),
        "balanced" => Ok(Playstyle::Balanced),
        "defensive" => Ok(Playstyle::Defensive),
        _ => Err(format!("Unknown playstyle: {s}")),
    }
}

fn parse_focus(s: &str) -> Result<Focus, String> {
    match s {
        "teamfight" => Ok(Focus::Teamfight),
        "splitpush" => Ok(Focus::Splitpush),
        "objective" => Ok(Focus::Objective),
        _ => Err(format!("Unknown focus: {s}")),
    }
}

fn playstyle_str(p: Playstyle) -> &'static str {
    match p {
        Playstyle::Aggressive => "aggressive",
        Playstyle::Balanced => "balanced",
        Playstyle::Defensive => "defensive",
    }
}

fn focus_str(f: Focus) -> &'static str {
    match f {
        Focus::Teamfight => "teamfight",
        Focus::Splitpush => "splitpush",
        Focus::Objective => "objective",
    }
}

#[tauri::command]
fn set_tactics(
    state: State<'_, AppState>,
    params: SetTacticsParams,
) -> Result<TacticsInfo, String> {
    let playstyle = parse_playstyle(&params.playstyle)?;
    let focus = parse_focus(&params.focus)?;
    let mut lock = state.match_tactics.lock().unwrap();
    lock.playstyle = playstyle;
    lock.focus = focus;
    Ok(TacticsInfo {
        playstyle: playstyle_str(lock.playstyle).to_string(),
        focus: focus_str(lock.focus).to_string(),
    })
}

#[tauri::command]
fn get_tactics(state: State<'_, AppState>) -> TacticsInfo {
    let lock = state.match_tactics.lock().unwrap();
    TacticsInfo {
        playstyle: playstyle_str(lock.playstyle).to_string(),
        focus: focus_str(lock.focus).to_string(),
    }
}

// ---------------------------------------------------------------------------
// Scheduling DTOs & commands
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct ScheduleSlotInfo {
    pub time_slot: String,
    pub entry_type: String, // "free", "scrim", "solo_queue", "rest"
    pub scrim_id: Option<u32>,
    pub opponent: Option<String>,
    pub players: Option<Vec<usize>>,
    pub focus: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DayScheduleInfo {
    pub day_index: usize,
    pub slots: Vec<ScheduleSlotInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WeekScheduleInfo {
    pub days: Vec<DayScheduleInfo>,
    pub total_scrims: usize,
    pub occupied_slots: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScrimInfo {
    pub id: u32,
    pub home_team: String,
    pub away_team: String,
    pub scheduled_day: u32,
    pub time_slot: String,
    pub game_count: u32,
    pub draft_rules: String,
    pub status: String,
    pub home_wins: u32,
    pub away_wins: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScheduleScrimParams {
    pub away_team_index: usize,
    pub scheduled_day: u32,
    pub time_slot: String,
    pub game_count: u32,
    pub draft_rules: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScheduleSoloQueueParams {
    pub day_index: usize,
    pub time_slot: String,
    pub players: Vec<usize>,
    pub focus: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScheduleRestParams {
    pub day_index: usize,
    pub time_slot: String,
}

fn parse_time_slot(s: &str) -> Result<TimeSlot, String> {
    s.parse::<TimeSlot>()
}

fn time_slot_str(ts: TimeSlot) -> &'static str {
    ts.as_str()
}

fn parse_solo_queue_focus(s: &str) -> Result<SoloQueueFocus, String> {
    match s {
        "champions" => Ok(SoloQueueFocus::Champions),
        "tactics" => Ok(SoloQueueFocus::Tactics),
        "mechanics" => Ok(SoloQueueFocus::Mechanics),
        "mentality" => Ok(SoloQueueFocus::Mentality),
        _ => Err(format!("Unknown focus: {s}")),
    }
}

fn focus_to_str(f: SoloQueueFocus) -> &'static str {
    match f {
        SoloQueueFocus::Champions => "champions",
        SoloQueueFocus::Tactics => "tactics",
        SoloQueueFocus::Mechanics => "mechanics",
        SoloQueueFocus::Mentality => "mentality",
    }
}

fn parse_draft_rules(s: &str) -> Result<ScrimDraftRules, String> {
    match s {
        "standard" => Ok(ScrimDraftRules::Standard),
        "fearless" => Ok(ScrimDraftRules::Fearless),
        _ => Err(format!("Unknown draft rules: {s}")),
    }
}

fn slot_to_info(
    ts: TimeSlot,
    entry: Option<&ScheduleEntry>,
    scrim_mgr: &ScrimManager,
) -> ScheduleSlotInfo {
    match entry {
        None => ScheduleSlotInfo {
            time_slot: time_slot_str(ts).to_string(),
            entry_type: "free".to_string(),
            scrim_id: None,
            opponent: None,
            players: None,
            focus: None,
        },
        Some(ScheduleEntry::Scrim { scrim_id }) => {
            let opponent = scrim_mgr
                .scrim_by_id(*scrim_id)
                .map(|s| s.away_team().to_string());
            ScheduleSlotInfo {
                time_slot: time_slot_str(ts).to_string(),
                entry_type: "scrim".to_string(),
                scrim_id: Some(*scrim_id),
                opponent,
                players: None,
                focus: None,
            }
        }
        Some(ScheduleEntry::SoloQueue { players, focus }) => ScheduleSlotInfo {
            time_slot: time_slot_str(ts).to_string(),
            entry_type: "solo_queue".to_string(),
            scrim_id: None,
            opponent: None,
            players: Some(players.clone()),
            focus: Some(focus_to_str(*focus).to_string()),
        },
        Some(ScheduleEntry::Rest) => ScheduleSlotInfo {
            time_slot: time_slot_str(ts).to_string(),
            entry_type: "rest".to_string(),
            scrim_id: None,
            opponent: None,
            players: None,
            focus: None,
        },
    }
}

fn week_schedule_to_info(
    schedule: &TeamWeeklySchedule,
    scrim_mgr: &ScrimManager,
) -> WeekScheduleInfo {
    let days = (0..7)
        .map(|i| {
            let day = schedule.day(i);
            let slots = TimeSlot::ALL
                .iter()
                .map(|&ts| slot_to_info(ts, day.get(ts), scrim_mgr))
                .collect();
            DayScheduleInfo {
                day_index: i,
                slots,
            }
        })
        .collect();
    WeekScheduleInfo {
        days,
        total_scrims: schedule.total_scrims(),
        occupied_slots: schedule.occupied_slots(),
    }
}

#[tauri::command]
fn get_team_schedule(state: State<'_, AppState>) -> Result<WeekScheduleInfo, String> {
    let gs = state.game_state.lock().unwrap();
    let gs = gs.as_ref().ok_or("No active game session")?;
    let schedules = state.team_schedules.lock().unwrap();
    let scrim_mgr = state.scrim_manager.lock().unwrap();
    let idx = gs.player_team_index();
    if idx >= schedules.len() {
        return Err("Schedule not initialized".to_string());
    }
    Ok(week_schedule_to_info(&schedules[idx], &scrim_mgr))
}

#[tauri::command]
fn schedule_scrim(
    state: State<'_, AppState>,
    params: ScheduleScrimParams,
) -> Result<ScrimInfo, String> {
    let gs = state.game_state.lock().unwrap();
    let gs = gs.as_ref().ok_or("No active game session")?;
    let mut schedules = state.team_schedules.lock().unwrap();
    let mut scrim_mgr = state.scrim_manager.lock().unwrap();

    let home_idx = gs.player_team_index();
    let away_idx = params.away_team_index;
    let home_name = gs.teams()[home_idx].name().to_string();
    let away_name = gs.teams()[away_idx].name().to_string();
    let time_slot = parse_time_slot(&params.time_slot)?;
    let draft_rules = parse_draft_rules(&params.draft_rules)?;
    let current_day = gs.calendar().days_elapsed();

    let scrim_id = scrim_mgr
        .schedule_scrim(
            &mut schedules,
            home_idx,
            away_idx,
            &home_name,
            &away_name,
            params.scheduled_day,
            time_slot,
            params.game_count,
            draft_rules,
            current_day,
        )
        .map_err(|e| format!("Schedule error: {:?}", e))?;

    let scrim = scrim_mgr.scrim_by_id(scrim_id).unwrap();
    Ok(ScrimInfo {
        id: scrim.id(),
        home_team: scrim.home_team().to_string(),
        away_team: scrim.away_team().to_string(),
        scheduled_day: scrim.scheduled_day(),
        time_slot: time_slot_str(scrim.time_slot()).to_string(),
        game_count: scrim.game_count(),
        draft_rules: format!("{:?}", scrim.draft_rules()),
        status: format!("{:?}", scrim.status()),
        home_wins: scrim.home_wins(),
        away_wins: scrim.away_wins(),
    })
}

#[tauri::command]
fn cancel_scrim(state: State<'_, AppState>, scrim_id: u32) -> Result<String, String> {
    let gs = state.game_state.lock().unwrap();
    let gs = gs.as_ref().ok_or("No active game session")?;
    let mut schedules = state.team_schedules.lock().unwrap();
    let mut scrim_mgr = state.scrim_manager.lock().unwrap();

    let scrim = scrim_mgr.scrim_by_id(scrim_id).ok_or("Scrim not found")?;
    let home_idx = gs
        .teams()
        .iter()
        .position(|t| t.name() == scrim.home_team())
        .ok_or("Home team not found")?;
    let away_idx = gs
        .teams()
        .iter()
        .position(|t| t.name() == scrim.away_team())
        .ok_or("Away team not found")?;
    let current_day = gs.calendar().days_elapsed();

    scrim_mgr
        .cancel_scrim(scrim_id, &mut schedules, home_idx, away_idx, current_day)
        .map_err(|e| format!("Cancel error: {:?}", e))?;

    Ok("Scrim cancelled".to_string())
}

#[tauri::command]
fn schedule_solo_queue(
    state: State<'_, AppState>,
    params: ScheduleSoloQueueParams,
) -> Result<WeekScheduleInfo, String> {
    let gs = state.game_state.lock().unwrap();
    let gs = gs.as_ref().ok_or("No active game session")?;
    let mut schedules = state.team_schedules.lock().unwrap();
    let scrim_mgr = state.scrim_manager.lock().unwrap();

    let idx = gs.player_team_index();
    let time_slot = parse_time_slot(&params.time_slot)?;
    let focus = parse_solo_queue_focus(&params.focus)?;

    if !schedules[idx].day(params.day_index).is_free(time_slot) {
        return Err("Slot is already occupied".to_string());
    }

    schedules[idx].day_mut(params.day_index).set(
        time_slot,
        ScheduleEntry::SoloQueue {
            players: params.players,
            focus,
        },
    );

    Ok(week_schedule_to_info(&schedules[idx], &scrim_mgr))
}

#[tauri::command]
fn schedule_rest(
    state: State<'_, AppState>,
    params: ScheduleRestParams,
) -> Result<WeekScheduleInfo, String> {
    let gs = state.game_state.lock().unwrap();
    let gs = gs.as_ref().ok_or("No active game session")?;
    let mut schedules = state.team_schedules.lock().unwrap();
    let scrim_mgr = state.scrim_manager.lock().unwrap();

    let idx = gs.player_team_index();
    let time_slot = parse_time_slot(&params.time_slot)?;

    if !schedules[idx].day(params.day_index).is_free(time_slot) {
        return Err("Slot is already occupied".to_string());
    }

    schedules[idx]
        .day_mut(params.day_index)
        .set(time_slot, ScheduleEntry::Rest);

    Ok(week_schedule_to_info(&schedules[idx], &scrim_mgr))
}

#[tauri::command]
fn clear_schedule_slot(
    state: State<'_, AppState>,
    day_index: usize,
    time_slot: String,
) -> Result<WeekScheduleInfo, String> {
    let gs = state.game_state.lock().unwrap();
    let gs = gs.as_ref().ok_or("No active game session")?;
    let mut schedules = state.team_schedules.lock().unwrap();
    let scrim_mgr = state.scrim_manager.lock().unwrap();

    let idx = gs.player_team_index();
    let ts = parse_time_slot(&time_slot)?;

    schedules[idx].day_mut(day_index).clear(ts);

    Ok(week_schedule_to_info(&schedules[idx], &scrim_mgr))
}

#[tauri::command]
fn get_scrims_list(state: State<'_, AppState>) -> Result<Vec<ScrimInfo>, String> {
    let gs = state.game_state.lock().unwrap();
    let gs = gs.as_ref().ok_or("No active game session")?;
    let scrim_mgr = state.scrim_manager.lock().unwrap();
    let team_name = gs.player_team().name();

    let scrims: Vec<ScrimInfo> = scrim_mgr
        .scrims_for_team(team_name)
        .iter()
        .map(|s| ScrimInfo {
            id: s.id(),
            home_team: s.home_team().to_string(),
            away_team: s.away_team().to_string(),
            scheduled_day: s.scheduled_day(),
            time_slot: time_slot_str(s.time_slot()).to_string(),
            game_count: s.game_count(),
            draft_rules: format!("{:?}", s.draft_rules()),
            status: format!("{:?}", s.status()),
            home_wins: s.home_wins(),
            away_wins: s.away_wins(),
        })
        .collect();

    Ok(scrims)
}

// ---------------------------------------------------------------------------
// Match simulation command
// ---------------------------------------------------------------------------

fn event_kind_label(kind: &MatchEventKind) -> &'static str {
    match kind {
        MatchEventKind::FarmTick => "farm",
        MatchEventKind::SoloKill { .. } => "solo_kill",
        MatchEventKind::Teamfight { .. } => "teamfight",
        MatchEventKind::TowerDestroyed { .. } => "tower",
        MatchEventKind::DragonKill { .. } => "dragon",
        MatchEventKind::HeraldKill { .. } => "herald",
        MatchEventKind::BaronKill { .. } => "baron",
        MatchEventKind::InhibitorDestroyed { .. } => "inhibitor",
        MatchEventKind::NexusDestroyed { .. } => "nexus",
        MatchEventKind::MultiKill { .. } => "multi_kill",
        MatchEventKind::KillingSpree { .. } => "killing_spree",
    }
}

fn match_result_to_info(
    result: &MobaMatchResult,
    blue_name: &str,
    red_name: &str,
) -> SimulateMatchResultInfo {
    let events: Vec<MatchEventInfo> = result
        .events
        .iter()
        .filter(|e| !matches!(e.kind(), MatchEventKind::FarmTick))
        .map(|e| {
            let snapshot = e.snapshot().map(|s| {
                let convert_players = |players: &[esm_engine::moba_match::event::PlayerSnapshot]| -> Vec<PlayerSnapshotInfo> {
                    players.iter().map(|p| PlayerSnapshotInfo {
                        kills: p.kills,
                        deaths: p.deaths,
                        assists: p.assists,
                        cs: p.cs,
                        gold: p.gold,
                        is_dead: p.is_dead,
                    }).collect()
                };
                GameSnapshotInfo {
                    blue_players: convert_players(&s.blue_players),
                    red_players: convert_players(&s.red_players),
                    blue_team_gold: s.blue_team_gold,
                    red_team_gold: s.red_team_gold,
                    dragons_blue: s.dragons_blue,
                    dragons_red: s.dragons_red,
                    baron_alive: s.baron_alive,
                    baron_timer: s.baron_timer,
                    dragon_timer: s.dragon_timer,
                    herald_available: s.herald_available,
                }
            });
            MatchEventInfo {
                minute: e.minute(),
                phase: format!("{:?}", e.phase()),
                kind: event_kind_label(e.kind()).to_string(),
                commentary: e.commentary().map(|c| c.text().to_string()),
                snapshot,
            }
        })
        .collect();

    let winner = match result.winner {
        TeamSide::Blue => blue_name.to_string(),
        TeamSide::Red => red_name.to_string(),
    };

    SimulateMatchResultInfo {
        winner,
        duration_minutes: result.duration_minutes,
        blue_team: blue_name.to_string(),
        red_team: red_name.to_string(),
        blue_gold: result.blue_team_gold,
        red_gold: result.red_team_gold,
        events,
    }
}

/// Simulate one game of the player's current series.
/// Uses add_game_win to track incremental wins. Only applies match_result
/// effects when the series is complete.
#[tauri::command]
fn simulate_match(state: State<'_, AppState>) -> Result<SimulateMatchResultInfo, String> {
    let mut lock = state.game_state.lock().unwrap();
    let gs = lock.as_mut().ok_or("No active game session")?;
    let mut t_lock = state.tournament.lock().unwrap();
    let m_lock = state.moba_teams.lock().unwrap();

    if let (Some(tournament), Some(moba_teams)) = (t_lock.as_mut(), m_lock.as_ref()) {
        let player_idx = gs.player_team_index();
        let day = gs.calendar().days_elapsed();

        let player_match = tournament.matches_today(day).into_iter().find(|m| {
            (m.blue_team_idx() == player_idx || m.red_team_idx() == player_idx)
                && m.winner_team_idx().is_none()
        });

        if let Some(m) = player_match {
            let match_id = m.id();
            let blue_idx = m.blue_team_idx();
            let red_idx = m.red_team_idx();

            let config = MobaMatchConfig::default();
            let blue_attrs = extract_team_attrs(&moba_teams[blue_idx]);
            let red_attrs = extract_team_attrs(&moba_teams[red_idx]);

            let tactics = state.match_tactics.lock().unwrap().clone();
            let result = MobaMatchEngine::simulate_with_tactics(
                gs.rng_mut(),
                &blue_attrs,
                &red_attrs,
                &config,
                &tactics,
            );

            let blue_won = matches!(result.winner, TeamSide::Blue);
            let series_complete = tournament.add_game_win(match_id, blue_won);

            // Only apply team effects when the series is fully decided
            if series_complete {
                let is_blue = blue_idx == player_idx;
                let player_won = (is_blue && blue_won) || (!is_blue && !blue_won);
                // For Bo3, check actual series winner
                // Re-fetch match to get final wins
                let final_match = tournament
                    .matches_today(day)
                    .into_iter()
                    .find(|m| m.id() == match_id);
                if let Some(fm) = final_match {
                    let player_won_series = if fm.blue_team_idx() == player_idx {
                        fm.blue_wins() > fm.red_wins()
                    } else {
                        fm.red_wins() > fm.blue_wins()
                    };
                    gs.teams_mut()[player_idx].apply_match_result(player_won_series);
                } else {
                    gs.teams_mut()[player_idx].apply_match_result(player_won);
                }

                if tournament.is_complete() {
                    let msg = esm_core::inbox::Message::new(
                        "Tournament Concluded".to_string(),
                        format!(
                            "The {} has concluded! Check the final standings.",
                            tournament.name()
                        ),
                        esm_core::inbox::MessagePriority::HardBlock,
                        esm_core::inbox::MessageCategory::News,
                        gs.calendar().days_elapsed(),
                    );
                    gs.inbox_mut().push(msg);
                }
            }

            let info = match_result_to_info(
                &result,
                moba_teams[blue_idx].name(),
                moba_teams[red_idx].name(),
            );

            return Ok(info);
        }
    }

    Err("No pending match today".to_string())
}

/// Get the current series state for the player's match today.
#[tauri::command]
fn get_series_info(state: State<'_, AppState>) -> Result<SeriesInfo, String> {
    let lock = state.game_state.lock().unwrap();
    let gs = lock.as_ref().ok_or("No active game session")?;
    let t_lock = state.tournament.lock().unwrap();
    let m_lock = state.moba_teams.lock().unwrap();

    if let (Some(tournament), Some(moba_teams)) = (t_lock.as_ref(), m_lock.as_ref()) {
        let player_idx = gs.player_team_index();
        let day = gs.calendar().days_elapsed();

        let player_match = tournament
            .matches_today(day)
            .into_iter()
            .find(|m| m.blue_team_idx() == player_idx || m.red_team_idx() == player_idx);

        if let Some(m) = player_match {
            let blue_idx = m.blue_team_idx();
            let red_idx = m.red_team_idx();
            let blue_name = moba_teams
                .get(blue_idx)
                .map(|t| t.name().to_string())
                .unwrap_or_default();
            let red_name = moba_teams
                .get(red_idx)
                .map(|t| t.name().to_string())
                .unwrap_or_default();

            return Ok(SeriesInfo {
                match_id: m.id(),
                blue_team: blue_name,
                red_team: red_name,
                blue_wins: m.blue_wins(),
                red_wins: m.red_wins(),
                wins_needed: m.bracket().wins_needed(),
                is_complete: m.winner_team_idx().is_some(),
                game_number: m.blue_wins() + m.red_wins() + 1,
            });
        }
    }

    Err("No match today".to_string())
}

#[tauri::command]
fn resolve_message(msg_id: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut lock = state.game_state.lock().unwrap();
    let gs = lock.as_mut().ok_or("No active game session")?;

    // msg_id is formatted as "msg_{index}"
    if let Some(idx_str) = msg_id.strip_prefix("msg_") {
        if let Ok(idx) = idx_str.parse::<usize>() {
            gs.inbox_mut().resolve_at(idx);
            return Ok(());
        }
    }

    Err(format!("Invalid message ID: {msg_id}"))
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

    let player_idx = gs.player_team_index();
    let is_match_day = tournament
        .matches_today(gs.calendar().days_elapsed())
        .iter()
        .any(|m| {
            (m.blue_team_idx() == player_idx || m.red_team_idx() == player_idx)
                && m.winner_team_idx().is_none()
        });

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

fn extract_team_attrs(moba_team: &MobaTeam) -> Vec<MatchPlayerSimulationData> {
    moba_team
        .roster()
        .iter()
        .map(|p| {
            let a = p.attributes();
            let attributes = [
                a.endurance.value(),
                a.reaction_time.value(),
                a.decision_making.value(),
                a.clutch.value(),
                a.discipline.value(),
                a.tilt_resistance.value(),
                a.mechanics.value(),
                a.vision_control.value(),
                a.teamfighting.value(),
            ];

            MatchPlayerSimulationData {
                attributes,
                stamina: p.state().stamina.value(),
                morale: p.state().morale.value(),
                mastery_multiplier: 1.0, // UI simulation might not specify full champion drafts yet
            }
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
            play_match_delegate,
            get_roster,
            get_inbox,
            resolve_message,
            get_game_info,
            get_standings,
            get_schedule,
            start_draft,
            draft_hover,
            draft_lock,
            get_draft_state,
            simulate_match,
            get_series_info,
            set_tactics,
            get_tactics,
            get_roster_state,
            apply_player_talk,
            get_team_schedule,
            schedule_scrim,
            cancel_scrim,
            schedule_solo_queue,
            schedule_rest,
            clear_schedule_slot,
            get_scrims_list,
            auto_draft_complete,
            draft_swap_picks,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
