use chain_core::{
    error::BlockchainClientError,
    types::{
        Address, BlockchainClient, BroadcastResult, CryptoWallet, EstimateWithdrawableRequest,
        TransactionId, TransferRequest,
    },
};
use jamsrpay_types::{currency::PaymentCurrency, money::Money};
use utxo_core::{
    client::UtxoClient,
    types::{UtxoNetwork, UtxoPreparedTransfer, UtxoSignedTransfer},
};

/// Thin Litecoin wrapper around `UtxoClient`.
///
/// Adds:
/// - Litecoin network defaults (mainnet / testnet)
/// - `PaymentCurrency::LTC` routing in `BlockchainClient`
///
/// ```no_run
/// use litecoin::client::LtcClient;
///
/// let client = LtcClient::new("https://litecoinspace.org/api", false).unwrap();
/// ```
pub struct LtcClient {
    utxo: UtxoClient,
}

impl LtcClient {
    /// Create a new Litecoin client.
    ///
    /// - `api_base_url`: Esplora-compatible REST API base URL.
    /// - `testnet`: if `true`, use Litecoin testnet; otherwise mainnet.
    pub fn new(api_base_url: &str, testnet: bool) -> Result<Self, BlockchainClientError> {
        let network = if testnet {
            UtxoNetwork::LitecoinTestnet
        } else {
            UtxoNetwork::LitecoinMainnet
        };
        Ok(Self {
            utxo: UtxoClient::new(api_base_url, network),
        })
    }

    pub fn utxo(&self) -> &UtxoClient {
        &self.utxo
    }

    fn is_native(currency: PaymentCurrency) -> bool {
        matches!(currency, PaymentCurrency::LTC)
    }
}

// ── BlockchainClient implementation ─────────────────────────────────────────

impl BlockchainClient for LtcClient {
    type PreparedTransfer = UtxoPreparedTransfer;
    type SignedTransfer = UtxoSignedTransfer;

    async fn prepare_transfer(
        &self,
        request: TransferRequest,
    ) -> Result<UtxoPreparedTransfer, BlockchainClientError> {
        if !Self::is_native(request.currency) {
            return Err(BlockchainClientError::Unknown(format!(
                "unsupported currency for Litecoin: {}",
                request.currency
            )));
        }
        Ok(self
            .utxo
            .build_transfer(
                request.from.as_str(),
                request.to.as_str(),
                request.amount.atomic() as u64,
            )
            .await?)
    }

    async fn broadcast(
        &self,
        signed: UtxoSignedTransfer,
    ) -> Result<BroadcastResult, BlockchainClientError> {
        let txid = self.utxo.broadcast(&signed.signed_tx_bytes).await?;
        Ok(BroadcastResult {
            txid: TransactionId::new(txid),
        })
    }

    async fn balance(
        &self,
        address: Address,
        currency: PaymentCurrency,
    ) -> Result<Money, BlockchainClientError> {
        if !Self::is_native(currency) {
            return Err(BlockchainClientError::Unknown(format!(
                "unsupported currency for Litecoin: {}",
                currency
            )));
        }
        let decimals = currency.decimals();
        let raw = self.utxo.balance(address.as_str()).await?;
        Ok(Money::from_atomic(raw as i128, decimals))
    }

    async fn estimate_withdrawable(
        &self,
        request: EstimateWithdrawableRequest,
    ) -> Result<Money, BlockchainClientError> {
        let decimals = request.currency.decimals();
        let raw = self
            .utxo
            .estimate_withdrawable(request.from.as_str())
            .await?;
        Ok(Money::from_atomic(raw as i128, decimals))
    }

    fn generate_wallet(&self) -> Result<CryptoWallet, BlockchainClientError> {
        Ok(UtxoClient::generate_wallet(self.utxo.network()).into())
    }

    fn sign_transaction(
        &self,
        private_key: &[u8],
        raw_tx: &[u8],
    ) -> Result<Vec<u8>, BlockchainClientError> {
        let pk_hex = hex::encode(private_key);
        Ok(UtxoClient::sign_raw(&pk_hex, raw_tx)?)
    }

    fn is_valid_address(&self, address: &str) -> bool {
        self.utxo.is_valid_address(address)
    }
}
