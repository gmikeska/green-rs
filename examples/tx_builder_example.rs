//! Example demonstrating the TxBuilder for transaction creation

use green_rs::api::TxBuilder;
use green_rs::Result;

fn main() -> Result<()> {
    // Example of using TxBuilder to create a transaction
    let tx_builder = TxBuilder::new()
        .add_output("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(), 100000)
        .add_output("bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq".to_string(), 50000)
        .set_fee_rate(10)
        .set_subaccount(0)
        .add_input("previous_txid:0".to_string());

    // Dump the transaction to a temporary file
    let tx_builder = tx_builder.dump()?;

    // Get the JSON representation
    if let Some(json) = tx_builder.to_json() {
        println!("Transaction JSON: {}", json);
    }

    // Get the temporary file path
    if let Some(path) = tx_builder.get_temp_path() {
        println!("Transaction file: {}", path);
    }

    // Sign the transaction (would call green-cli tx sign)
    let tx_builder = tx_builder.sign()?;

    // Broadcast the transaction (would call green-cli tx send)
    let txid = tx_builder.broadcast()?;
    println!("Transaction ID: {}", txid);

    Ok(())
}
