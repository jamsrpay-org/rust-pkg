use crate::types::BscWallet;
use secp256k1::Secp256k1;
use sha3::{Digest, Keccak256};

impl BscWallet {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut secp256k1::rand::rng());
        let public_key_bytes = &public_key.serialize_uncompressed()[1..];

        let mut hasher = Keccak256::new();
        hasher.update(public_key_bytes);
        let hashed_public_key = hasher.finalize();

        // EVM address: last 20 bytes of the Keccak256 hash
        let address_bytes = &hashed_public_key[12..];
        let address = format!("0x{}", hex::encode(address_bytes));
        let private_key = hex::encode(secret_key.secret_bytes());
        let public_key_hex = hex::encode(public_key.serialize_uncompressed());

        Self {
            private_key,
            public_key: public_key_hex,
            address,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let wallet = BscWallet::new();
        dbg!(&wallet);
        assert!(!wallet.private_key.is_empty());
        assert!(!wallet.public_key.is_empty());
        assert!(wallet.address.starts_with("0x"));
        assert_eq!(wallet.address.len(), 42);
    }
}
