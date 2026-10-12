use jamsrpay_types::currency::{AssetId, Decimals, NetworkId};

use crate::error::AssetNetworkRegistryError;
use crate::registry::AssetNetworkRegistry;
use crate::standard::TokenStandard as Standard;

#[test]
fn resolves_litecoin_mainnet() {
    let definition = AssetNetworkRegistry::resolve(AssetId::Ltc, NetworkId::Litecoin).unwrap();

    assert_eq!(definition.decimals(), Decimals::EIGHT);
    assert_eq!(definition.decimals(), 8);
    assert!(definition.is_native());
    assert_eq!(definition.token_address(), None);
}

#[test]
fn resolves_litecoin_testnet() {
    let definition =
        AssetNetworkRegistry::resolve(AssetId::Ltc, NetworkId::LitecoinTestnet).unwrap();

    assert_eq!(definition.decimals(), Decimals::EIGHT);
    assert_eq!(definition.decimals(), 8);
    assert!(definition.is_native());
}

#[test]
fn resolves_native_asset_decimals() {
    assert_eq!(
        AssetNetworkRegistry::resolve(AssetId::Eth, NetworkId::Ethereum)
            .unwrap()
            .decimals(),
        Decimals::EIGHTEEN
    );

    assert_eq!(
        AssetNetworkRegistry::resolve(AssetId::Trx, NetworkId::Tron)
            .unwrap()
            .decimals(),
        Decimals::SIX
    );

    assert_eq!(
        AssetNetworkRegistry::resolve(AssetId::Sol, NetworkId::Solana)
            .unwrap()
            .decimals(),
        Decimals::NINE
    );
}

#[test]
fn resolves_different_usdt_deployments() {
    let tron = AssetNetworkRegistry::resolve(AssetId::Usdt, NetworkId::Tron).unwrap();
    let ethereum = AssetNetworkRegistry::resolve(AssetId::Usdt, NetworkId::Ethereum).unwrap();
    let bsc = AssetNetworkRegistry::resolve(AssetId::Usdt, NetworkId::Bsc).unwrap();

    assert_eq!(tron.decimals(), Decimals::SIX);
    assert_eq!(ethereum.decimals(), Decimals::SIX);
    assert_eq!(bsc.decimals(), Decimals::EIGHTEEN);

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

    assert_eq!(definition.decimals(), Decimals::SIX);
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
        Decimals::EIGHTEEN
    );
    assert_eq!(
        AssetNetworkRegistry::decimals(AssetId::Usdt, NetworkId::Tron).unwrap(),
        Decimals::SIX
    );
    assert_eq!(
        AssetNetworkRegistry::decimals(AssetId::Btc, NetworkId::Bitcoin).unwrap(),
        Decimals::EIGHT
    );
    assert_eq!(
        AssetNetworkRegistry::decimals(AssetId::Ltc, NetworkId::Litecoin).unwrap(),
        Decimals::EIGHT
    );
    assert_eq!(
        AssetNetworkRegistry::decimals(AssetId::Sol, NetworkId::Solana).unwrap(),
        Decimals::NINE
    );
}
