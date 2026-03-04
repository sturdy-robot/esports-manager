import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, it, expect } from "vitest";
import { renderWithProviders } from "@/test/render";
import App from "./App";

describe("App navigation", () => {
  it("starts on the main menu", () => {
    renderWithProviders(<App />);
    expect(screen.getByText("eSports Manager")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /new game/i })).toBeInTheDocument();
  });

  it("navigates to new-game screen when New Game is clicked", async () => {
    const user = userEvent.setup();
    renderWithProviders(<App />);

    await user.click(screen.getByRole("button", { name: /new game/i }));
    expect(screen.getByRole("heading", { name: /manager creation/i })).toBeInTheDocument();
  });

  it("navigates to load-game screen when Load Game is clicked", async () => {
    const user = userEvent.setup();
    renderWithProviders(<App />);

    await user.click(screen.getByRole("button", { name: /load game/i }));
    expect(screen.getByText(/load game/i)).toBeInTheDocument();
  });

  it("can navigate back to main menu from new-game", async () => {
    const user = userEvent.setup();
    renderWithProviders(<App />);

    await user.click(screen.getByRole("button", { name: /new game/i }));
    await user.click(screen.getByRole("button", { name: /back/i }));
    expect(screen.getByRole("button", { name: /new game/i })).toBeInTheDocument();
  });

  it("can navigate back to main menu from load-game", async () => {
    const user = userEvent.setup();
    renderWithProviders(<App />);

    await user.click(screen.getByRole("button", { name: /load game/i }));
    await user.click(screen.getByRole("button", { name: /back/i }));
    expect(screen.getByRole("button", { name: /new game/i })).toBeInTheDocument();
  });
});
