use crate::error::Result;
use crate::openai::completion;

use axum::{
    extract::Request,
    routing::get,
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(root))
}

async fn root(req: Request) -> Result<String> {
    println!("{:?}", req.headers().get("Authorization"));

    Ok(completion::query("".into(), "string[] cars = [\"ford\", \"audi\", \"chevrolet\"];".into())?)
}
