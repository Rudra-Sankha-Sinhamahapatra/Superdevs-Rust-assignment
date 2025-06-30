use axum::{extract::Json, http::StatusCode, response::Json as ResponseJson};
use base64::{Engine as _, engine::general_purpose};
use solana_sdk::signature::{Signature, Signer};

use crate::models::{
    ApiResponse, SignMessageData, SignMessageRequest, VerifyMessageData, VerifyMessageRequest,
};
use crate::utils::{keypair_from_base58, parse_pubkey};

pub async fn sign_message(
    Json(req): Json<SignMessageRequest>,
) -> Result<ResponseJson<ApiResponse<SignMessageData>>, StatusCode> {
    if req.message.is_empty() || req.secret.is_empty() {
        return Ok(ResponseJson(ApiResponse {
            success: false,
            data: None,
            error: Some("Missing required fields".to_string()),
        }));
    }

    let keypair = keypair_from_base58(&req.secret).map_err(|_| StatusCode::BAD_REQUEST)?;

    let message_bytes = req.message.as_bytes();
    let signature = keypair.sign_message(message_bytes);

    Ok(ResponseJson(ApiResponse::success(SignMessageData {
        signature: general_purpose::STANDARD.encode(&signature.as_ref()),
        public_key: keypair.pubkey().to_string(),
        message: req.message,
    })))
}

pub async fn verify_message(
    Json(req): Json<VerifyMessageRequest>,
) -> Result<ResponseJson<ApiResponse<VerifyMessageData>>, StatusCode> {
    let pubkey = parse_pubkey(&req.pubkey).map_err(|_| StatusCode::BAD_REQUEST)?;

    let signature_bytes = general_purpose::STANDARD.decode(&req.signature).map_err(|_| StatusCode::BAD_REQUEST)?;

    let signature =
        Signature::try_from(signature_bytes.as_slice()).map_err(|_| StatusCode::BAD_REQUEST)?;

    let message_bytes = req.message.as_bytes();
    let valid = signature.verify(&pubkey.to_bytes(), message_bytes);

    Ok(ResponseJson(ApiResponse::success(VerifyMessageData {
        valid,
        message: req.message,
        pubkey: req.pubkey,
    })))
} 