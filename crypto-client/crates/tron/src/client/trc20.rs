use crate::client::TronClient;
use crate::client::transaction::Transaction;
use crate::error::TronClientError;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Default fee limit for TRC20 transfers (100 TRX in SUN).
const DEFAULT_TRC20_FEE_LIMIT: u64 = 100_000_000;

// ── Request / Response types ────────────────────────────────────────────────

#[derive(Debug, Serialize)]
struct TriggerSmartContractRequest {
    owner_address: String,
    contract_address: String,
    function_selector: String,
    parameter: String,
    fee_limit: u64,
    visible: bool,
}

#[derive(Debug, Deserialize)]
struct TriggerResult {
    result: bool,
    #[serde(default)]
    message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TriggerSmartContractResponse {
    result: TriggerResult,
    transaction: Transaction,
}

#[derive(Debug, Serialize)]
struct TriggerConstantContractRequest {
    owner_address: String,
    contract_address: String,
    function_selector: String,
    parameter: String,
    visible: bool,
}

#[derive(Debug, Deserialize)]
struct TriggerConstantContractResponse {
    result: TriggerResult,
    #[serde(default)]
    constant_result: Vec<String>,
}

// ── TRC20 methods on TronClient ─────────────────────────────────────────────

impl TronClient {
    /// Create a TRC20 `transfer(address,uint256)` transaction via `triggersmartcontract`.
    pub(crate) async fn create_trc20_transfer(
        &self,
        from: &str,
        to: &str,
        amount: u128,
        contract_address: &str,
    ) -> Result<Transaction, TronClientError> {
        let url = format!("{}/triggersmartcontract", self.base_url());

        let parameter = format!(
            "{}{}",
            super::abi::address_to_parameter(to)?,
            super::abi::uint256_to_parameter(amount),
        );

        let body = TriggerSmartContractRequest {
            owner_address: from.to_string(),
            contract_address: contract_address.to_string(),
            function_selector: "transfer(address,uint256)".to_string(),
            parameter,
            fee_limit: DEFAULT_TRC20_FEE_LIMIT,
            visible: true,
        };

        let resp = self.client().post(&url).json(&body).send().await?;
        let json: Value = resp.json().await?;

        if let Some(err) = json.get("Error") {
            return Err(TronClientError::ApiError(err.to_string()));
        }

        let response: TriggerSmartContractResponse = serde_json::from_value(json)?;

        if !response.result.result {
            let msg = decode_tron_error_message(response.result.message);
            return Err(TronClientError::ApiError(msg));
        }

        Ok(response.transaction)
    }

    /// Query a TRC20 `balanceOf(address)` via `triggerconstantcontract`.
    pub(crate) async fn get_trc20_balance(
        &self,
        address: &str,
        contract_address: &str,
    ) -> Result<u128, TronClientError> {
        let url = format!("{}/triggerconstantcontract", self.base_url());

        let parameter = super::abi::address_to_parameter(address)?;

        let body = TriggerConstantContractRequest {
            owner_address: address.to_string(),
            contract_address: contract_address.to_string(),
            function_selector: "balanceOf(address)".to_string(),
            parameter,
            visible: true,
        };

        let resp = self.client().post(&url).json(&body).send().await?;
        let json: Value = resp.json().await?;

        if let Some(err) = json.get("Error") {
            return Err(TronClientError::ApiError(err.to_string()));
        }

        let response: TriggerConstantContractResponse = serde_json::from_value(json)?;

        if !response.result.result {
            let msg = decode_tron_error_message(response.result.message);
            return Err(TronClientError::ApiError(msg));
        }

        let hex_value = response
            .constant_result
            .first()
            .ok_or_else(|| TronClientError::ApiError("empty constant_result".into()))?;

        parse_uint256_hex(hex_value)
    }
}

// ── Helpers ─────────────────────────────────────────────────────────────────

/// Decode a Tron hex-encoded error message, falling back to the raw string.
fn decode_tron_error_message(message: Option<String>) -> String {
    message
        .map(|hex_msg| {
            hex::decode(&hex_msg)
                .ok()
                .and_then(|bytes| String::from_utf8(bytes).ok())
                .unwrap_or(hex_msg)
        })
        .unwrap_or_else(|| "unknown error".to_string())
}

/// Parse a hex-encoded uint256 string to u128.
fn parse_uint256_hex(hex_str: &str) -> Result<u128, TronClientError> {
    let trimmed = hex_str.trim_start_matches('0');
    if trimmed.is_empty() {
        return Ok(0);
    }
    u128::from_str_radix(trimmed, 16)
        .map_err(|e| TronClientError::ApiError(format!("invalid balance hex: {}", e)))
}
