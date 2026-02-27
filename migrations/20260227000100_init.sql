-- PostgreSQL: Foreign keys are enabled by default

CREATE TABLE IF NOT EXISTS artists (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    country VARCHAR,
    created_at VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS albums (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    artist_id BIGINT,
    release_year INTEGER,
    created_at VARCHAR NOT NULL,
    FOREIGN KEY (artist_id) REFERENCES artists(id)
);

CREATE TABLE IF NOT EXISTS tracks (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    album_id BIGINT,
    duration_seconds INTEGER,
    bpm INTEGER,
    tone VARCHAR CHECK (tone IN ('A','B','C','D','E','F','G')),
    created_at VARCHAR NOT NULL,
    FOREIGN KEY (album_id) REFERENCES albums(id)
);

CREATE TABLE IF NOT EXISTS batches (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    created_at VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS entregas (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    batch_id BIGINT,
    created_at VARCHAR NOT NULL,
    FOREIGN KEY (batch_id) REFERENCES batches(id)
);
