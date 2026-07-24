use chain_core::{
    error::BlockchainClientError,
    types::{
        Address, BlockchainClient, BroadcastResult, CryptoWallet, EstimateWithdrawableRequest,
        TransactionId, TransferRequest,
    },
};
use evm_core::{
    client::EvmClient,
    types::{EvmPreparedTransfer, EvmSignedTransfer},
};
use jamsrpay_types::{currency::PaymentCurrency, money::Money};
use std::collections::HashMap;

/// Thin Polygon wrapper around `EvmClient`.
///
/// Adds:
/// - Polygon chain defaults (chain_id = 137 mainnet, 80002 amoy testnet)
/// - ERC20 contract address registry
/// - `PaymentCurrency` → native/token routing in `BlockchainClient`
///
/// ```no_run
/// use polygon::client::PolygonClient;
/// use polygon::contracts;
///
/// let client = PolygonClient::new("https://polygon-rpc.com", 137).unwrap();
/// ```
pub struct PolygonClient {
    evm: EvmClient,
    token_contracts: HashMap<PaymentCurrency, String>,
}

impl PolygonClient {
    pub fn new(rpc_url: &str, chain_id: u64) -> Result<Self, BlockchainClientError> {
        let evm = EvmClient::new(rpc_url, chain_id)?;
        Ok(Self {
            evm,
            token_contracts: HashMap::new(),
        })
    }

    /// Register an ERC20 token contract address for a given currency.
    pub fn register_token(
        mut self,
        currency: PaymentCurrency,
        contract_address: impl Into<String>,
    ) -> Self {
        self.token_contracts
            .insert(currency, contract_address.into());
        self
    }

    pub fn evm(&self) -> &EvmClient {
        &self.evm
    }

    fn contract_address(
        &self,
        currency: PaymentCurrency,
    ) -> Result<&str, BlockchainClientError> {
        self.token_contracts
            .get(&currency)
            .map(|s| s.as_str())
            .ok_or_else(|| {
                BlockchainClientError::Unknown(format!(
                    "no ERC20 contract registered for {}",
                    currency
                ))
            })
    }

    /// Returns true if the currency is the native gas token (POL/MATIC).
    ///
    /// NOTE: Update this when `PaymentCurrency::POL` is added to jamsrpay-types.
    fn is_native(_currency: PaymentCurrency) -> bool {
        // No POL/MATIC variant exists yet in PaymentCurrency.
        // When added, this becomes: matches!(currency, PaymentCurrency::POL)
        false
    }
}

// ── BlockchainClient implementation ─────────────────────────────────────────

impl BlockchainClient for PolygonClient {
    type PreparedTransfer = EvmPreparedTransfer;
    type SignedTransfer = EvmSignedTransfer;

    async fn prepare_transfer(
        &self,
        request: TransferRequest,
    ) -> Result<EvmPreparedTransfer, BlockchainClientError> {
        if Self::is_native(request.currency) {
            Ok(self
                .evm
                .build_native_transfer(
                    request.from.as_str(),
                    request.to.as_str(),
                    request.amount.atomic() as u128,
                )
                .await?)
        } else {
            let contract = self.contract_address(request.currency)?;
            Ok(self
                .evm
                .build_erc20_transfer(
                    request.from.as_str(),
                    request.to.as_str(),
                    request.amount.atomic() as u128,
                    contract,
                )
                .await?)
        }
    }

    async fn broadcast(
        &self,
        signed: EvmSignedTransfer,
    ) -> Result<BroadcastResult, BlockchainClientError> {
        let tx_hash = self.evm.broadcast(&signed.signed_tx_bytes).await?;
        Ok(BroadcastResult {
            txid: TransactionId::new(tx_hash),
        })
    }

    async fn balance(
        &self,
        address: Address,
        currency: PaymentCurrency,
    ) -> Result<Money, BlockchainClientError> {
        let decimals = currency.decimals();
        if Self::is_native(currency) {
            let raw = self.evm.native_balance(address.as_str()).await?;
            Ok(Money::from_atomic(raw as i128, decimals))
        } else {
            let contract = self.contract_address(currency)?;
            let raw = self
                .evm
                .erc20_balance(address.as_str(), contract)
                .await?;
            Ok(Money::from_atomic(raw as i128, decimals))
        }
    }

    async fn estimate_withdrawable(
        &self,
        request: EstimateWithdrawableRequest,
    ) -> Result<Money, BlockchainClientError> {
        let decimals = request.currency.decimals();
        if Self::is_native(request.currency) {
            let raw = self
                .evm
                .estimate_native_withdrawable(request.from.as_str())
                .await?;
            Ok(Money::from_atomic(raw as i128, decimals))
        } else {
            // For ERC20 tokens, the full balance is withdrawable.
            // Gas fees are paid in POL/MATIC, not the token.
            let contract = self.contract_address(request.currency)?;
            let raw = self
                .evm
                .erc20_balance(request.from.as_str(), contract)
                .await?;
            Ok(Money::from_atomic(raw as i128, decimals))
        }
    }

    fn generate_wallet(&self) -> Result<CryptoWallet, BlockchainClientError> {
        Ok(EvmClient::generate_wallet().into())
    }

    fn sign_transaction(
        &self,
        private_key: &[u8],
        raw_tx: &[u8],
    ) -> Result<Vec<u8>, BlockchainClientError> {
        Ok(EvmClient::sign_raw(private_key, raw_tx)?)
    }

    fn is_valid_address(&self, address: &str) -> bool {
        EvmClient::is_valid_address(address)
    }
}
