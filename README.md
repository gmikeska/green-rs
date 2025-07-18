# Green.rs

Green.rs is a Rust library for interacting with the Green API to handle cryptocurrency transactions. It provides a Builder pattern to facilitate transaction creation, signing, and broadcasting.

## Installation

To use Green.rs in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
green-rs = "0.1"
```

## Usage

Here's a basic example of how to use the `TxBuilder` to create a transaction:

```rust
use green_rs::api::TxBuilder;

fn main() {
    let tx_builder = TxBuilder::new()
        .add_output("bc1qxy2kg...", 100000)
        .set_fee_rate(10)
        .set_subaccount(0);

    // Dump, sign, and broadcast the transaction
}
```

## Example

You can find a complete example in the `examples` directory:

```sh
cargo run --example tx_builder_example
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for more details.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
