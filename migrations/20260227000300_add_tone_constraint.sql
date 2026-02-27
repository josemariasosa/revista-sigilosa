-- Add CHECK constraint for musical keys/tones
-- Allows all standard musical keys including enharmonic equivalents and minor keys

-- SQLite doesn't support adding CHECK constraints to existing columns, so we need to recreate the table
CREATE TABLE tracks_with_tone_check (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    artist_name TEXT NOT NULL,
    album_id INTEGER,
    duration_seconds INTEGER,
    bpm REAL,
    tone TEXT CHECK (tone IN (
        -- Major keys
        'C', 'C#', 'Db', 'D', 'D#', 'Eb', 'E', 'E#', 'Fb', 
        'F', 'F#', 'Gb', 'G', 'G#', 'Ab', 'A', 'A#', 'Bb', 'B', 'B#', 'Cb',
        -- Minor keys
        'Cm', 'C#m', 'Dbm', 'Dm', 'D#m', 'Ebm', 'Em', 'E#m', 'Fbm',
        'Fm', 'F#m', 'Gbm', 'Gm', 'G#m', 'Abm', 'Am', 'A#m', 'Bbm', 'Bm', 'B#m', 'Cbm'
    )),
    position TEXT,
    score TEXT,
    entrega_id INTEGER,
    created_at TEXT NOT NULL,
    FOREIGN KEY (album_id) REFERENCES albums(id),
    FOREIGN KEY (entrega_id) REFERENCES entregas(id)
);

-- Copy existing data
INSERT INTO tracks_with_tone_check (id, title, artist_name, album_id, duration_seconds, bpm, tone, position, score, entrega_id, created_at)
SELECT id, title, artist_name, album_id, duration_seconds, bpm, tone, position, score, entrega_id, created_at FROM tracks;

-- Drop old table
DROP TABLE tracks;

-- Rename new table
ALTER TABLE tracks_with_tone_check RENAME TO tracks;
