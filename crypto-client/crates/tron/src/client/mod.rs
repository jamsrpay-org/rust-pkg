use crate::error::TronClientError;
use reqwest::ClientBuilder;
use std::time::Duration;

mod account;
mod bandwidth;
mod transaction;

pub struct TronClient {
    http_base_url: String,
    client: reqwest::Client,
}

impl TronClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();
        Self {
            http_base_url: base_url.into(),
            client,
        }
    }

    pub fn base_url(&self) -> &str {
        &self.http_base_url
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub async fn call(&self, _payload: &str) -> Result<String, TronClientError> {
        // real HTTP call goes here
        Ok("ok".to_string())
    }
}
