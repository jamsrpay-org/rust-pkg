#[derive(thiserror::Error, Debug)]
pub enum CryptoAssetClientError {
    #[error("rpc error: {0}")]
    Rpc(String),

    #[error("invalid address")]
    InvalidAddress,

    #[error("insufficient balance")]
    InsufficientBalance,

    #[error("unknown error")]
    Unknown,
}

#[derive(Debug, thiserror::Error)]
pub enum ChainClientError {}
