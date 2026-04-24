use strum::AsRefStr;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, AsRefStr)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, AsRefStr)]
pub enum CryptoAsset {
    TRX,
    USDT,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, AsRefStr)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, AsRefStr)]
pub enum PaymentCurrency {
    TRX,
    USDT_TRC20,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Chain {
    Tron,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub chain: Chain,
    pub kind: AssetKind,
    pub decimals: u8,
}

pub struct PricingCurrencyMeta {
    pub asset_id: &'static str,
    pub symbol: &'static str,
    pub name: &'static str,
    pub decimals: u8,
}

impl PaymentCurrency {
    pub const fn meta(&self) -> PaymentCurrencyMeta {
        match self {
            PaymentCurrency::TRX => PaymentCurrencyMeta {
                asset_id: "TRX",
                symbol: "TRX",
                name: "TRX",
                chain: Chain::Tron,
                kind: AssetKind::Native,
                decimals: 6,
            },
            PaymentCurrency::USDT_TRC20 => PaymentCurrencyMeta {
                asset_id: "USDT_TRC20",
                symbol: "USDT",
                name: "Tether USD",
                chain: Chain::Tron,
                kind: AssetKind::Token {
                    standard: TokenStandard::Trc20,
                },
                decimals: 6,
            },
        }
    }
}

impl PaymentCurrency {
    pub fn asset(&self) -> &'static str {
        self.meta().asset_id
    }

    pub fn symbol(&self) -> &'static str {
        self.meta().symbol
    }

    pub fn name(&self) -> &'static str {
        self.meta().name
    }

    pub fn decimals(&self) -> u8 {
        self.meta().decimals
    }

    pub fn chain(&self) -> Chain {
        self.meta().chain
    }

    pub fn kind(&self) -> AssetKind {
        self.meta().kind
    }
}

impl PaymentCurrency {
    pub fn address_view_url(&self, address: &str) -> String {
        match self {
            PaymentCurrency::TRX => format!("https://tronscan.org/#/address/{}", address),
            PaymentCurrency::USDT_TRC20 => format!("https://tronscan.org/#/address/{}", address),
        }
    }

    pub fn transaction_view_url(&self, tx_id: &str) -> String {
        match self {
            PaymentCurrency::TRX => format!("https://tronscan.org/#/transaction/{}", tx_id),
            PaymentCurrency::USDT_TRC20 => format!("https://tronscan.org/#/transaction/{}", tx_id),
        }
    }
}

// FiatCurrency
impl FiatCurrency {
    pub const fn meta(&self) -> PricingCurrencyMeta {
        match *self {
            FiatCurrency::USD => PricingCurrencyMeta {
                asset_id: "USD",
                symbol: "$",
                name: "US Dollar",
                decimals: 2,
            },
            FiatCurrency::EUR => PricingCurrencyMeta {
                asset_id: "EUR",
                symbol: "€",
                name: "Euro",
                decimals: 2,
            },
            FiatCurrency::GBP => PricingCurrencyMeta {
                asset_id: "GBP",
                symbol: "£",
                name: "British Pound",
                decimals: 2,
            },
            FiatCurrency::CAD => PricingCurrencyMeta {
                asset_id: "CAD",
                symbol: "CA$",
                name: "Canadian Dollar",
                decimals: 2,
            },
            FiatCurrency::CHF => PricingCurrencyMeta {
                asset_id: "CHF",
                symbol: "CHF",
                name: "Swiss Franc",
                decimals: 2,
            },
            FiatCurrency::HKD => PricingCurrencyMeta {
                asset_id: "HKD",
                symbol: "HK$",
                name: "Hong Kong Dollar",
                decimals: 2,
            },
            FiatCurrency::ILS => PricingCurrencyMeta {
                asset_id: "ILS",
                symbol: "₪",
                name: "Israeli Shekel",
                decimals: 2,
            },
            FiatCurrency::INR => PricingCurrencyMeta {
                asset_id: "INR",
                symbol: "₹",
                name: "Indian Rupee",
                decimals: 2,
            },
            FiatCurrency::JPY => PricingCurrencyMeta {
                asset_id: "JPY",
                symbol: "¥",
                name: "Japanese Yen",
                decimals: 0,
            },
            FiatCurrency::PHP => PricingCurrencyMeta {
                asset_id: "PHP",
                symbol: "₱",
                name: "Philippine Peso",
                decimals: 2,
            },
        }
    }
}

impl FiatCurrency {
    pub fn asset(&self) -> &'static str {
        self.meta().asset_id
    }

    pub fn symbol(&self) -> &'static str {
        self.meta().symbol
    }

    pub fn name(&self) -> &'static str {
        self.meta().name
    }

    pub fn decimals(&self) -> u8 {
        self.meta().decimals
    }
}

//  PricingCurrency
impl PricingCurrency {
    pub const fn meta(&self) -> PricingCurrencyMeta {
        match *self {
            PricingCurrency::TRX => {
                let meta = PaymentCurrency::TRX.meta();
                PricingCurrencyMeta {
                    asset_id: meta.asset_id,
                    symbol: meta.symbol,
                    name: meta.name,
                    decimals: meta.decimals,
                }
            }
            PricingCurrency::USDT => {
                let meta = PaymentCurrency::USDT_TRC20.meta();
                PricingCurrencyMeta {
                    asset_id: meta.asset_id,
                    symbol: meta.symbol,
                    name: meta.name,
                    decimals: meta.decimals,
                }
            }
            // Fiat Currencies
            PricingCurrency::USD => {
                let fiat_meta = FiatCurrency::USD.meta();
                PricingCurrencyMeta {
                    asset_id: fiat_meta.asset_id,
                    symbol: fiat_meta.symbol,
                    name: fiat_meta.name,
                    decimals: fiat_meta.decimals,
                }
            }
            PricingCurrency::EUR => {
                let fiat_meta = FiatCurrency::EUR.meta();
                PricingCurrencyMeta {
                    asset_id: fiat_meta.asset_id,
                    symbol: fiat_meta.symbol,
                    name: fiat_meta.name,
                    decimals: fiat_meta.decimals,
                }
            }
            PricingCurrency::GBP => {
                let fiat_meta = FiatCurrency::GBP.meta();
                PricingCurrencyMeta {
                    asset_id: fiat_meta.asset_id,
                    symbol: fiat_meta.symbol,
                    name: fiat_meta.name,
                    decimals: fiat_meta.decimals,
                }
            }
            PricingCurrency::CAD => {
                let fiat_meta = FiatCurrency::CAD.meta();
                PricingCurrencyMeta {
                    asset_id: fiat_meta.asset_id,
                    symbol: fiat_meta.symbol,
                    name: fiat_meta.name,
                    decimals: fiat_meta.decimals,
                }
            }
            PricingCurrency::CHF => {
                let fiat_meta = FiatCurrency::CHF.meta();
                PricingCurrencyMeta {
                    asset_id: fiat_meta.asset_id,
                    symbol: fiat_meta.symbol,
                    name: fiat_meta.name,
                    decimals: fiat_meta.decimals,
                }
            }
            PricingCurrency::HKD => {
                let fiat_meta = FiatCurrency::HKD.meta();
                PricingCurrencyMeta {
                    asset_id: fiat_meta.asset_id,
                    symbol: fiat_meta.symbol,
                    name: fiat_meta.name,
                    decimals: fiat_meta.decimals,
                }
            }
            PricingCurrency::ILS => {
                let fiat_meta = FiatCurrency::ILS.meta();
                PricingCurrencyMeta {
                    asset_id: fiat_meta.asset_id,
                    symbol: fiat_meta.symbol,
                    name: fiat_meta.name,
                    decimals: fiat_meta.decimals,
                }
            }
            PricingCurrency::INR => {
                let fiat_meta = FiatCurrency::INR.meta();
                PricingCurrencyMeta {
                    asset_id: fiat_meta.asset_id,
                    symbol: fiat_meta.symbol,
                    name: fiat_meta.name,
                    decimals: fiat_meta.decimals,
                }
            }
            PricingCurrency::JPY => {
                let fiat_meta = FiatCurrency::JPY.meta();
                PricingCurrencyMeta {
                    asset_id: fiat_meta.asset_id,
                    symbol: fiat_meta.symbol,
                    name: fiat_meta.name,
                    decimals: fiat_meta.decimals,
                }
            }
            PricingCurrency::PHP => {
                let fiat_meta = FiatCurrency::PHP.meta();
                PricingCurrencyMeta {
                    asset_id: fiat_meta.asset_id,
                    symbol: fiat_meta.symbol,
                    name: fiat_meta.name,
                    decimals: fiat_meta.decimals,
                }
            }
        }
    }
}

impl PricingCurrency {
    pub fn asset(&self) -> &'static str {
        self.meta().asset_id
    }

    pub fn symbol(&self) -> &'static str {
        self.meta().symbol
    }

    pub fn name(&self) -> &'static str {
        self.meta().name
    }

    pub fn decimals(&self) -> u8 {
        self.meta().decimals
    }
}
