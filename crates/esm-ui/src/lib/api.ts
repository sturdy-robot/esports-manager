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
}

export interface StartDraftParams {
  player_side: "blue" | "red";
  format: "three_ban" | "five_ban" | "fearless";
  fearless_bans?: string[];
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

// ---------------------------------------------------------------------------
// Match Simulation API
// ---------------------------------------------------------------------------

export interface MatchEventInfo {
  minute: number;
  phase: string;
  kind: string;
  commentary: string | null;
}

export interface SimulateMatchResult {
  winner: string;
  duration_minutes: number;
  blue_team: string;
  red_team: string;
  blue_gold: number;
  red_gold: number;
  events: MatchEventInfo[];
}

export async function simulateMatch(): Promise<SimulateMatchResult> {
  return invoke<SimulateMatchResult>("simulate_match");
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
