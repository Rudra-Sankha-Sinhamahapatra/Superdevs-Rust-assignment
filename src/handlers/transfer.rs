use axum::{extract::Json, http::StatusCode, response::Json as ResponseJson};
use base64::{Engine as _, engine::general_purpose};
use solana_program::system_instruction;
use spl_token::instruction as token_instruction;

use crate::models::{ApiResponse, SendSolRequest, SendTokenRequest, SolTransferData, TokenTransferData, TokenAccountInfo};
use crate::utils::{parse_pubkey};

pub async fn send_sol(
    Json(req): Json<SendSolRequest>,
) -> Result<ResponseJson<ApiResponse<SolTransferData>>, StatusCode> {
    let from = parse_pubkey(&req.from).map_err(|_| StatusCode::BAD_REQUEST)?;
    let to = parse_pubkey(&req.to).map_err(|_| StatusCode::BAD_REQUEST)?;

    if req.lamports == 0 {
        return Ok(ResponseJson(ApiResponse {
            success: false,
            data: None,
            error: Some("Amount must be greater than 0".to_string()),
        }));
    }

    let instruction = system_instruction::transfer(&from, &to, req.lamports);

    let response_data = SolTransferData {
        program_id: instruction.program_id.to_string(),
        accounts: instruction.accounts.iter().map(|acc| acc.pubkey.to_string()).collect(),
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    };

    Ok(ResponseJson(ApiResponse::success(response_data)))
}

pub async fn send_token(
    Json(req): Json<SendTokenRequest>,
) -> Result<ResponseJson<ApiResponse<TokenTransferData>>, StatusCode> {
    let mint = parse_pubkey(&req.mint).map_err(|_| StatusCode::BAD_REQUEST)?;
    let owner = parse_pubkey(&req.owner).map_err(|_| StatusCode::BAD_REQUEST)?;
    let destination = parse_pubkey(&req.destination).map_err(|_| StatusCode::BAD_REQUEST)?;

    if req.amount == 0 {
        return Ok(ResponseJson(ApiResponse {
            success: false,
            data: None,
            error: Some("Amount must be greater than 0".to_string()),
        }));
    }

    let source_ata = spl_associated_token_account::get_associated_token_address(&owner, &mint);
    let dest_ata = spl_associated_token_account::get_associated_token_address(&destination, &mint);

    let instruction = token_instruction::transfer(
        &spl_token::id(),
        &source_ata,
        &dest_ata,
        &owner,
        &[],
        req.amount,
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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

    Ok(ResponseJson(ApiResponse::success(response_data)))
} 