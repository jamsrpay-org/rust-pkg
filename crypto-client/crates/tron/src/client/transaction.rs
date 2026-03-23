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
}

#[derive(Debug, Deserialize)]
pub struct BroadcastTransactionResponse {
    // pub result: bool,
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
            amount: amount,
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
            raw_data_hex: raw_data_hex,
            signature: sigs_hex,
        };

        let resp = self.client.post(&url).json(&body).send().await?;
        let json: Value = resp.json().await?;

        if let Some(err) = json.get("Error") {
            return Err(TronClientError::ApiError(err.to_string()));
        }

        let resp: BroadcastTransactionResponse = serde_json::from_value(json)?;
        Ok(resp)
    }
}
