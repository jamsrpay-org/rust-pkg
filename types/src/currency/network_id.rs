use crate::currency::Chain;
use strum::{AsRefStr, Display, EnumString};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Display,
    EnumString,
    AsRefStr,
    serde::Serialize,
    serde::Deserialize,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum NetworkId {
    Tron,
    TronNile,

    Bsc,
    BscTestnet,

    Ethereum,
    EthereumSepolia,

    Polygon,
    PolygonAmoy,

    Bitcoin,
    BitcoinTestnet,

    Litecoin,
    LitecoinTestnet,

    Solana,
    SolanaDevnet,
}

impl NetworkId {
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Tron => "Tron",
            Self::TronNile => "Tron Nile",
            Self::Bsc => "BNB Smart Chain",
            Self::BscTestnet => "BNB Smart Chain Testnet",
            Self::Ethereum => "Ethereum",
            Self::EthereumSepolia => "Ethereum Sepolia",
            Self::Polygon => "Polygon",
            Self::PolygonAmoy => "Polygon Amoy",
            Self::Bitcoin => "Bitcoin",
            Self::BitcoinTestnet => "Bitcoin Testnet",
            Self::Litecoin => "Litecoin",
            Self::LitecoinTestnet => "Litecoin Testnet",
            Self::Solana => "Solana",
            Self::SolanaDevnet => "Solana Devnet",
        }
    }

    pub const fn is_testnet(self) -> bool {
        matches!(
            self,
            Self::TronNile
                | Self::EthereumSepolia
                | Self::BscTestnet
                | Self::PolygonAmoy
                | Self::BitcoinTestnet
                | Self::LitecoinTestnet
                | Self::SolanaDevnet
        )
    }

    pub const fn chain(self) -> Chain {
        match self {
            Self::Tron | Self::TronNile => Chain::Tron,
            Self::Ethereum | Self::EthereumSepolia => Chain::Ethereum,
            Self::Bsc | Self::BscTestnet => Chain::BinanceSmartChain,
            Self::Polygon | Self::PolygonAmoy => Chain::Polygon,
            Self::Bitcoin | Self::BitcoinTestnet => Chain::Bitcoin,
            Self::Litecoin | Self::LitecoinTestnet => Chain::Litecoin,
            Self::Solana | Self::SolanaDevnet => Chain::Solana,
        }
    }

    pub fn get_base_url(&self) -> &'static str {
        match *self {
            NetworkId::Tron => "https://tronscan.org",
            NetworkId::TronNile => "https://nile.tronscan.org",

            NetworkId::Bsc => "https://bscscan.com",
            NetworkId::BscTestnet => "https://testnet.bscscan.com",

            NetworkId::Bitcoin => "https://mempool.space",
            NetworkId::BitcoinTestnet => "https://mempool.space/testnet4",

            NetworkId::Ethereum => "https://etherscan.io",
            NetworkId::EthereumSepolia => "https://sepolia.etherscan.io",

            NetworkId::Litecoin => "https://litecoinspace.org/",
            NetworkId::LitecoinTestnet => "https://litecoinspace.org/testnet",

            NetworkId::Polygon => "https://polygonscan.com",
            NetworkId::PolygonAmoy => "https://amoy.polygonscan.com",

            NetworkId::Solana => "https://solscan.io",
            NetworkId::SolanaDevnet => "https://solscan.io",
        }
    }

    pub fn address_view_url(&self, address: &str) -> String {
        let base_url = self.get_base_url();
        match *self {
            NetworkId::Tron => format!("{}/address/{}", base_url, address),
            NetworkId::TronNile => format!("{}/address/{}", base_url, address),
            NetworkId::Bsc => format!("{}/address/{}", base_url, address),
            NetworkId::BscTestnet => format!("{}/address/{}", base_url, address),
            NetworkId::Ethereum => format!("{}/address/{}", base_url, address),
            NetworkId::EthereumSepolia => format!("{}/address/{}", base_url, address),
            NetworkId::Polygon => format!("{}/address/{}", base_url, address),
            NetworkId::PolygonAmoy => format!("{}/address/{}", base_url, address),
            NetworkId::Bitcoin => format!("{}/address/{}", base_url, address),
            NetworkId::BitcoinTestnet => format!("{}/address/{}", base_url, address),
            NetworkId::Litecoin => format!("{}/address/{}", base_url, address),
            NetworkId::LitecoinTestnet => format!("{}/address/{}", base_url, address),
            NetworkId::Solana => format!("{}/account/{}", base_url, address),
            NetworkId::SolanaDevnet => format!("{}/account/{}?cluster=devnet", base_url, address),
        }
    }

    pub fn transaction_view_url(&self, tx_id: &str) -> String {
        let base_url = self.get_base_url();
        match *self {
            NetworkId::Tron => format!("{}/transaction/{}", base_url, tx_id),
            NetworkId::TronNile => format!("{}/transaction/{}", base_url, tx_id),
            NetworkId::Bsc => format!("{}/tx/{}", base_url, tx_id),
            NetworkId::BscTestnet => format!("{}/tx/{}", base_url, tx_id),
            NetworkId::Ethereum => format!("{}/tx/{}", base_url, tx_id),
            NetworkId::EthereumSepolia => format!("{}/tx/{}", base_url, tx_id),
            NetworkId::Polygon => format!("{}/tx/{}", base_url, tx_id),
            NetworkId::PolygonAmoy => format!("{}/tx/{}", base_url, tx_id),
            NetworkId::Bitcoin => format!("{}/tx/{}", base_url, tx_id),
            NetworkId::BitcoinTestnet => format!("{}/tx/{}", base_url, tx_id),
            NetworkId::Litecoin => format!("{}/tx/{}", base_url, tx_id),
            NetworkId::LitecoinTestnet => format!("{}/tx/{}", base_url, tx_id),
            NetworkId::Solana => format!("{}/tx/{}", base_url, tx_id),
            NetworkId::SolanaDevnet => format!("{}/tx/{}?cluster=devnet", base_url, tx_id),
        }
    }
}
