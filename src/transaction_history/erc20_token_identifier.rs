use crate::utils::chain_from_chain_id;
use crate::database::reader;

use serde::{Deserialize, Serialize};
use web3::types::{Address, CallRequest, H160, U256};
use web3::transports::Http;
use web3::Web3;
use dotenv::dotenv;
use std::env;
use hex;
// use std::str::FromStr;
// use hex_literal::hex;

#[derive(Serialize, Deserialize)]
struct UserDetails {
    chain_id: u64,
    token_name: String,
    token_symbol: String,
    token_decimals: U256,
    token_balance: U256,
}

pub async fn get_data_of_token_from_address_list(address_list: Vec<H160>, user_address: Address, chain_id: u64) -> web3::Result<Vec<UserDetails>> {
    let mut user_details = Vec::<UserDetails>::new();
    for address in address_list {
        let (token_name, token_symbol, token_decimals) = erc20_identifier(address, chain_id).await?;
        let token_balance = erc20_balance_query(address, user_address, chain_id).await?;
        
        user_details.push(UserDetails {
            chain_id: chain_id,
            token_name: token_name,
            token_symbol: token_symbol,
            token_decimals: token_decimals,
            token_balance: token_balance,
        });
    }
    Ok(user_details)
}

pub async fn erc20_balance_query(token_contract_address: Address, user_address: Address, chain_id: u64) -> web3::Result<U256> {
    let (web3, from_address) = query_contract_with_signature(chain_id).await?;
    // ERC-20 balanceOf function selector (keccak256 hash of "balanceOf(address)")
    let balance_of_signature = hex::decode("70a08231").unwrap(); 

    // Pack the function call data: balanceOf(user_address)
    let mut data = balance_of_signature;
    data.extend_from_slice(&user_address.as_bytes());

    // Perform the call
    let result = query_contract(&web3, token_contract_address, from_address, data).await?;

    // Convert the result to U256 (token balance)
    let balance = U256::from_big_endian(&result.as_slice());
    
    Ok(balance)
}

//TODO: Add function for querying ERC721 contracts
pub async fn erc20_identifier(token_contract_address: Address, chain_id: u64) -> web3::Result<(String, String, U256)> {
    let (web3, from_address) = query_contract_with_signature(chain_id).await?;
    
    // ERC-20 function signatures
    let name_signature = hex::decode("06fdde03").unwrap();      // Keccak-256 hash of 'name()'
    let decimals_signature = hex::decode("313ce567").unwrap();  // Keccak-256 hash of 'decimals()'
    let symbol_signature = hex::decode("95d89b41").unwrap();  // Keccak-256 hash of 'decimals()'

    // Query the token's name
    let name_result = query_contract(&web3, token_contract_address, from_address, name_signature).await?;
    let token_name = String::from_utf8_lossy(&name_result);
    println!("Token Name: {}", token_name);

    // Query the token's decimals
    let decimals_result = query_contract(&web3, token_contract_address, from_address, decimals_signature).await?;
    let token_decimals = U256::from(&decimals_result[..]);
    println!("Token Decimals: {}", token_decimals);

    // Query the token's symbol
    let symbol_result = query_contract(&web3, token_contract_address, from_address, symbol_signature).await?;
    let token_symbol = String::from_utf8_lossy(&symbol_result);
    println!("Token symbol: {}", token_symbol);

    Ok((token_name.to_string(), token_symbol.to_string(), token_decimals))
}

async fn query_contract_with_signature(chain_id: u64) -> web3::Result<(Web3<Http>, Address)> {
    dotenv().ok();
    let chain = chain_from_chain_id(chain_id)?;
    let env_var = format!("{}_RPC_URL", chain);

    let rpc_url = env::var(env_var.clone()).expect(format!("{} not set", env_var).as_str());
    let transport = Http::new(&rpc_url)?;
    let web3 = Web3::new(transport);
    
    let from_address: Address = "0xeccf26e9F5474882a671D6136B32BE1DF8b2CDda".parse().unwrap();

    Ok((web3, from_address))
}

// Helper function to query the contract
async fn query_contract(web3: &Web3<Http>, contract_address: Address, from_address: Address, data: Vec<u8>) -> web3::Result<Vec<u8>> {
    // Prepare the transaction parameters (call)    
    let req = CallRequest {
        from: Some(from_address),
        to: Some(contract_address),
        data: Some(web3::types::Bytes(data)),
        ..Default::default()
    };

    // Perform the call
    let result = web3.eth().call(req, None).await?;
    Ok(result.0)
}
