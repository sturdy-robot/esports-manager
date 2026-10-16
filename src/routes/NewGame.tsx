import { Localized, useLocalization } from "@fluent/react";
import { Link, useNavigate } from "@tanstack/react-router";
import { useId, useState } from "react";
import type { RegionOption } from "../bindings";
import { Crest } from "../components/Crest";
import { FailureList } from "../components/Failure";
import { StrengthStars } from "../components/StrengthStars";
import { backLink, primaryButton, secondaryButton } from "../components/styles";
import { useNewGameOptions, useStartNewGame } from "../hooks/game";
import { failureMessages } from "../lib/backend/result";
import { ContentText, messageText } from "../lib/i18n/content";

type Step = 1 | 2 | 3;

/**
 * New Game, in three steps: the manager's name, the team, then the save's
 * name (new game spec). Going back keeps what was entered; every rule is the
 * backend's, so problems show when the game is started.
 */
export function NewGame() {
  const { l10n } = useLocalization();
  const options = useNewGameOptions();
  const start = useStartNewGame();
  const navigate = useNavigate();
  const [step, setStep] = useState<Step>(1);
  const [manager, setManager] = useState("");
  const [region, setRegion] = useState(0);
  const [team, setTeam] = useState<string | undefined>(undefined);
  const [saveName, setSaveName] = useState<string | undefined>(undefined);
  const [seed, setSeed] = useState("");
  const ids = { manager: useId(), save: useId(), seed: useId(), seedHint: useId() };

  const regions = options.data?.regions ?? [];
  const chosen = regions.flatMap((r) => r.teams).find((t) => t.id === team);
  const defaultSaveName = chosen === undefined ? "" : messageText(l10n, chosen.nameMessage);

  const submit = () =>
    start.mutate(
      {
        managerName: manager,
        team: team ?? "",
        saveName: saveName ?? defaultSaveName,
        seed: seed.trim() === "" ? null : seed,
      },
      { onSuccess: () => navigate({ to: "/roster" }) },
    );

  return (
    <section className="mx-auto flex max-w-5xl flex-col gap-4">
      <div className="flex flex-wrap items-center gap-x-4 gap-y-1">
        {step === 1 ? (
          <Localized id="nav-back">
            <Link to="/" className={backLink} />
          </Localized>
        ) : (
          <Localized id="nav-back">
            <button
              type="button"
              className={backLink}
              onClick={() => setStep((step - 1) as Step)}
            />
          </Localized>
        )}
        <h2 className="font-semibold text-xl">{l10n.getString("new-game-title")}</h2>
        <Localized id="new-game-progress" vars={{ step }}>
          <p className="text-muted" />
        </Localized>
      </div>
      <h3 className="font-semibold text-lg">
        {step === 1 && l10n.getString("new-game-step-manager")}
        {step === 2 && l10n.getString("new-game-step-team")}
        {step === 3 && l10n.getString("new-game-step-confirm")}
      </h3>

      {step === 1 && (
        <form
          className="flex max-w-sm flex-col gap-2"
          onSubmit={(event) => {
            event.preventDefault();
            setStep(2);
          }}
        >
          <label htmlFor={ids.manager}>{l10n.getString("new-game-manager-label")}</label>
          <input
            id={ids.manager}
            value={manager}
            onChange={(event) => setManager(event.target.value)}
            autoComplete="off"
            className="min-h-11 rounded-md border border-border bg-surface-panel px-3"
          />
          <Localized id="nav-next">
            <button type="submit" className={primaryButton} />
          </Localized>
        </form>
      )}

      {step === 2 && (
        <TeamStep
          regions={regions}
          region={region}
          onRegion={setRegion}
          team={team}
          onTeam={setTeam}
          onNext={() => setStep(3)}
        />
      )}

      {step === 3 && (
        <form
          className="flex max-w-sm flex-col gap-2"
          onSubmit={(event) => {
            event.preventDefault();
            submit();
          }}
        >
          <label htmlFor={ids.save}>{l10n.getString("new-game-save-label")}</label>
          <input
            id={ids.save}
            value={saveName ?? defaultSaveName}
            onChange={(event) => setSaveName(event.target.value)}
            autoComplete="off"
            className="min-h-11 rounded-md border border-border bg-surface-panel px-3"
          />
          <details className="mt-2">
            <Localized id="new-game-advanced">
              <summary className="min-h-11 cursor-pointer py-2 text-muted" />
            </Localized>
            <div className="flex flex-col gap-2">
              <label htmlFor={ids.seed}>{l10n.getString("new-game-seed-label")}</label>
              <input
                id={ids.seed}
                value={seed}
                inputMode="numeric"
                aria-describedby={ids.seedHint}
                onChange={(event) => setSeed(event.target.value)}
                className="min-h-11 rounded-md border border-border bg-surface-panel px-3"
              />
              <Localized id="new-game-seed-hint">
                <p id={ids.seedHint} className="text-muted text-sm" />
              </Localized>
            </div>
          </details>
          {start.isPending ? (
            <Localized id="new-game-starting">
              <p role="status" className="text-muted" />
            </Localized>
          ) : (
            <Localized id="new-game-start">
              <button type="submit" className={primaryButton} />
            </Localized>
          )}
          {start.isError && (
            <FailureList
              heading={l10n.getString("new-game-failed")}
              messages={failureMessages(start.error)}
            />
          )}
        </form>
      )}

      {options.isError && (
        <FailureList
          heading={l10n.getString("new-game-failed")}
          messages={failureMessages(options.error)}
        />
      )}
    </section>
  );
}

interface TeamStepProps {
  regions: RegionOption[];
  region: number;
  onRegion: (index: number) => void;
  team: string | undefined;
  onTeam: (id: string) => void;
  onNext: () => void;
}

/** Region tabs, and a radio group of team cards (arrow keys move, Space picks). */
function TeamStep({ regions, region, onRegion, team, onTeam, onNext }: TeamStepProps) {
  const { l10n } = useLocalization();
  const shown = regions[region];
  const groupName = useId();
  return (
    <div className="flex flex-col gap-4">
      <div
        role="tablist"
        aria-label={l10n.getString("new-game-region-tabs")}
        className="flex flex-wrap gap-2"
      >
        {regions.map((option, index) => (
          <button
            key={option.id}
            type="button"
            role="tab"
            aria-selected={index === region}
            className={index === region ? primaryButton : secondaryButton}
            onClick={() => onRegion(index)}
          >
            <ContentText id={option.leagueMessage ?? option.nameMessage} />
          </button>
        ))}
      </div>
      {shown !== undefined && (
        <div
          role="radiogroup"
          aria-label={messageText(l10n, shown.leagueMessage ?? shown.nameMessage)}
          className="grid grid-cols-1 gap-3 sm:grid-cols-3 lg:grid-cols-5"
        >
          {shown.teams.map((option) => (
            <label
              key={option.id}
              className="flex min-h-11 cursor-pointer items-center gap-3 rounded-md border border-border bg-surface-raised p-3 has-[:checked]:border-accent has-[:focus-visible]:outline-2 has-[:focus-visible]:outline-accent-text"
            >
              <input
                type="radio"
                name={groupName}
                value={option.id}
                checked={team === option.id}
                onChange={() => onTeam(option.id)}
                className="sr-only"
              />
              <Crest crest={option.crest} />
              <span className="flex flex-col">
                <span className="font-semibold">
                  <ContentText id={option.nameMessage} />
                </span>
                <StrengthStars stars={option.strength} />
              </span>
            </label>
          ))}
        </div>
      )}
      <div>
        <Localized id="nav-next">
          <button
            type="button"
            className={primaryButton}
            disabled={team === undefined}
            onClick={onNext}
          />
        </Localized>
      </div>
    </div>
  );
}
