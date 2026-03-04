import { screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { renderWithProviders } from "@/test/render";
import { Dashboard } from "./Dashboard";

describe("Dashboard", () => {
  const defaults = {
    teamName: "T1",
    record: "8-2",
    standing: "2nd place · LCK Spring",
    winStreak: 3,
    budget: "$1.2M",
    inboxCount: 3,
    urgentCount: 1,
  };

  it("renders the KPI stat cards", () => {
    renderWithProviders(<Dashboard {...defaults} />);
    expect(screen.getByText("Record")).toBeInTheDocument();
    expect(screen.getByText("8-2")).toBeInTheDocument();
  });

  it("shows win streak value", () => {
    renderWithProviders(<Dashboard {...defaults} winStreak={5} inboxCount={1} />);
    expect(screen.getByText("Win Streak")).toBeInTheDocument();
    expect(screen.getByText("5")).toBeInTheDocument();
  });

  it("shows budget value", () => {
    renderWithProviders(<Dashboard {...defaults} />);
    expect(screen.getByText("Budget")).toBeInTheDocument();
    expect(screen.getByText("$1.2M")).toBeInTheDocument();
  });

  it("shows inbox count with urgent badge", () => {
    renderWithProviders(<Dashboard {...defaults} />);
    expect(screen.getByText("Inbox")).toBeInTheDocument();
    expect(screen.getByText("1 urgent")).toBeInTheDocument();
  });

  it("shows the roster overview section", () => {
    renderWithProviders(<Dashboard {...defaults} />);
    expect(screen.getByText("Roster Overview")).toBeInTheDocument();
  });

  it("shows the upcoming match section", () => {
    renderWithProviders(<Dashboard {...defaults} />);
    expect(screen.getByText("Next Match")).toBeInTheDocument();
  });

  it("renders with default props when none provided", () => {
    renderWithProviders(<Dashboard />);
    expect(screen.getByText("Record")).toBeInTheDocument();
  });
});
