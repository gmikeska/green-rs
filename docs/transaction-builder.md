# Transaction Builder

The `TxBuilder` provides a fluent interface for creating, signing, and broadcasting Bitcoin transactions using the Green CLI.

## Overview

The Transaction Builder follows the builder pattern, allowing you to chain method calls to construct a transaction step by step. The typical flow is:

1. Create a new builder
2. Add outputs (recipients)
3. Configure transaction parameters
4. Dump the transaction data
5. Sign the transaction
6. Broadcast to the network

## Basic Usage

```rust
use green_rs::api::TxBuilder;
use green_rs::Result;

fn main() -> Result<()> {
    let txid = TxBuilder::new()
        .add_output("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(), 100000)
        .add_output("bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq".to_string(), 50000)
        .set_fee_rate(10)
        .dump()?
        .sign()?
        .broadcast()?;
    
    println!("Transaction ID: {}", txid);
    Ok(())
}
```

## Method Reference

### new()
Creates a new transaction builder instance.

```rust
let builder = TxBuilder::new();
```

### add_output()
Adds a recipient to the transaction.

```rust
builder.add_output(address: Address, amount: Satoshis)
```

- `address`: The recipient's Bitcoin address
- `amount`: The amount to send in satoshis

### set_fee_rate()
Sets the transaction fee rate.

```rust
builder.set_fee_rate(fee_rate: u64)
```

- `fee_rate`: Fee rate in satoshis per virtual byte

### set_subaccount()
Specifies which subaccount to use for the transaction.

```rust
builder.set_subaccount(subaccount: u32)
```

### add_input()
Manually adds a UTXO to be spent.

```rust
builder.add_input(utxo: String)
```

- `utxo`: UTXO identifier in format "txid:vout"

### dump()
Serializes the transaction data and saves it to a temporary file.

```rust
let builder = builder.dump()?;
```

This method:
- Creates a JSON representation of the transaction
- Writes it to a temporary file
- Returns the builder for further chaining

### sign()
Signs the transaction using the Green CLI.

```rust
let builder = builder.sign()?;
```

**Note**: Must be called after `dump()`.

### send() / broadcast()
Broadcasts the signed transaction to the network.

```rust
let txid = builder.send()?;
// or
let txid = builder.broadcast()?;
```

Returns the transaction ID as a string.

### Helper Methods

#### to_json()
Get the JSON representation of the transaction data.

```rust
if let Some(json) = builder.to_json() {
    println!("Transaction JSON: {}", json);
}
```

#### get_temp_path()
Get the path to the temporary file containing transaction data.

```rust
if let Some(path) = builder.get_temp_path() {
    println!("Transaction file: {}", path);
}
```

## Advanced Example

```rust
use green_rs::api::TxBuilder;
use green_rs::Result;

fn create_complex_transaction() -> Result<String> {
    let builder = TxBuilder::new()
        // Add multiple outputs
        .add_output("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(), 100000)
        .add_output("bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq".to_string(), 50000)
        .add_output("bc1q5l9p6h5gvpe3t8xynwjk5tn9lhrkx5shjqj8x2".to_string(), 25000)
        
        // Specify inputs
        .add_input("abc123...def456:0".to_string())
        .add_input("789ghi...jkl012:1".to_string())
        
        // Set parameters
        .set_fee_rate(15)
        .set_subaccount(0);
    
    // Dump and inspect
    let builder = builder.dump()?;
    
    if let Some(json) = builder.to_json() {
        println!("Transaction data:\n{}", json);
    }
    
    // Sign and broadcast
    builder.sign()?.broadcast()
}
```

## Error Handling

The builder methods that interact with the filesystem or Green CLI return `Result<T>`. Common errors include:

- **IO errors**: When creating or writing to temporary files
- **Serialization errors**: When converting data to JSON
- **CLI errors**: When green-cli commands fail
- **State errors**: When methods are called in the wrong order

Example with error handling:

```rust
match TxBuilder::new()
    .add_output(address, amount)
    .dump() 
{
    Ok(builder) => {
        match builder.sign() {
            Ok(signed) => {
                match signed.broadcast() {
                    Ok(txid) => println!("Success: {}", txid),
                    Err(e) => eprintln!("Broadcast failed: {}", e),
                }
            }
            Err(e) => eprintln!("Signing failed: {}", e),
        }
    }
    Err(e) => eprintln!("Dump failed: {}", e),
}
```

## Best Practices

1. **Always check Results**: Don't unwrap in production code
2. **Validate addresses**: Ensure addresses are valid before adding them
3. **Set appropriate fees**: Use current network fee estimates
4. **Handle temporary files**: The builder manages temp files, but be aware they exist
5. **Test on testnet**: Always test transaction building on testnet first
