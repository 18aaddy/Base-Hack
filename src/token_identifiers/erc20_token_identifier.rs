use serde::{Deserialize, Serialize};
// use web3::ethabi::AbiError;
use web3::types::{Address, BlockNumber, H160, U256};
use web3::transports::Http;
use web3::Web3;
// use hex;
// use std::str::FromStr;
use web3::contract::{Contract, Options};
use crate::price_feeds::price_feed;
use futures::future::join_all;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserDetails {
    pub chain: String,
    pub token_name: String,
    pub token_symbol: String,
    pub token_decimals: u8,
    pub token_balance: U256,
    pub token_price: Option<f64>,
}

const ABI: &str = r#"[
        {"constant":true,"inputs":[{"name":"_owner","type":"address"}],"name":"balanceOf","outputs":[{"name":"balance","type":"uint256"}],"type":"function"},
        {"constant":true,"inputs":[],"name":"name","outputs":[{"name":"","type":"string"}],"type":"function"},
        {"constant":true,"inputs":[],"name":"symbol","outputs":[{"name":"","type":"string"}],"type":"function"},
        {"constant":true,"inputs":[],"name":"decimals","outputs":[{"name":"","type":"uint8"}],"type":"function"}
    ]"#;

const abi_bytes: &[u8] = ABI.as_bytes();

// const POLYGON_ABI: &str = "";

pub async fn get_user_details(rpc_url: &String, address_list: Vec<H160>, user_address: Address, chain: String) -> web3::Result<Vec<UserDetails>> {
    let mut user_details = Vec::<UserDetails>::new();

    // Spawn all requests in parallel for the address_list
    let tasks = address_list.into_iter().map(|address| {
        let rpc_url = rpc_url.clone(); // Clone the rpc_url to ensure it lives long enough
        let user_address = user_address.clone(); // Clone user_address to avoid lifetime issues
        let chain = chain.clone(); // Clone the chain to be used inside the async block
        
        async move {
            let result = async {
                let (name, symbol, decimals, balance) = match get_erc20_info(&rpc_url, address, user_address).await {
                    Ok(p) => p,
                    Err(_) => return Err(web3::Error::InvalidResponse("error in fetching token price".to_string())),
                };
                let price = match price_feed::fetch_token_price(symbol.as_str()).await {
                    Ok(p) => p,
                    Err(_) => return Err(web3::Error::InvalidResponse("error in fetching token price".to_string())),
                };
                Ok::<UserDetails, web3::error::Error>(UserDetails {
                    chain: chain.clone(),
                    token_name: name,
                    token_symbol: symbol,
                    token_decimals: decimals,
                    token_balance: balance,
                    token_price: price,
                })
            }.await;

            match result {
                Ok(details) => Some(details),
                Err(e) => {
                    eprintln!("Error fetching token details for address {:?}: {:?}", address, e);
                    None
                }
            }
        }
    });

    // Execute all tasks in parallel
    let results: Vec<Option<UserDetails>> = join_all(tasks).await;
    for res in results {
        if let Some(details) = res {
            user_details.push(details);
        }
    }

    // Fetch ETH details separately
    let eth_amount = get_eth_amount_of_user(rpc_url, user_address, "ETHEREUM".to_string()).await?;
    let eth_price = match price_feed::fetch_token_price("ETH").await {
        Ok(p) => p,
        Err(_) => return Err(web3::Error::InvalidResponse("error in fetching token price".to_string())),
    };
    user_details.push(UserDetails {
        chain: "ETHEREUM".to_string(),
        token_name: "ETHER".to_string(),
        token_symbol: "ETH".to_string(),
        token_decimals: 18,
        token_balance: eth_amount,
        token_price: eth_price,
    });

    // Fetch BASE ETH details separately
    let base_eth_amount = get_eth_amount_of_user(rpc_url, user_address, "BASE".to_string()).await?;
    let base_eth_price = match price_feed::fetch_token_price("ETH").await {
        Ok(p) => p,
        Err(_) => return Err(web3::Error::InvalidResponse("error in fetching token price".to_string())),
    };
    user_details.push(UserDetails {
        chain: "BASE".to_string(),
        token_name: "ETHER".to_string(),
        token_symbol: "ETH".to_string(),
        token_decimals: 18,
        token_balance: base_eth_amount,
        token_price: base_eth_price,
    });

    Ok(user_details)
}

pub async fn get_eth_amount_of_user(rpc_url: &String, user_address: Address, chain: String) -> web3::Result<U256> {
    // Connect to the Ethereum node (replace with your own RPC URL)
    if chain != "ETHEREUM" && chain  != "POLYGON" && chain != "BASE" {
        return Err(web3::Error::from(format!("This function cannot be used for this chain. Chain: {}", chain)));
    }
    let transport = Http::new(&rpc_url)?;
    let web3 = Web3::new(transport);

    // Get the balance of the user at the latest block
    let balance: U256 = web3.eth().balance(user_address, Some(BlockNumber::Latest)).await?;

    // Balance is returned in Wei, so convert to Ether for readability (1 ETH = 10^18 Wei)
    let eth_balance = web3::types::U256::from(balance);
    // let eth_balance_in_ether = eth_balance.as_u128() as f64 / 1e18;

    println!("{} Balance of {}: {} {}", chain, user_address, eth_balance, chain);

    Ok(eth_balance)
}

pub async fn get_erc20_info(
    rpc_url: &String,
    contract_address: Address,
    user_address: Address,
) -> Result<(String, String, u8, U256), Box<dyn std::error::Error>> {

    // Connect to the Ethereum node
    let transport = Http::new(rpc_url.as_str())?;
    let web3 = Web3::new(transport);
    // ERC20 token ABI for required functions

    // Create contract instance
    let contract = Contract::from_json(web3.eth(), contract_address, abi_bytes)?;

    // Query token information
    let name: String = contract.query("name", (), None, Options::default(), None).await?;
    let symbol: String = contract.query("symbol", (), None, Options::default(), None).await?;
    let decimals: u8 = contract.query("decimals", (), None, Options::default(), None).await?;
    
    // Query balance
    let balance: U256 = contract
        .query("balanceOf", (user_address,), None, Options::default(), None)
        .await?;

    Ok((name, symbol, decimals, balance))
}