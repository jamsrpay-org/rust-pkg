use jamsrpay_types::currency::{AssetId, NetworkId};
use thiserror::Error;

// ============================================================
// Asset-network identity
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetNetworkKey {
    pub asset_id: AssetId,
    pub network_id: NetworkId,
}

impl AssetNetworkKey {
    pub const fn new(asset_id: AssetId, network_id: NetworkId) -> Self {
        Self {
            asset_id,
            network_id,
        }
    }
}

// ============================================================
// Token standards
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenStandard {
    Trc20,
    Bep20,
    Erc20,
    Spl,
}

// ============================================================
// Asset-network representation
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetNetworkRepresentation {
    /// Native blockchain asset, such as LTC, ETH, TRX or SOL.
    Native,

    /// Token deployed on a blockchain.
    ///
    /// `token_address` means:
    /// - EVM/TRON: token contract address.
    /// - Solana: token mint address.
    Token {
        standard: TokenStandard,
        token_address: &'static str,
    },
}

// ============================================================
// Asset-network definition
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AssetNetworkDefinition {
    pub key: AssetNetworkKey,
    pub decimals: u8,
    pub representation: AssetNetworkRepresentation,
}

impl AssetNetworkDefinition {
    pub const fn native(asset_id: AssetId, network_id: NetworkId, decimals: u8) -> Self {
        Self {
            key: AssetNetworkKey::new(asset_id, network_id),
            decimals,
            representation: AssetNetworkRepresentation::Native,
        }
    }

    pub const fn token(
        asset_id: AssetId,
        network_id: NetworkId,
        decimals: u8,
        standard: TokenStandard,
        token_address: &'static str,
    ) -> Self {
        Self {
            key: AssetNetworkKey::new(asset_id, network_id),
            decimals,
            representation: AssetNetworkRepresentation::Token {
                standard,
                token_address,
            },
        }
    }

    pub const fn asset_id(&self) -> AssetId {
        self.key.asset_id
    }

    pub const fn network_id(&self) -> NetworkId {
        self.key.network_id
    }

    pub const fn decimals(&self) -> u8 {
        self.decimals
    }

    pub const fn representation(&self) -> AssetNetworkRepresentation {
        self.representation
    }

    pub const fn token_standard(&self) -> Option<TokenStandard> {
        match self.representation {
            AssetNetworkRepresentation::Native => None,
            AssetNetworkRepresentation::Token { standard, .. } => Some(standard),
        }
    }

    pub const fn token_address(&self) -> Option<&'static str> {
        match self.representation {
            AssetNetworkRepresentation::Native => None,
            AssetNetworkRepresentation::Token { token_address, .. } => Some(token_address),
        }
    }

    pub const fn is_native(&self) -> bool {
        matches!(self.representation, AssetNetworkRepresentation::Native)
    }
}

// ============================================================
// Registry errors
// ============================================================

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum AssetNetworkRegistryError {
    #[error("unsupported asset/network combination: {asset_id:?} on {network_id:?}")]
    UnsupportedAssetNetwork {
        asset_id: AssetId,
        network_id: NetworkId,
    },
}

// ============================================================
// Registry
// ============================================================

/// Immutable registry of engineering-defined asset/network combinations.
///
/// Contains technical protocol metadata only:
/// - Atomic-unit precision
/// - Native/token classification
/// - Token standard
/// - Token contract or mint address
///
/// Does not contain runtime availability or operational policy.
/// Those remain the responsibility of asset-service.
pub struct AssetNetworkRegistry;

impl AssetNetworkRegistry {
    /// Resolve one supported asset/network combination.
    pub fn get(
        key: AssetNetworkKey,
    ) -> Result<&'static AssetNetworkDefinition, AssetNetworkRegistryError> {
        DEFINITIONS
            .iter()
            .find(|definition| definition.key == key)
            .ok_or(AssetNetworkRegistryError::UnsupportedAssetNetwork {
                asset_id: key.asset_id,
                network_id: key.network_id,
            })
    }

    /// Resolve using the asset and network identifiers directly.
    pub fn resolve(
        asset_id: AssetId,
        network_id: NetworkId,
    ) -> Result<&'static AssetNetworkDefinition, AssetNetworkRegistryError> {
        Self::get(AssetNetworkKey::new(asset_id, network_id))
    }

    /// Return all registered definitions.
    pub fn all() -> &'static [AssetNetworkDefinition] {
        DEFINITIONS
    }

    /// Iterate over all definitions registered for a network.
    pub fn for_network(
        network_id: NetworkId,
    ) -> impl Iterator<Item = &'static AssetNetworkDefinition> {
        DEFINITIONS
            .iter()
            .filter(move |definition| definition.network_id() == network_id)
    }

    /// Iterate over all registered network representations of an asset.
    pub fn for_asset(asset_id: AssetId) -> impl Iterator<Item = &'static AssetNetworkDefinition> {
        DEFINITIONS
            .iter()
            .filter(move |definition| definition.asset_id() == asset_id)
    }

    /// Check whether a combination is present in the technical registry.
    pub fn supports(asset_id: AssetId, network_id: NetworkId) -> bool {
        Self::resolve(asset_id, network_id).is_ok()
    }

    /// Resolve precision (decimals) for an asset/network combination.
    pub fn decimals(
        asset_id: AssetId,
        network_id: NetworkId,
    ) -> Result<u8, AssetNetworkRegistryError> {
        Self::resolve(asset_id, network_id).map(|definition| definition.decimals())
    }
}

// ============================================================
// Registry definitions
// ============================================================

use AssetNetworkDefinition as Definition;
use TokenStandard as Standard;

static DEFINITIONS: &[AssetNetworkDefinition] = &[
    // --------------------------------------------------------
    // Native assets: mainnet
    // --------------------------------------------------------
    Definition::native(AssetId::Trx, NetworkId::Tron, 6),
    Definition::native(AssetId::Bnb, NetworkId::Bsc, 18),
    Definition::native(AssetId::Eth, NetworkId::Ethereum, 18),
    Definition::native(AssetId::Pol, NetworkId::Polygon, 18),
    Definition::native(AssetId::Btc, NetworkId::Bitcoin, 8),
    Definition::native(AssetId::Ltc, NetworkId::Litecoin, 8),
    Definition::native(AssetId::Sol, NetworkId::Solana, 9),
    // --------------------------------------------------------
    // Native assets: testnet
    // --------------------------------------------------------
    Definition::native(AssetId::Trx, NetworkId::TronNile, 6),
    Definition::native(AssetId::Bnb, NetworkId::BscTestnet, 18),
    Definition::native(AssetId::Eth, NetworkId::EthereumSepolia, 18),
    Definition::native(AssetId::Pol, NetworkId::PolygonAmoy, 18),
    Definition::native(AssetId::Btc, NetworkId::BitcoinTestnet, 8),
    Definition::native(AssetId::Ltc, NetworkId::LitecoinTestnet, 8),
    Definition::native(AssetId::Sol, NetworkId::SolanaDevnet, 9),
    // --------------------------------------------------------
    // USDT: mainnet
    // --------------------------------------------------------
    Definition::token(
        AssetId::Usdt,
        NetworkId::Tron,
        6,
        Standard::Trc20,
        "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t",
    ),
    Definition::token(
        AssetId::Usdt,
        NetworkId::Ethereum,
        6,
        Standard::Erc20,
        "0xdAC17F958D2ee523a2206206994597C13D831ec7",
    ),
    Definition::token(
        AssetId::Usdt,
        NetworkId::Bsc,
        18,
        Standard::Bep20,
        "0x55d398326f99059fF775485246999027B3197955",
    ),
    Definition::token(
        AssetId::Usdt,
        NetworkId::Solana,
        6,
        Standard::Spl,
        "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB",
    ),
    Definition::token(
        AssetId::Usdt,
        NetworkId::Polygon,
        6,
        Standard::Erc20,
        "0xc2132D05D31c914a87C6611C10748AEb04B58e8F",
    ),
    // --------------------------------------------------------
    // USDT: testnet
    // --------------------------------------------------------
    Definition::token(
        AssetId::Usdt,
        NetworkId::EthereumSepolia,
        6,
        Standard::Erc20,
        "0xaA8E23Fb1079EA71e0a56F48a2aA51851D8433D0",
    ),
    Definition::token(
        AssetId::Usdt,
        NetworkId::BscTestnet,
        18,
        Standard::Bep20,
        "0x337610d27c682E347C9cD60BD4b3b107C9d34dDd",
    ),
    Definition::token(
        AssetId::Usdt,
        NetworkId::PolygonAmoy,
        6,
        Standard::Erc20,
        "0x1616d425Cd540B256475cBfb604586C8598eC0FB",
    ),
    Definition::token(
        AssetId::Usdt,
        NetworkId::SolanaDevnet,
        6,
        Standard::Spl,
        "CXk2AMBfi3TwaEL2468s6zP8xq9NxTXjp9gjMgzeUynM",
    ),
    // --------------------------------------------------------
    // USDC: mainnet
    // --------------------------------------------------------
    Definition::token(
        AssetId::Usdc,
        NetworkId::Ethereum,
        6,
        Standard::Erc20,
        "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    ),
    Definition::token(
        AssetId::Usdc,
        NetworkId::Polygon,
        6,
        Standard::Erc20,
        "0x3c499c542cef5e3811e1192ce70d8cc03d5c3359",
    ),
    Definition::token(
        AssetId::Usdc,
        NetworkId::Bsc,
        18,
        Standard::Bep20,
        "0x8AC76a51cc950d9822D68b83fE1Ad97B32Cd580d",
    ),
    Definition::token(
        AssetId::Usdc,
        NetworkId::Solana,
        6,
        Standard::Spl,
        "EPjFWdd5AufqSSqeM2q1NzybapC8G4wEGGkZwyTDt1v",
    ),
    // --------------------------------------------------------
    // USDC: testnet
    // --------------------------------------------------------
    Definition::token(
        AssetId::Usdc,
        NetworkId::EthereumSepolia,
        6,
        Standard::Erc20,
        "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238",
    ),
    Definition::token(
        AssetId::Usdc,
        NetworkId::BscTestnet,
        18,
        Standard::Bep20,
        "0x64544969ed7EBf5f083679233325356EbE738930",
    ),
    Definition::token(
        AssetId::Usdc,
        NetworkId::PolygonAmoy,
        6,
        Standard::Erc20,
        "0x41E94Eb71898E8A90Fe2dC87e92a03C0cf1Fa5AB",
    ),
    Definition::token(
        AssetId::Usdc,
        NetworkId::SolanaDevnet,
        6,
        Standard::Spl,
        "CXk2AMBfi3TwaEL2468s6zP8xq9NxTXjp9gjMgzeUynM",
    ),
    // --------------------------------------------------------
    // DAI: mainnet
    // --------------------------------------------------------
    Definition::token(
        AssetId::Dai,
        NetworkId::Ethereum,
        18,
        Standard::Erc20,
        "0x6B175474E89094C44Da98b954EedeAC495271d0F",
    ),
    Definition::token(
        AssetId::Dai,
        NetworkId::Bsc,
        18,
        Standard::Bep20,
        "0x1AF3F329e8BE154074D8769D1FFa4eE058B1DBc3",
    ),
    Definition::token(
        AssetId::Dai,
        NetworkId::Polygon,
        18,
        Standard::Erc20,
        "0x8f3Cf7ad23Cd3CaDbD9735AFf958023239c6A063",
    ),
    // --------------------------------------------------------
    // EURC: mainnet
    // --------------------------------------------------------
    Definition::token(
        AssetId::Eurc,
        NetworkId::Ethereum,
        6,
        Standard::Erc20,
        "0x1aBaEA1f7C830bD89Acc67eC4af516284b1bC33c",
    ),
];

// ============================================================
// Unit tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_litecoin_mainnet() {
        let definition = AssetNetworkRegistry::resolve(AssetId::Ltc, NetworkId::Litecoin).unwrap();

        assert_eq!(definition.decimals(), 8);
        assert!(definition.is_native());
        assert_eq!(definition.token_address(), None);
    }

    #[test]
    fn resolves_litecoin_testnet() {
        let definition =
            AssetNetworkRegistry::resolve(AssetId::Ltc, NetworkId::LitecoinTestnet).unwrap();

        assert_eq!(definition.decimals(), 8);
        assert!(definition.is_native());
    }

    #[test]
    fn resolves_native_asset_decimals() {
        assert_eq!(
            AssetNetworkRegistry::resolve(AssetId::Eth, NetworkId::Ethereum)
                .unwrap()
                .decimals(),
            18
        );

        assert_eq!(
            AssetNetworkRegistry::resolve(AssetId::Trx, NetworkId::Tron)
                .unwrap()
                .decimals(),
            6
        );

        assert_eq!(
            AssetNetworkRegistry::resolve(AssetId::Sol, NetworkId::Solana)
                .unwrap()
                .decimals(),
            9
        );
    }

    #[test]
    fn resolves_different_usdt_deployments() {
        let tron = AssetNetworkRegistry::resolve(AssetId::Usdt, NetworkId::Tron).unwrap();

        let ethereum = AssetNetworkRegistry::resolve(AssetId::Usdt, NetworkId::Ethereum).unwrap();

        let bsc = AssetNetworkRegistry::resolve(AssetId::Usdt, NetworkId::Bsc).unwrap();

        assert_eq!(tron.decimals(), 6);
        assert_eq!(ethereum.decimals(), 6);
        assert_eq!(bsc.decimals(), 18);

        assert_eq!(tron.token_standard(), Some(Standard::Trc20));
        assert_eq!(ethereum.token_standard(), Some(Standard::Erc20));
        assert_eq!(bsc.token_standard(), Some(Standard::Bep20));
    }

    #[test]
    fn resolves_usdc_on_sepolia() {
        let definition =
            AssetNetworkRegistry::resolve(AssetId::Usdc, NetworkId::EthereumSepolia).unwrap();

        assert_eq!(definition.decimals(), 6);
        assert_eq!(definition.token_standard(), Some(Standard::Erc20));
    }

    #[test]
    fn rejects_unsupported_combinations() {
        let result = AssetNetworkRegistry::resolve(AssetId::Usdt, NetworkId::TronNile);

        assert!(matches!(
            result,
            Err(AssetNetworkRegistryError::UnsupportedAssetNetwork {
                asset_id: AssetId::Usdt,
                network_id: NetworkId::TronNile,
            })
        ));
    }

    #[test]
    fn iterates_definitions_by_network() {
        let definitions: Vec<_> = AssetNetworkRegistry::for_network(NetworkId::Ethereum).collect();

        assert!(
            definitions
                .iter()
                .any(|definition| { definition.asset_id() == AssetId::Eth })
        );

        assert!(
            definitions
                .iter()
                .any(|definition| { definition.asset_id() == AssetId::Usdt })
        );
    }

    #[test]
    fn iterates_definitions_by_asset() {
        let definitions: Vec<_> = AssetNetworkRegistry::for_asset(AssetId::Usdt).collect();

        assert!(definitions.len() >= 3);
    }

    #[test]
    fn resolves_decimals_helper() {
        assert_eq!(
            AssetNetworkRegistry::decimals(AssetId::Usdt, NetworkId::Bsc).unwrap(),
            18
        );
        assert_eq!(
            AssetNetworkRegistry::decimals(AssetId::Usdt, NetworkId::Tron).unwrap(),
            6
        );
        assert_eq!(
            AssetNetworkRegistry::decimals(AssetId::Btc, NetworkId::Bitcoin).unwrap(),
            8
        );
        assert_eq!(
            AssetNetworkRegistry::decimals(AssetId::Ltc, NetworkId::Litecoin).unwrap(),
            8
        );
        assert_eq!(
            AssetNetworkRegistry::decimals(AssetId::Sol, NetworkId::Solana).unwrap(),
            9
        );
    }
}
