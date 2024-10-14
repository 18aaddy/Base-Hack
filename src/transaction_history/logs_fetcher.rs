use dotenv::dotenv;
use std::env;
use std::str::FromStr;
use web3::transports::Http;
use web3::{
    types::{Address, FilterBuilder, Log, H256},
    Result, Web3
};

pub async fn fetch_transaction_logs(address: Address, chain: String) -> Result<Vec<Log>> {
    dotenv().ok();
    let env_var = format!("{}_RPC_URL", chain);

    let rpc_url = env::var(env_var.clone()).expect(format!("{} not set", env_var).as_str());
    let transport = Http::new(&rpc_url)?;
    let web3 = Web3::new(transport);
    let transfer_event_signature =
        H256::from_str("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef")
            .unwrap();

    let filter = FilterBuilder::default()
        .topics(
            Some(vec![transfer_event_signature]), // Filter by Transfer event signature
            None,                                 // From address (optional)
            Some(vec![H256::from(address)]),                                 // To address (optional)
            None,                                 // Token amount or tokenId (optional)
        )
        .from_block(web3::types::BlockNumber::Earliest) // Starting block
        .to_block(web3::types::BlockNumber::Latest) // Up to latest block
        .build();

    // Fetch logs
    let to_logs: Vec<Log> = web3.eth().logs(filter).await?;

    Ok(to_logs)   
}