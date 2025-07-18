//! Integration tests for wallet API functionality
//! These tests mock green-cli with a shell script stub to test JSON deserialization and error paths

use assert_cmd::prelude::*;
use green_rs::api::{AsyncWalletExt, WalletExt};
use green_rs::types::{Balance, FeeEstimates};
use green_rs::{AsyncGreenClient, Error, GreenClient};
use predicates::prelude::*;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::process::Command;

/// Helper function to set up PATH to use our mock green-cli
fn setup_mock_path() -> String {
    let fixtures_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures");
    let current_path = env::var("PATH").unwrap_or_default();
    format!("{}:{}", fixtures_path.display(), current_path)
}

#[test]
fn test_mock_green_cli_balance_success() {
    // Test that our mock green-cli returns correct JSON for balance
    let mut cmd = Command::new("bash");
    cmd.env("PATH", setup_mock_path())
        .arg("-c")
        .arg("green-cli get balance --json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("btc"))
        .stdout(predicate::str::contains("100000"))
        .stdout(predicate::str::contains("liquid-btc"))
        .stdout(predicate::str::contains("50000"))
        .stdout(predicate::str::contains("usdt"))
        .stdout(predicate::str::contains("250000000"));
}

#[test]
fn test_mock_green_cli_balance_empty() {
    // Test empty balance scenario
    let mut cmd = Command::new("bash");
    cmd.env("PATH", setup_mock_path())
        .env("MOCK_EMPTY_BALANCE", "1")
        .arg("-c")
        .arg("green-cli get balance --json");

    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*\{\s*\}\s*$").unwrap());
}

#[test]
fn test_mock_green_cli_invalid_json() {
    // Test invalid JSON response
    let mut cmd = Command::new("bash");
    cmd.env("PATH", setup_mock_path())
        .env("MOCK_INVALID_JSON", "1")
        .arg("-c")
        .arg("green-cli get balance --json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("this is not valid json"));
}

#[test]
fn test_mock_green_cli_fee_estimates() {
    // Test fee estimates
    let mut cmd = Command::new("bash");
    cmd.env("PATH", setup_mock_path())
        .arg("-c")
        .arg("green-cli get fee-estimates --json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("fees"))
        .stdout(predicate::str::contains("\"1\": 50"))
        .stdout(predicate::str::contains("\"6\": 25"))
        .stdout(predicate::str::contains("\"144\": 10"));
}

#[test]
fn test_mock_green_cli_error() {
    // Test CLI error scenario
    let mut cmd = Command::new("bash");
    cmd.env("PATH", setup_mock_path())
        .env("MOCK_FAIL", "Wallet not initialized")
        .arg("-c")
        .arg("green-cli get balance --json");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: Wallet not initialized"));
}

#[test]
fn test_sync_wallet_get_balance_with_mock() {
    // Set up environment to use mock green-cli
    let original_path = env::var("PATH").unwrap_or_default();
    env::set_var("PATH", setup_mock_path());

    let client = GreenClient::new();
    let result = client.get_balance();

    // Restore original PATH
    env::set_var("PATH", original_path);

    // Verify successful deserialization
    match result {
        Ok(balance) => {
            assert_eq!(balance.get("btc"), Some(100000));
            assert_eq!(balance.get("liquid-btc"), Some(50000));
            assert_eq!(balance.get("usdt"), Some(250000000));
            assert_eq!(balance.asset_count(), 3);
        }
        Err(e) => panic!("Expected successful balance retrieval, got error: {:?}", e),
    }
}

#[test]
fn test_sync_wallet_get_balance_empty_with_mock() {
    // Set up environment to use mock green-cli with empty balance
    let original_path = env::var("PATH").unwrap_or_default();
    env::set_var("PATH", setup_mock_path());
    env::set_var("MOCK_EMPTY_BALANCE", "1");

    let client = GreenClient::new();
    let result = client.get_balance();

    // Restore environment
    env::set_var("PATH", original_path);
    env::remove_var("MOCK_EMPTY_BALANCE");

    // Verify empty balance
    match result {
        Ok(balance) => {
            assert!(balance.is_empty());
            assert_eq!(balance.asset_count(), 0);
        }
        Err(e) => panic!("Expected successful empty balance, got error: {:?}", e),
    }
}

#[test]
fn test_sync_wallet_json_error_with_mock() {
    // Set up environment to use mock green-cli with invalid JSON
    let original_path = env::var("PATH").unwrap_or_default();
    env::set_var("PATH", setup_mock_path());
    env::set_var("MOCK_INVALID_JSON", "1");

    let client = GreenClient::new();
    let result = client.get_balance();

    // Restore environment
    env::set_var("PATH", original_path);
    env::remove_var("MOCK_INVALID_JSON");

    // Verify JSON error
    match result {
        Err(Error::Json(_)) => {
            // Expected JSON parsing error
        }
        Ok(_) => panic!("Expected JSON error, got successful result"),
        Err(e) => panic!("Expected JSON error, got different error: {:?}", e),
    }
}

#[test]
fn test_sync_wallet_cli_error_with_mock() {
    // Set up environment to use mock green-cli with CLI error
    let original_path = env::var("PATH").unwrap_or_default();
    env::set_var("PATH", setup_mock_path());
    env::set_var("MOCK_FAIL", "Wallet locked");

    let client = GreenClient::new();
    let result = client.get_balance();

    // Restore environment
    env::set_var("PATH", original_path);
    env::remove_var("MOCK_FAIL");

    // Verify CLI error
    match result {
        Err(Error::Cli(msg)) => {
            assert!(msg.contains("Wallet locked"));
        }
        Ok(_) => panic!("Expected CLI error, got successful result"),
        Err(e) => panic!("Expected CLI error, got different error: {:?}", e),
    }
}

#[test]
fn test_sync_wallet_get_fee_estimates_with_mock() {
    // Set up environment to use mock green-cli
    let original_path = env::var("PATH").unwrap_or_default();
    env::set_var("PATH", setup_mock_path());

    let client = GreenClient::new();
    let result = client.get_fee_estimates();

    // Restore original PATH
    env::set_var("PATH", original_path);

    // Verify successful deserialization
    match result {
        Ok(fee_estimates) => {
            assert_eq!(fee_estimates.fees.len(), 4);
            assert_eq!(fee_estimates.fees.get(&1), Some(&50));
            assert_eq!(fee_estimates.fees.get(&6), Some(&25));
            assert_eq!(fee_estimates.fees.get(&12), Some(&15));
            assert_eq!(fee_estimates.fees.get(&144), Some(&10));
        }
        Err(e) => panic!("Expected successful fee estimates, got error: {:?}", e),
    }
}

#[tokio::test]
async fn test_async_wallet_get_balance_with_mock() {
    // Set up environment to use mock green-cli
    let original_path = env::var("PATH").unwrap_or_default();
    env::set_var("PATH", setup_mock_path());

    let client = AsyncGreenClient::new();
    let result = client.get_balance().await;

    // Restore original PATH
    env::set_var("PATH", original_path);

    // Verify successful deserialization
    match result {
        Ok(balance) => {
            assert_eq!(balance.get("btc"), Some(100000));
            assert_eq!(balance.get("liquid-btc"), Some(50000));
            assert_eq!(balance.get("usdt"), Some(250000000));
            assert_eq!(balance.asset_count(), 3);
        }
        Err(e) => panic!("Expected successful balance retrieval, got error: {:?}", e),
    }
}

#[tokio::test]
async fn test_async_wallet_get_fee_estimates_with_mock() {
    // Set up environment to use mock green-cli
    let original_path = env::var("PATH").unwrap_or_default();
    env::set_var("PATH", setup_mock_path());

    let client = AsyncGreenClient::new();
    let result = client.get_fee_estimates().await;

    // Restore original PATH
    env::set_var("PATH", original_path);

    // Verify successful deserialization
    match result {
        Ok(fee_estimates) => {
            assert_eq!(fee_estimates.fees.len(), 4);
            assert_eq!(fee_estimates.fees.get(&1), Some(&50));
            assert_eq!(fee_estimates.fees.get(&6), Some(&25));
            assert_eq!(fee_estimates.fees.get(&12), Some(&15));
            assert_eq!(fee_estimates.fees.get(&144), Some(&10));
        }
        Err(e) => panic!("Expected successful fee estimates, got error: {:?}", e),
    }
}

#[tokio::test]
async fn test_async_wallet_cli_error_with_mock() {
    // Set up environment to use mock green-cli with CLI error
    let original_path = env::var("PATH").unwrap_or_default();
    env::set_var("PATH", setup_mock_path());
    env::set_var("MOCK_FAIL", "Network error");

    let client = AsyncGreenClient::new();
    let result = client.get_balance().await;

    // Restore environment
    env::set_var("PATH", original_path);
    env::remove_var("MOCK_FAIL");

    // Verify CLI error
    match result {
        Err(Error::Cli(msg)) => {
            assert!(msg.contains("Network error"));
        }
        Ok(_) => panic!("Expected CLI error, got successful result"),
        Err(e) => panic!("Expected CLI error, got different error: {:?}", e),
    }
}

#[tokio::test]
async fn test_async_wallet_json_error_with_mock() {
    // Set up environment to use mock green-cli with invalid JSON
    let original_path = env::var("PATH").unwrap_or_default();
    env::set_var("PATH", setup_mock_path());
    env::set_var("MOCK_INVALID_JSON", "1");

    let client = AsyncGreenClient::new();
    let result = client.get_fee_estimates().await;

    // Restore environment
    env::set_var("PATH", original_path);
    env::remove_var("MOCK_INVALID_JSON");

    // Verify JSON error
    match result {
        Err(Error::Json(_)) => {
            // Expected JSON parsing error
        }
        Ok(_) => panic!("Expected JSON error, got successful result"),
        Err(e) => panic!("Expected JSON error, got different error: {:?}", e),
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
