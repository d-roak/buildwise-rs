use axum::{
    routing::get,
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(root))
}

async fn root() -> String {
    "Hello, World!".into()
}
