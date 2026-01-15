use crate::error::TronError;

pub struct TronClient {
    pub endpoint: String,
}

impl TronClient {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
        }
    }

    pub async fn call(&self, _payload: &str) -> Result<String, TronError> {
        // real HTTP call goes here
        Ok("ok".to_string())
    }
}
