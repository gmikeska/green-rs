//! Transaction builder example demonstrating the builder pattern
//!
//! This example shows how to:
//! - Use TxBuilder to construct transactions
//! - Add multiple outputs
//! - Set fee rates
//! - Build, sign, and send transactions
//!
//! Run with: cargo run --example tx_builder

use green_rs::api::address::AddressApi;
use green_rs::api::builder::GetReceiveAddressBuilder;
use green_rs::api::transaction::TxBuilder;
use green_rs::api::wallet::WalletExt;
use green_rs::client::GreenClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client for getting addresses
    let client = GreenClient::new();

    println!("Transaction Builder Example");
    println!("==========================\n");

    // Get current balance before transaction
    match client.get_balance() {
        Ok(balance) => {
            println!(
                "Current balance: {} BTC satoshis\n",
                balance.get("btc").unwrap_or(0)
            );
        }
        Err(e) => {
            eprintln!("Failed to get balance: {}", e);
        }
    }

    // Generate a destination address (in real usage, this would be external)
    let address_request = GetReceiveAddressBuilder::new().subaccount(0).build();

    let destination_address = match client.get_new_address(address_request) {
        Ok(addr) => {
            println!("Generated destination address: {}", addr.address);
            addr.address
        }
        Err(e) => {
            eprintln!("Failed to generate address: {}", e);
            return Err(e.into());
        }
    };

    // Example 1: Simple transaction with single output
    println!("\n1. Building simple transaction with single output");
    println!("------------------------------------------------");

    let simple_tx = TxBuilder::new()
        .add_output(destination_address.clone(), 10000) // 10,000 satoshis
        .set_fee_rate(5) // 5 sat/vB
        .set_subaccount(0);

    // Dump the transaction data
    match simple_tx.dump() {
        Ok(tx) => {
            println!("Transaction data created:");
            if let Some(json) = tx.to_json() {
                println!("{}", json);
            }
            if let Some(path) = tx.get_temp_path() {
                println!("Transaction saved to: {}", path);
            }
        }
        Err(e) => {
            eprintln!("Failed to dump transaction: {}", e);
        }
    }

    // Example 2: Complex transaction with multiple outputs
    println!("\n2. Building complex transaction with multiple outputs");
    println!("----------------------------------------------------");

    // Generate another address for demonstration
    let address_request2 = GetReceiveAddressBuilder::new()
        .subaccount(1) // Different subaccount
        .build();

    let second_address = match client.get_new_address(address_request2) {
        Ok(addr) => addr.address,
        Err(e) => {
            eprintln!("Failed to generate second address: {}", e);
            destination_address.clone() // Fallback to first address
        }
    };

    let complex_tx = TxBuilder::new()
        .add_output(destination_address, 25000) // 25,000 satoshis to first address
        .add_output(second_address, 15000) // 15,000 satoshis to second address
        .set_fee_rate(10) // Higher fee rate for faster confirmation
        .set_subaccount(0);

    match complex_tx.dump() {
        Ok(tx) => {
            println!("Complex transaction created:");
            if let Some(json) = tx.to_json() {
                println!("{}", json);
            }

            // In a real scenario, you would sign and send the transaction
            println!("\n3. Transaction workflow demonstration");
            println!("------------------------------------");

            // Sign the transaction (demonstration only)
            match tx.sign() {
                Ok(signed_tx) => {
                    println!("Transaction signed successfully!");

                    // Send/broadcast the transaction (demonstration only)
                    match signed_tx.send() {
                        Ok(txid) => {
                            println!("Transaction broadcast successfully!");
                            println!("Transaction ID: {}", txid);
                        }
                        Err(e) => {
                            println!("Note: Broadcasting not implemented in demo: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Note: Signing not implemented in demo: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to create complex transaction: {}", e);
        }
    }

    // Example 3: Using builder pattern with chained methods
    println!("\n4. Builder pattern with method chaining");
    println!("--------------------------------------");

    let chained_result = TxBuilder::new()
        .add_output("bc1qexampleaddress...".to_string(), 50000)
        .add_output("bc1qanotheraddress...".to_string(), 30000)
        .set_fee_rate(20)
        .set_subaccount(0)
        .dump()
        .and_then(|tx| {
            println!("Transaction built with method chaining");
            if let Some(json) = tx.to_json() {
                println!("Transaction JSON: {}", json);
            }
            Ok(tx)
        });

    match chained_result {
        Ok(_) => println!("Method chaining example completed successfully"),
        Err(e) => eprintln!("Method chaining example failed: {}", e),
    }

    Ok(())
}
