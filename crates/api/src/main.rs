use axum::{
    routing::get,
    Router,
};

use std::env;

mod auth;
mod v1;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(healthcheck))
        .nest("/v1", v1::routes());
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:12000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn healthcheck() -> String {
    "alive".into()
}
