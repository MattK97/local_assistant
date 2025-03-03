use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP request error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
}