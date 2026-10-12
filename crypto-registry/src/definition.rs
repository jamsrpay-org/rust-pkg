use jamsrpay_types::currency::{AssetId, Decimals, NetworkId};

use crate::key::AssetNetworkKey;
use crate::representation::AssetNetworkRepresentation;
use crate::standard::TokenStandard;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AssetNetworkDefinition {
    pub key: AssetNetworkKey,
    pub decimals: Decimals,
    pub representation: AssetNetworkRepresentation,
}

impl AssetNetworkDefinition {
    pub const fn native(asset_id: AssetId, network_id: NetworkId, decimals: Decimals) -> Self {
        Self {
            key: AssetNetworkKey::new(asset_id, network_id),
            decimals,
            representation: AssetNetworkRepresentation::Native,
        }
    }

    pub const fn token(
        asset_id: AssetId,
        network_id: NetworkId,
        decimals: Decimals,
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

    pub const fn decimals(&self) -> Decimals {
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

    pub const fn is_token(&self) -> bool {
        !self.is_native()
    }
}
