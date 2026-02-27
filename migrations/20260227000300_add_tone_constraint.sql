-- Add CHECK constraint for musical keys/tones
-- Allows all standard musical keys including enharmonic equivalents and minor keys

ALTER TABLE tracks ADD CONSTRAINT tracks_tone_check CHECK (tone IN (
    -- Major keys
    'C', 'C#', 'Db', 'D', 'D#', 'Eb', 'E', 'E#', 'Fb', 
    'F', 'F#', 'Gb', 'G', 'G#', 'Ab', 'A', 'A#', 'Bb', 'B', 'B#', 'Cb',
    -- Minor keys
    'Cm', 'C#m', 'Dbm', 'Dm', 'D#m', 'Ebm', 'Em', 'E#m', 'Fbm',
    'Fm', 'F#m', 'Gbm', 'Gm', 'G#m', 'Abm', 'Am', 'A#m', 'Bbm', 'Bm', 'B#m', 'Cbm'
));
