use crate::error::SolanaClientError;
use ed25519_dalek::{Signer, SigningKey};

/// Sign a Solana transaction message with an Ed25519 private key.
///
/// - `message`: serialized transaction message bytes.
/// - `private_key`: 32-byte Ed25519 seed.
///
/// Returns a 64-byte Ed25519 signature.
pub fn ed25519_sign(message: &[u8], private_key: &[u8]) -> Result<Vec<u8>, SolanaClientError> {
    let seed: [u8; 32] = private_key
        .try_into()
        .map_err(|_| SolanaClientError::InvalidPrivateKey)?;
    let signing_key = SigningKey::from_bytes(&seed);
    let signature = signing_key.sign(message);
    Ok(signature.to_bytes().to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_and_verify() {
        let seed = [42u8; 32];
        let message = b"test message";

        let sig_bytes = ed25519_sign(message, &seed).unwrap();
        assert_eq!(sig_bytes.len(), 64);

        // Verify the signature
        let signing_key = SigningKey::from_bytes(&seed);
        let verifying_key = signing_key.verifying_key();
        let sig_array: [u8; 64] = sig_bytes.as_slice().try_into().unwrap();
        let signature = ed25519_dalek::Signature::from_bytes(&sig_array);
        assert!(verifying_key.verify_strict(message, &signature).is_ok());
    }

    #[test]
    fn test_sign_rejects_invalid_key() {
        let short_key = vec![0u8; 16]; // too short
        let result = ed25519_sign(b"test", &short_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_sign_deterministic() {
        let seed = [7u8; 32];
        let message = b"deterministic check";

        let sig1 = ed25519_sign(message, &seed).unwrap();
        let sig2 = ed25519_sign(message, &seed).unwrap();
        assert_eq!(sig1, sig2, "Ed25519 signatures should be deterministic");
    }
}
