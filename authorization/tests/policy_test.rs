use jamsrpay_authorization::{Permission, StoreMemberRole, allows};

/// Reference expected policy decision based on Specification Section 12:
///
/// | Permission             | Owner | Admin | Developer | Accountant | Viewer |
/// |------------------------|:-----:|:-----:|:---------:|:----------:|:------:|
/// | store.read             |   ✓   |   ✓   |     ✓     |     ✓      |   ✓    |
/// | store.update           |   ✓   |   ✓   |     ✗     |     ✗      |   ✗    |
/// | store.member.read      |   ✓   |   ✓   |     ✗     |     ✗      |   ✗    |
/// | store.member.invite    |   ✓   |   ✓   |     ✗     |     ✗      |   ✗    |
/// | store.member.update    |   ✓   |   ✓   |     ✗     |     ✗      |   ✗    |
/// | store.member.remove    |   ✓   |   ✓   |     ✗     |     ✗      |   ✗    |
/// | store.member.suspend   |   ✓   |   ✓   |     ✗     |     ✗      |   ✗    |
/// | store.member.activate  |   ✓   |   ✓   |     ✗     |     ✗      |   ✗    |
/// | invoice.read           |   ✓   |   ✓   |     ✓     |     ✓      |   ✓    |
/// | invoice.create         |   ✓   |   ✓   |     ✓     |     ✗      |   ✗    |
/// | invoice.update         |   ✓   |   ✓   |     ✓     |     ✗      |   ✗    |
/// | invoice.cancel         |   ✓   |   ✓   |     ✓     |     ✗      |   ✗    |
/// | payout.read            |   ✓   |   ✓   |     ✓     |     ✓      |   ✓    |
/// | payout.create          |   ✓   |   ✓   |     ✓     |     ✗      |   ✗    |
/// | payout.cancel          |   ✓   |   ✓   |     ✓     |     ✗      |   ✗    |
/// | webhook.read           |   ✓   |   ✓   |     ✓     |     ✗      |   ✓    |
/// | webhook.create         |   ✓   |   ✓   |     ✓     |     ✗      |   ✗    |
/// | webhook.update         |   ✓   |   ✓   |     ✓     |     ✗      |   ✗    |
/// | webhook.delete         |   ✓   |   ✓   |     ✓     |     ✗      |   ✗    |
fn expected_policy(role: StoreMemberRole, permission: Permission) -> bool {
    match (role, permission) {
        // Owner has all permissions
        (StoreMemberRole::Owner, _) => true,

        // Admin has all permissions except StoreDelete
        (StoreMemberRole::Admin, Permission::StoreDelete) => false,
        (StoreMemberRole::Admin, _) => true,

        // Developer
        (StoreMemberRole::Developer, Permission::StoreRead) => true,
        (StoreMemberRole::Developer, Permission::InvoiceRead) => true,
        (StoreMemberRole::Developer, Permission::InvoiceCreate) => true,
        (StoreMemberRole::Developer, Permission::InvoiceUpdate) => true,
        (StoreMemberRole::Developer, Permission::InvoiceCancel) => true,
        (StoreMemberRole::Developer, Permission::PayoutRead) => true,
        (StoreMemberRole::Developer, Permission::PayoutCreate) => true,
        (StoreMemberRole::Developer, Permission::PayoutCancel) => true,
        (StoreMemberRole::Developer, Permission::WebhookRead) => true,
        (StoreMemberRole::Developer, Permission::WebhookCreate) => true,
        (StoreMemberRole::Developer, Permission::WebhookUpdate) => true,
        (StoreMemberRole::Developer, Permission::WebhookDelete) => true,
        (StoreMemberRole::Developer, Permission::ApiKeyRead) => true,
        (StoreMemberRole::Developer, Permission::ApiKeyCreate) => true,
        (StoreMemberRole::Developer, Permission::ApiKeyUpdate) => true,
        (StoreMemberRole::Developer, Permission::ApiKeyRevoke) => true,
        (StoreMemberRole::Developer, Permission::ApiKeyRotate) => true,
        (StoreMemberRole::Developer, _) => false,

        // Accountant
        (StoreMemberRole::Accountant, Permission::StoreRead) => true,
        (StoreMemberRole::Accountant, Permission::InvoiceRead) => true,
        (StoreMemberRole::Accountant, Permission::PayoutRead) => true,
        (StoreMemberRole::Accountant, _) => false,

        // Viewer
        (StoreMemberRole::Viewer, Permission::StoreRead) => true,
        (StoreMemberRole::Viewer, Permission::InvoiceRead) => true,
        (StoreMemberRole::Viewer, Permission::PayoutRead) => true,
        (StoreMemberRole::Viewer, Permission::WebhookRead) => true,
        (StoreMemberRole::Viewer, _) => false,
    }
}

#[test]
fn test_complete_policy_matrix_125_cases() {
    let mut tested_count = 0;

    for role in StoreMemberRole::ALL {
        for permission in Permission::ALL {
            let expected = expected_policy(role, permission);
            let actual_fn = allows(role, permission);
            let actual_method = role.allows(permission);

            assert_eq!(
                actual_fn, expected,
                "Failed policy check for role {:?} and permission {:?}",
                role, permission
            );
            assert_eq!(
                actual_method, expected,
                "Failed role.allows() check for role {:?} and permission {:?}",
                role, permission
            );

            tested_count += 1;
        }
    }

    assert_eq!(tested_count, 125);
}

#[test]
fn test_owner_has_all_permissions() {
    for perm in Permission::ALL {
        assert!(allows(StoreMemberRole::Owner, perm));
        assert!(StoreMemberRole::Owner.allows(perm));
    }
}

#[test]
fn test_admin_permissions() {
    for perm in Permission::ALL {
        if perm == Permission::StoreDelete {
            assert!(!allows(StoreMemberRole::Admin, perm));
            assert!(!StoreMemberRole::Admin.allows(perm));
        } else {
            assert!(allows(StoreMemberRole::Admin, perm));
            assert!(StoreMemberRole::Admin.allows(perm));
        }
    }
}

#[test]
fn test_developer_permissions() {
    // Allowed
    assert!(allows(StoreMemberRole::Developer, Permission::StoreRead));
    assert!(allows(StoreMemberRole::Developer, Permission::InvoiceRead));
    assert!(allows(
        StoreMemberRole::Developer,
        Permission::InvoiceCreate
    ));
    assert!(allows(
        StoreMemberRole::Developer,
        Permission::InvoiceUpdate
    ));
    assert!(allows(
        StoreMemberRole::Developer,
        Permission::InvoiceCancel
    ));
    assert!(allows(StoreMemberRole::Developer, Permission::PayoutRead));
    assert!(allows(StoreMemberRole::Developer, Permission::PayoutCreate));
    assert!(allows(StoreMemberRole::Developer, Permission::PayoutCancel));
    assert!(allows(StoreMemberRole::Developer, Permission::WebhookRead));
    assert!(allows(
        StoreMemberRole::Developer,
        Permission::WebhookCreate
    ));
    assert!(allows(
        StoreMemberRole::Developer,
        Permission::WebhookUpdate
    ));
    assert!(allows(
        StoreMemberRole::Developer,
        Permission::WebhookDelete
    ));

    // Denied
    assert!(!allows(StoreMemberRole::Developer, Permission::StoreUpdate));
    assert!(!allows(
        StoreMemberRole::Developer,
        Permission::StoreMemberRead
    ));
    assert!(!allows(
        StoreMemberRole::Developer,
        Permission::StoreMemberInvite
    ));
    assert!(!allows(
        StoreMemberRole::Developer,
        Permission::StoreMemberUpdate
    ));
    assert!(!allows(
        StoreMemberRole::Developer,
        Permission::StoreMemberRemove
    ));
    assert!(!allows(
        StoreMemberRole::Developer,
        Permission::StoreMemberSuspend
    ));
    assert!(!allows(
        StoreMemberRole::Developer,
        Permission::StoreMemberActivate
    ));
}

#[test]
fn test_accountant_permissions() {
    // Allowed
    assert!(allows(StoreMemberRole::Accountant, Permission::StoreRead));
    assert!(allows(StoreMemberRole::Accountant, Permission::InvoiceRead));
    assert!(allows(StoreMemberRole::Accountant, Permission::PayoutRead));

    // Denied mutations
    assert!(!allows(
        StoreMemberRole::Accountant,
        Permission::InvoiceCreate
    ));
    assert!(!allows(
        StoreMemberRole::Accountant,
        Permission::InvoiceUpdate
    ));
    assert!(!allows(
        StoreMemberRole::Accountant,
        Permission::InvoiceCancel
    ));
    assert!(!allows(
        StoreMemberRole::Accountant,
        Permission::PayoutCreate
    ));
    assert!(!allows(
        StoreMemberRole::Accountant,
        Permission::PayoutCancel
    ));

    // Denied store & members
    assert!(!allows(
        StoreMemberRole::Accountant,
        Permission::StoreUpdate
    ));
    assert!(!allows(
        StoreMemberRole::Accountant,
        Permission::StoreMemberRead
    ));
    assert!(!allows(
        StoreMemberRole::Accountant,
        Permission::StoreMemberInvite
    ));
    assert!(!allows(
        StoreMemberRole::Accountant,
        Permission::StoreMemberUpdate
    ));
    assert!(!allows(
        StoreMemberRole::Accountant,
        Permission::StoreMemberRemove
    ));
    assert!(!allows(
        StoreMemberRole::Accountant,
        Permission::StoreMemberSuspend
    ));
    assert!(!allows(
        StoreMemberRole::Accountant,
        Permission::StoreMemberActivate
    ));

    // Denied webhooks (even webhook.read)
    assert!(!allows(
        StoreMemberRole::Accountant,
        Permission::WebhookRead
    ));
    assert!(!allows(
        StoreMemberRole::Accountant,
        Permission::WebhookCreate
    ));
    assert!(!allows(
        StoreMemberRole::Accountant,
        Permission::WebhookUpdate
    ));
    assert!(!allows(
        StoreMemberRole::Accountant,
        Permission::WebhookDelete
    ));
}

#[test]
fn test_viewer_permissions() {
    // Allowed
    assert!(allows(StoreMemberRole::Viewer, Permission::StoreRead));
    assert!(allows(StoreMemberRole::Viewer, Permission::InvoiceRead));
    assert!(allows(StoreMemberRole::Viewer, Permission::PayoutRead));
    assert!(allows(StoreMemberRole::Viewer, Permission::WebhookRead));

    // Denied mutations
    assert!(!allows(StoreMemberRole::Viewer, Permission::StoreUpdate));
    assert!(!allows(StoreMemberRole::Viewer, Permission::InvoiceCreate));
    assert!(!allows(StoreMemberRole::Viewer, Permission::InvoiceUpdate));
    assert!(!allows(StoreMemberRole::Viewer, Permission::InvoiceCancel));
    assert!(!allows(StoreMemberRole::Viewer, Permission::PayoutCreate));
    assert!(!allows(StoreMemberRole::Viewer, Permission::PayoutCancel));
    assert!(!allows(StoreMemberRole::Viewer, Permission::WebhookCreate));
    assert!(!allows(StoreMemberRole::Viewer, Permission::WebhookUpdate));
    assert!(!allows(StoreMemberRole::Viewer, Permission::WebhookDelete));

    // Denied members
    assert!(!allows(
        StoreMemberRole::Viewer,
        Permission::StoreMemberRead
    ));
    assert!(!allows(
        StoreMemberRole::Viewer,
        Permission::StoreMemberInvite
    ));
    assert!(!allows(
        StoreMemberRole::Viewer,
        Permission::StoreMemberUpdate
    ));
    assert!(!allows(
        StoreMemberRole::Viewer,
        Permission::StoreMemberRemove
    ));
    assert!(!allows(
        StoreMemberRole::Viewer,
        Permission::StoreMemberSuspend
    ));
    assert!(!allows(
        StoreMemberRole::Viewer,
        Permission::StoreMemberActivate
    ));
}

#[test]
fn test_accountant_vs_viewer_webhook_read() {
    // Spec highlight: Viewer has webhook.read, Accountant does NOT
    assert!(!allows(
        StoreMemberRole::Accountant,
        Permission::WebhookRead
    ));
    assert!(allows(StoreMemberRole::Viewer, Permission::WebhookRead));
}
