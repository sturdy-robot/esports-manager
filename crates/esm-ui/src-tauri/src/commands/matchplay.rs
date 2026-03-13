use tauri::State;

use esm_engine::moba_match::engine::{MobaMatchEngine, MobaMatchResult};
use esm_engine::moba_match::event::MatchEventKind;
use esm_engine::moba_match::state::TeamSide;

use crate::app_state::AppState;
use crate::commands::advance_after_match_slot;
use crate::helpers::{
    extract_roster_entries, extract_team_attrs, match_time_slot_for_day,
};
use crate::types::{
    GameSnapshotInfo, MatchEventInfo, MatchRosterEntry, PlayerSnapshotInfo, SeriesInfo,
    SimulateMatchResultInfo,
};

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
pub fn simulate_match(state: State<'_, AppState>) -> Result<SimulateMatchResultInfo, String> {
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

                let config = esm_engine::moba_match::engine::MobaMatchConfig::default();
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
pub fn get_series_info(state: State<'_, AppState>) -> Result<SeriesInfo, String> {
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
