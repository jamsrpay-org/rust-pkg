use crate::{
    error::{ChainClientError, CryptoAssetClientError},
    wallet::CryptoWallet,
};
use async_trait::async_trait;

#[derive(Debug)]
pub enum CryptoCurrency {
    Tron,
}

#[derive(Debug)]
pub struct UnsignedTx {
    pub raw_tx: Vec<u8>,
    pub tx_id: String,
    /// Optional serialized JSON bytes for chain-specific metadata (e.g. Tron's `raw_data`).
    pub raw_data_json: Option<Vec<u8>>,
}

#[async_trait]
pub trait CryptoAssetClientTrait {
    fn symbol(&self) -> &'static str;
    fn decimals(&self) -> u8;

    async fn balance(&self, address: &str) -> Result<u128, CryptoAssetClientError>;
    async fn create_transfer_tx(
        &self,
        from_address: &str,
        to_address: &str,
        amount: u128,
    ) -> Result<UnsignedTx, CryptoAssetClientError>; // tx hash
    fn sign(&self, raw_tx: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoAssetClientError>;
    async fn broadcast(
        &self,
        raw_tx: &[u8],
        signatures: &[Vec<u8>],
        raw_data_json: Option<&[u8]>,
    ) -> Result<String, CryptoAssetClientError>;
    async fn estimate_withdrawable(
        &self,
        from_address: &str,
        to_address: &str,
    ) -> Result<u128, CryptoAssetClientError>;
}

pub trait ChainClientTrait {
    fn generate_wallet(&self) -> Result<CryptoWallet, ChainClientError>;
}
