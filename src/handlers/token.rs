use axum::{extract::Json, http::StatusCode, response::Json as ResponseJson};
use spl_token::instruction as token_instruction;

use crate::models::{ApiResponse, CreateTokenRequest, InstructionData, MintTokenRequest};
use crate::utils::{instruction_to_response, parse_pubkey};

pub async fn create_token(
    Json(req): Json<CreateTokenRequest>,
) -> (StatusCode, ResponseJson<ApiResponse<InstructionData>>) {
    // Validate required fields
    let mint_authority = match &req.mint_authority {
        Some(val) if !val.is_empty() => val,
        _ => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Missing required fields".to_string()))),
    };
    
    let mint = match &req.mint {
        Some(val) if !val.is_empty() => val,
        _ => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Missing required fields".to_string()))),
    };
    
    let decimals = match req.decimals {
        Some(val) => val,
        None => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Missing required fields".to_string()))),
    };

    let mint_authority_pubkey = match parse_pubkey(mint_authority) {
        Ok(key) => key,
        Err(err) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error(err))),
    };
    
    let mint_pubkey = match parse_pubkey(mint) {
        Ok(key) => key,
        Err(err) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error(err))),
    };

    let instruction = match token_instruction::initialize_mint(
        &spl_token::id(),
        &mint_pubkey,
        &mint_authority_pubkey,
        Some(&mint_authority_pubkey),
        decimals,
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
    let mint = match &req.mint {
        Some(val) if !val.is_empty() => val,
        _ => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Missing required fields".to_string()))),
    };
    
    let destination = match &req.destination {
        Some(val) if !val.is_empty() => val,
        _ => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Missing required fields".to_string()))),
    };
    
    let authority = match &req.authority {
        Some(val) if !val.is_empty() => val,
        _ => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Missing required fields".to_string()))),
    };
    
    let amount = match req.amount {
        Some(val) if val > 0 => val,
        _ => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Missing required fields".to_string()))),
    };

    let mint_pubkey = match parse_pubkey(mint) {
        Ok(key) => key,
        Err(err) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error(err))),
    };
    
    let destination_pubkey = match parse_pubkey(destination) {
        Ok(key) => key,
        Err(err) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error(err))),
    };
    
    let authority_pubkey = match parse_pubkey(authority) {
        Ok(key) => key,
        Err(err) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error(err))),
    };

    let instruction = match token_instruction::mint_to(
        &spl_token::id(),
        &mint_pubkey,
        &destination_pubkey,
        &authority_pubkey,
        &[],
        amount,
    ) {
        Ok(inst) => inst,
        Err(_) => return (StatusCode::BAD_REQUEST, ResponseJson(ApiResponse::error("Failed to create mint instruction".to_string()))),
    };

    (StatusCode::OK, ResponseJson(ApiResponse::success(instruction_to_response(instruction))))
} 