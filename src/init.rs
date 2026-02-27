use sqlx::SqlitePool;

/// Initialize database with initial data
/// This function is idempotent - it only runs if the database is empty
/// Similar to NEAR blockchain's init pattern
pub async fn init_data(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    // Check if data already exists (fail fast if initialized)
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM entregas")
        .fetch_one(pool)
        .await?;

    if count.0 > 0 {
        println!("⏭️  Database already initialized, skipping init data");
        return Ok(());
    }

    println!("🔧 Initializing database with initial data...");

    let mut tx = pool.begin().await?;

    // Create first entrega
    sqlx::query("INSERT INTO entregas (name, batch_id, created_at) VALUES (?1, ?2, ?3)")
        .bind("Primera Entrega")
        .bind(None::<i64>)
        .bind("2026-02-26T00:00:00Z")
        .execute(&mut *tx)
        .await?;

    // Insert tracks
    let tracks = vec![
        ("A1", "Fred Fresh", "5 Mouths", 126.46, "Db", "✅"),
        ("A2", "Bob Fitzgerald", "Mihac", 129.25, "Db", "🟡"),
        ("B1", "Kem M", "The Burger King", 129.83, "Fm", "✅"),
        ("B2", "Mind Phase One", "Work", 122.00, "Bb", "🔴"),
        (
            "C1",
            "Chris Sattinger",
            "Tonque Crystal Moss",
            131.98,
            "Db",
            "✅",
        ),
        (
            "C2",
            "Astrocat & Kenny S.",
            "Conception",
            128.00,
            "Fm",
            "😈",
        ),
        ("D1", "ESP", "Teacher", 129.27, "Cm", "🟡"),
        (
            "D2",
            "Deadly Buddha",
            "The Gods Must Be Crazy",
            140.00,
            "Cm",
            "🔴",
        ),
    ];

    for (position, artist, title, bpm, tone, score) in tracks {
        sqlx::query(
            "INSERT INTO tracks (title, artist_name, album_id, duration_seconds, bpm, tone, position, score, entrega_id, created_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)"
        )
        .bind(title)
        .bind(artist)
        .bind(None::<i64>)
        .bind(None::<i64>)
        .bind(bpm)
        .bind(tone)
        .bind(position)
        .bind(score)
        .bind(1) // entrega_id
        .bind("2026-02-26T00:00:00Z")
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    println!("✅ Initial data created successfully!");
    println!("   → 1 entrega created");
    println!("   → 8 tracks created");

    Ok(())
}
