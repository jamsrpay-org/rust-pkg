use chain_core::wallet::CryptoWallet;
use serde_json::Value;

// ── Wallet types ────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct TronWalletAddress {
    pub base58: String,
    pub hex: String,
}

#[derive(Debug)]
pub struct TronWallet {
    pub private_key: String,
    pub public_key: String,
    pub address: TronWalletAddress,
}

impl From<TronWallet> for CryptoWallet {
    fn from(wallet: TronWallet) -> Self {
        CryptoWallet {
            address: wallet.address.base58,
            private_key: wallet.private_key,
        }
    }
}

// ── Transfer types (chain-specific, shared across all Tron currencies) ──────

/// Opaque prepared transfer — holds everything needed to sign and broadcast.
#[derive(Debug)]
pub struct TronPreparedTransfer {
    /// Raw serialized `raw_data` protobuf bytes (the bytes to sign).
    pub raw_data_bytes: Vec<u8>,
    /// Hex-encoded `raw_data` (for bandwidth estimation and broadcast).
    pub raw_data_hex: String,
    /// Transaction ID assigned by the Tron node.
    pub tx_id: String,
    /// Serialized JSON `raw_data` — required for broadcast payload.
    pub raw_data_json: Value,
}

/// Signed transfer — prepared transfer with collected signatures, ready to broadcast.
#[derive(Debug)]
pub struct TronSignedTransfer {
    pub prepared: TronPreparedTransfer,
    pub signatures: Vec<Vec<u8>>,
}
