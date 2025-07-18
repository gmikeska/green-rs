//! Address-related types for the Green API

use super::common::{Address, Pointer, Script};
use serde::{Deserialize, Serialize};

/// Receive address response from the API
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReceiveAddress {
    /// The receiving address
    pub address: Address,
    /// Subaccount pointer
    pub pointer: Pointer,
    /// Address type (e.g., "p2sh-p2wpkh", "p2wpkh", etc.)
    #[serde(rename = "address_type")]
    pub address_type: String,
    /// Branch (0 for external, 1 for internal/change)
    pub branch: u32,
    /// Subaccount index
    pub subaccount: u32,
    /// Script pubkey
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_pubkey: Option<Script>,
    /// Whether this is a confidential address (Liquid only)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_confidential: Option<bool>,
    /// Unconfidential address (Liquid only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unconfidential_address: Option<Address>,
}

/// Request to generate a new receive address
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetReceiveAddressRequest {
    /// Subaccount to generate address for (default: 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    /// Address type to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
}

/// Previous addresses query parameters
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetPreviousAddressesRequest {
    /// Subaccount to query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    /// Last pointer to start from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_pointer: Option<Pointer>,
    /// Whether to include only unused addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_only: Option<bool>,
}

/// Address details
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AddressDetails {
    /// The address
    pub address: Address,
    /// Address type
    #[serde(rename = "address_type")]
    pub address_type: String,
    /// Subaccount
    pub subaccount: u32,
    /// Pointer
    pub pointer: Pointer,
    /// User-defined label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Transaction count for this address
    #[serde(default)]
    pub tx_count: u32,
    /// Whether the address has been used
    #[serde(default)]
    pub is_used: bool,
}
