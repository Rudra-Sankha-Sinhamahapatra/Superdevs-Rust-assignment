use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

#[derive(Serialize)]
pub struct KeypairData {
    pub pubkey: String,
    pub secret: String,
}

#[derive(Serialize)]
pub struct SignMessageData {
    pub signature: String,
    pub public_key: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct VerifyMessageData {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}

#[derive(Serialize)]
pub struct InstructionData {
    pub program_id: String,
    pub accounts: Vec<AccountInfo>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct AccountInfo {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Serialize)]
pub struct SolTransferData {
    pub program_id: String,
    pub accounts: Vec<String>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct TokenTransferData {
    pub program_id: String,
    pub accounts: Vec<TokenAccountInfo>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct TokenAccountInfo {
    pub pubkey: String,
    #[serde(rename = "isSigner")]
    pub is_signer: bool,
} 