import { ArrowLeft } from "lucide-react";

interface LoadGameProps {
  onBack: () => void;
  onLoad: (saveName: string) => void;
}

export function LoadGame({ onBack }: LoadGameProps) {
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
          Load Game
        </h1>
      </header>

      {/* Content placeholder */}
      <main className="flex-1 flex items-center justify-center">
        <p style={{ color: "var(--text-muted)" }}>
          Saved games list — coming soon
        </p>
      </main>
    </div>
  );
}
