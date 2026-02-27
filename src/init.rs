use sqlx::PgPool;
use std::collections::HashMap;

/// Initialize database with initial data from init_data.json
/// This function is granular and idempotent - checks each item individually
/// Only adds entregas, albums, and tracks that don't already exist
pub async fn init_data(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Checking initial data...");

    // Read init_data.json
    let init_file = std::fs::read_to_string("init_data.json")?;
    let init_payload: crate::models::ImportPayload = serde_json::from_str(&init_file)?;

    let mut tx = pool.begin().await?;
    let mut new_entregas_count = 0;
    let mut new_albums_count = 0;
    let mut new_tracks_count = 0;
    
    // Map JSON entrega positions (1, 2, 3...) to actual database IDs
    let mut json_entrega_pos_to_db_id: HashMap<i64, i64> = HashMap::new();
    
    // Map JSON album positions (1, 2, 3...) to actual database IDs
    let mut json_album_pos_to_db_id: HashMap<i64, i64> = HashMap::new();
    
    // Get all existing entregas to build the name->id map
    let mut entrega_name_to_id: HashMap<String, i64> = HashMap::new();
    let existing_entregas: Vec<(i64, String)> =
        sqlx::query_as("SELECT id, name FROM entregas")
            .fetch_all(&mut *tx)
            .await?;

    for (id, name) in existing_entregas {
        entrega_name_to_id.insert(name, id);
    }
    
    // Get all existing albums to build the title->id map
    let mut album_title_to_id: HashMap<String, i64> = HashMap::new();
    let existing_albums: Vec<(i64, String)> =
        sqlx::query_as("SELECT id, title FROM albums")
            .fetch_all(&mut *tx)
            .await?;

    for (id, title) in existing_albums {
        album_title_to_id.insert(title, id);
    }

    // Process entregas from JSON
    if let Some(entregas) = &init_payload.entregas {
        for (json_index, entrega) in entregas.iter().enumerate() {
            let json_position = (json_index + 1) as i64; // JSON uses 1-based indexing
            
            // Check if this entrega already exists
            if let Some(&existing_id) = entrega_name_to_id.get(&entrega.name) {
                println!("  ⏭️  Entrega '{}' already exists, skipping", entrega.name);
                json_entrega_pos_to_db_id.insert(json_position, existing_id);
                continue;
            }

            // Create the entrega
            let entrega_id: i64 = sqlx::query_scalar(
                "INSERT INTO entregas (name, batch_id, created_at) VALUES ($1, $2, $3) RETURNING id",
            )
            .bind(&entrega.name)
            .bind(entrega.batch_id)
            .bind(&entrega.created_at)
            .fetch_one(&mut *tx)
            .await?;

            json_entrega_pos_to_db_id.insert(json_position, entrega_id);
            entrega_name_to_id.insert(entrega.name.clone(), entrega_id);
            new_entregas_count += 1;
            println!("  ✅ Created entrega: {}", entrega.name);
        }
    }

    // Process albums from JSON
    if let Some(albums) = &init_payload.albums {
        for (json_index, album) in albums.iter().enumerate() {
            let json_position = (json_index + 1) as i64; // JSON uses 1-based indexing
            
            // Check if this album already exists
            if let Some(&existing_id) = album_title_to_id.get(&album.title) {
                println!("  ⏭️  Album '{}' already exists, skipping", album.title);
                json_album_pos_to_db_id.insert(json_position, existing_id);
                continue;
            }

            // Create the album
            let album_id: i64 = sqlx::query_scalar(
                "INSERT INTO albums (title, artist_id, release_year, label, format, country, genre, style, created_at) 
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING id",
            )
            .bind(&album.title)
            .bind(album.artist_id)
            .bind(album.release_year)
            .bind(&album.label)
            .bind(&album.format)
            .bind(&album.country)
            .bind(&album.genre)
            .bind(&album.style)
            .bind(&album.created_at)
            .fetch_one(&mut *tx)
            .await?;

            json_album_pos_to_db_id.insert(json_position, album_id);
            album_title_to_id.insert(album.title.clone(), album_id);
            new_albums_count += 1;
            println!("  ✅ Created album: {}", album.title);
        }
    }

    // Get all existing tracks to detect duplicates
    let existing_tracks: Vec<(String, String, Option<i64>)> =
        sqlx::query_as("SELECT title, artist_name, entrega_id FROM tracks")
            .fetch_all(&mut *tx)
            .await?;
    
    let mut existing_tracks_set = std::collections::HashSet::new();
    for (title, artist, entrega_id) in existing_tracks {
        existing_tracks_set.insert((title, artist, entrega_id));
    }

    // Process tracks from JSON
    if let Some(tracks) = &init_payload.tracks {
        for track in tracks {
            // Map the JSON entrega_id to actual database ID
            let actual_entrega_id = track.entrega_id.and_then(|json_id| {
                json_entrega_pos_to_db_id.get(&json_id).copied()
            });
            
            // Map the JSON album_id to actual database ID
            let actual_album_id = track.album_id.and_then(|json_id| {
                json_album_pos_to_db_id.get(&json_id).copied()
            });
            
            // Check if this track already exists
            let track_key = (track.title.clone(), track.artist_name.clone(), actual_entrega_id);
            if existing_tracks_set.contains(&track_key) {
                continue; // Skip existing track silently
            }
            
            sqlx::query(
                "INSERT INTO tracks (title, artist_name, album_id, duration_seconds, bpm, tone, position, score, entrega_id, created_at) 
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"
            )
            .bind(&track.title)
            .bind(&track.artist_name)
            .bind(actual_album_id)
            .bind(track.duration_seconds)
            .bind(track.bpm)
            .bind(&track.tone)
            .bind(&track.position)
            .bind(&track.score)
            .bind(actual_entrega_id)
            .bind(&track.created_at)
            .execute(&mut *tx)
            .await?;

            new_tracks_count += 1;
            existing_tracks_set.insert(track_key);
        }

        if new_tracks_count > 0 {
            println!("     → Added {} tracks", new_tracks_count);
        }
    }

    tx.commit().await?;

    if new_entregas_count > 0 || new_albums_count > 0 || new_tracks_count > 0 {
        println!("\n✅ Initial data initialized successfully!");
        println!("   → {} entrega(s) created", new_entregas_count);
        println!("   → {} album(s) created", new_albums_count);
        println!("   → {} track(s) created", new_tracks_count);
    } else {
        println!("\n⏭️  All initial data already exists, nothing to add");
    }

    Ok(())
}
