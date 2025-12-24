use chrono::{Duration, Utc};
pub use jsonwebtoken::errors::Error as JWTError;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims<T> {
    pub sub: String,     // Subject, typically the user ID
    pub purpose: String, // Purpose or reason for the token
    pub iat: usize,      // Issued at
    pub exp: usize,      // Expiration time
    #[serde(flatten)]
    pub extra: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoExtra;

pub struct JWTService<'a> {
    secret_key: &'a str,
    purpose: &'a str,
}

impl<'a> JWTService<'a> {
    pub fn new(secret_key: &'a str, purpose: &'a str) -> JWTService<'a> {
        JWTService {
            secret_key,
            purpose,
        }
    }

    pub fn create_token_with_extra<T>(
        &self,
        subject: String,
        expiration: Option<chrono::TimeDelta>,
        extra: T,
    ) -> Result<String, JWTError>
    where
        T: DeserializeOwned + Serialize,
    {
        let secret_key = self.secret_key;
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let expiration = expiration.unwrap_or(Duration::hours(1));
        let exp = (now + expiration).timestamp() as usize;

        let claims = TokenClaims::<T> {
            sub: subject,
            purpose: self.purpose.to_owned(),
            exp,
            iat,
            extra,
        };
        let header = Header::new(Algorithm::HS256);
        encode(
            &header,
            &claims,
            &EncodingKey::from_secret(secret_key.as_bytes()),
        )
    }

    pub fn create_token(
        &self,
        subject: String,
        expiration: Option<chrono::TimeDelta>,
    ) -> Result<String, JWTError> {
        self.create_token_with_extra::<NoExtra>(subject, expiration, NoExtra)
    }

    pub fn decode_token_with_extra<T>(&self, token: &str) -> Result<TokenClaims<T>, JWTError>
    where
        T: DeserializeOwned,
    {
        let secret_key = self.secret_key.as_bytes();

        let mut required_spec_claims = HashSet::new();
        required_spec_claims.insert("purpose".to_string());
        required_spec_claims.insert("sub".to_string());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.leeway = 0;
        validation.required_spec_claims = required_spec_claims;

        let token_data =
            decode::<TokenClaims<T>>(token, &DecodingKey::from_secret(secret_key), &validation)?;

        if token_data.claims.purpose != self.purpose {
            return Err(jsonwebtoken::errors::Error::from(
                jsonwebtoken::errors::ErrorKind::InvalidToken,
            ));
        }

        Ok(token_data.claims)
    }

    pub fn decode_token(&self, token: &str) -> Result<TokenClaims<NoExtra>, JWTError> {
        self.decode_token_with_extra(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use jsonwebtoken::errors::ErrorKind;
    use std::{thread, time::Duration as StdDuration};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ExtraClaims {
        pub email: String,
        pub name: String,
    }

    fn get_jwt_service() -> JWTService<'static> {
        JWTService::new("super-secret-key", "Authorization")
    }

    #[test]
    fn test_create_and_decode_token_success() {
        let jwt_service = get_jwt_service();
        let token = jwt_service
            .create_token("user-id".to_string(), None)
            .unwrap();
        let claims = jwt_service.decode_token(&token).unwrap();

        assert_eq!(claims.sub, "user-id");
        assert_eq!(claims.purpose, "Authorization");
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn test_token_expiration() {
        let jwt_service = get_jwt_service();
        let token = jwt_service
            .create_token("user-id".to_string(), Some(Duration::seconds(1)))
            .unwrap();

        // Wait for token to expire
        thread::sleep(StdDuration::from_secs(2));
        let result = jwt_service.decode_token(&token);
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(matches!(err.kind(), ErrorKind::ExpiredSignature));
        }
    }

    #[test]
    fn test_invalid_token_signature() {
        let jwt_service = get_jwt_service();
        let token = jwt_service
            .create_token("user-id".to_string(), None)
            .unwrap();

        // Use a different secret
        let other_service = JWTService::new("wrong-secret-key", jwt_service.purpose);

        let result = other_service.decode_token(&token);
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(matches!(err.kind(), ErrorKind::InvalidSignature));
        }
    }

    #[test]
    fn test_invalid_token_purpose() {
        let jwt_service = get_jwt_service();
        let token = jwt_service
            .create_token("user-id".to_string(), None)
            .unwrap();

        // Use a different purpose
        let other_service = JWTService::new(jwt_service.secret_key, "Test");

        let result = other_service.decode_token(&token);
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(matches!(err.kind(), ErrorKind::InvalidToken));
        }
    }

    #[test]
    fn test_malformed_token() {
        let jwt_service = get_jwt_service();
        let result = jwt_service.decode_token("this.is.not.jwt");
        assert!(result.is_err());
    }

    #[test]
    fn test_custom_expiration() {
        let jwt_service = get_jwt_service();
        let token = jwt_service
            .create_token("user-id".to_string(), Some(Duration::hours(10)))
            .unwrap();
        let claims = jwt_service.decode_token(&token).unwrap();

        let expected_exp = (Utc::now() + Duration::hours(10)).timestamp() as usize;
        assert!(claims.exp <= expected_exp);
    }

    #[test]
    fn test_extra_claims() {
        let jwt_service = get_jwt_service();
        let token = jwt_service
            .create_token_with_extra(
                "Login".to_string(),
                Some(Duration::minutes(10)),
                Some(ExtraClaims {
                    email: "user@example.com".to_string(),
                    name: "John Doe".to_string(),
                }),
            )
            .unwrap();
        let claims = jwt_service
            .decode_token_with_extra::<ExtraClaims>(&token)
            .unwrap();
        dbg!(&claims);

        // assert_eq!(claims.extra.is_some(), true)
    }
}
