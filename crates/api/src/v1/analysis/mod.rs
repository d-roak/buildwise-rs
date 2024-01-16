use crate::error::Result;
use crate::openai::completion;

use axum::{
    routing::get,
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(root))
}

async fn root() -> Result<String> {
    Ok(completion::query("".into(), "string[] cars = [\"ford\", \"audi\", \"chevrolet\"];".into())?)
}
