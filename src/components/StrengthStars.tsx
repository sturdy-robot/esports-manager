import { useLocalization } from "@fluent/react";

const FULL = "★";
const EMPTY = "☆";

/** A 1–5 strength from the backend, as stars with a spoken equivalent. */
export function StrengthStars({ stars }: { stars: number }) {
  const { l10n } = useLocalization();
  return (
    <span
      role="img"
      aria-label={l10n.getString("team-strength", { stars })}
      className="text-warning tracking-widest"
    >
      {FULL.repeat(stars) + EMPTY.repeat(5 - stars)}
    </span>
  );
}
