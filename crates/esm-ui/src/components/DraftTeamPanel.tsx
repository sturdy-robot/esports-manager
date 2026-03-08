import type { ChampionDraftInfo } from '@/lib/api';
import { classColor, roleShort } from './draftUiShared';

interface DraftPlayerInfo {
  nickname: string;
  role: string;
}

interface DraftTeamPanelProps {
  side: 'blue' | 'red';
  teamName: string;
  players: DraftPlayerInfo[];
  picks: string[];
  bans: string[];
  isActive: boolean;
  championDetails: Record<string, ChampionDraftInfo>;
}

export function DraftTeamPanel({
  side,
  teamName,
  players,
  picks,
  bans,
  isActive,
  championDetails,
}: DraftTeamPanelProps) {
  const accent = side === 'blue' ? '#3B82F6' : '#EF4444';
  const bgTint = side === 'blue' ? 'rgba(59,130,246,' : 'rgba(239,68,68,';
  const slots = Array.from({ length: 5 }, (_, i) => ({
    player: players[i] ?? null,
    champion: picks[i] ?? null,
  }));

  return (
    <div
      className="flex flex-col w-56 shrink-0 rounded-xl border overflow-hidden"
      style={{
        backgroundColor: 'var(--bg-surface)',
        borderColor: isActive ? accent : 'var(--border-subtle)',
        boxShadow: isActive ? `0 0 16px ${bgTint}0.25)` : 'none',
        transition: 'border-color 0.3s, box-shadow 0.3s',
      }}
    >
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
              <div
                className="w-8 h-5 rounded text-[0.6rem] font-semibold flex items-center justify-center shrink-0"
                style={{
                  backgroundColor: `${bgTint}0.12)`,
                  color: isLocked ? accent : 'var(--text-muted)',
                }}
              >
                {slot.player ? roleShort(slot.player.role) : `P${i + 1}`}
              </div>

              <div className="flex flex-col min-w-0 flex-1">
                <span
                  className="text-xs font-semibold truncate"
                  style={{ color: isLocked ? 'var(--text-primary)' : 'var(--text-secondary)' }}
                >
                  {slot.player?.nickname ?? `Player ${i + 1}`}
                </span>
                {isLocked ? (
                  <>
                    <div className="flex items-center gap-1 min-w-0">
                      <span
                        className="text-[0.65rem] font-bold truncate"
                        style={{ color: accent }}
                      >
                        {slot.champion}
                      </span>
                      {champInfo && (
                        <span
                          className="text-[0.5rem]"
                          style={{ color: classColor(champInfo.class) }}
                        >
                          {champInfo.class}
                        </span>
                      )}
                    </div>
                    {champInfo?.preferred_roles?.length ? (
                      <div className="flex gap-1 mt-1 flex-wrap">
                        {champInfo.preferred_roles.slice(0, 3).map((role: string) => (
                          <span
                            key={role}
                            className="px-1.5 py-0.5 rounded text-[0.45rem] font-bold tracking-wide"
                            style={{
                              backgroundColor: `${bgTint}0.12)`,
                              color: accent,
                            }}
                          >
                            {roleShort(role)}
                          </span>
                        ))}
                      </div>
                    ) : null}
                  </>
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
                className="flex-1 h-5 rounded flex items-center justify-center text-[0.5rem] border"
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
