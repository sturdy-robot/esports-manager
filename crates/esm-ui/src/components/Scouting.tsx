import { Search } from "lucide-react";

export interface ScoutingTarget {
  id: string;
  nickname: string;
  role: string;
  team: string;
  mechanics: number;
  vision: number;
  teamfighting: number;
  estimatedValue: number;
  scoutingAccuracy: number;
}

interface ScoutingProps {
  targets: ScoutingTarget[];
}

const ROLE_COLORS: Record<string, string> = {
  Top: "#E879F9",
  Jungle: "#34D399",
  Mid: "#60A5FA",
  Bot: "#F97316",
  Support: "#FBBF24",
};

function formatCurrency(value: number): string {
  return `$${value.toLocaleString("en-US")}`;
}

function AttributeCell({ value }: { value: number }) {
  const color =
    value >= 90
      ? "var(--color-win)"
      : value >= 70
        ? "var(--color-accent-cyan)"
        : value >= 50
          ? "var(--color-warning)"
          : "var(--color-loss)";

  return (
    <td
      className="py-3 px-3 text-sm tabular-nums text-center"
      style={{ color }}
    >
      {value}
    </td>
  );
}

export function Scouting({ targets }: ScoutingProps) {
  return (
    <div className="flex flex-col gap-6 animate-fade-in-up">
      <div className="flex flex-col gap-1.5">
        <span className="app-eyebrow">Talent Pipeline</span>
        <h2 className="app-page-title">Scouting</h2>
        <p className="app-page-subtitle">
          Survey external talent, compare projected transfer value, and assess
          how reliable each report really is.
        </p>
      </div>

      {targets.length === 0 && (
        <div className="app-panel rounded-2xl flex flex-col items-center justify-center gap-3 py-16">
          <Search size={48} style={{ color: "var(--text-muted)" }} />
          <p style={{ color: "var(--text-muted)" }}>No scouting targets</p>
        </div>
      )}

      {targets.length > 0 && (
        <div
          className="app-panel rounded-2xl overflow-hidden"
          style={{
            backgroundColor: "var(--bg-surface)",
          }}
        >
          <table className="w-full">
            <thead>
              <tr
                style={{
                  borderBottom: "1px solid var(--border-subtle)",
                  backgroundColor: "rgba(255, 255, 255, 0.03)",
                }}
              >
                <th
                  className="text-left text-xs font-semibold uppercase tracking-wider py-2.5 px-3"
                  style={{ color: "var(--text-muted)" }}
                >
                  Player
                </th>
                <th
                  className="text-left text-xs font-semibold uppercase tracking-wider py-2.5 px-3"
                  style={{ color: "var(--text-muted)" }}
                >
                  Role
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
                  MEC
                </th>
                <th
                  className="text-center text-xs font-semibold uppercase tracking-wider py-2.5 px-3"
                  style={{ color: "var(--text-muted)" }}
                >
                  VIS
                </th>
                <th
                  className="text-center text-xs font-semibold uppercase tracking-wider py-2.5 px-3"
                  style={{ color: "var(--text-muted)" }}
                >
                  TF
                </th>
                <th
                  className="text-right text-xs font-semibold uppercase tracking-wider py-2.5 px-3"
                  style={{ color: "var(--text-muted)" }}
                >
                  Est. Value
                </th>
                <th
                  className="text-center text-xs font-semibold uppercase tracking-wider py-2.5 px-3"
                  style={{ color: "var(--text-muted)" }}
                >
                  Accuracy
                </th>
              </tr>
            </thead>
            <tbody>
              {targets.map((t) => {
                const accColor =
                  t.scoutingAccuracy >= 80
                    ? "var(--color-win)"
                    : t.scoutingAccuracy >= 60
                      ? "var(--color-warning)"
                      : "var(--color-loss)";

                return (
                  <tr
                    key={t.id}
                    className="transition-colors"
                    style={{ borderBottom: "1px solid var(--border-subtle)" }}
                    onMouseEnter={(e) => {
                      e.currentTarget.style.backgroundColor =
                        "var(--bg-elevated)";
                    }}
                    onMouseLeave={(e) => {
                      e.currentTarget.style.backgroundColor = "transparent";
                    }}
                  >
                    <td className="py-3 px-3">
                      <span
                        className="text-sm font-semibold"
                        style={{ color: "var(--text-primary)" }}
                      >
                        {t.nickname}
                      </span>
                    </td>
                    <td className="py-3 px-3">
                      <span
                        className="px-2 py-0.5 rounded-full text-xs font-semibold"
                        style={{
                          backgroundColor: `${ROLE_COLORS[t.role] || "var(--color-accent-cyan)"}20`,
                          color:
                            ROLE_COLORS[t.role] || "var(--color-accent-cyan)",
                        }}
                      >
                        {t.role}
                      </span>
                    </td>
                    <td
                      className="py-3 px-3 text-sm"
                      style={{ color: "var(--text-secondary)" }}
                    >
                      {t.team}
                    </td>
                    <AttributeCell value={t.mechanics} />
                    <AttributeCell value={t.vision} />
                    <AttributeCell value={t.teamfighting} />
                    <td
                      className="py-3 px-3 text-sm tabular-nums text-right"
                      style={{ color: "var(--text-primary)" }}
                    >
                      {formatCurrency(t.estimatedValue)}
                    </td>
                    <td className="py-3 px-3 text-center">
                      <span
                        className="text-sm tabular-nums font-semibold"
                        style={{ color: accColor }}
                      >
                        {t.scoutingAccuracy}%
                      </span>
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
