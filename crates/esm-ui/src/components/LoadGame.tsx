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
          <span className="app-eyebrow">Career Archives</span>
          <h1
            className="text-xl font-bold font-display"
            style={{ color: "var(--text-primary)" }}
          >
            Load Game
          </h1>
        </div>
      </header>

      {/* Content */}
      <main className="flex-1 flex justify-center py-12 px-6">
        <div className="w-full max-w-lg flex flex-col gap-4">
          <div className="flex flex-col gap-1.5">
            <span className="app-eyebrow">Save Files</span>
            <h2 className="app-page-title">Continue a career</h2>
            <p className="app-page-subtitle">
              Resume an existing run and return to the active management cycle.
            </p>
          </div>

          {loading && (
            <div className="app-panel rounded-2xl flex items-center justify-center gap-2 py-12">
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
            <div className="app-panel rounded-2xl flex flex-col items-center justify-center gap-3 py-16">
              <FolderOpen size={48} style={{ color: "var(--text-muted)" }} />
              <p style={{ color: "var(--text-muted)" }}>No saved games found</p>
            </div>
          )}

          {!loading &&
            saves.map((save) => (
              <div
                key={save.name}
                className="app-panel flex items-center justify-between p-4 rounded-2xl glow-hover transition-all"
                style={{
                  backgroundColor: "var(--bg-surface)",
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
                    className="text-xs tabular-nums"
                    style={{ color: "var(--text-muted)" }}
                  >
                    {save.checksum.slice(0, 12)}
                  </span>
                </div>
                <div className="flex items-center gap-2">
                  {onDelete && (
                    <button
                      onClick={() => onDelete(save.name)}
                      className="app-icon-button flex items-center justify-center w-8 h-8 rounded-xl cursor-pointer transition-colors"
                      style={{
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
                    className="app-button-primary flex items-center gap-1.5 px-3 py-1.5 rounded-xl text-xs font-semibold cursor-pointer border-none transition-all"
                    style={{
                      background: "var(--accent-gradient)",
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
