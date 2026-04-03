use crate::{client::TronClient, error::TronClientError};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
struct CreateTransactionRequest {
    owner_address: String,
    to_address: String,
    amount: u128,
    visible: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub raw_data_hex: String,
    #[serde(rename = "txID")]
    pub tx_id: String,
    #[serde(default)]
    pub signature: Vec<String>,
    pub raw_data: Value,
}

#[derive(Debug, Serialize)]
pub struct BroadcastTransactionRequest {
    pub raw_data: Value,
    pub raw_data_hex: String,
    pub signature: Vec<String>,
    pub visible: bool,
}

#[derive(Debug, Deserialize)]
pub struct BroadcastTransactionResponse {
    #[serde(rename = "txid")]
    pub tx_id: String,
}

impl TronClient {
    pub async fn create_transaction(
        &self,
        from: &str,
        to: &str,
        amount: u128,
    ) -> Result<Transaction, TronClientError> {
        let url = format!("{}/createtransaction", self.http_base_url);

        let body = CreateTransactionRequest {
            owner_address: from.to_string(),
            to_address: to.to_string(),
            amount,
            visible: true,
        };
        let resp = self.client.post(&url).json(&body).send().await?;
        let json: Value = resp.json().await?;

        if let Some(err) = json.get("Error") {
            return Err(TronClientError::ApiError(err.as_str().unwrap().to_string()));
        }
        let tx: Transaction = serde_json::from_value(json)?;
        Ok(tx)
    }

    pub async fn broadcast_transaction(
        &self,
        raw_tx: &[u8],
        signatures: &[Vec<u8>],
        raw_data: &Value,
    ) -> Result<BroadcastTransactionResponse, TronClientError> {
        let url = format!("{}/broadcasttransaction", self.http_base_url);

        let raw_data_hex = hex::encode(raw_tx);

        let sigs_hex: Vec<String> = signatures.iter().map(|s| hex::encode(s)).collect();

        let body = BroadcastTransactionRequest {
            raw_data: raw_data.clone(),
            raw_data_hex,
            signature: sigs_hex,
            visible: true,
        };

        let resp = self.client.post(&url).json(&body).send().await?;
        let json: Value = resp.json().await?;

        // Check for Tron error responses: {"code": "CONTRACT_VALIDATE_ERROR", "message": "..."}
        if let Some(code) = json.get("code") {
            let code_str = code.as_str().unwrap_or("UNKNOWN");
            let message = json
                .get("message")
                .and_then(|m| m.as_str())
                .map(|hex_msg| {
                    // Tron returns hex-encoded error messages
                    hex::decode(hex_msg)
                        .ok()
                        .and_then(|bytes| String::from_utf8(bytes).ok())
                        .unwrap_or_else(|| hex_msg.to_string())
                })
                .unwrap_or_default();
            return Err(TronClientError::ApiError(format!(
                "{}: {}",
                code_str, message
            )));
        }

        if let Some(err) = json.get("Error") {
            return Err(TronClientError::ApiError(err.to_string()));
        }

        let resp: BroadcastTransactionResponse = serde_json::from_value(json)?;
        Ok(resp)
    }
}
