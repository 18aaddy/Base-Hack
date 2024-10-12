use mongodb::{
    bson::doc,
    options::{ClientOptions, IndexOptions, InsertManyOptions},
    Client, IndexModel,
};
use web3::{ethabi::Address, types::Log};

use crate::utils::chain_from_chain_id;

pub async fn write_to_db(logs: Vec<Log>, chain_id: u64, address: Address) -> mongodb::error::Result<()> {
    // Replace the URI string with your MongoDB deployment's connection string.
    let uri = "mongodb+srv://aaddyrocks123:NppGYkdW5FzLA35I@cluster0.svbav.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0"; // or your MongoDB Atlas connection string

    // Configure the client options NppGYkdW5FzLA35I
    let client_options = ClientOptions::parse(uri).await?;

    // Create the MongoDB client
    let client = Client::with_options(client_options)?;

    let db = client.database("Transfer_Logs");
    let collection =
        db.collection::<Log>(format!("{}_logs_{}", chain_from_chain_id(chain_id).unwrap(), address.to_string()).as_str());

    let index_model = IndexModel::builder()
        .keys(doc! { "address": 1 })
        .options(IndexOptions::builder().unique(true).build())
        .build();

    collection.create_index(index_model).await?;

    let options = InsertManyOptions::builder().ordered(false).build();

    match collection.insert_many(logs).with_options(options).await {
        Ok(result) => println!("Documents inserted: {:?}", result),
        Err(e) => {
            if e.to_string().contains("E11000") {
                println!("Duplicate entry detected: {}", e);
            } else {
                println!("Failed to insert documents: {}", e);
            }
        }
    }
    Ok(())
}
