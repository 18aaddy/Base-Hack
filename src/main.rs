use actix_web::{App, HttpServer};
use actix_cors::Cors;

mod database;
mod transaction_history;
mod price_feeds;
mod token_identifiers;
mod portfolio_overview;
mod routes;

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
//     let address: Address = Address::from_str("0x87d2E27147A640092e06c9329bF8110ca9927446").unwrap();

//     let result = logs_fetcher::fetch_transaction_logs(address, "BASE".to_string()).await;
//     match result {
//         Ok(r) => match database::writer::write_to_db(r.clone(), "BASE".to_string(), address).await {
//             Ok(_) => println!("Number of logs: {}", r.len()),
//             Err(e) => println!("Error: {}", e),
//         },
//         Err(e) => println!("Error: {}", e),
//     }

//     let contract_address: Address = Address::from_str("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48").unwrap();
//     let result = erc20_token_identifier::erc20_identifier(contract_address, "BASE".to_string()).await;
//     match result {
//         Ok((r, s, v)) => println!("Name of token: {}, Symbol {}, Decimals {}", r, s, v),
//         Err(e) => println!("Error: {}", e),
//     }
//     // let token_contract_address = Address::from_str("0x1195Cf65f83B3A5768F3C496D3A05AD6412c64B7").unwrap();
//     // let user_address = Address::from_str("0x8B1510D9aaF015F23ACF13E328Ffb5AB065C5bd9").unwrap();

//     // erc721_identifier(token_contract_address, user_address, 10).await;
// }