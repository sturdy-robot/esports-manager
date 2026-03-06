import { useState, useEffect, useCallback } from 'react';
import { Lock, CheckCircle, Clock } from 'lucide-react';
import type { DraftSessionState } from '@/lib/api';

/**
 * Custom hook: countdown timer that resets when `step` changes.
 * All state mutations happen inside async callbacks (setTimeout / setInterval),
 * never synchronously in the effect body, to satisfy react-compiler purity rules.
 */
function useDraftTimer(totalSeconds: number, step: number, active: boolean) {
  const [timeLeft, setTimeLeft] = useState(totalSeconds);

  useEffect(() => {
    // Reset via microtask — avoids synchronous setState in effect body
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
}

export function DraftUI({
  draftState,
  playerSide,
  teamName,
  opponentName,
  onHover,
  onLock,
  onComplete,
}: DraftUIProps) {
  const blueName = playerSide === 'blue' ? teamName : opponentName;
  const redName = playerSide === 'red' ? teamName : opponentName;

  const phaseLabel = draftState.is_complete
    ? 'Draft Complete'
    : draftState.current_phase === 'Ban'
      ? 'Ban Phase'
      : 'Pick Phase';

  // ---- Draft Timer ----
  const timeLeft = useDraftTimer(
    draftState.timer_seconds,
    draftState.current_step,
    !draftState.is_complete,
  );

  // Auto-lock when timer expires on player's turn
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
    <div className="flex flex-col w-full flex-1 min-h-0 gap-4">
      {/* Header: team names + phase + progress */}
      <div
        className="flex items-center justify-between px-6 py-3 rounded-xl border"
        style={{ backgroundColor: 'var(--bg-surface)', borderColor: 'var(--border-subtle)' }}
      >
        <div
          data-testid="team-header-blue"
          data-active={isBlueActive ? 'true' : 'false'}
          className="text-lg font-bold px-4 py-2 rounded-lg transition-all duration-300"
          style={{
            color: '#3B82F6',
            backgroundColor: isBlueActive ? 'rgba(59,130,246,0.15)' : 'rgba(59,130,246,0.05)',
            boxShadow: isBlueActive ? '0 0 12px rgba(59,130,246,0.3)' : 'none',
            border: isBlueActive ? '1px solid rgba(59,130,246,0.4)' : '1px solid transparent',
          }}
        >
          {blueName}
        </div>

        <div className="text-center">
          <div
            className="text-sm font-semibold uppercase tracking-wider mb-1"
            style={{ color: 'var(--text-secondary)' }}
          >
            {phaseLabel}
          </div>
          <div className="flex items-center justify-center gap-2">
            <div
              className="text-xs font-mono"
              style={{ color: 'var(--text-muted)' }}
            >
              {draftState.current_step} / {draftState.total_steps}
            </div>
            {!draftState.is_complete && (
              <div className="flex items-center gap-1" style={{ color: timerColor }}>
                <Clock size={12} />
                <span className="text-sm font-mono font-bold">{timeLeft}s</span>
              </div>
            )}
          </div>
          {/* Timer bar */}
          {!draftState.is_complete && (
            <div
              className="mt-1 h-0.5 rounded-full overflow-hidden"
              style={{ backgroundColor: 'var(--bg-elevated)', width: '120px' }}
            >
              <div
                className="h-full rounded-full transition-all duration-1000 ease-linear"
                style={{
                  width: `${timerPct}%`,
                  backgroundColor: timerColor,
                }}
              />
            </div>
          )}
        </div>

        <div
          data-testid="team-header-red"
          data-active={isRedActive ? 'true' : 'false'}
          className="text-lg font-bold px-4 py-2 rounded-lg transition-all duration-300"
          style={{
            color: '#EF4444',
            backgroundColor: isRedActive ? 'rgba(239,68,68,0.15)' : 'rgba(239,68,68,0.05)',
            boxShadow: isRedActive ? '0 0 12px rgba(239,68,68,0.3)' : 'none',
            border: isRedActive ? '1px solid rgba(239,68,68,0.4)' : '1px solid transparent',
          }}
        >
          {redName}
        </div>
      </div>

      {/* Bans row */}
      <div
        className="flex items-center justify-between px-6 py-2 gap-4 rounded-lg border"
        style={{ backgroundColor: 'var(--bg-surface)', borderColor: 'var(--border-subtle)' }}
      >
        <BanSlots bans={draftState.blue_bans} maxBans={5} side="blue" />
        <div
          className="text-xs font-semibold uppercase tracking-wider"
          style={{ color: 'var(--text-muted)' }}
        >
          Bans
        </div>
        <BanSlots bans={draftState.red_bans} maxBans={5} side="red" />
      </div>

      {/* Picks row */}
      <div
        className="flex items-center justify-between px-6 py-2 gap-4 rounded-lg border"
        style={{ backgroundColor: 'var(--bg-surface)', borderColor: 'var(--border-subtle)' }}
      >
        <PickSlots picks={draftState.blue_picks} maxPicks={5} side="blue" />
        <div
          className="text-xs font-semibold uppercase tracking-wider"
          style={{ color: 'var(--text-muted)' }}
        >
          Picks
        </div>
        <PickSlots picks={draftState.red_picks} maxPicks={5} side="red" />
      </div>

      {/* Champion grid */}
      {!draftState.is_complete && (
        <div
          className="flex-1 rounded-xl border p-4 overflow-y-auto"
          style={{
            backgroundColor: 'var(--bg-surface)',
            borderColor: 'var(--border-subtle)',
          }}
        >
          <div className="grid grid-cols-5 sm:grid-cols-10 gap-2">
            {draftState.available_champions.map((champ) => {
              const isHovered = draftState.active_hover === champ;
              return (
                <button
                  key={champ}
                  aria-label={champ}
                  data-hovered={isHovered ? 'true' : 'false'}
                  disabled={!draftState.is_player_turn}
                  onClick={() => onHover(champ)}
                  className="flex flex-col items-center justify-center p-2 rounded-lg text-xs font-semibold transition-all duration-150 border"
                  style={{
                    backgroundColor: isHovered
                      ? 'rgba(6,182,212,0.15)'
                      : 'var(--bg-elevated)',
                    borderColor: isHovered
                      ? 'var(--color-accent-cyan)'
                      : 'var(--border-subtle)',
                    color: isHovered
                      ? 'var(--color-accent-cyan)'
                      : 'var(--text-primary)',
                    opacity: draftState.is_player_turn ? 1 : 0.5,
                    cursor: draftState.is_player_turn ? 'pointer' : 'not-allowed',
                  }}
                >
                  {champ}
                </button>
              );
            })}
          </div>
        </div>
      )}

      {/* Lock / Complete button */}
      <div className="flex justify-center px-4 pb-2">
        {draftState.is_complete ? (
          <button
            onClick={onComplete}
            className="px-8 py-3 rounded-lg font-bold text-white flex items-center gap-2 transition-all duration-150"
            style={{
              background: 'linear-gradient(135deg, #06B6D4, #8B5CF6)',
              boxShadow: '0 0 16px rgba(6,182,212,0.3)',
            }}
          >
            <CheckCircle size={18} />
            Continue
          </button>
        ) : draftState.active_hover ? (
          <button
            onClick={onLock}
            className="px-8 py-3 rounded-lg font-bold text-white flex items-center gap-2 transition-all duration-150"
            style={{
              background: 'linear-gradient(135deg, #06B6D4, #8B5CF6)',
              boxShadow: '0 0 16px rgba(6,182,212,0.3)',
            }}
          >
            <Lock size={18} />
            Lock In
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
    </div>
  );
}

// ---------------------------------------------------------------------------
// Sub-components
// ---------------------------------------------------------------------------

function BanSlots({
  bans,
  maxBans,
  side,
}: {
  bans: string[];
  maxBans: number;
  side: 'blue' | 'red';
}) {
  const slots = Array.from({ length: maxBans }, (_, i) => bans[i] ?? null);
  const borderColor = side === 'blue' ? 'rgba(59,130,246,0.3)' : 'rgba(239,68,68,0.3)';

  return (
    <div className="flex gap-1">
      {slots.map((champ, i) => (
        <div
          key={i}
          className="w-16 h-8 rounded flex items-center justify-center text-xs font-mono border"
          style={{
            backgroundColor: champ ? 'rgba(239,68,68,0.1)' : 'var(--bg-elevated)',
            borderColor: champ ? borderColor : 'var(--border-subtle)',
            color: champ ? '#EF4444' : 'var(--text-muted)',
            textDecoration: champ ? 'line-through' : 'none',
          }}
        >
          {champ ?? '—'}
        </div>
      ))}
    </div>
  );
}

function PickSlots({
  picks,
  maxPicks,
  side,
}: {
  picks: string[];
  maxPicks: number;
  side: 'blue' | 'red';
}) {
  const slots = Array.from({ length: maxPicks }, (_, i) => picks[i] ?? null);
  const accentColor = side === 'blue' ? '#3B82F6' : '#EF4444';
  const bgTint = side === 'blue' ? 'rgba(59,130,246,0.1)' : 'rgba(239,68,68,0.1)';

  return (
    <div className="flex gap-1">
      {slots.map((champ, i) => (
        <div
          key={i}
          className="w-20 h-10 rounded-lg flex items-center justify-center text-xs font-bold border"
          style={{
            backgroundColor: champ ? bgTint : 'var(--bg-elevated)',
            borderColor: champ ? accentColor : 'var(--border-subtle)',
            color: champ ? accentColor : 'var(--text-muted)',
          }}
        >
          {champ ?? '—'}
        </div>
      ))}
    </div>
  );
}
