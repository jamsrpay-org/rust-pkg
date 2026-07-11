use crate::{
    currency::{
        AssetKind, BlockchainMeta, Chain, ChainNetwork, FiatCurrency, GasCurrency, PaymentCurrency,
        PaymentCurrencyMeta, PricingCurrency, PricingCurrencyMeta, TokenStandard,
    },
    money::Money,
};

impl Chain {
    pub const fn meta(&self) -> BlockchainMeta {
        match self {
            Chain::Tron => BlockchainMeta {
                symbol: "TRON",
                name: "TRON",
                gas_currency: GasCurrency::TRX,
            },
            Chain::BinanceSmartChain => BlockchainMeta {
                symbol: "BSC",
                name: "Binance Smart Chain",
                gas_currency: GasCurrency::BNB,
            },
        }
    }

    pub fn gas_currency(&self) -> GasCurrency {
        self.meta().gas_currency
    }

    pub fn symbol(&self) -> &'static str {
        self.meta().symbol
    }

    pub fn name(&self) -> &'static str {
        self.meta().name
    }
}

impl From<GasCurrency> for PaymentCurrency {
    fn from(value: GasCurrency) -> Self {
        match value {
            GasCurrency::TRX => PaymentCurrency::TRX,
            GasCurrency::BNB => PaymentCurrency::BNB,
        }
    }
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
            PaymentCurrency::BNB => PaymentCurrencyMeta {
                asset_id: "BNB",
                symbol: "BNB",
                name: "Binance Coin",
                chain: Chain::BinanceSmartChain,
                kind: AssetKind::Native,
                decimals: 8,
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
            PaymentCurrency::USDT_BEP20 => PaymentCurrencyMeta {
                asset_id: "USDT_BEP20",
                symbol: "USDT",
                name: "Tether USD",
                chain: Chain::BinanceSmartChain,
                kind: AssetKind::Token {
                    standard: TokenStandard::Bep20,
                },
                decimals: 18,
            },
            PaymentCurrency::USDC_BEP20 => PaymentCurrencyMeta {
                asset_id: "USDC_BEP20",
                symbol: "USDC",
                name: "USD Coin",
                chain: Chain::BinanceSmartChain,
                kind: AssetKind::Token {
                    standard: TokenStandard::Bep20,
                },
                decimals: 18,
            },

            PaymentCurrency::BUSD_BEP20 => PaymentCurrencyMeta {
                asset_id: "BUSD_BEP20",
                symbol: "BUSD",
                name: "Binance USD",
                chain: Chain::BinanceSmartChain,
                kind: AssetKind::Token {
                    standard: TokenStandard::Bep20,
                },
                decimals: 18,
            },
            PaymentCurrency::DAI_BEP20 => PaymentCurrencyMeta {
                asset_id: "DAI_BEP20",
                symbol: "DAI",
                name: "Dai",
                chain: Chain::BinanceSmartChain,
                kind: AssetKind::Token {
                    standard: TokenStandard::Bep20,
                },
                decimals: 18,
            },
        }
    }

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

    //
    pub fn format_money(self, money: Money) -> String {
        let formatted = money.to_formatted();
        format!("{formatted} {}", self.symbol())
    }
}

impl PaymentCurrency {
    pub fn get_base_url(&self, network: ChainNetwork) -> &'static str {
        match network {
            ChainNetwork::TronMainnet => "https://tronscan.org",
            ChainNetwork::TronNile => "https://nile.tronscan.org",
            ChainNetwork::BSCMainnet => "https://bscscan.com",
            ChainNetwork::BSCTestnet => "https://testnet.bscscan.com",
        }
    }

    pub fn address_view_url(&self, network: ChainNetwork, address: &str) -> String {
        let base_url = self.get_base_url(network);
        match network {
            ChainNetwork::TronMainnet => match self {
                PaymentCurrency::TRX | PaymentCurrency::USDT_TRC20 => {
                    format!("{}/#/address/{}", base_url, address)
                }
                _ => unreachable!(),
            },
            ChainNetwork::TronNile => match self {
                PaymentCurrency::TRX | PaymentCurrency::USDT_TRC20 => {
                    format!("{}/#/address/{}", base_url, address)
                }
                _ => unreachable!(),
            },
            ChainNetwork::BSCMainnet => match self {
                PaymentCurrency::DAI_BEP20
                | PaymentCurrency::BNB
                | PaymentCurrency::BUSD_BEP20
                | PaymentCurrency::USDC_BEP20
                | PaymentCurrency::USDT_BEP20 => {
                    format!("{}/#/address/{}", base_url, address)
                }
                _ => unreachable!(),
            },
            ChainNetwork::BSCTestnet => match self {
                PaymentCurrency::DAI_BEP20
                | PaymentCurrency::BNB
                | PaymentCurrency::BUSD_BEP20
                | PaymentCurrency::USDC_BEP20
                | PaymentCurrency::USDT_BEP20 => {
                    format!("{}/#/address/{}", base_url, address)
                }
                _ => unreachable!(),
            },
        }
    }

    pub fn transaction_view_url(&self, network: ChainNetwork, tx_id: &str) -> String {
        let base_url = self.get_base_url(network);
        match network {
            ChainNetwork::TronMainnet => match self {
                PaymentCurrency::TRX => format!("{}/#/transaction/{}", base_url, tx_id),
                PaymentCurrency::USDT_TRC20 => {
                    format!("{}/#/transaction/{}", base_url, tx_id)
                }
                _ => unreachable!(),
            },
            ChainNetwork::TronNile => match self {
                PaymentCurrency::TRX => format!("{}/#/transaction/{}", base_url, tx_id),
                PaymentCurrency::USDT_TRC20 => {
                    format!("{}/#/transaction/{}", base_url, tx_id)
                }
                _ => unreachable!(),
            },
            ChainNetwork::BSCMainnet => match self {
                PaymentCurrency::DAI_BEP20
                | PaymentCurrency::BNB
                | PaymentCurrency::BUSD_BEP20
                | PaymentCurrency::USDT_BEP20 => {
                    format!("{}/#/tx/{}", base_url, tx_id)
                }
                _ => unreachable!(),
            },
            ChainNetwork::BSCTestnet => match self {
                PaymentCurrency::DAI_BEP20
                | PaymentCurrency::BNB
                | PaymentCurrency::BUSD_BEP20
                | PaymentCurrency::USDC_BEP20
                | PaymentCurrency::USDT_BEP20 => {
                    format!("{}/#/tx/{}", base_url, tx_id)
                }
                _ => unreachable!(),
            },
        }
    }
}

impl PaymentCurrency {
    pub fn min_payment_amount(&self) -> Money {
        match self {
            PaymentCurrency::TRX => Money::from_atomic(3_000_000, self.decimals()),
            PaymentCurrency::USDT_TRC20 => Money::from_atomic(5_000_000, self.decimals()),
            PaymentCurrency::BNB => Money::from_atomic(1, 1),
            PaymentCurrency::USDT_BEP20
            | PaymentCurrency::USDC_BEP20
            | PaymentCurrency::DAI_BEP20
            | PaymentCurrency::BUSD_BEP20 => Money::from_atomic(1, 0),
        }
    }

    pub fn fixed_network_fee(&self) -> Money {
        match self {
            PaymentCurrency::TRX => Money::from_atomic(2_000_000, self.decimals()),
            PaymentCurrency::USDT_TRC20 => Money::from_atomic(0, self.decimals()),
            PaymentCurrency::BNB
            | PaymentCurrency::USDT_BEP20
            | PaymentCurrency::USDC_BEP20
            | PaymentCurrency::DAI_BEP20
            | PaymentCurrency::BUSD_BEP20 => Money::from_atomic(0, self.decimals()),
        }
    }
}

impl From<PaymentCurrency> for Chain {
    fn from(value: PaymentCurrency) -> Self {
        match value {
            PaymentCurrency::BNB
            | PaymentCurrency::USDT_BEP20
            | PaymentCurrency::USDC_BEP20
            | PaymentCurrency::DAI_BEP20
            | PaymentCurrency::BUSD_BEP20 => Chain::BinanceSmartChain,
            PaymentCurrency::TRX | PaymentCurrency::USDT_TRC20 => Chain::Tron,
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

    //
    pub fn format_money(self, money: Money) -> String {
        let formatted = money.to_formatted();
        format!("{formatted} {}", self.symbol())
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
            PricingCurrency::USDT => PricingCurrencyMeta {
                asset_id: "USDT",
                symbol: "USDT",
                name: "Tether USD",
                decimals: 6,
            },
            PricingCurrency::BNB => {
                let meta = PaymentCurrency::BNB.meta();
                PricingCurrencyMeta {
                    asset_id: meta.asset_id,
                    symbol: meta.symbol,
                    name: meta.name,
                    decimals: meta.decimals,
                }
            }
            PricingCurrency::USDC => PricingCurrencyMeta {
                asset_id: "USDC",
                symbol: "USDC",
                name: "USD Coin",
                decimals: 18,
            },
            PricingCurrency::DAI => PricingCurrencyMeta {
                asset_id: "DAI",
                symbol: "DAI",
                name: "Dai",
                decimals: 18,
            },
            PricingCurrency::BUSD => PricingCurrencyMeta {
                asset_id: "BUSD",
                symbol: "BUSD",
                name: "Binance USD",
                decimals: 18,
            },
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

    //
    pub fn format_money(self, money: Money) -> String {
        let formatted = money.to_formatted();
        format!("{formatted} {}", self.symbol())
    }
}
