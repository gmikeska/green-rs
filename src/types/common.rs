//! Common type aliases used throughout the Green API

/// Asset identifier type (typically a hex string for the asset hash)
pub type AssetId = String;

/// Amount in satoshis (smallest unit)
pub type Satoshis = u64;

/// Transaction identifier (hex string)
pub type TxId = String;

/// Block height
pub type BlockHeight = u32;

/// Account/Subaccount pointer
pub type Pointer = u32;

/// Script type (hex encoded)
pub type Script = String;

/// Address string
pub type Address = String;
