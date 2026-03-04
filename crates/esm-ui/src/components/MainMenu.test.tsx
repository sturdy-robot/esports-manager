import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, it, expect, vi } from "vitest";
import { renderWithProviders } from "@/test/render";
import { MainMenu } from "./MainMenu";

describe("MainMenu", () => {
  it("renders the game title", () => {
    renderWithProviders(<MainMenu onNavigate={() => {}} />);
    expect(screen.getByText("eSports Manager")).toBeInTheDocument();
  });

  it("renders all four menu buttons", () => {
    renderWithProviders(<MainMenu onNavigate={() => {}} />);
    expect(screen.getByRole("button", { name: /new game/i })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /load game/i })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /settings/i })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /exit/i })).toBeInTheDocument();
  });

  it("calls onNavigate with 'new-game' when New Game is clicked", async () => {
    const user = userEvent.setup();
    const onNavigate = vi.fn();
    renderWithProviders(<MainMenu onNavigate={onNavigate} />);

    await user.click(screen.getByRole("button", { name: /new game/i }));
    expect(onNavigate).toHaveBeenCalledWith("new-game");
  });

  it("calls onNavigate with 'load-game' when Load Game is clicked", async () => {
    const user = userEvent.setup();
    const onNavigate = vi.fn();
    renderWithProviders(<MainMenu onNavigate={onNavigate} />);

    await user.click(screen.getByRole("button", { name: /load game/i }));
    expect(onNavigate).toHaveBeenCalledWith("load-game");
  });

  it("calls onNavigate with 'settings' when Settings is clicked", async () => {
    const user = userEvent.setup();
    const onNavigate = vi.fn();
    renderWithProviders(<MainMenu onNavigate={onNavigate} />);

    await user.click(screen.getByRole("button", { name: /settings/i }));
    expect(onNavigate).toHaveBeenCalledWith("settings");
  });

  it("renders version text", () => {
    renderWithProviders(<MainMenu onNavigate={() => {}} />);
    expect(screen.getByText(/v0\./)).toBeInTheDocument();
  });
});
