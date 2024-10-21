use crate::portfolio_overview::erc20_portfolio_tracker::PortfolioRequest;
// use std::str::FromStr;
// use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use web3::types::U64;
// use std::env;
// use web3::transports::Http;
use web3::{
    types::{Address, Log, H160, H256, U256},
    Result
};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use super::logs_fetcher::fetch_transaction_logs;

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub from: H160,
    pub to: H160,
    pub value: U256,
    pub block_number: U64,
    pub txn_hash: H256,
}

#[derive(Serialize)]
pub struct TransactionResponse {
    pub transaction_details: Option<Vec<Transaction>>,
    pub err: Option<String>,
}

pub async fn web_route_transaction_history(request_body: web::Json<PortfolioRequest>) -> impl Responder {
    let address: Address = request_body.user_address.parse().expect("Invalid Address");
    let history = match fetch_transaction_history(address, request_body.chain.clone()).await {
        Ok(h) => h,
        Err(e) => return HttpResponse::BadRequest().json(json!({
            "transaction_details": null,
            "err": e.to_string(),
        })),
    };

    let result = TransactionResponse{
        transaction_details: Some(history),
        err: None,
    };

    HttpResponse::Ok().json(result)
}
pub async fn fetch_transaction_history(address: Address, chain: String) -> Result<Vec<Transaction>> {
    // dotenv().ok();
    // let env_var = format!("{}_RPC_URL", chain);

    // let rpc_url = env::var(env_var.clone()).expect(format!("{} not set", env_var).as_str());
    // let transport = Http::new(&rpc_url)?;
    // let web3 = Web3::new(transport);
    // let transfer_event_signature =
    //     H256::from_str("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef")
    //         .unwrap();

    // let filter = FilterBuilder::default()
    //     .address(vec![address])
    //     .topics(
    //         Some(vec![transfer_event_signature]), // Filter by Transfer event signature
    //         None,                                 // From address (optional)
    //         Some(vec![H256::from(address)]),                                 // To address (optional)
    //         None,                                 // Token amount or tokenId (optional)
    //     )
    //     .from_block(web3::types::BlockNumber::Earliest) // Starting block
    //     .to_block(web3::types::BlockNumber::Latest) // Up to latest block
    //     .build();

    // let latest_block_number = web3.eth().block_number().await?;

    // // Calculate the starting block (latest - 5000)
    // let start_block_number = if latest_block_number > 5000.into() {
    //     web3::types::BlockNumber::Number(latest_block_number - 5000)
    // } else {
    //     web3::types::BlockNumber::Earliest  // If there are fewer than 5000 blocks, start from the earliest
    // };

    // Fetch logs
    let logs: Vec<Log> = fetch_transaction_logs(address, chain).await?;
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