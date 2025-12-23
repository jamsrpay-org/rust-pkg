use crate::error::SessionError;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac as _};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinuationToken {
    pub operation_id: String,
    pub expires_at: DateTime<Utc>,
    pub signature: String,
}

type HmacSha256 = Hmac<Sha256>;

impl ContinuationToken {
    pub fn new(operation_id: String, expires_at: DateTime<Utc>, secret_key: &[u8]) -> Self {
        let signature = Self::create_signature(&operation_id, &expires_at, secret_key);
        Self {
            operation_id,
            expires_at,
            signature,
        }
    }

    pub fn encode(&self) -> Result<String, SessionError> {
        let json = serde_json::to_string(self)?;
        let encoded = URL_SAFE_NO_PAD.encode(json.as_bytes());
        Ok(encoded)
    }

    pub fn decode(input: &str, secret_key: &[u8]) -> Result<Self, SessionError> {
        let bytes = URL_SAFE_NO_PAD.decode(input)?;
        let json = String::from_utf8(bytes)?;
        let token: ContinuationToken = serde_json::from_str(&json)?;

        // Verify Signature
        let expected_signature =
            Self::create_signature(&token.operation_id, &token.expires_at, secret_key);
        if token.signature != expected_signature {
            return Err(SessionError::InvalidTokenSignature);
        }

        // Check expiration
        if token.expires_at < Utc::now() {
            return Err(SessionError::TokenExpired);
        }

        Ok(token)
    }

    fn create_signature(
        operation_id: &str,
        expires_at: &DateTime<Utc>,
        secret_key: &[u8],
    ) -> String {
        let mut mac =
            HmacSha256::new_from_slice(secret_key).expect("HMAC can take key of any size");
        mac.update(operation_id.as_bytes());
        mac.update(expires_at.to_rfc3339().as_bytes());
        let result = mac.finalize();
        URL_SAFE_NO_PAD.encode(result.into_bytes())
    }
}
