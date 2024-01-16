use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct APIKey {
    pub admin: bool,
    pub key: String,
    pub scope: Vec<String>,
}

impl APIKey {
    pub fn delete() -> bool {
        true
    }

    pub fn new(payload: APIKey) -> Self {
        Self {
            admin: payload.admin,
            key: Uuid::new_v4().into(),
            scope: payload.scope,
        }
    }
}
