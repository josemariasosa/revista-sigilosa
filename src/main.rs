mod db;
mod init;
mod models;
mod routes;

use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use axum::Router;
use db::{create_pool, run_migrations};
use routes::{app_routes, AppState};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let articles_dir = PathBuf::from("./articles");
    ensure_articles_dir(&articles_dir).await?;
    println!("✓ Articles directory is readable: {}", articles_dir.display());

    let pool = create_pool("sqlite://sonido_sigiloso.db").await?;
    run_migrations(&pool).await?;

    // Initialize with initial data (only runs if database is empty)
    init::init_data(&pool).await?;

    let state = AppState { pool, articles_dir };
    let app: Router = app_routes(state).layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("Server running at http://{addr}");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn ensure_articles_dir(path: &Path) -> Result<(), std::io::Error> {
    if tokio::fs::metadata(path).await.is_err() {
        tokio::fs::create_dir_all(path).await?;
    }

    let _ = tokio::fs::read_dir(path).await?;
    Ok(())
}
