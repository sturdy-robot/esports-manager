/**
 * Smoke test of the real desktop binary through tauri-driver (ADR 0008):
 * the app starts, the frontend loads, and a command reaches the Rust side.
 *
 * Build first with `pnpm tauri build --debug --no-bundle`; `just smoke`
 * does both. Needs tauri-driver and, on Linux, WebKitWebDriver.
 */
import { type ChildProcess, spawn } from "node:child_process";
import { existsSync, rmSync, writeFileSync } from "node:fs";
import { createConnection } from "node:net";
import { join } from "node:path";
import { remote } from "webdriverio";

const DRIVER_PORT = 4444;
const binaryName = process.platform === "win32" ? "esports-manager.exe" : "esports-manager";
const binaryDir = join(import.meta.dirname, "../target/debug");
const application = join(binaryDir, binaryName);
// Nothing the app writes may land in the OS data folder of the machine
// running the test. Driven runs get a scratch XDG_DATA_HOME (Linux, where the
// smoke test runs); the portable check uses target/debug/data.
const scratchData = join(binaryDir, "smoke-data");
const portableMarker = join(binaryDir, "portable");
const portableData = join(binaryDir, "data");

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
  rmSync(scratchData, { recursive: true, force: true });
  const driver: ChildProcess = spawn("tauri-driver", ["--port", String(DRIVER_PORT)], {
    stdio: "inherit",
    env: { ...process.env, XDG_DATA_HOME: scratchData },
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
      if (!existsSync(join(scratchData, "esports-manager", "saves"))) {
        throw new Error(`the app did not create its saves folder under ${scratchData}`);
      }
      console.log(`smoke test passed: "${title}" ${text}`);
    } finally {
      await browser.deleteSession();
    }
  } finally {
    driver.kill();
  }
}

/**
 * Starts the app in portable mode for a few seconds and checks that the
 * webview kept its data inside the portable folder (SAVE-006). WebKitWebDriver
 * can't drive a webview with its own data folder, so this run isn't driven.
 */
async function portable(): Promise<void> {
  rmSync(portableData, { recursive: true, force: true });
  writeFileSync(portableMarker, "");
  try {
    const app = spawn(application, [], { stdio: "inherit" });
    const folders = ["saves", "webview"].map((folder) => join(portableData, folder));
    const deadline = Date.now() + 60_000;
    while (!folders.every(existsSync) && Date.now() < deadline && app.exitCode === null) {
      await new Promise((resolve) => setTimeout(resolve, 500));
    }
    app.kill();
    for (const folder of folders) {
      if (!existsSync(folder)) {
        throw new Error(`portable mode did not create ${folder} (app exit code ${app.exitCode})`);
      }
    }
    console.log(`portable check passed: data in ${portableData}`);
  } finally {
    rmSync(portableMarker, { force: true });
  }
}

await main();
await portable();
