//! Wallet-related types for the Green API

use super::common::{AssetId, Satoshis};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Wallet information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletInfo {
    /// Wallet name
    pub name: String,
    /// Network (e.g., "mainnet", "testnet", "liquid")
    pub network: String,
    /// Whether the wallet is watch-only
    #[serde(default)]
    pub watch_only: bool,
    /// Whether the wallet is locked
    #[serde(default)]
    pub is_locked: bool,
    /// Wallet version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// Network information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    /// Network name
    pub name: String,
    /// Network ID
    pub network: String,
    /// Whether this is mainnet
    #[serde(default)]
    pub mainnet: bool,
    /// Whether this is a Liquid network
    #[serde(default)]
    pub liquid: bool,
    /// Development/testing network
    #[serde(default)]
    pub development: bool,
    /// Default servers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<String>>,
    /// Policy asset (for Liquid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_asset: Option<AssetId>,
}

/// Login credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginCredentials {
    /// Mnemonic phrase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnemonic: Option<String>,
    /// PIN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin: Option<String>,
    /// Password
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Watch-only data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_only: Option<WatchOnlyData>,
}

/// Watch-only wallet data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchOnlyData {
    /// Extended public key
    pub xpub: String,
    /// Core descriptors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_descriptors: Option<Vec<String>>,
}

/// Wallet settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletSettings {
    /// Unit for displaying amounts (e.g., "btc", "mbtc", "ubtc", "bits", "sats")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Required number of blocks for confirmation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_num_blocks: Option<u32>,
    /// Whether to enable RBF by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rbf: Option<bool>,
    /// Default fee rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_fee_rate: Option<u64>,
    /// Dust limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dust_limit: Option<Satoshis>,
    /// Custom user settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<HashMap<String, serde_json::Value>>,
}

/// Fee estimates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeEstimates {
    /// Fee rates indexed by number of blocks
    pub fees: HashMap<u32, u64>,
}

/// Block information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockInfo {
    /// Current block height
    pub block_height: u32,
    /// Current block hash
    pub block_hash: String,
    /// Timestamp
    pub timestamp: u64,
}

/// Wallet limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletLimits {
    /// Is amount rate limited
    #[serde(default)]
    pub is_fiat_rate_limited: bool,
    /// Spending limits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<SpendingLimits>,
}

/// Spending limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendingLimits {
    /// BTC limits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btc: Option<HashMap<String, Satoshis>>,
    /// Fiat limits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiat: Option<HashMap<String, f64>>,
}
