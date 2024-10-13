use serde::{Deserialize, Serialize};
use web3::ethabi::Address;
use web3::{Result, Error};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::token_identifiers::{erc20_token_identifier, erc20_token_identifier::UserDetails};
use crate::transaction_history::logs_fetcher;
use crate::database::reader;

pub async fn web_route_erc20(request_body: web::Json<PortfolioRequest>) -> impl Responder {
    let user_details = match get_erc20_portfolio_data(request_body.user_address, request_body.chain.clone()).await {
        Ok(r) => r,
        Err(e) => return HttpResponse::BadRequest().json(json!({
            "user_details": null,
            "err": e.to_string(),
        })) 
    };

    let result = PortfolioResponse{
        user_details: Some(user_details),
        err: None,
    };

    HttpResponse::Ok().json(result)
}

pub async fn get_erc20_portfolio_data(user_address: Address, chain: String) -> Result<Vec<UserDetails>>{
    let logs = match logs_fetcher::fetch_transaction_logs(user_address, chain.clone()).await{
        Ok(l) => l,
        Err(e) => return Err(Error::from(e.to_string())),
    };
    
    let token_contract_address_list = match reader::get_contract_address_list_from_logs(logs){
        Ok(t) => t,
        Err(e) => return Err(Error::from(e.to_string())),
    };

    let user_details = erc20_token_identifier::get_user_details(token_contract_address_list, user_address, chain).await?;
    Ok(user_details)
}

#[derive(Deserialize)]
pub struct PortfolioRequest {
    pub user_address: Address,
    pub chain: String,
}

#[derive(Serialize)]
pub struct PortfolioResponse {
    pub user_details: Option<Vec<UserDetails>>,
    pub err: Option<String>,
}