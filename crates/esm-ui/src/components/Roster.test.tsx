import { screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { renderWithProviders } from "@/test/render";
import { Roster } from "./Roster";
import type { RosterPlayer } from "./Roster";

const MOCK_PLAYERS: RosterPlayer[] = [
  {
    nickname: "Faker",
    firstName: "Sang-hyeok",
    lastName: "Lee",
    role: "Mid",
    stamina: 82,
    morale: 90,
    mechanics: 97,
    vision: 88,
    teamfighting: 92,
  },
  {
    nickname: "Zeus",
    firstName: "Woo-je",
    lastName: "Choi",
    role: "Top",
    stamina: 75,
    morale: 85,
    mechanics: 88,
    vision: 80,
    teamfighting: 85,
  },
  {
    nickname: "Oner",
    firstName: "Hyeon-jun",
    lastName: "Moon",
    role: "Jungle",
    stamina: 90,
    morale: 78,
    mechanics: 85,
    vision: 91,
    teamfighting: 88,
  },
];

describe("Roster", () => {
  it("renders the Roster heading", () => {
    renderWithProviders(<Roster players={MOCK_PLAYERS} />);
    expect(
      screen.getByRole("heading", { name: /roster/i })
    ).toBeInTheDocument();
  });

  it("renders a row for each player", () => {
    renderWithProviders(<Roster players={MOCK_PLAYERS} />);
    expect(screen.getByText("Faker")).toBeInTheDocument();
    expect(screen.getByText("Zeus")).toBeInTheDocument();
    expect(screen.getByText("Oner")).toBeInTheDocument();
  });

  it("shows player role badges", () => {
    renderWithProviders(<Roster players={MOCK_PLAYERS} />);
    expect(screen.getByText("Mid")).toBeInTheDocument();
    expect(screen.getByText("Top")).toBeInTheDocument();
    expect(screen.getByText("Jungle")).toBeInTheDocument();
  });

  it("shows player full name", () => {
    renderWithProviders(<Roster players={MOCK_PLAYERS} />);
    expect(screen.getByText("Sang-hyeok Lee")).toBeInTheDocument();
  });

  it("shows attribute values", () => {
    renderWithProviders(<Roster players={MOCK_PLAYERS} />);
    expect(screen.getByText("97")).toBeInTheDocument(); // Faker mechanics
  });

  it("shows stamina and morale bars or values", () => {
    renderWithProviders(<Roster players={MOCK_PLAYERS} />);
    // Faker's stamina and morale should be visible
    expect(screen.getAllByText("82").length).toBeGreaterThanOrEqual(1); // stamina
    expect(screen.getAllByText("90").length).toBeGreaterThanOrEqual(1); // morale
  });

  it("renders empty state when no players", () => {
    renderWithProviders(<Roster players={[]} />);
    expect(screen.getByText(/no players/i)).toBeInTheDocument();
  });
});
