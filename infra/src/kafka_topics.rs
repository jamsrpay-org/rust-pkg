pub const INVOICE_EVENTS: &str = "billing.invoice.events.v1";
pub const PAYMENT_INTENT_EVENTS: &str = "billing.payment_intent.events.v1";
pub const DEPOSIT_WALLET_EVENTS: &str = "billing.deposit_wallet.events.v1";

pub const USER_EVENTS: &str = "user.events.v1";

pub const IDENTITY_EVENTS: &str = "identity.events.v1";

pub const STORE_EVENTS: &str = "store.store.events.v1";
pub const API_KEY_EVENTS: &str = "store.api_key.events.v1";
pub const STORE_CURRENCY_EVENTS: &str = "store.store_currency.events.v1";

pub const PAYOUT_EVENTS: &str = "payout.events.v1";
pub const PAYOUT_WALLET_EVENTS: &str = "payout.payout_wallet.events.v1";
pub const GAS_WALLET_EVENTS: &str = "payout.gas_wallet.events.v1";

// ----

pub const TRON_NATIVE_TRANSFERS: &str = "tron.transfers.native.v1";
pub const TRON_TOKEN_TRANSFERS: &str = "tron.transfers.token.v1";
pub const TRON_BLOCKS: &str = "tron.blocks.v1";

pub const BSC_NATIVE_TRANSFERS: &str = "bsc.transfers.native.v1";
pub const BSC_TOKEN_TRANSFERS: &str = "bsc.transfers.token.v1";
pub const BSC_BLOCKS: &str = "bsc.blocks.v1";

pub const ETH_NATIVE_TRANSFERS: &str = "eth.transfers.native.v1";
pub const ETH_TOKEN_TRANSFERS: &str = "eth.transfers.token.v1";
pub const ETH_BLOCKS: &str = "eth.blocks.v1";

pub const POL_NATIVE_TRANSFERS: &str = "pol.transfers.native.v1";
pub const POL_TOKEN_TRANSFERS: &str = "pol.transfers.token.v1";
pub const POL_BLOCKS: &str = "pol.blocks.v1";

pub const BTC_NATIVE_TRANSFERS: &str = "btc.transfers.native.v1";
pub const BTC_TOKEN_TRANSFERS: &str = "btc.transfers.token.v1";
pub const BTC_BLOCKS: &str = "btc.blocks.v1";

pub const LTC_NATIVE_TRANSFERS: &str = "ltc.transfers.native.v1";
pub const LTC_TOKEN_TRANSFERS: &str = "ltc.transfers.token.v1";
pub const LTC_BLOCKS: &str = "ltc.blocks.v1";

pub const SOL_NATIVE_TRANSFERS: &str = "sol.transfers.native.v1";
pub const SOL_TOKEN_TRANSFERS: &str = "sol.transfers.token.v1";
pub const SOL_BLOCKS: &str = "sol.blocks.v1";

// Blockchain events (from blockchain service)
pub const BLOCKCHAIN_INVOICE_PAYMENT_DETECTED: &str = "blockchain.invoice.payment_detected.v1";
pub const BLOCKCHAIN_INVOICE_PAYMENT_CONFIRMED: &str = "blockchain.invoice.payment_confirmed.v1";
pub const BLOCKCHAIN_GAS_WALLET_TRANSACTION_DETECTED: &str =
    "blockchain.gas_wallet.transaction_detected.v1";
pub const BLOCKCHAIN_PAYOUT_TRANSACTION_DETECTED: &str =
    "blockchain.payout.transaction_detected.v1";
pub const BLOCKCHAIN_PAYOUT_NATIVE_TRANSFER_DETECTED: &str =
    "blockchain.payout.native_transfer_detected.v1";
pub const BLOCKCHAIN_PAYOUT_TRANSACTION_STATUS_CHANGED: &str =
    "blockchain.payout.transaction_status_changed.v1";
