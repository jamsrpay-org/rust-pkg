use crate::error::TronClientError;
use secp256k1::{Message, Secp256k1, SecretKey, ecdsa::RecoverableSignature};
use sha2::{Digest, Sha256};

pub fn ec_key_sign(raw_tx: &[u8], private_key: &[u8]) -> Result<Vec<u8>, TronClientError> {
    // SHA256 hash → txID
    let tx_hash = Sha256::digest(raw_tx);

    // Sign with secp256k1
    let secp = Secp256k1::new();
    let secret_array: [u8; 32] = private_key.try_into().map_err(|e| {
        eprintln!("Invalid private key: {}", e);
        TronClientError::InvalidPrivateKey
    })?;
    let secret_key = SecretKey::from_byte_array(secret_array)
        .map_err(|e| TronClientError::SignError(e.to_string()))?;
    let message = Message::from_digest(tx_hash.into());

    let signature: RecoverableSignature = secp.sign_ecdsa_recoverable(message, &secret_key);
    let (recovery_id, sig_bytes) = signature.serialize_compact();

    // Build 65-byte signature: r (32) || s (32) || v (1)
    let mut sig_65 = Vec::with_capacity(65);
    sig_65.extend_from_slice(&sig_bytes);
    let v: i32 = recovery_id.into();
    sig_65.push(v as u8);
    Ok(sig_65)
}
