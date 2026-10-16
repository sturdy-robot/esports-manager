import react from "@vitejs/plugin-react";
import { defineConfig } from "vitest/config";

// Tauri expects a fixed dev server port (src-tauri/tauri.conf.json).
export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: { port: 1420, strictPort: true },
  envPrefix: ["VITE_", "TAURI_ENV_"],
  test: {
    environment: "jsdom",
    setupFiles: ["src/test/setup.ts"],
    restoreMocks: true,
    coverage: {
      provider: "v8",
      // ADR 0008: logic in src/lib and hooks must stay covered; components
      // are covered by Playwright.
      include: ["src/lib/**", "src/hooks/**"],
      thresholds: { lines: 80 },
    },
  },
});
