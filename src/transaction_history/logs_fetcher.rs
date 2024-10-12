use crate::utils::chain_from_chain_id;
use dotenv::dotenv;
use std::env;
use std::str::FromStr;
use web3::transports::Http;
use web3::{
    types::{Address, FilterBuilder, Log, H160, H256},
    Result, Web3
};

// pub struct TransferLog {
//     logs: Vec<Log>,
//     chain_id: u64
// }

pub async fn fetch_transaction_logs(address: Address, chain_id: u64) -> Result<Vec<Log>> {
    dotenv().ok();
    let chain = chain_from_chain_id(chain_id)?;
    let env_var = format!("{}_RPC_URL", chain);

    let rpc_url = env::var(env_var.clone()).expect(format!("{} not set", env_var).as_str());
    let transport = Http::new(&rpc_url)?;
    let web3 = Web3::new(transport);
    let transfer_event_signature =
        H256::from_str("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef")
            .unwrap();

    // // Optional: Filter logs by contract address (replace with your token's address)
    // let address: Address = Address::from_str("0x89CC5cD900dae8AfC788DCBAB11f5c2F5f660636").unwrap();

    // // Build a filter for the logs
    // let filter = FilterBuilder::default()
    //     .topics(
    //         Some(vec![transfer_event_signature]), // Filter by Transfer event signature
    //         Some(vec![H256::from(address)]),                                 // From address (optional)
    //         None,                                 // To address (optional)
    //         None,                                 // Token amount or tokenId (optional)
    //     )
    //     .from_block(web3::types::BlockNumber::Earliest) // Starting block
    //     .to_block(web3::types::BlockNumber::Latest) // Up to latest block
    //     .build();

    // // Fetch logs
    // let mut from_logs: Vec<Log> = web3.eth().logs(filter).await?;

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

    // println!("Logs: {:?}", logs);

    // Process logs
    for log in to_logs.clone() {
        let _from = H160::from_slice(&log.topics[1].as_bytes()[12..]);
        let _to = H160::from_slice(&log.topics[2].as_bytes()[12..]);
        let _value = web3::types::U256::from(log.data.0.as_slice());

        // println!(
        //     "Transfer: From: {:?}, To: {:?}, Value: {:?}",
        //     from, to, value
        // );
    }
    // from_logs.append(&mut to_logs);
    Ok(to_logs)   
}