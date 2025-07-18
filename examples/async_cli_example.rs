//! Example demonstrating asynchronous command execution with AsyncGreenClient

use green_rs::client::{run_cli_async, AsyncGreenClient};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Async Green CLI Example ===\n");

    // Example 1: Using the standalone async function
    println!("Example 1: Using run_cli_async directly");
    match run_cli_async(&["--help"]).await {
        Ok(output) => {
            println!("Successfully executed green-cli --help");
            println!("Output length: {} bytes", output.len());
        }
        Err(e) => {
            println!("Error executing command: {}", e);
        }
    }

    println!("\n---\n");

    // Example 2: Using AsyncGreenClient
    println!("Example 2: Using AsyncGreenClient");
    let client = AsyncGreenClient::new();

    // Try to get network information
    match client.run_command(&["getnetworks"]).await {
        Ok(output) => {
            println!("Networks info length: {} bytes", output.len());
        }
        Err(e) => {
            println!("Could not get networks: {}", e);
        }
    }

    println!("\n---\n");

    // Example 3: Running multiple commands concurrently
    println!("Example 3: Running multiple commands concurrently");

    let futures = vec![
        run_cli_async(&["--help"]),
        run_cli_async(&["getnetworks"]),
        run_cli_async(&["getnetwork"]),
    ];

    let results = futures::future::join_all(futures).await;

    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(output) => {
                println!(
                    "Command {} succeeded with {} bytes of output",
                    i + 1,
                    output.len()
                );
            }
            Err(e) => {
                println!("Command {} failed: {}", i + 1, e);
            }
        }
    }

    Ok(())
}
