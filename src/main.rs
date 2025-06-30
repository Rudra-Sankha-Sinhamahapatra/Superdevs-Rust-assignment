use axum::{Router, routing::post};
use tower_http::cors::CorsLayer;

mod handlers;
mod models;
mod utils;

use handlers::*;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/keypair", post(generate_keypair))
        .route("/token/create", post(create_token))
        .route("/token/mint", post(mint_token))
        .route("/message/sign", post(sign_message))
        .route("/message/verify", post(verify_message))
        .route("/send/sol", post(send_sol))
        .route("/send/token", post(send_token))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8084")
        .await
        .expect("Failed to bind to address");

    println!("Solana HTTP Server running on http://0.0.0.0:8084");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}