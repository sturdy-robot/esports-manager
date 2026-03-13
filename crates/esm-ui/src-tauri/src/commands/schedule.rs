use tauri::State;

use crate::app_state::AppState;
use crate::helpers::{
    calendar_schedule_to_info, ensure_day_is_schedulable, match_time_slot_for_day,
    parse_draft_rules, parse_solo_queue_focus, parse_time_slot, team_match_for_day,
    teams_play_each_other_in_week, time_slot_str,
};
use crate::types::{
    ScheduleRestParams, ScheduleScrimParams, ScheduleSoloQueueParams, ScrimInfo, WeekScheduleInfo,
};

use esm_engine::schedule::ScheduleEntry;

// ---------------------------------------------------------------------------
// Scheduling DTOs & commands
// ---------------------------------------------------------------------------

#[tauri::command]
pub fn get_team_schedule(state: State<'_, AppState>) -> Result<WeekScheduleInfo, String> {
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
pub fn schedule_scrim(
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
pub fn cancel_scrim(state: State<'_, AppState>, scrim_id: u32) -> Result<String, String> {
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
pub fn schedule_solo_queue(
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
pub fn schedule_rest(
    state: State<'_, AppState>,
    params: ScheduleRestParams,
) -> Result<WeekScheduleInfo, String> {
    let lock = state.game_state.lock().unwrap();
    let _gs = lock.as_ref().ok_or("No active game session")?;
    let _ = (params.day_index, params.time_slot);
    Err("Rest days are implicit — leave the slot empty instead".to_string())
}

#[tauri::command]
pub fn clear_schedule_slot(
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
pub fn get_scrims_list(state: State<'_, AppState>) -> Result<Vec<ScrimInfo>, String> {
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
