use crate::standard::TokenStandard;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetNetworkRepresentation {
    /// Native blockchain asset, such as LTC, ETH, TRX or SOL.
    Native,

    /// Token deployed on a blockchain.
    ///
    /// `token_address` means:
    /// - EVM/TRON: token contract address.
    /// - Solana: token mint address.
    Token {
        standard: TokenStandard,
        token_address: &'static str,
    },
}
