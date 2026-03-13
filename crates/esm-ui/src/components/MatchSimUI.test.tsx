import { describe, it, expect, vi } from "vitest";
import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { renderWithProviders } from "../test/render";
import { MatchSimUI } from "./MatchSimUI";
import type { SimulateMatchResult, MatchEventInfo } from "@/lib/api";

const SAMPLE_EVENTS: MatchEventInfo[] = [
  {
    minute: 3,
    phase: "Early",
    kind: "solo_kill",
    commentary: "FIRST BLOOD! Player1 takes down Player3 in the mid lane!",
    snapshot: null,
  },
  {
    minute: 8,
    phase: "Early",
    kind: "dragon",
    commentary: "Blue slays the dragon! That's dragon number 1 for them.",
    snapshot: null,
  },
  {
    minute: 16,
    phase: "Mid",
    kind: "teamfight",
    commentary: "Blue wins the teamfight 3 to 1!",
    snapshot: null,
  },
  {
    minute: 20,
    phase: "Mid",
    kind: "baron",
    commentary: "BARON NASHOR IS DOWN! Blue secures the baron buff!",
    snapshot: null,
  },
  {
    minute: 32,
    phase: "Late",
    kind: "nexus",
    commentary: "AND THAT'S THE GAME! Blue destroys the Nexus! GG!",
    snapshot: {
      blue_players: [],
      red_players: [],
      blue_team_gold: 58200,
      red_team_gold: 51800,
      blue_towers: 11,
      red_towers: 8,
      blue_inhibitors: 2,
      red_inhibitors: 0,
      dragons_blue: 3,
      dragons_red: 1,
      baron_alive: false,
      baron_timer: 4,
      dragon_timer: 2,
      herald_available: false,
    },
  },
];

function makeResult(
  overrides: Partial<SimulateMatchResult> = {},
): SimulateMatchResult {
  return {
    winner: "T1",
    duration_minutes: 32,
    blue_team: "T1",
    red_team: "Gen.G",
    blue_gold: 58200,
    red_gold: 51800,
    blue_roster: [],
    red_roster: [],
    events: SAMPLE_EVENTS,
    ...overrides,
  };
}

/** Click the skip button to reveal all events instantly. */
async function skipToEnd(user: ReturnType<typeof userEvent.setup>) {
  const skipBtn = screen.getByTitle("Skip to end");
  await user.click(skipBtn);
}

describe("MatchSimUI", () => {
  const defaultProps = {
    result: makeResult(),
    onComplete: vi.fn(),
  };

  it("renders team names", () => {
    renderWithProviders(<MatchSimUI {...defaultProps} />);
    expect(screen.getAllByText("T1").length).toBeGreaterThanOrEqual(1);
    expect(screen.getAllByText("Gen.G").length).toBeGreaterThanOrEqual(1);
  });

  it("renders the event log with commentary after skip", async () => {
    const user = userEvent.setup();
    renderWithProviders(<MatchSimUI {...defaultProps} />);
    await skipToEnd(user);
    expect(screen.getByText(/FIRST BLOOD/)).toBeInTheDocument();
  });

  it("shows the game clock after skip", async () => {
    const user = userEvent.setup();
    renderWithProviders(<MatchSimUI {...defaultProps} />);
    await skipToEnd(user);
    const matches = screen.getAllByText(/32:00/);
    expect(matches.length).toBeGreaterThanOrEqual(1);
  });

  it("shows the winner after skip", async () => {
    const user = userEvent.setup();
    renderWithProviders(<MatchSimUI {...defaultProps} />);
    await skipToEnd(user);
    expect(screen.getByText(/T1\s+wins/i)).toBeInTheDocument();
  });

  it("calls onComplete when continue button is clicked after skip", async () => {
    const onComplete = vi.fn();
    const user = userEvent.setup();
    renderWithProviders(
      <MatchSimUI {...defaultProps} onComplete={onComplete} />,
    );
    await skipToEnd(user);

    await user.click(screen.getByRole("button", { name: /continue/i }));
    expect(onComplete).toHaveBeenCalled();
  });

  it("displays event timestamps after skip", async () => {
    const user = userEvent.setup();
    renderWithProviders(<MatchSimUI {...defaultProps} />);
    await skipToEnd(user);
    expect(screen.getByText("3:00")).toBeInTheDocument();
    expect(screen.getByText("8:00")).toBeInTheDocument();
  });

  it("shows phase badges on events after skip", async () => {
    const user = userEvent.setup();
    renderWithProviders(<MatchSimUI {...defaultProps} />);
    await skipToEnd(user);
    const earlyBadges = screen.getAllByText("Early");
    expect(earlyBadges.length).toBeGreaterThanOrEqual(1);
  });

  it("shows speed controls before match ends", () => {
    renderWithProviders(<MatchSimUI {...defaultProps} />);
    expect(screen.getByText("1x")).toBeInTheDocument();
  });

  it("starts in playing state and shows pause button", () => {
    renderWithProviders(<MatchSimUI {...defaultProps} />);
    expect(screen.getByTitle("Skip to end")).toBeInTheDocument();
  });

  it("starts with no events visible", () => {
    renderWithProviders(<MatchSimUI {...defaultProps} />);
    expect(screen.getByText(/Match starting/)).toBeInTheDocument();
    // No commentary visible yet since events are progressively revealed
    expect(screen.queryByText(/FIRST BLOOD/)).not.toBeInTheDocument();
  });

  it("uses flex-1 layout to fill available space instead of h-full", () => {
    const { container } = renderWithProviders(<MatchSimUI {...defaultProps} />);
    const root = container.firstElementChild as HTMLElement;
    expect(root.className).toContain("flex-1");
    expect(root.className).toContain("min-h-0");
    expect(root.className).not.toContain("h-full");
  });

  it("shows winner banner with victory styling after match ends", async () => {
    const user = userEvent.setup();
    renderWithProviders(<MatchSimUI {...defaultProps} />);
    await skipToEnd(user);
    const banner = screen.getByTestId("winner-banner");
    expect(banner).toBeInTheDocument();
    expect(banner.textContent).toMatch(/T1\s+wins/i);
  });

  it("shows tower and inhibitor counts when snapshot data is available", async () => {
    const user = userEvent.setup();
    renderWithProviders(<MatchSimUI {...defaultProps} />);
    await skipToEnd(user);
    const towersLabel = screen.getByText("Towers");
    const inhibitorsLabel = screen.getByText("Inhibitors");
    expect(towersLabel).toBeInTheDocument();
    expect(inhibitorsLabel).toBeInTheDocument();
    expect(towersLabel.parentElement?.textContent).toContain("11 – 8");
    expect(inhibitorsLabel.parentElement?.textContent).toContain("2 – 0");
  });
});
