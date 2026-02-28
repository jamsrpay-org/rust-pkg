use crate::{Claims, error::JwtError};
use chrono::{TimeDelta, Utc};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use uuid::Uuid;

/// Parameters for creating a new JWT token.
///
/// The caller provides the subject-specific fields; the encoder
/// fills in `iss`, `aud`, `iat`, `exp`, and `jti` automatically.
pub struct TokenParams {
    /// Subject — the user or entity UUID.
    pub sub: String,
    /// Token scope — use constants from [`scope`](crate::scope).
    pub scope: String,
    /// Role — "merchant_admin", "admin", etc.
    pub role: String,
    /// Tenant ID — merchant UUID (`None` for platform admins).
    pub tenant_id: Option<String>,
    /// Session ID — login session UUID.
    pub session_id: String,
    /// Custom expiration override. Falls back to the encoder's default if `None`.
    pub expires_in: Option<TimeDelta>,
}

/// JWT token encoder. Requires the RSA private key.
///
/// The encoding key is parsed once at construction time so that
/// repeated calls to [`encode`](Self::encode) are fast.
pub struct JwtEncoder {
    encoding_key: EncodingKey,
    issuer: String,
    audience: String,
    default_expiration: TimeDelta,
}

impl JwtEncoder {
    /// Create a new encoder.
    ///
    /// # Arguments
    /// - `private_key_pem` — RSA private key in PEM format.
    /// - `issuer` — value written to the `iss` claim.
    /// - `audience` — value written to the `aud` claim.
    /// - `default_expiration` — used when [`TokenParams::expires_in`] is `None`.
    ///
    /// # Errors
    /// Returns [`JwtError`] if the PEM key cannot be parsed.
    pub fn new(
        private_key_pem: &str,
        issuer: String,
        audience: String,
        default_expiration: TimeDelta,
    ) -> Result<Self, JwtError> {
        let encoding_key = EncodingKey::from_rsa_pem(private_key_pem.as_bytes())?;
        Ok(Self {
            encoding_key,
            issuer,
            audience,
            default_expiration,
        })
    }

    /// Encode a JWT token from the given parameters.
    ///
    /// Automatically sets `iss`, `aud`, `iat`, `exp`, and generates a v4 UUID for `jti`.
    pub fn encode(&self, params: TokenParams) -> Result<String, JwtError> {
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let expires_in = params.expires_in.unwrap_or(self.default_expiration);
        let exp = (now + expires_in).timestamp() as usize;

        let claims = Claims {
            iss: self.issuer.clone(),
            sub: params.sub,
            aud: self.audience.clone(),
            scope: params.scope,
            role: params.role,
            tenant_id: params.tenant_id,
            session_id: params.session_id,
            iat,
            exp,
            jti: Uuid::new_v4().to_string(),
        };

        let header = Header::new(Algorithm::RS256);
        Ok(encode(&header, &claims, &self.encoding_key)?)
    }
}
