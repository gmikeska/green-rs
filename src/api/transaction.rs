//! Transaction API implementation

/// Transaction API trait for Green clients
pub trait TransactionApi {
fn create_transaction(
        &self,
        request: CreateTransactionRequest,
        human_readable: bool,
    ) -> Result<CreateTransactionResult> {
        let mut args = vec!["createtransaction"];
        if human_readable {
            args.push("--human-readable");
        }
        // Add logic for asset_id and other parameters
        // Convert request to command line arguments
        let output = self.run_command(&args)?;
        let result: CreateTransactionResult = serde_json::from_str(&output)?;
        Ok(result)
    }

    fn send_to_address(
        &self,
        address: &str,
        amount: Satoshis,
        asset_id: Option<AssetId>,
        human_readable: bool,
    ) -> Result<Transaction> {
        let mut args = vec!["sendtoaddress", address, &amount.to_string()];
        if let Some(asset) = asset_id {
            args.push("--asset-id");
            args.push(&asset.to_string());
        }
        if human_readable {
            args.push("--human-readable");
        }
        let output = self.run_command(&args)?;
        let transaction: Transaction = serde_json::from_str(&output)?;
        Ok(transaction)
    }

    fn get_transactions(
        &self,
        asset_id: Option<AssetId>,
        human_readable: bool,
    ) -> Result<TransactionList> {
        let mut args = vec!["gettransactions"];
        if let Some(asset) = asset_id {
            args.push("--asset-id");
            args.push(&asset.to_string());
        }
        if human_readable {
            args.push("--human-readable");
        }
        let output = self.run_command(&args)?;
        let transactions: TransactionList = serde_json::from_str(&output)?;
        Ok(transactions)
    }

    fn get_transaction_details(
        &self,
        txid: &str,
        asset_id: Option<AssetId>,
        human_readable: bool,
    ) -> Result<Transaction> {
        let mut args = vec!["gettransactiondetails", txid];
        if let Some(asset) = asset_id {
            args.push("--asset-id");
            args.push(&asset.to_string());
        }
        if human_readable {
            args.push("--human-readable");
        }
        let output = self.run_command(&args)?;
        let transaction: Transaction = serde_json::from_str(&output)?;
        Ok(transaction)
    }
}
