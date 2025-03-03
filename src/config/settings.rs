use crate::errors::api_error::ApiError;

pub struct Settings {
    pub api_url: String,
}

impl Settings {
    pub fn new() -> Result<Self, ApiError> {
        let api_url = String::from("http://localhost:11434");
        Ok(Self { api_url })
    }
}
