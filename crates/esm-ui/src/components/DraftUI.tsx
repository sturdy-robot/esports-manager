import { useState, useEffect, useCallback, useMemo } from 'react';
import { Lock, CheckCircle, Clock, Shield, Swords, Search, X } from 'lucide-react';
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

const ALL_CLASSES: ChampionClass[] = ['Tank', 'Fighter', 'Assassin', 'Mage', 'Marksman', 'Support'];

function masteryColor(mastery: string): string {
  switch (mastery) {
    case 'Challenger': return '#F59E0B';
    case 'Master':     return '#A855F7';
    case 'Diamond':    return '#06B6D4';
    case 'Platinum':   return '#22D3EE';
    case 'Gold':       return '#EAB308';
    case 'Silver':     return '#94A3B8';
    case 'Bronze':     return 'var(--text-muted)';
    default:           return 'var(--text-muted)';
  }
}

function masteryShort(mastery: string): string {
  switch (mastery) {
    case 'Challenger': return 'CHL';
    case 'Master':     return 'MAS';
    case 'Diamond':    return 'DIA';
    case 'Platinum':   return 'PLT';
    case 'Gold':       return 'GLD';
    case 'Silver':     return 'SLV';
    case 'Bronze':     return 'BRZ';
    default:           return mastery.slice(0, 3).toUpperCase();
  }
}

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

  // ---- Champion grid filters ----
  const [searchText, setSearchText] = useState('');
  const [classFilter, setClassFilter] = useState<ChampionClass | null>(null);
  const [tooltipChamp, setTooltipChamp] = useState<string | null>(null);

  const filteredChampions = useMemo(() => {
    return draftState.available_champions.filter((champ) => {
      if (searchText && !champ.toLowerCase().includes(searchText.toLowerCase())) {
        return false;
      }
      if (classFilter) {
        const info = draftState.champion_details?.[champ];
        if (info && info.class !== classFilter) return false;
      }
      return true;
    });
  }, [draftState.available_champions, draftState.champion_details, searchText, classFilter]);

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

        {/* Champion grid with filters */}
        {!draftState.is_complete && (
          <div
            className="flex flex-col flex-1 rounded-xl border overflow-hidden"
            style={{
              backgroundColor: 'var(--bg-surface)',
              borderColor: 'var(--border-subtle)',
            }}
          >
            {/* Filter bar */}
            <div
              className="flex items-center gap-2 px-3 py-2 border-b shrink-0"
              style={{ borderColor: 'var(--border-subtle)' }}
            >
              {/* Search input */}
              <div
                className="flex items-center gap-1.5 px-2 py-1 rounded-lg border flex-1 max-w-48"
                style={{
                  backgroundColor: 'var(--bg-elevated)',
                  borderColor: searchText ? 'var(--color-accent-cyan)' : 'var(--border-subtle)',
                }}
              >
                <Search size={12} style={{ color: 'var(--text-muted)', flexShrink: 0 }} />
                <input
                  type="text"
                  placeholder="Search..."
                  value={searchText}
                  onChange={(e) => setSearchText(e.target.value)}
                  className="bg-transparent text-xs outline-none flex-1 min-w-0"
                  style={{ color: 'var(--text-primary)' }}
                />
                {searchText && (
                  <button onClick={() => setSearchText('')} className="shrink-0">
                    <X size={10} style={{ color: 'var(--text-muted)' }} />
                  </button>
                )}
              </div>

              {/* Class filter pills */}
              <div className="flex gap-1">
                {ALL_CLASSES.map((cls) => {
                  const isActive = classFilter === cls;
                  return (
                    <button
                      key={cls}
                      onClick={() => setClassFilter(isActive ? null : cls)}
                      className="px-2 py-0.5 rounded text-[0.6rem] font-mono font-bold transition-all duration-150 border"
                      style={{
                        backgroundColor: isActive ? 'rgba(6,182,212,0.15)' : 'transparent',
                        borderColor: isActive ? classColor(cls) : 'var(--border-subtle)',
                        color: isActive ? classColor(cls) : 'var(--text-muted)',
                      }}
                    >
                      {cls}
                    </button>
                  );
                })}
              </div>

              {/* Result count */}
              <span
                className="text-[0.6rem] font-mono ml-auto shrink-0"
                style={{ color: 'var(--text-muted)' }}
              >
                {filteredChampions.length}/{draftState.available_champions.length}
              </span>
            </div>

            {/* Grid */}
            <div className="flex-1 p-3 overflow-y-auto relative">
              {/* Tooltip */}
              {tooltipChamp && draftState.champion_details?.[tooltipChamp] && (
                <ChampionTooltip info={draftState.champion_details[tooltipChamp]} />
              )}
              <div className="grid grid-cols-5 sm:grid-cols-10 gap-1.5">
                {filteredChampions.map((champ) => {
                  const isHovered = draftState.active_hover === champ;
                  const info = draftState.champion_details?.[champ];
                  return (
                    <button
                      key={champ}
                      aria-label={champ}
                      data-hovered={isHovered ? 'true' : 'false'}
                      disabled={!draftState.is_player_turn}
                      onClick={() => onHover(champ)}
                      onMouseEnter={() => setTooltipChamp(champ)}
                      onMouseLeave={() => setTooltipChamp(null)}
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
                            {info.meta_tier && (
                              <span
                                className="text-[0.5rem] font-mono font-bold leading-tight"
                                style={{ color: metaTierColor(info.meta_tier) }}
                              >
                                {info.meta_tier}
                              </span>
                            )}
                            {info.best_mastery && info.best_mastery !== 'Bronze' && (
                              <span
                                className="text-[0.5rem] font-mono leading-tight"
                                style={{ color: masteryColor(info.best_mastery) }}
                              >
                                {masteryShort(info.best_mastery)}
                              </span>
                            )}
                          </div>
                        </>
                      )}
                    </button>
                  );
                })}
                {filteredChampions.length === 0 && (
                  <div
                    className="col-span-full text-center py-8 text-sm"
                    style={{ color: 'var(--text-muted)' }}
                  >
                    No champions match filters
                  </div>
                )}
              </div>
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

function ChampionTooltip({ info }: { info: import('@/lib/api').ChampionDraftInfo }) {
  return (
    <div
      className="absolute top-2 right-2 z-10 pointer-events-none rounded-lg border p-3 shadow-lg"
      style={{
        backgroundColor: 'var(--bg-elevated)',
        borderColor: 'var(--border-subtle)',
        minWidth: '180px',
      }}
    >
      <div className="flex items-center gap-2 mb-2">
        <span
          className="text-sm font-bold"
          style={{ color: 'var(--text-primary)' }}
        >
          {info.name}
        </span>
        {info.meta_tier && (
          <span
            className="text-[0.6rem] font-mono font-bold px-1.5 py-0.5 rounded"
            style={{
              color: metaTierColor(info.meta_tier),
              backgroundColor: 'rgba(255,255,255,0.05)',
              border: `1px solid ${metaTierColor(info.meta_tier)}`,
            }}
          >
            {info.meta_tier}-Tier
          </span>
        )}
      </div>

      <div className="flex flex-col gap-1.5">
        <div className="flex items-center gap-2">
          <span
            className="text-[0.6rem] font-mono uppercase"
            style={{ color: 'var(--text-muted)', width: '48px' }}
          >
            Class
          </span>
          <span
            className="text-xs font-semibold"
            style={{ color: classColor(info.class as ChampionClass) }}
          >
            {info.class}
          </span>
        </div>

        <div className="flex items-center gap-2">
          <span
            className="text-[0.6rem] font-mono uppercase"
            style={{ color: 'var(--text-muted)', width: '48px' }}
          >
            Scale
          </span>
          <span
            className="text-xs font-semibold"
            style={{ color: scalingColor(info.scaling as ChampionScaling) }}
          >
            {info.scaling} Game
          </span>
        </div>

        {info.best_mastery && (
          <div className="flex items-center gap-2">
            <span
              className="text-[0.6rem] font-mono uppercase"
              style={{ color: 'var(--text-muted)', width: '48px' }}
            >
              Mstr
            </span>
            <span
              className="text-xs font-semibold"
              style={{ color: masteryColor(info.best_mastery) }}
            >
              {info.best_mastery}
            </span>
          </div>
        )}

        {info.tags.length > 0 && (
          <div className="flex items-start gap-2">
            <span
              className="text-[0.6rem] font-mono uppercase shrink-0 mt-0.5"
              style={{ color: 'var(--text-muted)', width: '48px' }}
            >
              Tags
            </span>
            <div className="flex flex-wrap gap-1">
              {info.tags.map((tag) => (
                <span
                  key={tag}
                  className="text-[0.55rem] font-mono px-1.5 py-0.5 rounded"
                  style={{
                    backgroundColor: 'rgba(6,182,212,0.08)',
                    color: 'var(--color-accent-cyan)',
                    border: '1px solid rgba(6,182,212,0.2)',
                  }}
                >
                  {tag}
                </span>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
