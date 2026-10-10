use std::fmt;
use std::str::FromStr;

use crate::currency::{AssetId, FiatCurrencyId};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(untagged)]
pub enum PricingCurrency {
    Fiat(FiatCurrencyId),
    AssetId(AssetId),
}

impl PricingCurrency {
    // Fiat constants
    pub const CAD: Self = Self::Fiat(FiatCurrencyId::Cad);
    pub const CHF: Self = Self::Fiat(FiatCurrencyId::Chf);
    pub const EUR: Self = Self::Fiat(FiatCurrencyId::Eur);
    pub const GBP: Self = Self::Fiat(FiatCurrencyId::Gbp);
    pub const HKD: Self = Self::Fiat(FiatCurrencyId::Hkd);
    pub const ILS: Self = Self::Fiat(FiatCurrencyId::Ils);
    pub const INR: Self = Self::Fiat(FiatCurrencyId::Inr);
    pub const JPY: Self = Self::Fiat(FiatCurrencyId::Jpy);
    pub const PHP: Self = Self::Fiat(FiatCurrencyId::Php);
    pub const USD: Self = Self::Fiat(FiatCurrencyId::Usd);

    // Crypto constants
    pub const TRX: Self = Self::AssetId(AssetId::Trx);
    pub const BNB: Self = Self::AssetId(AssetId::Bnb);
    pub const ETH: Self = Self::AssetId(AssetId::Eth);
    pub const POL: Self = Self::AssetId(AssetId::Pol);
    pub const BTC: Self = Self::AssetId(AssetId::Btc);
    pub const LTC: Self = Self::AssetId(AssetId::Ltc);
    pub const SOL: Self = Self::AssetId(AssetId::Sol);
    pub const USDT: Self = Self::AssetId(AssetId::Usdt);
    pub const USDC: Self = Self::AssetId(AssetId::Usdc);
    pub const DAI: Self = Self::AssetId(AssetId::Dai);
    pub const EURC: Self = Self::AssetId(AssetId::Eurc);

    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Fiat(f) => f.symbol(),
            Self::AssetId(a) => a.symbol(),
        }
    }

    pub const fn decimals(self) -> u8 {
        match self {
            Self::Fiat(f) => f.decimals(),
            Self::AssetId(a) => match a {
                AssetId::Trx => 6,
                AssetId::Usdt => 6,
                AssetId::Bnb => 18,
                AssetId::Usdc => 6,
                AssetId::Dai => 18,
                AssetId::Eurc => 6,
                AssetId::Eth => 18,
                AssetId::Pol => 18,
                AssetId::Btc => 8,
                AssetId::Ltc => 8,
                AssetId::Sol => 9,
            },
        }
    }

    pub fn is_fiat(&self) -> bool {
        matches!(self, Self::Fiat(_))
    }

    pub fn is_asset(&self) -> bool {
        matches!(self, Self::AssetId(_))
    }

    pub fn as_fiat(&self) -> Option<FiatCurrencyId> {
        match self {
            Self::Fiat(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_asset(&self) -> Option<AssetId> {
        match self {
            Self::AssetId(a) => Some(*a),
            _ => None,
        }
    }
}

impl AsRef<str> for PricingCurrency {
    fn as_ref(&self) -> &str {
        self.symbol()
    }
}

impl fmt::Display for PricingCurrency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl FromStr for PricingCurrency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(f) = FiatCurrencyId::from_str(s) {
            return Ok(Self::Fiat(f));
        }
        if let Ok(a) = AssetId::from_str(s) {
            return Ok(Self::AssetId(a));
        }
        Err(format!("Invalid pricing currency: {s}"))
    }
}

impl From<FiatCurrencyId> for PricingCurrency {
    fn from(fiat: FiatCurrencyId) -> Self {
        Self::Fiat(fiat)
    }
}

impl From<AssetId> for PricingCurrency {
    fn from(asset: AssetId) -> Self {
        Self::AssetId(asset)
    }
}
