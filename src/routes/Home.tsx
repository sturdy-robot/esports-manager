import { Localized } from "@fluent/react";
import { useAbout } from "../hooks/useAbout";

/** Placeholder home screen until the walking skeleton lands. */
export function Home() {
  const about = useAbout();

  if (about.isPending) {
    return (
      <Localized id="app-loading">
        <p className="text-muted" />
      </Localized>
    );
  }
  if (about.isError) {
    return (
      <Localized id="app-backend-error">
        <p role="alert" className="text-negative" />
      </Localized>
    );
  }
  return (
    <Localized id="app-version" vars={{ version: about.data.version }}>
      <p data-testid="version" className="text-muted" />
    </Localized>
  );
}
