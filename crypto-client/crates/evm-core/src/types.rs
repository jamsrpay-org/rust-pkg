use chain_core::types::CryptoWallet;

// ── Transfer types (shared across all EVM chains) ───────────────────────────

/// Prepared unsigned EVM transfer — holds everything needed to sign and broadcast.
///
/// Uses EIP-1559 (Type 2) fee model with `max_fee_per_gas` and
/// `max_priority_fee_per_gas` instead of the legacy `gas_price`.
#[derive(Debug, Clone)]
pub struct EvmPreparedTransfer {
    /// Nonce used for this transaction.
    pub nonce: u64,
    /// Maximum total fee per gas unit the sender is willing to pay (in wei).
    /// Includes both the base fee and the priority fee.
    pub max_fee_per_gas: u128,
    /// Maximum priority fee (tip) per gas unit for the miner (in wei).
    pub max_priority_fee_per_gas: u128,
    /// Gas limit.
    pub gas_limit: u64,
    /// Destination address (0x-prefixed hex).
    pub to: String,
    /// Value in wei (for native transfers, 0 for ERC20).
    pub value: u128,
    /// Calldata (empty for native transfers, ABI-encoded for ERC20).
    pub data: Vec<u8>,
    /// Chain ID (e.g. 56 for BSC mainnet, 1 for Ethereum mainnet).
    pub chain_id: u64,
}

/// Signed EVM transfer — prepared transfer with signed raw bytes, ready to broadcast.
#[derive(Debug)]
pub struct EvmSignedTransfer {
    pub prepared: EvmPreparedTransfer,
    /// Full RLP-encoded signed transaction (ready for `eth_sendRawTransaction`).
    pub signed_tx_bytes: Vec<u8>,
}

/// An EVM wallet — private key + address generated from secp256k1.
#[derive(Debug)]
pub struct EvmWallet {
    pub private_key: String,
    pub address: String,
}

impl From<EvmWallet> for CryptoWallet {
    fn from(wallet: EvmWallet) -> Self {
        CryptoWallet {
            address: wallet.address,
            private_key: wallet.private_key,
        }
    }
}
