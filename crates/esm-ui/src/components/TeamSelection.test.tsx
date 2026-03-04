import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, it, expect, vi } from "vitest";
import { renderWithProviders } from "@/test/render";
import { TeamSelection } from "./TeamSelection";
import type { TeamOption } from "./TeamSelection";

describe("TeamSelection", () => {
  const sampleTeams: TeamOption[] = [
    { name: "T1", tag: "T1", playerCount: 5, reputation: 85 },
    { name: "Gen.G", tag: "GEN", playerCount: 5, reputation: 78 },
    { name: "DRX", tag: "DRX", playerCount: 5, reputation: 62 },
  ];

  const defaults = {
    teams: sampleTeams,
    onSelect: vi.fn(),
    onBack: vi.fn(),
  };

  it("renders the Team Selection heading", () => {
    renderWithProviders(<TeamSelection {...defaults} />);
    expect(
      screen.getByRole("heading", { name: /team selection/i })
    ).toBeInTheDocument();
  });

  it("renders all team cards", () => {
    renderWithProviders(<TeamSelection {...defaults} />);
    // Tag and name may render separately, so use getAllByText
    expect(screen.getAllByText("T1").length).toBeGreaterThanOrEqual(1);
    expect(screen.getAllByText("Gen.G").length).toBeGreaterThanOrEqual(1);
    expect(screen.getAllByText("DRX").length).toBeGreaterThanOrEqual(1);
  });

  it("shows player count for each team", () => {
    renderWithProviders(<TeamSelection {...defaults} />);
    const badges = screen.getAllByText(/5 players/i);
    expect(badges.length).toBe(3);
  });

  it("shows reputation for each team", () => {
    renderWithProviders(<TeamSelection {...defaults} />);
    expect(screen.getByText("85")).toBeInTheDocument();
    expect(screen.getByText("78")).toBeInTheDocument();
    expect(screen.getByText("62")).toBeInTheDocument();
  });

  it("Continue button is disabled when no team is selected", () => {
    renderWithProviders(<TeamSelection {...defaults} />);
    expect(screen.getByRole("button", { name: /continue/i })).toBeDisabled();
  });

  it("clicking a team card selects it and enables Continue", async () => {
    const user = userEvent.setup();
    renderWithProviders(<TeamSelection {...defaults} />);

    // Click the first team card (T1)
    const teamCards = screen.getAllByRole("button").filter(b => b.textContent?.includes("T1") && b.textContent?.includes("players"));
    await user.click(teamCards[0]);
    expect(screen.getByRole("button", { name: /continue/i })).toBeEnabled();
  });

  it("calls onSelect with team index when Continue is clicked", async () => {
    const user = userEvent.setup();
    const onSelect = vi.fn();
    renderWithProviders(<TeamSelection {...defaults} onSelect={onSelect} />);

    // Click the Gen.G team card
    const teamCards = screen.getAllByRole("button").filter(b => b.textContent?.includes("Gen.G") && b.textContent?.includes("players"));
    await user.click(teamCards[0]);
    await user.click(screen.getByRole("button", { name: /continue/i }));
    expect(onSelect).toHaveBeenCalledWith(1);
  });

  it("calls onBack when Back is clicked", async () => {
    const user = userEvent.setup();
    const onBack = vi.fn();
    renderWithProviders(<TeamSelection {...defaults} onBack={onBack} />);

    await user.click(screen.getByRole("button", { name: /back/i }));
    expect(onBack).toHaveBeenCalledTimes(1);
  });

  it("shows empty state when no teams are provided", () => {
    renderWithProviders(<TeamSelection {...defaults} teams={[]} />);
    expect(screen.getByText(/no teams available/i)).toBeInTheDocument();
  });
});
