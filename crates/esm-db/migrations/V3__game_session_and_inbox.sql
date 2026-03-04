-- V3: Game session state and inbox messages for SQLite-backed persistence.
-- The game_session table stores the transient session state (calendar, RNG,
-- manager snapshot, player team) that lives in memory during play and gets
-- flushed to SQLite on explicit save.
-- The inbox_messages table persists the player's inbox across saves.

CREATE TABLE game_session (
    id                      INTEGER PRIMARY KEY,
    game_version            TEXT    NOT NULL,
    esport_type             TEXT    NOT NULL CHECK(esport_type IN ('Moba','Rts','Fps')),
    rng_seed                INTEGER NOT NULL,
    rng_state               INTEGER NOT NULL,
    calendar_year           INTEGER NOT NULL,
    calendar_month          INTEGER NOT NULL CHECK(calendar_month BETWEEN 1 AND 12),
    calendar_day            INTEGER NOT NULL CHECK(calendar_day BETWEEN 1 AND 31),
    calendar_phase          TEXT    NOT NULL CHECK(calendar_phase IN ('Morning','Afternoon','Evening')),
    calendar_days_elapsed   INTEGER NOT NULL DEFAULT 0,
    player_team_name        TEXT    NOT NULL,
    manager_nickname        TEXT    NOT NULL,
    manager_first_name      TEXT    NOT NULL,
    manager_last_name       TEXT    NOT NULL,
    manager_nationality     TEXT    NOT NULL,
    manager_archetype       TEXT    NOT NULL CHECK(manager_archetype IN ('TacticalGenius','PlayerDeveloper','Motivator','Analyst','Balanced')),
    manager_reputation      INTEGER NOT NULL DEFAULT 50 CHECK(manager_reputation BETWEEN 0 AND 100),
    created_at              TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at              TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE inbox_messages (
    id              INTEGER PRIMARY KEY,
    subject         TEXT    NOT NULL,
    body            TEXT    NOT NULL,
    priority        TEXT    NOT NULL CHECK(priority IN ('ReadOptional','RequiresResponse','HardBlock')),
    category        TEXT    NOT NULL CHECK(category IN ('News','Transfer','Contract','Scrim','Board','Staff','Injury','MetaShift')),
    day_received    INTEGER NOT NULL,
    is_resolved     INTEGER NOT NULL DEFAULT 0 CHECK(is_resolved IN (0, 1))
);
