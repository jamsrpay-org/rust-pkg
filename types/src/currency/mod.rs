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
    EURC,
    ETH,
    POL,
    BTC,
    LTC,
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
    EURC,
    ETH,
    POL,
    BTC,
    LTC,
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
    USDT_ERC20,
    USDC_ERC20,
    EURC_ERC20,
    DAI_ERC20,
    ETH,
    POL,
    BTC,
    LTC,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum GasCurrency {
    TRX,
    BNB,
    ETH,
    POL,
    BTC,
    LTC,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum Chain {
    Tron,
    BinanceSmartChain,
    Ethereum,
    Polygon,
    Bitcoin,
    Litecoin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum ChainNetwork {
    TronMainnet,
    TronNile,
    BSCMainnet,
    BSCTestnet,
    EthMainnet,
    EthSepolia,
    PolMainnet,
    PolAmoy,
    BtcMainnet,
    BtcTestnet,
    LtcMainnet,
    LtcTestnet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum TokenStandard {
    Trc20,
    Bep20,
    Erc20,
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
