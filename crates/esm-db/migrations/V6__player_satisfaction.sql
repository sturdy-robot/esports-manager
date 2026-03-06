-- Adding 'satisfaction' column to moba_players
ALTER TABLE moba_players
ADD COLUMN satisfaction INTEGER NOT NULL DEFAULT 50 CHECK (satisfaction >= 0 AND satisfaction <= 100);
