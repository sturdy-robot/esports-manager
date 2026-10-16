import { mockIPC } from "@tauri-apps/api/mocks";
import type {
  About,
  ContentText,
  Failure,
  GameOpened,
  NewGameInput,
  NewGameOptions,
  PlayerProfile,
  RosterView,
  SaveSummary,
} from "../bindings";
import newGameOptions from "./fixtures/new-game-options.json";
import player from "./fixtures/player.json";
import roster from "./fixtures/roster.json";
import savedGames from "./fixtures/saves.json";

/**
 * A stand-in backend for browser-only E2E runs (`vite --mode e2e`). It
 * answers with views the real backend produced (`src/test/fixtures`, kept
 * current by a Rust test) and the core package's real text, and remembers
 * the saves a test creates.
 *
 * A test can ask for a broken package with
 * `window.__ESM_E2E__ = { brokenPackage: true }` before the page loads.
 */
declare global {
  interface Window {
    __ESM_E2E__?: { brokenPackage?: boolean; saves?: boolean };
  }
}

const packageText = import.meta.glob<string>("../../content/core/locales/*/*.ftl", {
  query: "?raw",
  import: "default",
  eager: true,
});

function contentText(): ContentText[] {
  return Object.entries(packageText).map(([path, source]) => {
    const [locale = "en", file = ""] = path.split("/locales/")[1]?.split("/") ?? [];
    return { locale, file: `locales/${locale}/${file}`, source };
  });
}

function failure(id: string, args: Record<string, string> = {}): Failure {
  return {
    messages: [
      {
        id,
        args: Object.entries(args).map(([name, value]) => ({
          name,
          value: { kind: "text", value },
        })),
      },
    ],
  };
}

const brokenPackage: Failure = {
  messages: [
    {
      id: "problem-unknown-field",
      args: [
        { name: "field", value: { kind: "text", value: "reputaton" } },
        { name: "file", value: { kind: "text", value: "teams/seorabeol/azure_tigers.json" } },
        { name: "path", value: { kind: "text", value: "reputaton" } },
      ],
    },
  ],
};

export function installIpcMocks(): void {
  const options = window.__ESM_E2E__ ?? {};
  const saves: SaveSummary[] = options.saves ? (savedGames as SaveSummary[]) : [];
  let open: string | undefined;

  const handlers: Record<string, (args: Record<string, unknown>) => unknown> = {
    get_about: (): About => ({ version: "0.0.0-e2e" }),
    get_content_text: (): ContentText[] => contentText(),
    get_new_game_options: (): NewGameOptions => {
      if (options.brokenPackage) {
        throw brokenPackage;
      }
      return newGameOptions as NewGameOptions;
    },
    start_new_game: (args): GameOpened => {
      const input = args.input as NewGameInput;
      if (input.managerName.trim() === "") {
        throw failure("error-manager-name-empty");
      }
      const file = `${input.saveName.toLowerCase().replace(/[^a-z0-9]+/g, "-")}.esmsave`;
      saves.push({
        status: "ready",
        file,
        name: input.saveName,
        managerName: input.managerName,
        teamMessage: input.team.replace(":", "-"),
        gameDate: "2027-01-01",
        lastPlayedAt: 1_792_000_000,
      });
      open = file;
      return { file, team: input.team };
    },
    list_saves: (): SaveSummary[] => saves,
    open_save: (args): GameOpened => {
      const file = args.file as string;
      if (!saves.some((save) => save.file === file)) {
        throw failure("error-save-not-found", { file });
      }
      open = file;
      return { file, team: "team:azure_tigers" };
    },
    get_roster: (): RosterView => {
      if (open === undefined) {
        throw failure("error-no-save-open");
      }
      return roster as RosterView;
    },
    get_player: (args): PlayerProfile => {
      const id = args.id as string;
      if (open === undefined || id !== "player:kairos") {
        throw failure("error-unknown-player", { player: id });
      }
      return player as PlayerProfile;
    },
    "plugin:window|close": () => null,
  };

  mockIPC((command, args) => {
    const handler = handlers[command];
    if (handler === undefined) {
      throw new Error(`No E2E mock for command "${command}"`);
    }
    return handler((args ?? {}) as Record<string, unknown>);
  });
}
