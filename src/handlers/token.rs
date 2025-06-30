use axum::{extract::Json, http::StatusCode, response::Json as ResponseJson};
use spl_token::instruction as token_instruction;

use crate::models::{ApiResponse, CreateTokenRequest, InstructionData, MintTokenRequest};
use crate::utils::{instruction_to_response, parse_pubkey};

pub async fn create_token(
    Json(req): Json<CreateTokenRequest>,
) -> Result<ResponseJson<ApiResponse<InstructionData>>, StatusCode> {
    let mint_authority = parse_pubkey(&req.mint_authority).map_err(|_| StatusCode::BAD_REQUEST)?;
    let mint = parse_pubkey(&req.mint).map_err(|_| StatusCode::BAD_REQUEST)?;

    let instruction = token_instruction::initialize_mint(
        &spl_token::id(),
        &mint,
        &mint_authority,
        Some(&mint_authority),
        req.decimals,
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ResponseJson(ApiResponse::success(instruction_to_response(
        instruction,
    ))))
}

pub async fn mint_token(
    Json(req): Json<MintTokenRequest>,
) -> Result<ResponseJson<ApiResponse<InstructionData>>, StatusCode> {
    let mint = parse_pubkey(&req.mint).map_err(|_| StatusCode::BAD_REQUEST)?;
    let destination = parse_pubkey(&req.destination).map_err(|_| StatusCode::BAD_REQUEST)?;
    let authority = parse_pubkey(&req.authority).map_err(|_| StatusCode::BAD_REQUEST)?;

    let instruction = token_instruction::mint_to(
        &spl_token::id(),
        &mint,
        &destination,
        &authority,
        &[],
        req.amount,
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ResponseJson(ApiResponse::success(instruction_to_response(
        instruction,
    ))))
} 