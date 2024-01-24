use axum::Router;

mod analysis;
mod auth;
mod repos;

pub fn routes() -> Router {
    Router::new()
        .nest("/analysis", analysis::routes())
        .nest("/auth", auth::routes())
        .nest("/repos", repos::routes())
}
