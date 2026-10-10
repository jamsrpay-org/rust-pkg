use strum::{AsRefStr, Display, EnumString};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Display, EnumString, AsRefStr,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
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
