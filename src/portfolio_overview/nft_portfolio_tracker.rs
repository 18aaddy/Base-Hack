use serde::Serialize;
use web3::{ethabi::Address, Error, Result};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::token_identifiers::nft_identifier::{self, AlchemyNftResponse};
use crate::portfolio_overview::erc20_portfolio_tracker::PortfolioRequest;

pub async fn web_route_erc721(request_body: web::Json<PortfolioRequest>) -> impl Responder {
    let nft_details = match get_nft_portfolio_data(request_body.user_address, request_body.chain.clone()).await {
        Ok(n) => n,
        Err(e) => return HttpResponse::BadRequest().json(json!({
            "nft_details": null,
            "err": e.to_string(),
        }))
    };

    let result = NftResponse{
        nft_details: Some(nft_details),
        err: None,
    };

    HttpResponse::Ok().json(result)
}

pub async fn get_nft_portfolio_data(user_address: Address, chain: String) -> Result<nft_identifier::AlchemyNftResponse> {
    let result = match nft_identifier::fetch_nft_data(chain, user_address).await {
        Ok(r) => r,
        Err(e) => return Err(Error::from(e.to_string())),
    };
    Ok(result)
}

#[derive(Serialize)]
pub struct NftResponse {
    pub nft_details: Option<AlchemyNftResponse>,
    pub err: Option<String>,
}