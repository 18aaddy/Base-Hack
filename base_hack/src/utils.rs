use web3::{Result, Error};

pub fn chain_from_chain_id(chain_id: u64) -> Result<String> {
    let network = match chain_id {
        1 => "ETHEREUM",
        8453 => "BASE",
        _ => return Err(Error::from("Unknown network".to_string())),
    };
    Ok(network.to_string())
}