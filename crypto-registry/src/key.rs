use jamsrpay_types::currency::{AssetId, NetworkId};

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
