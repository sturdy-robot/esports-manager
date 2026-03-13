use serde::{Deserialize, Serialize};

use esm_db::save_manager::SaveEntry;

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

#[derive(Debug, Clone, Deserialize)]
pub struct AdvanceTurnParams {
    pub mode: Option<String>,
}

// ---------------------------------------------------------------------------
// Draft commands
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct StartDraftParams {
    pub player_side: String,
    pub format: String,
    #[serde(default)]
    pub fearless_bans: Vec<String>,
    #[serde(default)]
    pub blue_team: String,
    #[serde(default)]
    pub red_team: String,
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

// ---------------------------------------------------------------------------
// Scheduling DTOs & commands
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct ScheduleSlotInfo {
    pub time_slot: String,
    pub entry_type: String,
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
