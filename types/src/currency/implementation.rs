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
            Chain::Solana => BlockchainMeta {
                symbol: "SOL",
                name: "Solana",
                gas_currency: Some(GasCurrency::SOL),
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

impl Chain {
    /// Returns true for EVM-compatible chains (Ethereum, BSC, Polygon).
    pub const fn is_evm(&self) -> bool {
        matches!(
            self,
            Chain::Ethereum | Chain::BinanceSmartChain | Chain::Polygon
        )
    }
}

impl From<GasCurrency> for PaymentCurrency {
    fn from(value: GasCurrency) -> Self {
        match value {
            GasCurrency::TRX => PaymentCurrency::TRX,
            GasCurrency::BNB => PaymentCurrency::BNB,
            GasCurrency::ETH => PaymentCurrency::ETH,
            GasCurrency::POL => PaymentCurrency::POL,
            GasCurrency::SOL => PaymentCurrency::SOL,
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
            PaymentCurrency::SOL => Some(PaymentCurrency::SOL),
            PaymentCurrency::USDT => None,
            PaymentCurrency::USDC => None,
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
            PaymentCurrency::SOL => PaymentCurrencyMeta {
                asset_id: "SOL",
                symbol: "SOL",
                name: "Solana",
                chain: Chain::Solana,
                kind: AssetKind::Native,
                decimals: 9,
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
            PaymentCurrency::TRX => &[
                N {
                    network_id: NetworkId::TronMainnet,
                    chain: Chain::Tron,
                    standard: None,
                    is_native: true,
                },
                N {
                    network_id: NetworkId::TronNile,
                    chain: Chain::Tron,
                    standard: None,
                    is_native: true,
                },
            ],
            PaymentCurrency::BNB => &[
                N {
                    network_id: NetworkId::BscMainnet,
                    chain: Chain::BinanceSmartChain,
                    standard: None,
                    is_native: true,
                },
                N {
                    network_id: NetworkId::BscTestnet,
                    chain: Chain::BinanceSmartChain,
                    standard: None,
                    is_native: true,
                },
            ],
            PaymentCurrency::ETH => &[
                N {
                    network_id: NetworkId::EthMainnet,
                    chain: Chain::Ethereum,
                    standard: None,
                    is_native: true,
                },
                N {
                    network_id: NetworkId::EthSepolia,
                    chain: Chain::Ethereum,
                    standard: None,
                    is_native: true,
                },
            ],
            PaymentCurrency::POL => &[
                N {
                    network_id: NetworkId::PolMainnet,
                    chain: Chain::Polygon,
                    standard: None,
                    is_native: true,
                },
                N {
                    network_id: NetworkId::PolAmoy,
                    chain: Chain::Polygon,
                    standard: None,
                    is_native: true,
                },
            ],
            PaymentCurrency::BTC => &[
                N {
                    network_id: NetworkId::BtcMainnet,
                    chain: Chain::Bitcoin,
                    standard: None,
                    is_native: true,
                },
                N {
                    network_id: NetworkId::BtcTestnet,
                    chain: Chain::Bitcoin,
                    standard: None,
                    is_native: true,
                },
            ],
            PaymentCurrency::LTC => &[
                N {
                    network_id: NetworkId::LtcMainnet,
                    chain: Chain::Litecoin,
                    standard: None,
                    is_native: true,
                },
                N {
                    network_id: NetworkId::LtcTestnet,
                    chain: Chain::Litecoin,
                    standard: None,
                    is_native: true,
                },
            ],
            PaymentCurrency::USDT => &[
                N {
                    network_id: NetworkId::TronMainnet,
                    chain: Chain::Tron,
                    standard: Some(TokenStandard::Trc20),
                    is_native: false,
                },
                N {
                    network_id: NetworkId::TronNile,
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
                    network_id: NetworkId::BscTestnet,
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
                    network_id: NetworkId::EthSepolia,
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
                N {
                    network_id: NetworkId::PolAmoy,
                    chain: Chain::Polygon,
                    standard: Some(TokenStandard::Erc20),
                    is_native: false,
                },
                N {
                    network_id: NetworkId::SolMainnet,
                    chain: Chain::Solana,
                    standard: Some(TokenStandard::Spl),
                    is_native: false,
                },
                N {
                    network_id: NetworkId::SolDevnet,
                    chain: Chain::Solana,
                    standard: Some(TokenStandard::Spl),
                    is_native: false,
                },
            ],
            PaymentCurrency::USDC => &[
                N {
                    network_id: NetworkId::BscMainnet,
                    chain: Chain::BinanceSmartChain,
                    standard: Some(TokenStandard::Bep20),
                    is_native: false,
                },
                N {
                    network_id: NetworkId::BscTestnet,
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
                    network_id: NetworkId::EthSepolia,
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
                N {
                    network_id: NetworkId::PolAmoy,
                    chain: Chain::Polygon,
                    standard: Some(TokenStandard::Erc20),
                    is_native: false,
                },
                N {
                    network_id: NetworkId::SolMainnet,
                    chain: Chain::Solana,
                    standard: Some(TokenStandard::Spl),
                    is_native: false,
                },
                N {
                    network_id: NetworkId::SolDevnet,
                    chain: Chain::Solana,
                    standard: Some(TokenStandard::Spl),
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
                    network_id: NetworkId::BscTestnet,
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
                    network_id: NetworkId::EthSepolia,
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
                N {
                    network_id: NetworkId::PolAmoy,
                    chain: Chain::Polygon,
                    standard: Some(TokenStandard::Erc20),
                    is_native: false,
                },
            ],
            PaymentCurrency::EURC => &[
                N {
                    network_id: NetworkId::EthMainnet,
                    chain: Chain::Ethereum,
                    standard: Some(TokenStandard::Erc20),
                    is_native: false,
                },
                N {
                    network_id: NetworkId::EthSepolia,
                    chain: Chain::Ethereum,
                    standard: Some(TokenStandard::Erc20),
                    is_native: false,
                },
            ],
            PaymentCurrency::SOL => &[
                N {
                    network_id: NetworkId::SolMainnet,
                    chain: Chain::Solana,
                    standard: None,
                    is_native: true,
                },
                N {
                    network_id: NetworkId::SolDevnet,
                    chain: Chain::Solana,
                    standard: None,
                    is_native: true,
                },
            ],
        }
    }
}

impl PaymentCurrency {
    pub fn min_payment_amount(&self) -> Money {
        match self {
            PaymentCurrency::TRX => Money::from_atomic(3_000_000, self.decimals()),
            PaymentCurrency::USDT => Money::from_atomic(5_000_000, self.decimals()),
            PaymentCurrency::BNB => Money::from_atomic(1, 1),
            PaymentCurrency::USDC | PaymentCurrency::DAI | PaymentCurrency::EURC => {
                Money::from_atomic(1, 0)
            }
            PaymentCurrency::BTC => Money::from_atomic(1, 0),
            PaymentCurrency::LTC => Money::from_atomic(1, 0),
            PaymentCurrency::ETH => Money::from_atomic(1, 0),
            PaymentCurrency::POL => Money::from_atomic(1, 0),
            PaymentCurrency::SOL => Money::from_atomic(1, 0),
        }
    }

    pub fn fixed_network_fee(&self) -> Money {
        match self {
            PaymentCurrency::TRX => Money::from_atomic(2_000_000, self.decimals()),
            PaymentCurrency::USDT => Money::from_atomic(0, self.decimals()),
            PaymentCurrency::BNB
            | PaymentCurrency::USDC
            | PaymentCurrency::DAI
            | PaymentCurrency::EURC => Money::from_atomic(0, self.decimals()),
            PaymentCurrency::BTC => Money::from_atomic(0, self.decimals()),
            PaymentCurrency::LTC => Money::from_atomic(0, self.decimals()),
            PaymentCurrency::ETH => Money::from_atomic(0, self.decimals()),
            PaymentCurrency::POL => Money::from_atomic(0, self.decimals()),
            PaymentCurrency::SOL => Money::from_atomic(0, self.decimals()),
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
            PricingCurrency::SOL => PricingCurrencyMeta {
                asset_id: "SOL",
                symbol: "SOL",
                name: "Solana",
                decimals: 9,
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
                | Self::SolDevnet
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

            Self::SolMainnet => "Solana",
            Self::SolDevnet => "Solana Devnet",
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
            Self::SolMainnet | Self::SolDevnet => Chain::Solana,
        }
    }

    pub fn get_base_url(&self) -> &'static str {
        match *self {
            NetworkId::TronMainnet => "https://tronscan.org",
            NetworkId::TronNile => "https://nile.tronscan.org",

            NetworkId::BscMainnet => "https://bscscan.com",
            NetworkId::BscTestnet => "https://testnet.bscscan.com",

            NetworkId::BtcMainnet => "https://mempool.space",
            NetworkId::BtcTestnet => "https://mempool.space/testnet4",

            NetworkId::EthMainnet => "https://etherscan.io",
            NetworkId::EthSepolia => "https://sepolia.etherscan.io",

            NetworkId::LtcMainnet => "https://litecoinspace.org/",
            NetworkId::LtcTestnet => "https://litecoinspace.org/testnet",

            NetworkId::PolMainnet => "https://polygonscan.com",
            NetworkId::PolAmoy => "https://amoy.polygonscan.com",

            NetworkId::SolMainnet => "https://solscan.io",
            NetworkId::SolDevnet => "https://solscan.io",
        }
    }

    pub fn address_view_url(&self, address: &str) -> String {
        let base_url = self.get_base_url();
        match *self {
            NetworkId::TronMainnet => format!("{}/address/{}", base_url, address),
            NetworkId::TronNile => format!("{}/address/{}", base_url, address),
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
            NetworkId::SolMainnet => format!("{}/account/{}", base_url, address),
            NetworkId::SolDevnet => format!("{}/account/{}?cluster=devnet", base_url, address),
        }
    }

    pub fn transaction_view_url(&self, tx_id: &str) -> String {
        let base_url = self.get_base_url();
        match *self {
            NetworkId::TronMainnet => format!("{}/transaction/{}", base_url, tx_id),
            NetworkId::TronNile => format!("{}/transaction/{}", base_url, tx_id),
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
            NetworkId::SolMainnet => format!("{}/tx/{}", base_url, tx_id),
            NetworkId::SolDevnet => format!("{}/tx/{}?cluster=devnet", base_url, tx_id),
        }
    }

    /// Default number of block confirmations required for a transaction to be
    /// considered final on this network.
    ///
    /// These are sensible production defaults. Services may override via
    /// configuration where appropriate.
    pub const fn required_confirmations(self) -> u32 {
        match self {
            // TRON: ~3s blocks, 20 confirmations ≈ 1 minute
            Self::TronMainnet | Self::TronNile => 20,

            // Ethereum: ~12s blocks, 12 confirmations ≈ 2.5 minutes
            Self::EthMainnet | Self::EthSepolia => 12,

            // BSC: ~3s blocks, 15 confirmations ≈ 45 seconds
            Self::BscMainnet | Self::BscTestnet => 15,

            // Polygon: ~2s blocks, 30 confirmations ≈ 1 minute
            Self::PolMainnet | Self::PolAmoy => 30,

            // Bitcoin: ~10min blocks, 3 confirmations ≈ 30 minutes
            Self::BtcMainnet | Self::BtcTestnet => 3,

            // Litecoin: ~2.5min blocks, 6 confirmations ≈ 15 minutes
            Self::LtcMainnet | Self::LtcTestnet => 6,

            // Solana: uses slot-based finality, 1 = finalized slot
            Self::SolMainnet | Self::SolDevnet => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_token_standard_spl() {
        assert_eq!(TokenStandard::Spl.to_string(), "SPL");
        assert_eq!(TokenStandard::Spl.as_ref(), "SPL");
        assert_eq!(TokenStandard::from_str("SPL"), Ok(TokenStandard::Spl));
    }

    #[test]
    fn test_usdt_solana_networks() {
        let networks = PaymentCurrency::USDT.networks();
        let sol_mainnet = networks
            .iter()
            .find(|n| n.network_id == NetworkId::SolMainnet)
            .expect("USDT should support SolMainnet");
        assert_eq!(sol_mainnet.chain, Chain::Solana);
        assert_eq!(sol_mainnet.standard, Some(TokenStandard::Spl));
        assert!(!sol_mainnet.is_native);

        let sol_devnet = networks
            .iter()
            .find(|n| n.network_id == NetworkId::SolDevnet)
            .expect("USDT should support SolDevnet");
        assert_eq!(sol_devnet.chain, Chain::Solana);
        assert_eq!(sol_devnet.standard, Some(TokenStandard::Spl));
        assert!(!sol_devnet.is_native);
    }

    #[test]
    fn test_usdc_solana_networks() {
        let networks = PaymentCurrency::USDC.networks();
        let sol_mainnet = networks
            .iter()
            .find(|n| n.network_id == NetworkId::SolMainnet)
            .expect("USDC should support SolMainnet");
        assert_eq!(sol_mainnet.chain, Chain::Solana);
        assert_eq!(sol_mainnet.standard, Some(TokenStandard::Spl));
        assert!(!sol_mainnet.is_native);

        let sol_devnet = networks
            .iter()
            .find(|n| n.network_id == NetworkId::SolDevnet)
            .expect("USDC should support SolDevnet");
        assert_eq!(sol_devnet.chain, Chain::Solana);
        assert_eq!(sol_devnet.standard, Some(TokenStandard::Spl));
        assert!(!sol_devnet.is_native);
    }
}
