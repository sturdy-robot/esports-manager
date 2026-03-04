import { screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { renderWithProviders } from "@/test/render";
import { Scouting } from "./Scouting";
import type { ScoutingTarget } from "./Scouting";

const MOCK_TARGETS: ScoutingTarget[] = [
  {
    id: "p1",
    nickname: "Chovy",
    role: "Mid",
    team: "Gen.G",
    mechanics: 95,
    vision: 85,
    teamfighting: 88,
    estimatedValue: 800000,
    scoutingAccuracy: 72,
  },
  {
    id: "p2",
    nickname: "Peyz",
    role: "Bot",
    team: "Gen.G",
    mechanics: 82,
    vision: 74,
    teamfighting: 80,
    estimatedValue: 350000,
    scoutingAccuracy: 55,
  },
  {
    id: "p3",
    nickname: "Doran",
    role: "Top",
    team: "Hanwha Life",
    mechanics: 78,
    vision: 80,
    teamfighting: 82,
    estimatedValue: 300000,
    scoutingAccuracy: 90,
  },
];

describe("Scouting", () => {
  it("renders the Scouting heading", () => {
    renderWithProviders(<Scouting targets={MOCK_TARGETS} />);
    expect(
      screen.getByRole("heading", { name: /scouting/i })
    ).toBeInTheDocument();
  });

  it("renders a row for each target", () => {
    renderWithProviders(<Scouting targets={MOCK_TARGETS} />);
    expect(screen.getByText("Chovy")).toBeInTheDocument();
    expect(screen.getByText("Peyz")).toBeInTheDocument();
    expect(screen.getByText("Doran")).toBeInTheDocument();
  });

  it("shows team names", () => {
    renderWithProviders(<Scouting targets={MOCK_TARGETS} />);
    expect(screen.getAllByText("Gen.G").length).toBe(2);
    expect(screen.getByText("Hanwha Life")).toBeInTheDocument();
  });

  it("shows estimated value", () => {
    renderWithProviders(<Scouting targets={MOCK_TARGETS} />);
    expect(screen.getByText("$800,000")).toBeInTheDocument();
    expect(screen.getByText("$350,000")).toBeInTheDocument();
  });

  it("shows scouting accuracy indicator", () => {
    renderWithProviders(<Scouting targets={MOCK_TARGETS} />);
    expect(screen.getByText("72%")).toBeInTheDocument();
    expect(screen.getByText("90%")).toBeInTheDocument();
  });

  it("renders empty state when no targets", () => {
    renderWithProviders(<Scouting targets={[]} />);
    expect(screen.getByText(/no scouting/i)).toBeInTheDocument();
  });
});
