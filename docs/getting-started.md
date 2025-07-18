# Getting Started with Green.rs

This guide will help you get started with Green.rs quickly.

## Installation

Add Green.rs to your `Cargo.toml`:

```toml
[dependencies]
green-rs = "0.1"
```

For async operations, you'll also need an async runtime:

```toml
[dependencies]
green-rs = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Basic Setup

```rust
use green_rs::{GreenClient, Result};

fn main() -> Result<()> {
    // Create a new Green client
    let client = GreenClient::new();
    
    // Your code here
    
    Ok(())
}
```

## First Transaction

Here's how to create your first transaction:

```rust
use green_rs::api::TxBuilder;

fn main() -> Result<()> {
    let tx = TxBuilder::new()
        .add_output("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(), 100000)
        .set_fee_rate(10)
        .dump()?
        .sign()?
        .broadcast()?;
    
    println!("Transaction broadcasted: {}", tx);
    Ok(())
}
```

## Environment Setup

Green.rs expects the `green-cli` command to be available in your system PATH. Make sure you have:

1. Green wallet CLI installed
2. Proper authentication set up
3. Network configuration (mainnet/testnet)

## Next Steps

- Read the [API Reference](api-reference.md) for detailed API documentation
- Check out [Examples](examples.md) for more use cases
- Learn about [Error Handling](error-handling.md) best practices
