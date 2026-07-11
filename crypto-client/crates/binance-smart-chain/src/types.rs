// ── Wallet types ────────────────────────────────────────────────────────────

use chain_core::types::CryptoWallet;

#[derive(Debug)]
pub struct BscWallet {
    pub private_key: String,
    pub public_key: String,
    /// 0x-prefixed, lowercase hex address.
    pub address: String,
}

impl From<BscWallet> for CryptoWallet {
    fn from(wallet: BscWallet) -> Self {
        CryptoWallet {
            address: wallet.address,
            private_key: wallet.private_key,
        }
    }
}

// ── Transfer types (chain-specific, shared across all BSC currencies) ───────

/// Opaque prepared transfer — holds everything needed to sign and broadcast.
#[derive(Debug)]
pub struct BscPreparedTransfer {
    /// RLP-encoded unsigned transaction bytes (the bytes to sign).
    pub unsigned_tx_bytes: Vec<u8>,
    /// Nonce used for this transaction.
    pub nonce: u64,
    /// Gas price in wei.
    pub gas_price: u128,
    /// Gas limit.
    pub gas_limit: u64,
    /// Destination address (0x-prefixed hex).
    pub to: String,
    /// Value in wei (for native BNB transfers, 0 for BEP20).
    pub value: u128,
    /// Calldata (empty for native transfers, ABI-encoded for BEP20).
    pub data: Vec<u8>,
    /// Chain ID (56 for mainnet, 97 for testnet).
    pub chain_id: u64,
}

/// Signed transfer — prepared transfer with RLP-encoded signed tx, ready to broadcast.
#[derive(Debug)]
pub struct BscSignedTransfer {
    pub prepared: BscPreparedTransfer,
    /// RLP-encoded signed transaction (ready for `eth_sendRawTransaction`).
    pub signed_tx_bytes: Vec<u8>,
}
