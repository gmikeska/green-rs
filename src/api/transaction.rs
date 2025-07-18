//! Transaction API implementation

use crate::Result;
use crate::types::common::{Address, AssetId, Satoshis, Script};
use crate::types::transaction::{CreateTransactionRequest, CreateTransactionResult, Transaction, TransactionList, TxOutput};
use serde_json::{self, json};
use tempfile::NamedTempFile;

/// Transaction API trait for Green clients
pub trait TransactionApi {
    /// Create a new transaction
    fn create_transaction(
        &self,
        request: CreateTransactionRequest,
    ) -> Result<CreateTransactionResult>;

    /// Send to an address
    fn send_to_address(
        &self,
        address: &str,
        amount: Satoshis,
        asset_id: Option<AssetId>,
    ) -> Result<Transaction>;

    /// Get transaction list
    fn get_transactions(
        &self,
        subaccount: Option<u32>,
        first: Option<u32>,
        count: Option<u32>,
    ) -> Result<TransactionList>;

    /// Get transaction details
    fn get_transaction_details(
        &self,
        txid: &str,
    ) -> Result<Transaction>;
}

/// Builder object for creating transactions
pub struct TxBuilder {
    outputs: Vec<TxOutput>,
    inputs: Vec<String>,  // Store input UTXOs
    fee_rate: Option<u64>,
    subaccount: Option<u32>,
    temp_file_path: Option<String>,
    json_data: Option<String>,  // Store JSON representation
}

impl TxBuilder {
    /// Initialize a new TxBuilder
    pub fn new() -> Self {
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
    pub fn set_fee_rate(mut self, fee_rate: u64) -> Self {
        self.fee_rate = Some(fee_rate);
        self
    }

    /// Set the subaccount
    pub fn set_subaccount(mut self, subaccount: u32) -> Self {
        self.subaccount = Some(subaccount);
        self
    }

    /// Add an input UTXO
    pub fn add_input(mut self, utxo: String) -> Self {
        self.inputs.push(utxo);
        self
    }

    /// Serialize and dump transaction data
    pub fn dump(mut self) -> Result<Self> {
        // Create a JSON representation of the transaction
        let tx_data = serde_json::json!({
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
        println!("Transaction data dumped to: {}", temp_path);
        
        Ok(self)
    }

    /// Sign the transaction using green-cli
    pub fn sign(self) -> Result<Self> {
        if let Some(ref path) = self.temp_file_path {
            println!("Signing transaction from file: {}", path);
            // TODO: Execute green-cli tx sign command
            // Example: green-cli tx sign --file <path>
        } else {
            return Err(crate::Error::unexpected("No transaction data to sign. Call dump() first."));
        }
        Ok(self)
    }

    /// Send or broadcast the transaction using green-cli
    pub fn send(self) -> Result<String> {
        if let Some(ref path) = self.temp_file_path {
            println!("Broadcasting transaction from file: {}", path);
            // TODO: Execute green-cli tx send command
            // Example: green-cli tx send --file <path>
            // Return transaction ID
            Ok("dummy_txid".to_string())
        } else {
            Err(crate::Error::unexpected("No transaction data to broadcast. Call dump() and sign() first."))
        }
    }

    /// Alias for send()
    pub fn broadcast(self) -> Result<String> {
        self.send()
    }

    /// Get the JSON representation
    pub fn to_json(&self) -> Option<&str> {
        self.json_data.as_deref()
    }

    /// Get the temporary file path
    pub fn get_temp_path(&self) -> Option<&str> {
        self.temp_file_path.as_deref()
    }
}

/// Async Transaction API trait for Green clients
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
    async fn get_transaction_details(
        &self,
        txid: &str,
    ) -> Result<Transaction>;
}
