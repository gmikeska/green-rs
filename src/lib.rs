//! Green API client library for Rust
//!
//! This library provides synchronous and asynchronous clients for interacting
//! with the Green API through the `green-cli` command-line interface.
//!
//! # Overview
//!
//! The library wraps the Green Liquid wallet CLI, providing a type-safe Rust interface
//! for wallet operations, transaction management, and UTXO handling.
//!
//! # Liquid Network Caveats
//!
//! When working with Liquid assets:
//! - Asset IDs are required for all non-Bitcoin assets
//! - Confidential transactions hide amounts and asset types
//! - Blinding factors are needed to reveal transaction details
//! - Fee calculations differ from Bitcoin mainnet
//!
//! # Examples
//!
//! ## Synchronous Client
//! ```no_run
//! use green_rs::{GreenClient, api::WalletExt};
//!
//! let client = GreenClient::new();
//! let balance = client.get_balance().expect("Failed to get balance");
//! ```
//!
//! ## Asynchronous Client
//! ```no_run
//! use green_rs::{AsyncGreenClient, api::AsyncWalletExt};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = AsyncGreenClient::new();
//!     let balance = client.get_balance().await.expect("Failed to get balance");
//! }
//! ```

#![deny(missing_docs)]

// Re-export main client types
pub use client::{AsyncGreenClient, GreenClient};

// Re-export error types
pub use error::{Error, Result};

// Re-export types module
pub mod types;

// Re-export API modules
pub mod api;

// Public modules
pub mod client;

// Internal modules
mod error;
