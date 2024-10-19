use serde::{Deserialize, Serialize};
use web3::ethabi::AbiError;
use web3::types::{Address, BlockNumber, CallRequest, H160, H256, U256};
use web3::transports::Http;
use web3::Web3;
use dotenv::dotenv;
use std::env;
use hex;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use crate::price_feeds::price_feed::{self, portfolio_value};

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

const POLYGON_ABI: &str = "";

pub async fn get_user_details(address_list: Vec<H160>, user_address: Address, chain: String) -> web3::Result<Vec<UserDetails>> {
    let mut user_details = Vec::<UserDetails>::new();

    for address in address_list.clone() {
        let (name, symbol, decimals, balance) = match get_erc20_info(address, user_address, chain.clone()).await {
            Ok((n,s,d,b)) => (n,s,d,b),
            Err(e) => return Err(web3::error::Error::from(e.to_string())),
        };

        let price = match price_feed::fetch_token_price(symbol.as_str()).await {
            Ok(p) => p,
            Err(e) => return Err(web3::error::Error::from(e.to_string())),
        };

        user_details.push(UserDetails {
            chain: chain.clone(),
            token_name: name,
            token_symbol: symbol,
            token_decimals: decimals,
            token_balance: balance,
            token_price: price,
        });
    }
    Ok(user_details)
}

// pub async fn erc20_balance_query(token_contract_address: Address, user_address: Address, chain: String) -> web3::Result<U256> {
//     let (web3, from_address) = query_contract_with_signature(chain).await?;
//     // ERC-20 balanceOf function selector (keccak256 hash of "balanceOf(address)")
//     let balance_of_signature = hex::decode("70a08231").unwrap(); 

//     // Pack the function call data: balanceOf(user_address)
//     let mut data = balance_of_signature;
//     data.extend_from_slice(user_address.as_bytes());

//     // Perform the call
//     let result = query_contract(&web3, token_contract_address, from_address, data).await?;

//     // Convert the result to U256 (token balance)
//     let balance = U256::from(&result[..]);

//     println!("Token balance: {:?}", result);
    
//     Ok(balance)
// }

// pub async fn erc20_identifier(token_contract_address: Address, chain: String) -> web3::Result<(String, String, U256)> {
//     let (web3, from_address) = query_contract_with_signature(chain).await?;
    
//     // ERC-20 function signatures
//     let name_signature = hex::decode("06fdde03").unwrap();      // Keccak-256 hash of 'name()'
//     let decimals_signature = hex::decode("313ce567").unwrap();  // Keccak-256 hash of 'decimals()'
//     let symbol_signature = hex::decode("95d89b41").unwrap();  // Keccak-256 hash of 'decimals()'

//     // Query the token's name
//     let name_result = query_contract(&web3, token_contract_address, from_address, name_signature).await?;
//     let token_name = String::from_utf8_lossy(&name_result);
//     println!("Token Name: {}", token_name);

//     // Query the token's decimals
//     let decimals_result = query_contract(&web3, token_contract_address, from_address, decimals_signature).await?;
//     let token_decimals = U256::from(&decimals_result[..]);
//     println!("Token Decimals: {}", token_decimals);

//     // Query the token's symbol
//     let symbol_result = query_contract(&web3, token_contract_address, from_address, symbol_signature).await?;
//     let token_symbol = String::from_utf8_lossy(&symbol_result);
//     println!("Token symbol: {}", token_symbol);

//     Ok((token_name.to_string(), token_symbol.to_string(), token_decimals))
// }

// async fn query_contract_with_signature(chain: String) -> web3::Result<(Web3<Http>, Address)> {
//     dotenv().ok();
//     let env_var = format!("{}_RPC_URL", chain);

//     let rpc_url = env::var(env_var.clone()).expect(format!("{} not set", env_var).as_str());
//     let transport = Http::new(&rpc_url)?;
//     let web3 = Web3::new(transport);
    
//     let from_address: Address = "0xeccf26e9F5474882a671D6136B32BE1DF8b2CDda".parse().unwrap();

//     Ok((web3, from_address))
// }

// // Helper function to query the contract
// async fn query_contract(web3: &Web3<Http>, contract_address: Address, from_address: Address, data: Vec<u8>) -> web3::Result<Vec<u8>> {
//     // Prepare the transaction parameters (call)    
//     let req = CallRequest {
//         from: Some(from_address),
//         to: Some(contract_address),
//         data: Some(web3::types::Bytes(data)),
//         gas: Some(U256::from(1000000)),
//         ..Default::default()
//     };

//     // Perform the call
//     let result = web3.eth().call(req, None).await;

//     match result {
//         Ok(data) => {
//             // If the call succeeds
//             return Ok(data.0)
//         }
//         Err(web3::Error::Rpc(rpc_error)) => {
//             // If the call reverts, try to decode the revert reason
//             if let Some(error_data) = rpc_error.clone().data {
//                 println!("Revert reason data: {:?}", error_data);
    
//                 println!("Revert reason: {:?}", error_data.as_str());
//             } else {
//                 println!("Call reverted but no data available.");
//             }
            
//             return Err(web3::Error::Rpc(rpc_error))  // Re-throw the error
//         }
//         Err(e) => {
//             // Other errors
//             println!("Failed to perform call: {:?}", e);
//             return Err(e)
//         }
//     }
// }

pub async fn get_eth_amount_of_user(user_address: Address, chain: String) -> web3::Result<U256> {
    // Connect to the Ethereum node (replace with your own RPC URL)
    if chain != "ETHEREUM" && chain  != "POLYGON" {
        return Err(web3::Error::from(format!("This function cannot be used for this chain. Chain: {}", chain)));
    }
    dotenv().ok();
    let env_var = format!("{}_RPC_URL", chain);

    let rpc_url = env::var(env_var.clone()).expect(format!("{} not set", env_var).as_str());
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

// // Function to query an ERC20 contract for name, symbol, decimals, and balance of a user
// pub async fn query_erc20(
//     chain: String,
//     token_contract_address: Address,
//     user_address: Address
// ) -> web3::Result<(String, String, u8, U256)> {
//     dotenv().ok();
//     let env_var = format!("{}_RPC_URL", chain);

//     let rpc_url = env::var(env_var.clone()).expect(format!("{} not set", env_var).as_str());

//     // Connect to the Ethereum node
//     let transport = Http::new(rpc_url.as_str())?;
//     let web3 = Web3::new(transport);

//     // Define function selectors (ERC-20 standards)
//     let name_signature = hex::decode("06fdde03").unwrap();      // Keccak-256 hash of 'name()'
//     let symbol_signature = hex::decode("95d89b41").unwrap();    // Keccak-256 hash of 'symbol()'
//     let decimals_signature = hex::decode("313ce567").unwrap();  // Keccak-256 hash of 'decimals()'
//     let balance_of_signature = hex::decode("70a08231").unwrap(); // Keccak-256 hash of 'balanceOf(address)'

//     // Query the token's name
//     let token_name = query_contract_string(&web3, token_contract_address, name_signature).await?;
    
//     // Query the token's symbol
//     let token_symbol = query_contract_string(&web3, token_contract_address, symbol_signature).await?;
    
//     // Query the token's decimals
//     let decimals_data = query_contract_1(&web3, token_contract_address, decimals_signature).await?;
//     let token_decimals = decimals_data[31]; // The last byte holds the decimals

//     // Query the user's token balance
//     let mut balance_of_data = balance_of_signature.clone();
//     let mut extended_address = H256::zero(); // Create a zero-initialized H256
//     extended_address.0[..20].copy_from_slice(&user_address.0);
//     balance_of_data.extend_from_slice(extended_address.as_bytes()); // Append the user address to the function selector
//     let user_balance = query_contract_1(&web3, token_contract_address, balance_of_data).await?;
//     let balance = U256::from_big_endian(&user_balance); // Convert the returned data to U256

//     // Return the results
//     Ok((token_name, token_symbol, token_decimals, balance))
// }

// // Helper function to query contract for string data (name and symbol)
// async fn query_contract_string(web3: &Web3<Http>, contract_address: Address, data: Vec<u8>) -> web3::Result<String> {
//     let result = query_contract_1(web3, contract_address, data).await?;
//     Ok(String::from_utf8_lossy(&result).to_string())
// }

// // Helper function to query contract for raw data
// async fn query_contract_1(web3: &Web3<Http>, contract_address: Address, data: Vec<u8>) -> web3::Result<Vec<u8>> {
//     let req = CallRequest {
//         from: None, // You can specify a from address if needed, but not necessary here
//         to: Some(contract_address),
//         data: Some(web3::types::Bytes(data)),
//         ..Default::default()
//     };

//     // Perform the call
//     let result = match web3.eth().call(req, None).await {
//         Ok(r) => r,
//         Err(e) => {println!("Error in rpc call function"); return Err(e)}
//     };
//     Ok(result.0)
// }

pub async fn get_erc20_info(
    contract_address: Address,
    user_address: Address,
    chain: String,
) -> Result<(String, String, u8, U256), Box<dyn std::error::Error>> {
    dotenv().ok();
    let env_var = format!("{}_RPC_URL", chain);

    let rpc_url = env::var(env_var.clone()).expect(format!("{} not set", env_var).as_str());

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

// pub async fn erc721_identifier(token_contract_address: Address, user_address: Address, chain: String) -> Result<Vec<U256>>{
//     let (web3, from_address) = query_contract_with_signature(chain).await?;
    
//     // ERC-20 function signatures
//     let name_signature = hex::decode("06fdde03").unwrap();      // Keccak-256 hash of 'name()'
//     let symbol_signature = hex::decode("95d89b41").unwrap();
//     let balance_signature = hex::decode("70a08231").unwrap();
//     let owner_of_id_signature = hex::decode("6352211e").unwrap();

//     // Query the token's name
//     let name_result = query_contract(&web3, token_contract_address, from_address, name_signature).await?;
//     let token_name = String::from_utf8_lossy(&name_result);
//     println!("Token name: {}", token_name);

//     // Query the token's symbol
//     let symbol_result = query_contract(&web3, token_contract_address, from_address, symbol_signature).await?;
//     let token_symbol = String::from_utf8_lossy(&symbol_result);
//     println!("Token symbol: {}", token_symbol);

//     // Query the token's symbol
//     let balance_result = query_contract(&web3, token_contract_address, from_address, balance_signature).await?;
//     let token_balance = U64::from(&balance_result[..]);
//     println!("Token balance: {}", token_balance);
    
//     // Step 1: Get the balance of NFTs owned by the address
//     let balance_of_signature = "70a08231".to_string(); // Keccak-256 hash of 'balanceOf(address)'
//     let balance_request = CallRequest {
//         from: None,
//         to: Some(token_contract_address),
//         data: Some(web3::types::Bytes(hex::decode(balance_of_signature + hex::encode(user_address).as_str()).unwrap())),
//         ..Default::default()
//     };

//     let balance_result = web3.eth().call(balance_request, None).await?;
//     let balance = U256::from(balance_result.0.as_slice());

//     // Step 2: Retrieve each token ID owned by the address
//     let mut token_ids = Vec::new();
//     for index in 0..balance.as_u64() {
//         let token_of_owner_by_index_signature = "2f745c59"; // Keccak-256 hash of 'tokenOfOwnerByIndex(address,uint256)'
//         let token_request = CallRequest {
//             from: None,
//             to: Some(token_contract_address),
//             data: Some(web3::types::Bytes(hex::decode(format!("{}{}{:02x}", token_of_owner_by_index_signature, user_address.as_bytes().iter().map(|b| format!("{:02x}", b)).collect::<String>(), index)).unwrap())),
//             ..Default::default()
//         };

//         let token_result = web3.eth().call(token_request, None).await?;
//         let token_id = U256::from(token_result.0.as_slice());
//         token_ids.push(token_id);
//     }

//     println!("{:?}", token_ids);

//     Ok(token_ids)
// } 
