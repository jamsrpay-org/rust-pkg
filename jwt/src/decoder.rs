use crate::{NoExtra, TokenClaims, error::JwtError};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::de::DeserializeOwned;
use std::collections::HashSet;

/// Service for verifying JWT tokens — only needs public key.
pub struct JwtDecoder {
    public_key: String,
}

impl JwtDecoder {
    pub fn new(public_key: String) -> Self {
        Self { public_key }
    }

    /// Decode a token with extra custom claims.
    pub fn decode_token_with_extra<T>(
        &self,
        token: &str,
        purpose: &str,
        issuer: Option<&str>,
        audience: Option<&[&str]>,
    ) -> Result<TokenClaims<T>, JwtError>
    where
        T: DeserializeOwned,
    {
        let decoding_key = DecodingKey::from_rsa_pem(self.public_key.as_bytes())?;

        let mut validation = Validation::new(Algorithm::RS256);

        // Require standard spec claims
        let mut required_spec_claims = HashSet::new();
        required_spec_claims.insert("sub".to_string());
        required_spec_claims.insert("exp".to_string());
        required_spec_claims.insert("iat".to_string());
        validation.required_spec_claims = required_spec_claims;

        // Set issuer validation if provided
        if let Some(issuer) = issuer {
            validation.set_issuer(&[issuer]);
        }

        // Set audience validation if provided
        if let Some(audience) = audience {
            validation.set_audience(audience);
        }

        validation.leeway = 0;

        let token_data = decode::<TokenClaims<T>>(token, &decoding_key, &validation)?;

        // Validate purpose matches
        if token_data.claims.purpose != purpose {
            return Err(JwtError::PurposeMismatch {
                expected: purpose.to_string(),
                actual: token_data.claims.purpose.clone(),
            });
        }

        Ok(token_data.claims)
    }

    /// Decode a token without extra claims.
    pub fn decode_token(
        &self,
        token: &str,
        purpose: &str,
        issuer: Option<&str>,
        audience: Option<&[&str]>,
    ) -> Result<TokenClaims<NoExtra>, JwtError> {
        self.decode_token_with_extra(token, purpose, issuer, audience)
    }
}
