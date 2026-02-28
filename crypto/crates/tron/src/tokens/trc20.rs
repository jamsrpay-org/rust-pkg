use crate::client::TronClient;
use async_trait::async_trait;
use chain_core::{asset::Asset, error::ChainError, wallet::Wallet};

pub struct Trc20Token {
    pub symbol: &'static str,
    pub contract_address: &'static str,
    pub decimals: u8,
    pub client: TronClient,
}

#[async_trait]
impl Asset for Trc20Token {
    fn symbol(&self) -> &'static str {
        self.symbol
    }

    fn decimals(&self) -> u8 {
        self.decimals
    }

    async fn balance(&self, address: &str) -> Result<u128, ChainError> {
        Ok(5_000_000)
    }

    async fn transfer(
        &self,
        _from: &Wallet,
        _to: &str,
        _amount: u128,
    ) -> Result<String, ChainError> {
        Ok("trc20_tx_hash".to_string())
    }
}
