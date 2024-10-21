
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
    dotenv().ok(); // Load environment variables from the .env file

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

async fn get_price_nft(chain: String, contract_address: String, token_id: String) -> Result<f64> {
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