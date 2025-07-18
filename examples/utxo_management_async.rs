//! Async example demonstrating UTXO management API usage

use green_rs::api::utxo::AsyncUtxoApi;
use green_rs::client::AsyncGreenClient;
use green_rs::types::{GetUnspentOutputsParams, UtxoSortBy};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new async Green client
    let client = AsyncGreenClient::new();

    // Example: Get UTXOs with complex filtering
    println!("=== Getting filtered UTXOs (async) ===");

    let params = GetUnspentOutputsParams {
        min_confs: Some(3),
        max_confs: Some(100),
        include_frozen: Some(false),
        sort_by: Some(UtxoSortBy::ValueDesc),
        min_value: Some(5000),
        ..Default::default()
    };

    match client.get_unspent_outputs(params).await {
        Ok(utxos_by_asset) => {
            for (asset_id, utxos) in &utxos_by_asset {
                println!("\nAsset: {}", asset_id);
                println!("Found {} UTXOs matching criteria", utxos.len());

                // Calculate statistics
                let total: u64 = utxos.iter().map(|u| u.satoshi).sum();
                let avg = if utxos.is_empty() {
                    0
                } else {
                    total / utxos.len() as u64
                };
                let max_value = utxos.iter().map(|u| u.satoshi).max().unwrap_or(0);
                let min_value = utxos.iter().map(|u| u.satoshi).min().unwrap_or(0);

                println!("Total value: {} sats", total);
                println!("Average UTXO: {} sats", avg);
                println!("Largest UTXO: {} sats", max_value);
                println!("Smallest UTXO: {} sats", min_value);

                // Show distribution by confirmation count
                let mut conf_distribution = std::collections::HashMap::new();
                for utxo in utxos {
                    if let Some(confs) = utxo.confirmations {
                        *conf_distribution.entry(confs).or_insert(0) += 1;
                    }
                }

                if !conf_distribution.is_empty() {
                    println!("\nConfirmation distribution:");
                    let mut confs: Vec<_> = conf_distribution.keys().collect();
                    confs.sort();
                    for conf in confs {
                        println!(
                            "  {} confirmations: {} UTXOs",
                            conf, conf_distribution[conf]
                        );
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error getting UTXOs: {}", e);
        }
    }

    // Example: Get UTXOs for Liquid assets
    println!("\n\n=== Getting Liquid asset UTXOs ===");

    let params = GetUnspentOutputsParams {
        confidential_only: Some(true),
        ..Default::default()
    };

    match client.get_unspent_outputs(params).await {
        Ok(utxos_by_asset) => {
            if utxos_by_asset.is_empty() {
                println!("No Liquid UTXOs found");
            } else {
                for (asset_id, utxos) in &utxos_by_asset {
                    if asset_id != "btc" {
                        // Skip Bitcoin, show only Liquid assets
                        println!("\nLiquid Asset ID: {}", asset_id);
                        println!("Number of UTXOs: {}", utxos.len());
                        let total: u64 = utxos.iter().map(|u| u.satoshi).sum();
                        println!("Total amount: {} units", total);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error getting Liquid UTXOs: {}", e);
        }
    }

    Ok(())
}
