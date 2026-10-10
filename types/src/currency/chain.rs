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
