use crate::types::{SolanaPreparedTransfer, SolanaSignedTransfer, SolanaWallet};
use chain_core::types::CryptoWallet;
use chain_core::{
    error::BlockchainClientError,
    types::{
        Address, BlockchainClient, BroadcastResult, EstimateWithdrawableRequest, TransactionId,
        TransferRequest,
    },
};
use jamsrpay_types::{currency::PaymentCurrency, money::Money};
use reqwest::ClientBuilder;
use std::collections::HashMap;
use std::time::Duration;

mod account;
mod rpc;
pub mod sign;
mod spl;
pub(crate) mod transaction;

/// Solana JSON-RPC client.
///
/// Supports:
/// - Native SOL transfers
/// - SPL token transfers (via registered mint addresses)
///
/// ```no_run
/// use solana::client::SolanaClient;
/// use solana::contracts;
///
/// let client = SolanaClient::new("https://api.mainnet-beta.solana.com");
/// ```
pub struct SolanaClient {
    rpc_url: String,
    client: reqwest::Client,
    spl_mints: HashMap<PaymentCurrency, String>,
}

impl SolanaClient {
    /// Create a new Solana client.
    ///
    /// - `rpc_url`: Solana JSON-RPC endpoint URL.
    pub fn new(rpc_url: impl Into<String>) -> Self {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();
        Self {
            rpc_url: rpc_url.into(),
            client,
            spl_mints: HashMap::new(),
        }
    }

    /// Register an SPL token mint address for a given currency.
    ///
    /// ```no_run
    /// use solana::client::SolanaClient;
    /// use solana::contracts;
    /// use jamsrpay_types::currency::PaymentCurrency;
    ///
    /// let client = SolanaClient::new("https://api.mainnet-beta.solana.com")
    ///     .register_spl(PaymentCurrency::USDT, contracts::mainnet::USDT_SPL);
    /// ```
    pub fn register_spl(
        mut self,
        currency: PaymentCurrency,
        mint_address: impl Into<String>,
    ) -> Self {
        self.spl_mints.insert(currency, mint_address.into());
        self
    }

    pub(crate) fn mint_address(
        &self,
        currency: PaymentCurrency,
    ) -> Result<&str, BlockchainClientError> {
        self.spl_mints
            .get(&currency)
            .map(|s| s.as_str())
            .ok_or_else(|| {
                BlockchainClientError::Unknown(format!(
                    "no SPL mint registered for {}",
                    currency
                ))
            })
    }

    pub fn rpc_url(&self) -> &str {
        &self.rpc_url
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// NOTE: Update this when `PaymentCurrency::SOL` is added to jamsrpay-types.
    fn is_native(_currency: PaymentCurrency) -> bool {
        // No SOL variant exists yet in PaymentCurrency.
        // When added, this becomes: matches!(currency, PaymentCurrency::SOL)
        false
    }

    /// Standard Solana transaction fee in lamports (5000 per signature).
    pub const TX_FEE_LAMPORTS: u64 = 5000;

    /// Sign a prepared transfer with a private key (local operation).
    pub fn sign(
        prepared: &SolanaPreparedTransfer,
        private_key: &[u8],
    ) -> Result<SolanaSignedTransfer, BlockchainClientError> {
        let signature = sign::ed25519_sign(&prepared.message_bytes, private_key)?;
        let sig_array: [u8; 64] = signature
            .try_into()
            .map_err(|_| BlockchainClientError::Unknown("invalid signature length".into()))?;
        let signed_bytes =
            transaction::build_signed_transaction(&prepared.message_bytes, &sig_array);
        Ok(SolanaSignedTransfer {
            signed_transaction_bytes: signed_bytes,
        })
    }
}

// ── BlockchainClient implementation ─────────────────────────────────────────

impl BlockchainClient for SolanaClient {
    type PreparedTransfer = SolanaPreparedTransfer;
    type SignedTransfer = SolanaSignedTransfer;

    async fn prepare_transfer(
        &self,
        request: TransferRequest,
    ) -> Result<SolanaPreparedTransfer, BlockchainClientError> {
        let from = transaction::pubkey_from_base58(request.from.as_str())?;
        let to = transaction::pubkey_from_base58(request.to.as_str())?;
        let (blockhash_str, _) = self.get_latest_blockhash().await?;
        let recent_blockhash = transaction::pubkey_from_base58(&blockhash_str)?;

        let message_bytes = if Self::is_native(request.currency) {
            let lamports = request.amount.atomic() as u64;
            transaction::build_sol_transfer_message(&from, &to, lamports, &recent_blockhash)
        } else {
            let mint_str = self.mint_address(request.currency)?;
            let mint = transaction::pubkey_from_base58(mint_str)?;
            let amount = request.amount.atomic() as u64;
            spl::build_spl_transfer_message(&from, &to, &mint, amount, &recent_blockhash)?
        };

        Ok(SolanaPreparedTransfer {
            message_bytes,
            recent_blockhash: blockhash_str,
        })
    }

    async fn broadcast(
        &self,
        signed: SolanaSignedTransfer,
    ) -> Result<BroadcastResult, BlockchainClientError> {
        let txid = self
            .send_transaction(&signed.signed_transaction_bytes)
            .await?;
        Ok(BroadcastResult {
            txid: TransactionId::new(txid),
        })
    }

    async fn balance(
        &self,
        address: Address,
        currency: PaymentCurrency,
    ) -> Result<Money, BlockchainClientError> {
        let decimals = currency.decimals();
        if Self::is_native(currency) {
            let lamports = self.get_balance(address.as_str()).await?;
            Ok(Money::from_atomic(lamports as i128, decimals))
        } else {
            let mint = self.mint_address(currency)?;
            let raw = self.get_spl_token_balance(address.as_str(), mint).await?;
            Ok(Money::from_atomic(raw as i128, decimals))
        }
    }

    async fn estimate_withdrawable(
        &self,
        request: EstimateWithdrawableRequest,
    ) -> Result<Money, BlockchainClientError> {
        let decimals = request.currency.decimals();
        if Self::is_native(request.currency) {
            let lamports = self.get_balance(request.from.as_str()).await?;
            let withdrawable = lamports.saturating_sub(Self::TX_FEE_LAMPORTS);
            Ok(Money::from_atomic(withdrawable as i128, decimals))
        } else {
            // For SPL tokens, the full token balance is withdrawable.
            // Transaction fees are paid in SOL, not in the token.
            let mint = self.mint_address(request.currency)?;
            let raw = self
                .get_spl_token_balance(request.from.as_str(), mint)
                .await?;
            Ok(Money::from_atomic(raw as i128, decimals))
        }
    }

    fn generate_wallet(&self) -> Result<CryptoWallet, BlockchainClientError> {
        let wallet = SolanaWallet::new();
        Ok(wallet.into())
    }

    fn sign_transaction(
        &self,
        private_key: &[u8],
        raw_tx: &[u8],
    ) -> Result<Vec<u8>, BlockchainClientError> {
        Ok(sign::ed25519_sign(raw_tx, private_key)?)
    }

    fn is_valid_address(&self, address: &str) -> bool {
        // Solana addresses are base58-encoded 32-byte Ed25519 public keys.
        match bs58::decode(address).into_vec() {
            Ok(bytes) => bytes.len() == 32,
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::SolanaWallet;

    #[test]
    fn test_is_valid_address_valid() {
        let client = SolanaClient::new("https://api.devnet.solana.com");

        // System program address
        assert!(client.is_valid_address("11111111111111111111111111111111"));

        // Token program address
        assert!(client.is_valid_address("TokenkegQEqKXcsBR3MgFiQn4c5oSp3xMaKNvpqGMvN"));

        // Generated wallet address should be valid
        let wallet = SolanaWallet::new();
        assert!(
            client.is_valid_address(&wallet.address),
            "Generated Solana address should be valid, got: {}",
            wallet.address
        );
    }

    #[test]
    fn test_is_valid_address_invalid() {
        let client = SolanaClient::new("https://api.devnet.solana.com");

        // Empty
        assert!(!client.is_valid_address(""));
        // EVM address
        assert!(!client.is_valid_address(
            "0xdAC17F958D2ee523a2206206994597C13D831ec7"
        ));
        // Random garbage
        assert!(!client.is_valid_address("not-a-solana-address"));
        // Tron address
        assert!(!client.is_valid_address("TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t"));
    }
}
