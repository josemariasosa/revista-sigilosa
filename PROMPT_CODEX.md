You are building the backend for a music magazine called "Sonido Sigiloso".

This is a Rust project.

Your goal is to implement the initial backend MVP using:

- Rust
- Axum
- SQLite
- SQLx
- Tokio

This backend must run as a single binary.

No Docker.

No external database server.

SQLite must be used as a file database.

---

# Requirements

Create a Rust project with:

Dependencies:

- axum
- tokio
- serde
- serde_json
- sqlx (features: sqlite, runtime-tokio, macros)
- tower
- tower-http

---

# Database

Use SQLite.

Database file name:

sonido_sigiloso.db

If the database does not exist, create it automatically.

Use SQLx migrations.

Create migrations folder.

---

# Tables

Create initial migrations for these tables:

artists
    id INTEGER PRIMARY KEY
    name TEXT NOT NULL
    country TEXT
    created_at TEXT NOT NULL

albums
    id INTEGER PRIMARY KEY
    title TEXT NOT NULL
    artist_id INTEGER
    release_year INTEGER
    created_at TEXT NOT NULL

tracks
    id INTEGER PRIMARY KEY
    title TEXT NOT NULL
    album_id INTEGER
    duration_seconds INTEGER
    bpm INTEGER
    tone ENUM (not sure if available in sqlite.)
    created_at TEXT NOT NULL

batches
    id INTEGER PRIMARY KEY
    name TEXT NOT NULL
    created_at TEXT NOT NULL

entregas
    id INTEGER PRIMARY KEY
    name TEXT NOT NULL
    batch_id INTEGER
    created_at TEXT NOT NULL

---

# Server

Create Axum server.

Port:

3000

---

# Endpoints

Implement:

GET /health

returns:

{ "status": "ok" }

GET /artists
GET /albums
GET /tracks
GET /batches
GET /entregas

Return JSON.

---

# Architecture

Create modules:

main.rs
db.rs
models.rs
routes.rs

---

# db.rs

Create SQLite pool.

Run migrations automatically on startup.

---

# models.rs

Create Rust structs for all tables.

Use serde Serialize.

---

# routes.rs

Implement handlers.

---

# main.rs

Initialize database
Run migrations
Start Axum server

---

# Code quality

Must compile.

Must run with:

cargo run

---

# Output

Generate complete code.

Generate migrations.

Generate Cargo.toml dependencies.

---

Do not overengineer.

Keep code simple and clear.

This is MVP.