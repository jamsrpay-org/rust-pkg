use crate::{NoExtra, TokenClaims};
use chrono::Utc;
pub use jsonwebtoken::errors::Error as JWTError;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Serialize, de::DeserializeOwned};
use std::collections::HashSet;

/// Service for creating JWT tokens - requires private key
pub struct JWTEncoder {
    private_key: String,
    purpose: String,
}

/// Service for verifying JWT tokens - only needs public key
pub struct JWTDecoder {
    public_key: String,
    purpose: String,
    issuer: Option<String>,
    audience: Option<Vec<String>>,
}

impl JWTEncoder {
    pub fn new(private_key: String, purpose: String) -> Self {
        JWTEncoder {
            private_key,
            purpose,
        }
    }

    /// Create a token with extra custom claims
    pub fn create_token_with_extra<T>(
        &self,
        subject: String,
        expiration: Option<chrono::TimeDelta>,
        extra: T,
    ) -> Result<String, JWTError>
    where
        T: Serialize,
    {
        let encoding_key = EncodingKey::from_rsa_pem(self.private_key.as_bytes())?;
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let expiration = expiration.unwrap_or(chrono::Duration::days(7));
        let exp = (now + expiration).timestamp() as usize;

        let claims = TokenClaims {
            sub: subject,
            purpose: self.purpose.clone(),
            exp,
            iat,
            extra,
        };

        let header = Header::new(Algorithm::RS256);
        encode(&header, &claims, &encoding_key)
    }

    /// Create a token without extra claims
    pub fn create_token(
        &self,
        subject: String,
        expiration: Option<chrono::TimeDelta>,
    ) -> Result<String, JWTError> {
        self.create_token_with_extra(subject, expiration, NoExtra)
    }
}

impl JWTDecoder {
    pub fn new(public_key: String, purpose: String) -> Self {
        JWTDecoder {
            public_key,
            purpose,
            issuer: None,
            audience: None,
        }
    }

    /// Create decoder with issuer and audience validation
    pub fn with_validation(
        public_key: String,
        purpose: String,
        issuer: Option<String>,
        audience: Option<Vec<String>>,
    ) -> Self {
        JWTDecoder {
            public_key,
            purpose,
            issuer,
            audience,
        }
    }

    /// Decode a token with extra custom claims
    pub fn decode_token_with_extra<T>(&self, token: &str) -> Result<TokenClaims<T>, JWTError>
    where
        T: DeserializeOwned,
    {
        let decoding_key = DecodingKey::from_rsa_pem(self.public_key.as_bytes())?;

        let mut validation = Validation::new(Algorithm::RS256);

        // Set required claims
        let mut required_spec_claims = HashSet::new();
        required_spec_claims.insert("sub".to_string());
        required_spec_claims.insert("exp".to_string());
        required_spec_claims.insert("iat".to_string());
        required_spec_claims.insert("purpose".to_string());
        validation.required_spec_claims = required_spec_claims;

        // Set issuer validation if provided
        if let Some(ref issuer) = self.issuer {
            validation.set_issuer(&[issuer]);
        }

        // Set audience validation if provided
        if let Some(ref audience) = self.audience {
            validation.set_audience(audience);
        }

        validation.leeway = 0;

        let token_data = decode::<TokenClaims<T>>(token, &decoding_key, &validation)?;

        // Validate purpose matches
        if token_data.claims.purpose != self.purpose {
            return Err(jsonwebtoken::errors::Error::from(
                jsonwebtoken::errors::ErrorKind::InvalidToken,
            ));
        }

        Ok(token_data.claims)
    }

    /// Decode a token without extra claims
    pub fn decode_token(&self, token: &str) -> Result<TokenClaims<NoExtra>, JWTError> {
        self.decode_token_with_extra(token)
    }
}

#[cfg(test)]
mod jwt_rs_tests {
    use super::*;
    use chrono::Duration;
    use jsonwebtoken::errors::ErrorKind;
    use serde::{Deserialize, Serialize};
    use std::{thread, time::Duration as StdDuration};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct ExtraClaims {
        pub email: String,
        pub name: String,
    }

    fn get_encoder() -> JWTEncoder {
        let private_key = std::fs::read_to_string("jwt_private.pem")
            .expect("missing private key file: jwt_private.pem");
        JWTEncoder::new(private_key, "Authorization".to_string())
    }

    fn get_decoder() -> JWTDecoder {
        let public_key = std::fs::read_to_string("jwt_public.pem")
            .expect("missing public key file: jwt_public.pem");
        JWTDecoder::new(public_key, "Authorization".to_string())
    }

    #[test]
    fn test_create_and_decode_token_success() {
        let encoder = get_encoder();
        let decoder = get_decoder();

        let token = encoder
            .create_token("user-id-123".to_string(), None)
            .unwrap();

        let claims = decoder.decode_token(&token).unwrap();

        assert_eq!(claims.sub, "user-id-123");
        assert_eq!(claims.purpose, "Authorization");
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn test_token_expiration() {
        let encoder = get_encoder();
        let decoder = get_decoder();

        let token = encoder
            .create_token("user-id".to_string(), Some(Duration::seconds(1)))
            .unwrap();

        thread::sleep(StdDuration::from_secs(2));

        let result = decoder.decode_token(&token);
        assert!(result.is_err());

        if let Err(err) = result {
            assert!(matches!(err.kind(), ErrorKind::ExpiredSignature));
        }
    }

    #[test]
    fn test_invalid_token_signature() {
        let decoder = get_decoder();

        // Manually corrupted token
        let corrupted_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJzdWIiOiJ1c2VyLWlkIiwicHVycG9zZSI6IkF1dGhvcml6YXRpb24iLCJpYXQiOjE3MzUyMTU5MDAsImV4cCI6MTczNTgyMDcwMH0.corrupted";

        let result = decoder.decode_token(&corrupted_token);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_token_purpose() {
        let encoder = get_encoder();
        let token = encoder.create_token("user-id".to_string(), None).unwrap();

        // Decoder with different purpose
        let public_key = std::fs::read_to_string("jwt_public.pem").unwrap();
        let wrong_decoder = JWTDecoder::new(public_key, "RefreshToken".to_string());

        let result = wrong_decoder.decode_token(&token);
        assert!(result.is_err());

        if let Err(err) = result {
            assert!(matches!(err.kind(), ErrorKind::InvalidToken));
        }
    }

    #[test]
    fn test_malformed_token() {
        let decoder = get_decoder();
        let result = decoder.decode_token("this.is.not.a.valid.jwt");
        assert!(result.is_err());
    }

    #[test]
    fn test_custom_expiration() {
        let encoder = get_encoder();
        let decoder = get_decoder();

        let token = encoder
            .create_token("user-id".to_string(), Some(Duration::hours(10)))
            .unwrap();

        let claims = decoder.decode_token(&token).unwrap();
        let expected_exp = (Utc::now() + Duration::hours(10)).timestamp() as usize;

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
                Some(Duration::minutes(10)),
                extra,
            )
            .unwrap();

        let claims = decoder
            .decode_token_with_extra::<ExtraClaims>(&token)
            .unwrap();

        assert_eq!(claims.sub, "user-login-123");
        assert_eq!(claims.extra.email, "user@example.com");
        assert_eq!(claims.extra.name, "John Doe");
    }

    #[test]
    fn test_multiple_tokens_independence() {
        let encoder = get_encoder();
        let decoder = get_decoder();

        let token1 = encoder.create_token("user-1".to_string(), None).unwrap();
        let token2 = encoder.create_token("user-2".to_string(), None).unwrap();

        let claims1 = decoder.decode_token(&token1).unwrap();
        let claims2 = decoder.decode_token(&token2).unwrap();

        assert_eq!(claims1.sub, "user-1");
        assert_eq!(claims2.sub, "user-2");
        assert_ne!(token1, token2);
    }

    #[test]
    fn test_decoder_only_needs_public_key() {
        // Demonstrates that decoder can work independently
        // without ever having access to private key
        let public_key = std::fs::read_to_string("jwt_public.pem").unwrap();
        let decoder = JWTDecoder::new(public_key, "Authorization".to_string());

        // Use a pre-generated token (from auth service)
        let existing_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJzdWIiOiJMb2dpbiIsInB1cnBvc2UiOiJBdXRob3JpemF0aW9uIiwiaWF0IjoxNzM1MjE1OTAwLCJleHAiOjE3MzU4MjA3MDB9.signature";

        // This would work if the token was valid
        // Most microservices only need this decoder
        assert!(decoder.decode_token(&existing_token).is_err()); // Invalid for demo
    }

    #[test]
    fn test_with_validation_setup() {
        let public_key = std::fs::read_to_string("jwt_public.pem").unwrap();

        let decoder = JWTDecoder::with_validation(
            public_key,
            "Authorization".to_string(),
            Some("https://myapp.com".to_string()),
            Some(vec!["api://default".to_string()]),
        );

        assert_eq!(decoder.issuer, Some("https://myapp.com".to_string()));
        assert_eq!(decoder.audience, Some(vec!["api://default".to_string()]));
    }
}
