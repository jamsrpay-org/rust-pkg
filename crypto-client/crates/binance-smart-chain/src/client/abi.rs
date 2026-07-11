use crate::error::BscClientError;

/// Strip the `0x` prefix from a hex address and left-pad to 32 bytes (64 hex chars).
pub(crate) fn address_to_parameter(address: &str) -> Result<String, BscClientError> {
    let stripped = address.strip_prefix("0x").unwrap_or(address);

    if stripped.len() != 40 {
        return Err(BscClientError::RpcError(format!(
            "invalid EVM address length: {}",
            address
        )));
    }

    // Validate hex characters
    if !stripped.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(BscClientError::RpcError(format!(
            "invalid hex in address: {}",
            address
        )));
    }

    // 20-byte address hex → left-pad to 64 chars (32 bytes)
    Ok(format!("{:0>64}", stripped.to_lowercase()))
}

/// Encode a u128 amount as a uint256 ABI parameter (64 hex chars).
pub(crate) fn uint256_to_parameter(value: u128) -> String {
    format!("{:064x}", value)
}

/// Build the 4-byte function selector from a function signature.
///
/// e.g. `"transfer(address,uint256)"` → first 4 bytes of keccak256.
pub(crate) fn function_selector(signature: &str) -> Vec<u8> {
    use sha3::{Digest, Keccak256};
    let hash = Keccak256::digest(signature.as_bytes());
    hash[..4].to_vec()
}

/// Encode a full `transfer(address,uint256)` calldata.
pub(crate) fn encode_transfer(to: &str, amount: u128) -> Result<Vec<u8>, BscClientError> {
    let selector = function_selector("transfer(address,uint256)");
    let addr_param = address_to_parameter(to)?;
    let amount_param = uint256_to_parameter(amount);

    let mut data = selector;
    data.extend_from_slice(
        &hex::decode(format!("{}{}", addr_param, amount_param))
            .map_err(|e| BscClientError::RpcError(e.to_string()))?,
    );
    Ok(data)
}

/// Encode a `balanceOf(address)` calldata.
pub(crate) fn encode_balance_of(address: &str) -> Result<Vec<u8>, BscClientError> {
    let selector = function_selector("balanceOf(address)");
    let addr_param = address_to_parameter(address)?;

    let mut data = selector;
    data.extend_from_slice(
        &hex::decode(addr_param).map_err(|e| BscClientError::RpcError(e.to_string()))?,
    );
    Ok(data)
}
