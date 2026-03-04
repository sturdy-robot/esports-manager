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

### ✅ Done — Frontend (Tauri + React)

- [x] Vitest + Testing Library test infrastructure (jsdom, test helpers)
- [x] Main Menu screen — New Game / Load Game / Settings / Exit with Broadcast Arena styling
- [x] App-level navigation state machine (main-menu → new-game → team-selection → load-game → settings → playing)
- [x] Manager Creation form — identity fields, esport type radio selector, validation, form data passthrough
- [x] Team Selection screen — card grid with badge/reputation/player count, selection glow
- [x] Load Game screen — save list, checksum badges, empty state, loading indicator, delete
- [x] Settings screen — saves folder path, theme toggle
- [x] TopBar with game clock (date + phase), team badge, Continue button, Save & Exit
- [x] GameShell layout — Sidebar + TopBar + content routing for all pages
- [x] Dashboard with props-driven KPI cards (record, streak, budget, inbox count)
- [x] Roster screen — player table with attributes, role badges, stamina/morale bars
- [x] Inbox screen — message list with Urgent/Action/Info priority badges, blocking indicator
- [x] Schedule screen — match cards with date, teams, result/VS, format badge
- [x] Standings screen — league table with W-L, map diff, streak, rank highlighting
- [x] Sidebar with Inbox nav item, collapsible, grouped navigation
- [x] React API hooks (useListSaves, useDeleteSave, useLoadSave, useNewGame, useLoadDatapack) with mock fallbacks
- [x] Full New Game pipeline wired: Manager Creation → Team Selection → createGame → playing
- [x] LoadGame wired to live saves list via hooks, delete + refresh, load with GameInfo
- [x] Tauri commands: new_game, list_saves, load_save, delete_save, get_game_info
- [x] TypeScript API layer (src/lib/api.ts) matching Rust DTOs
- [x] build_teams_from_datapack() helper in esm-db for DataPack→Team conversion

### 📋 Backlog — 0.1.0-alpha

- [ ] Wire EntityId into core entities (deferred to when DB integration demands it)
- [ ] Random generation: player names, team names, roster filling from config JSON files (esm-data)
- [ ] Separate config file schemas: teams.json, tournaments.json, player_names.json, etc.
- [ ] Full game loop: new game → data import → team selection → play season (CLI or Tauri)
- [ ] Advance turn command (Tauri → TurnProcessor → update UI state)
- [ ] Finances screen — budget overview, transactions list
- [ ] Results screen — past match results with scores

### 📋 Backlog — Future

- [ ] Modding / custom data pack ecosystem
- [ ] RTS / FPS game mode support

## Test Coverage

- **702 tests** across the workspace (618 Rust + 84 frontend), 0 failures
- `esm-core`: 106 tests (calendar, RNG, game state, inbox, turn processor, board, save/load, string conversions)
- `esm-models`: 132 tests (player, manager, champion, team, staff, contract, entity ID, esport type)
- `esm-engine`: 272 tests (activity, match sim, draft, economy, tournament, transfer, staff influence, patch, game loop integration)
- `esm-ai`: 8 tests (draft AI evaluation and selection)
- `esm-db`: 85 tests (migrations V1–V4, schema, repository CRUD, session repository, GameSession, SaveManager, import + build_teams)
- `esm-data`: 15 tests (JSON parsing, validation pipeline)
- `esm-ui`: 84 frontend tests (MainMenu, NewGame, LoadGame, TeamSelection, Settings, TopBar, Dashboard, Roster, Inbox, Schedule, Standings, App navigation)
