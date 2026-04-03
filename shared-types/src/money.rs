use rust_decimal::{Decimal, prelude::ToPrimitive};
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    ops::{Add, AddAssign, Sub, SubAssign},
};

/// Type-safe wrapper for monetary amounts stored in **atomic units** (smallest indivisible unit).
///
/// Each `Money` value is self-describing — it carries the number of decimal places
/// so it can always convert between atomic and human-readable representations
/// without external context.
///
/// The `atomic` field is an `i128` — enforcing that atomic amounts are always integers.
/// This prevents accidental fractional atomic values (e.g. 1.5 satoshis) which are nonsensical.
///
/// Examples of atomic units:
/// - BTC: satoshis (1 BTC = 100_000_000 satoshis, decimals = 8)
/// - ETH: wei (1 ETH = 10^18 wei, decimals = 18)
/// - USDT: micro-dollars (1 USDT = 1_000_000, decimals = 6)
/// - USD: cents (1 USD = 100 cents, decimals = 2)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Money {
    /// The amount in the smallest indivisible unit (always an integer).
    atomic: i128,
    /// Number of decimal places for this currency/token
    decimals: u8,
}

impl Money {
    /// Zero amount with 0 decimals
    pub const ZERO: Self = Self {
        atomic: 0,
        decimals: 0,
    };

    /// Create a zero amount with the given decimal places.
    pub fn zero(decimals: u8) -> Self {
        Self {
            atomic: 0,
            decimals,
        }
    }

    /// Create Money directly from an integer atomic value and its decimal places.
    ///
    /// Example: `Money::from_atomic(150_000_000, 8)` → 1.5 BTC in satoshis
    pub fn from_atomic(atomic: i128, decimals: u8) -> Self {
        Self { atomic, decimals }
    }

    /// Create Money from a DB `Decimal` value that represents an atomic (integer) amount.
    ///
    /// This is intended for use at the persistence boundary where the DB stores
    /// atomic amounts as `Decimal(78, 0)`.
    ///
    /// # Panics
    /// Panics if the Decimal value cannot be represented as `i128` (e.g. has fractional part).
    pub fn from_atomic_decimal(value: Decimal, decimals: u8) -> Self {
        let atomic = value
            .to_i128()
            .expect("Decimal atomic value must be an integer representable as i128");
        Self { atomic, decimals }
    }

    /// Create Money from a human-readable amount by converting to atomic units.
    ///
    /// Example: `Money::from_human_readable(dec!(1.5), 8)` → 150_000_000 satoshis
    pub fn from_human_readable(amount: Decimal, decimals: u8) -> Self {
        let multiplier = Decimal::from_i128_with_scale(10i128.pow(decimals as u32), 0);
        let atomic_decimal = amount * multiplier;
        let atomic = atomic_decimal
            .to_i128()
            .expect("Human-readable amount overflows i128 when converted to atomic units");
        Self { atomic, decimals }
    }

    /// Get the atomic value (smallest unit) as an integer.
    pub fn atomic(&self) -> i128 {
        self.atomic
    }

    /// Get the number of decimal places.
    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    /// Convert the atomic value to a `Decimal`.
    ///
    /// Useful at the persistence boundary where DB columns use `Decimal`.
    pub fn to_atomic_decimal(&self) -> Decimal {
        Decimal::from_i128_with_scale(self.atomic, 0)
    }

    /// Returns the atomic value as a string with no decimal point.
    ///
    /// Example: `Money::from_atomic(150_000_000, 8).to_atomic_string()` → `"150000000"`
    pub fn to_atomic_string(&self) -> String {
        self.atomic.to_string()
    }

    /// Converts atomic units to a human-readable formatted string.
    ///
    /// Example: `Money::from_atomic(150_000_000, 8).to_formatted()` → `"1.5"`
    pub fn to_formatted(&self) -> String {
        let divisor = Decimal::from_i128_with_scale(10i128.pow(self.decimals as u32), 0);
        let atomic_decimal = Decimal::from_i128_with_scale(self.atomic, 0);
        let human = (atomic_decimal / divisor).normalize();
        human.to_string()
    }

    /// Check if the amount is zero.
    pub fn is_zero(&self) -> bool {
        self.atomic == 0
    }

    /// Check if the amount is positive.
    pub fn is_positive(&self) -> bool {
        self.atomic > 0
    }

    /// Check if the amount is negative.
    pub fn is_negative(&self) -> bool {
        self.atomic < 0
    }
}

impl Default for Money {
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_formatted())
    }
}

/// At the persistence boundary, convert `Money` to a `Decimal` (atomic value).
impl From<Money> for Decimal {
    fn from(money: Money) -> Self {
        money.to_atomic_decimal()
    }
}

impl Add for Money {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        debug_assert_eq!(
            self.decimals, rhs.decimals,
            "Cannot add Money with different decimals"
        );
        Self {
            atomic: self.atomic + rhs.atomic,
            decimals: self.decimals,
        }
    }
}

impl AddAssign for Money {
    fn add_assign(&mut self, rhs: Self) {
        debug_assert_eq!(
            self.decimals, rhs.decimals,
            "Cannot add Money with different decimals"
        );
        self.atomic += rhs.atomic;
    }
}

impl Sub for Money {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        debug_assert_eq!(
            self.decimals, rhs.decimals,
            "Cannot subtract Money with different decimals"
        );
        Self {
            atomic: self.atomic - rhs.atomic,
            decimals: self.decimals,
        }
    }
}

impl SubAssign for Money {
    fn sub_assign(&mut self, rhs: Self) {
        debug_assert_eq!(
            self.decimals, rhs.decimals,
            "Cannot subtract Money with different decimals"
        );
        self.atomic -= rhs.atomic;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_zero() {
        let zero = Money::zero(8);
        assert!(zero.is_zero());
        assert_eq!(zero.decimals(), 8);
        assert_eq!(zero.to_atomic_string(), "0");
    }

    #[test]
    fn test_from_atomic() {
        let money = Money::from_atomic(150_000_000, 8);
        assert_eq!(money.atomic(), 150_000_000);
        assert_eq!(money.decimals(), 8);
    }

    #[test]
    fn test_from_atomic_decimal() {
        let money = Money::from_atomic_decimal(dec!(150000000), 8);
        assert_eq!(money.atomic(), 150_000_000);
        assert_eq!(money.decimals(), 8);
    }

    #[test]
    fn test_from_human_readable_btc() {
        // 1.5 BTC = 150_000_000 satoshis
        let money = Money::from_human_readable(dec!(1.5), 8);
        assert_eq!(money.atomic(), 150_000_000);
        assert_eq!(money.decimals(), 8);
    }

    #[test]
    fn test_from_human_readable_usdt() {
        // 100 USDT = 100_000_000 (6 decimals)
        let money = Money::from_human_readable(dec!(100), 6);
        assert_eq!(money.atomic(), 100_000_000);
    }

    #[test]
    fn test_from_human_readable_eth() {
        // 0.001 ETH = 1_000_000_000_000_000 wei (18 decimals)
        let money = Money::from_human_readable(dec!(0.001), 18);
        assert_eq!(money.atomic(), 1_000_000_000_000_000);
    }

    #[test]
    fn test_to_atomic_string() {
        let money = Money::from_atomic(150_000_000, 8);
        assert_eq!(money.to_atomic_string(), "150000000");
    }

    #[test]
    fn test_to_formatted_btc() {
        // 150_000_000 satoshis = 1.50000000 BTC
        let money = Money::from_atomic(150_000_000, 8);
        assert_eq!(money.to_formatted(), "1.5");
    }

    #[test]
    fn test_to_formatted_usdt() {
        // 100_000_000 = 100.000000 USDT
        let money = Money::from_atomic(100_000_000, 6);
        assert_eq!(money.to_formatted(), "100");
    }

    #[test]
    fn test_to_formatted_trx() {
        // 1_500_000 = 1.500000 TRX
        let money = Money::from_atomic(1_500_000, 6);
        assert_eq!(money.to_formatted(), "1.5");
    }

    #[test]
    fn test_display() {
        let money = Money::from_atomic(150_000_000, 8);
        assert_eq!(format!("{}", money), "1.5");
    }

    #[test]
    fn test_roundtrip_human_to_atomic_to_formatted() {
        let money = Money::from_human_readable(dec!(42.12345678), 8);
        assert_eq!(money.to_formatted(), "42.12345678");
    }

    #[test]
    fn test_add() {
        let a = Money::from_atomic(100, 6);
        let b = Money::from_atomic(50, 6);
        let result = a + b;
        assert_eq!(result.atomic(), 150);
        assert_eq!(result.decimals(), 6);
    }

    #[test]
    fn test_sub() {
        let a = Money::from_atomic(100, 6);
        let b = Money::from_atomic(30, 6);
        assert_eq!((a - b).atomic(), 70);
    }

    #[test]
    fn test_is_positive() {
        assert!(Money::from_atomic(1, 6).is_positive());
        assert!(!Money::zero(6).is_positive());
    }

    #[test]
    fn test_is_negative() {
        assert!(Money::from_atomic(-1, 6).is_negative());
        assert!(!Money::zero(6).is_negative());
    }

    #[test]
    fn test_comparison() {
        let a = Money::from_atomic(100, 6);
        let b = Money::from_atomic(200, 6);
        assert!(a < b);
        assert!(b > a);
    }

    #[test]
    fn test_to_atomic_decimal() {
        let money = Money::from_atomic(150_000_000, 8);
        assert_eq!(money.to_atomic_decimal(), dec!(150000000));
    }
}
