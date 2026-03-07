import { useState, useEffect, useCallback } from 'react';
import { DraftUI } from './DraftUI';
import { MatchSimUI } from './MatchSimUI';
import { TacticsPanel } from './TacticsPanel';
import { useDraft, useMatchSimulation, useTactics } from '@/lib/use-api';
import type { MatchMode } from './PlayMatchButton';
import type { PlaystyleType, FocusType } from '@/lib/api';

type MatchPhase = 'pre-match' | 'draft' | 'tactics' | 'match' | 'simulating' | 'results';

interface MatchFlowProps {
  mode: MatchMode;
  teamName: string;
  opponentName: string;
  teamSide: 'blue' | 'red';
  fearlessBans?: string[];
  onComplete: () => void;
}

function initialPhase(mode: MatchMode): MatchPhase {
  switch (mode) {
    case 'delegate':
      return 'simulating';
    case 'participate':
    case 'spectate':
    case 'draft-delegate':
      return 'pre-match';
  }
}

export function MatchFlow({
  mode,
  teamName,
  opponentName,
  teamSide,
  fearlessBans = [],
  onComplete,
}: MatchFlowProps) {
  const [phase, setPhase] = useState<MatchPhase>(() => initialPhase(mode));
  const { draftState, startDraft, hover, lock, autoDraftComplete, swapPicks } = useDraft();
  const { result: matchResult, simulate } = useMatchSimulation();
  const { tactics, update: updateTactics } = useTactics();

  // For simulating phase, run the backend simulation then advance
  useEffect(() => {
    if (phase === 'simulating') {
      simulate().then(() => setPhase('results'));
    }
  }, [phase, simulate]);

  const handleProceedToDraft = useCallback(async () => {
    await startDraft({
      player_side: teamSide,
      format: fearlessBans.length > 0 ? 'fearless' : 'five_ban',
      fearless_bans: fearlessBans,
      blue_team: teamSide === 'blue' ? teamName : opponentName,
      red_team: teamSide === 'red' ? teamName : opponentName,
    });
    if (mode === 'spectate') {
      await autoDraftComplete();
      setPhase('tactics');
    } else {
      setPhase('draft');
    }
  }, [startDraft, autoDraftComplete, mode, teamSide, teamName, opponentName, fearlessBans]);

  const handleDraftHover = useCallback(async (champion: string) => {
    await hover(champion);
  }, [hover]);

  const handleDraftLock = useCallback(async () => {
    await lock();
  }, [lock]);

  const handleDraftComplete = useCallback(() => {
    if (mode === 'draft-delegate') {
      setPhase('simulating');
    } else {
      setPhase('tactics');
    }
  }, [mode]);

  const handleSwap = useCallback(async (a: number, b: number) => {
    await swapPicks(a, b);
  }, [swapPicks]);


  const handleTacticsConfirm = useCallback(async (playstyle: PlaystyleType, focus: FocusType) => {
    await updateTactics(playstyle, focus);
    simulate().then(() => setPhase('match'));
  }, [updateTactics, simulate]);

  const handleMatchComplete = () => {
    setPhase('results');
  };

  return (
    <div
      className="flex flex-col flex-1 min-h-0"
      style={{ backgroundColor: 'var(--bg-base)' }}
    >
      {/* Match header — replaces sidebar/topbar */}
      <header
        className="flex items-center justify-between h-16 px-8 border-b shrink-0"
        style={{
          borderColor: 'var(--border-subtle)',
          backgroundColor: 'var(--bg-surface)',
        }}
      >
        <div className="flex items-center gap-4">
          <div className="flex items-center gap-3">
            <span
              className="text-lg font-bold"
              style={{ color: 'var(--text-primary)' }}
            >
              {teamName}
            </span>
            <span
              className="text-xs font-semibold px-2 py-0.5 rounded"
              style={{
                backgroundColor: teamSide === 'blue' ? 'rgba(6,182,212,0.15)' : 'rgba(239,68,68,0.15)',
                color: teamSide === 'blue' ? 'var(--color-accent-cyan)' : 'var(--color-loss)',
              }}
            >
              {teamSide === 'blue' ? 'Blue Side' : 'Red Side'}
            </span>
          </div>
          <span className="text-2xl font-black" style={{ color: 'var(--border-subtle)' }}>
            VS
          </span>
          <span
            className="text-lg font-bold"
            style={{ color: 'var(--text-primary)' }}
          >
            {opponentName}
          </span>
        </div>

        <div
          className="px-3 py-1 rounded-md text-xs font-semibold uppercase tracking-wider"
          style={{
            backgroundColor: 'var(--bg-elevated)',
            color: 'var(--text-secondary)',
          }}
        >
          {phase === 'pre-match' && 'Pre-Match'}
          {phase === 'draft' && 'Draft Phase'}
          {phase === 'tactics' && 'Tactics'}
          {phase === 'match' && 'Live Match'}
          {phase === 'simulating' && 'Simulating...'}
          {phase === 'results' && 'Match Results'}
        </div>
      </header>

      {/* Main content — full-height phases vs centered panels */}
      {(phase === 'draft' || phase === 'match' || phase === 'tactics') ? (
        <main className="flex-1 flex flex-col min-h-0">
          {phase === 'draft' && draftState && (
            <DraftUI
              draftState={draftState}
              playerSide={teamSide}
              teamName={teamName}
              opponentName={opponentName}
              onHover={handleDraftHover}
              onLock={handleDraftLock}
              onComplete={handleDraftComplete}
              onSwap={mode === 'participate' ? handleSwap : undefined}
            />
          )}

          {phase === 'draft' && !draftState && (
            <div className="flex-1 flex items-center justify-center">
              <div className="text-center">
                <div
                  className="w-16 h-16 mx-auto mb-4 rounded-full animate-pulse"
                  style={{ background: 'linear-gradient(135deg, var(--color-accent-cyan), var(--color-accent-violet))' }}
                />
                <p className="text-sm" style={{ color: 'var(--text-secondary)' }}>
                  Loading draft...
                </p>
              </div>
            </div>
          )}

          {phase === 'tactics' && (
            <div className="flex-1 flex items-center justify-center p-8">
              <TacticsPanel
                tactics={tactics}
                onConfirm={handleTacticsConfirm}
                context="Pre-Match Strategy"
              />
            </div>
          )}

          {phase === 'match' && matchResult && (
            <MatchSimUI
              result={matchResult}
              onComplete={handleMatchComplete}
              tactics={tactics}
              onTacticsChange={(p, f) => updateTactics(p, f)}
            />
          )}

          {phase === 'match' && !matchResult && (
            <div className="flex-1 flex items-center justify-center">
              <SimulatingPanel />
            </div>
          )}
        </main>
      ) : (
        <main className="flex-1 flex items-center justify-center p-8">
          {phase === 'pre-match' && (
            <PreMatchPanel
              teamName={teamName}
              opponentName={opponentName}
              onProceed={handleProceedToDraft}
            />
          )}

          {phase === 'simulating' && (
            <SimulatingPanel />
          )}

          {phase === 'results' && (
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
  onProceed,
}: {
  teamName: string;
  opponentName: string;
  onProceed: () => void;
}) {
  return (
    <div
      className="w-full max-w-2xl p-8 rounded-xl border text-center"
      style={{
        backgroundColor: 'var(--bg-surface)',
        borderColor: 'var(--border-subtle)',
      }}
    >
      <h2
        className="text-2xl font-bold mb-2"
        style={{ color: 'var(--text-primary)' }}
      >
        Pre-Match
      </h2>
      <p className="text-sm mb-6" style={{ color: 'var(--text-secondary)' }}>
        {teamName} vs {opponentName} — prepare your strategy.
      </p>
      <button
        onClick={onProceed}
        className="px-6 py-2.5 rounded-md text-sm font-semibold cursor-pointer border-none transition-all"
        style={{
          background: 'linear-gradient(135deg, var(--color-accent-cyan), var(--color-accent-violet))',
          color: '#fff',
        }}
      >
        Proceed to Draft
      </button>
    </div>
  );
}

function SimulatingPanel() {
  return (
    <div className="text-center">
      <div
        className="w-16 h-16 mx-auto mb-4 rounded-full animate-pulse"
        style={{ background: 'linear-gradient(135deg, var(--color-accent-cyan), var(--color-accent-violet))' }}
      />
      <h2
        className="text-xl font-bold mb-2"
        style={{ color: 'var(--text-primary)' }}
      >
        Simulating...
      </h2>
      <p className="text-sm" style={{ color: 'var(--text-secondary)' }}>
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
      className="w-full max-w-2xl p-8 rounded-xl border text-center"
      style={{
        backgroundColor: 'var(--bg-surface)',
        borderColor: 'var(--border-subtle)',
      }}
    >
      <h2
        className="text-2xl font-bold mb-2"
        style={{ color: 'var(--text-primary)' }}
      >
        Match Results
      </h2>
      <p className="text-sm mb-6" style={{ color: 'var(--text-secondary)' }}>
        {teamName} vs {opponentName}
      </p>
      <button
        onClick={onExit}
        className="px-6 py-2.5 rounded-md text-sm font-semibold cursor-pointer border-none transition-all"
        style={{
          background: 'linear-gradient(135deg, var(--color-accent-cyan), var(--color-accent-violet))',
          color: '#fff',
        }}
      >
        Return to Dashboard
      </button>
    </div>
  );
}
