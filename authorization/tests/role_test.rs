use std::str::FromStr;

use jamsrpay_authorization::{ParseRoleError, StoreMemberRole};

#[test]
fn test_role_canonical_strings() {
    assert_eq!(StoreMemberRole::Owner.as_str(), "owner");
    assert_eq!(StoreMemberRole::Admin.as_str(), "admin");
    assert_eq!(StoreMemberRole::Developer.as_str(), "developer");
    assert_eq!(StoreMemberRole::Accountant.as_str(), "accountant");
    assert_eq!(StoreMemberRole::Viewer.as_str(), "viewer");
}

#[test]
fn test_role_display() {
    assert_eq!(StoreMemberRole::Owner.to_string(), "owner");
    assert_eq!(StoreMemberRole::Admin.to_string(), "admin");
    assert_eq!(StoreMemberRole::Developer.to_string(), "developer");
    assert_eq!(StoreMemberRole::Accountant.to_string(), "accountant");
    assert_eq!(StoreMemberRole::Viewer.to_string(), "viewer");
}

#[test]
fn test_role_parse_valid() {
    assert_eq!(
        StoreMemberRole::parse("owner").unwrap(),
        StoreMemberRole::Owner
    );
    assert_eq!(
        StoreMemberRole::parse("admin").unwrap(),
        StoreMemberRole::Admin
    );
    assert_eq!(
        StoreMemberRole::parse("developer").unwrap(),
        StoreMemberRole::Developer
    );
    assert_eq!(
        StoreMemberRole::parse("accountant").unwrap(),
        StoreMemberRole::Accountant
    );
    assert_eq!(
        StoreMemberRole::parse("viewer").unwrap(),
        StoreMemberRole::Viewer
    );
}

#[test]
fn test_role_parse_case_and_whitespace() {
    assert_eq!(
        StoreMemberRole::parse("OWNER").unwrap(),
        StoreMemberRole::Owner
    );
    assert_eq!(
        StoreMemberRole::parse(" Admin ").unwrap(),
        StoreMemberRole::Admin
    );
    assert_eq!(
        StoreMemberRole::parse("  Developer  ").unwrap(),
        StoreMemberRole::Developer
    );
    assert_eq!(
        StoreMemberRole::parse("ACCOUNTANT\n").unwrap(),
        StoreMemberRole::Accountant
    );
    assert_eq!(
        StoreMemberRole::parse("\tViEwEr\t").unwrap(),
        StoreMemberRole::Viewer
    );
}

#[test]
fn test_role_from_str() {
    assert_eq!(
        StoreMemberRole::from_str("owner").unwrap(),
        StoreMemberRole::Owner
    );
    assert_eq!(
        StoreMemberRole::from_str("admin").unwrap(),
        StoreMemberRole::Admin
    );
    assert_eq!(
        StoreMemberRole::from_str("developer").unwrap(),
        StoreMemberRole::Developer
    );
    assert_eq!(
        StoreMemberRole::from_str("accountant").unwrap(),
        StoreMemberRole::Accountant
    );
    assert_eq!(
        StoreMemberRole::from_str("viewer").unwrap(),
        StoreMemberRole::Viewer
    );
}

#[test]
fn test_role_try_from() {
    assert_eq!(
        StoreMemberRole::try_from("owner").unwrap(),
        StoreMemberRole::Owner
    );
    assert_eq!(
        StoreMemberRole::try_from(String::from("developer")).unwrap(),
        StoreMemberRole::Developer
    );
}

#[test]
fn test_role_parse_invalid() {
    let invalid_inputs = [
        "",
        " ",
        "superadmin",
        "guest",
        "member",
        "root",
        "user",
        "owners",
        "null",
        "undefined",
        "1",
    ];

    for input in invalid_inputs {
        let err = StoreMemberRole::parse(input).unwrap_err();
        assert_eq!(err, ParseRoleError(input.trim().to_ascii_lowercase()));
        assert_eq!(err.as_str(), input.trim().to_ascii_lowercase());
        assert_eq!(err.into_inner(), input.trim().to_ascii_lowercase());
    }
}

#[test]
fn test_role_never_silently_falls_back() {
    assert!(StoreMemberRole::parse("invalid").is_err());
    assert_ne!(
        StoreMemberRole::parse("invalid").unwrap_err(),
        ParseRoleError("viewer".to_string())
    );
}

#[test]
fn test_role_is_owner() {
    assert!(StoreMemberRole::Owner.is_owner());
    assert!(!StoreMemberRole::Admin.is_owner());
    assert!(!StoreMemberRole::Developer.is_owner());
    assert!(!StoreMemberRole::Accountant.is_owner());
    assert!(!StoreMemberRole::Viewer.is_owner());
}

#[test]
fn test_role_all_constant() {
    assert_eq!(StoreMemberRole::ALL.len(), 5);
    assert_eq!(
        StoreMemberRole::ALL,
        [
            StoreMemberRole::Owner,
            StoreMemberRole::Admin,
            StoreMemberRole::Developer,
            StoreMemberRole::Accountant,
            StoreMemberRole::Viewer,
        ]
    );
}
