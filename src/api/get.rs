use axum::{extract::Path, http::StatusCode, response::IntoResponse};

pub async fn get(Path(id): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, id)
}