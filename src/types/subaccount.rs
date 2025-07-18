//! Subaccount-related types for the Green API

use super::common::{AssetId, Pointer, Satoshis};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Subaccount information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subaccount {
    /// Subaccount pointer/index
    pub pointer: Pointer,
    /// Subaccount name
    pub name: String,
    /// Subaccount type (e.g., "2of2", "2of3", "2of2_no_recovery")
    #[serde(rename = "type")]
    pub subaccount_type: String,
    /// Recovery mnemonic (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_mnemonic: Option<String>,
    /// Recovery xpub
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_xpub: Option<String>,
    /// Required signatures
    #[serde(default)]
    pub required_ca: u32,
    /// Available signatures
    #[serde(default)]
    pub available_ca: u32,
    /// Whether this subaccount is hidden
    #[serde(default)]
    pub hidden: bool,
    /// BIP44 account index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bip44_discovered: Option<bool>,
}

/// Subaccount balance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubaccountBalance {
    /// Subaccount pointer
    pub pointer: Pointer,
    /// Balances by asset
    pub balance: HashMap<AssetId, BalanceDetail>,
    /// Fiat values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiat_value: Option<HashMap<String, f64>>,
}

/// Balance details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceDetail {
    /// Confirmed balance
    pub satoshi: Satoshis,
    /// Unconfirmed balance
    #[serde(default)]
    pub unconfirmed_satoshi: Satoshis,
    /// Fiat value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiat_value: Option<f64>,
    /// Fiat currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiat_currency: Option<String>,
}

/// Create subaccount request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubaccountRequest {
    /// Subaccount name
    pub name: String,
    /// Subaccount type
    #[serde(rename = "type")]
    pub subaccount_type: String,
    /// Recovery mnemonic (for 2of3)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_mnemonic: Option<String>,
    /// Recovery xpub (for 2of3)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_xpub: Option<String>,
}

/// Update subaccount request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSubaccountRequest {
    /// Subaccount pointer
    pub subaccount: Pointer,
    /// New name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Hidden status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
}

/// Subaccount list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubaccountList {
    /// List of subaccounts
    pub subaccounts: Vec<Subaccount>,
}

/// Subaccount unspent outputs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubaccountUtxos {
    /// Subaccount pointer
    pub subaccount: Pointer,
    /// List of UTXOs
    pub utxos: Vec<Utxo>,
    /// Total number of UTXOs
    #[serde(default)]
    pub total_utxos: u32,
    /// Total value
    #[serde(default)]
    pub total_satoshi: Satoshis,
}

/// Unspent transaction output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Utxo {
    /// Transaction ID
    pub txhash: String,
    /// Output index
    pub vout: u32,
    /// Amount in satoshis
    pub satoshi: Satoshis,
    /// Asset ID (for Liquid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<AssetId>,
    /// Block height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_height: Option<u32>,
    /// Address type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
    /// Subaccount
    pub subaccount: Pointer,
    /// Pointer
    pub pointer: u32,
    /// Whether this UTXO is confidential (Liquid)
    #[serde(default)]
    pub is_confidential: bool,
    /// Whether this UTXO is currently frozen
    #[serde(default)]
    pub is_frozen: bool,
}
