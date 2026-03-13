import { Users } from "lucide-react";

export interface RosterPlayer {
  nickname: string;
  firstName: string;
  lastName: string;
  role: string;
  stamina: number;
  morale: number;
  mechanics: number;
  vision: number;
  teamfighting: number;
}

interface RosterProps {
  players: RosterPlayer[];
}

const ROLE_COLORS: Record<string, string> = {
  Top: "#E879F9",
  Jungle: "#34D399",
  Mid: "#60A5FA",
  Bot: "#F97316",
  Support: "#FBBF24",
};

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

function StatBar({ value, label }: { value: number; label: string }) {
  const pct = Math.min(100, Math.max(0, value));
  const color =
    pct >= 70
      ? "var(--color-win)"
      : pct >= 40
        ? "var(--color-warning)"
        : "var(--color-loss)";

  return (
    <td className="py-3 px-3">
      <div className="flex items-center gap-2">
        <div
          className="flex-1 h-1.5 rounded-full overflow-hidden"
          style={{ backgroundColor: "var(--bg-elevated)" }}
          title={`${label}: ${value}`}
        >
          <div
            className="h-full rounded-full transition-all"
            style={{ width: `${pct}%`, backgroundColor: color }}
          />
        </div>
        <span
          className="text-xs tabular-nums w-6 text-right"
          style={{ color: "var(--text-secondary)" }}
        >
          {value}
        </span>
      </div>
    </td>
  );
}

export function Roster({ players }: RosterProps) {
  return (
    <div className="flex flex-col gap-4 animate-fade-in-up">
      <div className="flex flex-col gap-1.5">
        <span className="app-eyebrow">Squad Intelligence</span>
        <h2 className="app-page-title">Roster</h2>
        <p className="app-page-subtitle">
          Monitor core attributes, stamina, and morale across the active lineup.
        </p>
      </div>

      {players.length === 0 && (
        <div className="app-panel rounded-2xl flex flex-col items-center justify-center gap-3 py-16">
          <Users size={48} style={{ color: "var(--text-muted)" }} />
          <p style={{ color: "var(--text-muted)" }}>No players on the roster</p>
        </div>
      )}

      {players.length > 0 && (
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
                  className="text-left text-xs font-semibold uppercase tracking-wider py-2.5 px-3"
                  style={{ color: "var(--text-muted)" }}
                >
                  Stamina
                </th>
                <th
                  className="text-left text-xs font-semibold uppercase tracking-wider py-2.5 px-3"
                  style={{ color: "var(--text-muted)" }}
                >
                  Morale
                </th>
              </tr>
            </thead>
            <tbody>
              {players.map((p) => (
                <tr
                  key={p.nickname}
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
                  {/* Player identity */}
                  <td className="py-3 px-3">
                    <div className="flex flex-col">
                      <span
                        className="text-sm font-semibold"
                        style={{ color: "var(--text-primary)" }}
                      >
                        {p.nickname}
                      </span>
                      <span
                        className="text-xs"
                        style={{ color: "var(--text-muted)" }}
                      >
                        {p.firstName} {p.lastName}
                      </span>
                    </div>
                  </td>

                  {/* Role badge */}
                  <td className="py-3 px-3">
                    <span
                      className="px-2 py-0.5 rounded text-xs font-semibold"
                      style={{
                        backgroundColor: `${ROLE_COLORS[p.role] || "var(--color-accent-cyan)"}20`,
                        color:
                          ROLE_COLORS[p.role] || "var(--color-accent-cyan)",
                      }}
                    >
                      {p.role}
                    </span>
                  </td>

                  {/* Attributes */}
                  <AttributeCell value={p.mechanics} />
                  <AttributeCell value={p.vision} />
                  <AttributeCell value={p.teamfighting} />

                  {/* State bars */}
                  <StatBar value={p.stamina} label="Stamina" />
                  <StatBar value={p.morale} label="Morale" />
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}
