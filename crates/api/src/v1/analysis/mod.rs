use crate::error::Result;
use crate::openai::completion;

use axum::{
    extract::Request,
    routing::post,
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/gh_action", post(gh_action))
        .route("/git_repo", post(git_repo))
}

async fn gh_action(body: String) -> Result<String> {
    Ok(completion::query("".into(), body)?)
}

async fn git_repo(url: String) -> Result<String> {
    Ok("Soon!".into())
}
