import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, it, expect, vi } from "vitest";
import { renderWithProviders } from "@/test/render";
import { TopBar } from "./TopBar";

describe("TopBar", () => {
  const defaults = {
    title: "Dashboard",
    year: 2025,
    month: 1,
    day: 1,
    phase: "Morning",
    teamName: "T1",
    onContinue: vi.fn(),
    onSave: vi.fn(),
  };

  it("renders the page title", () => {
    renderWithProviders(<TopBar {...defaults} />);
    expect(screen.getByText("Dashboard")).toBeInTheDocument();
  });

  it("shows the game date", () => {
    renderWithProviders(<TopBar {...defaults} />);
    expect(screen.getByText(/2025/)).toBeInTheDocument();
    expect(screen.getByText(/jan/i)).toBeInTheDocument();
  });

  it("shows the day phase", () => {
    renderWithProviders(<TopBar {...defaults} />);
    expect(screen.getByText(/morning/i)).toBeInTheDocument();
  });

  it("shows the team name", () => {
    renderWithProviders(<TopBar {...defaults} />);
    expect(screen.getByText("T1")).toBeInTheDocument();
  });

  it("renders the Continue button", () => {
    renderWithProviders(<TopBar {...defaults} />);
    expect(screen.getByRole("button", { name: /continue/i })).toBeInTheDocument();
  });

  it("calls onContinue when Continue is clicked", async () => {
    const user = userEvent.setup();
    const onContinue = vi.fn();
    renderWithProviders(<TopBar {...defaults} onContinue={onContinue} />);

    await user.click(screen.getByRole("button", { name: /continue/i }));
    expect(onContinue).toHaveBeenCalledTimes(1);
  });

  it("renders Save button", () => {
    renderWithProviders(<TopBar {...defaults} />);
    expect(screen.getByRole("button", { name: /^save$/i })).toBeInTheDocument();
  });

  it("calls onSave when Save is clicked", async () => {
    const user = userEvent.setup();
    const onSave = vi.fn();
    renderWithProviders(<TopBar {...defaults} onSave={onSave} />);

    await user.click(screen.getByRole("button", { name: /^save$/i }));
    expect(onSave).toHaveBeenCalledTimes(1);
  });

  it("renders theme toggle button", () => {
    renderWithProviders(<TopBar {...defaults} />);
    expect(screen.getByTitle(/switch to/i)).toBeInTheDocument();
  });

  it("renders Play Match instead of Continue on match day", () => {
    const onPlayMatch = vi.fn();
    renderWithProviders(<TopBar {...defaults} isMatchDay={true} onPlayMatch={onPlayMatch} />);
    expect(screen.getByRole("button", { name: /play match/i })).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: /continue/i })).not.toBeInTheDocument();
  });

  it("calls onPlayMatch when Play Match is clicked", async () => {
    const user = userEvent.setup();
    const onPlayMatch = vi.fn();
    renderWithProviders(<TopBar {...defaults} isMatchDay={true} onPlayMatch={onPlayMatch} />);

    await user.click(screen.getByRole("button", { name: /play match/i }));
    expect(onPlayMatch).toHaveBeenCalledTimes(1);
  });
});
