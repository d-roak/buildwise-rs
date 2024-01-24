use crate::error::Result;

use axum::{
    extract::Path,
    routing::{delete, get, post},
    Json,
    Router,
};

use crate::git::repos;

pub fn routes() -> Router {
    Router::new()
        .route("/repos", get(list_repos))
        .route("/repos", post(add_repo))
        .route("/repos/:name", delete(delete_repo))
}

async fn add_repo(body: String) -> Result<()> {
    Ok(repos::add_repo(body)?)
}

async fn delete_repo(Path(name): Path<String>) -> Result<()> {
    Ok(repos::delete_repo(name)?)
}

async fn list_repos() -> Result<Json<Vec<String>>> {
    Ok(Json(repos::list_repos()?))
}
