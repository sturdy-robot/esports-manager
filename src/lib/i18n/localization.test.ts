import { describe, expect, it } from "vitest";
import { SUPPORTED_LOCALES } from "./locales";
import { createBundle, createLocalization } from "./localization";

describe("createBundle", () => {
  it.each(SUPPORTED_LOCALES)("loads the %s catalog", (locale) => {
    const bundle = createBundle(locale);
    expect(bundle.hasMessage("app-title")).toBe(true);
  });
});

describe("createLocalization", () => {
  it("formats messages with arguments in the preferred locale", () => {
    const l10n = createLocalization(["pt-BR"]);
    expect(l10n.getString("app-loading")).toBe("Carregando…");
    expect(l10n.getString("app-version", { version: "1.2.3" })).toBe("v1.2.3");
  });

  it("resolves terms inside messages", () => {
    expect(createLocalization(["en"]).getString("app-title")).toBe("eSports Manager");
  });

  it("falls back to English for unsupported locales", () => {
    expect(createLocalization(["ja"]).getString("app-loading")).toBe("Loading…");
  });
});
