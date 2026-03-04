import { screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { renderWithProviders } from "@/test/render";
import { Staff } from "./Staff";
import type { StaffMember } from "./Staff";

const MOCK_STAFF: StaffMember[] = [
  { id: "s1", name: "Park Ji-sung", role: "Head Coach", skill: 88 },
  { id: "s2", name: "Kim Dae-ho", role: "Assistant Coach", skill: 75 },
  { id: "s3", name: "Lee Min-ji", role: "Positional Coach", skill: 70 },
  { id: "s4", name: "Choi Yeon-su", role: "Psychologist", skill: 82 },
];

describe("Staff", () => {
  it("renders the Staff heading", () => {
    renderWithProviders(<Staff members={MOCK_STAFF} />);
    expect(
      screen.getByRole("heading", { name: /staff/i })
    ).toBeInTheDocument();
  });

  it("renders a card for each staff member", () => {
    renderWithProviders(<Staff members={MOCK_STAFF} />);
    expect(screen.getByText("Park Ji-sung")).toBeInTheDocument();
    expect(screen.getByText("Kim Dae-ho")).toBeInTheDocument();
    expect(screen.getByText("Lee Min-ji")).toBeInTheDocument();
    expect(screen.getByText("Choi Yeon-su")).toBeInTheDocument();
  });

  it("shows role labels", () => {
    renderWithProviders(<Staff members={MOCK_STAFF} />);
    expect(screen.getByText("Head Coach")).toBeInTheDocument();
    expect(screen.getByText("Assistant Coach")).toBeInTheDocument();
    expect(screen.getByText("Positional Coach")).toBeInTheDocument();
    expect(screen.getByText("Psychologist")).toBeInTheDocument();
  });

  it("shows skill values", () => {
    renderWithProviders(<Staff members={MOCK_STAFF} />);
    expect(screen.getByText("88")).toBeInTheDocument();
    expect(screen.getByText("75")).toBeInTheDocument();
  });

  it("renders empty state when no staff", () => {
    renderWithProviders(<Staff members={[]} />);
    expect(screen.getByText(/no staff/i)).toBeInTheDocument();
  });
});
