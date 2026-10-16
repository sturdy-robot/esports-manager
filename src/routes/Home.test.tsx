import { screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { commands } from "../bindings";
import { renderWithProviders } from "../test/render";
import { Home } from "./Home";

vi.mock("../bindings", () => ({ commands: { getAbout: vi.fn() } }));

describe("Home", () => {
  it("shows a loading message, then the version from the backend", async () => {
    // Given
    vi.mocked(commands.getAbout).mockResolvedValue({ version: "1.2.3" });
    // When
    renderWithProviders(<Home />);
    // Then
    expect(screen.getByText("Loading…")).toBeInTheDocument();
    expect(await screen.findByTestId("version")).toHaveTextContent("v1.2.3");
  });

  it("speaks the user's language", async () => {
    // Given
    vi.mocked(commands.getAbout).mockReturnValue(new Promise(() => {}));
    // When
    renderWithProviders(<Home />, { locale: "pt-BR" });
    // Then
    expect(screen.getByText("Carregando…")).toBeInTheDocument();
  });

  it("tells the user when the backend fails", async () => {
    // Given
    vi.mocked(commands.getAbout).mockRejectedValue(new Error("boom"));
    // When
    renderWithProviders(<Home />);
    // Then
    expect(await screen.findByRole("alert")).toHaveTextContent(
      "Something went wrong while talking to the game.",
    );
  });
});
