-- V5: Add tournament_json and moba_teams_json blobs to game_session.
-- Tournament tracks schedule + match results as JSON.
-- MobaTeams store the full MOBA-specific player attributes for match simulation.

ALTER TABLE game_session ADD COLUMN tournament_json TEXT NOT NULL DEFAULT '';
ALTER TABLE game_session ADD COLUMN moba_teams_json TEXT NOT NULL DEFAULT '[]';
