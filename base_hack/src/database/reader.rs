use mongodb::{options::ClientOptions, Client};
use web3::types::Log;

use crate::utils::chain_from_chain_id;

pub async fn read_from_db(logs: Vec<Log>, chain_id: u64) -> mongodb::error::Result<()> {
    // Replace the URI string with your MongoDB deployment's connection string.
    let uri = "mongodb+srv://aaddyrocks123:NppGYkdW5FzLA35I@cluster0.svbav.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0"; // or your MongoDB Atlas connection string
    
    // Configure the client options NppGYkdW5FzLA35I
    let client_options = ClientOptions::parse(uri).await?;
    
    // Create the MongoDB client
    let client = Client::with_options(client_options)?;

    let db = client.database("Transfer_Logs");
    let collection = db.collection::<Log>(format!("{:?}_logs", chain_from_chain_id(chain_id)).as_str());

    collection.insert_many(logs).await?;

    Ok(())
}