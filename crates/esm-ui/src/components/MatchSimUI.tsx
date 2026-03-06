import { Swords, Shield, Flame, Crown, Castle, Trophy, Zap, ChevronRight } from 'lucide-react';
import type { SimulateMatchResult, MatchEventInfo } from '@/lib/api';

interface MatchSimUIProps {
  result: SimulateMatchResult;
  onComplete: () => void;
}

function formatMinute(minute: number): string {
  return `${minute}:00`;
}

function formatGold(gold: number): string {
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

export function MatchSimUI({ result, onComplete }: MatchSimUIProps) {
  return (
    <div className="flex flex-col w-full h-full gap-4 max-w-5xl mx-auto">
      {/* Scoreboard header */}
      <div
        className="flex items-center justify-between p-4 rounded-xl border"
        style={{
          backgroundColor: 'var(--bg-surface)',
          borderColor: 'var(--border-subtle)',
        }}
      >
        <div className="text-center flex-1">
          <div
            className="text-lg font-bold"
            style={{ color: '#3B82F6' }}
          >
            {result.blue_team}
          </div>
          <div
            className="text-2xl font-mono font-black"
            style={{ color: 'var(--text-primary)' }}
          >
            {formatGold(result.blue_gold)}
          </div>
          <div className="text-xs" style={{ color: 'var(--text-muted)' }}>gold</div>
        </div>

        <div className="text-center px-6">
          <div
            className="text-xs font-semibold uppercase tracking-wider mb-1"
            style={{ color: 'var(--color-win)' }}
          >
            Victory
          </div>
          <div
            className="text-xl font-bold"
            style={{ color: 'var(--text-primary)' }}
          >
            {result.winner}
          </div>
          <div
            className="text-sm font-mono mt-1"
            style={{ color: 'var(--text-secondary)' }}
          >
            {formatMinute(result.duration_minutes)}
          </div>
        </div>

        <div className="text-center flex-1">
          <div
            className="text-lg font-bold"
            style={{ color: '#EF4444' }}
          >
            {result.red_team}
          </div>
          <div
            className="text-2xl font-mono font-black"
            style={{ color: 'var(--text-primary)' }}
          >
            {formatGold(result.red_gold)}
          </div>
          <div className="text-xs" style={{ color: 'var(--text-muted)' }}>gold</div>
        </div>
      </div>

      {/* Event log */}
      <div
        className="flex-1 rounded-xl border overflow-y-auto p-3"
        style={{
          backgroundColor: 'var(--bg-surface)',
          borderColor: 'var(--border-subtle)',
        }}
      >
        <div className="space-y-1">
          {result.events.map((event, i) => (
            <EventRow key={i} event={event} />
          ))}
        </div>
      </div>

      {/* Continue button */}
      <div className="flex justify-center pb-2">
        <button
          onClick={onComplete}
          className="px-8 py-3 rounded-lg font-bold text-white flex items-center gap-2"
          style={{
            background: 'linear-gradient(135deg, #06B6D4, #8B5CF6)',
          }}
        >
          <ChevronRight size={18} />
          Continue
        </button>
      </div>
    </div>
  );
}

function EventRow({ event }: { event: MatchEventInfo }) {
  const color = eventColor(event.kind);
  const phaseColor = phaseBadgeColor(event.phase);

  return (
    <div
      className="flex items-start gap-3 px-3 py-2 rounded-lg transition-colors"
      style={{ backgroundColor: 'transparent' }}
    >
      {/* Timestamp */}
      <div
        className="w-12 text-right text-xs font-mono font-bold shrink-0 pt-0.5"
        style={{ color: 'var(--text-muted)' }}
      >
        {formatMinute(event.minute)}
      </div>

      {/* Phase badge */}
      <div
        className="text-xs font-semibold px-1.5 py-0.5 rounded shrink-0"
        style={{
          color: phaseColor,
          backgroundColor: `${phaseColor}15`,
        }}
      >
        {event.phase}
      </div>

      {/* Icon */}
      <div className="shrink-0 pt-0.5" style={{ color }}>
        {eventIcon(event.kind)}
      </div>

      {/* Commentary */}
      <div
        className="text-sm flex-1"
        style={{ color: 'var(--text-primary)' }}
      >
        {event.commentary ?? event.kind}
      </div>
    </div>
  );
}
