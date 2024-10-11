use web3::types::{Address, TransactionParameters, U256};
use web3::transports::Http;
use web3::Web3;
use crate::fetcher::chain_from_chain_id;
use dotenv::dotenv;
use std::env;
// use std::str::FromStr;
// use hex_literal::hex;


async fn erc20_identifier(token_contract_address: Address, chain_id: u64) -> web3::Result<()> {
    dotenv().ok();
    let chain = chain_from_chain_id(chain_id)?;
    let env_var = format!("{}_RPC_URL", chain);

    let rpc_url = env::var(env_var.clone()).expect(format!("{} not set", env_var).as_str());
    let transport = Http::new(&rpc_url)?;
    let web3 = Web3::new(transport);
    
    let from_address: Address = "0xeccf26e9F5474882a671D6136B32BE1DF8b2CDda".parse().unwrap();

    // ERC-20 function signatures
    let name_signature = hex!("06fdde03");      // Keccak-256 hash of 'name()'
    let decimals_signature = hex!("313ce567");  // Keccak-256 hash of 'decimals()'

    // Query the token's name
    let name_result = query_contract(&web3, token_address, from_address, name_signature).await?;
    let token_name = String::from_utf8_lossy(&name_result);
    println!("Token Name: {}", token_name);

    // Query the token's decimals
    let decimals_result = query_contract(&web3, token_address, from_address, decimals_signature).await?;
    let token_decimals = U256::from(&decimals_result[..]);
    println!("Token Decimals: {}", token_decimals);

    Ok(())
}

// Helper function to query the contract
async fn query_contract(web3: &Web3<Http>, contract_address: Address, from_address: Address, data: Vec<u8>) -> web3::Result<Vec<u8>> {
    // Prepare the transaction parameters (call)
    let tx = TransactionParameters {
        to: Some(contract_address),
        data: web3::types::Bytes(data),
        from: from_address,
        ..Default::default()
    };

    // Perform the call
    let result = web3.eth().call(tx, None).await?;
    Ok(result.0)
}
