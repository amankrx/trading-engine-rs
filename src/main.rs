pub mod api;
pub mod models;
pub mod strategy;
pub mod utils;

use api::client::{Client, ClientConfig};
use api::scraper::Scraper;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let base_url = env::var("BASE_URL").expect("BASE_URL not found in environment variables");
    let key = env::var("API_KEY").expect("API_KEY not found in environment variables");
    let secret = env::var("API_SECRET").expect("API_SECRET not found in environment variables");
    let config = ClientConfig::new(base_url, key, secret);
    let client = Client::new(config)?;
    let scraper = Scraper::new(client.clone());
    let account = scraper.fetch_account().await?;
    println!("{}", account);

    Ok(())
}
