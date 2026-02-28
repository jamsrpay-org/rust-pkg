pub use ::types::{CryptoCurrency, CryptoWallet};
use tron::types::TronWallet;

pub struct CryptoClient {
    currency: CryptoCurrency,
}

impl CryptoClient {
    pub fn new(currency: CryptoCurrency) -> Self {
        Self { currency }
    }

    pub fn generate_wallet(&self) -> CryptoWallet {
        match self.currency {
            CryptoCurrency::Tron => TronWallet::new().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_wallet() {
        let client = CryptoClient::new(CryptoCurrency::Tron);
        let wallet = client.generate_wallet();
        dbg!(&wallet);
        assert!(wallet.private_key.len() > 0);
    }
}
