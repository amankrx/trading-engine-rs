// src/api/scraper.rs

use crate::api::client::Client;
use crate::models::trading::{account::Account, asset::Asset};
use crate::utils::errors::ErrorType;
pub struct Scraper {
    client: Client,
}

impl Scraper {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn fetch_account(&self) -> Result<Account, ErrorType> {
        let response_text = self.client.get("/v2/account").await?.text().await?;
        let account = Account::from_json(&response_text)?;
        Ok(account)
    }

    pub async fn fetch_assets(&self) -> Result<Vec<Asset>, ErrorType> {
        let response_text = self.client.get("/v2/assets").await?.text().await?;
        let assets = Asset::to_json(&response_text)?;
        Ok(assets)
    }

    pub async fn fetch_asset(&self, symbol: &str) -> Result<Asset, ErrorType> {
        let endpoint = format!("/v2/assets/{}", symbol);
        let response_text = self.client.get(&endpoint).await?.text().await?;
        let asset = Asset::from_json(&response_text)?;
        Ok(asset)
    }
}
