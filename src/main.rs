mod api;
mod config;
mod errors;
use crate::api::models::{Message, Role};
use api::client::ApiClient;
use api::models::ApiRequest;
use config::settings::Settings;
use errors::api_error::ApiError;
use std::io::{self, Write};
use tokio::io::{AsyncBufReadExt, BufReader};

async fn api_communication() -> Result<(), ApiError> {
    let settings = Settings::new()?;
    let client = ApiClient::new(&settings.api_url);
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin).lines();
    let mut messages: Vec<Message> = Vec::new();

    println!("Hey type anything, I'll try to answer");

    loop {
        print!("You: ");
        io::stdout().flush().unwrap();

        let input = match reader.next_line().await {
            Ok(Some(line)) => line,
            Ok(None) => break,
            Err(e) => {
                eprintln!("There was an error during reading your question: {}", e);
                break;
            }
        };
        if input.trim().eq_ignore_ascii_case("exit") {
            println!("See ya!");
            break;
        }

        let new_user_message = Message {
            role: Role::User,
            content: input.clone(),
        };

        messages.push(new_user_message);

        let request = ApiRequest {
            model: "deepseek-r1:14b".to_string(),
            stream: false,
            messages: messages.clone(),
        };

        let response = client.post_data(request).await?;

        messages.push(response.message.clone());

        println!("Bot: {:?}", response.message.clone().content);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    api_communication().await?;
    Ok(())
}
