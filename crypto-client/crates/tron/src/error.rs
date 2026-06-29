use chain_core::error::BlockchainClientError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TronClientError {
    #[error("tron api error: {0}")]
    ApiError(String),
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Invalid private key")]
    InvalidPrivateKey,
    #[error("Sign Error: {0}")]
    SignError(String),
}

impl From<TronClientError> for BlockchainClientError {
    fn from(value: TronClientError) -> Self {
        eprintln!("{}", value);
        BlockchainClientError::Unknown(value.to_string())
    }
}
