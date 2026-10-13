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
    let result = AssetNetworkRegistry::resolve(AssetId::Btc, NetworkId::TronNile);

    assert!(matches!(
        result,
        Err(AssetNetworkRegistryError::UnsupportedAssetNetwork {
            asset_id: AssetId::Btc,
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

#[test]
fn resolves_token_and_native_helpers() {
    // Native assets
    assert!(AssetNetworkRegistry::is_native(
        AssetId::Eth,
        NetworkId::Ethereum
    ));
    assert!(!AssetNetworkRegistry::is_token(
        AssetId::Eth,
        NetworkId::Ethereum
    ));

    assert!(AssetNetworkRegistry::is_native(
        AssetId::Trx,
        NetworkId::Tron
    ));
    assert!(!AssetNetworkRegistry::is_token(
        AssetId::Trx,
        NetworkId::Tron
    ));

    assert!(AssetNetworkRegistry::is_native(
        AssetId::Btc,
        NetworkId::Bitcoin
    ));
    assert!(!AssetNetworkRegistry::is_token(
        AssetId::Btc,
        NetworkId::Bitcoin
    ));

    // Token assets
    assert!(AssetNetworkRegistry::is_token(
        AssetId::Usdt,
        NetworkId::Tron
    ));
    assert!(!AssetNetworkRegistry::is_native(
        AssetId::Usdt,
        NetworkId::Tron
    ));

    assert!(AssetNetworkRegistry::is_token(
        AssetId::Usdt,
        NetworkId::Ethereum
    ));
    assert!(!AssetNetworkRegistry::is_native(
        AssetId::Usdt,
        NetworkId::Ethereum
    ));

    assert!(AssetNetworkRegistry::is_token(
        AssetId::Usdt,
        NetworkId::Bsc
    ));
    assert!(!AssetNetworkRegistry::is_native(
        AssetId::Usdt,
        NetworkId::Bsc
    ));

    assert!(AssetNetworkRegistry::is_token(
        AssetId::Usdt,
        NetworkId::TronNile
    ));
    assert!(!AssetNetworkRegistry::is_native(
        AssetId::Usdt,
        NetworkId::TronNile
    ));

    // Unsupported asset network returns false for both
    assert!(!AssetNetworkRegistry::is_token(
        AssetId::Btc,
        NetworkId::TronNile
    ));
    assert!(!AssetNetworkRegistry::is_native(
        AssetId::Btc,
        NetworkId::TronNile
    ));
}

#[test]
fn resolves_usdt_on_tron_nile() {
    let definition = AssetNetworkRegistry::resolve(AssetId::Usdt, NetworkId::TronNile).unwrap();

    assert_eq!(definition.decimals(), Decimals::SIX);
    assert_eq!(definition.decimals(), 6);
    assert_eq!(definition.token_standard(), Some(Standard::Trc20));
    assert_eq!(
        definition.token_address(),
        Some("TXYZopYRdj2D9XRtbG411XZZ3kM5VkAeBf")
    );
}

#[test]
fn finds_by_contract_address_and_network() {
    // USDT on Tron mainnet
    let def = AssetNetworkRegistry::find_by_contract_address(
        NetworkId::Tron,
        "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t",
    )
    .unwrap();
    assert_eq!(def.asset_id(), AssetId::Usdt);
    assert_eq!(def.decimals(), Decimals::SIX);
    assert_eq!(def.decimals().as_u8(), 6);

    // Case-insensitive matching for EVM
    let def_evm = AssetNetworkRegistry::find_by_contract_address(
        NetworkId::Bsc,
        "0x55d398326f99059ff775485246999027b3197955",
    )
    .unwrap();
    assert_eq!(def_evm.asset_id(), AssetId::Usdt);
    assert_eq!(def_evm.decimals(), Decimals::EIGHTEEN);
    assert_eq!(def_evm.decimals().as_u8(), 18);

    // Helper for AssetId
    assert_eq!(
        AssetNetworkRegistry::find_asset_id_by_contract_address(
            NetworkId::Bsc,
            "0x55d398326f99059ff775485246999027b3197955",
        ),
        Some(AssetId::Usdt)
    );
}

#[test]
fn resolves_native_for_network() {
    assert_eq!(
        AssetNetworkRegistry::native_asset_for_network(NetworkId::Tron),
        Some(AssetId::Trx)
    );
    assert_eq!(
        AssetNetworkRegistry::native_asset_for_network(NetworkId::TronNile),
        Some(AssetId::Trx)
    );
    assert_eq!(
        AssetNetworkRegistry::native_asset_for_network(NetworkId::Ethereum),
        Some(AssetId::Eth)
    );
    assert_eq!(
        AssetNetworkRegistry::native_asset_for_network(NetworkId::EthereumSepolia),
        Some(AssetId::Eth)
    );
    assert_eq!(
        AssetNetworkRegistry::native_asset_for_network(NetworkId::Bsc),
        Some(AssetId::Bnb)
    );
    assert_eq!(
        AssetNetworkRegistry::native_asset_for_network(NetworkId::BscTestnet),
        Some(AssetId::Bnb)
    );
    assert_eq!(
        AssetNetworkRegistry::native_asset_for_network(NetworkId::Polygon),
        Some(AssetId::Pol)
    );
    assert_eq!(
        AssetNetworkRegistry::native_asset_for_network(NetworkId::PolygonAmoy),
        Some(AssetId::Pol)
    );
    assert_eq!(
        AssetNetworkRegistry::native_asset_for_network(NetworkId::Bitcoin),
        Some(AssetId::Btc)
    );
    assert_eq!(
        AssetNetworkRegistry::native_asset_for_network(NetworkId::BitcoinTestnet),
        Some(AssetId::Btc)
    );
    assert_eq!(
        AssetNetworkRegistry::native_asset_for_network(NetworkId::Litecoin),
        Some(AssetId::Ltc)
    );
    assert_eq!(
        AssetNetworkRegistry::native_asset_for_network(NetworkId::LitecoinTestnet),
        Some(AssetId::Ltc)
    );
    assert_eq!(
        AssetNetworkRegistry::native_asset_for_network(NetworkId::Solana),
        Some(AssetId::Sol)
    );
    assert_eq!(
        AssetNetworkRegistry::native_asset_for_network(NetworkId::SolanaDevnet),
        Some(AssetId::Sol)
    );
}
