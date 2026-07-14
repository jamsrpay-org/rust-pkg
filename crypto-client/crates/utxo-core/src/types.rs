use chain_core::types::CryptoWallet;
use serde::Deserialize;

// ── Network configuration ───────────────────────────────────────────────────

/// Supported UTXO network variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UtxoNetwork {
    BitcoinMainnet,
    BitcoinTestnet,
    LitecoinMainnet,
    LitecoinTestnet,
}

impl UtxoNetwork {
    /// Returns the `bitcoin::Network` value for this UTXO network.
    ///
    /// For Litecoin we re-use Bitcoin's network type internally since the
    /// rust-bitcoin crate's `Network` enum covers the serialization logic we
    /// need.  Address encoding differences (HRP, version bytes) are handled
    /// separately in the client.
    pub fn to_bitcoin_network(self) -> bitcoin::Network {
        match self {
            Self::BitcoinMainnet => bitcoin::Network::Bitcoin,
            Self::BitcoinTestnet => bitcoin::Network::Testnet,
            // Litecoin re-uses Bitcoin network values for tx serialization;
            // address encoding is handled in the client via bech32 HRP.
            Self::LitecoinMainnet => bitcoin::Network::Bitcoin,
            Self::LitecoinTestnet => bitcoin::Network::Testnet,
        }
    }

    /// Returns `true` if this is a Litecoin network variant.
    pub fn is_litecoin(self) -> bool {
        matches!(self, Self::LitecoinMainnet | Self::LitecoinTestnet)
    }

    /// Returns the bech32 human-readable part (HRP) for address encoding.
    pub fn bech32_hrp(self) -> &'static str {
        match self {
            Self::BitcoinMainnet => "bc",
            Self::BitcoinTestnet => "tb",
            Self::LitecoinMainnet => "ltc",
            Self::LitecoinTestnet => "tltc",
        }
    }

    /// Returns the P2PKH address version byte.
    pub fn p2pkh_version(self) -> u8 {
        match self {
            Self::BitcoinMainnet => 0x00,
            Self::BitcoinTestnet => 0x6F,
            Self::LitecoinMainnet => 0x30,
            Self::LitecoinTestnet => 0x6F,
        }
    }

    /// Returns the WIF (Wallet Import Format) version byte.
    pub fn wif_version(self) -> u8 {
        match self {
            Self::BitcoinMainnet => 0x80,
            Self::BitcoinTestnet => 0xEF,
            Self::LitecoinMainnet => 0xB0,
            Self::LitecoinTestnet => 0xEF,
        }
    }
}

// ── UTXO (unspent transaction output) ───────────────────────────────────────

/// An unspent transaction output returned by the Esplora API.
#[derive(Debug, Clone, Deserialize)]
pub struct Utxo {
    pub txid: String,
    pub vout: u32,
    pub value: u64,
    #[serde(default)]
    pub status: UtxoStatus,
}

/// Confirmation status of a UTXO.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct UtxoStatus {
    #[serde(default)]
    pub confirmed: bool,
}

// ── Transfer types ──────────────────────────────────────────────────────────

/// A prepared (unsigned) UTXO transfer.
#[derive(Debug, Clone)]
pub struct UtxoPreparedTransfer {
    /// Raw unsigned transaction bytes (serialized Bitcoin tx without witness sigs).
    pub raw_tx_bytes: Vec<u8>,
    /// Estimated fee in satoshis.
    pub fee: u64,
    /// Network this transfer targets.
    pub network: UtxoNetwork,
}

/// A signed UTXO transfer, ready to broadcast.
#[derive(Debug)]
pub struct UtxoSignedTransfer {
    /// The prepared transfer metadata.
    pub prepared: UtxoPreparedTransfer,
    /// Fully signed, serialized transaction bytes (ready for broadcast).
    pub signed_tx_bytes: Vec<u8>,
}

// ── Wallet ──────────────────────────────────────────────────────────────────

/// A UTXO wallet — private key (WIF) + address.
#[derive(Debug)]
pub struct UtxoWallet {
    /// Private key in hex encoding.
    pub private_key: String,
    /// Address string (bech32 for BTC, Litecoin-specific for LTC).
    pub address: String,
}

impl From<UtxoWallet> for CryptoWallet {
    fn from(wallet: UtxoWallet) -> Self {
        CryptoWallet {
            address: wallet.address,
            private_key: wallet.private_key,
        }
    }
}

// ── Esplora API response types ──────────────────────────────────────────────

/// Fee estimates from Esplora — map of confirmation target → sat/vByte.
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct FeeEstimates(pub std::collections::HashMap<String, f64>);

impl FeeEstimates {
    /// Get the recommended fee rate for a target of ~6 blocks (~1 hour).
    /// Falls back to 1.0 sat/vByte minimum.
    pub fn normal_fee_rate(&self) -> f64 {
        self.0
            .get("6")
            .copied()
            .or_else(|| self.0.get("3").copied())
            .unwrap_or(1.0)
            .max(1.0)
    }
}

/// Broadcast result from Esplora — returns the txid as plain text.
#[derive(Debug)]
pub struct EsploraBroadcastResult {
    pub txid: String,
}

/// Address balance/stats from Esplora.
#[derive(Debug, Deserialize)]
pub struct EsploraAddressStats {
    pub address: String,
    pub chain_stats: EsploraChainStats,
}

#[derive(Debug, Deserialize)]
pub struct EsploraChainStats {
    pub funded_txo_count: u64,
    pub funded_txo_sum: u64,
    pub spent_txo_count: u64,
    pub spent_txo_sum: u64,
}
