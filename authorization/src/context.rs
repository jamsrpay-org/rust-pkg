use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub use jamsrpay_types::store_id::StoreId;
pub use jamsrpay_types::user_id::UserId;

use crate::{AuthorizationError, Permission, StoreMemberRole, policy::allows};

/// Strongly-typed identifier for a merchant.
///
/// Aliased to [`UserId`] to remain fully compatible with JamsrPay's identity
/// and user models where merchants are authenticated users.
pub type MerchantId = UserId;

/// Trusted session context representing an authenticated merchant and their
/// accessible stores along with their respective roles.
///
/// # Security & Architecture
///
/// This context **MUST** contain data that has already been authenticated and
/// validated by upstream authentication layers (such as the JWT interceptor or
/// session middleware).
///
/// This crate does NOT parse tokens, verify cryptographic signatures, or query
/// databases. It operates only on already-trusted context data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorizationContext {
    pub merchant_id: MerchantId,
    pub stores: HashMap<StoreId, StoreMemberRole>,
}

impl AuthorizationContext {
    /// Creates a new [`AuthorizationContext`].
    #[inline]
    pub fn new(merchant_id: MerchantId, stores: HashMap<StoreId, StoreMemberRole>) -> Self {
        Self {
            merchant_id,
            stores,
        }
    }

    /// Determines the merchant's role for a specific store.
    #[inline]
    pub fn role_for_store(
        &self,
        store_id: &StoreId,
    ) -> Result<StoreMemberRole, AuthorizationError> {
        role_for_store(self, store_id)
    }

    /// Evaluates whether the merchant is authorized for `permission` on `store_id`.
    #[inline]
    pub fn authorize(
        &self,
        store_id: &StoreId,
        permission: Permission,
    ) -> Result<AuthorizedStoreContext, AuthorizationError> {
        authorize(self, store_id, permission)
    }
}

/// Represents a successfully authorized request on a specific store.
///
/// After authorization, downstream application use cases and domain services
/// can accept this typed context rather than repeatedly passing and validating
/// `merchant_id`, `store_id`, and `role` separately.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AuthorizedStoreContext {
    pub merchant_id: MerchantId,
    pub store_id: StoreId,
    pub role: StoreMemberRole,
}

impl AuthorizedStoreContext {
    /// Creates a new [`AuthorizedStoreContext`].
    #[inline]
    pub fn new(merchant_id: MerchantId, store_id: StoreId, role: StoreMemberRole) -> Self {
        Self {
            merchant_id,
            store_id,
            role,
        }
    }

    /// Returns the authorized merchant ID.
    #[inline]
    pub fn merchant_id(&self) -> MerchantId {
        self.merchant_id
    }

    /// Returns the authorized store ID.
    #[inline]
    pub fn store_id(&self) -> StoreId {
        self.store_id
    }

    /// Returns the member role within the authorized store.
    #[inline]
    pub fn role(&self) -> StoreMemberRole {
        self.role
    }
}

/// Determines whether the authenticated merchant has access to the given store.
///
/// Returns the associated [`StoreMemberRole`] if the store exists in the context's
/// membership map; otherwise returns [`AuthorizationError::StoreAccessDenied`].
///
/// Does not distinguish between "store does not exist", "merchant is not a member",
/// or "merchant was removed" at this layer.
#[inline]
pub fn role_for_store(
    context: &AuthorizationContext,
    store_id: &StoreId,
) -> Result<StoreMemberRole, AuthorizationError> {
    context
        .stores
        .get(store_id)
        .copied()
        .ok_or(AuthorizationError::StoreAccessDenied)
}

/// Synchronously and deterministically evaluates whether the trusted context is authorized
/// to execute `permission` on `store_id`.
///
/// # Execution Flow
///
/// 1. Look up `store_id` in `context.stores`:
///    - Missing → returns `Err(AuthorizationError::StoreAccessDenied)`
/// 2. Evaluate `role.allows(permission)`:
///    - False → returns `Err(AuthorizationError::PermissionDenied)`
/// 3. Returns `Ok(AuthorizedStoreContext { merchant_id, store_id, role })`.
#[inline]
pub fn authorize(
    context: &AuthorizationContext,
    store_id: &StoreId,
    permission: Permission,
) -> Result<AuthorizedStoreContext, AuthorizationError> {
    let role = role_for_store(context, store_id)?;

    if !allows(role, permission) {
        return Err(AuthorizationError::PermissionDenied);
    }

    Ok(AuthorizedStoreContext {
        merchant_id: context.merchant_id,
        store_id: *store_id,
        role,
    })
}
