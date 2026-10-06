use std::collections::HashMap;

use jamsrpay_authorization::{
    AuthorizationContext, AuthorizedStoreContext, Permission, StoreId, StoreMemberRole, UserId,
};

#[test]
fn test_role_serialization_exact_lowercase_json() {
    assert_eq!(
        serde_json::to_string(&StoreMemberRole::Owner).unwrap(),
        "\"owner\""
    );
    assert_eq!(
        serde_json::to_string(&StoreMemberRole::Admin).unwrap(),
        "\"admin\""
    );
    assert_eq!(
        serde_json::to_string(&StoreMemberRole::Developer).unwrap(),
        "\"developer\""
    );
    assert_eq!(
        serde_json::to_string(&StoreMemberRole::Accountant).unwrap(),
        "\"accountant\""
    );
    assert_eq!(
        serde_json::to_string(&StoreMemberRole::Viewer).unwrap(),
        "\"viewer\""
    );

    // Deserialization
    assert_eq!(
        serde_json::from_str::<StoreMemberRole>("\"owner\"").unwrap(),
        StoreMemberRole::Owner
    );
    assert_eq!(
        serde_json::from_str::<StoreMemberRole>("\"admin\"").unwrap(),
        StoreMemberRole::Admin
    );
    assert_eq!(
        serde_json::from_str::<StoreMemberRole>("\"developer\"").unwrap(),
        StoreMemberRole::Developer
    );
    assert_eq!(
        serde_json::from_str::<StoreMemberRole>("\"accountant\"").unwrap(),
        StoreMemberRole::Accountant
    );
    assert_eq!(
        serde_json::from_str::<StoreMemberRole>("\"viewer\"").unwrap(),
        StoreMemberRole::Viewer
    );

    // Invalid JSON value errors
    assert!(serde_json::from_str::<StoreMemberRole>("\"Owner\"").is_err());
    assert!(serde_json::from_str::<StoreMemberRole>("\"superadmin\"").is_err());
    assert!(serde_json::from_str::<StoreMemberRole>("123").is_err());
}

#[test]
fn test_permission_serialization_exact_canonical_json() {
    let cases = [
        (Permission::StoreRead, "\"store.read\""),
        (Permission::StoreUpdate, "\"store.update\""),
        (Permission::StoreMemberRead, "\"store.member.read\""),
        (Permission::StoreMemberInvite, "\"store.member.invite\""),
        (Permission::StoreMemberUpdate, "\"store.member.update\""),
        (Permission::StoreMemberRemove, "\"store.member.remove\""),
        (Permission::StoreMemberSuspend, "\"store.member.suspend\""),
        (Permission::StoreMemberActivate, "\"store.member.activate\""),
        (Permission::InvoiceRead, "\"invoice.read\""),
        (Permission::InvoiceCreate, "\"invoice.create\""),
        (Permission::InvoiceUpdate, "\"invoice.update\""),
        (Permission::InvoiceCancel, "\"invoice.cancel\""),
        (Permission::PayoutRead, "\"payout.read\""),
        (Permission::PayoutCreate, "\"payout.create\""),
        (Permission::PayoutCancel, "\"payout.cancel\""),
        (Permission::WebhookRead, "\"webhook.read\""),
        (Permission::WebhookCreate, "\"webhook.create\""),
        (Permission::WebhookUpdate, "\"webhook.update\""),
        (Permission::WebhookDelete, "\"webhook.delete\""),
    ];

    for (perm, expected_json) in cases {
        let serialized = serde_json::to_string(&perm).unwrap();
        assert_eq!(serialized, expected_json);

        let deserialized: Permission = serde_json::from_str(expected_json).unwrap();
        assert_eq!(deserialized, perm);
    }

    assert!(serde_json::from_str::<Permission>("\"InvoiceRead\"").is_err());
    assert!(serde_json::from_str::<Permission>("\"unknown.perm\"").is_err());
}

#[test]
fn test_authorization_context_serde_roundtrip() {
    let merchant_id = UserId::generate();
    let store_1 = StoreId::generate();
    let store_2 = StoreId::generate();

    let mut stores = HashMap::new();
    stores.insert(store_1, StoreMemberRole::Owner);
    stores.insert(store_2, StoreMemberRole::Developer);

    let ctx = AuthorizationContext::new(merchant_id, stores);

    let json = serde_json::to_string(&ctx).unwrap();

    let deserialized: AuthorizationContext = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, ctx);

    // Authorization against deserialized context
    let authed = deserialized
        .authorize(&store_1, Permission::StoreUpdate)
        .unwrap();
    assert_eq!(authed.role, StoreMemberRole::Owner);
}

#[test]
fn test_authorized_store_context_serde_roundtrip() {
    let merchant_id = UserId::generate();
    let store_id = StoreId::generate();
    let role = StoreMemberRole::Developer;

    let authed = AuthorizedStoreContext::new(merchant_id, store_id, role);

    let json = serde_json::to_string(&authed).unwrap();
    let deserialized: AuthorizedStoreContext = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, authed);
}
