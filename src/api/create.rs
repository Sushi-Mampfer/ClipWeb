use std::time::{SystemTime, UNIX_EPOCH};

use axum::{http::StatusCode, response::IntoResponse, Json};
use rand::distr::{Alphabetic, SampleString};

use crate::{datatypes::Create, DB};

const MAX_TIME: i32 = 24 * 60 * 60;

pub async fn create(Json(json): Json<Create>) -> impl IntoResponse {
    let content = match json.content {
        Some(content) => content,
        None => return (StatusCode::BAD_REQUEST, "Don't waste my storage.".to_string())
    };
    let expiery = match json.expiery {
        Some(expiery) if expiery <= 0 => MAX_TIME,
        Some(expiery) if expiery > MAX_TIME => MAX_TIME,
        Some(expiery) => expiery,
        None => MAX_TIME,
    };

    let expiery = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i32) + expiery;

    let mut id = String::new();

    for i in 0..100 {
        if i == 99 {
            return (StatusCode::INSUFFICIENT_STORAGE, "No IDs available, try again later.".to_string())
        }
        id = Alphabetic.sample_string(&mut rand::rng(), 5).to_lowercase();
        match sqlx::query!("INSERT into pastes (id, data, expiery) VALUES (?, ?, ?)", id, content, expiery).execute(&*DB).await {
            Ok(_) => break,
            Err(sqlx::Error::Database(e)) if e.code() == Some(std::borrow::Cow::Borrowed("1555")) => continue,
            Err(e) => {
                println!("{}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "An unexpected error occured.".to_string())
            }
        };
    }
    (StatusCode::OK, id)
}