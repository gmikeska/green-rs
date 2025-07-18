# Green API Types

This module contains all the JSON request and response types for the Green API, with full serde serialization support.

## Structure

### Common Types (`common.rs`)
- Type aliases for commonly used types:
  - `AssetId`: String identifier for assets (hex string)
  - `Satoshis`: u64 amount in satoshis
  - `TxId`: Transaction identifier (hex string)
  - `BlockHeight`: u32 block height
  - `Pointer`: u32 HD derivation pointer
  - `Script`: Hex-encoded script
  - `Address`: Bitcoin/Liquid address string

### Balance Types (`balance.rs`)
- `Balance`: HashMap-based balance container for multiple assets
- `DetailedBalance`: Balance with confirmation details

### Address Types (`address.rs`)
- `ReceiveAddress`: Complete address information with derivation details
- `GetReceiveAddressRequest`: Request parameters for generating addresses
- `AddressDetails`: Detailed address information including usage stats

### Transaction Types (`transaction.rs`)
- `Transaction`: Full transaction details with inputs, outputs, and metadata
- `TxInput`: Transaction input with witness data and prevout info
- `TxOutput`: Transaction output with amount and script details
- `CreateTransactionRequest`: Parameters for creating new transactions
- `TransactionList`: Paginated transaction results

### Wallet Types (`wallet.rs`)
- `WalletInfo`: Basic wallet information
- `NetworkInfo`: Network configuration details
- `LoginCredentials`: Authentication data
- `WalletSettings`: User preferences and configuration
- `FeeEstimates`: Fee rate suggestions

### Subaccount Types (`subaccount.rs`)
- `Subaccount`: Subaccount details including type and recovery info
- `SubaccountBalance`: Balance information per subaccount
- `CreateSubaccountRequest`: Parameters for creating new subaccounts

### UTXO Types (`utxo.rs`)
- `UtxoDetails`: Complete UTXO information
- `GetUtxosRequest`: Query parameters for UTXO listing
- `UtxoSummary`: Aggregated UTXO statistics

## Usage Example

```rust
use green_rs::types::*;

// Create a balance response
let mut balance = Balance::new();
balance.set("btc".to_string(), 100000);

// Create a receive address
let addr = ReceiveAddress {
    address: "bc1q...".to_string(),
    pointer: 0,
    address_type: "p2wpkh".to_string(),
    branch: 0,
    subaccount: 0,
    script_pubkey: None,
    is_confidential: None,
    unconfidential_address: None,
};

// Serialize to JSON
let json = serde_json::to_string(&addr)?;
```

## Notes

- All types use `#[serde(skip_serializing_if = "Option::is_none")]` for optional fields to minimize JSON size
- Fields that might be missing use `Option<T>` types
- Boolean fields use `#[serde(default)]` to handle missing values as `false`
- Some fields use `#[serde(rename = "...")]` to match the API's JSON field names
