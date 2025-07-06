use std::time::{SystemTime, UNIX_EPOCH};

use sqlx::QueryBuilder;

use crate::{DB, RL};

pub async fn remove_expired() {
    match QueryBuilder::new("DELETE FROM pastes WHERE expiery < ")
        .push_bind(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i32)
        .build().execute(&*DB).await {
            Ok(r) if r.rows_affected() == 0 => (),
            Ok(r) if r.rows_affected() == 1 => println!("{} paste removed", r.rows_affected()),
            Ok(r) => println!("{} pastes removed", r.rows_affected()),
            Err(e) => println!("{}", e),
        }
}

pub async fn clear_ratelimit() {
    match RL.lock() {
        Ok(mut l) => {
            if l.len() > 1000 {
                println!("Global ratelimit exceeded, {} requests in the last 60 seconds.", l.len());
            }
            l.retain(|v| v.elapsed().as_secs() < 60);
        }
        Err(e) => panic!("{}", e) 
    }
}