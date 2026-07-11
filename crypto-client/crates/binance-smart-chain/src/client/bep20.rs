use crate::client::BscClient;
use crate::error::BscClientError;

impl BscClient {
    /// Query a BEP20 `balanceOf(address)` via `eth_call`.
    pub(crate) async fn get_bep20_balance(
        &self,
        address: &str,
        contract_address: &str,
    ) -> Result<u128, BscClientError> {
        let data = super::abi::encode_balance_of(address)?;
        let data_hex = format!("0x{}", hex::encode(&data));

        let result_hex = self.eth_call(contract_address, &data_hex).await?;
        parse_uint256_hex(&result_hex)
    }
}

// ── Helpers ─────────────────────────────────────────────────────────────────

/// Parse a hex-encoded uint256 string to u128.
fn parse_uint256_hex(hex_str: &str) -> Result<u128, BscClientError> {
    let stripped = hex_str.strip_prefix("0x").unwrap_or(hex_str);
    let trimmed = stripped.trim_start_matches('0');
    if trimmed.is_empty() {
        return Ok(0);
    }
    u128::from_str_radix(trimmed, 16)
        .map_err(|e| BscClientError::RpcError(format!("invalid balance hex: {}", e)))
}
