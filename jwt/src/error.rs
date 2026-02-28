use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("{0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("scope mismatch: expected '{expected}', got '{actual}'")]
    ScopeMismatch { expected: String, actual: String },
}
