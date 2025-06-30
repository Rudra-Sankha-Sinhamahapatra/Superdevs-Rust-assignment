use axum::{extract::Json, http::StatusCode, response::Json as ResponseJson};
use spl_token::instruction as token_instruction;

use crate::models::{ApiResponse, CreateTokenRequest, InstructionData, MintTokenRequest};
use crate::utils::{instruction_to_response, parse_pubkey};

pub async fn create_token(
    Json(req): Json<CreateTokenRequest>,
) -> (StatusCode, ResponseJson<ApiResponse<InstructionData>>) {
    // Validate required fields
    if req.mint_authority.is_empty() || req.mint.is_empty() {
        return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Missing required fields".to_string())));
    }

    let mint_authority = match parse_pubkey(&req.mint_authority) {
        Ok(key) => key,
        Err(err) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error(err))),
    };
    
    let mint = match parse_pubkey(&req.mint) {
        Ok(key) => key,
        Err(err) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error(err))),
    };

    let instruction = match token_instruction::initialize_mint(
        &spl_token::id(),
        &mint,
        &mint_authority,
        Some(&mint_authority),
        req.decimals,
    ) {
        Ok(inst) => inst,
        Err(_) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Failed to create token instruction".to_string()))),
    };

    (StatusCode::OK, ResponseJson(ApiResponse::success(instruction_to_response(instruction))))
}

pub async fn mint_token(
    Json(req): Json<MintTokenRequest>,
) -> (StatusCode, ResponseJson<ApiResponse<InstructionData>>) {
    // Validate required fields
    if req.mint.is_empty() || req.destination.is_empty() || req.authority.is_empty() || req.amount == 0 {
        return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Missing required fields".to_string())));
    }

    let mint = match parse_pubkey(&req.mint) {
        Ok(key) => key,
        Err(err) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error(err))),
    };
    
    let destination = match parse_pubkey(&req.destination) {
        Ok(key) => key,
        Err(err) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error(err))),
    };
    
    let authority = match parse_pubkey(&req.authority) {
        Ok(key) => key,
        Err(err) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error(err))),
    };

    let instruction = match token_instruction::mint_to(
        &spl_token::id(),
        &mint,
        &destination,
        &authority,
        &[],
        req.amount,
    ) {
        Ok(inst) => inst,
        Err(_) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Failed to create mint instruction".to_string()))),
    };

    (StatusCode::OK, ResponseJson(ApiResponse::success(instruction_to_response(instruction))))
} 