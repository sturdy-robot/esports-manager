# eSports Manager — Task Tracker

## Current Sprint: Foundation & Core Systems

### ✅ Done

- [x] Initialize Rust workspace with 6 crates (core, models, engine, db, ai, data)
- [x] Create project documentation structure (docs/, TODO.md, docs/architecture.md)
- [x] Time & Calendar system (`esm-core`) — daily tick, phases, weekly tick, leap years
- [x] Base Player model (`esm-models`) — BoundedAttribute, attributes (physical/mental/technical), dynamic state (stamina/morale/confidence)
- [x] Manager model (`esm-models`) — identity, nationality, archetype, reputation
- [x] Seed-controlled RNG engine (`esm-core`) — xorshift64, range generation, probability checks, deterministic forking
- [x] Champion model (`esm-models`) — class, scaling, tags, mastery levels with multipliers
- [x] Team model (`esm-models`) — roster, synergy, reputation, role-based player lookup
- [x] Staff model (`esm-models`) — 6 staff roles, skill attribute
- [x] Contract model (`esm-models`) — salary, buyout, daily tick with auto-expiry, negotiation state machine
- [x] GameState orchestrator (`esm-core`) — ties calendar, RNG, manager, teams, inbox
- [x] Inbox / Message system (`esm-core`) — 3 priority levels, 8 categories, can_continue gate
- [x] TurnProcessor (`esm-core`) — end-of-day logic, blocking gate, stamina recovery, auto-resolve actionable messages
- [x] Board expectations & manager satisfaction (`esm-core`) — weighted evaluation, warning at ≤30, termination at 0
- [x] Activity scheduling (`esm-engine`) — ScrimBlock/SoloQueue/RestDay, 4 daily slots, effect aggregation
- [x] Match simulation engine (`esm-engine`) — Monte Carlo events, 3 phases, power-weighted outcomes, gold tracking, deterministic
- [x] Draft system (`esm-engine`) — state machine, 3-ban/5-ban/Fearless formats, snake pick order, duplicate rejection
- [x] Economy system (`esm-engine`) — transactions, balance tracking, financial reporting, debt detection
- [x] Save/Load roundtrip (`esm-core`) — full GameState JSON serialization verified (calendar, RNG state, teams, inbox)
- [x] Entity ID system (`esm-models`) — EntityId value object, deterministic IdGenerator
- [x] Draft AI evaluation (`esm-ai`) — composite scoring (meta × mastery + synergy + counter), randomized selection

- [x] SQLite schema & migrations (`esm-db`) — refinery-managed, V1 schema with CHECK constraints
- [x] Repository layer (`esm-db`) — PlayerRow/TeamRow CRUD (insert, get, list, update, delete)
- [x] JSON data pack loading (`esm-data`) — parse + validate teams/players/champions with referential integrity
- [x] DataPack → Database import pipeline (`esm-db`) — transactional import of teams, players, champions with tag support
- [x] Tournament system (`esm-engine`) — RoundRobin/DoubleRoundRobin, Bo1/Bo3/Bo5, scheduling, standings
- [x] Transfer negotiation engine (`esm-engine`) — buyout evaluation, agent contract offer logic, negotiation state machine
- [x] Staff influence hooks (`esm-engine`) — delegation quality, training multiplier, morale buffer, stamina reduction, scouting accuracy
- [x] Patch/meta shifting system (`esm-engine`) — MetaTier (S–D), PatchModifier, PatchCycle with scheduled progression
- [x] Game loop integration tests (`esm-engine`) — full season simulation: advance days, play matches, record results, verify standings, deterministic replay

### ✅ Done — Game Session & Persistence

- [x] EsportType enum (esm-models) — Moba/Rts/Fps variants with as_str/FromStr
- [x] Wire EsportType into GameState (esm-core) — new field + accessor + update all call sites
- [x] String conversion traits — GameRng state/from_state, DayPhase, ManagerArchetype, MessagePriority, MessageCategory
- [x] V3 migration: `game_session` table + `inbox_messages` table with CHECK constraints
- [x] V4 migration: `teams_json` + `player_team_index` columns on game_session
- [x] Session repository (esm-db) — SessionRow/InboxMessageRow DTOs with CRUD
- [x] GameSession struct (esm-db) — save/load GameState to/from SQLite (calendar, RNG, manager, teams JSON, inbox)
- [x] SaveManager (esm-db) — saves folder, `saves.json` index, FNV-1a checksum validation

### �📋 Backlog — 0.1.0-alpha

- [ ] Wire EntityId into core entities (deferred to when DB integration demands it)
- [ ] Random generation: player names, team names, roster filling from config JSON files (esm-data)
- [ ] Separate config file schemas: teams.json, tournaments.json, player_names.json, etc.
- [ ] Full game loop: new game → data import → team selection → play season (CLI or Tauri)

### 📋 Backlog — Future

- [ ] Tauri desktop integration (scaffold in crates/esm-ui)
- [ ] React frontend (Vite + TailwindCSS + Broadcast Arena design)
- [ ] Modding / custom data pack ecosystem
- [ ] RTS / FPS game mode support

## Test Coverage

- **614 tests** across the workspace, 0 failures
- `esm-core`: 106 tests (calendar, RNG, game state, inbox, turn processor, board, save/load, string conversions)
- `esm-models`: 132 tests (player, manager, champion, team, staff, contract, entity ID, esport type)
- `esm-engine`: 272 tests (activity, match sim, draft, economy, tournament, transfer, staff influence, patch, game loop integration)
- `esm-ai`: 8 tests (draft AI evaluation and selection)
- `esm-db`: 81 tests (migrations V1–V4, schema, repository CRUD, session repository, GameSession save/load, SaveManager, data pack import)
- `esm-data`: 15 tests (JSON parsing, validation pipeline)
