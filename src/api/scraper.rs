// src/api/scraper.rs

use crate::api::client::Client;
use crate::models::trading::account::Account;
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
}
