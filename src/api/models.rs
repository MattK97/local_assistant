use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ApiRequest {
    pub model: String,
    pub stream: bool,
    pub messages: Vec<Message>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse {
    pub message: Message,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Role {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}
