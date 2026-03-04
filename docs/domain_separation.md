# Domain Separation: eSports-Specific vs Shared

## Problem

The current architecture mixes MOBA-specific concepts (champions, lanes, roles like Top/Jungle/Mid)
with potentially shared concepts (manager, staff, contracts, economy). When RTS and FPS eSports
are added later, having everything in a flat namespace will require painful untangling.

## Solution

Separate the domain into **shared** (cross-eSport) and **game-specific** (MOBA, future RTS, FPS) layers.

### Shared Domain (`esm-models`)

These types are universal across all eSports:

- `Manager` — identity, archetype, reputation
- `Staff` — coaches, scouts, psychologists
- `Contract` — salary, duration, negotiation pipeline
- `EntityId` / `IdGenerator`
- `BoundedAttribute` — generic 0-100 clamped value

### MOBA Domain (`esm-models::moba`)

These types are specific to MOBA simulation:

- `MobaPlayer` — MOBA-specific attributes (mechanics, vision control, teamfighting, etc.)
- `MobaRole` — Top, Jungle, Mid, Bot, Support
- `MobaTeam` — roster of MobaPlayers
- `Champion` — class, scaling, tags (all MOBA concepts)
- `MasteryLevel` — champion mastery

### Database Schema

```sql
-- Shared tables
managers, staff, contracts

-- MOBA-specific tables (prefixed)
moba_players, moba_teams, moba_champions, moba_champion_tags, moba_tournaments
```

### Multi-Role Players

Players have a `primary_role` and zero or more `secondary_roles`.
When playing off-role, a skill penalty applies based on how far the role is
from their primary expertise.

## Match Simulation Architecture

The match simulation tracks full game state per the MOBA rules:

- **Map state**: towers per lane (3 tiers + inhibitor + 2 nexus), inhibitor respawn timers
- **Objective state**: dragon type/number, herald availability, baron availability, elder dragon
- **Player state**: gold, farm (CS), kills/deaths/assists, death timer, kill/death streak
- **Team state**: total gold, towers destroyed, dragons taken, barons taken
- **Event resolution**: individual player attributes + champion scaling + gold advantage
- **Commentary**: each event generates shoutcaster-style text
- **Real-time**: simulation yields state per tick for frontend rendering with speed control
- **Tactical input**: user can change team tactics mid-match
