-- Add new fields to tracks table and fix data types
-- PostgreSQL supports ALTER COLUMN, so no need to recreate the table

ALTER TABLE tracks ADD COLUMN IF NOT EXISTS position VARCHAR;
ALTER TABLE tracks ADD COLUMN IF NOT EXISTS score VARCHAR;
ALTER TABLE tracks ADD COLUMN IF NOT EXISTS entrega_id BIGINT REFERENCES entregas(id);
ALTER TABLE tracks ADD COLUMN IF NOT EXISTS artist_name VARCHAR NOT NULL DEFAULT 'Unknown';

-- Change bpm from INTEGER to DOUBLE PRECISION for decimal values
ALTER TABLE tracks ALTER COLUMN bpm TYPE DOUBLE PRECISION;

-- Remove tone constraint (will be added back with more keys in next migration)
ALTER TABLE tracks DROP CONSTRAINT IF EXISTS tracks_tone_check;
