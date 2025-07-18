//! Builder patterns for API requests
//!
//! This module provides builder patterns for constructing complex API requests
//! with optional parameters. Builders ensure that requests are constructed
//! correctly and provide a fluent interface for better developer experience.
//!
//! # Design Philosophy
//!
//! Builders provide:
//! - Type-safe construction of requests
//! - Optional parameter handling
//! - Validation of required fields
//! - Fluent API for chaining method calls

use crate::error::Result;
use crate::types::address::{GetPreviousAddressesRequest, GetReceiveAddressRequest};
use crate::types::common::Pointer;

/// Builder trait for constructing API requests
///
/// This trait provides a common interface for all request builders,
/// ensuring consistent behavior across different request types.
pub trait RequestBuilder {
    /// The type of request this builder produces
    type Request;

    /// Build the request
    ///
    /// # Errors
    ///
    /// Returns an error if the builder's state is invalid
    fn build(self) -> Result<Self::Request>;
}

/// Builder for `GetReceiveAddressRequest`
///
/// Constructs requests for retrieving receive addresses with optional
/// parameters for subaccount and address type filtering.
///
/// # Example
///
/// ```no_run
/// use green_rs::api::builder::GetReceiveAddressBuilder;
///
/// let request = GetReceiveAddressBuilder::new()
///     .subaccount(0)
///     .address_type("p2wpkh".to_string())
///     .build();
/// ```
pub struct GetReceiveAddressBuilder {
    subaccount: Option<u32>,
    address_type: Option<String>,
}

impl Default for GetReceiveAddressBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl GetReceiveAddressBuilder {
    /// Create a new builder instance
    #[must_use]
    pub const fn new() -> Self {
        Self {
            subaccount: None,
            address_type: None,
        }
    }

    /// Set the subaccount to retrieve addresses from
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // Cannot be const due to Option::Some
    pub fn subaccount(mut self, subaccount: u32) -> Self {
        self.subaccount = Some(subaccount);
        self
    }

    /// Set the address type (e.g., "p2wpkh", "p2sh-p2wpkh")
    #[must_use]
    pub fn address_type(mut self, address_type: String) -> Self {
        self.address_type = Some(address_type);
        self
    }

    /// Build the final request
    #[must_use]
    pub fn build(self) -> GetReceiveAddressRequest {
        GetReceiveAddressRequest {
            subaccount: self.subaccount,
            address_type: self.address_type,
        }
    }
}

/// Builder for `GetPreviousAddressesRequest`
///
/// Constructs requests for retrieving previously generated addresses
/// with filtering options for subaccount, pointer range, and usage status.
///
/// # Example
///
/// ```no_run
/// use green_rs::api::builder::GetPreviousAddressesBuilder;
///
/// let request = GetPreviousAddressesBuilder::new()
///     .subaccount(0)
///     .unused_only(true)
///     .build();
/// ```
pub struct GetPreviousAddressesBuilder {
    subaccount: Option<u32>,
    last_pointer: Option<Pointer>,
    unused_only: Option<bool>,
}

impl Default for GetPreviousAddressesBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl GetPreviousAddressesBuilder {
    /// Create a new builder instance
    #[must_use]
    pub const fn new() -> Self {
        Self {
            subaccount: None,
            last_pointer: None,
            unused_only: None,
        }
    }

    /// Set the subaccount to retrieve addresses from
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // Cannot be const due to Option::Some
    pub fn subaccount(mut self, subaccount: u32) -> Self {
        self.subaccount = Some(subaccount);
        self
    }

    /// Set the last pointer to retrieve addresses up to
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // Cannot be const due to Option::Some
    pub fn last_pointer(mut self, last_pointer: Pointer) -> Self {
        self.last_pointer = Some(last_pointer);
        self
    }

    /// Set whether to retrieve only unused addresses
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // Cannot be const due to Option::Some
    pub fn unused_only(mut self, unused_only: bool) -> Self {
        self.unused_only = Some(unused_only);
        self
    }

    /// Build the final request
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // Cannot be const due to struct construction
    pub fn build(self) -> GetPreviousAddressesRequest {
        GetPreviousAddressesRequest {
            subaccount: self.subaccount,
            last_pointer: self.last_pointer,
            unused_only: self.unused_only,
        }
    }
}
// Examples:
// - TransactionBuilder
// - WalletBuilder
// - etc.
