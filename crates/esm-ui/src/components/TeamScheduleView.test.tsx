import { screen } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import { renderWithProviders } from "@/test/render";
import { TeamScheduleView } from "./TeamScheduleView";
import type { WeekScheduleInfo, ScrimInfo, ScheduleSlotInfo } from "@/lib/api";

const freeSlot = (ts: string): ScheduleSlotInfo => ({
  time_slot: ts,
  entry_type: "free",
  scrim_id: null,
  opponent: null,
  players: null,
  focus: null,
});

const MOCK_SCHEDULE: WeekScheduleInfo = {
  days: [
    {
      day_index: 0,
      slots: [
        { time_slot: "Morning", entry_type: "scrim", scrim_id: 1, opponent: "Gen.G", players: null, focus: null },
        { time_slot: "Afternoon", entry_type: "solo_queue", scrim_id: null, opponent: null, players: [0, 2], focus: "mechanics" },
        freeSlot("Evening"),
      ],
    },
    ...Array.from({ length: 6 }, (_, i) => ({
      day_index: i + 1,
      slots: [freeSlot("Morning"), freeSlot("Afternoon"), freeSlot("Evening")],
    })),
  ],
  total_scrims: 1,
  occupied_slots: 2,
};

const MOCK_SCRIMS: ScrimInfo[] = [
  {
    id: 1,
    home_team: "T1",
    away_team: "Gen.G",
    scheduled_day: 5,
    time_slot: "Morning",
    game_count: 3,
    draft_rules: "Standard",
    status: "Scheduled",
    home_wins: 0,
    away_wins: 0,
  },
];

const defaultProps = {
  schedule: MOCK_SCHEDULE,
  scrims: MOCK_SCRIMS,
  rosterNames: ["Zeus", "Oner", "Faker", "Gumayusi", "Keria"],
  teamNames: ["T1", "Gen.G", "DRX"],
  onScheduleScrim: vi.fn().mockResolvedValue(undefined),
  onScheduleSoloQueue: vi.fn().mockResolvedValue(undefined),
  onScheduleRest: vi.fn().mockResolvedValue(undefined),
  onClearSlot: vi.fn().mockResolvedValue(undefined),
  onCancelScrim: vi.fn().mockResolvedValue(undefined),
};

describe("TeamScheduleView", () => {
  it("renders the Weekly Schedule heading", () => {
    renderWithProviders(<TeamScheduleView {...defaultProps} />);
    expect(
      screen.getByRole("heading", { name: /weekly schedule/i })
    ).toBeInTheDocument();
  });

  it("renders day column headers", () => {
    renderWithProviders(<TeamScheduleView {...defaultProps} />);
    expect(screen.getByText("Mon")).toBeInTheDocument();
    expect(screen.getByText("Tue")).toBeInTheDocument();
    expect(screen.getByText("Sun")).toBeInTheDocument();
  });

  it("renders time slot row labels", () => {
    renderWithProviders(<TeamScheduleView {...defaultProps} />);
    expect(screen.getByText("Morning")).toBeInTheDocument();
    expect(screen.getByText("Afternoon")).toBeInTheDocument();
    expect(screen.getByText("Evening")).toBeInTheDocument();
  });

  it("shows scrim slot with opponent name", () => {
    renderWithProviders(<TeamScheduleView {...defaultProps} />);
    // Appears in both the calendar grid cell and scrims list
    const matches = screen.getAllByText(/vs Gen\.G/);
    expect(matches.length).toBeGreaterThanOrEqual(1);
  });

  it("shows solo queue slot with focus", () => {
    renderWithProviders(<TeamScheduleView {...defaultProps} />);
    expect(screen.getByText(/SoloQ · mechanics/)).toBeInTheDocument();
  });

  it("shows scrim count and occupied slots stats", () => {
    renderWithProviders(<TeamScheduleView {...defaultProps} />);
    expect(screen.getByText("1")).toBeInTheDocument();
    expect(screen.getByText("scrims")).toBeInTheDocument();
    expect(screen.getByText("2")).toBeInTheDocument();
    expect(screen.getByText("/ 21 slots")).toBeInTheDocument();
  });

  it("renders scrims list with status", () => {
    renderWithProviders(<TeamScheduleView {...defaultProps} />);
    // Scrims section header
    expect(screen.getByRole("heading", { name: /scrims/i })).toBeInTheDocument();
    // Status badge
    expect(screen.getByText("Scheduled")).toBeInTheDocument();
  });

  it("shows loading state when schedule is null", () => {
    renderWithProviders(<TeamScheduleView {...defaultProps} schedule={null} />);
    expect(screen.getByText(/loading schedule/i)).toBeInTheDocument();
  });

  it("renders legend items", () => {
    renderWithProviders(<TeamScheduleView {...defaultProps} />);
    expect(screen.getByText("Scrim")).toBeInTheDocument();
    expect(screen.getByText("Solo Queue")).toBeInTheDocument();
    expect(screen.getByText("Rest")).toBeInTheDocument();
  });
});
