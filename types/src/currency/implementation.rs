use crate::{
    currency::{
        AssetKind, BlockchainMeta, Chain, FiatCurrency, GasCurrency, NetworkId, PaymentCurrency,
        PaymentCurrencyMeta, PaymentCurrencyNetwork, PricingCurrency, PricingCurrencyMeta,
        TokenStandard,
    },
    money::Money,
};

impl Chain {
    pub const fn meta(&self) -> BlockchainMeta {
        match self {
            Chain::Tron => BlockchainMeta {
                symbol: "TRON",
                name: "TRON",
                gas_currency: Some(GasCurrency::TRX),
            },
            Chain::BinanceSmartChain => BlockchainMeta {
                symbol: "BSC",
                name: "Binance Smart Chain",
                gas_currency: Some(GasCurrency::BNB),
            },
            Chain::Ethereum => BlockchainMeta {
                symbol: "ETH",
                name: "Ethereum",
                gas_currency: Some(GasCurrency::ETH),
            },
            Chain::Polygon => BlockchainMeta {
                symbol: "POL",
                name: "Polygon",
                gas_currency: Some(GasCurrency::POL),
            },
            Chain::Bitcoin => BlockchainMeta {
                symbol: "BTC",
                name: "Bitcoin",
                gas_currency: None,
            },
            Chain::Litecoin => BlockchainMeta {
                symbol: "LTC",
                name: "Litecoin",
                gas_currency: None,
            },
        }
    }

    pub fn gas_currency(&self) -> Option<GasCurrency> {
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
            GasCurrency::ETH => PaymentCurrency::ETH,
            GasCurrency::POL => PaymentCurrency::POL,
        }
    }
}

impl PaymentCurrency {
    pub const fn gas_currency(&self) -> Option<Self> {
        match *self {
            PaymentCurrency::BNB => Some(PaymentCurrency::BNB),
            PaymentCurrency::ETH => Some(PaymentCurrency::ETH),
            PaymentCurrency::POL => Some(PaymentCurrency::POL),
            PaymentCurrency::TRX => Some(PaymentCurrency::TRX),
            PaymentCurrency::USDT => None,
            PaymentCurrency::USDC => None,
            PaymentCurrency::BUSD => None,
            PaymentCurrency::DAI => None,
            PaymentCurrency::EURC => None,
            PaymentCurrency::BTC => None,
            PaymentCurrency::LTC => None,
        }
    }

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
            PaymentCurrency::USDT => PaymentCurrencyMeta {
                asset_id: "USDT",
                symbol: "USDT",
                name: "Tether USD",
                chain: Chain::Tron,
                kind: AssetKind::Token {
                    standard: TokenStandard::Trc20,
                },
                decimals: 6,
            },
            PaymentCurrency::USDC => PaymentCurrencyMeta {
                asset_id: "USDC",
                symbol: "USDC",
                name: "USD Coin",
                chain: Chain::BinanceSmartChain,
                kind: AssetKind::Token {
                    standard: TokenStandard::Bep20,
                },
                decimals: 18,
            },
            PaymentCurrency::BUSD => PaymentCurrencyMeta {
                asset_id: "BUSD",
                symbol: "BUSD",
                name: "Binance USD",
                chain: Chain::BinanceSmartChain,
                kind: AssetKind::Token {
                    standard: TokenStandard::Bep20,
                },
                decimals: 18,
            },
            PaymentCurrency::DAI => PaymentCurrencyMeta {
                asset_id: "DAI",
                symbol: "DAI",
                name: "Dai",
                chain: Chain::BinanceSmartChain,
                kind: AssetKind::Token {
                    standard: TokenStandard::Bep20,
                },
                decimals: 18,
            },
            PaymentCurrency::EURC => PaymentCurrencyMeta {
                asset_id: "EURC",
                symbol: "EURC",
                name: "EURC",
                chain: Chain::Ethereum,
                kind: AssetKind::Token {
                    standard: TokenStandard::Erc20,
                },
                decimals: 6,
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

    pub fn networks(&self) -> &'static [PaymentCurrencyNetwork] {
        use PaymentCurrencyNetwork as N;
        match self {
            PaymentCurrency::TRX => &[N {
                network_id: NetworkId::TronMainnet,
                chain: Chain::Tron,
                standard: None,
                is_native: true,
            }],
            PaymentCurrency::BNB => &[N {
                network_id: NetworkId::BscMainnet,
                chain: Chain::BinanceSmartChain,
                standard: None,
                is_native: true,
            }],
            PaymentCurrency::ETH => &[N {
                network_id: NetworkId::EthMainnet,
                chain: Chain::Ethereum,
                standard: None,
                is_native: true,
            }],
            PaymentCurrency::POL => &[N {
                network_id: NetworkId::PolMainnet,
                chain: Chain::Polygon,
                standard: None,
                is_native: true,
            }],
            PaymentCurrency::BTC => &[N {
                network_id: NetworkId::BtcMainnet,
                chain: Chain::Bitcoin,
                standard: None,
                is_native: true,
            }],
            PaymentCurrency::LTC => &[N {
                network_id: NetworkId::LtcMainnet,
                chain: Chain::Litecoin,
                standard: None,
                is_native: true,
            }],
            PaymentCurrency::USDT => &[
                N {
                    network_id: NetworkId::TronMainnet,
                    chain: Chain::Tron,
                    standard: Some(TokenStandard::Trc20),
                    is_native: false,
                },
                N {
                    network_id: NetworkId::BscMainnet,
                    chain: Chain::BinanceSmartChain,
                    standard: Some(TokenStandard::Bep20),
                    is_native: false,
                },
                N {
                    network_id: NetworkId::EthMainnet,
                    chain: Chain::Ethereum,
                    standard: Some(TokenStandard::Erc20),
                    is_native: false,
                },
                N {
                    network_id: NetworkId::PolMainnet,
                    chain: Chain::Polygon,
                    standard: Some(TokenStandard::Erc20),
                    is_native: false,
                },
            ],
            PaymentCurrency::USDC => &[
                N {
                    network_id: NetworkId::TronMainnet,
                    chain: Chain::Tron,
                    standard: Some(TokenStandard::Trc20),
                    is_native: false,
                },
                N {
                    network_id: NetworkId::BscMainnet,
                    chain: Chain::BinanceSmartChain,
                    standard: Some(TokenStandard::Bep20),
                    is_native: false,
                },
                N {
                    network_id: NetworkId::EthMainnet,
                    chain: Chain::Ethereum,
                    standard: Some(TokenStandard::Erc20),
                    is_native: false,
                },
                N {
                    network_id: NetworkId::PolMainnet,
                    chain: Chain::Polygon,
                    standard: Some(TokenStandard::Erc20),
                    is_native: false,
                },
            ],
            PaymentCurrency::DAI => &[
                N {
                    network_id: NetworkId::BscMainnet,
                    chain: Chain::BinanceSmartChain,
                    standard: Some(TokenStandard::Bep20),
                    is_native: false,
                },
                N {
                    network_id: NetworkId::EthMainnet,
                    chain: Chain::Ethereum,
                    standard: Some(TokenStandard::Erc20),
                    is_native: false,
                },
                N {
                    network_id: NetworkId::PolMainnet,
                    chain: Chain::Polygon,
                    standard: Some(TokenStandard::Erc20),
                    is_native: false,
                },
            ],
            PaymentCurrency::BUSD => &[N {
                network_id: NetworkId::BscMainnet,
                chain: Chain::BinanceSmartChain,
                standard: Some(TokenStandard::Bep20),
                is_native: false,
            }],
            PaymentCurrency::EURC => &[N {
                network_id: NetworkId::EthMainnet,
                chain: Chain::Ethereum,
                standard: Some(TokenStandard::Erc20),
                is_native: false,
            }],
        }
    }
}

impl PaymentCurrency {
    pub fn get_base_url(&self, network: NetworkId) -> &'static str {
        match network {
            NetworkId::TronMainnet => "https://tronscan.org",
            NetworkId::TronNile => "https://nile.tronscan.org",
            NetworkId::BscMainnet => "https://bscscan.com",
            NetworkId::BscTestnet => "https://testnet.bscscan.com",
            NetworkId::BtcMainnet => "https://btc.com",
            NetworkId::BtcTestnet => "https://testnet.btc.com",
            NetworkId::EthMainnet => "https://eth.com",
            NetworkId::EthSepolia => "https://testnet.eth.com",
            NetworkId::LtcMainnet => "https://ltc.com",
            NetworkId::LtcTestnet => "https://testnet.ltc.com",
            NetworkId::PolMainnet => "https://pol.com",
            NetworkId::PolAmoy => "https://testnet.pol.com",
        }
    }

    pub fn address_view_url(&self, network: NetworkId, address: &str) -> String {
        let base_url = self.get_base_url(network);
        match network {
            NetworkId::TronMainnet => format!("{}/#/address/{}", base_url, address),
            NetworkId::TronNile => format!("{}/#/address/{}", base_url, address),
            NetworkId::BscMainnet => format!("{}/address/{}", base_url, address),
            NetworkId::BscTestnet => format!("{}/address/{}", base_url, address),
            NetworkId::EthMainnet => format!("{}/address/{}", base_url, address),
            NetworkId::EthSepolia => format!("{}/address/{}", base_url, address),
            NetworkId::PolMainnet => format!("{}/address/{}", base_url, address),
            NetworkId::PolAmoy => format!("{}/address/{}", base_url, address),
            NetworkId::BtcMainnet => format!("{}/address/{}", base_url, address),
            NetworkId::BtcTestnet => format!("{}/address/{}", base_url, address),
            NetworkId::LtcMainnet => format!("{}/address/{}", base_url, address),
            NetworkId::LtcTestnet => format!("{}/address/{}", base_url, address),
        }
    }

    pub fn transaction_view_url(&self, network: NetworkId, tx_id: &str) -> String {
        let base_url = self.get_base_url(network);
        match network {
            NetworkId::TronMainnet => format!("{}/#/transaction/{}", base_url, tx_id),
            NetworkId::TronNile => format!("{}/#/transaction/{}", base_url, tx_id),
            NetworkId::BscMainnet => format!("{}/tx/{}", base_url, tx_id),
            NetworkId::BscTestnet => format!("{}/tx/{}", base_url, tx_id),
            NetworkId::EthMainnet => format!("{}/tx/{}", base_url, tx_id),
            NetworkId::EthSepolia => format!("{}/tx/{}", base_url, tx_id),
            NetworkId::PolMainnet => format!("{}/tx/{}", base_url, tx_id),
            NetworkId::PolAmoy => format!("{}/tx/{}", base_url, tx_id),
            NetworkId::BtcMainnet => format!("{}/tx/{}", base_url, tx_id),
            NetworkId::BtcTestnet => format!("{}/tx/{}", base_url, tx_id),
            NetworkId::LtcMainnet => format!("{}/tx/{}", base_url, tx_id),
            NetworkId::LtcTestnet => format!("{}/tx/{}", base_url, tx_id),
        }
    }
}

impl PaymentCurrency {
    pub fn min_payment_amount(&self) -> Money {
        match self {
            PaymentCurrency::TRX => Money::from_atomic(3_000_000, self.decimals()),
            PaymentCurrency::USDT => Money::from_atomic(5_000_000, self.decimals()),
            PaymentCurrency::BNB => Money::from_atomic(1, 1),
            PaymentCurrency::USDC
            | PaymentCurrency::DAI
            | PaymentCurrency::BUSD
            | PaymentCurrency::EURC => Money::from_atomic(1, 0),
            PaymentCurrency::BTC => Money::from_atomic(1, 0),
            PaymentCurrency::LTC => Money::from_atomic(1, 0),
            PaymentCurrency::ETH => Money::from_atomic(1, 0),
            PaymentCurrency::POL => Money::from_atomic(1, 0),
        }
    }

    pub fn fixed_network_fee(&self) -> Money {
        match self {
            PaymentCurrency::TRX => Money::from_atomic(2_000_000, self.decimals()),
            PaymentCurrency::USDT => Money::from_atomic(0, self.decimals()),
            PaymentCurrency::BNB
            | PaymentCurrency::USDC
            | PaymentCurrency::DAI
            | PaymentCurrency::BUSD
            | PaymentCurrency::EURC => Money::from_atomic(0, self.decimals()),
            PaymentCurrency::BTC => Money::from_atomic(0, self.decimals()),
            PaymentCurrency::LTC => Money::from_atomic(0, self.decimals()),
            PaymentCurrency::ETH => Money::from_atomic(0, self.decimals()),
            PaymentCurrency::POL => Money::from_atomic(0, self.decimals()),
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
        format!("{formatted} {}", self.asset())
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

impl NetworkId {
    pub const fn is_testnet(self) -> bool {
        matches!(
            self,
            Self::TronNile
                | Self::EthSepolia
                | Self::BscTestnet
                | Self::PolAmoy
                | Self::BtcTestnet
                | Self::LtcTestnet
        )
    }

    pub const fn display_name(self) -> &'static str {
        match self {
            Self::TronMainnet => "Tron",
            Self::TronNile => "Tron Nile",

            Self::EthMainnet => "Ethereum",
            Self::EthSepolia => "Ethereum Sepolia",

            Self::BscMainnet => "BNB Smart Chain",
            Self::BscTestnet => "BNB Smart Chain Testnet",

            Self::PolMainnet => "Polygon",
            Self::PolAmoy => "Polygon Amoy",

            Self::BtcMainnet => "Bitcoin",
            Self::BtcTestnet => "Bitcoin Testnet",

            Self::LtcMainnet => "Litecoin",
            Self::LtcTestnet => "Litecoin Testnet",
        }
    }

    pub const fn chain(self) -> Chain {
        match self {
            Self::TronMainnet | Self::TronNile => Chain::Tron,
            Self::EthMainnet | Self::EthSepolia => Chain::Ethereum,
            Self::BscMainnet | Self::BscTestnet => Chain::BinanceSmartChain,
            Self::PolMainnet | Self::PolAmoy => Chain::Polygon,
            Self::BtcMainnet | Self::BtcTestnet => Chain::Bitcoin,
            Self::LtcMainnet | Self::LtcTestnet => Chain::Litecoin,
        }
    }
}
