use thiserror::Error;

/// Dedicated authentication and JWT error type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
pub enum JwtError {
    #[error("invalid token")]
    InvalidToken,

    #[error("invalid signature")]
    InvalidSignature,

    #[error("token expired")]
    Expired,

    #[error("invalid issuer")]
    InvalidIssuer,

    #[error("invalid audience")]
    InvalidAudience,

    #[error("invalid scope")]
    InvalidScope,

    #[error("invalid claims")]
    InvalidClaims,

    #[error("encoding error")]
    EncodingError,

    #[error("decoding error")]
    DecodingError,
}

impl From<jsonwebtoken::errors::Error> for JwtError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        use jsonwebtoken::errors::ErrorKind;
        match err.kind() {
            ErrorKind::ExpiredSignature => JwtError::Expired,
            ErrorKind::InvalidSignature => JwtError::InvalidSignature,
            ErrorKind::InvalidIssuer => JwtError::InvalidIssuer,
            ErrorKind::InvalidAudience => JwtError::InvalidAudience,
            ErrorKind::InvalidToken => JwtError::InvalidToken,
            ErrorKind::Base64(_) => JwtError::InvalidToken,
            ErrorKind::Json(_) => JwtError::InvalidClaims,
            _ => JwtError::DecodingError,
        }
    }
}
