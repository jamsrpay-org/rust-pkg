use thiserror::Error;

#[derive(Error, Debug)]
pub enum TronError {
    #[error("tron rpc error: {0}")]
    Rpc(String),
}
