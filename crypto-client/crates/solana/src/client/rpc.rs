use crate::error::SolanaClientError;
use serde_json::{json, Value};

impl super::SolanaClient {
    /// Make a JSON-RPC 2.0 call to the Solana node.
    pub(crate) async fn rpc_call(
        &self,
        method: &str,
        params: Value,
    ) -> Result<Value, SolanaClientError> {
        let body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params,
        });

        let resp = self.client.post(&self.rpc_url).json(&body).send().await?;
        let json: Value = resp.json().await?;

        if let Some(error) = json.get("error") {
            let message = error
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("unknown error");
            return Err(SolanaClientError::RpcError(message.to_string()));
        }

        json.get("result")
            .cloned()
            .ok_or_else(|| SolanaClientError::RpcError("missing result in response".to_string()))
    }

    /// Broadcast a signed transaction via `sendTransaction`.
    ///
    /// Returns the transaction signature (base58-encoded) as the transaction ID.
    pub(crate) async fn send_transaction(
        &self,
        signed_tx: &[u8],
    ) -> Result<String, SolanaClientError> {
        let encoded = bs58::encode(signed_tx).into_string();
        let result = self
            .rpc_call("sendTransaction", json!([encoded, {"encoding": "base58"}]))
            .await?;
        result
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| {
                SolanaClientError::RpcError("invalid sendTransaction response".to_string())
            })
    }
}
