import { screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { renderWithProviders } from "@/test/render";
import { Standings } from "./Standings";
import type { StandingsEntry } from "./Standings";

const MOCK_STANDINGS: StandingsEntry[] = [
  { rank: 1, teamName: "T1", wins: 8, losses: 2, mapWins: 18, mapLosses: 7, streak: "W3" },
  { rank: 2, teamName: "Gen.G", wins: 7, losses: 3, mapWins: 16, mapLosses: 9, streak: "L1" },
  { rank: 3, teamName: "Dplus KIA", wins: 6, losses: 4, mapWins: 14, mapLosses: 11, streak: "W1" },
  { rank: 4, teamName: "Hanwha Life", wins: 5, losses: 5, mapWins: 13, mapLosses: 12, streak: "L2" },
  { rank: 5, teamName: "KT Rolster", wins: 3, losses: 7, mapWins: 9, mapLosses: 16, streak: "L3" },
  { rank: 6, teamName: "DRX", wins: 1, losses: 9, mapWins: 5, mapLosses: 20, streak: "L5" },
];

describe("Standings", () => {
  it("renders the Standings heading", () => {
    renderWithProviders(<Standings entries={MOCK_STANDINGS} />);
    expect(
      screen.getByRole("heading", { name: /standings/i })
    ).toBeInTheDocument();
  });

  it("renders a row for each team", () => {
    renderWithProviders(<Standings entries={MOCK_STANDINGS} />);
    expect(screen.getByText("T1")).toBeInTheDocument();
    expect(screen.getByText("Gen.G")).toBeInTheDocument();
    expect(screen.getByText("DRX")).toBeInTheDocument();
  });

  it("shows win-loss records", () => {
    renderWithProviders(<Standings entries={MOCK_STANDINGS} />);
    expect(screen.getByText("8-2")).toBeInTheDocument();
    expect(screen.getByText("7-3")).toBeInTheDocument();
  });

  it("shows map differential", () => {
    renderWithProviders(<Standings entries={MOCK_STANDINGS} />);
    // T1 map diff = 18 - 7 = +11
    expect(screen.getByText("+11")).toBeInTheDocument();
  });

  it("shows streak values", () => {
    renderWithProviders(<Standings entries={MOCK_STANDINGS} />);
    expect(screen.getByText("W3")).toBeInTheDocument();
    expect(screen.getByText("L1")).toBeInTheDocument();
  });

  it("renders column headers", () => {
    renderWithProviders(<Standings entries={MOCK_STANDINGS} />);
    expect(screen.getByText("Team")).toBeInTheDocument();
    expect(screen.getByText("W-L")).toBeInTheDocument();
    expect(screen.getByText("Streak")).toBeInTheDocument();
  });

  it("renders empty state when no entries", () => {
    renderWithProviders(<Standings entries={[]} />);
    expect(screen.getByText(/no standings/i)).toBeInTheDocument();
  });
});
