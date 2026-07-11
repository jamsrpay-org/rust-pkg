use chain_core::error::BlockchainClientError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BscClientError {
    #[error("bsc rpc error: {0}")]
    RpcError(String),
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Invalid private key")]
    InvalidPrivateKey,
    #[error("Sign Error: {0}")]
    SignError(String),
    #[error("RLP encode error: {0}")]
    RlpEncodeError(String),
}

impl From<BscClientError> for BlockchainClientError {
    fn from(value: BscClientError) -> Self {
        eprintln!("{}", value);
        BlockchainClientError::Unknown(value.to_string())
    }
}
