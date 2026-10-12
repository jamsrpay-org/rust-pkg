use jamsrpay_types::currency::{AssetId, NetworkId};
use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum AssetNetworkRegistryError {
    #[error("unsupported asset/network combination: {asset_id:?} on {network_id:?}")]
    UnsupportedAssetNetwork {
        asset_id: AssetId,
        network_id: NetworkId,
    },
}
