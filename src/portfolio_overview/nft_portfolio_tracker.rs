use serde::Serialize;
use web3::ethabi::Address;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use anyhow::{Context,Result};
use crate::token_identifiers::nft_identifier;
use crate::portfolio_overview::erc20_portfolio_tracker::PortfolioRequest;

#[derive(Serialize)]
struct NftResponse {
    nft_details: Option<Vec<nft_identifier::NftSummary>>,
    err: Option<String>,
}
pub async fn web_route_erc721(request_body: web::Json<PortfolioRequest>) -> impl Responder {
    let addess2=request_body.user_address.clone();
    //let address:  = &request_body.user_address.to_string().parse().expect("Invalid Address");
    let nft_details = match get_nft_portfolio_data(&addess2, request_body.chain.clone()).await {
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
pub async fn get_nft_portfolio_data(user_address: &str, chain: String) -> Result<Vec<nft_identifier::NftSummary>> {
    // Fetch the NFT data
    let nft_response = nft_identifier::fetch_nft_data(chain.clone(), user_address)
        .await
        .context("Failed to fetch NFT data")?;

    // Fetch the summaries
    let nft_summaries = nft_identifier::fetch_nft_summary(chain.clone(), &nft_response)
        .await
        .context("Failed to fetch NFT summaries")?;

    // Print each NFT summary
    for nft_summary in &nft_summaries {
        println!("{:#?}", nft_summary);
    }

    // Return the list of summaries
    Ok(nft_summaries)
}
/*
pub async fn get_nft_portfolio_data(user_address: Address, chain: String) -> Result<nft_identifier::AlchemyNftResponse> {
    let result = match nft_identifier::fetch_nft_data(chain, user_address).await {
        Ok(r) => r,
        Err(e) => return Err(Error::from(e.to_string())),
    };
    Ok(result)
}
*/
//Previous Code
/* 
use serde::Serialize;
use web3::{ethabi::Address, Error, Result};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::token_identifiers::nft_identifier::{self, AlchemyNftResponse};
use crate::portfolio_overview::erc20_portfolio_tracker::PortfolioRequest;

pub async fn web_route_erc721(request_body: web::Json<PortfolioRequest>) -> impl Responder {
    let address: Address = request_body.user_address.parse().expect("Invalid Address");
    let nft_details = match get_nft_portfolio_data(address, request_body.chain.clone()).await {
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
    */