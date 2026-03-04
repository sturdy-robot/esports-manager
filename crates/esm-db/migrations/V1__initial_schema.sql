-- V1: Core entity tables for the eSports Manager game.
-- Per DESIGN.md, the database MUST NOT use JSON blobs in columns —
-- all attributes are stored as individual columns.

CREATE TABLE teams (
    id              INTEGER PRIMARY KEY,
    name            TEXT    NOT NULL UNIQUE,
    tag             TEXT    NOT NULL,
    synergy         INTEGER NOT NULL DEFAULT 0   CHECK(synergy BETWEEN 0 AND 100),
    reputation      INTEGER NOT NULL DEFAULT 50  CHECK(reputation BETWEEN 0 AND 100),
    budget          INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE players (
    id              INTEGER PRIMARY KEY,
    nickname        TEXT    NOT NULL,
    first_name      TEXT    NOT NULL,
    last_name       TEXT    NOT NULL,
    role            TEXT    NOT NULL CHECK(role IN ('Top','Jungle','Mid','Bot','Support')),
    endurance       INTEGER NOT NULL CHECK(endurance BETWEEN 0 AND 100),
    reaction_time   INTEGER NOT NULL CHECK(reaction_time BETWEEN 0 AND 100),
    decision_making INTEGER NOT NULL CHECK(decision_making BETWEEN 0 AND 100),
    clutch          INTEGER NOT NULL CHECK(clutch BETWEEN 0 AND 100),
    discipline      INTEGER NOT NULL CHECK(discipline BETWEEN 0 AND 100),
    tilt_resistance INTEGER NOT NULL CHECK(tilt_resistance BETWEEN 0 AND 100),
    mechanics       INTEGER NOT NULL CHECK(mechanics BETWEEN 0 AND 100),
    vision_control  INTEGER NOT NULL CHECK(vision_control BETWEEN 0 AND 100),
    teamfighting    INTEGER NOT NULL CHECK(teamfighting BETWEEN 0 AND 100),
    stamina         INTEGER NOT NULL DEFAULT 100 CHECK(stamina BETWEEN 0 AND 100),
    morale          INTEGER NOT NULL DEFAULT 50  CHECK(morale BETWEEN 0 AND 100),
    confidence      TEXT    NOT NULL DEFAULT 'Neutral' CHECK(confidence IN ('Slumping','Neutral','Confident','Hyped')),
    team_id         INTEGER REFERENCES teams(id)
);

CREATE TABLE champions (
    id              INTEGER PRIMARY KEY,
    name            TEXT    NOT NULL UNIQUE,
    class           TEXT    NOT NULL CHECK(class IN ('Tank','Fighter','Assassin','Mage','Marksman','Support')),
    scaling         TEXT    NOT NULL CHECK(scaling IN ('Early','Mid','Late'))
);

CREATE TABLE champion_tags (
    champion_id     INTEGER NOT NULL REFERENCES champions(id),
    tag             TEXT    NOT NULL CHECK(tag IN ('Knockup','Engage','Poke','Splitpush','Waveclear','Peel','Burst','Sustain')),
    PRIMARY KEY (champion_id, tag)
);

CREATE TABLE staff (
    id              INTEGER PRIMARY KEY,
    nickname        TEXT    NOT NULL,
    first_name      TEXT    NOT NULL,
    last_name       TEXT    NOT NULL,
    role            TEXT    NOT NULL CHECK(role IN ('AssistantCoach','DraftAnalyst','PositionalCoach','SportsPsychologist','Scout','FinancialOfficer')),
    skill           INTEGER NOT NULL CHECK(skill BETWEEN 0 AND 100),
    team_id         INTEGER REFERENCES teams(id)
);

CREATE TABLE contracts (
    id              INTEGER PRIMARY KEY,
    player_id       INTEGER NOT NULL REFERENCES players(id),
    team_id         INTEGER NOT NULL REFERENCES teams(id),
    salary          INTEGER NOT NULL,
    length_days     INTEGER NOT NULL,
    remaining_days  INTEGER NOT NULL,
    buyout_clause   INTEGER,
    status          TEXT    NOT NULL DEFAULT 'Active' CHECK(status IN ('Active','Expired','Terminated'))
);

CREATE TABLE managers (
    id              INTEGER PRIMARY KEY,
    nickname        TEXT    NOT NULL,
    first_name      TEXT    NOT NULL,
    last_name       TEXT    NOT NULL,
    nationality     TEXT    NOT NULL,
    archetype       TEXT    NOT NULL CHECK(archetype IN ('TacticalGenius','PlayerDeveloper','Motivator','Analyst','Balanced')),
    reputation      INTEGER NOT NULL DEFAULT 50 CHECK(reputation BETWEEN 0 AND 100)
);
