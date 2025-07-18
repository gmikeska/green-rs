# CLI Integration

Green.rs interacts with the `green-cli` to enable various blockchain operations such as signing and broadcasting transactions.

## Dependency

Ensure that `green-cli` is installed and accessible in the system's PATH.

## Signing Transactions

The following is an example command for signing a transaction:

```shell
green-cli tx sign --file /path/to/transaction/file
```

The `TxBuilder` will automatically handle file creation and cleanup for transactions.

## Broadcasting Transactions

To broadcast a transaction:

```shell
green-cli tx send --file /path/to/signed/transaction/file
```

## Error Handling

Any errors from the CLI will be captured and returned by the API.

## Custom Commands

If additional functionality is needed beyond what Green.rs provides, custom CLI commands can be executed using the `run_cli` or `run_cli_async` functions.

### Usage

```rust
use green_rs::client::run_cli;

match run_cli(&["tx", "status", "--txid", "abc123..."]) {
    Ok(output) => println!("Status: {}", output),
    Err(e) => eprintln!("Command failed: {}", e),
}
```

Asynchronously:

```rust
use green_rs::client::run_cli_async;

let result = run_cli_async(&["tx", "status", "--txid", "abc123..."]).await;
match result {
    Ok(output) => println!("Status: {}", output),
    Err(e) => eprintln!("Command failed: {}", e),
}
```

