import { UserCog } from "lucide-react";

export interface StaffMember {
  id: string;
  name: string;
  role: string;
  skill: number;
}

interface StaffProps {
  members: StaffMember[];
}

export function Staff({ members }: StaffProps) {
  return (
    <div className="flex flex-col gap-6 animate-fade-in-up">
      <div className="flex flex-col gap-1.5">
        <span className="app-eyebrow">Support Structure</span>
        <h2 className="app-page-title">Staff</h2>
        <p className="app-page-subtitle">
          Evaluate the specialists around your roster and monitor the support
          quality behind daily performance.
        </p>
      </div>

      {members.length === 0 && (
        <div className="app-panel rounded-2xl flex flex-col items-center justify-center gap-3 py-16">
          <UserCog size={48} style={{ color: "var(--text-muted)" }} />
          <p style={{ color: "var(--text-muted)" }}>No staff members</p>
        </div>
      )}

      {members.length > 0 && (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {members.map((m) => {
            const skillColor =
              m.skill >= 85
                ? "var(--color-win)"
                : m.skill >= 70
                  ? "var(--color-accent-cyan)"
                  : m.skill >= 50
                    ? "var(--color-warning)"
                    : "var(--color-loss)";

            return (
              <div
                key={m.id}
                className="app-panel flex items-center gap-4 p-5 rounded-2xl glow-hover transition-all"
                style={{
                  backgroundColor: "var(--bg-surface)",
                }}
              >
                {/* Avatar */}
                <div
                  className="w-11 h-11 rounded-2xl flex items-center justify-center shrink-0"
                  style={{
                    background: "var(--accent-gradient-soft)",
                    border: "1px solid var(--border-strong)",
                    color: "var(--color-accent-cyan)",
                  }}
                >
                  <UserCog size={18} />
                </div>

                {/* Info */}
                <div className="flex-1 min-w-0">
                  <div
                    className="text-sm font-semibold truncate"
                    style={{ color: "var(--text-primary)" }}
                  >
                    {m.name}
                  </div>
                  <span
                    className="inline-flex mt-1 px-2 py-0.5 rounded-full text-xs font-semibold"
                    style={{
                      backgroundColor: "rgba(6, 182, 212, 0.1)",
                      color: "var(--color-accent-cyan)",
                    }}
                  >
                    {m.role}
                  </span>
                </div>

                {/* Skill badge */}
                <div className="flex flex-col items-center shrink-0">
                  <span
                    className="text-xs font-semibold uppercase"
                    style={{ color: "var(--text-muted)" }}
                  >
                    Skill
                  </span>
                  <span
                    className="text-lg font-bold tabular-nums font-display"
                    style={{ color: skillColor }}
                  >
                    {m.skill}
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
