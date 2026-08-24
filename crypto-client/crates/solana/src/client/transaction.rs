use chain_core::error::BlockchainClientError;

/// Decode a base58 string into a 32-byte public key / hash.
pub(crate) fn pubkey_from_base58(s: &str) -> Result<[u8; 32], BlockchainClientError> {
    let bytes = bs58::decode(s)
        .into_vec()
        .map_err(|e| BlockchainClientError::Unknown(format!("invalid base58: {}", e)))?;
    if bytes.len() != 32 {
        return Err(BlockchainClientError::Unknown(format!(
            "expected 32 bytes, got {}",
            bytes.len()
        )));
    }
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&bytes);
    Ok(arr)
}

/// Encode a u16 value using Solana's compact-u16 encoding.
fn encode_compact_u16(buf: &mut Vec<u8>, value: u16) {
    let mut val = value;
    loop {
        let mut elem = (val & 0x7f) as u8;
        val >>= 7;
        if val > 0 {
            elem |= 0x80;
        }
        buf.push(elem);
        if val == 0 {
            break;
        }
    }
}

/// A compiled instruction within a transaction message.
pub(crate) struct CompiledInstruction {
    pub program_id_index: u8,
    pub accounts: Vec<u8>,
    pub data: Vec<u8>,
}

/// Serialize a transaction message into bytes.
///
/// Format (legacy message):
/// - header: `[num_required_signatures, num_readonly_signed, num_readonly_unsigned]`
/// - account_keys: compact-u16 count + 32 bytes each
/// - recent_blockhash: 32 bytes
/// - instructions: compact-u16 count + compiled instructions
pub(crate) fn serialize_message(
    num_required_signatures: u8,
    num_readonly_signed: u8,
    num_readonly_unsigned: u8,
    account_keys: &[[u8; 32]],
    recent_blockhash: &[u8; 32],
    instructions: &[CompiledInstruction],
) -> Vec<u8> {
    let mut buf = Vec::new();

    // Header
    buf.push(num_required_signatures);
    buf.push(num_readonly_signed);
    buf.push(num_readonly_unsigned);

    // Account keys
    encode_compact_u16(&mut buf, account_keys.len() as u16);
    for key in account_keys {
        buf.extend_from_slice(key);
    }

    // Recent blockhash
    buf.extend_from_slice(recent_blockhash);

    // Instructions
    encode_compact_u16(&mut buf, instructions.len() as u16);
    for ix in instructions {
        buf.push(ix.program_id_index);
        encode_compact_u16(&mut buf, ix.accounts.len() as u16);
        buf.extend_from_slice(&ix.accounts);
        encode_compact_u16(&mut buf, ix.data.len() as u16);
        buf.extend_from_slice(&ix.data);
    }

    buf
}

/// Build a serialized message for a native SOL transfer.
///
/// Accounts layout:
/// - `[0]` sender (signer, writable)
/// - `[1]` recipient (writable)
/// - `[2]` System Program (readonly, unsigned)
pub(crate) fn build_sol_transfer_message(
    from: &[u8; 32],
    to: &[u8; 32],
    lamports: u64,
    recent_blockhash: &[u8; 32],
) -> Vec<u8> {
    // System Program: Transfer instruction
    // data = instruction index (u32 LE = 2) + lamports (u64 LE)
    let mut data = Vec::with_capacity(12);
    data.extend_from_slice(&2u32.to_le_bytes());
    data.extend_from_slice(&lamports.to_le_bytes());

    let system_program = [0u8; 32];

    let instruction = CompiledInstruction {
        program_id_index: 2,
        accounts: vec![0, 1],
        data,
    };

    serialize_message(
        1, // 1 signer (sender)
        0, // 0 readonly signed
        1, // 1 readonly unsigned (System Program)
        &[*from, *to, system_program],
        recent_blockhash,
        &[instruction],
    )
}

/// Build a fully serialized signed transaction from message bytes and a signature.
pub(crate) fn build_signed_transaction(message_bytes: &[u8], signature: &[u8; 64]) -> Vec<u8> {
    let mut tx = Vec::new();
    // Number of signatures (compact-u16)
    encode_compact_u16(&mut tx, 1);
    // Signature (64 bytes)
    tx.extend_from_slice(signature);
    // Message
    tx.extend_from_slice(message_bytes);
    tx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact_u16_encoding() {
        // Values < 0x80 → 1 byte
        let mut buf = Vec::new();
        encode_compact_u16(&mut buf, 0);
        assert_eq!(buf, vec![0]);

        buf.clear();
        encode_compact_u16(&mut buf, 1);
        assert_eq!(buf, vec![1]);

        buf.clear();
        encode_compact_u16(&mut buf, 127);
        assert_eq!(buf, vec![127]);

        // Values 0x80..0x3FFF → 2 bytes
        buf.clear();
        encode_compact_u16(&mut buf, 128);
        assert_eq!(buf, vec![0x80, 0x01]);

        buf.clear();
        encode_compact_u16(&mut buf, 256);
        assert_eq!(buf, vec![0x80, 0x02]);
    }

    #[test]
    fn test_sol_transfer_message_structure() {
        let from = [1u8; 32];
        let to = [2u8; 32];
        let blockhash = [3u8; 32];
        let lamports = 1_000_000u64;

        let msg = build_sol_transfer_message(&from, &to, lamports, &blockhash);

        // Header: 3 bytes
        assert_eq!(msg[0], 1); // num_required_signatures
        assert_eq!(msg[1], 0); // num_readonly_signed
        assert_eq!(msg[2], 1); // num_readonly_unsigned

        // Account keys count: 3 (compact-u16 = 1 byte for value 3)
        assert_eq!(msg[3], 3);

        // 3 × 32-byte keys = 96 bytes (offset 4..100)
        assert_eq!(&msg[4..36], &[1u8; 32]); // sender
        assert_eq!(&msg[36..68], &[2u8; 32]); // recipient
        assert_eq!(&msg[68..100], &[0u8; 32]); // System Program

        // Recent blockhash (offset 100..132)
        assert_eq!(&msg[100..132], &[3u8; 32]);

        // Instructions count (offset 132): 1
        assert_eq!(msg[132], 1);

        // Instruction: program_id_index = 2
        assert_eq!(msg[133], 2);

        // Total message length for a SOL transfer:
        // 3 (header) + 1 (count) + 96 (keys) + 32 (blockhash) + 1 (ix count)
        // + 1 (program_id_index) + 1 (accounts count) + 2 (account indices)
        // + 1 (data count) + 12 (instruction data)
        // = 3 + 1 + 96 + 32 + 1 + 1 + 1 + 2 + 1 + 12 = 150
        assert_eq!(msg.len(), 150);
    }

    #[test]
    fn test_pubkey_from_base58() {
        // System program: 32 "1"s → 32 zero bytes
        let result = pubkey_from_base58("11111111111111111111111111111111");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), [0u8; 32]);
    }

    #[test]
    fn test_signed_transaction_structure() {
        let message = vec![0u8; 150];
        let signature = [42u8; 64];

        let tx = build_signed_transaction(&message, &signature);

        // 1 byte (compact-u16 for count 1) + 64 bytes (signature) + 150 bytes (message)
        assert_eq!(tx.len(), 1 + 64 + 150);
        assert_eq!(tx[0], 1); // signature count
        assert_eq!(&tx[1..65], &[42u8; 64]); // signature
        assert_eq!(&tx[65..], &[0u8; 150]); // message
    }
}
