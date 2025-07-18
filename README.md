# Green.rs

[![Crates.io](https://img.shields.io/crates/v/green-rs.svg)](https://crates.io/crates/green-rs)
[![Documentation](https://docs.rs/green-rs/badge.svg)](https://docs.rs/green-rs)
[![License](https://img.shields.io/crates/l/green-rs.svg)](https://github.com/gmikeska/green-rs#license)
[![CI](https://github.com/gmikeska/green-rs/workflows/CI/badge.svg)](https://github.com/gmikeska/green-rs/actions)

Green.rs is a Rust library for interacting with the Green API to handle cryptocurrency transactions. It provides a Builder pattern to facilitate transaction creation, signing, and broadcasting.

## Features

- **Dual API Design**: Both synchronous and asynchronous interfaces
- **Transaction Builder**: Fluent interface for constructing complex transactions
- **Type Safety**: Leverages Rust's type system for compile-time guarantees
- **Comprehensive Error Handling**: Detailed error types for debugging
- **Well Tested**: Extensive test coverage
- **Examples**: Complete working examples for common use cases

## Installation

To use Green.rs in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
green-rs = "0.1"
```

## Quick Start

### Synchronous Usage

```rust
use green_rs::client::GreenClient;
use green_rs::api::wallet::WalletExt;
use green_rs::api::address::AddressApi;
use green_rs::api::builder::GetReceiveAddressBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a synchronous client
    let client = GreenClient::new();
    
    // Get wallet balance
    let balance = client.get_balance()?;
    println!("Balance: {} BTC satoshis", balance.get_btc_balance());
    
    // Generate a new address
    let address_request = GetReceiveAddressBuilder::new()
        .subaccount(0)
        .build();
    let address = client.get_new_address(address_request)?;
    println!("New address: {}", address.address);
    
    Ok(())
}
```

### Asynchronous Usage with Tokio

```rust
use green_rs::client::AsyncGreenClient;
use green_rs::api::wallet::AsyncWalletExt;
use green_rs::api::address::AsyncAddressApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AsyncGreenClient::new();
    
    // Concurrent operations
    let (balance, fees) = tokio::join!(
        client.get_balance(),
        client.get_fee_estimates()
    );
    
    println!("Balance: {} sats", balance?.get_btc_balance());
    println!("Fast fee: {} sat/vB", fees?.get_fee_for_target(1).unwrap_or(0));
    
    Ok(())
}
```

### Transaction Builder

```rust
use green_rs::api::transaction::TxBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tx = TxBuilder::new()
        .add_output("bc1qxy2kg...".to_string(), 100000)
        .add_output("bc1qab3de...".to_string(), 50000)
        .set_fee_rate(10)
        .set_subaccount(0)
        .dump()?
        .sign()?
        .send()?;
    
    println!("Transaction sent: {}", tx);
    Ok(())
}
```

## Examples

The `examples/` directory contains complete working examples:

- **`basic_sync.rs`** - Synchronous client usage for getting balance and generating addresses
- **`basic_async.rs`** - Asynchronous client usage with Tokio, including concurrent operations
- **`tx_builder.rs`** - Transaction builder pattern demonstration with multiple outputs

Run examples with:

```sh
cargo run --example basic_sync
cargo run --example basic_async
cargo run --example tx_builder
```

## API Documentation

For detailed API documentation, please visit [docs.rs/green-rs](https://docs.rs/green-rs).

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute to this project.

## Security

If you discover a security vulnerability, please email security@yourdomain.com instead of using the issue tracker.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
