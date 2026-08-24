use chain_core::types::CryptoWallet;

// ── Wallet types ────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct SolanaWallet {
    /// Hex-encoded 32-byte Ed25519 seed (private key).
    pub private_key: String,
    /// Hex-encoded 32-byte Ed25519 public key.
    pub public_key: String,
    /// Base58-encoded address (same as the public key on Solana).
    pub address: String,
}

impl From<SolanaWallet> for CryptoWallet {
    fn from(wallet: SolanaWallet) -> Self {
        CryptoWallet {
            address: wallet.address,
            private_key: wallet.private_key,
        }
    }
}

// ── Transfer types (chain-specific) ─────────────────────────────────────────

/// Prepared transfer — holds the serialized message ready for signing.
#[derive(Debug)]
pub struct SolanaPreparedTransfer {
    /// Serialized transaction message bytes (the data to sign).
    pub message_bytes: Vec<u8>,
    /// Base58-encoded recent blockhash used in the transaction.
    pub recent_blockhash: String,
}

/// Signed transfer — fully serialized transaction, ready to broadcast.
#[derive(Debug)]
pub struct SolanaSignedTransfer {
    /// Fully serialized signed transaction bytes.
    pub signed_transaction_bytes: Vec<u8>,
}
