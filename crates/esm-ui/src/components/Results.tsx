import { FileText } from "lucide-react";

export interface MatchResult {
  id: string;
  homeTeam: string;
  awayTeam: string;
  homeWins: number;
  awayWins: number;
  day: number;
  month: number;
  year: number;
  bestOf: number;
  playerTeamWon: boolean;
}

interface ResultsProps {
  results: MatchResult[];
}

const MONTH_NAMES = [
  "Jan", "Feb", "Mar", "Apr", "May", "Jun",
  "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

export function Results({ results }: ResultsProps) {
  return (
    <div className="flex flex-col gap-4 animate-fade-in-up">
      <h2
        className="text-lg font-bold"
        style={{ color: "var(--text-primary)" }}
      >
        Results
      </h2>

      {results.length === 0 && (
        <div className="flex flex-col items-center justify-center gap-3 py-16">
          <FileText size={48} style={{ color: "var(--text-muted)" }} />
          <p style={{ color: "var(--text-muted)" }}>No results yet</p>
        </div>
      )}

      {results.length > 0 && (
        <div className="flex flex-col gap-3">
          {results.map((r) => {
            const monthLabel = MONTH_NAMES[(r.month - 1) % 12];

            return (
              <div
                key={r.id}
                className="flex items-center p-4 rounded-lg border glow-hover transition-all"
                style={{
                  backgroundColor: "var(--bg-surface)",
                  borderColor: "var(--border-subtle)",
                  borderLeftWidth: "3px",
                  borderLeftColor: r.playerTeamWon
                    ? "var(--color-win)"
                    : "var(--color-loss)",
                }}
              >
                {/* Date */}
                <div
                  className="flex flex-col items-center w-16 shrink-0"
                  style={{ color: "var(--text-muted)" }}
                >
                  <span className="text-xs font-semibold uppercase">
                    {monthLabel}
                  </span>
                  <span
                    className="text-lg font-bold font-mono"
                    style={{ color: "var(--text-primary)" }}
                  >
                    {r.day}
                  </span>
                </div>

                {/* Teams + score */}
                <div className="flex-1 flex items-center justify-center gap-4">
                  <span
                    className="text-sm font-semibold text-right min-w-[100px]"
                    style={{ color: "var(--text-primary)" }}
                  >
                    {r.homeTeam}
                  </span>

                  <span
                    className="text-sm font-bold font-mono px-3 py-1 rounded"
                    style={{
                      backgroundColor: "var(--bg-elevated)",
                      color: "var(--text-primary)",
                    }}
                  >
                    {r.homeWins} - {r.awayWins}
                  </span>

                  <span
                    className="text-sm font-semibold text-left min-w-[100px]"
                    style={{ color: "var(--text-primary)" }}
                  >
                    {r.awayTeam}
                  </span>
                </div>

                {/* Win/Loss badge + format */}
                <div className="flex items-center gap-3 shrink-0">
                  <span
                    className="px-2 py-0.5 rounded text-xs font-bold"
                    style={{
                      backgroundColor: r.playerTeamWon
                        ? "rgba(34, 197, 94, 0.15)"
                        : "rgba(239, 68, 68, 0.15)",
                      color: r.playerTeamWon
                        ? "var(--color-win)"
                        : "var(--color-loss)",
                    }}
                  >
                    {r.playerTeamWon ? "WIN" : "LOSS"}
                  </span>
                  <span
                    className="px-2 py-0.5 rounded text-xs font-mono"
                    style={{
                      backgroundColor: "var(--bg-elevated)",
                      color: "var(--text-secondary)",
                    }}
                  >
                    Bo{r.bestOf}
                  </span>
                </div>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}
