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
      className="app-shell flex flex-col min-h-screen w-full animate-fade-in-up"
      style={{ backgroundColor: "var(--bg-base)" }}
    >
      {/* Header */}
      <header
        className="app-panel flex items-center gap-3 h-18 px-6 shrink-0"
        style={{
          borderTop: "none",
          borderLeft: "none",
          borderRight: "none",
          borderRadius: 0,
        }}
      >
        <button
          onClick={onBack}
          className="app-button-secondary flex items-center gap-2 px-3 py-1.5 rounded-xl text-sm font-medium cursor-pointer transition-colors"
        >
          <ArrowLeft size={16} />
          Back
        </button>
        <div className="flex flex-col gap-0.5">
          <span className="app-eyebrow">New Career</span>
          <h1
            className="text-xl font-bold font-display"
            style={{ color: "var(--text-primary)" }}
          >
            Team Selection
          </h1>
        </div>
      </header>

      {/* Content */}
      <main className="flex-1 flex justify-center py-12 px-6">
        <div className="w-full max-w-2xl flex flex-col gap-6">
          <div className="flex flex-col gap-1.5">
            <span className="app-eyebrow">Organization Search</span>
            <h2 className="app-page-title">Choose your club</h2>
            <p className="app-page-subtitle">
              Compare roster depth and reputation, then lock in the organization
              you want to lead.
            </p>
          </div>

          {teams.length === 0 && (
            <div className="app-panel rounded-2xl flex flex-col items-center justify-center gap-3 py-16">
              <Users size={48} style={{ color: "var(--text-muted)" }} />
              <p style={{ color: "var(--text-muted)" }}>No teams available</p>
            </div>
          )}

          {/* Team grid */}
          {teams.length > 0 && (
            <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
              {teams.map((team, idx) => {
                const isSelected = selectedIndex === idx;
                return (
                  <button
                    key={team.name}
                    type="button"
                    onClick={() => setSelectedIndex(idx)}
                    className="app-panel flex items-center gap-4 p-4 rounded-2xl text-left cursor-pointer transition-all"
                    style={{
                      backgroundColor: isSelected
                        ? "var(--bg-elevated)"
                        : "var(--bg-surface)",
                      borderColor: isSelected
                        ? "var(--border-strong)"
                        : "var(--border-subtle)",
                      boxShadow: isSelected ? "var(--accent-glow)" : "none",
                    }}
                  >
                    {/* Team badge */}
                    <div
                      className="flex items-center justify-center w-12 h-12 rounded-2xl text-sm font-bold shrink-0"
                      style={{
                        background: isSelected
                          ? "var(--accent-gradient)"
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
              className="app-button-primary flex items-center justify-center gap-2 w-full px-6 py-3 rounded-2xl text-sm font-semibold cursor-pointer border-none transition-all"
              style={{
                background:
                  selectedIndex !== null
                    ? "var(--accent-gradient)"
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
