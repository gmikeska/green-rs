//! Tests for the types module

use green_rs::types::*;

#[test]
fn test_balance_serialization() {
    let mut balance = Balance::new();
    balance.set("btc".to_string(), 100000);
    balance.set(
        "6f0279e9ed041c3d710a9f57d0c02928416460c4b722ae3457a11eec381c526d".to_string(),
        50000,
    );

    let json = serde_json::to_string(&balance).unwrap();
    let deserialized: Balance = serde_json::from_str(&json).unwrap();

    assert_eq!(balance, deserialized);
    assert_eq!(deserialized.get("btc"), Some(100000));
}

#[test]
fn test_receive_address_serialization() {
    let addr = ReceiveAddress {
        address: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
        pointer: 5,
        address_type: "p2wpkh".to_string(),
        branch: 0,
        subaccount: 0,
        script_pubkey: Some("0014...".to_string()),
        is_confidential: None,
        unconfidential_address: None,
    };

    let json = serde_json::to_string(&addr).unwrap();
    let deserialized: ReceiveAddress = serde_json::from_str(&json).unwrap();

    assert_eq!(addr, deserialized);
}

#[test]
fn test_transaction_serialization() {
    let tx = Transaction {
        txid: "abcd1234".to_string(),
        version: 2,
        locktime: 0,
        inputs: vec![TxInput {
            txid: "prev123".to_string(),
            vout: 0,
            script_sig: None,
            witness: None,
            sequence: 0xfffffffe,
            prevout: None,
            is_relevant: true,
            address: Some("bc1qtest".to_string()),
            subaccount: Some(0),
            pointer: Some(10),
        }],
        outputs: vec![TxOutput {
            satoshi: 50000,
            script_pubkey: "0014...".to_string(),
            address: Some("bc1qrecipient".to_string()),
            asset_id: None,
            is_relevant: false,
            subaccount: None,
            pointer: None,
            is_change: false,
        }],
        weight: Some(400),
        size: Some(200),
        vsize: Some(100),
        fee: Some(1000),
        fee_rate: Some(10.0),
        block_hash: None,
        block_height: None,
        confirmations: 0,
        timestamp: None,
        memo: None,
        tx_type: Some("outgoing".to_string()),
        subaccounts: vec![0],
        can_rbf: true,
        has_been_replaced: false,
        hex: None,
    };

    let json = serde_json::to_string_pretty(&tx).unwrap();
    let deserialized: Transaction = serde_json::from_str(&json).unwrap();

    assert_eq!(tx, deserialized);
}

#[test]
fn test_type_aliases() {
    // Test that our type aliases work correctly
    let asset_id: AssetId = "btc".to_string();
    let satoshis: Satoshis = 100000;
    let tx_id: TxId = "deadbeef".to_string();
    let block_height: BlockHeight = 700000;
    let pointer: Pointer = 42;
    let script: Script = "0014abcd".to_string();
    let address: Address = "bc1qtest".to_string();

    // Ensure they can be used in structs
    let _utxo = UtxoDetails {
        txhash: tx_id,
        vout: 0,
        satoshi: satoshis,
        asset_id: Some(asset_id),
        block_height: Some(block_height),
        address: Some(address),
        address_type: None,
        script_pubkey: Some(script),
        subaccount: 0,
        pointer,
        is_internal: false,
        is_confidential: false,
        is_frozen: false,
        memo: None,
    };
}
