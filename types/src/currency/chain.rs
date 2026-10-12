use strum::{AsRefStr, Display, EnumString};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Display, EnumString, AsRefStr,
)]
#[strum(serialize_all = "snake_case")]
pub enum Chain {
    Tron,
    BinanceSmartChain,
    Ethereum,
    Polygon,
    Bitcoin,
    Litecoin,
    Solana,
}

impl Chain {
    pub fn is_evm(&self) -> bool {
        matches!(
            self,
            Chain::Ethereum | Chain::Polygon | Chain::BinanceSmartChain
        )
    }

    pub fn is_utxo(&self) -> bool {
        matches!(self, Chain::Bitcoin | Chain::Litecoin)
    }
}
