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
pub enum FiatCurrencyId {
    Gbp,
    Hkd,
    Ils,
    Inr,
    Jpy,
    Cad,
    Chf,
    Eur,
    Php,
    Usd,
}

impl FiatCurrencyId {
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Gbp => "GBP",
            Self::Hkd => "HKD",
            Self::Ils => "ILS",
            Self::Inr => "INR",
            Self::Jpy => "JPY",
            Self::Cad => "CAD",
            Self::Chf => "CHF",
            Self::Eur => "EUR",
            Self::Php => "PHP",
            Self::Usd => "USD",
        }
    }

    pub const fn decimals(self) -> u8 {
        match self {
            Self::Jpy => 0,
            _ => 2,
        }
    }
}
