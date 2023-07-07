// src/api/client.rs

use crate::utils::errors::ErrorType;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client as ReqwestClient,
};

#[derive(Clone)]
pub struct Client {
    pub client: ReqwestClient,
    pub config: ClientConfig,
}

#[derive(Clone)]
pub struct ClientConfig {
    pub base_url: String,
    pub key: String,
    pub secret: String,
}

impl Client {
    pub fn new(config: ClientConfig) -> Result<Self, reqwest::Error> {
        let client = ReqwestClient::builder().build()?;
        Ok(Self { client, config })
    }

    pub async fn authenticate_request(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<reqwest::RequestBuilder, ErrorType> {
        let mut headers = HeaderMap::new();
        let key_header = HeaderValue::from_str(&self.config.key)?;
        headers.insert("APCA-API-KEY-ID", key_header);
        let secret_header = HeaderValue::from_str(&self.config.secret)?;
        headers.insert("APCA-API-SECRET-KEY", secret_header);
        Ok(request.headers(headers))
    }

    pub async fn get(&self, uri: &str) -> Result<reqwest::Response, ErrorType> {
        let endpoint = format!("{}{}", self.config.base_url, uri);
        let request = self.client.get(endpoint);
        let request = self.authenticate_request(request).await?;
        let response = request.send().await?;
        Ok(response)
    }
}

impl ClientConfig {
    pub fn new(base_url: String, key: String, secret: String) -> Self {
        Self {
            base_url,
            key,
            secret,
        }
    }
}
