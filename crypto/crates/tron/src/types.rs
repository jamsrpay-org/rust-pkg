use types::CryptoWallet;

#[derive(Debug)]
pub struct TronWalletAddress {
    pub base58: String,
    pub hex: String,
}

#[derive(Debug)]
pub struct TronWallet {
    pub private_key: String,
    pub public_key: String,
    pub address: TronWalletAddress,
}

impl From<TronWallet> for CryptoWallet {
    fn from(wallet: TronWallet) -> Self {
        CryptoWallet {
            address: wallet.address.base58,
            private_key: wallet.private_key,
        }
    }
}
