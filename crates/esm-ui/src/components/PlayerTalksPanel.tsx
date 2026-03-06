import { useState, useEffect } from 'react';
import {
  Flame, Shield, Target, Coffee,
  Heart, Zap, TrendingUp, TrendingDown,
} from 'lucide-react';
import { usePlayerTalks } from '@/lib/use-api';
import type { TalkType, PlayerStateInfo } from '@/lib/api';

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

interface PlayerTalksPanelProps {
  onDone: () => void;
}

interface TalkOption {
  value: TalkType;
  label: string;
  icon: React.ReactNode;
  desc: string;
  color: string;
}

// ---------------------------------------------------------------------------
// Talk options
// ---------------------------------------------------------------------------

const TALKS: TalkOption[] = [
  {
    value: 'motivate',
    label: 'Motivate',
    icon: <Flame size={14} />,
    desc: 'Morale +8, Stamina -3',
    color: '#EF4444',
  },
  {
    value: 'calm',
    label: 'Calm Down',
    icon: <Shield size={14} />,
    desc: 'Morale +3, Stamina +2',
    color: '#06B6D4',
  },
  {
    value: 'strategize',
    label: 'Strategize',
    icon: <Target size={14} />,
    desc: 'Satisfaction +5, Morale +2',
    color: '#8B5CF6',
  },
  {
    value: 'rest',
    label: 'Rest',
    icon: <Coffee size={14} />,
    desc: 'Stamina +8, Morale -2',
    color: '#22C55E',
  },
];

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

export function PlayerTalksPanel({ onDone }: PlayerTalksPanelProps) {
  const { roster, refresh, applyTalk, loading } = usePlayerTalks();
  const [talkedSet, setTalkedSet] = useState<Set<number>>(new Set());

  useEffect(() => {
    refresh();
  }, [refresh]);

  const handleTalk = async (playerIndex: number, talk: TalkType) => {
    await applyTalk(playerIndex, talk);
    setTalkedSet((prev) => new Set(prev).add(playerIndex));
  };

  return (
    <div
      className="w-full max-w-3xl p-6 rounded-xl border"
      style={{
        backgroundColor: 'var(--bg-surface)',
        borderColor: 'var(--border-subtle)',
      }}
    >
      <h2
        className="text-xl font-bold text-center mb-1"
        style={{ color: 'var(--text-primary)' }}
      >
        Team Talk
      </h2>
      <p
        className="text-xs text-center mb-5"
        style={{ color: 'var(--text-secondary)' }}
      >
        Talk to your players between games. Each talk affects their state for the next game.
      </p>

      <div className="space-y-2 mb-6">
        {roster.map((player, i) => (
          <PlayerCard
            key={player.nickname}
            player={player}
            index={i}
            onTalk={handleTalk}
            disabled={loading || talkedSet.has(i)}
          />
        ))}
      </div>

      <div className="text-center">
        <button
          onClick={onDone}
          className="px-8 py-2.5 rounded-lg font-bold text-white cursor-pointer border-none"
          style={{
            background: 'linear-gradient(135deg, #06B6D4, #8B5CF6)',
          }}
        >
          Continue to Next Game
        </button>
      </div>
    </div>
  );
}

// ---------------------------------------------------------------------------
// Sub-components
// ---------------------------------------------------------------------------

function PlayerCard({
  player,
  index,
  onTalk,
  disabled,
}: {
  player: PlayerStateInfo;
  index: number;
  onTalk: (index: number, talk: TalkType) => void;
  disabled: boolean;
}) {
  return (
    <div
      data-testid="player-card"
      className="flex items-center gap-3 p-3 rounded-lg border"
      style={{
        backgroundColor: 'var(--bg-elevated)',
        borderColor: 'var(--border-subtle)',
      }}
    >
      {/* Player info */}
      <div className="w-28 shrink-0">
        <div className="flex items-center gap-1.5 mb-1">
          <span
            className="text-[10px] font-bold uppercase tracking-wider px-1.5 py-0.5 rounded"
            style={{
              backgroundColor: 'rgba(6,182,212,0.1)',
              color: 'var(--color-accent-cyan)',
            }}
          >
            {player.role}
          </span>
          <span
            className="text-sm font-bold truncate"
            style={{ color: 'var(--text-primary)' }}
          >
            {player.nickname}
          </span>
        </div>
        <ConfidenceBadge confidence={player.confidence} />
      </div>

      {/* Stats bars */}
      <div className="flex-1 flex gap-3 min-w-0">
        <StatBar label="STA" value={player.stamina} icon={<Zap size={10} />} color="#22C55E" />
        <StatBar label="MOR" value={player.morale} icon={<Heart size={10} />} color="#F59E0B" />
        <StatBar label="SAT" value={player.satisfaction} icon={<TrendingUp size={10} />} color="#8B5CF6" />
      </div>

      {/* Talk buttons */}
      <div className="flex gap-1 shrink-0">
        {TALKS.map((talk) => (
          <button
            key={talk.value}
            onClick={() => onTalk(index, talk.value)}
            disabled={disabled}
            title={`${talk.label}: ${talk.desc}`}
            className="flex items-center gap-1 px-2 py-1.5 rounded-md border text-[10px] font-bold cursor-pointer transition-all disabled:opacity-50"
            style={{
              backgroundColor: 'var(--bg-surface)',
              borderColor: 'var(--border-subtle)',
              color: talk.color,
            }}
          >
            {talk.icon}
            <span className="hidden xl:inline">{talk.label}</span>
          </button>
        ))}
      </div>
    </div>
  );
}

function StatBar({
  label,
  value,
  icon,
  color,
}: {
  label: string;
  value: number;
  icon: React.ReactNode;
  color: string;
}) {
  return (
    <div className="flex-1 min-w-0">
      <div className="flex items-center gap-1 mb-0.5">
        <span style={{ color }}>{icon}</span>
        <span className="text-[9px] font-bold uppercase tracking-wider" style={{ color: 'var(--text-muted)' }}>
          {label}
        </span>
        <span className="text-[10px] font-mono font-bold ml-auto" style={{ color: 'var(--text-primary)' }}>
          {value}
        </span>
      </div>
      <div
        className="h-1.5 rounded-full overflow-hidden"
        style={{ backgroundColor: 'var(--bg-base)' }}
      >
        <div
          className="h-full rounded-full transition-all duration-300"
          style={{
            width: `${value}%`,
            backgroundColor: color,
            opacity: value < 30 ? 1 : 0.8,
          }}
        />
      </div>
    </div>
  );
}

function ConfidenceBadge({ confidence }: { confidence: string }) {
  const config: Record<string, { label: string; color: string; icon: React.ReactNode }> = {
    slumping: { label: 'Slumping', color: '#EF4444', icon: <TrendingDown size={10} /> },
    neutral: { label: 'Neutral', color: 'var(--text-muted)', icon: null },
    confident: { label: 'Confident', color: '#22C55E', icon: <TrendingUp size={10} /> },
    hyped: { label: 'Hyped', color: '#F59E0B', icon: <Flame size={10} /> },
  };

  const c = config[confidence] ?? config.neutral;

  return (
    <div className="flex items-center gap-1">
      {c.icon && <span style={{ color: c.color }}>{c.icon}</span>}
      <span className="text-[9px] font-semibold" style={{ color: c.color }}>
        {c.label}
      </span>
    </div>
  );
}
