use crate::client::TronClient;
use async_trait::async_trait;
use chain_core::{error::CryptoAssetClientError, types::CryptoAssetClientTrait};

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

    async fn transfer(
        &self,
        _from: &str,
        _to: &str,
        _amount: u128,
    ) -> Result<String, CryptoAssetClientError> {
        Ok("trx_tx_hash".to_string())
    }

    async fn estimate_withdrawable(
        &self,
        from_address: &str,
        to_address: &str,
        amount: u128,
    ) -> Result<u128, CryptoAssetClientError> {
        // create transaction
        let unsigned_tx = self
            .client
            .create_transaction(from_address, to_address, amount)
            .await?;
        let required_bandwidth = TronClient::estimate_bandwidth(&unsigned_tx.raw_data_hex);

        let account = self.client.get_account(from_address).await?;
        let balance = account.balance;
        if balance <= 0 {
            return Ok(0);
        }

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
