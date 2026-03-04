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

### 🔧 In Progress

- [ ] Economy / financial basics (`esm-engine`)
- [ ] Save/Load serialization roundtrip (`esm-core`)

### 📋 Backlog — 0.1.0-alpha

- [ ] Entity ID system (unique IDs for DB/cross-references)
- [ ] SQLite schema & migrations (`esm-db`)
- [ ] JSON data pack loading & validation (`esm-data`)
- [ ] Transfer & contract orchestration (`esm-engine` + `esm-ai`)
- [ ] Draft AI evaluation logic (`esm-ai`)
- [ ] Staff influence hooks into simulation (`esm-engine`)

### 📋 Backlog — Future

- [ ] Tauri desktop integration
- [ ] React frontend (Vite + TailwindCSS)
- [ ] Modding / custom data pack ecosystem
- [ ] RTS / FPS game mode support

## Test Coverage

- **208 tests** across the workspace, 0 failures
- `esm-core`: 82 tests (calendar, RNG, game state, inbox, turn processor, board)
- `esm-models`: 74 tests (player, manager, champion, team, staff, contract)
- `esm-engine`: 52 tests (activity, match simulation, draft)
