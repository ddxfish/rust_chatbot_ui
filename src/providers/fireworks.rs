use super::Provider;
use std::fmt;
use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;

pub struct Fireworks {
    client: Client,
    api_key: String,
}

impl Fireworks {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }
}

#[async_trait::async_trait]
impl Provider for Fireworks {
    fn name(&self) -> &'static str {
        "Fireworks"
    }

    fn models(&self) -> Vec<&'static str> {
        vec![
            "llama-v3p1-405b-instruct",
            "llama-v3p1-70b-instruct",
            "llama-v3p1-8b-instruct",
        ]
    }

    async fn generate_response(&self, messages: Vec<Value>) -> Result<String, Box<dyn Error>> {
        let response = self.client
            .post("https://api.fireworks.ai/inference/v1/chat/completions")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
                "model": "accounts/fireworks/models/llama-v3p1-70b-instruct",
                "max_tokens": 16384,
                "top_p": 1,
                "top_k": 40,
                "presence_penalty": 0,
                "frequency_penalty": 0,
                "temperature": 0.6,
                "messages": messages
            }))
            .send()
            .await?;

        let response_json: Value = response.json().await?;
        Ok(response_json["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string())
    }
}

impl fmt::Display for Fireworks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fireworks")
    }
}