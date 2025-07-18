//! Example demonstrating UTXO management API usage

use green_rs::api::utxo::UtxoApi;
use green_rs::client::GreenClient;
use green_rs::types::{GetUnspentOutputsParams, UtxoSortBy};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new Green client
    let client = GreenClient::new();

    // Example 1: Get all unspent outputs with default parameters
    println!("=== Getting all UTXOs ===");
    let params = GetUnspentOutputsParams::default();
    match client.get_unspent_outputs(params) {
        Ok(utxos_by_asset) => {
            print_utxos(&utxos_by_asset);
        }
        Err(e) => {
            eprintln!("Error getting UTXOs: {}", e);
        }
    }

    // Example 2: Get UTXOs with minimum confirmations
    println!("\n=== Getting UTXOs with at least 6 confirmations ===");
    let params = GetUnspentOutputsParams {
        min_confs: Some(6),
        ..Default::default()
    };
    match client.get_unspent_outputs(params) {
        Ok(utxos_by_asset) => {
            print_utxos(&utxos_by_asset);
        }
        Err(e) => {
            eprintln!("Error getting UTXOs: {}", e);
        }
    }

    // Example 3: Get UTXOs sorted by value (descending)
    println!("\n=== Getting UTXOs sorted by value (largest first) ===");
    let params = GetUnspentOutputsParams {
        sort_by: Some(UtxoSortBy::ValueDesc),
        ..Default::default()
    };
    match client.get_unspent_outputs(params) {
        Ok(utxos_by_asset) => {
            print_utxos(&utxos_by_asset);
        }
        Err(e) => {
            eprintln!("Error getting UTXOs: {}", e);
        }
    }

    // Example 4: Get UTXOs for a specific subaccount
    println!("\n=== Getting UTXOs for subaccount 0 ===");
    let params = GetUnspentOutputsParams {
        subaccount: Some(0),
        ..Default::default()
    };
    match client.get_unspent_outputs(params) {
        Ok(utxos_by_asset) => {
            print_utxos(&utxos_by_asset);
        }
        Err(e) => {
            eprintln!("Error getting UTXOs: {}", e);
        }
    }

    // Example 5: Advanced filtering
    println!("\n=== Advanced filtering: 6+ confs, exclude frozen, value > 10000 sats ===");
    let params = GetUnspentOutputsParams {
        min_confs: Some(6),
        include_frozen: Some(false),
        min_value: Some(10000),
        sort_by: Some(UtxoSortBy::ValueDesc),
        ..Default::default()
    };
    match client.get_unspent_outputs(params) {
        Ok(utxos_by_asset) => {
            print_utxos(&utxos_by_asset);
        }
        Err(e) => {
            eprintln!("Error getting UTXOs: {}", e);
        }
    }

    Ok(())
}

fn print_utxos(utxos_by_asset: &HashMap<String, Vec<green_rs::types::UnspentOutput>>) {
    if utxos_by_asset.is_empty() {
        println!("No UTXOs found");
        return;
    }

    for (asset_id, utxos) in utxos_by_asset {
        println!("Asset: {}", asset_id);
        println!("Number of UTXOs: {}", utxos.len());
        
        let total_value: u64 = utxos.iter().map(|u| u.satoshi).sum();
        println!("Total value: {} sats", total_value);
        
        // Print first few UTXOs as examples
        for (i, utxo) in utxos.iter().take(3).enumerate() {
            println!("  UTXO {}: {}:{} - {} sats", 
                i + 1, 
                &utxo.txhash[..8], // First 8 chars of txhash
                utxo.vout,
                utxo.satoshi
            );
            if let Some(confs) = utxo.confirmations {
                println!("    Confirmations: {}", confs);
            }
            if utxo.is_frozen {
                println!("    Status: FROZEN");
            }
        }
        
        if utxos.len() > 3 {
            println!("  ... and {} more", utxos.len() - 3);
        }
    }
}
