use axum::response::Json as ResponseJson;
use solana_sdk::signature::{Keypair, Signer};

use crate::models::{ApiResponse, KeypairData};

pub async fn generate_keypair() -> ResponseJson<ApiResponse<KeypairData>> {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    let secret = bs58::encode(&keypair.to_bytes()).into_string();

    ResponseJson(ApiResponse::success(KeypairData { pubkey, secret }))
} 