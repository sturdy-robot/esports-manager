import { useState, useEffect, useCallback } from "react";
import type { SaveInfo, GameInfo, NewGameParams, TeamInfo, PlayerInfo, InboxMessageInfo, StandingInfo, ScheduleMatchInfo, DraftSessionState, StartDraftParams, SimulateMatchResult, SeriesInfo } from "./api";

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
  playMatchDelegate(): Promise<GameInfo>;
  startDraft(params: StartDraftParams): Promise<DraftSessionState>;
  draftHover(champion: string): Promise<DraftSessionState>;
  draftLock(): Promise<DraftSessionState>;
  getDraftState(): Promise<DraftSessionState>;
  simulateMatch(): Promise<SimulateMatchResult>;
  getSeriesInfo(): Promise<SeriesInfo>;
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
    playMatchDelegate: api.playMatchDelegate,
    startDraft: api.startDraft,
    draftHover: api.draftHover,
    draftLock: api.draftLock,
    getDraftState: api.getDraftState,
    simulateMatch: api.simulateMatch,
    getSeriesInfo: api.getSeriesInfo,
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
  async playMatchDelegate() {
    await delay(500);
    return { ...MOCK_GAME_INFO, is_match_day: false };
  },
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  async startDraft(_params: StartDraftParams) {
    await delay(200);
    return MOCK_DRAFT_STATE();
  },
  async draftHover(champion: string) {
    await delay(50);
    return { ...MOCK_DRAFT_STATE(), active_hover: champion };
  },
  async draftLock() {
    await delay(100);
    const s = MOCK_DRAFT_STATE();
    return { ...s, current_step: s.current_step + 2 };
  },
  async getDraftState() {
    return MOCK_DRAFT_STATE();
  },
  async simulateMatch() {
    await delay(300);
    return MOCK_MATCH_RESULT();
  },
  async getSeriesInfo() {
    return MOCK_SERIES_INFO();
  },
};

const MOCK_CHAMPIONS = [
  "Orianna", "Azir", "Ahri", "Syndra", "Zed",
  "Malphite", "Ornn", "Gnar", "Fiora", "Jayce",
  "Lee Sin", "Viego", "Jarvan IV", "Jinx", "Kai'Sa",
  "Ezreal", "Aphelios", "Thresh", "Nautilus", "Lulu",
];

const mockSeriesBlueWins = 0;
const mockSeriesRedWins = 0;

function MOCK_SERIES_INFO(): SeriesInfo {
  return {
    match_id: 1,
    blue_team: 'T1',
    red_team: 'Gen.G',
    blue_wins: mockSeriesBlueWins,
    red_wins: mockSeriesRedWins,
    wins_needed: 2,
    is_complete: mockSeriesBlueWins >= 2 || mockSeriesRedWins >= 2,
    game_number: mockSeriesBlueWins + mockSeriesRedWins + 1,
  };
}

function MOCK_MATCH_RESULT(): SimulateMatchResult {
  return {
    winner: 'T1',
    duration_minutes: 32,
    blue_team: 'T1',
    red_team: 'Gen.G',
    blue_gold: 58200,
    red_gold: 51800,
    events: [
      { minute: 3, phase: 'Early', kind: 'solo_kill', commentary: 'FIRST BLOOD! Player1 takes down Player3 in the mid lane!', snapshot: null },
      { minute: 6, phase: 'Early', kind: 'dragon', commentary: 'T1 slays the dragon! That\'s dragon number 1 for them.', snapshot: null },
      { minute: 8, phase: 'Early', kind: 'herald', commentary: 'T1 takes down the Rift Herald! Time to crack open a tower.', snapshot: null },
      { minute: 10, phase: 'Early', kind: 'tower', commentary: 'T1 takes down the top tower! The map opens up.', snapshot: null },
      { minute: 14, phase: 'Early', kind: 'solo_kill', commentary: 'Player2 finds the solo kill onto Player4 in bot!', snapshot: null },
      { minute: 16, phase: 'Mid', kind: 'teamfight', commentary: 'T1 wins the teamfight 3 to 1!', snapshot: null },
      { minute: 18, phase: 'Mid', kind: 'tower', commentary: 'T1 takes down the mid tower! The map opens up.', snapshot: null },
      { minute: 20, phase: 'Mid', kind: 'baron', commentary: 'BARON NASHOR IS DOWN! T1 secures the baron buff — this is huge!', snapshot: null },
      { minute: 22, phase: 'Mid', kind: 'tower', commentary: 'T1 takes down the bot tower! The map opens up.', snapshot: null },
      { minute: 24, phase: 'Mid', kind: 'dragon', commentary: 'T1 slays the dragon! That\'s dragon number 2 for them.', snapshot: null },
      { minute: 26, phase: 'Late', kind: 'teamfight', commentary: 'AN ACE! T1 wipes the floor in that teamfight — 4 for 1!', snapshot: null },
      { minute: 28, phase: 'Late', kind: 'inhibitor', commentary: 'T1 destroys the mid inhibitor! Super minions incoming!', snapshot: null },
      { minute: 30, phase: 'Late', kind: 'baron', commentary: 'BARON NASHOR IS DOWN! T1 secures the baron buff — this is huge!', snapshot: null },
      { minute: 32, phase: 'Late', kind: 'nexus', commentary: 'AND THAT\'S THE GAME! T1 destroys the Nexus for the victory! GG!', snapshot: null },
    ],
  };
}

function MOCK_DRAFT_STATE(): DraftSessionState {
  return {
    current_step: 0,
    total_steps: 20,
    current_phase: "Ban",
    current_team: "Blue",
    blue_bans: [],
    red_bans: [],
    blue_picks: [],
    red_picks: [],
    active_hover: null,
    is_complete: false,
    is_player_turn: true,
    available_champions: [...MOCK_CHAMPIONS],
    timer_seconds: 30,
  };
}

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

export function usePlayMatchDelegate() {
  const [playing, setPlaying] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const playMatchDelegate = useCallback(async (): Promise<GameInfo | null> => {
    setPlaying(true);
    setError(null);
    try {
      const adapter = await getAdapter();
      return await adapter.playMatchDelegate();
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      console.error('usePlayMatchDelegate error:', msg);
      setError(msg);
      return null;
    } finally {
      setPlaying(false);
    }
  }, []);

  return { playMatchDelegate, playing, error };
}

export function useDraft() {
  const [draftState, setDraftState] = useState<DraftSessionState | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const startDraft = useCallback(async (params: StartDraftParams): Promise<DraftSessionState | null> => {
    setLoading(true);
    setError(null);
    try {
      const adapter = await getAdapter();
      const state = await adapter.startDraft(params);
      setDraftState(state);
      return state;
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      console.error('useDraft startDraft error:', msg);
      setError(msg);
      return null;
    } finally {
      setLoading(false);
    }
  }, []);

  const hover = useCallback(async (champion: string): Promise<DraftSessionState | null> => {
    setError(null);
    try {
      const adapter = await getAdapter();
      const state = await adapter.draftHover(champion);
      setDraftState(state);
      return state;
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      setError(msg);
      return null;
    }
  }, []);

  const lock = useCallback(async (): Promise<DraftSessionState | null> => {
    setError(null);
    try {
      const adapter = await getAdapter();
      const state = await adapter.draftLock();
      setDraftState(state);
      return state;
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      setError(msg);
      return null;
    }
  }, []);

  const refresh = useCallback(async (): Promise<DraftSessionState | null> => {
    try {
      const adapter = await getAdapter();
      const state = await adapter.getDraftState();
      setDraftState(state);
      return state;
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      setError(msg);
      return null;
    }
  }, []);

  return { draftState, startDraft, hover, lock, refresh, loading, error };
}

export function useMatchSimulation() {
  const [result, setResult] = useState<SimulateMatchResult | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const simulate = useCallback(async (): Promise<SimulateMatchResult | null> => {
    setLoading(true);
    setError(null);
    try {
      const adapter = await getAdapter();
      const res = await adapter.simulateMatch();
      setResult(res);
      return res;
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      console.error('useMatchSimulation error:', msg);
      setError(msg);
      return null;
    } finally {
      setLoading(false);
    }
  }, []);

  return { result, simulate, loading, error };
}

export function useSeriesInfo() {
  const [series, setSeries] = useState<SeriesInfo | null>(null);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async (): Promise<SeriesInfo | null> => {
    setError(null);
    try {
      const adapter = await getAdapter();
      const info = await adapter.getSeriesInfo();
      setSeries(info);
      return info;
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      setError(msg);
      return null;
    }
  }, []);

  return { series, refresh, error };
}
