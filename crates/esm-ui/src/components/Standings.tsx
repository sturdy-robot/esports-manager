import { Trophy } from "lucide-react";

export interface StandingsEntry {
  rank: number;
  teamName: string;
  wins: number;
  losses: number;
  mapWins: number;
  mapLosses: number;
  streak: string;
}

interface StandingsProps {
  entries: StandingsEntry[];
}

export function Standings({ entries }: StandingsProps) {
  return (
    <div className="flex flex-col gap-4 animate-fade-in-up">
      <div className="flex flex-col gap-1.5">
        <span className="app-eyebrow">League Table</span>
        <h2 className="app-page-title">Standings</h2>
        <p className="app-page-subtitle">
          Follow placement, map differential, and momentum across the season.
        </p>
      </div>

      {entries.length === 0 && (
        <div className="app-panel rounded-2xl flex flex-col items-center justify-center gap-3 py-16">
          <Trophy size={48} style={{ color: "var(--text-muted)" }} />
          <p style={{ color: "var(--text-muted)" }}>No standings available</p>
        </div>
      )}

      {entries.length > 0 && (
        <div
          className="app-data-grid rounded-2xl overflow-hidden"
          style={{
            backgroundColor: "var(--bg-surface)",
          }}
        >
          <table className="w-full">
            <thead>
              <tr
                style={{
                  borderBottom: "1px solid var(--border-subtle)",
                }}
              >
                <th
                  className="text-center text-xs font-semibold uppercase tracking-wider py-2.5 px-3 w-12"
                  style={{ color: "var(--text-muted)" }}
                >
                  #
                </th>
                <th
                  className="text-left text-xs font-semibold uppercase tracking-wider py-2.5 px-3"
                  style={{ color: "var(--text-muted)" }}
                >
                  Team
                </th>
                <th
                  className="text-center text-xs font-semibold uppercase tracking-wider py-2.5 px-3"
                  style={{ color: "var(--text-muted)" }}
                >
                  W-L
                </th>
                <th
                  className="text-center text-xs font-semibold uppercase tracking-wider py-2.5 px-3"
                  style={{ color: "var(--text-muted)" }}
                >
                  Map Diff
                </th>
                <th
                  className="text-center text-xs font-semibold uppercase tracking-wider py-2.5 px-3"
                  style={{ color: "var(--text-muted)" }}
                >
                  Streak
                </th>
              </tr>
            </thead>
            <tbody>
              {entries.map((entry) => {
                const mapDiff = entry.mapWins - entry.mapLosses;
                const diffStr = mapDiff > 0 ? `+${mapDiff}` : String(mapDiff);
                const diffColor =
                  mapDiff > 0
                    ? "var(--color-win)"
                    : mapDiff < 0
                      ? "var(--color-loss)"
                      : "var(--text-secondary)";
                const streakColor = entry.streak.startsWith("W")
                  ? "var(--color-win)"
                  : "var(--color-loss)";

                return (
                  <tr
                    key={entry.rank}
                    className="transition-colors"
                    style={{
                      borderBottom: "1px solid var(--border-subtle)",
                    }}
                    onMouseEnter={(e) => {
                      e.currentTarget.style.backgroundColor =
                        "var(--bg-elevated)";
                    }}
                    onMouseLeave={(e) => {
                      e.currentTarget.style.backgroundColor = "transparent";
                    }}
                  >
                    <td
                      className="text-center py-3 px-3 text-sm font-bold tabular-nums"
                      style={{
                        color:
                          entry.rank <= 3
                            ? "var(--color-accent-cyan)"
                            : "var(--text-muted)",
                      }}
                    >
                      {entry.rank}
                    </td>
                    <td
                      className="py-3 px-3 text-sm font-semibold"
                      style={{ color: "var(--text-primary)" }}
                    >
                      {entry.teamName}
                    </td>
                    <td
                      className="text-center py-3 px-3 text-sm tabular-nums"
                      style={{ color: "var(--text-primary)" }}
                    >
                      {entry.wins}-{entry.losses}
                    </td>
                    <td
                      className="text-center py-3 px-3 text-sm tabular-nums"
                      style={{ color: diffColor }}
                    >
                      {diffStr}
                    </td>
                    <td
                      className="text-center py-3 px-3 text-sm font-semibold"
                      style={{ color: streakColor }}
                    >
                      {entry.streak}
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}
