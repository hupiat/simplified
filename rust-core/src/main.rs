use axum::{extract::Json, routing::post, Router, serve};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, str::FromStr};
use tokio::net::TcpListener;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{Instruction, AccountMeta},
    message::Message,
    pubkey::Pubkey,
    signature::{read_keypair_file, Signer},
    transaction::Transaction,
};

#[derive(Deserialize)]
struct LaunchRequest {
    token_name: String,
    symbol: String,
    decimals: u8,
}

#[derive(Serialize)]
struct LaunchResponse {
    status: String,
    signature: String,
}

async fn handle_launch(Json(payload): Json<LaunchRequest>) -> Json<LaunchResponse> {
    let client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    let payer = read_keypair_file("~/.config/solana/id.json").expect("keypair failed");

    let program_id = Pubkey::from_str("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P").unwrap();

    let (bonding_curve, _) = Pubkey::find_program_address(
        &[b"bonding-curve", payer.pubkey().as_ref()],
        &program_id,
    );

    let accounts = vec![
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new(bonding_curve, false),
    ];

    let mut data = anchor_discriminator("global", "launch");
    data.extend([payload.decimals, 0]); // placeholder

    let ix = Instruction { program_id, accounts, data };
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let message = Message::new(&[ix], Some(&payer.pubkey()));
    let tx = Transaction::new(&[&payer], message, recent_blockhash);
    let sig = client.send_and_confirm_transaction(&tx).unwrap();

    Json(LaunchResponse { status: "ok".into(), signature: sig.to_string() })
}

fn anchor_discriminator(namespace: &str, name: &str) -> Vec<u8> {
    let hash = solana_sdk::hash::hash(format!("{}:{}", namespace, name).as_bytes());
    hash.to_bytes()[..8].to_vec()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/launch", post(handle_launch));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {:?}", addr);
    serve(listener, app).await.unwrap();
}
