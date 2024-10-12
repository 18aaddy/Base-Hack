use reqwest::Error;
use serde::Deserialize;
use web3::types::{Address};
use crate::utils::chain_from_chain_id;
use std::env;
use anyhow::{Context, Result};
use dotenv::dotenv;

#[derive(Debug, Deserialize)]
pub struct AlchemyNftResponse {
    #[serde(rename = "ownedNfts")]
    pub owned_nfts: Vec<Nft>,
}

#[derive(Debug, Deserialize)]
pub struct Nft {
    pub contract: Contract,
    pub id: NftId,
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: Vec<Media>,
    #[serde(rename = "contractMetadata")]
    pub contract_metadata: ContractMetadata,
}

#[derive(Debug, Deserialize)]
struct Contract {
    pub address: String,
}

#[derive(Debug, Deserialize)]
struct NftId {
    pub token_id: String,
}

#[derive(Debug, Deserialize)]
struct Media {
    pub gateway: String,
}

#[derive(Debug, Deserialize)]
struct ContractMetadata {
    #[serde(rename = "contractDeployer")]
    pub contract_deployer: String,
    #[serde(rename = "deployedBlockNumber")]
    pub deployed_block_number: u64,
    pub name: String,
    pub symbol: String,
    #[serde(rename = "tokenType")]
    pub token_type: String,
}

#[tokio::main]
pub async fn fetch_nft_data(chain_id: u64, owner_address: Address) -> Result<AlchemyNftResponse> {
    dotenv().ok(); // Load environment variables from the .env file

    let chain = chain_from_chain_id(chain_id).context("Failed to get chain")?;

    let env_var = format!("{}_RPC_URL", chain);
    let rpc_url = env::var(&env_var)
        .context(format!("{} not set", env_var))?;
    
    // Construct the URL using the environment variable
    let url = format!(
        "{}/getNFTs?owner={}",
        rpc_url, owner_address
    );

    // Make the API request
    let client = reqwest::Client::new();
    let res = client.get(&url).send().await.context("Failed to send request")?;

    if res.status().is_success() {
        let nft_response: AlchemyNftResponse = res.json().await.context("Failed to parse response")?;
        Ok(nft_response) // Return the response
    } else {
        let status_code = res.status();
        Err(anyhow::anyhow!("Request failed with status: {}", status_code)) // Return an error
    }
}

pub fn print_nft_info(response: AlchemyNftResponse) {
    for nft in response.owned_nfts {
        println!("NFT Title: {}", nft.title.unwrap_or_else(|| "Unknown Title".to_string()));
        println!("Contract Address: {}", nft.contract.address);
        println!("Token ID: {}", nft.id.token_id);
        
        // Print Contract Metadata
        println!("Contract Deployer: {}", nft.contract_metadata.contract_deployer);
        println!("Deployed Block Number: {}", nft.contract_metadata.deployed_block_number);
        println!("Symbol: {}", nft.contract_metadata.symbol);
        println!("Token Type: {}", nft.contract_metadata.token_type);

        // Print Media URLs
        for media in &nft.media {
            println!("Media URL: {}", media.gateway);
        }

        println!("\n-----------------------\n");
    }
}
