use alloy::{
    consensus::{SignableTransaction, TxEnvelope, TxLegacy},
    eips::eip2718::Encodable2718,
    transports::http::reqwest,
    primitives::{Address, Bytes, TxKind, B256, U256},
    providers::{Provider, ProviderBuilder},
    rlp::Decodable,
    signers::{local::PrivateKeySigner, SignerSync},
    sol,
    sol_types::SolCall,
};

use crate::error::EvmClientError;
use crate::types::{EvmPreparedTransfer, EvmSignedTransfer, EvmWallet};

// ── ERC20 ABI (shared by BEP20, ERC20, etc.) ───────────────────────────────

sol! {
    function balanceOf(address account) external view returns (uint256);
    function transfer(address to, uint256 amount) external returns (bool);
}

/// Default gas limit for a native (ETH/BNB/MATIC) transfer.
const NATIVE_TRANSFER_GAS: u128 = 21_000;

// ── EvmClient ───────────────────────────────────────────────────────────────

/// A reusable EVM client powered by alloy.
///
/// Provides wallet generation, transaction building, signing, broadcasting,
/// and balance queries for any EVM-compatible chain (BSC, ETH, Polygon, etc.).
///
/// Chain-specific crates (e.g. `binance-smart-chain`) wrap this client and
/// add only currency routing + contract address configuration.
pub struct EvmClient {
    rpc_url: reqwest::Url,
    chain_id: u64,
}

impl EvmClient {
    pub fn new(rpc_url: &str, chain_id: u64) -> Result<Self, EvmClientError> {
        let url = rpc_url
            .parse::<reqwest::Url>()
            .map_err(|e| EvmClientError::RpcError(format!("invalid RPC URL: {}", e)))?;
        Ok(Self {
            rpc_url: url,
            chain_id,
        })
    }

    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }

    // ── Wallet ──────────────────────────────────────────────────────────────

    /// Generate a new EVM wallet (private key + 0x-prefixed checksummed address).
    pub fn generate_wallet() -> EvmWallet {
        let signer = PrivateKeySigner::random();
        let address = signer.address().to_checksum(None);
        let private_key = hex::encode(signer.credential().to_bytes());
        EvmWallet {
            private_key,
            address,
        }
    }

    // ── Native balance ──────────────────────────────────────────────────────

    /// Get native token balance (ETH, BNB, MATIC, etc.) in wei.
    pub async fn native_balance(&self, address: &str) -> Result<u128, EvmClientError> {
        let provider = ProviderBuilder::new().connect_http(self.rpc_url.clone());
        let addr = parse_address(address)?;
        let balance = provider
            .get_balance(addr)
            .await
            .map_err(|e| EvmClientError::RpcError(e.to_string()))?;
        Ok(balance.to::<u128>())
    }

    // ── ERC20 / BEP20 balance ───────────────────────────────────────────────

    /// Get ERC20/BEP20 token balance in smallest unit.
    pub async fn erc20_balance(
        &self,
        address: &str,
        contract_address: &str,
    ) -> Result<u128, EvmClientError> {
        let provider = ProviderBuilder::new().connect_http(self.rpc_url.clone());
        let addr = parse_address(address)?;
        let contract_addr = parse_address(contract_address)?;

        let calldata = balanceOfCall { account: addr }.abi_encode();
        let tx = alloy::rpc::types::TransactionRequest::default()
            .to(contract_addr)
            .input(Bytes::from(calldata).into());

        let result = provider
            .call(tx)
            .await
            .map_err(|e| EvmClientError::RpcError(e.to_string()))?;

        let balance: U256 = balanceOfCall::abi_decode_returns(&result)
            .map_err(|e| EvmClientError::RpcError(format!("ABI decode error: {}", e)))?;

        Ok(balance.to::<u128>())
    }

    // ── Build native transfer ───────────────────────────────────────────────

    /// Build an unsigned native transfer transaction.
    /// Fetches nonce, gas price, and estimates gas from the RPC node.
    pub async fn build_native_transfer(
        &self,
        from: &str,
        to: &str,
        amount: u128,
    ) -> Result<EvmPreparedTransfer, EvmClientError> {
        let provider = ProviderBuilder::new().connect_http(self.rpc_url.clone());
        let from_addr = parse_address(from)?;
        let to_addr = parse_address(to)?;

        let nonce = provider
            .get_transaction_count(from_addr)
            .await
            .map_err(|e| EvmClientError::RpcError(e.to_string()))?;
        let gas_price = provider
            .get_gas_price()
            .await
            .map_err(|e| EvmClientError::RpcError(e.to_string()))?;

        let tx_request = alloy::rpc::types::TransactionRequest::default()
            .from(from_addr)
            .to(to_addr)
            .value(U256::from(amount));
        let gas_limit = provider
            .estimate_gas(tx_request)
            .await
            .map_err(|e| EvmClientError::RpcError(e.to_string()))?;

        Ok(EvmPreparedTransfer {
            nonce,
            gas_price,
            gas_limit,
            to: to.to_string(),
            value: amount,
            data: vec![],
            chain_id: self.chain_id,
        })
    }

    // ── Build ERC20 / BEP20 transfer ────────────────────────────────────────

    /// Build an unsigned ERC20/BEP20 token transfer transaction.
    pub async fn build_erc20_transfer(
        &self,
        from: &str,
        to: &str,
        amount: u128,
        contract_address: &str,
    ) -> Result<EvmPreparedTransfer, EvmClientError> {
        let provider = ProviderBuilder::new().connect_http(self.rpc_url.clone());
        let from_addr = parse_address(from)?;
        let to_addr = parse_address(to)?;
        let contract_addr = parse_address(contract_address)?;

        // Encode ERC20 transfer(address, uint256) calldata
        let calldata = transferCall {
            to: to_addr,
            amount: U256::from(amount),
        }
        .abi_encode();

        let nonce = provider
            .get_transaction_count(from_addr)
            .await
            .map_err(|e| EvmClientError::RpcError(e.to_string()))?;
        let gas_price = provider
            .get_gas_price()
            .await
            .map_err(|e| EvmClientError::RpcError(e.to_string()))?;

        let tx_request = alloy::rpc::types::TransactionRequest::default()
            .from(from_addr)
            .to(contract_addr)
            .input(Bytes::from(calldata.clone()).into());
        let gas_limit = provider
            .estimate_gas(tx_request)
            .await
            .map_err(|e| EvmClientError::RpcError(e.to_string()))?;

        Ok(EvmPreparedTransfer {
            nonce,
            gas_price,
            gas_limit,
            to: contract_address.to_string(),
            value: 0,
            data: calldata,
            chain_id: self.chain_id,
        })
    }

    // ── Sign ────────────────────────────────────────────────────────────────

    /// Sign a prepared transfer with a private key (local, no RPC call).
    /// Uses alloy's secp256k1 signer with EIP-155 replay protection.
    pub fn sign(
        prepared: &EvmPreparedTransfer,
        private_key: &[u8],
    ) -> Result<EvmSignedTransfer, EvmClientError> {
        let to_addr = parse_address(&prepared.to)?;

        let tx = TxLegacy {
            chain_id: Some(prepared.chain_id),
            nonce: prepared.nonce,
            gas_price: prepared.gas_price,
            gas_limit: prepared.gas_limit,
            to: TxKind::Call(to_addr),
            value: U256::from(prepared.value),
            input: Bytes::from(prepared.data.clone()),
        };

        let signed_bytes = sign_legacy_tx(tx, private_key)?;

        Ok(EvmSignedTransfer {
            prepared: prepared.clone(),
            signed_tx_bytes: signed_bytes,
        })
    }

    /// Sign raw unsigned transaction bytes with a private key.
    ///
    /// Input: RLP-encoded unsigned `TxLegacy` (with EIP-155 chain_id fields).
    /// Output: Full RLP-encoded signed transaction bytes (EIP-2718 envelope).
    pub fn sign_raw(private_key: &[u8], raw_tx: &[u8]) -> Result<Vec<u8>, EvmClientError> {
        let tx = TxLegacy::decode(&mut &raw_tx[..])
            .map_err(|e| EvmClientError::RpcError(format!("RLP decode error: {}", e)))?;
        sign_legacy_tx(tx, private_key)
    }

    // ── Broadcast ───────────────────────────────────────────────────────────

    /// Broadcast a signed transaction to the network.
    /// Returns the transaction hash.
    pub async fn broadcast(&self, signed_tx_bytes: &[u8]) -> Result<String, EvmClientError> {
        let provider = ProviderBuilder::new().connect_http(self.rpc_url.clone());
        let pending = provider
            .send_raw_transaction(signed_tx_bytes)
            .await
            .map_err(|e| EvmClientError::RpcError(e.to_string()))?;
        Ok(pending.tx_hash().to_string())
    }

    // ── Gas estimation ──────────────────────────────────────────────────────

    /// Estimate the maximum native token amount that can be sent from an address
    /// after accounting for gas costs.
    pub async fn estimate_native_withdrawable(
        &self,
        address: &str,
    ) -> Result<u128, EvmClientError> {
        let balance = self.native_balance(address).await?;
        if balance == 0 {
            return Ok(0);
        }

        let provider = ProviderBuilder::new().connect_http(self.rpc_url.clone());
        let gas_price = provider
            .get_gas_price()
            .await
            .map_err(|e| EvmClientError::RpcError(e.to_string()))?;

        let gas_cost = gas_price * NATIVE_TRANSFER_GAS;
        Ok(balance.saturating_sub(gas_cost))
    }
}

// ── Private helpers ─────────────────────────────────────────────────────────

fn parse_address(address: &str) -> Result<Address, EvmClientError> {
    address
        .parse::<Address>()
        .map_err(|_| EvmClientError::RpcError(format!("invalid address: {}", address)))
}

/// Sign a TxLegacy with a raw private key, returning the EIP-2718 encoded signed bytes.
fn sign_legacy_tx(tx: TxLegacy, private_key: &[u8]) -> Result<Vec<u8>, EvmClientError> {
    if private_key.len() != 32 {
        return Err(EvmClientError::InvalidPrivateKey);
    }
    let key_bytes = B256::from_slice(private_key);
    let signer = PrivateKeySigner::from_bytes(&key_bytes)
        .map_err(|e| EvmClientError::SignError(e.to_string()))?;

    let sig = signer
        .sign_hash_sync(&tx.signature_hash())
        .map_err(|e| EvmClientError::SignError(e.to_string()))?;

    let signed = tx.into_signed(sig);
    let envelope = TxEnvelope::Legacy(signed);

    Ok(envelope.encoded_2718())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_wallet() {
        let wallet = EvmClient::generate_wallet();
        assert!(wallet.address.starts_with("0x"));
        assert_eq!(wallet.address.len(), 42); // 0x + 40 hex chars
        assert_eq!(wallet.private_key.len(), 64); // 32 bytes = 64 hex chars
        dbg!(&wallet);
    }

    #[test]
    fn test_sign_roundtrip() {
        // Generate a wallet and sign a dummy prepared transfer
        let wallet = EvmClient::generate_wallet();
        let private_key = hex::decode(&wallet.private_key).unwrap();

        let prepared = EvmPreparedTransfer {
            nonce: 0,
            gas_price: 5_000_000_000, // 5 gwei
            gas_limit: 21_000,
            to: "0x0000000000000000000000000000000000000001".to_string(),
            value: 1_000_000_000_000_000_000, // 1 ETH/BNB in wei
            data: vec![],
            chain_id: 56, // BSC
        };

        let signed = EvmClient::sign(&prepared, &private_key).unwrap();
        assert!(!signed.signed_tx_bytes.is_empty());
        assert_eq!(signed.prepared.chain_id, 56);
    }
}
