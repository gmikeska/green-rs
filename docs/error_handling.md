# Error Handling in green-rs

## Overview

The `green-rs` library implements a comprehensive error handling system using the `thiserror` crate. The error module provides a unified `Error` enum that covers all possible failure modes in the library.

## Error Variants

The `Error` enum includes the following variants:

### 1. **Io(std::io::Error)**
- Handles I/O-related errors from file operations, network operations, etc.
- Automatically converts from `std::io::Error` using the `#[from]` attribute
- Example: File not found, permission denied, etc.

### 2. **Json(serde_json::Error)**
- Handles JSON serialization/deserialization errors
- Automatically converts from `serde_json::Error` using the `#[from]` attribute
- Example: Invalid JSON syntax, missing fields, type mismatches

### 3. **Cli(String)**
- Handles CLI command execution failures
- Contains stderr output when a command exits with non-zero status
- Created using the `Error::cli_error()` helper method
- Example: Command not found, permission denied, invalid arguments

### 4. **Timeout**
- Represents operations that exceed time limits
- No associated data
- Example: Network requests timing out, long-running operations

### 5. Additional Variants
- **Network(String)**: Network-related errors
- **InvalidResponse**: Invalid API responses
- **Unexpected(String)**: Catch-all for unexpected errors

## Result Type Alias

The library provides a convenient type alias:

```rust
pub type Result<T> = std::result::Result<T, Error>;
```

This allows functions to return `Result<T>` instead of `std::result::Result<T, Error>`.

## Usage Examples

### Basic Error Handling

```rust
use green_rs::{Error, Result};

fn read_config() -> Result<String> {
    // This will automatically convert std::io::Error to Error::Io
    std::fs::read_to_string("config.json")
        .map_err(Into::into)
}
```

### CLI Error Handling

```rust
use std::process::Command;
use green_rs::{Error, Result};

fn run_command(cmd: &str) -> Result<String> {
    let output = Command::new(cmd).output()?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::cli_error(format!(
            "Command failed: {}", stderr
        )));
    }
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
```

### Timeout Handling

```rust
use tokio::time::{timeout, Duration};
use green_rs::{Error, Result};

async fn with_timeout<T>(future: impl Future<Output = T>) -> Result<T> {
    timeout(Duration::from_secs(30), future)
        .await
        .map_err(|_| Error::Timeout)
}
```

## Error Display

All error variants implement the `Display` trait with descriptive messages:

- `Io`: "IO error: {details}"
- `Json`: "JSON error: {details}"
- `Cli`: "CLI error: {stderr_output}"
- `Timeout`: "Operation timed out"
- `Network`: "Network error: {details}"
- `InvalidResponse`: "Invalid response received"
- `Unexpected`: "Unexpected error: {details}"

## Testing

The error module includes comprehensive tests in `tests/error_test.rs` that verify:
- Automatic conversion from `std::io::Error` and `serde_json::Error`
- CLI error creation with stderr capture
- Timeout error handling
- Error display formatting
- Result type alias functionality

## Best Practices

1. Use the `?` operator for automatic error conversion when possible
2. Use `Error::cli_error()` for creating CLI errors with stderr output
3. Map timeout errors explicitly to `Error::Timeout`
4. Provide descriptive error messages for `Network` and `Unexpected` variants
5. Let the `#[from]` conversions handle `Io` and `Json` errors automatically
