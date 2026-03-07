import { invoke } from "@tauri-apps/api/core";

// ---------------------------------------------------------------------------
// Types matching Rust DTOs
// ---------------------------------------------------------------------------

export interface TeamInfo {
  name: string;
  tag: string;
  player_count: number;
}

export interface SaveInfo {
  name: string;
  checksum: string;
}

export interface GameInfo {
  year: number;
  month: number;
  day: number;
  day_of_week: string;
  phase: string;
  manager_nickname: string;
  team_name: string;
  teams_count: number;
  is_match_day: boolean;
  match_results: MatchResultInfo[];
}

export interface MatchResultInfo {
  blue_team: string;
  red_team: string;
  winner: string;
  duration_minutes: number;
}

export interface StandingInfo {
  rank: number;
  team_name: string;
  wins: number;
  losses: number;
  win_pct: number;
}

export interface ScheduleMatchInfo {
  id: number;
  blue_team: string;
  red_team: string;
  scheduled_day: number;
  status: string;
  winner: string | null;
}

export interface PlayerInfo {
  nickname: string;
  first_name: string;
  last_name: string;
  role: string;
  stamina: number;
  morale: number;
  mechanics: number;
  vision: number;
  teamfighting: number;
}

export interface InboxMessageInfo {
  id: string;
  subject: string;
  category: string;
  priority: "Urgent" | "Action" | "Info";
  day: number;
  read: boolean;
}

export interface NewGameParams {
  first_name: string;
  last_name: string;
  nickname: string;
  nationality: string;
  esport_type: string;
  datapack_path: string;
  team_index: number;
  save_name: string;
}

// ---------------------------------------------------------------------------
// API functions
// ---------------------------------------------------------------------------

export async function loadDatapack(path: string): Promise<TeamInfo[]> {
  return invoke<TeamInfo[]>("load_datapack", { path });
}

export async function listSaves(): Promise<SaveInfo[]> {
  return invoke<SaveInfo[]>("list_saves");
}

export async function newGame(params: NewGameParams): Promise<GameInfo> {
  return invoke<GameInfo>("new_game", { params });
}

export async function loadSave(name: string): Promise<GameInfo> {
  return invoke<GameInfo>("load_save", { name });
}

export async function deleteSave(name: string): Promise<void> {
  return invoke<void>("delete_save", { name });
}

export async function saveGame(name: string): Promise<void> {
  return invoke<void>("save_game", { name });
}

export async function advanceTurn(): Promise<GameInfo> {
  return invoke<GameInfo>("advance_turn");
}

export async function getRoster(): Promise<PlayerInfo[]> {
  return invoke<PlayerInfo[]>("get_roster");
}

export async function getInbox(): Promise<InboxMessageInfo[]> {
  return invoke<InboxMessageInfo[]>("get_inbox");
}

export async function getGameInfo(): Promise<GameInfo> {
  return invoke<GameInfo>("get_game_info");
}

export async function getStandings(): Promise<StandingInfo[]> {
  return invoke<StandingInfo[]>("get_standings");
}

export async function getSchedule(): Promise<ScheduleMatchInfo[]> {
  return invoke<ScheduleMatchInfo[]>("get_schedule");
}

export async function resolveMessage(msgId: string): Promise<void> {
  return invoke<void>("resolve_message", { msgId });
}

export async function playMatchDelegate(): Promise<GameInfo> {
  return invoke<GameInfo>("play_match_delegate");
}

// ---------------------------------------------------------------------------
// Draft API
// ---------------------------------------------------------------------------

export type DraftPhase = "Ban" | "Pick";
export type DraftTeamSide = "Blue" | "Red";

export type ChampionClass = "Tank" | "Fighter" | "Assassin" | "Mage" | "Marksman" | "Support";
export type ChampionScaling = "Early" | "Mid" | "Late";

export interface ChampionDraftInfo {
  name: string;
  class: ChampionClass;
  scaling: ChampionScaling;
  tags: string[];
  meta_tier: string;
}

export interface DraftPlayerInfo {
  nickname: string;
  role: string;
}

export interface DraftSessionState {
  current_step: number;
  total_steps: number;
  current_phase: DraftPhase | null;
  current_team: DraftTeamSide | null;
  blue_bans: string[];
  red_bans: string[];
  blue_picks: string[];
  red_picks: string[];
  active_hover: string | null;
  is_complete: boolean;
  is_player_turn: boolean;
  available_champions: string[];
  timer_seconds: number;
  champion_details: Record<string, ChampionDraftInfo>;
  blue_players: DraftPlayerInfo[];
  red_players: DraftPlayerInfo[];
  blue_team_name: string;
  red_team_name: string;
}

export interface StartDraftParams {
  player_side: "blue" | "red";
  format: "three_ban" | "five_ban" | "fearless";
  fearless_bans?: string[];
  blue_team?: string;
  red_team?: string;
}

export async function startDraft(params: StartDraftParams): Promise<DraftSessionState> {
  return invoke<DraftSessionState>("start_draft", { params });
}

export async function draftHover(champion: string): Promise<DraftSessionState> {
  return invoke<DraftSessionState>("draft_hover", { champion });
}

export async function draftLock(): Promise<DraftSessionState> {
  return invoke<DraftSessionState>("draft_lock");
}

export async function getDraftState(): Promise<DraftSessionState> {
  return invoke<DraftSessionState>("get_draft_state");
}

export async function autoDraftComplete(): Promise<DraftSessionState> {
  return invoke<DraftSessionState>("auto_draft_complete");
}

export async function draftSwapPicks(a: number, b: number): Promise<DraftSessionState> {
  return invoke<DraftSessionState>("draft_swap_picks", { a, b });
}

// ---------------------------------------------------------------------------
// Match Simulation API
// ---------------------------------------------------------------------------

export interface PlayerSnapshotInfo {
  kills: number;
  deaths: number;
  assists: number;
  cs: number;
  gold: number;
  is_dead: boolean;
}

export interface GameSnapshotInfo {
  blue_players: PlayerSnapshotInfo[];
  red_players: PlayerSnapshotInfo[];
  blue_team_gold: number;
  red_team_gold: number;
  dragons_blue: number;
  dragons_red: number;
  baron_alive: boolean;
  baron_timer: number;
  dragon_timer: number;
  herald_available: boolean;
}

export interface MatchEventInfo {
  minute: number;
  phase: string;
  kind: string;
  commentary: string | null;
  snapshot: GameSnapshotInfo | null;
}

export interface MatchRosterEntry {
  nickname: string;
  role: string;
  champion: string;
}

export interface SimulateMatchResult {
  winner: string;
  duration_minutes: number;
  blue_team: string;
  red_team: string;
  blue_gold: number;
  red_gold: number;
  blue_roster: MatchRosterEntry[];
  red_roster: MatchRosterEntry[];
  events: MatchEventInfo[];
}

export async function simulateMatch(): Promise<SimulateMatchResult> {
  return invoke<SimulateMatchResult>("simulate_match");
}

// ---------------------------------------------------------------------------
// Tactics API
// ---------------------------------------------------------------------------

export type PlaystyleType = "aggressive" | "balanced" | "defensive";
export type FocusType = "teamfight" | "splitpush" | "objective";

export interface TacticsInfo {
  playstyle: PlaystyleType;
  focus: FocusType;
}

export async function setTactics(playstyle: PlaystyleType, focus: FocusType): Promise<TacticsInfo> {
  return invoke<TacticsInfo>("set_tactics", { params: { playstyle, focus } });
}

export async function getTactics(): Promise<TacticsInfo> {
  return invoke<TacticsInfo>("get_tactics");
}

// ---------------------------------------------------------------------------
// Player Talk API (between-match motivational system)
// ---------------------------------------------------------------------------

export type TalkType = "motivate" | "calm" | "strategize" | "rest";

export interface PlayerStateInfo {
  nickname: string;
  role: string;
  stamina: number;
  morale: number;
  confidence: string;
  satisfaction: number;
}

export async function getRosterState(): Promise<PlayerStateInfo[]> {
  return invoke<PlayerStateInfo[]>("get_roster_state");
}

export async function applyPlayerTalk(playerIndex: number, talk: TalkType): Promise<PlayerStateInfo[]> {
  return invoke<PlayerStateInfo[]>("apply_player_talk", { params: { player_index: playerIndex, talk } });
}

// ---------------------------------------------------------------------------
// Team Schedule API
// ---------------------------------------------------------------------------

export type TimeSlotType = "Morning" | "Afternoon" | "Evening";
export type EntryType = "free" | "scrim" | "solo_queue" | "rest";
export type SoloQueueFocusType = "champions" | "tactics" | "mechanics" | "mentality";
export type DraftRulesType = "standard" | "fearless";

export interface ScheduleSlotInfo {
  time_slot: string;
  entry_type: EntryType;
  scrim_id: number | null;
  opponent: string | null;
  players: number[] | null;
  focus: string | null;
}

export interface DayScheduleInfo {
  day_index: number;
  slots: ScheduleSlotInfo[];
}

export interface WeekScheduleInfo {
  days: DayScheduleInfo[];
  total_scrims: number;
  occupied_slots: number;
}

export interface ScrimInfo {
  id: number;
  home_team: string;
  away_team: string;
  scheduled_day: number;
  time_slot: string;
  game_count: number;
  draft_rules: string;
  status: string;
  home_wins: number;
  away_wins: number;
}

export interface ScheduleScrimParams {
  away_team_index: number;
  scheduled_day: number;
  time_slot: TimeSlotType;
  game_count: number;
  draft_rules: DraftRulesType;
}

export interface ScheduleSoloQueueParams {
  day_index: number;
  time_slot: TimeSlotType;
  players: number[];
  focus: SoloQueueFocusType;
}

export interface ScheduleRestParams {
  day_index: number;
  time_slot: TimeSlotType;
}

export async function getTeamSchedule(): Promise<WeekScheduleInfo> {
  return invoke<WeekScheduleInfo>("get_team_schedule");
}

export async function scheduleScrim(params: ScheduleScrimParams): Promise<ScrimInfo> {
  return invoke<ScrimInfo>("schedule_scrim", { params });
}

export async function cancelScrim(scrimId: number): Promise<string> {
  return invoke<string>("cancel_scrim", { scrimId });
}

export async function scheduleSoloQueue(params: ScheduleSoloQueueParams): Promise<WeekScheduleInfo> {
  return invoke<WeekScheduleInfo>("schedule_solo_queue", { params });
}

export async function scheduleRest(params: ScheduleRestParams): Promise<WeekScheduleInfo> {
  return invoke<WeekScheduleInfo>("schedule_rest", { params });
}

export async function clearScheduleSlot(dayIndex: number, timeSlot: TimeSlotType): Promise<WeekScheduleInfo> {
  return invoke<WeekScheduleInfo>("clear_schedule_slot", { dayIndex, timeSlot });
}

export async function getScrimsList(): Promise<ScrimInfo[]> {
  return invoke<ScrimInfo[]>("get_scrims_list");
}

// ---------------------------------------------------------------------------
// Series API
// ---------------------------------------------------------------------------

export interface SeriesInfo {
  match_id: number;
  blue_team: string;
  red_team: string;
  blue_wins: number;
  red_wins: number;
  wins_needed: number;
  is_complete: boolean;
  game_number: number;
}

export async function getSeriesInfo(): Promise<SeriesInfo> {
  return invoke<SeriesInfo>("get_series_info");
}
