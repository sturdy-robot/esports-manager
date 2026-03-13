import { useMemo, useState } from "react";
import { Search, X } from "lucide-react";
import type { ChampionClass, ChampionDraftInfo } from "@/lib/api";
import {
  ALL_CLASSES,
  ALL_ROLES,
  classColor,
  masteryColor,
  masteryShort,
  metaTierColor,
  roleShort,
  scalingColor,
} from "./draftUiShared";

interface DraftChampionGridProps {
  availableChampions: string[];
  activeHover: string | null;
  isPlayerTurn: boolean;
  championDetails: Record<string, ChampionDraftInfo>;
  onHover: (champion: string) => void;
}

export function DraftChampionGrid({
  availableChampions,
  activeHover,
  isPlayerTurn,
  championDetails,
  onHover,
}: DraftChampionGridProps) {
  const [searchText, setSearchText] = useState("");
  const [classFilter, setClassFilter] = useState<ChampionClass | null>(null);
  const [roleFilter, setRoleFilter] = useState<string | null>(null);

  const filteredChampions = useMemo(() => {
    return availableChampions.filter((champ) => {
      const info = championDetails?.[champ];
      if (
        searchText &&
        !champ.toLowerCase().includes(searchText.toLowerCase())
      ) {
        return false;
      }
      if (classFilter && info && info.class !== classFilter) {
        return false;
      }
      if (roleFilter && info && !info.preferred_roles?.includes(roleFilter)) {
        return false;
      }
      return true;
    });
  }, [
    availableChampions,
    championDetails,
    searchText,
    classFilter,
    roleFilter,
  ]);

  return (
    <div
      className="flex flex-col flex-1 rounded-2xl border overflow-hidden shadow-lg"
      style={{
        background:
          "linear-gradient(180deg, rgba(6,182,212,0.08) 0%, rgba(20,20,31,0.96) 14%, rgba(20,20,31,1) 100%)",
        borderColor: "var(--border-subtle)",
        boxShadow: "0 10px 30px rgba(6,182,212,0.08)",
      }}
    >
      <div
        className="flex flex-wrap items-center gap-2 px-4 py-3 border-b shrink-0"
        style={{
          borderColor: "rgba(6,182,212,0.14)",
          background:
            "linear-gradient(135deg, rgba(6,182,212,0.12), rgba(255,255,255,0.02))",
        }}
      >
        <div
          className="flex items-center gap-1.5 px-3 py-2 rounded-xl border flex-1 min-w-56"
          style={{
            backgroundColor: "rgba(255,255,255,0.03)",
            borderColor: searchText
              ? "var(--color-accent-cyan)"
              : "var(--border-subtle)",
            boxShadow: searchText
              ? "0 0 0 1px rgba(6,182,212,0.18) inset"
              : "none",
          }}
        >
          <Search
            size={12}
            style={{ color: "var(--text-muted)", flexShrink: 0 }}
          />
          <input
            type="text"
            placeholder="Search champion..."
            value={searchText}
            onChange={(e) => setSearchText(e.target.value)}
            className="bg-transparent text-xs outline-none flex-1 min-w-0"
            style={{ color: "var(--text-primary)" }}
          />
          {searchText && (
            <button onClick={() => setSearchText("")} className="shrink-0">
              <X size={10} style={{ color: "var(--text-muted)" }} />
            </button>
          )}
        </div>

        <div className="flex gap-1 flex-wrap">
          {ALL_CLASSES.map((cls) => {
            const isActive = classFilter === cls;
            return (
              <button
                key={cls}
                onClick={() => setClassFilter(isActive ? null : cls)}
                className="px-2.5 py-1 rounded-full text-[0.6rem] font-semibold transition-all duration-150 border"
                style={{
                  backgroundColor: isActive
                    ? "rgba(6,182,212,0.15)"
                    : "rgba(255,255,255,0.03)",
                  borderColor: isActive
                    ? classColor(cls)
                    : "var(--border-subtle)",
                  color: isActive ? classColor(cls) : "var(--text-muted)",
                }}
              >
                {cls}
              </button>
            );
          })}
        </div>

        <div className="flex gap-1 flex-wrap">
          {ALL_ROLES.map((role) => {
            const isActive = roleFilter === role;
            return (
              <button
                key={role}
                onClick={() => setRoleFilter(isActive ? null : role)}
                className="px-2.5 py-1 rounded-full text-[0.6rem] font-bold tracking-wide transition-all duration-150 border"
                style={{
                  backgroundColor: isActive
                    ? "rgba(16,185,129,0.15)"
                    : "rgba(255,255,255,0.03)",
                  borderColor: isActive
                    ? "var(--color-accent-emerald)"
                    : "var(--border-subtle)",
                  color: isActive
                    ? "var(--color-accent-emerald)"
                    : "var(--text-muted)",
                }}
              >
                {roleShort(role)}
              </button>
            );
          })}
        </div>

        <span
          className="text-[0.6rem] tabular-nums ml-auto shrink-0 rounded-full px-2.5 py-1 font-bold uppercase tracking-[0.16em]"
          style={{
            color: "var(--color-accent-cyan)",
            backgroundColor: "rgba(6,182,212,0.12)",
            border: "1px solid rgba(6,182,212,0.2)",
          }}
        >
          {filteredChampions.length}/{availableChampions.length}
        </span>
      </div>

      <div className="flex-1 p-4 overflow-y-auto relative">
        <div className="grid grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 gap-2.5">
          {filteredChampions.map((champ) => {
            const info = championDetails?.[champ];
            const isHovered = activeHover === champ;
            return (
              <button
                key={champ}
                aria-label={champ}
                data-hovered={isHovered ? "true" : "false"}
                disabled={!isPlayerTurn}
                onClick={() => onHover(champ)}
                className="flex flex-col items-stretch gap-2 p-3 rounded-2xl text-left transition-all duration-150 border h-36"
                style={{
                  background: isHovered
                    ? "linear-gradient(135deg, rgba(6,182,212,0.16), rgba(255,255,255,0.03))"
                    : "linear-gradient(135deg, rgba(255,255,255,0.05) 0%, rgba(255,255,255,0.02) 100%)",
                  borderColor: isHovered
                    ? "var(--color-accent-cyan)"
                    : "rgba(255,255,255,0.08)",
                  color: "var(--text-primary)",
                  opacity: isPlayerTurn ? 1 : 0.5,
                  cursor: isPlayerTurn ? "pointer" : "not-allowed",
                  boxShadow: isHovered
                    ? "0 0 16px rgba(6,182,212,0.16)"
                    : "inset 0 1px 0 rgba(255,255,255,0.03)",
                }}
              >
                <div className="flex items-start justify-between gap-2">
                  <div className="min-w-0">
                    <div className="font-bold text-sm leading-tight truncate">
                      {champ}
                    </div>
                    {info?.tags?.length ? (
                      <div
                        className="text-[0.55rem] uppercase tracking-wider mt-0.5 truncate"
                        style={{ color: "var(--text-muted)" }}
                      >
                        {info.tags.slice(0, 2).join(" · ")}
                      </div>
                    ) : null}
                  </div>
                  {info?.meta_tier ? (
                    <span
                      className="px-2 py-0.5 rounded-full text-[0.65rem] font-black shrink-0"
                      style={{
                        color: metaTierColor(info.meta_tier),
                        backgroundColor: `${metaTierColor(info.meta_tier)}18`,
                      }}
                    >
                      {info.meta_tier}
                    </span>
                  ) : null}
                </div>

                <div className="flex flex-wrap gap-1 min-h-[22px]">
                  {info?.preferred_roles?.length ? (
                    info.preferred_roles.map((role: string) => (
                      <span
                        key={`${champ}-${role}`}
                        className="px-1.5 py-0.5 rounded-full text-[0.55rem] font-bold tracking-wide"
                        style={{
                          backgroundColor: "rgba(16,185,129,0.12)",
                          color: "var(--color-accent-emerald)",
                        }}
                      >
                        {roleShort(role)}
                      </span>
                    ))
                  ) : (
                    <span
                      className="text-[0.55rem]"
                      style={{ color: "var(--text-muted)" }}
                    >
                      Flexible
                    </span>
                  )}
                </div>

                {info ? (
                  <div className="flex flex-wrap gap-1.5 mt-auto">
                    <span
                      className="px-1.5 py-0.5 rounded-full text-[0.55rem] font-semibold"
                      style={{
                        backgroundColor: `${classColor(info.class)}18`,
                        color: classColor(info.class),
                      }}
                    >
                      {info.class}
                    </span>
                    <span
                      className="px-1.5 py-0.5 rounded-full text-[0.55rem] font-semibold"
                      style={{
                        backgroundColor: `${scalingColor(info.scaling)}18`,
                        color: scalingColor(info.scaling),
                      }}
                    >
                      {info.scaling}
                    </span>
                    {info.best_mastery ? (
                      <span
                        className="px-1.5 py-0.5 rounded-full text-[0.55rem] font-bold"
                        style={{
                          backgroundColor: `${masteryColor(info.best_mastery)}18`,
                          color: masteryColor(info.best_mastery),
                        }}
                      >
                        {masteryShort(info.best_mastery)}
                      </span>
                    ) : null}
                  </div>
                ) : null}
              </button>
            );
          })}
          {filteredChampions.length === 0 && (
            <div
              className="col-span-full text-center py-8 text-sm"
              style={{ color: "var(--text-muted)" }}
            >
              No champions match the current filters
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
