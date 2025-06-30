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
    if req.from.is_empty() || req.to.is_empty() || req.lamports == 0 {
        return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Missing required fields".to_string())));
    }

    let from = match parse_pubkey(&req.from) {
        Ok(key) => key,
        Err(err) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error(err))),
    };
    
    let to = match parse_pubkey(&req.to) {
        Ok(key) => key,
        Err(err) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error(err))),
    };

    let instruction = system_instruction::transfer(&from, &to, req.lamports);

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
    if req.destination.is_empty() || req.mint.is_empty() || req.owner.is_empty() || req.amount == 0 {
        return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Missing required fields".to_string())));
    }

    let mint = match parse_pubkey(&req.mint) {
        Ok(key) => key,
        Err(err) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error(err))),
    };
    
    let owner = match parse_pubkey(&req.owner) {
        Ok(key) => key,
        Err(err) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error(err))),
    };
    
    let destination = match parse_pubkey(&req.destination) {
        Ok(key) => key,
        Err(err) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error(err))),
    };

    if req.amount == 0 {
        return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Amount must be greater than 0".to_string())));
    }

    let source_ata = spl_associated_token_account::get_associated_token_address(&owner, &mint);
    let dest_ata = spl_associated_token_account::get_associated_token_address(&destination, &mint);

    let instruction = match token_instruction::transfer(
        &spl_token::id(),
        &source_ata,
        &dest_ata,
        &owner,
        &[],
        req.amount,
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