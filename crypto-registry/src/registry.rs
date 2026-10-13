use jamsrpay_types::currency::{AssetId, Decimals, NetworkId};

use crate::definition::AssetNetworkDefinition;
use crate::definitions::DEFINITIONS;
use crate::error::AssetNetworkRegistryError;
use crate::key::AssetNetworkKey;

/// Immutable registry of engineering-defined asset/network combinations.
///
/// Contains technical protocol metadata only:
/// - Atomic-unit precision (`Decimals`)
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

    /// Check whether a combination is a token representation.
    pub fn is_token(asset_id: AssetId, network_id: NetworkId) -> bool {
        Self::resolve(asset_id, network_id)
            .map(|definition| definition.is_token())
            .unwrap_or(false)
    }

    /// Check whether a combination is a native asset representation.
    pub fn is_native(asset_id: AssetId, network_id: NetworkId) -> bool {
        Self::resolve(asset_id, network_id)
            .map(|definition| definition.is_native())
            .unwrap_or(false)
    }

    /// Resolve precision (`Decimals`) for an asset/network combination.
    pub fn decimals(
        asset_id: AssetId,
        network_id: NetworkId,
    ) -> Result<Decimals, AssetNetworkRegistryError> {
        Self::resolve(asset_id, network_id).map(|definition| definition.decimals())
    }

    /// Find an asset definition by token contract address on a specific network.
    ///
    /// The contract address comparison is case-insensitive.
    pub fn find_by_contract_address(
        network_id: NetworkId,
        contract_address: &str,
    ) -> Option<&'static AssetNetworkDefinition> {
        let trimmed = contract_address.trim();
        DEFINITIONS.iter().find(|definition| {
            definition.network_id() == network_id
                && definition
                    .token_address()
                    .is_some_and(|addr| addr.eq_ignore_ascii_case(trimmed))
        })
    }

    /// Resolve `AssetId` by token contract address on a specific network.
    pub fn find_asset_id_by_contract_address(
        network_id: NetworkId,
        contract_address: &str,
    ) -> Option<AssetId> {
        Self::find_by_contract_address(network_id, contract_address).map(|d| d.asset_id())
    }

    /// Find the native asset definition for a network.
    pub fn native_for_network(network_id: NetworkId) -> Option<&'static AssetNetworkDefinition> {
        DEFINITIONS
            .iter()
            .find(|d| d.is_native() && d.network_id() == network_id)
    }

    /// Resolve the native `AssetId` for a network.
    pub fn native_asset_for_network(network_id: NetworkId) -> Option<AssetId> {
        Self::native_for_network(network_id).map(|d| d.asset_id())
    }
}
