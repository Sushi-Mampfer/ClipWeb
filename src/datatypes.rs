use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Create {
    pub content: Option<String>,
    pub private: Option<bool>,
}