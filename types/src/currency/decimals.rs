use std::fmt;
use std::ops::Deref;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum DecimalsError {
    #[error("invalid decimals {value}: must be between {min} and {max}")]
    InvalidDecimals { value: u8, min: u8, max: u8 },
}

/// Value Object representing precision (number of decimal places) for a currency or token.
///
/// Follows strict domain invariants:
/// - Immutable value object wrapping `u8`.
/// - Constrains decimals between `MIN` (0) and `MAX` (18).
/// - Implements total ordering, equality, and formatting helpers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Decimals(u8);

impl Decimals {
    pub const MIN: u8 = 0;
    pub const MAX: u8 = 18;

    pub const ZERO: Self = Self(0);
    pub const SIX: Self = Self(6);
    pub const EIGHT: Self = Self(8);
    pub const NINE: Self = Self(9);
    pub const EIGHTEEN: Self = Self(18);

    /// Creates a new `Decimals` value object after validating domain constraints.
    pub const fn new(value: u8) -> Result<Self, DecimalsError> {
        if value > Self::MAX {
            return Err(DecimalsError::InvalidDecimals {
                value,
                min: Self::MIN,
                max: Self::MAX,
            });
        }
        Ok(Self(value))
    }

    /// Constructs a `Decimals` instance from a trusted source, asserting domain constraints.
    #[inline]
    pub const fn from_trusted(value: u8) -> Self {
        assert!(value <= Self::MAX, "decimals exceed maximum allowed");
        Self(value)
    }

    /// Returns the underlying numeric precision as `u8`.
    #[inline]
    pub const fn value(&self) -> u8 {
        self.0
    }

    /// Returns the underlying numeric precision as `u8`.
    #[inline]
    pub const fn as_u8(&self) -> u8 {
        self.0
    }

    /// Returns the underlying numeric precision as `u32`.
    #[inline]
    pub const fn as_u32(&self) -> u32 {
        self.0 as u32
    }
}

impl Default for Decimals {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl Deref for Decimals {
    type Target = u8;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Decimals {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Decimals> for u8 {
    #[inline]
    fn from(decimals: Decimals) -> Self {
        decimals.0
    }
}

impl From<Decimals> for u32 {
    #[inline]
    fn from(decimals: Decimals) -> Self {
        decimals.0 as u32
    }
}

impl From<Decimals> for i32 {
    #[inline]
    fn from(decimals: Decimals) -> Self {
        decimals.0 as i32
    }
}

impl TryFrom<u8> for Decimals {
    type Error = DecimalsError;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<u32> for Decimals {
    type Error = DecimalsError;

    #[inline]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > Self::MAX as u32 {
            return Err(DecimalsError::InvalidDecimals {
                value: value as u8,
                min: Self::MIN,
                max: Self::MAX,
            });
        }
        Self::new(value as u8)
    }
}

macro_rules! impl_partial_eq_unsigned {
    ($($t:ty),*) => {
        $(
            impl PartialEq<$t> for Decimals {
                #[inline]
                fn eq(&self, other: &$t) -> bool {
                    (self.0 as $t) == *other
                }
            }

            impl PartialEq<Decimals> for $t {
                #[inline]
                fn eq(&self, other: &Decimals) -> bool {
                    other == self
                }
            }
        )*
    };
}

macro_rules! impl_partial_eq_signed {
    ($($t:ty),*) => {
        $(
            impl PartialEq<$t> for Decimals {
                #[inline]
                fn eq(&self, other: &$t) -> bool {
                    *other >= 0 && (self.0 as $t) == *other
                }
            }

            impl PartialEq<Decimals> for $t {
                #[inline]
                fn eq(&self, other: &Decimals) -> bool {
                    other == self
                }
            }
        )*
    };
}

impl_partial_eq_unsigned!(u8, u16, u32, u64, usize);
impl_partial_eq_signed!(i8, i16, i32, i64, isize);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_decimals() {
        assert_eq!(Decimals::new(0).unwrap().value(), 0);
        assert_eq!(Decimals::new(6).unwrap().value(), 6);
        assert_eq!(Decimals::new(8).unwrap().value(), 8);
        assert_eq!(Decimals::new(9).unwrap().value(), 9);
        assert_eq!(Decimals::new(18).unwrap().value(), 18);
    }

    #[test]
    fn test_invalid_decimals_exceeds_max() {
        let err = Decimals::new(19).unwrap_err();
        match err {
            DecimalsError::InvalidDecimals { value, min, max } => {
                assert_eq!(value, 19);
                assert_eq!(min, 0);
                assert_eq!(max, 18);
            }
        }
    }

    #[test]
    fn test_from_trusted() {
        assert_eq!(Decimals::from_trusted(18).value(), 18);
    }

    #[test]
    #[should_panic(expected = "decimals exceed maximum allowed")]
    fn test_from_trusted_panics_on_overflow() {
        Decimals::from_trusted(19);
    }

    #[test]
    fn test_equality_with_numbers() {
        let d = Decimals::from_trusted(6);
        assert_eq!(d, 6u8);
        assert_eq!(6u8, d);
        assert_eq!(d, 6u32);
        assert_eq!(6u32, d);
    }

    #[test]
    fn test_deref_and_display() {
        let d = Decimals::from_trusted(8);
        assert_eq!(*d, 8);
        assert_eq!(d.to_string(), "8");
    }
}
