import { useState } from "react";
import { ArrowLeft, Sun, Moon } from "lucide-react";
import { useTheme } from "@/lib/use-theme";

interface SettingsProps {
  onBack: () => void;
}

export function Settings({ onBack }: SettingsProps) {
  const { theme, toggleTheme } = useTheme();
  const [savesFolder, setSavesFolder] = useState("~/.esm/saves");

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
          Settings
        </h1>
      </header>

      {/* Content */}
      <main className="flex-1 flex justify-center py-12 px-6">
        <div className="w-full max-w-lg flex flex-col gap-6">
          {/* Saves folder */}
          <div
            className="p-6 rounded-lg border"
            style={{
              backgroundColor: "var(--bg-surface)",
              borderColor: "var(--border-subtle)",
            }}
          >
            <h2
              className="text-sm font-semibold uppercase tracking-wider mb-4"
              style={{ color: "var(--text-muted)" }}
            >
              Storage
            </h2>
            <div className="flex flex-col gap-1.5">
              <label
                htmlFor="saves-folder"
                className="text-xs font-semibold uppercase tracking-wider"
                style={{ color: "var(--text-muted)" }}
              >
                Saves Folder
              </label>
              <input
                id="saves-folder"
                type="text"
                value={savesFolder}
                onChange={(e) => setSavesFolder(e.target.value)}
                className="px-3 py-2 rounded-md text-sm border outline-none transition-colors"
                style={{
                  backgroundColor: "var(--bg-elevated)",
                  borderColor: "var(--border-subtle)",
                  color: "var(--text-primary)",
                }}
                onFocus={(e) => {
                  e.currentTarget.style.borderColor =
                    "var(--color-accent-cyan)";
                }}
                onBlur={(e) => {
                  e.currentTarget.style.borderColor = "var(--border-subtle)";
                }}
              />
            </div>
          </div>

          {/* Appearance */}
          <div
            className="p-6 rounded-lg border"
            style={{
              backgroundColor: "var(--bg-surface)",
              borderColor: "var(--border-subtle)",
            }}
          >
            <h2
              className="text-sm font-semibold uppercase tracking-wider mb-4"
              style={{ color: "var(--text-muted)" }}
            >
              Appearance
            </h2>
            <div className="flex items-center justify-between">
              <div className="flex flex-col gap-0.5">
                <span
                  className="text-sm font-medium"
                  style={{ color: "var(--text-primary)" }}
                >
                  Theme
                </span>
                <span
                  className="text-xs"
                  style={{ color: "var(--text-secondary)" }}
                >
                  Currently using{" "}
                  <span className="font-semibold">{theme}</span> mode
                </span>
              </div>
              <button
                onClick={toggleTheme}
                className="flex items-center gap-2 px-3 py-1.5 rounded-md text-xs font-medium cursor-pointer border transition-colors"
                style={{
                  borderColor: "var(--border-subtle)",
                  backgroundColor: "transparent",
                  color: "var(--text-secondary)",
                }}
                onMouseEnter={(e) => {
                  e.currentTarget.style.borderColor =
                    "var(--color-accent-cyan)";
                  e.currentTarget.style.color = "var(--text-primary)";
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.borderColor = "var(--border-subtle)";
                  e.currentTarget.style.color = "var(--text-secondary)";
                }}
              >
                {theme === "dark" ? <Sun size={14} /> : <Moon size={14} />}
                Switch to {theme === "dark" ? "light" : "dark"}
              </button>
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}
