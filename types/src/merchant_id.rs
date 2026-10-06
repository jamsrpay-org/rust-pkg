use crate::user_id::UserId;

/// Strongly-typed identifier for a merchant.
///
/// Aliased to [`UserId`] to remain fully compatible with JamsrPay's identity
/// and user models where merchants are authenticated users.
pub type MerchantId = UserId;
