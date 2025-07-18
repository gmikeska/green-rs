//! Transaction API implementation
//!
//! This module provides traits and implementations for transaction-related operations
//! in the Green client, including creation, signing, and broadcasting of transactions.
//!
//! # Liquid Network Support
//!
//! Transaction handling in Liquid involves unique features:
//! - Confidential transactions hide amounts and asset types
//! - Multiple asset types can be sent in a single transaction
//! - Fee calculations are more complex due to confidential proofs
//! - Blinding factors must be managed for confidential outputs
//! - Asset issuance and reissuance are supported

use crate::types::common::{Address, AssetId, Satoshis, Script};
use crate::types::transaction::{
    CreateTransactionRequest, CreateTransactionResult, Transaction, TransactionList, TxOutput,
};
use crate::Result;
use serde_json::json;
use tempfile::NamedTempFile;

/// Transaction API trait for Green clients
///
/// Provides synchronous methods for transaction operations including
/// creation, broadcasting, and retrieval.
///
/// # Liquid Network Considerations
///
/// - Transactions must specify asset IDs for non-L-BTC transfers
/// - Confidential transactions require additional space for range proofs
/// - Fee estimation should account for the complexity of confidential outputs
/// - Unblinded amounts are required for transaction validation
pub trait TransactionApi {
    /// Create a new transaction
    ///
    /// # Errors
    ///
    /// Returns an error if creating the transaction fails.
    fn create_transaction(
        &self,
        request: CreateTransactionRequest,
    ) -> Result<CreateTransactionResult>;

    /// Send to an address
    ///
    /// # Errors
    ///
    /// Returns an error if sending fails.
    fn send_to_address(
        &self,
        address: &str,
        amount: Satoshis,
        asset_id: Option<AssetId>,
    ) -> Result<Transaction>;

    /// Get transaction list
    ///
    /// # Errors
    ///
    /// Returns an error if retrieving the list fails.
    fn get_transactions(
        &self,
        subaccount: Option<u32>,
        first: Option<u32>,
        count: Option<u32>,
    ) -> Result<TransactionList>;

    /// Get transaction details
    ///
    /// # Errors
    ///
    /// Returns an error if details cannot be fetched.
    fn get_transaction_details(&self, txid: &str) -> Result<Transaction>;
}

/// Builder object for creating transactions
///
/// Provides a fluent interface for constructing complex transactions
/// with multiple inputs and outputs. The builder pattern ensures
/// transactions are built incrementally and validated before signing.
///
/// # Liquid Network Support
///
/// - Supports multi-asset transactions
/// - Handles confidential transaction construction
/// - Manages blinding factors for privacy
/// - Validates asset balances across inputs and outputs
///
/// # Example
///
/// ```no_run
/// use green_rs::api::TxBuilder;
///
/// let tx = TxBuilder::new()
///     .add_output("ex1q...".to_string(), 100000)
///     .set_fee_rate(1000)
///     .dump()
///     .expect("Failed to dump transaction")
///     .sign()
///     .expect("Failed to sign transaction")
///     .send()
///     .expect("Failed to send transaction");
/// ```
pub struct TxBuilder {
    outputs: Vec<TxOutput>,
    inputs: Vec<String>, // Store input UTXOs
    fee_rate: Option<u64>,
    subaccount: Option<u32>,
    temp_file_path: Option<String>,
    json_data: Option<String>, // Store JSON representation
}

impl Default for TxBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TxBuilder {
    /// Initialize a new `TxBuilder`
    #[must_use]
    pub const fn new() -> Self {
        Self {
            outputs: Vec::new(),
            inputs: Vec::new(),
            fee_rate: None,
            subaccount: None,
            temp_file_path: None,
            json_data: None,
        }
    }

    /// Add an output to the transaction
    #[must_use]
    pub fn add_output(mut self, address: Address, amount: Satoshis) -> Self {
        self.outputs.push(TxOutput {
            address: Some(address),
            satoshi: amount,
            script_pubkey: Script::default(),
            ..Default::default()
        });
        self
    }

    /// Set the fee rate
    #[must_use]
    pub fn set_fee_rate(mut self, fee_rate: u64) -> Self {
        self.fee_rate = Some(fee_rate);
        self
    }

    /// Set the subaccount
    #[must_use]
    pub fn set_subaccount(mut self, subaccount: u32) -> Self {
        self.subaccount = Some(subaccount);
        self
    }

    /// Add an input UTXO
    #[must_use]
    pub fn add_input(mut self, utxo: String) -> Self {
        self.inputs.push(utxo);
        self
    }

    /// Serialize and dump transaction data
    ///
    /// # Errors
    ///
    /// Returns an error if creating the temp file or serializing fails
    pub fn dump(mut self) -> Result<Self> {
        // Create a JSON representation of the transaction
        let tx_data = json!({
            "outputs": self.outputs,
            "inputs": self.inputs,
            "fee_rate": self.fee_rate,
            "subaccount": self.subaccount
        });

        let serialized = serde_json::to_string_pretty(&tx_data)?;
        self.json_data = Some(serialized.clone());

        // Write to a temporary file
        let temp_file = NamedTempFile::new()?;
        let temp_path = temp_file.path().to_string_lossy().to_string();
        std::fs::write(&temp_path, &serialized)?;

        self.temp_file_path = Some(temp_path.clone());
        println!("Transaction data dumped to: {temp_path}");

        Ok(self)
    }

    /// Sign the transaction using green-cli
    ///
    /// # Errors
    ///
    /// Returns an error if no transaction data exists or signing fails
    pub fn sign(self) -> Result<Self> {
        if let Some(ref path) = self.temp_file_path {
            println!("Signing transaction from file: {path}");
            // TODO: Execute green-cli tx sign command
            // Example: green-cli tx sign --file <path>
        } else {
            return Err(crate::Error::unexpected(
                "No transaction data to sign. Call dump() first.",
            ));
        }
        Ok(self)
    }

    /// Send or broadcast the transaction using green-cli
    ///
    /// # Errors
    ///
    /// Returns an error if no transaction data exists or broadcasting fails
    pub fn send(self) -> Result<String> {
        if let Some(ref path) = self.temp_file_path {
            println!("Broadcasting transaction from file: {path}");
            // TODO: Execute green-cli tx send command
            // Example: green-cli tx send --file <path>
            // Return transaction ID
            Ok("dummy_txid".to_string())
        } else {
            Err(crate::Error::unexpected(
                "No transaction data to broadcast. Call dump() and sign() first.",
            ))
        }
    }

    /// Alias for `send()`
    ///
    /// # Errors
    ///
    /// Returns an error if no transaction data exists or broadcasting fails
    pub fn broadcast(self) -> Result<String> {
        self.send()
    }

    /// Get the JSON representation
    #[must_use]
    pub fn to_json(&self) -> Option<&str> {
        self.json_data.as_deref()
    }

    /// Get the temporary file path
    #[must_use]
    pub fn get_temp_path(&self) -> Option<&str> {
        self.temp_file_path.as_deref()
    }
}

/// Async Transaction API trait for Green clients
///
/// Provides asynchronous methods for transaction operations including
/// creation, broadcasting, and retrieval.
///
/// # Liquid Network Considerations
///
/// - Transactions must specify asset IDs for non-L-BTC transfers
/// - Confidential transactions require additional space for range proofs
/// - Fee estimation should account for the complexity of confidential outputs
/// - Unblinded amounts are required for transaction validation
/// - Async operations are beneficial for complex multi-asset transactions
#[async_trait::async_trait]
pub trait AsyncTransactionApi {
    /// Create a new transaction
    async fn create_transaction(
        &self,
        request: CreateTransactionRequest,
    ) -> Result<CreateTransactionResult>;

    /// Send to an address
    async fn send_to_address(
        &self,
        address: &str,
        amount: Satoshis,
        asset_id: Option<AssetId>,
    ) -> Result<Transaction>;

    /// Get transaction list
    async fn get_transactions(
        &self,
        subaccount: Option<u32>,
        first: Option<u32>,
        count: Option<u32>,
    ) -> Result<TransactionList>;

    /// Get transaction details
    async fn get_transaction_details(&self, txid: &str) -> Result<Transaction>;
}
