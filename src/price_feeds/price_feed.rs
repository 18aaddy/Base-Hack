use crate::transaction_history::erc20_token_identifier;
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use web3::types::{Address, CallRequest, H160, U256};
#[derive(Deserialize)]
pub struct PriceResponse {
    pub USD: f64, // or U256 if you prefer to handle it as a big integer
}

pub async fn fetch_token_price(token_symbol: &str) -> Result<f64, Box<dyn Error>> {
    let url = format!("https://min-api.cryptocompare.com/data/price?fsym={}&tsyms=USD", token_symbol);
    let client = Client::new();
    let response: PriceResponse = client.get(&url).send().await?.json().await?;
    Ok(response.USD)
}
pub async fn portfolio_value(address_list: Vec<H160>, user_address: Address, chain_id: u64) -> Result<f64, Box<dyn std::error::Error>> {
    let user_details = erc20_token_identifier::get_data_of_token_from_address_list(address_list, user_address, chain_id).await?;
    let mut portfolio_value = 0.0;

    for user in user_details {
        let token_price = fetch_token_price(&user.token_symbol).await?;
        let token_balance_f64: f64 = user.token_balance.to_string().parse().unwrap();
        let token_value = token_balance_f64 * token_price;
        portfolio_value += token_value;
    }

    Ok(portfolio_value)
}
