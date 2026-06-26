use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::{
    Client, StatusCode,
    header::{CONTENT_TYPE, HeaderMap, HeaderValue},
};
use sha2::Sha256;
use std::{collections::BTreeMap, time::Duration};

use crate::error::ItrxError;
use crate::types::*;

type HmacSha256 = Hmac<Sha256>;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
pub const MAINNET_BASE_URL: &str = "https://itrx.io/api/v1/frontend";
pub const TESTNET_BASE_URL: &str = "https://nile.itrx.io/api/v1/frontend";

// ─── Config ─────────────────────────────────────────────────────────

/// Configuration for the ITRX API client.
#[derive(Debug, Clone)]
pub struct ItrxConfig {
    /// Base URL for the API (e.g. `https://itrx.io/api/v1/frontend`).
    pub base_url: String,
    /// API key obtained from the ITRX dashboard.
    pub api_key: String,
    /// API secret used for HMAC-SHA256 request signing.
    pub api_secret: String,
}

impl ItrxConfig {
    /// Create a config pointing at ITRX **mainnet**.
    pub fn mainnet(api_key: impl Into<String>, api_secret: impl Into<String>) -> Self {
        Self {
            base_url: MAINNET_BASE_URL.to_string(),
            api_key: api_key.into(),
            api_secret: api_secret.into(),
        }
    }

    /// Create a config pointing at the ITRX **Nile testnet** sandbox.
    pub fn testnet(api_key: impl Into<String>, api_secret: impl Into<String>) -> Self {
        Self {
            base_url: TESTNET_BASE_URL.to_string(),
            api_key: api_key.into(),
            api_secret: api_secret.into(),
        }
    }
}

// ─── Client ─────────────────────────────────────────────────────────

/// ITRX API client.
///
/// Provides methods for transferring energy, estimating costs, and
/// querying account balance on the ITRX platform.
#[derive(Debug, Clone)]
pub struct ItrxClient {
    config: ItrxConfig,
    http: Client,
}

impl ItrxClient {
    /// Create a new client with the given configuration.
    ///
    /// Uses a default 10-second request timeout.
    pub fn new(config: ItrxConfig) -> Self {
        let http = Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .build()
            .expect("failed to build HTTP client");

        Self { config, http }
    }

    /// Create a new client that reuses an existing `reqwest::Client`.
    pub fn with_http_client(config: ItrxConfig, http: Client) -> Self {
        Self { config, http }
    }

    // ── Public API ──────────────────────────────────────────────────

    /// Place an energy delegation order.
    ///
    /// This first calls [`estimate_energy_amount`](Self::estimate_energy_amount)
    /// using the `energy_address` to determine the required energy, then places
    /// the order to delegate that energy to `receive_address`.
    ///
    /// # Arguments
    /// * `receive_address` — TRON address that will receive the delegated energy.
    /// * `energy_address` — TRON address used to estimate the energy requirement
    ///   (typically the target of the USDT transfer).
    pub async fn transfer_energy(
        &self,
        receive_address: &str,
        energy_address: &str,
    ) -> Result<CreateOrderResponse, ItrxError> {
        let estimate = self
            .estimate_energy_amount(&EstimateEnergyParams {
                period: "1H",
                energy_amount: None,
                to_address: Some(energy_address),
            })
            .await?;

        let params = CreateOrderParams {
            energy_amount: estimate.energy_amount as u32,
            period: "1H",
            receive_address,
            callback_url: None,
            out_trade_no: Some(receive_address),
        };

        self.create_order(&params).await
    }

    /// Estimate the cost of an energy order.
    ///
    /// # Arguments
    /// * `params` — estimation parameters (period, energy amount, or target address).
    ///
    /// The API will auto-estimate USDT transfer energy if only `to_address` is set.
    pub async fn estimate_energy_amount(
        &self,
        params: &EstimateEnergyParams<'_>,
    ) -> Result<EstimateEnergyResponse, ItrxError> {
        let mut query: Vec<(&str, String)> = vec![("period", params.period.to_string())];

        if let Some(amount) = params.energy_amount {
            query.push(("energy_amount", amount.to_string()));
        }
        if let Some(addr) = params.to_address {
            query.push(("to_address", addr.to_string()));
        }

        let url = format!("{}/order/price", self.config.base_url);

        tracing::debug!(url = %url, ?query, "estimating energy amount");

        let response = self
            .http
            .get(&url)
            .header("API-KEY", &self.config.api_key)
            .query(&query)
            .send()
            .await?;

        Self::handle_get_response(response).await
    }

    /// Query account balance and platform public data.
    pub async fn get_balance(&self) -> Result<BalanceResponse, ItrxError> {
        let url = format!("{}/index-data", self.config.base_url);

        tracing::debug!(url = %url, "querying balance");

        let response = self
            .http
            .get(&url)
            .header("API-KEY", &self.config.api_key)
            .send()
            .await?;

        Self::handle_get_response(response).await
    }

    // ── Internal ────────────────────────────────────────────────────

    /// Place an order via the signed POST endpoint.
    async fn create_order(
        &self,
        params: &CreateOrderParams<'_>,
    ) -> Result<CreateOrderResponse, ItrxError> {
        let url = format!("{}/order", self.config.base_url);

        // Sort keys alphabetically via BTreeMap for deterministic signing.
        let body_value = serde_json::to_value(params)?;
        let sorted: BTreeMap<String, serde_json::Value> = serde_json::from_value(body_value)?;
        let json_body = serde_json::to_string(&sorted)?;

        let timestamp = Utc::now().timestamp().to_string();
        let signature = self.sign(&timestamp, &json_body)?;

        let mut headers = HeaderMap::new();
        headers.insert("API-KEY", HeaderValue::from_str(&self.config.api_key)?);
        headers.insert("TIMESTAMP", HeaderValue::from_str(&timestamp)?);
        headers.insert("SIGNATURE", HeaderValue::from_str(&signature)?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        tracing::debug!(url = %url, "creating order");

        let response = self
            .http
            .post(&url)
            .headers(headers)
            .body(json_body)
            .send()
            .await?;

        Self::handle_post_response(response).await
    }

    /// Generate HMAC-SHA256 signature for a POST request.
    ///
    /// The message format is: `{timestamp}&{json_body}`.
    pub fn sign(&self, timestamp: &str, json_body: &str) -> Result<String, ItrxError> {
        let message = format!("{}&{}", timestamp, json_body);

        let mut mac = HmacSha256::new_from_slice(self.config.api_secret.as_bytes())
            .map_err(|e| ItrxError::Hmac(e.to_string()))?;
        mac.update(message.as_bytes());

        Ok(hex::encode(mac.finalize().into_bytes()))
    }

    /// Parse a GET response (no `errno` envelope — just deserialize directly).
    async fn handle_get_response<T: serde::de::DeserializeOwned>(
        response: reqwest::Response,
    ) -> Result<T, ItrxError> {
        let status = response.status();

        if status == StatusCode::BAD_REQUEST {
            let body = response.text().await.unwrap_or_default();
            return Err(ItrxError::BadRequest(body));
        }

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(ItrxError::UnexpectedStatus {
                status: status.as_u16(),
                body,
            });
        }

        Ok(response.json().await?)
    }

    /// Parse a POST response that uses the `errno` / `message` envelope.
    async fn handle_post_response<T: serde::de::DeserializeOwned>(
        response: reqwest::Response,
    ) -> Result<T, ItrxError> {
        let status = response.status();

        if status == StatusCode::BAD_REQUEST {
            let body = response.text().await.unwrap_or_default();
            return Err(ItrxError::BadRequest(body));
        }

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(ItrxError::UnexpectedStatus {
                status: status.as_u16(),
                body,
            });
        }

        let envelope: ApiEnvelope<T> = response.json().await?;

        if envelope.errno != 0 {
            return Err(ItrxError::ApiError {
                errno: envelope.errno,
                message: envelope.message.unwrap_or_else(|| "unknown error".into()),
            });
        }

        envelope.data.ok_or_else(|| ItrxError::ApiError {
            errno: envelope.errno,
            message: "response contained no data".into(),
        })
    }
}
