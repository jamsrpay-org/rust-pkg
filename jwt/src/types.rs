use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TokenClaims<T> {
    pub sub: String,     // Subject, typically the user ID
    pub purpose: String, // Purpose or reason for the token
    pub iat: usize,      // Issued at
    pub exp: usize,      // Expiration time
    #[serde(flatten)]
    pub extra: T,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct NoExtra;
