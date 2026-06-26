/// Errors returned by the ITRX SDK.
#[derive(Debug, thiserror::Error)]
pub enum ItrxError {
    /// HTTP transport error (network, DNS, TLS, timeout, etc.).
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    /// Failed to construct a valid HTTP header value.
    #[error("invalid header value: {0}")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),

    /// JSON serialization or deserialization failed.
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    /// HMAC key creation failed (should never happen with valid UTF-8 secrets).
    #[error("hmac error: {0}")]
    Hmac(String),

    /// API returned HTTP 400 with a `detail` message.
    #[error("bad request: {0}")]
    BadRequest(String),

    /// API returned HTTP 200 but `errno > 0`, indicating a logical error.
    #[error("api error (errno={errno}): {message}")]
    ApiError {
        /// Error number from the API (1 = error, 0 = ok).
        errno: i64,
        /// Human-readable error description.
        message: String,
    },

    /// The API returned an unexpected HTTP status code.
    #[error("unexpected status {status}: {body}")]
    UnexpectedStatus {
        /// HTTP status code.
        status: u16,
        /// Response body text.
        body: String,
    },
}
