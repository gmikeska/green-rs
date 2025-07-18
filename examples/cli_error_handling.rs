//! Example demonstrating CLI error handling with stderr capture

use green_rs::{Error, Result};
use std::process::Command;

/// Execute a CLI command and handle errors properly
fn execute_cli_command(program: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|e| Error::Io(e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let exit_code = output.status.code().unwrap_or(-1);

        return Err(Error::cli_error(format!(
            "Command '{}' failed with exit code {}: {}",
            program,
            exit_code,
            stderr.trim()
        )));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Example with timeout handling using tokio
#[tokio::main]
async fn main() -> Result<()> {
    println!("CLI Error Handling Example\n");

    // Example 1: Successful command
    println!("1. Running successful command:");
    match execute_cli_command("echo", &["Hello, World!"]) {
        Ok(output) => println!("   Success: {}", output.trim()),
        Err(e) => println!("   Error: {}", e),
    }

    // Example 2: Command that fails (invalid command)
    println!("\n2. Running failing command:");
    match execute_cli_command("false", &[]) {
        Ok(_) => println!("   Unexpected success"),
        Err(e) => println!("   Expected error: {}", e),
    }

    // Example 3: Command not found
    println!("\n3. Running non-existent command:");
    match execute_cli_command("nonexistent_command", &[]) {
        Ok(_) => println!("   Unexpected success"),
        Err(e) => println!("   Expected error: {}", e),
    }

    // Example 4: Timeout simulation
    println!("\n4. Simulating timeout:");
    use tokio::time::{timeout, Duration};

    let long_running = async {
        tokio::time::sleep(Duration::from_secs(5)).await;
        Ok::<String, Error>("This would take too long".to_string())
    };

    match timeout(Duration::from_secs(1), long_running).await {
        Ok(Ok(result)) => println!("   Result: {}", result),
        Ok(Err(e)) => println!("   Error: {}", e),
        Err(_) => println!("   Error: {}", Error::Timeout),
    }

    Ok(())
}
