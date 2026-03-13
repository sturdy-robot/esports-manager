use std::collections::HashMap;

use tauri::State;

use esm_core::calendar::DayPhase;
use esm_core::game_state::GameState;
use esm_core::turn::TurnProcessor;
use esm_db::save_manager::SaveManager;
use esm_engine::draft_session::ChampionDraftInfo;
use esm_engine::moba_match::engine::{MobaMatchConfig, MobaMatchEngine};
use esm_engine::moba_match::state::TeamSide;
use esm_engine::schedule::processor::ScheduleProcessor;
use esm_engine::schedule::scrim::ScrimDraftRules;
use esm_engine::schedule::scrim_manager::ScrimManager;
use esm_engine::schedule::TeamWeeklySchedule;
use esm_engine::tournament::{BracketKind, Tournament, TournamentFormat};
use esm_models::esport_type::EsportType;
use esm_models::manager::{Manager, ManagerArchetype};
use esm_models::moba::team::MobaTeam;
use esm_models::time::TimeSlot;

use crate::app_state::AppState;
use crate::helpers::{
    empty_tournament, extract_team_attrs, game_info_from_state, init_team_schedules,
    match_time_slot_for_day, preferred_roles_for_champion, push_onboarding_messages,
    resolve_data_path, schedule_span_days,
};
use crate::types::{
    AdvanceTurnParams, GameInfo, InboxMessageInfo, MatchResultInfo, NewGameParams, PlayerInfo,
    SaveInfo, ScheduleMatchInfo, StandingInfo, TeamInfo,
};

// ---------------------------------------------------------------------------
// Tauri commands
// ---------------------------------------------------------------------------

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to eSports Manager.", name)
}

#[tauri::command]
pub fn load_datapack(path: String) -> Result<Vec<TeamInfo>, String> {
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
pub fn list_saves(state: State<'_, AppState>) -> Result<Vec<SaveInfo>, String> {
    let saves = SaveManager::list_saves(&state.saves_dir)
        .map_err(|e| format!("Failed to list saves: {e}"))?;
    Ok(saves.into_iter().map(SaveInfo::from).collect())
}

#[tauri::command]
pub fn new_game(params: NewGameParams, state: State<'_, AppState>) -> Result<GameInfo, String> {
    let esport_type: EsportType = params
        .esport_type
        .parse()
        .map_err(|e| format!("Invalid esport type: {e}"))?;

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

    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    let mut gs = GameState::new(2025, seed, esport_type, manager, params.team_index, teams);

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

    let tournament_json =
        serde_json::to_string(&tournament).map_err(|e| format!("Serialize tournament: {e}"))?;
    let moba_teams_json =
        serde_json::to_string(&moba_teams).map_err(|e| format!("Serialize moba teams: {e}"))?;

    SaveManager::create_save_full(
        &state.saves_dir,
        &params.save_name,
        &gs,
        &tournament_json,
        &moba_teams_json,
    )
    .map_err(|e| format!("Failed to save game: {e}"))?;

    let info = game_info_from_state(&gs, &tournament);

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

    let team_count = gs.teams().len();
    *state.team_schedules.lock().unwrap() = init_team_schedules(team_count, schedule_days);
    *state.scrim_manager.lock().unwrap() = ScrimManager::new();

    *state.game_state.lock().unwrap() = Some(gs);
    *state.tournament.lock().unwrap() = Some(tournament);
    *state.moba_teams.lock().unwrap() = Some(moba_teams);

    Ok(info)
}

#[tauri::command]
pub fn load_save(name: String, state: State<'_, AppState>) -> Result<GameInfo, String> {
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
pub fn delete_save(name: String, state: State<'_, AppState>) -> Result<(), String> {
    SaveManager::delete_save(&state.saves_dir, &name)
        .map_err(|e| format!("Failed to delete save: {e}"))?;
    Ok(())
}

#[tauri::command]
pub fn save_game(name: String, state: State<'_, AppState>) -> Result<(), String> {
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
pub fn get_roster(state: State<'_, AppState>) -> Result<Vec<PlayerInfo>, String> {
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
pub fn get_inbox(state: State<'_, AppState>) -> Result<Vec<InboxMessageInfo>, String> {
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

pub(crate) fn advance_after_match_slot(
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
pub fn advance_turn(
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

    if gs.calendar().phase() == DayPhase::Evening {
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

        if let Some(moba_teams) = m_lock.as_ref() {
            let today = gs.calendar().days_elapsed();
            let mut scrim_mgr = state.scrim_manager.lock().unwrap();
            let pending = scrim_mgr.pending_scrims_for_day(today);
            let config = MobaMatchConfig::default();

            for (scrim_id, game_count) in pending {
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
                    continue;
                }

                if blue_idx < moba_teams.len() && red_idx < moba_teams.len() {
                    let blue_attrs = extract_team_attrs(&moba_teams[blue_idx]);
                    let red_attrs = extract_team_attrs(&moba_teams[red_idx]);

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

                let mut body = format!("Coach:\nToday we face {}.\n", opponent_name);

                let standings = tournament.standings();
                if let Some(opp_standing) = standings.iter().find(|s| s.team_name == opponent_name) {
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

                if let Some(opp_team) = moba_teams.get(opponent_idx) {
                    body.push_str("\nRoster:\n");
                    for p in opp_team.roster() {
                        body.push_str(&format!("  {} — {:?}\n", p.nickname(), p.roles().primary(),));
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

        if gs.calendar().is_weekly_tick() {
            if let Some(moba_teams) = m_lock.as_ref() {
                let mut schedules = state.team_schedules.lock().unwrap();
                let mut scrim_mgr = state.scrim_manager.lock().unwrap();
                let player_idx = gs.player_team_index();
                let current_day = gs.calendar().days_elapsed();
                let team_count = moba_teams.len();

                let mut scrim_requests: Vec<(String, u32, String)> = Vec::new();

                for home_idx in 0..team_count {
                    if home_idx == player_idx {
                        continue;
                    }

                    let target_scrims = gs.rng_mut().range_u32(2, 4);
                    for _ in 0..target_scrims {
                        let mut away_idx = gs.rng_mut().range_u32(0, team_count as u32) as usize;
                        if away_idx == home_idx {
                            away_idx = (away_idx + 1) % team_count;
                        }

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

                        let day_offset = gs.rng_mut().range_u32(1, 7);
                        let scheduled_day = current_day + day_offset;
                        let slot_idx = gs.rng_mut().range_u32(0, 3) as usize;
                        let time_slot = TimeSlot::ALL[slot_idx];

                        let home_name = moba_teams[home_idx].name().to_string();
                        let away_name = moba_teams[away_idx].name().to_string();

                        let _ = scrim_mgr.schedule_scrim(
                            &mut schedules,
                            home_idx,
                            away_idx,
                            &home_name,
                            &away_name,
                            scheduled_day,
                            time_slot,
                            gs.rng_mut().range_u32(1, 4),
                            ScrimDraftRules::Standard,
                            current_day,
                        );
                    }
                }

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
pub fn play_match_delegate(state: State<'_, AppState>) -> Result<GameInfo, String> {
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

                let player_won_series = if blue_idx == player_idx { bw > rw } else { rw > bw };
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

#[tauri::command]
pub fn resolve_message(msg_id: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut lock = state.game_state.lock().unwrap();
    let gs = lock.as_mut().ok_or("No active game session")?;

    if let Some(idx_str) = msg_id.strip_prefix("msg_") {
        if let Ok(idx) = idx_str.parse::<usize>() {
            gs.inbox_mut().resolve_at(idx);
            return Ok(());
        }
    }

    Err(format!("Invalid message ID: {msg_id}"))
}

#[tauri::command]
pub fn get_game_info(state: State<'_, AppState>) -> Result<GameInfo, String> {
    let lock = state.game_state.lock().unwrap();
    let gs = lock.as_ref().ok_or("No active game session")?;
    let t_lock = state.tournament.lock().unwrap();
    Ok(game_info_from_state(
        gs,
        t_lock.as_ref().unwrap_or(&empty_tournament()),
    ))
}

#[tauri::command]
pub fn get_standings(state: State<'_, AppState>) -> Result<Vec<StandingInfo>, String> {
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
pub fn get_schedule(state: State<'_, AppState>) -> Result<Vec<ScheduleMatchInfo>, String> {
    let t_lock = state.tournament.lock().unwrap();
    let tournament = t_lock.as_ref().ok_or("No tournament active")?;

    let matches: Vec<ScheduleMatchInfo> = tournament
        .schedule()
        .matches()
        .iter()
        .map(|m| {
            let blue_name = tournament.team_name(m.blue_team_idx()).unwrap_or("").to_string();
            let red_name = tournament.team_name(m.red_team_idx()).unwrap_or("").to_string();
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
