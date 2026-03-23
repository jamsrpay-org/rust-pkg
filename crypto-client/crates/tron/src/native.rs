use crate::client::{TronClient, sign::ec_key_sign};
use async_trait::async_trait;
use chain_core::{
    error::CryptoAssetClientError,
    types::{BroadcastTxResponse, CryptoAssetClientTrait, UnsignedTx},
};

pub struct Trx {
    pub client: TronClient,
}

impl Trx {
    pub fn new(client: TronClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl CryptoAssetClientTrait for Trx {
    fn symbol(&self) -> &'static str {
        "TRX"
    }

    fn decimals(&self) -> u8 {
        6
    }

    async fn balance(&self, address: &str) -> Result<u128, CryptoAssetClientError> {
        let balance = self.client.get_balance(address).await?;
        Ok(balance)
    }

    async fn create_transfer_tx(
        &self,
        from_address: &str,
        to_address: &str,
        amount: u128,
    ) -> Result<UnsignedTx, CryptoAssetClientError> {
        let tx = self
            .client
            .create_transaction(from_address, to_address, amount)
            .await?;
        let raw_data_bytes = hex::decode(tx.raw_data_hex)
            .map_err(|e| CryptoAssetClientError::InvalidTransaction(e.to_string()))?;
        let tx_id = tx.tx_id;
        let raw_data_json = serde_json::to_vec(&tx.raw_data)
            .map_err(|e| CryptoAssetClientError::InvalidTransaction(e.to_string()))?;

        Ok(UnsignedTx {
            raw_tx: raw_data_bytes,
            tx_id,
            raw_data_json: Some(raw_data_json),
        })
    }

    fn sign(&self, raw_tx: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoAssetClientError> {
        let signature = ec_key_sign(raw_tx, key)?;
        Ok(signature)
    }

    async fn broadcast(
        &self,
        raw_tx: &[u8],
        signatures: &[Vec<u8>],
        raw_data_json: Option<&[u8]>,
    ) -> Result<BroadcastTxResponse, CryptoAssetClientError> {
        let raw_data: serde_json::Value =
            serde_json::from_slice(raw_data_json.ok_or_else(|| {
                CryptoAssetClientError::InvalidTransaction(
                    "missing raw_data for broadcast".to_string(),
                )
            })?)
            .map_err(|e| CryptoAssetClientError::InvalidTransaction(e.to_string()))?;

        let result = self
            .client
            .broadcast_transaction(raw_tx, signatures, &raw_data)
            .await?;
        Ok(BroadcastTxResponse {
            tx_id: result.tx_id,
        })
    }

    async fn estimate_withdrawable(
        &self,
        from_address: &str,
        to_address: &str,
    ) -> Result<u128, CryptoAssetClientError> {
        let account = self.client.get_account(from_address).await?;
        let balance = account.balance;
        if balance <= 0 {
            return Ok(0);
        }

        // create transaction
        let unsigned_tx = self
            .client
            .create_transaction(from_address, to_address, 1)
            .await?;
        let required_bandwidth = TronClient::estimate_bandwidth(&unsigned_tx.raw_data_hex);

        let resource = self.client.get_account_resource(from_address).await?;
        let available_bandwidth = TronClient::get_available_bandwidth(&resource);

        let missing_bandwidth = required_bandwidth.saturating_sub(available_bandwidth);
        if missing_bandwidth > 0 {
            let fee = TronClient::calculate_bandwidth_fee(missing_bandwidth);
            return Ok(balance.saturating_sub(fee));
        }

        Ok(balance)
    }
}
