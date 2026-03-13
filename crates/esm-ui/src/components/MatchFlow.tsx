import { useState, useEffect, useCallback } from "react";
import { DraftUI } from "./DraftUI";
import { MatchSimUI } from "./MatchSimUI";
import { TacticsPanel } from "./TacticsPanel";
import { useDraft, useMatchSimulation, useTactics } from "@/lib/use-api";
import type { MatchMode } from "./PlayMatchButton";
import type { PlaystyleType, FocusType } from "@/lib/api";

export type MatchPhase =
  | "pre-match"
  | "draft"
  | "tactics"
  | "match"
  | "simulating"
  | "results";

interface MatchFlowProps {
  mode: MatchMode;
  teamName: string;
  opponentName: string;
  teamSide: "blue" | "red";
  fearlessBans?: string[];
  onComplete: () => void;
  onPhaseChange?: (phase: MatchPhase) => void;
}

function initialPhase(mode: MatchMode): MatchPhase {
  switch (mode) {
    case "delegate":
      return "simulating";
    case "participate":
    case "spectate":
    case "draft-delegate":
      return "pre-match";
  }
}

export function MatchFlow({
  mode,
  teamName,
  opponentName,
  teamSide,
  fearlessBans = [],
  onComplete,
  onPhaseChange,
}: MatchFlowProps) {
  const [phase, setPhase] = useState<MatchPhase>(() => initialPhase(mode));
  const { draftState, startDraft, hover, lock, autoDraftComplete, swapPicks } =
    useDraft();
  const { result: matchResult, simulate } = useMatchSimulation();
  const { tactics, update: updateTactics } = useTactics();

  useEffect(() => {
    onPhaseChange?.(phase);
  }, [phase, onPhaseChange]);

  // For simulating phase, run the backend simulation then advance
  useEffect(() => {
    if (phase === "simulating") {
      simulate().then(() => setPhase("results"));
    }
  }, [phase, simulate]);

  const handleProceedToDraft = useCallback(async () => {
    await startDraft({
      player_side: teamSide,
      format: fearlessBans.length > 0 ? "fearless" : "five_ban",
      fearless_bans: fearlessBans,
      blue_team: teamSide === "blue" ? teamName : opponentName,
      red_team: teamSide === "red" ? teamName : opponentName,
    });
    if (mode === "spectate") {
      await autoDraftComplete();
      setPhase("tactics");
    } else {
      setPhase("draft");
    }
  }, [
    startDraft,
    autoDraftComplete,
    mode,
    teamSide,
    teamName,
    opponentName,
    fearlessBans,
  ]);

  const handleDraftHover = useCallback(
    async (champion: string) => {
      await hover(champion);
    },
    [hover],
  );

  const handleDraftLock = useCallback(async () => {
    await lock();
  }, [lock]);

  const handleDraftComplete = useCallback(() => {
    if (mode === "draft-delegate") {
      setPhase("simulating");
    } else {
      setPhase("tactics");
    }
  }, [mode]);

  const handleSwap = useCallback(
    async (a: number, b: number) => {
      await swapPicks(a, b);
    },
    [swapPicks],
  );

  const handleTacticsConfirm = useCallback(
    async (playstyle: PlaystyleType, focus: FocusType) => {
      await updateTactics(playstyle, focus);
      simulate().then(() => setPhase("match"));
    },
    [updateTactics, simulate],
  );

  const handleMatchComplete = () => {
    setPhase("results");
  };

  return (
    <div
      className="flex flex-col flex-1 min-h-0"
      style={{ backgroundColor: "var(--bg-base)" }}
    >
      {/* Main content — full-height phases vs centered panels */}
      {phase === "draft" || phase === "match" || phase === "tactics" ? (
        <main className="flex-1 flex flex-col min-h-0">
          {phase === "draft" && draftState && (
            <DraftUI
              draftState={draftState}
              playerSide={teamSide}
              teamName={teamName}
              opponentName={opponentName}
              onHover={handleDraftHover}
              onLock={handleDraftLock}
              onComplete={handleDraftComplete}
              onSwap={mode === "participate" ? handleSwap : undefined}
            />
          )}

          {phase === "draft" && !draftState && (
            <div className="flex-1 flex items-center justify-center">
              <div className="app-panel-strong w-full max-w-md rounded-[28px] p-8 text-center">
                <div
                  className="w-16 h-16 mx-auto mb-4 rounded-full animate-pulse"
                  style={{ background: "var(--accent-gradient)" }}
                />
                <div className="app-eyebrow mb-2">Draft Room</div>
                <p
                  className="text-sm"
                  style={{ color: "var(--text-secondary)" }}
                >
                  Loading draft...
                </p>
              </div>
            </div>
          )}

          {phase === "tactics" && (
            <div className="flex-1 flex items-center justify-center p-8">
              <TacticsPanel
                tactics={tactics}
                onConfirm={handleTacticsConfirm}
                context="Pre-Match Strategy"
              />
            </div>
          )}

          {phase === "match" && matchResult && (
            <MatchSimUI
              result={matchResult}
              onComplete={handleMatchComplete}
              tactics={tactics}
              onTacticsChange={(p, f) => updateTactics(p, f)}
            />
          )}

          {phase === "match" && !matchResult && (
            <div className="flex-1 flex items-center justify-center">
              <SimulatingPanel />
            </div>
          )}
        </main>
      ) : (
        <main className="flex-1 flex items-center justify-center p-8">
          {phase === "pre-match" && (
            <PreMatchPanel
              teamName={teamName}
              opponentName={opponentName}
              teamSide={teamSide}
              onProceed={handleProceedToDraft}
            />
          )}

          {phase === "simulating" && <SimulatingPanel />}

          {phase === "results" && (
            <ResultsPanel
              teamName={teamName}
              opponentName={opponentName}
              onExit={onComplete}
            />
          )}
        </main>
      )}
    </div>
  );
}

// ---------------------------------------------------------------------------
// Sub-panels
// ---------------------------------------------------------------------------

function PreMatchPanel({
  teamName,
  opponentName,
  teamSide,
  onProceed,
}: {
  teamName: string;
  opponentName: string;
  teamSide: "blue" | "red";
  onProceed: () => void;
}) {
  return (
    <div
      className="app-panel-strong w-full max-w-2xl p-8 rounded-[28px] text-center"
      style={{
        backgroundColor: "var(--bg-surface)",
      }}
    >
      <div className="app-eyebrow mb-2">Match Operations</div>
      <h2
        className="text-2xl font-bold mb-2 font-display"
        style={{ color: "var(--text-primary)" }}
      >
        Pre-Match
      </h2>
      <div className="flex items-center justify-center gap-3 mb-3">
        <span
          className="text-lg font-bold font-display"
          style={{ color: "var(--text-primary)" }}
        >
          {teamName}
        </span>
        <span
          className="text-xs font-bold uppercase tracking-widest"
          style={{ color: "var(--text-muted)" }}
        >
          vs
        </span>
        <span
          className="text-lg font-bold font-display"
          style={{ color: "var(--text-primary)" }}
        >
          {opponentName}
        </span>
      </div>
      <p className="text-sm mb-2" style={{ color: "var(--text-secondary)" }}>
        Prepare your strategy before the series begins.
      </p>
      <div
        className="w-full items-center mb-6"
        style={{ color: "var(--text-secondary)" }}
      >
        Your side:
        <span
          className="text-xs font-semibold uppercase tracking-wider px-3 py-1 rounded-full ml-2 inline-flex"
          style={{
            backgroundColor:
              teamSide === "blue"
                ? "rgba(59, 130, 246, 0.12)"
                : "rgba(239, 68, 68, 0.12)",
            color: teamSide === "blue" ? "#93C5FD" : "#FCA5A5",
          }}
        >
          {teamSide} Side
        </span>
      </div>

      <button
        onClick={onProceed}
        className="app-button-primary px-6 py-2.5 rounded-2xl text-sm font-semibold cursor-pointer border-none transition-all"
        style={{
          background: "var(--accent-gradient)",
          color: "#fff",
        }}
      >
        Proceed to Draft
      </button>
    </div>
  );
}

function SimulatingPanel() {
  return (
    <div className="app-panel-strong w-full max-w-md rounded-[28px] p-8 text-center">
      <div
        className="w-16 h-16 mx-auto mb-4 rounded-full animate-pulse"
        style={{ background: "var(--accent-gradient)" }}
      />
      <div className="app-eyebrow mb-2">Simulation</div>
      <h2
        className="text-xl font-bold mb-2 font-display"
        style={{ color: "var(--text-primary)" }}
      >
        Simulating...
      </h2>
      <p className="text-sm" style={{ color: "var(--text-secondary)" }}>
        Your assistant coach is handling the match.
      </p>
    </div>
  );
}

function ResultsPanel({
  teamName,
  opponentName,
  onExit,
}: {
  teamName: string;
  opponentName: string;
  onExit: () => void;
}) {
  return (
    <div
      className="app-panel-strong w-full max-w-2xl p-8 rounded-[28px] text-center"
      style={{
        backgroundColor: "var(--bg-surface)",
      }}
    >
      <div className="app-eyebrow mb-2">Post Match</div>
      <h2
        className="text-2xl font-bold mb-2 font-display"
        style={{ color: "var(--text-primary)" }}
      >
        Match Results
      </h2>
      <p className="text-sm mb-6" style={{ color: "var(--text-secondary)" }}>
        {teamName} vs {opponentName}
      </p>
      <button
        onClick={onExit}
        className="app-button-primary px-6 py-2.5 rounded-2xl text-sm font-semibold cursor-pointer border-none transition-all"
        style={{
          background: "var(--accent-gradient)",
          color: "#fff",
        }}
      >
        Return to Dashboard
      </button>
    </div>
  );
}
