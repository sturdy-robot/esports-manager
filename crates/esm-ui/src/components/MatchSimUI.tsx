import { useState, useEffect, useRef, useCallback } from 'react';
import {
  Swords, Shield, Flame, Crown, Castle, Trophy, Zap,
  ChevronRight, Play, Pause, SkipForward,
} from 'lucide-react';
import type { SimulateMatchResult, MatchEventInfo, GameSnapshotInfo, PlayerSnapshotInfo, PlaystyleType, FocusType, TacticsInfo, MatchRosterEntry } from '@/lib/api';

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
    case 'tower': return '#10B981';
    case 'dragon': return '#06B6D4';
    case 'herald': return '#10B981';
    case 'baron': return '#06B6D4';
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
  const feedRef = useRef<HTMLDivElement>(null);

  const allDone = revealedCount >= result.events.length;
  const visibleEvents = result.events.slice(0, revealedCount);
  const latestEvent = visibleEvents.length > 0 ? visibleEvents[visibleEvents.length - 1] : null;
  const snapshot = latestEvent?.snapshot ?? null;
  const currentMinute = latestEvent?.minute ?? 0;

  // Progressive reveal timer
  useEffect(() => {
    if (!isPlaying || allDone) return;

    const ms = BASE_INTERVAL_MS / SPEEDS[speedIdx];
    const id = setTimeout(() => setRevealedCount((prev) => prev + 1), ms);
    return () => clearTimeout(id);
  }, [isPlaying, speedIdx, revealedCount, allDone]);

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
    <div className="flex flex-col w-full flex-1 min-h-0 gap-2 p-2">
      {/* Top bar: teams + gold + timer */}
      <div
        className="flex items-center justify-between px-4 py-3 rounded-xl border shrink-0"
        style={{ backgroundColor: 'var(--bg-surface)', borderColor: 'var(--border-subtle)' }}
      >
        <TeamHeader name={result.blue_team} gold={blueGold} color="#3B82F6" />
        <div className="text-center px-4">
          <div className="text-lg tabular-nums font-bold font-display" style={{ color: 'var(--text-primary)' }}>
            {formatMinute(currentMinute)}
          </div>
          {allDone && (
            <div
              data-testid="winner-banner"
              className="text-sm font-bold uppercase tracking-wider px-3 py-1 rounded-md"
              style={{
                background: 'linear-gradient(135deg, rgba(16,185,129,0.15), rgba(6,182,212,0.15))',
                color: 'var(--color-win)',
                border: '1px solid rgba(34,197,94,0.3)',
                boxShadow: '0 0 12px rgba(34,197,94,0.2)',
              }}
            >
              {result.winner} wins
            </div>
          )}
        </div>
        <TeamHeader name={result.red_team} gold={redGold} color="#EF4444" />
      </div>

      {/* Draft banner */}
      {(result.blue_roster.length > 0 || result.red_roster.length > 0) && (
        <DraftBanner blueRoster={result.blue_roster} redRoster={result.red_roster} />
      )}

      {/* Gold bar */}
      <div className="h-1.5 rounded-full overflow-hidden shrink-0" style={{ backgroundColor: 'var(--bg-elevated)' }}>
        <div
          className="h-full transition-all duration-500 ease-out"
          style={{ width: `${bluePct}%`, background: 'linear-gradient(90deg, #3B82F6, #60A5FA)' }}
        />
      </div>

      {/* Objectives bar */}
      <ObjectivesBar snapshot={snapshot} />

      {/* Controls bar — playback + inline tactics (always at top, above event feed) */}
      <div
        className="flex items-center justify-between gap-3 shrink-0 px-3 py-1.5 rounded-lg border"
        style={{ backgroundColor: 'var(--bg-surface)', borderColor: 'var(--border-subtle)' }}
      >
        {/* Playback controls */}
        <div className="flex items-center gap-2">
          {!allDone ? (
            <>
              <button
                onClick={() => setIsPlaying((p) => !p)}
                className="p-1.5 rounded-md border cursor-pointer transition-all duration-150 glow-hover"
                style={{ backgroundColor: 'var(--bg-elevated)', borderColor: 'var(--border-subtle)', color: 'var(--text-primary)' }}
              >
                {isPlaying ? <Pause size={14} /> : <Play size={14} />}
              </button>
              <button
                onClick={cycleSpeed}
                className="px-2.5 py-1 rounded-md border text-xs tabular-nums font-bold cursor-pointer transition-all duration-150 glow-hover"
                style={{ backgroundColor: 'var(--bg-elevated)', borderColor: 'var(--border-subtle)', color: 'var(--text-primary)' }}
              >
                {SPEEDS[speedIdx]}x
              </button>
              <button
                onClick={handleSkip}
                className="p-1.5 rounded-md border cursor-pointer transition-all duration-150 glow-hover"
                style={{ backgroundColor: 'var(--bg-elevated)', borderColor: 'var(--border-subtle)', color: 'var(--text-primary)' }}
                title="Skip to end"
              >
                <SkipForward size={14} />
              </button>
            </>
          ) : (
            <button
              onClick={onComplete}
              className="px-5 py-1.5 rounded-md font-bold text-white text-xs flex items-center gap-1.5 transition-all duration-150"
              style={{
                background: 'linear-gradient(135deg, #10B981, #06B6D4)',
                boxShadow: '0 0 12px rgba(6,182,212,0.3)',
              }}
            >
              <ChevronRight size={14} />
              Continue
            </button>
          )}

          {/* Phase badge */}
          {latestEvent?.phase && (
            <span
              className="text-[0.6rem] font-bold uppercase tracking-wider px-2 py-0.5 rounded"
              style={{ color: phaseBadgeColor(latestEvent.phase), backgroundColor: `${phaseBadgeColor(latestEvent.phase)}15` }}
            >
              {latestEvent.phase}
            </span>
          )}
        </div>

        {/* Inline tactics — always accessible */}
        {tactics && onTacticsChange && (
          <InlineTactics tactics={tactics} onChange={onTacticsChange} />
        )}
      </div>

      {/* Main area: scoreboard | events | scoreboard */}
      <div className="flex gap-2 flex-1 min-h-0">
        {/* Blue scoreboard */}
        <TeamScoreboard
          players={snapshot?.blue_players ?? []}
          roster={result.blue_roster}
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
          roster={result.red_roster}
          color="#EF4444"
          side="red"
        />
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
      <div className="text-xl tabular-nums font-black font-display" style={{ color: 'var(--text-primary)' }}>
        {formatGoldFull(gold)}
      </div>
    </div>
  );
}

function TeamScoreboard({ players, roster, color, side }: { players: PlayerSnapshotInfo[]; roster: MatchRosterEntry[]; color: string; side: 'blue' | 'red' }) {
  return (
    <div
      className="w-56 rounded-xl border overflow-hidden shrink-0 flex flex-col"
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
            <PlayerRow key={i} player={p} position={POSITIONS[i] ?? '?'} entry={roster[i]} />
          ))
        )}
      </div>
    </div>
  );
}

function PlayerRow({ player, position, entry }: { player: PlayerSnapshotInfo; position: string; entry?: MatchRosterEntry }) {
  const deadStyle = player.is_dead ? { opacity: 0.4 } : {};
  return (
    <div
      className="flex flex-col gap-0.5 px-2 py-1.5 text-xs border-b"
      style={{ borderColor: 'var(--border-subtle)', ...deadStyle }}
    >
      {/* Row 1: role, nickname, champion */}
      <div className="flex items-center gap-1.5">
        <span className="w-7 font-bold shrink-0" style={{ color: 'var(--text-muted)', fontSize: '0.6rem' }}>
          {position}
        </span>
        <span className="font-semibold truncate flex-1" style={{ color: 'var(--text-primary)', fontSize: '0.7rem' }}>
          {entry?.nickname ?? `P${position}`}
        </span>
        {entry?.champion && (
          <span className="truncate" style={{ color: 'var(--color-accent-cyan)', fontSize: '0.6rem' }}>
            {entry.champion}
          </span>
        )}
      </div>
      {/* Row 2: KDA, CS, gold */}
      <div className="flex items-center gap-1.5 pl-8">
        <span className="tabular-nums font-bold" style={{ color: 'var(--text-primary)', fontSize: '0.65rem' }}>
          {player.kills}/{player.deaths}/{player.assists}
        </span>
        <span className="tabular-nums" style={{ color: 'var(--text-secondary)', fontSize: '0.6rem' }}>
          {player.cs}cs
        </span>
        <span className="tabular-nums" style={{ color: '#F59E0B', fontSize: '0.6rem' }}>
          {formatGold(player.gold)}
        </span>
      </div>
    </div>
  );
}

function ObjectivesBar({ snapshot }: { snapshot: GameSnapshotInfo | null }) {
  if (!snapshot) return null;
  return (
    <div
      className="flex items-center justify-center gap-6 px-4 py-2 rounded-xl border shrink-0 flex-wrap"
      style={{ backgroundColor: 'var(--bg-surface)', borderColor: 'var(--border-subtle)' }}
    >
      <ObjectiveChip
        icon={<Flame size={14} />}
        label="Dragons"
        blue={snapshot.dragons_blue}
        red={snapshot.dragons_red}
      />
      <ObjectiveChip
        icon={<Castle size={14} />}
        label="Towers"
        blue={snapshot.blue_towers}
        red={snapshot.red_towers}
      />
      <ObjectiveChip
        icon={<Shield size={14} />}
        label="Inhibitors"
        blue={snapshot.blue_inhibitors}
        red={snapshot.red_inhibitors}
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
        <span className="tabular-nums font-bold" style={{ color: 'var(--text-primary)' }}>
          <span style={{ color: '#3B82F6' }}>{blue}</span>
          {' – '}
          <span style={{ color: '#EF4444' }}>{red}</span>
        </span>
      ) : (
        <span className="tabular-nums" style={{ color: 'var(--text-muted)' }}>{status}</span>
      )}
    </div>
  );
}

function DraftBanner({ blueRoster, redRoster }: { blueRoster: MatchRosterEntry[]; redRoster: MatchRosterEntry[] }) {
  return (
    <div
      className="flex items-center gap-2 px-3 py-1.5 rounded-lg border shrink-0"
      style={{ backgroundColor: 'var(--bg-surface)', borderColor: 'var(--border-subtle)' }}
    >
      {/* Blue picks */}
      <div className="flex gap-1.5 flex-1 justify-end">
        {blueRoster.map((entry, i) => (
          <div
            key={i}
            className="text-center px-1.5 py-0.5 rounded"
            style={{ backgroundColor: 'rgba(59,130,246,0.08)' }}
          >
            <div className="text-[0.55rem] font-bold" style={{ color: '#3B82F6' }}>
              {entry.champion || '—'}
            </div>
            <div className="text-[0.5rem]" style={{ color: 'var(--text-muted)' }}>
              {entry.nickname}
            </div>
          </div>
        ))}
      </div>

      {/* VS */}
      <span
        className="text-[0.6rem] font-bold uppercase tracking-widest shrink-0 px-2"
        style={{ color: 'var(--text-muted)' }}
      >
        vs
      </span>

      {/* Red picks */}
      <div className="flex gap-1.5 flex-1 justify-start">
        {redRoster.map((entry, i) => (
          <div
            key={i}
            className="text-center px-1.5 py-0.5 rounded"
            style={{ backgroundColor: 'rgba(239,68,68,0.08)' }}
          >
            <div className="text-[0.55rem] font-bold" style={{ color: '#EF4444' }}>
              {entry.champion || '—'}
            </div>
            <div className="text-[0.5rem]" style={{ color: 'var(--text-muted)' }}>
              {entry.nickname}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

const PLAYSTYLE_OPTIONS: { value: PlaystyleType; label: string; color: string }[] = [
  { value: 'aggressive', label: 'AGR', color: '#EF4444' },
  { value: 'balanced',   label: 'BAL', color: '#06B6D4' },
  { value: 'defensive',  label: 'DEF', color: '#22C55E' },
];

const FOCUS_OPTIONS: { value: FocusType; label: string; color: string }[] = [
  { value: 'teamfight',  label: 'TF',  color: '#10B981' },
  { value: 'splitpush',  label: 'SP',  color: '#F59E0B' },
  { value: 'objective',  label: 'OBJ', color: '#06B6D4' },
];

function InlineTactics({ tactics, onChange }: { tactics: TacticsInfo; onChange: (p: PlaystyleType, f: FocusType) => void }) {
  return (
    <div className="flex items-center gap-3">
      <div className="flex items-center gap-1">
        <span className="text-[0.55rem] uppercase" style={{ color: 'var(--text-muted)' }}>Style</span>
        {PLAYSTYLE_OPTIONS.map((opt) => (
          <button
            key={opt.value}
            onClick={() => onChange(opt.value, tactics.focus)}
            className="px-1.5 py-0.5 rounded text-[0.6rem] font-bold cursor-pointer border transition-all duration-150"
            style={{
              backgroundColor: tactics.playstyle === opt.value ? `${opt.color}20` : 'transparent',
              borderColor: tactics.playstyle === opt.value ? opt.color : 'transparent',
              color: tactics.playstyle === opt.value ? opt.color : 'var(--text-muted)',
            }}
          >
            {opt.label}
          </button>
        ))}
      </div>
      <div className="flex items-center gap-1">
        <span className="text-[0.55rem] uppercase" style={{ color: 'var(--text-muted)' }}>Focus</span>
        {FOCUS_OPTIONS.map((opt) => (
          <button
            key={opt.value}
            onClick={() => onChange(tactics.playstyle, opt.value)}
            className="px-1.5 py-0.5 rounded text-[0.6rem] font-bold cursor-pointer border transition-all duration-150"
            style={{
              backgroundColor: tactics.focus === opt.value ? `${opt.color}20` : 'transparent',
              borderColor: tactics.focus === opt.value ? opt.color : 'transparent',
              color: tactics.focus === opt.value ? opt.color : 'var(--text-muted)',
            }}
          >
            {opt.label}
          </button>
        ))}
      </div>
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
        backgroundColor: isNew ? `${color}12` : 'transparent',
        borderLeft: isNew ? `2px solid ${color}` : '2px solid transparent',
      }}
    >
      <div
        className="w-10 text-right text-xs tabular-nums font-bold shrink-0 pt-0.5"
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
