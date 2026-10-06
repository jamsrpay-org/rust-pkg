use thiserror::Error;

/// Errors that can occur during authorization checks.
///
/// Designed to be intentionally generic to external callers, avoiding leaking
/// whether a store exists, whether the merchant was ever a member, or whether
/// the membership was suspended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
pub enum AuthorizationError {
    /// The authenticated context does not have access to the specified store.
    #[error("store access denied")]
    StoreAccessDenied,

    /// The merchant's role in the store does not have the required permission.
    #[error("permission denied")]
    PermissionDenied,
}

/// Error returned when failing to parse a [`StoreMemberRole`](crate::StoreMemberRole) from a string.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
#[error("invalid store member role: '{0}'")]
pub struct ParseRoleError(pub String);

impl ParseRoleError {
    /// Returns the invalid string that caused the parse error.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the error and returns the underlying invalid string.
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }
}

/// Error returned when failing to parse a [`Permission`](crate::Permission) from a string.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
#[error("invalid permission: '{0}'")]
pub struct ParsePermissionError(pub String);

impl ParsePermissionError {
    /// Returns the invalid string that caused the parse error.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the error and returns the underlying invalid string.
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }
}
