use crate::{error::ChainError, wallet::Wallet};
use async_trait::async_trait;

#[async_trait]
pub trait Asset {
    fn symbol(&self) -> &'static str;
    fn decimals(&self) -> u8;

    async fn balance(&self, address: &str) -> Result<u128, ChainError>;

    async fn transfer(&self, from: &Wallet, to: &str, amount: u128) -> Result<String, ChainError>; // tx hash
}
