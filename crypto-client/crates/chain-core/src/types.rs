use crate::error::BlockchainClientError;
use jamsrpay_types::{
    crypto_address::CryptoAddress, crypto_transaction_hash::CryptoTransactionHash,
    currency::PaymentCurrency, money::Money,
};

/// Re-export as `Address` for a cleaner blockchain-oriented API.
pub type Address = CryptoAddress;

/// Re-export as `TransactionId` — blockchain tx hashes are strings, not UUIDs.
pub type TransactionId = CryptoTransactionHash;

// ── Request / Result types ──────────────────────────────────────────────────

pub struct TransferRequest {
    pub currency: PaymentCurrency,
    pub from: Address,
    pub to: Address,
    pub amount: Money,
}

pub struct EstimateWithdrawableRequest {
    pub currency: PaymentCurrency,
    pub from: Address,
    pub to: Address,
}

#[derive(Debug)]
pub struct BroadcastResult {
    pub txid: TransactionId,
}

#[derive(Debug)]
pub struct CryptoWallet {
    pub private_key: String,
    pub address: String,
}

// ── Trait ────────────────────────────────────────────────────────────────────

pub trait BlockchainClient {
    type PreparedTransfer;
    type SignedTransfer;

    fn prepare_transfer(
        &self,
        request: TransferRequest,
    ) -> impl Future<Output = Result<Self::PreparedTransfer, BlockchainClientError>>;

    fn broadcast(
        &self,
        signed: Self::SignedTransfer,
    ) -> impl Future<Output = Result<BroadcastResult, BlockchainClientError>>;

    fn balance(
        &self,
        address: Address,
        currency: PaymentCurrency,
    ) -> impl Future<Output = Result<Money, BlockchainClientError>>;

    fn estimate_withdrawable(
        &self,
        request: EstimateWithdrawableRequest,
    ) -> impl Future<Output = Result<Money, BlockchainClientError>>;

    fn generate_wallet(&self) -> Result<CryptoWallet, BlockchainClientError>;

    /// Sign raw transaction bytes with a private key.
    ///
    /// Returns chain-specific signed output:
    /// - Tron: 65-byte signature (r ‖ s ‖ v)
    /// - BSC: full RLP-encoded signed transaction bytes
    fn sign_transaction(
        &self,
        private_key: &[u8],
        raw_tx: &[u8],
    ) -> Result<Vec<u8>, BlockchainClientError>;
}
