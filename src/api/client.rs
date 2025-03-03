use crate::api::models::{ApiRequest, ApiResponse};
use crate::errors::api_error::ApiError;
use reqwest::Client;

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn post_data(&self, request: ApiRequest) -> Result<ApiResponse, ApiError> {
        let url = format!("{}/api/chat", self.base_url);
        let response = self.client.post(&url).json(&request).send().await?;
        let data = response.json::<ApiResponse>().await?;
        Ok(data)
    }
}
