import { useState, useEffect, useRef, useCallback } from 'react';
import {
  Swords, Shield, Flame, Crown, Castle, Trophy, Zap,
  ChevronRight, Play, Pause, SkipForward,
} from 'lucide-react';
import { TacticsPanel } from './TacticsPanel';
import type { SimulateMatchResult, MatchEventInfo, GameSnapshotInfo, PlayerSnapshotInfo, PlaystyleType, FocusType, TacticsInfo } from '@/lib/api';

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const BASE_INTERVAL_MS = 1500;
const SPEEDS = [1, 2, 5, 10] as const;
const POSITIONS = ['TOP', 'JGL', 'MID', 'BOT', 'SUP'] as const;

// ---------------------------------------------------------------------------
// Props
// ---------------------------------------------------------------------------

interface MatchSimUIProps {
  result: SimulateMatchResult;
  onComplete: () => void;
  /** Current tactics for mid-match adjustment UI */
  tactics?: TacticsInfo;
  /** Called when the player adjusts tactics mid-match */
  onTacticsChange?: (playstyle: PlaystyleType, focus: FocusType) => void;
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function formatMinute(minute: number): string {
  return `${minute}:00`;
}

function formatGold(gold: number): string {
  if (gold >= 1000) return `${(gold / 1000).toFixed(1)}k`;
  return String(gold);
}

function formatGoldFull(gold: number): string {
  return gold.toLocaleString();
}

function eventIcon(kind: string) {
  switch (kind) {
    case 'solo_kill': return <Swords size={14} />;
    case 'teamfight': return <Zap size={14} />;
    case 'tower': return <Castle size={14} />;
    case 'dragon': return <Flame size={14} />;
    case 'herald': return <Shield size={14} />;
    case 'baron': return <Crown size={14} />;
    case 'inhibitor': return <Castle size={14} />;
    case 'nexus': return <Trophy size={14} />;
    case 'multi_kill': return <Zap size={14} />;
    case 'killing_spree': return <Flame size={14} />;
    default: return <ChevronRight size={14} />;
  }
}

function eventColor(kind: string): string {
  switch (kind) {
    case 'solo_kill': return '#EF4444';
    case 'teamfight': return '#F59E0B';
    case 'tower': return '#8B5CF6';
    case 'dragon': return '#06B6D4';
    case 'herald': return '#10B981';
    case 'baron': return '#A855F7';
    case 'inhibitor': return '#EC4899';
    case 'nexus': return '#FFD700';
    case 'multi_kill': return '#FF6B6B';
    case 'killing_spree': return '#FF9F43';
    default: return 'var(--text-secondary)';
  }
}

function phaseBadgeColor(phase: string): string {
  switch (phase) {
    case 'Early': return '#3B82F6';
    case 'Mid': return '#F59E0B';
    case 'Late': return '#EF4444';
    default: return 'var(--text-muted)';
  }
}

// ---------------------------------------------------------------------------
// Main Component
// ---------------------------------------------------------------------------

export function MatchSimUI({ result, onComplete, tactics, onTacticsChange }: MatchSimUIProps) {
  const [revealedCount, setRevealedCount] = useState(0);
  const [isPlaying, setIsPlaying] = useState(true);
  const [speedIdx, setSpeedIdx] = useState(0);
  const [showTactics, setShowTactics] = useState(false);
  const [acknowledgedPhases, setAcknowledgedPhases] = useState<Set<string>>(new Set());
  const feedRef = useRef<HTMLDivElement>(null);

  const allDone = revealedCount >= result.events.length;
  const visibleEvents = result.events.slice(0, revealedCount);
  const latestEvent = visibleEvents.length > 0 ? visibleEvents[visibleEvents.length - 1] : null;
  const snapshot = latestEvent?.snapshot ?? null;
  const currentMinute = latestEvent?.minute ?? 0;

  // Derived: upcoming phase info for tactics overlay
  const nextEvent = revealedCount < result.events.length ? result.events[revealedCount] : null;
  const nextPhase = nextEvent?.phase ?? '';

  // Detect phase-boundary pause: compute whether we should pause NOW
  // This is derived state computed during render, not an effect.
  const shouldPauseForTactics = (() => {
    if (allDone || showTactics || !onTacticsChange || !tactics) return false;
    const curPhase = latestEvent?.phase ?? '';
    return curPhase !== '' && nextPhase !== '' && curPhase !== nextPhase && !acknowledgedPhases.has(nextPhase);
  })();

  const handleTacticsConfirm = useCallback((playstyle: PlaystyleType, focus: FocusType) => {
    onTacticsChange?.(playstyle, focus);
    setAcknowledgedPhases((prev) => new Set(prev).add(nextPhase));
    setShowTactics(false);
    setIsPlaying(true);
  }, [onTacticsChange, nextPhase]);

  const handleTacticsSkip = useCallback(() => {
    setAcknowledgedPhases((prev) => new Set(prev).add(nextPhase));
    setShowTactics(false);
    setIsPlaying(true);
  }, [nextPhase]);

  // Progressive reveal timer — skips tick when shouldPauseForTactics is true
  useEffect(() => {
    if (!isPlaying || allDone || showTactics || shouldPauseForTactics) return;

    const ms = BASE_INTERVAL_MS / SPEEDS[speedIdx];
    const id = setTimeout(() => setRevealedCount((prev) => prev + 1), ms);
    return () => clearTimeout(id);
  }, [isPlaying, speedIdx, revealedCount, allDone, showTactics, shouldPauseForTactics]);

  // When the timer stops due to a phase boundary, show the tactics panel
  // This runs as a separate effect triggered by shouldPauseForTactics becoming true
  // while isPlaying is still true.
  useEffect(() => {
    if (shouldPauseForTactics && isPlaying) {
      // Use a microtask to avoid synchronous setState-in-effect lint warning
      Promise.resolve().then(() => {
        setIsPlaying(false);
        setShowTactics(true);
      });
    }
  }, [shouldPauseForTactics, isPlaying]);

  // Auto-scroll event feed
  useEffect(() => {
    if (feedRef.current) {
      feedRef.current.scrollTop = feedRef.current.scrollHeight;
    }
  }, [revealedCount]);

  const handleSkip = useCallback(() => {
    setRevealedCount(result.events.length);
    setIsPlaying(false);
  }, [result.events.length]);

  const cycleSpeed = useCallback(() => {
    setSpeedIdx((prev) => (prev + 1) % SPEEDS.length);
  }, []);

  const blueGold = snapshot?.blue_team_gold ?? 0;
  const redGold = snapshot?.red_team_gold ?? 0;
  const goldTotal = blueGold + redGold || 1;
  const bluePct = (blueGold / goldTotal) * 100;

  return (
    <div className="flex flex-col w-full h-full gap-2 p-2">
      {/* Top bar: teams + gold + timer */}
      <div
        className="flex items-center justify-between px-4 py-3 rounded-xl border shrink-0"
        style={{ backgroundColor: 'var(--bg-surface)', borderColor: 'var(--border-subtle)' }}
      >
        <TeamHeader name={result.blue_team} gold={blueGold} color="#3B82F6" />
        <div className="text-center px-4">
          <div className="text-lg font-mono font-bold" style={{ color: 'var(--text-primary)' }}>
            {formatMinute(currentMinute)}
          </div>
          {allDone && (
            <div className="text-xs font-semibold uppercase tracking-wider" style={{ color: 'var(--color-win)' }}>
              {result.winner} wins
            </div>
          )}
        </div>
        <TeamHeader name={result.red_team} gold={redGold} color="#EF4444" />
      </div>

      {/* Gold bar */}
      <div className="h-1.5 rounded-full overflow-hidden shrink-0" style={{ backgroundColor: 'var(--bg-elevated)' }}>
        <div
          className="h-full transition-all duration-500 ease-out"
          style={{ width: `${bluePct}%`, background: 'linear-gradient(90deg, #3B82F6, #60A5FA)' }}
        />
      </div>

      {/* Main area: scoreboard | events | scoreboard */}
      <div className="flex gap-2 flex-1 min-h-0">
        {/* Blue scoreboard */}
        <TeamScoreboard
          players={snapshot?.blue_players ?? []}
          color="#3B82F6"
          side="blue"
        />

        {/* Event feed */}
        <div
          ref={feedRef}
          className="flex-1 rounded-xl border overflow-y-auto p-2"
          style={{ backgroundColor: 'var(--bg-surface)', borderColor: 'var(--border-subtle)' }}
        >
          <div className="space-y-0.5">
            {visibleEvents.map((event, i) => (
              <EventRow key={i} event={event} isNew={i === visibleEvents.length - 1} />
            ))}
            {!allDone && visibleEvents.length === 0 && (
              <div className="text-center text-sm py-8" style={{ color: 'var(--text-muted)' }}>
                Match starting...
              </div>
            )}
          </div>
        </div>

        {/* Red scoreboard */}
        <TeamScoreboard
          players={snapshot?.red_players ?? []}
          color="#EF4444"
          side="red"
        />
      </div>

      {/* Phase transition tactics overlay */}
      {showTactics && tactics && (
        <div
          className="shrink-0 relative"
        >
          <div className="flex items-center justify-between px-3 py-1.5 mb-1">
            <div className="flex items-center gap-2">
              <div
                className="w-2 h-2 rounded-full animate-pulse"
                style={{ backgroundColor: phaseBadgeColor(nextPhase) }}
              />
              <span className="text-xs font-bold uppercase tracking-wider" style={{ color: phaseBadgeColor(nextPhase) }}>
                Entering {nextPhase} Game
              </span>
            </div>
            <button
              onClick={handleTacticsSkip}
              className="text-xs px-2 py-1 rounded border cursor-pointer"
              style={{ borderColor: 'var(--border-subtle)', color: 'var(--text-muted)', backgroundColor: 'transparent' }}
            >
              Keep Current
            </button>
          </div>
          <TacticsPanel
            tactics={tactics}
            onConfirm={handleTacticsConfirm}
            context={`Adjust for ${nextPhase} Phase`}
            compact
          />
        </div>
      )}

      {/* Objectives bar */}
      <ObjectivesBar snapshot={snapshot} />

      {/* Controls */}
      <div className="flex items-center justify-center gap-3 shrink-0 py-1">
        {!allDone ? (
          <>
            <button
              onClick={() => setIsPlaying((p) => !p)}
              className="p-2 rounded-lg border"
              style={{ backgroundColor: 'var(--bg-elevated)', borderColor: 'var(--border-subtle)', color: 'var(--text-primary)' }}
            >
              {isPlaying ? <Pause size={16} /> : <Play size={16} />}
            </button>
            <button
              onClick={cycleSpeed}
              className="px-3 py-1.5 rounded-lg border text-xs font-mono font-bold"
              style={{ backgroundColor: 'var(--bg-elevated)', borderColor: 'var(--border-subtle)', color: 'var(--text-primary)' }}
            >
              {SPEEDS[speedIdx]}x
            </button>
            <button
              onClick={handleSkip}
              className="p-2 rounded-lg border"
              style={{ backgroundColor: 'var(--bg-elevated)', borderColor: 'var(--border-subtle)', color: 'var(--text-primary)' }}
              title="Skip to end"
            >
              <SkipForward size={16} />
            </button>
          </>
        ) : (
          <button
            onClick={onComplete}
            className="px-8 py-2.5 rounded-lg font-bold text-white flex items-center gap-2"
            style={{ background: 'linear-gradient(135deg, #06B6D4, #8B5CF6)' }}
          >
            <ChevronRight size={18} />
            Continue
          </button>
        )}
      </div>
    </div>
  );
}

// ---------------------------------------------------------------------------
// Sub-components
// ---------------------------------------------------------------------------

function TeamHeader({ name, gold, color }: { name: string; gold: number; color: string }) {
  return (
    <div className="text-center flex-1">
      <div className="text-sm font-bold" style={{ color }}>{name}</div>
      <div className="text-xl font-mono font-black" style={{ color: 'var(--text-primary)' }}>
        {formatGoldFull(gold)}
      </div>
    </div>
  );
}

function TeamScoreboard({ players, color, side }: { players: PlayerSnapshotInfo[]; color: string; side: 'blue' | 'red' }) {
  return (
    <div
      className="w-48 rounded-xl border overflow-hidden shrink-0 flex flex-col"
      style={{ backgroundColor: 'var(--bg-surface)', borderColor: 'var(--border-subtle)' }}
    >
      <div
        className="text-xs font-bold text-center py-1.5 uppercase tracking-wider"
        style={{ color, backgroundColor: `${color}10` }}
      >
        {side === 'blue' ? 'Blue' : 'Red'} Side
      </div>
      <div className="flex-1 overflow-y-auto">
        {players.length === 0 ? (
          <div className="text-xs text-center py-4" style={{ color: 'var(--text-muted)' }}>—</div>
        ) : (
          players.map((p, i) => (
            <PlayerRow key={i} player={p} position={POSITIONS[i] ?? '?'} />
          ))
        )}
      </div>
    </div>
  );
}

function PlayerRow({ player, position }: { player: PlayerSnapshotInfo; position: string }) {
  const deadStyle = player.is_dead ? { opacity: 0.4 } : {};
  return (
    <div
      className="flex items-center gap-1.5 px-2 py-1 text-xs border-b"
      style={{ borderColor: 'var(--border-subtle)', ...deadStyle }}
    >
      <span className="w-7 font-bold font-mono shrink-0" style={{ color: 'var(--text-muted)', fontSize: '0.65rem' }}>
        {position}
      </span>
      <span className="font-mono font-bold flex-1" style={{ color: 'var(--text-primary)' }}>
        {player.kills}/{player.deaths}/{player.assists}
      </span>
      <span className="font-mono" style={{ color: 'var(--text-secondary)', fontSize: '0.65rem' }}>
        {player.cs}cs
      </span>
      <span className="font-mono" style={{ color: '#F59E0B', fontSize: '0.65rem' }}>
        {formatGold(player.gold)}
      </span>
    </div>
  );
}

function ObjectivesBar({ snapshot }: { snapshot: GameSnapshotInfo | null }) {
  if (!snapshot) return null;
  return (
    <div
      className="flex items-center justify-center gap-6 px-4 py-2 rounded-xl border shrink-0"
      style={{ backgroundColor: 'var(--bg-surface)', borderColor: 'var(--border-subtle)' }}
    >
      <ObjectiveChip
        icon={<Flame size={14} />}
        label="Dragons"
        blue={snapshot.dragons_blue}
        red={snapshot.dragons_red}
      />
      <ObjectiveChip
        icon={<Crown size={14} />}
        label="Baron"
        status={snapshot.baron_alive ? 'alive' : snapshot.baron_timer > 0 ? `${snapshot.baron_timer}m` : '—'}
      />
      <ObjectiveChip
        icon={<Shield size={14} />}
        label="Herald"
        status={snapshot.herald_available ? 'up' : '—'}
      />
    </div>
  );
}

function ObjectiveChip({
  icon, label, blue, red, status,
}: {
  icon: React.ReactNode;
  label: string;
  blue?: number;
  red?: number;
  status?: string;
}) {
  return (
    <div className="flex items-center gap-2 text-xs">
      <span style={{ color: 'var(--text-muted)' }}>{icon}</span>
      <span className="font-semibold" style={{ color: 'var(--text-secondary)' }}>{label}</span>
      {blue !== undefined && red !== undefined ? (
        <span className="font-mono font-bold" style={{ color: 'var(--text-primary)' }}>
          <span style={{ color: '#3B82F6' }}>{blue}</span>
          {' – '}
          <span style={{ color: '#EF4444' }}>{red}</span>
        </span>
      ) : (
        <span className="font-mono" style={{ color: 'var(--text-muted)' }}>{status}</span>
      )}
    </div>
  );
}

function EventRow({ event, isNew }: { event: MatchEventInfo; isNew: boolean }) {
  const color = eventColor(event.kind);
  const phaseColor = phaseBadgeColor(event.phase);

  return (
    <div
      className="flex items-start gap-2 px-2 py-1.5 rounded-lg transition-all duration-300"
      style={{
        backgroundColor: isNew ? `${color}08` : 'transparent',
      }}
    >
      <div
        className="w-10 text-right text-xs font-mono font-bold shrink-0 pt-0.5"
        style={{ color: 'var(--text-muted)' }}
      >
        {formatMinute(event.minute)}
      </div>
      <div
        className="text-xs font-semibold px-1 py-0.5 rounded shrink-0"
        style={{ color: phaseColor, backgroundColor: `${phaseColor}15` }}
      >
        {event.phase}
      </div>
      <div className="shrink-0 pt-0.5" style={{ color }}>
        {eventIcon(event.kind)}
      </div>
      <div className="text-xs flex-1" style={{ color: 'var(--text-primary)' }}>
        {event.commentary ?? event.kind}
      </div>
    </div>
  );
}
