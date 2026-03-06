import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { TournamentEnd } from "./TournamentEnd";

describe("TournamentEnd", () => {
    const mockStandings = [
        { rank: 1, team_name: "T1", wins: 10, losses: 0, win_pct: 100 },
        { rank: 2, team_name: "Gen.G", wins: 8, losses: 2, win_pct: 80 },
        { rank: 3, team_name: "DRX", wins: 6, losses: 4, win_pct: 60 },
        { rank: 4, team_name: "KT Rolster", wins: 4, losses: 6, win_pct: 40 },
    ];

    it("renders the season name and team position", () => {
        render(
            <TournamentEnd
                seasonName="2025 LCK Spring"
                standings={mockStandings}
                teamName="T1"
                onFinish={vi.fn()}
            />
        );
        expect(screen.getByText("2025 LCK Spring Concluded")).toBeInTheDocument();
        expect(screen.getByText(/T1 finished in/)).toBeInTheDocument();
        expect(screen.getByText(/1st place/)).toBeInTheDocument();
    });

    it("calls onFinish when Wrap up Season is clicked", () => {
        const onFinish = vi.fn();
        render(
            <TournamentEnd
                seasonName="2025 LCK Spring"
                standings={mockStandings}
                teamName="T1"
                onFinish={onFinish}
            />
        );
        fireEvent.click(screen.getByText("Wrap up Season"));
        expect(onFinish).toHaveBeenCalledOnce();
    });
});
