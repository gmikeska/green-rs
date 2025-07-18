//! Subaccount API implementation
//!
//! This module provides traits and implementations for subaccount-related operations
//! in the Green client. Subaccounts allow for logical separation of funds within
//! a single wallet, enabling better organization and management.
//!
//! # Liquid Network Support
//!
//! Subaccount operations in Liquid:
//! - Each subaccount can hold multiple asset types
//! - Asset isolation is maintained per subaccount
//! - Subaccount pointers are consistent across networks
//! - Cross-subaccount transactions require explicit authorization

use crate::error::Result;
use crate::types::common::Pointer;
use crate::types::subaccount::{CreateSubaccountParams, Subaccount, UpdateSubaccountParams};

/// Synchronous subaccount extension trait for Green clients
///
/// Provides blocking operations for subaccount management, including
/// creation, retrieval, and updates.
///
/// # Liquid Network Considerations
///
/// - Subaccounts can hold any Liquid asset type
/// - Balance queries per subaccount include all held assets
/// - Subaccount names are stored locally and not on-chain
/// - Hidden subaccounts remain accessible but are filtered from default lists
pub trait SubaccountExt {
    /// Get all subaccounts
    ///
    /// Returns a list of all subaccounts in the wallet
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Subaccount>)` - List of subaccounts
    /// * `Err(Error)` - On failure
    ///
    /// # Errors
    ///
    /// Returns an error if the CLI command fails or returns invalid JSON
    fn get_subaccounts(&self) -> Result<Vec<Subaccount>>;

    /// Get a specific subaccount
    ///
    /// Returns information about a specific subaccount
    ///
    /// # Arguments
    ///
    /// * `pointer` - The subaccount pointer/index
    ///
    /// # Returns
    ///
    /// * `Ok(Subaccount)` - The subaccount information
    /// * `Err(Error)` - On failure or if subaccount not found
    ///
    /// # Errors
    ///
    /// Returns an error if the CLI command fails or returns invalid JSON
    fn get_subaccount(&self, pointer: Pointer) -> Result<Subaccount>;

    /// Create a new subaccount
    ///
    /// Creates a new subaccount with the specified parameters
    ///
    /// # Arguments
    ///
    /// * `params` - Parameters for creating the subaccount
    ///
    /// # Returns
    ///
    /// * `Ok(Subaccount)` - The newly created subaccount
    /// * `Err(Error)` - On failure
    ///
    /// # Errors
    ///
    /// Returns an error if the CLI command fails or returns invalid JSON
    fn create_subaccount(&self, params: CreateSubaccountParams) -> Result<Subaccount>;

    /// Update an existing subaccount
    ///
    /// Updates subaccount properties like name or hidden status
    ///
    /// # Arguments
    ///
    /// * `pointer` - The subaccount pointer/index
    /// * `params` - Parameters to update
    ///
    /// # Returns
    ///
    /// * `Ok(Subaccount)` - The updated subaccount information
    /// * `Err(Error)` - On failure or if subaccount not found
    ///
    /// # Errors
    ///
    /// Returns an error if the CLI command fails or returns invalid JSON
    fn update_subaccount(
        &self,
        pointer: Pointer,
        params: UpdateSubaccountParams,
    ) -> Result<Subaccount>;
}

/// Asynchronous subaccount extension trait for Green clients
///
/// Provides non-blocking operations for subaccount management, including
/// creation, retrieval, and updates.
///
/// # Liquid Network Considerations
///
/// - Subaccounts can hold any Liquid asset type
/// - Balance queries per subaccount include all held assets  
/// - Subaccount names are stored locally and not on-chain
/// - Hidden subaccounts remain accessible but are filtered from default lists
#[async_trait::async_trait]
pub trait AsyncSubaccountExt {
    /// Get all subaccounts
    ///
    /// Returns a list of all subaccounts in the wallet
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Subaccount>)` - List of subaccounts
    /// * `Err(Error)` - On failure
    async fn get_subaccounts(&self) -> Result<Vec<Subaccount>>;

    /// Get a specific subaccount
    ///
    /// Returns information about a specific subaccount
    ///
    /// # Arguments
    ///
    /// * `pointer` - The subaccount pointer/index
    ///
    /// # Returns
    ///
    /// * `Ok(Subaccount)` - The subaccount information
    /// * `Err(Error)` - On failure or if subaccount not found
    async fn get_subaccount(&self, pointer: Pointer) -> Result<Subaccount>;

    /// Create a new subaccount
    ///
    /// Creates a new subaccount with the specified parameters
    ///
    /// # Arguments
    ///
    /// * `params` - Parameters for creating the subaccount
    ///
    /// # Returns
    ///
    /// * `Ok(Subaccount)` - The newly created subaccount
    /// * `Err(Error)` - On failure
    async fn create_subaccount(&self, params: CreateSubaccountParams) -> Result<Subaccount>;

    /// Update an existing subaccount
    ///
    /// Updates subaccount properties like name or hidden status
    ///
    /// # Arguments
    ///
    /// * `pointer` - The subaccount pointer/index
    /// * `params` - Parameters to update
    ///
    /// # Returns
    ///
    /// * `Ok(Subaccount)` - The updated subaccount information
    /// * `Err(Error)` - On failure or if subaccount not found
    async fn update_subaccount(
        &self,
        pointer: Pointer,
        params: UpdateSubaccountParams,
    ) -> Result<Subaccount>;
}

/// Subaccount API trait for Green clients
///
/// This trait is reserved for future lower-level subaccount operations
/// that may be implemented by different backend providers.
///
/// Future implementations may support:
/// - Direct database access for subaccount metadata
/// - Advanced subaccount policies and permissions
/// - Multi-signature requirements per subaccount
/// - Time-locked or conditional access controls
pub trait SubaccountApi {
    // Lower-level API methods that could be implemented by different backends
}
