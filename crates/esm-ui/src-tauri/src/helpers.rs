use std::path::PathBuf;

use esm_core::calendar::Calendar;
use esm_core::game_state::GameState;
use esm_engine::moba_match::game_state::MatchPlayerSimulationData;
use esm_engine::schedule::scrim::ScrimDraftRules;
use esm_engine::schedule::scrim_manager::ScrimManager;
use esm_engine::schedule::{ScheduleEntry, SoloQueueFocus, TeamWeeklySchedule};
use esm_engine::tournament::{BracketKind, Tournament, TournamentFormat};
use esm_models::moba::team::MobaTeam;
use esm_models::time::TimeSlot;

use crate::types::{
    DayScheduleInfo, GameInfo, MatchRosterEntry, ScheduleSlotInfo, WeekScheduleInfo,
};

fn push_role_if_missing(roles: &mut Vec<String>, role: &str) {
    if !roles.iter().any(|existing| existing == role) {
        roles.push(role.to_string());
    }
}

pub(crate) fn preferred_roles_for_champion(
    champion: &esm_data::datapack::ChampionData,
) -> Vec<String> {
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

pub(crate) fn schedule_span_days(tournament: &Tournament) -> usize {
    tournament
        .schedule()
        .matches()
        .iter()
        .map(|m| m.scheduled_day() as usize)
        .max()
        .unwrap_or(0)
        + 1
}

pub(crate) fn init_team_schedules(
    team_count: usize,
    total_days: usize,
) -> Vec<TeamWeeklySchedule> {
    (0..team_count)
        .map(|_| TeamWeeklySchedule::with_total_days(total_days))
        .collect()
}

pub(crate) fn push_onboarding_messages(gs: &mut GameState) {
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

pub(crate) fn parse_time_slot(s: &str) -> Result<TimeSlot, String> {
    s.parse::<TimeSlot>()
}

pub(crate) fn time_slot_str(ts: TimeSlot) -> &'static str {
    ts.as_str()
}

pub(crate) fn parse_solo_queue_focus(s: &str) -> Result<SoloQueueFocus, String> {
    match s {
        "champions" => Ok(SoloQueueFocus::Champions),
        "tactics" => Ok(SoloQueueFocus::Tactics),
        "mechanics" => Ok(SoloQueueFocus::Mechanics),
        "mentality" => Ok(SoloQueueFocus::Mentality),
        _ => Err(format!("Unknown focus: {s}")),
    }
}

pub(crate) fn focus_to_str(f: SoloQueueFocus) -> &'static str {
    match f {
        SoloQueueFocus::Champions => "champions",
        SoloQueueFocus::Tactics => "tactics",
        SoloQueueFocus::Mechanics => "mechanics",
        SoloQueueFocus::Mentality => "mentality",
    }
}

pub(crate) fn parse_draft_rules(s: &str) -> Result<ScrimDraftRules, String> {
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

pub(crate) fn team_match_for_day<'a>(
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

pub(crate) fn teams_play_each_other_in_week(
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

pub(crate) fn ensure_day_is_schedulable(
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

pub(crate) fn match_time_slot_for_day(
    tournament: &Tournament,
    day: u32,
    match_id: u32,
) -> Option<TimeSlot> {
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

pub(crate) fn calendar_schedule_to_info(
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
                tournament.team_name(opponent_idx).map(|name| name.to_string())
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
            .filter(|m| m.blue_team_idx() == player_team_index || m.red_team_idx() == player_team_index)
            .count(),
        occupied_slots: schedule.occupied_slots(),
    }
}

pub(crate) fn game_info_from_state(gs: &GameState, tournament: &Tournament) -> GameInfo {
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

pub(crate) fn extract_roster_entries(
    moba_team: &MobaTeam,
    picks: &[String],
) -> Vec<MatchRosterEntry> {
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

pub(crate) fn extract_team_attrs(moba_team: &MobaTeam) -> Vec<MatchPlayerSimulationData> {
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
                mastery_multiplier: 1.0,
            }
        })
        .collect()
}

pub(crate) fn empty_tournament() -> Tournament {
    Tournament::new(
        String::new(),
        Vec::new(),
        TournamentFormat::RoundRobin,
        BracketKind::Bo1,
        1,
    )
}

pub(crate) fn resolve_data_path(path: &str) -> PathBuf {
    let p = PathBuf::from(path);
    if p.is_absolute() && p.exists() {
        return p;
    }
    if p.exists() {
        return p;
    }
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
    p
}

pub(crate) fn default_saves_dir() -> PathBuf {
    dirs_next_or_fallback()
}

fn dirs_next_or_fallback() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".esm").join("saves")
}
