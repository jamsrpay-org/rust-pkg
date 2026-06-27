use strum::{AsRefStr, Display, EnumString};

mod implementation;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, EnumString, Display, Eq, AsRefStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum FiatCurrency {
    USD,
    EUR,
    GBP,
    CAD,
    CHF,
    HKD,
    ILS,
    INR,
    JPY,
    PHP,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum CryptoAsset {
    TRX,
    USDT,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum PricingCurrency {
    // Crypto Assets
    TRX,
    USDT,
    // Fiat Currencies
    USD,
    EUR,
    GBP,
    CAD,
    CHF,
    HKD,
    ILS,
    INR,
    JPY,
    PHP,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentCurrency {
    TRX,
    USDT_TRC20,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum GasCurrency {
    TRX,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum Blockchain {
    Tron,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum BlockchainNetwork {
    TronMainnet,
    TronNile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum TokenStandard {
    Trc20,
}

#[derive(Debug, Clone, Copy)]
pub enum AssetKind {
    Native,
    Token { standard: TokenStandard },
}

#[derive(Debug, Clone, Copy)]
pub struct PaymentCurrencyMeta {
    pub asset_id: &'static str,
    pub symbol: &'static str,
    pub name: &'static str,
    pub chain: Blockchain,
    pub kind: AssetKind,
    pub decimals: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct PricingCurrencyMeta {
    pub asset_id: &'static str,
    pub symbol: &'static str,
    pub name: &'static str,
    pub decimals: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct BlockchainMeta {
    pub symbol: &'static str,
    pub name: &'static str,
    pub gas_currency: GasCurrency,
}
