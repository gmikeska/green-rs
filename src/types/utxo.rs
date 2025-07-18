//! UTXO-related types for the Green API

use super::common::{Address, AssetId, BlockHeight, Satoshis, Script, TxId};
use serde::{Deserialize, Serialize};

/// UTXO details
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UtxoDetails {
    /// Transaction hash
    pub txhash: TxId,
    /// Output index
    pub vout: u32,
    /// Amount in satoshis
    pub satoshi: Satoshis,
    /// Asset ID (for Liquid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<AssetId>,
    /// Block height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_height: Option<BlockHeight>,
    /// Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    /// Address type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
    /// Script pubkey
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_pubkey: Option<Script>,
    /// Subaccount
    pub subaccount: u32,
    /// Pointer for HD derivation
    pub pointer: u32,
    /// Whether this UTXO is internal (change)
    #[serde(default)]
    pub is_internal: bool,
    /// Whether this UTXO is confidential (Liquid)
    #[serde(default)]
    pub is_confidential: bool,
    /// Whether this UTXO is currently frozen
    #[serde(default)]
    pub is_frozen: bool,
    /// User memo/label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// Get UTXOs request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetUtxosRequest {
    /// Subaccount to query (None for all)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    /// Number of confirmations required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_confs: Option<u32>,
    /// Include frozen UTXOs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_frozen: Option<bool>,
    /// Only confidential UTXOs (Liquid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidential_only: Option<bool>,
    /// Sort by value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by_value: Option<bool>,
}

/// UTXOs response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtxosResponse {
    /// List of UTXOs
    pub utxos: Vec<UtxoDetails>,
    /// Whether there are more UTXOs
    #[serde(default)]
    pub more: bool,
    /// Next page token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

/// Update UTXO request (for freezing/unfreezing)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUtxoRequest {
    /// Transaction hash
    pub txhash: TxId,
    /// Output index
    pub vout: u32,
    /// Freeze status
    pub is_frozen: bool,
    /// Optional memo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// UTXO summary by asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtxoSummary {
    /// Asset ID
    pub asset_id: AssetId,
    /// Number of UTXOs
    pub utxo_count: u32,
    /// Total amount
    pub total_satoshi: Satoshis,
    /// Number of frozen UTXOs
    #[serde(default)]
    pub frozen_count: u32,
    /// Total frozen amount
    #[serde(default)]
    pub frozen_satoshi: Satoshis,
}
