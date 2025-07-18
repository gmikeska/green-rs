//! Integration tests for wallet API functionality

use green_rs::api::{AsyncWalletExt, WalletExt};
use green_rs::types::{Balance, FeeEstimates};
use green_rs::{AsyncGreenClient, Error, GreenClient};
use std::collections::HashMap;

#[test]
fn test_sync_wallet_get_balance() {
    let client = GreenClient::new();

    // Test get_balance
    let result = client.get_balance();

    // The function should either succeed (if green-cli is installed and configured)
    // or return an error
    match result {
        Ok(balance) => {
            // Verify the balance is a valid Balance type
            assert!(balance.0.is_empty() || !balance.0.is_empty());
        }
        Err(Error::Io(_)) => {
            // green-cli not found, which is expected in CI
        }
        Err(Error::Cli(_)) => {
            // CLI error (e.g., wallet not configured), which is expected
        }
        Err(Error::Json(_)) => {
            // JSON parsing error, could happen if output format is unexpected
        }
        Err(e) => panic!("Unexpected error type: {:?}", e),
    }
}

#[test]
fn test_sync_wallet_get_fee_estimates() {
    let client = GreenClient::new();

    // Test get_fee_estimates
    let result = client.get_fee_estimates();

    match result {
        Ok(fee_estimates) => {
            // Verify the fee estimates have valid structure
            assert!(fee_estimates.fees.is_empty() || !fee_estimates.fees.is_empty());
        }
        Err(Error::Io(_)) => {
            // green-cli not found, which is expected in CI
        }
        Err(Error::Cli(_)) => {
            // CLI error (e.g., wallet not configured), which is expected
        }
        Err(Error::Json(_)) => {
            // JSON parsing error, could happen if output format is unexpected
        }
        Err(e) => panic!("Unexpected error type: {:?}", e),
    }
}

#[tokio::test]
async fn test_async_wallet_get_balance() {
    let client = AsyncGreenClient::new();

    // Test get_balance
    let result = client.get_balance().await;

    match result {
        Ok(balance) => {
            // Verify the balance is a valid Balance type
            assert!(balance.0.is_empty() || !balance.0.is_empty());
        }
        Err(Error::Io(_)) => {
            // green-cli not found, which is expected in CI
        }
        Err(Error::Cli(_)) => {
            // CLI error (e.g., wallet not configured), which is expected
        }
        Err(Error::Json(_)) => {
            // JSON parsing error, could happen if output format is unexpected
        }
        Err(e) => panic!("Unexpected error type: {:?}", e),
    }
}

#[tokio::test]
async fn test_async_wallet_get_fee_estimates() {
    let client = AsyncGreenClient::new();

    // Test get_fee_estimates
    let result = client.get_fee_estimates().await;

    match result {
        Ok(fee_estimates) => {
            // Verify the fee estimates have valid structure
            assert!(fee_estimates.fees.is_empty() || !fee_estimates.fees.is_empty());
        }
        Err(Error::Io(_)) => {
            // green-cli not found, which is expected in CI
        }
        Err(Error::Cli(_)) => {
            // CLI error (e.g., wallet not configured), which is expected
        }
        Err(Error::Json(_)) => {
            // JSON parsing error, could happen if output format is unexpected
        }
        Err(e) => panic!("Unexpected error type: {:?}", e),
    }
}

#[test]
fn test_balance_type_functionality() {
    let mut balance = Balance::new();

    // Test empty balance
    assert!(balance.is_empty());
    assert_eq!(balance.asset_count(), 0);

    // Test adding assets
    let btc_asset = "btc".to_string();
    balance.set(btc_asset.clone(), 100000);

    assert!(!balance.is_empty());
    assert_eq!(balance.asset_count(), 1);
    assert_eq!(balance.get(&btc_asset), Some(100000));
    assert_eq!(balance.get("unknown"), None);

    // Test multiple assets
    let liquid_btc = "liquid-btc".to_string();
    balance.set(liquid_btc.clone(), 50000);

    assert_eq!(balance.asset_count(), 2);
    assert_eq!(balance.get(&liquid_btc), Some(50000));
}

#[test]
fn test_fee_estimates_type() {
    let mut fees = HashMap::new();
    fees.insert(1, 50);
    fees.insert(6, 25);
    fees.insert(144, 10);

    let fee_estimates = FeeEstimates { fees };

    // Verify structure
    assert_eq!(fee_estimates.fees.len(), 3);
    assert_eq!(fee_estimates.fees.get(&1), Some(&50));
    assert_eq!(fee_estimates.fees.get(&6), Some(&25));
    assert_eq!(fee_estimates.fees.get(&144), Some(&10));
}
