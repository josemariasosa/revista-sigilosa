use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Artist {
    pub id: i64,
    pub name: String,
    pub country: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Album {
    pub id: i64,
    pub title: String,
    pub artist_id: Option<i64>,
    pub release_year: Option<i64>,
    pub label: Option<String>,
    pub format: Option<String>,
    pub country: Option<String>,
    pub genre: Option<String>,
    pub style: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Track {
    pub id: i64,
    pub title: String,
    pub artist_name: String,
    pub album_id: Option<i64>,
    pub duration_seconds: Option<i64>,
    pub bpm: Option<f64>, // Changed to f64 for decimal values
    pub tone: Option<String>,
    pub position: Option<String>, // New field for A1, A2, B1, etc.
    pub score: Option<String>,    // New field for emoji scores
    pub entrega_id: Option<i64>,  // Link to entrega/release
    pub created_at: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Batch {
    pub id: i64,
    pub name: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Entrega {
    pub id: i64,
    pub name: String,
    pub batch_id: Option<i64>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct NewAlbum {
    pub title: String,
    pub artist_id: Option<i64>,
    pub release_year: Option<i64>,
    pub label: Option<String>,
    pub format: Option<String>,
    pub country: Option<String>,
    pub genre: Option<String>,
    pub style: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct NewTrack {
    pub title: String,
    pub artist_name: String,
    pub album_id: Option<i64>,
    pub duration_seconds: Option<i64>,
    pub bpm: Option<f64>, // Changed to f64
    pub tone: Option<String>,
    pub position: Option<String>,
    pub score: Option<String>,
    pub entrega_id: Option<i64>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct NewBatch {
    pub name: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct NewEntrega {
    pub name: String,
    pub batch_id: Option<i64>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ImportPayload {
    pub albums: Option<Vec<NewAlbum>>,
    pub tracks: Option<Vec<NewTrack>>,
    pub batches: Option<Vec<NewBatch>>,
    pub entregas: Option<Vec<NewEntrega>>,
}
