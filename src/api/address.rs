//! Address API implementation

use crate::Result;
use crate::types::address::{AddressDetails, GetPreviousAddressesRequest, GetReceiveAddressRequest, ReceiveAddress};

/// Address API trait for Green clients
pub trait AddressApi {
    /// Get an existing receive address
    fn get_receive_address(&self, request: GetReceiveAddressRequest) -> Result<ReceiveAddress>;
    
    /// Generate a new receive address
    fn get_new_address(&self, request: GetReceiveAddressRequest) -> Result<ReceiveAddress>;
    
    /// Retrieve previous addresses
    fn get_previous_addresses(&self, request: GetPreviousAddressesRequest) -> Result<Vec<AddressDetails>>;
    
    // TODO: Implement additional address methods
    // - generate_address
    // - validate_address
    // - get_address_info
    // - list_addresses
}

/// Async Address API trait for Green clients
#[async_trait::async_trait]
pub trait AsyncAddressApi {
    /// Get an existing receive address
    async fn get_receive_address(&self, request: GetReceiveAddressRequest) -> Result<ReceiveAddress>;
    
    /// Generate a new receive address
    async fn get_new_address(&self, request: GetReceiveAddressRequest) -> Result<ReceiveAddress>;
    
    /// Retrieve previous addresses
    async fn get_previous_addresses(&self, request: GetPreviousAddressesRequest) -> Result<Vec<AddressDetails>>;
    
    // TODO: Implement additional async address methods
}
