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

        if let Some(err) = json.get("error") {
            return Err(TronClientError::ApiError(err.as_str().unwrap().to_string()));
        }
        let tx: Transaction = serde_json::from_value(json)?;
        Ok(tx)
    }

    pub async fn broadcast_transaction(&self, tx: &str) -> Result<String, String> {
        Ok("tx_hash".to_string())
    }
}
