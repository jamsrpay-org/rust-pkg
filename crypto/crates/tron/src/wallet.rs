use crate::types::{TronWallet, TronWalletAddress};
use secp256k1::Secp256k1;
use sha3::{Digest, Keccak256};

impl TronWallet {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut rand::rng());
        let public_key_bytes = &public_key.serialize_uncompressed()[1..];

        let mut hasher = Keccak256::new();
        hasher.update(public_key_bytes);
        let hashed_public_key = hasher.finalize();

        let address_bytes = &hashed_public_key[12..];

        let mut tron_address = vec![0x41];
        tron_address.extend_from_slice(address_bytes);

        let address_base58 = bs58::encode(&tron_address).with_check().into_string();
        let address_hex = hex::encode(tron_address);
        let private_key_hex = hex::encode(secret_key.secret_bytes());
        let public_key_hex = hex::encode(public_key.serialize_uncompressed());

        let private_key = private_key_hex;
        let public_key = public_key_hex;
        let address = TronWalletAddress {
            base58: address_base58,
            hex: address_hex,
        };
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
    fn test_new() {
        let wallet = TronWallet::new();
        dbg!(&wallet);
        assert!(!wallet.private_key.is_empty());
        assert!(!wallet.public_key.is_empty());
        assert!(!wallet.address.base58.is_empty());
        assert!(!wallet.address.hex.is_empty());
    }
}
