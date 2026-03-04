# eSports Manager — Task Tracker

## Current Sprint: Foundation & Core Systems

### ✅ Done

- [x] Initialize Rust workspace with 6 crates (core, models, engine, db, ai, data)
- [x] Create project documentation structure (docs/, TODO.md)

### 🔧 In Progress

- [ ] Time & Calendar system (`esm-core`) — daily tick, phases, weekly tick
- [ ] Base Player model (`esm-models`) — attributes, dynamic state modifiers

### 📋 Backlog — 0.1.0-alpha

- [ ] Manager creation model (name, nickname, birthdate, nationality, archetype)
- [ ] Team model (roster, staff, finances)
- [ ] Champion model (attributes, tags, scaling)
- [ ] Seed-controlled RNG engine (`esm-core`)
- [ ] Event dispatcher / inbox system (`esm-core`)
- [ ] Match simulation engine — Monte Carlo event system (`esm-engine`)
- [ ] Draft system — state machine + AI evaluation (`esm-ai`)
- [ ] SQLite schema & migrations (`esm-db`)
- [ ] Save/Load orchestration (`esm-core` + `esm-db`)
- [ ] JSON data pack loading & validation (`esm-data`)
- [ ] Training & activity scheduling (`esm-engine`)
- [ ] Transfer & contract state machine (`esm-engine` + `esm-ai`)
- [ ] Economy / financial engine (`esm-engine`)
- [ ] Board expectations & manager satisfaction (`esm-core`)
- [ ] Staff influence system (`esm-engine`)

### 📋 Backlog — Future

- [ ] Tauri desktop integration
- [ ] React frontend (Vite + TailwindCSS)
- [ ] Modding / custom data pack ecosystem
- [ ] RTS / FPS game mode support
