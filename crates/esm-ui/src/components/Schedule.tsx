import { Calendar } from "lucide-react";

export interface ScheduleMatch {
  id: string;
  homeTeam: string;
  awayTeam: string;
  day: number;
  month: number;
  year: number;
  bestOf: number;
  result: { homeWins: number; awayWins: number } | null;
}

interface ScheduleProps {
  matches: ScheduleMatch[];
}

const MONTH_NAMES = [
  "Jan", "Feb", "Mar", "Apr", "May", "Jun",
  "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

export function Schedule({ matches }: ScheduleProps) {
  return (
    <div className="flex flex-col gap-4 animate-fade-in-up">
      <h2
        className="text-lg font-bold"
        style={{ color: "var(--text-primary)" }}
      >
        Schedule
      </h2>

      {matches.length === 0 && (
        <div className="flex flex-col items-center justify-center gap-3 py-16">
          <Calendar size={48} style={{ color: "var(--text-muted)" }} />
          <p style={{ color: "var(--text-muted)" }}>No matches scheduled</p>
        </div>
      )}

      {matches.length > 0 && (
        <div className="flex flex-col gap-3">
          {matches.map((match) => {
            const completed = match.result !== null;
            const monthLabel = MONTH_NAMES[(match.month - 1) % 12];

            return (
              <div
                key={match.id}
                className="flex items-center p-4 rounded-lg border glow-hover transition-all"
                style={{
                  backgroundColor: "var(--bg-surface)",
                  borderColor: completed
                    ? "var(--border-subtle)"
                    : "var(--color-accent-cyan)",
                  borderLeftWidth: completed ? "1px" : "3px",
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
                    className="text-lg font-bold tabular-nums"
                    style={{ color: "var(--text-primary)" }}
                  >
                    {match.day}
                  </span>
                </div>

                {/* Teams + result */}
                <div className="flex-1 flex items-center justify-center gap-4">
                  <span
                    className="text-sm font-semibold text-right min-w-[100px]"
                    style={{ color: "var(--text-primary)" }}
                  >
                    {match.homeTeam}
                  </span>

                  {completed && match.result ? (
                    <span
                      className="text-sm font-bold tabular-nums px-3 py-1 rounded"
                      style={{
                        backgroundColor: "var(--bg-elevated)",
                        color: "var(--text-primary)",
                      }}
                    >
                      {match.result.homeWins} - {match.result.awayWins}
                    </span>
                  ) : (
                    <span
                      className="text-sm font-bold px-3 py-1 rounded"
                      style={{
                        background:
                          "linear-gradient(135deg, var(--color-accent-emerald), var(--color-accent-cyan))",
                        color: "#fff",
                      }}
                    >
                      VS
                    </span>
                  )}

                  <span
                    className="text-sm font-semibold text-left min-w-[100px]"
                    style={{ color: "var(--text-primary)" }}
                  >
                    {match.awayTeam}
                  </span>
                </div>

                {/* Format badge */}
                <div className="w-16 shrink-0 flex justify-end">
                  <span
                    className="px-2 py-0.5 rounded text-xs tabular-nums"
                    style={{
                      backgroundColor: "var(--bg-elevated)",
                      color: "var(--text-secondary)",
                    }}
                  >
                    Bo{match.bestOf}
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
