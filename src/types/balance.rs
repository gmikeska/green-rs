//! Balance-related types for the Green API

use super::common::{AssetId, Satoshis};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Balance response containing amounts for different assets
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Balance(pub HashMap<AssetId, Satoshis>);

impl Balance {
    /// Create a new empty balance
    #[must_use]
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Get balance for a specific asset
    #[must_use]
    pub fn get(&self, asset_id: &str) -> Option<Satoshis> {
        self.0.get(asset_id).copied()
    }

    /// Set balance for a specific asset
    pub fn set(&mut self, asset_id: AssetId, amount: Satoshis) {
        self.0.insert(asset_id, amount);
    }

    /// Get total number of assets
    #[must_use]
    pub fn asset_count(&self) -> usize {
        self.0.len()
    }

    /// Check if balance is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Default for Balance {
    fn default() -> Self {
        Self::new()
    }
}

/// Detailed balance information with confirmations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DetailedBalance {
    /// Confirmed balance
    pub confirmed: Satoshis,
    /// Unconfirmed balance
    pub unconfirmed: Satoshis,
    /// Asset ID
    pub asset_id: AssetId,
    /// Number of confirmations required
    pub min_confirmations: u32,
}
