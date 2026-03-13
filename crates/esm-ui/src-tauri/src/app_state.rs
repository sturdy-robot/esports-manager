use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

use esm_core::game_state::GameState;
use esm_engine::draft_session::{ChampionDraftInfo, DraftSession};
use esm_engine::moba_match::tactics::MatchTactics;
use esm_engine::patch::Patch;
use esm_engine::schedule::scrim_manager::ScrimManager;
use esm_engine::schedule::TeamWeeklySchedule;
use esm_engine::tournament::Tournament;
use esm_models::moba::team::MobaTeam;

pub struct AppState {
    pub saves_dir: PathBuf,
    pub game_state: Mutex<Option<GameState>>,
    pub tournament: Mutex<Option<Tournament>>,
    pub moba_teams: Mutex<Option<Vec<MobaTeam>>>,
    pub champion_names: Mutex<Vec<String>>,
    pub champion_detail_map: Mutex<HashMap<String, ChampionDraftInfo>>,
    pub draft_session: Mutex<Option<DraftSession>>,
    pub match_tactics: Mutex<MatchTactics>,
    pub team_schedules: Mutex<Vec<TeamWeeklySchedule>>,
    pub scrim_manager: Mutex<ScrimManager>,
    pub current_patch: Mutex<Patch>,
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
