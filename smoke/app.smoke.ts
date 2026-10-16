/**
 * Smoke test of the real desktop binary through tauri-driver (ADR 0008):
 * the app starts, the frontend loads, and a command reaches the Rust side.
 *
 * Build first with `pnpm tauri build --debug --no-bundle`; `just smoke`
 * does both. Needs tauri-driver and, on Linux, WebKitWebDriver.
 */
import { type ChildProcess, spawn } from "node:child_process";
import { existsSync } from "node:fs";
import { createConnection } from "node:net";
import { join } from "node:path";
import { remote } from "webdriverio";

const DRIVER_PORT = 4444;
const binaryName = process.platform === "win32" ? "esports-manager.exe" : "esports-manager";
const application = join(import.meta.dirname, "../target/debug", binaryName);

function waitForPort(port: number, timeoutMs: number): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  return new Promise((resolve, reject) => {
    const attempt = () => {
      const socket = createConnection({ port, host: "127.0.0.1" });
      socket.once("connect", () => {
        socket.end();
        resolve();
      });
      socket.once("error", () => {
        socket.destroy();
        if (Date.now() > deadline) {
          reject(new Error(`tauri-driver did not listen on port ${port}`));
        } else {
          setTimeout(attempt, 200);
        }
      });
    };
    attempt();
  });
}

async function main(): Promise<void> {
  if (!existsSync(application)) {
    throw new Error(`${application} not found; run \`pnpm tauri build --debug --no-bundle\``);
  }
  const driver: ChildProcess = spawn("tauri-driver", ["--port", String(DRIVER_PORT)], {
    stdio: "inherit",
  });
  try {
    await waitForPort(DRIVER_PORT, 15_000);
    const browser = await remote({
      hostname: "127.0.0.1",
      port: DRIVER_PORT,
      logLevel: "warn",
      capabilities: { "tauri:options": { application } } as WebdriverIO.Capabilities,
    });
    try {
      const heading = await browser.$("h1");
      await heading.waitForDisplayed({ timeout: 20_000 });
      const title = await heading.getText();
      if (title !== "eSports Manager") {
        throw new Error(`unexpected heading "${title}"`);
      }
      const version = await browser.$('[data-testid="version"]');
      await version.waitForDisplayed({ timeout: 20_000 });
      const text = await version.getText();
      if (!/^v\d+\.\d+\.\d+$/.test(text)) {
        throw new Error(`unexpected version text "${text}"`);
      }
      console.log(`smoke test passed: "${title}" ${text}`);
    } finally {
      await browser.deleteSession();
    }
  } finally {
    driver.kill();
  }
}

await main();
