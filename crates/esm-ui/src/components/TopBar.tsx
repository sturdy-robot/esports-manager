import { useState } from "react";
import { Activity, Play, Save } from "lucide-react";
import { PlayMatchButton } from "./PlayMatchButton";
import type { MatchMode } from "./PlayMatchButton";

export type ContinueMode = "smart" | "step";

const MONTH_NAMES = [
  "Jan",
  "Feb",
  "Mar",
  "Apr",
  "May",
  "Jun",
  "Jul",
  "Aug",
  "Sep",
  "Oct",
  "Nov",
  "Dec",
];

interface TopBarProps {
  title: string;
  year?: number;
  month?: number;
  day?: number;
  dayOfWeek?: string;
  phase?: string;
  teamName?: string;
  isMatchDay?: boolean;
  continueDisabled?: boolean;
  onSave?: () => void;
  onContinue?: (mode: ContinueMode) => void;
  onPlayMatch?: (mode: MatchMode) => void;
}

export function TopBar({
  title,
  year = 2025,
  month = 1,
  day = 1,
  dayOfWeek = "Wed",
  phase = "Morning",
  teamName,
  isMatchDay = false,
  continueDisabled = false,
  onSave,
  onContinue,
  onPlayMatch,
}: TopBarProps) {
  const monthLabel = MONTH_NAMES[(month - 1) % 12];
  const [continueMode, setContinueMode] = useState<ContinueMode>("smart");

  return (
    <header
      className="app-panel flex items-center justify-between min-h-18 px-8 shrink-0"
      style={{
        borderTop: "none",
        borderLeft: "none",
        borderRight: "none",
        borderRadius: 0,
      }}
    >
      {/* Left: title + team badge */}
      <div className="flex items-center gap-4">
        <div className="flex flex-col gap-0.5">
          <span className="app-eyebrow">Operations</span>
          <h1
            className="text-[1.75rem] font-bold font-display leading-none"
            style={{ color: "var(--text-primary)" }}
          >
            {title}
          </h1>
        </div>
        {teamName && (
          <span
            className="px-3 py-1 rounded-full text-xs font-semibold tabular-nums"
            style={{
              background: "var(--accent-gradient-soft)",
              color: "var(--text-primary)",
              border: "1px solid var(--border-strong)",
            }}
          >
            {teamName}
          </span>
        )}
      </div>

      {/* Right: game clock + actions */}
      <div className="flex items-center gap-3">
        {/* Game clock */}
        <div
          className="app-panel flex items-center gap-2.5 px-4 py-2 rounded-xl text-sm"
          style={{
            color: "var(--text-secondary)",
          }}
        >
          <Activity
            size={14}
            className="animate-live"
            style={{ color: "var(--color-accent-cyan)" }}
          />
          <span className="tabular-nums">
            {dayOfWeek}, {monthLabel} {day}, {year}
          </span>
          <span style={{ color: "var(--text-muted)" }}>·</span>
          <span>{phase}</span>
        </div>

        {/* Save */}
        {onSave && (
          <button
            onClick={onSave}
            className="app-button-secondary flex items-center gap-1.5 px-3.5 py-2 rounded-xl text-xs font-semibold cursor-pointer transition-colors"
          >
            <Save size={12} />
            Save
          </button>
        )}

        {/* Play Match or Continue — always rightmost */}
        {isMatchDay && onPlayMatch ? (
          <PlayMatchButton onConfirm={onPlayMatch} />
        ) : onContinue ? (
          <div className="flex items-center gap-2">
            <select
              aria-label="Continue mode"
              value={continueMode}
              onChange={(e) => setContinueMode(e.target.value as ContinueMode)}
              className="app-select px-3 py-2 rounded-xl text-xs font-semibold"
            >
              <option value="smart">Smart</option>
              <option value="step">Step</option>
            </select>
            <button
              onClick={() => onContinue(continueMode)}
              disabled={continueDisabled}
              className="app-button-primary flex items-center gap-1.5 px-4 py-2 rounded-xl text-xs font-semibold cursor-pointer border-none"
              style={{
                background: continueDisabled
                  ? "rgba(255, 255, 255, 0.05)"
                  : "var(--accent-gradient)",
                color: continueDisabled ? "var(--text-muted)" : "#fff",
                cursor: continueDisabled ? "not-allowed" : "pointer",
                boxShadow: continueDisabled ? "none" : undefined,
              }}
            >
              <Play size={12} />
              Continue
            </button>
          </div>
        ) : null}
      </div>
    </header>
  );
}
