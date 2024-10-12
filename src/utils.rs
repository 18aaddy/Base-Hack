use web3::{Result, Error};

//TODO: Add all networks to this
pub fn chain_from_chain_id(chain_id: u64) -> Result<String> {
    let network = match chain_id {
        1 => "ETHEREUM",
        8453 => "BASE",
        10 => "OPTIMISM",
        _ => return Err(Error::from("Unknown network".to_string())),
    };
    Ok(network.to_string())
}