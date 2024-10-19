use serde::{Deserialize, Serialize};
use web3::ethabi::Address;
use web3::{Result, Error};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::price_feeds::price_feed::portfolio_value;
use crate::token_identifiers::{erc20_token_identifier, erc20_token_identifier::UserDetails};
use crate::transaction_history::logs_fetcher;
use crate::database::{self, reader};
use crate::portfolio_overview::token_contract_addresses;

pub async fn web_route_erc20(request_body: web::Json<PortfolioRequest>) -> impl Responder {
    let address: Address = request_body.user_address.parse().expect("Invalid Address");
    let user_details: UserDetailsResponse;

    if request_body.chain.clone() == "ETHEREUM" {
        user_details = match get_erc20_portfolio_data(address, request_body.chain.clone()).await {
            Ok(r) => r,
            Err(e) => return HttpResponse::BadRequest().json(json!({
                "user_details": null,
                "err": e.to_string(),
            })) 
        };
    } else if request_body.chain.clone() == "BASE" {
        user_details = match get_base_erc20_portfolio_data(address).await {
            Ok(r) => r,
            Err(e) => return HttpResponse::BadRequest().json(json!({
                "user_details": null,
                "err": e.to_string(),
            })) 
        };
    } else {
        user_details = UserDetailsResponse{
            ..Default::default()
        };
    }


    let result = PortfolioResponse{
        user_details: Some(user_details),
        err: None,
    };

    HttpResponse::Ok().json(result)
}

pub async fn get_erc20_portfolio_data(user_address: Address, chain: String) -> Result<UserDetailsResponse>{
    let logs = match logs_fetcher::fetch_transaction_logs(user_address, chain.clone()).await{
        Ok(l) => {println!("Fetched transaction logs"); l},
        Err(e) => return Err(Error::from(e.to_string())),
    };

    match database::writer::write_to_db(logs, chain.clone(), user_address).await {
        Ok(_) => 1,
        Err(e) => return Err(Error::from(e.to_string())),
    };
    
    let logs = match database::reader::read_from_db(chain.clone(), user_address).await {
        Ok(r) => r,
        Err(e) => return Err(Error::from(e.to_string())),
    };
    
    let token_contract_address_list = match reader::get_contract_address_list_from_logs(logs){
        Ok(t) => {println!("Fetched contract address list"); t},
        Err(e) => return Err(Error::from(e.to_string())),
    };

    let user_details = erc20_token_identifier::get_user_details(token_contract_address_list.clone(), user_address, chain.clone()).await?;
    let user_portfolio_value = portfolio_value(token_contract_address_list, user_address, chain).await.unwrap();

    println!("Got user details");
    
    Ok(UserDetailsResponse{
        details: user_details,
        portfolio_value: user_portfolio_value,
    })
}

pub async fn get_base_erc20_portfolio_data(user_address: Address) -> Result<UserDetailsResponse>{
    let contract_address_list = token_contract_addresses::make_contract_address_list(token_contract_addresses::BASE_HEX_ADDRESS_LIST.to_vec());

    let user_details = erc20_token_identifier::get_user_details(contract_address_list.clone(), user_address, "BASE".to_string()).await?;
    let user_portfolio_value = portfolio_value(contract_address_list, user_address, "BASE".to_string()).await.unwrap();
    
    println!("Got user details");
    
    Ok(UserDetailsResponse{
        details: user_details,
        portfolio_value: user_portfolio_value,
    })
}

#[derive(Deserialize)]
pub struct PortfolioRequest {
    pub user_address: String,
    pub chain: String,
}

#[derive(Serialize)]
pub struct PortfolioResponse {
    pub user_details: Option<UserDetailsResponse>,
    pub err: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct UserDetailsResponse {
    pub details: Vec<UserDetails>,
    pub portfolio_value: f64,
}