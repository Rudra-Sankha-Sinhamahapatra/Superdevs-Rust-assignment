use base64::{Engine as _, engine::general_purpose};
use solana_program::{instruction::Instruction, pubkey::Pubkey};
use solana_sdk::signature::Keypair;
use std::str::FromStr;

use crate::models::{AccountInfo, InstructionData};

pub fn parse_pubkey(key_str: &str) -> Result<Pubkey, String> {
    Pubkey::from_str(key_str).map_err(|_| format!("Invalid public key: {}", key_str))
}

pub fn keypair_from_base58(secret_str: &str) -> Result<Keypair, String> {
    let secret_bytes = bs58::decode(secret_str)
        .into_vec()
        .map_err(|_| "Invalid base58 secret key")?;

    if secret_bytes.len() != 64 {
        return Err("Secret key must be 64 bytes".to_string());
    }

    Keypair::from_bytes(&secret_bytes).map_err(|_| "Invalid keypair bytes".to_string())
}

pub fn instruction_to_response(instruction: Instruction) -> InstructionData {
    let accounts = instruction
        .accounts
        .into_iter()
        .map(|acc| AccountInfo {
            pubkey: acc.pubkey.to_string(),
            is_signer: acc.is_signer,
            is_writable: acc.is_writable,
        })
        .collect();

    InstructionData {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    }
} 