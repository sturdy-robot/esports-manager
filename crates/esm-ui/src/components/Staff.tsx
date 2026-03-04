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
    <div className="flex flex-col gap-4 animate-fade-in-up">
      <h2
        className="text-lg font-bold"
        style={{ color: "var(--text-primary)" }}
      >
        Staff
      </h2>

      {members.length === 0 && (
        <div className="flex flex-col items-center justify-center gap-3 py-16">
          <UserCog size={48} style={{ color: "var(--text-muted)" }} />
          <p style={{ color: "var(--text-muted)" }}>No staff members</p>
        </div>
      )}

      {members.length > 0 && (
        <div className="grid grid-cols-2 gap-4">
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
                className="flex items-center gap-4 p-4 rounded-lg border glow-hover transition-all"
                style={{
                  backgroundColor: "var(--bg-surface)",
                  borderColor: "var(--border-subtle)",
                }}
              >
                {/* Avatar */}
                <div
                  className="w-10 h-10 rounded-full flex items-center justify-center shrink-0"
                  style={{
                    backgroundColor: "var(--bg-elevated)",
                    color: "var(--text-muted)",
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
                  <div
                    className="text-xs"
                    style={{ color: "var(--text-secondary)" }}
                  >
                    {m.role}
                  </div>
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
                    className="text-lg font-bold font-mono"
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
