use mongodb::{options::ClientOptions, Client};
use dotenv::dotenv;
use std::env;

pub async fn get_database() -> mongodb::Database {
    dotenv().ok();
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client_options = ClientOptions::parse(uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    client.database(&env::var("DATABASE_NAME").unwrap())
}
