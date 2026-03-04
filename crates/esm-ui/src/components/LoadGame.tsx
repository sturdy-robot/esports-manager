import { ArrowLeft, FolderOpen, Play, Loader2, Trash2 } from "lucide-react";

export interface SaveEntry {
  name: string;
  checksum: string;
}

interface LoadGameProps {
  onBack: () => void;
  onLoad: (saveName: string) => void;
  saves?: SaveEntry[];
  loading?: boolean;
  onDelete?: (saveName: string) => void;
}

export function LoadGame({
  onBack,
  onLoad,
  saves = [],
  loading = false,
  onDelete,
}: LoadGameProps) {
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

      {/* Content */}
      <main className="flex-1 flex justify-center py-12 px-6">
        <div className="w-full max-w-lg flex flex-col gap-4">
          {loading && (
            <div className="flex items-center justify-center gap-2 py-12">
              <Loader2
                size={20}
                className="animate-spin"
                style={{ color: "var(--color-accent-cyan)" }}
              />
              <span style={{ color: "var(--text-secondary)" }}>
                Loading saves…
              </span>
            </div>
          )}

          {!loading && saves.length === 0 && (
            <div className="flex flex-col items-center justify-center gap-3 py-16">
              <FolderOpen
                size={48}
                style={{ color: "var(--text-muted)" }}
              />
              <p style={{ color: "var(--text-muted)" }}>
                No saved games found
              </p>
            </div>
          )}

          {!loading &&
            saves.map((save) => (
              <div
                key={save.name}
                className="flex items-center justify-between p-4 rounded-lg border glow-hover transition-all"
                style={{
                  backgroundColor: "var(--bg-surface)",
                  borderColor: "var(--border-subtle)",
                }}
              >
                <div className="flex flex-col gap-1">
                  <span
                    className="text-sm font-semibold"
                    style={{ color: "var(--text-primary)" }}
                  >
                    {save.name}
                  </span>
                  <span
                    className="text-xs font-mono"
                    style={{ color: "var(--text-muted)" }}
                  >
                    {save.checksum.slice(0, 12)}
                  </span>
                </div>
                <div className="flex items-center gap-2">
                  {onDelete && (
                    <button
                      onClick={() => onDelete(save.name)}
                      className="flex items-center justify-center w-8 h-8 rounded-md cursor-pointer border transition-colors"
                      style={{
                        borderColor: "var(--border-subtle)",
                        backgroundColor: "transparent",
                        color: "var(--text-muted)",
                      }}
                      onMouseEnter={(e) => {
                        e.currentTarget.style.borderColor = "var(--color-loss)";
                        e.currentTarget.style.color = "var(--color-loss)";
                      }}
                      onMouseLeave={(e) => {
                        e.currentTarget.style.borderColor =
                          "var(--border-subtle)";
                        e.currentTarget.style.color = "var(--text-muted)";
                      }}
                      title={`Delete ${save.name}`}
                    >
                      <Trash2 size={14} />
                    </button>
                  )}
                  <button
                    onClick={() => onLoad(save.name)}
                    className="flex items-center gap-1.5 px-3 py-1.5 rounded-md text-xs font-semibold cursor-pointer border-none transition-all"
                    style={{
                      background:
                        "linear-gradient(135deg, var(--color-accent-cyan), var(--color-accent-violet))",
                      color: "#fff",
                    }}
                  >
                    <Play size={12} />
                    Load
                  </button>
                </div>
              </div>
            ))}
        </div>
      </main>
    </div>
  );
}
