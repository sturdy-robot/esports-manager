import { describe, expect, it } from "vitest";
import { checkUsage, findUsage } from "./usage";

describe("findUsage", () => {
  it("collects ids from <Localized> and getString()", () => {
    const usage = findUsage(
      "a.tsx",
      'const a = <Localized id="title"><h1 /></Localized>;\nconst b = l10n.getString("loading");\n',
    );
    expect(usage).toEqual({ ids: ["title", "loading"], violations: [] });
  });

  it("reports visible text and attributes written in code", () => {
    const usage = findUsage(
      "a.tsx",
      'const a = <p title="Hi">\n  Hello\n</p>;\nconst b = <img alt="" className="x" />;\n',
    );
    expect(usage.violations).toEqual([
      'a.tsx:1: attribute title="Hi" must come from a Fluent message',
      'a.tsx:1: text "Hello" must come from a Fluent message',
    ]);
  });

  it("allows whitespace-only text between elements", () => {
    expect(findUsage("a.tsx", "const a = <div>\n  <span />\n</div>;").violations).toEqual([]);
  });

  it("requires literal ids so they can be checked", () => {
    const usage = findUsage("a.tsx", "const a = <Localized id={k} />;\nl10n.getString(k);\n");
    expect(usage.violations).toEqual([
      "a.tsx:1: <Localized> needs a string-literal id so it can be checked",
      "a.tsx:2: getString() needs a string-literal id so it can be checked",
    ]);
  });

  it("reports parse errors", () => {
    expect(findUsage("a.tsx", "const = ;").violations[0]).toMatch(/^a\.tsx: could not parse/);
  });
});

describe("checkUsage", () => {
  it("passes when used ids and messages match, ignoring terms", () => {
    expect(checkUsage(new Set(["a"]), new Set(["a", "-brand"]))).toEqual([]);
  });

  it("reports undefined and unused messages", () => {
    expect(checkUsage(new Set(["a", "z"]), new Set(["a", "b"]))).toEqual([
      'message "z" is used in code but not defined in the en catalog',
      'message "b" is defined but never used',
    ]);
  });
});
