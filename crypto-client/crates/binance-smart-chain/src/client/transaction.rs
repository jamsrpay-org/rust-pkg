use crate::client::BscClient;
use crate::error::BscClientError;
use crate::types::BscPreparedTransfer;

impl BscClient {
    /// Build a native BNB transfer transaction.
    ///
    /// Fetches nonce, gas price, and estimates gas, then RLP-encodes
    /// the unsigned transaction for signing.
    pub(crate) async fn create_bnb_transfer(
        &self,
        from: &str,
        to: &str,
        amount: u128,
    ) -> Result<BscPreparedTransfer, BscClientError> {
        let nonce = self.eth_get_transaction_count(from).await?;
        let gas_price = self.eth_gas_price().await?;

        // Native transfer — estimate gas (typically 21000)
        let value_hex = format!("0x{:x}", amount);
        let gas_limit = self
            .eth_estimate_gas(from, to, None, Some(&value_hex))
            .await?;

        let unsigned_tx_bytes =
            rlp_encode_unsigned(nonce, gas_price, gas_limit, to, amount, &[], self.chain_id());

        Ok(BscPreparedTransfer {
            unsigned_tx_bytes,
            nonce,
            gas_price,
            gas_limit,
            to: to.to_string(),
            value: amount,
            data: vec![],
            chain_id: self.chain_id(),
        })
    }

    /// Build a BEP20 token transfer transaction.
    pub(crate) async fn create_bep20_transfer_tx(
        &self,
        from: &str,
        to: &str,
        amount: u128,
        contract_address: &str,
    ) -> Result<BscPreparedTransfer, BscClientError> {
        let nonce = self.eth_get_transaction_count(from).await?;
        let gas_price = self.eth_gas_price().await?;

        // Encode transfer(address, uint256) calldata
        let data = super::abi::encode_transfer(to, amount)?;
        let data_hex = format!("0x{}", hex::encode(&data));

        // Estimate gas for the contract call
        let gas_limit = self
            .eth_estimate_gas(from, contract_address, Some(&data_hex), None)
            .await?;

        let unsigned_tx_bytes = rlp_encode_unsigned(
            nonce,
            gas_price,
            gas_limit,
            contract_address,
            0, // value is 0 for token transfers
            &data,
            self.chain_id(),
        );

        Ok(BscPreparedTransfer {
            unsigned_tx_bytes,
            nonce,
            gas_price,
            gas_limit,
            to: contract_address.to_string(),
            value: 0,
            data,
            chain_id: self.chain_id(),
        })
    }
}

// ── RLP encoding ────────────────────────────────────────────────────────────

/// RLP-encode an unsigned EIP-155 transaction: [nonce, gasPrice, gasLimit, to, value, data, chainId, 0, 0]
pub(crate) fn rlp_encode_unsigned(
    nonce: u64,
    gas_price: u128,
    gas_limit: u64,
    to: &str,
    value: u128,
    data: &[u8],
    chain_id: u64,
) -> Vec<u8> {
    let mut stream = rlp::RlpStream::new_list(9);
    stream.append(&nonce);
    stream.append(&gas_price);
    stream.append(&(gas_limit as u128));

    // `to` address as raw 20-byte value
    let to_stripped = to.strip_prefix("0x").unwrap_or(to);
    let to_bytes = hex::decode(to_stripped).unwrap_or_default();
    stream.append(&to_bytes);

    stream.append(&value);
    stream.append(&data);
    // EIP-155 fields
    stream.append(&chain_id);
    stream.append(&0u8);
    stream.append(&0u8);

    stream.out().to_vec()
}

/// RLP-encode a signed transaction: [nonce, gasPrice, gasLimit, to, value, data, v, r, s]
pub(crate) fn rlp_encode_signed(
    nonce: u64,
    gas_price: u128,
    gas_limit: u64,
    to: &str,
    value: u128,
    data: &[u8],
    v: u64,
    r: &[u8],
    s: &[u8],
) -> Vec<u8> {
    let mut stream = rlp::RlpStream::new_list(9);
    stream.append(&nonce);
    stream.append(&gas_price);
    stream.append(&(gas_limit as u128));

    let to_stripped = to.strip_prefix("0x").unwrap_or(to);
    let to_bytes = hex::decode(to_stripped).unwrap_or_default();
    stream.append(&to_bytes);

    stream.append(&value);
    stream.append(&data);
    stream.append(&v);

    // r and s must be encoded as big-endian integers (strip leading zeros)
    let r_trimmed = trim_leading_zeros(r);
    let s_trimmed = trim_leading_zeros(s);
    stream.append(&r_trimmed);
    stream.append(&s_trimmed);

    stream.out().to_vec()
}

/// Strip leading zero bytes from a byte slice (for RLP integer encoding).
fn trim_leading_zeros(bytes: &[u8]) -> &[u8] {
    let start = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len());
    &bytes[start..]
}
