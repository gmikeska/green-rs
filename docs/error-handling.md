# Error Handling

Green.rs uses a comprehensive error handling system based on Rust's `Result` type and a custom `Error` enum.

## Error Types

The library defines the following error variants:

```rust
pub enum Error {
    /// IO error from std::io operations
    Io(std::io::Error),
    
    /// JSON serialization/deserialization error
    Json(serde_json::Error),
    
    /// CLI error with stderr output
    Cli(String),
    
    /// Operation timeout
    Timeout,
    
    /// Network-related errors
    Network(String),
    
    /// Invalid response received
    InvalidResponse,
    
    /// Catch-all for unexpected errors
    Unexpected(String),
}
```

## Result Type

All fallible operations return the custom `Result` type:

```rust
pub type Result<T> = std::result::Result<T, Error>;
```

## Basic Error Handling

Use pattern matching for fine-grained error handling:

```rust
use green_rs::{GreenClient, Error};

fn handle_balance() {
    let client = GreenClient::new();
    
    match client.get_balance(None, None) {
        Ok(balance) => println!("Balance: {:?}", balance),
        Err(Error::Timeout) => eprintln!("Request timed out"),
        Err(Error::Cli(stderr)) => eprintln!("CLI error: {}", stderr),
        Err(Error::Network(msg)) => eprintln!("Network error: {}", msg),
        Err(e) => eprintln!("Other error: {}", e),
    }
}
```

## Error Propagation

Use the `?` operator for clean error propagation:

```rust
use green_rs::Result;

fn process_transaction() -> Result<String> {
    let txid = TxBuilder::new()
        .add_output("bc1q...".to_string(), 100000)
        .dump()?  // Propagates any error
        .sign()?  // Propagates any error
        .send()?; // Propagates any error
    
    Ok(txid)
}
```

## Creating Custom Errors

The `Error` enum provides helper methods:

```rust
// Create a CLI error
let error = Error::cli_error("Command not found");

// Create a network error
let error = Error::network("Connection refused");

// Create an unexpected error
let error = Error::unexpected("Something went wrong");
```

## Error Conversion

Errors from std::io and serde_json are automatically converted:

```rust
fn read_config() -> Result<Config> {
    let file = std::fs::read_to_string("config.json")?; // io::Error -> Error
    let config: Config = serde_json::from_str(&file)?;   // serde_json::Error -> Error
    Ok(config)
}
```

## Best Practices

### 1. Don't Panic
Avoid using `unwrap()` or `expect()` in production code:

```rust
// Bad
let balance = client.get_balance(None, None).unwrap();

// Good
let balance = client.get_balance(None, None)?;
```

### 2. Provide Context
When creating custom errors, provide meaningful context:

```rust
// Bad
Err(Error::unexpected("failed"))

// Good
Err(Error::unexpected(format!("Failed to process transaction {}: insufficient funds", txid)))
```

### 3. Handle Specific Errors
Handle specific error cases when appropriate:

```rust
match run_cli(&["tx", "send"]) {
    Ok(output) => process_output(output),
    Err(Error::Cli(stderr)) if stderr.contains("insufficient funds") => {
        handle_insufficient_funds()
    }
    Err(Error::Timeout) => retry_operation(),
    Err(e) => return Err(e),
}
```

### 4. Log Errors
Log errors appropriately for debugging:

```rust
match operation() {
    Ok(result) => result,
    Err(e) => {
        log::error!("Operation failed: {}", e);
        return Err(e);
    }
}
```

## Example: Comprehensive Error Handling

```rust
use green_rs::{api::TxBuilder, Error, Result};
use log::{error, warn, info};

fn create_and_send_transaction(address: &str, amount: u64) -> Result<String> {
    info!("Creating transaction to {} for {} sats", address, amount);
    
    let builder = TxBuilder::new()
        .add_output(address.to_string(), amount)
        .set_fee_rate(10);
    
    // Dump with error handling
    let builder = match builder.dump() {
        Ok(b) => b,
        Err(Error::Io(e)) => {
            error!("Failed to create temp file: {}", e);
            return Err(Error::unexpected("Cannot create transaction file"));
        }
        Err(e) => return Err(e),
    };
    
    // Sign with retry logic
    let mut attempts = 3;
    let signed = loop {
        match builder.sign() {
            Ok(s) => break s,
            Err(Error::Timeout) if attempts > 0 => {
                warn!("Signing timed out, retrying... ({} attempts left)", attempts);
                attempts -= 1;
                continue;
            }
            Err(e) => {
                error!("Failed to sign transaction: {}", e);
                return Err(e);
            }
        }
    };
    
    // Broadcast
    match signed.broadcast() {
        Ok(txid) => {
            info!("Transaction broadcast successfully: {}", txid);
            Ok(txid)
        }
        Err(Error::Cli(stderr)) if stderr.contains("already in mempool") => {
            warn!("Transaction already in mempool");
            Ok("already_broadcast".to_string())
        }
        Err(e) => {
            error!("Failed to broadcast transaction: {}", e);
            Err(e)
        }
    }
}
```

## Testing Error Cases

When writing tests, ensure you test error paths:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_invalid_command() {
        let result = run_cli(&["invalid", "command"]);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Cli(_)));
    }
    
    #[test]
    fn test_json_parsing_error() {
        let invalid_json = "{invalid}";
        let result: Result<Transaction> = serde_json::from_str(invalid_json)
            .map_err(Into::into);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Json(_)));
    }
}
