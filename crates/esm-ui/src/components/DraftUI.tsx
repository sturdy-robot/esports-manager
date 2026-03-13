import { useState, useEffect, useCallback } from "react";
import { Lock, CheckCircle, Clock, Shield, Swords } from "lucide-react";
import type { DraftSessionState } from "@/lib/api";
import { DraftChampionGrid } from "./DraftChampionGrid";
import { SwapPhaseUI } from "./SwapPhaseUI";
import { DraftTeamPanel } from "./DraftTeamPanel";

/**
 * Custom hook: countdown timer that resets when `step` changes.
 * All state mutations happen inside async callbacks (setTimeout / setInterval),
 * never synchronously in the effect body, to satisfy react-compiler purity rules.
 */
function useDraftTimer(totalSeconds: number, step: number, active: boolean) {
  const [timeLeft, setTimeLeft] = useState(totalSeconds);

  useEffect(() => {
    const resetId = setTimeout(() => setTimeLeft(totalSeconds), 0);
    if (!active) return () => clearTimeout(resetId);
    const id = setInterval(() => {
      setTimeLeft((prev) => Math.max(0, prev - 1));
    }, 1000);
    return () => {
      clearTimeout(resetId);
      clearInterval(id);
    };
  }, [active, step, totalSeconds]);

  return timeLeft;
}

interface DraftUIProps {
  draftState: DraftSessionState;
  playerSide: "blue" | "red";
  teamName: string;
  opponentName: string;
  onHover: (champion: string) => void;
  onLock: () => void;
  onComplete: () => void;
  onSwap?: (a: number, b: number) => Promise<void>;
}

export function DraftUI({
  draftState,
  playerSide,
  teamName,
  opponentName,
  onHover,
  onLock,
  onComplete,
  onSwap,
}: DraftUIProps) {
  const blueName =
    draftState.blue_team_name ||
    (playerSide === "blue" ? teamName : opponentName);
  const redName =
    draftState.red_team_name ||
    (playerSide === "red" ? teamName : opponentName);

  const isBan = draftState.current_phase === "Ban";
  const phaseLabel = draftState.is_complete
    ? "Draft Complete"
    : isBan
      ? "Ban Phase"
      : "Pick Phase";

  const timeLeft = useDraftTimer(
    draftState.timer_seconds,
    draftState.current_step,
    !draftState.is_complete,
  );

  const handleAutoLock = useCallback(() => {
    if (!draftState.is_player_turn || draftState.is_complete) return;
    if (draftState.active_hover) {
      onLock();
    } else if (draftState.available_champions.length > 0) {
      onHover(draftState.available_champions[0]);
      setTimeout(() => onLock(), 50);
    }
  }, [draftState, onLock, onHover]);

  useEffect(() => {
    if (
      timeLeft === 0 &&
      draftState.is_player_turn &&
      !draftState.is_complete
    ) {
      handleAutoLock();
    }
  }, [
    timeLeft,
    draftState.is_player_turn,
    draftState.is_complete,
    handleAutoLock,
  ]);

  const timerColor =
    timeLeft <= 5
      ? "#EF4444"
      : timeLeft <= 10
        ? "#F59E0B"
        : "var(--text-secondary)";
  const timerPct =
    draftState.timer_seconds > 0
      ? (timeLeft / draftState.timer_seconds) * 100
      : 0;

  const isBlueActive = draftState.current_team === "Blue";
  const isRedActive = draftState.current_team === "Red";
  const currentTurnLabel = draftState.is_complete
    ? "Draft locked"
    : draftState.current_team
      ? `${draftState.current_team} on the clock`
      : "Waiting for next action";

  return (
    <div className="flex w-full flex-1 min-h-0 gap-2 p-2">
      {/* ---- Blue side panel ---- */}
      <TeamPanel
        side="blue"
        teamName={blueName}
        players={draftState.blue_players}
        picks={draftState.blue_picks}
        bans={draftState.blue_bans}
        isActive={isBlueActive}
        championDetails={draftState.champion_details}
      />

      {/* ---- Center area ---- */}
      <div className="flex flex-col flex-1 min-w-0 gap-2">
        {/* Header: phase + timer */}
        <div
          className="rounded-2xl border overflow-hidden shadow-lg shrink-0"
          style={{
            background: isBan
              ? "linear-gradient(180deg, rgba(239,68,68,0.14) 0%, rgba(20,20,31,0.96) 24%, rgba(20,20,31,1) 100%)"
              : "linear-gradient(180deg, rgba(6,182,212,0.14) 0%, rgba(20,20,31,0.96) 24%, rgba(20,20,31,1) 100%)",
            borderColor: "var(--border-subtle)",
            boxShadow: isBan
              ? "0 10px 30px rgba(239,68,68,0.12)"
              : "0 10px 30px rgba(6,182,212,0.12)",
          }}
        >
          <div
            className="px-4 py-3 border-b"
            style={{
              borderColor: isBan
                ? "rgba(239,68,68,0.16)"
                : "rgba(6,182,212,0.16)",
              background: isBan
                ? "linear-gradient(135deg, rgba(239,68,68,0.15), rgba(255,255,255,0.02))"
                : "linear-gradient(135deg, rgba(6,182,212,0.15), rgba(255,255,255,0.02))",
            }}
          >
            <div className="flex items-center justify-between gap-3 flex-wrap">
              <div className="flex items-center gap-3 min-w-0">
                <div
                  className="h-10 w-10 rounded-xl border flex items-center justify-center shrink-0"
                  style={{
                    borderColor: isBan
                      ? "rgba(239,68,68,0.35)"
                      : "rgba(6,182,212,0.35)",
                    backgroundColor: isBan
                      ? "rgba(239,68,68,0.14)"
                      : "rgba(6,182,212,0.14)",
                    color: isBan ? "#EF4444" : "var(--color-accent-cyan)",
                  }}
                >
                  {isBan ? <Shield size={18} /> : <Swords size={18} />}
                </div>
                <div className="min-w-0">
                  <div
                    className="text-[0.62rem] font-semibold uppercase"
                    style={{
                      color: "var(--text-muted)",
                      letterSpacing: "0.18em",
                    }}
                  >
                    Draft Room
                  </div>
                  <div
                    className="text-sm font-bold"
                    style={{ color: "var(--text-primary)" }}
                  >
                    {phaseLabel}
                  </div>
                </div>
              </div>
              <div
                className="rounded-full px-2.5 py-1 text-[0.62rem] font-bold uppercase tracking-[0.16em] shrink-0"
                style={{
                  color: isBan ? "#FCA5A5" : "var(--color-accent-cyan)",
                  backgroundColor: isBan
                    ? "rgba(239,68,68,0.14)"
                    : "rgba(6,182,212,0.14)",
                  border: `1px solid ${isBan ? "rgba(239,68,68,0.28)" : "rgba(6,182,212,0.28)"}`,
                }}
              >
                {draftState.current_step}/{draftState.total_steps}
              </div>
            </div>
          </div>

          <div className="px-4 py-3 flex items-center justify-between gap-3 flex-wrap">
            <div className="flex items-center gap-2 flex-wrap">
              <div
                className="rounded-full px-2.5 py-1 text-[0.62rem] font-bold uppercase tracking-[0.16em]"
                style={{
                  color: draftState.is_player_turn
                    ? "#10B981"
                    : "var(--text-secondary)",
                  backgroundColor: draftState.is_player_turn
                    ? "rgba(16,185,129,0.14)"
                    : "rgba(255,255,255,0.04)",
                }}
              >
                {draftState.is_player_turn ? "Your turn" : "Stand by"}
              </div>
              <div
                className="rounded-full px-2.5 py-1 text-[0.62rem] font-bold uppercase tracking-[0.16em]"
                style={{
                  color: "var(--text-secondary)",
                  backgroundColor: "rgba(255,255,255,0.04)",
                }}
              >
                {currentTurnLabel}
              </div>
            </div>

            {!draftState.is_complete ? (
              <div className="flex items-center gap-3">
                <div className="flex items-center gap-2 shrink-0">
                  <Clock size={14} style={{ color: timerColor }} />
                  <span
                    className="text-lg tabular-nums font-bold font-display"
                    style={{ color: timerColor }}
                  >
                    {timeLeft}s
                  </span>
                </div>
                <div
                  className="h-1.5 rounded-full overflow-hidden shrink-0"
                  style={{
                    backgroundColor: "var(--bg-elevated)",
                    width: "128px",
                  }}
                >
                  <div
                    className="h-full rounded-full transition-all duration-1000 ease-linear"
                    style={{
                      width: `${timerPct}%`,
                      background: isBan
                        ? "linear-gradient(90deg, #EF4444, #F87171)"
                        : "linear-gradient(90deg, #06B6D4, #67E8F9)",
                    }}
                  />
                </div>
              </div>
            ) : (
              <div
                className="rounded-full px-2.5 py-1 text-[0.62rem] font-bold uppercase tracking-[0.16em]"
                style={{
                  color: "var(--color-win)",
                  backgroundColor: "rgba(16,185,129,0.14)",
                }}
              >
                All picks locked
              </div>
            )}
          </div>
        </div>

        {/* Swap panel (shown when draft is complete) */}
        {draftState.is_complete && onSwap && (
          <SwapPhaseUI
            draftState={draftState}
            playerSide={playerSide}
            onSwap={onSwap}
            onConfirm={onComplete}
          />
        )}

        {/* Champion grid with filters */}
        {!draftState.is_complete && (
          <DraftChampionGrid
            availableChampions={draftState.available_champions}
            activeHover={draftState.active_hover}
            isPlayerTurn={draftState.is_player_turn}
            championDetails={draftState.champion_details}
            onHover={onHover}
          />
        )}

        {/* Lock / Complete button */}
        {!draftState.is_complete || !onSwap ? (
          <div
            className="rounded-xl border px-4 py-3 shrink-0 flex items-center justify-between gap-3 flex-wrap"
            style={{
              backgroundColor: "var(--bg-surface)",
              borderColor: "var(--border-subtle)",
            }}
          >
            <div className="flex flex-col min-w-0">
              <span
                className="text-[0.62rem] font-semibold uppercase"
                style={{ color: "var(--text-muted)", letterSpacing: "0.16em" }}
              >
                Action Desk
              </span>
              <span
                className="text-sm font-semibold truncate"
                style={{
                  color: draftState.active_hover
                    ? "var(--text-primary)"
                    : "var(--text-secondary)",
                }}
              >
                {draftState.is_complete
                  ? "Draft phase ready to continue"
                  : draftState.active_hover
                    ? `${isBan ? "Ban" : "Pick"} ${draftState.active_hover}`
                    : draftState.is_player_turn
                      ? "Select a champion to hover"
                      : "Waiting for opponent..."}
              </span>
            </div>
            {draftState.is_complete ? (
              <button
                onClick={onComplete}
                className="px-8 py-3 rounded-lg font-bold text-white flex items-center gap-2 transition-all duration-150"
                style={{
                  background: "linear-gradient(135deg, #10B981, #06B6D4)",
                  boxShadow: "0 0 16px rgba(6,182,212,0.3)",
                }}
              >
                <CheckCircle size={18} />
                Continue
              </button>
            ) : draftState.active_hover ? (
              <button
                aria-label="Lock In"
                onClick={onLock}
                className="px-8 py-3 rounded-lg font-bold text-white flex items-center gap-2 transition-all duration-150"
                style={{
                  background: isBan
                    ? "linear-gradient(135deg, #EF4444, #F87171)"
                    : "linear-gradient(135deg, #10B981, #06B6D4)",
                  boxShadow: isBan
                    ? "0 0 16px rgba(239,68,68,0.3)"
                    : "0 0 16px rgba(6,182,212,0.3)",
                }}
              >
                {isBan ? <Shield size={18} /> : <Lock size={18} />}
                {isBan ? "Ban" : "Pick"}
              </button>
            ) : (
              <div
                className="px-3 py-2 rounded-lg text-sm font-semibold"
                style={{
                  color: "var(--text-muted)",
                  backgroundColor: "rgba(255,255,255,0.03)",
                }}
              >
                Awaiting selection
              </div>
            )}
          </div>
        ) : null}
      </div>

      {/* ---- Red side panel ---- */}
      <TeamPanel
        side="red"
        teamName={redName}
        players={draftState.red_players}
        picks={draftState.red_picks}
        bans={draftState.red_bans}
        isActive={isRedActive}
        championDetails={draftState.champion_details}
      />
    </div>
  );
}

// ---------------------------------------------------------------------------
// Sub-components
// ---------------------------------------------------------------------------

const TeamPanel = DraftTeamPanel;
