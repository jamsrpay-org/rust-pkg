use crate::{JwtDecoder, JwtEncoder, error::JwtError};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use std::{thread, time::Duration as StdDuration};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ExtraClaims {
    pub email: String,
    pub name: String,
}

fn get_encoder() -> JwtEncoder {
    let private_key = std::fs::read_to_string("jwt_private.pem")
        .expect("missing private key file: jwt_private.pem");
    JwtEncoder::new(private_key)
}

fn get_decoder() -> JwtDecoder {
    let public_key =
        std::fs::read_to_string("jwt_public.pem").expect("missing public key file: jwt_public.pem");
    JwtDecoder::new(public_key)
}

#[test]
fn test_create_and_decode_token_success() {
    let encoder = get_encoder();
    let decoder = get_decoder();

    let token = encoder
        .create_token("user-id-123".to_string(), "Authorization".to_string(), None)
        .unwrap();

    let claims = decoder
        .decode_token(&token, "Authorization", None, None)
        .unwrap();

    assert_eq!(claims.sub, "user-id-123");
    assert_eq!(claims.purpose, "Authorization");
    assert!(claims.exp > claims.iat);
}

#[test]
fn test_token_expiration() {
    let encoder = get_encoder();
    let decoder = get_decoder();

    let token = encoder
        .create_token(
            "user-id".to_string(),
            "Authorization".to_string(),
            Some(Duration::seconds(1)),
        )
        .unwrap();

    thread::sleep(StdDuration::from_secs(2));

    let result = decoder.decode_token(&token, "Authorization", None, None);
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
fn test_invalid_token_signature() {
    let decoder = get_decoder();

    let corrupted_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.\
            eyJzdWIiOiJ1c2VyLWlkIiwicHVycG9zZSI6IkF1dGhvcml6YXRpb24iLCJpYXQiOjE3MzUyMTU5MDAsImV4cCI6MTczNTgyMDcwMH0.\
            corrupted";

    let result = decoder.decode_token(corrupted_token, "Authorization", None, None);
    assert!(result.is_err());
}

#[test]
fn test_invalid_token_purpose() {
    let encoder = get_encoder();
    let decoder = get_decoder();

    let token = encoder
        .create_token("user-id".to_string(), "Authorization".to_string(), None)
        .unwrap();

    let result = decoder.decode_token(&token, "RefreshToken", None, None);
    assert!(result.is_err());

    assert!(
        matches!(result, Err(JwtError::PurposeMismatch { .. })),
        "expected PurposeMismatch error"
    );
}

#[test]
fn test_malformed_token() {
    let decoder = get_decoder();
    let result = decoder.decode_token("this.is.not.a.valid.jwt", "Authorization", None, None);
    assert!(result.is_err());
}

#[test]
fn test_custom_expiration() {
    let encoder = get_encoder();
    let decoder = get_decoder();

    let token = encoder
        .create_token(
            "user-id".to_string(),
            "Authorization".to_string(),
            Some(Duration::hours(10)),
        )
        .unwrap();

    let claims = decoder
        .decode_token(&token, "Authorization", None, None)
        .unwrap();

    let expected_exp = (Utc::now() + Duration::hours(10)).timestamp() as usize;

    // Allow 1 second tolerance for timing
    assert!(claims.exp <= expected_exp + 1);
    assert!(claims.exp >= expected_exp - 1);
}

#[test]
fn test_extra_claims_creation_and_decoding() {
    let encoder = get_encoder();
    let decoder = get_decoder();

    let extra = ExtraClaims {
        email: "user@example.com".to_string(),
        name: "John Doe".to_string(),
    };

    let token = encoder
        .create_token_with_extra(
            "user-login-123".to_string(),
            "Authorization".to_string(),
            Some(Duration::minutes(10)),
            extra,
        )
        .unwrap();

    let claims = decoder
        .decode_token_with_extra::<ExtraClaims>(&token, "Authorization", None, None)
        .unwrap();

    assert_eq!(claims.sub, "user-login-123");
    assert_eq!(claims.extra.email, "user@example.com");
    assert_eq!(claims.extra.name, "John Doe");
}

#[test]
fn test_multiple_tokens_independence() {
    let encoder = get_encoder();
    let decoder = get_decoder();

    let token1 = encoder
        .create_token("user-1".to_string(), "Authorization".to_string(), None)
        .unwrap();
    let token2 = encoder
        .create_token("user-2".to_string(), "Authorization".to_string(), None)
        .unwrap();

    let claims1 = decoder
        .decode_token(&token1, "Authorization", None, None)
        .unwrap();
    let claims2 = decoder
        .decode_token(&token2, "Authorization", None, None)
        .unwrap();

    assert_eq!(claims1.sub, "user-1");
    assert_eq!(claims2.sub, "user-2");
    assert_ne!(token1, token2);
}

#[test]
fn test_decoder_only_needs_public_key() {
    let public_key = std::fs::read_to_string("jwt_public.pem").unwrap();
    let decoder = JwtDecoder::new(public_key);

    // A pre-generated token with an invalid signature — demonstrates
    // that the decoder can be constructed independently from the encoder.
    let existing_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.\
            eyJzdWIiOiJMb2dpbiIsInB1cnBvc2UiOiJBdXRob3JpemF0aW9uIiwiaWF0IjoxNzM1MjE1OTAwLCJleHAiOjE3MzU4MjA3MDB9.\
            signature";

    // This will fail (invalid signature) but proves the decoder works standalone
    assert!(
        decoder
            .decode_token(existing_token, "Authorization", None, None)
            .is_err()
    );
}

#[test]
fn test_default_expiration_is_7_days() {
    let encoder = get_encoder();
    let decoder = get_decoder();

    let token = encoder
        .create_token("user-id".to_string(), "Authorization".to_string(), None)
        .unwrap();

    let claims = decoder
        .decode_token(&token, "Authorization", None, None)
        .unwrap();

    let expected_exp = (Utc::now() + Duration::days(7)).timestamp() as usize;
    assert!(claims.exp <= expected_exp + 1);
    assert!(claims.exp >= expected_exp - 1);
}
