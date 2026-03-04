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

### 🔧 In Progress

- [ ] Board expectations & manager satisfaction (`esm-core`)
- [ ] Activity scheduling types (`esm-engine`)

### 📋 Backlog — 0.1.0-alpha

- [ ] Match simulation engine — Monte Carlo event system (`esm-engine`)
- [ ] Draft system — state machine + AI evaluation (`esm-ai`)
- [ ] SQLite schema & migrations (`esm-db`)
- [ ] Save/Load orchestration (`esm-core` + `esm-db`)
- [ ] JSON data pack loading & validation (`esm-data`)
- [ ] Transfer & contract state machine (`esm-engine` + `esm-ai`)
- [ ] Economy / financial engine (`esm-engine`)
- [ ] Staff influence system (`esm-engine`)

### 📋 Backlog — Future

- [ ] Tauri desktop integration
- [ ] React frontend (Vite + TailwindCSS)
- [ ] Modding / custom data pack ecosystem
- [ ] RTS / FPS game mode support

## Test Coverage

- **142 tests** across the workspace, 0 failures
- `esm-core`: 68 tests (calendar, RNG, game state, inbox, turn processor)
- `esm-models`: 74 tests (player, manager, champion, team, staff, contract)
