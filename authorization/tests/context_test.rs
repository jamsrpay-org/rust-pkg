use std::collections::HashMap;

use jamsrpay_authorization::{
    AuthorizationContext, AuthorizationError, AuthorizedStoreContext, Permission, StoreId,
    StoreMemberRole, UserId, authorize, role_for_store,
};

#[test]
fn test_authorization_context_success_and_denial() {
    let merchant_id = UserId::generate();
    let store_owner = StoreId::generate();
    let store_dev = StoreId::generate();
    let store_acct = StoreId::generate();
    let store_viewer = StoreId::generate();
    let unknown_store = StoreId::generate();

    let mut stores = HashMap::new();
    stores.insert(store_owner, StoreMemberRole::Owner);
    stores.insert(store_dev, StoreMemberRole::Developer);
    stores.insert(store_acct, StoreMemberRole::Accountant);
    stores.insert(store_viewer, StoreMemberRole::Viewer);

    let ctx = AuthorizationContext::new(merchant_id, stores);

    // ── role_for_store ───────────────────────────────────────────────────────
    assert_eq!(
        role_for_store(&ctx, &store_owner).unwrap(),
        StoreMemberRole::Owner
    );
    assert_eq!(
        role_for_store(&ctx, &store_dev).unwrap(),
        StoreMemberRole::Developer
    );
    assert_eq!(
        role_for_store(&ctx, &store_acct).unwrap(),
        StoreMemberRole::Accountant
    );
    assert_eq!(
        role_for_store(&ctx, &store_viewer).unwrap(),
        StoreMemberRole::Viewer
    );
    assert_eq!(
        role_for_store(&ctx, &unknown_store).unwrap_err(),
        AuthorizationError::StoreAccessDenied
    );

    // Method on context
    assert_eq!(
        ctx.role_for_store(&store_owner).unwrap(),
        StoreMemberRole::Owner
    );
    assert_eq!(
        ctx.role_for_store(&unknown_store).unwrap_err(),
        AuthorizationError::StoreAccessDenied
    );

    // ── Owner: has all permissions ───────────────────────────────────────────
    let authed = authorize(&ctx, &store_owner, Permission::StoreUpdate).unwrap();
    assert_eq!(authed.merchant_id, merchant_id);
    assert_eq!(authed.store_id, store_owner);
    assert_eq!(authed.role, StoreMemberRole::Owner);

    let authed_member = authorize(&ctx, &store_owner, Permission::StoreMemberInvite).unwrap();
    assert_eq!(authed_member.role, StoreMemberRole::Owner);

    // ── Developer: allowed technical/financial ops, denied members/store update ─
    let authed_dev = authorize(&ctx, &store_dev, Permission::InvoiceRead).unwrap();
    assert_eq!(authed_dev.role, StoreMemberRole::Developer);
    assert_eq!(authed_dev.merchant_id(), merchant_id);
    assert_eq!(authed_dev.store_id(), store_dev);
    assert_eq!(authed_dev.role(), StoreMemberRole::Developer);

    assert!(authorize(&ctx, &store_dev, Permission::InvoiceCreate).is_ok());
    assert!(authorize(&ctx, &store_dev, Permission::PayoutCreate).is_ok());
    assert!(authorize(&ctx, &store_dev, Permission::WebhookDelete).is_ok());

    assert_eq!(
        authorize(&ctx, &store_dev, Permission::StoreUpdate).unwrap_err(),
        AuthorizationError::PermissionDenied
    );
    assert_eq!(
        authorize(&ctx, &store_dev, Permission::StoreMemberRemove).unwrap_err(),
        AuthorizationError::PermissionDenied
    );

    // ── Accountant: allowed read ops, denied mutations and webhooks ──────────
    assert!(authorize(&ctx, &store_acct, Permission::InvoiceRead).is_ok());
    assert!(authorize(&ctx, &store_acct, Permission::PayoutRead).is_ok());
    assert!(authorize(&ctx, &store_acct, Permission::StoreRead).is_ok());

    assert_eq!(
        authorize(&ctx, &store_acct, Permission::InvoiceCreate).unwrap_err(),
        AuthorizationError::PermissionDenied
    );
    assert_eq!(
        authorize(&ctx, &store_acct, Permission::PayoutCreate).unwrap_err(),
        AuthorizationError::PermissionDenied
    );
    assert_eq!(
        authorize(&ctx, &store_acct, Permission::WebhookRead).unwrap_err(),
        AuthorizationError::PermissionDenied
    );

    // ── Viewer: allowed reads (including webhook.read), denied mutations ─────
    assert!(authorize(&ctx, &store_viewer, Permission::StoreRead).is_ok());
    assert!(authorize(&ctx, &store_viewer, Permission::InvoiceRead).is_ok());
    assert!(authorize(&ctx, &store_viewer, Permission::PayoutRead).is_ok());
    assert!(authorize(&ctx, &store_viewer, Permission::WebhookRead).is_ok());

    assert_eq!(
        authorize(&ctx, &store_viewer, Permission::InvoiceCreate).unwrap_err(),
        AuthorizationError::PermissionDenied
    );
    assert_eq!(
        authorize(&ctx, &store_viewer, Permission::WebhookCreate).unwrap_err(),
        AuthorizationError::PermissionDenied
    );
    assert_eq!(
        authorize(&ctx, &store_viewer, Permission::StoreMemberRead).unwrap_err(),
        AuthorizationError::PermissionDenied
    );

    // ── Unknown Store: StoreAccessDenied ─────────────────────────────────────
    assert_eq!(
        authorize(&ctx, &unknown_store, Permission::StoreRead).unwrap_err(),
        AuthorizationError::StoreAccessDenied
    );
    assert_eq!(
        authorize(&ctx, &unknown_store, Permission::InvoiceCreate).unwrap_err(),
        AuthorizationError::StoreAccessDenied
    );

    // ── Method on context ────────────────────────────────────────────────────
    assert!(ctx.authorize(&store_owner, Permission::StoreRead).is_ok());
    assert_eq!(
        ctx.authorize(&store_viewer, Permission::InvoiceCreate)
            .unwrap_err(),
        AuthorizationError::PermissionDenied
    );
    assert_eq!(
        ctx.authorize(&unknown_store, Permission::StoreRead)
            .unwrap_err(),
        AuthorizationError::StoreAccessDenied
    );
}

#[test]
fn test_authorized_store_context_constructors_and_getters() {
    let merchant_id = UserId::generate();
    let store_id = StoreId::generate();
    let role = StoreMemberRole::Admin;

    let ctx = AuthorizedStoreContext::new(merchant_id, store_id, role);

    assert_eq!(ctx.merchant_id(), merchant_id);
    assert_eq!(ctx.store_id(), store_id);
    assert_eq!(ctx.role(), role);

    assert_eq!(ctx.merchant_id, merchant_id);
    assert_eq!(ctx.store_id, store_id);
    assert_eq!(ctx.role, role);

    // Copy and Eq
    let copy = ctx;
    assert_eq!(ctx, copy);
}
