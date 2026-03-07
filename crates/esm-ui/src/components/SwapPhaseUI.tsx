import { useState, useCallback } from 'react';
import { ArrowLeftRight, Check } from 'lucide-react';
import type { DraftSessionState, DraftPlayerInfo, ChampionDraftInfo, ChampionClass } from '@/lib/api';

const ROLE_SHORT: Record<string, string> = {
  Top: 'TOP', Jungle: 'JNG', Mid: 'MID', Bot: 'BOT', Support: 'SUP',
};

function classColor(cls: ChampionClass): string {
  switch (cls) {
    case 'Tank':     return 'var(--color-info)';
    case 'Fighter':  return 'var(--color-warning)';
    case 'Assassin': return 'var(--color-loss)';
    case 'Mage':     return 'var(--color-accent-emerald)';
    case 'Marksman': return 'var(--color-accent-cyan)';
    case 'Support':  return 'var(--color-win)';
  }
}

interface SwapPhaseUIProps {
  draftState: DraftSessionState;
  playerSide: 'blue' | 'red';
  onSwap: (a: number, b: number) => Promise<void>;
  onConfirm: () => void;
}

export function SwapPhaseUI({
  draftState,
  playerSide,
  onSwap,
  onConfirm,
}: SwapPhaseUIProps) {
  const [selected, setSelected] = useState<number | null>(null);
  const [swapping, setSwapping] = useState(false);

  const players: DraftPlayerInfo[] = playerSide === 'blue'
    ? draftState.blue_players
    : draftState.red_players;
  const picks: string[] = playerSide === 'blue'
    ? draftState.blue_picks
    : draftState.red_picks;
  const details = draftState.champion_details;
  const teamName = playerSide === 'blue'
    ? draftState.blue_team_name
    : draftState.red_team_name;
  const accent = playerSide === 'blue' ? '#3B82F6' : '#EF4444';
  const bgTint = playerSide === 'blue' ? 'rgba(59,130,246,' : 'rgba(239,68,68,';

  const handleSlotClick = useCallback(async (idx: number) => {
    if (swapping) return;
    if (selected === null) {
      setSelected(idx);
    } else if (selected === idx) {
      setSelected(null);
    } else {
      setSwapping(true);
      await onSwap(selected, idx);
      setSelected(null);
      setSwapping(false);
    }
  }, [selected, swapping, onSwap]);

  return (
    <div className="flex flex-col items-center justify-center flex-1 gap-6 p-8">
      <div className="text-center">
        <h2
          className="text-xl font-bold mb-1"
          style={{ color: 'var(--text-primary)' }}
        >
          Champion Swap
        </h2>
        <p
          className="text-sm"
          style={{ color: 'var(--text-secondary)' }}
        >
          Click two players to swap their champions, then confirm.
        </p>
      </div>

      <div
        className="w-full max-w-md rounded-xl border overflow-hidden"
        style={{
          backgroundColor: 'var(--bg-surface)',
          borderColor: 'var(--border-subtle)',
        }}
      >
        {/* Team header */}
        <div
          className="px-4 py-3 text-center font-bold text-sm uppercase tracking-wider border-b"
          style={{
            color: accent,
            backgroundColor: `${bgTint}0.08)`,
            borderColor: `${bgTint}0.2)`,
          }}
        >
          {teamName || 'Your Team'}
        </div>

        {/* Player slots */}
        {Array.from({ length: 5 }, (_, i) => {
          const player = players[i] ?? null;
          const champ = picks[i] ?? null;
          const info: ChampionDraftInfo | undefined = champ ? details?.[champ] : undefined;
          const isSelected = selected === i;

          return (
            <button
              key={i}
              onClick={() => handleSlotClick(i)}
              disabled={!champ}
              className="flex items-center gap-3 w-full px-4 py-3 border-b last:border-b-0 transition-all duration-150"
              style={{
                borderColor: 'var(--border-subtle)',
                backgroundColor: isSelected
                  ? `${bgTint}0.15)`
                  : 'transparent',
                cursor: champ ? 'pointer' : 'default',
                boxShadow: isSelected ? `inset 0 0 0 2px ${accent}` : 'none',
              }}
            >
              {/* Role badge */}
              <div
                className="w-10 h-6 rounded text-[0.65rem] font-semibold flex items-center justify-center shrink-0"
                style={{
                  backgroundColor: `${bgTint}0.12)`,
                  color: accent,
                }}
              >
                {player
                  ? (ROLE_SHORT[player.role] ?? player.role.slice(0, 3).toUpperCase())
                  : `P${i + 1}`}
              </div>

              {/* Player name */}
              <span
                className="text-sm font-semibold flex-1 text-left truncate"
                style={{ color: 'var(--text-primary)' }}
              >
                {player?.nickname ?? `Player ${i + 1}`}
              </span>

              {/* Champion */}
              {champ && (
                <div className="flex items-center gap-2">
                  <span
                    className="text-sm font-bold"
                    style={{ color: accent }}
                  >
                    {champ}
                  </span>
                  {info && (
                    <span
                      className="text-[0.6rem] text-xs"
                      style={{ color: classColor(info.class as ChampionClass) }}
                    >
                      {info.class}
                    </span>
                  )}
                </div>
              )}

              {/* Swap indicator */}
              {isSelected && (
                <ArrowLeftRight
                  size={14}
                  style={{ color: accent, flexShrink: 0 }}
                />
              )}
            </button>
          );
        })}
      </div>

      {selected !== null && (
        <p
          className="text-xs animate-pulse"
          style={{ color: accent }}
        >
          Select another player to swap with
        </p>
      )}

      <button
        onClick={onConfirm}
        disabled={swapping}
        className="px-8 py-3 rounded-lg font-bold text-white flex items-center gap-2 transition-all duration-150"
        style={{
          background: 'linear-gradient(135deg, #10B981, #06B6D4)',
          boxShadow: '0 0 16px rgba(6,182,212,0.3)',
          opacity: swapping ? 0.5 : 1,
        }}
      >
        <Check size={18} />
        Confirm Lineup
      </button>
    </div>
  );
}
