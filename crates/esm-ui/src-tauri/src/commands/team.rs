use tauri::State;

use esm_engine::moba_match::tactics::{Focus, Playstyle};
use esm_models::player::PlayerTalk;

use crate::app_state::AppState;
use crate::types::{
    ApplyTalkParams, PlayerStateInfo, SetTacticsParams, TacticsInfo,
};

// ---------------------------------------------------------------------------
// Player Talk commands (between-match motivational system)
// ---------------------------------------------------------------------------

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
pub fn get_roster_state(state: State<'_, AppState>) -> Result<Vec<PlayerStateInfo>, String> {
    let gs = state.game_state.lock().unwrap();
    let gs = gs.as_ref().ok_or("No active game session")?;
    let m_lock = state.moba_teams.lock().unwrap();
    let moba_teams = m_lock.as_ref().ok_or("No teams loaded")?;
    let idx = gs.player_team_index();
    let team = &moba_teams[idx];
    Ok(team.roster().iter().map(player_state_info).collect())
}

#[tauri::command]
pub fn apply_player_talk(
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
pub fn set_tactics(
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
pub fn get_tactics(state: State<'_, AppState>) -> TacticsInfo {
    let lock = state.match_tactics.lock().unwrap();
    TacticsInfo {
        playstyle: playstyle_str(lock.playstyle).to_string(),
        focus: focus_str(lock.focus).to_string(),
    }
}
