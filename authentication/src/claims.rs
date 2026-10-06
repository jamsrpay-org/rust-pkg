use jamsrpay_authorization::{AuthorizationContext, StoreMemberRole};
use jamsrpay_types::{store_id::StoreId, user_id::UserId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    #[strum(serialize = "staff")]
    #[serde(rename = "staff")]
    Staff,
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
    /// Store memberships available to this merchant in this session.
    #[serde(default)]
    pub stores: HashMap<StoreId, StoreMemberRole>,
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

    /// Attempts to convert these claims into an [`AuthorizationContext`].
    ///
    /// Parses `sub` as a [`UserId`] (MerchantId). Returns a [`uuid::Error`]
    /// if `sub` is not a valid UUID string.
    pub fn to_authorization_context(&self) -> Result<AuthorizationContext, uuid::Error> {
        let merchant_id = UserId::parse(&self.sub)?;
        Ok(AuthorizationContext::new(merchant_id, self.stores.clone()))
    }
}

impl TryFrom<&Claims> for AuthorizationContext {
    type Error = uuid::Error;

    #[inline]
    fn try_from(claims: &Claims) -> Result<Self, Self::Error> {
        claims.to_authorization_context()
    }
}

impl TryFrom<Claims> for AuthorizationContext {
    type Error = uuid::Error;

    #[inline]
    fn try_from(claims: Claims) -> Result<Self, Self::Error> {
        let merchant_id = UserId::parse(&claims.sub)?;
        Ok(AuthorizationContext::new(merchant_id, claims.stores))
    }
}

