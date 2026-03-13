use std::collections::HashMap;

use tauri::State;

use esm_engine::draft::DraftFormat;
use esm_engine::draft_session::{
    build_champion_evals, DraftPlayerInfo, DraftSession, DraftSessionState,
};
use esm_engine::match_sim::TeamSide as DraftTeamSide;

use crate::app_state::AppState;
use crate::types::StartDraftParams;

// ---------------------------------------------------------------------------
// Draft commands
// ---------------------------------------------------------------------------

#[tauri::command]
pub fn start_draft(
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

    let pool: Vec<String> = if !params.fearless_bans.is_empty() {
        champion_pool
            .into_iter()
            .filter(|c| !params.fearless_bans.contains(c))
            .collect()
    } else {
        champion_pool
    };

    let patch = state.current_patch.lock().unwrap();
    let mastery_map: HashMap<String, esm_models::champion::MasteryLevel> = HashMap::new();
    let evals = build_champion_evals(&pool, &patch, &mastery_map);
    drop(patch);

    let gs_lock = state.game_state.lock().unwrap();
    let rng_seed = gs_lock.as_ref().map(|gs| gs.rng().state()).unwrap_or(42);
    drop(gs_lock);
    let rng = esm_core::rng::GameRng::from_seed(rng_seed);

    let mut session = DraftSession::with_evals(format, player_side, pool, evals, rng);

    let detail_map = state.champion_detail_map.lock().unwrap().clone();
    session.set_champion_details(detail_map);

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
pub fn draft_hover(
    champion: String,
    state: State<'_, AppState>,
) -> Result<DraftSessionState, String> {
    let mut lock = state.draft_session.lock().unwrap();
    let session = lock.as_mut().ok_or("No active draft session")?;

    session
        .hover(champion)
        .map_err(|e| format!("Draft error: {:?}", e))?;
    Ok(session.state())
}

#[tauri::command]
pub fn draft_lock(state: State<'_, AppState>) -> Result<DraftSessionState, String> {
    let mut lock = state.draft_session.lock().unwrap();
    let session = lock.as_mut().ok_or("No active draft session")?;

    session
        .lock()
        .map_err(|e| format!("Draft error: {:?}", e))?;
    session.run_ai_turns();
    Ok(session.state())
}

#[tauri::command]
pub fn get_draft_state(state: State<'_, AppState>) -> Result<DraftSessionState, String> {
    let lock = state.draft_session.lock().unwrap();
    let session = lock.as_ref().ok_or("No active draft session")?;
    Ok(session.state())
}

#[tauri::command]
pub fn auto_draft_complete(state: State<'_, AppState>) -> Result<DraftSessionState, String> {
    let mut ds = state.draft_session.lock().unwrap();
    let session = ds.as_mut().ok_or("No active draft session")?;
    while !session.is_complete() {
        session.force_ai_act();
    }
    Ok(session.state())
}

#[tauri::command]
pub fn draft_swap_picks(
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
