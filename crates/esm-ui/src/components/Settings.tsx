import { useState } from "react";
import { ArrowLeft } from "lucide-react";

interface SettingsProps {
  onBack: () => void;
}

export function Settings({ onBack }: SettingsProps) {
  const [savesFolder, setSavesFolder] = useState("~/.esm/saves");

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
          <span className="app-eyebrow">Configuration</span>
          <h1
            className="text-xl font-bold font-display"
            style={{ color: "var(--text-primary)" }}
          >
            Settings
          </h1>
        </div>
      </header>

      {/* Content */}
      <main className="flex-1 flex justify-center py-12 px-6">
        <div className="w-full max-w-lg flex flex-col gap-6">
          <div className="flex flex-col gap-1.5">
            <span className="app-eyebrow">Operations Preferences</span>
            <h2 className="app-page-title">Application settings</h2>
            <p className="app-page-subtitle">
              Configure storage and review the current visual profile for this
              build.
            </p>
          </div>

          {/* Saves folder */}
          <div
            className="app-panel p-6 rounded-2xl"
            style={{
              backgroundColor: "var(--bg-surface)",
            }}
          >
            <h2
              className="app-eyebrow mb-4"
              style={{ color: "var(--text-muted)" }}
            >
              Storage
            </h2>
            <div className="flex flex-col gap-1.5">
              <label
                htmlFor="saves-folder"
                className="app-eyebrow"
                style={{ color: "var(--text-muted)" }}
              >
                Saves Folder
              </label>
              <input
                id="saves-folder"
                type="text"
                value={savesFolder}
                onChange={(e) => setSavesFolder(e.target.value)}
                className="app-input px-3 py-2.5 rounded-xl text-sm transition-colors"
              />
            </div>
          </div>

          {/* Appearance */}
          <div
            className="app-panel p-6 rounded-2xl"
            style={{
              backgroundColor: "var(--bg-surface)",
            }}
          >
            <h2
              className="app-eyebrow mb-4"
              style={{ color: "var(--text-muted)" }}
            >
              Appearance
            </h2>
            <div className="flex flex-col gap-1.5">
              <span
                className="text-sm font-medium"
                style={{ color: "var(--text-primary)" }}
              >
                Visual profile
              </span>
              <span
                className="text-sm"
                style={{ color: "var(--text-secondary)" }}
              >
                Broadcaster Command Center + Performance Lab
              </span>
              <span className="text-xs" style={{ color: "var(--text-muted)" }}>
                This first redesign pass ships with a fixed dark presentation
                focused on contrast, legibility, and data density.
              </span>
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}
