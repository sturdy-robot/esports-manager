import { useState } from "react";
import {
  Calendar,
  Swords,
  Gamepad2,
  Plus,
  X,
  ChevronDown,
  AlertTriangle,
} from "lucide-react";
import type {
  WeekScheduleInfo,
  ScheduleSlotInfo,
  TimeSlotType,
  SoloQueueFocusType,
  DraftRulesType,
  ScrimInfo,
} from "@/lib/api";

// ---------------------------------------------------------------------------
// Props
// ---------------------------------------------------------------------------

interface TeamScheduleViewProps {
  schedule: WeekScheduleInfo | null;
  scrims: ScrimInfo[];
  rosterNames: string[];
  onScheduleScrim: (
    dayIndex: number,
    timeSlot: TimeSlotType,
    awayTeamIndex: number,
    gameCount: number,
    draftRules: DraftRulesType
  ) => Promise<void>;
  onScheduleSoloQueue: (
    dayIndex: number,
    timeSlot: TimeSlotType,
    players: number[],
    focus: SoloQueueFocusType
  ) => Promise<void>;
  onClearSlot: (dayIndex: number, timeSlot: TimeSlotType) => Promise<void>;
  onCancelScrim: (scrimId: number) => Promise<void>;
  teamNames: string[];
  playerTeamName?: string;
}

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const TIME_SLOTS: TimeSlotType[] = ["Morning", "Afternoon", "Evening"];
const FOCUS_OPTIONS: SoloQueueFocusType[] = [
  "champions",
  "tactics",
  "mechanics",
  "mentality",
];

// ---------------------------------------------------------------------------
// Slot cell component
// ---------------------------------------------------------------------------

function SlotCell({
  slot,
  dayIndex,
  timeSlot,
  onAdd,
  onClear,
  scrimResult,
  blockedReason,
  canClear,
}: {
  slot: ScheduleSlotInfo;
  dayIndex: number;
  timeSlot: TimeSlotType;
  onAdd: (dayIndex: number, timeSlot: TimeSlotType) => void;
  onClear: (dayIndex: number, timeSlot: TimeSlotType) => void;
  scrimResult?: { homeWins: number; awayWins: number; completed: boolean };
  blockedReason?: string;
  canClear: boolean;
}) {
  if (slot.entry_type === "free") {
    if (blockedReason) {
      return (
        <div
          className="w-full h-full min-h-[56px] flex items-center justify-center rounded border border-dashed"
          style={{
            borderColor: "var(--border-subtle)",
            backgroundColor: "rgba(255,255,255,0.02)",
            color: "var(--text-muted)",
          }}
        >
          <span className="text-[10px] font-semibold uppercase tracking-wide">
            {blockedReason}
          </span>
        </div>
      );
    }

    return (
      <button
        onClick={() => onAdd(dayIndex, timeSlot)}
        className="w-full h-full min-h-[56px] flex items-center justify-center rounded border border-dashed transition-all cursor-pointer group"
        style={{
          borderColor: "var(--border-subtle)",
          backgroundColor: "transparent",
          color: "var(--text-muted)",
        }}
        onMouseEnter={(e) => {
          e.currentTarget.style.borderColor = "var(--color-accent-cyan)";
          e.currentTarget.style.backgroundColor = "rgba(6, 182, 212, 0.05)";
        }}
        onMouseLeave={(e) => {
          e.currentTarget.style.borderColor = "var(--border-subtle)";
          e.currentTarget.style.backgroundColor = "transparent";
        }}
      >
        <Plus size={14} className="opacity-0 group-hover:opacity-100 transition-opacity" />
      </button>
    );
  }

  const bgColor =
    slot.entry_type === "scrim"
      ? "rgba(6, 182, 212, 0.12)"
      : slot.entry_type === "match"
        ? "rgba(251, 191, 36, 0.12)"
      : slot.entry_type === "solo_queue"
        ? "rgba(16, 185, 129, 0.12)"
        : "rgba(255,255,255,0.02)";

  const borderColor =
    slot.entry_type === "scrim"
      ? "var(--color-accent-cyan)"
      : slot.entry_type === "match"
        ? "var(--color-warning)"
      : slot.entry_type === "solo_queue"
        ? "var(--color-accent-emerald)"
        : "var(--border-subtle)";

  const icon =
    slot.entry_type === "scrim" ? (
      <Swords size={14} />
    ) : slot.entry_type === "match" ? (
      <Calendar size={14} />
    ) : slot.entry_type === "solo_queue" ? (
      <Gamepad2 size={14} />
    ) : null;

  const label =
    slot.entry_type === "scrim"
      ? `vs ${slot.opponent ?? "?"}`
      : slot.entry_type === "match"
        ? `Match vs ${slot.opponent ?? "?"}`
      : slot.entry_type === "solo_queue"
        ? `SoloQ · ${slot.focus ?? ""}`
        : "";

  const detail =
    slot.entry_type === "match"
      ? "Official series"
      :
    slot.entry_type === "scrim" && scrimResult?.completed
      ? null
      : slot.entry_type === "solo_queue" && slot.players
        ? `${slot.players.length} player${slot.players.length !== 1 ? "s" : ""}`
        : null;

  const scrimScore =
    slot.entry_type === "scrim" && scrimResult?.completed
      ? scrimResult
      : null;

  return (
    <div
      className="relative w-full min-h-[56px] flex flex-col justify-center px-2 py-1.5 rounded border transition-all group"
      style={{
        backgroundColor: bgColor,
        borderColor: borderColor,
        borderLeftWidth: "3px",
      }}
    >
      {canClear && (
        <button
          onClick={() => onClear(dayIndex, timeSlot)}
          className="absolute top-1 right-1 opacity-0 group-hover:opacity-100 transition-opacity w-5 h-5 flex items-center justify-center rounded cursor-pointer border-none"
          style={{
            backgroundColor: "var(--bg-elevated)",
            color: "var(--text-muted)",
          }}
        >
          <X size={10} />
        </button>
      )}
      <div className="flex items-center gap-1.5">
        <span style={{ color: borderColor }}>{icon}</span>
        <span
          className="text-xs font-semibold truncate"
          style={{ color: "var(--text-primary)" }}
        >
          {label}
        </span>
      </div>
      {detail && (
        <span
          className="text-[10px] mt-0.5"
          style={{ color: "var(--text-muted)" }}
        >
          {detail}
        </span>
      )}
      {scrimScore && (
        <span
          className="text-[10px] tabular-nums font-bold mt-0.5"
          style={{
            color: scrimScore.homeWins > scrimScore.awayWins
              ? "var(--color-win)"
              : "var(--color-loss)",
          }}
        >
          {scrimScore.homeWins}–{scrimScore.awayWins}
        </span>
      )}
    </div>
  );
}

// ---------------------------------------------------------------------------
// Add slot modal
// ---------------------------------------------------------------------------

type AddMode = "scrim" | "solo_queue";

function AddSlotModal({
  dayIndex,
  timeSlot,
  rosterNames,
  teamNames,
  playerTeamName,
  onClose,
  onScheduleScrim,
  onScheduleSoloQueue,
}: {
  dayIndex: number;
  timeSlot: TimeSlotType;
  rosterNames: string[];
  teamNames: string[];
  playerTeamName?: string;
  onClose: () => void;
  onScheduleScrim: (
    awayTeamIndex: number,
    gameCount: number,
    draftRules: DraftRulesType
  ) => Promise<void>;
  onScheduleSoloQueue: (
    players: number[],
    focus: SoloQueueFocusType
  ) => Promise<void>;
}) {
  const [mode, setMode] = useState<AddMode | null>(null);
  const [awayTeamIdx, setAwayTeamIdx] = useState(() => {
    const firstOpponent = teamNames.findIndex((n) => n !== playerTeamName);
    return firstOpponent >= 0 ? firstOpponent : 0;
  });
  const [gameCount, setGameCount] = useState(3);
  const [draftRules, setDraftRules] = useState<DraftRulesType>("standard");
  const [selectedPlayers, setSelectedPlayers] = useState<number[]>([]);
  const [focus, setFocus] = useState<SoloQueueFocusType>("mechanics");
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async () => {
    setSubmitting(true);
    setError(null);
    try {
      if (mode === "scrim") {
        await onScheduleScrim(awayTeamIdx, gameCount, draftRules);
      } else if (mode === "solo_queue") {
        await onScheduleSoloQueue(selectedPlayers, focus);
      }
      onClose();
    } catch (e) {
      setError(String(e));
    } finally {
      setSubmitting(false);
    }
  };

  const togglePlayer = (idx: number) => {
    setSelectedPlayers((prev) =>
      prev.includes(idx) ? prev.filter((p) => p !== idx) : [...prev, idx]
    );
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center"
      style={{ backgroundColor: "rgba(0,0,0,0.6)" }}
      onClick={onClose}
    >
      <div
        className="rounded-xl p-5 w-[400px] max-w-[90vw] animate-fade-in-up"
        style={{
          backgroundColor: "var(--bg-surface)",
          border: "1px solid var(--border-subtle)",
        }}
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between mb-4">
          <h3
            className="text-sm font-bold"
            style={{ color: "var(--text-primary)" }}
          >
            Day {dayIndex + 1} · {timeSlot}
          </h3>
          <button
            onClick={onClose}
            className="w-6 h-6 flex items-center justify-center rounded cursor-pointer border-none"
            style={{
              backgroundColor: "var(--bg-elevated)",
              color: "var(--text-muted)",
            }}
          >
            <X size={14} />
          </button>
        </div>

        {error && (
          <div
            className="flex items-center gap-2 px-3 py-2 rounded-lg mb-3 text-xs"
            style={{
              backgroundColor: "rgba(239, 68, 68, 0.1)",
              border: "1px solid rgba(239, 68, 68, 0.3)",
              color: "var(--color-loss)",
            }}
          >
            <AlertTriangle size={14} className="shrink-0" />
            <span>{error}</span>
          </div>
        )}

        {!mode && (
          <div className="flex flex-col gap-2">
            <button
              onClick={() => setMode("scrim")}
              className="flex items-center gap-3 p-3 rounded-lg border cursor-pointer transition-colors"
              style={{
                backgroundColor: "var(--bg-elevated)",
                borderColor: "var(--border-subtle)",
                color: "var(--text-primary)",
              }}
              onMouseEnter={(e) => {
                e.currentTarget.style.borderColor = "var(--color-accent-cyan)";
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.borderColor = "var(--border-subtle)";
              }}
            >
              <Swords size={18} style={{ color: "var(--color-accent-cyan)" }} />
              <div className="text-left">
                <div className="text-sm font-semibold">Scrim</div>
                <div
                  className="text-xs"
                  style={{ color: "var(--text-muted)" }}
                >
                  Practice match vs another team
                </div>
              </div>
            </button>
            <button
              onClick={() => setMode("solo_queue")}
              className="flex items-center gap-3 p-3 rounded-lg border cursor-pointer transition-colors"
              style={{
                backgroundColor: "var(--bg-elevated)",
                borderColor: "var(--border-subtle)",
                color: "var(--text-primary)",
              }}
              onMouseEnter={(e) => {
                e.currentTarget.style.borderColor =
                  "var(--color-accent-emerald)";
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.borderColor = "var(--border-subtle)";
              }}
            >
              <Gamepad2
                size={18}
                style={{ color: "var(--color-accent-emerald)" }}
              />
              <div className="text-left">
                <div className="text-sm font-semibold">Solo Queue</div>
                <div
                  className="text-xs"
                  style={{ color: "var(--text-muted)" }}
                >
                  Individual practice session
                </div>
              </div>
            </button>
          </div>
        )}

        {mode === "scrim" && (
          <div className="flex flex-col gap-3">
            <div>
              <label
                className="text-xs font-semibold mb-1 block"
                style={{ color: "var(--text-secondary)" }}
              >
                Opponent
              </label>
              <div className="relative">
                <select
                  value={awayTeamIdx}
                  onChange={(e) => setAwayTeamIdx(Number(e.target.value))}
                  className="w-full p-2 rounded text-sm appearance-none cursor-pointer"
                  style={{
                    backgroundColor: "var(--bg-elevated)",
                    color: "var(--text-primary)",
                    border: "1px solid var(--border-subtle)",
                  }}
                >
                  {teamNames.map((name, i) => {
                    if (playerTeamName && name === playerTeamName) return null;
                    return (
                      <option key={i} value={i}>
                        {name}
                      </option>
                    );
                  })}
                </select>
                <ChevronDown
                  size={14}
                  className="absolute right-2 top-1/2 -translate-y-1/2 pointer-events-none"
                  style={{ color: "var(--text-muted)" }}
                />
              </div>
            </div>
            <div className="flex gap-3">
              <div className="flex-1">
                <label
                  className="text-xs font-semibold mb-1 block"
                  style={{ color: "var(--text-secondary)" }}
                >
                  Games
                </label>
                <div className="flex gap-1">
                  {[3, 5].map((n) => (
                    <button
                      key={n}
                      onClick={() => setGameCount(n)}
                      className="flex-1 py-1.5 rounded text-xs tabular-nums font-bold cursor-pointer border-none transition-colors"
                      style={{
                        backgroundColor:
                          gameCount === n
                            ? "var(--color-accent-cyan)"
                            : "var(--bg-elevated)",
                        color:
                          gameCount === n ? "#fff" : "var(--text-secondary)",
                      }}
                    >
                      {n} games
                    </button>
                  ))}
                </div>
              </div>
              <div className="flex-1">
                <label
                  className="text-xs font-semibold mb-1 block"
                  style={{ color: "var(--text-secondary)" }}
                >
                  Draft
                </label>
                <div className="flex gap-1">
                  {(["standard", "fearless"] as DraftRulesType[]).map((r) => (
                    <button
                      key={r}
                      onClick={() => setDraftRules(r)}
                      className="flex-1 py-1.5 rounded text-xs font-semibold cursor-pointer border-none transition-colors capitalize"
                      style={{
                        backgroundColor:
                          draftRules === r
                            ? "var(--color-accent-emerald)"
                            : "var(--bg-elevated)",
                        color:
                          draftRules === r ? "#fff" : "var(--text-secondary)",
                      }}
                    >
                      {r}
                    </button>
                  ))}
                </div>
              </div>
            </div>
            <button
              onClick={handleSubmit}
              disabled={submitting}
              className="w-full py-2 rounded-lg text-sm font-bold cursor-pointer border-none transition-colors"
              style={{
                background:
                  "linear-gradient(135deg, var(--color-accent-emerald), var(--color-accent-cyan))",
                color: "#fff",
                opacity: submitting ? 0.6 : 1,
              }}
            >
              {submitting ? "Scheduling…" : "Schedule Scrim"}
            </button>
          </div>
        )}

        {mode === "solo_queue" && (
          <div className="flex flex-col gap-3">
            <div>
              <label
                className="text-xs font-semibold mb-1 block"
                style={{ color: "var(--text-secondary)" }}
              >
                Players
              </label>
              <div className="flex flex-wrap gap-1.5">
                {rosterNames.map((name, i) => (
                  <button
                    key={i}
                    onClick={() => togglePlayer(i)}
                    className="px-2.5 py-1 rounded text-xs font-semibold cursor-pointer border-none transition-colors"
                    style={{
                      backgroundColor: selectedPlayers.includes(i)
                        ? "var(--color-accent-emerald)"
                        : "var(--bg-elevated)",
                      color: selectedPlayers.includes(i)
                        ? "#fff"
                        : "var(--text-secondary)",
                    }}
                  >
                    {name}
                  </button>
                ))}
              </div>
            </div>
            <div>
              <label
                className="text-xs font-semibold mb-1 block"
                style={{ color: "var(--text-secondary)" }}
              >
                Focus
              </label>
              <div className="flex gap-1">
                {FOCUS_OPTIONS.map((f) => (
                  <button
                    key={f}
                    onClick={() => setFocus(f)}
                    className="flex-1 py-1.5 rounded text-xs font-semibold cursor-pointer border-none transition-colors capitalize"
                    style={{
                      backgroundColor:
                        focus === f
                          ? "var(--color-accent-emerald)"
                          : "var(--bg-elevated)",
                      color: focus === f ? "#fff" : "var(--text-secondary)",
                    }}
                  >
                    {f}
                  </button>
                ))}
              </div>
            </div>
            <button
              onClick={handleSubmit}
              disabled={submitting || selectedPlayers.length === 0}
              className="w-full py-2 rounded-lg text-sm font-bold cursor-pointer border-none transition-colors"
              style={{
                background:
                  "linear-gradient(135deg, var(--color-accent-emerald), var(--color-accent-cyan))",
                color: "#fff",
                opacity: submitting || selectedPlayers.length === 0 ? 0.6 : 1,
              }}
            >
              {submitting ? "Scheduling…" : "Schedule Solo Queue"}
            </button>
          </div>
        )}

      </div>
    </div>
  );
}

// ---------------------------------------------------------------------------
// Main component
// ---------------------------------------------------------------------------

export function TeamScheduleView({
  schedule,
  scrims,
  rosterNames,
  onScheduleScrim,
  onScheduleSoloQueue,
  onClearSlot,
  onCancelScrim,
  teamNames,
  playerTeamName,
}: TeamScheduleViewProps) {
  const [addingSlot, setAddingSlot] = useState<{
    dayIndex: number;
    timeSlot: TimeSlotType;
  } | null>(null);
  const [actionError, setActionError] = useState<string | null>(null);

  const handleClearSlot = async (dayIndex: number, timeSlot: TimeSlotType) => {
    setActionError(null);
    try {
      await onClearSlot(dayIndex, timeSlot);
    } catch (e) {
      setActionError(String(e));
    }
  };

  const handleCancelScrim = async (scrimId: number) => {
    setActionError(null);
    try {
      await onCancelScrim(scrimId);
    } catch (e) {
      setActionError(String(e));
    }
  };

  if (!schedule) {
    return (
      <div className="flex flex-col items-center justify-center gap-3 py-16">
        <Calendar size={48} style={{ color: "var(--text-muted)" }} />
        <p style={{ color: "var(--text-muted)" }}>Loading schedule…</p>
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-4 animate-fade-in-up">
      {/* Error banner */}
      {actionError && (
        <div
          className="flex items-center justify-between gap-2 px-3 py-2 rounded-lg text-xs"
          style={{
            backgroundColor: "rgba(239, 68, 68, 0.1)",
            border: "1px solid rgba(239, 68, 68, 0.3)",
            color: "var(--color-loss)",
          }}
        >
          <div className="flex items-center gap-2">
            <AlertTriangle size={14} className="shrink-0" />
            <span>{actionError}</span>
          </div>
          <button
            onClick={() => setActionError(null)}
            className="shrink-0 w-5 h-5 flex items-center justify-center rounded cursor-pointer border-none"
            style={{ backgroundColor: "transparent", color: "var(--color-loss)" }}
          >
            <X size={12} />
          </button>
        </div>
      )}

      {/* Header stats */}
      <div className="flex items-center justify-between">
        <h2
          className="text-lg font-bold"
          style={{ color: "var(--text-primary)" }}
        >
          Team Calendar
        </h2>
        <div className="flex items-center gap-4">
          <div className="flex items-center gap-1.5">
            <Swords size={14} style={{ color: "var(--color-accent-cyan)" }} />
            <span
              className="text-sm tabular-nums font-bold"
              style={{ color: "var(--text-primary)" }}
            >
              {schedule.total_scrims}
            </span>
            <span
              className="text-xs"
              style={{ color: "var(--text-muted)" }}
            >
              scrims
            </span>
          </div>
          <div className="flex items-center gap-1.5">
            <Calendar size={14} style={{ color: "var(--color-warning)" }} />
            <span
              className="text-sm tabular-nums font-bold"
              style={{ color: "var(--text-primary)" }}
            >
              {schedule.total_matches}
            </span>
            <span
              className="text-xs"
              style={{ color: "var(--text-muted)" }}
            >
              matches
            </span>
          </div>
          <div className="flex items-center gap-1.5">
            <Calendar size={14} style={{ color: "var(--text-muted)" }} />
            <span
              className="text-sm tabular-nums font-bold"
              style={{ color: "var(--text-primary)" }}
            >
              {schedule.occupied_slots}
            </span>
            <span className="text-xs" style={{ color: "var(--text-muted)" }}>
              / {schedule.days.length * 3} slots
            </span>
          </div>
        </div>
      </div>

      {/* Weekly grid */}
      <div
        className="rounded-xl border overflow-hidden"
        style={{
          backgroundColor: "var(--bg-surface)",
          borderColor: "var(--border-subtle)",
        }}
      >
        <div
          className="grid grid-cols-[180px_repeat(3,minmax(0,1fr))] border-b"
          style={{ borderColor: "var(--border-subtle)" }}
        >
          <div
            className="p-2 text-xs font-semibold"
            style={{ color: "var(--text-muted)" }}
          />
          {TIME_SLOTS.map((label) => (
            <div
              key={label}
              className="p-2 text-center text-xs font-bold uppercase tracking-wider"
              style={{ color: "var(--text-secondary)" }}
            >
              {label}
            </div>
          ))}
        </div>

        {schedule.days.map((day, rowIdx) => (
          <div
            key={day.day_index}
            className="grid grid-cols-[180px_repeat(3,minmax(0,1fr))]"
            style={{
              borderBottom:
                rowIdx < schedule.days.length - 1
                  ? "1px solid var(--border-subtle)"
                  : undefined,
            }}
          >
            <div
              className="p-3 flex flex-col justify-center"
              style={{ color: "var(--text-muted)" }}
            >
              <span className="text-xs font-bold uppercase tracking-wide">
                {day.day_label}
              </span>
              <span className="text-xs">{day.date_label}</span>
              {day.is_past && (
                <span className="text-[10px] mt-1" style={{ color: "var(--text-muted)" }}>
                  Passed
                </span>
              )}
              {!day.is_past && day.has_match && (
                <span className="text-[10px] mt-1" style={{ color: "var(--color-warning)" }}>
                  Match day
                </span>
              )}
            </div>
            {TIME_SLOTS.map((ts) => {
              const slot = day.slots.find((s) => s.time_slot === ts);
              if (!slot) return <div key={`${day.day_index}-${ts}`} className="p-1.5" />;
              const matchedScrim = slot.scrim_id != null
                ? scrims.find((s) => s.id === slot.scrim_id)
                : undefined;
              const scrimResult = matchedScrim
                ? { homeWins: matchedScrim.home_wins, awayWins: matchedScrim.away_wins, completed: matchedScrim.status === "Completed" }
                : undefined;
              const blockedReason = slot.entry_type === "free"
                ? day.is_past
                  ? "Past"
                  : day.has_match
                    ? "Match day"
                    : undefined
                : undefined;
              return (
                <div key={`${day.day_index}-${ts}`} className="p-1.5">
                  <SlotCell
                    slot={slot}
                    dayIndex={day.day_index}
                    timeSlot={ts}
                    onAdd={(d, t) => setAddingSlot({ dayIndex: d, timeSlot: t })}
                    onClear={handleClearSlot}
                    scrimResult={scrimResult}
                    blockedReason={blockedReason}
                    canClear={!day.is_past && slot.entry_type !== "match"}
                  />
                </div>
              );
            })}
          </div>
        ))}
      </div>

      {/* Legend */}
      <div className="flex items-center gap-4">
        <div className="flex items-center gap-1.5">
          <div
            className="w-3 h-3 rounded-sm"
            style={{ backgroundColor: "rgba(6, 182, 212, 0.3)" }}
          />
          <span className="text-xs" style={{ color: "var(--text-muted)" }}>
            Scrim
          </span>
        </div>
        <div className="flex items-center gap-1.5">
          <div
            className="w-3 h-3 rounded-sm"
            style={{ backgroundColor: "rgba(16, 185, 129, 0.3)" }}
          />
          <span className="text-xs" style={{ color: "var(--text-muted)" }}>
            Solo Queue
          </span>
        </div>
        <div className="flex items-center gap-1.5">
          <div
            className="w-3 h-3 rounded-sm"
            style={{ backgroundColor: "rgba(251, 191, 36, 0.3)" }}
          />
          <span className="text-xs" style={{ color: "var(--text-muted)" }}>
            Match
          </span>
        </div>
      </div>

      {/* Upcoming scrims list */}
      {scrims.length > 0 && (
        <div className="flex flex-col gap-2">
          <h3
            className="text-sm font-bold"
            style={{ color: "var(--text-primary)" }}
          >
            Scrims
          </h3>
          <div className="flex flex-col gap-1.5">
            {scrims.map((s) => (
              <div
                key={s.id}
                className="flex items-center justify-between p-3 rounded-lg border"
                style={{
                  backgroundColor: "var(--bg-elevated)",
                  borderColor: "var(--border-subtle)",
                }}
              >
                <div className="flex items-center gap-3">
                  <Swords
                    size={14}
                    style={{ color: "var(--color-accent-cyan)" }}
                  />
                  <div>
                    <span
                      className="text-sm font-semibold"
                      style={{ color: "var(--text-primary)" }}
                    >
                      vs {s.away_team}
                    </span>
                    <span
                      className="text-xs ml-2"
                      style={{ color: "var(--text-muted)" }}
                    >
                      Day {s.scheduled_day} · {s.time_slot} · {s.game_count} games
                    </span>
                  </div>
                </div>
                <div className="flex items-center gap-2">
                  {s.status === "Completed" && (
                    <span
                      className="text-xs tabular-nums font-bold px-2 py-0.5 rounded"
                      style={{
                        backgroundColor: s.home_wins > s.away_wins
                          ? "rgba(34, 197, 94, 0.15)"
                          : "rgba(239, 68, 68, 0.15)",
                        color: s.home_wins > s.away_wins
                          ? "var(--color-win)"
                          : "var(--color-loss)",
                      }}
                    >
                      {s.home_wins}–{s.away_wins}
                    </span>
                  )}
                  <span
                    className="text-xs tabular-nums px-2 py-0.5 rounded"
                    style={{
                      backgroundColor:
                        s.status === "Scheduled"
                          ? "rgba(6, 182, 212, 0.15)"
                          : s.status === "Completed"
                            ? "rgba(34, 197, 94, 0.15)"
                            : "rgba(239, 68, 68, 0.15)",
                      color:
                        s.status === "Scheduled"
                          ? "var(--color-accent-cyan)"
                          : s.status === "Completed"
                            ? "var(--color-win)"
                            : "var(--color-loss)",
                    }}
                  >
                    {s.status}
                  </span>
                  {s.status === "Scheduled" && (
                    <button
                      onClick={() => handleCancelScrim(s.id)}
                      className="w-6 h-6 flex items-center justify-center rounded cursor-pointer border-none transition-colors"
                      style={{
                        backgroundColor: "var(--bg-surface)",
                        color: "var(--text-muted)",
                      }}
                      onMouseEnter={(e) => {
                        e.currentTarget.style.color = "var(--color-loss)";
                      }}
                      onMouseLeave={(e) => {
                        e.currentTarget.style.color = "var(--text-muted)";
                      }}
                    >
                      <X size={12} />
                    </button>
                  )}
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Add slot modal */}
      {addingSlot && (
        <AddSlotModal
          dayIndex={addingSlot.dayIndex}
          timeSlot={addingSlot.timeSlot}
          rosterNames={rosterNames}
          teamNames={teamNames}
          playerTeamName={playerTeamName}
          onClose={() => setAddingSlot(null)}
          onScheduleScrim={(awayIdx, gc, dr) =>
            onScheduleScrim(
              addingSlot.dayIndex,
              addingSlot.timeSlot,
              awayIdx,
              gc,
              dr
            )
          }
          onScheduleSoloQueue={(players, focus) =>
            onScheduleSoloQueue(
              addingSlot.dayIndex,
              addingSlot.timeSlot,
              players,
              focus
            )
          }
        />
      )}
    </div>
  );
}
