use std::collections::HashMap;

use jamsrpay_authorization::{AuthorizationContext, StoreMemberRole};
use jamsrpay_types::{merchant_id::MerchantId, store_id::StoreId};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, AsRefStr, Display, EnumString)]
pub enum Issuer {
    #[strum(serialize = "auth-service")]
    #[serde(rename = "auth-service")]
    AuthService,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, AsRefStr, Display, EnumString)]
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
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, AsRefStr, Display, EnumString)]
pub enum Role {
    #[strum(serialize = "merchant")]
    #[serde(rename = "merchant")]
    Merchant,
    #[strum(serialize = "staff")]
    #[serde(rename = "staff")]
    Staff,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, AsRefStr, Display, EnumString)]
pub enum Audience {
    #[strum(serialize = "api-gateway")]
    #[serde(rename = "api-gateway")]
    ApiGateway,
    #[serde(other)]
    Unknown,
}

/// Standard + application-specific JWT claims for the JamsrPay platform.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Claims {
    /// Issuer — the service that created the token (e.g. "auth-service").
    pub iss: Issuer,
    /// Subject — strongly-typed merchant / user ID.
    pub sub: MerchantId,
    /// Audience — intended recipient (e.g. "api-gateway").
    pub aud: Audience,
    /// Token scope — "access_token", "refresh_token", etc.
    pub scope: Scope,
    /// Role — account-level role ("merchant", "staff").
    pub role: Role,
    /// Session ID — tracks the login session.
    pub session_id: String,
    /// Store memberships available to this merchant in this session.
    #[serde(default)]
    pub stores: HashMap<StoreId, StoreMemberRole>,
    /// Issued-at timestamp (seconds since epoch).
    pub iat: usize,
    /// Expiration timestamp (seconds since epoch).
    pub exp: usize,
    /// JWT ID — unique identifier for this token.
    pub jti: String,
}

impl Claims {
    /// Returns `true` if the subject's role is [`Role::Merchant`].
    #[inline]
    pub fn is_merchant(&self) -> bool {
        matches!(self.role, Role::Merchant)
    }

    /// Returns `true` if the subject's role is [`Role::Staff`].
    #[inline]
    pub fn is_staff(&self) -> bool {
        matches!(self.role, Role::Staff)
    }

    /// Returns the merchant's role for a specific store, if they are a member.
    #[inline]
    pub fn role_for_store(&self, store_id: &StoreId) -> Option<StoreMemberRole> {
        self.stores.get(store_id).copied()
    }

    /// Returns `true` if the merchant has an assigned role in the given store.
    #[inline]
    pub fn has_store_access(&self, store_id: &StoreId) -> bool {
        self.stores.contains_key(store_id)
    }

    /// Converts these claims into an [`AuthorizationContext`].
    #[inline]
    pub fn authorization_context(&self) -> AuthorizationContext {
        AuthorizationContext::new(self.sub, self.stores.clone())
    }

    /// Legacy / compatibility helper for converting claims to [`AuthorizationContext`].
    #[inline]
    pub fn to_authorization_context(&self) -> Result<AuthorizationContext, uuid::Error> {
        Ok(self.authorization_context())
    }
}

impl From<&Claims> for AuthorizationContext {
    #[inline]
    fn from(claims: &Claims) -> Self {
        claims.authorization_context()
    }
}

impl From<Claims> for AuthorizationContext {
    #[inline]
    fn from(claims: Claims) -> Self {
        AuthorizationContext::new(claims.sub, claims.stores)
    }
}
