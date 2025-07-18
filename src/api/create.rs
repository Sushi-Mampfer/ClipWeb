use std::time::{Instant, SystemTime, UNIX_EPOCH};

use axum::{http::StatusCode, response::IntoResponse, Json};
use rand::distr::{Alphabetic, SampleString};

use crate::{datatypes::Create, DB, RL};

const MAX_TIME: i32 = 24 * 60 * 60;

pub async fn create(Json(json): Json<Create>) -> impl IntoResponse {

    match RL.lock() {
        Ok(mut l) => {
            l.push(Instant::now());
            if l.len() > 1000 {
                return (StatusCode::TOO_MANY_REQUESTS, "Global Ratelimit exceeded.".to_string());
            }
        }
        Err(e) => {
            println!("{}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "An unexpected error occured.".to_string())
        }
    }

    let content = match json.content {
        Some(content) if content == "".to_string() => return (StatusCode::BAD_REQUEST, "Don't waste my storage.".to_string()),
        Some(content) => content,
        None => return (StatusCode::BAD_REQUEST, "Don't waste my storage.".to_string())
    };
    let private = match json.private {
        Some(p) => p,
        None => return (StatusCode::BAD_REQUEST, "Private field has to be set.".to_string())
    };

    let expiery = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i32) + MAX_TIME;

    let mut id = String::new();

    for i in 0..100 {
        if i == 99 {
            return (StatusCode::INSUFFICIENT_STORAGE, "No IDs available, try again later.".to_string())
        }
        id = Alphabetic.sample_string(&mut rand::rng(), 5).to_lowercase();
        match sqlx::query!("INSERT into pastes (id, data, expiery, private) VALUES (?, ?, ?, ?)", id, content, expiery, private).execute(&*DB).await {
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