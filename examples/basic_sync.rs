//! Basic synchronous example demonstrating green-rs usage
//!
//! This example shows how to:
//! - Create a synchronous client
//! - Get wallet balance
//! - Generate a new receive address

use green_rs::api::address::AddressApi;
use green_rs::api::builder::GetReceiveAddressBuilder;
use green_rs::api::wallet::WalletExt;
use green_rs::client::GreenClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a synchronous Green client
    let client = GreenClient::new();

    // Get wallet balance
    println!("Getting wallet balance...");
    match client.get_balance() {
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

    // Generate a new receive address
    println!("\nGenerating new receive address...");

    // Using the builder pattern to create the request
    let address_request = GetReceiveAddressBuilder::new()
        .subaccount(0) // Use the main account
        .build();

    match client.get_new_address(address_request) {
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

    // Get fee estimates
    println!("\nGetting fee estimates...");
    match client.get_fee_estimates() {
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

    Ok(())
}
