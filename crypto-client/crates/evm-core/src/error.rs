use chain_core::error::BlockchainClientError;

#[derive(thiserror::Error, Debug)]
pub enum EvmClientError {
    #[error("evm rpc error: {0}")]
    RpcError(String),

    #[error("invalid private key")]
    InvalidPrivateKey,

    #[error("sign error: {0}")]
    SignError(String),
}

impl From<EvmClientError> for BlockchainClientError {
    fn from(value: EvmClientError) -> Self {
        eprintln!("{}", value);
        match value {
            EvmClientError::RpcError(msg) => BlockchainClientError::Rpc(msg),
            EvmClientError::InvalidPrivateKey => {
                BlockchainClientError::Unknown("invalid private key".into())
            }
            EvmClientError::SignError(msg) => {
                BlockchainClientError::InvalidTransaction(format!("sign error: {}", msg))
            }
        }
    }
}
