use chain_core::error::BlockchainClientError;

#[derive(thiserror::Error, Debug)]
pub enum UtxoClientError {
    #[error("utxo rpc error: {0}")]
    RpcError(String),

    #[error("invalid address: {0}")]
    InvalidAddress(String),

    #[error("invalid private key")]
    InvalidPrivateKey,

    #[error("insufficient funds")]
    InsufficientFunds,

    #[error("sign error: {0}")]
    SignError(String),
}

impl From<UtxoClientError> for BlockchainClientError {
    fn from(value: UtxoClientError) -> Self {
        eprintln!("{}", value);
        match value {
            UtxoClientError::RpcError(msg) => BlockchainClientError::Rpc(msg),
            UtxoClientError::InvalidAddress(_) => BlockchainClientError::InvalidAddress,
            UtxoClientError::InvalidPrivateKey => {
                BlockchainClientError::Unknown("invalid private key".into())
            }
            UtxoClientError::InsufficientFunds => BlockchainClientError::InsufficientBalance,
            UtxoClientError::SignError(msg) => {
                BlockchainClientError::InvalidTransaction(format!("sign error: {}", msg))
            }
        }
    }
}
