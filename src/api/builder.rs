//! Builder patterns for API requests

use crate::error::Result;

/// Builder trait for constructing API requests
pub trait RequestBuilder {
    type Request;

    /// Build the request
    fn build(self) -> Result<Self::Request>;
}

// TODO: Implement specific builders for different API requests
// Examples:
// - TransactionBuilder
// - WalletBuilder
// - etc.
