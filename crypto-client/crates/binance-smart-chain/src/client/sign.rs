use crate::error::BscClientError;
use secp256k1::{Message, Secp256k1, SecretKey, ecdsa::RecoverableSignature};
use sha3::{Digest, Keccak256};

/// Sign unsigned transaction bytes (RLP-encoded) with a private key.
///
/// Returns (v, r, s) tuple where v includes the EIP-155 chain replay protection.
pub fn sign_transaction(
    unsigned_tx_bytes: &[u8],
    private_key: &[u8],
    chain_id: u64,
) -> Result<(u64, Vec<u8>, Vec<u8>), BscClientError> {
    // Keccak256 hash of the unsigned tx
    let tx_hash = Keccak256::digest(unsigned_tx_bytes);

    // Sign with secp256k1
    let secp = Secp256k1::new();
    let secret_array: [u8; 32] = private_key.try_into().map_err(|e| {
        eprintln!("Invalid private key: {}", e);
        BscClientError::InvalidPrivateKey
    })?;
    let secret_key = SecretKey::from_byte_array(secret_array)
        .map_err(|e| BscClientError::SignError(e.to_string()))?;
    let message = Message::from_digest(tx_hash.into());

    let signature: RecoverableSignature = secp.sign_ecdsa_recoverable(message, &secret_key);
    let (recovery_id, sig_bytes) = signature.serialize_compact();

    // r = first 32 bytes, s = last 32 bytes
    let r = sig_bytes[..32].to_vec();
    let s = sig_bytes[32..].to_vec();

    // EIP-155: v = chain_id * 2 + 35 + recovery_id
    let recovery: i32 = recovery_id.into();
    let v = chain_id * 2 + 35 + recovery as u64;

    Ok((v, r, s))
}
