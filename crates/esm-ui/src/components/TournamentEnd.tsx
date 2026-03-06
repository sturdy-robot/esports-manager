import { Trophy, ChevronRight, Medal } from "lucide-react";
import type { StandingInfo } from "@/lib/api";

interface TournamentEndProps {
    seasonName: string;
    standings: StandingInfo[];
    teamName: string;
    onFinish: () => void;
}

export function TournamentEnd({ seasonName, standings, teamName, onFinish }: TournamentEndProps) {
    const teamRank = standings.find((s) => s.team_name === teamName)?.rank || 0;

    const getRankColor = (rank: number) => {
        switch (rank) {
            case 1:
                return "#fbbf24"; // Gold
            case 2:
                return "#94a3b8"; // Silver
            case 3:
                return "#b45309"; // Bronze
            default:
                return "var(--text-muted)";
        }
    };

    return (
        <div className="flex flex-col items-center justify-center min-h-[500px] animate-fade-in-up">
            <div
                className="w-full max-w-2xl bg-surface border border-subtle rounded-xl p-8 flex flex-col items-center text-center shadow-lg relative overflow-hidden"
                style={{ backgroundColor: "var(--bg-surface)", borderColor: "var(--border-subtle)" }}
            >
                {/* Decorative background glow */}
                <div
                    className="absolute top-0 left-1/2 -translate-x-1/2 w-96 h-96 rounded-full blur-[100px] opacity-20 pointer-events-none"
                    style={{ backgroundColor: "var(--color-accent-cyan)" }}
                />

                <Trophy size={64} className="mb-4" style={{ color: "#fbbf24" }} />

                <h2 className="text-3xl font-bold mb-2" style={{ color: "var(--text-primary)" }}>
                    {seasonName} Concluded
                </h2>

                <p className="text-lg mb-8" style={{ color: "var(--text-secondary)" }}>
                    The season is over. {teamName} finished in <strong style={{ color: "var(--color-accent-blue)" }}>{teamRank}{teamRank === 1 ? "st" : teamRank === 2 ? "nd" : teamRank === 3 ? "rd" : "th"} place</strong>.
                </p>

                <div className="w-full mb-8 relative z-10">
                    <h3 className="text-left text-sm font-semibold mb-3 uppercase tracking-wider" style={{ color: "var(--text-muted)" }}>
                        Final Standings (Top 4)
                    </h3>
                    <div className="flex flex-col gap-2">
                        {standings.slice(0, 4).map((s) => (
                            <div
                                key={s.team_name}
                                className="flex items-center justify-between p-3 rounded-lg border"
                                style={{
                                    backgroundColor: s.team_name === teamName ? "rgba(96, 165, 250, 0.1)" : "var(--bg-elevated)",
                                    borderColor: s.team_name === teamName ? "var(--color-accent-cyan)" : "transparent",
                                }}
                            >
                                <div className="flex items-center gap-3">
                                    <div className="flex items-center justify-center w-8 h-8 rounded-full bg-base shadow-inner">
                                        <Medal size={16} style={{ color: getRankColor(s.rank) }} />
                                    </div>
                                    <span className="font-semibold" style={{ color: "var(--text-primary)" }}>
                                        {s.rank}. {s.team_name}
                                    </span>
                                </div>
                                <div className="font-mono text-sm" style={{ color: "var(--text-secondary)" }}>
                                    {s.wins}W - {s.losses}L
                                </div>
                            </div>
                        ))}
                    </div>
                </div>

                <button
                    onClick={onFinish}
                    className="flex items-center gap-2 px-6 py-3 rounded-lg font-bold transition-all relative z-10 hover:opacity-90 active:scale-95"
                    style={{ backgroundColor: "var(--color-accent-blue)", color: "var(--bg-base)" }}
                >
                    Wrap up Season
                    <ChevronRight size={18} />
                </button>
            </div>
        </div>
    );
}
