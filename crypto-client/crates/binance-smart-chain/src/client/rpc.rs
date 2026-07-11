use crate::client::BscClient;
use crate::error::BscClientError;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Generic JSON-RPC request body.
#[derive(Debug, Serialize)]
struct JsonRpcRequest {
    jsonrpc: &'static str,
    method: String,
    params: Value,
    id: u64,
}

/// Generic JSON-RPC response.
#[derive(Debug, Deserialize)]
struct JsonRpcResponse {
    #[serde(default)]
    result: Option<Value>,
    #[serde(default)]
    error: Option<JsonRpcError>,
}

#[derive(Debug, Deserialize)]
struct JsonRpcError {
    #[serde(default)]
    code: i64,
    #[serde(default)]
    message: String,
}

impl BscClient {
    /// Send a JSON-RPC request and return the `result` field.
    async fn rpc_call(
        &self,
        method: &str,
        params: Value,
    ) -> Result<Value, BscClientError> {
        let body = JsonRpcRequest {
            jsonrpc: "2.0",
            method: method.to_string(),
            params,
            id: 1,
        };

        let resp = self.client().post(self.base_url()).json(&body).send().await?;
        let json: JsonRpcResponse = resp.json().await?;

        if let Some(err) = json.error {
            return Err(BscClientError::RpcError(format!(
                "code {}: {}",
                err.code, err.message
            )));
        }

        json.result
            .ok_or_else(|| BscClientError::RpcError("empty result".into()))
    }

    /// `eth_getBalance` — returns the BNB balance in wei.
    pub(crate) async fn eth_get_balance(&self, address: &str) -> Result<u128, BscClientError> {
        let result = self
            .rpc_call(
                "eth_getBalance",
                serde_json::json!([address, "latest"]),
            )
            .await?;
        parse_hex_u128(&result)
    }

    /// `eth_getTransactionCount` — returns the account nonce.
    pub(crate) async fn eth_get_transaction_count(
        &self,
        address: &str,
    ) -> Result<u64, BscClientError> {
        let result = self
            .rpc_call(
                "eth_getTransactionCount",
                serde_json::json!([address, "latest"]),
            )
            .await?;
        let val = parse_hex_u128(&result)?;
        Ok(val as u64)
    }

    /// `eth_gasPrice` — returns the current gas price in wei.
    pub(crate) async fn eth_gas_price(&self) -> Result<u128, BscClientError> {
        let result = self
            .rpc_call("eth_gasPrice", serde_json::json!([]))
            .await?;
        parse_hex_u128(&result)
    }

    /// `eth_estimateGas` — returns the estimated gas for a transaction.
    pub(crate) async fn eth_estimate_gas(
        &self,
        from: &str,
        to: &str,
        data: Option<&str>,
        value: Option<&str>,
    ) -> Result<u64, BscClientError> {
        let mut tx = serde_json::json!({
            "from": from,
            "to": to,
        });

        if let Some(d) = data {
            tx["data"] = Value::String(d.to_string());
        }
        if let Some(v) = value {
            tx["value"] = Value::String(v.to_string());
        }

        let result = self
            .rpc_call("eth_estimateGas", serde_json::json!([tx]))
            .await?;
        let val = parse_hex_u128(&result)?;
        Ok(val as u64)
    }

    /// `eth_call` — execute a read-only smart contract call.
    pub(crate) async fn eth_call(
        &self,
        to: &str,
        data: &str,
    ) -> Result<String, BscClientError> {
        let tx = serde_json::json!({
            "to": to,
            "data": data,
        });

        let result = self
            .rpc_call("eth_call", serde_json::json!([tx, "latest"]))
            .await?;

        result
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| BscClientError::RpcError("eth_call returned non-string".into()))
    }

    /// `eth_sendRawTransaction` — broadcast a signed transaction.
    pub(crate) async fn eth_send_raw_transaction(
        &self,
        raw_tx_hex: &str,
    ) -> Result<String, BscClientError> {
        let result = self
            .rpc_call(
                "eth_sendRawTransaction",
                serde_json::json!([raw_tx_hex]),
            )
            .await?;

        result
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| {
                BscClientError::RpcError("eth_sendRawTransaction returned non-string".into())
            })
    }
}

// ── Helpers ─────────────────────────────────────────────────────────────────

/// Parse a hex-encoded quantity string (e.g. "0x1a2b") to u128.
fn parse_hex_u128(value: &Value) -> Result<u128, BscClientError> {
    let hex_str = value
        .as_str()
        .ok_or_else(|| BscClientError::RpcError("expected hex string".into()))?;

    let stripped = hex_str.strip_prefix("0x").unwrap_or(hex_str);
    if stripped.is_empty() {
        return Ok(0);
    }
    u128::from_str_radix(stripped, 16)
        .map_err(|e| BscClientError::RpcError(format!("invalid hex value: {}", e)))
}
