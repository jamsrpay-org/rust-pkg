use crate::error::SolanaClientError;
use serde_json::json;

impl super::SolanaClient {
    /// Get native SOL balance in lamports.
    pub async fn get_balance(&self, address: &str) -> Result<u64, SolanaClientError> {
        let result = self.rpc_call("getBalance", json!([address])).await?;
        result
            .get("value")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| SolanaClientError::RpcError("invalid balance response".to_string()))
    }

    /// Get SPL token balance for a specific mint address.
    ///
    /// Returns the raw token amount (smallest unit).
    pub async fn get_spl_token_balance(
        &self,
        owner: &str,
        mint: &str,
    ) -> Result<u64, SolanaClientError> {
        let result = self
            .rpc_call(
                "getTokenAccountsByOwner",
                json!([
                    owner,
                    {"mint": mint},
                    {"encoding": "jsonParsed"}
                ]),
            )
            .await?;

        let accounts = result
            .get("value")
            .and_then(|v| v.as_array())
            .ok_or_else(|| {
                SolanaClientError::RpcError("invalid token accounts response".to_string())
            })?;

        if accounts.is_empty() {
            return Ok(0);
        }

        // Extract balance from the first token account for this mint
        let amount_str = accounts[0]
            .get("account")
            .and_then(|a| a.get("data"))
            .and_then(|d| d.get("parsed"))
            .and_then(|p| p.get("info"))
            .and_then(|i| i.get("tokenAmount"))
            .and_then(|t| t.get("amount"))
            .and_then(|a| a.as_str())
            .ok_or_else(|| {
                SolanaClientError::RpcError("invalid token balance response".to_string())
            })?;

        amount_str
            .parse::<u64>()
            .map_err(|e| SolanaClientError::RpcError(format!("invalid token amount: {}", e)))
    }

    /// Get the latest blockhash and last valid block height.
    pub async fn get_latest_blockhash(&self) -> Result<(String, u64), SolanaClientError> {
        let result = self.rpc_call("getLatestBlockhash", json!([])).await?;

        let blockhash = result
            .get("value")
            .and_then(|v| v.get("blockhash"))
            .and_then(|b| b.as_str())
            .ok_or_else(|| SolanaClientError::RpcError("invalid blockhash response".to_string()))?
            .to_string();

        let last_valid_block_height = result
            .get("value")
            .and_then(|v| v.get("lastValidBlockHeight"))
            .and_then(|h| h.as_u64())
            .unwrap_or(0);

        Ok((blockhash, last_valid_block_height))
    }
}

#[cfg(test)]
mod tests {
    use crate::client::SolanaClient;

    #[tokio::test]
    async fn get_balance_devnet() {
        let client = SolanaClient::new("https://api.devnet.solana.com");
        // Query a known address on devnet (system program, always exists)
        let balance = client.get_balance("11111111111111111111111111111111").await;
        // Should succeed (even if balance is 0)
        assert!(balance.is_ok(), "Failed to query balance: {:?}", balance);
    }
}
