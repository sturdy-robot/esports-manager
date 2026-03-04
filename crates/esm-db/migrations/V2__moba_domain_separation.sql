-- V2: Domain separation — rename MOBA-specific tables with moba_ prefix
-- and add multi-role support to players.
-- Shared tables (managers, staff, contracts) keep their names.

-- Rename MOBA-specific tables
ALTER TABLE players RENAME TO moba_players;
ALTER TABLE teams RENAME TO moba_teams;
ALTER TABLE champions RENAME TO moba_champions;
ALTER TABLE champion_tags RENAME TO moba_champion_tags;

-- Add multi-role columns to moba_players
-- primary_role replaces the old role column semantics
-- secondary_roles stored as comma-separated values (e.g. "Top,Support")
ALTER TABLE moba_players ADD COLUMN secondary_roles TEXT NOT NULL DEFAULT '';

-- Add a moba_tournaments table for tournament tracking
CREATE TABLE moba_tournaments (
    id              INTEGER PRIMARY KEY,
    name            TEXT    NOT NULL,
    format          TEXT    NOT NULL CHECK(format IN ('RoundRobin','DoubleRoundRobin')),
    bracket_kind    TEXT    NOT NULL CHECK(bracket_kind IN ('Bo1','Bo3','Bo5')),
    start_day       INTEGER NOT NULL DEFAULT 1,
    is_complete     INTEGER NOT NULL DEFAULT 0
);

-- Add a moba_matches table for individual match results
CREATE TABLE moba_matches (
    id              INTEGER PRIMARY KEY,
    tournament_id   INTEGER NOT NULL REFERENCES moba_tournaments(id),
    blue_team_id    INTEGER NOT NULL REFERENCES moba_teams(id),
    red_team_id     INTEGER NOT NULL REFERENCES moba_teams(id),
    scheduled_day   INTEGER NOT NULL,
    status          TEXT    NOT NULL DEFAULT 'Pending' CHECK(status IN ('Pending','Completed')),
    blue_wins       INTEGER NOT NULL DEFAULT 0,
    red_wins        INTEGER NOT NULL DEFAULT 0,
    duration_minutes INTEGER,
    blue_team_gold  INTEGER,
    red_team_gold   INTEGER
);

-- Rename the role column to primary_role for clarity
ALTER TABLE moba_players RENAME COLUMN role TO primary_role;
