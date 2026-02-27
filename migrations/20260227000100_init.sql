-- PostgreSQL: Foreign keys are enabled by default

CREATE TABLE IF NOT EXISTS artists (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    country VARCHAR,
    created_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS albums (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    artist_id INTEGER,
    release_year INTEGER,
    created_at TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (artist_id) REFERENCES artists(id)
);

CREATE TABLE IF NOT EXISTS tracks (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    album_id INTEGER,
    duration_seconds INTEGER,
    bpm INTEGER,
    tone VARCHAR CHECK (tone IN ('A','B','C','D','E','F','G')),
    created_at TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (album_id) REFERENCES albums(id)
);

CREATE TABLE IF NOT EXISTS batches (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS entregas (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    batch_id INTEGER,
    created_at TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (batch_id) REFERENCES batches(id)
);
