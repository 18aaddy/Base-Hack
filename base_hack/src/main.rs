mod database;
mod transaction_history;

use transaction_history::fetcher;
use tokio;
use web3::types::Address;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let address: Address = Address::from_str("0xa8CF667b34bf05bFDefc85B34fd086213728cb5a").unwrap();

    let result = fetcher::fetch_transaction_logs(address, 8453).await;
    match result {
        Ok(r) => println!("Vector Logs: {:?}", r),
        Err(e) => println!("Error: {}", e),
    }
}