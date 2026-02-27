-- Add new fields to tracks table and fix data types
ALTER TABLE tracks ADD COLUMN position TEXT;
ALTER TABLE tracks ADD COLUMN score TEXT;
ALTER TABLE tracks ADD COLUMN entrega_id INTEGER REFERENCES entregas(id);

-- Recreate tracks table with updated schema
-- SQLite doesn't support ALTER COLUMN, so we need to recreate the table

-- Step 1: Create new tracks table with correct types
CREATE TABLE tracks_new (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    artist_name TEXT NOT NULL,
    album_id INTEGER,
    duration_seconds INTEGER,
    bpm REAL,  -- Changed from INTEGER to REAL for decimal values
    tone TEXT,  -- Removed CHECK constraint to allow complex keys like "Db", "Fm", "Cm"
    position TEXT,  -- New field for A1, A2, B1, B2, etc.
    score TEXT,  -- New field for emoji scores
    entrega_id INTEGER,
    created_at TEXT NOT NULL,
    FOREIGN KEY (album_id) REFERENCES albums(id),
    FOREIGN KEY (entrega_id) REFERENCES entregas(id)
);

-- Step 2: Copy existing data
INSERT INTO tracks_new (id, title, artist_name, album_id, duration_seconds, bpm, tone, created_at)
SELECT id, title, 'Unknown', album_id, duration_seconds, bpm, tone, created_at FROM tracks;

-- Step 3: Drop old table
DROP TABLE tracks;

-- Step 4: Rename new table
ALTER TABLE tracks_new RENAME TO tracks;
