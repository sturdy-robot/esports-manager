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
  "Jan",
  "Feb",
  "Mar",
  "Apr",
  "May",
  "Jun",
  "Jul",
  "Aug",
  "Sep",
  "Oct",
  "Nov",
  "Dec",
];

export function Schedule({ matches }: ScheduleProps) {
  return (
    <div className="flex flex-col gap-6 animate-fade-in-up">
      <div className="flex flex-col gap-1.5">
        <span className="app-eyebrow">League Timeline</span>
        <h2 className="app-page-title">Schedule</h2>
        <p className="app-page-subtitle">
          Track completed and upcoming series across the season calendar with a
          clear view of what is already locked in.
        </p>
      </div>

      {matches.length === 0 && (
        <div className="app-panel rounded-2xl flex flex-col items-center justify-center gap-3 py-16">
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
                className="app-panel flex items-center p-4 rounded-2xl glow-hover transition-all"
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
                    className="text-sm font-semibold text-right min-w-25"
                    style={{ color: "var(--text-primary)" }}
                  >
                    {match.homeTeam}
                  </span>

                  {completed && match.result ? (
                    <span
                      className="text-sm font-bold tabular-nums px-3 py-1 rounded-xl"
                      style={{
                        backgroundColor: "var(--bg-elevated)",
                        color: "var(--text-primary)",
                      }}
                    >
                      {match.result.homeWins} - {match.result.awayWins}
                    </span>
                  ) : (
                    <span
                      className="text-sm font-bold px-3 py-1 rounded-xl"
                      style={{
                        background: "var(--accent-gradient)",
                        color: "#fff",
                      }}
                    >
                      VS
                    </span>
                  )}

                  <span
                    className="text-sm font-semibold text-left min-w-25"
                    style={{ color: "var(--text-primary)" }}
                  >
                    {match.awayTeam}
                  </span>
                </div>

                {/* Format badge */}
                <div className="w-16 shrink-0 flex justify-end">
                  <span
                    className="px-2 py-0.5 rounded-full text-xs tabular-nums"
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
