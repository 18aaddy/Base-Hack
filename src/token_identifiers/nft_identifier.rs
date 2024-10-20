
use reqwest::Client;
use serde_json::Value;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};
use std::cmp::min;
use std::env;
use anyhow::{Context, Result}; // Import Result from anyhow
use dotenv::dotenv;
use core::result::Result::Ok;
#[derive(Debug, Deserialize, Serialize)]
pub struct AlchemyNftResponse {
    #[serde(rename = "ownedNfts")]
    owned_nfts: Vec<Nft>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Nft {
    contract: Contract,
    id: NftId,
    title: Option<String>,
    description: Option<String>,
    media: Vec<Media>,
    #[serde(rename = "contractMetadata")]
    contract_metadata: ContractMetadata,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Contract {
    address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NftId {
    tokenId: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Media {
    gateway: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContractMetadata {
    #[serde(rename = "contractDeployer")]
    contract_deployer: String,
    #[serde(rename = "deployedBlockNumber")]
    deployed_block_number: u64,
    name: String,
    symbol: String,
    #[serde(rename = "tokenType")]
    token_type: String,
    #[serde(rename = "openSea")]
    open_sea: OpenSeaMetadata,  // New field to capture OpenSea data
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenSeaMetadata {
    #[serde(rename = "collectionName")]
    collection_name: String,
    #[serde(rename = "floorPrice")]
    floor_price: Option<f64>,  // Floor price is optional, in case it's not available
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NftSummary {
    pub name: String,
    pub symbol: String,
    pub description: Option<String>,
    pub media_urls: Vec<String>,
    pub collection_name: String,
    pub floor_price_eth: Option<f64>,
    pub last_traded_price_usd: Option<f64>,  // Optional because sales data might not always be available
}
pub async fn fetch_nft_data(chain: String, owner_address: &str) -> Result<AlchemyNftResponse> {
     /* 
    let api_key = "JfkCeSqLY74hGpZ28pXMa7sQg7aINFE1"; 

    let url = format!(
        "https://eth-mainnet.alchemyapi.io/v2/{}/getNFTs?owner={}",
        api_key, owner_address
    );*/
    
    dotenv().ok(); // Load environment variables from the .env file
    //let api_key = "JfkCeSqLY74hGpZ28pXMa7sQg7aINFE1"; 
    //let owner_address = "0xF039fbEfBA314ecF4Bf0C32bBe85f620C8C460D2";  

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

async fn print_nft_info(chain: String,response: &AlchemyNftResponse) {
    for nft in &response.owned_nfts {
        println!("NFT Title: {}", nft.title.clone().unwrap_or_else(|| "Unknown Title".to_string()));
        println!("Contract Address: {}", nft.contract.address);
        println!("Token ID: {}", nft.id.tokenId);
        
        // Print Contract Metadata
        println!("Contract Deployer: {}", nft.contract_metadata.contract_deployer);
        println!("Deployed Block Number: {}", nft.contract_metadata.deployed_block_number);
        println!("Symbol: {}", nft.contract_metadata.symbol);
        println!("Token Type: {}", nft.contract_metadata.token_type);

        // Print OpenSea metadata and floor price
        if let Some(floor_price) = nft.contract_metadata.open_sea.floor_price {
            println!("OpenSea Collection Name: {}", nft.contract_metadata.open_sea.collection_name);
            println!("OpenSea Floor Price: {} ETH", floor_price);
        } else {
            println!("Floor Price: Not available");
        }

        // Print Media URLs
        for media in &nft.media {
            println!("Media URL: {}", media.gateway);
        }

        // Convert token ID from hex to decimal
        let token_id_decimal = u128::from_str_radix(&nft.id.tokenId.trim_start_matches("0x"), 16)
            .expect("Failed to convert hex to decimal");
        let token_id_decimal_str = token_id_decimal.to_string();

        // Call the async function and await the result
        get_price_nft(chain.clone(),nft.contract.address.clone(), token_id_decimal_str).await;

        println!("\n-----------------------\n");
    }
}
async fn get_price_nft(chain: String, contract_address: String, token_id: String) -> Result<f64> {
    let api_key = "JfkCeSqLY74hGpZ28pXMa7sQg7aINFE1";
    let mut retry_count = 0;
    let max_retries = 5;
    let mut delay = 2; // Start with a 2-second delay
    dotenv().ok(); // Load environment variables from the .env file

    let env_var = format!("{}_RPC_URL", chain);
    let rpc_url = env::var(&env_var)
        .context(format!("{} not set", env_var))?;
    
    while retry_count < max_retries {
         
        let url = format!(
            "{}/getNFTSales?fromBlock=0&toBlock=latest&order=asc&contractAddress={}&tokenId={}",
            rpc_url, contract_address, token_id
        );

        
        /* 
        let url = format!(
            "https://eth-mainnet.g.alchemy.com/v2/{}/getNFTSales?fromBlock=0&toBlock=latest&order=asc&contractAddress={}&tokenId={}",
            api_key, contract_address, token_id
        );*/
        let client = Client::new();

        match client.get(&url).send().await {
            Ok(res) => {
                if res.status().is_success() {
                    let nft_response: Value = res.json().await.unwrap_or_default();
                    if let Some(nft_sales) = nft_response["nftSales"].as_array() {
                        if let Some(latest_sale) = nft_sales.last() {
                            let seller_fee_str = latest_sale["sellerFee"]["amount"].as_str().unwrap_or("0");
                            let protocol_fee_str = latest_sale["protocolFee"]["amount"].as_str().unwrap_or("0");
                            let royalty_fee_str = latest_sale["royaltyFee"]["amount"].as_str().unwrap_or("0");

                            let seller_fee = u128::from_str(seller_fee_str).unwrap_or(0);
                            let protocol_fee = u128::from_str(protocol_fee_str).unwrap_or(0);
                            let royalty_fee = u128::from_str(royalty_fee_str).unwrap_or(0);

                            let total_fee_wei = seller_fee + protocol_fee + royalty_fee;
                            let total_fee_eth = total_fee_wei as f64 / 1e18;

                            let eth_to_usd = fetch_eth_to_usd_price().await;
                            let total_fee_usd = total_fee_eth * eth_to_usd;

                            return Ok(total_fee_usd);
                        }
                    } else {
                        println!("No NFT sales found for contract: {}, token ID: {}", contract_address, token_id);
                    }
                    break; // Successful request, break out of retry loop
                } else if res.status().as_u16() == 429 { // 429 is "Too Many Requests"
                    //eprintln!("Rate limit exceeded, retrying...");
                    retry_count += 1;
                    sleep(Duration::from_secs(delay)).await;
                    delay = min(delay * 2, 64); // Exponential backoff with a cap
                } else {
                    eprintln!("Failed to fetch NFT sales: {:?}", res.status());
                    break;
                }
            },
            Err(e) => {
                eprintln!("Error fetching NFT sales: {:?}", e);
                retry_count += 1;
                sleep(Duration::from_secs(delay)).await;
                delay = min(delay * 2, 64);
            }
        }
    }
    
    if retry_count == max_retries {
        eprintln!("Max retries reached, aborting.");
    }
    Err(anyhow::anyhow!("Failed to fetch last traded price"))
}


// Function to fetch the current ETH to USD price
async fn fetch_eth_to_usd_price() -> f64 {
    let url = "https://min-api.cryptocompare.com/data/price?fsym=ETH&tsyms=USD";
    let client = Client::new();
    let res = client.get(url).send().await.unwrap();
    let price_response: Value = res.json().await.unwrap();
    
    price_response["USD"].as_f64().unwrap_or(0.0)
}
pub async fn fetch_nft_summary(chain: String, response: &AlchemyNftResponse) -> Result<Vec<NftSummary>> {
    let mut nft_summaries = Vec::new();

    for nft in &response.owned_nfts {
        let name = nft.contract_metadata.name.clone();
        let symbol = nft.contract_metadata.symbol.clone();
        let description = nft.description.clone();
        
        // Collect media URLs
        let media_urls: Vec<String> = nft.media.iter().map(|m| m.gateway.clone()).collect();

        // OpenSea data
        let collection_name = nft.contract_metadata.open_sea.collection_name.clone();
        let floor_price_eth = nft.contract_metadata.open_sea.floor_price;

        // Convert token ID from hex to decimal
        let token_id_decimal = u128::from_str_radix(&nft.id.tokenId.trim_start_matches("0x"), 16)
            .expect("Failed to convert hex to decimal");
        let token_id_decimal_str = token_id_decimal.to_string();

        // Fetch last traded price (in USD)
        let last_traded_price_usd = get_price_nft(chain.clone(), nft.contract.address.clone(), token_id_decimal_str).await.ok();

        // Create NftSummary and push it to the list
        let nft_summary = NftSummary {
            name,
            symbol,
            description,
            media_urls,
            collection_name,
            floor_price_eth,
            last_traded_price_usd,
        };

        nft_summaries.push(nft_summary);
    }

    Ok(nft_summaries)

}

//Previous Code: 
/*
use reqwest::Client;
use serde_json::Value;
use std::str::FromStr;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};
use std::cmp::min;
use web3::types::{Address};
use std::env;
use anyhow::{Context, Result}; // Import Result from anyhow
use dotenv::dotenv;

#[derive(Debug, Deserialize, Serialize)]
pub struct AlchemyNftResponse {
    #[serde(rename = "ownedNfts")]
    owned_nfts: Vec<Nft>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Nft {
    contract: Contract,
    id: NftId,
    title: Option<String>,
    description: Option<String>,
    media: Vec<Media>,
    #[serde(rename = "contractMetadata")]
    contract_metadata: ContractMetadata,
}

#[derive(Debug, Deserialize, Serialize)]
struct Contract {
    address: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct NftId {
    tokenId: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Media {
    gateway: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ContractMetadata {
    #[serde(rename = "contractDeployer")]
    contract_deployer: String,
    #[serde(rename = "deployedBlockNumber")]
    deployed_block_number: u64,
    name: String,
    symbol: String,
    #[serde(rename = "tokenType")]
    token_type: String,
    #[serde(rename = "openSea")]
    open_sea: OpenSeaMetadata,  // New field to capture OpenSea data
}

#[derive(Debug, Deserialize, Serialize)]
struct OpenSeaMetadata {
    #[serde(rename = "collectionName")]
    collection_name: String,
    #[serde(rename = "floorPrice")]
    floor_price: Option<f64>,  // Floor price is optional, in case it's not available
}

pub async fn fetch_nft_data(chain: String, owner_address: Address) -> Result<AlchemyNftResponse> {
    dotenv().ok(); // Load environment variables from the .env file
    //let api_key = "JfkCeSqLY74hGpZ28pXMa7sQg7aINFE1"; 
    //let owner_address = "0xF039fbEfBA314ecF4Bf0C32bBe85f620C8C460D2";  

    let env_var = format!("{}_RPC_URL", chain);
    let rpc_url = env::var(&env_var)
        .context(format!("{} not set", env_var))?;
    
    // Construct the URL using the environment variable
    let url = format!(
        "{}/getNFTs?owner={}",
        rpc_url, owner_address
    );
    /*let url = format!(
        "https://eth-mainnet.alchemyapi.io/v2/{}/getNFTs?owner={}",
        api_key, owner_address
    );*/
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
async fn print_nft_info(chain: String,response: AlchemyNftResponse) {
    for nft in response.owned_nfts {
        println!("NFT Title: {}", nft.title.unwrap_or_else(|| "Unknown Title".to_string()));
        println!("Contract Address: {}", nft.contract.address);
        println!("Token ID: {}", nft.id.tokenId);
        
        // Print Contract Metadata
        println!("Contract Deployer: {}", nft.contract_metadata.contract_deployer);
        println!("Deployed Block Number: {}", nft.contract_metadata.deployed_block_number);
        println!("Symbol: {}", nft.contract_metadata.symbol);
        println!("Token Type: {}", nft.contract_metadata.token_type);

        // Print OpenSea metadata and floor price
        if let Some(floor_price) = nft.contract_metadata.open_sea.floor_price {
            println!("OpenSea Collection Name: {}", nft.contract_metadata.open_sea.collection_name);
            println!("OpenSea Floor Price: {} ETH", floor_price);
        } else {
            println!("Floor Price: Not available");
        }

        // Print Media URLs
        for media in &nft.media {
            println!("Media URL: {}", media.gateway);
        }

        // Convert token ID from hex to decimal
        let token_id_decimal = u128::from_str_radix(&nft.id.tokenId.trim_start_matches("0x"), 16)
            .expect("Failed to convert hex to decimal");
        let token_id_decimal_str = token_id_decimal.to_string();

        // Call the async function and await the result
        get_price_nft(chain.clone(),nft.contract.address, token_id_decimal_str).await;

        println!("\n-----------------------\n");
    }
}

async fn get_price_nft(chain: String,contract_address: String, token_id: String) -> Result<()>{
    let api_key = "JfkCeSqLY74hGpZ28pXMa7sQg7aINFE1";
    let mut retry_count = 0;
    let max_retries = 5;
    let mut delay = 2; // Start with a 2-second delay
    dotenv().ok(); // Load environment variables from the .env file

    let env_var = format!("{}_RPC_URL", chain);
    let rpc_url = env::var(&env_var)
        .context(format!("{} not set", env_var))?;
    

    while retry_count < max_retries {

        let url = format!(
            "{}/getNFTSales?fromBlock=0&toBlock=latest&order=asc&contractAddress={}&tokenId={}",
            rpc_url,contract_address, token_id
        );
        let client = Client::new();

        match client.get(&url).send().await {
            Ok(res) => {
                if res.status().is_success() {
                    let nft_response: Value = res.json().await.unwrap_or_default();
                    if let Some(nft_sales) = nft_response["nftSales"].as_array() {
                        if let Some(latest_sale) = nft_sales.last() {
                            let seller_fee_str = latest_sale["sellerFee"]["amount"].as_str().unwrap_or("0");
                            let protocol_fee_str = latest_sale["protocolFee"]["amount"].as_str().unwrap_or("0");
                            let royalty_fee_str = latest_sale["royaltyFee"]["amount"].as_str().unwrap_or("0");

                            let seller_fee = u128::from_str(seller_fee_str).unwrap_or(0);
                            let protocol_fee = u128::from_str(protocol_fee_str).unwrap_or(0);
                            let royalty_fee = u128::from_str(royalty_fee_str).unwrap_or(0);

                            let total_fee_wei = seller_fee + protocol_fee + royalty_fee;
                            let total_fee_eth = total_fee_wei as f64 / 1e18;

                            let eth_to_usd = fetch_eth_to_usd_price().await;
                            let total_fee_usd = total_fee_eth * eth_to_usd;

                            println!("Total fee in ETH: {}", total_fee_eth);
                            println!("Total fee in USD: {}", total_fee_usd);
                        }
                    } else {
                        // If no sales are found, print floor price
                        println!("No NFT sales found for contract: {}, token ID: {}", contract_address, token_id);
                        // Optionally, you can also fetch and display the floor price here
                    }
                    break; // Successful request, break out of retry loop
                } else if res.status().as_u16() == 429 { // 429 is "Too Many Requests"
                    eprintln!("Rate limit exceeded, retrying...");
                    retry_count += 1;
                    sleep(Duration::from_secs(delay)).await;
                    delay = min(delay * 2, 64); // Exponential backoff with a cap
                } else {
                    eprintln!("Failed to fetch NFT sales: {:?}", res.status());
                    let body = res.text().await.unwrap_or_default();
                    eprintln!("Response body: {}", body);
                    break; // Some other error, exit retry loop
                }
            },
            Err(e) => {
                eprintln!("Error fetching NFT sales: {:?}", e);
                retry_count += 1;
                sleep(Duration::from_secs(delay)).await;
                delay = min(delay * 2, 64);
            }
        }
    }
    
    if retry_count == max_retries {
        eprintln!("Max retries reached, aborting.");
    }
    Ok(())
}

// Function to fetch the current ETH to USD price
async fn fetch_eth_to_usd_price() -> f64 {
    let url = "https://min-api.cryptocompare.com/data/price?fsym=ETH&tsyms=USD";
    let client = Client::new();
    let res = client.get(url).send().await.unwrap();
    let price_response: Value = res.json().await.unwrap();
    
    price_response["USD"].as_f64().unwrap_or(0.0)
}
*/
/* 
#[tokio::main]
async fn main() -> Result<()> {
    let chain = "ETHEREUM".to_string();  // or another supported chain
    let owner_address = "0xF039fbEfBA314ecF4Bf0C32bBe85f620C8C460D2";
/* 
    let chain = "BASE".to_string();
    let owner_address = "0x91050Bf9F1B810093950456349e0C2D797F0b930";
*/
    // Fetch NFT data
    let nft_response = fetch_nft_data(chain.clone(), owner_address)
        .await
        .context("Failed to fetch NFT data")?;

    // Print the fetched NFT information
    //print_nft_info(chain.clone(), &nft_response).await;

    // Fetch NFT summaries
    let nft_summaries = fetch_nft_summary(chain.clone(), &nft_response)
    .await
    .context("Failed to fetch NFT summaries")?;

    // Print each NFT summary
    for nft_summary in &nft_summaries {
        println!("{:#?}", nft_summary);
    }

    Ok(())
}
*/

/*
pub async fn fetch_nft_data(chain: String, owner_address: Address) -> Result<AlchemyNftResponse> {
    dotenv().ok(); // Load environment variables from the .env file
    let api_key = "JfkCeSqLY74hGpZ28pXMa7sQg7aINFE1"; 
    //let owner_address = "0xF039fbEfBA314ecF4Bf0C32bBe85f620C8C460D2";  

    //let env_var = format!("{}_RPC_URL", chain);
    //let rpc_url = env::var(&env_var)
      //  .context(format!("{} not set", env_var))?;
    
    // Construct the URL using the environment variable
    /*let url = format!(
        "{}/getNFTs?owner={}",
        rpc_url, owner_address
    );*/
    let url = format!(
        "https://eth-mainnet.alchemyapi.io/v2/{}/getNFTs?owner={}",
        api_key, owner_address
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
*/
/*use reqwest::Client;
use serde_json::Value;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};
use std::cmp::min;
use web3::types::Address;
use std::env;
use anyhow::{Context, Result}; // Import Result from anyhow
use dotenv::dotenv;

#[derive(Debug, Deserialize, Serialize)]
pub struct AlchemyNftResponse {
    #[serde(rename = "ownedNfts")]
    owned_nfts: Vec<Nft>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Nft {
    contract: Contract,
    id: NftId,
    title: Option<String>,
    description: Option<String>,
    media: Vec<Media>,
    #[serde(rename = "contractMetadata")]
    contract_metadata: ContractMetadata,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Contract {
    address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NftId {
    tokenId: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Media {
    gateway: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContractMetadata {
    #[serde(rename = "contractDeployer")]
    contract_deployer: String,
    #[serde(rename = "deployedBlockNumber")]
    deployed_block_number: u64,
    name: String,
    symbol: String,
    #[serde(rename = "tokenType")]
    token_type: String,
    #[serde(rename = "openSea")]
    open_sea: OpenSeaMetadata,  // New field to capture OpenSea data
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenSeaMetadata {
    #[serde(rename = "collectionName")]
    collection_name: String,
    #[serde(rename = "floorPrice")]
    floor_price: Option<f64>,  // Floor price is optional, in case it's not available
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NftSummary {
    pub name: String,
    pub symbol: String,
    pub description: Option<String>,
    pub media_urls: Vec<String>,
    pub collection_name: String,
    pub floor_price_eth: Option<f64>,
    pub last_traded_price_usd: Option<f64>,  // Optional because sales data might not always be available
}

pub async fn fetch_nft_data(chain: String, owner_address: Address) -> Result<AlchemyNftResponse> {
    dotenv().ok(); // Load environment variables from the .env file
    //let api_key = "JfkCeSqLY74hGpZ28pXMa7sQg7aINFE1"; 
    //let owner_address = "0xF039fbEfBA314ecF4Bf0C32bBe85f620C8C460D2";  

    let env_var = format!("{}_RPC_URL", chain);
    let rpc_url = env::var(&env_var)
        .context(format!("{} not set", env_var))?;
    
    // Construct the URL using the environment variable
    let url = format!(
        "{}/getNFTs?owner={}",
        rpc_url, owner_address
    );
    /*let url = format!(
        "https://eth-mainnet.alchemyapi.io/v2/{}/getNFTs?owner={}",
        api_key, owner_address
    );*/
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
async fn print_nft_info(chain: String,response: AlchemyNftResponse) {
    for nft in response.owned_nfts {
        println!("NFT Title: {}", nft.title.unwrap_or_else(|| "Unknown Title".to_string()));
        println!("Contract Address: {}", nft.contract.address);
        println!("Token ID: {}", nft.id.tokenId);
        
        // Print Contract Metadata
        println!("Contract Deployer: {}", nft.contract_metadata.contract_deployer);
        println!("Deployed Block Number: {}", nft.contract_metadata.deployed_block_number);
        println!("Symbol: {}", nft.contract_metadata.symbol);
        println!("Token Type: {}", nft.contract_metadata.token_type);

        // Print OpenSea metadata and floor price
        if let Some(floor_price) = nft.contract_metadata.open_sea.floor_price {
            println!("OpenSea Collection Name: {}", nft.contract_metadata.open_sea.collection_name);
            println!("OpenSea Floor Price: {} ETH", floor_price);
        } else {
            println!("Floor Price: Not available");
        }

        // Print Media URLs
        for media in &nft.media {
            println!("Media URL: {}", media.gateway);
        }

        // Convert token ID from hex to decimal
        let token_id_decimal = u128::from_str_radix(&nft.id.tokenId.trim_start_matches("0x"), 16)
            .expect("Failed to convert hex to decimal");
        let token_id_decimal_str = token_id_decimal.to_string();

        // Call the async function and await the result
        get_price_nft(chain.clone(),nft.contract.address, token_id_decimal_str).await;

        println!("\n-----------------------\n");
    }
}
async fn get_price_nft(chain: String, contract_address: String, token_id: String) -> Result<f64> {
    //let api_key = "JfkCeSqLY74hGpZ28pXMa7sQg7aINFE1";
    let mut retry_count = 0;
    let max_retries = 5;
    let mut delay = 2; // Start with a 2-second delay
    dotenv().ok(); // Load environment variables from the .env file

    let env_var = format!("{}_RPC_URL", chain);
    let rpc_url = env::var(&env_var)
        .context(format!("{} not set", env_var))?;
    
    while retry_count < max_retries {
        let url = format!(
            "{}/getNFTSales?fromBlock=0&toBlock=latest&order=asc&contractAddress={}&tokenId={}",
            rpc_url, contract_address, token_id
        );
        let client = Client::new();

        match client.get(&url).send().await {
            Ok(res) => {
                if res.status().is_success() {
                    let nft_response: Value = res.json().await.unwrap_or_default();
                    if let Some(nft_sales) = nft_response["nftSales"].as_array() {
                        if let Some(latest_sale) = nft_sales.last() {
                            let seller_fee_str = latest_sale["sellerFee"]["amount"].as_str().unwrap_or("0");
                            let protocol_fee_str = latest_sale["protocolFee"]["amount"].as_str().unwrap_or("0");
                            let royalty_fee_str = latest_sale["royaltyFee"]["amount"].as_str().unwrap_or("0");

                            let seller_fee = u128::from_str(seller_fee_str).unwrap_or(0);
                            let protocol_fee = u128::from_str(protocol_fee_str).unwrap_or(0);
                            let royalty_fee = u128::from_str(royalty_fee_str).unwrap_or(0);

                            let total_fee_wei = seller_fee + protocol_fee + royalty_fee;
                            let total_fee_eth = total_fee_wei as f64 / 1e18;

                            let eth_to_usd = fetch_eth_to_usd_price().await;
                            let total_fee_usd = total_fee_eth * eth_to_usd;

                            return Ok(total_fee_usd);
                        }
                    } else {
                        println!("No NFT sales found for contract: {}, token ID: {}", contract_address, token_id);
                    }
                    break; // Successful request, break out of retry loop
                } else if res.status().as_u16() == 429 { // 429 is "Too Many Requests"
                    eprintln!("Rate limit exceeded, retrying...");
                    retry_count += 1;
                    sleep(Duration::from_secs(delay)).await;
                    delay = min(delay * 2, 64); // Exponential backoff with a cap
                } else {
                    eprintln!("Failed to fetch NFT sales: {:?}", res.status());
                    break;
                }
            },
            Err(e) => {
                eprintln!("Error fetching NFT sales: {:?}", e);
                retry_count += 1;
                sleep(Duration::from_secs(delay)).await;
                delay = min(delay * 2, 64);
            }
        }
    }
    
    if retry_count == max_retries {
        eprintln!("Max retries reached, aborting.");
    }
    Err(anyhow::anyhow!("Failed to fetch last traded price"))
}


// Function to fetch the current ETH to USD price
async fn fetch_eth_to_usd_price() -> f64 {
    let url = "https://min-api.cryptocompare.com/data/price?fsym=ETH&tsyms=USD";
    let client = Client::new();
    let res = client.get(url).send().await.unwrap();
    let price_response: Value = res.json().await.unwrap();
    
    price_response["USD"].as_f64().unwrap_or(0.0)
}
pub async fn fetch_nft_summary(chain: String, response: AlchemyNftResponse) -> Result<Vec<NftSummary>> {
    let mut nft_summaries = Vec::new();

    for nft in response.owned_nfts {
        let name = nft.contract_metadata.name;
        let symbol = nft.contract_metadata.symbol;
        let description = nft.description.clone();
        
        // Collect media URLs
        let media_urls: Vec<String> = nft.media.iter().map(|m| m.gateway.clone()).collect();

        // OpenSea data
        let collection_name = nft.contract_metadata.open_sea.collection_name;
        let floor_price_eth = nft.contract_metadata.open_sea.floor_price;

        // Convert token ID from hex to decimal
        let token_id_decimal = u128::from_str_radix(&nft.id.tokenId.trim_start_matches("0x"), 16)
            .expect("Failed to convert hex to decimal");
        let token_id_decimal_str = token_id_decimal.to_string();

        // Fetch last traded price (in USD)
        let last_traded_price_usd = get_price_nft(chain.clone(), nft.contract.address.clone(), token_id_decimal_str).await.ok();

        // Create NftSummary and push it to the list
        let nft_summary = NftSummary {
            name,
            symbol,
            description,
            media_urls,
            collection_name,
            floor_price_eth,
            last_traded_price_usd,
        };

        nft_summaries.push(nft_summary);
    }

    Ok(nft_summaries)

}
*/
//Use below to execute 
/* 
let response = fetch_nft_data(chain.clone(), owner_address).await?;
let nft_summaries = fetch_nft_summary(chain.clone(), response).await;
for nft_summary in nft_summaries {
    println!("{:#?}", nft_summary);
}
*/

/*use reqwest::Client;
use serde_json::Value;
use std::str::FromStr;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};
use std::cmp::min;
use web3::types::{Address};
use std::env;
use anyhow::{Context, Result}; // Import Result from anyhow
use dotenv::dotenv;

#[derive(Debug, Deserialize, Serialize)]
pub struct AlchemyNftResponse {
    #[serde(rename = "ownedNfts")]
    owned_nfts: Vec<Nft>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Nft {
    contract: Contract,
    id: NftId,
    title: Option<String>,
    description: Option<String>,
    media: Vec<Media>,
    #[serde(rename = "contractMetadata")]
    contract_metadata: ContractMetadata,
}

#[derive(Debug, Deserialize, Serialize)]
struct Contract {
    address: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct NftId {
    tokenId: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Media {
    gateway: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ContractMetadata {
    #[serde(rename = "contractDeployer")]
    contract_deployer: String,
    #[serde(rename = "deployedBlockNumber")]
    deployed_block_number: u64,
    name: String,
    symbol: String,
    #[serde(rename = "tokenType")]
    token_type: String,
    #[serde(rename = "openSea")]
    open_sea: OpenSeaMetadata,  // New field to capture OpenSea data
}

#[derive(Debug, Deserialize, Serialize)]
struct OpenSeaMetadata {
    #[serde(rename = "collectionName")]
    collection_name: String,
    #[serde(rename = "floorPrice")]
    floor_price: Option<f64>,  // Floor price is optional, in case it's not available
}

pub async fn fetch_nft_data(chain: String, owner_address: Address) -> Result<AlchemyNftResponse> {
    dotenv().ok(); // Load environment variables from the .env file
    //let api_key = "JfkCeSqLY74hGpZ28pXMa7sQg7aINFE1"; 
    //let owner_address = "0xF039fbEfBA314ecF4Bf0C32bBe85f620C8C460D2";  

    let env_var = format!("{}_RPC_URL", chain);
    let rpc_url = env::var(&env_var)
        .context(format!("{} not set", env_var))?;
    
    // Construct the URL using the environment variable
    let url = format!(
        "{}/getNFTs?owner={}",
        rpc_url, owner_address
    );
    /*let url = format!(
        "https://eth-mainnet.alchemyapi.io/v2/{}/getNFTs?owner={}",
        api_key, owner_address
    );*/
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
async fn print_nft_info(chain: String,response: AlchemyNftResponse) {
    for nft in response.owned_nfts {
        println!("NFT Title: {}", nft.title.unwrap_or_else(|| "Unknown Title".to_string()));
        println!("Contract Address: {}", nft.contract.address);
        println!("Token ID: {}", nft.id.tokenId);
        
        // Print Contract Metadata
        println!("Contract Deployer: {}", nft.contract_metadata.contract_deployer);
        println!("Deployed Block Number: {}", nft.contract_metadata.deployed_block_number);
        println!("Symbol: {}", nft.contract_metadata.symbol);
        println!("Token Type: {}", nft.contract_metadata.token_type);

        // Print OpenSea metadata and floor price
        if let Some(floor_price) = nft.contract_metadata.open_sea.floor_price {
            println!("OpenSea Collection Name: {}", nft.contract_metadata.open_sea.collection_name);
            println!("OpenSea Floor Price: {} ETH", floor_price);
        } else {
            println!("Floor Price: Not available");
        }

        // Print Media URLs
        for media in &nft.media {
            println!("Media URL: {}", media.gateway);
        }

        // Convert token ID from hex to decimal
        let token_id_decimal = u128::from_str_radix(&nft.id.tokenId.trim_start_matches("0x"), 16)
            .expect("Failed to convert hex to decimal");
        let token_id_decimal_str = token_id_decimal.to_string();

        // Call the async function and await the result
        get_price_nft(chain.clone(),nft.contract.address, token_id_decimal_str).await;

        println!("\n-----------------------\n");
    }
}

async fn get_price_nft(chain: String,contract_address: String, token_id: String) -> Result<()>{
    //let api_key = "JfkCeSqLY74hGpZ28pXMa7sQg7aINFE1";
    let mut retry_count = 0;
    let max_retries = 5;
    let mut delay = 2; // Start with a 2-second delay
    dotenv().ok(); // Load environment variables from the .env file

    let env_var = format!("{}_RPC_URL", chain);
    let rpc_url = env::var(&env_var)
        .context(format!("{} not set", env_var))?;
    

    while retry_count < max_retries {

        let url = format!(
            "{}/getNFTSales?fromBlock=0&toBlock=latest&order=asc&contractAddress={}&tokenId={}",
            rpc_url,contract_address, token_id
        );
        let client = Client::new();

        match client.get(&url).send().await {
            Ok(res) => {
                if res.status().is_success() {
                    let nft_response: Value = res.json().await.unwrap_or_default();
                    if let Some(nft_sales) = nft_response["nftSales"].as_array() {
                        if let Some(latest_sale) = nft_sales.last() {
                            let seller_fee_str = latest_sale["sellerFee"]["amount"].as_str().unwrap_or("0");
                            let protocol_fee_str = latest_sale["protocolFee"]["amount"].as_str().unwrap_or("0");
                            let royalty_fee_str = latest_sale["royaltyFee"]["amount"].as_str().unwrap_or("0");

                            let seller_fee = u128::from_str(seller_fee_str).unwrap_or(0);
                            let protocol_fee = u128::from_str(protocol_fee_str).unwrap_or(0);
                            let royalty_fee = u128::from_str(royalty_fee_str).unwrap_or(0);

                            let total_fee_wei = seller_fee + protocol_fee + royalty_fee;
                            let total_fee_eth = total_fee_wei as f64 / 1e18;

                            let eth_to_usd = fetch_eth_to_usd_price().await;
                            let total_fee_usd = total_fee_eth * eth_to_usd;

                            println!("Total fee in ETH: {}", total_fee_eth);
                            println!("Total fee in USD: {}", total_fee_usd);
                        }
                    } else {
                        // If no sales are found, print floor price
                        println!("No NFT sales found for contract: {}, token ID: {}", contract_address, token_id);
                        // Optionally, you can also fetch and display the floor price here
                    }
                    break; // Successful request, break out of retry loop
                } else if res.status().as_u16() == 429 { // 429 is "Too Many Requests"
                    eprintln!("Rate limit exceeded, retrying...");
                    retry_count += 1;
                    sleep(Duration::from_secs(delay)).await;
                    delay = min(delay * 2, 64); // Exponential backoff with a cap
                } else {
                    eprintln!("Failed to fetch NFT sales: {:?}", res.status());
                    let body = res.text().await.unwrap_or_default();
                    eprintln!("Response body: {}", body);
                    break; // Some other error, exit retry loop
                }
            },
            Err(e) => {
                eprintln!("Error fetching NFT sales: {:?}", e);
                retry_count += 1;
                sleep(Duration::from_secs(delay)).await;
                delay = min(delay * 2, 64);
            }
        }
    }
    
    if retry_count == max_retries {
        eprintln!("Max retries reached, aborting.");
    }
    Ok(())
}

// Function to fetch the current ETH to USD price
async fn fetch_eth_to_usd_price() -> f64 {
    let url = "https://min-api.cryptocompare.com/data/price?fsym=ETH&tsyms=USD";
    let client = Client::new();
    let res = client.get(url).send().await.unwrap();
    let price_response: Value = res.json().await.unwrap();
    
    price_response["USD"].as_f64().unwrap_or(0.0)
}
 */