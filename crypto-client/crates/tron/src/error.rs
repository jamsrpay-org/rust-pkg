use chain_core::error::CryptoAssetClientError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TronClientError {
    #[error("tron api error: {0}")]
    ApiError(String),
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

impl From<TronClientError> for CryptoAssetClientError {
    fn from(value: TronClientError) -> Self {
        CryptoAssetClientError::Unknown
    }
}
