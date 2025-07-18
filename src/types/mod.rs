//! Common types used throughout the Green API

// Common type aliases and utilities
pub mod common;

// API-specific types
pub mod address;
pub mod balance;
pub mod subaccount;
pub mod transaction;
pub mod utxo;
pub mod wallet;

// Re-export commonly used types
pub use address::{AddressDetails, ReceiveAddress};
pub use balance::Balance;
pub use common::*;
pub use subaccount::Subaccount;
pub use transaction::{Transaction, TxInput, TxOutput};
pub use utxo::{GetUnspentOutputsParams, UnspentOutput, UnspentOutputs, UtxoDetails, UtxoSortBy};
pub use wallet::{FeeEstimates, NetworkInfo, WalletInfo};
