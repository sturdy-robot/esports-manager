import { useState } from 'react';
import { Swords, Shield, Target, Users, Columns3, Crown } from 'lucide-react';
import type { PlaystyleType, FocusType, TacticsInfo } from '@/lib/api';

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

interface TacticsPanelProps {
  tactics: TacticsInfo;
  onConfirm: (playstyle: PlaystyleType, focus: FocusType) => void;
  /** Optional label for context, e.g. "Pre-Match" or "Phase Transition" */
  context?: string;
  /** If true, shows a compact inline version */
  compact?: boolean;
}

interface OptionDef<T extends string> {
  value: T;
  label: string;
  icon: React.ReactNode;
  desc: string;
  color: string;
}

// ---------------------------------------------------------------------------
// Option definitions
// ---------------------------------------------------------------------------

const PLAYSTYLES: OptionDef<PlaystyleType>[] = [
  {
    value: 'aggressive',
    label: 'Aggressive',
    icon: <Swords size={20} />,
    desc: 'More solo kills & teamfights. Higher risk, higher reward.',
    color: '#EF4444',
  },
  {
    value: 'balanced',
    label: 'Balanced',
    icon: <Target size={20} />,
    desc: 'Standard play. No particular emphasis.',
    color: '#06B6D4',
  },
  {
    value: 'defensive',
    label: 'Defensive',
    icon: <Shield size={20} />,
    desc: 'Focus on farming. Fewer fights, scale to late game.',
    color: '#22C55E',
  },
];

const FOCUSES: OptionDef<FocusType>[] = [
  {
    value: 'teamfight',
    label: 'Teamfight',
    icon: <Users size={20} />,
    desc: 'Group up and force 5v5 engages.',
    color: '#10B981',
  },
  {
    value: 'splitpush',
    label: 'Splitpush',
    icon: <Columns3 size={20} />,
    desc: 'Pressure side lanes. Take towers methodically.',
    color: '#F59E0B',
  },
  {
    value: 'objective',
    label: 'Objective',
    icon: <Crown size={20} />,
    desc: 'Prioritize dragons, baron, and herald.',
    color: '#06B6D4',
  },
];

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

export function TacticsPanel({ tactics, onConfirm, context, compact }: TacticsPanelProps) {
  const [playstyle, setPlaystyle] = useState<PlaystyleType>(tactics.playstyle);
  const [focus, setFocus] = useState<FocusType>(tactics.focus);

  const handleConfirm = () => {
    onConfirm(playstyle, focus);
  };

  if (compact) {
    return (
      <div
        className="flex flex-col gap-3 p-4 rounded-xl border"
        style={{
          backgroundColor: 'var(--bg-surface)',
          borderColor: 'var(--border-subtle)',
        }}
      >
        {context && (
          <div
            className="text-xs font-semibold uppercase tracking-wider text-center"
            style={{ color: 'var(--text-muted)' }}
          >
            {context}
          </div>
        )}

        <div className="flex gap-2">
          <div className="flex-1">
            <div className="text-[10px] font-semibold uppercase tracking-wider mb-1"
              style={{ color: 'var(--text-muted)' }}>
              Playstyle
            </div>
            <div className="flex gap-1">
              {PLAYSTYLES.map((opt) => (
                <CompactOption
                  key={opt.value}
                  selected={playstyle === opt.value}
                  onClick={() => setPlaystyle(opt.value)}
                  icon={opt.icon}
                  label={opt.label}
                  color={opt.color}
                />
              ))}
            </div>
          </div>

          <div className="flex-1">
            <div className="text-[10px] font-semibold uppercase tracking-wider mb-1"
              style={{ color: 'var(--text-muted)' }}>
              Focus
            </div>
            <div className="flex gap-1">
              {FOCUSES.map((opt) => (
                <CompactOption
                  key={opt.value}
                  selected={focus === opt.value}
                  onClick={() => setFocus(opt.value)}
                  icon={opt.icon}
                  label={opt.label}
                  color={opt.color}
                />
              ))}
            </div>
          </div>
        </div>

        <button
          onClick={handleConfirm}
          className="w-full py-2 rounded-lg text-xs font-bold text-white cursor-pointer border-none"
          style={{
            background: 'linear-gradient(135deg, #10B981, #06B6D4)',
          }}
        >
          Confirm &amp; Continue
        </button>
      </div>
    );
  }

  return (
    <div
      className="w-full max-w-2xl p-8 rounded-xl border"
      style={{
        backgroundColor: 'var(--bg-surface)',
        borderColor: 'var(--border-subtle)',
      }}
    >
      {context && (
        <div
          className="text-xs font-semibold uppercase tracking-wider text-center mb-2"
          style={{ color: 'var(--text-muted)' }}
        >
          {context}
        </div>
      )}

      <h2
        className="text-2xl font-bold mb-1 text-center"
        style={{ color: 'var(--text-primary)' }}
      >
        Match Tactics
      </h2>
      <p className="text-sm text-center mb-6" style={{ color: 'var(--text-secondary)' }}>
        Choose your team&apos;s approach for this game.
      </p>

      {/* Playstyle */}
      <div className="mb-6">
        <div
          className="text-xs font-semibold uppercase tracking-wider mb-3"
          style={{ color: 'var(--text-muted)' }}
        >
          Playstyle
        </div>
        <div className="grid grid-cols-3 gap-3">
          {PLAYSTYLES.map((opt) => (
            <TacticCard
              key={opt.value}
              selected={playstyle === opt.value}
              onClick={() => setPlaystyle(opt.value)}
              icon={opt.icon}
              label={opt.label}
              desc={opt.desc}
              color={opt.color}
            />
          ))}
        </div>
      </div>

      {/* Focus */}
      <div className="mb-8">
        <div
          className="text-xs font-semibold uppercase tracking-wider mb-3"
          style={{ color: 'var(--text-muted)' }}
        >
          Strategic Focus
        </div>
        <div className="grid grid-cols-3 gap-3">
          {FOCUSES.map((opt) => (
            <TacticCard
              key={opt.value}
              selected={focus === opt.value}
              onClick={() => setFocus(opt.value)}
              icon={opt.icon}
              label={opt.label}
              desc={opt.desc}
              color={opt.color}
            />
          ))}
        </div>
      </div>

      {/* Confirm */}
      <div className="text-center">
        <button
          onClick={handleConfirm}
          className="px-8 py-3 rounded-lg font-bold text-white cursor-pointer border-none transition-all"
          style={{
            background: 'linear-gradient(135deg, #10B981, #06B6D4)',
          }}
        >
          Confirm Tactics &amp; Proceed
        </button>
      </div>
    </div>
  );
}

// ---------------------------------------------------------------------------
// Sub-components
// ---------------------------------------------------------------------------

function TacticCard({
  selected,
  onClick,
  icon,
  label,
  desc,
  color,
}: {
  selected: boolean;
  onClick: () => void;
  icon: React.ReactNode;
  label: string;
  desc: string;
  color: string;
}) {
  return (
    <button
      onClick={onClick}
      className="flex flex-col items-center gap-2 p-4 rounded-lg border cursor-pointer transition-all text-center"
      style={{
        backgroundColor: selected ? `${color}15` : 'var(--bg-elevated)',
        borderColor: selected ? color : 'var(--border-subtle)',
        boxShadow: selected ? `0 0 12px ${color}30` : 'none',
      }}
    >
      <div style={{ color: selected ? color : 'var(--text-muted)' }}>
        {icon}
      </div>
      <div
        className="text-sm font-bold"
        style={{ color: selected ? color : 'var(--text-primary)' }}
      >
        {label}
      </div>
      <div
        className="text-[11px] leading-tight"
        style={{ color: 'var(--text-secondary)' }}
      >
        {desc}
      </div>
    </button>
  );
}

function CompactOption({
  selected,
  onClick,
  icon,
  label,
  color,
}: {
  selected: boolean;
  onClick: () => void;
  icon: React.ReactNode;
  label: string;
  color: string;
}) {
  return (
    <button
      onClick={onClick}
      className="flex-1 flex flex-col items-center gap-1 py-2 px-1 rounded-md border cursor-pointer transition-all"
      style={{
        backgroundColor: selected ? `${color}15` : 'var(--bg-elevated)',
        borderColor: selected ? color : 'transparent',
      }}
    >
      <div style={{ color: selected ? color : 'var(--text-muted)' }}>
        {icon}
      </div>
      <div
        className="text-[10px] font-bold"
        style={{ color: selected ? color : 'var(--text-secondary)' }}
      >
        {label}
      </div>
    </button>
  );
}
