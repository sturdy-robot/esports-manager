import { UserRound } from "lucide-react";
import type { ChampionDraftInfo } from "@/lib/api";
import { classColor, roleShort } from "./draftUiShared";

interface DraftPlayerInfo {
  nickname: string;
  role: string;
}

interface DraftTeamPanelProps {
  side: "blue" | "red";
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
  const accent = side === "blue" ? "#3B82F6" : "#EF4444";
  const bgTint = side === "blue" ? "rgba(59,130,246," : "rgba(239,68,68,";
  const sideLabel = side === "blue" ? "Blue" : "Red";
  const slots = Array.from({ length: 5 }, (_, i) => ({
    player: players[i] ?? null,
    champion: picks[i] ?? null,
  }));

  return (
    <div
      className="flex flex-col w-72 shrink-0 rounded-2xl border overflow-hidden shadow-lg"
      style={{
        background:
          side === "blue"
            ? "linear-gradient(180deg, rgba(59,130,246,0.14) 0%, rgba(20,20,31,0.96) 18%, rgba(20,20,31,1) 100%)"
            : "linear-gradient(180deg, rgba(239,68,68,0.14) 0%, rgba(20,20,31,0.96) 18%, rgba(20,20,31,1) 100%)",
        borderColor: isActive ? accent : "var(--border-subtle)",
        boxShadow: isActive
          ? `0 10px 30px ${bgTint}0.22)`
          : side === "blue"
            ? "0 10px 30px rgba(59,130,246,0.1)"
            : "0 10px 30px rgba(239,68,68,0.1)",
        transition: "border-color 0.3s, box-shadow 0.3s",
      }}
    >
      <div
        className="px-4 py-3 border-b"
        style={{
          borderColor: `${bgTint}0.24)`,
          background: `linear-gradient(135deg, ${bgTint}0.16), rgba(255,255,255,0.02))`,
        }}
      >
        <div className="flex items-center justify-between gap-3">
          <div className="flex items-center gap-2 min-w-0">
            <div
              className="h-9 w-9 rounded-xl border flex items-center justify-center shrink-0"
              style={{
                borderColor: `${bgTint}0.4)`,
                backgroundColor: `${bgTint}0.16)`,
                color: accent,
              }}
            >
              <UserRound size={16} />
            </div>
            <div className="min-w-0">
              <div
                className="text-[0.6rem] font-semibold uppercase tracking-[0.22em]"
                style={{ color: "var(--text-muted)" }}
              >
                {sideLabel} Side
              </div>
              <div
                className="text-sm font-bold truncate"
                style={{ color: "var(--text-primary)" }}
              >
                {teamName}
              </div>
            </div>
          </div>
          <div
            className="rounded-full px-2.5 py-1 text-[0.6rem] font-bold uppercase tracking-[0.16em] shrink-0"
            style={{
              color: accent,
              backgroundColor: `${bgTint}0.14)`,
              border: `1px solid ${bgTint}0.28)`,
            }}
          >
            {picks.filter(Boolean).length}/5
          </div>
        </div>
      </div>

      <div className="flex flex-col flex-1 px-2 py-2 gap-2">
        {slots.map((slot, i) => {
          const champInfo = slot.champion
            ? championDetails?.[slot.champion]
            : null;
          const isLocked = !!slot.champion;
          return (
            <div
              key={i}
              className="flex items-center gap-2 px-3 py-3 rounded-xl border"
              style={{
                borderColor: "rgba(255,255,255,0.06)",
                background: isLocked
                  ? `linear-gradient(135deg, ${bgTint}0.12), rgba(255,255,255,0.015))`
                  : "linear-gradient(135deg, rgba(255,255,255,0.04) 0%, rgba(255,255,255,0.015) 100%)",
                boxShadow: "inset 0 1px 0 rgba(255,255,255,0.03)",
              }}
            >
              <div
                className="w-10 h-6 rounded-md text-[0.6rem] font-semibold flex items-center justify-center shrink-0 shadow-sm shadow-black/20"
                style={{
                  backgroundColor: `${bgTint}0.18)`,
                  color: isLocked ? accent : "var(--text-muted)",
                }}
              >
                {slot.player ? roleShort(slot.player.role) : `P${i + 1}`}
              </div>

              <div className="flex flex-col min-w-0 flex-1">
                <span
                  className="text-xs font-semibold truncate"
                  style={{
                    color: isLocked
                      ? "var(--text-primary)"
                      : "var(--text-secondary)",
                  }}
                >
                  {slot.player?.nickname ?? `Player ${i + 1}`}
                </span>
                {isLocked ? (
                  <>
                    <div className="flex items-center gap-1.5 min-w-0 mt-1">
                      <span
                        className="text-[0.72rem] font-bold truncate"
                        style={{ color: "var(--text-primary)" }}
                      >
                        {slot.champion}
                      </span>
                      {champInfo && (
                        <span
                          className="px-1.5 py-0.5 rounded text-[0.5rem] font-semibold shrink-0"
                          style={{
                            color: classColor(champInfo.class),
                            backgroundColor: `${classColor(champInfo.class)}18`,
                          }}
                        >
                          {champInfo.class}
                        </span>
                      )}
                    </div>
                    {champInfo?.preferred_roles?.length ? (
                      <div className="flex gap-1 mt-1 flex-wrap">
                        {champInfo.preferred_roles
                          .slice(0, 3)
                          .map((role: string) => (
                            <span
                              key={role}
                              className="px-1.5 py-0.5 rounded text-[0.45rem] font-bold tracking-wide"
                              style={{
                                backgroundColor: `${bgTint}0.16)`,
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
                    style={{ color: "var(--text-muted)" }}
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
        className="px-3 py-3 border-t"
        style={{
          borderColor: "rgba(255,255,255,0.06)",
          backgroundColor: `${bgTint}0.04)`,
        }}
      >
        <div
          className="text-[0.55rem] font-semibold uppercase tracking-wider mb-1"
          style={{ color: "var(--text-muted)" }}
        >
          Bans
        </div>
        <div className="flex gap-1.5">
          {Array.from({ length: 5 }, (_, i) => {
            const champ = bans[i] ?? null;
            return (
              <div
                key={i}
                className="flex-1 h-8 rounded-lg flex items-center justify-center border text-[0.62rem] font-bold"
                style={{
                  backgroundColor: champ
                    ? `${bgTint}0.14)`
                    : "rgba(255,255,255,0.03)",
                  borderColor: champ
                    ? `${bgTint}0.32)`
                    : "rgba(255,255,255,0.08)",
                  color: champ ? accent : "var(--text-muted)",
                  textDecoration: champ ? "line-through" : "none",
                }}
              >
                {champ ? champ.slice(0, 4) : "—"}
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
}
