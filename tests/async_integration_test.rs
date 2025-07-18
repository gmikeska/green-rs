//! Integration tests for asynchronous command execution

use green_rs::client::{run_cli_async, AsyncGreenClient};
use green_rs::Error;

#[tokio::test]
async fn test_async_command_execution() {
    // Test basic command execution
    let result = run_cli_async(&["--help"]).await;

    // We expect either success (if green-cli is installed)
    // or an IO error (if green-cli is not found)
    match result {
        Ok(output) => {
            assert!(!output.is_empty());
            println!("Command output: {} bytes", output.len());
        }
        Err(Error::Io(_)) => {
            // This is expected if green-cli is not installed
            println!("green-cli not found, which is expected in test environment");
        }
        Err(e) => {
            panic!("Unexpected error type: {}", e);
        }
    }
}

#[tokio::test]
async fn test_async_client_command_execution() {
    let client = AsyncGreenClient::new();

    // Test running a command through the client
    let result = client.run_command(&["--help"]).await;

    match result {
        Ok(output) => {
            assert!(!output.is_empty());
            println!("Version info retrieved: {} bytes", output.len());
        }
        Err(Error::Io(_)) => {
            println!("green-cli not found, which is expected in test environment");
        }
        Err(e) => {
            panic!("Unexpected error type: {}", e);
        }
    }
}

#[tokio::test]
async fn test_concurrent_async_commands() {
    use futures::future::join_all;

    // Create multiple async tasks
    let tasks = vec![
        run_cli_async(&["--help"]),
        run_cli_async(&["getnetworks"]),
        run_cli_async(&["getnetwork"]),
    ];

    // Execute all tasks concurrently
    let results = join_all(tasks).await;

    // Check that all tasks completed
    assert_eq!(results.len(), 3);

    // Verify each result
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(output) => {
                println!("Task {} succeeded with {} bytes", i, output.len());
            }
            Err(Error::Io(_)) => {
                println!(
                    "Task {} failed with IO error (expected if green-cli not installed)",
                    i
                );
            }
            Err(e) => {
                panic!("Task {} failed with unexpected error: {}", i, e);
            }
        }
    }
}

#[tokio::test]
async fn test_async_error_handling() {
    // Test with an invalid command that should fail
    let result = run_cli_async(&["invalid-command-that-does-not-exist"]).await;

    match result {
        Ok(_) => {
            // If green-cli is installed, it might still return output
            // even for invalid commands (like help text)
            println!("Command unexpectedly succeeded");
        }
        Err(Error::Cli(stderr)) => {
            println!("Got expected CLI error: {}", stderr);
        }
        Err(Error::Io(_)) => {
            println!("green-cli not found");
        }
        Err(e) => {
            panic!("Unexpected error type: {}", e);
        }
    }
}
