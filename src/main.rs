use std::char::from_u32;

use actix_web::{App, HttpServer};
use actix_cors::Cors;
use web3::types::{Address, U256};
use hex;

mod database;
mod transaction_history;
mod price_feeds;
mod token_identifiers;
mod portfolio_overview;
mod routes;

use transaction_history::logs_fetcher;
use token_identifiers::erc20_token_identifier::{self, UserDetails};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Started server at 127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())  
            .configure(routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


// #[tokio::main]
// async fn main() {
//     let address: Address = "0xC5f32bb698412eB964a2E29193107832c38C70F6".parse().expect("Error decoding address string");
//     // let token_address: Address = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".parse().expect("Error decoding address string");

//     let result = match portfolio_overview::erc20_portfolio_tracker::get_erc20_portfolio_data(address, "BASE".to_string()).await {
//         Ok(r) => r,
//         Err(e) => {println!("Error: {:?}", e); Vec::<UserDetails>::new()},
//     };
//     println!("{:?}", result);

//     let result = match token_identifiers::erc20_token_identifier::get_eth_amount_of_user(address, "ETHEREUM".to_string()).await {
//         Ok(r) => r,
//         Err(e) => {println!("Error: {:?}", e); U256::from_dec_str("0").unwrap()},
//     };
//     println!("{:?}", result);

//     // let result = match token_identifiers::erc20_token_identifier::get_user_details(token_address, address, "ETHEREUM".to_string()).await {
//     //     Ok(r) => r,
//     //     Err(e) => {println!("Error: {:?}", e); ("".to_string(), "".to_string(), 0u8, U256::from_dec_str("0").unwrap())},
//     // };
//     // println!("{:?}", result);

//     // let token_contract_address = Address::from_str("0x1195Cf65f83B3A5768F3C496D3A05AD6412c64B7").unwrap();
//     // let user_address = Address::from_str("0x8B1510D9aaF015F23ACF13E328Ffb5AB065C5bd9").unwrap();

//     // erc721_identifier(token_contract_address, user_address, 10).await;
// }