use crate::{JwtDecoder, JwtEncoder, TokenParams, error::JwtError, scope};
use chrono::{Duration, Utc};
use std::{thread, time::Duration as StdDuration};

const ISSUER: &str = "auth-service";
const AUDIENCE: &str = "api";

fn get_encoder() -> JwtEncoder {
    let private_key = std::fs::read_to_string("jwt_private.pem")
        .expect("missing private key file: jwt_private.pem");
    JwtEncoder::new(
        &private_key,
        ISSUER.into(),
        AUDIENCE.into(),
        Duration::days(7),
    )
    .unwrap()
}

fn get_decoder() -> JwtDecoder {
    let public_key =
        std::fs::read_to_string("jwt_public.pem").expect("missing public key file: jwt_public.pem");
    JwtDecoder::new(&public_key, ISSUER, AUDIENCE).unwrap()
}

fn access_token_params(sub: &str) -> TokenParams {
    TokenParams {
        sub: sub.to_string(),
        scope: scope::ACCESS_TOKEN.to_string(),
        role: "merchant_admin".to_string(),
        tenant_id: Some("tenant-uuid-001".to_string()),
        session_id: "session-uuid-001".to_string(),
        expires_in: None,
    }
}

// ─── Basic encode / decode ───────────────────────────────────────────

#[test]
fn test_encode_and_decode_success() {
    let encoder = get_encoder();
    let decoder = get_decoder();

    let token = encoder.encode(access_token_params("user-123")).unwrap();
    let claims = decoder.decode(&token).unwrap();

    assert_eq!(claims.sub, "user-123");
    assert_eq!(claims.iss, ISSUER);
    assert_eq!(claims.aud, AUDIENCE);
    assert_eq!(claims.scope, scope::ACCESS_TOKEN);
    assert_eq!(claims.role, "merchant_admin");
    assert_eq!(claims.tenant_id, Some("tenant-uuid-001".to_string()));
    assert_eq!(claims.session_id, "session-uuid-001");
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
        .decode_with_scope(&token, scope::ACCESS_TOKEN)
        .unwrap();

    assert_eq!(claims.scope, scope::ACCESS_TOKEN);
}

#[test]
fn test_decode_with_scope_mismatch() {
    let encoder = get_encoder();
    let decoder = get_decoder();

    let token = encoder.encode(access_token_params("user-1")).unwrap();
    let result = decoder.decode_with_scope(&token, scope::REFRESH_TOKEN);

    assert!(matches!(result, Err(JwtError::ScopeMismatch { .. })));
}

#[test]
fn test_refresh_token_scope() {
    let encoder = get_encoder();
    let decoder = get_decoder();

    let params = TokenParams {
        sub: "user-1".to_string(),
        scope: scope::REFRESH_TOKEN.to_string(),
        role: "merchant_admin".to_string(),
        tenant_id: Some("tenant-uuid-001".to_string()),
        session_id: "session-uuid-001".to_string(),
        expires_in: Some(Duration::days(30)),
    };

    let token = encoder.encode(params).unwrap();
    let claims = decoder
        .decode_with_scope(&token, scope::REFRESH_TOKEN)
        .unwrap();

    assert_eq!(claims.scope, scope::REFRESH_TOKEN);
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

    let params = TokenParams {
        sub: "admin-uuid".to_string(),
        scope: scope::ACCESS_TOKEN.to_string(),
        role: "platform_admin".to_string(),
        tenant_id: None,
        session_id: "session-uuid-002".to_string(),
        expires_in: None,
    };

    let token = encoder.encode(params).unwrap();
    let claims = decoder.decode(&token).unwrap();

    assert_eq!(claims.role, "platform_admin");
    assert_eq!(claims.tenant_id, None);
}

#[test]
fn test_token_with_tenant_id() {
    let encoder = get_encoder();
    let decoder = get_decoder();

    let token = encoder.encode(access_token_params("user-1")).unwrap();
    let claims = decoder.decode(&token).unwrap();

    assert_eq!(claims.tenant_id, Some("tenant-uuid-001".to_string()));
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
    let result = JwtEncoder::new("not-a-pem", "iss".into(), "aud".into(), Duration::days(1));
    assert!(result.is_err());
}

#[test]
fn test_decoder_rejects_invalid_pem() {
    let result = JwtDecoder::new("not-a-pem", "iss", "aud");
    assert!(result.is_err());
}
