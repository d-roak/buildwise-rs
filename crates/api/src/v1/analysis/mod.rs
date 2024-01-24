use crate::error::Result;
use crate::openai::completion;

use axum::{
    routing::post,
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/gh_action", post(gh_action))
}

async fn gh_action(body: String) -> Result<String> {
    Ok(completion::query("".into(), body)?)
}
