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
    BNB,
    USDT,
    USDC,
    BUSD,
    DAI,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum PricingCurrency {
    // Crypto Assets
    TRX,
    BNB,
    USDT,
    USDC,
    BUSD,
    DAI,
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
    BNB,
    USDT_TRC20,
    USDT_BEP20,
    USDC_BEP20,
    BUSD_BEP20,
    DAI_BEP20,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum GasCurrency {
    TRX,
    BNB,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum Chain {
    Tron,
    BinanceSmartChain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum ChainNetwork {
    TronMainnet,
    TronNile,
    BSCMainnet,
    BSCTestnet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum TokenStandard {
    Trc20,
    Bep20,
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
    pub chain: Chain,
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
