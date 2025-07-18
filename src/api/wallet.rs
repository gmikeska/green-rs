//! Wallet API implementation
//!
//! This module provides traits and implementations for wallet-related operations
//! in the Green client. It supports both synchronous and asynchronous operations
//! for managing wallet state, balances, and fee estimation.
//!
//! # Liquid Network Support
//!
//! When working with Liquid wallets:
//! - Balances include multiple asset types, not just L-BTC
//! - Fee estimates are denominated in L-BTC only
//! - Confidential transactions may hide balance information
//! - Blinding factors are required to reveal confidential amounts

use crate::error::Result;
use crate::types::{Balance, FeeEstimates};

/// Synchronous wallet extension trait for Green clients
///
/// Provides blocking wallet operations for balance queries and fee estimation.
///
/// # Liquid Network Considerations
///
/// - Balance queries return amounts for all assets in the wallet
/// - L-BTC is the fee asset for all Liquid transactions
/// - Confidential amounts require unblinding to view
pub trait WalletExt {
    /// Get wallet balance
    ///
    /// Returns the balance for all assets in the wallet
    ///
    /// # Returns
    ///
    /// * `Ok(Balance)` - The wallet balance
    /// * `Err(Error)` - On failure
    ///
    /// # Errors
    ///
    /// Returns an error if the balance cannot be retrieved
    fn get_balance(&self) -> Result<Balance>;

    /// Get fee estimates
    ///
    /// Returns fee estimates for different confirmation targets
    ///
    /// # Returns
    ///
    /// * `Ok(FeeEstimates)` - Fee estimates indexed by number of blocks
    /// * `Err(Error)` - On failure
    ///
    /// # Errors
    ///
    /// Returns an error if fee estimates cannot be retrieved
    fn get_fee_estimates(&self) -> Result<FeeEstimates>;
}

/// Asynchronous wallet extension trait for Green clients
///
/// Provides non-blocking wallet operations for balance queries and fee estimation.
///
/// # Liquid Network Considerations
///
/// - Balance queries return amounts for all assets in the wallet
/// - L-BTC is the fee asset for all Liquid transactions
/// - Confidential amounts require unblinding to view
/// - Fee estimates may vary based on transaction complexity
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
///
/// This trait is reserved for future wallet management operations.
/// Currently, wallet operations are handled through the `green-cli`
/// command-line interface outside of this library.
///
/// Future methods may include:
/// - Wallet creation and initialization
/// - Wallet listing and discovery
/// - Wallet metadata and configuration
/// - Wallet deletion and cleanup
pub trait WalletApi {
    // TODO: Define wallet API methods
    // Examples:
    // - create_wallet
    // - list_wallets
    // - get_wallet_info
    // - delete_wallet
}
