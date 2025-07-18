//! UTXO API implementation

use crate::types::{AssetId, GetUnspentOutputsParams, UnspentOutput};
use crate::Result;
use std::collections::HashMap;

/// UTXO API trait for Green clients
pub trait UtxoApi {
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
    async fn get_unspent_outputs(
        &self,
        params: GetUnspentOutputsParams,
    ) -> Result<HashMap<AssetId, Vec<UnspentOutput>>>;

    // TODO: Define other async UTXO API methods
}
