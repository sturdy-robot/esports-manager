# Attributes and overall

- **Prefix:** `ATTR`
- **Status:** Draft
- **Related:** [ADR 0002](../adr/0002-genre-seam.md), [condition](condition.md), [champions and mastery](champions-and-mastery.md)

## Summary

Attributes describe what a person can do and who they are. Every attribute must have at least one documented consumer in the simulation or in the development model; an attribute that nothing reads doesn't exist. The Python implementation had 13 skills that mostly collapsed into one "overall", and this spec exists to prevent that.

## Scale

- Shown on a 1–99 scale. Stored at finer precision (for example hundredths) so that small training gains accumulate without a visible jump every week.
- Three categories:
  - **skills:** trainable
  - **traits:** personality, slow to change
  - **hidden:** never shown raw; only estimated through scouting, with accuracy depending on staff

## Core attributes (`domain`, every genre)

| Attribute | Category | Consumed by |
|---|---|---|
| Teamwork | trait | Synergy accumulation, how well the player follows calls |
| Aggression | trait | How often fights are started, and the variance that brings. Neither good nor bad in itself |
| Composure | trait | Performance when behind or in high-stakes games, tilt resistance; the starting point of in-match composure |
| Work Ethic | trait | Training and solo-queue gains, tolerance of stamina costs |
| Consistency | hidden | How much performance varies from match to match |
| Potential | hidden | Ceiling of development |

## MOBA skills (`esports::moba`)

| Attribute | Consumed by |
|---|---|
| Mechanics | Execution in skirmishes and teamfights, outplay chance |
| Laning | CS rate, trades, lane pressure |
| Game Sense | Rotations, wave management, objective setup, jungle pathing |
| Awareness | Avoiding ganks and picks, vision use |
| Positioning | Survival and damage uptime in teamfights |
| Champion Pool Breadth | How fast new champions are learned, adaptation to patches |
| Shotcalling | Team coordination bonus when this player is the designated caller |

## Overall

- Overall is **for display and AI heuristics only**: squad screens, scouting, AI valuation of players. **The match simulation never reads it.**
- It is a weighted mean of attributes. The weights depend on the player's main role and are set in a table in package data, so they can be modded. The weights live in exactly one place.
- Read it like this: in the low 90s means elite; lower numbers mean weaker players.

## Open questions

- The exact role weight table.
- The development model: growth curves by age, potential and activity. This needs its own spec.
- Whether staff get their own attribute set (needed once staff arrive, after the MVP).
