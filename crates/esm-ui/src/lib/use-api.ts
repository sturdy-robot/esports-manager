import { useState, useEffect, useCallback } from "react";
import type { SaveInfo, GameInfo, NewGameParams, TeamInfo } from "./api";

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
  manager_nickname: "kkOma",
  team_name: "T1",
  teams_count: 6,
};

// ---------------------------------------------------------------------------
// API adapter — real Tauri calls or mock implementations
// ---------------------------------------------------------------------------

interface ApiAdapter {
  listSaves(): Promise<SaveInfo[]>;
  loadSave(name: string): Promise<GameInfo>;
  deleteSave(name: string): Promise<void>;
  newGame(params: NewGameParams): Promise<GameInfo>;
  saveGame(name: string): Promise<void>;
  loadDatapack(path: string): Promise<TeamInfo[]>;
  getGameInfo(): Promise<GameInfo>;
}

async function tauriAdapter(): Promise<ApiAdapter> {
  const api = await import("./api");
  return {
    listSaves: api.listSaves,
    loadSave: api.loadSave,
    deleteSave: api.deleteSave,
    newGame: api.newGame,
    saveGame: api.saveGame,
    loadDatapack: api.loadDatapack,
    getGameInfo: api.getGameInfo,
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
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  async loadDatapack(_path: string) {
    await delay(200);
    return [...MOCK_TEAMS];
  },
  async getGameInfo() {
    return { ...MOCK_GAME_INFO };
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
