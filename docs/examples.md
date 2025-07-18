# Examples

This page provides comprehensive examples of using Green.rs in various scenarios.

## Basic Transaction Creation

```rust
use green_rs::api::TxBuilder;
use green_rs::Result;

fn main() -> Result<()> {
    // Create and send a simple transaction
    let txid = TxBuilder::new()
        .add_output("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(), 100000)
        .set_fee_rate(10)
        .dump()?
        .sign()?
        .broadcast()?;
    
    println!("Transaction sent: {}", txid);
    Ok(())
}
```

## Multi-Output Transaction

```rust
use green_rs::api::TxBuilder;
use green_rs::Result;

fn send_to_multiple_recipients() -> Result<String> {
    TxBuilder::new()
        // Add multiple recipients
        .add_output("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(), 100000)
        .add_output("bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq".to_string(), 50000)
        .add_output("bc1q5l9p6h5gvpe3t8xynwjk5tn9lhrkx5shjqj8x2".to_string(), 25000)
        .set_fee_rate(15)
        .dump()?
        .sign()?
        .broadcast()
}
```

## Balance Checking

```rust
use green_rs::{GreenClient, api::WalletApi};

fn check_balance() -> Result<()> {
    let client = GreenClient::new();
    
    // Check balance for default account
    let balance = client.get_balance(None, None)?;
    println!("Total balance: {} sats", balance.satoshi);
    
    // Check balance for specific subaccount with confirmations
    let confirmed_balance = client.get_balance(Some(0), Some(6))?;
    println!("Confirmed balance (6+ confs): {} sats", confirmed_balance.satoshi);
    
    Ok(())
}
```

## Address Management

```rust
use green_rs::{GreenClient, api::AddressApi};

fn manage_addresses() -> Result<()> {
    let client = GreenClient::new();
    
    // Generate new receive address
    let addr = client.get_receive_address(None)?;
    println!("New address: {}", addr.address);
    
    // Validate an address
    let is_valid = client.validate_address("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh")?;
    println!("Address valid: {}", is_valid);
    
    Ok(())
}
```

## UTXO Management

```rust
use green_rs::{GreenClient, api::UtxoApi};

fn list_utxos() -> Result<()> {
    let client = GreenClient::new();
    
    // Get all unspent outputs
    let utxos = client.get_unspent_outputs(None, None)?;
    
    for utxo in utxos.unspent_outputs {
        println!("UTXO: {}:{} - {} sats", 
            utxo.txhash, 
            utxo.vout, 
            utxo.satoshi
        );
    }
    
    Ok(())
}
```

## Async Operations

```rust
use green_rs::{AsyncGreenClient, api::AsyncWalletApi};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let client = AsyncGreenClient::new();
    
    // Async balance check
    let balance = client.get_balance(None, None).await?;
    println!("Balance: {} sats", balance.satoshi);
    
    // Async fee estimates
    let fees = client.get_fee_estimates().await?;
    println!("Current fee rate: {} sat/vB", fees.regular);
    
    Ok(())
}
```

## Error Handling

```rust
use green_rs::{api::TxBuilder, Error, Result};

fn robust_transaction_handling() -> Result<()> {
    let result = TxBuilder::new()
        .add_output("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(), 100000)
        .set_fee_rate(10)
        .dump();
    
    match result {
        Ok(builder) => {
            match builder.sign() {
                Ok(signed) => {
                    match signed.broadcast() {
                        Ok(txid) => println!("Success: {}", txid),
                        Err(Error::Cli(stderr)) => {
                            eprintln!("Broadcast failed: {}", stderr);
                            // Handle specific CLI errors
                            if stderr.contains("insufficient funds") {
                                eprintln!("Not enough funds for transaction");
                            }
                        }
                        Err(e) => eprintln!("Unexpected error: {}", e),
                    }
                }
                Err(Error::Timeout) => eprintln!("Signing timed out"),
                Err(e) => eprintln!("Signing failed: {}", e),
            }
        }
        Err(Error::Io(e)) => eprintln!("IO error: {}", e),
        Err(e) => eprintln!("Failed to dump transaction: {}", e),
    }
    
    Ok(())
}
```

## Custom CLI Commands

```rust
use green_rs::client::{run_cli, run_cli_async};

fn custom_cli_operations() -> Result<()> {
    // Synchronous CLI call
    let network_info = run_cli(&["network", "info"])?;
    println!("Network info: {}", network_info);
    
    // Get mempool info
    let mempool = run_cli(&["mempool", "info"])?;
    println!("Mempool: {}", mempool);
    
    Ok(())
}

async fn async_cli_operations() -> Result<()> {
    // Async CLI call
    let blockchain_info = run_cli_async(&["blockchain", "info"]).await?;
    println!("Blockchain info: {}", blockchain_info);
    
    Ok(())
}
```

## Complete Wallet Example

```rust
use green_rs::{GreenClient, api::*, Result};
use std::collections::HashMap;

struct WalletManager {
    client: GreenClient,
}

impl WalletManager {
    fn new() -> Self {
        Self {
            client: GreenClient::new(),
        }
    }
    
    fn get_balance_summary(&self) -> Result<()> {
        let balance = self.client.get_balance(None, None)?;
        let fees = self.client.get_fee_estimates()?;
        
        println!("=== Wallet Summary ===");
        println!("Balance: {} BTC", balance.satoshi as f64 / 100_000_000.0);
        println!("Current fee rates:");
        println!("  - Priority: {} sat/vB", fees.priority);
        println!("  - Regular: {} sat/vB", fees.regular);
        println!("  - Economy: {} sat/vB", fees.economy);
        
        Ok(())
    }
    
    fn create_payment(&self, payments: HashMap<String, u64>) -> Result<String> {
        let mut builder = TxBuilder::new();
        
        // Add all payment outputs
        for (address, amount) in payments {
            builder = builder.add_output(address, amount);
        }
        
        // Set appropriate fee rate
        let fees = self.client.get_fee_estimates()?;
        builder = builder.set_fee_rate(fees.regular);
        
        // Execute transaction
        builder.dump()?.sign()?.broadcast()
    }
    
    fn list_recent_transactions(&self, count: u32) -> Result<()> {
        let txs = self.client.get_transactions(None, Some(0), Some(count))?;
        
        println!("=== Recent Transactions ===");
        for tx in txs.transactions {
            println!("TX: {} - {} confirmations", tx.txid, tx.confirmations);
            if let Some(memo) = tx.memo {
                println!("  Memo: {}", memo);
            }
        }
        
        Ok(())
    }
}

fn main() -> Result<()> {
    let wallet = WalletManager::new();
    
    // Show wallet summary
    wallet.get_balance_summary()?;
    
    // Create a payment
    let mut payments = HashMap::new();
    payments.insert("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(), 100000);
    payments.insert("bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq".to_string(), 50000);
    
    match wallet.create_payment(payments) {
        Ok(txid) => println!("Payment sent: {}", txid),
        Err(e) => eprintln!("Payment failed: {}", e),
    }
    
    // List recent transactions
    wallet.list_recent_transactions(10)?;
    
    Ok(())
}
```

## Running Examples

All examples in the `examples/` directory can be run with:

```bash
# Basic examples
cargo run --example wallet_example
cargo run --example tx_builder_example
cargo run --example utxo_management

# Async examples
cargo run --example async_cli_example
cargo run --example utxo_management_async

# Error handling example
cargo run --example cli_error_handling
```
