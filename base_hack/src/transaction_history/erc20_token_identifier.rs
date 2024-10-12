use crate::utils::chain_from_chain_id;

use web3::types::{Address, CallRequest, U256};
use web3::transports::Http;
use web3::Web3;
use dotenv::dotenv;
use std::env;
use hex;
// use std::str::FromStr;
// use hex_literal::hex;


pub async fn erc20_identifier(token_contract_address: Address, chain_id: u64) -> web3::Result<(String, String, U256)> {
    dotenv().ok();
    let chain = chain_from_chain_id(chain_id)?;
    let env_var = format!("{}_RPC_URL", chain);

    let rpc_url = env::var(env_var.clone()).expect(format!("{} not set", env_var).as_str());
    let transport = Http::new(&rpc_url)?;
    let web3 = Web3::new(transport);
    
    let from_address: Address = "0xeccf26e9F5474882a671D6136B32BE1DF8b2CDda".parse().unwrap();

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
