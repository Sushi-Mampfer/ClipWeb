use std::time::{SystemTime, UNIX_EPOCH};

use sqlx::QueryBuilder;

use crate::DB;

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