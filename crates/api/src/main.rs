use axum::{
    routing::get,
    Router,
};
use dotenvy::dotenv;

mod auth;
mod error;
mod openai;
mod v1;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    //let res = openai::fine_tuning::run().unwrap();
    //println!("{}", res);
    let app = Router::new()
        .route("/health", get(healthcheck))
        .nest("/v1", v1::routes());
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:12000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn healthcheck() -> String {
    "alive".into()
}
