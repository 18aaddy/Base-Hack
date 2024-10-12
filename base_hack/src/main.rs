mod database;
mod transaction_history;
mod utils;

use transaction_history::{fetcher, erc20_token_identifier};
use tokio;
use web3::types::Address;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let address: Address = Address::from_str("0x89CC5cD900dae8AfC788DCBAB11f5c2F5f660636").unwrap();

    let result = fetcher::fetch_transaction_logs(address, 1).await;
    match result {
        Ok(r) => match database::writer::write_to_db(r.clone(), 1).await {
            Ok(_) => println!("Number of logs: {}", r.len()),
            Err(e) => println!("Error: {}", e),
        },
        Err(e) => println!("Error: {}", e),
    }

    let contract_address: Address = Address::from_str("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48").unwrap();
    let result = erc20_token_identifier::erc20_identifier(contract_address, 1).await;
    match result {
        Ok((r, s, v)) => println!("Name of token: {}, Symbol {}, Decimals {}", r, s, v),
        Err(e) => println!("Error: {}", e),
    }

    
}