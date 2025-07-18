//! Green API client library for Rust
//!
//! This library provides synchronous and asynchronous clients for interacting
//! with the Green API.

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
