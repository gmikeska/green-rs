//! Subaccount API implementation

use crate::error::Result;
use crate::types::common::Pointer;
use crate::types::subaccount::{
    CreateSubaccountParams, Subaccount, UpdateSubaccountParams,
};

/// Synchronous subaccount extension trait for Green clients
pub trait SubaccountExt {
    /// Get all subaccounts
    ///
    /// Returns a list of all subaccounts in the wallet
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Subaccount>)` - List of subaccounts
    /// * `Err(Error)` - On failure
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
    fn update_subaccount(&self, pointer: Pointer, params: UpdateSubaccountParams)
        -> Result<Subaccount>;
}

/// Asynchronous subaccount extension trait for Green clients
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
pub trait SubaccountApi {
    // Lower-level API methods that could be implemented by different backends
}
