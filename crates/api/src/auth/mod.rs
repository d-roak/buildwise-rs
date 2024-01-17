use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct APIKey {
    pub admin: bool,
    pub key: String,
    pub scope: Vec<String>,
}

impl APIKey {
    pub fn new() -> Self {
        Self {
            admin: false,
            key: Uuid::new_v4().into(),
            scope: vec![],
        }
    }
}

// use diesel for these operations
pub fn get_api_key(key: String) -> APIKey {
    APIKey {
        admin: false,
        key,
        scope: vec![],
    }
}

pub fn delete_api_key() -> bool {
    true
}
