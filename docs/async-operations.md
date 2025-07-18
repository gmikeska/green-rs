# Async Operations

Green.rs provides full async support for all operations, making it suitable for high-performance applications and concurrent workloads.

## Setup

To use async operations, you need an async runtime. We recommend Tokio:

```toml
[dependencies]
green-rs = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Async Client

The `AsyncGreenClient` provides async versions of all operations:

```rust
use green_rs::AsyncGreenClient;

#[tokio::main]
async fn main() -> Result<()> {
    let client = AsyncGreenClient::new();
    
    // All operations are async
    let balance = client.get_balance(None, None).await?;
    println!("Balance: {:?}", balance);
    
    Ok(())
}
```

## Async Transaction API

All transaction operations have async equivalents:

```rust
#[async_trait]
pub trait AsyncTransactionApi {
    async fn create_transaction(&self, request: CreateTransactionRequest) -> Result<CreateTransactionResult>;
    async fn send_to_address(&self, address: &str, amount: Satoshis, asset_id: Option<AssetId>) -> Result<Transaction>;
    async fn get_transactions(&self, subaccount: Option<u32>, first: Option<u32>, count: Option<u32>) -> Result<TransactionList>;
    async fn get_transaction_details(&self, txid: &str) -> Result<Transaction>;
}
```

## Concurrent Operations

Async operations can be run concurrently for better performance:

```rust
use futures::future::join_all;

async fn check_multiple_addresses(client: &AsyncGreenClient, addresses: Vec<String>) -> Result<Vec<bool>> {
    let futures = addresses.into_iter()
        .map(|addr| client.validate_address(&addr))
        .collect::<Vec<_>>();
    
    let results = join_all(futures).await;
    results.into_iter().collect()
}
```

## Async CLI Execution

Execute CLI commands asynchronously:

```rust
use green_rs::client::run_cli_async;

async fn get_network_info() -> Result<String> {
    run_cli_async(&["network", "info"]).await
}
```

## Error Handling in Async Context

Async operations use the same error types as sync operations:

```rust
async fn handle_transaction() -> Result<()> {
    let client = AsyncGreenClient::new();
    
    match client.get_balance(None, None).await {
        Ok(balance) => {
            println!("Balance: {:?}", balance);
            Ok(())
        }
        Err(Error::Timeout) => {
            eprintln!("Operation timed out");
            Err(Error::Timeout)
        }
        Err(e) => {
            eprintln!("Unexpected error: {}", e);
            Err(e)
        }
    }
}
```

## Timeout Configuration

Configure timeouts for async operations:

```rust
use std::time::Duration;

let client = AsyncGreenClient::with_timeout(Duration::from_secs(30));
```

## Best Practices

1. **Use Tokio runtime**: It's the most mature and feature-complete async runtime
2. **Handle timeouts**: Always set reasonable timeouts for network operations
3. **Limit concurrency**: Don't spawn unlimited concurrent operations
4. **Error propagation**: Use `?` operator for clean error handling
5. **Resource cleanup**: Ensure resources are properly cleaned up in async contexts

## Example: Async Transaction Builder

While the current `TxBuilder` is synchronous, here's how you might use it in an async context:

```rust
use tokio::task;

async fn create_transaction_async() -> Result<String> {
    // Run the blocking operation in a Tokio blocking thread
    task::spawn_blocking(|| {
        TxBuilder::new()
            .add_output("bc1q...".to_string(), 100000)
            .set_fee_rate(10)
            .dump()?
            .sign()?
            .broadcast()
    })
    .await
    .map_err(|e| Error::unexpected(format!("Task failed: {}", e)))?
}
```

## Performance Considerations

- Async operations have a small overhead compared to sync operations
- Benefits are most apparent when dealing with multiple concurrent operations
- For single operations, sync API might be simpler and equally performant
- Use async when you need to handle multiple requests or integrate with async frameworks
