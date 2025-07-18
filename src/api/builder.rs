//! Builder patterns for API requests

use crate::error::Result;
use crate::types::address::{GetPreviousAddressesRequest, GetReceiveAddressRequest};
use crate::types::common::Pointer;

/// Builder trait for constructing API requests
pub trait RequestBuilder {
    type Request;

    /// Build the request
    fn build(self) -> Result<Self::Request>;
}

/// Builder for GetReceiveAddressRequest
pub struct GetReceiveAddressBuilder {
    subaccount: Option<u32>,
    address_type: Option<String>,
}

impl GetReceiveAddressBuilder {
    pub fn new() -> Self {
        Self { subaccount: None, address_type: None }
    }

    pub fn subaccount(mut self, subaccount: u32) -> Self {
        self.subaccount = Some(subaccount);
        self
    }

    pub fn address_type(mut self, address_type: String) -> Self {
        self.address_type = Some(address_type);
        self
    }

    pub fn build(self) -> GetReceiveAddressRequest {
        GetReceiveAddressRequest {
            subaccount: self.subaccount,
            address_type: self.address_type,
        }
    }
}

/// Builder for GetPreviousAddressesRequest
pub struct GetPreviousAddressesBuilder {
    subaccount: Option<u32>,
    last_pointer: Option<Pointer>,
    unused_only: Option<bool>,
}

impl GetPreviousAddressesBuilder {
    pub fn new() -> Self {
        Self {
            subaccount: None,
            last_pointer: None,
            unused_only: None,
        }
    }

    pub fn subaccount(mut self, subaccount: u32) -> Self {
        self.subaccount = Some(subaccount);
        self
    }

    pub fn last_pointer(mut self, last_pointer: Pointer) -> Self {
        self.last_pointer = Some(last_pointer);
        self
    }

    pub fn unused_only(mut self, unused_only: bool) -> Self {
        self.unused_only = Some(unused_only);
        self
    }

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
