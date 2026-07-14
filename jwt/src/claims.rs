use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsRefStr, Display, EnumString)]
pub enum Issuer {
    #[strum(serialize = "auth-service")]
    #[serde(rename = "auth-service")]
    AuthService,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsRefStr, Display, EnumString)]
pub enum Scope {
    #[strum(serialize = "access_token")]
    #[serde(rename = "access_token")]
    AccessToken,
    #[strum(serialize = "refresh_token")]
    #[serde(rename = "refresh_token")]
    RefreshToken,
    #[strum(serialize = "root_auth_token")]
    #[serde(rename = "root_auth_token")]
    RootAuthToken,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsRefStr, Display, EnumString)]
pub enum Role {
    #[strum(serialize = "merchant")]
    #[serde(rename = "merchant")]
    Merchant,
    #[strum(serialize = "admin")]
    #[serde(rename = "admin")]
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsRefStr, Display, EnumString)]
pub enum Audience {
    #[strum(serialize = "api-gateway")]
    #[serde(rename = "api-gateway")]
    ApiGateway,
}

/// Standard + application-specific JWT claims for the JamsrPay platform.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Claims {
    /// Issuer — the service that created the token (e.g. "auth-service").
    pub iss: Issuer,
    /// Subject — the user or entity UUID.
    pub sub: String,
    /// Audience — intended recipient (e.g. "api").
    pub aud: Audience,
    /// Token scope — "access_token", "refresh_token", etc.
    pub scope: Scope,
    /// Role — "merchant", "admin", etc.
    pub role: Role,
    /// Session ID — tracks the login session.
    pub session_id: String,
    /// Issued-at timestamp (seconds since epoch).
    pub iat: usize,
    /// Expiration timestamp (seconds since epoch).
    pub exp: usize,
    /// JWT ID — unique identifier for this token.
    pub jti: String,
}
