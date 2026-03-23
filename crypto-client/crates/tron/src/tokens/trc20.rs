use crate::client::TronClient;
use async_trait::async_trait;
use chain_core::{
    error::CryptoAssetClientError,
    types::{BroadcastTxResponse, CryptoAssetClientTrait, UnsignedTx},
};

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

    async fn create_transfer_tx(
        &self,
        _from: &str,
        _to: &str,
        _amount: u128,
    ) -> Result<UnsignedTx, CryptoAssetClientError> {
        Ok(UnsignedTx {
            raw_tx: vec![],
            tx_id: "trc20_tx_hash".to_string(),
            raw_data_json: None,
        })
    }

    fn sign(&self, _raw_tx: &[u8], _key: &[u8]) -> Result<Vec<u8>, CryptoAssetClientError> {
        todo!()
    }

    async fn broadcast(
        &self,
        _raw_tx: &[u8],
        _signatures: &[Vec<u8>],
        _raw_data_json: Option<&[u8]>,
    ) -> Result<BroadcastTxResponse, CryptoAssetClientError> {
        todo!()
    }

    async fn estimate_withdrawable(
        &self,
        _from_address: &str,
        _to_address: &str,
    ) -> Result<u128, CryptoAssetClientError> {
        Ok(0)
    }
}
