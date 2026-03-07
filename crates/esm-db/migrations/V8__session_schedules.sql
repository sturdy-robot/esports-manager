ALTER TABLE game_session ADD COLUMN schedules_json TEXT NOT NULL DEFAULT '[]';
ALTER TABLE game_session ADD COLUMN scrims_json TEXT NOT NULL DEFAULT '{"scrims":[],"next_id":1}';
