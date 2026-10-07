use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::ParsePermissionError;

/// Shared permission vocabulary for all merchant-facing services in JamsrPay.
///
/// Services do not create their own service-specific permission enums; instead,
/// they reference these centrally-defined permissions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    // ── Store ───────────────────────────────────────────────────────────────────
    #[serde(rename = "store.read")]
    StoreRead,
    #[serde(rename = "store.update")]
    StoreUpdate,
    #[serde(rename = "store.delete")]
    StoreDelete,

    // ── Store Members ───────────────────────────────────────────────────────────
    #[serde(rename = "store.member.read")]
    StoreMemberRead,
    #[serde(rename = "store.member.invite")]
    StoreMemberInvite,
    #[serde(rename = "store.member.update")]
    StoreMemberUpdate,
    #[serde(rename = "store.member.remove")]
    StoreMemberRemove,
    #[serde(rename = "store.member.suspend")]
    StoreMemberSuspend,
    #[serde(rename = "store.member.activate")]
    StoreMemberActivate,

    // ── Invoices ────────────────────────────────────────────────────────────────
    #[serde(rename = "invoice.read")]
    InvoiceRead,
    #[serde(rename = "invoice.create")]
    InvoiceCreate,
    #[serde(rename = "invoice.update")]
    InvoiceUpdate,
    #[serde(rename = "invoice.cancel")]
    InvoiceCancel,

    // ── Payouts ─────────────────────────────────────────────────────────────────
    #[serde(rename = "payout.read")]
    PayoutRead,
    #[serde(rename = "payout.create")]
    PayoutCreate,
    #[serde(rename = "payout.cancel")]
    PayoutCancel,

    // ── Webhooks ────────────────────────────────────────────────────────────────
    #[serde(rename = "webhook.read")]
    WebhookRead,
    #[serde(rename = "webhook.create")]
    WebhookCreate,
    #[serde(rename = "webhook.update")]
    WebhookUpdate,
    #[serde(rename = "webhook.delete")]
    WebhookDelete,

    // ── API Keys ────────────────────────────────────────────────────────────────
    #[serde(rename = "api_key.read")]
    ApiKeyRead,
    #[serde(rename = "api_key.create")]
    ApiKeyCreate,
    #[serde(rename = "api_key.update")]
    ApiKeyUpdate,
    #[serde(rename = "api_key.revoke")]
    ApiKeyRevoke,
    #[serde(rename = "api_key.rotate")]
    ApiKeyRotate,
}

impl Permission {
    pub const STORE_READ: &'static str = "store.read";
    pub const STORE_UPDATE: &'static str = "store.update";
    pub const STORE_DELETE: &'static str = "store.delete";

    pub const STORE_MEMBER_READ: &'static str = "store.member.read";
    pub const STORE_MEMBER_INVITE: &'static str = "store.member.invite";
    pub const STORE_MEMBER_UPDATE: &'static str = "store.member.update";
    pub const STORE_MEMBER_REMOVE: &'static str = "store.member.remove";
    pub const STORE_MEMBER_SUSPEND: &'static str = "store.member.suspend";
    pub const STORE_MEMBER_ACTIVATE: &'static str = "store.member.activate";

    pub const INVOICE_READ: &'static str = "invoice.read";
    pub const INVOICE_CREATE: &'static str = "invoice.create";
    pub const INVOICE_UPDATE: &'static str = "invoice.update";
    pub const INVOICE_CANCEL: &'static str = "invoice.cancel";

    pub const PAYOUT_READ: &'static str = "payout.read";
    pub const PAYOUT_CREATE: &'static str = "payout.create";
    pub const PAYOUT_CANCEL: &'static str = "payout.cancel";

    pub const WEBHOOK_READ: &'static str = "webhook.read";
    pub const WEBHOOK_CREATE: &'static str = "webhook.create";
    pub const WEBHOOK_UPDATE: &'static str = "webhook.update";
    pub const WEBHOOK_DELETE: &'static str = "webhook.delete";

    pub const API_KEY_READ: &'static str = "api_key.read";
    pub const API_KEY_CREATE: &'static str = "api_key.create";
    pub const API_KEY_UPDATE: &'static str = "api_key.update";
    pub const API_KEY_REVOKE: &'static str = "api_key.revoke";
    pub const API_KEY_ROTATE: &'static str = "api_key.rotate";

    /// Complete list of all 25 defined permissions.
    pub const ALL: [Self; 25] = [
        Self::StoreRead,
        Self::StoreUpdate,
        Self::StoreDelete,
        Self::StoreMemberRead,
        Self::StoreMemberInvite,
        Self::StoreMemberUpdate,
        Self::StoreMemberRemove,
        Self::StoreMemberSuspend,
        Self::StoreMemberActivate,
        Self::InvoiceRead,
        Self::InvoiceCreate,
        Self::InvoiceUpdate,
        Self::InvoiceCancel,
        Self::PayoutRead,
        Self::PayoutCreate,
        Self::PayoutCancel,
        Self::WebhookRead,
        Self::WebhookCreate,
        Self::WebhookUpdate,
        Self::WebhookDelete,
        Self::ApiKeyRead,
        Self::ApiKeyCreate,
        Self::ApiKeyUpdate,
        Self::ApiKeyRevoke,
        Self::ApiKeyRotate,
    ];

    /// Returns the canonical resource/action string representation of the permission.
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::StoreRead => Self::STORE_READ,
            Self::StoreUpdate => Self::STORE_UPDATE,
            Self::StoreDelete => Self::STORE_DELETE,
            Self::StoreMemberRead => Self::STORE_MEMBER_READ,
            Self::StoreMemberInvite => Self::STORE_MEMBER_INVITE,
            Self::StoreMemberUpdate => Self::STORE_MEMBER_UPDATE,
            Self::StoreMemberRemove => Self::STORE_MEMBER_REMOVE,
            Self::StoreMemberSuspend => Self::STORE_MEMBER_SUSPEND,
            Self::StoreMemberActivate => Self::STORE_MEMBER_ACTIVATE,
            Self::InvoiceRead => Self::INVOICE_READ,
            Self::InvoiceCreate => Self::INVOICE_CREATE,
            Self::InvoiceUpdate => Self::INVOICE_UPDATE,
            Self::InvoiceCancel => Self::INVOICE_CANCEL,
            Self::PayoutRead => Self::PAYOUT_READ,
            Self::PayoutCreate => Self::PAYOUT_CREATE,
            Self::PayoutCancel => Self::PAYOUT_CANCEL,
            Self::WebhookRead => Self::WEBHOOK_READ,
            Self::WebhookCreate => Self::WEBHOOK_CREATE,
            Self::WebhookUpdate => Self::WEBHOOK_UPDATE,
            Self::WebhookDelete => Self::WEBHOOK_DELETE,
            Self::ApiKeyRead => Self::API_KEY_READ,
            Self::ApiKeyCreate => Self::API_KEY_CREATE,
            Self::ApiKeyUpdate => Self::API_KEY_UPDATE,
            Self::ApiKeyRevoke => Self::API_KEY_REVOKE,
            Self::ApiKeyRotate => Self::API_KEY_ROTATE,
        }
    }

    /// Parses a permission from a string slice (case-insensitive, trims leading/trailing whitespace).
    ///
    /// Returns [`ParsePermissionError`] if the string does not match any known permission.
    pub fn parse(s: &str) -> Result<Self, ParsePermissionError> {
        match s.trim().to_ascii_lowercase().as_str() {
            Self::STORE_READ => Ok(Self::StoreRead),
            Self::STORE_UPDATE => Ok(Self::StoreUpdate),
            Self::STORE_DELETE => Ok(Self::StoreDelete),
            Self::STORE_MEMBER_READ => Ok(Self::StoreMemberRead),
            Self::STORE_MEMBER_INVITE => Ok(Self::StoreMemberInvite),
            Self::STORE_MEMBER_UPDATE => Ok(Self::StoreMemberUpdate),
            Self::STORE_MEMBER_REMOVE => Ok(Self::StoreMemberRemove),
            Self::STORE_MEMBER_SUSPEND => Ok(Self::StoreMemberSuspend),
            Self::STORE_MEMBER_ACTIVATE => Ok(Self::StoreMemberActivate),
            Self::INVOICE_READ => Ok(Self::InvoiceRead),
            Self::INVOICE_CREATE => Ok(Self::InvoiceCreate),
            Self::INVOICE_UPDATE => Ok(Self::InvoiceUpdate),
            Self::INVOICE_CANCEL => Ok(Self::InvoiceCancel),
            Self::PAYOUT_READ => Ok(Self::PayoutRead),
            Self::PAYOUT_CREATE => Ok(Self::PayoutCreate),
            Self::PAYOUT_CANCEL => Ok(Self::PayoutCancel),
            Self::WEBHOOK_READ => Ok(Self::WebhookRead),
            Self::WEBHOOK_CREATE => Ok(Self::WebhookCreate),
            Self::WEBHOOK_UPDATE => Ok(Self::WebhookUpdate),
            Self::WEBHOOK_DELETE => Ok(Self::WebhookDelete),
            Self::API_KEY_READ => Ok(Self::ApiKeyRead),
            Self::API_KEY_CREATE => Ok(Self::ApiKeyCreate),
            Self::API_KEY_UPDATE => Ok(Self::ApiKeyUpdate),
            Self::API_KEY_REVOKE => Ok(Self::ApiKeyRevoke),
            Self::API_KEY_ROTATE => Ok(Self::ApiKeyRotate),
            other => Err(ParsePermissionError(other.to_string())),
        }
    }
}

impl FromStr for Permission {
    type Err = ParsePermissionError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl TryFrom<&str> for Permission {
    type Error = ParsePermissionError;

    #[inline]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::parse(s)
    }
}

impl TryFrom<String> for Permission {
    type Error = ParsePermissionError;

    #[inline]
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::parse(&s)
    }
}

impl AsRef<str> for Permission {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
