use mongodb::{bson::doc, options::ClientOptions, Client};
use web3::{ethabi::Address, types::{Log, H160}};
use futures::stream::TryStreamExt;
use dotenv::dotenv;
use std::env;

pub async fn read_from_db(chain: String, address: Address) -> mongodb::error::Result<Vec<Log>> {
    dotenv().ok();
    let uri = env::var("DATABASE_URI").expect(format!("{} not set", "DATABASE_URI").as_str());
    
    // Configure the client options NppGYkdW5FzLA35I
    let client_options = ClientOptions::parse(uri).await?;
    
    // Create the MongoDB client
    let client = Client::with_options(client_options)?;

    let db = client.database("Transfer_Logs");
    let collection = db.collection::<Log>(format!("{}_logs_{}", chain, address.to_string()).as_str());

    // Step 3: Query all documents using `find` with an empty filter
    let filter = doc! {}; // An empty filter matches all documents
    let mut cursor = collection.find(filter).await?;
    let mut logs = Vec::<Log>::new();

    // Step 4: Iterate over the cursor and print each user
    while let Some(log) = cursor.try_next().await? {
        println!("Found user: {:?}", log);
        logs.push(log);
    }

    Ok(logs)
}

pub fn get_contract_address_list_from_logs(logs: Vec<Log>) -> mongodb::error::Result<Vec<H160>> {
    let mut result = Vec::<H160>::new();
    for log in logs {
        result.push(log.address);
    }
    Ok(result)
}