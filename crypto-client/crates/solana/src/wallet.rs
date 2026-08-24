use crate::types::SolanaWallet;
use ed25519_dalek::SigningKey;

impl SolanaWallet {
    /// Generate a new random Solana wallet (Ed25519 keypair).
    ///
    /// The address is the base58-encoded public key.
    pub fn new() -> Self {
        let seed: [u8; 32] = rand::random();
        let signing_key = SigningKey::from_bytes(&seed);
        let verifying_key = signing_key.verifying_key();

        let address = bs58::encode(verifying_key.as_bytes()).into_string();
        let private_key = hex::encode(seed);
        let public_key = hex::encode(verifying_key.as_bytes());

        Self {
            private_key,
            public_key,
            address,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_wallet() {
        let wallet = SolanaWallet::new();
        dbg!(&wallet);
        assert!(!wallet.private_key.is_empty());
        assert!(!wallet.public_key.is_empty());
        assert!(!wallet.address.is_empty());
        // Ed25519 seed is 32 bytes → 64 hex chars
        assert_eq!(wallet.private_key.len(), 64);
        // Ed25519 public key is 32 bytes → 64 hex chars
        assert_eq!(wallet.public_key.len(), 64);
        // Base58-encoded 32 bytes → typically 32–44 chars
        assert!(wallet.address.len() >= 32 && wallet.address.len() <= 44);
    }
}
