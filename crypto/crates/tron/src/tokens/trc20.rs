use crate::client::TronClient;
use async_trait::async_trait;
use chain_core::{error::CryptoAssetClientError, types::CryptoAssetClientTrait};

pub struct Trc20Token {
    pub symbol: &'static str,
    pub contract_address: &'static str,
    pub decimals: u8,
    pub client: TronClient,
}

#[async_trait]
impl CryptoAssetClientTrait for Trc20Token {
    fn symbol(&self) -> &'static str {
        self.symbol
    }

    fn decimals(&self) -> u8 {
        self.decimals
    }

    async fn balance(&self, _address: &str) -> Result<u128, CryptoAssetClientError> {
        Ok(5_000_000)
    }

    async fn transfer(
        &self,
        _from: &str,
        _to: &str,
        _amount: u128,
    ) -> Result<String, CryptoAssetClientError> {
        Ok("trc20_tx_hash".to_string())
    }

    async fn estimate_gas(&self) -> Result<u64, CryptoAssetClientError> {
        Ok(1)
    }
}
