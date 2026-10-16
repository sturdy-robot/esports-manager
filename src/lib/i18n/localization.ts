import { FluentBundle, FluentResource } from "@fluent/bundle";
import { ReactLocalization } from "@fluent/react";
import { type Locale, negotiateLocales } from "./locales";

const sources = import.meta.glob<string>("../../locales/*/*.ftl", {
  query: "?raw",
  import: "default",
  eager: true,
});

/** All `.ftl` sources of one locale, in file-name order. */
function sourcesFor(locale: Locale): string[] {
  const prefix = `../../locales/${locale}/`;
  return Object.keys(sources)
    .filter((path) => path.startsWith(prefix))
    .sort()
    .flatMap((path) => sources[path] ?? []);
}

export function createBundle(locale: Locale): FluentBundle {
  // No bidi isolation marks: every shipped locale is left-to-right.
  const bundle = new FluentBundle(locale, { useIsolating: false });
  for (const source of sourcesFor(locale)) {
    const errors = bundle.addResource(new FluentResource(source));
    if (errors.length > 0) {
      throw new Error(`Invalid Fluent messages for ${locale}: ${errors.join("; ")}`);
    }
  }
  return bundle;
}

/** The localization for the user's preferred languages, with English fallback. */
export function createLocalization(requested: readonly string[]): ReactLocalization {
  return new ReactLocalization(negotiateLocales(requested).map(createBundle));
}
