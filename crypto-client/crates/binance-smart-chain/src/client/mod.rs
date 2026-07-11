use crate::client::sign::sign_transaction;
use crate::client::transaction::rlp_encode_signed;
use crate::types::{BscPreparedTransfer, BscSignedTransfer, BscWallet};
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

mod abi;
mod bep20;
mod gas;
mod rpc;
pub mod sign;
mod transaction;

pub struct BscClient {
    http_base_url: String,
    client: reqwest::Client,
    bep20_contracts: HashMap<PaymentCurrency, String>,
    chain_id: u64,
}

impl BscClient {
    pub fn new(base_url: impl Into<String>, chain_id: u64) -> Self {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();
        Self {
            http_base_url: base_url.into(),
            client,
            bep20_contracts: HashMap::new(),
            chain_id,
        }
    }

    /// Register a BEP20 token contract address for a given currency.
    ///
    /// ```no_run
    /// use binance_smart_chain::client::BscClient;
    /// use binance_smart_chain::contracts;
    /// use jamsrpay_types::currency::PaymentCurrency;
    ///
    /// let client = BscClient::new("https://bsc-dataseed.binance.org", 56)
    ///     .register_bep20(PaymentCurrency::USDT_BEP20, contracts::mainnet::USDT_BEP20);
    /// ```
    pub fn register_bep20(
        mut self,
        currency: PaymentCurrency,
        contract_address: impl Into<String>,
    ) -> Self {
        self.bep20_contracts
            .insert(currency, contract_address.into());
        self
    }

    pub(crate) fn contract_address(
        &self,
        currency: PaymentCurrency,
    ) -> Result<&str, BlockchainClientError> {
        self.bep20_contracts
            .get(&currency)
            .map(|s| s.as_str())
            .ok_or_else(|| {
                BlockchainClientError::Unknown(format!(
                    "no BEP20 contract registered for {}",
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

    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }

    /// Sign a prepared transfer with a private key (local operation, not a blockchain call).
    pub fn sign(
        prepared: &BscPreparedTransfer,
        private_key: &[u8],
    ) -> Result<BscSignedTransfer, BlockchainClientError> {
        let (v, r, s) =
            sign_transaction(&prepared.unsigned_tx_bytes, private_key, prepared.chain_id)?;

        let signed_tx_bytes = rlp_encode_signed(
            prepared.nonce,
            prepared.gas_price,
            prepared.gas_limit,
            &prepared.to,
            prepared.value,
            &prepared.data,
            v,
            &r,
            &s,
        );

        Ok(BscSignedTransfer {
            prepared: BscPreparedTransfer {
                unsigned_tx_bytes: prepared.unsigned_tx_bytes.clone(),
                nonce: prepared.nonce,
                gas_price: prepared.gas_price,
                gas_limit: prepared.gas_limit,
                to: prepared.to.clone(),
                value: prepared.value,
                data: prepared.data.clone(),
                chain_id: prepared.chain_id,
            },
            signed_tx_bytes,
        })
    }
}

// ── BlockchainClient implementation ─────────────────────────────────────────

impl BlockchainClient for BscClient {
    type PreparedTransfer = BscPreparedTransfer;
    type SignedTransfer = BscSignedTransfer;

    async fn prepare_transfer(
        &self,
        request: TransferRequest,
    ) -> Result<BscPreparedTransfer, BlockchainClientError> {
        match request.currency {
            PaymentCurrency::BNB => {
                let tx = self
                    .create_bnb_transfer(
                        request.from.as_str(),
                        request.to.as_str(),
                        request.amount.atomic() as u128,
                    )
                    .await?;
                Ok(tx)
            }
            PaymentCurrency::USDT_BEP20
            | PaymentCurrency::USDC_BEP20
            | PaymentCurrency::BUSD_BEP20
            | PaymentCurrency::DAI_BEP20 => {
                let contract = self.contract_address(request.currency)?;
                let tx = self
                    .create_bep20_transfer_tx(
                        request.from.as_str(),
                        request.to.as_str(),
                        request.amount.atomic() as u128,
                        contract,
                    )
                    .await?;
                Ok(tx)
            }
            _ => unreachable!(),
        }
    }

    async fn broadcast(
        &self,
        signed: BscSignedTransfer,
    ) -> Result<BroadcastResult, BlockchainClientError> {
        let raw_tx_hex = format!("0x{}", hex::encode(&signed.signed_tx_bytes));
        let tx_hash = self.eth_send_raw_transaction(&raw_tx_hex).await?;
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
        match currency {
            PaymentCurrency::BNB => {
                let raw = self.eth_get_balance(address.as_str()).await?;
                Ok(Money::from_atomic(raw as i128, decimals))
            }
            PaymentCurrency::USDT_BEP20
            | PaymentCurrency::USDC_BEP20
            | PaymentCurrency::BUSD_BEP20
            | PaymentCurrency::DAI_BEP20 => {
                let contract = self.contract_address(currency)?;
                let raw = self.get_bep20_balance(address.as_str(), contract).await?;
                Ok(Money::from_atomic(raw as i128, decimals))
            }
            _ => unreachable!(),
        }
    }

    async fn estimate_withdrawable(
        &self,
        request: EstimateWithdrawableRequest,
    ) -> Result<Money, BlockchainClientError> {
        let decimals = request.currency.decimals();
        match request.currency {
            PaymentCurrency::BNB => self.estimate_bnb_withdrawable(&request, decimals).await,
            PaymentCurrency::USDT_BEP20
            | PaymentCurrency::USDC_BEP20
            | PaymentCurrency::BUSD_BEP20
            | PaymentCurrency::DAI_BEP20 => {
                self.estimate_bep20_withdrawable(&request, request.currency, decimals)
                    .await
            }
            _ => unreachable!(),
        }
    }

    fn generate_wallet(&self) -> Result<CryptoWallet, BlockchainClientError> {
        let wallet = BscWallet::new();
        Ok(CryptoWallet {
            private_key: wallet.private_key,
            address: wallet.address,
        })
    }

    fn sign_transaction(
        &self,
        private_key: &[u8],
        raw_tx: &[u8],
    ) -> Result<Vec<u8>, BlockchainClientError> {
        let (v, r, s) = sign_transaction(raw_tx, private_key, self.chain_id)?;

        // RLP-decode the unsigned tx to extract fields for re-encoding as signed tx.
        // Unsigned EIP-155 format: [nonce, gasPrice, gasLimit, to, value, data, chainId, 0, 0]
        let rlp_data = rlp::Rlp::new(raw_tx);
        let nonce: u64 = rlp_data.val_at(0)
            .map_err(|e| BlockchainClientError::InvalidTransaction(e.to_string()))?;
        let gas_price: u128 = rlp_data.val_at(1)
            .map_err(|e| BlockchainClientError::InvalidTransaction(e.to_string()))?;
        let gas_limit: u128 = rlp_data.val_at(2)
            .map_err(|e| BlockchainClientError::InvalidTransaction(e.to_string()))?;
        let to_bytes: Vec<u8> = rlp_data.val_at(3)
            .map_err(|e| BlockchainClientError::InvalidTransaction(e.to_string()))?;
        let value: u128 = rlp_data.val_at(4)
            .map_err(|e| BlockchainClientError::InvalidTransaction(e.to_string()))?;
        let data: Vec<u8> = rlp_data.val_at(5)
            .map_err(|e| BlockchainClientError::InvalidTransaction(e.to_string()))?;

        let to_hex = format!("0x{}", hex::encode(&to_bytes));

        let signed_bytes = rlp_encode_signed(
            nonce,
            gas_price,
            gas_limit as u64,
            &to_hex,
            value,
            &data,
            v,
            &r,
            &s,
        );

        Ok(signed_bytes)
    }
}
