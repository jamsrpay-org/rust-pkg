//! # ITRX SDK
//!
//! A Rust client for the [ITRX](https://itrx.io) Tron energy rental API.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use itrx::{ItrxClient, ItrxConfig};
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = ItrxConfig::mainnet("API_KEY", "API_SECRET");
//!     let client = ItrxClient::new(config);
//!
//!     let balance = client.get_balance().await.unwrap();
//!     println!("Balance: {} sun", balance.balance);
//! }
//! ```

pub mod client;
pub mod error;
pub mod types;

pub use client::{ItrxClient, ItrxConfig};
pub use error::ItrxError;
pub use types::*;
