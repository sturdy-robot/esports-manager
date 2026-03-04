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

export async function getGameInfo(): Promise<GameInfo> {
  return invoke<GameInfo>("get_game_info");
}
