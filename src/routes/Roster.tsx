import { Localized, useLocalization } from "@fluent/react";
import { Link } from "@tanstack/react-router";
import type { RosterPlayer, TeamHeader as TeamHeaderView } from "../bindings";
import { Crest } from "../components/Crest";
import { FailureList } from "../components/Failure";
import { NationalityBadge } from "../components/NationalityBadge";
import { usePersonName } from "../components/PersonName";
import { backLink } from "../components/styles";
import { useRoster } from "../hooks/game";
import { failureMessages } from "../lib/backend/result";
import { ContentText } from "../lib/i18n/content";

/** The team's name and league, under its crest. */
function TeamHeader({ team }: { team: TeamHeaderView }) {
  return (
    <div className="flex items-center gap-3">
      <Crest crest={team.crest} />
      <div className="flex flex-col">
        <h2 className="font-semibold text-xl">
          <ContentText id={team.nameMessage} />
        </h2>
        <p className="text-muted">
          <ContentText id={team.leagueMessage ?? team.regionMessage} />
        </p>
      </div>
    </div>
  );
}

/** The roster: starters, then subs, as a table on wide screens and cards on narrow ones. */
export function Roster({ team }: { team?: string | undefined }) {
  const { l10n } = useLocalization();
  const roster = useRoster(team);
  if (roster.isError) {
    return (
      <FailureList
        heading={l10n.getString("app-backend-error")}
        messages={failureMessages(roster.error)}
      />
    );
  }
  if (!roster.isSuccess) {
    return (
      <Localized id="app-loading">
        <p className="text-muted" />
      </Localized>
    );
  }
  const view = roster.data;
  return (
    <section className="mx-auto flex max-w-5xl flex-col gap-4">
      <Localized id="nav-back">
        <Link to="/" className={backLink} />
      </Localized>
      <TeamHeader team={view.team} />
      <PlayerGroup heading="roster-starters" players={view.starters} />
      <PlayerGroup heading="roster-subs" players={view.subs} />
    </section>
  );
}

function PlayerGroup({
  heading,
  players,
}: {
  heading: "roster-starters" | "roster-subs";
  players: RosterPlayer[];
}) {
  const { l10n } = useLocalization();
  const name = usePersonName();
  const title =
    heading === "roster-starters"
      ? l10n.getString("roster-starters")
      : l10n.getString("roster-subs");
  return (
    <div className="flex flex-col gap-2">
      <h3 className="font-semibold text-lg">{title}</h3>
      <table className="hidden w-full border-collapse text-left sm:table">
        <caption className="sr-only">{title}</caption>
        <thead className="text-muted text-sm">
          <tr>
            <Localized id="roster-column-role">
              <th scope="col" className="py-2 font-normal" />
            </Localized>
            <Localized id="roster-column-nickname">
              <th scope="col" className="py-2 font-normal" />
            </Localized>
            <Localized id="roster-column-name">
              <th scope="col" className="py-2 font-normal" />
            </Localized>
            <Localized id="roster-column-nationality">
              <th scope="col" className="py-2 font-normal" />
            </Localized>
            <Localized id="roster-column-age">
              <th scope="col" className="py-2 text-right font-normal" />
            </Localized>
            <Localized id="roster-column-overall">
              <th scope="col" className="py-2 text-right font-normal" />
            </Localized>
          </tr>
        </thead>
        <tbody>
          {players.map((player) => (
            <tr key={player.id} className="border-border border-t">
              <td className="py-2">
                <ContentText id={player.roleMessage} />
              </td>
              <td className="py-2">
                <PlayerLink player={player} />
              </td>
              <td className="py-2">{name(player)}</td>
              <td className="py-2">
                <NationalityBadge nationality={player.nationality} />
              </td>
              <td className="py-2 text-right tabular-nums">{player.age}</td>
              <td className="py-2 text-right font-semibold tabular-nums">{player.overall}</td>
            </tr>
          ))}
        </tbody>
      </table>
      <ul className="flex flex-col gap-2 sm:hidden">
        {players.map((player) => (
          <li key={player.id} className="rounded-md border border-border bg-surface-raised p-3">
            <div className="flex items-center justify-between gap-2">
              <span>
                <ContentText id={player.roleMessage} />
                {" · "}
                <PlayerLink player={player} />
              </span>
              <span className="font-semibold tabular-nums">{player.overall}</span>
            </div>
            <div className="mt-1 flex items-center gap-2 text-muted text-sm">
              <span>{name(player)}</span>
              <NationalityBadge nationality={player.nationality} />
              <span className="tabular-nums">{player.age}</span>
            </div>
          </li>
        ))}
      </ul>
    </div>
  );
}

function PlayerLink({ player }: { player: RosterPlayer }) {
  const { l10n } = useLocalization();
  return (
    <Link
      to="/player"
      search={{ id: player.id }}
      aria-label={l10n.getString("roster-view-player", { nickname: player.nickname })}
      className="inline-flex min-h-11 items-center font-semibold text-accent-text underline-offset-4 hover:underline focus-visible:outline-2 focus-visible:outline-accent-text"
    >
      {player.nickname}
    </Link>
  );
}
