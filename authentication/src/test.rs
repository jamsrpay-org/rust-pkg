use std::{collections::HashMap, thread, time::Duration as StdDuration};

use chrono::Duration;
use jamsrpay_authorization::{AuthorizationContext, Permission, StoreMemberRole};
use jamsrpay_types::{merchant_id::MerchantId, store_id::StoreId};

use crate::{
    Audience, Claims, Issuer, JwtDecoder, JwtEncoder, Role, Scope, TokenParams, error::JwtError,
};

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

fn access_token_params(merchant_id: MerchantId) -> TokenParams {
    TokenParams::new(
        merchant_id,
        Scope::AccessToken,
        Role::Merchant,
        "session-uuid-001",
    )
}

// ─── Valid JWT Encoding and Decoding & Round-trip ───────────────────────────

#[test]
fn test_valid_jwt_encoding_and_decoding_roundtrip() {
    let encoder = get_encoder();
    let decoder = get_decoder();
    let merchant_id = MerchantId::generate();

    let store_id = StoreId::generate();
    let mut stores = HashMap::new();
    stores.insert(store_id, StoreMemberRole::Owner);

    let params = TokenParams::new(
        merchant_id,
        Scope::AccessToken,
        Role::Merchant,
        "session-uuid-roundtrip",
    )
    .with_stores(stores);

    let token = encoder.encode(params).unwrap();
    let claims = decoder.decode(&token).unwrap();

    assert_eq!(claims.sub, merchant_id);
    assert_eq!(claims.iss, Issuer::AuthService);
    assert_eq!(claims.aud, Audience::ApiGateway);
    assert_eq!(claims.scope, Scope::AccessToken);
    assert_eq!(claims.role, Role::Merchant);
    assert_eq!(claims.session_id, "session-uuid-roundtrip");
    assert_eq!(claims.stores.len(), 1);
    assert_eq!(claims.stores.get(&store_id), Some(&StoreMemberRole::Owner));
    assert!(claims.exp > claims.iat);
    assert!(!claims.jti.is_empty());
}

// ─── Store Membership Serialization (Zero, Multiple, All 5 Roles) ─────────

#[test]
fn test_zero_stores_membership() {
    let encoder = get_encoder();
    let decoder = get_decoder();
    let merchant_id = MerchantId::generate();

    let params = TokenParams::new(
        merchant_id,
        Scope::AccessToken,
        Role::Merchant,
        "session-zero-stores",
    );
    assert!(params.stores.is_empty());

    let token = encoder.encode(params).unwrap();
    let claims = decoder.decode(&token).unwrap();

    assert_eq!(claims.sub, merchant_id);
    assert!(claims.stores.is_empty());
    assert!(!claims.has_store_access(&StoreId::generate()));
}

#[test]
fn test_multiple_stores_membership() {
    let encoder = get_encoder();
    let decoder = get_decoder();
    let merchant_id = MerchantId::generate();

    let s1 = StoreId::generate();
    let s2 = StoreId::generate();
    let s3 = StoreId::generate();

    let mut stores = HashMap::new();
    stores.insert(s1, StoreMemberRole::Owner);
    stores.insert(s2, StoreMemberRole::Developer);
    stores.insert(s3, StoreMemberRole::Viewer);

    let params = access_token_params(merchant_id).with_stores(stores);
    let token = encoder.encode(params).unwrap();
    let claims = decoder.decode(&token).unwrap();

    assert_eq!(claims.stores.len(), 3);
    assert_eq!(claims.role_for_store(&s1), Some(StoreMemberRole::Owner));
    assert_eq!(claims.role_for_store(&s2), Some(StoreMemberRole::Developer));
    assert_eq!(claims.role_for_store(&s3), Some(StoreMemberRole::Viewer));
    assert!(claims.has_store_access(&s1));
    assert!(claims.has_store_access(&s2));
    assert!(claims.has_store_access(&s3));
}

#[test]
fn test_jwt_containing_all_five_roles() {
    let encoder = get_encoder();
    let decoder = get_decoder();
    let merchant_id = MerchantId::generate();

    let s_owner = StoreId::generate();
    let s_admin = StoreId::generate();
    let s_dev = StoreId::generate();
    let s_acc = StoreId::generate();
    let s_view = StoreId::generate();

    let mut stores = HashMap::new();
    stores.insert(s_owner, StoreMemberRole::Owner);
    stores.insert(s_admin, StoreMemberRole::Admin);
    stores.insert(s_dev, StoreMemberRole::Developer);
    stores.insert(s_acc, StoreMemberRole::Accountant);
    stores.insert(s_view, StoreMemberRole::Viewer);

    let params = access_token_params(merchant_id).with_stores(stores);
    let token = encoder.encode(params).unwrap();
    let claims = decoder.decode(&token).unwrap();

    assert_eq!(claims.stores.len(), 5);
    assert_eq!(claims.role_for_store(&s_owner), Some(StoreMemberRole::Owner));
    assert_eq!(claims.role_for_store(&s_admin), Some(StoreMemberRole::Admin));
    assert_eq!(claims.role_for_store(&s_dev), Some(StoreMemberRole::Developer));
    assert_eq!(claims.role_for_store(&s_acc), Some(StoreMemberRole::Accountant));
    assert_eq!(claims.role_for_store(&s_view), Some(StoreMemberRole::Viewer));
}

// ─── Expiration ─────────────────────────────────────────────────────────────

#[test]
fn test_token_expiration() {
    let encoder = get_encoder();
    let decoder = get_decoder();
    let merchant_id = MerchantId::generate();

    let mut params = access_token_params(merchant_id);
    params.expires_in = Some(Duration::seconds(1));

    let token = encoder.encode(params).unwrap();

    thread::sleep(StdDuration::from_secs(2));

    let result = decoder.decode(&token);
    assert_eq!(result.unwrap_err(), JwtError::Expired);
}

// ─── Invalid Signature, Token, Issuer, Audience, Scope, Claims ─────────────

#[test]
fn test_invalid_signature() {
    let encoder = get_encoder();
    let decoder = get_decoder();
    let merchant_id = MerchantId::generate();
    let valid_token = encoder.encode(access_token_params(merchant_id)).unwrap();

    let parts: Vec<&str> = valid_token.split('.').collect();
    assert_eq!(parts.len(), 3);
    // Replace the last 4 characters with 'AAAA' which is always valid base64url
    let mut sig = parts[2].to_string();
    sig.replace_range(sig.len() - 4.., "AAAA");
    let tampered_token = format!("{}.{}.{}", parts[0], parts[1], sig);

    let err = decoder.decode(&tampered_token).unwrap_err();
    assert_eq!(err, JwtError::InvalidSignature);
}

#[test]
fn test_invalid_token_format() {
    let decoder = get_decoder();
    assert_eq!(decoder.decode("not-a-valid-jwt").unwrap_err(), JwtError::InvalidToken);
    assert_eq!(decoder.decode("part1.part2").unwrap_err(), JwtError::InvalidToken);
}

#[test]
fn test_invalid_issuer() {
    let decoder = get_decoder();
    let private_key = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/jwt_private.pem"
    ))
    .or_else(|_| std::fs::read_to_string("jwt_private.pem"))
    .expect("missing private key file: jwt_private.pem");
    let encoding_key = jsonwebtoken::EncodingKey::from_rsa_pem(private_key.as_bytes()).unwrap();

    #[derive(serde::Serialize)]
    struct ForeignClaims {
        iss: String,
        sub: String,
        aud: String,
        scope: String,
        role: String,
        session_id: String,
        iat: usize,
        exp: usize,
        jti: String,
    }

    let now = chrono::Utc::now().timestamp() as usize;
    let foreign = ForeignClaims {
        iss: "foreign-auth-service".to_string(),
        sub: MerchantId::generate().to_string(),
        aud: "api-gateway".to_string(),
        scope: "access_token".to_string(),
        role: "merchant".to_string(),
        session_id: "sid".to_string(),
        iat: now,
        exp: now + 3600,
        jti: "jti".to_string(),
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256),
        &foreign,
        &encoding_key,
    )
    .unwrap();

    let err = decoder.decode(&token).unwrap_err();
    assert_eq!(err, JwtError::InvalidIssuer);
}

#[test]
fn test_invalid_audience() {
    let decoder = get_decoder();
    let private_key = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/jwt_private.pem"
    ))
    .or_else(|_| std::fs::read_to_string("jwt_private.pem"))
    .expect("missing private key file: jwt_private.pem");
    let encoding_key = jsonwebtoken::EncodingKey::from_rsa_pem(private_key.as_bytes()).unwrap();

    #[derive(serde::Serialize)]
    struct ForeignClaims {
        iss: String,
        sub: String,
        aud: String,
        scope: String,
        role: String,
        session_id: String,
        iat: usize,
        exp: usize,
        jti: String,
    }

    let now = chrono::Utc::now().timestamp() as usize;
    let foreign = ForeignClaims {
        iss: "auth-service".to_string(),
        sub: MerchantId::generate().to_string(),
        aud: "foreign-audience".to_string(),
        scope: "access_token".to_string(),
        role: "merchant".to_string(),
        session_id: "sid".to_string(),
        iat: now,
        exp: now + 3600,
        jti: "jti".to_string(),
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256),
        &foreign,
        &encoding_key,
    )
    .unwrap();

    let err = decoder.decode(&token).unwrap_err();
    assert_eq!(err, JwtError::InvalidAudience);
}

#[test]
fn test_invalid_scope() {
    let encoder = get_encoder();
    let decoder = get_decoder();
    let merchant_id = MerchantId::generate();

    let token = encoder.encode(access_token_params(merchant_id)).unwrap();
    // Expecting RefreshToken, but token has AccessToken
    let result = decoder.decode_with_scope(&token, Scope::RefreshToken);
    assert_eq!(result.unwrap_err(), JwtError::InvalidScope);
}

#[test]
fn test_decode_with_matching_scope() {
    let encoder = get_encoder();
    let decoder = get_decoder();
    let merchant_id = MerchantId::generate();

    let token = encoder.encode(access_token_params(merchant_id)).unwrap();
    let claims = decoder.decode_with_scope(&token, Scope::AccessToken).unwrap();
    assert_eq!(claims.scope, Scope::AccessToken);
}

#[test]
fn test_malformed_claims() {
    // A token with payload that does not match Claims schema (e.g. invalid sub UUID)
    // We can test by deserializing invalid JSON directly
    let invalid_json = r#"{
        "iss": "auth-service",
        "sub": "not-a-uuid",
        "aud": "api-gateway",
        "scope": "access_token",
        "role": "merchant",
        "session_id": "sid",
        "iat": 100,
        "exp": 200,
        "jti": "jti"
    }"#;
    let res: Result<Claims, _> = serde_json::from_str(invalid_json);
    assert!(res.is_err(), "should fail deserialization when sub is not a valid UUID");
}

// ─── Claims to AuthorizationContext & authorize() Integration ──────────────

#[test]
fn test_claims_to_authorization_context_and_authorize() {
    let merchant_id = MerchantId::generate();
    let s_owner = StoreId::generate();
    let s_dev = StoreId::generate();
    let s_acc = StoreId::generate();
    let s_view = StoreId::generate();
    let unknown_store = StoreId::generate();

    let mut stores = HashMap::new();
    stores.insert(s_owner, StoreMemberRole::Owner);
    stores.insert(s_dev, StoreMemberRole::Developer);
    stores.insert(s_acc, StoreMemberRole::Accountant);
    stores.insert(s_view, StoreMemberRole::Viewer);

    let claims = Claims {
        iss: Issuer::AuthService,
        sub: merchant_id,
        aud: Audience::ApiGateway,
        scope: Scope::AccessToken,
        role: Role::Merchant,
        session_id: "sid-001".to_string(),
        iat: 1000,
        exp: 2000,
        jti: "jti-001".to_string(),
        stores: stores.clone(),
    };

    // 1. Direct authorization_context()
    let auth_ctx = claims.authorization_context();
    assert_eq!(auth_ctx.merchant_id, merchant_id);
    assert_eq!(auth_ctx.stores.len(), 4);

    // 2. From<&Claims>
    let auth_ctx_from_ref: AuthorizationContext = (&claims).into();
    assert_eq!(auth_ctx_from_ref.merchant_id, merchant_id);

    // 3. From<Claims>
    let auth_ctx_owned: AuthorizationContext = claims.clone().into();
    assert_eq!(auth_ctx_owned.merchant_id, merchant_id);

    // 4. Authorize Owner on StoreUpdate -> success
    let authed_owner = jamsrpay_authorization::authorize(&auth_ctx, &s_owner, Permission::StoreUpdate).unwrap();
    assert_eq!(authed_owner.merchant_id, merchant_id);
    assert_eq!(authed_owner.store_id, s_owner);
    assert_eq!(authed_owner.role, StoreMemberRole::Owner);

    // 5. Authorize Developer on StoreUpdate -> PermissionDenied
    let err_dev_update = jamsrpay_authorization::authorize(&auth_ctx, &s_dev, Permission::StoreUpdate).unwrap_err();
    assert_eq!(err_dev_update, jamsrpay_authorization::AuthorizationError::PermissionDenied);

    // 6. Authorize Developer on InvoiceCreate -> success
    let authed_dev_invoice = jamsrpay_authorization::authorize(&auth_ctx, &s_dev, Permission::InvoiceCreate).unwrap();
    assert_eq!(authed_dev_invoice.role, StoreMemberRole::Developer);

    // 7. Authorize Accountant on InvoiceCreate -> PermissionDenied
    let err_acc_create = jamsrpay_authorization::authorize(&auth_ctx, &s_acc, Permission::InvoiceCreate).unwrap_err();
    assert_eq!(err_acc_create, jamsrpay_authorization::AuthorizationError::PermissionDenied);

    // 8. Authorize Accountant on InvoiceRead -> success
    let authed_acc_read = jamsrpay_authorization::authorize(&auth_ctx, &s_acc, Permission::InvoiceRead).unwrap();
    assert_eq!(authed_acc_read.role, StoreMemberRole::Accountant);

    // 9. Authorize Viewer on WebhookRead -> success
    let authed_view_webhook = jamsrpay_authorization::authorize(&auth_ctx, &s_view, Permission::WebhookRead).unwrap();
    assert_eq!(authed_view_webhook.role, StoreMemberRole::Viewer);

    // 10. Unknown store -> StoreAccessDenied
    let err_unknown = jamsrpay_authorization::authorize(&auth_ctx, &unknown_store, Permission::StoreRead).unwrap_err();
    assert_eq!(err_unknown, jamsrpay_authorization::AuthorizationError::StoreAccessDenied);
}

// ─── Deserialization & Fixture ──────────────────────────────────────────────

#[test]
fn test_serde_backwards_compatibility_without_stores() {
    let merchant_id = MerchantId::generate();
    let json = format!(
        r#"{{
            "iss": "auth-service",
            "sub": "{merchant_id}",
            "aud": "api-gateway",
            "scope": "access_token",
            "role": "staff",
            "session_id": "session-456",
            "iat": 1700000000,
            "exp": 1700000900,
            "jti": "jti-789"
        }}"#
    );

    let claims: Claims = serde_json::from_str(&json).unwrap();
    assert_eq!(claims.sub, merchant_id);
    assert_eq!(claims.role, Role::Staff);
    assert!(claims.stores.is_empty());
}

#[test]
fn test_jwt_json_fixture() {
    let json = include_str!("../jwt.json");
    let claims: Claims = serde_json::from_str(json).expect("failed to deserialize jwt.json");

    assert_eq!(claims.iss, Issuer::AuthService);
    assert_eq!(claims.aud, Audience::ApiGateway);
    assert_eq!(claims.scope, Scope::AccessToken);
    assert_eq!(claims.role, Role::Merchant);
    assert_eq!(claims.stores.len(), 3);

    let auth_ctx = claims.authorization_context();
    assert_eq!(auth_ctx.stores.len(), 3);
}

#[test]
fn test_token_params_builder() {
    let merchant_id = MerchantId::generate();
    let store_id = StoreId::generate();
    let params = TokenParams::new(merchant_id, Scope::AccessToken, Role::Merchant, "session-1")
        .with_store(store_id, StoreMemberRole::Admin)
        .with_expires_in(Duration::minutes(10));

    assert_eq!(params.sub, merchant_id);
    assert_eq!(params.scope, Scope::AccessToken);
    assert_eq!(params.role, Role::Merchant);
    assert_eq!(params.session_id, "session-1");
    assert_eq!(params.stores.get(&store_id), Some(&StoreMemberRole::Admin));
    assert_eq!(params.expires_in, Some(Duration::minutes(10)));
}
