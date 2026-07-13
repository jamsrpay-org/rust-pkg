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
            Chain::Ethereum => BlockchainMeta {
                symbol: "ETH",
                name: "Ethereum",
                gas_currency: GasCurrency::ETH,
            },
            Chain::Polygon => BlockchainMeta {
                symbol: "POL",
                name: "Polygon",
                gas_currency: GasCurrency::POL,
            },
            Chain::Bitcoin => BlockchainMeta {
                symbol: "BTC",
                name: "Bitcoin",
                gas_currency: GasCurrency::BTC,
            },
            Chain::Litecoin => BlockchainMeta {
                symbol: "LTC",
                name: "Litecoin",
                gas_currency: GasCurrency::LTC,
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
            GasCurrency::ETH | GasCurrency::POL | GasCurrency::BTC | GasCurrency::LTC => {
                unimplemented!("PaymentCurrency not yet supported for {:?}", value)
            }
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
                decimals: 18,
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
            PaymentCurrency::USDT_ERC20 => PaymentCurrencyMeta {
                asset_id: "USDT_ERC20",
                symbol: "USDT",
                name: "Tether USD",
                chain: Chain::Ethereum,
                kind: AssetKind::Token {
                    standard: TokenStandard::Erc20,
                },
                decimals: 6,
            },
            PaymentCurrency::USDC_ERC20 => PaymentCurrencyMeta {
                asset_id: "USDC_ERC20",
                symbol: "USDC",
                name: "USD Coin",
                chain: Chain::Ethereum,
                kind: AssetKind::Token {
                    standard: TokenStandard::Erc20,
                },
                decimals: 6,
            },
            PaymentCurrency::EURC_ERC20 => PaymentCurrencyMeta {
                asset_id: "EURC_ERC20",
                symbol: "EURC",
                name: "EURC",
                chain: Chain::Ethereum,
                kind: AssetKind::Token {
                    standard: TokenStandard::Erc20,
                },
                decimals: 6,
            },
            PaymentCurrency::DAI_ERC20 => PaymentCurrencyMeta {
                asset_id: "DAI_ERC20",
                symbol: "DAI",
                name: "Dai",
                chain: Chain::Ethereum,
                kind: AssetKind::Token {
                    standard: TokenStandard::Erc20,
                },
                decimals: 18,
            },
            PaymentCurrency::BTC => PaymentCurrencyMeta {
                asset_id: "BTC",
                symbol: "BTC",
                name: "Bitcoin",
                chain: Chain::Bitcoin,
                kind: AssetKind::Native,
                decimals: 8,
            },
            PaymentCurrency::LTC => PaymentCurrencyMeta {
                asset_id: "LTC",
                symbol: "LTC",
                name: "Litecoin",
                chain: Chain::Litecoin,
                kind: AssetKind::Native,
                decimals: 8,
            },
            PaymentCurrency::ETH => PaymentCurrencyMeta {
                asset_id: "ETH",
                symbol: "ETH",
                name: "Ethereum",
                chain: Chain::Ethereum,
                kind: AssetKind::Native,
                decimals: 18,
            },
            PaymentCurrency::POL => PaymentCurrencyMeta {
                asset_id: "POL",
                symbol: "POL",
                name: "Polygon",
                chain: Chain::Polygon,
                kind: AssetKind::Native,
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
            ChainNetwork::BtcMainnet => "https://btc.com",
            ChainNetwork::BtcTestnet => "https://testnet.btc.com",
            ChainNetwork::EthMainnet => "https://eth.com",
            ChainNetwork::EthSepolia => "https://testnet.eth.com",
            ChainNetwork::LtcMainnet => "https://ltc.com",
            ChainNetwork::LtcTestnet => "https://testnet.ltc.com",
            ChainNetwork::PolMainnet => "https://pol.com",
            ChainNetwork::PolAmoy => "https://testnet.pol.com",
        }
    }

    pub fn address_view_url(&self, network: ChainNetwork, address: &str) -> String {
        let base_url = self.get_base_url(network);
        match network {
            ChainNetwork::TronMainnet => format!("{}/#/address/{}", base_url, address),
            ChainNetwork::TronNile => format!("{}/#/address/{}", base_url, address),
            ChainNetwork::BSCMainnet => format!("{}/address/{}", base_url, address),
            ChainNetwork::BSCTestnet => format!("{}/address/{}", base_url, address),
            ChainNetwork::EthMainnet => format!("{}/address/{}", base_url, address),
            ChainNetwork::EthSepolia => format!("{}/address/{}", base_url, address),
            ChainNetwork::PolMainnet => format!("{}/address/{}", base_url, address),
            ChainNetwork::PolAmoy => format!("{}/address/{}", base_url, address),
            ChainNetwork::BtcMainnet => format!("{}/address/{}", base_url, address),
            ChainNetwork::BtcTestnet => format!("{}/address/{}", base_url, address),
            ChainNetwork::LtcMainnet => format!("{}/address/{}", base_url, address),
            ChainNetwork::LtcTestnet => format!("{}/address/{}", base_url, address),
        }
    }

    pub fn transaction_view_url(&self, network: ChainNetwork, tx_id: &str) -> String {
        let base_url = self.get_base_url(network);
        match network {
            ChainNetwork::TronMainnet => format!("{}/#/transaction/{}", base_url, tx_id),
            ChainNetwork::TronNile => format!("{}/#/transaction/{}", base_url, tx_id),
            ChainNetwork::BSCMainnet => format!("{}/tx/{}", base_url, tx_id),
            ChainNetwork::BSCTestnet => format!("{}/tx/{}", base_url, tx_id),
            ChainNetwork::EthMainnet => format!("{}/tx/{}", base_url, tx_id),
            ChainNetwork::EthSepolia => format!("{}/tx/{}", base_url, tx_id),
            ChainNetwork::PolMainnet => format!("{}/tx/{}", base_url, tx_id),
            ChainNetwork::PolAmoy => format!("{}/tx/{}", base_url, tx_id),
            ChainNetwork::BtcMainnet => format!("{}/tx/{}", base_url, tx_id),
            ChainNetwork::BtcTestnet => format!("{}/tx/{}", base_url, tx_id),
            ChainNetwork::LtcMainnet => format!("{}/tx/{}", base_url, tx_id),
            ChainNetwork::LtcTestnet => format!("{}/tx/{}", base_url, tx_id),
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
            | PaymentCurrency::BUSD_BEP20
            | PaymentCurrency::USDT_ERC20
            | PaymentCurrency::USDC_ERC20
            | PaymentCurrency::DAI_ERC20
            | PaymentCurrency::EURC_ERC20 => Money::from_atomic(1, 0),
            PaymentCurrency::BTC => Money::from_atomic(1, 0),
            PaymentCurrency::LTC => Money::from_atomic(1, 0),
            PaymentCurrency::ETH => Money::from_atomic(1, 0),
            PaymentCurrency::POL => Money::from_atomic(1, 0),
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
            | PaymentCurrency::BUSD_BEP20
            | PaymentCurrency::USDT_ERC20
            | PaymentCurrency::USDC_ERC20
            | PaymentCurrency::DAI_ERC20
            | PaymentCurrency::EURC_ERC20 => Money::from_atomic(0, self.decimals()),
            PaymentCurrency::BTC => Money::from_atomic(0, self.decimals()),
            PaymentCurrency::LTC => Money::from_atomic(0, self.decimals()),
            PaymentCurrency::ETH => Money::from_atomic(0, self.decimals()),
            PaymentCurrency::POL => Money::from_atomic(0, self.decimals()),
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
            PaymentCurrency::USDT_ERC20
            | PaymentCurrency::USDC_ERC20
            | PaymentCurrency::DAI_ERC20
            | PaymentCurrency::EURC_ERC20 => Chain::Ethereum,
            PaymentCurrency::BTC => Chain::Bitcoin,
            PaymentCurrency::LTC => Chain::Litecoin,
            PaymentCurrency::ETH => Chain::Ethereum,
            PaymentCurrency::POL => Chain::Polygon,
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
            PricingCurrency::EURC => PricingCurrencyMeta {
                asset_id: "EURC",
                symbol: "EURC",
                name: "EURC",
                decimals: 6,
            },
            PricingCurrency::ETH => PricingCurrencyMeta {
                asset_id: "ETH",
                symbol: "ETH",
                name: "Ethereum",
                decimals: 18,
            },
            PricingCurrency::POL => PricingCurrencyMeta {
                asset_id: "POL",
                symbol: "POL",
                name: "Polygon",
                decimals: 18,
            },
            PricingCurrency::BTC => PricingCurrencyMeta {
                asset_id: "BTC",
                symbol: "BTC",
                name: "Bitcoin",
                decimals: 8,
            },
            PricingCurrency::LTC => PricingCurrencyMeta {
                asset_id: "LTC",
                symbol: "LTC",
                name: "Litecoin",
                decimals: 8,
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
