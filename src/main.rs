// Simple Rust CLI chat with Mistral AI

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    env,
    io::{self, Read},
};

const MISTRAL_API_URL: &str = "https://api.mistral.ai/v1/chat/completions";

#[derive(Serialize, Deserialize, Debug)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Debug)]
struct ApiRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Choice {
    message: ChatMessage,
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    choices: Vec<Choice>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut prompt: String = "".to_string();
    if args.len() == 2
    // prompt as an arg
    {
        prompt = args[1].clone()
    } else
    // prompt as a stdin pipe
    {
        io::stdin()
            .read_to_string(&mut prompt)
            .expect("failed to read stdin");
    }

    let api_key = env::var("MISTRAL_API_KEY").expect("MISTRAL_API_KEY env var not set!");

    let request_payload = ApiRequest {
        model: "mistral-small-latest".to_string(),
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: prompt,
        }],
    };

    let client = Client::new();
    let response = client
        .post(MISTRAL_API_URL)
        .bearer_auth(api_key)
        .json(&request_payload)
        .send()
        .await
        .expect("failed to send request to Mistral API");

    if response.status().is_success() {
        let api_response = response
            .json::<ApiResponse>()
            .await
            .expect("failed to parse API response");

        if let Some(first_choice) = api_response.choices.get(0) {
            println!("{}", first_choice.message.content.trim());
        } else {
            eprintln!("error: empty response from API");
        }
    } else {
        eprintln!(
            "error: non-200 response from API, body: {}",
            response.text().await?
        );
    }

    Ok(())
}
