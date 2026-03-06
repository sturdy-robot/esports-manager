import { useState, useEffect, useCallback } from 'react';
import { MatchFlow } from './MatchFlow';
import { useSeriesInfo, useDraft } from '@/lib/use-api';
import type { MatchMode } from './PlayMatchButton';
import type { SeriesInfo } from '@/lib/api';

type SeriesPhase = 'loading' | 'game' | 'between-games' | 'series-complete';

interface SeriesFlowProps {
  mode: MatchMode;
  teamName: string;
  opponentName: string;
  teamSide: 'blue' | 'red';
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
  const [phase, setPhase] = useState<SeriesPhase>('loading');
  const [gameKey, setGameKey] = useState(0);
  const { series, refresh } = useSeriesInfo();
  const { refresh: refreshDraft } = useDraft();
  const [fearlessBans, setFearlessBans] = useState<string[]>([]);

  // Fetch series info on mount
  useEffect(() => {
    refresh().then((info) => {
      if (info) {
        setPhase('game');
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
      setPhase('series-complete');
    } else {
      setPhase('between-games');
    }
  }, [refresh, refreshDraft]);

  const handleNextGame = useCallback(() => {
    setGameKey((k) => k + 1);
    setPhase('game');
  }, []);

  if (phase === 'loading' || !series) {
    return (
      <div
        className="flex flex-col min-h-screen items-center justify-center"
        style={{ backgroundColor: 'var(--bg-base)' }}
      >
        <div
          className="w-16 h-16 mx-auto mb-4 rounded-full animate-pulse"
          style={{ background: 'linear-gradient(135deg, var(--color-accent-cyan), var(--color-accent-violet))' }}
        />
        <p className="text-sm" style={{ color: 'var(--text-secondary)' }}>
          Loading series...
        </p>
      </div>
    );
  }

  return (
    <div className="flex flex-col min-h-screen" style={{ backgroundColor: 'var(--bg-base)' }}>
      {/* Series banner */}
      <SeriesBanner series={series} />

      {/* Current phase content */}
      <div className="flex-1 flex flex-col">
        {phase === 'game' && (
          <MatchFlow
            key={gameKey}
            mode={mode}
            teamName={teamName}
            opponentName={opponentName}
            teamSide={teamSide}
            fearlessBans={fearlessBans}
            onComplete={handleGameComplete}
          />
        )}

        {phase === 'between-games' && (
          <BetweenGamesPanel
            series={series}
            onNextGame={handleNextGame}
          />
        )}

        {phase === 'series-complete' && (
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

function SeriesBanner({ series }: { series: SeriesInfo }) {
  return (
    <div
      className="flex items-center justify-center gap-6 py-2 border-b shrink-0"
      style={{
        backgroundColor: 'var(--bg-elevated)',
        borderColor: 'var(--border-subtle)',
      }}
    >
      <span className="text-xs font-semibold uppercase tracking-wider"
        style={{ color: 'var(--text-muted)' }}>
        {formatBo(series.wins_needed)}
      </span>

      <div className="flex items-center gap-3">
        <span className="font-bold text-sm" style={{ color: '#3B82F6' }}>
          {series.blue_team}
        </span>
        <div className="flex items-center gap-1.5">
          <ScoreBox value={series.blue_wins} highlight={series.blue_wins >= series.wins_needed} />
          <span className="text-xs font-bold" style={{ color: 'var(--text-muted)' }}>-</span>
          <ScoreBox value={series.red_wins} highlight={series.red_wins >= series.wins_needed} />
        </div>
        <span className="font-bold text-sm" style={{ color: '#EF4444' }}>
          {series.red_team}
        </span>
      </div>

      <span className="text-xs font-mono font-semibold px-2 py-0.5 rounded"
        style={{
          backgroundColor: 'rgba(6,182,212,0.1)',
          color: 'var(--color-accent-cyan)',
        }}>
        Game {series.game_number}
      </span>
    </div>
  );
}

function ScoreBox({ value, highlight }: { value: number; highlight: boolean }) {
  return (
    <span
      className="w-7 h-7 flex items-center justify-center rounded font-mono font-black text-sm"
      style={{
        backgroundColor: highlight
          ? 'var(--color-win)'
          : 'var(--bg-surface)',
        color: highlight ? '#fff' : 'var(--text-primary)',
      }}
    >
      {value}
    </span>
  );
}

function BetweenGamesPanel({
  series,
  onNextGame,
}: {
  series: SeriesInfo;
  onNextGame: () => void;
}) {
  return (
    <div className="flex-1 flex items-center justify-center p-8">
      <div
        className="w-full max-w-lg p-8 rounded-xl border text-center"
        style={{
          backgroundColor: 'var(--bg-surface)',
          borderColor: 'var(--border-subtle)',
        }}
      >
        <h2
          className="text-2xl font-bold mb-4"
          style={{ color: 'var(--text-primary)' }}
        >
          Game {series.game_number - 1} Complete
        </h2>

        <div className="flex items-center justify-center gap-6 mb-6">
          <div className="text-center">
            <div className="text-sm font-bold mb-1" style={{ color: '#3B82F6' }}>
              {series.blue_team}
            </div>
            <div className="text-3xl font-mono font-black" style={{ color: 'var(--text-primary)' }}>
              {series.blue_wins}
            </div>
          </div>
          <span className="text-2xl font-black" style={{ color: 'var(--border-subtle)' }}>-</span>
          <div className="text-center">
            <div className="text-sm font-bold mb-1" style={{ color: '#EF4444' }}>
              {series.red_team}
            </div>
            <div className="text-3xl font-mono font-black" style={{ color: 'var(--text-primary)' }}>
              {series.red_wins}
            </div>
          </div>
        </div>

        <p className="text-sm mb-6" style={{ color: 'var(--text-secondary)' }}>
          First to {series.wins_needed} wins takes the series.
        </p>

        <button
          onClick={onNextGame}
          className="px-8 py-3 rounded-lg font-bold text-white"
          style={{
            background: 'linear-gradient(135deg, #06B6D4, #8B5CF6)',
          }}
        >
          Start Game {series.game_number}
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
  const winner = series.blue_wins > series.red_wins ? series.blue_team : series.red_team;
  const playerWon = winner === playerTeam;

  return (
    <div className="flex-1 flex items-center justify-center p-8">
      <div
        className="w-full max-w-lg p-8 rounded-xl border text-center"
        style={{
          backgroundColor: 'var(--bg-surface)',
          borderColor: 'var(--border-subtle)',
        }}
      >
        <div
          className="text-xs font-semibold uppercase tracking-wider mb-2"
          style={{ color: playerWon ? 'var(--color-win)' : 'var(--color-loss)' }}
        >
          {playerWon ? 'Series Victory' : 'Series Defeat'}
        </div>

        <h2
          className="text-3xl font-bold mb-4"
          style={{ color: 'var(--text-primary)' }}
        >
          {winner} Wins!
        </h2>

        <div className="flex items-center justify-center gap-6 mb-6">
          <div className="text-center">
            <div className="text-sm font-bold mb-1" style={{ color: '#3B82F6' }}>
              {series.blue_team}
            </div>
            <div className="text-3xl font-mono font-black" style={{ color: 'var(--text-primary)' }}>
              {series.blue_wins}
            </div>
          </div>
          <span className="text-2xl font-black" style={{ color: 'var(--border-subtle)' }}>-</span>
          <div className="text-center">
            <div className="text-sm font-bold mb-1" style={{ color: '#EF4444' }}>
              {series.red_team}
            </div>
            <div className="text-3xl font-mono font-black" style={{ color: 'var(--text-primary)' }}>
              {series.red_wins}
            </div>
          </div>
        </div>

        <button
          onClick={onExit}
          className="px-8 py-3 rounded-lg font-bold text-white"
          style={{
            background: 'linear-gradient(135deg, #06B6D4, #8B5CF6)',
          }}
        >
          Return to Dashboard
        </button>
      </div>
    </div>
  );
}
