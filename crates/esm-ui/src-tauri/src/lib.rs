use std::path::PathBuf;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::State;

use esm_core::game_state::GameState;
use esm_db::save_manager::{SaveEntry, SaveManager};
use esm_models::esport_type::EsportType;
use esm_models::manager::{Manager, ManagerArchetype};

// ---------------------------------------------------------------------------
// Application state
// ---------------------------------------------------------------------------

pub struct AppState {
    saves_dir: PathBuf,
    game_state: Mutex<Option<GameState>>,
}

impl AppState {
    pub fn new(saves_dir: PathBuf) -> Self {
        Self {
            saves_dir,
            game_state: Mutex::new(None),
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
    pub manager_nickname: String,
    pub team_name: String,
    pub teams_count: usize,
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

    // Save to disk
    SaveManager::create_save(&state.saves_dir, &params.save_name, &gs)
        .map_err(|e| format!("Failed to save game: {e}"))?;

    let info = game_info_from_state(&gs);

    // Store in memory
    *state.game_state.lock().unwrap() = Some(gs);

    Ok(info)
}

#[tauri::command]
fn load_save(name: String, state: State<'_, AppState>) -> Result<GameInfo, String> {
    let gs = SaveManager::load_save(&state.saves_dir, &name)
        .map_err(|e| format!("Failed to load save: {e}"))?;

    let info = game_info_from_state(&gs);
    *state.game_state.lock().unwrap() = Some(gs);

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
    SaveManager::create_save(&state.saves_dir, &name, gs)
        .map_err(|e| format!("Failed to save game: {e}"))?;
    Ok(())
}

#[tauri::command]
fn get_game_info(state: State<'_, AppState>) -> Result<GameInfo, String> {
    let lock = state.game_state.lock().unwrap();
    let gs = lock.as_ref().ok_or("No active game session")?;
    Ok(game_info_from_state(gs))
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn game_info_from_state(gs: &GameState) -> GameInfo {
    let team_name = gs
        .teams()
        .get(gs.player_team_index())
        .map(|t| t.name().to_string())
        .unwrap_or_default();

    GameInfo {
        year: gs.calendar().year(),
        month: gs.calendar().month(),
        day: gs.calendar().day(),
        manager_nickname: gs.manager().nickname().to_string(),
        team_name,
        teams_count: gs.teams().len(),
    }
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
            get_game_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
