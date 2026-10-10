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
