use askama::Template;
use axum::{extract::Path, http::StatusCode, response::{Html, IntoResponse}};
use sqlx::{QueryBuilder, Row};

use crate::{templates::{NotFoundTemplate, PasteTemplate}, DB};

pub async fn get(Path(id): Path<String>) -> impl IntoResponse {
    let content: String = match QueryBuilder::new("SELECT * FROM pastes WHERE id = ")
        .push_bind(&id)
        .build()
        .fetch_one(&*DB).await {
            Ok(r) => r.get("data"),
            Err(sqlx::Error::RowNotFound) => return Html(NotFoundTemplate {}.render().unwrap()).into_response(),
            Err(e) => {
                {
                println!("{}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "An unexpected error occured.".to_string()).into_response()
                }
            }
        };
        let paste_template = PasteTemplate {
            id,
            content
        };
        (Html(paste_template.render().unwrap())).into_response()
}