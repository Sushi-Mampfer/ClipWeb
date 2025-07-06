mod api;
mod datatypes;
mod templates;

use std::{net::SocketAddr, sync::{LazyLock, Mutex}, time::{Duration, Instant}};

use axum::{routing::{get, post}, Router};
use sqlx::SqlitePool;
use tokio::time::sleep;
use tower_http::{cors::{Any, CorsLayer}, services::ServeFile};

use crate::api::expiery::{clear_ratelimit, remove_expired};

static DB: LazyLock<SqlitePool> = LazyLock::new(|| SqlitePool::connect_lazy("sqlite://db.sqlite").unwrap());
static RL: LazyLock<Mutex<Vec<Instant>>> = LazyLock::new(|| Mutex::new(Vec::new()));


#[tokio::main]
async fn main() {
    tokio::spawn(async move {
        loop {
            remove_expired().await;
            sleep(Duration::from_secs(10)).await;
        }
    });

    tokio::spawn(async move {
        loop {
            clear_ratelimit().await;
            sleep(Duration::from_secs(1)).await;
        }
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let app = Router::new();
    
    let app = app
        .route("/create", post(api::create::create))
        .route("/{id}", get(api::get::get))
        .fallback_service(ServeFile::new("static/index.html"))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum_server::bind(addr).serve(app.into_make_service()).await.unwrap();
}
