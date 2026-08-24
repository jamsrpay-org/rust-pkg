use crate::error::SolanaClientError;
use chain_core::error::BlockchainClientError;
use curve25519_dalek::edwards::CompressedEdwardsY;
use sha2::{Digest, Sha256};

use super::transaction::{CompiledInstruction, pubkey_from_base58, serialize_message};

/// SPL Token Program ID.
const TOKEN_PROGRAM: &str = "TokenkegQEqKXcsBR3MgFiQn4c5oSp3xMaKNvpqGMvN";

/// Associated Token Program ID.
const ASSOCIATED_TOKEN_PROGRAM: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";

/// Check whether 32 bytes represent a point on the Ed25519 curve.
fn is_on_curve(bytes: &[u8; 32]) -> bool {
    CompressedEdwardsY(*bytes).decompress().is_some()
}

/// Find a Program Derived Address (PDA) for the given seeds and program ID.
///
/// Returns the derived address and bump seed, or `None` if no valid PDA was found
/// (extremely unlikely — would require all 256 bump values to land on the curve).
fn find_program_address(seeds: &[&[u8]], program_id: &[u8; 32]) -> Option<([u8; 32], u8)> {
    for bump in (0..=u8::MAX).rev() {
        let mut hasher = Sha256::new();
        for seed in seeds {
            hasher.update(seed);
        }
        hasher.update([bump]);
        hasher.update(program_id);
        hasher.update(b"ProgramDerivedAddress");
        let hash = hasher.finalize();
        let candidate: [u8; 32] = hash.into();

        if !is_on_curve(&candidate) {
            return Some((candidate, bump));
        }
    }
    None
}

/// Derive the Associated Token Account (ATA) address for a given owner and mint.
pub(crate) fn find_associated_token_address(
    owner: &[u8; 32],
    mint: &[u8; 32],
) -> Result<[u8; 32], SolanaClientError> {
    let token_program = pubkey_from_base58(TOKEN_PROGRAM)
        .map_err(|e| SolanaClientError::TransactionError(e.to_string()))?;
    let ata_program = pubkey_from_base58(ASSOCIATED_TOKEN_PROGRAM)
        .map_err(|e| SolanaClientError::TransactionError(e.to_string()))?;

    let seeds: &[&[u8]] = &[owner.as_ref(), token_program.as_ref(), mint.as_ref()];

    find_program_address(seeds, &ata_program)
        .map(|(addr, _)| addr)
        .ok_or_else(|| {
            SolanaClientError::TransactionError(
                "could not derive associated token address".to_string(),
            )
        })
}

/// Build a serialized message for an SPL token transfer.
///
/// Includes a `CreateIdempotent` instruction to ensure the recipient's
/// Associated Token Account exists (no-op if it already does), followed
/// by a `Transfer` instruction.
///
/// Account layout:
/// - `[0]` payer / authority (signer, writable)
/// - `[1]` source ATA (writable)
/// - `[2]` destination ATA (writable)
/// - `[3]` recipient owner (readonly)
/// - `[4]` mint (readonly)
/// - `[5]` System Program (readonly)
/// - `[6]` Token Program (readonly)
/// - `[7]` Associated Token Program (readonly)
pub(crate) fn build_spl_transfer_message(
    from: &[u8; 32],
    to: &[u8; 32],
    mint: &[u8; 32],
    amount: u64,
    recent_blockhash: &[u8; 32],
) -> Result<Vec<u8>, BlockchainClientError> {
    let token_program = pubkey_from_base58(TOKEN_PROGRAM)?;
    let ata_program = pubkey_from_base58(ASSOCIATED_TOKEN_PROGRAM)?;
    let system_program = [0u8; 32];

    let source_ata = find_associated_token_address(from, mint)
        .map_err(|e| BlockchainClientError::Unknown(e.to_string()))?;
    let dest_ata = find_associated_token_address(to, mint)
        .map_err(|e| BlockchainClientError::Unknown(e.to_string()))?;

    // Instruction 1: CreateIdempotent (Associated Token Program)
    // Creates the recipient's ATA if it doesn't exist (no-op if it does).
    let create_ata_ix = CompiledInstruction {
        program_id_index: 7,
        accounts: vec![0, 2, 3, 4, 5, 6], // payer, dest_ata, recipient, mint, system, token
        data: vec![1],                    // CreateIdempotent instruction index
    };

    // Instruction 2: Transfer (Token Program)
    let mut transfer_data = Vec::with_capacity(9);
    transfer_data.push(3); // Transfer instruction index
    transfer_data.extend_from_slice(&amount.to_le_bytes());

    let transfer_ix = CompiledInstruction {
        program_id_index: 6,
        accounts: vec![1, 2, 0], // source_ata, dest_ata, authority
        data: transfer_data,
    };

    let message = serialize_message(
        1, // 1 signer (payer / authority)
        0, // 0 readonly signed
        5, // 5 readonly unsigned: recipient, mint, system, token, ata_program
        &[
            *from,          // [0] payer / authority
            source_ata,     // [1] source ATA
            dest_ata,       // [2] destination ATA
            *to,            // [3] recipient owner
            *mint,          // [4] mint
            system_program, // [5] System Program
            token_program,  // [6] Token Program
            ata_program,    // [7] Associated Token Program
        ],
        recent_blockhash,
        &[create_ata_ix, transfer_ix],
    );

    Ok(message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_on_curve() {
        // The identity point (all zeros) IS on the Ed25519 curve
        assert!(is_on_curve(&[0u8; 32]));

        // A known PDA (result of find_associated_token_address) should NOT be on the curve
        let owner = pubkey_from_base58("11111111111111111111111111111111").unwrap();
        let mint =
            pubkey_from_base58("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB").unwrap();
        let ata = find_associated_token_address(&owner, &mint).unwrap();
        assert!(!is_on_curve(&ata));
    }

    #[test]
    fn test_find_associated_token_address() {
        // Test with known values — using the USDT mint and a test wallet
        let owner = pubkey_from_base58("11111111111111111111111111111111").unwrap();
        let mint = pubkey_from_base58("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB").unwrap();

        let result = find_associated_token_address(&owner, &mint);
        assert!(result.is_ok());

        let ata = result.unwrap();
        // ATA should be a valid 32-byte address
        assert_eq!(ata.len(), 32);
        // ATA should NOT be on the curve (it's a PDA)
        assert!(!is_on_curve(&ata));
    }

    #[test]
    fn test_build_spl_transfer_message() {
        let from = [1u8; 32];
        let to = [2u8; 32];
        let mint = [3u8; 32];
        let blockhash = [4u8; 32];
        let amount = 1_000_000u64;

        let result = build_spl_transfer_message(&from, &to, &mint, amount, &blockhash);
        assert!(result.is_ok());

        let msg = result.unwrap();
        // Verify header
        assert_eq!(msg[0], 1); // num_required_signatures
        assert_eq!(msg[1], 0); // num_readonly_signed
        assert_eq!(msg[2], 5); // num_readonly_unsigned

        // 8 account keys
        assert_eq!(msg[3], 8);
    }
}
