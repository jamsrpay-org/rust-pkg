#[derive(Debug)]
pub enum CryptoCurrency {
    Tron,
}

#[derive(Debug)]
pub struct CryptoWallet {
    pub address: String,
    pub private_key: String,
}
