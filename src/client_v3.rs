use solana_client::rpc_client::RpcClient;
use solana_transaction_status::{option_serializer::OptionSerializer,UiTransactionEncoding};
use log::debug;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    transaction::Transaction,
    signature::{read_keypair_file, Signer},
};
use std::str::FromStr;
use std::fs::File;
use std::io::Read;
use std::env;

fn main() {
    env_logger::init();
    let program_id = Pubkey::from_str("6HZ1ctyoSAeRuzSDJnqy63ihi3YfZ5sDaizcz8HQtnbW").unwrap();
    // let url = "http://localhost:8899".to_string();
    let url = "https://api.devnet.solana.com".to_string();
    let client = RpcClient::new_with_commitment(url, CommitmentConfig::confirmed());

    // Load keypair from file
    debug!("Executing query1");
    let mut file = File::open("../my-keypair2.json").expect("Unable to open keypair file");
    debug!("Executing query2");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read keypair file");
    debug!("Executing query3");
    // let payer = Keypair::from_base58_string(&contents);
    let payer = read_keypair_file("../my-keypair2.json").expect("Failed to read keypair file");
    debug!("Executing query4");

    // Check balance and request airdrop if needed
    let balance = client.get_balance(&payer.pubkey()).unwrap();
    if balance < 1_000_000_000 {
        let signature = client
            .request_airdrop(&payer.pubkey(), 1_000_000_000 - balance)
            .unwrap();
        client.confirm_transaction(&signature).unwrap();
        println!("Airdrop completed");
    }

    let instruction = Instruction::new_with_borsh(
        program_id,
        &(),
        vec![AccountMeta::new(payer.pubkey(), true)], // Changed to true for signing
    );

    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("Hey guys. Transaction signature: {}", signature);

    // Fetch transaction details
    let transaction_details = client.get_transaction_with_config(
    &signature,
    solana_client::rpc_config::RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::Json),
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
        },
    ).unwrap();

    // Print log messages
    if let Some(meta) = transaction_details.transaction.meta {
        // if let Some(log_messages) = meta.log_messages {
        match meta.log_messages {  
            OptionSerializer::Some(log_messages) => {  
                println!("Program log messages:");
                for log in log_messages {
                    println!("{}", log);
                }
            }
            OptionSerializer::None => {
                println!("No log messages found.");
            }
            OptionSerializer::Skip => {
                println!("Log messages were skipped.");
            }        
        }
    }
    else {
        println!("No transaction metadata found.");
    }
    
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
} 
