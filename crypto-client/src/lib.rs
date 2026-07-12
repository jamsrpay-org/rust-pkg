// ── chain-core re-exports ───────────────────────────────────────────────────
pub use chain_core::error::BlockchainClientError;
pub use chain_core::types::{
    Address, BlockchainClient, BroadcastResult, CryptoWallet, EstimateWithdrawableRequest,
    TransactionId, TransferRequest,
};

// ── evm-core re-exports ─────────────────────────────────────────────────────
pub use evm_core::client::EvmClient;
pub use evm_core::types::{EvmPreparedTransfer, EvmSignedTransfer, EvmWallet};

// ── tron re-exports ─────────────────────────────────────────────────────────
pub use tron::client::TronClient;
pub use tron::contracts as tron_contracts;
pub use tron::types::{TronPreparedTransfer, TronSignedTransfer};

// ── bsc re-exports ──────────────────────────────────────────────────────────
pub use binance_smart_chain::client::BscClient;
pub use binance_smart_chain::contracts as bsc_contracts;

// ── ethereum re-exports ─────────────────────────────────────────────────────
pub use ethereum::client::EthClient;
pub use ethereum::contracts as eth_contracts;

// ── polygon re-exports ──────────────────────────────────────────────────────
pub use polygon::client::PolygonClient;
pub use polygon::contracts as polygon_contracts;
