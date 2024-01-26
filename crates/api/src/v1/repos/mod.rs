use crate::error::Result;

use axum::{
    extract::Path,
    routing::{delete, get, post},
    Json,
    Router,
};

use crate::git::{parse, repos};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(list_repos))
        .route("/", post(add_repo))
        .route("/:name", delete(delete_repo))
        .route("/:name", get(test))
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

async fn test(Path(name): Path<String>) -> Result<Json<Vec<String>>> {
    Ok(Json(parse::get_src_files(name)?))
}
