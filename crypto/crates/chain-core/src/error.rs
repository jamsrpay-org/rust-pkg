#[derive(thiserror::Error, Debug)]
pub enum ChainError {
    #[error("rpc error: {0}")]
    Rpc(String),

    #[error("invalid address")]
    InvalidAddress,

    #[error("insufficient balance")]
    InsufficientBalance,

    #[error("unknown error")]
    Unknown,
}
