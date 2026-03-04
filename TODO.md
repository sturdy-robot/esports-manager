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

### 🔧 In Progress

- [ ] Wire EntityId into core entities (Player, Team, Champion, Staff) — deferred to DB layer
- [ ] SQLite schema & migrations (`esm-db`)

### 📋 Backlog — 0.1.0-alpha

- [ ] JSON data pack loading & validation (`esm-data`)
- [ ] Transfer & contract orchestration (`esm-engine` + `esm-ai`)
- [ ] Staff influence hooks into simulation (`esm-engine`)

### 📋 Backlog — Future

- [ ] Tauri desktop integration
- [ ] React frontend (Vite + TailwindCSS)
- [ ] Modding / custom data pack ecosystem
- [ ] RTS / FPS game mode support

## Test Coverage

- **248 tests** across the workspace, 0 failures
- `esm-core`: 92 tests (calendar, RNG, game state, inbox, turn processor, board, save/load)
- `esm-models`: 84 tests (player, manager, champion, team, staff, contract, entity ID)
- `esm-engine`: 64 tests (activity, match simulation, draft, economy)
- `esm-ai`: 8 tests (draft AI evaluation and selection)
