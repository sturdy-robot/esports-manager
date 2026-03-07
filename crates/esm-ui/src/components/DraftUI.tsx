import { useState, useEffect, useCallback } from 'react';
import { Lock, CheckCircle, Clock, Shield, Swords } from 'lucide-react';
import type { DraftSessionState, DraftPlayerInfo, ChampionClass, ChampionScaling } from '@/lib/api';

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

function classColor(cls: ChampionClass): string {
  switch (cls) {
    case 'Tank':     return 'var(--color-info)';
    case 'Fighter':  return 'var(--color-warning)';
    case 'Assassin': return 'var(--color-loss)';
    case 'Mage':     return 'var(--color-accent-violet)';
    case 'Marksman': return 'var(--color-accent-cyan)';
    case 'Support':  return 'var(--color-win)';
  }
}

function scalingColor(scaling: ChampionScaling): string {
  switch (scaling) {
    case 'Early': return 'var(--color-win)';
    case 'Mid':   return 'var(--color-warning)';
    case 'Late':  return 'var(--color-loss)';
  }
}

const ROLE_SHORT: Record<string, string> = {
  Top: 'TOP', Jungle: 'JNG', Mid: 'MID', Bot: 'BOT', Support: 'SUP',
};

function metaTierColor(tier: string): string {
  switch (tier) {
    case 'S': return '#F59E0B';
    case 'A': return '#06B6D4';
    case 'B': return 'var(--text-secondary)';
    case 'C': return 'var(--text-muted)';
    case 'D': return 'rgba(255,255,255,0.25)';
    default:  return 'var(--text-muted)';
  }
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
  const blueName = draftState.blue_team_name || (playerSide === 'blue' ? teamName : opponentName);
  const redName = draftState.red_team_name || (playerSide === 'red' ? teamName : opponentName);

  const phaseLabel = draftState.is_complete
    ? 'Draft Complete'
    : draftState.current_phase === 'Ban'
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
              {draftState.current_phase === 'Ban' ? (
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
                className="text-xs font-mono"
                style={{ color: 'var(--text-muted)' }}
              >
                {draftState.current_step}/{draftState.total_steps}
              </span>
            </div>
            {!draftState.is_complete && (
              <div className="flex items-center justify-center gap-2">
                <Clock size={14} style={{ color: timerColor }} />
                <span className="text-lg font-mono font-bold" style={{ color: timerColor }}>
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
                className="text-xs font-mono mt-1"
                style={{ color: 'var(--text-muted)' }}
              >
                All picks locked
              </div>
            )}
          </div>
        </div>

        {/* Champion grid */}
        {!draftState.is_complete && (
          <div
            className="flex-1 rounded-xl border p-3 overflow-y-auto"
            style={{
              backgroundColor: 'var(--bg-surface)',
              borderColor: 'var(--border-subtle)',
            }}
          >
            <div className="grid grid-cols-5 sm:grid-cols-10 gap-1.5">
              {draftState.available_champions.map((champ) => {
                const isHovered = draftState.active_hover === champ;
                const info = draftState.champion_details?.[champ];
                return (
                  <button
                    key={champ}
                    aria-label={champ}
                    data-hovered={isHovered ? 'true' : 'false'}
                    disabled={!draftState.is_player_turn}
                    onClick={() => onHover(champ)}
                    className="flex flex-col items-center justify-center gap-0.5 p-1.5 rounded-lg text-xs font-semibold transition-all duration-150 border"
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
                      boxShadow: isHovered ? '0 0 8px rgba(6,182,212,0.25)' : 'none',
                    }}
                  >
                    <span className="font-bold text-[0.65rem] leading-tight truncate w-full text-center">
                      {champ}
                    </span>
                    {info && (
                      <>
                        <span
                          className="text-[0.55rem] font-mono leading-tight"
                          style={{ color: classColor(info.class) }}
                        >
                          {info.class}
                        </span>
                        <div className="flex gap-1 items-center">
                          <span
                            className="text-[0.5rem] font-mono leading-tight"
                            style={{ color: scalingColor(info.scaling) }}
                          >
                            {info.scaling}
                          </span>
                          {info.meta_tier && info.meta_tier !== 'B' && (
                            <span
                              className="text-[0.5rem] font-mono font-bold leading-tight"
                              style={{ color: metaTierColor(info.meta_tier) }}
                            >
                              {info.meta_tier}
                            </span>
                          )}
                        </div>
                      </>
                    )}
                  </button>
                );
              })}
            </div>
          </div>
        )}

        {/* Lock / Complete button */}
        <div className="flex justify-center px-4 pb-1 shrink-0">
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

function TeamPanel({
  side,
  teamName,
  players,
  picks,
  bans,
  isActive,
  championDetails,
}: {
  side: 'blue' | 'red';
  teamName: string;
  players: DraftPlayerInfo[];
  picks: string[];
  bans: string[];
  isActive: boolean;
  championDetails: Record<string, import('@/lib/api').ChampionDraftInfo>;
}) {
  const accent = side === 'blue' ? '#3B82F6' : '#EF4444';
  const bgTint = side === 'blue' ? 'rgba(59,130,246,' : 'rgba(239,68,68,';
  const slots = Array.from({ length: 5 }, (_, i) => ({
    player: players[i] ?? null,
    champion: picks[i] ?? null,
  }));

  return (
    <div
      className="flex flex-col w-52 shrink-0 rounded-xl border overflow-hidden"
      style={{
        backgroundColor: 'var(--bg-surface)',
        borderColor: isActive ? accent : 'var(--border-subtle)',
        boxShadow: isActive ? `0 0 16px ${bgTint}0.25)` : 'none',
        transition: 'border-color 0.3s, box-shadow 0.3s',
      }}
    >
      {/* Team header */}
      <div
        className="px-4 py-2.5 text-center font-bold text-sm uppercase tracking-wider border-b"
        style={{
          color: accent,
          backgroundColor: `${bgTint}0.08)`,
          borderColor: `${bgTint}0.2)`,
        }}
      >
        {teamName}
      </div>

      {/* Player pick slots */}
      <div className="flex flex-col flex-1">
        {slots.map((slot, i) => {
          const champInfo = slot.champion ? championDetails?.[slot.champion] : null;
          const isLocked = !!slot.champion;
          return (
            <div
              key={i}
              className="flex items-center gap-2 px-3 py-2.5 border-b last:border-b-0"
              style={{
                borderColor: 'var(--border-subtle)',
                backgroundColor: isLocked ? `${bgTint}0.06)` : 'transparent',
              }}
            >
              {/* Role badge */}
              <div
                className="w-8 h-5 rounded text-[0.6rem] font-mono font-bold flex items-center justify-center shrink-0"
                style={{
                  backgroundColor: `${bgTint}0.12)`,
                  color: isLocked ? accent : 'var(--text-muted)',
                }}
              >
                {slot.player ? (ROLE_SHORT[slot.player.role] ?? slot.player.role.slice(0, 3).toUpperCase()) : `P${i + 1}`}
              </div>

              {/* Player + champion info */}
              <div className="flex flex-col min-w-0 flex-1">
                <span
                  className="text-xs font-semibold truncate"
                  style={{ color: isLocked ? 'var(--text-primary)' : 'var(--text-secondary)' }}
                >
                  {slot.player?.nickname ?? `Player ${i + 1}`}
                </span>
                {isLocked ? (
                  <div className="flex items-center gap-1">
                    <span
                      className="text-[0.65rem] font-bold truncate"
                      style={{ color: accent }}
                    >
                      {slot.champion}
                    </span>
                    {champInfo && (
                      <span
                        className="text-[0.5rem] font-mono"
                        style={{ color: classColor(champInfo.class as ChampionClass) }}
                      >
                        {champInfo.class}
                      </span>
                    )}
                  </div>
                ) : (
                  <span
                    className="text-[0.6rem] italic"
                    style={{ color: 'var(--text-muted)' }}
                  >
                    —
                  </span>
                )}
              </div>
            </div>
          );
        })}
      </div>

      {/* Bans row */}
      <div
        className="px-3 py-2 border-t"
        style={{ borderColor: 'var(--border-subtle)', backgroundColor: `${bgTint}0.04)` }}
      >
        <div
          className="text-[0.55rem] font-semibold uppercase tracking-wider mb-1"
          style={{ color: 'var(--text-muted)' }}
        >
          Bans
        </div>
        <div className="flex gap-1">
          {Array.from({ length: 5 }, (_, i) => {
            const champ = bans[i] ?? null;
            return (
              <div
                key={i}
                className="flex-1 h-5 rounded flex items-center justify-center text-[0.5rem] font-mono border"
                style={{
                  backgroundColor: champ ? 'rgba(239,68,68,0.1)' : 'var(--bg-elevated)',
                  borderColor: champ ? `${bgTint}0.3)` : 'var(--border-subtle)',
                  color: champ ? '#EF4444' : 'var(--text-muted)',
                  textDecoration: champ ? 'line-through' : 'none',
                }}
              >
                {champ ? champ.slice(0, 4) : '—'}
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
}
