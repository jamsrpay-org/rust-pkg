use crate::client::sign::ec_key_sign;
use crate::types::{TronPreparedTransfer, TronSignedTransfer};
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

mod abi;
mod account;
mod bandwidth;
pub mod sign;
mod transaction;
mod trc20;

pub struct TronClient {
    http_base_url: String,
    client: reqwest::Client,
    trc20_contracts: HashMap<PaymentCurrency, String>,
}

impl TronClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();
        Self {
            http_base_url: base_url.into(),
            client,
            trc20_contracts: HashMap::new(),
        }
    }

    /// Register a TRC20 token contract address for a given currency.
    ///
    /// ```no_run
    /// use tron::client::TronClient;
    /// use tron::contracts;
    /// use jamsrpay_types::currency::PaymentCurrency;
    ///
    /// let client = TronClient::new("https://api.trongrid.io/wallet")
    ///     .register_trc20(PaymentCurrency::USDT_TRC20, contracts::mainnet::USDT_TRC20);
    /// ```
    pub fn register_trc20(
        mut self,
        currency: PaymentCurrency,
        contract_address: impl Into<String>,
    ) -> Self {
        self.trc20_contracts
            .insert(currency, contract_address.into());
        self
    }

    pub(crate) fn contract_address(
        &self,
        currency: PaymentCurrency,
    ) -> Result<&str, BlockchainClientError> {
        self.trc20_contracts
            .get(&currency)
            .map(|s| s.as_str())
            .ok_or_else(|| {
                BlockchainClientError::Unknown(format!(
                    "no TRC20 contract registered for {}",
                    currency
                ))
            })
    }

    pub fn base_url(&self) -> &str {
        &self.http_base_url
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Sign a prepared transfer with a private key (local operation, not a blockchain call).
    pub fn sign(
        prepared: &TronPreparedTransfer,
        private_key: &[u8],
    ) -> Result<TronSignedTransfer, BlockchainClientError> {
        let signature = ec_key_sign(&prepared.raw_data_bytes, private_key)?;
        Ok(TronSignedTransfer {
            prepared: TronPreparedTransfer {
                raw_data_bytes: prepared.raw_data_bytes.clone(),
                raw_data_hex: prepared.raw_data_hex.clone(),
                tx_id: prepared.tx_id.clone(),
                raw_data_json: prepared.raw_data_json.clone(),
            },
            signatures: vec![signature],
        })
    }
}

// ── BlockchainClient implementation ─────────────────────────────────────────

impl BlockchainClient for TronClient {
    type PreparedTransfer = TronPreparedTransfer;
    type SignedTransfer = TronSignedTransfer;

    async fn prepare_transfer(
        &self,
        request: TransferRequest,
    ) -> Result<TronPreparedTransfer, BlockchainClientError> {
        let tx = match request.currency {
            PaymentCurrency::TRX => {
                self.create_transaction(
                    request.from.as_str(),
                    request.to.as_str(),
                    request.amount.atomic() as u128,
                )
                .await?
            }
            PaymentCurrency::USDT_TRC20 => {
                let contract = self.contract_address(request.currency)?;
                self.create_trc20_transfer(
                    request.from.as_str(),
                    request.to.as_str(),
                    request.amount.atomic() as u128,
                    contract,
                )
                .await?
            }
        };

        let raw_data_bytes = hex::decode(&tx.raw_data_hex)
            .map_err(|e| BlockchainClientError::InvalidTransaction(e.to_string()))?;

        Ok(TronPreparedTransfer {
            raw_data_bytes,
            raw_data_hex: tx.raw_data_hex,
            tx_id: tx.tx_id,
            raw_data_json: tx.raw_data,
        })
    }

    async fn broadcast(
        &self,
        signed: TronSignedTransfer,
    ) -> Result<BroadcastResult, BlockchainClientError> {
        let result = self
            .broadcast_transaction(
                &signed.prepared.raw_data_bytes,
                &signed.signatures,
                &signed.prepared.raw_data_json,
            )
            .await?;
        Ok(BroadcastResult {
            txid: TransactionId::new(result.tx_id),
        })
    }

    async fn balance(
        &self,
        address: Address,
        currency: PaymentCurrency,
    ) -> Result<Money, BlockchainClientError> {
        let decimals = currency.decimals();
        match currency {
            PaymentCurrency::TRX => {
                let raw = self.get_balance(address.as_str()).await?;
                Ok(Money::from_atomic(raw as i128, decimals))
            }
            PaymentCurrency::USDT_TRC20 => {
                let contract = self.contract_address(currency)?;
                let raw = self.get_trc20_balance(address.as_str(), contract).await?;
                Ok(Money::from_atomic(raw as i128, decimals))
            }
        }
    }

    async fn estimate_withdrawable(
        &self,
        request: EstimateWithdrawableRequest,
    ) -> Result<Money, BlockchainClientError> {
        let decimals = request.currency.decimals();
        match request.currency {
            PaymentCurrency::TRX => self.estimate_trx_withdrawable(&request, decimals).await,
            PaymentCurrency::USDT_TRC20 => {
                // For TRC20 tokens, the full balance is withdrawable.
                // Energy fees are paid in TRX, not in the token itself.
                let contract = self.contract_address(request.currency)?;
                let raw = self
                    .get_trc20_balance(request.from.as_str(), contract)
                    .await?;
                Ok(Money::from_atomic(raw as i128, decimals))
            }
        }
    }
}

impl TronClient {
    /// Estimate the maximum TRX that can be sent after accounting for bandwidth fees.
    async fn estimate_trx_withdrawable(
        &self,
        request: &EstimateWithdrawableRequest,
        decimals: u8,
    ) -> Result<Money, BlockchainClientError> {
        let account = self.get_account(request.from.as_str()).await?;
        let balance = account.balance;
        if balance == 0 {
            return Ok(Money::zero(decimals));
        }

        // Create a dummy transaction to measure bandwidth consumption.
        let unsigned_tx = self
            .create_transaction(request.from.as_str(), request.to.as_str(), 1)
            .await?;
        let required_bandwidth = TronClient::estimate_bandwidth(&unsigned_tx.raw_data_hex);

        let resource = self.get_account_resource(request.from.as_str()).await?;
        let available_bandwidth = TronClient::get_available_bandwidth(&resource);

        let missing_bandwidth = required_bandwidth.saturating_sub(available_bandwidth);
        if missing_bandwidth > 0 {
            let fee = TronClient::calculate_bandwidth_fee(missing_bandwidth);
            return Ok(Money::from_atomic(
                balance.saturating_sub(fee) as i128,
                decimals,
            ));
        }

        Ok(Money::from_atomic(balance as i128, decimals))
    }
}
