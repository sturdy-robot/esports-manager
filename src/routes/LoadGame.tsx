import { Localized, useLocalization } from "@fluent/react";
import { Link, useNavigate } from "@tanstack/react-router";
import type { DamageView, SaveSummary } from "../bindings";
import { FailureList } from "../components/Failure";
import { backLink } from "../components/styles";
import { useOpenSave, useSaves } from "../hooks/game";
import { failureMessages } from "../lib/backend/result";
import { ContentText, messageText } from "../lib/i18n/content";

const DAMAGE: Record<DamageView, string> = {
  notASave: "save-damage-not-a-save",
  newerVersion: "save-damage-newer-version",
  inUse: "save-damage-in-use",
};

/** Load Game: every save, newest first; damaged ones are shown but can't be opened. */
export function LoadGame() {
  const { l10n } = useLocalization();
  const saves = useSaves();
  const open = useOpenSave();
  const navigate = useNavigate();
  // Dates follow the UI's language; the frontend may format, never compute.
  const locale = document.documentElement.lang || undefined;
  const formatDate = new Intl.DateTimeFormat(locale, { dateStyle: "medium", timeZone: "UTC" });
  const formatPlayed = new Intl.DateTimeFormat(locale, { dateStyle: "medium", timeStyle: "short" });

  const choose = (file: string) =>
    open.mutate(file, { onSuccess: () => navigate({ to: "/roster" }) });

  return (
    <section className="mx-auto flex max-w-5xl flex-col gap-4">
      <div className="flex items-center gap-4">
        <Localized id="nav-back">
          <Link to="/" className={backLink} />
        </Localized>
        <h2 className="font-semibold text-xl">{l10n.getString("load-game-title")}</h2>
      </div>
      {saves.isSuccess && saves.data.length === 0 && (
        <Localized id="load-game-empty">
          <p className="text-muted" />
        </Localized>
      )}
      {saves.isSuccess && saves.data.length > 0 && (
        <ul className="flex flex-col gap-2">
          <li
            aria-hidden="true"
            className="hidden grid-cols-5 gap-x-4 px-3 text-muted text-sm sm:grid"
          >
            <Localized id="load-game-column-name">
              <span />
            </Localized>
            <Localized id="load-game-column-manager">
              <span />
            </Localized>
            <Localized id="load-game-column-team">
              <span />
            </Localized>
            <Localized id="load-game-column-date">
              <span />
            </Localized>
            <Localized id="load-game-column-played">
              <span />
            </Localized>
          </li>
          {saves.data.map((save) => (
            <li key={save.file}>
              <SaveRow
                save={save}
                onOpen={choose}
                gameDate={(date) => formatDate.format(new Date(`${date}T00:00:00Z`))}
                played={(seconds) => formatPlayed.format(new Date(seconds * 1000))}
              />
            </li>
          ))}
        </ul>
      )}
      {(saves.isError || open.isError) && (
        <FailureList
          heading={l10n.getString("load-game-failed")}
          messages={failureMessages(saves.error ?? open.error)}
        />
      )}
    </section>
  );
}

interface SaveRowProps {
  save: SaveSummary;
  onOpen: (file: string) => void;
  gameDate: (date: string) => string;
  played: (seconds: number) => string;
}

function SaveRow({ save, onOpen, gameDate, played }: SaveRowProps) {
  const { l10n } = useLocalization();
  if (save.status === "damaged") {
    return (
      <div className="flex min-h-11 flex-wrap items-center gap-x-4 rounded-md border border-border p-3 text-muted">
        <span className="font-mono">{save.file}</span>
        <span>
          {l10n.getString("load-game-damaged", { damage: messageText(l10n, DAMAGE[save.damage]) })}
        </span>
      </div>
    );
  }
  return (
    <button
      type="button"
      onClick={() => onOpen(save.file)}
      className="grid min-h-11 w-full grid-cols-1 gap-x-4 rounded-md border border-border bg-surface-raised p-3 text-left hover:border-accent focus-visible:outline-2 focus-visible:outline-accent-text sm:grid-cols-5"
    >
      <span className="font-semibold">{save.name}</span>
      <span>{save.managerName}</span>
      <span>
        <ContentText id={save.teamMessage} />
      </span>
      <span className="text-muted">{gameDate(save.gameDate)}</span>
      <span className="text-muted">{played(save.lastPlayedAt)}</span>
    </button>
  );
}
