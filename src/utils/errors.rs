// src/utils/errors.rs

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorType {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Header value error: {0}")]
    HeaderValueError(#[from] reqwest::header::InvalidHeaderValue),
    #[error("JSON deserialization error: {0}")]
    JsonError(#[from] serde_json::Error),
}
