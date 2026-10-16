import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { App } from "./App";
import { commands } from "./bindings";

vi.mock("./bindings", () => ({
  commands: { getAbout: vi.fn() },
}));

describe("App", () => {
  it("shows the version reported by the backend", async () => {
    // Given
    vi.mocked(commands.getAbout).mockResolvedValue({ version: "1.2.3" });
    // When
    render(<App />);
    // Then
    expect(await screen.findByTestId("version")).toHaveTextContent("v1.2.3");
  });
});
