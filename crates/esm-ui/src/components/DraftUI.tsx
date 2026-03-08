import { useState, useEffect, useCallback } from 'react';
import { Lock, CheckCircle, Clock, Shield, Swords } from 'lucide-react';
import type { DraftSessionState } from '@/lib/api';
import { DraftChampionGrid } from './DraftChampionGrid';
import { SwapPhaseUI } from './SwapPhaseUI';
import { DraftTeamPanel } from './DraftTeamPanel';

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
  playerSide: 'blue' | 'red';
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
  const blueName = draftState.blue_team_name || (playerSide === 'blue' ? teamName : opponentName);
  const redName = draftState.red_team_name || (playerSide === 'red' ? teamName : opponentName);

  const isBan = draftState.current_phase === 'Ban';
  const phaseLabel = draftState.is_complete
    ? 'Draft Complete'
    : isBan
      ? 'Ban Phase'
      : 'Pick Phase';

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
    if (timeLeft === 0 && draftState.is_player_turn && !draftState.is_complete) {
      handleAutoLock();
    }
  }, [timeLeft, draftState.is_player_turn, draftState.is_complete, handleAutoLock]);

  const timerColor = timeLeft <= 5 ? '#EF4444' : timeLeft <= 10 ? '#F59E0B' : 'var(--text-secondary)';
  const timerPct = draftState.timer_seconds > 0 ? (timeLeft / draftState.timer_seconds) * 100 : 0;

  const isBlueActive = draftState.current_team === 'Blue';
  const isRedActive = draftState.current_team === 'Red';

  return (
    <div className="flex w-full flex-1 min-h-0 gap-3">
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
      <div className="flex flex-col flex-1 min-w-0 gap-3">
        {/* Header: phase + timer */}
        <div
          className="flex items-center justify-center px-6 py-3 rounded-xl border shrink-0"
          style={{ backgroundColor: 'var(--bg-surface)', borderColor: 'var(--border-subtle)' }}
        >
          <div className="text-center">
            <div className="flex items-center justify-center gap-3 mb-1">
              {isBan ? (
                <Shield size={16} style={{ color: '#EF4444' }} />
              ) : (
                <Swords size={16} style={{ color: 'var(--color-accent-cyan)' }} />
              )}
              <span
                className="text-sm font-semibold uppercase tracking-wider"
                style={{ color: 'var(--text-secondary)' }}
              >
                {phaseLabel}
              </span>
              <span
                className="text-xs tabular-nums"
                style={{ color: 'var(--text-muted)' }}
              >
                {draftState.current_step}/{draftState.total_steps}
              </span>
            </div>
            {!draftState.is_complete && (
              <div className="flex items-center justify-center gap-2">
                <Clock size={14} style={{ color: timerColor }} />
                <span className="text-lg tabular-nums font-bold font-display" style={{ color: timerColor }}>
                  {timeLeft}s
                </span>
                <div
                  className="h-1 rounded-full overflow-hidden"
                  style={{ backgroundColor: 'var(--bg-elevated)', width: '100px' }}
                >
                  <div
                    className="h-full rounded-full transition-all duration-1000 ease-linear"
                    style={{ width: `${timerPct}%`, backgroundColor: timerColor }}
                  />
                </div>
              </div>
            )}
            {draftState.is_complete && (
              <div
                className="text-xs tabular-nums mt-1"
                style={{ color: 'var(--text-muted)' }}
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
        <div className="flex justify-center px-4 pb-1 shrink-0">
          {draftState.is_complete ? (
            <button
              onClick={onComplete}
              className="px-8 py-3 rounded-lg font-bold text-white flex items-center gap-2 transition-all duration-150"
              style={{
                background: 'linear-gradient(135deg, #10B981, #06B6D4)',
                boxShadow: '0 0 16px rgba(6,182,212,0.3)',
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
                background: isBan ? '#EF4444' : '#06B6D4',
                boxShadow: isBan ? '0 0 16px rgba(239,68,68,0.3)' : '0 0 16px rgba(6,182,212,0.3)',
              }}
            >
              {isBan ? <Shield size={18} /> : <Lock size={18} />}
              {isBan ? 'Ban' : 'Pick'}
            </button>
          ) : (
            <div
              className="px-8 py-3 text-sm font-semibold"
              style={{ color: 'var(--text-muted)' }}
            >
              {draftState.is_player_turn
                ? 'Select a champion to hover'
                : 'Waiting for opponent...'}
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

