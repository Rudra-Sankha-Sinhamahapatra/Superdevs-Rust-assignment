use axum::{extract::Json, http::StatusCode, response::Json as ResponseJson};
use base64::{Engine as _, engine::general_purpose};
use solana_sdk::signature::{Signature, Signer};

use crate::models::{
    ApiResponse, SignMessageData, SignMessageRequest, VerifyMessageData, VerifyMessageRequest,
};
use crate::utils::{keypair_from_base58, parse_pubkey};

pub async fn sign_message(
    Json(req): Json<SignMessageRequest>,
) -> (StatusCode, ResponseJson<ApiResponse<SignMessageData>>) {
    if req.message.is_empty() || req.secret.is_empty() {
        return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Missing required fields".to_string())));
    }

    let keypair = match keypair_from_base58(&req.secret) {
        Ok(kp) => kp,
        Err(err) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error(err))),
    };

    let message_bytes = req.message.as_bytes();
    let signature = keypair.sign_message(message_bytes);

    (StatusCode::OK, ResponseJson(ApiResponse::success(SignMessageData {
        signature: general_purpose::STANDARD.encode(&signature.as_ref()),
        public_key: keypair.pubkey().to_string(),
        message: req.message,
    })))
}

pub async fn verify_message(
    Json(req): Json<VerifyMessageRequest>,
) -> (StatusCode, ResponseJson<ApiResponse<VerifyMessageData>>) {
    let pubkey = match parse_pubkey(&req.pubkey) {
        Ok(key) => key,
        Err(err) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error(err))),
    };

    let signature_bytes = match general_purpose::STANDARD.decode(&req.signature) {
        Ok(bytes) => bytes,
        Err(_) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Invalid base64 signature".to_string()))),
    };

    let signature = match Signature::try_from(signature_bytes.as_slice()) {
        Ok(sig) => sig,
        Err(_) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Invalid signature format".to_string()))),
    };

    let message_bytes = req.message.as_bytes();
    let valid = signature.verify(&pubkey.to_bytes(), message_bytes);

    (StatusCode::OK, ResponseJson(ApiResponse::success(VerifyMessageData {
        valid,
        message: req.message,
        pubkey: req.pubkey,
    })))
} 