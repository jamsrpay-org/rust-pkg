use crate::client::{TronClient, account::AccountResource};

/// Estimated overhead added by signature (67 bytes) and result (64 bytes)
/// Signature: 65 bytes data + 2 bytes protobuf tag/length = 67
/// Result: fixed 64 bytes
const SIGNATURE_OVERHEAD: u128 = 67;
const RESULT_OVERHEAD: u128 = 64;

/// Bandwidth unit cost in SUN (1 bandwidth point = 1000 SUN)
const BANDWIDTH_UNIT_COST_SUN: u128 = 1000;

impl TronClient {
    /// Get the total available bandwidth for an account
    ///
    /// Available = (freeNetLimit - freeNetUsed) + (netLimit - netUsed)
    pub fn get_available_bandwidth(resource: &AccountResource) -> u128 {
        let free_available = resource.free_net_limit - resource.free_net_used;
        let staked_available = resource.net_limit - resource.net_used;
        free_available + staked_available
    }

    /// Estimate the bandwidth consumption for a transaction
    ///
    /// Formula: len(raw_data_bytes) + 67 (signature) + 64 (result)
    pub fn estimate_bandwidth(raw_data_hex: &str) -> u128 {
        let raw_data_len = raw_data_hex.len() as u128 / 2; // hex string → byte count
        raw_data_len + SIGNATURE_OVERHEAD + RESULT_OVERHEAD
    }

    /// Calculate the TRX fee (in SUN) required to cover missing bandwidth
    pub fn calculate_bandwidth_fee(bandwidth: u128) -> u128 {
        bandwidth * BANDWIDTH_UNIT_COST_SUN
    }
}
