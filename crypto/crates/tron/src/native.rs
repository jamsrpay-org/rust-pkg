use crate::client::TronClient;
use async_trait::async_trait;
use chain_core::{error::CryptoAssetClientError, types::CryptoAssetClientTrait};

pub struct Trx {
    pub client: TronClient,
}

#[async_trait]
impl CryptoAssetClientTrait for Trx {
    fn symbol(&self) -> &'static str {
        "TRX"
    }

    fn decimals(&self) -> u8 {
        6
    }

    async fn balance(&self, _address: &str) -> Result<u128, CryptoAssetClientError> {
        let _ = self.client.call("get_balance").await;
        Ok(1_000_000)
    }

    async fn transfer(
        &self,
        _from: &str,
        _to: &str,
        _amount: u128,
    ) -> Result<String, CryptoAssetClientError> {
        Ok("trx_tx_hash".to_string())
    }

    async fn estimate_gas(&self) -> Result<u64, CryptoAssetClientError> {
        Ok(1)
    }
}
