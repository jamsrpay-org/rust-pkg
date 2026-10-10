use strum::{AsRefStr, Display, EnumString};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Display,
    EnumString,
    AsRefStr,
    serde::Serialize,
    serde::Deserialize,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AssetId {
    Trx,
    Bnb,
    Eth,
    Pol,
    Btc,
    Ltc,
    Sol,
    Usdt,
    Usdc,
    Dai,
    Eurc,
}

impl AssetId {
    pub const TRX: Self = Self::Trx;
    pub const BNB: Self = Self::Bnb;
    pub const ETH: Self = Self::Eth;
    pub const POL: Self = Self::Pol;
    pub const BTC: Self = Self::Btc;
    pub const LTC: Self = Self::Ltc;
    pub const SOL: Self = Self::Sol;
    pub const USDT: Self = Self::Usdt;
    pub const USDC: Self = Self::Usdc;
    pub const DAI: Self = Self::Dai;
    pub const EURC: Self = Self::Eurc;

    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Trx => "TRX",
            Self::Bnb => "BNB",
            Self::Eth => "ETH",
            Self::Pol => "POL",
            Self::Btc => "BTC",
            Self::Ltc => "LTC",
            Self::Sol => "SOL",
            Self::Usdt => "USDT",
            Self::Usdc => "USDC",
            Self::Dai => "DAI",
            Self::Eurc => "EURC",
        }
    }

    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Trx => "TRON",
            Self::Bnb => "BNB",
            Self::Eth => "Ethereum",
            Self::Pol => "Polygon",
            Self::Btc => "Bitcoin",
            Self::Ltc => "Litecoin",
            Self::Sol => "Solana",
            Self::Usdt => "Tether USD",
            Self::Usdc => "USD Coin",
            Self::Dai => "Dai",
            Self::Eurc => "EURC",
        }
    }

    pub const fn name(self) -> &'static str {
        self.display_name()
    }
}
