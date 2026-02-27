use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("jwt error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("purpose mismatch: expected '{expected}', got '{actual}'")]
    PurposeMismatch { expected: String, actual: String },
}
