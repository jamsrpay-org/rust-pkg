use crate::{NoExtra, TokenClaims, error::JwtError};
use chrono::Utc;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::Serialize;

/// Service for creating JWT tokens — requires private key.
pub struct JwtEncoder {
    private_key: String,
}

impl JwtEncoder {
    pub fn new(private_key: String) -> Self {
        Self { private_key }
    }

    /// Create a token with extra custom claims.
    pub fn create_token_with_extra<T>(
        &self,
        subject: String,
        purpose: String,
        expiration: Option<chrono::TimeDelta>,
        extra: T,
    ) -> Result<String, JwtError>
    where
        T: Serialize,
    {
        let encoding_key = EncodingKey::from_rsa_pem(self.private_key.as_bytes())?;
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let expiration = expiration.unwrap_or(chrono::Duration::days(7));
        let exp = (now + expiration).timestamp() as usize;

        let claims = TokenClaims {
            sub: subject,
            purpose,
            exp,
            iat,
            extra,
        };

        let header = Header::new(Algorithm::RS256);
        Ok(encode(&header, &claims, &encoding_key)?)
    }

    /// Create a token without extra claims.
    pub fn create_token(
        &self,
        subject: String,
        purpose: String,
        expiration: Option<chrono::TimeDelta>,
    ) -> Result<String, JwtError> {
        self.create_token_with_extra(subject, purpose, expiration, NoExtra)
    }
}
