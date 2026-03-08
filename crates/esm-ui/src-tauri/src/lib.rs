use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::State;

use esm_core::calendar::{Calendar, DayPhase};
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

fn push_role_if_missing(roles: &mut Vec<String>, role: &str) {
    if !roles.iter().any(|existing| existing == role) {
        roles.push(role.to_string());
    }
}

fn preferred_roles_for_champion(champion: &esm_data::datapack::ChampionData) -> Vec<String> {
    if !champion.preferred_roles.is_empty() {
        return champion.preferred_roles.clone();
    }

    let mut roles = Vec::new();
    let has_tag = |needle: &str| champion.tags.iter().any(|tag| tag == needle);

    match champion.class.as_str() {
        "Support" => push_role_if_missing(&mut roles, "Support"),
        "Marksman" => push_role_if_missing(&mut roles, "Bot"),
        "Mage" => {
            push_role_if_missing(&mut roles, "Mid");
            if has_tag("Peel") {
                push_role_if_missing(&mut roles, "Support");
            }
        }
        "Assassin" => {
            push_role_if_missing(&mut roles, "Mid");
            push_role_if_missing(&mut roles, "Jungle");
        }
        "Tank" => {
            push_role_if_missing(&mut roles, "Top");
            if has_tag("Engage") {
                push_role_if_missing(&mut roles, "Jungle");
            }
            if has_tag("Peel") {
                push_role_if_missing(&mut roles, "Support");
            }
        }
        "Fighter" => {
            push_role_if_missing(&mut roles, "Top");
            if has_tag("Engage") {
                push_role_if_missing(&mut roles, "Jungle");
            }
            if has_tag("Poke") {
                push_role_if_missing(&mut roles, "Mid");
            }
        }
        _ => {}
    }

    roles
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
    pub day_of_week: String,
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
    pub blue_towers: u32,
    pub red_towers: u32,
    pub blue_inhibitors: u32,
    pub red_inhibitors: u32,
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
pub struct MatchRosterEntry {
    pub nickname: String,
    pub role: String,
    pub champion: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SimulateMatchResultInfo {
    pub winner: String,
    pub duration_minutes: u32,
    pub blue_team: String,
    pub red_team: String,
    pub blue_gold: u32,
    pub red_gold: u32,
    pub blue_roster: Vec<MatchRosterEntry>,
    pub red_roster: Vec<MatchRosterEntry>,
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

fn schedule_span_days(tournament: &Tournament) -> usize {
    tournament
        .schedule()
        .matches()
        .iter()
        .map(|m| m.scheduled_day() as usize)
        .max()
        .unwrap_or(0)
        + 1
}

fn init_team_schedules(team_count: usize, total_days: usize) -> Vec<TeamWeeklySchedule> {
    (0..team_count)
        .map(|_| TeamWeeklySchedule::with_total_days(total_days))
        .collect()
}

fn push_onboarding_messages(gs: &mut GameState) {
    let day = gs.calendar().days_elapsed();
    gs.inbox_mut().push(esm_core::inbox::Message::new(
        "Welcome from the Board".to_string(),
        "Board of Directors:\nWelcome to your new role. We expect disciplined weekly planning, steady results, and clear communication through the inbox. Resolve important messages promptly and keep the team on schedule.".
            to_string(),
        esm_core::inbox::MessagePriority::RequiresResponse,
        esm_core::inbox::MessageCategory::Board,
        day,
    ));
    gs.inbox_mut().push(esm_core::inbox::Message::new(
        "Assistant Coach Briefing".to_string(),
        "Assistant Coach:\nWelcome aboard. In your first days, check the schedule page, review upcoming opponents, and manage practice carefully around match days. I will flag important prep through the inbox.".
            to_string(),
        esm_core::inbox::MessagePriority::RequiresResponse,
        esm_core::inbox::MessageCategory::Staff,
        day,
    ));
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

    let mut gs = GameState::new(2025, seed, esport_type, manager, params.team_index, teams);

    // Create tournament (Double Round Robin, Bo3, starting day 3)
    let team_names: Vec<String> = moba_teams.iter().map(|t| t.name().to_string()).collect();
    let tournament = Tournament::new(
        "Season 2025".to_string(),
        team_names,
        TournamentFormat::DoubleRoundRobin,
        BracketKind::Bo3,
        3,
    );
    let schedule_days = schedule_span_days(&tournament);
    push_onboarding_messages(&mut gs);

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
                    preferred_roles: preferred_roles_for_champion(c),
                    meta_tier: "B".to_string(),
                    best_mastery: None,
                },
            )
        })
        .collect();
    *state.champion_detail_map.lock().unwrap() = detail_map;

    // Initialize team schedules (one per team)
    let team_count = gs.teams().len();
    *state.team_schedules.lock().unwrap() = init_team_schedules(team_count, schedule_days);
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
    let schedule_days = tournament.as_ref().map(schedule_span_days).unwrap_or(7);
    let schedules: Vec<TeamWeeklySchedule> = serde_json::from_str(&schedules_json)
        .ok()
        .filter(|v: &Vec<TeamWeeklySchedule>| v.len() == team_count)
        .unwrap_or_else(|| init_team_schedules(team_count, schedule_days));
    let mut schedules = schedules;
    for schedule in &mut schedules {
        schedule.day_mut(schedule_days.saturating_sub(1));
    }
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

#[derive(Debug, Clone, Deserialize)]
struct AdvanceTurnParams {
    mode: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AdvanceTurnMode {
    Smart,
    Step,
}

fn parse_advance_turn_mode(params: Option<AdvanceTurnParams>) -> Result<AdvanceTurnMode, String> {
    match params.and_then(|p| p.mode).as_deref().unwrap_or("smart") {
        "smart" => Ok(AdvanceTurnMode::Smart),
        "step" => Ok(AdvanceTurnMode::Step),
        other => Err(format!("Unknown advance mode: {other}")),
    }
}

fn has_attention_messages(gs: &GameState) -> bool {
    gs.inbox()
        .messages()
        .iter()
        .any(|m| !m.is_resolved() && m.priority() != esm_core::inbox::MessagePriority::ReadOptional)
}

fn advance_turn_smart_impl(state: &State<'_, AppState>) -> Result<GameInfo, String> {
    {
        let lock = state.game_state.lock().unwrap();
        let gs = lock.as_ref().ok_or("No active game session")?;
        let t_lock = state.tournament.lock().unwrap();
        let info = game_info_from_state(gs, t_lock.as_ref().unwrap_or(&empty_tournament()));
        if info.is_match_day || has_attention_messages(gs) {
            return Ok(info);
        }
    }

    let mut last_info: Option<GameInfo> = None;
    for _ in 0..(366 * 3) {
        let info = advance_turn_step_impl(state)?;
        let lock = state.game_state.lock().unwrap();
        let gs = lock.as_ref().ok_or("No active game session")?;
        if !info.match_results.is_empty() || info.is_match_day || has_attention_messages(gs) {
            return Ok(info);
        }
        last_info = Some(info);
    }

    last_info.ok_or("Failed to find a future actionable state".to_string())
}

fn advance_after_match_slot(
    state: &State<'_, AppState>,
    time_slot: TimeSlot,
) -> Result<GameInfo, String> {
    let steps = match time_slot {
        TimeSlot::Morning => 1,
        TimeSlot::Afternoon => 2,
        TimeSlot::Evening => 3,
    };

    let mut info = None;
    for _ in 0..steps {
        info = Some(advance_turn_step_impl(state)?);
    }

    info.ok_or("Failed to advance after match".to_string())
}

#[tauri::command]
fn advance_turn(
    state: State<'_, AppState>,
    params: Option<AdvanceTurnParams>,
) -> Result<GameInfo, String> {
    match parse_advance_turn_mode(params)? {
        AdvanceTurnMode::Step => advance_turn_step_impl(&state),
        AdvanceTurnMode::Smart => advance_turn_smart_impl(&state),
    }
}

fn advance_turn_step_impl(state: &State<'_, AppState>) -> Result<GameInfo, String> {
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
            let current_day = gs.calendar().days_elapsed() as usize;
            for (team_idx, team) in gs.teams_mut().iter_mut().enumerate() {
                if let Some(sched) = schedules.get(team_idx) {
                    let today = sched.day(current_day);
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

                // Build enriched pre-match report
                let mut body = format!("Coach:\nToday we face {}.\n", opponent_name);

                // Opponent record from standings
                let standings = tournament.standings();
                if let Some(opp_standing) = standings.iter().find(|s| s.team_name == opponent_name)
                {
                    body.push_str(&format!(
                        "\nRecord: {}W - {}L",
                        opp_standing.wins, opp_standing.losses,
                    ));
                    let rank = standings
                        .iter()
                        .position(|s| s.team_name == opponent_name)
                        .unwrap_or(0)
                        + 1;
                    body.push_str(&format!(" (Rank #{})\n", rank));
                }

                // Opponent roster
                if let Some(opp_team) = moba_teams.get(opponent_idx) {
                    body.push_str("\nRoster:\n");
                    for p in opp_team.roster() {
                        body.push_str(
                            &format!("  {} — {:?}\n", p.nickname(), p.roles().primary(),),
                        );
                    }
                }

                body.push_str(
                    "\nMake sure your activity schedule is set to manage player stamina.",
                );

                let msg = esm_core::inbox::Message::new(
                    format!("Pre-match Report: vs {}", opponent_name),
                    body,
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

        // AI teams auto-schedule scrims at the start of each new week
        if gs.calendar().is_weekly_tick() {
            if let Some(moba_teams) = m_lock.as_ref() {
                let mut schedules = state.team_schedules.lock().unwrap();
                let mut scrim_mgr = state.scrim_manager.lock().unwrap();
                let player_idx = gs.player_team_index();
                let current_day = gs.calendar().days_elapsed();
                let team_count = moba_teams.len();

                // Each AI team tries to schedule 2-3 scrims for the upcoming week
                // Some may request scrims with the player's team
                let mut scrim_requests: Vec<(String, u32, String)> = Vec::new(); // (team_name, day, time_slot_name)

                for home_idx in 0..team_count {
                    if home_idx == player_idx {
                        continue; // Skip player's team
                    }

                    let target_scrims = gs.rng_mut().range_u32(2, 4); // 2 or 3
                    for _ in 0..target_scrims {
                        // Pick a random opponent (not self)
                        let mut away_idx = gs.rng_mut().range_u32(0, team_count as u32) as usize;
                        if away_idx == home_idx {
                            away_idx = (away_idx + 1) % team_count;
                        }

                        // If the AI wants to scrim with the player, send an inbox message instead
                        if away_idx == player_idx {
                            let day_offset = gs.rng_mut().range_u32(1, 7);
                            let scheduled_day = current_day + day_offset;
                            let slot_idx = gs.rng_mut().range_u32(0, 3) as usize;
                            let time_slot_name = match slot_idx {
                                0 => "Morning",
                                1 => "Afternoon",
                                _ => "Evening",
                            };
                            let ai_name = moba_teams[home_idx].name().to_string();
                            scrim_requests.push((
                                ai_name,
                                scheduled_day,
                                time_slot_name.to_string(),
                            ));
                            continue;
                        }

                        // Pick random day (1-6 days ahead) and time slot
                        let day_offset = gs.rng_mut().range_u32(1, 7);
                        let scheduled_day = current_day + day_offset;
                        let slot_idx = gs.rng_mut().range_u32(0, 3) as usize;
                        let time_slot = TimeSlot::ALL[slot_idx];

                        let home_name = moba_teams[home_idx].name().to_string();
                        let away_name = moba_teams[away_idx].name().to_string();

                        // Attempt to schedule — validation will reject conflicts
                        let _ = scrim_mgr.schedule_scrim(
                            &mut schedules,
                            home_idx,
                            away_idx,
                            &home_name,
                            &away_name,
                            scheduled_day,
                            time_slot,
                            gs.rng_mut().range_u32(1, 4), // 1-3 games
                            ScrimDraftRules::Standard,
                            current_day,
                        );
                    }
                }

                // Send inbox messages for AI scrim requests to the player
                drop(schedules);
                drop(scrim_mgr);
                for (team_name, sday, slot_name) in scrim_requests {
                    let day_diff = sday - current_day;
                    let msg = esm_core::inbox::Message::new(
                        format!("Scrim Request from {}", team_name),
                        format!(
                            "{} would like to schedule a scrim against your team.\n\nRequested: Day {} ({} days from now), {} slot.\n\nHead to the Schedule page to set up scrims.",
                            team_name, sday, day_diff, slot_name,
                        ),
                        esm_core::inbox::MessagePriority::ReadOptional,
                        esm_core::inbox::MessageCategory::Scrim,
                        current_day,
                    );
                    gs.inbox_mut().push(msg);
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
fn play_match_delegate(state: State<'_, AppState>) -> Result<GameInfo, String> {
    let mut match_results: Vec<MatchResultInfo> = Vec::new();

    let post_match_slot = {
        let mut lock = state.game_state.lock().unwrap();
        let gs = lock.as_mut().ok_or("No active game session")?;
        let mut t_lock = state.tournament.lock().unwrap();
        let m_lock = state.moba_teams.lock().unwrap();
        let mut post_match_slot = None;

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
                post_match_slot = match_time_slot_for_day(tournament, day, match_id);

                let config = MobaMatchConfig::default();
                let blue_attrs = extract_team_attrs(&moba_teams[blue_idx]);
                let red_attrs = extract_team_attrs(&moba_teams[red_idx]);
                let wins_needed = m.bracket().wins_needed();
                let mut bw = 0u32;
                let mut rw = 0u32;
                let mut last_duration = 0u32;

                while bw < wins_needed && rw < wins_needed {
                    let result =
                        MobaMatchEngine::simulate(gs.rng_mut(), &blue_attrs, &red_attrs, &config);
                    match result.winner {
                        TeamSide::Blue => bw += 1,
                        TeamSide::Red => rw += 1,
                    }
                    last_duration = result.duration_minutes;
                }

                tournament.record_result(match_id, bw, rw);

                let player_won_series = if blue_idx == player_idx {
                    bw > rw
                } else {
                    rw > bw
                };
                gs.teams_mut()[player_idx].apply_match_result(player_won_series);

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

        post_match_slot
    };

    let mut info = if let Some(slot) = post_match_slot {
        advance_after_match_slot(&state, slot)?
    } else {
        get_game_info(state.clone())?
    };
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
    pub entry_type: String, // "free", "scrim", "solo_queue", "match"
    pub scrim_id: Option<u32>,
    pub opponent: Option<String>,
    pub players: Option<Vec<usize>>,
    pub focus: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DayScheduleInfo {
    pub day_index: usize,
    pub day_label: String,
    pub date_label: String,
    pub is_past: bool,
    pub has_match: bool,
    pub slots: Vec<ScheduleSlotInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WeekScheduleInfo {
    pub days: Vec<DayScheduleInfo>,
    pub total_scrims: usize,
    pub total_matches: usize,
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

fn month_short_name(month: u32) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "?",
    }
}

fn calendar_labels_for_day(start_year: u32, day_index: u32) -> (String, String) {
    let mut calendar = Calendar::new(start_year, 1, 1);
    for _ in 0..day_index {
        calendar.advance_day();
    }

    (
        calendar.day_of_week_name().to_string(),
        format!(
            "{} {}, {}",
            month_short_name(calendar.month()),
            calendar.day(),
            calendar.year()
        ),
    )
}

fn match_time_slot_for_position(position: usize) -> TimeSlot {
    match position {
        0 => TimeSlot::Morning,
        _ => TimeSlot::Evening,
    }
}

fn team_match_for_day<'a>(
    tournament: &'a Tournament,
    team_idx: usize,
    day: u32,
) -> Option<&'a esm_engine::tournament::Match> {
    tournament
        .schedule()
        .matches_for_day(day)
        .into_iter()
        .find(|m| m.blue_team_idx() == team_idx || m.red_team_idx() == team_idx)
}

fn teams_play_each_other_in_week(
    tournament: &Tournament,
    first_team_idx: usize,
    second_team_idx: usize,
    scheduled_day: u32,
) -> bool {
    let target_week = scheduled_day / 7;
    tournament.schedule().matches().iter().any(|m| {
        m.scheduled_day() / 7 == target_week
            && ((m.blue_team_idx() == first_team_idx && m.red_team_idx() == second_team_idx)
                || (m.blue_team_idx() == second_team_idx && m.red_team_idx() == first_team_idx))
    })
}

fn ensure_day_is_schedulable(
    tournament: &Tournament,
    team_idx: usize,
    scheduled_day: u32,
    current_day: u32,
) -> Result<(), String> {
    if scheduled_day <= current_day {
        return Err("Past days cannot be changed".to_string());
    }

    if team_match_for_day(tournament, team_idx, scheduled_day).is_some() {
        return Err("Cannot schedule activities on a match day".to_string());
    }

    Ok(())
}

fn match_time_slot_for_day(tournament: &Tournament, day: u32, match_id: u32) -> Option<TimeSlot> {
    let day_matches = tournament.schedule().matches_for_day(day);
    day_matches
        .iter()
        .position(|m| m.id() == match_id)
        .map(match_time_slot_for_position)
}

fn match_slot_info(ts: TimeSlot, opponent: String) -> ScheduleSlotInfo {
    ScheduleSlotInfo {
        time_slot: time_slot_str(ts).to_string(),
        entry_type: "match".to_string(),
        scrim_id: None,
        opponent: Some(opponent),
        players: None,
        focus: None,
    }
}

fn slot_to_info(
    ts: TimeSlot,
    entry: Option<&ScheduleEntry>,
    scrim_mgr: &ScrimManager,
    player_team_name: &str,
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
            let opponent = scrim_mgr.scrim_by_id(*scrim_id).map(|s| {
                if s.home_team() == player_team_name {
                    s.away_team().to_string()
                } else {
                    s.home_team().to_string()
                }
            });
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
            entry_type: "free".to_string(),
            scrim_id: None,
            opponent: None,
            players: None,
            focus: None,
        },
    }
}

fn calendar_schedule_to_info(
    start_year: u32,
    current_day: u32,
    player_team_name: &str,
    player_team_index: usize,
    tournament: &Tournament,
    schedule: &TeamWeeklySchedule,
    scrim_mgr: &ScrimManager,
) -> WeekScheduleInfo {
    let last_match_day = tournament
        .schedule()
        .matches()
        .iter()
        .filter(|m| m.blue_team_idx() == player_team_index || m.red_team_idx() == player_team_index)
        .map(|m| m.scheduled_day() as usize)
        .max()
        .unwrap_or(0);
    let last_scrim_day = scrim_mgr
        .scrims_for_team(player_team_name)
        .iter()
        .map(|s| s.scheduled_day() as usize)
        .max()
        .unwrap_or(0);
    let max_day = (current_day as usize)
        .max(last_match_day)
        .max(last_scrim_day)
        .min(schedule.days().len().saturating_sub(1));

    let days = (0..=max_day)
        .map(|i| {
            let day = schedule.day(i);
            let player_match = team_match_for_day(tournament, player_team_index, i as u32);
            let match_slot =
                player_match.and_then(|m| match_time_slot_for_day(tournament, i as u32, m.id()));
            let match_opponent = player_match.and_then(|m| {
                let opponent_idx = if m.blue_team_idx() == player_team_index {
                    m.red_team_idx()
                } else {
                    m.blue_team_idx()
                };
                tournament
                    .team_name(opponent_idx)
                    .map(|name| name.to_string())
            });
            let slots = TimeSlot::ALL
                .iter()
                .map(|&ts| {
                    if Some(ts) == match_slot {
                        match_slot_info(
                            ts,
                            match_opponent
                                .clone()
                                .unwrap_or_else(|| "Opponent".to_string()),
                        )
                    } else {
                        slot_to_info(ts, day.get(ts), scrim_mgr, player_team_name)
                    }
                })
                .collect();
            let (day_label, date_label) = calendar_labels_for_day(start_year, i as u32);
            DayScheduleInfo {
                day_index: i,
                day_label,
                date_label,
                is_past: (i as u32) < current_day,
                has_match: player_match.is_some(),
                slots,
            }
        })
        .collect();

    WeekScheduleInfo {
        days,
        total_scrims: scrim_mgr.scrims_for_team(player_team_name).len(),
        total_matches: tournament
            .schedule()
            .matches()
            .iter()
            .filter(|m| {
                m.blue_team_idx() == player_team_index || m.red_team_idx() == player_team_index
            })
            .count(),
        occupied_slots: schedule.occupied_slots(),
    }
}

#[tauri::command]
fn get_team_schedule(state: State<'_, AppState>) -> Result<WeekScheduleInfo, String> {
    let gs = state.game_state.lock().unwrap();
    let gs = gs.as_ref().ok_or("No active game session")?;
    let t_lock = state.tournament.lock().unwrap();
    let tournament = t_lock.as_ref().ok_or("No tournament active")?;
    let schedules = state.team_schedules.lock().unwrap();
    let scrim_mgr = state.scrim_manager.lock().unwrap();
    let idx = gs.player_team_index();
    if idx >= schedules.len() {
        return Err("Schedule not initialized".to_string());
    }
    Ok(calendar_schedule_to_info(
        gs.calendar().year(),
        gs.calendar().days_elapsed(),
        gs.player_team().name(),
        idx,
        tournament,
        &schedules[idx],
        &scrim_mgr,
    ))
}

#[tauri::command]
fn schedule_scrim(
    state: State<'_, AppState>,
    params: ScheduleScrimParams,
) -> Result<ScrimInfo, String> {
    let gs = state.game_state.lock().unwrap();
    let gs = gs.as_ref().ok_or("No active game session")?;
    let t_lock = state.tournament.lock().unwrap();
    let tournament = t_lock.as_ref().ok_or("No tournament active")?;
    let mut schedules = state.team_schedules.lock().unwrap();
    let mut scrim_mgr = state.scrim_manager.lock().unwrap();

    let home_idx = gs.player_team_index();
    let away_idx = params.away_team_index;
    if home_idx == away_idx {
        return Err("Cannot schedule a scrim against your own team".to_string());
    }
    if away_idx >= gs.teams().len() {
        return Err("Opponent is out of range".to_string());
    }
    let home_name = gs.teams()[home_idx].name().to_string();
    let away_name = gs.teams()[away_idx].name().to_string();
    let time_slot = parse_time_slot(&params.time_slot)?;
    let draft_rules = parse_draft_rules(&params.draft_rules)?;
    let current_day = gs.calendar().days_elapsed();

    ensure_day_is_schedulable(tournament, home_idx, params.scheduled_day, current_day)?;
    ensure_day_is_schedulable(tournament, away_idx, params.scheduled_day, current_day)?;

    if teams_play_each_other_in_week(tournament, home_idx, away_idx, params.scheduled_day) {
        return Err(
            "Cannot schedule a scrim against a team you will face in the same week".to_string(),
        );
    }

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
        .map_err(|e| {
            use esm_engine::schedule::scrim_manager::ScrimScheduleError;
            match e {
                ScrimScheduleError::PastDate => "Cannot schedule a scrim in the past".to_string(),
                ScrimScheduleError::SameDayScheduling => {
                    "Cannot schedule a scrim for today — must be at least 1 day in advance"
                        .to_string()
                }
                ScrimScheduleError::HomeSlotOccupied => {
                    "That time slot is already occupied on your schedule".to_string()
                }
                ScrimScheduleError::AwaySlotOccupied => {
                    "The opponent's schedule is full for that time slot".to_string()
                }
                ScrimScheduleError::MaxScrimsPerDay => {
                    "Maximum scrims per day reached (limit: 3)".to_string()
                }
                _ => format!("Schedule error: {:?}", e),
            }
        })?;

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
    let t_lock = state.tournament.lock().unwrap();
    let tournament = t_lock.as_ref().ok_or("No tournament active")?;
    let mut schedules = state.team_schedules.lock().unwrap();
    let scrim_mgr = state.scrim_manager.lock().unwrap();

    let idx = gs.player_team_index();
    let time_slot = parse_time_slot(&params.time_slot)?;
    let focus = parse_solo_queue_focus(&params.focus)?;
    let absolute_day = params.day_index as u32;

    ensure_day_is_schedulable(tournament, idx, absolute_day, gs.calendar().days_elapsed())?;

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

    Ok(calendar_schedule_to_info(
        gs.calendar().year(),
        gs.calendar().days_elapsed(),
        gs.player_team().name(),
        idx,
        tournament,
        &schedules[idx],
        &scrim_mgr,
    ))
}

#[tauri::command]
fn schedule_rest(
    state: State<'_, AppState>,
    params: ScheduleRestParams,
) -> Result<WeekScheduleInfo, String> {
    let lock = state.game_state.lock().unwrap();
    let _gs = lock.as_ref().ok_or("No active game session")?;
    let _ = params;
    Err("Rest days are implicit — leave the slot empty instead".to_string())
}

#[tauri::command]
fn clear_schedule_slot(
    state: State<'_, AppState>,
    day_index: usize,
    time_slot: String,
) -> Result<WeekScheduleInfo, String> {
    let gs = state.game_state.lock().unwrap();
    let gs = gs.as_ref().ok_or("No active game session")?;
    let t_lock = state.tournament.lock().unwrap();
    let tournament = t_lock.as_ref().ok_or("No tournament active")?;
    let mut schedules = state.team_schedules.lock().unwrap();
    let scrim_mgr = state.scrim_manager.lock().unwrap();

    let idx = gs.player_team_index();
    let ts = parse_time_slot(&time_slot)?;

    ensure_day_is_schedulable(
        tournament,
        idx,
        day_index as u32,
        gs.calendar().days_elapsed(),
    )?;

    if let Some(player_match) = team_match_for_day(tournament, idx, day_index as u32) {
        if match_time_slot_for_day(tournament, day_index as u32, player_match.id()) == Some(ts) {
            return Err("Matches cannot be cancelled or rescheduled".to_string());
        }
    }

    schedules[idx].day_mut(day_index).clear(ts);

    Ok(calendar_schedule_to_info(
        gs.calendar().year(),
        gs.calendar().days_elapsed(),
        gs.player_team().name(),
        idx,
        tournament,
        &schedules[idx],
        &scrim_mgr,
    ))
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
    blue_roster: Vec<MatchRosterEntry>,
    red_roster: Vec<MatchRosterEntry>,
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
                    blue_towers: s.blue_towers,
                    red_towers: s.red_towers,
                    blue_inhibitors: s.blue_inhibitors,
                    red_inhibitors: s.red_inhibitors,
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
        blue_roster,
        red_roster,
        events,
    }
}

/// Simulate one game of the player's current series.
/// Uses add_game_win to track incremental wins. Only applies match_result
/// effects when the series is complete.
#[tauri::command]
fn simulate_match(state: State<'_, AppState>) -> Result<SimulateMatchResultInfo, String> {
    let mut post_match_slot = None;

    let info = {
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
                let scheduled_slot = match_time_slot_for_day(tournament, day, match_id);

                let config = MobaMatchConfig::default();
                let blue_attrs = extract_team_attrs(&moba_teams[blue_idx]);
                let red_attrs = extract_team_attrs(&moba_teams[red_idx]);

                let tactics = state.match_tactics.lock().unwrap().clone();

                let blue_nicknames: Vec<String> = moba_teams[blue_idx]
                    .roster()
                    .iter()
                    .map(|p| p.nickname().to_string())
                    .collect();
                let red_nicknames: Vec<String> = moba_teams[red_idx]
                    .roster()
                    .iter()
                    .map(|p| p.nickname().to_string())
                    .collect();
                let blue_team_name = moba_teams[blue_idx].name().to_string();
                let red_team_name = moba_teams[red_idx].name().to_string();

                let result = MobaMatchEngine::simulate_with_names(
                    gs.rng_mut(),
                    &blue_attrs,
                    &red_attrs,
                    &config,
                    &tactics,
                    Some((blue_nicknames, red_nicknames, blue_team_name, red_team_name)),
                );

                let blue_won = matches!(result.winner, TeamSide::Blue);
                let series_complete = tournament.add_game_win(match_id, blue_won);

                if series_complete {
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

                    post_match_slot = scheduled_slot;
                }

                let (blue_picks, red_picks) = {
                    let ds = state.draft_session.lock().unwrap();
                    if let Some(session) = ds.as_ref() {
                        let st = session.state();
                        (st.blue_picks.clone(), st.red_picks.clone())
                    } else {
                        (Vec::new(), Vec::new())
                    }
                };
                let blue_roster = extract_roster_entries(&moba_teams[blue_idx], &blue_picks);
                let red_roster = extract_roster_entries(&moba_teams[red_idx], &red_picks);

                Some(match_result_to_info(
                    &result,
                    moba_teams[blue_idx].name(),
                    moba_teams[red_idx].name(),
                    blue_roster,
                    red_roster,
                ))
            } else {
                None
            }
        } else {
            None
        }
    };

    let info = info.ok_or("No pending match today".to_string())?;
    if let Some(slot) = post_match_slot {
        let _ = advance_after_match_slot(&state, slot)?;
    }

    Ok(info)
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
        day_of_week: gs.calendar().day_of_week_name().to_string(),
        phase: gs.calendar().phase().as_str().to_string(),
        manager_nickname: gs.manager().nickname().to_string(),
        team_name,
        teams_count: gs.teams().len(),
        is_match_day,
        match_results: Vec::new(),
    }
}

fn extract_roster_entries(moba_team: &MobaTeam, picks: &[String]) -> Vec<MatchRosterEntry> {
    moba_team
        .roster()
        .iter()
        .enumerate()
        .map(|(i, p)| MatchRosterEntry {
            nickname: p.nickname().to_string(),
            role: format!("{:?}", p.roles().primary()),
            champion: picks.get(i).cloned().unwrap_or_default(),
        })
        .collect()
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
