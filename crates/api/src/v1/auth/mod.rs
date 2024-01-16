use crate::error::Result;

use axum::{
    extract::Json,
    routing::{delete, post},
    Router,
};

use crate::auth;

pub fn routes() -> Router {
    Router::new()
        .route("/api_key", delete(delete_api_key))
        .route("/api_key", post(create_api_key))
}

async fn create_api_key(Json(payload): Json<auth::APIKey>) -> Result<String> {
    // get authorization header

    Ok(auth::APIKey::new(payload).key)
}

async fn delete_api_key() {
    auth::APIKey::delete();
}
