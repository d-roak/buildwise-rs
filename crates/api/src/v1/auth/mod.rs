use axum::{
    routing::get,
    Router,
};

use crate::auth;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(root))
}

async fn root() -> String {
    "Hello, World!".into()
}

async fn create_api_key() -> String {
    "create api key".into()
}

async fn delete_api_key() -> String {
    "delete api key".into()
}

async fn valid_key(admin: Option<bool>) -> bool {
    let admin = admin.unwrap_or(false);
    let api_key = auth::get_api_key();
    false
}
