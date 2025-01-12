use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_transaction_status::UiTransactionEncoding;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_sdk::commitment_config::CommitmentConfig;
use reqwest::header::{HeaderMap, HeaderValue, ORIGIN};
use std::str::FromStr;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program_id = Pubkey::from_str("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P").unwrap();
    
    // Create a custom reqwest client with the Origin header
    let mut headers = HeaderMap::new();
    headers.insert(ORIGIN, HeaderValue::from_static("https://pump.fun"));

    let reqwest_client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    // Use the custom client in RpcClient
    let client = RpcClient::new_with_client(
        "https://api.mainnet-beta.solana.com".to_string(),
        CommitmentConfig::confirmed(),
        reqwest_client,
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
            
            println!("Transaction Signature: {}", tx_signature.signature);
        }

        // Wait before next poll
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}