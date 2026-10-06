//! # JamsrPay Authorization Crate
//!
//! A shared, pure authorization policy library used by merchant-facing services:
//! - Store Service
//! - Billing Service
//! - Payout Service
//! - Webhook Service
//! - Future merchant-facing services
//!
//! ## Purpose
//!
//! Given a trusted merchant identity, a trusted store membership/role snapshot,
//! and a required permission, determine synchronously and deterministically whether
//! the operation is authorized.
//!
//! ## Non-Goals
//!
//! This crate contains NO database access, NO network calls, NO async operations,
//! and NO JWT validation or session loading. It operates strictly on already-trusted
//! context data provided by upstream authentication middleware.
//!
//! ## Example Usage
//!
//! ```rust
//! use std::collections::HashMap;
//! use jamsrpay_authorization::{
//!     authorize, allows, AuthorizationContext, AuthorizedStoreContext,
//!     AuthorizationError, Permission, StoreMemberRole, StoreId, UserId,
//! };
//!
//! let merchant_id = UserId::generate();
//! let store_id = StoreId::generate();
//!
//! let mut stores = HashMap::new();
//! stores.insert(store_id, StoreMemberRole::Developer);
//!
//! let auth_context = AuthorizationContext::new(merchant_id, stores);
//!
//! // Developer can read invoices:
//! let authorized = authorize(&auth_context, &store_id, Permission::InvoiceRead).unwrap();
//! assert_eq!(authorized.role, StoreMemberRole::Developer);
//!
//! // Developer cannot update store details:
//! let err = authorize(&auth_context, &store_id, Permission::StoreUpdate).unwrap_err();
//! assert_eq!(err, AuthorizationError::PermissionDenied);
//! ```

pub mod context;
pub mod error;
pub mod permission;
pub mod policy;
pub mod role;

pub use context::{
    AuthorizationContext, AuthorizedStoreContext, MerchantId, StoreId, UserId, authorize,
    role_for_store,
};
pub use error::{AuthorizationError, ParsePermissionError, ParseRoleError};
pub use permission::Permission;
pub use policy::allows;
pub use role::StoreMemberRole;
