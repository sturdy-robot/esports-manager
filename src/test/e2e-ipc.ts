import { mockIPC } from "@tauri-apps/api/mocks";
import type { About } from "../bindings";

/**
 * Backend answers for browser-only E2E runs (`vite --mode e2e`), keyed by
 * Tauri command name. Return types come from the generated bindings, so a
 * changed DTO breaks the build here too.
 */
const handlers: Record<string, () => unknown> = {
  get_about: (): About => ({ version: "0.0.0-e2e" }),
};

export function installIpcMocks(): void {
  mockIPC((command) => {
    const handler = handlers[command];
    if (handler === undefined) {
      throw new Error(`No E2E mock for command "${command}"`);
    }
    return handler();
  });
}
