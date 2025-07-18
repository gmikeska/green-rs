//! Wallet API implementation

use crate::error::Result;
use crate::types::{Balance, FeeEstimates};

/// Synchronous wallet extension trait for Green clients
pub trait WalletExt {
    /// Get wallet balance
    ///
    /// Returns the balance for all assets in the wallet
    ///
    /// # Returns
    ///
    /// * `Ok(Balance)` - The wallet balance
    /// * `Err(Error)` - On failure
    fn get_balance(&self) -> Result<Balance>;

    /// Get fee estimates
    ///
    /// Returns fee estimates for different confirmation targets
    ///
    /// # Returns
    ///
    /// * `Ok(FeeEstimates)` - Fee estimates indexed by number of blocks
    /// * `Err(Error)` - On failure
    fn get_fee_estimates(&self) -> Result<FeeEstimates>;
}

/// Asynchronous wallet extension trait for Green clients
#[async_trait::async_trait]
pub trait AsyncWalletExt {
    /// Get wallet balance
    ///
    /// Returns the balance for all assets in the wallet
    ///
    /// # Returns
    ///
    /// * `Ok(Balance)` - The wallet balance
    /// * `Err(Error)` - On failure
    async fn get_balance(&self) -> Result<Balance>;

    /// Get fee estimates
    ///
    /// Returns fee estimates for different confirmation targets
    ///
    /// # Returns
    ///
    /// * `Ok(FeeEstimates)` - Fee estimates indexed by number of blocks
    /// * `Err(Error)` - On failure
    async fn get_fee_estimates(&self) -> Result<FeeEstimates>;
}

/// Wallet API trait for Green clients
pub trait WalletApi {
    // TODO: Define wallet API methods
    // Examples:
    // - create_wallet
    // - list_wallets
    // - get_wallet_info
    // - delete_wallet
}
