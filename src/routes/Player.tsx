import { Localized, useLocalization } from "@fluent/react";
import { Link } from "@tanstack/react-router";
import type { AttributeView } from "../bindings";
import { FailureList } from "../components/Failure";
import { NationalityBadge } from "../components/NationalityBadge";
import { usePersonName } from "../components/PersonName";
import { backLink } from "../components/styles";
import { usePlayer } from "../hooks/game";
import { failureMessages } from "../lib/backend/result";
import { ContentText, messageText } from "../lib/i18n/content";

/** One player: identity, overall, and attributes by group; hidden ones are unknown. */
export function Player({ id }: { id: string }) {
  const { l10n } = useLocalization();
  const profile = usePlayer(id);
  const name = usePersonName();
  if (profile.isError) {
    return (
      <FailureList
        heading={l10n.getString("app-backend-error")}
        messages={failureMessages(profile.error)}
      />
    );
  }
  if (!profile.isSuccess) {
    return (
      <Localized id="app-loading">
        <p className="text-muted" />
      </Localized>
    );
  }
  const { player, team, birthDate, skills, traits, hidden } = profile.data;
  const locale = document.documentElement.lang || undefined;
  const born = new Intl.DateTimeFormat(locale, { dateStyle: "long", timeZone: "UTC" }).format(
    new Date(`${birthDate}T00:00:00Z`),
  );
  return (
    <section className="mx-auto flex max-w-3xl flex-col gap-4">
      <Localized id="profile-back">
        <Link to="/roster" search={{ team: team.id }} className={backLink} />
      </Localized>
      <div className="flex flex-wrap items-baseline justify-between gap-2">
        <h2 className="font-semibold text-2xl">{player.nickname}</h2>
        <Localized id="profile-overall" vars={{ overall: player.overall }}>
          <p className="font-semibold text-xl" />
        </Localized>
      </div>
      <p className="flex flex-wrap items-center gap-x-2 text-muted">
        <span>{name(player)}</span>
        <span>
          <ContentText id={player.roleMessage} />
        </span>
        <Localized id="profile-age" vars={{ age: player.age }}>
          <span />
        </Localized>
        <span>
          {l10n.getString("profile-nationality", {
            country: messageText(l10n, player.nationality.nameMessage),
            code: player.nationality.code,
          })}
        </span>
        <NationalityBadge nationality={player.nationality} />
      </p>
      <p className="text-muted">
        <ContentText id={team.nameMessage} />
        {" · "}
        {l10n.getString("profile-born", { date: born })}
      </p>
      <div className="grid gap-6 sm:grid-cols-2">
        <AttributeGroup heading={l10n.getString("profile-skills")} attributes={skills} />
        <div className="flex flex-col gap-6">
          <AttributeGroup heading={l10n.getString("profile-traits")} attributes={traits} />
          <AttributeGroup heading={l10n.getString("profile-hidden")} attributes={hidden} />
        </div>
      </div>
    </section>
  );
}

function AttributeGroup({ heading, attributes }: { heading: string; attributes: AttributeView[] }) {
  const { l10n } = useLocalization();
  const unknown = (
    <>
      <span aria-hidden="true">{l10n.getString("profile-unknown")}</span>
      <span className="sr-only">{l10n.getString("profile-unknown-label")}</span>
    </>
  );
  return (
    <div>
      <h3 className="mb-2 font-semibold">{heading}</h3>
      <dl className="flex flex-col">
        {attributes.map((attribute) => (
          <div
            key={attribute.nameMessage}
            className="flex justify-between border-border border-t py-1"
          >
            <dt>
              <ContentText id={attribute.nameMessage} />
            </dt>
            <dd className="font-semibold tabular-nums">{attribute.value ?? unknown}</dd>
          </div>
        ))}
      </dl>
    </div>
  );
}
