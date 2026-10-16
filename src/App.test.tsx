import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { App } from "./App";
import { commands } from "./bindings";
import { createLocalization } from "./lib/i18n/localization";
import { testQueryClient } from "./test/render";

vi.mock("./bindings", () => ({ commands: { getAbout: vi.fn() } }));

describe("App", () => {
  it("renders the shell and the home route", async () => {
    // Given
    vi.mocked(commands.getAbout).mockResolvedValue({ version: "1.2.3" });
    // When
    render(<App l10n={createLocalization(["en"])} queryClient={testQueryClient()} />);
    // Then
    expect(await screen.findByRole("heading", { name: "eSports Manager" })).toBeInTheDocument();
    expect(await screen.findByTestId("version")).toHaveTextContent("v1.2.3");
  });
});
