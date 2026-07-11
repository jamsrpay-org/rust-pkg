use crate::client::BscClient;
use chain_core::error::BlockchainClientError;
use jamsrpay_types::{currency::PaymentCurrency, money::Money};
use chain_core::types::EstimateWithdrawableRequest;

/// Default gas limit for a native BNB transfer.
const NATIVE_TRANSFER_GAS: u64 = 21_000;

impl BscClient {
    /// Estimate the maximum BNB that can be sent after accounting for gas fees.
    pub(crate) async fn estimate_bnb_withdrawable(
        &self,
        request: &EstimateWithdrawableRequest,
        decimals: u8,
    ) -> Result<Money, BlockchainClientError> {
        let balance = self.eth_get_balance(request.from.as_str()).await?;
        if balance == 0 {
            return Ok(Money::zero(decimals));
        }

        let gas_price = self.eth_gas_price().await?;
        let gas_cost = gas_price * NATIVE_TRANSFER_GAS as u128;

        Ok(Money::from_atomic(
            balance.saturating_sub(gas_cost) as i128,
            decimals,
        ))
    }

    /// For BEP20 tokens, the full balance is withdrawable.
    /// Gas fees are paid in BNB, not in the token itself.
    pub(crate) async fn estimate_bep20_withdrawable(
        &self,
        request: &EstimateWithdrawableRequest,
        currency: PaymentCurrency,
        decimals: u8,
    ) -> Result<Money, BlockchainClientError> {
        let contract = self.contract_address(currency)?;
        let raw = self
            .get_bep20_balance(request.from.as_str(), contract)
            .await?;
        Ok(Money::from_atomic(raw as i128, decimals))
    }
}
