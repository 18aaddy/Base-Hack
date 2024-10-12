use crate::utils::chain_from_chain_id;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use web3::types::U64;
use std::env;
use web3::transports::Http;
use web3::{
    types::{Address, FilterBuilder, Log, H160, H256, U256},
    Result, Web3
};

// pub struct TransferLog {
//     logs: Vec<Log>,
//     chain_id: u64
// }

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    from: H160,
    to: H160,
    value: U256,
    block_number: U64,
    txn_hash: H256,
}

pub async fn fetch_transaction_history(address: Address, chain_id: u64) -> Result<Vec<Transaction>> {
    dotenv().ok();
    let chain = chain_from_chain_id(chain_id)?;
    let env_var = format!("{}_RPC_URL", chain);

    let rpc_url = env::var(env_var.clone()).expect(format!("{} not set", env_var).as_str());
    let transport = Http::new(&rpc_url)?;
    let web3 = Web3::new(transport);

    let filter = FilterBuilder::default()
        .address(vec![address])
        .from_block(web3::types::BlockNumber::Earliest) // Starting block
        .to_block(web3::types::BlockNumber::Latest) // Up to latest block
        .build();

    // Fetch logs
    let logs: Vec<Log> = web3.eth().logs(filter).await?;
    let mut transactions = Vec::<Transaction>::new();

    // Process logs
    for log in logs.clone() {
        let _from = H160::from_slice(&log.topics[1].as_bytes()[12..]);
        let _to = H160::from_slice(&log.topics[2].as_bytes()[12..]);
        let _value = U256::from(log.data.0.as_slice());
        let _block_number = log.block_number.unwrap();
        let _transaction_hash = log.transaction_hash.unwrap();

        let txn = Transaction {
            from: _from,
            to: _to,
            value: _value,
            block_number: _block_number,
            txn_hash: _transaction_hash,
        };
        transactions.push(txn);
    }

    Ok(transactions)
}