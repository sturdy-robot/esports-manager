import { useState } from "react";
import { ArrowLeft, ChevronRight, Users, Star } from "lucide-react";

export interface TeamOption {
  name: string;
  tag: string;
  playerCount: number;
  reputation: number;
}

interface TeamSelectionProps {
  teams: TeamOption[];
  onSelect: (teamIndex: number) => void;
  onBack: () => void;
}

export function TeamSelection({ teams, onSelect, onBack }: TeamSelectionProps) {
  const [selectedIndex, setSelectedIndex] = useState<number | null>(null);

  const handleContinue = () => {
    if (selectedIndex !== null) {
      onSelect(selectedIndex);
    }
  };

  return (
    <div
      className="flex flex-col min-h-screen w-full animate-fade-in-up"
      style={{ backgroundColor: "var(--bg-base)" }}
    >
      {/* Header */}
      <header
        className="flex items-center gap-3 h-14 px-6 border-b shrink-0"
        style={{
          borderColor: "var(--border-subtle)",
          backgroundColor: "var(--bg-surface)",
        }}
      >
        <button
          onClick={onBack}
          className="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium cursor-pointer border transition-colors"
          style={{
            borderColor: "var(--border-subtle)",
            backgroundColor: "transparent",
            color: "var(--text-secondary)",
          }}
          onMouseEnter={(e) => {
            e.currentTarget.style.borderColor = "var(--color-accent-cyan)";
            e.currentTarget.style.color = "var(--text-primary)";
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.borderColor = "var(--border-subtle)";
            e.currentTarget.style.color = "var(--text-secondary)";
          }}
        >
          <ArrowLeft size={16} />
          Back
        </button>
        <h1
          className="text-xl font-bold"
          style={{ color: "var(--text-primary)" }}
        >
          Team Selection
        </h1>
      </header>

      {/* Content */}
      <main className="flex-1 flex justify-center py-12 px-6">
        <div className="w-full max-w-2xl flex flex-col gap-6">
          {teams.length === 0 && (
            <div className="flex flex-col items-center justify-center gap-3 py-16">
              <Users size={48} style={{ color: "var(--text-muted)" }} />
              <p style={{ color: "var(--text-muted)" }}>
                No teams available
              </p>
            </div>
          )}

          {/* Team grid */}
          {teams.length > 0 && (
            <div className="grid grid-cols-2 gap-3">
              {teams.map((team, idx) => {
                const isSelected = selectedIndex === idx;
                return (
                  <button
                    key={team.name}
                    type="button"
                    onClick={() => setSelectedIndex(idx)}
                    className="flex items-center gap-4 p-4 rounded-lg border text-left cursor-pointer transition-all"
                    style={{
                      backgroundColor: isSelected
                        ? "var(--bg-elevated)"
                        : "var(--bg-surface)",
                      borderColor: isSelected
                        ? "var(--color-accent-cyan)"
                        : "var(--border-subtle)",
                      boxShadow: isSelected ? "var(--accent-glow)" : "none",
                    }}
                  >
                    {/* Team badge */}
                    <div
                      className="flex items-center justify-center w-12 h-12 rounded-full text-sm font-bold shrink-0"
                      style={{
                        background: isSelected
                          ? "linear-gradient(135deg, var(--color-accent-emerald), var(--color-accent-cyan))"
                          : "var(--bg-elevated)",
                        color: isSelected ? "#fff" : "var(--text-primary)",
                        border: isSelected
                          ? "none"
                          : "1px solid var(--border-subtle)",
                      }}
                    >
                      {team.tag}
                    </div>

                    {/* Team info */}
                    <div className="flex flex-col gap-1 flex-1 min-w-0">
                      <span
                        className="text-sm font-semibold truncate"
                        style={{ color: "var(--text-primary)" }}
                      >
                        {team.name}
                      </span>
                      <div className="flex items-center gap-3">
                        <span
                          className="flex items-center gap-1 text-xs"
                          style={{ color: "var(--text-secondary)" }}
                        >
                          <Users size={12} />
                          {team.playerCount} players
                        </span>
                        <span
                          className="flex items-center gap-1 text-xs"
                          style={{ color: "var(--text-secondary)" }}
                        >
                          <Star size={12} />
                          <span
                            className="tabular-nums font-semibold"
                            style={{ color: "var(--text-primary)" }}
                          >
                            {team.reputation}
                          </span>
                        </span>
                      </div>
                    </div>
                  </button>
                );
              })}
            </div>
          )}

          {/* Continue */}
          {teams.length > 0 && (
            <button
              onClick={handleContinue}
              disabled={selectedIndex === null}
              className="flex items-center justify-center gap-2 w-full px-6 py-3 rounded-lg text-sm font-semibold cursor-pointer border-none transition-all"
              style={{
                background:
                  selectedIndex !== null
                    ? "linear-gradient(135deg, var(--color-accent-emerald), var(--color-accent-cyan))"
                    : "var(--bg-elevated)",
                color: selectedIndex !== null ? "#fff" : "var(--text-muted)",
                opacity: selectedIndex !== null ? 1 : 0.6,
                cursor: selectedIndex !== null ? "pointer" : "not-allowed",
              }}
            >
              Continue
              <ChevronRight size={16} />
            </button>
          )}
        </div>
      </main>
    </div>
  );
}
