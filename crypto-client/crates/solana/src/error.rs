use chain_core::error::BlockchainClientError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SolanaClientError {
    #[error("solana rpc error: {0}")]
    RpcError(String),
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid private key")]
    InvalidPrivateKey,
    #[error("sign error: {0}")]
    SignError(String),
    #[error("invalid address: {0}")]
    InvalidAddress(String),
    #[error("transaction error: {0}")]
    TransactionError(String),
}

impl From<SolanaClientError> for BlockchainClientError {
    fn from(value: SolanaClientError) -> Self {
        eprintln!("{}", value);
        BlockchainClientError::Unknown(value.to_string())
    }
}
