use crate::{Claims, error::JwtError};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use std::collections::HashSet;

/// JWT token decoder. Requires only the RSA public key.
///
/// The decoding key and validation rules are parsed once at
/// construction time so that repeated calls to [`decode`](Self::decode)
/// are fast and allocation-free.
#[derive(Debug, Clone)]
pub struct JwtDecoder {
    decoding_key: DecodingKey,
    validation: Validation,
}

impl JwtDecoder {
    /// Create a new decoder.
    ///
    /// # Arguments
    /// - `public_key_pem` — RSA public key in PEM format.
    /// - `issuer` — expected `iss` claim value.
    /// - `audience` — expected `aud` claim value.
    ///
    /// # Errors
    /// Returns [`JwtError`] if the PEM key cannot be parsed.
    pub fn new(public_key_pem: &str, issuer: &str, audience: &str) -> Result<Self, JwtError> {
        let decoding_key = DecodingKey::from_rsa_pem(public_key_pem.as_bytes())?;

        let mut validation = Validation::new(Algorithm::RS256);

        // Require the standard spec claims
        let mut required = HashSet::new();
        required.insert("sub".to_string());
        required.insert("exp".to_string());
        required.insert("iat".to_string());
        required.insert("iss".to_string());
        required.insert("aud".to_string());
        validation.required_spec_claims = required;

        validation.set_issuer(&[issuer]);
        validation.set_audience(&[audience]);
        validation.leeway = 0;

        Ok(Self {
            decoding_key,
            validation,
        })
    }

    /// Decode and validate a JWT token.
    ///
    /// Validates the signature, expiration, issuer, and audience.
    /// Does **not** check the `scope` — use [`decode_with_scope`](Self::decode_with_scope)
    /// if you need scope enforcement.
    pub fn decode(&self, token: &str) -> Result<Claims, JwtError> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &self.validation)?;
        Ok(token_data.claims)
    }

    /// Decode and validate a JWT token, also verifying the scope.
    ///
    /// Returns [`JwtError::ScopeMismatch`] if the token's `scope` does not
    /// match `expected_scope`.
    pub fn decode_with_scope(&self, token: &str, expected_scope: &str) -> Result<Claims, JwtError> {
        let claims = self.decode(token)?;
        if claims.scope != expected_scope {
            return Err(JwtError::ScopeMismatch {
                expected: expected_scope.to_string(),
                actual: claims.scope,
            });
        }
        Ok(claims)
    }
}
