//! UTXO API implementation
//!
//! This module provides traits and implementations for UTXO-related operations
//! in the Green client, allowing for filtering, retrieving, and managing
//! unspent outputs.
//!
//! # Liquid Network Support
//!
//! UTXOs in Liquid feature:
//! - Confidential amounts and asset types
//! - Multi-asset support within the same UTXO set
//! - Freeze and thaw operations for UTXO management
//! - Blinding factors for confidential output control

use crate::types::{AssetId, GetUnspentOutputsParams, UnspentOutput};
use crate::Result;
use std::collections::HashMap;

/// UTXO API trait for Green clients
///
/// Provides synchronous methods for UTXO operations including
/// retrieval, filtering, and potential future UTXO management extensions.
///
/// # Liquid Network Considerations
///
/// - UTXOs contain confidential amounts using blinding factors
/// - Asset IDs are required for retrieving specific asset UTXOs
/// - Freeze/thaw capabilities affect UTXO spending ability
/// - Sorting options affect UTXO selection in issuance and reissuance
pub trait UtxoApi {
    /// Get unspent outputs with filtering parameters
    ///
    /// Returns a `HashMap` grouped by asset ID, where each asset ID maps to a vector of unspent outputs.
    ///
    /// # Arguments
    ///
    /// * `params` - Filtering parameters for the UTXOs
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use green_rs::types::GetUnspentOutputsParams;
    /// # async fn example(client: impl green_rs::api::utxo::UtxoApi) -> Result<(), Box<dyn std::error::Error>> {
    /// let params = GetUnspentOutputsParams {
    ///     min_confs: Some(6),
    ///     include_frozen: Some(false),
    ///     ..Default::default()
    /// };
    /// let utxos = client.get_unspent_outputs(params)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails
    fn get_unspent_outputs(
        &self,
        params: GetUnspentOutputsParams,
    ) -> Result<HashMap<AssetId, Vec<UnspentOutput>>>;

    // TODO: Define other UTXO API methods
    // - get_utxo_info
    // - lock_utxo
    // - unlock_utxo
}

/// Async UTXO API trait for Green clients
///
/// Provides asynchronous methods for UTXO operations including
/// retrieval, filtering, and potential future UTXO management extensions.
///
/// # Liquid Network Considerations
///
/// - UTXOs contain confidential amounts using blinding factors
/// - Asset IDs are required for retrieving specific asset UTXOs
/// - Freeze/thaw capabilities affect UTXO spending ability
/// - Sorting options affect UTXO selection in issuance and reissuance
#[async_trait::async_trait]
pub trait AsyncUtxoApi {
    /// Get unspent outputs with filtering parameters
    ///
    /// Returns a HashMap grouped by asset ID, where each asset ID maps to a vector of unspent outputs.
    ///
    /// # Arguments
    ///
    /// * `params` - Filtering parameters for the UTXOs
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use green_rs::types::GetUnspentOutputsParams;
    /// # async fn example(client: impl green_rs::api::utxo::AsyncUtxoApi) -> Result<(), Box<dyn std::error::Error>> {
    /// let params = GetUnspentOutputsParams {
    ///     min_confs: Some(6),
    ///     include_frozen: Some(false),
    ///     ..Default::default()
    /// };
    /// let utxos = client.get_unspent_outputs(params).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails
    async fn get_unspent_outputs(
        &self,
        params: GetUnspentOutputsParams,
    ) -> Result<HashMap<AssetId, Vec<UnspentOutput>>>;

    // TODO: Define other async UTXO API methods
}
