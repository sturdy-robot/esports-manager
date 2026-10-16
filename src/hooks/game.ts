import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { commands, type NewGameInput } from "../bindings";
import { unwrap } from "../lib/backend/result";

/** Every query that depends on which save is open. */
const SAVE_QUERIES = ["contentText", "saves", "roster", "player"];

/** The package text the UI's Fluent bundles need: team, region and role names. */
export function useContentText() {
  return useQuery({
    queryKey: ["contentText"],
    queryFn: () => unwrap(commands.getContentText()),
  });
}

export function useNewGameOptions() {
  return useQuery({
    queryKey: ["newGameOptions"],
    queryFn: () => unwrap(commands.getNewGameOptions()),
  });
}

export function useSaves() {
  return useQuery({ queryKey: ["saves"], queryFn: () => unwrap(commands.listSaves()) });
}

/** The roster of `team`, or of the user's team. */
export function useRoster(team?: string | undefined) {
  return useQuery({
    queryKey: ["roster", team ?? null],
    queryFn: () => unwrap(commands.getRoster(team ?? null)),
  });
}

export function usePlayer(id: string) {
  return useQuery({ queryKey: ["player", id], queryFn: () => unwrap(commands.getPlayer(id)) });
}

function useSaveChange<T>(run: (input: T) => ReturnType<typeof commands.openSave>) {
  const client = useQueryClient();
  return useMutation({
    mutationFn: (input: T) => unwrap(run(input)),
    onSuccess: async () => {
      await Promise.all(SAVE_QUERIES.map((key) => client.invalidateQueries({ queryKey: [key] })));
    },
  });
}

/** Creates a game and opens it. */
export function useStartNewGame() {
  return useSaveChange((input: NewGameInput) => commands.startNewGame(input));
}

export function useOpenSave() {
  return useSaveChange((file: string) => commands.openSave(file));
}

/** Closes the window, which ends the app (main menu, desktop only). */
export function useQuit() {
  return useMutation({ mutationFn: () => getCurrentWindow().close() });
}
