import { useState, useEffect, useCallback } from "react";
import type { SaveInfo, GameInfo, NewGameParams, TeamInfo, PlayerInfo, InboxMessageInfo, StandingInfo, ScheduleMatchInfo } from "./api";

// ---------------------------------------------------------------------------
// Detect whether we're running inside Tauri or in a browser (dev/test)
// ---------------------------------------------------------------------------

const IS_TAURI =
  typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

// ---------------------------------------------------------------------------
// Mock data for dev/test mode
// ---------------------------------------------------------------------------

const MOCK_SAVES: SaveInfo[] = [
  { name: "Career_kkOma_2025", checksum: "a1b2c3d4" },
  { name: "Quick_Save_1", checksum: "e5f6a7b8" },
];

const MOCK_TEAMS: TeamInfo[] = [
  { name: "T1", tag: "T1", player_count: 5 },
  { name: "Gen.G", tag: "GEN", player_count: 5 },
  { name: "DRX", tag: "DRX", player_count: 5 },
  { name: "KT Rolster", tag: "KT", player_count: 5 },
  { name: "Hanwha Life", tag: "HLE", player_count: 5 },
  { name: "Dplus KIA", tag: "DK", player_count: 5 },
];

const MOCK_GAME_INFO: GameInfo = {
  year: 2025,
  month: 1,
  day: 1,
  phase: "Morning",
  manager_nickname: "kkOma",
  team_name: "T1",
  teams_count: 6,
  is_match_day: false,
  match_results: [],
};

const MOCK_PHASES = ["Morning", "Afternoon", "Evening"];
let mockPhaseIndex = 0;
let mockDay = 1;

// ---------------------------------------------------------------------------
// API adapter — real Tauri calls or mock implementations
// ---------------------------------------------------------------------------

interface ApiAdapter {
  listSaves(): Promise<SaveInfo[]>;
  loadSave(name: string): Promise<GameInfo>;
  deleteSave(name: string): Promise<void>;
  newGame(params: NewGameParams): Promise<GameInfo>;
  saveGame(name: string): Promise<void>;
  advanceTurn(): Promise<GameInfo>;
  getRoster(): Promise<PlayerInfo[]>;
  getInbox(): Promise<InboxMessageInfo[]>;
  loadDatapack(path: string): Promise<TeamInfo[]>;
  getGameInfo(): Promise<GameInfo>;
  getStandings(): Promise<StandingInfo[]>;
  getSchedule(): Promise<ScheduleMatchInfo[]>;
  resolveMessage(msgId: string): Promise<void>;
}

async function tauriAdapter(): Promise<ApiAdapter> {
  const api = await import("./api");
  return {
    listSaves: api.listSaves,
    loadSave: api.loadSave,
    deleteSave: api.deleteSave,
    newGame: api.newGame,
    saveGame: api.saveGame,
    advanceTurn: api.advanceTurn,
    getRoster: api.getRoster,
    getInbox: api.getInbox,
    loadDatapack: api.loadDatapack,
    getGameInfo: api.getGameInfo,
    getStandings: api.getStandings,
    getSchedule: api.getSchedule,
    resolveMessage: api.resolveMessage,
  };
}

const mockAdapter: ApiAdapter = {
  async listSaves() {
    await delay(300);
    return [...MOCK_SAVES];
  },
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  async loadSave(_name: string) {
    await delay(200);
    return { ...MOCK_GAME_INFO };
  },
  async deleteSave(name: string) {
    await delay(150);
    const idx = MOCK_SAVES.findIndex((s) => s.name === name);
    if (idx >= 0) MOCK_SAVES.splice(idx, 1);
  },
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  async newGame(_params: NewGameParams) {
    await delay(400);
    return { ...MOCK_GAME_INFO };
  },
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  async saveGame(_name: string) {
    await delay(200);
  },
  async advanceTurn() {
    await delay(100);
    mockPhaseIndex++;
    if (mockPhaseIndex >= MOCK_PHASES.length) {
      mockPhaseIndex = 0;
      mockDay++;
    }
    return { ...MOCK_GAME_INFO, day: mockDay, phase: MOCK_PHASES[mockPhaseIndex] };
  },
  async getRoster() {
    await delay(150);
    return [
      { nickname: "Zeus", first_name: "Woo-je", last_name: "Choi", role: "Top", stamina: 92, morale: 85, mechanics: 88, vision: 82, teamfighting: 90 },
      { nickname: "Oner", first_name: "Hyeon-jun", last_name: "Mun", role: "Jungle", stamina: 90, morale: 80, mechanics: 85, vision: 88, teamfighting: 87 },
      { nickname: "Faker", first_name: "Sang-hyeok", last_name: "Lee", role: "Mid", stamina: 85, morale: 95, mechanics: 97, vision: 90, teamfighting: 93 },
      { nickname: "Gumayusi", first_name: "Min-hyeok", last_name: "Lee", role: "Bot", stamina: 88, morale: 82, mechanics: 90, vision: 78, teamfighting: 86 },
      { nickname: "Keria", first_name: "Min-seok", last_name: "Ryu", role: "Support", stamina: 85, morale: 88, mechanics: 86, vision: 94, teamfighting: 91 },
    ];
  },
  async getInbox() {
    await delay(100);
    return [
      { id: "msg_0", subject: "Welcome to eSports Manager!", category: "General", priority: "Info" as const, day: 0, read: false },
      { id: "msg_1", subject: "Pre-season roster review", category: "Team", priority: "Action" as const, day: 0, read: false },
    ];
  },
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  async loadDatapack(_path: string) {
    await delay(200);
    return [...MOCK_TEAMS];
  },
  async getGameInfo() {
    return { ...MOCK_GAME_INFO };
  },
  async getStandings() {
    await delay(100);
    return MOCK_TEAMS.map((t, i) => ({
      rank: i + 1,
      team_name: t.name,
      wins: 6 - i,
      losses: i,
      win_pct: ((6 - i) / 6) * 100,
    }));
  },
  async getSchedule() {
    await delay(100);
    return [
      { id: 1, blue_team: "T1", red_team: "Gen.G", scheduled_day: 3, status: "Pending", winner: null },
      { id: 2, blue_team: "DRX", red_team: "KT Rolster", scheduled_day: 3, status: "Pending", winner: null },
    ];
  },
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  async resolveMessage(_msgId: string) {
    await delay(100);
  },
};

function delay(ms: number) {
  return new Promise((r) => setTimeout(r, ms));
}

let adapterPromise: Promise<ApiAdapter> | null = null;

function getAdapter(): Promise<ApiAdapter> {
  if (!adapterPromise) {
    adapterPromise = IS_TAURI ? tauriAdapter() : Promise.resolve(mockAdapter);
  }
  return adapterPromise;
}

// ---------------------------------------------------------------------------
// Hooks
// ---------------------------------------------------------------------------

export function useListSaves() {
  const [saves, setSaves] = useState<SaveInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const adapter = await getAdapter();
      const result = await adapter.listSaves();
      setSaves(result);
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    refresh();
  }, [refresh]);

  return { saves, loading, error, refresh };
}

export function useDeleteSave() {
  const [deleting, setDeleting] = useState(false);

  const deleteSave = useCallback(async (name: string) => {
    setDeleting(true);
    try {
      const adapter = await getAdapter();
      await adapter.deleteSave(name);
    } finally {
      setDeleting(false);
    }
  }, []);

  return { deleteSave, deleting };
}

export function useLoadSave() {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadSave = useCallback(async (name: string): Promise<GameInfo | null> => {
    setLoading(true);
    setError(null);
    try {
      const adapter = await getAdapter();
      return await adapter.loadSave(name);
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
      return null;
    } finally {
      setLoading(false);
    }
  }, []);

  return { loadSave, loading, error };
}

export function useNewGame() {
  const [creating, setCreating] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const createGame = useCallback(async (params: NewGameParams): Promise<GameInfo | null> => {
    setCreating(true);
    setError(null);
    try {
      const adapter = await getAdapter();
      return await adapter.newGame(params);
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      console.error('useNewGame error:', msg);
      setError(msg);
      return null;
    } finally {
      setCreating(false);
    }
  }, []);

  return { createGame, creating, error };
}

export function useSaveGame() {
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const saveGame = useCallback(async (name: string): Promise<boolean> => {
    setSaving(true);
    setError(null);
    try {
      const adapter = await getAdapter();
      await adapter.saveGame(name);
      return true;
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      console.error('useSaveGame error:', msg);
      setError(msg);
      return false;
    } finally {
      setSaving(false);
    }
  }, []);

  return { saveGame, saving, error };
}

export function useAdvanceTurn() {
  const [advancing, setAdvancing] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const advanceTurn = useCallback(async (): Promise<GameInfo | null> => {
    setAdvancing(true);
    setError(null);
    try {
      const adapter = await getAdapter();
      return await adapter.advanceTurn();
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      console.error('useAdvanceTurn error:', msg);
      setError(msg);
      return null;
    } finally {
      setAdvancing(false);
    }
  }, []);

  return { advanceTurn, advancing, error };
}

export function useLoadDatapack() {
  const [teams, setTeams] = useState<TeamInfo[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadDatapack = useCallback(async (path: string) => {
    setLoading(true);
    setError(null);
    try {
      const adapter = await getAdapter();
      const result = await adapter.loadDatapack(path);
      setTeams(result);
      return result;
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
      return [];
    } finally {
      setLoading(false);
    }
  }, []);

  return { teams, loadDatapack, loading, error };
}

export function useStandings() {
  const [standings, setStandings] = useState<StandingInfo[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchStandings = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const adapter = await getAdapter();
      const result = await adapter.getStandings();
      setStandings(result);
      return result;
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      console.error('useStandings error:', msg);
      setError(msg);
      return [];
    } finally {
      setLoading(false);
    }
  }, []);

  return { standings, fetchStandings, loading, error };
}

export function useSchedule() {
  const [schedule, setSchedule] = useState<ScheduleMatchInfo[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchSchedule = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const adapter = await getAdapter();
      const result = await adapter.getSchedule();
      setSchedule(result);
      return result;
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      console.error('useSchedule error:', msg);
      setError(msg);
      return [];
    } finally {
      setLoading(false);
    }
  }, []);

  return { schedule, fetchSchedule, loading, error };
}

export function useRoster() {
  const [roster, setRoster] = useState<PlayerInfo[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchRoster = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const adapter = await getAdapter();
      const result = await adapter.getRoster();
      setRoster(result);
      return result;
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      console.error('useRoster error:', msg);
      setError(msg);
      return [];
    } finally {
      setLoading(false);
    }
  }, []);

  return { roster, fetchRoster, loading, error };
}

export function useInbox() {
  const [messages, setMessages] = useState<InboxMessageInfo[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchInbox = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const adapter = await getAdapter();
      const result = await adapter.getInbox();
      setMessages(result);
      return result;
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      console.error('useInbox error:', msg);
      setError(msg);
      return [];
    } finally {
      setLoading(false);
    }
  }, []);

  return { messages, fetchInbox, loading, error };
}

export function useResolveMessage() {
  const [resolving, setResolving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const resolve = useCallback(async (msgId: string): Promise<boolean> => {
    setResolving(true);
    setError(null);
    try {
      const adapter = await getAdapter();
      await adapter.resolveMessage(msgId);
      return true;
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      console.error('useResolveMessage error:', msg);
      setError(msg);
      return false;
    } finally {
      setResolving(false);
    }
  }, []);

  return { resolve, resolving, error };
}
