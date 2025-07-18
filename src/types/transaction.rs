//! Transaction-related types for the Green API

use super::common::{Address, AssetId, BlockHeight, Satoshis, Script, TxId};
use serde::{Deserialize, Serialize};

/// Transaction input
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TxInput {
    /// Previous transaction ID
    pub txid: TxId,
    /// Output index in the previous transaction
    pub vout: u32,
    /// Script signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_sig: Option<Script>,
    /// Witness data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness: Option<Vec<String>>,
    /// Sequence number
    pub sequence: u32,
    /// Previous output information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prevout: Option<TxOutput>,
    /// Whether this input is from our wallet
    #[serde(default)]
    pub is_relevant: bool,
    /// Address that was spent from (if known)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    /// Subaccount this input belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    /// Pointer for HD derivation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<u32>,
}

/// Transaction output
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TxOutput {
    /// Amount in satoshis
    pub satoshi: Satoshis,
    /// Script pubkey
    pub script_pubkey: Script,
    /// Address (if standard)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    /// Asset ID (for Liquid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<AssetId>,
    /// Whether this output is to our wallet
    #[serde(default)]
    pub is_relevant: bool,
    /// Subaccount this output belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    /// Pointer for HD derivation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<u32>,
    /// Whether this is a change output
    #[serde(default)]
    pub is_change: bool,
}

/// Full transaction details
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    /// Transaction ID
    pub txid: TxId,
    /// Transaction version
    pub version: i32,
    /// Lock time
    pub locktime: u32,
    /// Transaction inputs
    pub inputs: Vec<TxInput>,
    /// Transaction outputs
    pub outputs: Vec<TxOutput>,
    /// Transaction weight
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u32>,
    /// Transaction size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
    /// Virtual size (vsize)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsize: Option<u32>,
    /// Fee paid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<Satoshis>,
    /// Fee rate in satoshis per vbyte
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_rate: Option<f64>,
    /// Block hash this transaction is included in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<String>,
    /// Block height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_height: Option<BlockHeight>,
    /// Confirmations
    #[serde(default)]
    pub confirmations: u32,
    /// Timestamp (block time or reception time)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
    /// Transaction memo/label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    /// Transaction type (e.g., "incoming", "outgoing", "redeposit")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_type: Option<String>,
    /// Affected subaccounts
    #[serde(default)]
    pub subaccounts: Vec<u32>,
    /// Whether this transaction can be replaced (RBF)
    #[serde(default)]
    pub can_rbf: bool,
    /// Whether this transaction has been replaced
    #[serde(default)]
    pub has_been_replaced: bool,
    /// Raw transaction hex
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
}

/// Transaction list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionList {
    /// List of transactions
    pub transactions: Vec<Transaction>,
    /// Whether there are more transactions
    #[serde(default)]
    pub more: bool,
    /// Next page token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
}

/// Create transaction request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateTransactionRequest {
    /// Recipients: address -> amount mapping
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addressees: Option<Vec<Addressee>>,
    /// Fee rate in satoshis per vbyte
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_rate: Option<u64>,
    /// Subaccount to send from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    /// Send all funds (sweep)
    #[serde(default)]
    pub send_all: bool,
    /// Transaction memo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    /// UTXOs to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxos: Option<Vec<UtxoRef>>,
}

/// Transaction recipient
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Addressee {
    /// Recipient address
    pub address: Address,
    /// Amount to send in satoshis
    pub satoshi: Satoshis,
    /// Asset ID (for Liquid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<AssetId>,
}

/// UTXO reference
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UtxoRef {
    /// Transaction ID
    pub txid: TxId,
    /// Output index
    pub vout: u32,
}

/// Transaction creation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTransactionResult {
    /// Unsigned transaction
    pub transaction: Transaction,
    /// Unsigned transaction hex
    pub unsigned_hex: String,
    /// Inputs to sign
    pub inputs_to_sign: Vec<InputToSign>,
    /// Estimated final size
    pub estimated_vsize: u32,
    /// Estimated fee
    pub estimated_fee: Satoshis,
}

/// Input signing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputToSign {
    /// Input index
    pub index: u32,
    /// Required signatures
    pub required_signatures: u32,
    /// Signing pubkeys
    pub pubkeys: Vec<String>,
    /// Derivation paths
    pub paths: Vec<Vec<u32>>,
    /// Script to sign
    pub script: Script,
    /// Sighash type
    #[serde(default)]
    pub sighash: u32,
}
