# eSports Manager: Complete Design Document

## Vision

An open-source, offline-first eSports management simulation initially focused on a MOBA title (League of Legends-inspired). The game emphasizes deep managerial decision-making, deterministic and replayable match simulations, extensibility through data-driven design, and long-term career progression. Future support for RTS and FPS titles is architecturally supported through modular game rules and simulation engines.

## Technology Stack

* **Backend:** Rust (Core logic, deterministic seed-based PRNG, state management)
* **Desktop Runtime:** Tauri (Lightweight, cross-platform window management)
* **Frontend:** Vite + React + TypeScript + TailwindCSS (Modern, reactive UI)
* **Database:** SQLite (Self-contained, reliable transactional storage)
* **Serialization:** JSON (For data packs, mods, and configuration)

## Architecture

The backend leverages Domain-Driven Design, divided into highly specialized, decoupled Rust crates:

| Crate | Primary Responsibilities |
| --- | --- |
| **core** | Global game state, turn processing, calendar system, event dispatcher, seed-controlled RNG engine, save/load orchestration. |
| **models** | Domain entities (Player, Team, Champion, Staff, Contract), value objects (Morale, Stamina), shared traits, and validation logic. |
| **engine** | Match simulation engine, draft system, training progression, economic simulation, and meta/patch shifting simulation. |
| **db** | SQLite schema management, migrations, repository layer, snapshot/transactional saving, and save validation/checksums. |
| **ai** | Draft AI logic, real-time tactical AI behavior, staff assistant behaviors, transfer negotiation, and opponent strategic profiling. |
| **data** | JSON schema validation, data pack loading, and underlying mod support infrastructure. |

## Game Flow: Initialization & Setup

### 1. Manager Creation

Users define their core identity (Name, Nickname, Birthdate, Nationality) and select a **Manager Personality Archetype**, which provides specific gameplay modifiers:

* **Tactical Genius:** Bonuses to draft synergy and mid-match tactical adaptability.
* **Player Developer:** Accelerates youth academy growth and champion mastery progression.
* **Motivator:** Mitigates morale loss from defeats and boosts the "Confidence" win-streak mechanic.
* **Analyst:** Enhances scouting accuracy and opponent strategic profiling.
* **Balanced:** A jack-of-all-trades with slight bonuses across all categories.

### 2. Database Selection & Generation

Each game session is a self-contained SQLite file. Users can use a default curated database, import a custom database, or generate a new one via JSON definition packs (`teams.json`, `tournaments.json`, `patches.json`, `meta_profiles.json`, `sponsors.json`, etc.).

* **Validation Pipeline:** Custom databases undergo strict JSON schema validation, referential integrity checks, and attribute bounds validation. Failure prompts a corruption warning.
* **Procedural Fallback:** If a generated team lacks roster data, the engine utilizes the procedural youth generation system to fill the slots.
* **Database schema**: Database schema is defined in the `db` crate. It must have a specific schema, and MUST NOT use a JSON blob in the column.
* **Migrations**: The `db` crate provides a migration system that allows the game to upgrade the database schema over time. Migrations should prioritize backwards compatibility with previous saves.

### 3. Save System & Determinism

* **Transactional Staging:** Changes are staged in memory and written to SQLite upon a manual save or via an optional autosave toggle.
* **Index & Integrity:** The game maintains a `saves.json` index tracking metadata, timestamps, and checksums for quick UI loading. Full validation occurs upon loading the save.
* **Replayability:** Every save file captures the exact Game Version, Data Pack Version, and RNG Seed, ensuring match results and world events are perfectly reproducible.

## Core Gameplay Loop & Dashboard

Gameplay is turn-based, utilizing daily ticks, weekly reports, seasonal splits, and distinct off-season transfer windows.

**Turn Phases:**

1. **Inbox Resolution:** Addressing mandatory actions, media, and board requests.
2. **Training & Staff Adjustments:** Setting practice focuses and assigning staff.
3. **Roster Management:** Handling injuries, burnout, and rotations.
4. **Transfers / Contracts:** Negotiating with agents and managing the youth academy.
5. **Matchday / Scrims:** Engaging in the draft and match simulation.
6. **End Turn & World Tick:** Processing global events (other leagues simulating, AI transfers, retirements, and meta shifts).

**The Dashboard:**
The central hub provides immediate visibility into the upcoming match, player stamina overviews, morale indicators, injury status, financial snapshots, board expectations, fan sentiment, and staff reports.

## Player Dynamics & Meta Evolution

* **Attributes (0-100):** Categorized into Mental (Decision Making, Clutch, Focus, Discipline), Technical (Mechanics, Champion Mastery, Vision Control, Teamfighting), and Physical (Endurance, Reaction Time).
* **Stamina & Injuries:** Stamina depletes from matches and scrims. Low stamina increases error probability, reduces morale, and significantly raises the risk of injuries or long-term burnout.
* **Morale, Satisfaction & Confidence:** High morale influences clutch moments. Satisfaction dictates transfer requests and public statements. Confidence is a volatile, performance-based win-streak multiplier.
* **Team Synergy:** A dynamic rating calculated from shared playtime, communication stats, and personality compatibility. High synergy boosts objective coordination and comeback potential.
* **Patch & Meta System:** The game world evolves through seasonal patches. Patches buff/nerf champions, alter objective gold, and shift meta archetypes, forcing the manager and AI to constantly adapt their drafting and training priorities.

## Drafting & Match Simulation Engine

### Drafting System

Supports 3-ban, 5-ban, and Fearless Draft formats. The draft features timed decisions and temporary selection states.

* **AI Drafting Layers:** The `ai` crate evaluates champion strength, player mastery, counter-matchups, team composition synergy, and current meta alignment to make highly realistic, multi-layered draft decisions.

### Match Simulation & Real-Time Interventions

Matches are event-driven, utilizing Monte Carlo weighted outcomes constrained by the exact RNG seed.

* **Match Phases:** Divided into Early Game (0-10 min), Mid Game (10-25 min), and Late Game (25+ min), each dynamically modifying objective probabilities, teamfight frequency, and scaling factors.
* **Simulation Weights:** Outcomes are influenced by gold difference, player attributes, champion scaling, team synergy, stamina, draft quality scores, and mistake probability.
* **Tactical Interventions:** Managers actively spectating can issue real-time commands with associated risks and cooldowns: change objective focus, shift aggression level, force grouping, assign split pushes, or call risky Baron attempts.
* **AI Personalities:** Opposing AI teams exhibit specific playstyles (Aggressive, Scaling, Objective-control, Early Snowball, Reactive) and will adapt their strategies mid-series (Bo3, Bo5, Bo7).

## Economy, Transfers & Staff Management

* **Financial Engine:** Budgets encompass player/staff salaries, sponsorships, tournament winnings, merchandise sales, and fan growth impact. An optional Financial Fair Play (FFP) rule toggle is available.
* **Sponsorships:** Sponsors feature varying risk tolerances, specific performance clauses, and branding alignment. Failing their objectives directly reduces income.
* **Transfers & Scouting:** The transfer market includes multi-round negotiations, agent influence, player ambition levels, buyout clauses, and a dynamic free agency/youth academy pool. Scouting accuracy relies entirely on the assigned staff's skill, revealing hidden attributes, personality traits, and injury risks.
* **Staff Roles:** Managers can hire Assistant Coaches, Draft Analysts, Positional Coaches, Sports Psychologists, Scouts, and Financial Officers.

## UI/UX & Extensibility

* **Visual Analytics:** The UI will feature draft visualization grids, timeline match feeds, performance heatmaps, player progression graphs, and financial dashboards.
* **Accessibility:** Full keyboard shortcuts, adjustable simulation speeds, dark/light modes, and high-contrast modes.
* **Modding Ecosystem:** The `data` crate allows the community to inject custom data packs, champion packs, rule-set definitions, and custom simulation weight configurations.
* **Future Hooks:** The architecture inherently supports future multiplayer (async league modes), historical stat tracking, Hall of Fame systems, and dynamic narrative storylines.

## Systems

### 1. Time & Calendar System (The Game Loop)

The game operates on a deterministic tick-based system managed by the `core` crate.

* **Global Epoch:** The game starts at a specific Unix timestamp (e.g., January 1st of the starting year).
* **Daily Tick:** The standard progression step. Triggers daily events: stamina recovery, morale decay/growth, inbox generation, and financial daily amortizations.
* **Phase Ticks:** A day is divided into phases: Morning (Training/Events), Afternoon (Scrims/Matches), and Evening (Rest/Post-match analysis).
* **Weekly Tick:** Triggers board evaluations, staff reports, and regional standings updates.
* **Patch Cycle:** A scheduled event array that applies `patches.json` modifiers every 2-4 in-game weeks, shifting the `meta_profiles`.

### 2. Player Data & Attribute Architecture

Player entities are the most complex models. Their performance is a composite of base attributes, current state, and dynamic modifiers.

#### Base Attributes (0-100 Scale)

| Category | Attribute | Implementation Impact |
| --- | --- | --- |
| **Physical** | Endurance | Reduces the rate of Stamina depletion per match/scrim. |
| **Physical** | Reaction Time | Modifier for mechanical outplays and dodging ganks. |
| **Mental** | Decision Making | Reduces unforced errors and bad macro calls in the simulation. |
| **Mental** | Clutch | Applies a performance multiplier during high-stakes matches or late-game phases. |
| **Mental** | Discipline | Reduces the likelihood of penalties or ignoring tactical interventions. |
| **Mental** | Tilt-Resistance | Mitigates the temporary attribute drop after losing a teamfight or game. |
| **Technical** | Mechanics | Base multiplier for champion-specific damage output and farming efficiency. |
| **Technical** | Vision Control | Increases the probability of securing neutral objectives (Dragons, Baron). |
| **Technical** | Teamfighting | Synergizes with the team's total score to determine 5v5 skirmish outcomes. |

#### Dynamic State Modifiers

* **Stamina (0-100):** Depletes by 15-25 points per match/scrim based on Endurance. Below 50, mechanics and reaction time suffer a penalty. Below 20, injury risk triggers.
* **Morale (0-100):** Defaults to 50. Influenced by inbox responses, match outcomes, and contract status. Modifies the "Clutch" and "Tilt-Resistance" attributes.
* **Confidence (Enum):** States include `Slumping`, `Neutral`, `Confident`, `Hyped`. Driven by a hidden win-streak counter. `Hyped` provides a flat +5 to all Technical attributes.
* **Synergy (0-100):** A team-wide metric calculated from the average shared matches between the current starting five. Acts as a multiplier for the "Teamfighting" attribute.

### 3. Drafting System Logic (The AI Crate)

The drafting phase is a turn-based state machine. For the AI to act intelligently, it calculates a **Draft Score** for every available champion during its turn.

#### AI Champion Evaluation Formula

The AI assigns a priority score to available champions based on four weighted factors:

* **Base Meta Strength:** The champion's tier in the current `patches.json` (e.g., S-tier = 1.5 multiplier, C-tier = 0.8 multiplier).
* **Player Mastery:** The assigned player's mastery level with that champion (Bronze = 0.5 to Challenger = 1.5).
* **Composition Synergy:** Checking the champion's tags against already locked champions (e.g., adding a "Knockup" champion when a "Yasuo-style" champion is locked adds +20 points).
* **Counter-Matchup:** If the enemy lane opponent is locked, the AI checks a hardcoded matchup matrix. Favorable matchups add +30 points; counters subtract 30 points.

The AI selects the champion with the highest composite score, applying a slight randomization seed to prevent 100% predictable drafts.

### 4. Match Simulation Engine (Monte Carlo Event System)

The simulation does not track 2D coordinates. Instead, it generates chronological "Events" based on weighted probabilities.

#### Match Phases & Constraints

| Phase | Time | Allowed Events | Weight Modifiers |
| --- | --- | --- | --- |
| **Early Game** | 0 - 15 min | First Blood, Tower Plates, First Dragon, Rift Herald, Solo Kills. | Player Mechanics, Reaction Time, Champion Early-Scaling. |
| **Mid Game** | 15 - 25 min | Tier 1/2 Towers, Dragons 2/3, Small Teamfights, Baron spawn (20m). | Vision Control, Decision Making, Team Synergy, Gold Delta. |
| **Late Game** | 25+ min | Inhibitors, Elder Dragon, Baron steals, Nexus, 5v5 Wipes. | Clutch, Teamfighting, Champion Late-Scaling, Respawn Timers. |

#### Event Resolution Logic

Every simulated minute, the engine rolls the RNG seed against the probability of an event occurring (e.g., "Top Lane Skirmish").
If an event triggers, the engine calculates the **Win Probability** for Team A vs Team B:

* Base calculation: `Team A Total relevant attributes` vs `Team B Total relevant attributes`.
* Add Gold Delta: Every 1,000 gold advantage shifts the probability weight by a defined percentage (e.g., +5% win chance).
* Add RNG Variance: A random number is rolled within the determined probability window to decide the victor of the event.
* Consequence: The winner receives Gold, XP, and potentially an objective. The loser loses map pressure and takes a temporary "Tilt" penalty.

### 5. Transfer & Contract State Machine

Transfers are not instant. They follow a negotiation pipeline managed by the `core` and `db` crates.

* **State 1: Approach & Scouting:** User pays a scouting fee. After X ticks, hidden attributes and the player's "Ambition Level" are revealed.
* **State 2: Buyout Negotiation (Team-to-Team):** If the player is under contract, the user bids against the owning team's AI. The AI evaluates the bid based on the player's current value, remaining contract length, and the team's financial health.
* **State 3: Contract Negotiation (Team-to-Player):** If the buyout is accepted (or the player is a free agent), the user negotiates with the player's agent.
* **Agent Logic:** The agent calculates an "Offer Score" based on Base Salary, Contract Length, the User's Manager Reputation, and Team Tournament Status. If the score beats the player's minimum threshold, the contract is signed.

### 6. Daily Scheduling & Activity Engine

The `engine` crate handles a daily planner where players are assigned activities. This system dictates attribute progression, champion mastery, and stamina decay.

#### Activity Types & Impact Matrix

| Activity | Stamina Cost | Mastery Gain | Attribute Growth | Morale/Satisfaction Impact |
| --- | --- | --- | --- | --- |
| **Scrim block** | High (-15 to -20) | High | High (Team Synergy, Macro) | Neutral (Positive if winning, negative if losing) |
| **Solo Queue** | Low (-5 to -10) | Medium | Low (Mechanics, Reaction) | Neutral to slightly negative (grind fatigue) |
| **Rest Day** | Recovery (+30) | None | None | Positive (Reduces burnout risk) |

#### Scrim Scheduling Logic

* **Time Blocks:** A day has exactly 4 available slots for scrims.
* **Matchmaking:** Users can approach teams or be approached. The success probability of a scrim request is a function of:
* `Delta = Target Team Reputation - User Team Reputation`
* If `Delta` is too high (user is trying to punch too far above their weight), the request is automatically rejected.

* **Execution:** Before a scheduled scrim block triggers, the game transitions to the Match Engagement flow (just like an official match).

#### Match & Scrim Engagement Options (The Delegation System)

When a match or scrim is next in the turn queue, the "Continue" button transforms. The user selects their level of involvement, which drastically changes how the Rust `engine` processes the event.

#### Engagement States & Execution Flow

* **Participate (Default):**
* **Draft:** User interacts with the UI. The `ai` crate calculates enemy picks/bans in real-time.
* **Simulation:** The backend yields state updates tick-by-tick (e.g., every simulated second) to the frontend via Tauri IPC. The frontend can send tactical commands back to the engine, which are queued and resolved in the next tick.

* **Spectate:**
* **Draft:** The `ai` crate completely handles the user's draft based on predefined tactical focuses.
* **Simulation:** The engine streams the match tick-by-tick to the frontend for viewing, but blocks all tactical input channels from the user.

* **Draft and Delegate:**
* **Draft:** User fully controls the pick/ban phase.
* **Simulation:** Once the draft confirms, the `engine` runs the entire match loop in a single, instantaneous backend thread without yielding to the frontend. It immediately returns the final post-match state object.

* **Delegate (Instantaneous):**
* **Draft & Simulation:** The user clicks the button, and the `ai` and `engine` instantly resolve both the draft and the match. The user is taken directly to the post-match summary screen.

### 7. Inbox, News, and Event Routing

The inbox is the primary driver of the turn-based state machine. The `core` crate utilizes an event queue that categorizes incoming messages.

#### Message State Machine

* **Informative (`State::Read_Optional`):** News about other leagues, meta shifts, or minor staff reports. Does not block the "Continue" button.
* **Actionable (`State::Requires_Response`):** Player transfer requests, contract negotiations, or scrim invitations. Can be ignored, but ignoring them automatically selects a default negative/neutral outcome upon ending the turn.
* **Urgent / Blocking (`State::Hard_Block`):** Unresolved roster gaps before a matchday, critical board ultimatums, or severe player discipline issues. **The `core` crate disables the "Continue" button until the user explicitly resolves this message.**

### 8. Staff Influence & Progression Modifiers

Staff members are not just passive stat boosts; they actively hook into the simulation and progression engines.

* **Assistant Coach:** Governs the quality of AI decisions when the user selects "Delegate" or "Spectate." A high-tier assistant coach will draft better team comps and make smarter mid-game tactical shifts.
* **Positional Coaches:** Assigned to specific players or roles (e.g., "Jungle Coach"). They apply a `1.x` multiplier to the daily attribute growth of players in that specific role during Scrims and Solo Queue.
* **Sports Psychologist:** Passively reduces the Stamina penalty of matches and acts as a buffer against Morale drops after a loss or a broken "Confidence" streak.

### 9. Board Expectations & Job Security

The `core` crate tracks a `ManagerSatisfaction` object, continually evaluated by the Board of Directors.

* **Metrics Tracked:** Tournament placement (Primary), Roster overall rating (Secondary), Financial health (Tertiary).
* **Evaluation Cycle:** Checked every weekly tick. If `ManagerSatisfaction` drops below 30/100, the user receives an `Urgent` inbox warning. If it drops to 0, the termination event fires.
* **Termination State:** The user is stripped of their team association. The UI switches to a "Free Agent" dashboard, displaying job offers based strictly on the user's persistent `Reputation` score.

### 10. UI/UX Data Retrieval (Frontend/Backend Bridge)

Because the game utilizes SQLite via Rust, pulling lists of thousands of players or champions for the UI must be highly optimized to prevent frontend lag.

* **Pagination & Filtering:** The frontend (React) never holds the full database. It sends standard SQL offset/limit queries via Tauri commands (e.g., `invoke('get_players', { offset: 0, limit: 50, filters: { role: 'Jungle', min_reputation: 60 } })`).
* **Search:** Rust implements lightweight text search (like SQLite's `LIKE` or FTS5) to quickly return lists of teams, tournaments, or players to the frontend search bars.
