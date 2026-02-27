use serde::{Deserialize, Serialize};

/// Standard + application-specific JWT claims for the JamsrPay platform.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Claims {
    /// Issuer — the service that created the token (e.g. "auth-service").
    pub iss: String,
    /// Subject — the user or entity UUID.
    pub sub: String,
    /// Audience — intended recipient (e.g. "api").
    pub aud: String,
    /// Token scope — "access_token", "refresh_token", etc.
    pub scope: String,
    /// Role — "merchant_admin", "admin", etc.
    pub role: String,
    /// Tenant ID — merchant UUID for multi-tenancy (absent for platform admins).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    /// Session ID — tracks the login session.
    pub session_id: String,
    /// Issued-at timestamp (seconds since epoch).
    pub iat: usize,
    /// Expiration timestamp (seconds since epoch).
    pub exp: usize,
    /// JWT ID — unique identifier for this token.
    pub jti: String,
}

/// Well-known scope constants.
pub mod scope {
    pub const ACCESS_TOKEN: &str = "access_token";
    pub const REFRESH_TOKEN: &str = "refresh_token";
}
