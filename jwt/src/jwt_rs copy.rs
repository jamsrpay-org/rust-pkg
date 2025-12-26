use crate::{NoExtra, TokenClaims};
use chrono::Utc;
pub use jsonwebtoken::errors::Error as JWTError;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, encode};

pub struct JWTService {
    private_key: String,
    public_key: String,
}

impl JWTService {
    pub fn new(private_key: String, public_key: String) -> Self {
        JWTService {
            private_key,
            public_key,
        }
    }

    pub fn create_token(
        &self,
        subject: String,
        expiration: Option<chrono::TimeDelta>,
    ) -> Result<String, JWTError> {
        let encoding_key = EncodingKey::from_rsa_pem(self.private_key.as_bytes())?;
        let now = Utc::now();
        let expiration =
            (now + expiration.unwrap_or(chrono::Duration::days(7))).timestamp() as usize;

        let claims: TokenClaims<NoExtra> = TokenClaims {
            sub: subject,
            purpose: "login".to_string(),
            exp: expiration,
            iat: now.timestamp() as usize,
            extra: NoExtra,
        };

        let headers = Header::new(Algorithm::RS256);

        encode(&headers, &claims, &encoding_key)
    }

    pub fn decode_token(&self, token: &str) -> Result<TokenClaims<NoExtra>, JWTError> {
        let decoding_key = DecodingKey::from_rsa_pem(self.public_key.as_bytes())?;
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_required_spec_claims(&["sub", "exp", "iat"]);

        let token_data =
            jsonwebtoken::decode::<TokenClaims<NoExtra>>(token, &decoding_key, &validation)?;

        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod jwt_rs_tests {
    use crate::jwt_rs::JWTService;

    fn get_jwt_service() -> JWTService {
        let private_key = std::fs::read_to_string("jwt_private.pem").expect("missing private key");
        let public_key = std::fs::read_to_string("jwt_public.pem").expect("missing public key");

        JWTService::new(private_key, public_key)
    }

    #[test]
    fn test_creation() {
        let jwt_service = get_jwt_service();
        let token = jwt_service.create_token("Login".to_string(), None).unwrap();
        let token = token.replace("eyJ", "ejJ");
        let claims = jwt_service.decode_token(&token).unwrap();

        dbg!(&claims);
        dbg!(&token);
        assert_eq!(token.len() > 0, true);
    }

    #[test]
    fn test_decoding() {
        let jwt_service = get_jwt_service();
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJzdWIiOiJMb2dpbiIsInB1cnBvc2UiOiJsb2dpbiIsImlhdCI6MTc2Njc0OTY1OCwiZXhwIjoxNzY3MzU0NDU4fQ.r4REixvfyLkAUSBxw93sJaAlqpIG2d4WAmuZzkOAB_Xnp8Fh9lSeTV5E-SQuz4ZbPRynB2RhwgHme2jSfJ2CC0NF3Ale4Kn7biUo5PixPCiU5OzUTBHA0cVUw1MtuDM3u3X20EmB9pJHDJNeRNgQm0B1oey_Y8_uP7C5s5d_yPtGl9OvllnsEAINKPAoZhCqzfWehlcvHKgCVjiFjhJPRPH8oaLycQgZEPIxcV_KetIwcepZXH3ZkTWaHiK6QBPV-J-TFhB-RXiIx1sYFQja-k_w-9zg6i_5wvdIV-N79yHdAuaOj_94rkfSL1uq4M9etMTRmDb7zynKjE8-2X4-qw".to_string();
        let claims = jwt_service.decode_token(&token).unwrap();

        dbg!(&claims);
        assert_eq!(claims.sub, "Login");
    }
}
