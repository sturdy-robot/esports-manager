-- V4: Add teams_json blob and player_team_index to game_session.
-- Teams are serialized as JSON for full in-memory state preservation
-- (stamina, morale, confidence, etc.) across save/load cycles.

ALTER TABLE game_session ADD COLUMN player_team_index INTEGER NOT NULL DEFAULT 0;
ALTER TABLE game_session ADD COLUMN teams_json TEXT NOT NULL DEFAULT '[]';
