use base64::DecodeError;
use redis_rs::RedisClientError;
use std::string::FromUtf8Error;

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("Invalid base64 encoding")]
    InvalidBase64Encoding(#[from] DecodeError),
    #[error("Invalid UTF-8 in token")]
    InvalidUtf8(#[from] FromUtf8Error),
    #[error("Invalid token signature")]
    InvalidTokenSignature,
    #[error("Invalid token format")]
    InvalidTokenFormat,
    #[error("Token expired")]
    TokenExpired,
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Redis error: {0}")]
    Redis(#[from] RedisClientError),
}
