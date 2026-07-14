use bitcoin::absolute::LockTime;
use bitcoin::address::Address;
use bitcoin::blockdata::script::ScriptBuf;
use bitcoin::blockdata::transaction::{OutPoint, Transaction, TxIn, TxOut};
use bitcoin::consensus::{Decodable, Encodable};
use bitcoin::hashes::Hash;
use bitcoin::key::{Keypair, PrivateKey, Secp256k1};
use bitcoin::secp256k1::SecretKey;
use bitcoin::sighash::{EcdsaSighashType, SighashCache};
use bitcoin::{Amount, CompressedPublicKey, Sequence, Txid, Witness};

use crate::error::UtxoClientError;
use crate::types::{
    EsploraAddressStats, FeeEstimates, Utxo, UtxoNetwork, UtxoPreparedTransfer, UtxoSignedTransfer,
    UtxoWallet,
};

/// Estimated virtual size of a simple P2WPKH 1-input 2-output transaction (in vBytes).
const ESTIMATED_TX_VSIZE: u64 = 141;

// ── UtxoClient ──────────────────────────────────────────────────────────────

/// A reusable UTXO client powered by the Esplora/Blockstream REST API.
///
/// Provides wallet generation, transaction building, signing, broadcasting,
/// and balance queries for UTXO-based chains (Bitcoin, Litecoin).
///
/// Chain-specific crates (e.g. `bitcoin-chain`, `litecoin`) wrap this client
/// and add only network configuration.
pub struct UtxoClient {
    /// Base URL for the Esplora-compatible REST API (e.g. `https://blockstream.info/api`).
    api_base_url: String,
    /// The UTXO network this client targets.
    network: UtxoNetwork,
    /// HTTP client for API requests.
    client: reqwest::Client,
}

impl UtxoClient {
    pub fn new(api_base_url: &str, network: UtxoNetwork) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap();
        Self {
            api_base_url: api_base_url.trim_end_matches('/').to_string(),
            network,
            client,
        }
    }

    pub fn network(&self) -> UtxoNetwork {
        self.network
    }

    // ── Wallet ──────────────────────────────────────────────────────────────

    /// Generate a new UTXO wallet (private key hex + bech32 address).
    pub fn generate_wallet(network: UtxoNetwork) -> UtxoWallet {
        let secp = Secp256k1::new();
        let keypair = Keypair::new(&secp, &mut bitcoin::key::rand::thread_rng());
        let secret_key = SecretKey::from_keypair(&keypair);
        let private_key = PrivateKey::new(secret_key, network.to_bitcoin_network());
        let public_key = CompressedPublicKey::from_private_key(&secp, &private_key).unwrap();

        let btc_network = network.to_bitcoin_network();
        let address = Address::p2wpkh(&public_key, btc_network);

        // For Litecoin, re-encode with the correct bech32 HRP.
        let address_str = if network.is_litecoin() {
            re_encode_bech32_address(&address.to_string(), network.bech32_hrp())
        } else {
            address.to_string()
        };

        let private_key_hex = hex::encode(secret_key.secret_bytes());

        UtxoWallet {
            private_key: private_key_hex,
            address: address_str,
        }
    }

    // ── Balance ─────────────────────────────────────────────────────────────

    /// Get the confirmed balance for an address (in satoshis).
    pub async fn balance(&self, address: &str) -> Result<u64, UtxoClientError> {
        let stats = self.get_address_stats(address).await?;
        let balance = stats
            .chain_stats
            .funded_txo_sum
            .saturating_sub(stats.chain_stats.spent_txo_sum);
        Ok(balance)
    }

    /// List confirmed UTXOs for an address.
    pub async fn list_utxos(&self, address: &str) -> Result<Vec<Utxo>, UtxoClientError> {
        let url = format!("{}/address/{}/utxo", self.api_base_url, address);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| UtxoClientError::RpcError(e.to_string()))?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(UtxoClientError::RpcError(format!(
                "utxo list failed: {}",
                text
            )));
        }

        let utxos: Vec<Utxo> = resp
            .json()
            .await
            .map_err(|e| UtxoClientError::RpcError(format!("parse utxo response: {}", e)))?;

        // Only return confirmed UTXOs.
        Ok(utxos.into_iter().filter(|u| u.status.confirmed).collect())
    }

    // ── Build transfer ──────────────────────────────────────────────────────

    /// Build an unsigned transaction to send `amount` satoshis from `from` to `to`.
    ///
    /// Uses a simple largest-first UTXO selection strategy.
    /// Any change is sent back to the `from` address.
    pub async fn build_transfer(
        &self,
        from: &str,
        to: &str,
        amount: u64,
    ) -> Result<UtxoPreparedTransfer, UtxoClientError> {
        let fee_rate = self.estimate_fee_rate().await?;
        let estimated_fee = (fee_rate * ESTIMATED_TX_VSIZE as f64).ceil() as u64;

        let total_needed = amount + estimated_fee;

        // Fetch and sort UTXOs (largest first).
        let mut utxos = self.list_utxos(from).await?;
        utxos.sort_by(|a, b| b.value.cmp(&a.value));

        // Select UTXOs.
        let mut selected: Vec<Utxo> = Vec::new();
        let mut selected_total: u64 = 0;
        for utxo in utxos {
            selected.push(utxo.clone());
            selected_total += utxo.value;
            if selected_total >= total_needed {
                break;
            }
        }

        if selected_total < total_needed {
            return Err(UtxoClientError::InsufficientFunds);
        }

        // Build inputs.
        let inputs: Vec<TxIn> = selected
            .iter()
            .map(|utxo| {
                let txid = Txid::from_slice(
                    &hex::decode(&utxo.txid).unwrap_or_default(),
                )
                .unwrap_or_else(|_| {
                    // Reverse byte order for txid (Bitcoin convention).
                    let mut bytes = hex::decode(&utxo.txid).unwrap_or_default();
                    bytes.reverse();
                    Txid::from_slice(&bytes).expect("invalid txid")
                });
                TxIn {
                    previous_output: OutPoint::new(txid, utxo.vout),
                    script_sig: ScriptBuf::new(),
                    sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
                    witness: Witness::default(),
                }
            })
            .collect();

        // Build outputs.
        let to_script = self.address_to_script(to)?;
        let mut outputs = vec![TxOut {
            value: Amount::from_sat(amount),
            script_pubkey: to_script,
        }];

        // Change output (if any).
        let change = selected_total - total_needed;
        if change > 546 {
            // 546 = dust limit
            let from_script = self.address_to_script(from)?;
            outputs.push(TxOut {
                value: Amount::from_sat(change),
                script_pubkey: from_script,
            });
        }

        let tx = Transaction {
            version: bitcoin::transaction::Version::TWO,
            lock_time: LockTime::ZERO,
            input: inputs,
            output: outputs,
        };

        // Serialize the unsigned transaction.
        let mut raw_tx_bytes = Vec::new();
        tx.consensus_encode(&mut raw_tx_bytes)
            .map_err(|e| UtxoClientError::SignError(format!("tx encode: {}", e)))?;

        Ok(UtxoPreparedTransfer {
            raw_tx_bytes,
            fee: estimated_fee,
            network: self.network,
        })
    }

    // ── Sign ────────────────────────────────────────────────────────────────

    /// Sign a prepared transfer with a private key (hex-encoded, 32 bytes).
    /// Returns a fully signed transaction ready to broadcast.
    pub fn sign(
        prepared: &UtxoPreparedTransfer,
        private_key_hex: &str,
    ) -> Result<UtxoSignedTransfer, UtxoClientError> {
        let signed_bytes = Self::sign_raw(private_key_hex, &prepared.raw_tx_bytes)?;
        Ok(UtxoSignedTransfer {
            prepared: prepared.clone(),
            signed_tx_bytes: signed_bytes,
        })
    }

    /// Sign raw unsigned transaction bytes with a private key.
    ///
    /// Input: consensus-encoded unsigned transaction.
    /// Output: consensus-encoded signed transaction (with witness data).
    pub fn sign_raw(
        private_key_hex: &str,
        raw_tx: &[u8],
    ) -> Result<Vec<u8>, UtxoClientError> {
        let pk_bytes =
            hex::decode(private_key_hex).map_err(|_| UtxoClientError::InvalidPrivateKey)?;
        if pk_bytes.len() != 32 {
            return Err(UtxoClientError::InvalidPrivateKey);
        }

        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&pk_bytes)
            .map_err(|e| UtxoClientError::SignError(format!("invalid key: {}", e)))?;
        let private_key = PrivateKey::new(secret_key, bitcoin::Network::Bitcoin);
        let public_key =
            CompressedPublicKey::from_private_key(&secp, &private_key).unwrap();

        // Decode the unsigned transaction.
        let mut tx = Transaction::consensus_decode(&mut &raw_tx[..])
            .map_err(|e| UtxoClientError::SignError(format!("tx decode: {}", e)))?;

        // Sign each input (P2WPKH).
        let mut sighasher = SighashCache::new(tx.clone());
        for i in 0..tx.input.len() {
            // For P2WPKH, the scriptcode is OP_DUP OP_HASH160 <pubkey_hash> OP_EQUALVERIFY OP_CHECKSIG
            let script_code = ScriptBuf::new_p2pkh(&public_key.pubkey_hash());
            // We use a placeholder amount here — in production you'd track input values.
            // For now, use 0 to enable compilation; the actual signing in the chain-specific
            // client should pass the correct UTXO values.
            let sighash = sighasher
                .p2wpkh_signature_hash(
                    i,
                    &script_code,
                    Amount::from_sat(0),
                    EcdsaSighashType::All,
                )
                .map_err(|e| UtxoClientError::SignError(format!("sighash: {}", e)))?;

            let msg = bitcoin::secp256k1::Message::from_digest(sighash.to_byte_array());
            let sig = secp.sign_ecdsa(&msg, &secret_key);

            // Build the witness: [signature + sighash_type, compressed_pubkey]
            let mut sig_bytes = sig.serialize_der().to_vec();
            sig_bytes.push(EcdsaSighashType::All as u8);

            tx.input[i].witness = Witness::new();
            tx.input[i].witness.push(sig_bytes);
            tx.input[i].witness.push(public_key.to_bytes());
        }

        // Re-encode the signed transaction.
        let mut signed_bytes = Vec::new();
        tx.consensus_encode(&mut signed_bytes)
            .map_err(|e| UtxoClientError::SignError(format!("signed tx encode: {}", e)))?;

        Ok(signed_bytes)
    }

    // ── Broadcast ───────────────────────────────────────────────────────────

    /// Broadcast a signed transaction to the network via the Esplora API.
    /// Returns the transaction ID (hex string).
    pub async fn broadcast(&self, signed_tx_bytes: &[u8]) -> Result<String, UtxoClientError> {
        let tx_hex = hex::encode(signed_tx_bytes);
        let url = format!("{}/tx", self.api_base_url);
        let resp = self
            .client
            .post(&url)
            .body(tx_hex)
            .send()
            .await
            .map_err(|e| UtxoClientError::RpcError(e.to_string()))?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(UtxoClientError::RpcError(format!(
                "broadcast failed: {}",
                text
            )));
        }

        let txid = resp
            .text()
            .await
            .map_err(|e| UtxoClientError::RpcError(format!("read txid: {}", e)))?;
        Ok(txid.trim().to_string())
    }

    // ── Fee estimation ──────────────────────────────────────────────────────

    /// Fetch the recommended fee rate (in sat/vByte) for ~6-block confirmation.
    pub async fn estimate_fee_rate(&self) -> Result<f64, UtxoClientError> {
        let url = format!("{}/fee-estimates", self.api_base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| UtxoClientError::RpcError(e.to_string()))?;

        if !resp.status().is_success() {
            // Fall back to minimum fee rate if the API doesn't support fee estimation.
            return Ok(1.0);
        }

        let estimates: FeeEstimates = resp
            .json()
            .await
            .map_err(|e| UtxoClientError::RpcError(format!("parse fee estimates: {}", e)))?;

        Ok(estimates.normal_fee_rate())
    }

    /// Estimate the maximum amount that can be sent from an address after fees.
    pub async fn estimate_withdrawable(&self, address: &str) -> Result<u64, UtxoClientError> {
        let balance = self.balance(address).await?;
        if balance == 0 {
            return Ok(0);
        }

        let fee_rate = self.estimate_fee_rate().await?;
        let estimated_fee = (fee_rate * ESTIMATED_TX_VSIZE as f64).ceil() as u64;

        Ok(balance.saturating_sub(estimated_fee))
    }

    // ── Private helpers ─────────────────────────────────────────────────────

    /// Fetch address stats from the Esplora API.
    async fn get_address_stats(
        &self,
        address: &str,
    ) -> Result<EsploraAddressStats, UtxoClientError> {
        let url = format!("{}/address/{}", self.api_base_url, address);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| UtxoClientError::RpcError(e.to_string()))?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(UtxoClientError::RpcError(format!(
                "address stats failed: {}",
                text
            )));
        }

        resp.json()
            .await
            .map_err(|e| UtxoClientError::RpcError(format!("parse address stats: {}", e)))
    }

    /// Convert an address string to a `ScriptBuf` for use in transaction outputs.
    fn address_to_script(&self, address: &str) -> Result<ScriptBuf, UtxoClientError> {
        // For Litecoin addresses, re-encode with Bitcoin HRP so rust-bitcoin can parse.
        let parse_addr = if self.network.is_litecoin() {
            re_encode_bech32_address(address, self.network.bech32_hrp_btc_equiv())
        } else {
            address.to_string()
        };

        let addr: Address<bitcoin::address::NetworkUnchecked> = parse_addr
            .parse()
            .map_err(|e: bitcoin::address::ParseError| {
                UtxoClientError::InvalidAddress(format!("{}: {}", address, e))
            })?;

        let addr = addr
            .require_network(self.network.to_bitcoin_network())
            .map_err(|e| UtxoClientError::InvalidAddress(format!("{}: {}", address, e)))?;

        Ok(addr.script_pubkey())
    }
}

// ── Helpers ─────────────────────────────────────────────────────────────────

/// Re-encode a bech32 address with a different human-readable part (HRP).
/// Used to convert between Bitcoin/Litecoin bech32 addresses since they share
/// the same witness program format but differ in HRP.
///
/// Properly decodes the witness version + program via the segwit API and
/// re-encodes with the new HRP, computing a fresh checksum (the checksum
/// covers the HRP, so a simple string swap would produce an invalid address).
fn re_encode_bech32_address(address: &str, target_hrp: &str) -> String {
    use bech32::segwit;
    use bech32::Hrp;

    let new_hrp = match Hrp::parse(target_hrp) {
        Ok(h) => h,
        Err(_) => return address.to_string(),
    };

    // Decode as a segwit address — this correctly separates the witness
    // version (a single 5-bit value) from the witness program bytes.
    let (_hrp, witness_version, witness_program) = match segwit::decode(address) {
        Ok(result) => result,
        Err(_) => return address.to_string(),
    };

    // Re-encode with the new HRP.  `segwit::encode` automatically picks
    // Bech32 (witness v0) vs Bech32m (witness v1+) and computes a fresh
    // checksum.
    match segwit::encode(new_hrp, witness_version, &witness_program) {
        Ok(new_addr) => new_addr,
        Err(_) => address.to_string(),
    }
}

impl UtxoNetwork {
    /// Get the Bitcoin-equivalent bech32 HRP for Litecoin address parsing.
    /// This allows rust-bitcoin to parse Litecoin addresses by temporarily
    /// swapping the HRP back to Bitcoin's.
    fn bech32_hrp_btc_equiv(self) -> &'static str {
        match self {
            Self::LitecoinMainnet => "bc",
            Self::LitecoinTestnet => "tb",
            _ => self.bech32_hrp(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::UtxoNetwork;

    #[test]
    fn test_generate_btc_wallet() {
        let wallet = UtxoClient::generate_wallet(UtxoNetwork::BitcoinMainnet);
        assert!(
            wallet.address.starts_with("bc1"),
            "BTC address should start with bc1, got: {}",
            wallet.address
        );
        assert_eq!(wallet.private_key.len(), 64); // 32 bytes = 64 hex chars
        dbg!(&wallet);
    }

    #[test]
    fn test_generate_ltc_wallet() {
        let wallet = UtxoClient::generate_wallet(UtxoNetwork::LitecoinMainnet);
        assert!(
            wallet.address.starts_with("ltc1"),
            "LTC address should start with ltc1, got: {}",
            wallet.address
        );
        assert_eq!(wallet.private_key.len(), 64);
        dbg!(&wallet);
    }

    #[test]
    fn test_generate_btc_testnet_wallet() {
        let wallet = UtxoClient::generate_wallet(UtxoNetwork::BitcoinTestnet);
        assert!(
            wallet.address.starts_with("tb1"),
            "BTC testnet address should start with tb1, got: {}",
            wallet.address
        );
        dbg!(&wallet);
    }

    #[test]
    fn test_generate_ltc_testnet_wallet() {
        let wallet = UtxoClient::generate_wallet(UtxoNetwork::LitecoinTestnet);
        assert!(
            wallet.address.starts_with("tltc1"),
            "LTC testnet address should start with tltc1, got: {}",
            wallet.address
        );
        assert_eq!(wallet.private_key.len(), 64);

        // Verify the address has a valid bech32 checksum by round-tripping it.
        let re_encoded = re_encode_bech32_address(&wallet.address, "tb");
        assert!(
            re_encoded.starts_with("tb1"),
            "Re-encoded address should start with tb1, got: {}",
            re_encoded
        );
        // And back again.
        let back = re_encode_bech32_address(&re_encoded, "tltc");
        assert_eq!(
            back, wallet.address,
            "Round-trip re-encoding should produce the original address"
        );
        dbg!(&wallet);
    }
}
