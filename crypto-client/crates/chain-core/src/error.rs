#[derive(thiserror::Error, Debug)]
pub enum BlockchainClientError {
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

    #[error("unsupported asset: {0}")]
    UnsupportedAsset(String),
}

impl From<registry::AssetNetworkRegistryError> for BlockchainClientError {
    fn from(value: registry::AssetNetworkRegistryError) -> Self {
        BlockchainClientError::UnsupportedAsset(value.to_string())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ChainClientError {}

