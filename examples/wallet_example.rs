//! Example demonstrating wallet API usage

use green_rs::api::{AsyncWalletExt, WalletExt};
use green_rs::{AsyncGreenClient, GreenClient, Result};

fn main() -> Result<()> {
    // Synchronous example
    sync_example()?;

    // Asynchronous example
    tokio::runtime::Runtime::new()?.block_on(async_example())?;

    Ok(())
}

fn sync_example() -> Result<()> {
    println!("=== Synchronous Wallet API Example ===");

    // Create a synchronous client
    let client = GreenClient::new();

    // Get wallet balance
    match client.get_balance() {
        Ok(balance) => {
            println!("Wallet balance:");
            for (asset_id, amount) in &balance.0 {
                println!("  Asset {}: {} sats", asset_id, amount);
            }
        }
        Err(e) => {
            println!("Failed to get balance: {}", e);
        }
    }

    // Get fee estimates
    match client.get_fee_estimates() {
        Ok(fee_estimates) => {
            println!("\nFee estimates (sats/vbyte):");
            let mut blocks: Vec<_> = fee_estimates.fees.keys().collect();
            blocks.sort();
            for blocks in blocks {
                if let Some(fee_rate) = fee_estimates.fees.get(blocks) {
                    println!("  {} blocks: {} sats/vbyte", blocks, fee_rate);
                }
            }
        }
        Err(e) => {
            println!("Failed to get fee estimates: {}", e);
        }
    }

    Ok(())
}

async fn async_example() -> Result<()> {
    println!("\n=== Asynchronous Wallet API Example ===");

    // Create an asynchronous client
    let client = AsyncGreenClient::new();

    // Get wallet balance
    match client.get_balance().await {
        Ok(balance) => {
            println!("Wallet balance:");
            for (asset_id, amount) in &balance.0 {
                println!("  Asset {}: {} sats", asset_id, amount);
            }
        }
        Err(e) => {
            println!("Failed to get balance: {}", e);
        }
    }

    // Get fee estimates
    match client.get_fee_estimates().await {
        Ok(fee_estimates) => {
            println!("\nFee estimates (sats/vbyte):");
            let mut blocks: Vec<_> = fee_estimates.fees.keys().collect();
            blocks.sort();
            for blocks in blocks {
                if let Some(fee_rate) = fee_estimates.fees.get(blocks) {
                    println!("  {} blocks: {} sats/vbyte", blocks, fee_rate);
                }
            }
        }
        Err(e) => {
            println!("Failed to get fee estimates: {}", e);
        }
    }

    Ok(())
}
