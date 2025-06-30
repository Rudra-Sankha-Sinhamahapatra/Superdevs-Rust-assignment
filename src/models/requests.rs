use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    #[serde(rename = "mintAuthority")]
    pub mint_authority: Option<String>,
    pub mint: Option<String>,
    pub decimals: Option<u8>,
}

#[derive(Deserialize)]
pub struct MintTokenRequest {
    pub mint: Option<String>,
    pub destination: Option<String>,
    pub authority: Option<String>,
    pub amount: Option<u64>,
}

#[derive(Deserialize)]
pub struct SignMessageRequest {
    pub message: Option<String>,
    pub secret: Option<String>,
}

#[derive(Deserialize)]
pub struct VerifyMessageRequest {
    pub message: Option<String>,
    pub signature: Option<String>,
    pub pubkey: Option<String>,
}

#[derive(Deserialize)]
pub struct SendSolRequest {
    pub from: Option<String>,
    pub to: Option<String>,
    pub lamports: Option<u64>,
}

#[derive(Deserialize)]
pub struct SendTokenRequest {
    pub destination: Option<String>,
    pub mint: Option<String>,
    pub owner: Option<String>,
    pub amount: Option<u64>,
} 