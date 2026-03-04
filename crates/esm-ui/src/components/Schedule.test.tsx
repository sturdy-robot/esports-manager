import { screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { renderWithProviders } from "@/test/render";
import { Schedule } from "./Schedule";
import type { ScheduleMatch } from "./Schedule";

const MOCK_MATCHES: ScheduleMatch[] = [
  {
    id: "1",
    homeTeam: "T1",
    awayTeam: "Gen.G",
    day: 5,
    month: 1,
    year: 2025,
    bestOf: 3,
    result: null,
  },
  {
    id: "2",
    homeTeam: "T1",
    awayTeam: "DRX",
    day: 3,
    month: 1,
    year: 2025,
    bestOf: 3,
    result: { homeWins: 2, awayWins: 1 },
  },
  {
    id: "3",
    homeTeam: "KT Rolster",
    awayTeam: "T1",
    day: 8,
    month: 1,
    year: 2025,
    bestOf: 3,
    result: null,
  },
];

describe("Schedule", () => {
  it("renders the Schedule heading", () => {
    renderWithProviders(<Schedule matches={MOCK_MATCHES} />);
    expect(
      screen.getByRole("heading", { name: /schedule/i })
    ).toBeInTheDocument();
  });

  it("renders each match", () => {
    renderWithProviders(<Schedule matches={MOCK_MATCHES} />);
    expect(screen.getAllByText("T1").length).toBeGreaterThanOrEqual(3);
    expect(screen.getByText("Gen.G")).toBeInTheDocument();
    expect(screen.getByText("DRX")).toBeInTheDocument();
    expect(screen.getByText("KT Rolster")).toBeInTheDocument();
  });

  it("shows match format", () => {
    renderWithProviders(<Schedule matches={MOCK_MATCHES} />);
    expect(screen.getAllByText("Bo3").length).toBe(3);
  });

  it("shows result for completed matches", () => {
    renderWithProviders(<Schedule matches={MOCK_MATCHES} />);
    expect(screen.getByText("2 - 1")).toBeInTheDocument();
  });

  it("shows upcoming indicator for future matches", () => {
    renderWithProviders(<Schedule matches={MOCK_MATCHES} />);
    expect(screen.getAllByText("VS").length).toBe(2);
  });

  it("renders empty state when no matches", () => {
    renderWithProviders(<Schedule matches={[]} />);
    expect(screen.getByText(/no matches/i)).toBeInTheDocument();
  });
});
