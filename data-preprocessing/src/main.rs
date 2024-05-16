use mongodb::{Client, error::Error};

async fn connect_to_mongodb() -> Result<(), Error> {
    // Connect to MongoDB
    let _client = Client::with_uri_str("mongodb://localhost:27017").await?;
    println!("Connected to MongoDB");

    // Access databases and collections here

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = connect_to_mongodb().await {
        eprintln!("Error connecting to MongoDB: {}", e);
    }
}
