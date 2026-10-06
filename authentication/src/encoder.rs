use std::collections::HashMap;

use chrono::{TimeDelta, Utc};
use jamsrpay_authorization::StoreMemberRole;
use jamsrpay_types::{merchant_id::MerchantId, store_id::StoreId};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use uuid::Uuid;

use crate::{
    Claims,
    claims::{Audience, Issuer, Role, Scope},
    error::JwtError,
};

/// Parameters for creating a new JWT token.
///
/// The caller provides the subject-specific fields; the encoder
/// fills in `iss`, `aud`, `iat`, `exp`, and `jti` automatically.
#[derive(Debug, Clone, PartialEq)]
pub struct TokenParams {
    /// Subject — the merchant or staff user ID.
    pub sub: MerchantId,
    /// Token scope — use variants from [`Scope`].
    pub scope: Scope,
    /// Role — "merchant", "staff", etc.
    pub role: Role,
    /// Session ID — login session UUID.
    pub session_id: String,
    /// Store memberships available to this merchant in this session.
    pub stores: HashMap<StoreId, StoreMemberRole>,
    /// Custom expiration override. Falls back to the encoder's default if `None`.
    pub expires_in: Option<TimeDelta>,
}

impl TokenParams {
    /// Creates a new [`TokenParams`] with empty `stores` and default expiration.
    pub fn new(
        sub: impl Into<MerchantId>,
        scope: Scope,
        role: Role,
        session_id: impl Into<String>,
    ) -> Self {
        Self {
            sub: sub.into(),
            scope,
            role,
            session_id: session_id.into(),
            stores: HashMap::new(),
            expires_in: None,
        }
    }

    /// Sets the store memberships map.
    pub fn with_stores(mut self, stores: HashMap<StoreId, StoreMemberRole>) -> Self {
        self.stores = stores;
        self
    }

    /// Adds a single store membership to the map.
    pub fn with_store(mut self, store_id: StoreId, role: StoreMemberRole) -> Self {
        self.stores.insert(store_id, role);
        self
    }

    /// Sets a custom expiration duration.
    pub fn with_expires_in(mut self, expires_in: TimeDelta) -> Self {
        self.expires_in = Some(expires_in);
        self
    }
}

/// JWT token encoder. Requires the RSA private key.
///
/// The encoding key is parsed once at construction time so that
/// repeated calls to [`encode`](Self::encode) are fast.
pub struct JwtEncoder {
    encoding_key: EncodingKey,
    issuer: Issuer,
    audience: Audience,
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
    /// Returns [`JwtError::EncodingError`] if the PEM key cannot be parsed.
    pub fn new(
        private_key_pem: &str,
        issuer: Issuer,
        audience: Audience,
        default_expiration: TimeDelta,
    ) -> Result<Self, JwtError> {
        let encoding_key = EncodingKey::from_rsa_pem(private_key_pem.as_bytes())
            .map_err(|_| JwtError::EncodingError)?;
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
            session_id: params.session_id,
            iat,
            exp,
            jti: Uuid::new_v4().to_string(),
            stores: params.stores,
        };

        let header = Header::new(Algorithm::RS256);
        encode(&header, &claims, &self.encoding_key).map_err(|_| JwtError::EncodingError)
    }
}
