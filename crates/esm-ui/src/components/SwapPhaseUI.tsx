import { useState, useCallback } from "react";
import { ArrowLeftRight, Check, UserRound } from "lucide-react";
import type {
  DraftSessionState,
  DraftPlayerInfo,
  ChampionDraftInfo,
} from "@/lib/api";
import { classColor, roleShort } from "./draftUiShared";

interface SwapPhaseUIProps {
  draftState: DraftSessionState;
  playerSide: "blue" | "red";
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

  const players: DraftPlayerInfo[] =
    playerSide === "blue" ? draftState.blue_players : draftState.red_players;
  const picks: string[] =
    playerSide === "blue" ? draftState.blue_picks : draftState.red_picks;
  const details = draftState.champion_details;
  const teamName =
    playerSide === "blue"
      ? draftState.blue_team_name
      : draftState.red_team_name;
  const accent = playerSide === "blue" ? "#3B82F6" : "#EF4444";
  const bgTint = playerSide === "blue" ? "rgba(59,130,246," : "rgba(239,68,68,";

  const handleSlotClick = useCallback(
    async (idx: number) => {
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
    },
    [selected, swapping, onSwap],
  );

  return (
    <div className="flex flex-col items-center justify-center flex-1 gap-4 p-4">
      <div
        className="w-full max-w-2xl rounded-2xl border overflow-hidden shadow-lg"
        style={{
          background: `linear-gradient(180deg, ${bgTint}0.14) 0%, rgba(20,20,31,0.96) 18%, rgba(20,20,31,1) 100%)`,
          borderColor: "var(--border-subtle)",
          boxShadow: `0 10px 30px ${bgTint}0.16)`,
        }}
      >
        <div
          className="px-4 py-3 border-b"
          style={{
            borderColor: `${bgTint}0.22)`,
            background: `linear-gradient(135deg, ${bgTint}0.16), rgba(255,255,255,0.02))`,
          }}
        >
          <div className="flex items-center justify-between gap-3 flex-wrap">
            <div className="flex items-center gap-3 min-w-0">
              <div
                className="h-10 w-10 rounded-xl border flex items-center justify-center shrink-0"
                style={{
                  borderColor: `${bgTint}0.38)`,
                  backgroundColor: `${bgTint}0.16)`,
                  color: accent,
                }}
              >
                <ArrowLeftRight size={18} />
              </div>
              <div className="min-w-0">
                <div
                  className="text-[0.62rem] font-semibold uppercase"
                  style={{
                    color: "var(--text-muted)",
                    letterSpacing: "0.18em",
                  }}
                >
                  Swap Phase
                </div>
                <h2
                  className="text-lg font-bold"
                  style={{ color: "var(--text-primary)" }}
                >
                  Champion Swap
                </h2>
              </div>
            </div>
            <div
              className="rounded-full px-2.5 py-1 text-[0.62rem] font-bold uppercase tracking-[0.16em] shrink-0"
              style={{
                color: accent,
                backgroundColor: `${bgTint}0.14)`,
                border: `1px solid ${bgTint}0.28)`,
              }}
            >
              {selected !== null ? "Select target" : "Ready"}
            </div>
          </div>
          <p
            className="text-sm mt-2"
            style={{ color: "var(--text-secondary)" }}
          >
            Click two players to swap their champions, then confirm.
          </p>
        </div>

        <div className="p-3">
          <div
            className="w-full rounded-2xl border overflow-hidden"
            style={{
              backgroundColor: "rgba(255,255,255,0.02)",
              borderColor: "rgba(255,255,255,0.06)",
            }}
          >
            <div
              className="px-4 py-3 border-b"
              style={{
                borderColor: `${bgTint}0.2)`,
                background: `linear-gradient(135deg, ${bgTint}0.1), rgba(255,255,255,0.02))`,
              }}
            >
              <div className="flex items-center justify-between gap-3">
                <div className="flex items-center gap-2 min-w-0">
                  <div
                    className="h-9 w-9 rounded-xl border flex items-center justify-center shrink-0"
                    style={{
                      borderColor: `${bgTint}0.34)`,
                      backgroundColor: `${bgTint}0.14)`,
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
                      Your Team
                    </div>
                    <div
                      className="text-sm font-bold truncate"
                      style={{ color: "var(--text-primary)" }}
                    >
                      {teamName || "Your Team"}
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

            <div className="flex flex-col gap-2 px-2 py-2">
              {Array.from({ length: 5 }, (_, i) => {
                const player = players[i] ?? null;
                const champ = picks[i] ?? null;
                const info: ChampionDraftInfo | undefined = champ
                  ? details?.[champ]
                  : undefined;
                const isSelected = selected === i;

                return (
                  <button
                    key={i}
                    onClick={() => handleSlotClick(i)}
                    disabled={!champ}
                    className="flex items-center gap-3 w-full px-4 py-3 rounded-xl border transition-all duration-150"
                    style={{
                      borderColor: isSelected
                        ? accent
                        : "rgba(255,255,255,0.06)",
                      background: isSelected
                        ? `linear-gradient(135deg, ${bgTint}0.16), rgba(255,255,255,0.03))`
                        : "linear-gradient(135deg, rgba(255,255,255,0.04) 0%, rgba(255,255,255,0.015) 100%)",
                      cursor: champ ? "pointer" : "default",
                      boxShadow: isSelected
                        ? `0 0 0 1px ${accent}20 inset`
                        : "inset 0 1px 0 rgba(255,255,255,0.03)",
                      opacity: champ ? 1 : 0.55,
                    }}
                  >
                    <div
                      className="w-10 h-6 rounded-md text-[0.65rem] font-semibold flex items-center justify-center shrink-0 shadow-sm shadow-black/20"
                      style={{
                        backgroundColor: `${bgTint}0.16)`,
                        color: accent,
                      }}
                    >
                      {player ? roleShort(player.role) : `P${i + 1}`}
                    </div>

                    <div className="flex flex-col min-w-0 flex-1 text-left">
                      <span
                        className="text-sm font-semibold truncate"
                        style={{ color: "var(--text-primary)" }}
                      >
                        {player?.nickname ?? `Player ${i + 1}`}
                      </span>

                      {champ ? (
                        <div className="flex items-center gap-2 mt-1 min-w-0 flex-wrap">
                          <span
                            className="text-[0.75rem] font-bold truncate"
                            style={{ color: "var(--text-primary)" }}
                          >
                            {champ}
                          </span>
                          {info && (
                            <span
                              className="px-1.5 py-0.5 rounded-full text-[0.55rem] font-semibold shrink-0"
                              style={{
                                color: classColor(info.class),
                                backgroundColor: `${classColor(info.class)}18`,
                              }}
                            >
                              {info.class}
                            </span>
                          )}
                        </div>
                      ) : (
                        <span
                          className="text-[0.65rem] mt-1"
                          style={{ color: "var(--text-muted)" }}
                        >
                          No champion locked
                        </span>
                      )}
                    </div>

                    {isSelected && (
                      <ArrowLeftRight
                        size={16}
                        style={{ color: accent, flexShrink: 0 }}
                      />
                    )}
                  </button>
                );
              })}
            </div>
          </div>
        </div>
      </div>

      {selected !== null && (
        <div
          className="rounded-full px-3 py-1.5 text-xs font-semibold animate-pulse"
          style={{ color: accent, backgroundColor: `${bgTint}0.12)` }}
        >
          Select another player to swap with
        </div>
      )}

      <button
        onClick={onConfirm}
        disabled={swapping}
        className="px-8 py-3 rounded-lg font-bold text-white flex items-center gap-2 transition-all duration-150"
        style={{
          background: "linear-gradient(135deg, #10B981, #06B6D4)",
          boxShadow: "0 0 16px rgba(6,182,212,0.3)",
          opacity: swapping ? 0.5 : 1,
        }}
      >
        <Check size={18} />
        Confirm Lineup
      </button>
    </div>
  );
}
