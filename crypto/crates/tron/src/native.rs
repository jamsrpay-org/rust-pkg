use crate::client::TronClient;
use async_trait::async_trait;
use chain_core::{asset::Asset, error::ChainError, wallet::Wallet};

pub struct Trx {
    pub client: TronClient,
}

#[async_trait]
impl Asset for Trx {
    fn symbol(&self) -> &'static str {
        "TRX"
    }

    fn decimals(&self) -> u8 {
        6
    }

    async fn balance(&self, _address: &str) -> Result<u128, ChainError> {
        let _ = self.client.call("get_balance").await;
        Ok(1_000_000)
    }

    async fn transfer(
        &self,
        _from: &Wallet,
        _to: &str,
        _amount: u128,
    ) -> Result<String, ChainError> {
        Ok("trx_tx_hash".to_string())
    }
}
