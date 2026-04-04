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
    Native,
    Trc20,
}

#[derive(Debug, Clone, Copy)]
pub struct CryptoMeta {
    pub asset: &'static str,
    pub symbol: &'static str,
    pub name: &'static str,
    pub decimals: u8,
}

impl PaymentCurrency {
    pub const fn meta(&self) -> CryptoMeta {
        match self {
            PaymentCurrency::TRX => CryptoMeta {
                asset: "TRX",
                symbol: "TRX",
                name: "Tron",
                decimals: 6,
            },
            PaymentCurrency::USDT_TRC20 => CryptoMeta {
                asset: "USDT",
                symbol: "USDT",
                name: "Tether",
                decimals: 6,
            },
        }
    }
}

impl PaymentCurrency {
    pub fn asset(&self) -> &'static str {
        self.meta().asset
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

impl PaymentCurrency{
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

//  PricingCurrency
impl PricingCurrency {
    pub const fn meta(&self) -> CryptoMeta {
        match *self {
            PricingCurrency::TRX => CryptoMeta {
                asset: "TRX",
                symbol: "TRX",
                name: "Tron",
                decimals: 6,
            },
            PricingCurrency::USDT => CryptoMeta {
                asset: "USDT",
                symbol: "USDT",
                name: "Tether",
                decimals: 6,
            },
            // Fiat Currencies
            PricingCurrency::USD => CryptoMeta {
                asset: "USD",
                symbol: "$",
                name: "US Dollar",
                decimals: 2,
            },
            PricingCurrency::EUR => CryptoMeta {
                asset: "EUR",
                symbol: "€",
                name: "Euro",
                decimals: 2,
            },
            PricingCurrency::GBP => CryptoMeta {
                asset: "GBP",
                symbol: "£",
                name: "British Pound",
                decimals: 2,
            },
            PricingCurrency::CAD => CryptoMeta {
                asset: "CAD",
                symbol: "CA$",
                name: "Canadian Dollar",
                decimals: 2,
            },
            PricingCurrency::CHF => CryptoMeta {
                asset: "CHF",
                symbol: "CHF",
                name: "Swiss Franc",
                decimals: 2,
            },
            PricingCurrency::HKD => CryptoMeta {
                asset: "HKD",
                symbol: "HK$",
                name: "Hong Kong Dollar",
                decimals: 2,
            },
            PricingCurrency::ILS => CryptoMeta {
                asset: "ILS",
                symbol: "₪",
                name: "Israeli Shekel",
                decimals: 2,
            },
            PricingCurrency::INR => CryptoMeta {
                asset: "INR",
                symbol: "₹",
                name: "Indian Rupee",
                decimals: 2,
            },
            PricingCurrency::JPY => CryptoMeta {
                asset: "JPY",
                symbol: "¥",
                name: "Japanese Yen",
                decimals: 0,
            },
            PricingCurrency::PHP => CryptoMeta {
                asset: "PHP",
                symbol: "₱",
                name: "Philippine Peso",
                decimals: 2,
            },
        }
    }
}

impl PricingCurrency {
    pub fn asset(&self) -> &'static str {
        self.meta().asset
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
