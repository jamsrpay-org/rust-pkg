#[derive(thiserror::Error, Debug)]
pub enum CryptoAssetClientError {
    #[error("rpc error: {0}")]
    Rpc(String),

    #[error("invalid address")]
    InvalidAddress,

    #[error("insufficient balance")]
    InsufficientBalance,

    #[error("Error: {0}")]
    Unknown(String),

    #[error("invalid transaction: {0}")]
    InvalidTransaction(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ChainClientError {}
