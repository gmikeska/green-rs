# API Reference

This document provides a comprehensive reference for all public APIs in Green.rs.

## Core Types

### GreenClient

The main client for interacting with the Green API.

```rust
pub struct GreenClient {
    // Implementation details
}

impl GreenClient {
    pub fn new() -> Self
    pub fn with_timeout(timeout: Duration) -> Self
}
```

### AsyncGreenClient

Async version of the Green client.

```rust
pub struct AsyncGreenClient {
    // Implementation details
}

impl AsyncGreenClient {
    pub fn new() -> Self
    pub fn with_timeout(timeout: Duration) -> Self
}
```

## Transaction API

### TxBuilder

Builder pattern for creating transactions.

```rust
pub struct TxBuilder {
    // Private fields
}

impl TxBuilder {
    /// Create a new transaction builder
    pub fn new() -> Self
    
    /// Add an output to the transaction
    pub fn add_output(self, address: Address, amount: Satoshis) -> Self
    
    /// Set the fee rate in satoshis per vbyte
    pub fn set_fee_rate(self, fee_rate: u64) -> Self
    
    /// Set the subaccount to use
    pub fn set_subaccount(self, subaccount: u32) -> Self
    
    /// Add an input UTXO
    pub fn add_input(self, utxo: String) -> Self
    
    /// Dump transaction to temporary file
    pub fn dump(self) -> Result<Self>
    
    /// Sign the transaction
    pub fn sign(self) -> Result<Self>
    
    /// Send/broadcast the transaction
    pub fn send(self) -> Result<String>
    pub fn broadcast(self) -> Result<String>
    
    /// Get JSON representation
    pub fn to_json(&self) -> Option<&str>
    
    /// Get temporary file path
    pub fn get_temp_path(&self) -> Option<&str>
}
```

### TransactionApi Trait

```rust
pub trait TransactionApi {
    fn create_transaction(&self, request: CreateTransactionRequest) -> Result<CreateTransactionResult>;
    fn send_to_address(&self, address: &str, amount: Satoshis, asset_id: Option<AssetId>) -> Result<Transaction>;
    fn get_transactions(&self, subaccount: Option<u32>, first: Option<u32>, count: Option<u32>) -> Result<TransactionList>;
    fn get_transaction_details(&self, txid: &str) -> Result<Transaction>;
}
```

## Address API

### AddressApi Trait

```rust
pub trait AddressApi {
    fn get_receive_address(&self, subaccount: Option<u32>) -> Result<ReceiveAddress>;
    fn validate_address(&self, address: &str) -> Result<bool>;
}
```

## UTXO API

### UtxoApi Trait

```rust
pub trait UtxoApi {
    fn get_unspent_outputs(&self, subaccount: Option<u32>, num_confs: Option<u32>) -> Result<UnspentOutputs>;
}
```

## Wallet API

### WalletApi Trait

```rust
pub trait WalletApi {
    fn get_balance(&self, subaccount: Option<u32>, num_confs: Option<u32>) -> Result<Balance>;
    fn get_fee_estimates(&self) -> Result<FeeEstimates>;
}
```

## Common Types

### Address
```rust
pub type Address = String;
```

### AssetId
```rust
pub type AssetId = String;
```

### Satoshis
```rust
pub type Satoshis = u64;
```

### TxId
```rust
pub type TxId = String;
```

### BlockHeight
```rust
pub type BlockHeight = u32;
```

### Script
```rust
pub type Script = String;
```

## Error Types

### Error
```rust
pub enum Error {
    Io(std::io::Error),
    Json(serde_json::Error),
    Cli(String),
    Timeout,
    Network(String),
    InvalidResponse,
    Unexpected(String),
}
```

### Result
```rust
pub type Result<T> = std::result::Result<T, Error>;
```

## CLI Functions

### run_cli
Execute a CLI command synchronously.

```rust
pub fn run_cli(args: &[&str]) -> Result<String>
```

### run_cli_async
Execute a CLI command asynchronously.

```rust
pub async fn run_cli_async(args: &[&str]) -> Result<String>
```
