# Asynchronous Command Runner Implementation

## Overview

This implementation provides asynchronous command execution capabilities for the Green API client using `tokio::process::Command`.

## Key Components

### 1. `run_cli_async` Function
- **Location**: `src/client.rs`
- **Purpose**: Asynchronous helper function to run green-cli commands
- **Features**:
  - Uses `tokio::process::Command` for async execution
  - Sets environment variables `-L` and `-T` by default
  - Captures stdout/stderr and returns stdout on success
  - Returns `Error::Cli` with stderr content on failure

### 2. `AsyncGreenClient` Struct
- **Location**: `src/client.rs`
- **Purpose**: Asynchronous Green API client
- **Features**:
  - `new()` method to create a new instance
  - `run_command()` method that delegates to `run_cli_async`
  - Ready for future extensions with configuration fields

## Usage Examples

### Direct Function Usage
```rust
use green_rs::client::run_cli_async;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_cli_async(&["getnetworks"]).await?;
    println!("Networks: {}", output);
    Ok(())
}
```

### Using AsyncGreenClient
```rust
use green_rs::client::AsyncGreenClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AsyncGreenClient::new();
    let output = client.run_command(&["getnetwork"]).await?;
    println!("Current network: {}", output);
    Ok(())
}
```

### Concurrent Command Execution
```rust
use green_rs::client::run_cli_async;
use futures::future::join_all;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let futures = vec![
        run_cli_async(&["--help"]),
        run_cli_async(&["getnetworks"]),
        run_cli_async(&["getnetwork"]),
    ];
    
    let results = join_all(futures).await;
    
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(output) => println!("Command {} succeeded: {} bytes", i, output.len()),
            Err(e) => println!("Command {} failed: {}", i, e),
        }
    }
    
    Ok(())
}
```

## Testing

The implementation includes comprehensive tests:

1. **Unit Tests** (`src/client.rs`):
   - `test_run_cli_async_with_invalid_command`
   - `test_async_green_client`

2. **Integration Tests** (`tests/async_integration_test.rs`):
   - `test_async_command_execution`
   - `test_async_client_command_execution`
   - `test_concurrent_async_commands`
   - `test_async_error_handling`

3. **Example** (`examples/async_cli_example.rs`):
   - Demonstrates all usage patterns
   - Shows concurrent execution capabilities

## Dependencies

The implementation requires:
- `tokio` with features: `["macros", "rt-multi-thread", "process", "time"]`
- `futures` for concurrent execution examples

## Error Handling

The async implementation maintains the same error handling as the synchronous version:
- Returns `Ok(String)` with stdout content on success
- Returns `Err(Error::Cli(stderr))` on non-zero exit codes
- Returns `Err(Error::Io(_))` if the command cannot be found or executed
