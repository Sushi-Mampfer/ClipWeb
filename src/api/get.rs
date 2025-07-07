use askama::Template;
use axum::{extract::Path, http::StatusCode, response::{Html, IntoResponse, Redirect}};
use sqlx::{QueryBuilder, Row};

use crate::{templates::{PasteTemplate}, DB};

pub async fn get(Path(id): Path<String>) -> impl IntoResponse {
    let content: String = match QueryBuilder::new("SELECT * FROM pastes WHERE id = ")
        .push_bind(&id)
        .build()
        .fetch_one(&*DB).await {
            Ok(r) => {
                if r.get("private") {
                    let _ = QueryBuilder::new("DELETE FROM pastes WHERE id = ")
                    .push_bind(&id)
                    .build()
                    .execute(&*DB).await;
                }
                r.get("data")
            }
            Err(sqlx::Error::RowNotFound) => return Redirect::to("/").into_response(),
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