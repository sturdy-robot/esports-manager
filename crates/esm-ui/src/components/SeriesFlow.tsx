import { useState, useEffect, useCallback } from "react";
import { MatchFlow } from "./MatchFlow";
import { PlayerTalksPanel } from "./PlayerTalksPanel";
import { useSeriesInfo, useDraft } from "@/lib/use-api";
import type { MatchMode } from "./PlayMatchButton";
import type { SeriesInfo } from "@/lib/api";
import type { MatchPhase } from "./MatchFlow";

type SeriesPhase = "loading" | "game" | "between-games" | "series-complete";

interface SeriesFlowProps {
  mode: MatchMode;
  teamName: string;
  opponentName: string;
  teamSide: "blue" | "red";
  onComplete: () => void;
}

function formatBo(winsNeeded: number): string {
  return `Best of ${winsNeeded * 2 - 1}`;
}

export function SeriesFlow({
  mode,
  teamName,
  opponentName,
  teamSide,
  onComplete,
}: SeriesFlowProps) {
  const [phase, setPhase] = useState<SeriesPhase>("loading");
  const [matchPhase, setMatchPhase] = useState<MatchPhase | null>(null);
  const [gameKey, setGameKey] = useState(0);
  const { series, refresh } = useSeriesInfo();
  const { refresh: refreshDraft } = useDraft();
  const [fearlessBans, setFearlessBans] = useState<string[]>([]);

  // Fetch series info on mount
  useEffect(() => {
    refresh().then((info) => {
      if (info) {
        setPhase("game");
      }
    });
  }, [refresh]);

  const handleGameComplete = useCallback(async () => {
    // Collect draft picks from the completed game for Fearless carry-over
    const draftState = await refreshDraft();
    if (draftState) {
      const gamePicks = [...draftState.blue_picks, ...draftState.red_picks];
      setFearlessBans((prev) => [...prev, ...gamePicks]);
    }

    // Refresh series info from backend
    const updated = await refresh();
    if (updated && updated.is_complete) {
      setPhase("series-complete");
      setMatchPhase(null);
    } else {
      setPhase("between-games");
      setMatchPhase(null);
    }
  }, [refresh, refreshDraft]);

  const handleNextGame = useCallback(() => {
    setGameKey((k) => k + 1);
    setPhase("game");
    setMatchPhase(null);
  }, []);

  // Derive real opponent name from backend series info
  const resolvedOpponent = series
    ? teamSide === "blue"
      ? series.red_team
      : series.blue_team
    : opponentName;

  if (phase === "loading" || !series) {
    return (
      <div
        className="flex flex-col min-h-screen items-center justify-center"
        style={{ backgroundColor: "var(--bg-base)" }}
      >
        <div
          className="w-16 h-16 mx-auto mb-4 rounded-full animate-pulse"
          style={{
            background:
              "linear-gradient(135deg, var(--color-accent-cyan), var(--color-accent-emerald))",
          }}
        />
        <p className="text-sm" style={{ color: "var(--text-secondary)" }}>
          Loading series...
        </p>
      </div>
    );
  }

  return (
    <div
      data-testid="series-flow-root"
      className="flex flex-col w-full h-screen"
      style={{ backgroundColor: "var(--bg-base)" }}
    >
      {/* Series banner */}
      <SeriesBanner series={series} matchPhase={matchPhase} />

      {/* Current phase content */}
      <div className="flex-1 flex flex-col min-h-0 overflow-hidden">
        {phase === "game" && (
          <MatchFlow
            key={gameKey}
            mode={mode}
            teamName={teamName}
            opponentName={resolvedOpponent}
            teamSide={teamSide}
            fearlessBans={fearlessBans}
            onComplete={handleGameComplete}
            onPhaseChange={setMatchPhase}
          />
        )}

        {phase === "between-games" && (
          <BetweenGamesPanel
            series={series}
            fearlessBans={fearlessBans}
            onNextGame={handleNextGame}
          />
        )}

        {phase === "series-complete" && (
          <SeriesCompletePanel
            series={series}
            playerTeam={teamName}
            onExit={onComplete}
          />
        )}
      </div>
    </div>
  );
}

// ---------------------------------------------------------------------------
// Sub-components
// ---------------------------------------------------------------------------

function SeriesBanner({
  series,
  matchPhase,
}: {
  series: SeriesInfo;
  matchPhase: MatchPhase | null;
}) {
  const status = getSeriesBannerStatus(matchPhase);

  return (
    <div
      className="flex flex-col items-center gap-2 py-2 border-b"
      style={{
        backgroundColor: "var(--bg-elevated)",
        borderColor: "var(--border-subtle)",
      }}
    >
      <div className="w-full max-w-5xl flex flex-col items-center gap-2 px-4">
        <span
          className="text-xs font-semibold uppercase tracking-wider"
          style={{ color: "var(--text-muted)" }}
        >
          {formatBo(series.wins_needed)}
        </span>

        <div className="w-full grid grid-cols-[minmax(0,1fr)_auto_minmax(0,1fr)] items-center gap-4">
          <span
            className="font-bold text-2xl text-right truncate"
            style={{ color: "#3B82F6" }}
          >
            {series.blue_team}
          </span>
          <div className="flex items-center justify-center gap-1.5">
            <ScoreBox
              value={series.blue_wins}
              highlight={series.blue_wins >= series.wins_needed}
            />
            <span
              className="text-xl font-bold"
              style={{ color: "var(--text-muted)" }}
            >
              -
            </span>
            <ScoreBox
              value={series.red_wins}
              highlight={series.red_wins >= series.wins_needed}
            />
          </div>
          <span
            className="font-bold text-2xl text-left truncate"
            style={{ color: "#EF4444" }}
          >
            {series.red_team}
          </span>
        </div>

        <div className="w-full flex items-center justify-center gap-2">
          <span
            className="text-xs tabular-nums font-semibold px-2 py-0.5 rounded"
            style={{
              backgroundColor: "rgba(6,182,212,0.1)",
              color: "var(--color-accent-cyan)",
            }}
          >
            Game {series.game_number}
          </span>
          {status && (
            <span
              className="text-xs font-semibold tracking-wider rounded px-2 py-0.5"
              style={{
                backgroundColor: status.backgroundColor,
                color: status.color,
              }}
            >
              {status.label}
            </span>
          )}
        </div>
      </div>
    </div>
  );
}

function getSeriesBannerStatus(matchPhase: MatchPhase | null): {
  label: string;
  backgroundColor: string;
  color: string;
} | null {
  if (matchPhase === "draft") {
    return {
      label: "Draft",
      backgroundColor: "rgba(239, 68, 68, 0.18)",
      color: "#FCA5A5",
    };
  }

  if (matchPhase === "match" || matchPhase === "simulating") {
    return {
      label: "In Progress",
      backgroundColor: "rgba(34, 197, 94, 0.18)",
      color: "#86EFAC",
    };
  }

  return null;
}

function ScoreBox({ value, highlight }: { value: number; highlight: boolean }) {
  return (
    <span
      className="w-7 h-7 flex items-center justify-center rounded tabular-nums font-black text-xl"
      style={{
        backgroundColor: highlight ? "var(--color-win)" : "var(--bg-surface)",
        color: highlight ? "#fff" : "var(--text-primary)",
      }}
    >
      {value}
    </span>
  );
}

function BetweenGamesPanel({
  series,
  fearlessBans,
  onNextGame,
}: {
  series: SeriesInfo;
  fearlessBans: string[];
  onNextGame: () => void;
}) {
  const [step, setStep] = useState<"summary" | "talks">("summary");

  if (step === "talks") {
    return (
      <div className="flex-1 flex items-center justify-center p-8 overflow-y-auto">
        <PlayerTalksPanel onDone={onNextGame} />
      </div>
    );
  }

  return (
    <div className="flex-1 flex items-center justify-center p-8">
      <div
        className="w-full max-w-lg p-8 rounded-xl border text-center"
        style={{
          backgroundColor: "var(--bg-surface)",
          borderColor: "var(--border-subtle)",
        }}
      >
        <h2
          className="text-2xl font-bold mb-4"
          style={{ color: "var(--text-primary)" }}
        >
          Game {series.game_number - 1} Complete
        </h2>

        <div className="flex items-center justify-center gap-6 mb-6">
          <div className="text-center">
            <div
              className="text-sm font-bold mb-1"
              style={{ color: "#3B82F6" }}
            >
              {series.blue_team}
            </div>
            <div
              className="text-3xl font-display tabular-nums font-black"
              style={{ color: "var(--text-primary)" }}
            >
              {series.blue_wins}
            </div>
          </div>
          <span
            className="text-2xl font-black"
            style={{ color: "var(--border-subtle)" }}
          >
            -
          </span>
          <div className="text-center">
            <div
              className="text-sm font-bold mb-1"
              style={{ color: "#EF4444" }}
            >
              {series.red_team}
            </div>
            <div
              className="text-3xl font-display tabular-nums font-black"
              style={{ color: "var(--text-primary)" }}
            >
              {series.red_wins}
            </div>
          </div>
        </div>

        <p className="text-sm mb-4" style={{ color: "var(--text-secondary)" }}>
          First to {series.wins_needed} wins takes the series.
        </p>

        {fearlessBans.length > 0 && (
          <div className="mb-6">
            <div
              className="text-xs font-semibold uppercase tracking-wider mb-2"
              style={{ color: "var(--text-muted)" }}
            >
              Fearless — Unavailable Champions ({fearlessBans.length})
            </div>
            <div className="flex flex-wrap gap-1.5 justify-center">
              {fearlessBans.map((champ) => (
                <span
                  key={champ}
                  className="text-xs px-2 py-0.5 rounded"
                  style={{
                    backgroundColor: "rgba(239,68,68,0.1)",
                    color: "var(--color-loss)",
                  }}
                >
                  {champ}
                </span>
              ))}
            </div>
          </div>
        )}

        <button
          onClick={() => setStep("talks")}
          className="px-8 py-3 rounded-lg font-bold text-white"
          style={{
            background: "linear-gradient(135deg, #10B981, #06B6D4)",
          }}
        >
          Talk to Players
        </button>
      </div>
    </div>
  );
}

function SeriesCompletePanel({
  series,
  playerTeam,
  onExit,
}: {
  series: SeriesInfo;
  playerTeam: string;
  onExit: () => void;
}) {
  const winner =
    series.blue_wins > series.red_wins ? series.blue_team : series.red_team;
  const playerWon = winner === playerTeam;

  return (
    <div className="flex-1 flex items-center justify-center p-8">
      <div
        className="w-full max-w-lg p-8 rounded-xl border text-center"
        style={{
          backgroundColor: "var(--bg-surface)",
          borderColor: "var(--border-subtle)",
        }}
      >
        <div
          className="text-xs font-semibold uppercase tracking-wider mb-2"
          style={{
            color: playerWon ? "var(--color-win)" : "var(--color-loss)",
          }}
        >
          {playerWon ? "Series Victory" : "Series Defeat"}
        </div>

        <h2
          className="text-3xl font-bold mb-4"
          style={{ color: "var(--text-primary)" }}
        >
          {winner} Wins!
        </h2>

        <div className="flex items-center justify-center gap-6 mb-6">
          <div className="text-center">
            <div
              className="text-sm font-bold mb-1"
              style={{ color: "#3B82F6" }}
            >
              {series.blue_team}
            </div>
            <div
              className="text-3xl font-display tabular-nums font-black"
              style={{ color: "var(--text-primary)" }}
            >
              {series.blue_wins}
            </div>
          </div>
          <span
            className="text-2xl font-black"
            style={{ color: "var(--border-subtle)" }}
          >
            -
          </span>
          <div className="text-center">
            <div
              className="text-sm font-bold mb-1"
              style={{ color: "#EF4444" }}
            >
              {series.red_team}
            </div>
            <div
              className="text-3xl font-display tabular-nums font-black"
              style={{ color: "var(--text-primary)" }}
            >
              {series.red_wins}
            </div>
          </div>
        </div>

        <button
          onClick={onExit}
          className="px-8 py-3 rounded-lg font-bold text-white"
          style={{
            background: "linear-gradient(135deg, #10B981, #06B6D4)",
          }}
        >
          Return to Dashboard
        </button>
      </div>
    </div>
  );
}
