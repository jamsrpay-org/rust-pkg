use serde::{Deserialize, Serialize};

// ─── Create Order ───────────────────────────────────────────────────

/// Parameters for placing an energy rental order.
///
/// Corresponds to `POST /api/v1/frontend/order`.
#[derive(Debug, Clone, Serialize)]
pub struct CreateOrderParams<'a> {
    /// Amount of energy to delegate (min 10,000).
    pub energy_amount: u32,
    /// Rental period: `"1H"`, `"1D"`, `"3D"`, or `"30D"`.
    pub period: &'a str,
    /// TRON address to receive the delegated energy (must be activated).
    pub receive_address: &'a str,
    /// Optional webhook URL for delegation-success notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<&'a str>,
    /// Optional external order ID returned in callbacks for reconciliation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<&'a str>,
}

/// Successful response from creating an order.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateOrderResponse {
    /// Internal order serial number.
    pub serial: String,
    /// Order cost in sun (÷ 1,000,000 = TRX).
    pub amount: i64,
    /// Remaining account balance in sun.
    pub balance: i64,
}

// ─── Estimate Energy ────────────────────────────────────────────────

/// Parameters for estimating energy cost.
///
/// At least one of `energy_amount` or `to_address` should be provided.
/// If only `to_address` is given, the API auto-estimates USDT transfer energy.
#[derive(Debug, Clone, Default)]
pub struct EstimateEnergyParams<'a> {
    /// Rental period: `"1H"`, `"1D"`, `"3D"`, or `"30D"`.
    pub period: &'a str,
    /// Specific energy amount to estimate (min 10,000). Optional.
    pub energy_amount: Option<u32>,
    /// Target address — the API will auto-estimate USDT transfer energy.
    pub to_address: Option<&'a str>,
}

/// Response from the energy price estimation endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct EstimateEnergyResponse {
    /// The energy amount for the order.
    pub energy_amount: i64,
    /// Rental period string.
    pub period: String,
    /// Unit price in sun.
    pub price: i64,
    /// Total TRX to be paid in sun.
    pub total_price: i64,
    /// Small-order handling fee in sun (applies when energy < 50,000).
    #[serde(default)]
    pub addition: i64,
}

// ─── Balance / Public Data ──────────────────────────────────────────

/// A price tier for a specific rental period.
#[derive(Debug, Clone, Deserialize)]
pub struct PriceTier {
    /// Period code: 0 = 1 hour, 1 = 1 day, 3 = 3 days, 30 = 30 days.
    pub period: i32,
    /// Unit price in sun.
    pub price: i64,
}

/// Account balance and platform metadata.
///
/// Returned by `GET /api/v1/frontend/index-data`.
#[derive(Debug, Clone, Deserialize)]
pub struct BalanceResponse {
    /// Total energy available on the platform.
    pub platform_avail_energy: i64,
    /// Maximum energy for a single unsplit order.
    pub platform_max_energy: i64,
    /// Minimum order energy.
    pub minimum_order_energy: i64,
    /// Maximum order energy.
    pub maximum_order_energy: i64,
    /// Orders below this threshold incur a small-order fee.
    pub small_amount: i64,
    /// Small-order handling fee in TRX.
    pub small_addition: f64,
    /// Energy required for a USDT transfer to an existing address.
    pub usdt_energy_need_old: i64,
    /// Energy required for a USDT transfer to a new address.
    pub usdt_energy_need_new: i64,
    /// Pricing tiers per rental period.
    pub tiered_pricing: Vec<PriceTier>,
    /// Account balance in sun (÷ 1,000,000 = TRX).
    pub balance: i64,
}

// ─── Internal: API envelope ─────────────────────────────────────────

/// Wrapper for responses that include `errno` / `message` alongside data.
#[derive(Debug, Deserialize)]
pub struct ApiEnvelope<T> {
    pub errno: i64,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(flatten)]
    pub data: Option<T>,
}

