import { useState, useEffect, useRef, useCallback } from "react";
import {
  Swords,
  Shield,
  Flame,
  Crown,
  Castle,
  Trophy,
  Zap,
  ChevronRight,
  Play,
  Pause,
  SkipForward,
  Coins,
  Crosshair,
  SlidersHorizontal,
  UserRound,
  X,
} from "lucide-react";
import type {
  SimulateMatchResult,
  MatchEventInfo,
  GameSnapshotInfo,
  PlayerSnapshotInfo,
  PlaystyleType,
  FocusType,
  TacticsInfo,
  MatchRosterEntry,
} from "@/lib/api";

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const BASE_INTERVAL_MS = 1500;
const SPEEDS = [1, 2, 5, 10] as const;
const POSITIONS = ["TOP", "JGL", "MID", "BOT", "SUP"] as const;
const PLAYER_ROLE_TAG = {
  TOP: "bg-indigo-500/60",
  JGL: "bg-green-500/60",
  MID: "bg-blue-500/60",
  BOT: "bg-red-500/60",
  SUP: "bg-purple-500/60",
};

// ---------------------------------------------------------------------------
// Props
// ---------------------------------------------------------------------------

interface MatchSimUIProps {
  result: SimulateMatchResult;
  onComplete: () => void;
  /** Current tactics for mid-match adjustment UI */
  tactics?: TacticsInfo;
  /** Called when the player adjusts tactics mid-match */
  onTacticsChange?: (playstyle: PlaystyleType, focus: FocusType) => void;
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function formatMinute(minute: number): string {
  return `${minute}:00`;
}

function formatGold(gold: number): string {
  if (gold >= 1000) return `${(gold / 1000).toFixed(1)}k`;
  return String(gold);
}

function formatGoldFull(gold: number): string {
  return gold.toLocaleString();
}

function eventIcon(kind: string) {
  switch (kind) {
    case "solo_kill":
      return <Swords size={14} />;
    case "teamfight":
      return <Zap size={14} />;
    case "tower":
      return <Castle size={14} />;
    case "dragon":
      return <Flame size={14} />;
    case "herald":
      return <Shield size={14} />;
    case "baron":
      return <Crown size={14} />;
    case "inhibitor":
      return <Castle size={14} />;
    case "nexus":
      return <Trophy size={14} />;
    case "multi_kill":
      return <Zap size={14} />;
    case "killing_spree":
      return <Flame size={14} />;
    default:
      return <ChevronRight size={14} />;
  }
}

function eventColor(kind: string): string {
  switch (kind) {
    case "solo_kill":
      return "#EF4444";
    case "teamfight":
      return "#F59E0B";
    case "tower":
      return "#10B981";
    case "dragon":
      return "#06B6D4";
    case "herald":
      return "#10B981";
    case "baron":
      return "#06B6D4";
    case "inhibitor":
      return "#EC4899";
    case "nexus":
      return "#FFD700";
    case "multi_kill":
      return "#FF6B6B";
    case "killing_spree":
      return "#FF9F43";
    default:
      return "var(--text-secondary)";
  }
}

function phaseBadgeColor(phase: string): string {
  switch (phase) {
    case "Early":
      return "#3B82F6";
    case "Mid":
      return "#F59E0B";
    case "Late":
      return "#EF4444";
    default:
      return "var(--text-muted)";
  }
}

// ---------------------------------------------------------------------------
// Main Component
// ---------------------------------------------------------------------------

export function MatchSimUI({
  result,
  onComplete,
  tactics,
  onTacticsChange,
}: MatchSimUIProps) {
  const [revealedCount, setRevealedCount] = useState(0);
  const [isPlaying, setIsPlaying] = useState(true);
  const [speedIdx, setSpeedIdx] = useState(0);
  const [isTacticsOpen, setIsTacticsOpen] = useState(false);
  const feedRef = useRef<HTMLDivElement>(null);
  const resumePlaybackRef = useRef(false);

  const allDone = revealedCount >= result.events.length;
  const visibleEvents = result.events.slice(0, revealedCount);
  const latestEvent =
    visibleEvents.length > 0 ? visibleEvents[visibleEvents.length - 1] : null;
  const snapshot = latestEvent?.snapshot ?? null;
  const currentMinute = latestEvent?.minute ?? 0;

  // Progressive reveal timer
  useEffect(() => {
    if (!isPlaying || allDone) return;

    const ms = BASE_INTERVAL_MS / SPEEDS[speedIdx];
    const id = setTimeout(() => setRevealedCount((prev) => prev + 1), ms);
    return () => clearTimeout(id);
  }, [isPlaying, speedIdx, revealedCount, allDone]);

  // Auto-scroll event feed
  useEffect(() => {
    if (feedRef.current) {
      feedRef.current.scrollTop = feedRef.current.scrollHeight;
    }
  }, [revealedCount]);

  const handleSkip = useCallback(() => {
    setRevealedCount(result.events.length);
    setIsPlaying(false);
  }, [result.events.length]);

  const cycleSpeed = useCallback(() => {
    setSpeedIdx((prev) => (prev + 1) % SPEEDS.length);
  }, []);

  const openTacticsPanel = useCallback(() => {
    resumePlaybackRef.current = isPlaying;
    setIsPlaying(false);
    setIsTacticsOpen(true);
  }, [isPlaying]);

  const closeTacticsPanel = useCallback(() => {
    setIsTacticsOpen(false);
    if (resumePlaybackRef.current && !allDone) {
      setIsPlaying(true);
    }
    resumePlaybackRef.current = false;
  }, [allDone]);

  useEffect(() => {
    if (!isTacticsOpen) return;

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        closeTacticsPanel();
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [isTacticsOpen, closeTacticsPanel]);

  const blueGold = snapshot?.blue_team_gold ?? 0;
  const redGold = snapshot?.red_team_gold ?? 0;
  const goldTotal = blueGold + redGold || 1;
  const bluePct = (blueGold / goldTotal) * 100;
  const currentPlaystyle = tactics
    ? PLAYSTYLE_OPTIONS.find((opt) => opt.value === tactics.playstyle)
    : null;
  const currentFocus = tactics
    ? FOCUS_OPTIONS.find((opt) => opt.value === tactics.focus)
    : null;

  return (
    <div className="relative app-shell flex flex-col w-full flex-1 min-h-0 gap-3 p-3">
      {/* Top bar: teams + gold + timer */}
      <div
        className="app-panel-strong flex items-center justify-between px-4 py-3 rounded-3xl shrink-0"
        style={{
          backgroundColor: "var(--bg-surface)",
        }}
      >
        <TeamHeader name={result.blue_team} gold={blueGold} color="#3B82F6" />
        <div className="text-center px-4">
          <div
            className="text-xl tabular-nums font-bold font-display"
            style={{ color: "var(--text-primary)" }}
          >
            {formatMinute(currentMinute)}
          </div>
          {allDone && (
            <div
              data-testid="winner-banner"
              className="text-sm font-bold uppercase tracking-wider px-3 py-1 rounded-full"
              style={{
                background:
                  "linear-gradient(135deg, rgba(16,185,129,0.15), rgba(6,182,212,0.15))",
                color: "var(--color-win)",
                border: "1px solid rgba(34,197,94,0.3)",
                boxShadow: "0 0 12px rgba(34,197,94,0.2)",
              }}
            >
              {result.winner} wins
            </div>
          )}
        </div>
        <TeamHeader name={result.red_team} gold={redGold} color="#EF4444" />
      </div>

      {/* Main area: scoreboard | events | scoreboard */}
      <div className="flex gap-3 flex-1 min-h-0">
        {/* Blue scoreboard */}
        <TeamScoreboard
          players={snapshot?.blue_players ?? []}
          roster={result.blue_roster}
          color="#3B82F6"
          side="blue"
          teamName={result.blue_team}
        />

        <div className="flex-1 min-h-0 flex flex-col gap-2">
          {/* Objectives bar */}
          <ObjectivesBar snapshot={snapshot} />
          {/* Gold bar */}
          <div
            className="h-1.5 rounded-full overflow-hidden shrink-0"
            style={{ backgroundColor: "var(--bg-elevated)" }}
          >
            <div
              className="h-full transition-all duration-500 ease-out"
              style={{
                width: `${bluePct}%`,
                background: "linear-gradient(90deg, #3B82F6, #60A5FA)",
              }}
            />
          </div>

          {/* Controls bar — playback */}
          <div
            className="app-panel flex flex-col items-center gap-3 px-3 py-2 rounded-2xl"
            style={{
              backgroundColor: "var(--bg-surface)",
            }}
          >
            {/* Playback controls */}
            <div className="flex items-center justify-between gap-3 w-full flex-wrap">
              <div className="flex items-center gap-2">
                {!allDone ? (
                  <>
                    <button
                      onClick={cycleSpeed}
                      className="app-button-secondary px-2.5 py-1 rounded-xl text-sm tabular-nums font-bold cursor-pointer transition-all duration-150 glow-hover"
                      style={{
                        color: "var(--text-primary)",
                      }}
                    >
                      {SPEEDS[speedIdx]}x
                    </button>
                    <button
                      onClick={() => setIsPlaying((p) => !p)}
                      className="app-icon-button p-1.5 rounded-xl cursor-pointer transition-all duration-150 glow-hover"
                      style={{
                        color: "var(--text-primary)",
                      }}
                    >
                      {isPlaying ? <Pause size={16} /> : <Play size={16} />}
                    </button>
                    <button
                      onClick={handleSkip}
                      className="app-icon-button p-1.5 rounded-xl cursor-pointer transition-all duration-150 glow-hover"
                      style={{
                        color: "var(--text-primary)",
                      }}
                      title="Skip to end"
                    >
                      <SkipForward size={16} />
                    </button>
                  </>
                ) : (
                  <button
                    onClick={onComplete}
                    className="app-button-primary px-5 py-1.5 rounded-2xl font-bold text-white text-sm flex items-center gap-1.5 transition-all duration-150"
                    style={{
                      background: "var(--accent-gradient)",
                      boxShadow: "0 0 12px rgba(6,182,212,0.3)",
                    }}
                  >
                    <ChevronRight size={16} />
                    Continue
                  </button>
                )}
              </div>
              {tactics && onTacticsChange && !allDone && (
                <button
                  onClick={openTacticsPanel}
                  className="app-panel flex items-center gap-2 px-2.5 py-1.5 rounded-2xl cursor-pointer transition-all duration-150 glow-hover"
                  style={{
                    backgroundColor: isTacticsOpen
                      ? "rgba(6,182,212,0.12)"
                      : "var(--bg-elevated)",
                    color: "var(--text-primary)",
                  }}
                >
                  <div
                    className="h-8 w-8 rounded-lg border flex items-center justify-center shrink-0"
                    style={{
                      borderColor: isTacticsOpen
                        ? "rgba(6,182,212,0.4)"
                        : "rgba(255,255,255,0.08)",
                      backgroundColor: isTacticsOpen
                        ? "rgba(6,182,212,0.16)"
                        : "rgba(255,255,255,0.03)",
                      color: "#06B6D4",
                    }}
                  >
                    <SlidersHorizontal size={14} />
                  </div>
                  <div className="text-left leading-tight">
                    <div
                      className="text-[0.55rem] uppercase"
                      style={{
                        color: "var(--text-muted)",
                        letterSpacing: "0.1em",
                      }}
                    >
                      Tactics
                    </div>
                    <div
                      className="text-xs font-bold"
                      style={{ color: "var(--text-primary)" }}
                    >
                      {currentPlaystyle?.title ?? "Set style"}
                      {currentFocus ? ` · ${currentFocus.title}` : ""}
                    </div>
                  </div>
                </button>
              )}
            </div>
          </div>
          {/* Event feed */}
          <div
            ref={feedRef}
            className="app-panel flex-1 rounded-3xl overflow-y-auto p-3"
            style={{
              backgroundColor: "var(--bg-surface)",
            }}
          >
            <div className="space-y-0.5">
              {visibleEvents.map((event, i) => (
                <EventRow
                  key={i}
                  event={event}
                  isNew={i === visibleEvents.length - 1}
                />
              ))}
              {!allDone && visibleEvents.length === 0 && (
                <div
                  className="text-center text-sm py-8"
                  style={{ color: "var(--text-muted)" }}
                >
                  Match starting...
                </div>
              )}
            </div>
          </div>
        </div>

        {/* Red scoreboard */}
        <TeamScoreboard
          players={snapshot?.red_players ?? []}
          roster={result.red_roster}
          color="#EF4444"
          side="red"
          teamName={result.red_team}
        />
      </div>
      {/* Inline tactics — always accessible */}
      {tactics && onTacticsChange && isTacticsOpen && (
        <TacticsPanel
          tactics={tactics}
          onChange={onTacticsChange}
          onClose={closeTacticsPanel}
        />
      )}
    </div>
  );
}

// ---------------------------------------------------------------------------
// Sub-components
// ---------------------------------------------------------------------------

function TeamHeader({
  name,
  gold,
  color,
}: {
  name: string;
  gold: number;
  color: string;
}) {
  return (
    <div className="text-center flex-1">
      <div className="app-eyebrow" style={{ color }}>
        {name}
      </div>
      <div
        className="text-2xl tabular-nums font-black font-display"
        style={{ color: "var(--text-primary)" }}
      >
        {formatGoldFull(gold)}
      </div>
    </div>
  );
}

function TeamScoreboard({
  players,
  roster,
  color,
  side,
  teamName,
}: {
  players: PlayerSnapshotInfo[];
  roster: MatchRosterEntry[];
  color: string;
  side: "blue" | "red";
  teamName: string;
}) {
  return (
    <div
      className="w-72 rounded-2xl border overflow-hidden shrink-0 flex flex-col shadow-lg"
      style={{
        background:
          side === "blue"
            ? "linear-gradient(180deg, rgba(59,130,246,0.14) 0%, rgba(20,20,31,0.96) 18%, rgba(20,20,31,1) 100%)"
            : "linear-gradient(180deg, rgba(239,68,68,0.14) 0%, rgba(20,20,31,0.96) 18%, rgba(20,20,31,1) 100%)",
        borderColor: "var(--border-subtle)",
        boxShadow:
          side === "blue"
            ? "0 10px 30px rgba(59,130,246,0.14)"
            : "0 10px 30px rgba(239,68,68,0.14)",
      }}
    >
      <div
        className="px-4 py-1 border-b"
        style={{
          borderColor: `${color}22`,
          background: `linear-gradient(135deg, ${color}1F, rgba(255,255,255,0.02))`,
        }}
      >
        <div className="flex items-center justify-between gap-3">
          <div className="flex items-center gap-2 min-w-0">
            <div
              className="h-9 w-9 rounded-xl border flex items-center justify-center shrink-0"
              style={{
                borderColor: `${color}45`,
                backgroundColor: `${color}18`,
                color,
              }}
            >
              <UserRound size={16} />
            </div>
            <div className="min-w-0">
              <div
                className="text-sm font-bold truncate"
                style={{ color: "var(--text-primary)" }}
              >
                {teamName}
              </div>
            </div>
          </div>
          <div
            className="rounded-full px-2.5 py-1 text-[0.6rem] font-bold uppercase tracking-[0.16em] shrink-0"
            style={{
              color,
              backgroundColor: `${color}14`,
              border: `1px solid ${color}28`,
            }}
          >
            {players.length}/5
          </div>
        </div>
      </div>
      <div className="flex-1 overflow-y-auto px-2 py-2">
        {players.length === 0 ? (
          <div
            className="text-xs text-center py-8 rounded-xl border"
            style={{
              color: "var(--text-muted)",
              borderColor: "var(--border-subtle)",
              backgroundColor: "rgba(255,255,255,0.02)",
            }}
          >
            —
          </div>
        ) : (
          players.map((p, i) => (
            <PlayerRow
              key={i}
              player={p}
              position={POSITIONS[i] ?? "?"}
              entry={roster[i]}
            />
          ))
        )}
      </div>
    </div>
  );
}

function PlayerRow({
  player,
  position,
  entry,
}: {
  player: PlayerSnapshotInfo;
  position: string;
  entry?: MatchRosterEntry;
}) {
  const roleTagClass =
    PLAYER_ROLE_TAG[position as keyof typeof PLAYER_ROLE_TAG];
  const deadStyle = player.is_dead ? { opacity: 0.55 } : {};
  return (
    <div
      className="flex flex-col gap-3 px-3 py-3 text-sm border rounded-xl"
      style={{
        borderColor: "rgba(255,255,255,0.06)",
        background:
          "linear-gradient(135deg, rgba(255,255,255,0.04) 0%, rgba(255,255,255,0.015) 100%)",
        boxShadow: "inset 0 1px 0 rgba(255,255,255,0.03)",
        ...deadStyle,
      }}
    >
      {/* Row 1: role, nickname, champion */}
      <div className="flex items-start justify-between gap-3">
        <div className="flex items-center gap-2 min-w-0 flex-1">
          <span
            className={`font-bold ${roleTagClass} text-white rounded-md px-2 py-1 shadow-sm shadow-black/20`}
            style={{ fontSize: "0.65rem", letterSpacing: "0.08em" }}
          >
            {position}
          </span>
          <div className="min-w-0 flex flex-col">
            <span
              className="font-semibold truncate leading-none"
              style={{ color: "var(--text-primary)", fontSize: "0.85rem" }}
            >
              {entry?.nickname ?? `P${position}`}
            </span>
            <span
              className="mt-1 font-medium uppercase"
              style={{
                color: player.is_dead ? "#FCA5A5" : "var(--text-muted)",
                fontSize: "0.55rem",
                letterSpacing: "0.12em",
              }}
            >
              {player.is_dead ? "Dead" : "Alive"}
            </span>
          </div>
        </div>
        {entry?.champion && (
          <div
            className="rounded-xl border px-2.5 py-2 shrink-0 max-w-36 min-w-30"
            style={{
              borderColor: "rgba(255,255,255,0.08)",
              background:
                "linear-gradient(135deg, rgba(255,255,255,0.05) 0%, rgba(255,255,255,0.015) 100%)",
            }}
          >
            <div className="flex items-center gap-2">
              <span
                className={`h-7 w-1.5 rounded-full ${roleTagClass} shrink-0`}
              />
              <div className="min-w-0">
                <div
                  className="uppercase font-semibold"
                  style={{
                    color: "var(--text-muted)",
                    fontSize: "0.5rem",
                    letterSpacing: "0.16em",
                  }}
                >
                  Champion
                </div>
                <div
                  className="truncate font-semibold"
                  style={{ color: "var(--text-primary)", fontSize: "0.72rem" }}
                >
                  {entry.champion}
                </div>
              </div>
            </div>
          </div>
        )}
      </div>
      {/* Row 2: KDA, CS, gold */}
      <div className="flex items-center gap-1.5 flex-wrap">
        <div
          className="rounded-lg border px-2 py-1.5"
          style={{
            minWidth: "4.6rem",
            borderColor: "rgba(255,255,255,0.08)",
            backgroundColor: "rgba(255,255,255,0.03)",
          }}
        >
          <div className="flex items-center gap-1 mb-1">
            <Swords size={10} style={{ color: "#F87171" }} />
            <div
              className="uppercase"
              style={{
                color: "var(--text-muted)",
                fontSize: "0.52rem",
                letterSpacing: "0.1em",
              }}
            >
              KDA
            </div>
          </div>
          <div
            className="tabular-nums font-bold leading-none"
            style={{ color: "var(--text-primary)", fontSize: "0.72rem" }}
          >
            {player.kills}/{player.deaths}/{player.assists}
          </div>
        </div>
        <div
          className="rounded-lg border px-2 py-1.5 min-w-16"
          style={{
            borderColor: "rgba(255,255,255,0.08)",
            backgroundColor: "rgba(255,255,255,0.03)",
          }}
        >
          <div className="flex items-center gap-1 mb-1">
            <Crosshair size={10} style={{ color: "var(--text-secondary)" }} />
            <div
              className="uppercase"
              style={{
                color: "var(--text-muted)",
                fontSize: "0.52rem",
                letterSpacing: "0.1em",
              }}
            >
              CS
            </div>
          </div>
          <div
            className="tabular-nums font-semibold leading-none"
            style={{ color: "var(--text-secondary)", fontSize: "0.72rem" }}
          >
            {player.cs}
          </div>
        </div>
        <div
          className="rounded-lg border px-2 py-1.5"
          style={{
            minWidth: "4.7rem",
            borderColor: "rgba(245,158,11,0.18)",
            backgroundColor: "rgba(245,158,11,0.08)",
          }}
        >
          <div className="flex items-center gap-1 mb-1">
            <Coins size={10} style={{ color: "#FCD34D" }} />
            <div
              className="uppercase"
              style={{
                color: "#FCD34D",
                fontSize: "0.52rem",
                letterSpacing: "0.1em",
              }}
            >
              Gold
            </div>
          </div>
          <div
            className="tabular-nums font-semibold leading-none"
            style={{ color: "#F59E0B", fontSize: "0.72rem" }}
          >
            {formatGold(player.gold)}
          </div>
        </div>
      </div>
    </div>
  );
}

function ObjectivesBar({ snapshot }: { snapshot: GameSnapshotInfo | null }) {
  if (!snapshot) return null;
  return (
    <div
      className="flex items-center justify-center gap-6 px-4 py-2 rounded-xl border shrink-0 flex-wrap"
      style={{
        backgroundColor: "var(--bg-surface)",
        borderColor: "var(--border-subtle)",
      }}
    >
      <ObjectiveChip
        icon={<Flame size={14} />}
        label="Dragons"
        blue={snapshot.dragons_blue}
        red={snapshot.dragons_red}
      />
      <ObjectiveChip
        icon={<Castle size={14} />}
        label="Towers"
        blue={snapshot.blue_towers}
        red={snapshot.red_towers}
      />
      <ObjectiveChip
        icon={<Shield size={14} />}
        label="Inhibitors"
        blue={snapshot.blue_inhibitors}
        red={snapshot.red_inhibitors}
      />
      <ObjectiveChip
        icon={<Crown size={14} />}
        label="Baron"
        status={
          snapshot.baron_alive
            ? "alive"
            : snapshot.baron_timer > 0
              ? `${snapshot.baron_timer}m`
              : "—"
        }
      />
      <ObjectiveChip
        icon={<Shield size={14} />}
        label="Herald"
        status={snapshot.herald_available ? "up" : "—"}
      />
    </div>
  );
}

function ObjectiveChip({
  icon,
  label,
  blue,
  red,
  status,
}: {
  icon: React.ReactNode;
  label: string;
  blue?: number;
  red?: number;
  status?: string;
}) {
  return (
    <div className="flex items-center gap-2 text-xs">
      <span style={{ color: "var(--text-muted)" }}>{icon}</span>
      <span
        className="font-semibold"
        style={{ color: "var(--text-secondary)" }}
      >
        {label}
      </span>
      {blue !== undefined && red !== undefined ? (
        <span
          className="tabular-nums font-bold"
          style={{ color: "var(--text-primary)" }}
        >
          <span style={{ color: "#3B82F6" }}>{blue}</span>
          {" – "}
          <span style={{ color: "#EF4444" }}>{red}</span>
        </span>
      ) : (
        <span className="tabular-nums" style={{ color: "var(--text-muted)" }}>
          {status}
        </span>
      )}
    </div>
  );
}
const PLAYSTYLE_OPTIONS: {
  value: PlaystyleType;
  label: string;
  title: string;
  description: string;
  color: string;
}[] = [
  {
    value: "aggressive",
    label: "AGR",
    title: "Aggressive",
    description: "Push for picks, skirmishes, and tempo swings.",
    color: "#EF4444",
  },
  {
    value: "balanced",
    label: "BAL",
    title: "Balanced",
    description: "Stay flexible and adapt around the current game state.",
    color: "#06B6D4",
  },
  {
    value: "defensive",
    label: "DEF",
    title: "Defensive",
    description: "Prioritize safer setups, vision, and cleaner resets.",
    color: "#22C55E",
  },
];

const FOCUS_OPTIONS: {
  value: FocusType;
  label: string;
  title: string;
  description: string;
  color: string;
}[] = [
  {
    value: "teamfight",
    label: "TF",
    title: "Teamfight",
    description: "Group earlier and draft fights around cooldown spikes.",
    color: "#10B981",
  },
  {
    value: "splitpush",
    label: "SP",
    title: "Split Push",
    description: "Stretch side lanes and pressure rotations across the map.",
    color: "#F59E0B",
  },
  {
    value: "objective",
    label: "OBJ",
    title: "Objective",
    description: "Play around dragon, baron, towers, and setup control.",
    color: "#06B6D4",
  },
];

function TacticsPanel({
  tactics,
  onChange,
  onClose,
}: {
  tactics: TacticsInfo;
  onChange: (p: PlaystyleType, f: FocusType) => void;
  onClose: () => void;
}) {
  const currentPlaystyle = PLAYSTYLE_OPTIONS.find(
    (opt) => opt.value === tactics.playstyle,
  );
  const currentFocus = FOCUS_OPTIONS.find((opt) => opt.value === tactics.focus);

  return (
    <div
      className="absolute inset-2 z-20 flex items-center justify-center"
      onClick={onClose}
    >
      <div
        className="absolute inset-0 rounded-3xl"
        style={{
          backgroundColor: "rgba(10,10,15,0.78)",
          backdropFilter: "blur(10px)",
        }}
      />
      <div
        className="relative app-panel-strong w-full max-w-4xl rounded-[28px] p-4 md:p-5"
        onClick={(event) => event.stopPropagation()}
        style={{
          background:
            "linear-gradient(180deg, rgba(20,20,31,0.98) 0%, rgba(14,14,22,0.98) 100%)",
          boxShadow: "0 20px 50px rgba(0,0,0,0.35)",
        }}
      >
        <div className="flex items-start justify-between gap-3">
          <div className="min-w-0">
            <div
              className="app-eyebrow"
              style={{ color: "#06B6D4", letterSpacing: "0.18em" }}
            >
              Match Paused
            </div>
            <div
              className="mt-1 text-lg font-bold"
              style={{ color: "var(--text-primary)" }}
            >
              Tactical Adjustment
            </div>
            <div
              className="mt-1 text-xs"
              style={{ color: "var(--text-secondary)" }}
            >
              Choose a new playstyle and focus, then close the panel to resume.
            </div>
          </div>
          <button
            onClick={onClose}
            className="app-icon-button h-9 w-9 rounded-xl flex items-center justify-center cursor-pointer transition-all duration-150 glow-hover"
            style={{
              color: "var(--text-primary)",
            }}
          >
            <X size={16} />
          </button>
        </div>
        <div className="grid gap-3 md:grid-cols-2 mt-4">
          <div
            className="app-panel rounded-2xl p-3"
            style={{
              backgroundColor: "rgba(255,255,255,0.02)",
            }}
          >
            <div
              className="text-[0.62rem] font-semibold uppercase"
              style={{ color: "var(--text-muted)", letterSpacing: "0.16em" }}
            >
              Playstyle
            </div>
            <div className="grid gap-2 mt-3">
              {PLAYSTYLE_OPTIONS.map((opt) => {
                const isSelected = tactics.playstyle === opt.value;

                return (
                  <button
                    key={opt.value}
                    onClick={() => onChange(opt.value, tactics.focus)}
                    className="w-full rounded-xl border px-3 py-3 text-left cursor-pointer transition-all duration-150"
                    style={{
                      borderColor: isSelected
                        ? opt.color
                        : "rgba(255,255,255,0.08)",
                      background: isSelected
                        ? `linear-gradient(135deg, ${opt.color}20, rgba(255,255,255,0.02))`
                        : "rgba(255,255,255,0.02)",
                      boxShadow: isSelected
                        ? `0 0 0 1px ${opt.color}18 inset`
                        : "none",
                    }}
                  >
                    <div className="flex items-center justify-between gap-3">
                      <div
                        className="text-sm font-bold"
                        style={{
                          color: isSelected ? opt.color : "var(--text-primary)",
                        }}
                      >
                        {opt.title}
                      </div>
                      <div
                        className="rounded-full px-2 py-0.5 text-[0.6rem] font-bold uppercase shrink-0"
                        style={{
                          color: isSelected ? opt.color : "var(--text-muted)",
                          backgroundColor: isSelected
                            ? `${opt.color}18`
                            : "rgba(255,255,255,0.04)",
                        }}
                      >
                        {opt.label}
                      </div>
                    </div>
                    <div
                      className="mt-1.5 text-xs"
                      style={{
                        color: isSelected
                          ? "var(--text-primary)"
                          : "var(--text-secondary)",
                      }}
                    >
                      {opt.description}
                    </div>
                  </button>
                );
              })}
            </div>
          </div>
          <div
            className="app-panel rounded-2xl p-3"
            style={{
              backgroundColor: "rgba(255,255,255,0.02)",
            }}
          >
            <div
              className="text-[0.62rem] font-semibold uppercase"
              style={{ color: "var(--text-muted)", letterSpacing: "0.16em" }}
            >
              Focus
            </div>
            <div className="grid gap-2 mt-3">
              {FOCUS_OPTIONS.map((opt) => {
                const isSelected = tactics.focus === opt.value;

                return (
                  <button
                    key={opt.value}
                    onClick={() => onChange(tactics.playstyle, opt.value)}
                    className="w-full rounded-xl border px-3 py-3 text-left cursor-pointer transition-all duration-150"
                    style={{
                      borderColor: isSelected
                        ? opt.color
                        : "rgba(255,255,255,0.08)",
                      background: isSelected
                        ? `linear-gradient(135deg, ${opt.color}20, rgba(255,255,255,0.02))`
                        : "rgba(255,255,255,0.02)",
                      boxShadow: isSelected
                        ? `0 0 0 1px ${opt.color}18 inset`
                        : "none",
                    }}
                  >
                    <div className="flex items-center justify-between gap-3">
                      <div
                        className="text-sm font-bold"
                        style={{
                          color: isSelected ? opt.color : "var(--text-primary)",
                        }}
                      >
                        {opt.title}
                      </div>
                      <div
                        className="rounded-full px-2 py-0.5 text-[0.6rem] font-bold uppercase shrink-0"
                        style={{
                          color: isSelected ? opt.color : "var(--text-muted)",
                          backgroundColor: isSelected
                            ? `${opt.color}18`
                            : "rgba(255,255,255,0.04)",
                        }}
                      >
                        {opt.label}
                      </div>
                    </div>
                    <div
                      className="mt-1.5 text-xs"
                      style={{
                        color: isSelected
                          ? "var(--text-primary)"
                          : "var(--text-secondary)",
                      }}
                    >
                      {opt.description}
                    </div>
                  </button>
                );
              })}
            </div>
          </div>
        </div>
        <div
          className="mt-4 pt-3 border-t flex items-center justify-between gap-3 flex-wrap"
          style={{ borderColor: "rgba(255,255,255,0.08)" }}
        >
          <div className="flex items-center gap-2 flex-wrap">
            <div
              className="rounded-full px-2.5 py-1 text-[0.62rem] font-bold uppercase"
              style={{
                color: currentPlaystyle?.color ?? "var(--text-primary)",
                backgroundColor: `${currentPlaystyle?.color ?? "#06B6D4"}16`,
              }}
            >
              {currentPlaystyle?.title ?? "Playstyle"}
            </div>
            <div
              className="rounded-full px-2.5 py-1 text-[0.62rem] font-bold uppercase"
              style={{
                color: currentFocus?.color ?? "var(--text-primary)",
                backgroundColor: `${currentFocus?.color ?? "#10B981"}16`,
              }}
            >
              {currentFocus?.title ?? "Focus"}
            </div>
          </div>
          <button
            onClick={onClose}
            className="app-button-primary px-4 py-2 rounded-2xl font-bold text-sm text-white cursor-pointer transition-all duration-150"
            style={{
              background: "var(--accent-gradient)",
              boxShadow: "0 0 16px rgba(6,182,212,0.22)",
            }}
          >
            Resume Match
          </button>
        </div>
      </div>
    </div>
  );
}

function EventRow({ event, isNew }: { event: MatchEventInfo; isNew: boolean }) {
  const color = eventColor(event.kind);

  return (
    <div
      className="flex items-start gap-2 px-3 py-2 rounded-2xl transition-all duration-300"
      style={{
        backgroundColor: isNew ? `${color}12` : "transparent",
        borderLeft: isNew ? `2px solid ${color}` : "2px solid transparent",
      }}
    >
      <div
        className="w-10 text-right text-xs tabular-nums font-bold shrink-0 pt-1"
        style={{ color: "var(--text-muted)" }}
      >
        {formatMinute(event.minute)}
      </div>
      <div className="shrink-0 pt-1" style={{ color }}>
        {eventIcon(event.kind)}
      </div>
      <div
        className="flex-1 flex flex-col gap-1"
        style={{ color: "var(--text-primary)" }}
      >
        <div className="flex items-center gap-2 flex-wrap">
          <span
            className="px-2 py-0.5 rounded-full text-[0.65rem] font-bold uppercase tracking-[0.12em]"
            style={{
              color: phaseBadgeColor(event.phase),
              backgroundColor: `${phaseBadgeColor(event.phase)}18`,
            }}
          >
            {event.phase}
          </span>
        </div>
        <div className="text-medium">{event.commentary ?? event.kind}</div>
      </div>
    </div>
  );
}
