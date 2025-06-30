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
    let message = match &req.message {
        Some(val) if !val.is_empty() => val,
        _ => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Missing required fields".to_string()))),
    };
    
    let secret = match &req.secret {
        Some(val) if !val.is_empty() => val,
        _ => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Missing required fields".to_string()))),
    };

    let keypair = match keypair_from_base58(secret) {
        Ok(kp) => kp,
        Err(err) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error(err))),
    };

    let message_bytes = message.as_bytes();
    let signature = keypair.sign_message(message_bytes);

    (StatusCode::OK, ResponseJson(ApiResponse::success(SignMessageData {
        signature: general_purpose::STANDARD.encode(&signature.as_ref()),
        public_key: keypair.pubkey().to_string(),
        message: message.clone(),
    })))
}

pub async fn verify_message(
    Json(req): Json<VerifyMessageRequest>,
) -> (StatusCode, ResponseJson<ApiResponse<VerifyMessageData>>) {
    let message = match &req.message {
        Some(val) if !val.is_empty() => val,
        _ => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Missing required fields".to_string()))),
    };
    
    let signature_str = match &req.signature {
        Some(val) if !val.is_empty() => val,
        _ => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Missing required fields".to_string()))),
    };
    
    let pubkey_str = match &req.pubkey {
        Some(val) if !val.is_empty() => val,
        _ => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Missing required fields".to_string()))),
    };

    let pubkey = match parse_pubkey(pubkey_str) {
        Ok(key) => key,
        Err(err) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error(err))),
    };

    let signature_bytes = match general_purpose::STANDARD.decode(signature_str) {
        Ok(bytes) => bytes,
        Err(_) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Invalid base64 signature".to_string()))),
    };

    let signature = match Signature::try_from(signature_bytes.as_slice()) {
        Ok(sig) => sig,
        Err(_) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Invalid signature format".to_string()))),
    };

    let message_bytes = message.as_bytes();
    let valid = signature.verify(&pubkey.to_bytes(), message_bytes);

    (StatusCode::OK, ResponseJson(ApiResponse::success(VerifyMessageData {
        valid,
        message: message.clone(),
        pubkey: pubkey_str.clone(),
    })))
} 