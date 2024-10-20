//let url = format!("https://base-mainnet.g.alchemy.com/v2/JfkCeSqLY74hGpZ28pXMa7sQg7aINFE1/getNFTs?owner=0x439c36f21d961Dc81Bfb39331845FbDC8C9E8be8");
use serde::{Deserialize, Serialize};
use anyhow::{Context, Result}; // Import Result from anyhow
use crate::token_identifiers::nft_identifier::NftSummary;
#[derive(Debug, Serialize, Deserialize)]
pub struct AlchemyNftResponse {
    #[serde(rename = "ownedNfts")]
    pub owned_nfts: Vec<OwnedNft>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnedNft {
    pub contract: Contract,
    pub id: TokenId,
    pub metadata: Option<Metadata>, // Metadata may be absent in some cases
    pub media: Option<Vec<Media>>,   // Updated to reflect actual structure
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenId {
    #[serde(rename = "tokenId")]
    pub token_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Media {
    pub gateway: Option<String>,
    pub raw: Option<String>,
}

#[derive(Deserialize, Debug)]
struct ReservoirPriceResponse {
    sales: Vec<Sale>,
}

#[derive(Deserialize, Debug)]
struct Sale {
    price: Price,
    token: TokenData,
}

#[derive(Deserialize, Debug)]
struct Price {
    amount: Amount,
}

#[derive(Deserialize, Debug)]
struct Amount {
    usd: f64,
}

#[derive(Deserialize, Debug)]
struct TokenData {
    collection: Collection,
}

#[derive(Deserialize, Debug)]
struct Collection {
    name: Option<String>,
}

pub async fn base_fetch_nft_data( owner_address: &str) -> Result<AlchemyNftResponse> {
    let url = format!("https://base-mainnet.g.alchemy.com/v2/JfkCeSqLY74hGpZ28pXMa7sQg7aINFE1/getNFTs?owner={}",owner_address);

    let client = reqwest::Client::new();
    let res = client
        .get(&url)
        .send()
        .await
        .expect("Failed to send request");

        if res.status().is_success() {
            let nft_response: AlchemyNftResponse = res.json().await.context("Failed to parse response")?;
            Ok(nft_response) // Return the response
        } else {
            let status_code = res.status();
            Err(anyhow::anyhow!("Request failed with status: {}", status_code)) // Return an error
        }
}
pub async fn base_fetch_nft_summary( response: &AlchemyNftResponse) -> Result<Vec<NftSummary>> {
    let mut nft_summaries = Vec::new();
    for nft in &response.owned_nfts {
        let contract_address = &nft.contract.address;
        let token_id = &nft.id.token_id;

        // Default values if metadata is missing
        let name = nft.metadata.as_ref().and_then(|m| m.name.clone()).unwrap_or_else(|| "Unknown".to_string());
        let symbol = nft.metadata.as_ref().and_then(|m| m.symbol.clone()).unwrap_or_else(|| "Unknown".to_string());
        let description = nft.metadata.as_ref().and_then(|m| m.description.clone());

        // Collect media URLs
        let media_urls = nft.media.clone().unwrap_or_else(Vec::new)
            .into_iter()
            .filter_map(|m| m.gateway.clone().or(m.raw.clone()))
            .collect::<Vec<String>>();
        let client = reqwest::Client::new();


        // Fetch the price and collection name in USD
        match fetch_nft_price(&client, contract_address, token_id).await {
            Ok((price, collection_name)) => {
                let collection_name = collection_name.unwrap_or_else(|| "Unknown".to_string());

                let nft_summary = NftSummary {
                    name,
                    symbol,
                    description,
                    media_urls,
                    collection_name,
                    floor_price_eth: None,
                    last_traded_price_usd: Some(price),
                };

                nft_summaries.push(nft_summary);
            }
            Err(e) => eprintln!("Failed to fetch price for {}: {}", name, e),
        }
    }

    for nft_summary in &nft_summaries {
        println!("{:#?}", nft_summary);
    }

    Ok(nft_summaries)

}


async fn fetch_nft_price(client: &reqwest::Client, contract_address: &str, token_id: &str) -> Result<(f64, Option<String>), Box<dyn std::error::Error>> {
    let token_id_decimal = u64::from_str_radix(token_id.trim_start_matches("0x"), 16)?;

    let url = format!("https://api-base.reservoir.tools/sales/v6?tokens={}%3A{}", contract_address, token_id_decimal);
    
    let res = client.get(&url)
        .send()
        .await?
        .json::<ReservoirPriceResponse>()
        .await?;
    
    if let Some(sale) = res.sales.first() {
        let price = sale.price.amount.usd;
        let collection_name = sale.token.collection.name.clone();
        Ok((price, collection_name))
    } else {
        Err("No sales data found".into())
    }
}