import { screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { renderWithProviders } from "@/test/render";
import { Results } from "./Results";
import type { MatchResult } from "./Results";

const MOCK_RESULTS: MatchResult[] = [
  {
    id: "r1",
    homeTeam: "T1",
    awayTeam: "DRX",
    homeWins: 2,
    awayWins: 0,
    day: 3,
    month: 1,
    year: 2025,
    bestOf: 3,
    playerTeamWon: true,
  },
  {
    id: "r2",
    homeTeam: "Gen.G",
    awayTeam: "T1",
    homeWins: 2,
    awayWins: 1,
    day: 5,
    month: 1,
    year: 2025,
    bestOf: 3,
    playerTeamWon: false,
  },
  {
    id: "r3",
    homeTeam: "T1",
    awayTeam: "KT Rolster",
    homeWins: 2,
    awayWins: 1,
    day: 8,
    month: 1,
    year: 2025,
    bestOf: 3,
    playerTeamWon: true,
  },
];

describe("Results", () => {
  it("renders the Results heading", () => {
    renderWithProviders(<Results results={MOCK_RESULTS} />);
    expect(
      screen.getByRole("heading", { name: /results/i })
    ).toBeInTheDocument();
  });

  it("renders each match result", () => {
    renderWithProviders(<Results results={MOCK_RESULTS} />);
    expect(screen.getByText("DRX")).toBeInTheDocument();
    expect(screen.getByText("KT Rolster")).toBeInTheDocument();
  });

  it("shows match scores", () => {
    renderWithProviders(<Results results={MOCK_RESULTS} />);
    expect(screen.getByText("2 - 0")).toBeInTheDocument();
    expect(screen.getAllByText("2 - 1").length).toBe(2);
  });

  it("shows win/loss badges", () => {
    renderWithProviders(<Results results={MOCK_RESULTS} />);
    expect(screen.getAllByText("WIN").length).toBe(2);
    expect(screen.getAllByText("LOSS").length).toBe(1);
  });

  it("renders empty state when no results", () => {
    renderWithProviders(<Results results={[]} />);
    expect(screen.getByText(/no results/i)).toBeInTheDocument();
  });
});
