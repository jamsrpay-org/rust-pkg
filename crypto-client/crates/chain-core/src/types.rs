use crate::{
    error::{ChainClientError, CryptoAssetClientError},
    wallet::CryptoWallet,
};
use async_trait::async_trait;

pub enum CryptoCurrency {
    Tron,
}

#[async_trait]
pub trait CryptoAssetClientTrait {
    fn symbol(&self) -> &'static str;
    fn decimals(&self) -> u8;
    async fn balance(&self, address: &str) -> Result<u128, CryptoAssetClientError>;
    async fn transfer(
        &self,
        from_address: &str,
        to_address: &str,
        amount: u128,
    ) -> Result<String, CryptoAssetClientError>; // tx hash
    async fn estimate_withdrawable(
        &self,
        from_address: &str,
        to_address: &str,
        amount: u128,
    ) -> Result<u128, CryptoAssetClientError>;
}

pub trait ChainClientTrait {
    fn generate_wallet(&self) -> Result<CryptoWallet, ChainClientError>;
}
