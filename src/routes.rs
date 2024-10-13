use actix_web::web;

use crate::portfolio_overview::{erc20_portfolio_tracker, nft_portfolio_tracker};
use crate::transaction_history::transaction_history_fetcher;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/erc20")
            .route(web::post().to(erc20_portfolio_tracker::web_route_erc20))
    )
    .service(
        web::resource("/erc721")
            .route(web::post().to(nft_portfolio_tracker::web_route_erc721))
    )
    .service(
        web::resource("/transaction-history")
            .route(web::post().to(transaction_history_fetcher::web_route_transaction_history))
    );
}