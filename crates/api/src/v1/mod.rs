use axum::Router;

mod analysis;
mod auth;

pub fn routes() -> Router {
    Router::new()
        .nest("/analysis", analysis::routes())
        .nest("/auth", auth::routes())
}
