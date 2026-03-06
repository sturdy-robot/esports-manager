-- V7: Add player ChampionPool (mastery levels)
CREATE TABLE moba_player_masteries (
    player_id   INTEGER NOT NULL REFERENCES moba_players(id) ON DELETE CASCADE,
    champion_id INTEGER NOT NULL REFERENCES moba_champions(id) ON DELETE CASCADE,
    mastery     INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (player_id, champion_id)
);
