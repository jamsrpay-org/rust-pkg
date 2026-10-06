use crate::{
    Audience, AuthorizationContext, Claims, Issuer, JwtDecoder, JwtEncoder, Role, Scope, StoreId,
    StoreMemberRole, TokenParams, UserId, error::JwtError,
};
use chrono::{Duration, Utc};
use jamsrpay_authorization::Permission;
use std::{collections::HashMap, thread, time::Duration as StdDuration};

fn get_encoder() -> JwtEncoder {
    let private_key = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/jwt_private.pem"
    ))
    .or_else(|_| std::fs::read_to_string("jwt_private.pem"))
    .expect("missing private key file: jwt_private.pem");
    JwtEncoder::new(
        &private_key,
        Issuer::AuthService,
        Audience::ApiGateway,
        Duration::days(7),
    )
    .unwrap()
}

fn get_decoder() -> JwtDecoder {
    let public_key = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/jwt_public.pem"
    ))
    .or_else(|_| std::fs::read_to_string("jwt_public.pem"))
    .expect("missing public key file: jwt_public.pem");
    JwtDecoder::new(&public_key, Issuer::AuthService, Audience::ApiGateway).unwrap()
}

fn access_token_params(sub: &str) -> TokenParams {
    TokenParams::new(sub, Scope::AccessToken, Role::Staff, "session-uuid-001")
}

// ─── Basic encode / decode ───────────────────────────────────────────

#[test]
fn test_encode_and_decode_success() {
    let encoder = get_encoder();
    let decoder = get_decoder();

    let token = encoder.encode(access_token_params("user-123")).unwrap();
    let claims = decoder.decode(&token).unwrap();

    assert_eq!(claims.sub, "user-123");
    assert_eq!(claims.iss, Issuer::AuthService);
    assert_eq!(claims.aud, Audience::ApiGateway);
    assert_eq!(claims.scope, Scope::AccessToken);
    assert_eq!(claims.role, Role::Staff);
    assert_eq!(claims.session_id, "session-uuid-001");
    assert!(claims.stores.is_empty());
    assert!(claims.exp > claims.iat);
    assert!(!claims.jti.is_empty());
}

// ─── Scope validation ────────────────────────────────────────────────

#[test]
fn test_decode_with_scope_success() {
    let encoder = get_encoder();
    let decoder = get_decoder();

    let token = encoder.encode(access_token_params("user-1")).unwrap();
    let claims = decoder
        .decode_with_scope(&token, Scope::AccessToken)
        .unwrap();

    assert_eq!(claims.scope, Scope::AccessToken);
}

#[test]
fn test_decode_with_scope_mismatch() {
    let encoder = get_encoder();
    let decoder = get_decoder();

    let token = encoder.encode(access_token_params("user-1")).unwrap();
    let result = decoder.decode_with_scope(&token, Scope::RefreshToken);

    assert!(matches!(result, Err(JwtError::ScopeMismatch { .. })));
}

#[test]
fn test_refresh_token_scope() {
    let encoder = get_encoder();
    let decoder = get_decoder();

    let params = TokenParams::new("user-1", Scope::RefreshToken, Role::Staff, "session-uuid-001")
        .with_expires_in(Duration::days(30));

    let token = encoder.encode(params).unwrap();
    let claims = decoder
        .decode_with_scope(&token, Scope::RefreshToken)
        .unwrap();

    assert_eq!(claims.scope, Scope::RefreshToken);
}

// ─── Expiration ──────────────────────────────────────────────────────

#[test]
fn test_token_expiration() {
    let encoder = get_encoder();
    let decoder = get_decoder();

    let mut params = access_token_params("user-1");
    params.expires_in = Some(Duration::seconds(1));

    let token = encoder.encode(params).unwrap();

    thread::sleep(StdDuration::from_secs(2));

    let result = decoder.decode(&token);
    assert!(result.is_err());

    if let Err(JwtError::Jwt(err)) = result {
        assert!(matches!(
            err.kind(),
            jsonwebtoken::errors::ErrorKind::ExpiredSignature
        ));
    } else {
        panic!("expected JwtError::Jwt with ExpiredSignature");
    }
}

#[test]
fn test_custom_expiration() {
    let encoder = get_encoder();
    let decoder = get_decoder();

    let mut params = access_token_params("user-1");
    params.expires_in = Some(Duration::hours(2));

    let token = encoder.encode(params).unwrap();
    let claims = decoder.decode(&token).unwrap();

    let expected_exp = (Utc::now() + Duration::hours(2)).timestamp() as usize;
    assert!(claims.exp <= expected_exp + 1);
    assert!(claims.exp >= expected_exp - 1);
}

#[test]
fn test_default_expiration_is_7_days() {
    let encoder = get_encoder();
    let decoder = get_decoder();

    let token = encoder.encode(access_token_params("user-1")).unwrap();
    let claims = decoder.decode(&token).unwrap();

    let expected_exp = (Utc::now() + Duration::days(7)).timestamp() as usize;
    assert!(claims.exp <= expected_exp + 1);
    assert!(claims.exp >= expected_exp - 1);
}

// ─── Invalid tokens ──────────────────────────────────────────────────

#[test]
fn test_invalid_signature() {
    let decoder = get_decoder();

    let corrupted = "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.\
        eyJzdWIiOiJ1c2VyIiwiaXNzIjoiYXV0aC1zZXJ2aWNlIn0.\
        corrupted";

    assert!(decoder.decode(corrupted).is_err());
}

#[test]
fn test_malformed_token() {
    let decoder = get_decoder();
    assert!(decoder.decode("not.a.jwt").is_err());
}

// ─── Multi-tenancy ───────────────────────────────────────────────────

#[test]
fn test_token_without_tenant_id() {
    let encoder = get_encoder();
    let decoder = get_decoder();

    let params = TokenParams::new(
        "admin-uuid",
        Scope::AccessToken,
        Role::Staff,
        "session-uuid-002",
    );

    let token = encoder.encode(params).unwrap();
    let claims = decoder.decode(&token).unwrap();

    assert_eq!(claims.role, Role::Staff);
    assert!(claims.is_staff());
    assert!(!claims.is_merchant());
}

// ─── JTI uniqueness ─────────────────────────────────────────────────

#[test]
fn test_jti_is_unique_per_token() {
    let encoder = get_encoder();

    let token1 = encoder.encode(access_token_params("user-1")).unwrap();
    let token2 = encoder.encode(access_token_params("user-1")).unwrap();

    let decoder = get_decoder();
    let claims1 = decoder.decode(&token1).unwrap();
    let claims2 = decoder.decode(&token2).unwrap();

    assert_ne!(claims1.jti, claims2.jti);
    assert!(!claims1.jti.is_empty());
    assert!(!claims2.jti.is_empty());
}

// ─── Independence ────────────────────────────────────────────────────

#[test]
fn test_multiple_tokens_independence() {
    let encoder = get_encoder();
    let decoder = get_decoder();

    let t1 = encoder.encode(access_token_params("user-1")).unwrap();
    let t2 = encoder.encode(access_token_params("user-2")).unwrap();

    let c1 = decoder.decode(&t1).unwrap();
    let c2 = decoder.decode(&t2).unwrap();

    assert_eq!(c1.sub, "user-1");
    assert_eq!(c2.sub, "user-2");
    assert_ne!(t1, t2);
}

// ─── Constructor errors ─────────────────────────────────────────────

#[test]
fn test_encoder_rejects_invalid_pem() {
    let result = JwtEncoder::new(
        "not-a-pem",
        Issuer::AuthService,
        Audience::ApiGateway,
        Duration::days(1),
    );
    assert!(result.is_err());
}

#[test]
fn test_decoder_rejects_invalid_pem() {
    let result = JwtDecoder::new("not-a-pem", Issuer::AuthService, Audience::ApiGateway);
    assert!(result.is_err());
}

// ─── Store memberships & Authorization Context ──────────────────────

#[test]
fn test_encode_and_decode_with_stores() {
    let encoder = get_encoder();
    let decoder = get_decoder();

    let store_1 = StoreId::generate();
    let store_2 = StoreId::generate();
    let store_3 = StoreId::generate();
    let unknown_store = StoreId::generate();

    let mut stores = HashMap::new();
    stores.insert(store_1, StoreMemberRole::Owner);
    stores.insert(store_2, StoreMemberRole::Developer);
    stores.insert(store_3, StoreMemberRole::Viewer);

    let merchant_id = UserId::generate();
    let params = TokenParams::new(
        merchant_id.to_string(),
        Scope::AccessToken,
        Role::Merchant,
        "session-uuid-001",
    )
    .with_stores(stores);

    let token = encoder.encode(params).unwrap();
    let claims = decoder.decode(&token).unwrap();

    assert!(claims.is_merchant());
    assert!(!claims.is_staff());
    assert_eq!(claims.stores.len(), 3);
    assert_eq!(claims.role_for_store(&store_1), Some(StoreMemberRole::Owner));
    assert_eq!(
        claims.role_for_store(&store_2),
        Some(StoreMemberRole::Developer)
    );
    assert_eq!(
        claims.role_for_store(&store_3),
        Some(StoreMemberRole::Viewer)
    );
    assert_eq!(claims.role_for_store(&unknown_store), None);

    assert!(claims.has_store_access(&store_1));
    assert!(claims.has_store_access(&store_2));
    assert!(!claims.has_store_access(&unknown_store));
}

#[test]
fn test_claims_to_authorization_context() {
    let merchant_id = UserId::generate();
    let store_1 = StoreId::generate();
    let store_2 = StoreId::generate();

    let mut stores = HashMap::new();
    stores.insert(store_1, StoreMemberRole::Owner);
    stores.insert(store_2, StoreMemberRole::Developer);

    let claims = Claims {
        iss: Issuer::AuthService,
        sub: merchant_id.to_string(),
        aud: Audience::ApiGateway,
        scope: Scope::AccessToken,
        role: Role::Merchant,
        session_id: "session-123".to_string(),
        iat: 1000,
        exp: 2000,
        jti: "jti-123".to_string(),
        stores: stores.clone(),
    };

    // Test explicit to_authorization_context()
    let auth_ctx = claims.to_authorization_context().unwrap();
    assert_eq!(auth_ctx.merchant_id, merchant_id);
    assert_eq!(
        auth_ctx.role_for_store(&store_1).unwrap(),
        StoreMemberRole::Owner
    );

    // Verify seamless integration with jamsrpay_authorization::authorize
    let authed = jamsrpay_authorization::authorize(&auth_ctx, &store_1, Permission::StoreUpdate)
        .unwrap();
    assert_eq!(authed.role, StoreMemberRole::Owner);

    // Developer cannot update store
    let err = jamsrpay_authorization::authorize(&auth_ctx, &store_2, Permission::StoreUpdate)
        .unwrap_err();
    assert_eq!(err, jamsrpay_authorization::AuthorizationError::PermissionDenied);

    // Test TryFrom<&Claims>
    let auth_ctx_ref: AuthorizationContext = (&claims).try_into().unwrap();
    assert_eq!(auth_ctx_ref.merchant_id, merchant_id);

    // Test TryFrom<Claims>
    let auth_ctx_owned: AuthorizationContext = claims.try_into().unwrap();
    assert_eq!(auth_ctx_owned.merchant_id, merchant_id);
}

#[test]
fn test_claims_to_authorization_context_invalid_uuid() {
    let claims = Claims {
        iss: Issuer::AuthService,
        sub: "not-a-valid-uuid".to_string(),
        aud: Audience::ApiGateway,
        scope: Scope::AccessToken,
        role: Role::Merchant,
        session_id: "session-123".to_string(),
        iat: 1000,
        exp: 2000,
        jti: "jti-123".to_string(),
        stores: HashMap::new(),
    };

    assert!(claims.to_authorization_context().is_err());
}

#[test]
fn test_serde_backwards_compatibility_without_stores() {
    let json = r#"{
        "iss": "auth-service",
        "sub": "user-123",
        "aud": "api-gateway",
        "scope": "access_token",
        "role": "staff",
        "session_id": "session-456",
        "iat": 1700000000,
        "exp": 1700000900,
        "jti": "jti-789"
    }"#;

    let claims: Claims = serde_json::from_str(json).unwrap();
    assert_eq!(claims.sub, "user-123");
    assert_eq!(claims.role, Role::Staff);
    assert!(claims.stores.is_empty());
}

#[test]
fn test_token_params_builder() {
    let store_id = StoreId::generate();
    let params = TokenParams::new("sub-1", Scope::AccessToken, Role::Merchant, "session-1")
        .with_store(store_id, StoreMemberRole::Admin)
        .with_expires_in(Duration::minutes(10));

    assert_eq!(params.sub, "sub-1");
    assert_eq!(params.scope, Scope::AccessToken);
    assert_eq!(params.role, Role::Merchant);
    assert_eq!(params.session_id, "session-1");
    assert_eq!(params.stores.get(&store_id), Some(&StoreMemberRole::Admin));
    assert_eq!(params.expires_in, Some(Duration::minutes(10)));
}

#[test]
fn test_jwt_json_example_file() {
    let json = include_str!("../jwt.json");
    let claims: Claims = serde_json::from_str(json).expect("failed to deserialize jwt.json");

    assert_eq!(claims.iss, Issuer::AuthService);
    assert_eq!(claims.aud, Audience::ApiGateway);
    assert_eq!(claims.scope, Scope::AccessToken);
    assert_eq!(claims.role, Role::Merchant);
    assert_eq!(claims.stores.len(), 3);

    // Verify it converts to AuthorizationContext
    let auth_ctx = claims.to_authorization_context().unwrap();
    assert_eq!(auth_ctx.stores.len(), 3);
}

