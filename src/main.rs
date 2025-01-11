use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_transaction_status::UiTransactionEncoding;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_sdk::commitment_config::CommitmentConfig;
use std::str::FromStr;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program_id = Pubkey::from_str("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P").unwrap();
    
    // Use `new_with_commitment` with default commitment and specify transaction version support
    let client = RpcClient::new_with_commitment(
        "https://api.mainnet-beta.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    loop {
        // Fetch the latest transactions for the program
        let transactions = client.get_signatures_for_address_with_config(&program_id, Default::default())?;

        for tx_signature in transactions {
            // Convert the signature string to a Signature type
            let signature = Signature::from_str(&tx_signature.signature)?;
            
            // Fetch the full transaction details with the correct encoding and version support
            let _transaction = client.get_transaction_with_config(
                &signature,
                RpcTransactionConfig {
                    encoding: Some(UiTransactionEncoding::JsonParsed),
                    max_supported_transaction_version: Some(0),
                    ..RpcTransactionConfig::default()
                },
            )?;
            
            // Here you would parse the transaction to find your `CreateEvent`
            // This part would depend on how your smart contract logs events
            println!("Transaction Signature: {}", tx_signature.signature);
            // You'd need to implement logic to check if this transaction is a CreateEvent
        }

        // Wait before next poll
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
// Program Id: 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P
// Owner: BPFLoaderUpgradeab1e11111111111111111111111
// ProgramData Address: B5MvUwXdiW1NMM6QFFD3ssPKBujD4zMohncbM73Z2BQu
// Authority: 7gZufwwAo17y5kg8FMyJy2phgpvv9RSdzWtdXiWHjFr8
// Last Deployed In Slot: 267728521
// Data Length: 727600 (0xb1a30) bytes
// Balance: 5.06530008 SOL

