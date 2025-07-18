//! Address API implementation
//!
//! This module provides traits and implementations for address-related operations
//! in the Green client, facilitating address generation and retrieval.
//!
//! # Liquid Network Support
//!
//! Address operations within Liquid involve unique constraints:
//! - Confidential addresses support blinded transactions
//! - Asset support for multisig setups
//! - Address reuse is generally discouraged for privacy

use crate::types::address::{
    AddressDetails, GetPreviousAddressesRequest, GetReceiveAddressRequest, ReceiveAddress,
};
use crate::Result;

/// Address API trait for Green clients
///
/// Provides synchronous access to address operations for retrieval,
/// generation, and management.
///
/// # Liquid Network Considerations
///
/// - Receiving addresses may change following network forks or upgrades
/// - Address generation supports asset-labeled addresses
/// - Misuse of addresses could lead to asset loss
pub trait AddressApi {
    /// Get an existing receive address
    ///
    /// # Errors
    ///
    /// Returns an error if the CLI command fails or returns invalid JSON
    fn get_receive_address(&self, request: GetReceiveAddressRequest) -> Result<ReceiveAddress>;

    /// Generate a new receive address
    ///
    /// # Errors
    ///
    /// Returns an error if the CLI command fails or returns invalid JSON
    fn get_new_address(&self, request: GetReceiveAddressRequest) -> Result<ReceiveAddress>;

    /// Retrieve previous addresses
    ///
    /// # Errors
    ///
    /// Returns an error if the CLI command fails or returns invalid JSON
    fn get_previous_addresses(
        &self,
        request: GetPreviousAddressesRequest,
    ) -> Result<Vec<AddressDetails>>;

    // TODO: Implement additional address methods
    // - generate_address
    // - validate_address
    // - get_address_info
    // - list_addresses
}

/// Async Address API trait for Green clients
///
/// Provides asynchronous access to address operations for retrieval,
/// generation, and management.
///
/// # Liquid Network Considerations
///
/// - Receiving addresses may change following network forks or upgrades
/// - Address generation supports asset-labeled addresses
/// - Misuse of addresses could lead to asset loss
#[async_trait::async_trait]
pub trait AsyncAddressApi {
    /// Get an existing receive address
    async fn get_receive_address(
        &self,
        request: GetReceiveAddressRequest,
    ) -> Result<ReceiveAddress>;

    /// Generate a new receive address
    async fn get_new_address(&self, request: GetReceiveAddressRequest) -> Result<ReceiveAddress>;

    /// Retrieve previous addresses
    async fn get_previous_addresses(
        &self,
        request: GetPreviousAddressesRequest,
    ) -> Result<Vec<AddressDetails>>;

    // TODO: Implement additional async address methods
}
