use axum::{extract::Json, http::StatusCode, response::Json as ResponseJson};
use base64::{Engine as _, engine::general_purpose};
use solana_program::system_instruction;
use spl_token::instruction as token_instruction;

use crate::models::{ApiResponse, SendSolRequest, SendTokenRequest, SolTransferData, TokenTransferData, TokenAccountInfo};
use crate::utils::{parse_pubkey};

pub async fn send_sol(
    Json(req): Json<SendSolRequest>,
) -> (StatusCode, ResponseJson<ApiResponse<SolTransferData>>) {
    // Validate required fields
    let from = match &req.from {
        Some(val) if !val.is_empty() => val,
        _ => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Missing required fields".to_string()))),
    };
    
    let to = match &req.to {
        Some(val) if !val.is_empty() => val,
        _ => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Missing required fields".to_string()))),
    };
    
    let lamports = match req.lamports {
        Some(0) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Amount must be greater than 0".to_string()))),
        Some(val) => val,
        None => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Missing required fields".to_string()))),
    };

    let from_pubkey = match parse_pubkey(from) {
        Ok(key) => key,
        Err(err) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error(err))),
    };
    
    let to_pubkey = match parse_pubkey(to) {
        Ok(key) => key,
        Err(err) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error(err))),
    };

    let instruction = system_instruction::transfer(&from_pubkey, &to_pubkey, lamports);

    let response_data = SolTransferData {
        program_id: instruction.program_id.to_string(),
        accounts: instruction.accounts.iter().map(|acc| acc.pubkey.to_string()).collect(),
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    };

    (StatusCode::OK, ResponseJson(ApiResponse::success(response_data)))
}

pub async fn send_token(
    Json(req): Json<SendTokenRequest>,
) -> (StatusCode, ResponseJson<ApiResponse<TokenTransferData>>) {
    // Validate required fields
    let destination = match &req.destination {
        Some(val) if !val.is_empty() => val,
        _ => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Missing required fields".to_string()))),
    };
    
    let mint = match &req.mint {
        Some(val) if !val.is_empty() => val,
        _ => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Missing required fields".to_string()))),
    };
    
    let owner = match &req.owner {
        Some(val) if !val.is_empty() => val,
        _ => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Missing required fields".to_string()))),
    };
    
    let amount = match req.amount {
        Some(0) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Amount must be greater than 0".to_string()))),
        Some(val) => val,
        None => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Missing required fields".to_string()))),
    };

    let mint_pubkey = match parse_pubkey(mint) {
        Ok(key) => key,
        Err(err) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error(err))),
    };
    
    let owner_pubkey = match parse_pubkey(owner) {
        Ok(key) => key,
        Err(err) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error(err))),
    };
    
    let destination_pubkey = match parse_pubkey(destination) {
        Ok(key) => key,
        Err(err) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error(err))),
    };

    let source_ata = spl_associated_token_account::get_associated_token_address(&owner_pubkey, &mint_pubkey);
    let dest_ata = spl_associated_token_account::get_associated_token_address(&destination_pubkey, &mint_pubkey);

    let instruction = match token_instruction::transfer(
        &spl_token::id(),
        &source_ata,
        &dest_ata,
        &owner_pubkey,
        &[],
        amount,
    ) {
        Ok(inst) => inst,
        Err(_) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Failed to create transfer instruction".to_string()))),
    };

    let accounts = instruction
        .accounts
        .into_iter()
        .map(|acc| TokenAccountInfo {
            pubkey: acc.pubkey.to_string(),
            is_signer: acc.is_signer,
        })
        .collect();

    let response_data = TokenTransferData {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    };

    (StatusCode::OK, ResponseJson(ApiResponse::success(response_data)))
} 