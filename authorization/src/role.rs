use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{ParseRoleError, Permission, policy::allows};

/// Represents the role of a merchant member within a store.
///
/// Five canonical roles are supported:
/// - [`StoreMemberRole::Owner`]: Full store access and owner-level authority.
/// - [`StoreMemberRole::Admin`]: Operational management across store members, settings, invoices, payouts, and webhooks.
/// - [`StoreMemberRole::Developer`]: Technical integration access (invoices, payouts, webhooks).
/// - [`StoreMemberRole::Accountant`]: Financial read-only access (store, invoices, payouts).
/// - [`StoreMemberRole::Viewer`]: Operational read-only visibility (store, invoices, payouts, webhooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StoreMemberRole {
    Owner,
    Admin,
    Developer,
    Accountant,
    Viewer,
}

impl StoreMemberRole {
    pub const OWNER: &'static str = "owner";
    pub const ADMIN: &'static str = "admin";
    pub const DEVELOPER: &'static str = "developer";
    pub const ACCOUNTANT: &'static str = "accountant";
    pub const VIEWER: &'static str = "viewer";

    /// All canonical store member roles.
    pub const ALL: [Self; 5] = [
        Self::Owner,
        Self::Admin,
        Self::Developer,
        Self::Accountant,
        Self::Viewer,
    ];

    /// Returns the canonical string representation of the role.
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Owner => Self::OWNER,
            Self::Admin => Self::ADMIN,
            Self::Developer => Self::DEVELOPER,
            Self::Accountant => Self::ACCOUNTANT,
            Self::Viewer => Self::VIEWER,
        }
    }

    /// Parses a role from a string slice (case-insensitive, trims leading/trailing whitespace).
    ///
    /// Unknown values return [`ParseRoleError`]. Never silently falls back to another role.
    pub fn parse(s: &str) -> Result<Self, ParseRoleError> {
        match s.trim().to_ascii_lowercase().as_str() {
            Self::OWNER => Ok(Self::Owner),
            Self::ADMIN => Ok(Self::Admin),
            Self::DEVELOPER => Ok(Self::Developer),
            Self::ACCOUNTANT => Ok(Self::Accountant),
            Self::VIEWER => Ok(Self::Viewer),
            other => Err(ParseRoleError(other.to_string())),
        }
    }

    /// Returns `true` if the role is [`StoreMemberRole::Owner`].
    #[inline]
    pub fn is_owner(&self) -> bool {
        matches!(self, Self::Owner)
    }

    /// Evaluates whether this role is granted the given [`Permission`].
    #[inline]
    pub fn allows(&self, permission: Permission) -> bool {
        allows(*self, permission)
    }
}

impl FromStr for StoreMemberRole {
    type Err = ParseRoleError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl TryFrom<&str> for StoreMemberRole {
    type Error = ParseRoleError;

    #[inline]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::parse(s)
    }
}

impl TryFrom<String> for StoreMemberRole {
    type Error = ParseRoleError;

    #[inline]
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::parse(&s)
    }
}

impl AsRef<str> for StoreMemberRole {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for StoreMemberRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
