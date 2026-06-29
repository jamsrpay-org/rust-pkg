use crate::error::TronClientError;

/// Convert a Tron base58 address to a 32-byte ABI-encoded parameter (64 hex chars).
///
/// Tron base58check → 21 bytes (0x41 prefix + 20-byte address) → strip prefix → left-pad to 32 bytes.
pub(crate) fn address_to_parameter(address: &str) -> Result<String, TronClientError> {
    let bytes = bs58::decode(address)
        .with_check(None)
        .into_vec()
        .map_err(|_| TronClientError::ApiError(format!("invalid base58 address: {}", address)))?;

    if bytes.len() != 21 || bytes[0] != 0x41 {
        return Err(TronClientError::ApiError(format!(
            "invalid Tron address format: {}",
            address
        )));
    }

    // 20-byte address → hex → left-pad to 64 chars (32 bytes)
    let addr_hex = hex::encode(&bytes[1..]);
    Ok(format!("{:0>64}", addr_hex))
}

/// Encode a u128 amount as a uint256 ABI parameter (64 hex chars).
pub(crate) fn uint256_to_parameter(value: u128) -> String {
    format!("{:064x}", value)
}
