//! Basic asynchronous example demonstrating green-rs usage with Tokio
//!
//! This example shows how to:
//! - Create an asynchronous client
//! - Get wallet balance asynchronously
//! - Generate a new receive address asynchronously
//!
//! Run with: cargo run --example basic_async

use green_rs::api::address::AsyncAddressApi;
use green_rs::api::builder::GetReceiveAddressBuilder;
use green_rs::api::wallet::AsyncWalletExt;
use green_rs::client::AsyncGreenClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an asynchronous Green client
    let client = AsyncGreenClient::new();

    // Get wallet balance asynchronously
    println!("Getting wallet balance...");
    match client.get_balance().await {
        Ok(balance) => {
            println!("Wallet balance retrieved successfully:");
            println!("  BTC: {} satoshis", balance.get("btc").unwrap_or(0));

            // Display balances for all assets
            for (asset_id, amount) in &balance.0 {
                println!("  Asset {}: {} satoshis", asset_id, amount);
            }
        }
        Err(e) => {
            eprintln!("Failed to get balance: {}", e);
        }
    }

    // Generate a new receive address asynchronously
    println!("\nGenerating new receive address...");

    // Using the builder pattern to create the request
    let address_request = GetReceiveAddressBuilder::new()
        .subaccount(0) // Use the main account
        .build();

    match client.get_new_address(address_request).await {
        Ok(address) => {
            println!("New address generated successfully:");
            println!("  Address: {}", address.address);
            println!("  Pointer: {}", address.pointer);
            if !address.address_type.is_empty() {
                println!("    Type: {}", address.address_type);
            }
        }
        Err(e) => {
            eprintln!("Failed to generate new address: {}", e);
        }
    }

    // Get fee estimates asynchronously
    println!("\nGetting fee estimates...");
    match client.get_fee_estimates().await {
        Ok(fee_estimates) => {
            println!("Fee estimates (sat/vB):");
            for (blocks, fee_rate) in &fee_estimates.fees {
                println!("  {} blocks: {} sat/vB", blocks, fee_rate);
            }
        }
        Err(e) => {
            eprintln!("Failed to get fee estimates: {}", e);
        }
    }

    // Example of concurrent operations using Tokio
    println!("\nPerforming concurrent operations...");

    // Run multiple async operations concurrently
    let (balance_result, fee_result) =
        tokio::join!(client.get_balance(), client.get_fee_estimates());

    if let (Ok(balance), Ok(fees)) = (balance_result, fee_result) {
        println!("Concurrent operations completed successfully!");
        println!("  Balance: {} BTC satoshis", balance.get("btc").unwrap_or(0));
        println!(
            "  Fastest fee: {} sat/vB",
            fees.fees.get(&1).copied().unwrap_or(0)
        );
    }

    Ok(())
}
