use std::ffi::OsStr;
use std::path::PathBuf;

use axum::extract::{Form, Path, State};
use axum::http::StatusCode;
use axum::response::{Html, Redirect};
use axum::routing::{get, post, put};
use axum::{Json, Router};
use serde_json::json;

use crate::db::DbPool;
use crate::models::{
    Album, Artist, Batch, Entrega, ImportPayload, NewAlbum, NewBatch, NewEntrega, NewTrack, Track,
};

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub articles_dir: PathBuf,
}

pub fn app_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(home))
        .route("/health", get(health))
        .route("/artists", get(get_artists))
        .route("/albums", get(get_albums).post(create_album))
        .route("/albums/{id}", put(update_album))
        .route("/tracks", get(get_tracks).post(create_track))
        .route("/tracks/{id}", put(update_track))
        .route("/batches", get(get_batches).post(create_batch))
        .route("/batches/{id}", put(update_batch))
        .route("/entregas", get(get_entregas).post(create_entrega))
        .route("/entregas/{id}", put(update_entrega))
        .route("/import/json", post(import_json))
        .route("/admin", get(admin_page))
        .route("/admin/albums", post(create_album_form))
        .route("/admin/tracks", post(create_track_form))
        .route("/admin/batches", post(create_batch_form))
        .route("/admin/entregas", post(create_entrega_form))
        .route("/articles", get(list_articles))
        .route("/articles/{filename}", get(view_article))
        .with_state(state)
}

async fn home() -> Html<&'static str> {
    Html(
        r#"
<h1>Sonido Sigiloso</h1>
<ul>
  <li><a href="/health">Health</a></li>
  <li><a href="/admin">Admin</a></li>
  <li><a href="/articles">Articles</a></li>
</ul>
"#,
    )
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

async fn get_artists(State(state): State<AppState>) -> Result<Json<Vec<Artist>>, StatusCode> {
    let artists = sqlx::query_as::<_, Artist>(
        "SELECT id, name, country, created_at FROM artists ORDER BY id",
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(artists))
}

async fn get_albums(State(state): State<AppState>) -> Result<Json<Vec<Album>>, StatusCode> {
    let albums = sqlx::query_as::<_, Album>(
        "SELECT id, title, artist_id, release_year, label, format, country, genre, style, created_at FROM albums ORDER BY id",
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(albums))
}

async fn get_tracks(State(state): State<AppState>) -> Result<Json<Vec<Track>>, StatusCode> {
    let tracks = sqlx::query_as::<_, Track>(
        "SELECT id, title, artist_name, album_id, duration_seconds, bpm, tone, position, score, entrega_id, created_at FROM tracks ORDER BY position, id",
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(tracks))
}

async fn get_batches(State(state): State<AppState>) -> Result<Json<Vec<Batch>>, StatusCode> {
    let batches =
        sqlx::query_as::<_, Batch>("SELECT id, name, created_at FROM batches ORDER BY id")
            .fetch_all(&state.pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(batches))
}

async fn get_entregas(State(state): State<AppState>) -> Result<Json<Vec<Entrega>>, StatusCode> {
    let entregas = sqlx::query_as::<_, Entrega>(
        "SELECT id, name, batch_id, created_at FROM entregas ORDER BY id",
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(entregas))
}

async fn create_album(
    State(state): State<AppState>,
    Json(input): Json<NewAlbum>,
) -> Result<(StatusCode, Json<Album>), StatusCode> {
    let album_id: i64 = sqlx::query_scalar(
        "INSERT INTO albums (title, artist_id, release_year, label, format, country, genre, style, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING id",
    )
    .bind(&input.title)
    .bind(input.artist_id)
    .bind(input.release_year)
    .bind(&input.label)
    .bind(&input.format)
    .bind(&input.country)
    .bind(&input.genre)
    .bind(&input.style)
    .bind(&input.created_at)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let album = sqlx::query_as::<_, Album>(
        "SELECT id, title, artist_id, release_year, label, format, country, genre, style, created_at FROM albums WHERE id = $1",
    )
    .bind(album_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(album)))
}

async fn update_album(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(input): Json<NewAlbum>,
) -> Result<Json<Album>, StatusCode> {
    let result = sqlx::query(
        "UPDATE albums SET title = $1, artist_id = $2, release_year = $3, label = $4, format = $5, country = $6, genre = $7, style = $8, created_at = $9 WHERE id = $10",
    )
    .bind(&input.title)
    .bind(input.artist_id)
    .bind(input.release_year)
    .bind(&input.label)
    .bind(&input.format)
    .bind(&input.country)
    .bind(&input.genre)
    .bind(&input.style)
    .bind(&input.created_at)
    .bind(id)
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    let album = sqlx::query_as::<_, Album>(
        "SELECT id, title, artist_id, release_year, label, format, country, genre, style, created_at FROM albums WHERE id = $1",
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(album))
}

async fn create_track(
    State(state): State<AppState>,
    Json(input): Json<NewTrack>,
) -> Result<(StatusCode, Json<Track>), StatusCode> {
    let track_id: i64 = sqlx::query_scalar(
        "INSERT INTO tracks (title, artist_name, album_id, duration_seconds, bpm, tone, position, score, entrega_id, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING id",
    )
    .bind(&input.title)
    .bind(&input.artist_name)
    .bind(input.album_id)
    .bind(input.duration_seconds)
    .bind(input.bpm)
    .bind(&input.tone)
    .bind(&input.position)
    .bind(&input.score)
    .bind(input.entrega_id)
    .bind(&input.created_at)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let track = sqlx::query_as::<_, Track>(
        "SELECT id, title, artist_name, album_id, duration_seconds, bpm, tone, position, score, entrega_id, created_at FROM tracks WHERE id = $1",
    )
    .bind(track_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(track)))
}

async fn update_track(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(input): Json<NewTrack>,
) -> Result<Json<Track>, StatusCode> {
    let result = sqlx::query(
        "UPDATE tracks SET title = $1, artist_name = $2, album_id = $3, duration_seconds = $4, bpm = $5, tone = $6, position = $7, score = $8, entrega_id = $9, created_at = $10 WHERE id = $11",
    )
    .bind(&input.title)
    .bind(&input.artist_name)
    .bind(input.album_id)
    .bind(input.duration_seconds)
    .bind(input.bpm)
    .bind(&input.tone)
    .bind(&input.position)
    .bind(&input.score)
    .bind(input.entrega_id)
    .bind(&input.created_at)
    .bind(id)
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    let track = sqlx::query_as::<_, Track>(
        "SELECT id, title, artist_name, album_id, duration_seconds, bpm, tone, position, score, entrega_id, created_at FROM tracks WHERE id = $1",
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(track))
}

async fn create_batch(
    State(state): State<AppState>,
    Json(input): Json<NewBatch>,
) -> Result<(StatusCode, Json<Batch>), StatusCode> {
    let batch_id: i64 =
        sqlx::query_scalar("INSERT INTO batches (name, created_at) VALUES ($1, $2) RETURNING id")
            .bind(&input.name)
            .bind(&input.created_at)
            .fetch_one(&state.pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let batch =
        sqlx::query_as::<_, Batch>("SELECT id, name, created_at FROM batches WHERE id = $1")
            .bind(batch_id)
            .fetch_one(&state.pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(batch)))
}

async fn update_batch(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(input): Json<NewBatch>,
) -> Result<Json<Batch>, StatusCode> {
    let result = sqlx::query("UPDATE batches SET name = $1, created_at = $2 WHERE id = $3")
        .bind(&input.name)
        .bind(&input.created_at)
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    let batch =
        sqlx::query_as::<_, Batch>("SELECT id, name, created_at FROM batches WHERE id = $1")
            .bind(id)
            .fetch_one(&state.pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(batch))
}

async fn create_entrega(
    State(state): State<AppState>,
    Json(input): Json<NewEntrega>,
) -> Result<(StatusCode, Json<Entrega>), StatusCode> {
    let entrega_id: i64 = sqlx::query_scalar(
        "INSERT INTO entregas (name, batch_id, created_at) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(&input.name)
    .bind(input.batch_id)
    .bind(&input.created_at)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let entrega = sqlx::query_as::<_, Entrega>(
        "SELECT id, name, batch_id, created_at FROM entregas WHERE id = $1",
    )
    .bind(entrega_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(entrega)))
}

async fn update_entrega(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(input): Json<NewEntrega>,
) -> Result<Json<Entrega>, StatusCode> {
    let result =
        sqlx::query("UPDATE entregas SET name = $1, batch_id = $2, created_at = $3 WHERE id = $4")
            .bind(&input.name)
            .bind(input.batch_id)
            .bind(&input.created_at)
            .bind(id)
            .execute(&state.pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    let entrega = sqlx::query_as::<_, Entrega>(
        "SELECT id, name, batch_id, created_at FROM entregas WHERE id = $1",
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(entrega))
}

async fn import_json(
    State(state): State<AppState>,
    Json(payload): Json<ImportPayload>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut tx = state
        .pool
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(items) = payload.albums {
        for item in items {
            sqlx::query(
                "INSERT INTO albums (title, artist_id, release_year, label, format, country, genre, style, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            )
            .bind(&item.title)
            .bind(item.artist_id)
            .bind(item.release_year)
            .bind(&item.label)
            .bind(&item.format)
            .bind(&item.country)
            .bind(&item.genre)
            .bind(&item.style)
            .bind(&item.created_at)
            .execute(&mut *tx)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }
    }

    if let Some(items) = payload.tracks {
        for item in items {
            sqlx::query(
                "INSERT INTO tracks (title, artist_name, album_id, duration_seconds, bpm, tone, position, score, entrega_id, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
            )
            .bind(&item.title)
            .bind(&item.artist_name)
            .bind(item.album_id)
            .bind(item.duration_seconds)
            .bind(item.bpm)
            .bind(&item.tone)
            .bind(&item.position)
            .bind(&item.score)
            .bind(item.entrega_id)
            .bind(&item.created_at)
            .execute(&mut *tx)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }
    }

    if let Some(items) = payload.batches {
        for item in items {
            sqlx::query("INSERT INTO batches (name, created_at) VALUES ($1, $2)")
                .bind(&item.name)
                .bind(&item.created_at)
                .execute(&mut *tx)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }
    }

    if let Some(items) = payload.entregas {
        for item in items {
            sqlx::query("INSERT INTO entregas (name, batch_id, created_at) VALUES ($1, $2, $3)")
                .bind(&item.name)
                .bind(item.batch_id)
                .bind(&item.created_at)
                .execute(&mut *tx)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }
    }

    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({ "status": "ok" })))
}

async fn admin_page() -> Html<&'static str> {
    Html(
        r#"
<h1>Admin</h1>
<p>Use these forms to insert records quickly.</p>

<h2>Album</h2>
<form action="/admin/albums" method="post">
  <input name="title" placeholder="title" required />
  <input name="artist_id" placeholder="artist_id (optional)" />
  <input name="release_year" placeholder="release_year (optional)" />
  <input name="created_at" placeholder="created_at (YYYY-MM-DDTHH:MM:SSZ)" required />
  <button type="submit">Create album</button>
</form>

<h2>Track</h2>
<form action="/admin/tracks" method="post">
  <input name="title" placeholder="title" required />
  <input name="album_id" placeholder="album_id (optional)" />
  <input name="duration_seconds" placeholder="duration_seconds (optional)" />
  <input name="bpm" placeholder="bpm (optional)" />
  <input name="tone" placeholder="tone A-G (optional)" />
  <input name="created_at" placeholder="created_at (YYYY-MM-DDTHH:MM:SSZ)" required />
  <button type="submit">Create track</button>
</form>

<h2>Batch</h2>
<form action="/admin/batches" method="post">
  <input name="name" placeholder="name" required />
  <input name="created_at" placeholder="created_at (YYYY-MM-DDTHH:MM:SSZ)" required />
  <button type="submit">Create batch</button>
</form>

<h2>Entrega</h2>
<form action="/admin/entregas" method="post">
  <input name="name" placeholder="name" required />
  <input name="batch_id" placeholder="batch_id (optional)" />
  <input name="created_at" placeholder="created_at (YYYY-MM-DDTHH:MM:SSZ)" required />
  <button type="submit">Create entrega</button>
</form>
"#,
    )
}

async fn create_album_form(
    State(state): State<AppState>,
    Form(input): Form<NewAlbum>,
) -> Result<Redirect, StatusCode> {
    sqlx::query(
        "INSERT INTO albums (title, artist_id, release_year, label, format, country, genre, style, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
    )
    .bind(&input.title)
    .bind(input.artist_id)
    .bind(input.release_year)
    .bind(&input.label)
    .bind(&input.format)
    .bind(&input.country)
    .bind(&input.genre)
    .bind(&input.style)
    .bind(&input.created_at)
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/admin"))
}

async fn create_track_form(
    State(state): State<AppState>,
    Form(input): Form<NewTrack>,
) -> Result<Redirect, StatusCode> {
    sqlx::query(
        "INSERT INTO tracks (title, artist_name, album_id, duration_seconds, bpm, tone, position, score, entrega_id, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
    )
    .bind(&input.title)
    .bind(&input.artist_name)
    .bind(input.album_id)
    .bind(input.duration_seconds)
    .bind(input.bpm)
    .bind(&input.tone)
    .bind(&input.position)
    .bind(&input.score)
    .bind(input.entrega_id)
    .bind(&input.created_at)
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/admin"))
}

async fn create_batch_form(
    State(state): State<AppState>,
    Form(input): Form<NewBatch>,
) -> Result<Redirect, StatusCode> {
    sqlx::query("INSERT INTO batches (name, created_at) VALUES ($1, $2)")
        .bind(&input.name)
        .bind(&input.created_at)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/admin"))
}

async fn create_entrega_form(
    State(state): State<AppState>,
    Form(input): Form<NewEntrega>,
) -> Result<Redirect, StatusCode> {
    sqlx::query("INSERT INTO entregas (name, batch_id, created_at) VALUES ($1, $2, $3)")
        .bind(&input.name)
        .bind(input.batch_id)
        .bind(&input.created_at)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/admin"))
}

async fn list_articles(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    let mut entries = tokio::fs::read_dir(&state.articles_dir)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut files: Vec<String> = Vec::new();

    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        let path = entry.path();
        if path.extension() == Some(OsStr::new("md")) {
            let filename = entry.file_name();
            if let Some(name) = filename.to_str() {
                files.push(name.to_string());
            }
        }
    }

    files.sort();

    let mut html = String::from("<h1>Articles</h1><ul>");
    for file in files {
        let encoded = urlencoding::encode(&file);
        html.push_str(&format!(
            r#"<li><a href="/articles/{}">{}</a></li>"#,
            encoded, file
        ));
    }
    html.push_str("</ul>");

    Ok(Html(html))
}

async fn view_article(
    Path(filename): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, StatusCode> {
    if !is_safe_markdown_name(&filename) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let file_path = state.articles_dir.join(&filename);
    let content = tokio::fs::read_to_string(file_path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let escaped = html_escape::encode_text(&content);
    let body = format!(
        "<h1>{}</h1><p><a href=\"/articles\">Back</a></p><pre>{}</pre>",
        filename, escaped
    );

    Ok(Html(body))
}

fn is_safe_markdown_name(name: &str) -> bool {
    name.ends_with(".md") && !name.contains('/') && !name.contains('\\') && !name.contains("..")
}
