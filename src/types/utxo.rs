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

/// Unspent output representation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnspentOutput {
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
    /// Number of confirmations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmations: Option<u32>,
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

impl From<UtxoDetails> for UnspentOutput {
    fn from(utxo: UtxoDetails) -> Self {
        Self {
            txhash: utxo.txhash,
            vout: utxo.vout,
            satoshi: utxo.satoshi,
            asset_id: utxo.asset_id,
            block_height: utxo.block_height,
            confirmations: None, // Will be calculated separately
            address: utxo.address,
            address_type: utxo.address_type,
            script_pubkey: utxo.script_pubkey,
            subaccount: utxo.subaccount,
            pointer: utxo.pointer,
            is_internal: utxo.is_internal,
            is_confidential: utxo.is_confidential,
            is_frozen: utxo.is_frozen,
            memo: utxo.memo,
        }
    }
}

/// Sort options for unspent outputs
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UtxoSortBy {
    /// Sort by value (ascending)
    Value,
    /// Sort by value (descending)
    ValueDesc,
    /// Sort by age (oldest first)
    Age,
    /// Sort by age (newest first)
    AgeDesc,
    /// Sort by number of confirmations (ascending)
    Confirmations,
    /// Sort by number of confirmations (descending)
    ConfirmationsDesc,
}

/// Parameters for getting unspent outputs
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetUnspentOutputsParams {
    /// Subaccount to query (None for all)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    /// Minimum number of confirmations required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_confs: Option<u32>,
    /// Maximum number of confirmations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_confs: Option<u32>,
    /// Include frozen UTXOs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_frozen: Option<bool>,
    /// Only confidential UTXOs (Liquid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidential_only: Option<bool>,
    /// Sort criteria
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<UtxoSortBy>,
    /// Filter by asset ID (for Liquid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<AssetId>,
    /// Minimum value filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<Satoshis>,
    /// Maximum value filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<Satoshis>,
}
