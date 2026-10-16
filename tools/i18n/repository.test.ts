/**
 * The repository-wide i18n gates (ADR 0007). They run with the other tests,
 * so `just check` and CI fail on any violation.
 */
import { readdirSync, readFileSync } from "node:fs";
import { join, relative } from "node:path";
import { describe, expect, it } from "vitest";
import { DEFAULT_LOCALE } from "../../src/lib/i18n/locales";
import { checkParity, loadCatalogs } from "./catalog";
import { checkUsage, findUsage } from "./usage";

const root = join(import.meta.dirname, "../..");
const localesDir = join(root, "src/locales");

/** UI source files: everything under src/ except tests and generated code. */
function uiSources(dir = join(root, "src")): string[] {
  return readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) {
      return entry.name === "test" ? [] : uiSources(path);
    }
    const isSource = /\.tsx?$/.test(entry.name);
    const excluded = /\.test\.tsx?$/.test(entry.name) || entry.name === "bindings.ts";
    return isSource && !excluded ? [path] : [];
  });
}

describe("i18n gates", () => {
  const catalogs = loadCatalogs(localesDir);

  it("every locale defines every message, with the same variables", () => {
    expect(checkParity(DEFAULT_LOCALE, catalogs)).toEqual([]);
  });

  it("UI code shows no text that bypasses Fluent, and uses every message", () => {
    const used = new Set<string>();
    const violations: string[] = [];
    for (const file of uiSources()) {
      const usage = findUsage(relative(root, file), readFileSync(file, "utf8"));
      for (const id of usage.ids) {
        used.add(id);
      }
      violations.push(...usage.violations);
    }
    const catalogIds = new Set(catalogs.get(DEFAULT_LOCALE)?.catalog.keys());
    expect([...violations, ...checkUsage(used, catalogIds)]).toEqual([]);
  });
});
