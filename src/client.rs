//! Green API client implementations

use crate::api::subaccount::SubaccountExt;
use crate::api::wallet::{AsyncWalletExt, WalletExt};
use crate::error::{Error, Result};
use crate::types::common::Pointer;
use crate::types::subaccount::{
    CreateSubaccountParams, Subaccount, SubaccountList, UpdateSubaccountParams,
};
use crate::types::{Balance, FeeEstimates};
use std::process::Command;
use tokio::process::Command as TokioCommand;

/// Synchronous Green API client
pub struct GreenClient {
    // TODO: Add client configuration fields
}

impl GreenClient {
    /// Create a new synchronous Green API client
    pub fn new() -> Self {
        Self {}
    }

    /// Run a green-cli command
    ///
    /// This is a convenience method that delegates to `run_cli`
    ///
    /// # Arguments
    ///
    /// * `args` - Command line arguments to pass to green-cli
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The stdout output on successful execution
    /// * `Err(Error::Cli)` - Error containing stderr on non-zero exit code
    pub fn run_command(&self, args: &[&str]) -> Result<String> {
        run_cli(args)
    }
}

impl Default for GreenClient {
    fn default() -> Self {
        Self::new()
    }
}

impl WalletExt for GreenClient {
    fn get_balance(&self) -> Result<Balance> {
        let output = self.run_command(&["get", "balance", "--json"])?;
        let balance: Balance = serde_json::from_str(&output)?;
        Ok(balance)
    }

    fn get_fee_estimates(&self) -> Result<FeeEstimates> {
        let output = self.run_command(&["get", "fee-estimates", "--json"])?;
        let fee_estimates: FeeEstimates = serde_json::from_str(&output)?;
        Ok(fee_estimates)
    }
}

impl crate::api::utxo::UtxoApi for GreenClient {
    fn get_unspent_outputs(
        &self,
        params: crate::types::GetUnspentOutputsParams,
    ) -> crate::Result<std::collections::HashMap<crate::types::AssetId, Vec<crate::types::UnspentOutput>>> {
        let params_json = serde_json::to_string(&params)?;
        let output = self.run_command(&[
            "get",
            "utxos",
            "--params",
            &params_json,
            "--json",
        ])?;
        let utxos: Vec<crate::types::UnspentOutput> = serde_json::from_str(&output)?;
        let mut grouped_utxos: std::collections::HashMap<crate::types::AssetId, Vec<crate::types::UnspentOutput>> = std::collections::HashMap::new();

        // Group UTXOs by asset ID
        // For Bitcoin mainnet, asset_id is None, so we use "btc" as the default key
        for utxo in utxos {
            let asset_id = utxo.asset_id.clone().unwrap_or_else(|| "btc".to_string());
            grouped_utxos.entry(asset_id).or_insert_with(Vec::new).push(utxo);
        }
        
        // Apply sorting if specified
        if let Some(sort_by) = params.sort_by {
            for utxos in grouped_utxos.values_mut() {
                match sort_by {
                    crate::types::UtxoSortBy::Value => {
                        utxos.sort_by_key(|u| u.satoshi);
                    }
                    crate::types::UtxoSortBy::ValueDesc => {
                        utxos.sort_by_key(|u| std::cmp::Reverse(u.satoshi));
                    }
                    crate::types::UtxoSortBy::Confirmations => {
                        utxos.sort_by_key(|u| u.confirmations);
                    }
                    crate::types::UtxoSortBy::ConfirmationsDesc => {
                        utxos.sort_by_key(|u| std::cmp::Reverse(u.confirmations));
                    }
                    crate::types::UtxoSortBy::Age => {
                        utxos.sort_by_key(|u| u.block_height);
                    }
                    crate::types::UtxoSortBy::AgeDesc => {
                        utxos.sort_by_key(|u| std::cmp::Reverse(u.block_height));
                    }
                }
            }
        }
        
        Ok(grouped_utxos)
    }
}

#[async_trait::async_trait]
impl crate::api::utxo::AsyncUtxoApi for AsyncGreenClient {
    async fn get_unspent_outputs(
        &self,
        params: crate::types::GetUnspentOutputsParams,
    ) -> crate::Result<std::collections::HashMap<crate::types::AssetId, Vec<crate::types::UnspentOutput>>> {
        let params_json = serde_json::to_string(&params)?;
        let output = self.run_command(&[
            "get",
            "utxos",
            "--params",
            &params_json,
            "--json",
        ]).await?;
        let utxos: Vec<crate::types::UnspentOutput> = serde_json::from_str(&output)?;
        let mut grouped_utxos: std::collections::HashMap<crate::types::AssetId, Vec<crate::types::UnspentOutput>> = std::collections::HashMap::new();

        // Group UTXOs by asset ID
        // For Bitcoin mainnet, asset_id is None, so we use "btc" as the default key
        for utxo in utxos {
            let asset_id = utxo.asset_id.clone().unwrap_or_else(|| "btc".to_string());
            grouped_utxos.entry(asset_id).or_insert_with(Vec::new).push(utxo);
        }
        
        // Apply sorting if specified
        if let Some(sort_by) = params.sort_by {
            for utxos in grouped_utxos.values_mut() {
                match sort_by {
                    crate::types::UtxoSortBy::Value => {
                        utxos.sort_by_key(|u| u.satoshi);
                    }
                    crate::types::UtxoSortBy::ValueDesc => {
                        utxos.sort_by_key(|u| std::cmp::Reverse(u.satoshi));
                    }
                    crate::types::UtxoSortBy::Confirmations => {
                        utxos.sort_by_key(|u| u.confirmations);
                    }
                    crate::types::UtxoSortBy::ConfirmationsDesc => {
                        utxos.sort_by_key(|u| std::cmp::Reverse(u.confirmations));
                    }
                    crate::types::UtxoSortBy::Age => {
                        utxos.sort_by_key(|u| u.block_height);
                    }
                    crate::types::UtxoSortBy::AgeDesc => {
                        utxos.sort_by_key(|u| std::cmp::Reverse(u.block_height));
                    }
                }
            }
        }
        
        Ok(grouped_utxos)
    }
}

impl SubaccountExt for GreenClient {
    fn get_subaccounts(&self) -> Result<Vec<Subaccount>> {
        let output = self.run_command(&["get", "subaccounts", "--json"])?;
        let list: SubaccountList = serde_json::from_str(&output)?;
        Ok(list.subaccounts)
    }

    fn get_subaccount(&self, pointer: Pointer) -> Result<Subaccount> {
        let output = self.run_command(&[
            "get",
            "subaccount",
            "--subaccount",
            &pointer.to_string(),
            "--json",
        ])?;
        let subaccount: Subaccount = serde_json::from_str(&output)?;
        Ok(subaccount)
    }

    fn create_subaccount(&self, params: CreateSubaccountParams) -> Result<Subaccount> {
        let params_json = serde_json::to_string(&params)?;
        let output = self.run_command(&[
            "create",
            "subaccount",
            "--params",
            &params_json,
            "--json",
        ])?;
        let subaccount: Subaccount = serde_json::from_str(&output)?;
        Ok(subaccount)
    }

    fn update_subaccount(
        &self,
        pointer: Pointer,
        params: UpdateSubaccountParams,
    ) -> Result<Subaccount> {
        let params_json = serde_json::to_string(&params)?;
        let output = self.run_command(&[
            "update",
            "subaccount",
            "--subaccount",
            &pointer.to_string(),
            "--params",
            &params_json,
            "--json",
        ])?;
        let subaccount: Subaccount = serde_json::from_str(&output)?;
        Ok(subaccount)
    }
}

/// Asynchronous Green API client
pub struct AsyncGreenClient {
    // TODO: Add client configuration fields
}

impl AsyncGreenClient {
    /// Create a new asynchronous Green API client
    pub fn new() -> Self {
        Self {}
    }

    /// Run a green-cli command asynchronously
    ///
    /// This is a convenience method that delegates to `run_cli_async`
    ///
    /// # Arguments
    ///
    /// * `args` - Command line arguments to pass to green-cli
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The stdout output on successful execution
    /// * `Err(Error::Cli)` - Error containing stderr on non-zero exit code
    pub async fn run_command(&self, args: &[&str]) -> Result<String> {
        run_cli_async(args).await
    }
}

impl Default for AsyncGreenClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl AsyncWalletExt for AsyncGreenClient {
    async fn get_balance(&self) -> Result<Balance> {
        let output = self.run_command(&["get", "balance", "--json"]).await?;
        let balance: Balance = serde_json::from_str(&output)?;
        Ok(balance)
    }

    async fn get_fee_estimates(&self) -> Result<FeeEstimates> {
        let output = self
            .run_command(&["get", "fee-estimates", "--json"])
            .await?;
        let fee_estimates: FeeEstimates = serde_json::from_str(&output)?;
        Ok(fee_estimates)
    }
}

/// Helper function to run green-cli commands
///
/// Invokes `green-cli` with the provided arguments, setting `-L` and `-T`
/// environment variables by default. Captures stdout/stderr and returns
/// stdout as a String on success, or Error::Cli on failure.
///
/// # Arguments
///
/// * `args` - Command line arguments to pass to green-cli
///
/// # Returns
///
/// * `Ok(String)` - The stdout output on successful execution
/// * `Err(Error::Cli)` - Error containing stderr on non-zero exit code
///
/// # Example
///
/// ```no_run
/// use green_rs::client::run_cli;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let output = run_cli(&["status", "--json"])?;
///     println!("Green CLI output: {}", output);
///     Ok(())
/// }
/// ```
pub fn run_cli(args: &[&str]) -> Result<String> {
    let output = Command::new("green-cli")
        .args(args)
        .env("GREEN_CLI_L", "-L")
        .env("GREEN_CLI_T", "-T")
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(Error::Cli(stderr))
    }
}

/// Asynchronous helper function to run green-cli commands
///
/// Invokes `green-cli` with the provided arguments, setting `-L` and `-T`
/// environment variables by default. Captures stdout/stderr and returns
/// stdout as a String on success, or Error::Cli on failure.
///
/// # Arguments
///
/// * `args` - Command line arguments to pass to green-cli
///
/// # Returns
///
/// * `Ok(String)` - The stdout output on successful execution
/// * `Err(Error::Cli)` - Error containing stderr on non-zero exit code
///
/// # Example
///
/// ```no_run
/// use green_rs::client::run_cli_async;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let output = run_cli_async(&["status", "--json"]).await?;
///     println!("Green CLI output: {}", output);
///     Ok(())
/// }
/// ```
pub async fn run_cli_async(args: &[&str]) -> Result<String> {
    let output = TokioCommand::new("green-cli")
        .args(args)
        .env("GREEN_CLI_L", "-L")
        .env("GREEN_CLI_T", "-T")
        .output()
        .await?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(Error::Cli(stderr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_cli_with_invalid_command() {
        // This test assumes green-cli might not be installed
        // and verifies error handling
        let result = run_cli(&["--help"]);

        // The function should either succeed (if green-cli is installed)
        // or return an IO error (if green-cli is not found)
        match result {
            Ok(_) => {}             // green-cli is installed and returned help
            Err(Error::Io(_)) => {} // green-cli not found, which is expected
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_run_cli_async_with_invalid_command() {
        // This test assumes green-cli might not be installed
        // and verifies error handling
        let result = run_cli_async(&["--help"]).await;

        // The function should either succeed (if green-cli is installed)
        // or return an IO error (if green-cli is not found)
        match result {
            Ok(_) => {}             // green-cli is installed and returned help
            Err(Error::Io(_)) => {} // green-cli not found, which is expected
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_async_green_client() {
        let client = AsyncGreenClient::new();

        // Test the run_command method
        let result = client.run_command(&["--help"]).await;

        // The function should either succeed (if green-cli is installed)
        // or return an IO error (if green-cli is not found)
        match result {
            Ok(_) => {}             // green-cli is installed and returned help
            Err(Error::Io(_)) => {} // green-cli not found, which is expected
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }
}
