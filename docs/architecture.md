# Architecture Overview

## Workspace Layout

The backend is a Rust workspace with six domain-driven crates:

```text
crates/
├── esm-core     # Game state, turn processing, calendar, event dispatcher, RNG, save/load
├── esm-models   # Domain entities (Player, Team, Champion, Staff, Contract), value objects
├── esm-engine   # Match simulation, draft, training, economy, meta/patch shifting
├── esm-db       # SQLite schema, migrations, repositories, snapshot saving
├── esm-ai       # Draft AI, tactical AI, transfer negotiation, opponent profiling
└── esm-data     # JSON schema validation, data pack loading, mod support
```

## Dependency Graph

```text
esm-data ──► esm-models
esm-db   ──► esm-models
esm-ai   ──► esm-core, esm-models
esm-engine ──► esm-core, esm-models
esm-core ──► esm-models
```

`esm-models` is the leaf dependency — it has no internal workspace deps and contains
pure domain types with no I/O, no side effects, and no framework coupling.

## Key Design Decisions

### Determinism

All game logic is deterministic. The RNG engine in `esm-core` uses a seed-controlled
PRNG. Given the same seed and the same sequence of inputs, every simulation produces
identical results.

### Turn-Based Tick System

The game advances via discrete daily ticks. Each day is subdivided into phases
(Morning, Afternoon, Evening). Weekly ticks aggregate daily state for board
evaluations, staff reports, and standings.

### Event-Driven Match Simulation

Matches generate chronological events via weighted Monte Carlo sampling —
no spatial/2D simulation. Events are resolved by comparing composite team/player
attributes against RNG rolls.

### Clean Architecture

Domain logic lives in `esm-models` and `esm-core`. Infrastructure (SQLite, JSON I/O,
file system) is isolated in `esm-db` and `esm-data`. The `esm-engine` orchestrates
simulation rules. The `esm-ai` encapsulates all AI decision-making.
