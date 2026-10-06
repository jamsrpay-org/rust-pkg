use std::str::FromStr;

use jamsrpay_authorization::{ParsePermissionError, Permission};

#[test]
fn test_permission_canonical_strings() {
    let expected = [
        (Permission::StoreRead, "store.read"),
        (Permission::StoreUpdate, "store.update"),
        (Permission::StoreMemberRead, "store.member.read"),
        (Permission::StoreMemberInvite, "store.member.invite"),
        (Permission::StoreMemberUpdate, "store.member.update"),
        (Permission::StoreMemberRemove, "store.member.remove"),
        (Permission::StoreMemberSuspend, "store.member.suspend"),
        (Permission::StoreMemberActivate, "store.member.activate"),
        (Permission::InvoiceRead, "invoice.read"),
        (Permission::InvoiceCreate, "invoice.create"),
        (Permission::InvoiceUpdate, "invoice.update"),
        (Permission::InvoiceCancel, "invoice.cancel"),
        (Permission::PayoutRead, "payout.read"),
        (Permission::PayoutCreate, "payout.create"),
        (Permission::PayoutCancel, "payout.cancel"),
        (Permission::WebhookRead, "webhook.read"),
        (Permission::WebhookCreate, "webhook.create"),
        (Permission::WebhookUpdate, "webhook.update"),
        (Permission::WebhookDelete, "webhook.delete"),
    ];

    assert_eq!(expected.len(), 19);

    for (perm, canonical) in expected {
        assert_eq!(perm.as_str(), canonical);
        assert_eq!(perm.to_string(), canonical);
        assert_eq!(Permission::parse(canonical).unwrap(), perm);
        assert_eq!(Permission::from_str(canonical).unwrap(), perm);
        assert_eq!(Permission::try_from(canonical).unwrap(), perm);
        assert_eq!(Permission::try_from(canonical.to_string()).unwrap(), perm);
    }
}

#[test]
fn test_permission_parse_case_and_whitespace() {
    assert_eq!(
        Permission::parse(" STORE.READ ").unwrap(),
        Permission::StoreRead
    );
    assert_eq!(
        Permission::parse("  invoice.create\n").unwrap(),
        Permission::InvoiceCreate
    );
    assert_eq!(
        Permission::parse("\tPAYOUT.CANCEL\t").unwrap(),
        Permission::PayoutCancel
    );
    assert_eq!(
        Permission::parse("Webhook.Delete").unwrap(),
        Permission::WebhookDelete
    );
}

#[test]
fn test_permission_parse_invalid() {
    let invalid_inputs = [
        "",
        " ",
        "store",
        "store.",
        "store.delete",
        "store.create",
        "invoice.destroy",
        "payout.update",
        "webhook",
        "admin",
        "owner",
        "all",
        "store.read.extra",
    ];

    for input in invalid_inputs {
        let err = Permission::parse(input).unwrap_err();
        assert_eq!(err, ParsePermissionError(input.trim().to_ascii_lowercase()));
        assert_eq!(err.as_str(), input.trim().to_ascii_lowercase());
        assert_eq!(err.into_inner(), input.trim().to_ascii_lowercase());
    }
}

#[test]
fn test_permission_all_constant() {
    assert_eq!(Permission::ALL.len(), 19);
    for perm in Permission::ALL {
        assert_eq!(Permission::parse(perm.as_str()).unwrap(), perm);
    }
}
