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
pub enum PricingCurrency {
    // Crypto Currencies
    TRX,
    BNB,
    USDT,
    USDC,
    DAI,
    EURC,
    ETH,
    POL,
    BTC,
    LTC,
    SOL,
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
    USDT,
    USDC,
    DAI,
    EURC,
    ETH,
    POL,
    BTC,
    LTC,
    SOL,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum GasCurrency {
    TRX,
    BNB,
    ETH,
    POL,
    SOL,
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
    Solana,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum NetworkId {
    TronMainnet,
    TronNile,

    BscMainnet,
    BscTestnet,

    EthMainnet,
    EthSepolia,

    PolMainnet,
    PolAmoy,

    BtcMainnet,
    BtcTestnet,

    LtcMainnet,
    LtcTestnet,

    SolMainnet,
    SolDevnet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum TokenStandard {
    Trc20,
    Bep20,
    Erc20,
    Spl,
}

#[derive(Debug, Clone, Copy)]
pub enum AssetKind {
    Native,
    Token { standard: TokenStandard },
}

#[derive(Debug, Clone, Copy)]
pub struct PaymentCurrencyNetwork {
    pub network_id: NetworkId,
    pub chain: Chain,
    pub standard: Option<TokenStandard>,
    pub is_native: bool,
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
    pub gas_currency: Option<GasCurrency>,
}
