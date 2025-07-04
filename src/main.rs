mod api;

use std::{net::SocketAddr, sync::LazyLock};

use axum::{routing::{get, post}, Router};
use sqlx::SqlitePool;
use tower_http::{cors::{Any, CorsLayer}, services::ServeFile};

static DB: LazyLock<SqlitePool> = LazyLock::new(|| SqlitePool::connect_lazy("sqlite://db.sqlite").unwrap());


#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let app = Router::new();
    
    let app = app
        .nest_service("/style.css", ServeFile::new("static/style.css"))
        .route("/create", post(api::create::create))
        .route("/{id}", get(api::get::get))
        .fallback_service(ServeFile::new("static/index.html"))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum_server::bind(addr).serve(app.into_make_service()).await.unwrap();
}
