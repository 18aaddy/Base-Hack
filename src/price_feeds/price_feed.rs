use crate::token_identifiers::erc20_token_identifier;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use web3::types::{Address, H160};
#[derive(Deserialize, Serialize)]
pub struct PriceResponse {
    pub USD: Option<f64>, // or U256 if you prefer to handle it as a big integer
}

#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    pub Response: String,
    pub Message: String,
}

pub async fn fetch_token_price(token_symbol: &str) -> Result<Option<f64>, Box<dyn Error>> {
    let url = format!(
        "https://min-api.cryptocompare.com/data/price?fsym={}&tsyms=USD",
        token_symbol
    );
    let client = Client::new();
    
    // Fetch the response body as a string
    let response = client.get(&url).send().await?;
    let response_body = response.text().await?;

    // Debug: Print the full response body
    println!("Response body: {}", response_body);

    // Check if the response contains an error
    if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&response_body) {
        if error_response.Response == "Error" {
            return Ok(None);
        }
    }

    // Deserialize the success response containing the USD price
    let price_response: PriceResponse = serde_json::from_str(&response_body)?;

    if let Some(usd_price) = price_response.USD {
        return Ok(Some(usd_price))
    } 
    Ok(None)
}
pub async fn portfolio_value(rpc_url: &String, address_list: Vec<H160>, user_address: Address, chain: String) -> Result<f64, Box<dyn std::error::Error>> {
    let user_details = erc20_token_identifier::get_user_details(rpc_url, address_list, user_address, chain).await?;
    let mut portfolio_value_user = 0.0;

    for user in user_details {
        let token_price = match fetch_token_price(&user.token_symbol).await? {
            Some(r) => r,
            None => 0f64,
        };
        let token_balance_f64: f64 = user.token_balance.to_string().parse().unwrap();
        let token_decimals_f64: f64 = user.token_decimals.to_string().parse().unwrap();
        let token_value = token_balance_f64 * token_price / (10f64.powf(token_decimals_f64));
        portfolio_value_user += token_value;
    }

    Ok(portfolio_value_user)
}
