use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Type-safe wrapper for currency exchange rates.
/// Separates exchange rates from regular amounts for type safety.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ExchangeRate(Decimal);

impl ExchangeRate {
    /// Identity rate (1:1)
    pub const ONE: Self = Self(Decimal::ONE);

    /// Create a new ExchangeRate from a Decimal
    pub fn new(value: Decimal) -> Self {
        Self(value)
    }

    /// Get the inner Decimal value
    pub fn value(&self) -> Decimal {
        self.0
    }

    /// Consume and return the inner Decimal
    pub fn into_inner(self) -> Decimal {
        self.0
    }

    /// Round to the specified number of decimal places
    pub fn round_dp(&self, dp: u32) -> Self {
        Self(self.0.round_dp(dp))
    }

    pub fn normalize(&self) -> Self {
        Self(self.0.normalize())
    }
}

impl Default for ExchangeRate {
    fn default() -> Self {
        Self::ONE
    }
}

impl fmt::Display for ExchangeRate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Decimal> for ExchangeRate {
    fn from(value: Decimal) -> Self {
        Self::new(value)
    }
}

impl From<ExchangeRate> for Decimal {
    fn from(rate: ExchangeRate) -> Self {
        rate.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_exchange_rate_one() {
        assert_eq!(ExchangeRate::ONE.value(), Decimal::ONE);
    }

    #[test]
    fn test_exchange_rate_round_dp() {
        let rate = ExchangeRate::new(dec!(1.23456789));
        assert_eq!(rate.round_dp(4).value(), dec!(1.2346));
    }
}
