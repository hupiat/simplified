use axum::{extract::Json, routing::post, Router, serve};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct LaunchRequest {
    token_name: String,
    symbol: String,
    decimals: u8,
}

#[derive(Serialize)]
struct LaunchResponse {
    status: String,
    message: String,
}

async fn handle_launch(Json(payload): Json<LaunchRequest>) -> Json<LaunchResponse> {
    println!(
        "Received launch request: {} ({})",
        payload.token_name, payload.symbol
    );

    // Ici tu peux construire ta TX avec web3.rs ou solana-client

    Json(LaunchResponse {
        status: "ok".to_string(),
        message: format!("Token {} launched!", payload.symbol),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/launch", post(handle_launch));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Rust backend listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    serve(listener, app).await.unwrap();
}
