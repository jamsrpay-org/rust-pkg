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
    /// Works for both standard Token Program and Token-2022 mints.
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

    /// Get the owner program of an on-chain account.
    ///
    /// Returns the base58-encoded owner program ID, or `None` if the account
    /// doesn't exist.
    ///
    /// This is used to detect whether a mint is owned by the standard Token
    /// Program or the Token-2022 Program.
    pub async fn get_account_owner(
        &self,
        address: &str,
    ) -> Result<Option<String>, SolanaClientError> {
        let result = self
            .rpc_call(
                "getAccountInfo",
                json!([address, {"encoding": "base64"}]),
            )
            .await?;

        let value = result.get("value");

        // Account doesn't exist → null
        if value.is_none() || value == Some(&serde_json::Value::Null) {
            return Ok(None);
        }

        let owner = value
            .and_then(|v| v.get("owner"))
            .and_then(|o| o.as_str())
            .map(|s| s.to_string());

        Ok(owner)
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
